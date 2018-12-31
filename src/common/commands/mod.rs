pub mod exec;
pub mod get;
pub mod init;

use crate::config::*;
use crate::error::*;
use crate::git::*;
use crate::model::*;

enum VersionModification {
    NoneOrSnapshot,
    None,
    NoneOrNext,
    OneMore,
}

fn get_latest_version(
    repo: &Repo,
    project_config: &ProjectConfig,
    modification: VersionModification,
) -> Result<Version, CromError> {
    let versions = project_config.get_current_versions(&repo)?;

    let latest_version = match versions.last() {
        Some(v) => v.clone(),
        None => project_config.build_default_version()?,
    };

    let return_version = match modification {
        VersionModification::NoneOrSnapshot => match repo.is_version_head(&latest_version) {
            Ok(true) => latest_version,
            Ok(false) => latest_version.next_snapshot(),
            Err(msg) => {
                debug!("Unable to compair version to HEAD: {:?}", msg);
                latest_version
            }
        },
        VersionModification::None => match repo.is_version_head(&latest_version) {
            Ok(true) => latest_version,
            Ok(false) => latest_version.self_without_snapshot(),
            Err(msg) => {
                debug!("Unable to compair version to HEAD: {:?}", msg);
                latest_version
            }
        },
        VersionModification::NoneOrNext => match repo.is_version_head(&latest_version) {
            Ok(true) => latest_version,
            Ok(false) => latest_version.next_version(),
            Err(msg) => {
                debug!("Unable to compair version to HEAD: {:?}", msg);
                latest_version
            }
        },
        VersionModification::OneMore => latest_version.next_version(),
    };

    return Ok(return_version);
}
