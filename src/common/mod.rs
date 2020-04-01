pub mod commands;
mod logging;

use clap::ArgMatches;
use std::io::Write;

pub use self::logging::configure_logging;

use crate::crom_lib::*;

type CromResult<T> = Result<T, CliErrors>;

fn are_you_sure(default: bool) -> CromResult<bool> {
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    match input.trim().to_lowercase().as_str() {
        "y" => Ok(default),
        "n" => Ok(!default),
        _ => {
            error!("Didn't understand. Please try again.");
            Err(CliErrors::UserInput(UserError::Unknown))
        }
    }
}

pub fn parse_pre_release(args: &ArgMatches) -> crate::crom_lib::VersionModification {
    match args
        .value_of("pre_release")
        .unwrap_or("snapshot")
        .to_lowercase()
        .as_str()
    {
        "snapshot" => VersionModification::NoneOrSnapshot,
        "none" => VersionModification::None,
        "release" => VersionModification::NoneOrNext,
        _ => unreachable!(),
    }
}
