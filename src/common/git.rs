use std::path::PathBuf;
use std::vec::Vec;

use git2::*;
use crate::error::*;

pub struct Repo {
    repo: Repository
}

impl Repo {
    pub fn new(path: PathBuf) -> Result<Self, CromError> {
        let repo = Repository::discover(path)?;
        return Ok(Repo { repo: repo });
    }

    pub fn get_tags(self) -> Result<Vec<String>, CromError> {
        let tags = self.repo.tag_names(None)?;

        return Ok(tags.iter().map(|x| x.unwrap().to_string()).collect());
    }
}