use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Request {
    method: Method,
    path: String,
    query: Option<String>,
    version: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
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

fn parse_method(txt: String) -> Method {
    match txt.as_str() {
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

fn parse_method_line(txt: String) -> (Method, String, Option<String>, String) {
    let mut parts = txt.split_whitespace();

    let method = parse_method(parts.next().unwrap().to_string());
    let target = parts.next().unwrap();
    let version = parts.next().unwrap().to_string();

    let (path, query) = match target.split_once('?') {
        Some((path, query)) => (path.to_string(), Some(query.to_string())),
        None => (target.to_string(), None),
    };

    (method, path, query, version)
}

fn parse_header(txt: String) -> (String, String) {
    match txt.split_once(':') {
        Some((key, value)) => (key.trim().to_string(), value.trim().to_string()),
        None => (txt, String::new()),
    }
}

pub fn parse_request(txt: String) -> Request {
    let mut lines = txt.split("\r\n");

    let (method, path, query, version) = parse_method_line(lines.next().unwrap().to_string());

    let (header_part, body_part) = txt.split_once("\r\n\r\n").unwrap_or((&txt, ""));

    let mut headers = HashMap::new();

    for line in header_part.split("\r\n").skip(1) {
        if line.is_empty() {
            break;
        }

        let (hk, hv) = parse_header(line.to_string());
        headers.insert(hk, hv);
    }

    Request {
        method,
        path,
        query,
        version,
        headers,
        body: match body_part {
            "" => None,
            _ => Some(body_part.as_bytes().to_vec()),
        },
    }
}
