use std::process::Command;
use crate::utils::command::CommandError::CommandFailed;
use crate::utils::Env;

#[derive(thiserror::Error, Debug)]
pub enum CommandError {
    #[error("Failed to execute command")]
    CommandFailed
}

pub fn command(command: &str, args: Vec<&str>, env: Vec<Env>) -> Result<(), CommandError> {
    let mut command = Command::new("bash");
    command
        .args(args);

    env.iter().for_each(|env| { command.env(&env.key, &env.value); });

    let test = command.output().map_err(|_| CommandFailed)?;

    println!("out \n {}", String::from_utf8_lossy(&test.stdout));
    println!("err \n {}", String::from_utf8_lossy(&test.stderr));

    Ok(())
}