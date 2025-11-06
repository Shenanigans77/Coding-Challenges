//For this Coding Challenge your goal is to build an echo server that implements all of the Echo Protocol as defined in RFC 862.
//1.    In this step your goal is to build a simple server that will start-up, bind to all the local IP addresses, listen on port 7, and accept a TCP connection. 
//      To complete this step simply have the server print out a log message to show a connection has been accepted and then have it shutdown. 
//      Refer to the documentation for you programming language to find out how to write network programs using it.
//2.    In this step your goal is to extend your server to accept multiple concurrent connections. 
//      This will require you to keep the main ‘thread’ of execution running and listening for incoming connections as well as spawning a new ‘thread’ 
//      of execution to handle each client. https://codingchallenges.substack.com/p/coding-challenge-101-echo-server
use std::net::TcpListener;
use std::io::{Read, Error};

fn main() -> Result<(), Error> {
    //In this step your goal is to build a simple server that will start-up 
    // bind to all the local IP addresses 
    let listener = TcpListener::bind("localhost:0")?;
    // listen on port 7 
    let port = listener.local_addr()?;
    println!("Listening on {}, access this port to end the program.", port);
    
    // accept a TCP connection
    let (mut tcp_stream, addr) = listener.accept()?; //block until requested
    println!("Connection received! {:?} is sending data.", addr);
    let mut input = String::new();
    let _ = tcp_stream.read_to_string(&mut input)?;
    println!("{:?} says {}", addr, input);
    Ok(())
}
