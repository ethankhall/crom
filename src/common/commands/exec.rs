use clap::ArgMatches;

use crate::error::*;
use crate::git::*;

pub fn exec_claim_version(args: &ArgMatches) -> Result<i32, CromError> {
    let (root_path, configs) = crate::config::find_and_parse_config()?;

    let project_name = args.value_of("project").unwrap_or("default");
    let project_config = match configs.projects.get(project_name) {
        Some(config) => config,
        None => {
            return Err(CromError::ConfigError(format!("Unable to find project {}", project_name)));
        }
    };

    let repo = Repo::new(root_path)?;

    if !args.is_present("ignore_changes") {
        match repo.is_working_repo_clean() {
            Ok(true) => {},
            Ok(false) => return Err(CromError::GitWorkspaceNotClean),
            Err(err) => {
                debug!("Error working with git repo: {:?}", err);
                return Err(err);
            }
        }
    } else {
        warn!("Skipping check for workspace changes.");
    }

    let versions = project_config.get_current_versions(&repo)?;

    let version = match versions.last() {
        Some(v) => v,
        None => {
            return Err(CromError::GitTagNotFound);
        }
    };

    let version = version.next_version();

    match repo.tag_version(&version) {
        Ok(true) => {
            info!("Created tag {}", version);
            return Ok(0);
        },
        Ok(false) => return Ok(1),
        Err(err) => return Err(err)
    };
}