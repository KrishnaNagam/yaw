use std::{sync::Arc, thread, time::Duration};

use tokio::net::{TcpListener, TcpStream};

use crate::{
    config::Config,
    http::{
        request::{self, Request, Method},
        response::{Response, Status},
    },
    request_processor::{RequestProcessor},
};

pub struct Server {
    config: Arc<Config>,
    request_processor: Arc<RequestProcessor>,
}

impl Server {
    pub fn new() -> Result<Server, Box<dyn std::error::Error>> {
        let config = Arc::new(Config::default());
        Ok(Server {
            config: config.clone(),
            request_processor: Arc::new(RequestProcessor::new(config.clone())),
        })
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let host = "127.0.0.1";
        let listener = TcpListener::bind(host.to_string() + ":" + self.config.port.to_string().as_str())
            .await
            .expect("unable to bind to port 8080");
        loop {
            let (socket, _) = listener.accept().await?;
            let request_processor = self.request_processor.clone();
            tokio::spawn(async move { Self::handle_connection(socket, request_processor).await });
        }
    }

    pub fn set_config(&mut self, config: Config) {
        self.config = Arc::new(config);
        self.request_processor = Arc::new(RequestProcessor::new(self.config.clone()))
    }

    pub async fn handle_connection(mut stream: TcpStream, request_processor: Arc<RequestProcessor>) {
        let mut response = Response::new();
        let request: Request;

        match request::Request::load(&mut stream).await {
            Ok(request_from_stream) => request = request_from_stream,
            Err(http_error) => {
                response = http_error.to_response();
                response.send_to(stream).await;
                return;
            }
        }
        match request_processor.process(request) {
            Ok(response_from_processor) => response = response_from_processor,
            Err(http_error) => {
                response = http_error.to_response();
                response.send_to(stream).await;
                return;
            }
        }
        response.send_to(stream).await;
    }

    
}
