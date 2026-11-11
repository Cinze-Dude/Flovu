use std::collections::HashMap;

use crate::{
    error::HttpStatus,
    request::{Method, Request},
    response::Response,
};

type Handler = Box<dyn Fn(Request) -> Response + Send + Sync + 'static>;

pub trait IntoMethod {
    fn into_method(self) -> Method;
}

impl IntoMethod for Method {
    fn into_method(self) -> Method {
        self
    }
}

impl IntoMethod for &str {
    fn into_method(self) -> Method {
        match self {
            "CONNECT" => Method::Connect,
            "DELETE" => Method::Delete,
            "GET" => Method::Get,
            "HEAD" => Method::Head,
            "OPTIONS" => Method::Options,
            "PATCH" => Method::Patch,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "TRACE" => Method::Trace,
            s => Method::Unknown(s.to_string()),
        }
    }
}

pub struct Router {
    routes: HashMap<(Method, String /* path */), Handler>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn arc(self) -> std::sync::Arc<Self> {
        std::sync::Arc::new(self)
    }

    pub fn handle(&self, request: &Request) -> Response {
        let errr = Response::new(HttpStatus::NotFound, request.version.clone());
        self.routes
            .get(&(request.method.clone(), request.path.clone()))
            .map_or_else(|| errr, |handler| handler(request.clone()))
    }

    pub fn route<F, M: IntoMethod>(&mut self, method: M, path: &str, handler: F)
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.routes
            .insert((method.into_method(), path.to_string()), Box::new(handler));
    }
}
