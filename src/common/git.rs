use std::path::PathBuf;
use std::vec::Vec;

use git2::*;

use crate::error::*;
use crate::model::*;

pub struct Repo {
    repo: Repository
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

        if tag_commit.id() != head_commit.id() {
            return Ok(false);
        }

        let status = self.repo.statuses(Some(&mut StatusOptions::new()))?;
        return Ok(status.is_empty());
    }
}