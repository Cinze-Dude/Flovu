use crate::{
    error::HttpStatus,
    request::{Method, Request},
    response::Response,
};

pub fn handle(request: Request) -> Response {
    match request.method {
        Method::Get => handle_get(&request),
        Method::Head => handle_head(&request),
        Method::Post => handle_post(&request),
        Method::Put => handle_put(&request),
        Method::Delete => handle_delete(&request),
        Method::Options => handle_options(&request),
        Method::Trace => handle_trace(&request),
        Method::Connect => handle_connect(&request),
        Method::Patch => handle_patch(&request),

        Method::Unknown(_) => Response::new(HttpStatus::BadRequest, request.version.clone())
            .body(b"Bad Request".to_vec()),
    }
}

pub fn handle_get(request: &Request) -> Response {
    let version = request.version.clone();

    match request.path.as_str() {
        "/" => Response::new(HttpStatus::Ok, version)
            .header("content-type", "text/html")
            .body(b"<h1>Hello Flovu!</h1>".to_vec()),

        "/anas" => Response::new(HttpStatus::Ok, version)
            .header("content-type", "text/html")
            .body(b"<h1>Hello Anas!</h1>".to_vec()),

        "/adam" => Response::new(HttpStatus::Ok, version)
            .header("content-type", "text/html")
            .body(b"<h1>Hello Adam!</h1>".to_vec()),

        "/goat/omar" => Response::new(HttpStatus::Ok, version)
            .header("content-type", "text/html")
            .body(b"<h1>Hello Omar!</h1>".to_vec()),

        "/favicon.ico" => match std::fs::read("cinze.png") {
            Ok(body) => Response::new(HttpStatus::Ok, version)
                .header("content-type", "image/png")
                .body(body),

            Err(_) => Response::new(HttpStatus::InternalServerError, version)
                .body(b"Internal Server Error".to_vec()),
        },

        _ => Response::new(HttpStatus::NotFound, version).body(b"Not Found".to_vec()),
    }
}

pub fn handle_head(request: &Request) -> Response {
    let mut response = handle_get(request);

    response.remove_body();

    response
}

pub fn handle_post(request: &Request) -> Response {
    let version = request.version.clone();

    match request.path.as_str() {
        "/echo" => Response::new(HttpStatus::Ok, version)
            .header("content-type", "text/plain")
            .body(request.body.clone().unwrap_or_default()),

        _ => Response::new(HttpStatus::NotFound, version).body(b"Not Found".to_vec()),
    }
}

pub fn handle_put(request: &Request) -> Response {
    let version = request.version.clone();

    match request.path.as_str() {
        "/echo" => Response::new(HttpStatus::Ok, version)
            .header("content-type", "text/plain")
            .body(request.body.clone().unwrap_or_default()),

        _ => Response::new(HttpStatus::NotFound, version).body(b"Not Found".to_vec()),
    }
}

pub fn handle_delete(request: &Request) -> Response {
    Response::new(HttpStatus::Ok, request.version.clone()).body(b"Deleted".to_vec())
}

pub fn handle_options(request: &Request) -> Response {
    Response::new(HttpStatus::Ok, request.version.clone())
        .header(
            "allow",
            "GET, HEAD, POST, PUT, PATCH, DELETE, OPTIONS, TRACE",
        )
        .body(Vec::new())
}

pub fn handle_trace(request: &Request) -> Response {
    let version = request.version.clone();

    let mut body = format!(
        "{:?} {} {}\r\n",
        request.method, request.path, request.version
    );

    for (key, value) in &request.headers {
        body.push_str(&format!("{}: {}\r\n", key, value));
    }

    body.push_str("\r\n");

    if let Some(data) = &request.body {
        body.push_str(&String::from_utf8_lossy(data));
    }

    Response::new(HttpStatus::Ok, version)
        .header("content-type", "message/http")
        .body(body.into_bytes())
}

pub fn handle_patch(request: &Request) -> Response {
    let version = request.version.clone();

    match request.path.as_str() {
        "/echo" => Response::new(HttpStatus::Ok, version)
            .header("content-type", "text/plain")
            .body(request.body.clone().unwrap_or_default()),

        _ => Response::new(HttpStatus::NotFound, version).body(b"Not Found".to_vec()),
    }
}

pub fn handle_connect(request: &Request) -> Response {
    Response::new(HttpStatus::NotImplemented, request.version.clone())
        .body(b"CONNECT not implemented".to_vec())
}
