use crate::config::{Configs, ConfigsError};
use crate::sdk::conda::environment::{CondaEnvironment, CondaEnvironmentError};
use crate::sdk::conda::{Conda, CondaError};
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum PythonBuilderError {
    #[error("Cannot create a download directory")]
    PathError,

    #[error("Conda environment error")]
    CondaEnvironmentError(#[from] CondaEnvironmentError),

    #[error("Conda error")]
    CondaError(#[from] CondaError),

    #[error("Can't access to configs")]
    ConfigsError(#[from] ConfigsError),
}

pub struct PythonBuilder {
    conda_environment: CondaEnvironment,
}

fn get_conda() -> Result<Conda, PythonBuilderError> {
    let config = Configs::load()?;

    let conda_path = config.wg_mod_home.join("conda");
    let conda = Conda::from(conda_path);

    if !conda.is_installed().expect("") {
        println!("Installing conda...");
        conda.install().expect("");
    }

    Ok(conda)
}

fn get_python_2_environment(
    conda: Conda,
) -> Result<CondaEnvironment, PythonBuilderError> {
    if !conda.has_environment("wg-mod") {
        println!("Create conda env...");
        conda.create_environment("wg-mod", "2")?;
    }

    Ok(conda.get_environment("wg-mod"))
}

impl PythonBuilder {
    pub fn new() -> Result<Self, PythonBuilderError> {
        let conda = get_conda()?;
        let conda_environment = get_python_2_environment(conda)?;

        Ok(Self { conda_environment })
    }

    pub fn build(&self, directory: PathBuf) -> Result<(), PythonBuilderError> {
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
