pub mod template;
mod tests;

use crate::utils::file_template::TemplateError;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum NewError {
    #[error("Unable to create this template file")]
    FileTemplateError(#[from] TemplateError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewArgs {
    pub name: String,
    pub directory: PathBuf,
    pub version: String,
    pub description: String,
    pub package_name: String,
}
