use std::{
    thread,
    sync::{Arc, Mutex, mpsc},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub mod request {
    use std::{net::TcpStream, io::{BufReader, prelude::*}, collections::HashMap};

    pub struct Request {
        method: Method,
        uri: String,
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
            let mut request_items = request_line.split_ascii_whitespace().map(|s| s.to_string());
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
                method: match request_items.next().unwrap().as_str() {
                    "GET" => Method::GET,
                    "POST" => Method::POST,
                    "PUT" => Method::PUT,
                    "DELETE" => Method::DELETE,
                    "HEAD" => Method::HEAD,
                    _ => panic!("Unkown Request Method")
                },
                uri: request_items.next().unwrap(),
                http_version: request_items.next().unwrap(),
                headers: headers,
            }
        }
        pub fn get_uri(&self) -> &str{
            &self.uri
        }
        pub fn get_method(&self) -> &Method{
            &self.method
        }
        pub fn get_header(&self, header: &str) -> Option<&String>{
            self.headers.get(header)
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
                    println!("Woekr {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}

