mod command;
pub mod commands;

use self::{
    command::RunnableCommand, commands::channel::ChannelCommand,
    commands::export::ExportCommand, commands::new::NewCommand,
    commands::pycharm::PycharmCommand,
};
use crate::cli::command::CommandError;

pub fn run() -> Result<(), CommandError> {
    let matches = command::command().get_matches();

    match matches.subcommand() {
        | Some(("new", args)) => NewCommand::run(args),
        | Some(("export", args)) => ExportCommand::run(args),
        | Some(("pycharm", args)) => PycharmCommand::run(args),
        | Some(("channel", args)) => ChannelCommand::run(args),
        | Some((_, _)) => Err(CommandError::CommandNotImplemented),
        | None => Err(CommandError::NoCommandProvided),
    }
}
