// Handle sending and receiving messages

// Path: message.rs

use std::io::{Read, Write};
use std::net::TcpStream;
//use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq)]
pub enum MessageState {
    Empty,
    Full,
    Overflow,
    Underflow,
}

impl Clone for MessageState {
    fn clone(&self) -> Self {
        match *self {
            MessageState::Empty => MessageState::Empty,
            MessageState::Full => MessageState::Full,
            MessageState::Overflow => MessageState::Overflow,
            MessageState::Underflow => MessageState::Underflow,
        }
    }
}

impl Copy for MessageState {}

#[derive(Debug, PartialEq)]
pub enum MessageType {
    Auth,
    Form,
    Command,
    Data,
    Error,
}

impl Clone for MessageType {
    fn clone(&self) -> Self {
        match *self {
            MessageType::Auth => MessageType::Auth,
            MessageType::Form => MessageType::Form,
            MessageType::Command => MessageType::Command,
            MessageType::Data => MessageType::Data,
            MessageType::Error => MessageType::Error,
        }
    }
}

impl Copy for MessageType {}

#[derive(Debug, PartialEq)]
pub struct Message {
    pub buffer: [u8; 1024],
    pub length: usize,
    pub index: usize,
    pub start: usize,
    pub end: usize,
    pub state: MessageState,
    pub mtype: MessageType,
}

pub trait MessageHandler {
    fn auth(&mut self, stream: &mut TcpStream) -> Message;

    fn from(&mut self, stream: &mut TcpStream) -> Message;

    fn command(&mut self, stream: &mut TcpStream) -> Message;

    fn data(&mut self, stream: &mut TcpStream) -> Message;

    fn error(&mut self, stream: &mut TcpStream) -> Message;
}

impl MessageHandler for Message {

    fn auth(&mut self, stream: &mut TcpStream) -> Message {
        self.data(stream)
    }

    fn from(&mut self, stream: &mut TcpStream) -> Message {
        self.data(stream)
    }

    fn data(&mut self, stream: &mut TcpStream) -> Message {
        self.data(stream)
    }
        
    fn error(&mut self, stream: &mut TcpStream) -> Message {
        self.data(stream)
    }

    fn command(&mut self, stream: &mut TcpStream) -> Message {
        self.data(stream)
    }
}

impl Clone for Message {
    fn clone(&self) -> Message {
        Message {
            buffer: self.buffer,
            length: self.length,
            index: self.index,
            start: self.start,
            end: self.end,
            state: self.state,
            mtype: self.mtype,
        }
    }
}

impl Message {
    pub fn new() -> Message {
        Message {
            buffer: [0; 1024],
            length: 0,
            index: 0,
            start: 0,
            end: 0,
            state: MessageState::Empty,
            mtype: MessageType::Data,
        }
    }

    pub fn data(&mut self, stream: &mut TcpStream) -> Message {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(n) => {
                self.buffer = buffer;
                self.length = n;
                self.index = 0;
                self.start = 0;
                self.end = n;
                self.state = MessageState::Full;
            }
            Err(e) => {
                println!("Error reading from stream: {}", e);
            }
        }
        self.clone()
    }

    pub fn read(&mut self, stream: &mut TcpStream) {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(n) => {
                self.buffer = buffer;
                self.length = n;
                self.index = 0;
                self.start = 0;
                self.end = n;
                self.state = MessageState::Full;
            }
            Err(e) => {
                println!("Error reading from stream: {}", e);
            }
        }
    }

    pub fn write(&mut self, stream: &mut TcpStream) {
        match stream.write(&self.buffer) {
            Ok(n) => {
                self.index = 0;
                self.start = 0;
                self.end = 0;
                self.state = MessageState::Empty;
            }
            Err(e) => {
                println!("Error writing to stream: {}", e);
            }
        }
    }

    pub fn push(&mut self, byte: u8) {
        if self.state == MessageState::Full {
            self.state = MessageState::Overflow;
        } else {
            self.buffer[self.end] = byte;
            self.end += 1;
            self.length += 1;
            if self.end == 1024 {
                self.end = 0;
            }
            if self.end == self.start {
                self.state = MessageState::Full;
            }
        }
    }

    pub fn parse(&mut self, buffer: &[u8]) {
        for byte in buffer {
            self.push(*byte);
        }
    }

    pub fn pop(&mut self) -> u8 {
        if self.state == MessageState::Empty {
            self.state = MessageState::Underflow;
            0
        } else {
            let byte = self.buffer[self.start];
            self.start += 1;
            self.length -= 1;
            if self.start == 1024 {
                self.start = 0;
            }
            if self.start == self.end {
                self.state = MessageState::Empty;
            }
            byte
        }
    }

    pub fn peek(&mut self) -> u8 {
        if self.state == MessageState::Empty {
            self.state = MessageState::Underflow;
            0
        } else {
            self.buffer[self.start]
        }
    }

    pub fn exit(&mut self) {
        self.state = MessageState::Empty;
    }
}
