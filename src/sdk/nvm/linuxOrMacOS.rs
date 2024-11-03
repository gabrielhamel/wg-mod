use std::path::PathBuf;
use std::process::Command;
use clap::builder::Str;
use serde_json::json;
use crate::sdk::nvm::{create_nvm_directory, NVMError, NVM};
use crate::utils::command::command;
use crate::utils::convert_pathbuf_to_string::convert_pathbuf_to_string;
use crate::utils::downloader::download_file;
use crate::utils::Env;
use crate::utils::file_template::{write_template, TemplateError};

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
        println!("Install nvm ...");
        create_nvm_directory(&self.nvm_path)?;
        let downloaded_file_path = self.nvm_path.join("install.sh");
        let downloaded_file = convert_pathbuf_to_string(&downloaded_file_path);

        download_file("https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh", &downloaded_file).expect("");


        let mut command = Command::new("bash");
        command
            .arg(&downloaded_file)
            .env("NVM_DIR", &self.nvm_path);

        let _ = command.output().map_err(|_| NVMError::InstallError)?;

        self.create_executable(&self.nvm_path).map_err(|err| println!("{}", err)).expect("TODO: panic message");

        Ok(())
    }

    fn install_node(&self, destination: &PathBuf) -> Result<(), NVMError> {
        println!("Installing Node via nvm...");
        let nvm_bin_env = Env{
            key: "NVM_INC".to_string(),
            value: convert_pathbuf_to_string(destination).to_string()
        };

        self.exec(
            vec!["install", "node"],
            vec![]).map_err(|_| NVMError::InstallNodeError)?;

        Ok(())
    }

    fn exec(&self, args: Vec<&str>, env: Vec<Env>) -> Result<(), NVMError> {
        let executable_path = self.get_executable_path();
        let executable = convert_pathbuf_to_string(&executable_path);
        let mut mutable_args = args.clone();
        mutable_args.insert(0, executable);

        let nvm_dir_env = Env{
            key: "NVM_DIR".to_string(),
            value: convert_pathbuf_to_string(&self.nvm_path).to_string(),
        };
        let mut mutable_envs = env.clone();
        mutable_envs.push(nvm_dir_env);

        command("bash", mutable_args, mutable_envs).map_err(|_| NVMError::ExecError)?;

        Ok(())
    }
}