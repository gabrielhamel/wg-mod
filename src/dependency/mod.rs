use std::path::PathBuf;

pub mod store;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Installation failed: {0}")]
    InstallationFailed(String),

    #[error("Dependency not installed")]
    DependencyNotInstalled,

    #[error("Unable to get version of the dependency: {0}")]
    UnableToGetVersion(String),
}

pub trait Dependency {
    fn version(&self, path: &PathBuf) -> Result<String, Error>;

    fn is_installed(&self, path: &PathBuf) -> bool;

    fn depends_on(&self) -> Vec<&'static str>;

    fn install(&self, path: &PathBuf) -> Result<(), Error>;
}
