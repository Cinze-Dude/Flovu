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

    pub fn route_get(&mut self, path: &str, content: &str, body: &str) {
        let content = content.to_string();
        let body = body.as_bytes().to_vec();

        self.route(Method::Get, path, move |req| {
            Response::new(HttpStatus::Ok, req.version.clone())
                .header("content-type", &content)
                .body(body.clone())
        });
    }

    pub fn route_head(&mut self, path: &str, content: &str) {
        let content = content.to_string();

        self.route(Method::Head, path, move |req| {
            Response::new(HttpStatus::Ok, req.version.clone()).header("content-type", &content)
        });
    }

    pub fn route_post(&mut self, path: &str) {
        self.route(Method::Post, path, move |req| {
            let content = req.body.unwrap_or_default();

            Response::new(HttpStatus::Ok, req.version.clone())
                .header("content-type", "text/plain")
                .body(content)
        });
    }

    pub fn route_put(&mut self, path: &str) {
        self.route(Method::Put, path, |req| {
            let content = req.body.unwrap_or_default();

            Response::new(HttpStatus::Ok, req.version.clone())
                .header("content-type", "text/plain")
                .body(content)
        });
    }

    pub fn route_delete(&mut self, path: &str) {
        self.route(Method::Delete, path, |req| {
            Response::new(HttpStatus::Ok, req.version.clone()).body(Vec::new())
        });
    }

    pub fn route_trace(&mut self, path: &str) {
        self.route(Method::Trace, path, |req| {
            let content = req.body.unwrap_or_default();

            Response::new(HttpStatus::Ok, req.version.clone())
                .header("content-type", "message/http")
                .body(content)
        });
    }

    pub fn route_options(&mut self, path: &str) {
        self.route(Method::Options, path, |req| {
            Response::new(HttpStatus::Forbidden, req.version.clone())
                .header("allow", "GET, POST, PUT, DELETE, TRACE, OPTIONS")
                .body(Vec::new())
        });
    }

    fn route_patch(&mut self, path: &str) {
        self.route(Method::Patch, path, |req| {
            let content = req.body.unwrap_or_default();

            Response::new(HttpStatus::Ok, req.version.clone())
                .header("content-type", "text/plain")
                .body(content)
        });
    }

    fn route_connect(&mut self, path: &str) {
        self.route(Method::Connect, path, |req| {
            Response::new(HttpStatus::Ok, req.version.clone()).body(Vec::new())
        });
    }
}
