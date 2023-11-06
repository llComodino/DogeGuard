// Handle events from the server
// Such as: new message, new user, user left, etc.

use crate::client::Client;
use crate::message::Message;
use crate::user::User;
use crate::utils::get_time;
use std::io::Write;
use std::sync::mpsc::Sender;

trait Event {
    fn handle(&self, client: &mut Client, sender: &Sender<Message>);
}

pub struct NewMessageEvent {
    pub message: Message,
}

impl Event for NewMessageEvent {
    fn handle(&self, client: &mut Client, sender: &Sender<Message>) {
        let message = &self.message;
        let user = &message.user;
        let time = get_time();
        let mut stdout = std::io::stdout();
        stdout
            .write_all(format!("\r{} {} > {}\n> ", time, user.name, message.content).as_bytes())
            .unwrap();
        stdout.flush().unwrap();
    }
}

pub struct NewUserEvent {
    pub user: User,
}

impl Event for NewUserEvent {
    fn handle(&self, client: &mut Client, sender: &Sender<Message>) {
        let user = &self.user;
        let time = get_time();
        let mut stdout = std::io::stdout();
        stdout
            .write_all(format!("\r{} {} joined the chat\n> ", time, user.name).as_bytes())
            .unwrap();
        stdout.flush().unwrap();
    }
}

pub struct UserLeftEvent {
    pub user: User,
}

impl Event for UserLeftEvent {
    fn handle(&self, client: &mut Client, sender: &Sender<Message>) {
        let user = &self.user;
        let time = get_time();
        let mut stdout = std::io::stdout();
        stdout
            .write_all(format!("\r{} {} left the chat\n> ", time, user.name).as_bytes())
            .unwrap();
        stdout.flush().unwrap();
    }
}

pub struct UserListEvent {
    pub users: Vec<User>,
}

impl Event for UserListEvent {
    fn handle(&self, client: &mut Client, sender: &Sender<Message>) {
        let users = &self.users;
        let time = get_time();
        let mut stdout = std::io::stdout();
        stdout
            .write_all(format!("\r{} Users: ", time).as_bytes())
            .unwrap();
        for user in users {
            stdout.write_all(format!("{} ", user.name).as_bytes()).unwrap();
        }
        stdout.write_all(b"\n> ").unwrap();
        stdout.flush().unwrap();
    }
}

pub struct ErrorEvent {
    pub message: String,
}

impl Event for ErrorEvent {
    fn handle(&self, client: &mut Client, sender: &Sender<Message>) {
        let message = &self.message;
        let time = get_time();
        let mut stdout = std::io::stdout();
        stdout
            .write_all(format!("\r{} Error: {}\n> ", time, message).as_bytes())
            .unwrap();
        stdout.flush().unwrap();
    }
}
