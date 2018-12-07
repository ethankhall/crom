#[macro_use]
extern crate serde_derive;
extern crate toml;

#[macro_use]
extern crate log;

pub static CONFIG_FILE: &'static str = ".crom.toml";

pub mod logging;
pub mod models;
pub mod config;
pub mod commands;
pub mod git;

use clap::ArgMatches;
use std::io::Write;

pub use self::logging::configure_logging;

pub fn run_init(args: &ArgMatches) -> Result<i32, models::CromError> {
    let path = std::env::current_dir()?.join(CONFIG_FILE);
    let pattern = match args.value_of("bumper").unwrap() {
        "semver" => "0.1.%d",
        "atomic" => "%d",
        _ => unimplemented!()
    };

    if path.exists() {
        print!("About to overwrite {:#?} ([Ny])? ", path);
        if are_you_sure(false)? {
            return Ok(1);
        }
    }

    commands::init::write_default_config(pattern, path.clone())?;

    info!("Created {:#?}. Please update it to match your project's specific needs.", path);
    return Ok(0);
}

fn are_you_sure(default: bool) -> Result<bool, models::CromError> {
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    return match input.trim().to_lowercase().as_str() {
        "y" => Ok(default),
        "n" => Ok(!default),
        _ => {
            error!("Didn't understand. Please try again.");
            Err(models::CromError::UserInput)
        }
    };    
}