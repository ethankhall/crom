use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use clap::ArgMatches;

use crate::error::*;

use crom_lib::*;

pub fn handle_init_command(args: &ArgMatches) -> Result<i32, CromError> {
    let path = std::env::current_dir()?.join(crate::CONFIG_FILE);
    let pattern = match args.value_of("bumper").unwrap() {
        "semver" => "v0.1.%d",
        "atomic" => "%d",
        _ => unimplemented!(),
    };

    if path.exists() {
        print!("About to overwrite {:#?} ([Ny])? ", path);
        if crate::are_you_sure(false)? {
            return Ok(1);
        }
    }

    write_default_config(pattern, path.clone())?;

    info!(
        "Created {:#?}. Please update it to match your project's specific needs.",
        path
    );
    return Ok(0);
}

fn write_default_config<P: AsRef<Path>>(default_format: &str, dest: P) -> Result<(), CromError> {
    let default_config = build_default_config(default_format);

    let mut file = File::create(dest)?;
    file.write_all(default_config.as_bytes())?;
    Ok(())
}
