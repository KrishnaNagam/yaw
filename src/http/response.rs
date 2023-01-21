use std::fmt;
use crate::http::*;

use super::headers::Headers;


#[derive(Debug, PartialEq)]
pub enum StatusCode {
    STATUS200,
    STATUS404,
    STATUS400,
    STATUS401,
    STATUS405,
    STATUS500,
    STATUS501,
    STATUS502,
    STATUS503,
    UNKNOWN,
}

pub struct StatusLine {
    http_version: HttpVerion,
    status_code: StatusCode,
    reason_phrase: String,
}

impl StatusLine {
    pub fn new() -> StatusLine {
        StatusLine {
            http_version: "HTTP/1.1".to_string(),
            status_code: StatusCode::UNKNOWN,
            reason_phrase: "Internal Server Error".to_string()
        }
    }
    
    pub fn set_status_code(&mut self, status_code: StatusCode) {
        self.status_code = status_code;
        self.reason_phrase = match self.status_code {
            StatusCode::STATUS200 => "Ok".to_string(),
            StatusCode::STATUS404 => "Not Found".to_string(),
            StatusCode::STATUS501 => "Not Implemented".to_string(),
            StatusCode::STATUS400 => "Invalid Request".to_string(),
            StatusCode::STATUS401 => "Unauthorized".to_string(),
            StatusCode::STATUS405 => "Method Not Allowed".to_string(),
            StatusCode::STATUS500 => "Internal Server Error".to_string(),
            StatusCode::STATUS502 => "Bad Gateway".to_string(),
            StatusCode::STATUS503 => "Service Unavailable".to_string(),
            StatusCode::UNKNOWN => "Internal Server Error".to_string()
        };
    }

    pub fn get_status_code(&self) -> &StatusCode {
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
                StatusCode::STATUS200 => "200",
                StatusCode::STATUS404 => "404",
                StatusCode::STATUS501 => "501",
                StatusCode::STATUS400 => "400",
                StatusCode::STATUS401 => "401",
                StatusCode::STATUS405 => "405",
                StatusCode::STATUS500 => "500",
                StatusCode::STATUS502 => "502",
                StatusCode::STATUS503 => "503",
                StatusCode::UNKNOWN => "500",
            }.to_string(), 
            self.reason_phrase
        )
    }
}


pub struct Response {
    status_line: StatusLine, //TODO change to enum
    headers: Headers,
    body: String,
}

impl Response {
    pub fn new() -> Response{
        let mut headers = Headers::new();
        headers.add_header("Server".to_string(), "rust server".to_string());
        Response {
            status_line: StatusLine::new(),
            headers: headers,
            body: "".to_string()
        }
    }
    pub fn set_status_code(&mut self,status_code: StatusCode) {
        self.status_line.set_status_code(status_code);
    }

    pub fn add_header(&mut self, header_key: String, header_value: String) {
        self.headers.add_header(header_key, header_value);
    }

    pub fn set_body(&mut self,content: String){
        let length = content.len();
        self.add_header("Content-Length".to_string(), length.to_string());
        self.body = content;
    }

    pub fn get_status_code(&self) -> &StatusCode{
        &self.status_line.get_status_code()
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "{}{}{}{}{}",
            self.status_line,
            CRLF,
            self.headers,
            CRLF,
            self.body
        )
    }
}
