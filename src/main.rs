use std::{
    net::{TcpListener,TcpStream},
    io::{prelude::*}
};

use web_server::{ThreadPool,response as response_module, request as request_module};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream)
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    
    let request = request_module::Request::load(&mut stream);

    let response = request.process();   
    
    stream.write(response.string().as_bytes()).unwrap();
    

}