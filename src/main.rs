#[macro_use]
extern crate clap;
extern crate chrono;

use crom::logging::configure_logging;

fn main() {
    let matches = clap_app!(MyApp =>
        (name: "crom")
        (@setting SubcommandRequiredElseHelp)
        (@setting ColorAuto)
        (@setting VersionlessSubcommands)
        (version: crate_version!())
        (about: "Crom, the version bean counter.")
        (@group logging =>
                (@arg debug: -d --debug ... +global "Turn debugging information on")
                (@arg quite: -q --quite +global "Only error output will be displayed")
                (@arg warn: -w --warn +global "Only error output will be displayed")
        )
        (@arg project: -p --project +takes_value +global "Name of the project to operate on")
        (@arg project: -r --repo +takes_value +multiple +global "Determine the project(s) to operate on based on provided commits ")
        (@subcommand init =>
            (about: "Bootstrap a project")
            (long_about: "Create a .crom.toml file in the working directory, add a note about the current version.")
            (@arg bumper: --bumper +takes_value default_value[semver] possible_value[semver atomic] "what logic should the project use to set versions?")
            (@arg CURRENT_VERSION: +takes_value +required "The version the project is currently on. If unsure, use 0.1.0."))
        (@subcommand get =>
            (about: "Retrieve information from the current repo")
            (@setting SubcommandRequiredElseHelp)
            (@subcommand current_version =>
                (name: "current-version")
                (about: "Geneated the current version number")
                (long_about: "When the repo is unmodified, and pointing at a a tag, the tag name will be used, otherwise -SNAPSHOT will be appended after the lowest version bump part"))
            (@subcommand next_version =>
                (name: "next-version")
                (about: "Print what the next version will be")
                (long_about: "Based on current config, what would the next version be for this project"))
            (@subcommand projects =>
                (about: "Lists projects avaliable"))
        )
        (@subcommand set =>
            (about: "Set config within the project")
            (@setting SubcommandRequiredElseHelp)
            (@subcommand version_pattern =>
                (name: "version-pattern")
                (about: "Sets the version pattern from this point on. It's required to have `%d` somewhere in the path")
                (@arg PATTERN:  +required +takes_value { |a| if a.contains("%d") { Ok(()) } else {Err(String::from("Must contain %d")) }} "Pattern to use for versions"))
        )
        (@subcommand claim_version =>
            (name: "claim-version")
            (about: "Tags the current repo with the next version")
            (long_about: "Finds the most recent version in the tags, and set the version to be one more than that. When running this command we expect files to be consistant with the repo. That means that there are no changes to tracked files. This way we can ensure that a tag is for something `real`.")
            (@arg ignore_changes: --("ignore-changes") "Disables check for workspace changes"))
        (@subcommand update_version =>
            (name: "update-version")
            (about: "Set the version to be most recent from tags")
            (@arg version: --("override-version") +takes_value "Don't look at history, use this value instead"))
        ).get_matches();

    
    configure_logging(
        matches.occurrences_of("debug") as i32,
        matches.is_present("warn"),
        matches.is_present("quite"),
    );

    match matches.subcommand() {
        // ("time", Some(time_matches)) => do_time_command(time_matches),
        // ("har", Some(time_matches)) => do_har_command(time_matches),
        _           => unreachable!()
    }
}