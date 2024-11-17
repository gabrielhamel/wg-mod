pub mod linux_or_mac_os;
pub mod windows;

use crate::sdk::node::{Node, NodeError};
use crate::sdk::nvm::linux_or_mac_os::LinuxOrMacOsNVM;
use crate::sdk::nvm::windows::WindowsNVM;
use crate::sdk::Installable;
use crate::utils::convert_pathbuf_to_string::PathBufToStringError;
use crate::utils::Env;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::Output;
use std::string::FromUtf8Error;

#[derive(thiserror::Error, Debug)]
pub enum NVMError {
    #[error("nvm install failed")]
    InstallError(String),
    #[error("Download nvm install binary failed")]
    DownloadError(String),
    #[error("node install failed")]
    InstallNodeError(#[from] NodeError),
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
    ConversionError(#[from] PathBufToStringError),
    #[error("Failed to convert utf8 to string")]
    Utf8Error(#[from] FromUtf8Error),
}

pub trait NVM {
    fn install_node(&self) -> Result<(), NVMError>;

    fn exec(&self, args: Vec<&str>, envs: Vec<Env>)
        -> Result<Output, NVMError>;

    fn get_node(&self) -> Result<Box<dyn Node>, NVMError>;

    fn nvm_use(&self, version: &str) -> Result<Output, NVMError> {
        self.exec(vec!["use", version], vec![])
            .map_err(|_| NVMError::ExecUseError)
    }

    fn nvm_current_version(&self) -> Result<String, NVMError> {
        let out = self.exec(vec!["current"], vec![])?;
        Ok(String::from_utf8(out.stdout)
            .map_err(|_| NVMError::ExecCurrentError)?
            .trim()
            .to_string())
    }
}

fn create_nvm_directory(nvm_path: &PathBuf) -> Result<(), NVMError> {
    if !nvm_path.exists() {
        create_dir_all(nvm_path).map_err(|_| NVMError::CreateNVMDirectory)?;
    } else {
        Err(NVMError::DirExists)?
    }

    Ok(())
}

pub type BoxedNVM = Box<dyn NVM>;

pub fn load_nvm(nvm_path: &PathBuf) -> Result<BoxedNVM, NVMError> {
    let nvm_installer: Box<dyn Installable> = if cfg!(target_os = "windows") {
        Box::new(WindowsNVM::from(nvm_path))
    } else {
        Box::new(LinuxOrMacOsNVM::from(nvm_path))
    };

    if !nvm_installer.is_installed() {
        println!("Install nvm ...");
        nvm_installer
            .install()
            .map_err(|e| NVMError::InstallError(e))?;
    }

    let nvm: BoxedNVM = if cfg!(target_os = "windows") {
        Box::new(WindowsNVM::from(nvm_path))
    } else {
        Box::new(LinuxOrMacOsNVM::from(nvm_path))
    };

    Ok(nvm)
}
