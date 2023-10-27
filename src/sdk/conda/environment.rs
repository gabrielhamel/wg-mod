use std::path::PathBuf;
use std::process::{Command, Output};
use std::str::Utf8Error;

#[derive(thiserror::Error, Debug)]
pub enum CondaEnvironmentError {
    #[error("Can't invoke command: {0}")]
    CommandInvocationError(PathBuf),

    #[error("Command error")]
    CommandError(Output),

    #[error("Cannot read the command output")]
    CommandOutputParsingError(#[from] Utf8Error),
}

pub struct CondaEnvironment {
    environment_path: PathBuf,
}

impl From<PathBuf> for CondaEnvironment {
    fn from(path: PathBuf) -> Self {
        Self {
            environment_path: path,
        }
    }
}

impl CondaEnvironment {
    fn get_executable_path(&self, name: &str) -> PathBuf {
        let mut conda_binaries_path = self.environment_path.clone();

        let executable_name = if cfg!(target_os = "windows") {
            format!("{name}.exe")
        } else {
            conda_binaries_path = conda_binaries_path.join("bin");
            name.to_string()
        };

        conda_binaries_path.join(executable_name)
    }

    fn command(
        &self, executable_name: &str, args: Vec<&str>,
    ) -> Result<(String, String), CondaEnvironmentError> {
        let executable_path = self.get_executable_path(executable_name);
        let mut command = Command::new(&executable_path);

        let output = command.args(args).output().map_err(|_| {
            CondaEnvironmentError::CommandInvocationError(executable_path)
        })?;

        if !output.status.success() {
            return Err(CondaEnvironmentError::CommandError(output));
        }

        let stdout = std::str::from_utf8(&output.stdout)?.to_string();
        let stderr = std::str::from_utf8(&output.stderr)?.to_string();

        Ok((stdout, stderr))
    }

    pub fn python(
        &self, args: Vec<&str>,
    ) -> Result<(String, String), CondaEnvironmentError> {
        self.command("python", args)
    }
}
