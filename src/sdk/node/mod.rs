pub mod linux_or_macos;
pub mod windows;

use crate::sdk::npm::NPM;
use std::process::Output;
use std::result;
use std::string::FromUtf8Error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unable to decode output of the command")]
    DecodeOutputError(#[from] FromUtf8Error),

    #[error("Unable to run command")]
    FailedExecution,
}

type Result<T> = result::Result<T, Error>;

pub trait Node {
    fn get_npm(&self) -> NPM;

    fn exec(&self, args: Vec<&str>) -> Result<Output>;

    fn version(&self) -> Result<String> {
        let out = self.exec(vec!["--version"])?;

        Ok(String::from_utf8(out.stdout)?.trim().to_string())
    }
}

#[cfg(test)]
pub mod tests {
    use crate::sdk::nvm::load_nvm;
    use regex::Regex;
    use tempfile::tempdir;

    #[test]
    fn install_node() {
        let tmp_dir = tempdir().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();

        let nvm_path = tmp_dir_path.join("nvm");

        let nvm = load_nvm(&nvm_path).unwrap();
        let node = nvm.get_node().unwrap();

        let version = node.version().unwrap();

        let semantic_version_pattern = Regex::new("^v([0-9]+)\\.([0-9]+)\\.([0-9]+)(?:-([0-9A-Za-z-]+(?:\\.[0-9A-Za-z-]+)*))?(?:\\+[0-9A-Za-z-]+)?$").unwrap();
        assert!(semantic_version_pattern.is_match(&version));
    }
}
