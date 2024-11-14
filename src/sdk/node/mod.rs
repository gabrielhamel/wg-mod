use crate::sdk::npm::{NPMError, NPM};
pub mod linux_or_macos;

pub mod windows;

#[derive(thiserror::Error, Debug)]
pub enum NodeError {
    #[error("NPM error")]
    NPMError(#[from] NPMError),
}

pub trait Node {
    fn get_npm(&self) -> NPM;
}
