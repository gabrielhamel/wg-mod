use crate::builder;
use crate::builder::ModBuilder;
use crate::cli::command;
use crate::cli::command::RunnableCommand;
use clap::{ArgMatches, Command};
use std::path::PathBuf;
use std::result;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to use build tools\n{0}")]
    ModBuilderError(#[from] builder::Error),
}

type Result<T> = result::Result<T, Error>;

pub struct ExportCommand;

fn build() -> Result<()> {
    let mod_path = PathBuf::from(".");
    let mod_builder = ModBuilder::new(mod_path)?;
    mod_builder.build()?;

    Ok(())
}

impl RunnableCommand for ExportCommand {
    fn command() -> Command {
        Command::new("export")
            .about("Assemble sources into a .wotmod")
            .long_about("Compile the local mod directory as a .wotmod file")
    }

    fn run(_: &ArgMatches) -> result::Result<(), command::Error> {
        match build() {
            | Ok(()) => Ok(()),
            | Err(e) => {
                Err(command::Error::CommandExecutionError(e.to_string()))
            },
        }
    }
}
