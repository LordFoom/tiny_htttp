#[derive(Debug)]
use std::collections::HashMap;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PATCH,
    //the others...um, delete? and then?
}

#[derive(Debug)]
pub struct Request {
    method: Method,
    params: HashMap<String, String>,
    path: String,
    ///The full path as we received it
    full_path: String,
    version: String,
    host: String,
    user_agent: String,
    headers: HashMap<String, String>,
}
