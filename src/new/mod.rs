use clap::{Arg, ArgMatches, Command};

use self::{command::collect_command, error::NewError};

mod command;
mod error;

use std::fs::create_dir;

pub fn command() -> Command {
    Command::new("new")
        .about("Create a new mod project")
        .long_about("Create a new mod project at <path>")
        .arg(Arg::new("path").default_value("."))
}

pub fn new(args: &ArgMatches) -> Result<(), NewError> {
    let new_command = collect_command(args)?;

    // Create mod directory
    create_dir(new_command.path).map_err(NewError::UnableToCreateDirectory)?;

    Ok(())
}

pub fn execute(args: &ArgMatches) {
    if let Err(e) = new(args) {
        eprintln!("Error {}", e)
    }
}
