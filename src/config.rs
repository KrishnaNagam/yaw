use std::collections::HashMap;

use crate::http::{request::Request, response::Response};

pub struct Config {
    pub port: u32,
    pub root_path: String,
    pub index: String,
    pub username: String,
    pub password: String,
    pub request_handlers: HashMap<String, fn(request: &Request,response: &mut Response)>,
}

impl Config {
    pub fn default() -> Config {
        Config {
            port: 8080,
            root_path: "root/".to_string(),
            index: "index.html".to_string(),
            username: "user".to_string(),
            password: "password".to_string(),
            request_handlers: HashMap::new(),
        }
    }

    pub fn set_index(&mut self, index: &str) {
        self.index = index.to_string();
    }
    pub fn add_request_handler(&mut self, path: &str,handler: fn(request: &Request,response: &mut Response)) {
        self.request_handlers.insert(path.to_string(), handler);
    }
}