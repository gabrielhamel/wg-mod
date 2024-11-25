use crate::utils::Env;
use std::ffi::OsStr;
use std::process::{Command, Output};
use std::{io, result};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to execute command")]
    ExecutionError(#[from] io::Error),
}

type Result<T> = result::Result<T, Error>;

pub fn command<S: AsRef<OsStr>>(
    command: S, args: Vec<&str>, env: Vec<Env>,
) -> Result<Output> {
    let mut command = Command::new(command);
    command.args(args);

    env.iter().for_each(|env| {
        command.env(&env.key, &env.value);
    });

    let out = command.output().map_err(Error::ExecutionError)?;

    Ok(out)
}
