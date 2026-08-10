use std::{
    io::{self, Read, Write},
    net::{IpAddr, TcpListener, TcpStream},
    thread,
};

pub struct PortState {
    port: u16,
    socket: TcpListener,
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
    pub fn begin(port: u16, opts: VecInitOption) -> io::Result<Self> {
        let ip = opts
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
        let socket = TcpListener::bind((ip, port))?;

        Ok(Self(PortState { port, socket, opts }))
    }

    pub fn handle_client(mut client: TcpStream) -> io::Result<()> {
        let mut buffer = Vec::new();
        client.read_to_end(&mut buffer)?;

        let request =
            String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        println!("{}", request);

        let response = b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHello";

        client.write_all(response)?;

        Ok(())
    }

    pub fn run(&self) -> io::Result<()> {
        for connection in self.0.socket.incoming() {
            let client = connection?;
            println!("Client connected: {:?}", client.peer_addr());

            thread::spawn(move || Self::handle_client(client));
        }

        Ok(())
    }
}
