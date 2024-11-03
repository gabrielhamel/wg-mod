use std::path::PathBuf;
use crate::sdk::nvm::{ NVMError, NVM};
use crate::utils::Env;

pub struct WindowsNVM {
    nvm_path: PathBuf,
}

impl From<PathBuf> for WindowsNVM {
    fn from(nvm_path: PathBuf) -> Self {Self{nvm_path}}
}

impl NVM for WindowsNVM {

    fn install(&self) -> Result<(), NVMError> {
        todo!()
    }

    fn install_node(&self, destination: &PathBuf) -> Result<(), NVMError> {
        todo!()
    }

    fn exec(&self, args: Vec<&str>, env: Vec<Env>) -> Result<(), NVMError> {
        todo!()
    }
}