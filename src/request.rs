use std::collections::HashMap;

use crate::{
    error::HttpError,
    validate::{valid_header_name, valid_header_value, valid_path, valid_query, valid_version},
};

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

    let method = parse_method(parts.next().ok_or(HttpError::InvalidRequestLine)?)?;
    let target = parts.next().ok_or(HttpError::InvalidRequestLine)?;
    let version = parts.next().ok_or(HttpError::InvalidRequestLine)?;

    if parts.next().is_some() {
        return Err(HttpError::InvalidRequestLine);
    }

    let (path, query) = match target.split_once('?') {
        Some((path, query)) => (path.to_string(), Some(query.to_string())),
        None => (target.to_string(), None),
    };

    Ok((method, path, query, version.to_string()))
}

fn parse_header(txt: &str) -> Result<(String, String), HttpError> {
    let (key, value) = txt
        .split_once(':')
        .ok_or_else(|| HttpError::InvalidHeader(txt.to_string(), String::new()))?;

    let key = key.trim().to_string();
    let value = value.trim().to_string();

    Ok((key, value))
}

pub fn parse_request(txt: String) -> Result<Request, HttpError> {
    let (header_part, body_part) = txt.split_once("\r\n\r\n").ok_or(HttpError::InvalidBody)?;

    let mut lines = header_part.split("\r\n");

    let (method, path, query, version) =
        parse_method_line(lines.next().ok_or(HttpError::InvalidRequestLine)?)?;

    let mut headers = HashMap::new();

    for line in lines {
        let (key, value) = parse_header(line)?;
        if !(valid_header_name(&key) && valid_header_value(&value)) {
            return Err(HttpError::InvalidHeader(key, value));
        }
        headers.insert(key, value);
    }

    if !valid_path(&path) {
        return Err(HttpError::InvalidPath(path));
    }

    if !valid_query(query.as_deref()) {
        return Err(HttpError::InvalidQuery(query.unwrap()));
    }

    if !valid_version(&version) {
        return Err(HttpError::InvalidVersion(version));
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
