pub mod environment;
mod install;

use crate::sdk::conda::environment::CondaEnvironment;
use crate::sdk::conda::install::install_conda;
use crate::utils::downloader::DownloadError;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    str::Utf8Error,
};

#[derive(thiserror::Error, Debug)]
pub enum CondaError {
    #[error("Cannot download provided url")]
    DownloadError(#[from] DownloadError),

    #[error("Cannot create the conda directory")]
    CreateCondaDirectory(std::io::Error),

    #[error("Cannot create a download directory")]
    PathError,

    #[error("Conda isn't installed")]
    NotInstalledError,

    #[error("Conda is already installed")]
    CondaAlreadyInstalled,

    #[error("Conda install error")]
    InstallError(std::io::Error),

    #[error("Can't invoke command")]
    CommandInvocationError(std::io::Error),

    #[error("Command error")]
    CommandError(Output),

    #[error("Cannot read the command output")]
    CommandOutputParsingError(#[from] Utf8Error),
}

pub struct Conda {
    conda_path: PathBuf,
}

impl From<PathBuf> for Conda {
    fn from(path: PathBuf) -> Self {
        Self { conda_path: path }
    }
}

impl Conda {
    fn get_executable_path(&self) -> PathBuf {
        let executable_name = if cfg!(target_os = "windows") {
            "condabin\\conda.bat"
        } else {
            "bin/conda"
        };

        self.conda_path.join(executable_name)
    }

    fn command(&self, args: Vec<&str>) -> Result<(String, String), CondaError> {
        if !self.is_installed()? {
            return Err(CondaError::NotInstalledError);
        }

        let executable_path = self.get_executable_path();
        let mut command = Command::new(executable_path);

        let output = command
            .args(args)
            .output()
            .map_err(CondaError::CommandInvocationError)?;

        if !output.status.success() {
            return Err(CondaError::CommandError(output));
        }

        let stdout = std::str::from_utf8(&output.stdout)?.to_string();
        let stderr = std::str::from_utf8(&output.stderr)?.to_string();
        Ok((stdout, stderr))
    }

    pub fn is_installed(&self) -> Result<bool, CondaError> {
        match fs::metadata(self.get_executable_path()) {
            | Ok(metadata) => Ok(metadata.is_file()),
            | Err(_) => Ok(false),
        }
    }

    pub fn create_environment(
        &self, name: &str, python_version: &str,
    ) -> Result<(), CondaError> {
        self.command(vec![
            "create",
            "-p",
            self.conda_path
                .join("envs")
                .join(name)
                .to_str()
                .ok_or(CondaError::PathError)?,
            &format!("python={}", python_version),
        ])?;

        Ok(())
    }

    pub fn get_environment(&self, name: &str) -> CondaEnvironment {
        let conda_envs_path = self.conda_path.join("envs");
        let environment_path = conda_envs_path.join(name);

        CondaEnvironment::from(environment_path)
    }

    pub fn has_environment(&self, name: &str) -> bool {
        let conda_envs_path = self.conda_path.join("envs");
        let environment_path = conda_envs_path.join(name);

        environment_path.exists()
    }

    pub fn install(&self) -> Result<(), CondaError> {
        if self.is_installed()? {
            Err(CondaError::CondaAlreadyInstalled)
        } else {
            install_conda(&self.conda_path)
        }
    }
}
