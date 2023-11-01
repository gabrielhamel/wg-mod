use git2::{
    Branch, BranchType, FetchOptions, Remote, RemoteCallbacks, Repository,
};
use std::fs::create_dir_all;
use std::io;
use std::io::Write;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum GameSourcesError {
    #[error("Unable to create directory\n{0}")]
    CreateDirectoryError(io::Error),

    #[error("Git error occurred")]
    GitError(#[from] git2::Error),

    #[error("Unable to read branch name")]
    GitBranchError,
}

pub struct GameSources {
    repository: Repository,
}

fn fetch(remote: &mut Remote) -> Result<(), GameSourcesError> {
    let mut cb = RemoteCallbacks::new();
    cb.transfer_progress(|stats| {
        let download_progress = 100_f32 * stats.received_objects() as f32
            / stats.total_objects() as f32;
        let unzip_progress = 100_f32 * stats.indexed_deltas() as f32
            / stats.total_deltas() as f32;

        if stats.received_objects() != stats.total_objects() {
            print!("Fetching WoT sources ... {:.0}%\r", download_progress);
        } else {
            print!("Unpacking WoT sources ... {:.0}%\r", unzip_progress);
        }
        io::stdout().flush().ok();
        true
    });

    let mut fetch_options = FetchOptions::default();
    fetch_options.remote_callbacks(cb);
    remote.fetch(
        &["+refs/heads/*:refs/remotes/origin/*"],
        Some(&mut fetch_options),
        None,
    )?;

    Ok(())
}

fn is_initialized(path: &PathBuf) -> bool {
    path.exists()
}

fn get_repository(path: &PathBuf) -> Result<Repository, GameSourcesError> {
    let repository = if !is_initialized(&path) {
        create_dir_all(&path)
            .map_err(GameSourcesError::CreateDirectoryError)?;
        Repository::init(&path)?
    } else {
        Repository::open(&path)?
    };

    Ok(repository)
}

fn get_default_remote<'s>(
    path: &PathBuf, repository: &'s git2::Repository,
) -> Result<Remote<'s>, GameSourcesError> {
    let wot_src_remote_url = "https://github.com/IzeBerg/wot-src.git";
    let remote = if !is_initialized(path) {
        repository.remote("origin", wot_src_remote_url)?
    } else {
        repository.find_remote("origin")?
    };

    Ok(remote)
}

impl GameSources {
    pub fn new(path: &PathBuf) -> Result<Self, GameSourcesError> {
        let repository = get_repository(&path)?;
        let mut remote = get_default_remote(&path, &repository)?;

        fetch(&mut remote)?;

        Ok(GameSources {
            repository: get_repository(&path)?,
        })
    }

    pub fn list_branches(&self) -> Result<Vec<String>, GameSourcesError> {
        let branches_options = Some(BranchType::Remote);
        let it = self.repository.branches(branches_options)?;
        let mut branches: Vec<String> = vec![];

        let branches_result: Vec<Result<(Branch, BranchType), git2::Error>> =
            it.collect();

        for branch_result in branches_result {
            let (branch, _) = branch_result?;
            let branch_name = branch
                .name()?
                .ok_or(GameSourcesError::GitBranchError)?
                .to_string();
            let short_branch_name = branch_name.replace("origin/", "");
            branches.push(short_branch_name);
        }

        Ok(branches)
    }
}
