use crate::sdk::npm::{NPMError, NPM};
use crate::sdk::nvm::{BoxedNVM, NVMError};
use crate::sdk::{InstallResult, Installable};
use crate::utils::command::{self, command};
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::process;
use std::string::FromUtf8Error;

#[derive(thiserror::Error, Debug)]
pub enum ASConfigcError {
    #[error("Execution error")]
    ExecutionError(#[from] command::Error),

    #[error("Bad exit status")]
    BadExitStatus(process::Output),

    #[error("NVM error")]
    NVMError(#[from] NVMError),

    #[error("NPM error")]
    NPMError(#[from] NPMError),

    #[error("Install error")]
    InstallError(String),

    #[error("Unable to decode output of the command")]
    DecodeOutputError(#[from] FromUtf8Error),
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
    fn exec(&self, args: Vec<&str>) -> Result<process::Output, ASConfigcError> {
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
            .map_err(ASConfigcError::ExecutionError)?;

        if !output.status.success() {
            return Err(ASConfigcError::BadExitStatus(output));
        }

        Ok(output)
    }

    pub fn version(&self) -> Result<String, ASConfigcError> {
        let out = self.exec(vec!["--version"])?;
        let version = String::from_utf8(out.stdout)?.trim().to_string();

        Ok(version)
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

fn recurse_files(path: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
    let mut buf = vec![];
    let entries = read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let meta = entry.metadata()?;

        if meta.is_dir() {
            let mut subdir = recurse_files(entry.path())?;
            buf.append(&mut subdir);
        }

        if meta.is_file() {
            buf.push(entry.path());
        }
    }

    Ok(buf)
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
