use std::fs::create_dir_all;
use std::path::PathBuf;
use crate::sdk::nvm::NVM;

#[derive(thiserror::Error, Debug)]

pub enum NodeError {
    #[error("Failed to create node directory")]
    CreateNodeDirectory,
    #[error("Failed to install Node")]
    InstallNode,
}

pub struct Node {
    node_path: PathBuf,
}
impl Node {
    pub fn new(node_path: PathBuf) -> Self {
        Self{node_path}
    }
    pub fn install(&self, nvm: &Box<dyn NVM>) -> Result<(), NodeError> {
        create_dir_all(self.node_path.to_path_buf()).map_err(|_| NodeError::CreateNodeDirectory)?;

        nvm.install_node(&self.node_path.to_path_buf()).map_err(|_| NodeError::InstallNode)?;

        Ok(())
    }
}