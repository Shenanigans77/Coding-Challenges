//For this Coding Challenge your goal is to build an echo server that implements all of the Echo Protocol as defined in RFC 862.
//1.    In this step your goal is to build a simple server that will start-up, bind to all the local IP addresses, listen on port 7, and accept a TCP connection. 
//      To complete this step simply have the server print out a log message to show a connection has been accepted and then have it shutdown. 
//      Refer to the documentation for you programming language to find out how to write network programs using it.
//2.    In this step your goal is to extend your server to accept multiple concurrent connections. 
//      This will require you to keep the main ‘thread’ of execution running and listening for incoming connections as well as spawning a new ‘thread’ 
//      of execution to handle each client. https://codingchallenges.substack.com/p/coding-challenge-101-echo-server
use std::{
    io::{self, BufReader, Error, LineWriter, prelude::*}, net::{TcpListener, TcpStream}, thread
};

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

fn main() -> Result<(), Error> {
    //In this step your goal is to build a simple server that will start-up 
    // bind to all the local IP addresses 
    let listener = TcpListener::bind("localhost:7878")?;
    // listen on port 7878 
    let port = listener.local_addr()?; // Only needed for the random port approach.
    println!("Listening on {}, access this port to end the program.", port);
    
    // accept a TCP connection
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    let peer_addr = stream.peer_addr().unwrap();
                    println!("Accepted connection from {:?}", &peer_addr); // This works because the possible error has been handled
                    let mut codec = DataCodec::new(stream)
                        .expect("Failed to create DataCodec.");

                    loop {
                        let message = match codec.read_message() {
                            Ok(msg) if !msg.is_empty() => msg,
                            _ => break, // Break on error or EOF. Also breaks on newlines - I'm not sure if this is expected. 
                        };

                        codec.send_message(&message).expect("Send error");
                    }
                    println!("Connection from {:?} closed.", &peer_addr);
                });
            }
            Err(e) => { 
                /* connection failed */
                eprintln!("Connection Error: {e:?}");
            }
        }
    }
    Ok(())
}


//fn handle_connection(stream: TcpStream) -> io::Result<()> {
//    let mut codec = DataCodec::new(stream)?;
//    
//    let message: String = codec.read_message()?;
//
//    codec.send_message(&message)?;
//    Ok(())
//}

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