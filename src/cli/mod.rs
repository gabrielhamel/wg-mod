mod command;
pub mod commands;

use self::{
    command::RunnableCommand, commands::channel::ChannelCommand,
    commands::export::ExportCommand, commands::new::NewCommand,
    commands::pycharm::PycharmCommand,
};

pub fn run() -> Result<(), command::Error> {
    let matches = command::command().get_matches();

    match matches.subcommand() {
        | Some(("new", args)) => NewCommand::run(args),
        | Some(("export", args)) => ExportCommand::run(args),
        | Some(("pycharm", args)) => PycharmCommand::run(args),
        | Some(("channel", args)) => ChannelCommand::run(args),
        | Some((_, _)) => Err(command::Error::CommandNotImplemented),
        | None => Err(command::Error::NoCommandProvided),
    }
}
