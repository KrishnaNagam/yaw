pub mod server;
mod request;
mod response;
mod auth;
mod headers;

use std::{collections::HashMap,fmt};

type Body = String;

const CRLF: &str = "\r\n";

