use crate::version::*;

pub enum RepoError {
    GitError(String),
    GitRemoteUnkown(String),
    RegexError(String)
}

pub enum RepoRemote {
    GitHub(String, String)
}

pub struct RepoDetails {
    known_versions: Vec<Version>,
    is_workspace_clean: bool,
    head_version: Option<Version>,
    head_ref: String,
    remote: RepoRemote,
}

pub mod git;