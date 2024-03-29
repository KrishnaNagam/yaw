use std::{collections::HashMap, fmt};

use super::{CRLF, ParseError};

type HeaderKey = String;
type HeaderValue = String;
pub struct Headers {
    headers: HashMap<HeaderKey, HeaderValue>,
}

pub const WWW_AUTHENTICATE: &str = "WWW-Authenticate";
pub const CONTENT_LENGTH: &str = "Content-Length";
pub const SERVER: &str = "Server";


impl Headers {
    pub fn new() -> Headers {
        Headers {
            headers: HashMap::new()
         }
    }
    pub fn parse_and_add_header_from(&mut self, header_field: String) -> Result<(), ParseError> {
        match header_field.split_once(':') {
            Some((header_key, header_value)) => {
                self.add_header(header_key, header_value);
                Ok(())
            }
            None => Err(ParseError)
        }
    }

    pub fn get_header(&self, header: &str) -> Option<&String> {
        self.headers.get(header)
    }

    pub fn add_header(&mut self, header_key: &str, header_value: &str) {
        self.headers.insert(header_key.trim().to_string(), header_value.trim().to_string());
    }
}

impl fmt::Display for Headers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (_size, (header_key, header_value)) in self.headers.iter().enumerate() {
            write!(f, "{}: {}{}", header_key, header_value, CRLF)?
        }

        write!(f, "")
    }
}
