use std::{
    fs,
    net::{TcpListener,TcpStream},
    thread,
    time::Duration,
    io::{prelude::*}
};

use web_server::{ThreadPool,response,request};

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
    
    let request = request::Request::load(&mut stream);

    let mut response = response::Response::new();
    let path = "root".to_string() + request.get_uri();
    let auth_key = "Basic dXNlcjpwYXNz".to_string();

    let (status_code, file_name) = match (request.get_method(),request.get_uri()) {

        (request::Method::GET, "/") => (response::Code::STATUS200, "root/hello.html"),
        (request::Method::GET, "/admin") => {
            if request.get_header("Authorization") == Some(&auth_key) {
                (response::Code::STATUS200, "root/admin.html")
            } else {
                (response::Code::STATUS401, "root/401.html")
            }
        },

        (request::Method::GET, "/sleep") => {
            thread::sleep(Duration::from_secs(5));
            (response::Code::STATUS200, "hello.html")
        },

        (request::Method::GET, _) => {
            if fs::metadata(path.as_str()).is_ok() {
                (response::Code::STATUS200, path.as_str() )
            } else {
                (response::Code::STATUS404, "root/404.html")
            }
        },

        (_, _) => (response::Code::STATUS501,"501.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();

    response.set_status(status_code);
    response.set_body(contents);
    response.add_header("WWW-Authenticate: Basic realm=\"WallyWorld\"".to_string());

    stream.write(response.string().as_bytes()).unwrap();
    

}