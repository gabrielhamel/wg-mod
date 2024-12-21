use crate::config::Configs;
use crate::sdk::npm::NPM;
use crate::sdk::nvm::BoxedNVM;
use crate::sdk::{npm, nvm, InstallResult, Installable};
use crate::utils::command::{self, command};
use std::path::PathBuf;
use std::string::FromUtf8Error;
use std::{process, result};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Execution error")]
    ExecutionError(#[from] command::Error),

    #[error("Bad asconfigc exit status")]
    BadExitStatus(process::Output),

    #[error("NVM error")]
    NVMError(#[from] nvm::Error),

    #[error("NPM error")]
    NPMError(#[from] npm::Error),

    #[error("Install error")]
    InstallError(String),

    #[error("Unable to decode output of the command")]
    DecodeOutputError(#[from] FromUtf8Error),

    #[error("Failed to build flash")]
    BuildError(String),

    #[error("Failed to convert")]
    ConvertionError(String),
}

type Result<T> = result::Result<T, Error>;

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
    fn exec(&self, args: Vec<&str>) -> Result<process::Output> {
        let bin_dir = self.npm.get_bin_directory()?;
        let mut args_override = args.clone();
        let exec_path: PathBuf;

        if cfg!(target_os = "windows") {
            exec_path = bin_dir.join("npx.cmd");
            args_override.insert(0, "asconfigc");
        } else {
            exec_path = bin_dir.join("asconfigc");
        };

        let executable = exec_path.as_os_str();
        let output = command(executable, args_override, vec![])
            .map_err(Error::ExecutionError)?;

        if !output.status.success() {
            return Err(Error::BadExitStatus(output));
        }

        Ok(output)
    }

    pub fn build(&self, input_path: &PathBuf) -> Result<()> {
        let config =
            Configs::load().map_err(|e| Error::BuildError(e.to_string()))?;
        let as3_sdk_path = config.as3.get_as3_path();
        let as3_sdk_path_string = as3_sdk_path.to_str().ok_or(
            Error::ConvertionError("as3_sdk_path to string".to_string()),
        )?;

        let input_path_string = input_path.to_str().ok_or(
            Error::ConvertionError("input_path to string".to_string()),
        )?;

        let _ = self.exec(vec![
            "--sdk",
            as3_sdk_path_string,
            "-p",
            input_path_string,
        ])?;

        Ok(())
    }

    pub fn version(&self) -> Result<String> {
        let out = self.exec(vec!["--version"])?;
        let version = String::from_utf8(out.stdout)?.trim().to_string();

        Ok(version)
    }
}

pub fn load_asconfigc(nvm: BoxedNVM) -> Result<ASConfigc> {
    let node = nvm.get_node()?;
    let npm = node.get_npm();
    let asconfigc = ASConfigc::from(npm);

    if !asconfigc.is_installed() {
        asconfigc.install().map_err(|e| Error::InstallError(e))?;
    }

    Ok(asconfigc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sdk::nvm;
    use regex::Regex;
    use tempfile::tempdir;

    #[test]
    fn install_asconfigc() {
        let tmp_dir = tempdir().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let nvm_path = tmp_dir_path.join("nvm");
        let nvm = nvm::load_nvm(&nvm_path).unwrap();

        let asconfigc = load_asconfigc(nvm).unwrap();
        let version = asconfigc.version().unwrap();

        let semantic_version_pattern = Regex::new("^Version: ([0-9]+)\\.([0-9]+)\\.([0-9]+)(?:-([0-9A-Za-z-]+(?:\\.[0-9A-Za-z-]+)*))?(?:\\+[0-9A-Za-z-]+)?$").unwrap();
        assert!(semantic_version_pattern.is_match(&version));
    }
}
