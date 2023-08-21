use inquire::{
    validator::{StringValidator, Validation},
    CustomUserError,
};
use regex::{Error, Regex};
use std::str::FromStr;

#[derive(Clone)]
pub struct PatternValidator {
    pattern: Regex,
    error_message: String,
}

impl PatternValidator {
    pub fn new(pattern: &str, error_message: &str) -> Result<Self, Error> {
        Ok(PatternValidator {
            pattern: Regex::from_str(pattern)?,
            error_message: error_message.into(),
        })
    }
}

impl StringValidator for PatternValidator {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        if self.pattern.is_match(input) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid(self.error_message.clone().into()))
        }
    }
}

#[test]
fn pattern_validator() {
    let validator = PatternValidator::new(
        r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$",
        "Must respect the semantic versioning",
    )
    .unwrap();

    assert_eq!(validator.validate("0.0.1").unwrap(), Validation::Valid);
    assert_eq!(
        validator.validate("Hello world").unwrap(),
        Validation::Invalid(inquire::validator::ErrorMessage::Custom(
            "Must respect the semantic versioning".to_owned()
        ))
    );
}
