mod routes;

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use dotenv::dotenv;

use routes::user::{handle_get, handle_post};

use std::collections::HashMap;

type HandleRequest = fn(Vec<String>, String) -> String;
type RoutesMap = HashMap<&'static str, HashMap<&'static str, HandleRequest>>;

fn defining_routes(routes_map: &mut RoutesMap) {
    // Route: /
    let mut map_1: HashMap<&str, HandleRequest> = HashMap::new();
    // Methods: GET, POST
    map_1.insert("GET", handle_get);
    map_1.insert("POST", handle_post);
    routes_map.insert("/", map_1);
}

fn manage_headers(
    buf_reader: &mut BufReader<&mut TcpStream>,
    content_length: &mut usize,
) -> Vec<String> {
    let mut request_headers: Vec<String> = Vec::new();

    for line in buf_reader.by_ref().lines() {
        let line = line.unwrap();
        if line.contains("Content-Length:") {
            let parts: Vec<_> = line.split_whitespace().collect();
            *content_length = parts[1].parse::<usize>().unwrap();
        }
        if line.is_empty() {
            break;
        }
        request_headers.push(line);
    }

    request_headers
}

fn get_body(
    buf_reader: &mut BufReader<&mut TcpStream>,
    content_length: &usize,
) -> Result<String, String> {
    let mut body = String::new();
    if content_length <= &0 {
        return Err(String::from("Body does not exists"));
    } else {
        buf_reader
            .by_ref()
            .take(*content_length as u64)
            .read_to_string(&mut body)
            .unwrap();
    }
    Ok(body)
}

fn handle_connection(mut stream: TcpStream, routes_map: &RoutesMap) {
    let mut buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let mut content_length = 0usize;

    let request_headers = manage_headers(&mut buf_reader, &mut content_length);

    let body = match get_body(&mut buf_reader, &content_length) {
        Ok(body) => body,
        _ => String::new(),
    };

    println!("Request: {:#?}", request_headers);
    println!("Body: {body}");

    let mut request_words = request_headers[0].split_whitespace();

    let method = request_words.next().unwrap();
    let requested_route = request_words.next().unwrap();

    println!("Method: {method}, route: {requested_route}");

    let response = match routes_map.get(requested_route) {
        Some(map) => match map.get(method) {
            Some(func) => func(request_headers, body),
            _ => String::from("HTTP/1.1 404 NOT_FOUND\r\n\r\n"),
        },
        _ => String::from("HTTP/1.1 404 NOT_FOUND\r\n\r\n"),
    };
    // let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    // Defining server address
    let address: &str = "0.0.0.0";
    let port: &str = "3000";

    dotenv().ok();

    // Defining routes
    let mut routes_map: RoutesMap = HashMap::new();

    defining_routes(&mut routes_map);

    // Exec server
    let listener: TcpListener = TcpListener::bind(format!("{address}:{port}")).unwrap();
    println!("Server running in http://{address}:{port}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Receiving data ...");
        handle_connection(stream, &routes_map)
    }
}
