// Multithreaded TCP server implementing:
// - Client authentication
// - Client registration
// - Client login
// - Client logout
// - Client message sending
// - Client message receiving

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

mod client;
mod message;
// mod server;

use client::{Client, ClientStatus};
use message::{Message, MessageState};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mut clients : HashMap<Client, i32> = HashMap::new();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        let mut message = Message::new();

        stream.read(&mut buffer).unwrap();
        message.parse(&buffer);

        let mut client = Client::new(stream.try_clone().unwrap());
        let mut client = Arc::new(Mutex::new(client));

        let clients = Arc::new(Mutex::new(clients));

        thread::spawn(move || {
            let mut buffer = [0; 1024];
            let mut message = Message::new();

            loop {
 
                let mut client = client.lock().unwrap();

                client.stream.read(&mut buffer).unwrap();
                message.parse(&buffer);

            }

        });

    }

}
