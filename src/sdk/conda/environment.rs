use std::path::PathBuf;
use std::process::{Command, Output};
use std::str::Utf8Error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Can't invoke command")]
    CommandInvokationError(std::io::Error),

    #[error("Command error")]
    CommandError(Output),

    #[error("Cannot read the command output")]
    CommandOutputParsingError(#[from] Utf8Error),
}

pub struct Environment {
    environment_path: PathBuf,
}

impl From<PathBuf> for Environment {
    fn from(path: PathBuf) -> Self {
        Self {
            environment_path: path,
        }
    }
}

impl Environment {
    fn get_executable_path(&self) -> PathBuf {
        let executable_name = if cfg!(target_os = "windows") {
            "python.exe"
        } else {
            "python"
        };

        let binaries_path = self.environment_path.join("bin");
        binaries_path.join(executable_name)
    }

    fn command(&self, args: Vec<&str>) -> Result<(String, String), Error> {
        let executable_path = self.get_executable_path();
        let mut command = Command::new(executable_path);

        let output = command
            .args(args)
            .output()
            .map_err(Error::CommandInvokationError)?;

        if !output.status.success() {
            return Err(Error::CommandError(output));
        }

        let stdout = std::str::from_utf8(&output.stdout)?.to_string();
        let stderr = std::str::from_utf8(&output.stderr)?.to_string();
        Ok((stdout, stderr))
    }

    pub fn version(&self) -> Result<String, Error> {
        let (_, mut out) = self.command(vec!["--version"])?;
        out = out.trim().to_string();
        out = out.replace("Python ", "");
        out = out.replace(" :: Anaconda, Inc.", "");
        Ok(out)
    }
}
