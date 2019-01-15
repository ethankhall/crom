extern crate serde_derive;
extern crate json;
extern crate toml;
#[macro_use]
extern crate log;
extern crate hyper;
extern crate url;
extern crate xmltree;
extern crate crom_lib;

pub mod commands;
pub mod error;
mod logging;

use clap::ArgMatches;
use std::io::Write;

pub use self::logging::configure_logging;

use self::error::*;
use crom_lib::*;

type CromResult<T> = Result<T, CromError>;

fn are_you_sure(default: bool) -> CromResult<bool> {
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    return match input.trim().to_lowercase().as_str() {
        "y" => Ok(default),
        "n" => Ok(!default),
        _ => {
            error!("Didn't understand. Please try again.");
            Err(CromError::UserInput)
        }
    };
}

pub fn parse_pre_release(args: &ArgMatches) -> crom_lib::VersionModification {
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
