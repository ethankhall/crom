use clap::ArgMatches;

use crate::crom_lib::*;

pub fn handle_get_command(args: &ArgMatches, project: &ParsedProjectConfig) -> Result<i32, ErrorContainer> {
    match args.subcommand() {
        ("current-version", Some(run_matches)) => {
            let modifier = if run_matches.is_present("no_snapshot") {
                VersionModification::None
            } else {
                VersionModification::NoneOrSnapshot
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
    project: &ParsedProjectConfig,
    modification: VersionModification,
) -> Result<i32, ErrorContainer> {
    let latest_version = project.find_latest_version(modification);

    info!("{}", latest_version);

    Ok(0)
}
