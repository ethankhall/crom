#![deny(clippy::all)]
#[macro_use]
extern crate clap;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate json;
extern crate toml;
#[macro_use]
extern crate log;
extern crate hyper;
extern crate libflate;
extern crate reqwest;
extern crate serde_json;
extern crate tempfile;
extern crate url;
extern crate xmltree;
extern crate zip;

#[macro_export]
macro_rules! s {
    ($x:expr) => {
        $x.to_string()
    };
}

mod common;
mod crom_lib;

use clap::ArgMatches;
use std::process;

use common::configure_logging;
use crom_lib::*;

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
        (@subcommand init =>
            (about: "Bootstrap a project")
            (long_about: "Create a .crom.toml file in the working directory, add a note about the current version.")
            (@arg bumper: --bumper +takes_value default_value[semver] possible_value[semver atomic] "what logic should the project use to set versions?"))
        (@subcommand get =>
            (about: "Retrieve information from the current repo")
            (@setting SubcommandRequiredElseHelp)
            (@subcommand current_version =>
                (name: "current-version")
                (about: "Geneated the current version number")
                (long_about: "When the repo is unmodified, and pointing at a a tag, the tag name will be used, otherwise -SNAPSHOT will be appended after the lowest version bump part")
                (@arg no_snapshot: --("no-snapshot") "Disable the `-SNAPSHOT` version postfix"))
            (@subcommand next_version =>
                (name: "next-version")
                (about: "Print what the next version will be")
                (long_about: "Based on current config, what would the next version be for this project"))
        )
        // TODO: This would be nice, but not for now...
        // (@subcommand set =>
        //     (about: "Set config within the project")
        //     (@setting SubcommandRequiredElseHelp)
        //     (@subcommand version_pattern =>
        //         (name: "version-pattern")
        //         (about: "Sets the version pattern from this point on. It's required to have `%d` somewhere in the path")
        //         (@arg project: -p --project +takes_value "Name of the project to operate on")
        //         (@arg PATTERN:  +required +takes_value { |a| if a.contains("%d") { Ok(()) } else {Err(String::from("Must contain %d")) }} "Pattern to use for versions"))
        // )
        (@subcommand tag_version =>
            (name: "tag-version")
            (about: "Tags the current repo with the next version")
            (long_about: "Finds the most recent version in the tags, and set the version to be one more than that. When running this command we expect files to be consistant with the repo. That means that there are no changes to tracked files. This way we can ensure that a tag is for something `real`.")
            (@arg source: --source +multiple +takes_value +use_delimiter default_value[local] possible_value[local github] "Should the tag be created locally or on GitHub?")
            (@arg ignore_changes: --("ignore-changes") "Disables check for workspace changes"))
        (@subcommand update_version =>
            (name: "update-version")
            (about: "Set the version to be most recent from tags")
            (@arg pre_release: --("pre-release") +takes_value default_value[snapshot] possible_value[snapshot release none] "If the version if pre-release, `snapshot` will append `-SNAPSHOT`, `release` will take the next version, `none` will omit it.")
            (@arg override_version: --("override-version") +takes_value "Don't look at history, use this value instead"))
        (@subcommand upload_artifacts =>
            (name: "upload-artifacts")
            (alias: "upload-artifact")
            (about: "Upload artifacts to store")
            (@arg root_artifact_path: --("root-artifact-path") -a +takes_value "Path to the root artifact dir")
            (@arg override_version: --("override-version") +takes_value "Don't look at history, use this value instead")
            (@arg NAMES: +takes_value +use_delimiter +multiple +required "Artifact names from `.crom.toml` to publish"))
        ).get_matches();

    configure_logging(
        matches.occurrences_of("debug") as i32,
        matches.is_present("warn"),
        matches.is_present("quite"),
    );

    let command_result = exec_commad(&matches);

    let return_code = match command_result {
        Ok(v) => v,
        Err(err) => {
            error!("{:?}", err);
            i32::from(err)
        }
    };

    process::exit(return_code);
}

fn exec_commad(matches: &ArgMatches) -> Result<i32, ErrorContainer> {
    let project = make_project();

    match matches.subcommand() {
        ("init", Some(arg_matches)) => common::commands::init::handle_init_command(arg_matches),
        ("get", Some(arg_matches)) => {
            common::commands::get::handle_get_command(arg_matches, &project?)
        }
        ("tag-version", Some(arg_matches)) => {
            common::commands::exec::exec_claim_version(arg_matches, &project?)
        }
        ("update-version", Some(arg_matches)) => {
            common::commands::exec::exec_update_version(arg_matches, &project?)
        }
        ("upload-artifacts", Some(arg_matches)) => {
            common::commands::exec::exec_upload_artifacts(arg_matches, &project?)
        }
        _ => unreachable!(),
    }
}
