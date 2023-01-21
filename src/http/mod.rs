pub mod request;
pub mod response;
pub mod auth;
pub mod errors;
mod headers;

pub type HttpVerion = String;
pub type Body = String;

const CRLF: &str = "\r\n";

pub struct ParseError;