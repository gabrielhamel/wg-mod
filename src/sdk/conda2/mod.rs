mod install;

use crate::dependency::{Dependency, Error};
use install::install_conda;
use std::fs;
use std::path::PathBuf;

pub struct CondaV2;

impl Default for CondaV2 {
    fn default() -> Self {
        Self
    }
}

impl CondaV2 {
    fn get_executable_path(&self, path: &PathBuf) -> PathBuf {
        if cfg!(target_os = "windows") {
            path.join("condabin").join("conda.bat")
        } else {
            path.join("bin").join("conda")
        }
    }
}

impl Dependency for CondaV2 {
    fn version(&self, path: &PathBuf) -> Result<String, Error> {
        todo!()
    }

    fn is_installed(&self, path: &PathBuf) -> bool {
        match fs::metadata(self.get_executable_path(path)) {
            | Ok(metadata) => metadata.is_file(),
            | Err(_) => false,
        }
    }

    fn depends_on(&self) -> Vec<&'static str> {
        vec![]
    }

    fn install(&self, path: &PathBuf) -> Result<(), Error> {
        install_conda(path)
            .map_err(|error| Error::InstallationFailed(error.to_string()))
    }
}
