use crate::cli::commands::new::NewCommand;
use clap::{ArgMatches, Command};

#[derive(thiserror::Error, Debug)]
pub enum CommandError {
    #[error("This command isn't implemented")]
    CommandNotImplemented,

    #[error("No command provided, refer to the help section")]
    NoCommandProvided,

    #[error("Error occurred during the command run")]
    CommandExecutionError,
}

pub trait RunnableCommand {
    fn command() -> Command;

    fn run(args: &ArgMatches) -> Result<(), CommandError>;
}

pub fn command() -> Command {
    Command::new("wg-mod")
        .version("0.1.0")
        .author("Gabriel Hamel <gabriel.hamel.pro@gmail.com>")
        .about("Provides tools for wargaming modding")
        .subcommand_required(true)
        .subcommand(NewCommand::command())
}
