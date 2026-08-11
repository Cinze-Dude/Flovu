use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Request {
    method: Method,
    path: String,
    query: Option<String>,
    version: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum Method {
    Connect,
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Trace,
    Unknown(String),
}

mod RequestParser {
    use crate::parser::Method;

    fn parse_method(bytes: &[u8]) -> Method {
        let txt = String::from_utf8_lossy(bytes);
        match txt.as_ref() {
            "CONNECT" => Method::Connect,
            "DELETE" => Method::Delete,
            "GET" => Method::Get,
            "HEAD" => Method::Head,
            "OPTIONS" => Method::Options,
            "PATCH" => Method::Patch,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "TRACE" => Method::Trace,
            &_ => Method::Unknown(txt.to_string()),
        }
    }

    fn parse_method_line(bytes: &[u8]) -> (Method, String, String) {
        let txt = String::from_utf8_lossy(bytes);
        let mpq: Vec<&str> = txt.as_ref().split(" ").collect();
        let pq: Vec<&str> = mpq[1].split("?").collect();
        (
            parse_method(mpq[0].as_bytes()),
            pq[0].to_string(),
            pq[1].to_string(),
        )
    }

    fn parse_header(bytes: &[u8]) -> (String, String) {
        let txt = String::from_utf8_lossy(bytes);
        let ti: Vec<&str> = txt.as_ref().split(": ").collect();
        (ti[0].to_string(), ti[1].to_string())
    }
}
