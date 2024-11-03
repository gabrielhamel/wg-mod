pub mod windows;
pub mod linuxOrMacOS;

use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::path::PathBuf;
use crate::utils::Env;

#[derive(thiserror::Error, Debug)]
pub enum NVMError {
    #[error("nvm install failed")]
    InstallError,
    #[error("Download nvm install binary failed")]
    DownloadError,
    #[error("node install failed")]
    InstallNodeError,
    #[error("Failed to create NVM directory ")]
    CreateNVMDirectory,
    #[error("NVM directory already exists  ")]
    DirExists,
    #[error("PathBuf conversion failed")]
    ConversionError,
    #[error("Failed to execute nvm command")]
    ExecError,

}


pub trait NVM {
    fn install(&self) -> Result<(), NVMError>;

    fn install_node(&self, destination: &PathBuf) -> Result<(), NVMError>;

    fn exec(&self, args: Vec<&str>, env: Vec<Env>) -> Result<(), NVMError>;
}

pub fn create_nvm_directory(nvm_path: &PathBuf) -> Result<(), NVMError> {
    if !nvm_path.exists() {
        create_dir_all(nvm_path.join("nvm")).map_err(|_| NVMError::CreateNVMDirectory)?;
    }else {
        Err(NVMError::DirExists)?
    }

    Ok(())
}