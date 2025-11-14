use std::{
    io::{self, Error}, net::{TcpListener, UdpSocket}
};

#[derive(Debug)]
pub enum ConnProtocol {
    TCP(TcpListener),
    UDP(UdpSocket),
}

#[derive(Debug)]
pub struct ConnectionConfig {
    protocol: ConnProtocol,
    address: String,
}

impl ConnectionConfig {
    pub fn new(proto: ConnProtocol, addr: String) -> io::Result<Self> {
        let protocol = proto;
        let address = addr;
        Ok(Self { protocol, address })
    } 
}