use std::fmt;
use std::fmt::write;
use std::io;

use crate::parser::Method;

#[derive(Debug)]
pub enum HttpError {
    Io(io::Error),

    Http(u16),

    InvalidRequestLine,
    InvalidMethod(Method),
    InvalidPath(String),
    InvalidQuery(String),
    InvalidVersion(String),

    InvalidHeader(String),

    InvalidContentLength,
    MissingContentLength,
    MissingHeaderTerminator,
    InvalidBody,
}

impl From<io::Error> for HttpError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<u16> for HttpError {
    fn from(status: u16) -> Self {
        Self::Http(status)
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {}", err),

            Self::Http(status) => {
                let reason = match *status {
                    400 => "Bad Request",
                    401 => "Unauthorized",
                    403 => "Forbidden",
                    404 => "Not Found",
                    405 => "Method Not Allowed",
                    408 => "Request Timeout",
                    409 => "Conflict",
                    413 => "Content Too Large",
                    414 => "URI Too Long",
                    415 => "Unsupported Media Type",
                    418 => "I'm a teapot",
                    429 => "Too Many Requests",

                    500 => "Internal Server Error",
                    501 => "Not Implemented",
                    502 => "Bad Gateway",
                    503 => "Service Unavailable",
                    504 => "Gateway Timeout",

                    _ => "Unknown HTTP Status",
                };

                write!(f, "{} {}", status, reason)
            }

            Self::InvalidRequestLine => write!(f, "Invalid HTTP request line"),

            Self::MissingHeaderTerminator => write!(f, "Missing header terminator"),

            Self::InvalidMethod(method) => write!(f, "Invalid HTTP method: {:#?}", method),

            Self::InvalidPath(path) => write!(f, "Invalid HTTP path: {}", path),

            Self::InvalidQuery(query) => write!(f, "Invalid HTTP query: {}", query),

            Self::InvalidVersion(version) => write!(f, "Invalid HTTP version: {}", version),

            Self::InvalidHeader(header) => write!(f, "Invalid HTTP header: {}", header),

            Self::InvalidContentLength => write!(f, "Invalid Content-Length"),

            Self::MissingContentLength => write!(f, "Missing Content-Length"),

            Self::InvalidBody => write!(f, "Invalid HTTP body"),
        }
    }
}

impl std::error::Error for HttpError {}
