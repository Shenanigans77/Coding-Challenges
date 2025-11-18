use std::{
    io::{self, Error}, net::{TcpListener, UdpSocket}
};

//use crate::data_codec;

#[derive(Debug)]
pub enum ConnProtocol {
    TCP(TcpListener),
    UDP(UdpSocket),
}

#[derive(Debug)]
pub struct ConnectionConfig {
    connection: ConnProtocol,
}

impl ConnectionConfig {
    pub fn new(proto: ConnProtocol) -> io::Result<Self> {
        let connection = proto;
        Ok(Self { connection })
    } 
}