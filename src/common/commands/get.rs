use clap::ArgMatches;

use crate::error::*;
use crate::git::Repo;
use super::*;

pub fn handle_get_command(args: &ArgMatches) -> Result<i32, CromError> {
    return match args.subcommand() {
        ("current-version", Some(run_matches)) => {
            let modifier = match run_matches.is_present("no_snapshot")  {
                true => VersionModification::NoneOrOneMore,
                false => VersionModification::NoneOrSnapshot
            };

            print_version(run_matches, modifier)
        },
        ("next-version", Some(run_matches)) => {
            print_version(run_matches, VersionModification::OneMore)
        },
        ("projects", Some(_run_matches)) => unimplemented!(),
        _ => unreachable!()
    }
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
    let latest_version = get_latest_version(&repo, &project_config, modification)?;

    info!("{}", latest_version);

    return Ok(0);
}