use std::{net::{TcpListener, TcpStream}, sync::Arc, io::Write};

use web_server::ThreadPool;

use super::request;


pub struct Config {
    pub port: u32,
    pub root_path: String,
    pub index: String,
    pub username: String,
    pub password: String
}

impl Config {
    pub fn new() -> Config {
        Config {
            port: 8080,
            root_path: "root/".to_string(),
            index: "index.html".to_string(),
            username: "user".to_string(),
            password: "password".to_string()
        }
    }

    pub fn set_index(&mut self, index: &str) {
        self.index = index.to_string();
    }
}
pub struct Server {
    config: Config,
    thread_pool: ThreadPool
}

impl Server {
    pub fn new() -> Server {
        let config = Config::new();
        Server { 
            config: config,
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
    
    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }
    
    pub fn handle_connection (mut stream: TcpStream, config: Arc<Config>) {
    
        let request = request::Request::load(&mut stream);

        let request_processor= request::RequestProcessor::new(config);
    
        let response = request_processor.process(request);   
        
        stream.write(response.to_string().as_bytes()).unwrap();
        
    
    }
}
