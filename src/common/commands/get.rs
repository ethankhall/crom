use clap::ArgMatches;

use crate::error::*;
use crate::model::*;
use crate::git::Repo;

static DEFAULT_VERSION: &'static i32 = &0;

pub fn handle_get_command(args: &ArgMatches) -> Result<i32, CromError> {
    return match args.subcommand() {
        ("current-version", Some(run_matches)) => print_version(run_matches, VersionModification::NoneOrSnapshot),
        ("next-version", Some(run_matches)) => print_version(run_matches, VersionModification::OneMore),
        ("projects", Some(run_matches)) => unimplemented!(),
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
    let version_matcher = project_config.build_version_matcher()?;
    let versions = get_current_version(repo, &version_matcher)?;

    let latest_version = match versions.last() {
        Some(v) => v.clone(),
        None => version_matcher.build_default_version(*DEFAULT_VERSION)
    };

    let version_component = match modification {
        VersionModification::NoneOrSnapshot => latest_version,
        VersionModification::OneMore => latest_version.next_version()
    };

    println!("{}", version_component);

    return Ok(0);
}

fn get_current_version(repo: Repo, version_matcher: &VersionMatcher) -> Result<Vec<Version>, CromError> {
    let versions: Vec<Version> = repo.get_tags()?.into_iter()
        .filter_map(|tag| version_matcher.match_version(tag))
        .collect();

    return Ok(versions);
}