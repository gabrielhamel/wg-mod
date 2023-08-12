use handlebars::Handlebars;
use serde::Serialize;
use std::{fs::File, path::PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum FileTemplateError {
    #[error("Cannot apply template")]
    RenderError(#[from] handlebars::RenderError),

    #[error("Unable to create the file")]
    UnableToCreateFile(#[from] std::io::Error),
}

pub fn create_templated_file<T>(
    filepath: &PathBuf, template_string: &str, data: &T,
) -> Result<(), FileTemplateError>
where
    T: Serialize,
{
    let handlebar = Handlebars::new();

    let file = File::create(filepath).map_err(FileTemplateError::UnableToCreateFile)?;

    handlebar.render_template_to_write(template_string, data, file)?;

    Ok(())
}
