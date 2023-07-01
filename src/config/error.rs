#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Unable to find the user home directory")]
    UserHomeNotFound,

    #[error("Unable to parse tools directory")]
    WgToolHomeInvalid,

    #[error("Can't build or read configurations: {0}")]
    UnableToBuildConfig(#[from] config::ConfigError),
}
