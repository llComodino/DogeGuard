// Multithreaded rocket HTTP server implementing:
// - Client authentication
// - Client registration
// - Client login
// - Client logout
// - Client message sending
// - Client message receiving

// Rocket imports
// Macros
#[macro_use] extern crate rocket;
use rocket::State;
use rocket::response::content;
/*
use rocket::http::RawStr;
use rocket::http::hyper::StatusCode::InternalServerError;
use rocket::response::NamedFile;
use rocket::response::Redirect;
use rocket::response::status;
use rocket::response::status::NotFound;
use rocket::response::status::Unauthorized;
use rocket::response::status::BadRequest;
use rocket::response::status::Created;
use rocket::response::status::Accepted;
use rocket::response::status::Forbidden;

use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Sender as OneshotSender;
use tokio::sync::oneshot::Receiver as OneshotReceiver;
*/

#[get("/")]
fn index() -> content::Html<&'static str> {
    content::Html(include_str!("./www/index.html"))
}

#[launch]
fn rocket() -> _ {
    rocket::build().manage(State::new())
}

#[tokio::main]
async fn main() {
    // Rocket server configuration
    rocket::ignite()
        .mount("/", routes![index])
        .launch()
        .await
        .expect("Failed to launch Rocket server");
}
