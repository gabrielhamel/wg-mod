use crate::utils::convert_to_absolute_path::convert_to_absolute_path;
use fs_extra::dir::get_dir_content;
use git2::{
    Branch, BranchType, FetchOptions, Remote, RemoteCallbacks, Repository,
};
use inquire::Select;
use std::fs::create_dir_all;
use std::io;
use std::io::Write;
use std::path::{PathBuf, MAIN_SEPARATOR};

#[derive(thiserror::Error, Debug)]
pub enum GameSourcesError {
    #[error("Unable to create directory\n{0}")]
    CreateDirectoryError(io::Error),

    #[error("Git error occurred")]
    GitError(#[from] git2::Error),

    #[error("Unable to read branch name")]
    GitBranchError,

    #[error("An error occurred during user prompting")]
    CliPromptError(#[from] inquire::InquireError),

    #[error("Invalid path")]
    PathError,

    #[error("Unable to walk in the directory")]
    FilesystemError(#[from] fs_extra::error::Error),
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

fn get_repository(
    path: &PathBuf, need_to_be_initialized: bool,
) -> Result<Repository, GameSourcesError> {
    let repository = if need_to_be_initialized {
        create_dir_all(&path)
            .map_err(GameSourcesError::CreateDirectoryError)?;
        Repository::init(&path)?
    } else {
        Repository::open(&path)?
    };

    Ok(repository)
}

fn get_default_remote(
    repository: &Repository, need_to_be_initialized: bool,
) -> Result<Remote, GameSourcesError> {
    let wot_src_remote_url = "https://github.com/IzeBerg/wot-src.git";
    let remote = if need_to_be_initialized {
        repository.remote("origin", wot_src_remote_url)?
    } else {
        repository.find_remote("origin")?
    };

    Ok(remote)
}

impl GameSources {
    pub fn load(path: &PathBuf) -> Result<Self, GameSourcesError> {
        let already_exists = path.exists();

        let repository = get_repository(&path, !already_exists)?;
        let mut remote = get_default_remote(&repository, !already_exists)?;

        fetch(&mut remote)?;

        let game_sources = GameSources {
            repository: get_repository(&path, false)?,
        };

        if !already_exists {
            game_sources.prompt_channel()?;
        }

        Ok(game_sources)
    }

    fn list_channels(&self) -> Result<Vec<String>, GameSourcesError> {
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

    pub fn prompt_channel(&self) -> Result<(), GameSourcesError> {
        let channels_available = self.list_channels()?;
        let channel_selected = Select::new(
            "Select a World of Tanks development channel:",
            channels_available,
        )
        .prompt()?;

        self.switch_channel(&channel_selected)?;

        Ok(())
    }

    fn switch_channel(
        &self, channel_name: &str,
    ) -> Result<(), GameSourcesError> {
        let branch_name = format!("origin/{channel_name}");
        let (object, reference) = self.repository.revparse_ext(&branch_name)?;

        self.repository.checkout_tree(&object, None)?;
        match reference {
            | Some(reference) => self.repository.set_head(
                reference.name().ok_or(GameSourcesError::GitBranchError)?,
            ),
            | None => self.repository.set_head_detached(object.id()),
        }?;

        Ok(())
    }

    pub fn get_channel(&self) -> Result<String, GameSourcesError> {
        let current_commit = self.repository.head()?.peel_to_commit()?;
        let references = self.repository.references()?;

        for reference_pack in references {
            if let Ok(reference) = reference_pack {
                if let Some(target) = reference.target() {
                    if target == current_commit.id()
                        && reference.name().is_some()
                    {
                        return Ok(reference
                            .name()
                            .ok_or(GameSourcesError::GitBranchError)?
                            .to_string()
                            .replace("refs/remotes/origin/", ""));
                    }
                }
            }
        }

        Err(GameSourcesError::GitBranchError)
    }

    fn list_directory_paths(
        &self, path: &PathBuf,
    ) -> Result<Vec<String>, GameSourcesError> {
        let directory_content = get_dir_content(path)?;
        let folders = directory_content.directories;

        let cleaned_path = folders
            .iter()
            .map(|p| {
                convert_to_absolute_path(&PathBuf::from(p))
                    .map_err(|_| GameSourcesError::PathError)
            })
            .collect::<Result<Vec<String>, GameSourcesError>>()?;

        Ok(cleaned_path)
    }

    fn is_game_python_root_module(&self, folder: &String) -> bool {
        folder.ends_with(&format!("scripts{}common", MAIN_SEPARATOR))
            || folder.ends_with(&format!("scripts{}client", MAIN_SEPARATOR))
            || folder
                .ends_with(&format!("scripts{}client_common", MAIN_SEPARATOR))
    }

    pub fn list_python_root_modules(
        &self,
    ) -> Result<Vec<String>, GameSourcesError> {
        let sources_path = self
            .repository
            .path()
            .parent()
            .ok_or(GameSourcesError::PathError)?;
        let python_sources_path = sources_path.join("sources/res");
        let sub_paths = self.list_directory_paths(&python_sources_path)?;

        Ok(sub_paths
            .iter()
            .filter_map(|folder| {
                if self.is_game_python_root_module(folder) {
                    Some(folder.clone())
                } else {
                    None
                }
            })
            .collect())
    }
}
