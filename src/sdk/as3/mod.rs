mod install;

use crate::sdk::as3::install::install_flex_sdk;
use crate::sdk::{InstallResult, Installable};
use std::fs;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum AS3Error {
    #[error("Wasn't able to install AS3 SDK")]
    InstallError(String),
}

pub struct AS3 {
    as3_path: PathBuf,
}

impl From<&PathBuf> for AS3 {
    fn from(as3_path: &PathBuf) -> Self {
        Self {
            as3_path: as3_path.clone(),
        }
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

pub fn load_as3(as3_path: &PathBuf) -> Result<AS3, AS3Error> {
    let as3 = AS3::from(as3_path);

    if !as3.is_installed() {
        println!("Installing action script SDK...");
        as3.install().map_err(|e| AS3Error::InstallError(e))?;
    }

    Ok(as3)
}
