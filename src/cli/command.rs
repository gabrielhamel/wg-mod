use crate::cli::commands::build::BuildCommand;
use crate::cli::commands::new::NewCommand;
use clap::{ArgMatches, Command};

#[derive(thiserror::Error, Debug)]
pub enum CommandError {
    #[error("This command isn't implemented")]
    CommandNotImplemented,

    #[error("No command provided, refer to the help section")]
    NoCommandProvided,

    #[error("Error occurred during the command execution\n{0}")]
    CommandExecutionError(String),
}

pub trait RunnableCommand {
    fn command() -> Command;

    fn run(args: &ArgMatches) -> Result<(), CommandError>;
}

pub fn command() -> Command {
    Command::new("wg-mod")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Gabriel Hamel <gabriel.hamel.pro@gmail.com>")
        .about("Provides cli tools for Wargaming games modding")
        .subcommand_required(true)
        .subcommand(NewCommand::command())
        .subcommand(BuildCommand::command())
}
