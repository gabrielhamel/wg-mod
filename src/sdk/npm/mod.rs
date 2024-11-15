use crate::utils::command::command;
use crate::utils::Env;
use std::path::PathBuf;
use std::process::Output;

#[derive(thiserror::Error, Debug)]
pub enum NPMError {
    #[error("Execution failed")]
    FailedExecution,
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
    pub fn exec(
        &self, args: Vec<&str>, envs: Vec<Env>,
    ) -> Result<Output, NPMError> {
        let executable =
            self.npm_bin.to_str().ok_or(NPMError::FailedExecution)?;

        command(executable, args, envs).map_err(|_| NPMError::FailedExecution)
    }
}
