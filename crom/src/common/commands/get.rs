use clap::ArgMatches;

use crate::error::CromError;
use crom_lib::*;

pub fn handle_get_command(args: &ArgMatches, project: &dyn Project) -> Result<i32, CromError> {
    match args.subcommand() {
        ("current-version", Some(run_matches)) => {
            let modifier = match run_matches.is_present("no_snapshot") {
                true => VersionModification::None,
                false => VersionModification::NoneOrSnapshot,
            };

            print_version(project, modifier)
        }
        ("next-version", Some(_run_matches)) => {
            print_version(project, VersionModification::OneMore)
        }
        _ => unimplemented!(),
    }
}

fn print_version(
    project: &dyn Project,
    modification: VersionModification,
) -> Result<i32, CromError> {
    let latest_version = project.find_latest_version(modification);

    info!("{}", latest_version);

    return Ok(0);
}
