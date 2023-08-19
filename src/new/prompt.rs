use crate::utils::pattern_validator::PatternValidator;
use convert_case::{Case, Casing};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid regex provided")]
    RegexBuildError(#[from] regex::Error),

    #[error("Error occured during prompt")]
    PromptError(#[from] inquire::InquireError),
}

type PromptResult<T> = Result<T, Error>;

pub fn prompt_version() -> PromptResult<String> {
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

pub fn prompt_name() -> PromptResult<String> {
    let value = inquire::Text::new("Mod name:")
        .with_placeholder("Better Matchmaking")
        .prompt()?;

    Ok(value)
}

pub fn prompt_description() -> PromptResult<String> {
    let value = inquire::Text::new("Description:")
        .with_placeholder("My first mod ! Hello world")
        .with_initial_value("")
        .prompt()?;

    Ok(value)
}

pub fn prompt_package_name(name: &String) -> PromptResult<String> {
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
