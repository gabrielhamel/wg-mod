use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum PathBufToStringError {
    #[error("Conversion failed")]
    ConversionFailed,
}

pub trait Stringify {
    fn to_string(&self) -> Result<String, PathBufToStringError>;
}

impl Stringify for PathBuf {
    fn to_string(&self) -> Result<String, PathBufToStringError> {
        Ok(self
            .to_str()
            .ok_or(PathBufToStringError::ConversionFailed)?
            .to_string())
    }
}
