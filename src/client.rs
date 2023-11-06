// Client able to connect/communicate to a TCP server
// Features:
// - Client authentication
// - Client registration
// - Client login
// - Client logout
// - Client message sending
// - Client message receiving
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::message::Message;
use crate::message::MessageHandler;

pub enum ClientStatus {
    Connected,
    Authenticated,
    Disconnected,
}

// Can't implement Copy trait
// !!!!!!! DO NOT TRY FOR THE THIRD TIME !!!!!!!!!
pub struct Client {
    pub stream: TcpStream,
    pub username: String,
    pub password: String,
    pub status: ClientStatus,
    pub message: Message,
}

impl Client {
    // Creates a new client
    pub fn new(client_stream: TcpStream) -> Client {

        Client {
            stream: client_stream,
            username: String::new(),
            password: String::new(),
            status: ClientStatus::Connected,
            message: Message::new(),
        }
    }

    // Reads data from the client
    pub fn read(&mut self) {
        let mut buffer = [0; 1024];
        let bytes_read = self.stream.read(&mut buffer).unwrap();

        if bytes_read == 0 {
            self.status = ClientStatus::Disconnected;
            return;
        }

        self.message = Message::new();
        self.message.from(&mut self.stream);
        
        println!("Message: {:?}", self.message);
    }

    // Writes data to the client
    pub fn set_username(&mut self) {
        // Promt the user for a password 
        self.stream.write(b"Enter username: ").unwrap();

        // Gets username from client
        let mut buffer = [0; 1024];
        
        self.username = self.stream.read(&mut buffer).unwrap().to_string();
    }

    // Writes data to the client
    pub fn set_password(&mut self) {
        // Promt the user for a password 
        self.stream.write(b"Enter password: ").unwrap();

        // Gets password from client
        let mut buffer = [0; 1024];
        
        self.password = self.stream.read(&mut buffer).unwrap().to_string(); 
    }

    // Authenticates the client
    pub fn authenticate(&mut self) {
        // Promt the user for a password 
        self.stream.write(b"Enter password: ").unwrap();

        // Gets password from client
        let mut buffer = [0; 1024];
        
        self.password = self.stream.read(&mut buffer).unwrap().to_string();
    }

    // Closes the client
    pub fn close(&mut self) {
        self.stream.shutdown(std::net::Shutdown::Both).unwrap();
    }

}
