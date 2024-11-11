use crate::sdk::node::Node;
use crate::sdk::nvm::{NVMError, NVM};
use crate::utils::Env;
use std::path::PathBuf;
use std::process::Output;

pub struct WindowsNVM {
    nvm_path: PathBuf,
}

impl From<PathBuf> for WindowsNVM {
    fn from(nvm_path: PathBuf) -> Self {
        Self { nvm_path }
    }
}

impl NVM for WindowsNVM {
    fn install(&self) -> Result<(), NVMError> {
        todo!()
    }

    fn install_node(&self) -> Result<(), NVMError> {
        todo!()
    }

    fn exec(
        &self, args: Vec<&str>, envs: Vec<Env>,
    ) -> Result<Output, NVMError> {
        todo!()
    }

    fn get_node(&self) -> Result<Node, NVMError> {
        todo!()
    }
}
