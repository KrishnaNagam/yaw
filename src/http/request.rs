use super::{headers::Headers, Body, CRLF, HttpVerion};

use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

type URI = String;
type Params = HashMap<String, String>;

pub struct Request {
    request_line: RequestLine,
    headers: Headers,
    body: Body,
}

pub struct RequestLine {
    pub method: Method,
    pub request_target: RequestTarget,
    pub http_version: HttpVerion,
}

pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
}

pub struct RequestTarget {
    absolute_path: String,
    query: Option<Query>,
}

pub struct Query {
    params: Params,
}

impl RequestLine {
    pub fn parse(request_line: String) -> RequestLine {
        let mut request_line_items = request_line.split_ascii_whitespace().map(|s| s.to_string());
        let method = Method::parse(request_line_items.next().unwrap().as_str());
        let request_target = request_line_items.next().unwrap();
        let request_target = RequestTarget::parse(request_target);
        let http_version = request_line_items.next().unwrap();

        RequestLine {
            method: method,
            request_target: request_target,
            http_version: http_version,
        }
    }

    pub fn get_method(&self) -> &Method {
        &self.method
    }

    pub fn get_path(&self) -> &str {
        &self.request_target.get_path()
    }

    pub fn get_param(&self, param: &str) -> Option<&String> {
        self.request_target.get_param(param)
    }
}

impl RequestTarget {
    pub fn parse(request_target: String) -> RequestTarget {
        let request_target_items = request_target.split_once('?');
        let (absolute_path, query_string) = if request_target_items.is_none() {
            (request_target, None)
        } else {
            (
                request_target_items.unwrap().0.to_string(),
                Some(request_target_items.unwrap().1),
            )
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

    pub fn get_path(&self) -> &str {
        &self.absolute_path
    }

    pub fn get_param(&self, param: &str) -> Option<&String> {
        self.query.as_ref().and_then(|query| query.get_param(param))
    }
}

impl Method {
    pub fn parse(method_string: &str) -> Method {
        match method_string {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "HEAD" => Method::HEAD,
            _ => panic!("Unkown Request Method"),
        }
    }
}

impl Query {
    pub fn parse(query: String) -> Query {
        let params_strings_list = query.split('&');
        let mut params: HashMap<String, String> = HashMap::new();
        for param_string in params_strings_list {
            let (param_key, param_value) = param_string
                .split_once('=')
                .unwrap_or_else(|| (param_string, ""));
            params.insert(param_key.to_string(), param_value.to_string());
        }
        Query { params: params }
    }

    pub fn get_param(&self, param: &str) -> Option<&String> {
        self.params.get(param)
    }
}

impl Request {
    pub fn load(request_stream: &mut TcpStream) -> Request {
        let mut buf_reader = BufReader::new(request_stream);
        let mut request_line = String::new();
        buf_reader.read_line(&mut request_line).unwrap();
        let request_line = RequestLine::parse(request_line);

        let mut headers = Headers::new();
        loop {
            let mut line = String::new();
            buf_reader.read_line(&mut line).unwrap();
            if line == CRLF {
                break;
            }
            headers.parse_and_add_header_from(line);
        }

        Request {
            request_line: request_line,
            headers: headers,
            body: "".to_string(),
        }
    }

    pub fn get_path(&self) -> &str {
        &self.request_line.get_path()
    }

    pub fn get_method(&self) -> &Method {
        &self.request_line.get_method()
    }

    pub fn get_header(&self, header: &str) -> Option<&String> {
        self.headers.get_header(header)
    }

    pub fn get_param(&self, param: &str) -> Option<&String> {
        self.request_line.get_param(param)
    }
}
