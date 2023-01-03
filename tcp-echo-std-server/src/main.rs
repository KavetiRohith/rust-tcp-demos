use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::{thread, time::Duration};

// constants
const SERVER_ADDRESS: &str = "localhost:3010";
const DELAY: u64 = 100;

fn main() {
    // starting
    println!("server starting {}", SERVER_ADDRESS);

    // bind
    let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();

    // starting
    println!("server listening {}", SERVER_ADDRESS);

    // loop through incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // handle connection
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // create the buffer
    let mut buffer = [0; 1024];

    // read the stream
    let len = stream.read(&mut buffer).unwrap();

    // output the request
    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("received: {}", message);

    // put a delay in
    thread::sleep(Duration::from_millis(DELAY));

    // send out message
    let _ = stream.write_all(message.as_bytes());
    println!("sent: {}", message);
}
