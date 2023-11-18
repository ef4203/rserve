// RFC 2616, Section 6.1.1
// https://www.rfc-editor.org/rfc/rfc2616

#[derive(Copy, Clone)]
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

pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: Vec<String>,
    pub body: String,
}

impl HttpRequest {
    pub fn from_request(a: Vec<String>) -> HttpRequest {
        let mut request_line = a[0].split(' ').collect::<Vec<&str>>();
        return HttpRequest {
            method: request_line[0].to_string(),
            path: request_line[1].to_string(),
            headers: a,
            body: "123".to_string(),
        };
    }
}