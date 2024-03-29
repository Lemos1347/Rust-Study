use server_rustbook::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("TCP Port succesfully binded in: '127.0.0.1:7878'");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let my_thread = thread::spawn(|| {
            handle_connection(stream);
        });

        let res = my_thread.join();

        println!("{:?}", res);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "../hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(20));
            ("HTTP/1.1 200 OK", "../hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "../404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
