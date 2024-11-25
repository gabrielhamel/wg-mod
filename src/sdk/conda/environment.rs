use std::path::PathBuf;
use std::process::{Command, Output};
use std::result;
use std::str::Utf8Error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Can't invoke command: {0}")]
    CommandInvocationError(PathBuf),

    #[error("Command error")]
    CommandError(Output),

    #[error("Cannot read the command output")]
    CommandOutputParsingError(#[from] Utf8Error),

    #[error("Unable to reads sources directory")]
    PathError,
}

type Result<T> = result::Result<T, Error>;

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
    pub fn compile_all(&self, directory: &PathBuf) -> Result<()> {
        let python_src = directory.to_str().ok_or(Error::PathError)?;

        self.python(vec!["-m", "compileall", python_src])?;

        Ok(())
    }

    fn python(&self, args: Vec<&str>) -> Result<(String, String)> {
        self.command("python", args)
    }

    fn command(
        &self, executable_name: &str, args: Vec<&str>,
    ) -> Result<(String, String)> {
        let executable_path = self.get_executable_path(executable_name);
        let mut command = Command::new(&executable_path);

        let output = command
            .args(args)
            .output()
            .map_err(|_| Error::CommandInvocationError(executable_path))?;

        if !output.status.success() {
            return Err(Error::CommandError(output));
        }

        let stdout = std::str::from_utf8(&output.stdout)?.to_string();
        let stderr = std::str::from_utf8(&output.stderr)?.to_string();

        Ok((stdout, stderr))
    }

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
}
