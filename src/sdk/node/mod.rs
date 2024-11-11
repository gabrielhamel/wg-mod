use crate::sdk::npm::NPM;
use std::path::PathBuf;

pub struct Node {
    node_path: PathBuf,
}
impl Node {
    pub fn new(node_path: PathBuf) -> Self {
        Self { node_path }
    }

    pub fn get_npm(&self) -> NPM {
        NPM::new(self.node_path.join("bin").join("npm"))
    }
}
