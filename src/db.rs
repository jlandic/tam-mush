use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL env var is not set.");
    PgConnection::establish(&url).expect(&format!(
        "Error connecting to the database. Check URL and credentials."
    ))
}
