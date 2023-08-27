use crate::{config::Configs, utils::downloader::download_file};
use spinners::{Spinner, Spinners};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    str::Utf8Error,
};

pub async fn download() {
    download_file(
        "https://repo.anaconda.com/miniconda/Miniconda3-latest-MacOSX-arm64.sh",
        "Miniconda3-latest-MacOSX-arm64.sh",
    )
    .await
    .unwrap();
}

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
        let output = Command::new(self.conda_path.join("bin/conda"))
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
        let version = self.version();

        Ok(version.is_ok())
    }

    pub fn version(&self) -> Result<String, Error> {
        let mut out = self.command(vec!["--version"])?;
        out = out.trim().to_string();
        out = out.replace("conda ", "");
        Ok(out)
    }

    pub async fn install(&self) -> Result<(), Error> {
        if self.is_installed()? {
            println!("Conda is already installed ({})", self.version()?);
            return Ok(());
        }

        let arch = match (std::env::consts::OS, std::env::consts::ARCH) {
            | ("macos", "aarch64") => ("MacOSX", "arm64", "sh"),
            | ("macos", arch) => ("MacOSX", arch, "sh"),
            | ("windows", arch) => ("Windows", arch, "exe"),
            | (os, arch) => (os, arch, "sh"),
        };

        fs::create_dir_all(&self.conda_path).map_err(Error::CreateCondaDirectory)?;

        let install_destination = self
            .conda_path
            .parent()
            .ok_or(Error::PathError)?
            .join(PathBuf::from(format!("install-conda.{}", arch.2)))
            .to_str()
            .ok_or(Error::PathError)?
            .to_string();

        download_file(
            format!(
                "https://repo.anaconda.com/miniconda/Miniconda3-latest-{}-{}.{}",
                arch.0, arch.1, arch.2
            )
            .as_str(),
            install_destination.as_str(),
        )
        .await?;

        println!("");
        let mut sp = Spinner::new(Spinners::Arrow3, "Installing conda...".into());

        // TODO INSTALL WINDOWS
        Command::new("sh")
            .args([
                install_destination.as_str(),
                "-p",
                self.conda_path.to_str().ok_or(Error::PathError)?,
                "-b",
                "-u",
            ])
            .output()
            .map_err(Error::InstallError)?;

        sp.stop_with_newline();

        Ok(())
    }
}
