use crate::sdk::node::Node;
use crate::sdk::npm::NPM;
use std::path::PathBuf;

pub struct NodeLinuxOrMac {
    node_path: PathBuf,
}

impl From<PathBuf> for NodeLinuxOrMac {
    fn from(node_path: PathBuf) -> Self {
        Self { node_path }
    }
}

impl Node for NodeLinuxOrMac {
    fn get_npm(&self) -> NPM {
        NPM::from(self.node_path.join("bin").join("npm"))
    }
}
