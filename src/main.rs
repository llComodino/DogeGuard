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
    let mut clients = HashMap::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut client = Client::new(stream);
                let mut client_id = 0;

                loop {
                    match client.status {
                        ClientStatus::Connected => {
                            client.read();
                            let _message = client.message.clone();

                            /*match message.state {
                                MessageState::Full => {
                                    /*match message {
                                        Message::auth { username, password } => {
                                            println!("Authenticating user: {}", username);
                                            println!("Password: {}", password);
                                            client.status = ClientStatus::Authenticated;
                                        }
                                        Message::command { command } => {
                                            println!("Command: {}", command);
                                        }
                                        Message::data { data } => {
                                            println!("Data: {}", data);
                                        }
                                        Message::error { error } => {
                                            println!("Error: {}", error);
                                        }
                                        Message::exit => {
                                            println!("Exiting...");
                                            client.status = ClientStatus::Disconnected;
                                        }
                                        _ => {}
                                    }*/
                                }
                                _ => {}
                            }*/
                        }
                        ClientStatus::Authenticated => {
                            client_id += 1;
                            clients.insert(client_id, client);
                            break;
                        }
                        ClientStatus::Disconnected => {
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
