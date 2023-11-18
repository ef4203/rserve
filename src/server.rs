use std::fs;
use std::io::{Write, BufReader};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use crate::http::{HttpRequest, HttpResponse, StatusCode};

pub struct Server {
}

impl Server {
    pub fn run(port: i32) {
        let mut address = "0.0.0.0:".to_string();
        address.push_str(port.to_string().as_str());

        let listener = TcpListener::bind(address).unwrap();
        for stream in listener.incoming() {
            handle(stream.unwrap());
        }
    }
}
fn handle(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let mut parsed: HttpRequest = HttpRequest::from_buffer(buf_reader);
    // println!("parsed: {:#?}", parsed);

    let response = handle_get(&mut parsed);
    stream.write_all(response.to_string().as_bytes()).unwrap();
}

fn handle_get(request: &mut HttpRequest) -> HttpResponse {
    let mut path = "".to_string();
    path.push_str(request.path.as_str());
    return if Path::new(path.as_str()).exists() {
        HttpResponse {
            status: StatusCode::Ok,
            body: fs::read_to_string(path).unwrap(),
        }
    } else {
        HttpResponse {
            status: StatusCode::NotFound,
            body: "Not Found".to_string(),
        }
    }
}
