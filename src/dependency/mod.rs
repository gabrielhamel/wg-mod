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
    const NAME: &'static str;

    fn get_version(&self) -> Result<String, Error>;

    fn is_installed(&self) -> bool;

    fn depends_on(&self) -> Vec<&'static str>;

    fn install(&self) -> Result<(), Error>;
}