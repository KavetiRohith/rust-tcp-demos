use std::io::prelude::*;
use std::net::TcpStream;

const ECHO_SERVER_ADDRESS: &str = "localhost:3010";

fn main() {
    println!("Connecting to {}", ECHO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS) {
        let local_addr = stream.local_addr().unwrap();

        println!(
            "Connected to echo server {}:{}",
            local_addr.ip(),
            local_addr.port()
        );

        // write message
        let message = "Hello World";
        let _ = stream.write(message.as_bytes()).unwrap();

        // flush stream to ensure the message is sent
        stream.flush().unwrap();

        println!("sent {}", message);

        // read the result
        let mut buffer = [0u8; 1024];
        let _len = stream.read(&mut buffer).unwrap();
        let message_recvd = String::from_utf8_lossy(&buffer);
        println!("Received {}", message_recvd)
    } else {
        println!("Unable to Connect to {}", ECHO_SERVER_ADDRESS);
    }
}
