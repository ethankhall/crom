use log::{debug};
use std::vec::Vec;

use error_chain::bail;
use git2::*;
use regex::Regex;

use crate::errors::{Error as CromError, ErrorKind};
use crate::version::{Version, VersionMatcher};

type Result<T> = std::result::Result<T, CromError>;

#[derive(Debug)]
pub enum RepoRemote {
    GitHub { owner: String, repo: String },
}

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

pub fn get_head_sha(repo: &Repository) -> Result<String> {
    let head = repo.head()?.peel_to_commit()?;
    let strs: Vec<String> = head
        .id()
        .as_bytes()
        .to_vec()
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect();
    Ok(strs.join(""))
}

pub fn get_owner_repo_info(repo: &Repository) -> Result<RepoRemote> {
    let config = repo.config()?;

    let remote = config.get_string("remote.origin.url")?;

    parse_remote(&remote)
}

fn parse_remote(remote: &str) -> Result<RepoRemote> {
    let re =
        Regex::new("^(https://github.com/|git@github.com:)(?P<owner>.+?)/(?P<repo>.+?)(\\.git)?$")?;

    match re.captures(remote) {
        Some(matches) => {
            let owner = matches.name("owner").unwrap().as_str().to_string();
            let repo = matches.name("repo").unwrap().as_str().to_string();

            Ok(RepoRemote::GitHub { owner, repo })
        }
        None => bail!(ErrorKind::UnknownGitRemotes(remote.to_string())),
    }
}

#[test]
fn test_parse_remote_https() {
    let https = parse_remote("https://github.com/ethankhall/crom");
    match https {
        Ok(RepoRemote::GitHub { owner, repo }) => {
            assert_eq!("ethankhall", owner);
            assert_eq!("crom", repo);
        }
        Err(_) => unreachable!(),
    };
}

#[test]
fn test_parse_remote_git() {
    let https = parse_remote("git@github.com:ethankhall/crom.git");
    match https {
        Ok(RepoRemote::GitHub { owner, repo }) => {
            assert_eq!("ethankhall", owner);
            assert_eq!("crom", repo);
        }
        Err(_) => unreachable!(),
    };
}
