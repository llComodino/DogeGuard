/*
 * This server waits for a connection on port 8080
 * then spawns a thread to handle the connection
 *
 * The thread is a client handler that reads the request
 * The client can:
 * 1. Search for a value in the database
 * 2. Add a value to the database
 * 3. Delete a value from the database
 * 4. List all values in the database
 * 5. Exit
 *
 * The database stores the following:
 * 1. Website name
 * 2. username
 * 3. password
 * 4. url
 *
 * The database is hosted on a remote server
 * The client can connect to the server and perform the above actions
 * The server will return the result of the action 
 * The client will display the result to the user 
 * The client will then prompt the user for another action 
 * The client will continue to prompt the user until the user exits
 * The client will then close the connection to the server
 * The client will then exit 
 * 
 */

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::str;
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufRead;
use std::io::ErrorKind;
use std::io::Error;
use std::path::Path;
use std::fs::OpenOptions;

// This function handles the client 
fn handle_client(mut stream: TcpStream) {
    // Create a buffer to store the data from the client
    let mut data = [0 as u8; 50]; // using 50 byte buffer

    // Read the data from the client
    match stream.read(&mut data) {
        Ok(size) => {
            // Convert the data to a string
            let s = match str::from_utf8(&data[0..size]) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            // Print the data to the console
            println!("{}", s);

            // Parse the data
            let mut split = s.split_whitespace();
            let command = split.next().unwrap();
            let arg = split.next().unwrap_or("");
        }
    }
}

fn main() {
    // Create a listener on port 8080
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // Wait for connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a thread to handle the connection
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }

    // close the socket server
    drop(listener);
}
