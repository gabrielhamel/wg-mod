use crate::builder::{ModBuilder, ModBuilderError};
use crate::cli::command::{CommandError, RunnableCommand};
use clap::{ArgMatches, Command};
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum BuildCommandError {
    #[error("Failed to use build tools")]
    ModBuilderError(#[from] ModBuilderError),
}

pub struct BuildCommand;

fn build() -> Result<(), BuildCommandError> {
    let mod_path = PathBuf::from(".");
    let mod_builder = ModBuilder::new(mod_path)?;
    mod_builder.build()?;

    Ok(())
}

impl RunnableCommand for BuildCommand {
    fn command() -> Command {
        Command::new("build")
            .about("Compile the current mod")
            .long_about("Compile a local mod directory as a .wotmod file")
    }

    fn run(_: &ArgMatches) -> Result<(), CommandError> {
        match build() {
            | Ok(()) => Ok(()),
            | Err(_) => Err(CommandError::CommandExecutionError),
        }
    }
}
