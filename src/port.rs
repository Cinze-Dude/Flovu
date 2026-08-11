use std::{
    io::{self, Read, Write},
    net::{IpAddr, TcpListener, TcpStream},
    thread,
};

use crate::parser;

pub struct PortState {
    port: u16,
    socket: Option<TcpListener>,
    opts: VecInitOption,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitOption {
    Binary,
    Active,
    Ip(IpAddr),
    Nodelay,
    Packet(Packet),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VecInitOption(Vec<InitOption>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Packet {
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

    pub fn lex_request(mut client: &TcpStream, mut buffer: Vec<u8>) -> io::Result<String> {
        loop {
            let mut chunk = [0u8; 1024];
            let n = client.read(&mut chunk)?;

            if n == 0 {
                break;
            }

            buffer.extend_from_slice(&chunk[..n]);

            if buffer.windows(4).any(|window| window == b"\r\n\r\n") {
                break;
            }
        }
        String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn handle_client(mut client: TcpStream) -> io::Result<()> {
        let buffer = Vec::new();
        let request = Port::lex_request(&client, buffer)?;
        println!("{}", request);
        println!("{:#?}", parser::parse_request(request));

        let response = b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHello";

        client.write_all(response)?;

        Ok(())
    }

    pub fn run(&self) -> io::Result<()> {
        for connection in self
            .0
            .socket
            .as_ref()
            .ok_or(io::Error::new(
                io::ErrorKind::NotConnected,
                "socket not initialized",
            ))?
            .incoming()
        {
            let client = connection?;
            println!("Client connected: {:?}", client.peer_addr());

            thread::spawn(move || Self::handle_client(client));
        }

        Ok(())
    }
}
