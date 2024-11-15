use crate::sdk::node::windows::WindowsNode;
use crate::sdk::node::Node;
use crate::sdk::nvm::windows::install::install_nvm_windows;
use crate::sdk::nvm::{NVMError, NVM};
use crate::sdk::{InstallResult, Installable};
use crate::utils::command::command;
use crate::utils::convert_pathbuf_to_string::Stringify;
use crate::utils::Env;
use std::path::PathBuf;
use std::process::Output;

mod install;

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

impl Installable for WindowsNVM {
    fn is_installed(&self) -> bool {
        self.nvm_path.exists()
    }

    fn install(&self) -> InstallResult {
        if self.is_installed() {
            Err("NVM already installed".into())
        } else {
            install_nvm_windows(&self.nvm_path).map_err(|err| err.to_string())
        }
    }
}

impl NVM for WindowsNVM {
    fn install_node(&self) -> Result<(), NVMError> {
        println!("Installing Node via nvm...");

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

        if !node_path.exists() {
            self.install_node()?;
        }

        Ok(Box::new(WindowsNode::from(node_path)))
    }
}
