use std::{collections::HashMap, io};

use crate::error::HttpError;

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

fn parse_method(txt: &str) -> Result<Method, HttpError> {
    match txt {
        "CONNECT" => Ok(Method::Connect),
        "DELETE" => Ok(Method::Delete),
        "GET" => Ok(Method::Get),
        "HEAD" => Ok(Method::Head),
        "OPTIONS" => Ok(Method::Options),
        "PATCH" => Ok(Method::Patch),
        "POST" => Ok(Method::Post),
        "PUT" => Ok(Method::Put),
        "TRACE" => Ok(Method::Trace),
        &_ => Err(HttpError::InvalidMethod(Method::Unknown(txt.to_string()))),
    }
}

fn parse_method_line(txt: &str) -> Result<(Method, String, Option<String>, String), HttpError> {
    let mut parts = txt.split_whitespace();

    let method = parse_method(parts.next().unwrap())?;
    let target = parts.next().unwrap();
    let version = parts.next().unwrap().to_string();

    let (path, query) = match target.split_once('?') {
        Some((path, query)) => (path.to_string(), Some(query.to_string())),
        None => (target.to_string(), None),
    };

    Ok((method, path, query, version))
}

fn parse_header(txt: &str) -> Result<(String, String), HttpError> {
    txt.split_once(':')
        .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
        .ok_or_else(|| HttpError::InvalidHeader(txt.to_string()))
}

pub fn parse_request(txt: String) -> Result<Request, HttpError> {
    let (header_part, body_part) = txt.split_once("\r\n\r\n").ok_or(HttpError::InvalidBody)?;

    let mut lines = header_part.split("\r\n");

    let (method, path, query, version) =
        parse_method_line(lines.next().ok_or(HttpError::InvalidRequestLine)?)?;

    let mut headers = HashMap::new();

    for line in lines {
        let (key, value) = parse_header(line)?;
        headers.insert(key, value);
    }

    Ok(Request {
        method,
        path,
        query,
        version,
        headers,
        body: match body_part {
            "" => None,
            _ => Some(body_part.as_bytes().to_vec()),
        },
    })
}
