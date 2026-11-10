use std::collections::HashMap;

use crate::{
    error::HttpError,
    validate::{valid_header_name, valid_header_value, valid_path, valid_query, valid_version},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    query: Option<String>,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

pub fn parse_request(raw: &[u8]) -> Result<Request, HttpError> {
    let boundary = raw
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .ok_or(HttpError::MissingHeaderTerminator)?;

    let header_part = str::from_utf8(&raw[..boundary])?;
    let body_part = &raw[boundary + 4..];

    let mut lines = header_part.split("\r\n");

    let (method, path, query, version) =
        parse_method_line(lines.next().ok_or(HttpError::InvalidRequestLine)?)?;

    let mut headers = HashMap::new();

    for line in lines {
        let (key, value) = parse_header(line)?;
        if !(valid_header_name(&key) && valid_header_value(&value)) {
            return Err(HttpError::InvalidHeader(key, value));
        }
        headers.insert(key.to_ascii_lowercase(), value);
    }

    let content_length = headers
        .get("content-length")
        .map(|value| {
            value
                .parse::<usize>()
                .map_err(|_| HttpError::InvalidContentLength)
        })
        .transpose()?
        .unwrap_or(0usize);

    let body = if content_length == 0 {
        None
    } else {
        if body_part.len() < content_length {
            return Err(HttpError::InvalidContentLength);
        }

        Some(body_part[..content_length].to_vec())
    };

    if !valid_path(&path) {
        return Err(HttpError::InvalidPath(path));
    }

    if let Some(query) = query.as_deref() {
        if !valid_query(Some(query)) {
            return Err(HttpError::InvalidQuery(query.to_string()));
        }
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
        body,
    })
}
