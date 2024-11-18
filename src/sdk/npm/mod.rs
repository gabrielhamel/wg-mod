use crate::utils::command::command;
use crate::utils::convert_pathbuf_to_string::Stringify;
use crate::utils::Env;
use std::path::PathBuf;
use std::process::Output;

#[derive(thiserror::Error, Debug)]
pub enum NPMError {
    #[error("Execution failed")]
    FailedExecution,

    #[error("Get node bin directory")]
    GetBinDirectoryError,

    #[error("Unable to install package")]
    InstallPackageFailed(String),
}

pub struct NPM {
    npm_bin: PathBuf,
}

impl From<PathBuf> for NPM {
    fn from(npm_bin: PathBuf) -> Self {
        Self { npm_bin }
    }
}

impl NPM {
    fn exec(&self, args: Vec<&str>) -> Result<Output, NPMError> {
        let executable = self.npm_bin.as_os_str();

        let env = vec![
            (Env {
                key: "PATH".to_string(),
                value: self
                    .get_bin_directory()?
                    .to_string()
                    .map_err(|_| NPMError::GetBinDirectoryError)?,
            }),
        ];

        command(executable, args, env).map_err(|_| NPMError::FailedExecution)
    }

    pub fn is_package_installed(&self, name: &str) -> Result<bool, NPMError> {
        let result = self
            .exec(vec!["list", "-g", name])
            .map_err(|e| NPMError::InstallPackageFailed(e.to_string()))?;

        Ok(result.status.success())
    }

    pub fn get_bin_directory(&self) -> Result<PathBuf, NPMError> {
        self.npm_bin
            .parent()
            .ok_or(NPMError::GetBinDirectoryError)
            .and_then(|res| Ok(PathBuf::from(res)))
    }

    pub fn install_package(&self, name: &str) -> Result<(), NPMError> {
        println!("Installing {}...", name);

        let result = self
            .exec(vec!["install", "-g", name])
            .map_err(|e| NPMError::InstallPackageFailed(e.to_string()))?;

        if result.status.success() {
            return Ok(());
        }

        let stdout = String::from_utf8_lossy(&result.stdout);
        let stderr = String::from_utf8_lossy(&result.stderr);

        Err(NPMError::InstallPackageFailed(format!(
            "{}\n{}",
            stdout, stderr
        )))
    }

    pub fn version(&self) -> Result<String, NPMError> {
        let out = self.exec(vec!["--version"])?;

        Ok(String::from_utf8(out.stdout)
            .map_err(|_| NPMError::FailedExecution)?
            .trim()
            .to_string())
    }
}

#[cfg(test)]
pub mod tests {
    use crate::sdk::nvm::load_nvm;
    use regex::Regex;
    use tempfile::tempdir;

    #[test]
    fn install_npm() {
        let tmp_dir = tempdir().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let nvm_path = tmp_dir_path.join("nvm");

        let nvm = load_nvm(&nvm_path).unwrap();
        let node = nvm.get_node().unwrap();
        let npm = node.get_npm();

        let version = npm.version().unwrap();

        let semantic_version_pattern = Regex::new("^([0-9]+)\\.([0-9]+)\\.([0-9]+)(?:-([0-9A-Za-z-]+(?:\\.[0-9A-Za-z-]+)*))?(?:\\+[0-9A-Za-z-]+)?$").unwrap();
        assert!(semantic_version_pattern.is_match(&version));
    }
}
