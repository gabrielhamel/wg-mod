use crate::new::template::template_nvm_comfig;
use crate::sdk::node::windows::NodeWindows;
use crate::sdk::node::Node;
use crate::sdk::nvm::{create_nvm_directory, NVMError, NVM};
use crate::utils::command::command;
use crate::utils::convert_pathbuf_to_string::Stringify;
use crate::utils::downloader::download_file;
use crate::utils::Env;
use std::fs;
use std::path::PathBuf;
use std::process::Output;
use zip_extensions::zip_extract;

pub struct WindowsNVM {
    nvm_path: PathBuf,
}

impl WindowsNVM {
    fn get_executable_path(&self) -> PathBuf {
        self.nvm_path.join("nvm.exe")
    }
}

impl From<PathBuf> for WindowsNVM {
    fn from(nvm_path: PathBuf) -> Self {
        Self { nvm_path }
    }
}

impl NVM for WindowsNVM {
    fn install(&self) -> Result<(), NVMError> {
        create_nvm_directory(&self.nvm_path)?;
        let downloaded_file_path = self.nvm_path.join("nvm.zip");
        let downloaded_file = downloaded_file_path.to_string()?;

        // not in 1.1.12 because issue in it: https://github.com/coreybutler/nvm-windows/issues/1068
        download_file(
            "https://github.com/coreybutler/nvm-windows/releases/download/1.1.11/nvm-noinstall.zip",
            &downloaded_file,
        )
            .map_err(|_| NVMError::DownloadError)?;

        zip_extract(&downloaded_file_path, &self.nvm_path)
            .map_err(|_| NVMError::InstallError)?;

        fs::remove_file(&downloaded_file_path)
            .map_err(|_| NVMError::InstallError)?;

        template_nvm_comfig(&self.nvm_path)
            .map_err(|_| NVMError::InstallError)?;

        Ok(())
    }

    fn is_installed(&self) -> bool {
        self.nvm_path.exists()
    }

    fn install_node(&self) -> Result<(), NVMError> {
        let args = vec!["install", "latest"];
        let envs = vec![];
        let _ = self
            .exec(args, envs)
            .map_err(|_| NVMError::InstallNodeError);

        self.nvm_use("latest")?;

        Ok(())
    }

    fn exec(
        &self, args: Vec<&str>, envs: Vec<Env>,
    ) -> Result<Output, NVMError> {
        let executable = self.get_executable_path();
        let executable_str = executable.to_str().ok_or(NVMError::ExecError)?;

        let mut mutable_args = args.clone();
        mutable_args.insert(0, executable_str);

        let nvm_dir_env = Env {
            key: "NVM_HOME".to_string(),
            value: self.nvm_path.to_string()?,
        };
        let nvm_simlink = Env {
            key: "NVM_SYMLINK".to_string(),
            value: self.nvm_path.join("nodejs").to_string()?,
        };

        let mut mutable_envs = envs.clone();
        mutable_envs.push(nvm_dir_env);
        mutable_envs.push(nvm_simlink);

        command("powershell.exe", mutable_args, mutable_envs)
            .map_err(|_| NVMError::ExecError)
    }

    fn get_node(&self) -> Result<Box<dyn Node>, NVMError> {
        let node_path = self.nvm_path.join("nodejs");

        // if !node_path.exists() {
        self.install_node()?;
        // }

        Ok(Box::new(NodeWindows::from(node_path)))
    }
}
