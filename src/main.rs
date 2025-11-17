/*For this Coding Challenge your goal is to build an echo server that implements all of the Echo Protocol as defined in RFC 862.
    1.    In this step your goal is to build a simple server that will start-up, bind to all the local IP addresses, listen on port 7, and accept a TCP connection. 
          To complete this step simply have the server print out a log message to show a connection has been accepted and then have it shutdown. 
          Refer to the documentation for you programming language to find out how to write network programs using it.
    2.    In this step your goal is to extend your server to accept multiple concurrent connections. 
          This will require you to keep the main ‘thread’ of execution running and listening for incoming connections as well as spawning a new ‘thread’ 
          of execution to handle each client. https://codingchallenges.substack.com/p/coding-challenge-101-echo-server
    3.    In this step your goal is to read data from the client and write that data back to the client. That should continue until the client terminates the connection.
    4.    In this step your goal is to add a command line flag so your echo server can be started up using either TCP or UDP on port 7878.
*/
use std::{
    env::args, io::{self, BufReader, Error, LineWriter, prelude::*}, net::{TcpListener, TcpStream, UdpSocket}, thread
};
pub mod data_codec;
pub mod connection;

fn main() -> Result<(), Error> {
    //In this step your goal is to build a simple server that will start-up 
    // bind to all the local IP addresses 
    const SERVER_ADDR: &str = "localhost:7878";
    /*
    Step 4 requires a command line flag to specify udp (tcp is default).
    */
    // Store arguments
    let args: Vec<String> = args().collect();
    dbg!(&args);
    //let protocol;
    let flag_protocol = &args[1];
    if flag_protocol == &String::from("udp") {
        //protocol = connection::ConnProtocol::UDP((UdpSocket::bind(SERVER_ADDR)).unwrap());
        
        Ok(())
    }
    else { // TCP
        //protocol = connection::ConnProtocol::TCP((TcpListener::bind(SERVER_ADDR)).unwrap());
        let listener = TcpListener::bind(SERVER_ADDR)?;
        // listen on port 7878 
        //let port = listener.local_addr()?; // Only needed for the random port approach.
        println!("Listening on {}, access this port to end the program.", SERVER_ADDR);
        
        // accept a TCP connection
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| {
                        let peer_addr = stream.peer_addr().unwrap();
                        println!("Accepted connection from {:?}", &peer_addr); // This works because the possible error has been handled
                        let mut codec = data_codec::DataCodec::new(stream)
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
    
    


    // TCP
    
}