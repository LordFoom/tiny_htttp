#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PATCH,
    //the others...um, delete? and then?
}

#[derive(Debug)]
struct Request {
    method: Method,
    params: HashMap<String, String>,
}
