use std::path::PathBuf;
use std::vec::Vec;

use git2::*;
use regex::Regex;

use crate::error::*;
use crate::model::*;

pub struct Repo {
    repo: Repository,
}

impl Repo {
    pub fn new(path: PathBuf) -> Result<Self, CromError> {
        let repo = Repository::discover(path)?;
        return Ok(Repo { repo: repo });
    }

    pub fn get_tags(&self) -> Result<Vec<String>, CromError> {
        let tags = self.repo.tag_names(None)?;

        return Ok(tags.iter().map(|x| x.unwrap().to_string()).collect());
    }

    pub fn is_version_head(&self, version: &Version) -> Result<bool, CromError> {
        let tag = self.repo.find_reference(&format!("refs/tags/{}", version))?;
        let tag_commit = tag.peel_to_commit()?;

        let head = self.repo.head()?;
        let head_commit = head.peel_to_commit()?;

        debug!("Comparing tag {:?} to head {:?}", tag_commit, head_commit);

        if tag_commit.id() != head_commit.id() {
            return Ok(false);
        }

        return self.is_working_repo_clean();
    }

    pub fn is_working_repo_clean(&self) -> Result<bool, CromError> {
        let status = self.repo.statuses(Some(&mut StatusOptions::new()))?;
        return Ok(status.is_empty());
    }

    pub fn tag_version(&self, version: &Version, message: &str) -> Result<bool, CromError> {
        let head = self.repo.head()?.peel_to_commit()?;
        let sig = git2::Signature::now("crom", "cli@crom.tech")?;

        return match self.repo.tag(
            &format!("{}", version),
            head.as_object(),
            &sig,
            message,
            false,
        ) {
            Ok(_) => Ok(true),
            Err(e) => {
                return Err(CromError::GitError(e.to_string()));
            }
        };
    }

    pub fn get_head_sha(&self) -> Result<String, CromError> {
        let head = self.repo.head()?.peel_to_commit()?;
        let strs: Vec<String> = head
            .id()
            .as_bytes()
            .to_vec()
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect();
        return Ok(strs.join(""));
    }

    pub fn get_owner_repo_info(&self) -> Result<(String, String), CromError> {
        let config = self.repo.config()?;

        let remote = config.get_string("remote.origin.url")?;

        parse_remote(&remote)
    }
}

fn parse_remote(remote: &str) -> Result<(String, String), CromError> {
    let re =
        Regex::new("^(https://github.com/|git@github.com:)(?P<owner>.+?)/(?P<repo>.+?)(\\.git)?$")?;

    return match re.captures(remote) {
        Some(matches) => Ok((
            matches.name("owner").unwrap().as_str().to_string(),
            matches.name("repo").unwrap().as_str().to_string(),
        )),
        None => Err(CromError::GitRemoteUnkown(remote.to_string())),
    };
}

#[test]
fn test_parse_remote_https() {
    let https = parse_remote("https://github.com/ethankhall/crom");
    match https {
        Ok((owner, repo)) => {
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
        Ok((owner, repo)) => {
            assert_eq!("ethankhall", owner);
            assert_eq!("crom", repo);
        }
        Err(_) => assert!(false),
    };
}
