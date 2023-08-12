mod command;
mod error;

use self::{command::collect_command, error::NewError};
use crate::utils::file_template::create_templated_file;
use clap::{Arg, ArgMatches, Command};
use serde_json::json;
use std::fs::create_dir;

pub fn command() -> Command {
    Command::new("new")
        .about("Create a new mod project")
        .long_about("Create a new mod project at <path>")
        .arg(Arg::new("path").default_value("."))
}

pub fn new(args: &ArgMatches) -> Result<(), NewError> {
    let new_command = collect_command(args)?;

    // Create mod directory
    create_dir(&new_command.path).map_err(NewError::UnableToCreateDirectory)?;

    create_templated_file(
        &new_command.path.join("meta.xml"),
        "<root>
    <id>{{package_name}}</id>
    <version>{{version}}</version>
    <name>{{name}}</name>
    <description>{{description}}</description>
</root>",
        &json!({
            "package_name": new_command.package_name,
            "version": new_command.version,
            "name": new_command.name,
            "description": new_command.description
        }),
    )?;

    Ok(())
}

pub fn execute(args: &ArgMatches) {
    if let Err(e) = new(args) {
        eprintln!("Error {}", e)
    }
}
