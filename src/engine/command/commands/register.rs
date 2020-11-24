use crate::engine::db::establish_connection;
use crate::engine::{Command, CommandHandler};
use crate::models::User;

pub struct RegisterHandler {}
impl CommandHandler for RegisterHandler {
    fn can_respond_to(&self, command: &Command) -> bool {
        command.root == "register"
    }

    fn handle(&self, command: &Command, _user: Option<&User>) -> Result<(), String> {
        if command.args.get(0).is_some() && command.args.get(1).is_some() {
            let username = &command.args[0];
            let password = &command.args[1];

            let connection = establish_connection();
            match User::create(&connection, username, password) {
                Ok(_user) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        } else {
            return Err("Usage: `register <username> <password>`".to_string());
        }
    }
}
