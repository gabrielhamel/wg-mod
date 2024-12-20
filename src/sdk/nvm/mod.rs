pub mod linux_or_mac_os;
pub mod windows;

use crate::sdk::node::Node;
use crate::sdk::nvm::linux_or_mac_os::LinuxOrMacOsNVM;
use crate::sdk::nvm::windows::WindowsNVM;
use crate::sdk::{node, Installable};
use crate::utils::convert_pathbuf_to_string;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::Output;
use std::result;
use std::string::FromUtf8Error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("nvm install failed")]
    InstallError(String),
    #[error("Download nvm install binary failed")]
    DownloadError(String),
    #[error("node install failed")]
    InstallNodeError(#[from] node::Error),
    #[error("Failed to create NVM directory ")]
    CreateNVMDirectory,
    #[error("NVM directory already exists  ")]
    DirExists,
    #[error("Failed to execute nvm command")]
    ExecError,
    #[error("Failed to execute nvm use command")]
    ExecUseError,
    #[error("Failed to execute nvm current command")]
    ExecCurrentError,
    #[error("Conversion failed")]
    ConversionError(#[from] convert_pathbuf_to_string::Error),
    #[error("Failed to convert utf8 to string")]
    Utf8Error(#[from] FromUtf8Error),
}

type Result<T> = result::Result<T, Error>;

pub trait NVM {
    fn install_node(&self) -> Result<()>;

    fn exec(&self, args: Vec<&str>) -> Result<Output>;

    fn get_node(&self) -> Result<Box<dyn Node>>;

    fn nvm_use(&self, version: &str) -> Result<Output> {
        self.exec(vec!["use", version])
            .map_err(|_| Error::ExecUseError)
    }

    fn current_node_version(&self) -> Result<String>;

    fn version(&self) -> Result<String> {
        let out = self.exec(vec!["--version"])?;

        Ok(String::from_utf8(out.stdout)
            .map_err(|_| Error::ExecCurrentError)?
            .trim()
            .to_string())
    }
}

fn create_nvm_directory(nvm_path: &PathBuf) -> Result<()> {
    if !nvm_path.exists() {
        create_dir_all(nvm_path).map_err(|_| Error::CreateNVMDirectory)?;
    } else {
        Err(Error::DirExists)?
    }

    Ok(())
}

pub type BoxedNVM = Box<dyn NVM>;

pub fn load_nvm(nvm_path: &PathBuf) -> Result<BoxedNVM> {
    let nvm_installer: Box<dyn Installable> = if cfg!(target_os = "windows") {
        Box::new(WindowsNVM::from(nvm_path))
    } else {
        Box::new(LinuxOrMacOsNVM::from(nvm_path))
    };

    if !nvm_installer.is_installed() {
        println!("Install nvm ...");
        nvm_installer
            .install()
            .map_err(|e| Error::InstallError(e))?;
    }

    let nvm: BoxedNVM = if cfg!(target_os = "windows") {
        Box::new(WindowsNVM::from(nvm_path))
    } else {
        Box::new(LinuxOrMacOsNVM::from(nvm_path))
    };

    Ok(nvm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use tempfile::tempdir;

    #[test]
    fn install_nvm() {
        let tmp_dir = tempdir().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let nvm_path = tmp_dir_path.join("nvm");

        let nvm = load_nvm(&nvm_path).unwrap();
        let version = nvm.version().unwrap();

        let semantic_version_pattern = Regex::new("^([0-9]+)\\.([0-9]+)\\.([0-9]+)(?:-([0-9A-Za-z-]+(?:\\.[0-9A-Za-z-]+)*))?(?:\\+[0-9A-Za-z-]+)?$").unwrap();
        assert!(semantic_version_pattern.is_match(&version));
    }
}
