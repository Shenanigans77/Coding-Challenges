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
                    //let thread_stream = stream;
                    loop {
                        //let handled_connection = handle_connection(thread_stream);
                        let handled_connection = handle_connection(stream);
                        match handled_connection {
                            Ok(handled_connection) => handled_connection,
                            Err(e) => {
                                eprintln!("Session Error: {e:?}");
                                break; // This assumes a closed connection would error this loop
                            }
                        }
                    }
                    // This should run as a while loop to keep the connection open until the client closes it.
                    
                    println!("Connection from {:?} closed.", &peer_addr);
                });
            }
            Err(e) => { 
                /* connection failed */
                eprintln!("Connection Error: {e:?}");
            }
        }
    }
    //let (mut tcp_stream, addr) = listener.accept()?; //block until requested
    //println!("Connection received! {:?} is sending data.", addr);
    
    //I don't care about handling input yet.
    //let mut input = String::new();
    //let _ = tcp_stream.read_to_string(&mut input)?;
    //println!("{:?} says {}", addr, input);
    Ok(())
}

fn handle_connection(stream: TcpStream) -> io::Result<()> {
    let mut codec = DataCodec::new(stream)?;

    let message: String = codec.read_message()?;

    codec.send_message(&message)?;
    Ok(())
}

/* 
    This does return a single echo, then a Ncat: broken pipe
*/
//fn handle_connection(stream: TcpStream) -> io::Result<()> {
//    let buf_reader = BufReader::new(&stream);
//    let mut writer = io::LineWriter::new(stream.try_clone().unwrap());
//    // echo input
//
//    /* 
//    Does this have to process fully before I move on to the next step?
//    I think what I want for now is for this to roll the response back to the sender.
//     */
//    let http_request: Vec<_> = buf_reader
//        .lines()
//        .map(|result| result.unwrap())
//        .take_while(|line| !line.is_empty())
//        .collect();
//    
//    println!("Request: {http_request:#?}");
//    //stream.write(http_request);
//    //let mut buf_writer = BufWriter::new(stream);
//    
//    for i in &http_request{
//        //This write doesn't appear to write
//        writer.write(&i.as_bytes())?; // ToDo - figure out this write
//        
//        //println!("{:?}", i);
//    }
//    let _ = writer.flush()?;
//    //println!("Wrote {http_request:#?} to {}", stream.peer_addr().unwrap());
//    //writer.flush().unwrap(); // the let _ ignores the error result which could be thrown
//    Ok(())
//}