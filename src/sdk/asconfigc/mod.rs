use crate::sdk::npm::{NPMError, NPM};
use crate::sdk::nvm::{BoxedNVM, NVMError};
use crate::sdk::{InstallResult, Installable};
use crate::utils::command::command;
use crate::utils::Env;
use std::process::Output;

#[derive(thiserror::Error, Debug)]
pub enum ASConfigcError {
    #[error("Execution failed")]
    FailedExecution,

    #[error("NVM error")]
    NVMError(#[from] NVMError),

    #[error("NPM error")]
    NPMError(#[from] NPMError),

    #[error("Install error")]
    InstallError(String),
}

pub struct ASConfigc {
    npm: NPM,
}

impl Installable for ASConfigc {
    fn is_installed(&self) -> bool {
        match self.npm.is_package_installed("asconfigc") {
            | Ok(res) => res,
            | Err(e) => {
                eprintln!("{}", e);
                false
            },
        }
    }

    fn install(&self) -> InstallResult {
        self.npm
            .install_package("asconfigc")
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

impl From<NPM> for ASConfigc {
    fn from(npm: NPM) -> Self {
        ASConfigc { npm }
    }
}

impl ASConfigc {
    fn exec(
        &self, args: Vec<&str>, envs: Vec<Env>,
    ) -> Result<Output, ASConfigcError> {
        let bin_dir = self.npm.get_bin_directory()?;
        let exec_path = if cfg!(target_os = "windows") {
            bin_dir.join("asconfigc.cmd")
        } else {
            bin_dir.join("asconfigc")
        };

        let executable =
            exec_path.to_str().ok_or(ASConfigcError::FailedExecution)?;

        command(executable, args, envs)
            .map_err(|_| ASConfigcError::FailedExecution)
    }
}

pub fn load_asconfigc(nvm: BoxedNVM) -> Result<ASConfigc, ASConfigcError> {
    let node = nvm.get_node()?;
    let npm = node.get_npm();
    let asconfigc = ASConfigc::from(npm);

    if !asconfigc.is_installed() {
        asconfigc
            .install()
            .map_err(|e| ASConfigcError::InstallError(e))?;
    }

    Ok(asconfigc)
}
