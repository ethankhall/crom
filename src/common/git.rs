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

    pub fn tag_version(&self, version: &Version) -> Result<bool, CromError> {
        let head = self.repo.head()?.peel_to_commit()?;
        let sig = git2::Signature::now("crom", "cli@crom.tech")?;
        let message = format!("Crom is creating a version {}.", version);
        
        return match self.repo.tag(&format!("{}", version), head.as_object(), &sig, &message, false) {
            Ok(_) => Ok(true),
            Err(e) => {
                return Err(CromError::GitError(e.to_string()));
            }
        };
    }
}