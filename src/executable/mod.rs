#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to execute command: {0}")]
    ExecutionFailed(String),
}

struct Output {
    stdout: String,
    stderr: String,
    status_code: i32,
}

pub trait Executable {
    fn exec(&self, args: Vec<&str>) -> Result<Output, Error>;
}
