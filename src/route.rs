use std::collections::HashMap;
use std::net::TcpStream;
use anyhow::{Result, bail};
use tracing::{debug, info, warn, instrument};

#[derive(Debug)]
pub enum Action {
    GET,
    POST,
    PATCH,
}

const RAW_REQ_URL: &str = "raw_request";
const ACTION: &str = "action";
const PATH: &str = "path";
const VERSION: &str = "version";
const HOST: &str = "host";
const USER_AGENT: &str = "user-agent";
const ACCEPT: &str = "accept";

const METHODS: &[&str] = &["GET", "POST", "PATCH"];

///EG
///GET /hello HTTP/1.1
///Host: localhost:8080
///User-Agent: curl/8.21.0
///Accept: */*
#[instrument]
pub fn parse_request(req: &str) -> anyhow::Result<HashMap<String, String>> {
    let mut parsed_request = HashMap::<String, String>::new();
    let mut first = true;
    for line in req.split('\n'){
        let lower_line = line.to_lowercase();
        debug!(%lower_line, "current request line being parsed");
        if first {
            debug!("We are in the first loop");
            //now we split it up some more
            let tokens = line.split_whitespace().collect::<Vec<&str>>();
            if !tokens.len() >= 3 {
                bail!("Expected at least 3 tokens");
            }

            let allowd_action = METHODS.iter().any(|a| a==&tokens[0].to_uppercase());
            if !allowd_action {
                bail!("Unsupported HTTP method: {}", tokens[0]);
            } 

            parsed_request.insert(RAW_REQ_URL.to_string(), line.to_string());
            parsed_request.insert(ACTION.to_string(), tokens[0].to_string());
            parsed_request.insert(PATH.to_string(), tokens[1].to_string());
            parsed_request.insert(VERSION.to_string(), tokens[2].to_string());
            first = false;
            continue;
        }
        match lower_line{
            l if l.starts_with(HOST) => {
                let tokens: Vec<&str> = line.split_whitespace().collect();
                parsed_request.insert(HOST.to_string(), tokens[1].to_string());
            }
            l if l.starts_with(USER_AGENT) => {
                let tokens: Vec<&str> = line.split_whitespace().collect();
                parsed_request.insert(HOST.to_string(), tokens[1].to_string());
            }
            l if l.starts_with(ACCEPT) => {}
            _ =>  {warn!("unrecognized request part");}
        }
    }
    Ok(parsed_request)
}

pub fn route_post(parsed_request: &HashMap<String, String>, stream: &mut TcpStream)->anyhow::Result<()>{
    Ok(())
}

pub fn route_request(parsed_request: &HashMap<String, String>, stream: &mut TcpStream) -> anyhow::Result<()> {

    let action = &parsed_request[ACTION];
    match action.as_str() {
        "GET" => route_get(parsed_request, stream)?,
        "POST" => route_post(parsed_request, stream)?,
        _ => {
            println!("Nothing else unimplemented!()")
        }
    }
    Ok(())
}

pub fn route_get(parsed_request: &HashMap<String, String>, stream: &mut TcpStream) -> Result<()> {
    let action_str = &parsed_request[ACTION];
    match action_str {
       val if val.starts_with("/hello") => {
            println!("We have hello!");
        },

        _ => {
            println!("Nothing else unimplemented!()")
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request_line() {
        let req = "\
GET /hello HTTP/1.1
Host: localhost:8080
User-Agent: curl/8.21.0
Accept: */*";

        let parsed = parse_request(req);

        assert_eq!(parsed.get(RAW_REQ_URL), Some(&"GET /hello HTTP/1.1".to_string()));
        assert_eq!(parsed.get(ACTION), Some(&"GET".to_string()));
        assert_eq!(parsed.get(PATH), Some(&"/hello".to_string()));
        assert_eq!(parsed.get(VERSION), Some(&"HTTP/1.1".to_string()));
    }

    #[test]
    fn parses_different_verb_and_path() {
        let req = "\
POST /users/123 HTTP/1.1
Host: localhost:8080";

        let parsed = parse_request(req);

        assert_eq!(parsed.get(ACTION), Some(&"POST".to_string()));
        assert_eq!(parsed.get(PATH), Some(&"/users/123".to_string()));
        assert_eq!(parsed.get(VERSION), Some(&"HTTP/1.1".to_string()));
    }

    #[test]
    fn action_url_is_case_insensitive() {
        let req = "GET /hello HTTP/1.1";

        let parsed = parse_request(req);

        assert_eq!(parsed.get(ACTION), Some(&"GET".to_string()));
        assert_eq!(parsed.get(PATH), Some(&"/hello".to_string()));
    }

    #[test]
    fn request_without_action_url_returns_empty_map() {
        let req = "\
Host: localhost:8080
Accept: */*";

        let parsed = parse_request(req);

        assert!(parsed.is_empty());
    }
}
