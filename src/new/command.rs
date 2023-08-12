use crate::{new::NewError, utils::pattern_validator::PatternValidator};
use clap::ArgMatches;
use convert_case::{Case, Casing};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct NewModCommand {
    pub name: String,
    pub path: PathBuf,
    pub version: String,
    pub description: String,
    pub package_name: String,
}

fn parse_path(args: &ArgMatches, mod_name: &String) -> Result<PathBuf, NewError> {
    let path = args.get_one::<String>("path").ok_or(NewError::PathError)?;
    let mut complete_path = PathBuf::from(path);

    if !complete_path.exists() {
        return Err(NewError::PathNotExists(complete_path));
    }

    complete_path.push(mod_name.from_case(Case::Alternating).to_case(Case::Kebab));

    Ok(complete_path)
}

pub fn collect_command(args: &ArgMatches) -> Result<NewModCommand, NewError> {
    let name = inquire::Text::new("Mod name:")
        .with_placeholder("Better Matchmaking")
        .prompt()?;

    let version_validator = PatternValidator::new(
        r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$",
        "Your mod version must respect the semantic versioning",
    )?;

    let version = inquire::Text::new("Version:")
        .with_default("0.0.1")
        .with_validator(version_validator)
        .prompt()?;

    let description = inquire::Text::new("Description:")
        .with_placeholder("My first mod ! Hello world")
        .with_initial_value("")
        .prompt()?;

    let package_name_validator = PatternValidator::new(
        r"^([a-z]{1}[a-z-\d_]*\.)+[a-z][a-z-\d_]*$",
        "Your package name must be formated like this <prefix>.<dotted-namespace>.<mod-name>, only lower case allowed",
    )?;

    let package_name = inquire::Text::new("Package name:")
        .with_default(
            format!(
                "com.example.{}",
                name.from_case(Case::Alternating).to_case(Case::Kebab)
            )
            .as_str(),
        )
        .with_validator(package_name_validator)
        .prompt()?;

    let path = parse_path(args, &name)?;

    Ok(NewModCommand {
        name,
        description,
        package_name,
        version,
        path,
    })
}
