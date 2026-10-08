use std::collections::HashMap;
use anyhow::bail;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PATCH,
    //the others...um, delete? and then?
}

impl Method {
    pub fn from_str(text: &str) -> anyhow::Result<Self>{
    let method = match text.to_uppercase().as_str() {
            "GET"=>Method::GET,
            "POST"=>Method::POST,
            "PATCH" =>Method::PATCH,
            _ => bail!("Unknown/unimplemented method: {}", text),
        };
    Ok(method)
    }
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
