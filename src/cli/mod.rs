mod command;
mod commands;
pub mod errors;

use self::{command::RunnableCommand, commands::new::NewCommand};
use errors::Error;

pub fn run() -> Result<(), Error> {
    let matches = command::command().get_matches();

    match matches.subcommand() {
        | Some(("new", args)) => NewCommand::run(args),
        | Some((_, _)) => Err(Error::CommandNotImplemented),
        | None => Err(Error::NoCommandProvided),
    }
}
