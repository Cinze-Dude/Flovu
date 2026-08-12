use std::{
    io::{self, Read, Write},
    net::{IpAddr, TcpListener, TcpStream},
    thread,
};

use crate::{
    error::{HttpError, HttpStatus},
    handler::handle,
    request::parse_request,
};

pub struct PortState {
    port: u16,
    socket: Option<TcpListener>,
    opts: VecInitOption,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitOption {
    Binary,
    Ip(IpAddr),
    Nodelay,
    Packet(Packet),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VecInitOption(Vec<InitOption>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Packet {
    Raw,
}

impl Default for VecInitOption {
    fn default() -> Self {
        VecInitOption(vec![InitOption::Binary, InitOption::Packet(Packet::Raw)])
    }
}

pub struct Port(PortState);

impl Port {
    pub fn new(port: u16) -> Self {
        Self(PortState {
            port,
            socket: None,
            opts: VecInitOption::default(),
        })
    }

    pub fn begin(&mut self) -> io::Result<()> {
        let ip = self
            .0
            .opts
            .0
            .iter()
            .find_map(|option| {
                if let InitOption::Ip(inner_ip) = option {
                    Some(*inner_ip)
                } else {
                    None
                }
            })
            .unwrap_or(IpAddr::from([0, 0, 0, 0]));
        let socket = TcpListener::bind((ip, self.0.port))?;

        self.0.socket = Some(socket);

        Ok(())
    }

    pub fn lex_request(client: &mut TcpStream, buffer: &mut Vec<u8>) -> Result<Vec<u8>, HttpError> {
        loop {
            // Read until we have the headers.
            let boundary = loop {
                if let Some(pos) = buffer.windows(4).position(|w| w == b"\r\n\r\n") {
                    break pos;
                }

                let mut chunk = [0u8; 1024];
                let n = client.read(&mut chunk)?;

                if n == 0 {
                    return Err(HttpError::IncompleteRequest);
                }

                buffer.extend_from_slice(&chunk[..n]);
            };

            let body_start = boundary + 4;

            let headers = std::str::from_utf8(&buffer[..boundary]).map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "invalid UTF-8 in headers")
            })?;

            let content_length = headers
                .lines()
                .find_map(|line| {
                    let (key, value) = line.split_once(':')?;

                    if key.trim().eq_ignore_ascii_case("content-length") {
                        value.trim().parse::<usize>().ok()
                    } else {
                        None
                    }
                })
                .unwrap_or(0);

            let request_length = body_start + content_length;

            // We already have the complete request.
            if buffer.len() >= request_length {
                let request = buffer[..request_length].to_vec();

                // Keep anything belonging to the next request.
                buffer.drain(..request_length);

                return Ok(request);
            }

            // Headers are complete, but the body isn't.
            while buffer.len() < request_length {
                let mut chunk = [0u8; 1024];
                let n = client.read(&mut chunk)?;

                if n == 0 {
                    return Err(HttpError::ClientDisconnected);
                }

                buffer.extend_from_slice(&chunk[..n]);
            }

            let request = buffer[..request_length].to_vec();
            buffer.drain(..request_length);

            return Ok(request);
        }
    }

    pub fn handle_client(mut client: TcpStream) -> Result<(), HttpError> {
        let mut buffer = Vec::new();

        loop {
            let raw = Port::lex_request(&mut client, &mut buffer)?;

            let request = parse_request(&raw)?;

            let response = handle(request);

            client.write_all(&response.serialize())?;
        }
    }

    pub fn run(&self) -> io::Result<()> {
        let socket = self.0.socket.as_ref().ok_or(io::Error::new(
            io::ErrorKind::NotConnected,
            "socket not initialized",
        ))?;

        let nodelay = self
            .0
            .opts
            .0
            .iter()
            .any(|option| matches!(option, InitOption::Nodelay));

        for connection in socket.incoming() {
            let client = connection?;

            if nodelay {
                client.set_nodelay(true)?;
            }

            println!("Client connected: {:?}", client.peer_addr());

            thread::spawn(move || {
                if let Err(err) = Self::handle_client(client) {
                    eprintln!("Client error: {}", err);
                }
            });
        }

        Ok(())
    }
}
