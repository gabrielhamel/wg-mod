pub mod environment;
mod install;

use crate::sdk::conda::environment::PythonEnvironment;
use crate::sdk::conda::install::install_conda;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    str::Utf8Error,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Can't access to configs")]
    ConfigError(#[from] crate::config::Error),

    #[error("Cannot download provided url")]
    DownloadError(#[from] crate::utils::downloader::Error),

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
    CommandInvokationError(std::io::Error),

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
            "_conda"
        } else {
            "bin/conda"
        };

        self.conda_path.join(executable_name)
    }

    fn command(&self, args: Vec<&str>) -> Result<(String, String), Error> {
        if !self.is_installed()? {
            return Err(Error::NotInstalledError);
        }

        let executable_path = self.get_executable_path();
        let mut command = Command::new(executable_path);

        let output = command
            .args(args)
            .output()
            .map_err(Error::CommandInvokationError)?;

        if !output.status.success() {
            return Err(Error::CommandError(output));
        }

        let stdout = std::str::from_utf8(&output.stdout)?.to_string();
        let stderr = std::str::from_utf8(&output.stderr)?.to_string();
        Ok((stdout, stderr))
    }

    pub fn is_installed(&self) -> Result<bool, Error> {
        match fs::metadata(self.get_executable_path()) {
            | Ok(metadata) => Ok(metadata.is_file()),
            | Err(_) => Ok(false),
        }
    }

    pub fn version(&self) -> Result<String, Error> {
        let (mut out, _) = self.command(vec!["--version"])?;
        out = out.trim().to_string();
        out = out.replace("conda ", "");
        Ok(out)
    }

    pub fn create_environment(
        &self, name: &str, python_version: &str,
    ) -> Result<(), Error> {
        self.command(vec![
            "create",
            "-p",
            self.conda_path
                .join("envs")
                .join(name)
                .to_str()
                .ok_or(Error::PathError)?,
            &format!("python={}", python_version),
        ])?;

        Ok(())
    }

    pub fn get_environment(&self, name: &str) -> PythonEnvironment {
        let conda_envs_path = self.conda_path.join("envs");
        let environment_path = conda_envs_path.join(name);

        PythonEnvironment::from(environment_path)
    }

    pub fn has_environment(&self, name: &str) -> bool {
        let conda_envs_path = self.conda_path.join("envs");
        let environment_path = conda_envs_path.join(name);

        environment_path.exists()
    }

    pub async fn install(&self) -> Result<(), Error> {
        if self.is_installed()? {
            Err(Error::CondaAlreadyInstalled)
        } else {
            install_conda(&self.conda_path).await
        }
    }
}
