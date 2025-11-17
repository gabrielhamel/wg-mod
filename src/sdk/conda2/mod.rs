mod install;

use crate::dependency::{Dependency, Error};
use install::install_conda;
use std::fs;

pub struct CondaV2 {}

impl Dependency for CondaV2 {
    const NAME: &'static str = "conda";

    fn get_version(&self) -> Result<String, Error> {
        todo!()
    }

    fn is_installed(&self) -> bool {
        match fs::metadata(self.get_executable_path()) {
            | Ok(metadata) => metadata.is_file(),
            | Err(_) => false,
        }
    }

    fn depends_on(&self) -> Vec<&'static str> {
        vec![]
    }

    fn install(&self) -> Result<(), Error> {
        install_conda(&self.install_destination)
            .map_err(|error| Error::InstallationFailed(error.to_string()))
    }
}
