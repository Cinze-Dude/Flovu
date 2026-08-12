use std::fmt;
use std::io;
use std::str::Utf8Error;

use crate::request::Method;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HttpStatus {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    RequestTimeout,
    Conflict,
    ContentTooLarge,
    URITooLong,
    UnsupportedMediaType,
    Teapot,
    TooManyRequests,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    Ok,
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (code, reason) = match self {
            HttpStatus::Ok => (200, "Ok"),
            HttpStatus::BadRequest => (400, "Bad Request"),
            HttpStatus::Unauthorized => (401, "Unauthorized"),
            HttpStatus::Forbidden => (403, "Forbidden"),
            HttpStatus::NotFound => (404, "Not Found"),
            HttpStatus::MethodNotAllowed => (405, "Method Not Allowed"),
            HttpStatus::RequestTimeout => (408, "Request Timeout"),
            HttpStatus::Conflict => (409, "Conflict"),
            HttpStatus::ContentTooLarge => (413, "Content Too Large"),
            HttpStatus::URITooLong => (414, "URI Too Long"),
            HttpStatus::UnsupportedMediaType => (415, "Unsupported Media Type"),
            HttpStatus::Teapot => (418, "I'm a teapot"),
            HttpStatus::TooManyRequests => (429, "Too Many Requests"),

            HttpStatus::InternalServerError => (500, "Internal Server Error"),
            HttpStatus::NotImplemented => (501, "Not Implemented"),
            HttpStatus::BadGateway => (502, "Bad Gateway"),
            HttpStatus::ServiceUnavailable => (503, "Service Unavailable"),
            HttpStatus::GatewayTimeout => (504, "Gateway Timeout"),
        };
        write!(f, "{} {}", code, reason)
    }
}

#[derive(Debug)]
pub enum HttpError {
    Io(io::Error),
    Utf8(Utf8Error),
    Http(HttpStatus),

    IncompleteRequest,
    ClientDisconnected,

    InvalidRequestLine,
    InvalidMethod(Method),
    InvalidPath(String),
    InvalidQuery(String),
    InvalidVersion(String),

    InvalidHeader(String, String),

    InvalidContentLength,
    MissingContentLength,
    MissingHeaderTerminator,
    InvalidBody,
}

impl HttpStatus {
    pub fn code(&self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::MethodNotAllowed => 405,
            Self::RequestTimeout => 408,
            Self::Conflict => 409,
            Self::ContentTooLarge => 413,
            Self::URITooLong => 414,
            Self::UnsupportedMediaType => 415,
            Self::Teapot => 418,
            Self::TooManyRequests => 429,

            Self::InternalServerError => 500,
            Self::NotImplemented => 501,
            Self::BadGateway => 502,
            Self::ServiceUnavailable => 503,
            Self::GatewayTimeout => 504,
        }
    }
}

impl From<io::Error> for HttpError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<Utf8Error> for HttpError {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8(value)
    }
}

impl From<HttpStatus> for HttpError {
    fn from(status: HttpStatus) -> Self {
        Self::Http(status)
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {}", err),

            Self::Http(status) => write!(f, "{status}"),

            Self::Utf8(err) => write!(f, "{err}"),

            Self::ClientDisconnected => write!(f, "Client Disconnected while sending a message"),

            Self::IncompleteRequest => write!(f, "Incomplete Request"),

            Self::InvalidRequestLine => write!(f, "Invalid HTTP request line"),

            Self::MissingHeaderTerminator => write!(f, "Missing header terminator"),

            Self::InvalidMethod(method) => write!(f, "Invalid HTTP method: {:#?}", method),

            Self::InvalidPath(path) => write!(f, "Invalid HTTP path: {}", path),

            Self::InvalidQuery(query) => write!(f, "Invalid HTTP query: {}", query),

            Self::InvalidVersion(version) => write!(f, "Invalid HTTP version: {}", version),

            Self::InvalidHeader(header, val) => {
                write!(f, "Invalid HTTP header: \"{}: {}\"", header, val)
            }

            Self::InvalidContentLength => write!(f, "Invalid Content-Length"),

            Self::MissingContentLength => write!(f, "Missing Content-Length"),

            Self::InvalidBody => write!(f, "Invalid HTTP body"),
        }
    }
}

impl std::error::Error for HttpError {}
