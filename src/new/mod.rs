pub mod template;
mod tests;

use crate::utils::file_template;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unable to create this template file")]
    FileTemplateError(#[from] file_template::Error),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewArgs {
    pub name: String,
    pub directory: PathBuf,
    pub version: String,
    pub description: String,
    pub package_name: String,
}
