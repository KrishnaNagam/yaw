use std::{
    thread,
    sync::{Arc, Mutex, mpsc},
};

type Job = Box<dyn FnOnce() + Send + 'static>;


pub mod request {
    use std::{net::TcpStream, io::{BufReader, prelude::*}, collections::HashMap};
    type URI = String;
    type HttpVerion = String;
    type Params = HashMap<String, String>;

    pub struct Request {
        method: Method,
        path: String,
        params: HashMap<String, String>,
        http_version: String,
        headers: HashMap<String, String>
    }
    
    pub enum Method {
        GET,
        POST,
        PUT,
        DELETE,
        HEAD
    }
    
    impl Request {
        pub fn load(request_stream: &mut TcpStream) -> Request{
            let mut buf_reader = BufReader::new(request_stream);
            let mut request_line = String::new();
            buf_reader.read_line(&mut request_line).unwrap();
            let (method, path, params, http_version) = Self::parse_request_line(request_line);
            let mut headers: HashMap<String,String> = HashMap::new();
            loop {
                let mut line = String::new();
                buf_reader.read_line(&mut line).unwrap();
                if line == "\r\n" {
                    break;
                }
                let (header_key, header_value) = line.split_once(':').unwrap();
                headers.insert(header_key.to_string(), header_value.trim().to_string());
                //print!("{}",line)
            }
            
            //print!("{}\n",headers.get("Authorization").unwrap_or(&"Auth header not found".to_string()));
    
            Request {
                method: method,
                path: path,
                params: params,
                http_version: http_version,
                headers: headers,
            }
        }

        pub fn get_path(&self) -> &str{
            &self.path
        }

        pub fn get_method(&self) -> &Method{
            &self.method
        }

        pub fn get_header(&self, header: &str) -> Option<&String>{
            self.headers.get(header)
        }

        pub fn get_param(&self, param: &str) -> Option<&String>{
            self.params.get(param)
        }

        fn parse_request_line(request_line: String) -> (Method, URI, Params, HttpVerion){
            let mut request_items = request_line.split_ascii_whitespace().map(|s| s.to_string());
            let method = match request_items.next().unwrap().as_str() {
                "GET" => Method::GET,
                "POST" => Method::POST,
                "PUT" => Method::PUT,
                "DELETE" => Method::DELETE,
                "HEAD" => Method::HEAD,
                _ => panic!("Unkown Request Method")
            };
            let request_target = request_items.next().unwrap();
            let (path, query_string) = request_target.split_once('?').unwrap_or_else( || (request_target.as_str(), ""));
            let params_strings_list = query_string.split('&');
            let mut params: HashMap<String,String> = HashMap::new();
            for param_string in params_strings_list {
                let (param_key, param_value) = param_string.split_once('=').unwrap_or_else( || (param_string, ""));
                params.insert(param_key.to_string(), param_value.to_string());
            }
            let http_version = request_items.next().unwrap();

            (method, path.to_string(), params, http_version)
        }
        
    }
    
}

pub mod response {
    pub enum Code {
        STATUS200,
        STATUS404,
        STATUS401,
        STATUS501
    }
    
    
    pub struct Response {
        status: String, //TODO change to enum
        headers: String,
        body: String,
    }
    
    impl Response {
        pub fn new() -> Response{
            Response {
                status: "200 Ok".to_string(),
                headers: "Server: rust server\r\n".to_string(),
                body: "".to_string()
            }
        }
        pub fn set_status(&mut self,status: Code){
            self.status = match status {
                Code::STATUS200 => "200 Ok".to_string(),
                Code::STATUS404 => "404 Not Found".to_string(),
                Code::STATUS501 => "501 Not Implemented".to_string(),
                Code::STATUS401 => "401 Unauthorized".to_string()
            };
        }
    
        pub fn add_header(&mut self,header: String){
            self.headers.push_str(&format!("{header}\r\n"));
        }
    
        pub fn set_body(&mut self,content: String){
            let length = content.len();
            self.add_header(format!("Content-Length: {length}"));
            self.body = content;
        }
        pub fn string(&self) -> String {
            format!("HTTP/1.1 {}\r\n{}\r\n{}",self.status,self.headers,self.body)
        }
    }
    
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool.
    /// 
    ///  # panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push( Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f:F) 
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = std::thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}

