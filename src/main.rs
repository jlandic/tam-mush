#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

use dotenv::dotenv;
use std::env;

mod engine;
mod models;
mod schema;

use crate::engine::Server;

const BINDING: &str = ":::8787";

#[tokio::main]
async fn main() {
    dotenv().ok();

    let result = Server::start(&env::var("BINDING").unwrap_or(BINDING.to_string())).await;
    if let Err(e) = result {
        eprintln!("{:?}", e);
    }
}
