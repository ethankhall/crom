use git2::*;
use log::debug;
use std::vec::Vec;
use error_chain::bail;
use crate::errors::ErrorKind;
use std::path::PathBuf;

use crate::errors::Error as CromError;
use crate::version::{Version, VersionMatcher};

type Result<T> = std::result::Result<T, CromError>;

pub fn get_tags(repo: &Repository, matcher: &VersionMatcher) -> Result<Vec<Version>> {
    let tags = repo.tag_names(None)?;
    let mut tags: Vec<Version> = tags
        .iter()
        .map(|x| x.unwrap().to_string())
        .flat_map(|version| matcher.match_version(version))
        .collect();

    tags.sort();

    debug!("Tags discovered: {:?}", tags);
    Ok(tags)
}

pub fn is_working_repo_clean(repo: &Repository) -> Result<bool> {
    let mut options = StatusOptions::new();
    let options = options.include_unmodified(true).include_untracked(false);
    let statuses = repo.statuses(Some(options))?;
    Ok(statuses.is_empty())
}

pub fn get_head_sha(location: PathBuf, repo: &Repository) -> Result<String> {
    let head = match repo.head()?.target() {
        Some(head) => head,
        None => bail!(ErrorKind::UnknownGitHead(location))
    };

    let head = repo.find_commit(head)?;

    let strs: Vec<String> = head
        .id()
        .as_bytes()
        .to_vec()
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect();
    Ok(strs.join(""))
}
