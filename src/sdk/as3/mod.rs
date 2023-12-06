mod install;

use crate::sdk::as3::install::{install_flex_sdk, AS3InstallError};
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum AS3Error {
    #[error("AS3 SDK is already installed")]
    AS3AlreadyInstalled,

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

impl AS3 {
    pub fn is_installed(&self) -> Result<bool, AS3Error> {
        Ok(false)
    }

    pub fn install(&self) -> Result<(), AS3Error> {
        if self.is_installed()? {
            Err(AS3Error::AS3AlreadyInstalled)
        } else {
            install_flex_sdk(&self.as3_path).map_err(AS3Error::InstallError)
        }
    }
}
