use std::path::PathBuf;
use std::vec::Vec;

use git2::Error as GitError;
use git2::*;
use regex::Regex;
use regex::Error as RegexError;

use super::*;
use crate::error::*;

type Result<T> = std::result::Result<T, RepoError>;

impl RepoDetails {
    pub fn new(path: &PathBuf, matcher: VersionMatcher) -> Result<RepoDetails> {
        let repo = Repository::discover(path)?;

        let tags = get_tags(&repo, matcher)?;
        let is_working_repo_clean = is_working_repo_clean(&repo)?;
        let head_ref = get_head_sha(&repo)?;
        let remote = get_owner_repo_info(&repo)?;

        let details = RepoDetails {
            known_versions: tags,
            is_workspace_clean: is_working_repo_clean,
            head_version: None,
            head_ref,
            remote,
            path: path.clone()
        };

        return Ok(details);
    }
}

pub fn tag_version(repo_details: &RepoDetails, version: &Version, message: &str) -> Result<()> {
    let repo = Repository::discover(repo_details.path.clone())?;

    let head = format!("{}", repo_details.head_ref);
    let sig = git2::Signature::now("crom", "cli@crom.tech")?;

    let head_obj = repo.find_object(Oid::from_str(&head)?, Some(ObjectType::Commit))?;

        return match repo.tag(
            &format!("{}", version),
            &head_obj,
            &sig,
            message,
            false,
        ) {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(RepoError::UnableToTagRepo(e.to_string()));
            }
        };
}

fn get_tags(repo: &Repository, matcher: VersionMatcher) -> Result<Vec<Version>> {
    let tags = repo.tag_names(None)?;
    Ok(tags.iter().map(|x| x.unwrap().to_string()).flat_map(|version| matcher.match_version(version)).collect())
}

pub fn is_working_repo_clean(repo: &Repository) -> Result<bool> {
    let status = repo.statuses(Some(&mut StatusOptions::new()))?;
    return Ok(status.is_empty());
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
    return Ok(strs.join(""));
}

fn get_owner_repo_info(repo: &Repository) -> Result<RepoRemote> {
    let config = repo.config()?;

    let remote = config.get_string("remote.origin.url")?;

    parse_remote(&remote)
}

fn parse_remote(remote: &str) -> Result<RepoRemote> {
    let re =
        Regex::new("^(https://github.com/|git@github.com:)(?P<owner>.+?)/(?P<repo>.+?)(\\.git)?$")?;

    return match re.captures(remote) {
        Some(matches) => {
            let owner = matches.name("owner").unwrap().as_str().to_string();
            let repo = matches.name("repo").unwrap().as_str().to_string();

            Ok(RepoRemote::GitHub(owner, repo))
        },
        None => Err(RepoError::GitRemoteUnkown(remote.to_string())),
    };
}

#[test]
fn test_parse_remote_https() {
    let https = parse_remote("https://github.com/ethankhall/crom");
    match https {
        Ok(RepoRemote::GitHub(owner, repo)) => {
            assert_eq!("ethankhall", owner);
            assert_eq!("crom", repo);
        }
        Err(_) => assert!(false),
    };
}

#[test]
fn test_parse_remote_git() {
    let https = parse_remote("git@github.com:ethankhall/crom.git");
    match https {
        Ok(RepoRemote::GitHub(owner, repo)) => {
            assert_eq!("ethankhall", owner);
            assert_eq!("crom", repo);
        }
        Err(_) => assert!(false),
    };
}

impl From<GitError> for RepoError {
    fn from(error: GitError) -> Self {
        RepoError::GitError(error.to_string())
    }
}

impl From<RegexError> for RepoError {
    fn from(error: RegexError) -> Self {
        RepoError::RegexError(error.to_string())
    }
}