use crate::engine::db::establish_connection;
use crate::engine::{Command, CommandHandler};
use crate::models::User;
use crate::schema::users::dsl;

use diesel::prelude::*;

pub struct LoginHandler {}
impl CommandHandler for LoginHandler {
    fn can_respond_to(&self, command: &Command) -> bool {
        command.root == "login"
    }

    fn handle(&self, command: &Command, _user: Option<&User>) -> Result<(), String> {
        if command.args.get(0).is_some() && command.args.get(1).is_some() {
            let username = &command.args[0];
            let password = &command.args[1];

            let connection = establish_connection();
            match dsl::users
                .filter(dsl::username.eq(username))
                .first::<User>(&connection)
            {
                Ok(user) => match user.verify_password(&password) {
                    true => Ok(()),
                    false => Err("Invalid password".to_string()),
                },
                Err(_) => Err(format!(
                    "Could not find any user with username {}",
                    username
                )),
            }
        } else {
            return Err("Usage: `login <username> <password>`".to_string());
        }
    }
}
