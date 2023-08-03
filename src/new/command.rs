use crate::new::NewError;
use clap::ArgMatches;
use convert_case::{Case, Casing};
use inquire::validator::{StringValidator, Validation};
use regex::Regex;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug)]
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

    complete_path.push(mod_name.from_case(Case::UpperCamel).to_case(Case::Kebab));

    Ok(complete_path)
}

#[derive(Clone)]
struct PatternValidator {
    pattern: Regex,
    error_message: String,
}

impl PatternValidator {
    fn new(pattern: &str, error_message: &str) -> Result<Self, regex::Error> {
        Ok(PatternValidator {
            pattern: Regex::from_str(pattern)?,
            error_message: error_message.into(),
        })
    }
}

impl StringValidator for PatternValidator {
    fn validate(&self, input: &str) -> Result<Validation, inquire::CustomUserError> {
        if self.pattern.is_match(input) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid(self.error_message.clone().into()))
        }
    }
}

pub fn collect_command(args: &ArgMatches) -> Result<NewModCommand, NewError> {
    let name_validator = PatternValidator::new(
        r"^(?:[A-Z][a-z0-9]+)(?:[A-Z]+[a-z0-9]*)*$",
        "Your mod name must respect the CamelCase",
    )?;

    let name = inquire::Text::new("Mod name:")
        .with_placeholder("BetterMatchmaking")
        .with_validator(name_validator)
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
        r"^([A-Za-z]{1}[A-Za-z\d_]*\.)+[A-Za-z][A-Za-z\d_]*$",
        "Your package name must be formated like this <prefix>.<dotted-namespace>.<modname>",
    )?;

    let package_name = inquire::Text::new("Package name:")
        .with_default(format!("com.example.{}", name).as_str())
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
