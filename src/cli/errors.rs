#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("This command isn't implemented")]
    CommandNotImplemented,

    #[error("No command provided, refer to the help section")]
    NoCommandProvided,

    #[error("Error occured during the command run")]
    CommandExecutionError,
}
