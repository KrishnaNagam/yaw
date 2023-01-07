pub mod request;
pub mod response;

use std::{collections::HashMap,fmt};

type HeaderKey = String;
type HeaderValue = String;
type Body = String;

pub const CRLF: &str = "\r\n";

pub struct Headers {
    headers: HashMap<String, String>,
}

impl Headers {
    pub fn new() -> Headers {
        Headers {
            headers: HashMap::new()
         }
    }
    pub fn parse_header_field(&mut self, header_field: String) {
        let (header_key, header_value) = header_field.split_once(':').unwrap();
        self.add_header(header_key.to_string(), header_value.to_string());
    }
    pub fn get_header(&self, header: &str) -> Option<&String> {
        self.headers.get(header)
    }

    pub fn add_header(&mut self, header_key: String, header_value: String) {
        self.headers.insert(header_key.trim().to_string(), header_value.trim().to_string());
    }
}

impl fmt::Display for Headers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (_size, (header_key, header_value)) in self.headers.iter().enumerate() {
            write!(f, "{}: {}\r\n", header_key, header_value)?
        }

        write!(f, "")
    }
}
