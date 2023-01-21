use super::{
    errors::{ClientError, HttpError, ServerError},
    headers::Headers,
    Body, HttpVerion, ParseError, CRLF,
};

use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

type _URI = String;
type Params = HashMap<String, String>;

pub struct Request {
    request_line: RequestLine,
    headers: Headers,
    _body: Body,
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
    pub fn parse(request_line: String) -> Result<RequestLine, ParseError> {
        let mut request_line_items = request_line.split_ascii_whitespace().map(|s| s.to_string());
        let method = match request_line_items.next() {
            Some(method_string) => match Method::parse(&method_string) {
                Ok(method) => method,
                Err(e) => return Err(e),
            },
            None => return Err(ParseError),
        };
        let request_target = match request_line_items.next() {
            Some(request_target_string) => match RequestTarget::parse(request_target_string) {
                Ok(request_target) => request_target,
                Err(e) => return Err(e),
            },
            None => return Err(ParseError),
        };
        let http_version = match request_line_items.next() {
            Some(http_version_string) => http_version_string,
            None => return Err(ParseError),
        };

        Ok(RequestLine {
            method: method,
            request_target: request_target,
            http_version: http_version,
        })
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
    pub fn parse(request_target: String) -> Result<RequestTarget, ParseError> {
        let (absolute_path, query_string) = match request_target.split_once('?') {
            Some((absolute_path, query_string)) => (absolute_path.to_string(), Some(query_string)),
            None => (request_target, None),
        };
        let query = match query_string {
            Some(query_string) => Some(Query::parse(query_string.to_string())),
            None => None,
        };
        Ok(RequestTarget {
            absolute_path: absolute_path,
            query: query,
        })
    }

    pub fn get_path(&self) -> &str {
        &self.absolute_path
    }

    pub fn get_param(&self, param: &str) -> Option<&String> {
        self.query.as_ref().and_then(|query| query.get_param(param))
    }
}

impl Method {
    pub fn parse(method_string: &str) -> Result<Method, ParseError> {
        match method_string {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "HEAD" => Ok(Method::HEAD),
            _ => Err(ParseError),
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
    pub fn load(request_stream: &mut TcpStream) -> Result<Request, HttpError> {
        let mut buf_reader = BufReader::new(request_stream);
        let mut request_line = String::new();
        match buf_reader.read_line(&mut request_line) {
            Ok(_) => (),
            Err(e) => return Err(HttpError::ServerError(ServerError::InternalServerError)),
        }
        let request_line = match RequestLine::parse(request_line) {
            Ok(request_line) => request_line,
            Err(e) => return Err(HttpError::ClientError(ClientError::BadRequest)),
        };

        let mut headers = Headers::new();
        loop {
            let mut line = String::new();
            match buf_reader.read_line(&mut line) {
                Ok(_) => (),
                Err(e) => return Err(HttpError::ServerError(ServerError::InternalServerError)),
            };
            if line == CRLF {
                break;
            }
            headers.parse_and_add_header_from(line);
        }

        Ok(Request {
            request_line: request_line,
            headers: headers,
            _body: "".to_string(),
        })
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
