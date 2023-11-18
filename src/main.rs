use std::io::{Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};


mod http;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}

fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
    let parsed = http::HttpRequest::from_request(http_request);

    println!("METHOD: {}", parsed.method);
    println!("PATH: {}", parsed.path);
    println!("HEADERS: {:#?}", parsed.headers);
    println!("BODY: {}", parsed.body);

    let mut response = "HTTP/1.1".to_string();
    response.push_str(" ");
    response.push_str(http::StatusCode::NotFound.to_string().as_str());
    response.push_str(" ");
    response.push_str(http::StatusCode::NotFound.to_reason_phrase());
    response.push_str("\nContent-length: 2\n\nOk\n");

    stream.write_all(response.as_bytes()).unwrap();
}