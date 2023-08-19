mod prompt;
mod template;

use self::{prompt::*, template::create_mod_files};
use crate::utils::file_template;
use clap::{ArgMatches, Command};
use std::io;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unable to create directory")]
    DirectoryCreateError(io::Error),

    #[error("Error occured during prompt")]
    PromptError(#[from] prompt::Error),

    #[error("Unable to create this template file")]
    FileTemplateError(#[from] file_template::Error),
}

#[derive(Debug, Clone)]
pub struct NewArgs {
    pub name: String,
    pub directory: PathBuf,
    pub version: String,
    pub description: String,
    pub package_name: String,
}

fn collect_args() -> Result<NewArgs, Error> {
    let name = prompt_name()?;
    let version = prompt_version()?;
    let description = prompt_description()?;
    let package_name = prompt_package_name(&name)?;
    let directory = PathBuf::from(".");

    Ok(NewArgs {
        name,
        description,
        package_name,
        version,
        directory,
    })
}

pub fn command() -> Command {
    Command::new("new")
        .about("Create a new mod project")
        .long_about("Create a directory with all default configs files and mod entrypoints")
}

pub fn execute(_: &ArgMatches) {
    match collect_args() {
        | Ok(args) => match create_mod_files(args) {
            | Ok(()) => (),
            | Err(e) => eprintln!("Command execution error {}", e),
        },
        | Err(e) => {
            eprintln!("Command error {}", e)
        },
    }
}
