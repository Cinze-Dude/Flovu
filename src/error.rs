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
        let reason = match self {
            HttpStatus::Ok => "Ok",
            HttpStatus::BadRequest => "Bad Request",
            HttpStatus::Unauthorized => "Unauthorized",
            HttpStatus::Forbidden => "Forbidden",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::MethodNotAllowed => "Method Not Allowed",
            HttpStatus::RequestTimeout => "Request Timeout",
            HttpStatus::Conflict => "Conflict",
            HttpStatus::ContentTooLarge => "Content Too Large",
            HttpStatus::URITooLong => "URI Too Long",
            HttpStatus::UnsupportedMediaType => "Unsupported Media Type",
            HttpStatus::Teapot => "I'm a teapot",
            HttpStatus::TooManyRequests => "Too Many Requests",

            HttpStatus::InternalServerError => "Internal Server Error",
            HttpStatus::NotImplemented => "Not Implemented",
            HttpStatus::BadGateway => "Bad Gateway",
            HttpStatus::ServiceUnavailable => "Service Unavailable",
            HttpStatus::GatewayTimeout => "Gateway Timeout",
        };
        let code = self.code();
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
