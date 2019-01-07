#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate json;
extern crate toml;
#[macro_use]
extern crate log;
extern crate hyper;
extern crate url;
extern crate xmltree;

pub static CONFIG_FILE: &'static str = ".crom.toml";

#[macro_export]
macro_rules! s {
    ($x:expr) => {
        $x.to_string()
    };
}

pub mod commands;
mod config;
pub mod error;
pub mod git;
pub mod github;
mod logging;
pub mod model;
pub mod updater;

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
