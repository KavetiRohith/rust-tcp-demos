use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use uuid::Uuid;

// constants
const SERVER_ADDRESS: &str = "127.0.0.1:8001";

#[tokio::main]
async fn main() {
    // starting
    println!("server starting {}", SERVER_ADDRESS);

    // bind
    let listener = TcpListener::bind(SERVER_ADDRESS).await.unwrap();

    // starting
    println!("server listening {}", SERVER_ADDRESS);

    // loop through incoming connections
    loop {
        // accept the connection
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    // create uuid
    let id = Uuid::new_v4();

    // create the buffer
    let mut buffer = [0; 1024];

    // read the stream
    let len = stream.read(&mut buffer).await.unwrap();

    // output the request
    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("{} - received: {}", id, message);

    // send out message
    let _ = stream.write_all(message.as_bytes()).await;
    println!("{} - sent: {}", id, message);
}
