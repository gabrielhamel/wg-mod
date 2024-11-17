use crate::sdk::node::{Node, NodeError};
use crate::sdk::npm::NPM;
use crate::utils::command::command;
use std::path::PathBuf;
use std::process::Output;

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

    fn exec(&self, args: Vec<&str>) -> Result<Output, NodeError> {
        let binaries_path = self.node_path.join("bin");
        let node_exec_path = binaries_path.join("node");

        let executable =
            node_exec_path.to_str().ok_or(NodeError::FailedExecution)?;
        command(executable, args, vec![])
            .map_err(|_| NodeError::FailedExecution)
    }
}
