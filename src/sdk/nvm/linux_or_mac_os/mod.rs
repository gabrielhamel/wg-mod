use crate::new::template::create_nvm_executable;
use crate::sdk::node::linux_or_macos::LinuxOrMacNode;
use crate::sdk::node::Node;
use crate::sdk::nvm::linux_or_mac_os::install::install_nvm_sdk;
use crate::sdk::nvm::{NVMError, NVM};
use crate::sdk::{InstallResult, Installable};
use crate::utils::command::command;
use crate::utils::convert_pathbuf_to_string::Stringify;
use crate::utils::Env;
use std::path::PathBuf;
use std::process::Output;

mod install;

pub struct LinuxOrMacOsNVM {
    nvm_path: PathBuf,
}

impl From<PathBuf> for LinuxOrMacOsNVM {
    fn from(nvm_path: PathBuf) -> Self {
        Self { nvm_path }
    }
}

impl LinuxOrMacOsNVM {
    fn get_executable_name(&self) -> String {
        "wg-mod-nvm.sh".to_string()
    }

    fn get_executable_path(&self) -> PathBuf {
        self.nvm_path.join(self.get_executable_name())
    }

    fn prepare(&self) -> Result<(), NVMError> {
        install_nvm_sdk(&self.nvm_path)?;
        create_nvm_executable(
            &self.nvm_path,
            self.get_executable_name().as_str(),
        )
        .map_err(|_| NVMError::InstallError)?;

        Ok(())
    }
}

impl Installable for LinuxOrMacOsNVM {
    fn is_installed(&self) -> bool {
        self.nvm_path.exists()
    }

    fn install(&self) -> InstallResult {
        if self.is_installed() {
            Err("NVM already installed".into())
        } else {
            self.prepare().map_err(|err| err.to_string())
        }
    }
}

impl NVM for LinuxOrMacOsNVM {
    fn install_node(&self) -> Result<(), NVMError> {
        println!("Installing Node via nvm...");

        self.exec(vec!["install", "node"], vec![])?;

        Ok(())
    }

    fn exec(
        &self, args: Vec<&str>, envs: Vec<Env>,
    ) -> Result<Output, NVMError> {
        let executable_path = self.get_executable_path();
        let executable = &executable_path.to_string()?;
        let mut mutable_args = args.clone();
        mutable_args.insert(0, executable);

        let nvm_dir_env = Env {
            key: "NVM_DIR".to_string(),
            value: self.nvm_path.to_string()?,
        };
        let mut mutable_envs = envs.clone();
        mutable_envs.push(nvm_dir_env);

        command("bash", mutable_args, mutable_envs)
            .map_err(|_| NVMError::ExecError)
    }

    fn get_node(&self) -> Result<Box<dyn Node>, NVMError> {
        let node_path = self.nvm_path.join("versions").join("node");

        if !node_path.exists() {
            self.install_node()?;
        }

        let current_version = self.nvm_current_version()?;
        let current_node_path = node_path.join(current_version);

        Ok(Box::new(LinuxOrMacNode::from(current_node_path)))
    }
}
