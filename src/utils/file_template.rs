use handlebars::Handlebars;
use serde::Serialize;
use std::{fs::File, io, path::PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error occured during file creation")]
    FileCreateError(io::Error, PathBuf),

    #[error("Unable to write in this file")]
    TemplateWriteError(#[from] handlebars::RenderError),
}

pub fn write_template<T>(filepath: PathBuf, template: &str, data: &T) -> Result<(), Error>
where
    T: Serialize,
{
    let file = File::create(&filepath).map_err(|e| Error::FileCreateError(e, filepath))?;

    Handlebars::new()
        .render_template_to_write(template, data, file)
        .map_err(Error::TemplateWriteError)
}
