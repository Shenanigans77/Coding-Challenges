use std::{
    array, io::{self, BufReader, LineWriter, prelude::*}, net::{TcpListener, TcpStream, UdpSocket}
};

pub enum Protocol {
    TCP,
    UDP,
}

#[derive(Debug)]
pub struct DataCodec {
    // Buffered reader and writers
    reader: BufReader<TcpStream>,
    writer: LineWriter<TcpStream>,
}

impl DataCodec {
    // Encapsulate a stream with read/write functionality
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let writer = LineWriter::new(stream.try_clone()?);
        let reader = BufReader::new(stream);
        Ok(Self { reader, writer})
    }

    // Write the given message, appending a new line.
    pub fn send_message(&mut self, message: &str) -> io::Result<()> {
        self.writer.write(&message.as_bytes())?;
        // This line flushes the buffer
        self.writer.write(&['\n' as u8])?;
        Ok(())
    }

    // Read a received message from TcpStream
    pub fn read_message(&mut self) -> io::Result<String> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        line.pop();
        Ok(line)
    }
}

#[derive(Debug)]
pub struct UdpConnector {
    receiver: [u8; 10], // This may need to be expanded. Could I use a vector?
    socket: UdpSocket,
}

impl UdpConnector {
    pub fn new(sock: UdpSocket) -> io::Result<Self> {
        let socket = sock;

        Ok(Self { receiver: [0; 10], socket })
    }

    // Read message from the UDP socket
    pub fn read_message(&mut self) -> io::Result<String> {
        match self.socket.recv(&mut self.receiver) {
            Ok(_received) => {
                let msg = match str::from_utf8(&self.receiver) {
                    Ok(v) => v,
                    Err(e) =>  panic!("Invalid UTF-8 sequence: {}", e),
                };
            },
            Err(e) => println!("recv function failed: {:?}", e),
        };
        //let msg = match String::from_utf8(&self.receiver) {
        //    Ok(v) => v,
        //    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        //};
        Ok(v)
    }
}

fn udp() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
        socket.connect("127.0.0.1:34254").expect("connect function failed");
        println!("ready");
        let mut buf = [0; 2048];
        match socket.recv(&mut buf) {
            Ok(_received) =>{
                let s = match str::from_utf8(&buf) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                println!("result: {}", s);
            },
            Err(e) => println!("recv function failed: {:?}", e),
        }
    } // the socket is closed here
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    // These tests should include a quick spin up of an incoming connection to simulate various cases
    const TEST_ADDR: &str = "localhost:7878";

    // This should take an argument for the test message and return a stream to work with.
    fn setup_test_requirements(addr: &str) -> io::Result<(TcpStream, TcpStream)> {
        let listener = TcpListener::bind(addr).unwrap();
        let test_client: TcpStream = TcpStream::connect(addr).unwrap();
        let test_stream = listener.accept().unwrap();
        let test_stream_result = test_stream.0;
        // Return test_client and test_stream
        return Ok((test_client, test_stream_result));
    }

    #[test]
    fn codec_read_message() {
        // General setup 
        let (client, server) = setup_test_requirements(TEST_ADDR).unwrap();
        
        /* Test Spec: The server should receive the same message the client sends. */
        // Test message
        let msg_sent: String = String::from("Frog");
        
        // Send a test message. This could be extended to test a series of messages - ToDo
        let mut client_codec = DataCodec::new(client).unwrap();
        client_codec.send_message(&msg_sent).unwrap();
        
        // Read messages sent to the server.
        let mut server_codec = DataCodec::new(server).unwrap();
        let msg_received = server_codec.read_message().unwrap();
        
        assert_eq!(msg_received,"Frog")
    }

    #[test]
    fn echo_message() {
        // General Setup
        let (client, server) = setup_test_requirements(TEST_ADDR).unwrap();
        let mut client_codec = DataCodec::new(client).unwrap();
        let mut server_codec = DataCodec::new(server).unwrap();

        /* Test Spec: A client sending a message to the server should receive the same message back. */ 
        let test_msg = String::from("Tomato");

        client_codec.send_message(&test_msg).unwrap();
        let received_msg = server_codec.read_message().unwrap();
        server_codec.send_message(&received_msg).unwrap();
        let echoed_msg = client_codec.read_message().unwrap();

        assert_eq!(echoed_msg,test_msg)
    }
}