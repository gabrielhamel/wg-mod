use handlebars::Handlebars;
use serde::Serialize;
use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error occured during file creation")]
    FileCreateError(io::Error, PathBuf),

    #[error("Unable to write in this file")]
    TemplateWriteError(#[from] handlebars::RenderError),

    #[error("Unable to create directory")]
    DirectoryCreateError(io::Error),
}

pub fn write_template<T>(
    dir: &PathBuf, filename: &str, template: &str, data: &T,
) -> Result<(), Error>
where
    T: Serialize,
{
    fs::create_dir_all(&dir).map_err(Error::DirectoryCreateError)?;

    let filepath = dir.join(filename);
    let file = File::create(&filepath)
        .map_err(|e| Error::FileCreateError(e, filepath))?;

    Handlebars::new()
        .render_template_to_write(template, data, file)
        .map_err(Error::TemplateWriteError)
}

#[test]
fn file_template() {
    use serde_json::json;
    use std::io::Read;
    use tempfile::tempdir;

    let tmp_dir = tempdir().unwrap();
    let filepath = tmp_dir.path().join("file.txt");

    write_template(
        &tmp_dir.path().to_path_buf(),
        "file.txt",
        "{{one}} {{two}} !",
        &json!({
            "one": "Hello",
            "two": "world"
        }),
    )
    .unwrap();

    let mut file = File::open(&filepath).unwrap();
    let mut file_content = String::new();
    let bytes_readed = file.read_to_string(&mut file_content).unwrap();

    assert_eq!(bytes_readed, 13);
    assert_eq!(file_content, "Hello world !");

    tmp_dir.close().unwrap();
}
