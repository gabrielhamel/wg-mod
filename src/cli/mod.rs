mod command;
pub mod commands;

use self::{
    command::RunnableCommand, commands::build::BuildCommand,
    commands::channel::ChannelCommand, commands::new::NewCommand,
    commands::pycharm::PycharmCommand,
};
use crate::cli::command::CommandError;

pub fn run() -> Result<(), CommandError> {
    let matches = command::command().get_matches();

    match matches.subcommand() {
        | Some(("new", args)) => NewCommand::run(args),
        | Some(("build", args)) => BuildCommand::run(args),
        | Some(("pycharm", args)) => PycharmCommand::run(args),
        | Some(("channel", args)) => ChannelCommand::run(args),
        | Some((_, _)) => Err(CommandError::CommandNotImplemented),
        | None => Err(CommandError::NoCommandProvided),
    }
}
