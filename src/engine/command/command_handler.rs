use crate::engine::command::Command;
use crate::models::User;

pub trait CommandHandler {
    fn can_respond_to(&self, command: &Command) -> bool;
    fn handle(&self, command: &Command, user: Option<&User>) -> Result<(), String>;
}
