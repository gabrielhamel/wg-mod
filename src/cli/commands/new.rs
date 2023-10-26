use crate::cli::command::{CommandError, RunnableCommand};
use crate::new::{template::create_mod_files, NewArgs};
use crate::utils::pattern_validator::PatternValidator;
use clap::{ArgMatches, Command};
use convert_case::{Case, Casing};
use inquire::min_length;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum NewCommandError {
    #[error("Invalid regex provided")]
    RegexBuildError(#[from] regex::Error),

    #[error("Error occurred during prompt")]
    PromptError(#[from] inquire::InquireError),
}

fn prompt_version() -> Result<String, NewCommandError> {
    let validator = PatternValidator::new(
        r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$",
        "Your mod version must respect the semantic versioning",
    )?;

    let value = inquire::Text::new("Version:")
        .with_default("0.0.1")
        .with_validator(validator)
        .prompt()?;

    Ok(value)
}

fn prompt_name() -> Result<String, NewCommandError> {
    let value = inquire::Text::new("Mod name:")
        .with_placeholder("Better Matchmaking")
        .with_validator(min_length!(2, "Minimum of 2 characters required"))
        .prompt()?;

    Ok(value)
}

fn prompt_description() -> Result<String, NewCommandError> {
    let value = inquire::Text::new("Description:")
        .with_placeholder("My first mod ! Hello world")
        .with_initial_value("")
        .prompt()?;

    Ok(value)
}

fn prompt_package_name(name: &String) -> Result<String, NewCommandError> {
    let validator = PatternValidator::new(
        r"^([a-z]{1}[a-z-\d_]*\.)+[a-z][a-z-\d_]*$",
        "Your package name must be formated like this <prefix>.<dotted-namespace>.<mod-name>, only lower case allowed",
    )?;

    let value = inquire::Text::new("Package name:")
        .with_default(
            format!(
                "com.example.{}",
                name.from_case(Case::Alternating).to_case(Case::Kebab)
            )
            .as_str(),
        )
        .with_validator(validator)
        .prompt()?;

    Ok(value)
}

fn collect_args() -> Result<NewArgs, NewCommandError> {
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

pub struct NewCommand;

impl RunnableCommand for NewCommand {
    fn command() -> Command {
        Command::new("new")
            .about("Create a new mod project")
            .long_about("Create a directory with all default configs files and mod entrypoints")
    }

    fn run(_: &ArgMatches) -> Result<(), CommandError> {
        match collect_args() {
            | Ok(args) => match create_mod_files(args) {
                | Ok(()) => Ok(()),
                | Err(e) => {
                    Err(CommandError::CommandExecutionError(e.to_string()))
                },
            },
            | Err(e) => Err(CommandError::CommandExecutionError(e.to_string())),
        }
    }
}
