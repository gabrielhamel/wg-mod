use crate::sdk::node::linux_or_macos::NodeLinuxOrMac;
use crate::sdk::node::Node;
use crate::sdk::nvm::{NVMError, NVM};
use crate::utils::command::command;
use crate::utils::convert_pathbuf_to_string::Stringify;
use crate::utils::downloader::download_file;
use crate::utils::file_template::{write_template, TemplateError};
use crate::utils::Env;
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

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

    fn create_executable(&self, path: &PathBuf) -> Result<(), TemplateError> {
        write_template(
            &path,
            self.get_executable_name().as_str(),
            "[ -s \"$NVM_DIR/nvm.sh\" ] && \\. \"$NVM_DIR/nvm.sh\"  # This loads nvm
[ -s \"$NVM_DIR/bash_completion\" ] && \\. \"$NVM_DIR/bash_completion\"  # This loads nvm bash_completion
nvm $@",
                       &json!({}))
    }
}

impl NVM for LinuxOrMacOsNVM {
    fn install(&self) -> Result<(), NVMError> {
        let downloaded_file_path = self.nvm_path.join("install.sh");
        let downloaded_file = downloaded_file_path.to_string()?;

        download_file(
            "https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh",
            &downloaded_file,
        )
        .map_err(|_| NVMError::DownloadError)?;

        let mut command = Command::new("bash");
        command.arg(&downloaded_file).env("NVM_DIR", &self.nvm_path);

        let _ = command.output().map_err(|_| NVMError::InstallError)?;

        self.create_executable(&self.nvm_path)
            .map_err(|err| println!("{}", err))
            .expect("TODO: panic message");

        fs::remove_file(&downloaded_file_path)
            .map_err(|_| NVMError::InstallError)?;

        Ok(())
    }

    fn install_node(&self) -> Result<(), NVMError> {
        println!("Installing Node via nvm...");

        self.exec(vec!["install", "node"], vec![])?;

        self.exec(vec!["current"], vec![])?;

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

        Ok(Box::new(NodeLinuxOrMac::from(current_node_path)))
    }

    fn is_installed(&self) -> bool {
        self.nvm_path.exists()
    }
}
