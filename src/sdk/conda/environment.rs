use std::path::PathBuf;
use std::process::{Command, Output};
use std::str::Utf8Error;

#[derive(thiserror::Error, Debug)]
pub enum CondaEnvironmentError {
    #[error("Can't invoke command")]
    CommandInvokationError(std::io::Error),

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
        let executable_name = if cfg!(target_os = "windows") {
            format!("{name}.exe")
        } else {
            name.to_string()
        };

        let binaries_path = self.environment_path.join("bin");
        binaries_path.join(executable_name)
    }

    fn command(
        &self, executable_name: &str, args: Vec<&str>,
    ) -> Result<(String, String), CondaEnvironmentError> {
        let executable_path = self.get_executable_path(executable_name);
        let mut command = Command::new(executable_path);

        let output = command
            .args(args)
            .output()
            .map_err(CondaEnvironmentError::CommandInvokationError)?;

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
