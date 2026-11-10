use std::fs;

use crate::{error::HttpStatus, request::Method, response::Response, router::Router};

#[allow(dead_code)]
pub trait FileRfuncs {
    fn route_html(&mut self, path: &str, file_path: &str);
    fn route_image(&mut self, file_path: &str);
}

impl FileRfuncs for Router {
    fn route_html(&mut self, path: &str, file_path: &str) {
        let body = fs::read_to_string(file_path)
            .unwrap_or_default()
            .as_bytes()
            .to_vec();

        self.route(Method::Get, path, move |req| {
            Response::new(HttpStatus::Ok, req.version.clone())
                .header("content-type", "text/html")
                .body(body.clone())
        });
    }

    fn route_image(&mut self, file_path: &str) {
        let body = fs::read(file_path).unwrap_or_default();

        self.route(Method::Get, "/favicon.ico", move |req| {
            Response::new(HttpStatus::Ok, req.version.clone())
                .header("content-type", "image/*")
                .body(body.clone())
        });
    }
}
