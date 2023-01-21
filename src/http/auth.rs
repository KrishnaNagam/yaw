use base64::{engine::general_purpose, Engine};

use super::request::Request;



pub fn basic_auth_validate(request: &Request, username:  &String, password: &String) -> bool {
    let encoded_creds: String = general_purpose::STANDARD.encode(username.to_string() + ":" + password);
    let auth_key = "Basic ".to_string() + encoded_creds.as_str();
    request.get_header("Authorization") == Some(&auth_key) 
}