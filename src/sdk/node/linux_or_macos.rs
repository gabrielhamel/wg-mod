use crate::sdk::node::Node;
use crate::sdk::npm::NPM;
use std::path::PathBuf;

pub struct LinuxOrMacNode {
    node_path: PathBuf,
}

impl From<PathBuf> for LinuxOrMacNode {
    fn from(node_path: PathBuf) -> Self {
        Self { node_path }
    }
}

impl Node for LinuxOrMacNode {
    fn get_npm(&self) -> NPM {
        NPM::from(self.node_path.join("bin").join("npm"))
    }
}
