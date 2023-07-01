use crate::config::error::ConfigError;

#[derive(thiserror::Error, Debug)]
pub enum DoctorError {
    #[error("error when loading configurations: {0}")]
    ConfigError(#[from] ConfigError),
}
