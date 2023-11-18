// RFC 2616, Section 6.1.1
// https://www.rfc-editor.org/rfc/rfc2616

use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Continue = 100,
    SwitchingProtocols = 101,
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    // ...
    BadRequest = 400,
    // ...
    NotFound = 404,
    // ...
    InternalServerError = 500,
    // ..
}

impl StatusCode {
    pub fn to_reason_phrase(&self) -> &'static str{
        return match self {
            StatusCode::Continue => "Continue",
            StatusCode::SwitchingProtocols => "Switching Protocols",
            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::Accepted => "Accepted",
            StatusCode::NonAuthoritativeInformation => "Non-Authoritative Information",
            StatusCode::NoContent => "No Content",
            StatusCode::ResetContent => "Reset Content",
            // ...
            StatusCode::BadRequest => "Bad Request",
            // ...
            StatusCode::NotFound => "Not Found",
            // ...
            StatusCode::InternalServerError => "Internal Server Error",
            // ...
        }
    }

    pub fn to_string(&self) -> String {
        return (*self as i32).to_string();
    }
}

#[derive(Debug)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

impl HttpHeader {
    pub fn from_string(s: String) -> HttpHeader {
        let a = s.split(':').collect::<Vec<_>>();
        if a.len() < 2 {
            panic!("invalid header");
        }
        return HttpHeader {
            name: a[0].to_string(),
            value: a[1].to_string(),
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: Vec<HttpHeader>,
    pub body: String,
}

impl HttpRequest {
    pub fn from_buffer(mut buffer: BufReader<&mut TcpStream>) -> HttpRequest {
        let mut request_line = String::new();
        buffer.read_line(&mut request_line).unwrap();
        let l: Vec<_> = buffer.by_ref().lines()
            .map(|x| x.unwrap())
            .take_while(|x| !x.is_empty())
            .map(|x| HttpHeader::from_string(x))
            .collect();

        let request_line_pat: Vec<_> = request_line.split(' ')
            .collect();

        return HttpRequest {
            method: request_line_pat[0].to_string(),
            path: request_line_pat[1].to_string(),
            headers: l,
            body: "".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status: StatusCode,
    pub body: String,
}

impl HttpResponse {
    pub fn to_string(&self) -> String {
        let mut response = "HTTP/1.1".to_string();
        response.push_str(" ");
        response.push_str(self.status.to_string().as_str());
        response.push_str(" ");
        response.push_str(self.status.to_reason_phrase());
        response.push_str("\nContent-length: ");
        response.push_str(self.body.len().to_string().as_str());
        response.push_str("\n\n");
        response.push_str(self.body.as_str());
        response.push_str("\n");

        return response;
    }
}
