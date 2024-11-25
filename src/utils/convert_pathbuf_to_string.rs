use std::path::PathBuf;
use std::result;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Conversion failed")]
    ConversionFailed,
}

type Result<T> = result::Result<T, Error>;

pub trait Stringify {
    fn to_string(&self) -> Result<String>;
}

impl Stringify for PathBuf {
    fn to_string(&self) -> Result<String> {
        Ok(self.to_str().ok_or(Error::ConversionFailed)?.to_string())
    }
}
