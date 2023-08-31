use clap::{ArgMatches, Command};

use super::{commands::new::NewCommand, errors::Error};

pub trait RunnableCommand {
    fn command() -> Command;

    fn run(args: &ArgMatches) -> Result<(), Error>;
}

pub fn command() -> Command {
    Command::new("wg-mod")
        .version("0.1.0")
        .author("Gabriel Hamel <gabriel.hamel.pro@gmail.com>")
        .about("Provides tools for wargaming modding")
        .subcommand_required(true)
        .subcommand(NewCommand::command())
}
