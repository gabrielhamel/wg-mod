use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
enum PathBufToStringError {
    #[error("Convertion failed")]
    ConvertionFailed,
}

pub fn convert_pathbuf_to_string(path: &PathBuf) -> &str {
    path.to_str().expect("")
}