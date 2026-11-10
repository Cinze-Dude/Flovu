use std::collections::HashMap;

use crate::{
    error::HttpStatus,
    request::{Method, Request},
    response::Response,
};

type Handler = Box<dyn Fn(Request) -> Response + Send + Sync + 'static>;

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

    pub fn handle(&self, request: Request) -> Response {
        let errr = Response::new(HttpStatus::NotFound, request.version.clone());
        self.routes
            .get(&(request.method.clone(), request.path.clone()))
            .map_or_else(|| errr, |handler| handler(request))
    }

    pub fn route<F>(&mut self, method: Method, path: &str, handler: F)
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.routes
            .insert((method, path.to_string()), Box::new(handler));
    }
}
