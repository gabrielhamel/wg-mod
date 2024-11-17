use crate::sdk::node::{Node, NodeError};
use crate::sdk::npm::NPM;
use crate::utils::command::command;
use std::path::PathBuf;
use std::process::Output;

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

    fn exec(&self, args: Vec<&str>) -> Result<Output, NodeError> {
        let node_exec_path = self.node_path.join("node");
        let executable =
            node_exec_path.to_str().ok_or(NodeError::FailedExecution)?;

        command(executable, args, vec![]).map_err(|e| {
            eprintln!("{}", e);
            NodeError::FailedExecution
        })
    }
}
