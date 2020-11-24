use bcrypt::{hash, verify, DEFAULT_COST};
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

impl User {
    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_encrypted).unwrap_or(false)
    }

    pub fn create(conn: &PgConnection, username: &str, password: &str) -> Result<Self, String> {
        let password_encrypted = &hash_password(password)?;
        let new_user = NewUser {
            username,
            password_encrypted,
        };

        match diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
        {
            Ok(user) => Ok(user),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_encrypted: &'a str,
}

fn hash_password(password: &str) -> Result<String, String> {
    match hash(password, DEFAULT_COST) {
        Ok(encrypted) => Ok(encrypted),
        Err(e) => Err(e.to_string()),
    }
}
