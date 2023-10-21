mod python;

use crate::builder::python::{PythonBuilder, PythonBuilderError};
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum ModBuilderError {
    #[error("Failed to use the python mod builder")]
    PythonBuilderError(#[from] PythonBuilderError),

    #[error("Not a mod folder")]
    BadModFolderError,
}

pub struct ModBuilder {
    python_builder: PythonBuilder,
    mod_path: PathBuf,
}

impl ModBuilder {
    pub fn new(mod_path: PathBuf) -> Result<Self, ModBuilderError> {
        let python_builder = PythonBuilder::new()?;

        Ok(Self {
            python_builder,
            mod_path,
        })
    }

    pub fn build(&self) -> Result<(), ModBuilderError> {
        let is_mod_folder = self.is_mod_folder();
        if is_mod_folder == false {
            return Err(ModBuilderError::BadModFolderError);
        }

        let script_path = self.mod_path.join("scripts");
        self.python_builder.build(script_path)?;
        Ok(())
    }

    fn is_mod_folder(&self) -> bool {
        let meta_path = self.mod_path.join("meta.xml");
        meta_path.exists()
    }
}
