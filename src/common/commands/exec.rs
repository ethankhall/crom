use std::path::PathBuf;
use std::string::ToString;

use clap::ArgMatches;

use crate::common::*;
use crate::crom_lib::*;

pub fn exec_update_version(
    args: &ArgMatches,
    project: &ParsedProjectConfig,
) -> Result<i32, CliErrors> {
    let modifier = parse_pre_release(args);

    let latest_version = match args.value_of("override_version") {
        Some(version) => Version::from(s!(version)),
        None => project.find_latest_version(modifier),
    };

    project.update_versions(&latest_version)?;

    Ok(0)
}

pub fn exec_upload_artifacts(
    args: &ArgMatches,
    project: &ParsedProjectConfig,
) -> Result<i32, CliErrors> {
    let names = args
        .values_of("NAMES")
        .unwrap()
        .map(ToString::to_string)
        .collect();

    let version = match args.value_of("override_version") {
        Some(version) => Version::from(s!(version)),
        None => project.find_latest_version(VersionModification::None),
    };

    let github_token = args.value_of("GITHUB_TOKEN").map(|x| x.to_string());
    let root_artifact_path = args.value_of("root_artifact_path").map(PathBuf::from);
    project.publish(&version, names, root_artifact_path, &github_token)?;

    Ok(0)
}

pub fn exec_claim_version(args: &ArgMatches, project: &ParsedProjectConfig) -> Result<i32, CliErrors> {
    let allow_dirty_repo = if args.is_present("ignore_changes") {
        warn!("Skipping check for workspace changes.");
        true
    } else {
        false
    };

    let mut targets: Vec<TagTarget> = Vec::new();

    if args.is_present("github") {
        targets.push(TagTarget::GitHub);
    }

    if args.is_present("local") {
        targets.push(TagTarget::Local);
    }

    let github_token = args.value_of("GITHUB_TOKEN").map(|x| x.to_string());

    let version = project.find_latest_version(VersionModification::OneMore);

    project.tag_version(&version, targets, allow_dirty_repo, &github_token)?;

    info!("Created tag {}", version);
    Ok(0)
}
