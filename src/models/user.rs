use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_encrypted: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_encrypted: &'a str,
}

pub fn create_user<'a>(conn: &PgConnection, username: &'a str, password: &'a str) -> User {
    let new_user = NewUser {
        username,
        password_encrypted: password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}
