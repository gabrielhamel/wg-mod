mod install;

use crate::config::Configs;
use crate::sdk::conda::install::install_conda;
use crate::utils::task_progress::{TaskProgressionBar, TaskProgressionSpinner};
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

impl Conda {
    pub fn default() -> Result<Self, Error> {
        let config = Configs::load()?;
        let conda_path = config.wg_mod_home.join("conda");

        Ok(Self { conda_path })
    }

    fn command(&self, args: Vec<&str>) -> Result<String, Error> {
        if !self.is_installed()? {
            return Err(Error::NotInstalledError);
        }

        let mut command = if cfg!(target_os = "windows") {
            Command::new(self.conda_path.join("_conda"))
        } else {
            Command::new(self.conda_path.join("bin/conda"))
        };

        let output = command
            .args(args)
            .output()
            .map_err(Error::CommandInvokationError)?;

        if !output.status.success() {
            return Err(Error::CommandError(output));
        }

        let stdout = std::str::from_utf8(&output.stdout)?.to_string();
        Ok(stdout)
    }

    pub fn is_installed(&self) -> Result<bool, Error> {
        let path = if cfg!(target_os = "windows") {
            "_conda.exe"
        } else {
            "bin/conda"
        };

        match fs::metadata(self.conda_path.join(path)) {
            | Ok(metadata) => Ok(metadata.is_file()),
            | Err(_) => Ok(false),
        }
    }

    pub fn version(&self) -> Result<String, Error> {
        let mut out = self.command(vec!["--version"])?;
        out = out.trim().to_string();
        out = out.replace("conda ", "");
        Ok(out)
    }

    pub fn create_env(
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
        ])
        .and(Ok(()))
    }

    pub async fn install_if_not_installed(&self) -> Result<(), Error> {
        if self.is_installed()? {
            Ok(())
        } else {
            let download_progression =
                TaskProgressionBar::new("Downloading conda");
            let install_progression =
                TaskProgressionSpinner::new("Installing conda...");

            install_conda(
                &self.conda_path,
                download_progression,
                install_progression,
            )
            .await
        }
    }
}
