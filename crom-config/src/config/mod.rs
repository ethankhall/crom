use std::path::PathBuf;
use std::rc::Rc;

mod file;
mod parser;

use crate::version::*;
use crate::Project;
use crate::repo::*;
use crate::error::SharedError;

#[derive(Debug)]
pub enum ConfigError {
    UnableToFindConfig(PathBuf),
    IoError(String),
    ProjectNameNotDefined(String)
}

pub struct ParsedProjectConfig {
    pub project_config: Rc<file::ProjectConfig>,
    pub project_path: PathBuf,
    pub repo_details: RepoDetails
}

impl Project for ParsedProjectConfig {
    fn find_latest_version(&self, modification: VersionModification) -> Version {
        match self.repo_details.known_versions.last() {
            Some(v) => get_latest_version(&self.repo_details, v.clone(), modification),
            None => {
                VersionMatcher::new(&self.project_config.pattern).build_default_version()   
            }
        }
    }

    fn update_versions(&self, version: &Version) -> Result<(), SharedError> {
        return Ok(());
    }

    fn publish(&self, version: &Version, names: Vec<String>) -> Result<(), SharedError> {
        return Ok(());
    }

    fn tag_version(&self, version: &Version, targets: Vec<TagTarget>, allow_dirty_repo: bool) -> Result<(), SharedError> {
        return Ok(());
    }
}

fn get_latest_version(
    repo_details: &RepoDetails,
    latest_version: Version,
    modification: VersionModification,
) -> Version {
    let return_version = match modification {
        VersionModification::NoneOrSnapshot => match repo_details.is_version_head(&latest_version) {
            true => latest_version,
            false => latest_version.next_snapshot()
        },
        VersionModification::None => match repo_details.is_version_head(&latest_version) {
            true => latest_version,
            false => latest_version.self_without_snapshot()
        },
        VersionModification::NoneOrNext => match repo_details.is_version_head(&latest_version) {
            true => latest_version,
            false => latest_version.next_version()
        },
        VersionModification::OneMore => latest_version.next_version(),
    };

    return return_version;
}
