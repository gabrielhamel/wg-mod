mod install;

use crate::dependency::{Dependency, Error};
use install::install_conda;
use std::fs;
use std::path::PathBuf;

pub struct CondaV2 {
    conda_path: PathBuf,
}

impl CondaV2 {
    pub fn new(conda_path: &str) -> Self {
        Self {
            conda_path: PathBuf::from(conda_path),
        }
    }

    fn get_executable_path(&self) -> PathBuf {
        if cfg!(target_os = "windows") {
            self.conda_path.join("condabin").join("conda.bat")
        } else {
            self.conda_path.join("bin").join("conda")
        }
    }
}

impl Dependency for CondaV2 {
    fn version(&self) -> Result<String, Error> {
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
        install_conda(&self.conda_path)
            .map_err(|error| Error::InstallationFailed(error.to_string()))
    }
}
