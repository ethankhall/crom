use clap::ArgMatches;

use crate::error::*;
use crate::git::Repo;

pub fn handle_get_command(args: &ArgMatches) -> Result<i32, CromError> {
    return match args.subcommand() {
        ("current-version", Some(run_matches)) => print_version(run_matches, VersionModification::NoneOrSnapshot),
        ("next-version", Some(run_matches)) => print_version(run_matches, VersionModification::OneMore),
        ("projects", Some(_run_matches)) => unimplemented!(),
        _ => unreachable!()
    }
}

enum VersionModification {
    NoneOrSnapshot,
    OneMore
}

fn print_version(args: &ArgMatches, modification: VersionModification) -> Result<i32, CromError> {
    let (root_path, configs) = crate::config::find_and_parse_config()?;

    let project_name = args.value_of("project").unwrap_or("default");
    let project_config = match configs.projects.get(project_name) {
        Some(config) => config,
        None => {
            return Err(CromError::ConfigError(format!("Unable to find project {}", project_name)));
        }
    };

    let repo = Repo::new(root_path)?;
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

    info!("{}", return_version);

    return Ok(0);
}