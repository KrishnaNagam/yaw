use std::{fs, thread, time::Duration, net::TcpStream, io::{BufReader, prelude::*}, collections::HashMap};
use crate::http::*;
use super::response;

type URI = String;
type HttpVerion = String;
type Params = HashMap<String, String>;

pub struct Request {
    request_line: RequestLine,
    headers: Headers,
    body: Body,
}

pub struct Query {
    params: Params
}

impl Query {
    pub fn parse(query: String) -> Query {
        let params_strings_list = query.split('&');
        let mut params: HashMap<String,String> = HashMap::new();
        for param_string in params_strings_list {
            let (param_key, param_value) 
                    = param_string.split_once('=').unwrap_or_else( || (param_string, ""));
            params.insert(param_key.to_string(), param_value.to_string());
        }
        Query {
            params: params
        }
    }

    pub fn get_param(&self, param: &str) -> Option<&String> {
        self.params.get(param)
    }
}
pub struct RequestTarget {
    absolute_path: String,
    query: Option<Query>,
}

impl RequestTarget {
    pub fn parse(request_target: String) -> RequestTarget {
        let request_target_items = request_target.split_once('?');
        let (absolute_path, query_string) = if request_target_items.is_none() {
            (request_target, None)
        } else {
            (request_target_items.unwrap().0.to_string(), Some(request_target_items.unwrap().1))
        };
        let query = if query_string.is_none() {
            None
        } else {
            Some(Query::parse(query_string.unwrap().to_string()))
        };
        RequestTarget { 
            absolute_path: absolute_path.to_string(),
            query: query,
        }
    }

    pub fn get_path(&self) -> &str{
        &self.absolute_path
    }

    pub fn get_param(&self, param: &str) -> Option<&String> {
        self.query.as_ref()
            .and_then(|query| 
                query.get_param(param)
            )
    }
}

pub struct RequestLine {
    pub method: Method,
    pub request_target: RequestTarget,
    pub http_version: HttpVerion,
}

impl RequestLine {
    pub fn parse(request_line: String) -> RequestLine{
        let mut request_line_items 
                = request_line.split_ascii_whitespace().map(|s| s.to_string());
        let method = match request_line_items.next().unwrap().as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "HEAD" => Method::HEAD,
            _ => panic!("Unkown Request Method")
        };
        let request_target = request_line_items.next().unwrap();
        let request_target = RequestTarget::parse(request_target);
        let http_version = request_line_items.next().unwrap();

        RequestLine {
            method: method,
            request_target: request_target,
            http_version: http_version,
        }
    }

    pub fn get_method(&self) -> &Method{
        &self.method
    }

    pub fn get_path(&self) -> &str{
        &self.request_target.get_path()
    }

    pub fn get_param(&self, param: &str) -> Option<&String> {
        self.request_target.get_param(param)
    }
    
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
        let request_line = RequestLine::parse(request_line);
        //let (method, path, params, http_version) = Self::parse_request_line(request_line);
        
        let mut headers = Headers::new();
        loop {
            let mut line = String::new();
            buf_reader.read_line(&mut line).unwrap();
            if line == CRLF {
                break;
            }
            headers.parse_header_field(line);
            //print!("{}",line)
        }
        
        //print!("{}\n",headers.get("Authorization").unwrap_or(&"Auth header not found".to_string()));

        Request {
            request_line: request_line,
            headers: headers,
            body: "".to_string(),
        }
    }

    pub fn get_path(&self) -> &str{
        &self.request_line.get_path()
    }

    pub fn get_method(&self) -> &Method{
        &self.request_line.get_method()
    }

    pub fn get_header(&self, header: &str) -> Option<&String>{
        self.headers.get_header(header)
    }

    pub fn get_param(&self, param: &str) -> Option<&String> {
        self.request_line.get_param(param)
    }

    pub fn process(self) -> response::Response {
        let mut response = response::Response::new();
        let status = response.get_status_code();
        print!("status:{:?}\n", status);
        
        self.handle_authentication(&mut response);
        self.handle_custom_routes(&mut response);
        self.handle_everything_else(&mut response);

        response
    }

    fn handle_authentication(&self, response: &mut response::Response) {
        let auth_key = "Basic dXNlcjpwYXNz".to_string();
        let root_path = "root/";

        if response.get_status_code() == &response::StatusCode::UNKNOWN {
            match (self.get_method(), self.get_path()) {
                (Method::GET, "/admin") => {
                    if self.get_header("Authorization") == Some(&auth_key) {
                        let file_name = "admin.html";
                        response.set_status_code(response::StatusCode::STATUS200);
                        let contents = fs::read_to_string(root_path.to_string() + file_name).unwrap();
                        response.set_body(contents);
                    } else {
                        let file_name = "401.html";
                        response.add_header("WWW-Authenticate".to_string(), "Basic realm=\"WallyWorld\"".to_string());
                        response.set_status_code(response::StatusCode::STATUS401);
                        let contents = fs::read_to_string(root_path.to_string() + file_name).unwrap();
                        response.set_body(contents);
                    }
                },
                (_, _) => ()
            }
        } 

    }

    fn handle_custom_routes(&self, response: &mut response::Response) {
        let root_path = "root/";

        if response.get_status_code() == &response::StatusCode::UNKNOWN {
            match (self.get_method(), self.get_path()) {

                (self::Method::GET, "/sleep") => { 
                    let file_name = "hello.html";
                    thread::sleep(Duration::from_secs(
                        self.get_param("time").unwrap_or(&"5".to_string()).parse().unwrap_or(0)
                        //5
                    ));
                    let contents = fs::read_to_string(root_path.to_string() + file_name).unwrap();
                    response.set_status_code(response::StatusCode::STATUS200);
                    response.set_body(contents); 
                }
                (_, _) => ()
            };

        }
    }

    fn handle_everything_else(&self, response: &mut response::Response) {
        let root_path = "root/";

        if response.get_status_code() == &response::StatusCode::UNKNOWN {
            let (status_code, file_name) = 

            match (self.get_method(), self.get_path()) {

                (Method::GET, "/") => (response::StatusCode::STATUS200, "hello.html"),

                (self::Method::GET, _) => {
                    if fs::metadata(root_path.to_string() + self.get_path()).is_ok() {
                        (response::StatusCode::STATUS200, self.get_path() )
                    } else {
                        (response::StatusCode::STATUS404, "404.html")
                    }
                },

                (_, _) => (response::StatusCode::STATUS501,"501.html")
            };

            let contents = fs::read_to_string(root_path.to_string() + file_name).unwrap();
            response.set_status_code(status_code);
            response.set_body(contents); 

        }
    }

    
}