mod install;

use crate::sdk::as3::install::{install_flex_sdk, AS3InstallError};
use crate::sdk::{InstallResult, Installable};
use std::fs;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum AS3Error {
    #[error("Wasn't able to install AS3 SDK")]
    InstallError(#[from] AS3InstallError),
}

pub struct AS3 {
    as3_path: PathBuf,
}

impl From<PathBuf> for AS3 {
    fn from(path: PathBuf) -> Self {
        Self { as3_path: path }
    }
}

impl Installable for AS3 {
    fn is_installed(&self) -> bool {
        match fs::metadata(&self.as3_path) {
            | Ok(metadata) => metadata.is_dir(),
            | Err(_) => false,
        }
    }

    fn install(&self) -> InstallResult {
        if self.is_installed() {
            Err("AS3 SDK is already installed".into())
        } else {
            install_flex_sdk(&self.as3_path).map_err(|error| error.to_string())
        }
    }
}
