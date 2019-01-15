use std::path::PathBuf;

use crate::error::*;
use crate::version::*;

pub enum RepoRemote {
    GitHub(String, String),
}

pub enum TagTarget {
    GitHub,
    Local,
}

pub struct RepoDetails {
    pub known_versions: Vec<Version>,
    pub is_workspace_clean: bool,
    pub head_version: Option<Version>,
    pub head_ref: String,
    pub remote: RepoRemote,
    pub path: PathBuf,
}

impl RepoDetails {
    pub fn is_version_head(&self, version: &Version) -> bool {
        match &self.head_version {
            None => false,
            Some(v) => version == v,
        }
    }
}

pub fn tag_repo(
    details: &RepoDetails,
    version: &Version,
    message: &str,
    targets: Vec<TagTarget>,
) -> Result<i32, ErrorContainer> {
    for target in targets {
        match target {
            TagTarget::GitHub => github::tag_version(details, version, message)?,
            TagTarget::Local => git::tag_version(details, version, message)?,
        };
    }

    return Ok(1);
}

pub mod git;
pub mod github;
