#[macro_use]
extern crate serde_derive;
extern crate toml;

#[macro_use]
extern crate log;

pub static CONFIG_FILE: &'static str = ".crom.toml";

mod logging;
pub mod error;
mod config;
pub mod commands;
pub mod git;

use clap::ArgMatches;
use std::io::Write;

pub use self::logging::configure_logging;

use self::error::*;

fn are_you_sure(default: bool) -> Result<bool, CromError> {
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