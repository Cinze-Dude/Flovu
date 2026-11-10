use crate::{error::HttpStatus, request::Method, response::Response, router::Router};

#[allow(dead_code)]
pub trait BasicRfuncs {
    fn route_get(&mut self, path: &str, content: &str, body: &str);
    fn route_head(&mut self, path: &str, content: &str);
    fn route_post(&mut self, path: &str);
    fn route_put(&mut self, path: &str);
    fn route_delete(&mut self, path: &str);
    fn route_trace(&mut self, path: &str);
    fn route_options(&mut self, path: &str);
    fn route_patch(&mut self, path: &str);
    fn route_connect(&mut self, path: &str);
}

impl BasicRfuncs for Router {
    fn route_get(&mut self, path: &str, content: &str, body: &str) {
        let content = content.to_string();
        let body = body.as_bytes().to_vec();

        self.route(Method::Get, path, move |req| {
            Response::new(HttpStatus::Ok, req.version.clone())
                .header("content-type", &content)
                .body(body.clone())
        });
    }

    fn route_head(&mut self, path: &str, content: &str) {
        let content = content.to_string();

        self.route(Method::Head, path, move |req| {
            Response::new(HttpStatus::Ok, req.version.clone()).header("content-type", &content)
        });
    }

    fn route_post(&mut self, path: &str) {
        self.route(Method::Post, path, move |req| {
            let content = req.body.unwrap_or_default();

            Response::new(HttpStatus::Ok, req.version.clone())
                .header("content-type", "text/plain")
                .body(content)
        });
    }

    fn route_put(&mut self, path: &str) {
        self.route(Method::Put, path, |req| {
            let content = req.body.unwrap_or_default();

            Response::new(HttpStatus::Ok, req.version.clone())
                .header("content-type", "text/plain")
                .body(content)
        });
    }

    fn route_delete(&mut self, path: &str) {
        self.route(Method::Delete, path, |req| {
            Response::new(HttpStatus::Ok, req.version.clone()).body(Vec::new())
        });
    }

    fn route_trace(&mut self, path: &str) {
        self.route(Method::Trace, path, |req| {
            let content = req.body.unwrap_or_default();

            Response::new(HttpStatus::Ok, req.version.clone())
                .header("content-type", "message/http")
                .body(content)
        });
    }

    fn route_options(&mut self, path: &str) {
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
