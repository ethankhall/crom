use async_trait::async_trait;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::cli::{InitArgs, InitBumper};
use crate::statics::CONFIG_FILE;
use log::info;

use super::CromResult;

pub struct InitCommand;

#[async_trait]
impl super::CommandRunner<InitArgs> for InitCommand {
    async fn run_command(args: InitArgs) -> CromResult<i32> {
        let path = std::env::current_dir()?.join(CONFIG_FILE);
        let pattern = match args.bumper {
            InitBumper::SemanticVersion => "v0.1.%d",
            InitBumper::Atomic => "%d",
        };

        if path.exists() {
            print!("About to overwrite {:#?} ([Ny])? ", path);
            if super::are_you_sure(false)? {
                return Ok(1);
            }
        }

        write_default_config(pattern, path.clone())?;

        info!(
            "Created {:#?}. Please update it to match your project's specific needs.",
            path
        );
        Ok(0)
    }
}

fn write_default_config<P: AsRef<Path>>(default_format: &str, dest: P) -> CromResult<()> {
    use crate::models::CromConfig;

    let crom_config = CromConfig::create_default(
        default_format.to_string(),
        "Created {version} for release -- Crom".to_string(),
    );

    let text = toml::to_string_pretty(&crom_config)?;

    let mut file = File::create(dest)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}
