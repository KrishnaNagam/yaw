pub mod request;
pub mod response;

use std::{collections::HashMap,fmt,net::{TcpListener,TcpStream},io::{prelude::*}, thread};
use web_server::{ThreadPool};
use crate::http::request as request_module;
use std::sync::Arc;

type HeaderKey = String;
type HeaderValue = String;
type Body = String;

pub const CRLF: &str = "\r\n";

pub struct Headers {
    headers: HashMap<HeaderKey, HeaderValue>,
}

#[derive(Clone)]
pub struct Config {
    port: u32,
    root_path: String,
    index: String,
    username: String,
    password: String
}

pub struct Server {
    config: Config,
    thread_pool: ThreadPool
}

impl Server {
    pub fn new() -> Server {
        Server { 
            config: Config {
                port: 8080,
                root_path: "root/".to_string(),
                index: "hello.html".to_string(),
                username: "user".to_string(),
                password: "password".to_string()
            },
            thread_pool: ThreadPool::new(4)
        }
    }
    pub fn run (self) {
        let host = "127.0.0.1";
        let listener = TcpListener::bind(host.to_string() + ":" + self.config.port.to_string().as_str()).unwrap();
        let config = Arc::new(self.config);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let config = config.clone();
            self.thread_pool.execute(|| {
                Self::handle_connection(stream, config);
            });
        }
    }
    
    
    pub fn handle_connection (mut stream: TcpStream, config: Arc<Config>) {
    
        let request = request_module::Request::load(&mut stream);

        let request_processor= request_module::RequestProcessor::new(config);
    
        let response = request_processor.process(request);   
        
        stream.write(response.to_string().as_bytes()).unwrap();
        
    
    }
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
            write!(f, "{}: {}{}", header_key, header_value, CRLF)?
        }

        write!(f, "")
    }
}
