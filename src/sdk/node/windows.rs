use crate::sdk::node::Node;
use crate::sdk::npm::NPM;
use std::path::PathBuf;

pub struct WindowsNode {
    node_path: PathBuf,
}

impl From<PathBuf> for WindowsNode {
    fn from(node_path: PathBuf) -> Self {
        Self { node_path }
    }
}

impl Node for WindowsNode {
    fn get_npm(&self) -> NPM {
        NPM::from(self.node_path.join("npm.cmd").to_path_buf())
    }
}
