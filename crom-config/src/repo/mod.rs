use crate::version::*;

#[derive(Debug)]
pub enum RepoError {
    GitError(String),
    GitRemoteUnkown(String),
    RegexError(String)
}

pub enum RepoRemote {
    GitHub(String, String)
}

pub enum TagTarget {
    GitHub,
    Local
}

pub struct RepoDetails {
    pub known_versions: Vec<Version>,
    pub is_workspace_clean: bool,
    pub head_version: Option<Version>,
    pub head_ref: String,
    pub remote: RepoRemote,
}

impl RepoDetails {
    pub fn is_version_head(&self, version: &Version) -> bool {
        match &self.head_version { 
            None => false,
            Some(v) => version == v
        }
    }
}

pub mod git;