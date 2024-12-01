use std::path::PathBuf;

pub struct GameClient {
    pub path: PathBuf,
}

impl From<&PathBuf> for GameClient {
    fn from(path: &PathBuf) -> Self {
        GameClient { path: path.clone() }
    }
}
