use crate::utils::command::command;
use crate::utils::Env;
use std::path::PathBuf;
use std::process::Output;

#[derive(thiserror::Error, Debug)]
pub enum NPMError {
    #[error("Execution failed")]
    FailedExecution,

    #[error("Get node bin directory")]
    GetBinDirectoryError,

    #[error("Unable to install package")]
    InstallPackageFailed(String),
}

pub struct NPM {
    pub npm_bin: PathBuf,
}

impl From<PathBuf> for NPM {
    fn from(npm_bin: PathBuf) -> Self {
        Self { npm_bin }
    }
}

impl NPM {
    fn exec(
        &self, args: Vec<&str>, envs: Vec<Env>,
    ) -> Result<Output, NPMError> {
        let executable =
            self.npm_bin.to_str().ok_or(NPMError::FailedExecution)?;

        command(executable, args, envs).map_err(|_| NPMError::FailedExecution)
    }

    pub fn is_package_installed(&self, name: &str) -> Result<bool, NPMError> {
        let result = self
            .exec(vec!["list", "-g", name], vec![])
            .map_err(|e| NPMError::InstallPackageFailed(e.to_string()))?;

        Ok(result.status.success())
    }

    pub fn get_bin_directory(&self) -> Result<PathBuf, NPMError> {
        self.npm_bin
            .parent()
            .ok_or(NPMError::GetBinDirectoryError)
            .and_then(|res| Ok(PathBuf::from(res)))
    }

    pub fn install_package(&self, name: &str) -> Result<(), NPMError> {
        println!("Installing {}...", name);

        let result = self
            .exec(vec!["install", "-g", name], vec![])
            .map_err(|e| NPMError::InstallPackageFailed(e.to_string()))?;

        if result.status.success() {
            return Ok(());
        }

        let stdout = String::from_utf8_lossy(&result.stdout);
        let stderr = String::from_utf8_lossy(&result.stderr);

        Err(NPMError::InstallPackageFailed(format!(
            "{}\n{}",
            stdout, stderr
        )))
    }
}
