#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

use dotenv::dotenv;

mod command;
mod db;
mod models;
mod schema;
mod server;

use server::Server;

const BINDING: &str = ":::8787";

#[tokio::main]
async fn main() {
    dotenv().ok();

    let result = Server::start(BINDING).await;
    if let Err(e) = result {
        eprintln!("{:?}", e);
    }
}
