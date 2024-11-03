pub mod as3;
pub mod conda;
pub mod game_sources;
pub mod node;
pub mod nvm;

type InstallResult = Result<(), String>;

pub trait Installable {
    fn is_installed(&self) -> bool;
    fn install(&self) -> InstallResult;
}
