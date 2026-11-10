use std::collections::HashMap;

use crate::error::{HttpError, HttpStatus};

pub struct Response {
    pub(crate) version: String,
    status: HttpStatus,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl Response {
    pub fn new(status: HttpStatus, version: String) -> Self {
        Self {
            version,
            status,
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        if key.to_ascii_lowercase() == "content-length" {
            return self;
        }

        self.headers
            .insert(key.to_string().to_ascii_lowercase(), value.to_string());
        self
    }

    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.headers
            .insert("content-length".to_string(), body.len().to_string());

        self.body = Some(body);

        self
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut response = format!(
            "{} {}\r\n{}",
            self.version,
            HttpError::Http(self.status),
            self.headers
                .iter()
                .map(|(key, value)| format!("{}: {}\r\n", key, value))
                .collect::<String>(),
        );

        response.push_str("\r\n");

        let mut bytes = response.into_bytes();

        if let Some(body) = &self.body {
            bytes.extend_from_slice(body);
        }

        bytes
    }

    pub fn remove_body(&mut self) {
        self.body = None;
    }
}
