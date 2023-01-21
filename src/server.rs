use std::{net::{TcpListener, TcpStream}, sync::Arc, io::Write};

use crate::{http::request, request_processor::RequestProcessor, ThreadPool, config::Config};





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

        let request_processor= RequestProcessor::new(config);
    
        let response = request_processor.process(request);   
        
        stream.write(response.to_string().as_bytes()).unwrap();
        
    
    }
}
