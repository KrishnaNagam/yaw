pub mod request;
pub mod response;
pub mod auth;
mod headers;

pub type HttpVerion = String;
pub type Body = String;

const CRLF: &str = "\r\n";

