// Multithreaded TCP server implementing:
// - Client authentication
// - Client registration
// - Client login
// - Client logout
// - Client message sending
// - Client message receiving
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/* 
mod client;
mod message;
mod server;

use client::{Client, ClientStatus};
use message::{Message, MessageState};
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    loop {
        
        let (mut socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {

            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {

                // Ask for client authentication
                if let Err(e) = socket.write_all(b"Please authenticate yourself\n").await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }

                // Read the data
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                println!("received {} bytes from socket", n);
                println!("data: {:?}", &buf[0..n]);
                let content = String::from_utf8_lossy(&buf[0..n]);
                println!("content: {:?}", content);

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
