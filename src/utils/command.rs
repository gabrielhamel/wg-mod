use crate::utils::Env;
use std::io;
use std::process::{Command, Output};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to execute command")]
    ExecutionError(#[from] io::Error),
}

pub fn command(
    command: &str, args: Vec<&str>, env: Vec<Env>,
) -> Result<Output, Error> {
    let mut command = Command::new(command);
    command.args(args);

    env.iter().for_each(|env| {
        command.env(&env.key, &env.value);
    });

    let out = command.output().map_err(Error::ExecutionError)?;

    Ok(out)
}
