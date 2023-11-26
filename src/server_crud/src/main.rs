mod routes;

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use routes::user::{handle_get, handle_post};

use std::collections::HashMap;

type HandleRequest = fn(Vec<String>) -> String;
type RoutesMap = HashMap<&'static str, HashMap<&'static str, HandleRequest>>;

fn defining_routes(routes_map: &mut RoutesMap) {
    // Route: /
    let mut map_1: HashMap<&str, HandleRequest> = HashMap::new();
    // Methods: GET, POST
    map_1.insert("GET", handle_get);
    map_1.insert("POST", handle_post);
    routes_map.insert("/", map_1);
}

fn handle_connection(mut stream: TcpStream, routes_map: &RoutesMap) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let mut request_words = http_request[0].split_whitespace();

    let method = request_words.next().unwrap();
    let requested_route = request_words.next().unwrap();

    println!("Method: {method}, route: {requested_route}");

    let response = match routes_map.get(requested_route) {
        Some(map) => match map.get(method) {
            Some(func) => func(http_request),
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
