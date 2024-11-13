use crate::builder::{ModBuilder, ModBuilderError};
use crate::cli::command::{CommandError, RunnableCommand};
use clap::{ArgMatches, Command};
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum ExportCommandError {
    #[error("Failed to use build tools\n{0}")]
    ModBuilderError(#[from] ModBuilderError),
}

pub struct ExportCommand;

fn build() -> Result<(), ExportCommandError> {
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

    fn run(_: &ArgMatches) -> Result<(), CommandError> {
        match build() {
            | Ok(()) => Ok(()),
            | Err(e) => Err(CommandError::CommandExecutionError(e.to_string())),
        }
    }
}
