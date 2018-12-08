use std::path::PathBuf;

use clap::ArgMatches;
use regex::Regex;

use crate::error::*;
use crate::config::*;
use crate::git::Repo;

static DEFAULT_VERSION: &'static i32 = &0;

pub fn handle_get_command(args: &ArgMatches) -> Result<i32, CromError> {
    return match args.subcommand() {
        ("current-version", Some(run_matches)) => print_version(run_matches, VersionModification::None),
        ("next-version", Some(run_matches)) => print_version(run_matches, VersionModification::OneMore),
        ("projects", Some(run_matches)) => unimplemented!(),
        _ => unreachable!()
    }
}

enum VersionModification {
    None,
    OneMore
}

fn print_version(args: &ArgMatches, modification: VersionModification) -> Result<i32, CromError> {
    let (path, configs) = crate::config::find_and_parse_config()?;

    let project_name = args.value_of("project").unwrap_or("default");
    let project_config = match configs.projects.get(project_name) {
        Some(config) => config,
        None => {
            return Err(CromError::ConfigError(format!("Unable to find project {}", project_name)));
        }
    };

    let version = project_config.build_version();
    let current_version = get_current_version(path, version.to_regex()?)?;
    let version_component = match modification {
        VersionModification::None => current_version,
        VersionModification::OneMore => current_version + 1,
    };

    let version_string = version.make_version_number(version_component);

    println!("{}", version_string);

    return Ok(0);
}

fn get_current_version(root_path: PathBuf, version_regex: Regex) -> Result<i32, CromError> {
    let repo = Repo::new(root_path)?;

    let mut wildcard_version: Vec<i32> = repo.get_tags()?.into_iter()
        .filter(|tag| version_regex.is_match(tag))
        .map(|tag| version_regex.captures(&tag).unwrap()["sub"].to_string())
        .map(|part| part.parse::<i32>().unwrap())
        .collect();
    
    wildcard_version.sort();
    let part = wildcard_version.last().unwrap_or_else(|| DEFAULT_VERSION);
    return Ok(*part);
}