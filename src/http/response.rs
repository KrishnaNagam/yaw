use tokio::net::TcpStream;

use crate::http::*;
use std::fmt;

use super::headers::Headers;
use tokio::io::AsyncWriteExt;

const SERVER_NAME: &str = "yaw";

#[derive(Debug, PartialEq)]
pub enum Status {
    Ok,
    Found,
    NotFound,
    NotImplemented,
    InvalidRequest,
    Unauthorized,
    MethodNotAllowed,
    InternalServerError,
    BadGateway,
    ServiceUnavailable,
}

impl Status {
    fn get_reason_phrase(&self) -> String{
        match self {
            Status::Ok => "Ok".to_string(),
            Status::Found => "Found".to_string(),
            Status::NotFound => "Not Found".to_string(),
            Status::NotImplemented => "Not Implemented".to_string(),
            Status::InvalidRequest => "Invalid Request".to_string(),
            Status::Unauthorized => "Unauthorized".to_string(),
            Status::MethodNotAllowed => "Method Not Allowed".to_string(),
            Status::InternalServerError => "Internal Server Error".to_string(),
            Status::BadGateway => "Bad Gateway".to_string(),
            Status::ServiceUnavailable => "Service Unavailable".to_string(),
        }
    }
}

pub struct StatusLine {
    http_version: HttpVerion,
    status_code: Status,
    reason_phrase: String,
}

impl StatusLine {
    pub fn new() -> StatusLine {
        StatusLine {
            http_version: "HTTP/1.1".to_string(),
            status_code: Status::Ok,
            reason_phrase: Status::Ok.get_reason_phrase()
        }
    }

    pub fn set_status_code(&mut self, status_code: Status) {
        self.reason_phrase = status_code.get_reason_phrase();
        self.status_code = status_code;
    }

    pub fn get_status_code(&self) -> &Status {
        &self.status_code
    }
}

impl fmt::Display for StatusLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.http_version,
            match self.status_code {
                Status::Ok => "200",
                Status::Found => "302",
                Status::NotFound => "404",
                Status::NotImplemented => "501",
                Status::InvalidRequest => "400",
                Status::Unauthorized => "401",
                Status::MethodNotAllowed => "405",
                Status::InternalServerError => "500",
                Status::BadGateway => "502",
                Status::ServiceUnavailable => "503",
            }.to_string(), 
            self.reason_phrase
        )
    }
}

pub struct Response {
    status_line: StatusLine,
    headers: Headers,
    body: String,
}

impl Response {
    pub fn new() -> Response {
        let mut headers = Headers::new();
        headers.add_header(headers::SERVER, SERVER_NAME);
        headers.add_header(headers::CONTENT_LENGTH, "0");
        Response {
            status_line: StatusLine::new(),
            headers: headers,
            body: "".to_string(),
        }
    }
    pub fn set_status_code(&mut self, status_code: Status) {
        self.status_line.set_status_code(status_code);
    }

    pub fn add_header(&mut self, header_key: &str, header_value: &str) {
        self.headers.add_header(header_key, header_value);
    }

    pub fn set_body(&mut self, content: String) {
        let length = content.len();
        self.add_header(headers::CONTENT_LENGTH, &length.to_string());
        self.body = content;
    }

    pub fn get_status_code(&self) -> &Status{
        &self.status_line.get_status_code()
    }

    pub async fn send_to(&self, mut stream: TcpStream) -> Result<usize, std::io::Error> {
        stream.write(self.to_string().as_bytes()).await
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.status_line, CRLF, self.headers, CRLF, self.body
        )
    }
}
