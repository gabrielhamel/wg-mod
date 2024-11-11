pub mod linux_or_mac_os;
pub mod windows;

use crate::sdk::node::Node;
use crate::utils::convert_pathbuf_to_string::PathBufToStringError;
use crate::utils::Env;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::Output;
use std::string::FromUtf8Error;

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
    #[error("Failed to execute nvm command")]
    ExecError,
    #[error("Conversion failed")]
    ConversionError(#[from] PathBufToStringError),
    #[error("Failed to convert utf8 to string")]
    Utf8Error(#[from] FromUtf8Error),
}

pub trait NVM {
    fn install(&self) -> Result<(), NVMError>;

    fn install_node(&self) -> Result<(), NVMError>;

    fn exec(&self, args: Vec<&str>, envs: Vec<Env>)
        -> Result<Output, NVMError>;

    fn get_node(&self) -> Result<Node, NVMError>;
}

pub fn create_nvm_directory(nvm_path: &PathBuf) -> Result<(), NVMError> {
    if !nvm_path.exists() {
        create_dir_all(nvm_path.join("nvm"))
            .map_err(|_| NVMError::CreateNVMDirectory)?;
    } else {
        Err(NVMError::DirExists)?
    }

    Ok(())
}
