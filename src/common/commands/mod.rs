pub mod get;
pub mod init;
pub mod exec;

use crate::error::*;
use crate::model::*;
use crate::config::*;
use crate::git::*;

enum VersionModification {
    NoneOrSnapshot,
    OneMore
}

fn get_latest_version(repo: &Repo, project_config: &ProjectConfig, modification: VersionModification) -> Result<Version, CromError> {
    let versions = project_config.get_current_versions(&repo)?;

    let latest_version = match versions.last() {
        Some(v) => v.clone(),
        None => project_config.build_default_version()?
    };

    let return_version = match modification {
        VersionModification::NoneOrSnapshot => {
            match repo.is_version_head(&latest_version) {
                Ok(true) => latest_version,
                Ok(false) => latest_version.next_snapshot(),
                Err(msg) => { 
                    debug!("Unable to compair version to HEAD: {:?}", msg);
                    latest_version
                }
            }
        },
        VersionModification::OneMore => latest_version.next_version()
    };

    return Ok(return_version);
}