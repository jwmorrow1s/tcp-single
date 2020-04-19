use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::thread;
use std::time::Duration;

enum ResponseType {
    OK, NotFound 
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    if buffer.starts_with(get){
        handle_response(stream, ResponseType::OK);
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        handle_response(stream, ResponseType::OK);
    } else {
        handle_response(stream, ResponseType::NotFound);
    }
}

fn handle_response(mut stream: TcpStream, response_type: ResponseType) -> TcpStream {
    let (status_line, contents): (&str, String) = match response_type {
        ResponseType::OK => ("200 OK", fs::read_to_string("hello.html").unwrap()),
        _ => ("404 NOT FOUND", fs::read_to_string("404.html").unwrap())
    };
    let response = 
        format!("HTTP/1.1 {}\r\n\r\n{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    stream
}