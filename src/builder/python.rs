use crate::sdk::conda::environment::{CondaEnvironment, CondaEnvironmentError};
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum PythonBuilderError {
    #[error("Cannot create a download directory")]
    PathError,

    #[error("Conda error")]
    CondaError(#[from] CondaEnvironmentError),
}

struct PythonBuilder {
    conda_environment: CondaEnvironment,
}

impl From<CondaEnvironment> for PythonBuilder {
    fn from(conda_environment: CondaEnvironment) -> Self {
        Self { conda_environment }
    }
}

impl PythonBuilder {
    fn compile_all(
        &self, directory: PathBuf,
    ) -> Result<(), PythonBuilderError> {
        let readable_path =
            directory.to_str().ok_or(PythonBuilderError::PathError)?;

        self.conda_environment.python(vec![
            "-m",
            "compileall",
            readable_path,
        ])?;
        Ok(())
    }
}
