use crate::utils::command::CommandError::CommandFailed;
use crate::utils::Env;
use std::process::{Command, Output};

#[derive(thiserror::Error, Debug)]
pub enum CommandError {
    #[error("Failed to execute command")]
    CommandFailed,
}

pub fn command(
    command: &str, args: Vec<&str>, env: Vec<Env>,
) -> Result<Output, CommandError> {
    let mut command = Command::new(command);
    command.args(args);

    env.iter().for_each(|env| {
        command.env(&env.key, &env.value);
    });

    Ok(command.output().map_err(|_| CommandFailed)?)
}
