use clap::ArgMatches;

use crate::*;
use crate::error::*;
use crom_config::*;

pub fn exec_update_version(args: &ArgMatches) -> Result<i32, CromError> {
    let project = make_project()?;
    
    let modifier = parse_pre_release(args);

    let latest_version = match args.value_of("override_version") {
        Some(version) => Version::from(s!(version)),
        None => project.find_latest_version(modifier),
    };

    project.update_versions(&latest_version)?;

    return Ok(0);
}

pub fn exec_upload_artifacts(args: &ArgMatches) -> Result<i32, CromError> {
    let project = make_project()?;
    
    let names = args.values_of("NAMES").unwrap().map(|x| s!(x)).collect();

    let version = project.find_latest_version(VersionModification::None);

    project.publish(&version, names)?;

    return Ok(0);
}

pub fn exec_claim_version(args: &ArgMatches) -> Result<i32, CromError> {
    let project = make_project()?;

    let allow_dirty_repo = if !args.is_present("ignore_changes") {
        true
    } else {
        warn!("Skipping check for workspace changes.");
        false
    };

    let version = project.find_latest_version(VersionModification::OneMore);

    let targets: Vec<String> = args.values_of("source").unwrap().map(|x| s!(x)).collect();
    let targets: Vec<TagTarget> = targets.into_iter().map(|x| {
        match x.to_lowercase().as_str() {
            "local" => TagTarget::Local,
            "github" => TagTarget::GitHub,
            _ => unreachable!(),
        }
    }).collect();

    project.tag_version(&version, targets, allow_dirty_repo)?;

    info!("Created tag {}", version);
    Ok(0)
}
