use std::net::{IpAddr, SocketAddr};

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::{error::HttpError, request::parse_request, router::Router};

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

    pub async fn begin(&mut self) -> io::Result<()> {
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
        let socket = TcpListener::bind((ip, self.0.port)).await?;

        self.0.socket = Some(socket);

        Ok(())
    }

    pub async fn lex_request(
        client: &mut TcpStream,
        buffer: &mut Vec<u8>,
    ) -> Result<Option<Vec<u8>>, HttpError> {
        let boundary = loop {
            if let Some(pos) = buffer.windows(4).position(|w| w == b"\r\n\r\n") {
                break pos;
            }

            let mut chunk = [0u8; 1024];
            let n = client.read(&mut chunk).await?;

            if n == 0 {
                if buffer.is_empty() {
                    return Ok(None);
                }

                return Err(HttpError::IncompleteRequest);
            }

            buffer.extend_from_slice(&chunk[..n]);
        };

        let body_start = boundary + 4;

        let headers = std::str::from_utf8(&buffer[..boundary])?;

        let mut content_length = 0usize;

        for line in headers.lines().skip(1) {
            let Some((key, value)) = line.split_once(':') else {
                continue;
            };

            if key.trim().eq_ignore_ascii_case("content-length") {
                content_length = value
                    .trim()
                    .parse::<usize>()
                    .map_err(|_| HttpError::InvalidContentLength)?;

                break;
            }
        }

        let request_length = body_start + content_length;

        while buffer.len() < request_length {
            let mut chunk = [0u8; 1024];
            let n = client.read(&mut chunk).await?;

            if n == 0 {
                return Err(HttpError::IncompleteRequest);
            }

            buffer.extend_from_slice(&chunk[..n]);
        }

        let request = buffer[..request_length].to_vec();

        buffer.drain(..request_length);

        Ok(Some(request))
    }

    pub async fn handle_client(
        mut client: TcpStream,
        addr: SocketAddr,
        router: &Router,
    ) -> Result<(), HttpError> {
        let mut buffer = Vec::new();

        loop {
            let Some(raw) = Port::lex_request(&mut client, &mut buffer).await? else {
                break;
            };

            let request = parse_request(&raw)?;

            let closed = request
                .headers
                .get("connection")
                .is_some_and(|v| v.eq_ignore_ascii_case("close"));

            let response = router.handle(request);

            if closed {
                println!("Client Disconnected: {:?}", addr);
                break;
            }

            client.write_all(&response.serialize()).await?;
        }

        Ok(())
    }

    pub async fn run(&self, router: &Router) -> io::Result<()> {
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

        loop {
            let (client, addr) = socket.accept().await?;

            if nodelay {
                client.set_nodelay(true)?;
            }

            println!("Client connected: {:?}", addr);

            tokio::spawn(async move {
                if let Err(err) = Self::handle_client(client, addr, router).await {
                    eprintln!("Client error: {}", err);
                }
            });
        }
    }
}
