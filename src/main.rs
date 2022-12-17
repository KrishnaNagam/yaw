use std::{
    fs,
    net::{TcpListener,TcpStream},
    thread,
    time::Duration,
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

    let mut response = response_module::Response::new();
    let path = "root".to_string() + request.get_path();
    let auth_key = "Basic dXNlcjpwYXNz".to_string();

    let (status_code, file_name) = 
    
        match (request.get_method(),request.get_path()) {

            (request_module::Method::GET, "/") => (response_module::Code::STATUS200, "root/hello.html"),
            (request_module::Method::GET, "/admin") => {
                if request.get_header("Authorization") == Some(&auth_key) {
                    (response_module::Code::STATUS200, "root/admin.html")
                } else {
                    response.add_header("WWW-Authenticate: Basic realm=\"WallyWorld\"".to_string());
                    (response_module::Code::STATUS401, "root/401.html")
                }
            },

            (request_module::Method::GET, "/sleep") => { //TODO take from params
                thread::sleep(Duration::from_secs(request.get_param("time").unwrap_or(&"5".to_string()).parse().unwrap()));
                (response_module::Code::STATUS200, "root/hello.html")
            },

            (request_module::Method::GET, _) => {
                if fs::metadata(path.as_str()).is_ok() {
                    (response_module::Code::STATUS200, path.as_str() )
                } else {
                    (response_module::Code::STATUS404, "root/404.html")
                }
            },

            (_, _) => (response_module::Code::STATUS501,"501.html")
        };

    let contents = fs::read_to_string(file_name).unwrap();

    response.set_status(status_code);
    response.set_body(contents);
    
    stream.write(response.string().as_bytes()).unwrap();
    

}