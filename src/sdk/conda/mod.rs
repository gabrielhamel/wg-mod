pub mod environment;
mod install;

use crate::sdk::conda::environment::CondaEnvironment;
use crate::sdk::conda::install::install_conda;
use crate::sdk::{InstallResult, Installable};
use crate::utils::downloader;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    result,
    str::Utf8Error,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Cannot download provided url")]
    DownloadError(#[from] downloader::Error),

    #[error("Cannot create the conda directory")]
    CreateCondaDirectory(std::io::Error),

    #[error("Cannot create a download directory")]
    PathError,

    #[error("Conda isn't installed")]
    NotInstalledError,

    #[error("Conda install error")]
    InstallError(std::io::Error),

    #[error("Can't invoke command")]
    CommandInvocationError(std::io::Error),

    #[error("Command error")]
    CommandError(Output),

    #[error("Cannot read the command output")]
    CommandOutputParsingError(#[from] Utf8Error),
}

type Result<T> = result::Result<T, Error>;

pub struct Conda {
    conda_path: PathBuf,
}

impl From<&PathBuf> for Conda {
    fn from(conda_path: &PathBuf) -> Self {
        Self {
            conda_path: conda_path.clone(),
        }
    }
}

impl Conda {
    fn get_executable_path(&self) -> PathBuf {
        if cfg!(target_os = "windows") {
            self.conda_path.join("condabin").join("conda.bat")
        } else {
            self.conda_path.join("bin").join("conda")
        }
    }

    fn command(&self, args: Vec<&str>) -> Result<(String, String)> {
        if !self.is_installed() {
            return Err(Error::NotInstalledError);
        }

        let executable_path = self.get_executable_path();
        let mut command = Command::new(executable_path);

        let output = command
            .args(args)
            .output()
            .map_err(Error::CommandInvocationError)?;

        if !output.status.success() {
            return Err(Error::CommandError(output));
        }

        let stdout = std::str::from_utf8(&output.stdout)?.to_string();
        let stderr = std::str::from_utf8(&output.stderr)?.to_string();
        Ok((stdout, stderr))
    }

    pub fn create_environment(
        &self, name: &str, python_version: &str,
    ) -> Result<()> {
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

    pub fn version(&self) -> Result<String> {
        let (out, _) = self.command(vec!["--version"])?;

        Ok(out.trim().to_string())
    }
}

impl Installable for Conda {
    fn is_installed(&self) -> bool {
        match fs::metadata(self.get_executable_path()) {
            | Ok(metadata) => metadata.is_file(),
            | Err(_) => false,
        }
    }

    fn install(&self) -> InstallResult {
        if self.is_installed() {
            Err("Conda is already installed".into())
        } else {
            install_conda(&self.conda_path).map_err(|error| error.to_string())
        }
    }
}

pub fn load_conda(conda_path: &PathBuf) -> Result<Conda> {
    let conda = Conda::from(conda_path);

    if !conda.is_installed() {
        println!("Installing conda...");
        conda.install().expect("failed conda installation");
    }

    Ok(conda)
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use tempfile::tempdir;

    #[test]
    fn install_conda() {
        let tmp_dir = tempdir().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let conda_path = tmp_dir_path.join("conda");

        let conda = load_conda(&conda_path).unwrap();
        let version = conda.version().unwrap();

        let semantic_version_pattern = Regex::new("^conda ([0-9]+)\\.([0-9]+)\\.([0-9]+)(?:-([0-9A-Za-z-]+(?:\\.[0-9A-Za-z-]+)*))?(?:\\+[0-9A-Za-z-]+)?$").unwrap();
        assert!(semantic_version_pattern.is_match(&version));
    }
}
