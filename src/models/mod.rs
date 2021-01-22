use log::debug;
use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

use crate::errors::{Error as CromError, ErrorKind};
use error_chain::bail;
mod user_config;

pub use user_config::*;

pub async fn find_project_config() -> Result<(PathBuf, CromConfig), CromError> {
    let path = env::current_dir()?;
    for ancestor in path.ancestors() {
        let test_path = ancestor.join(crate::statics::CONFIG_FILE);
        if test_path.exists() {
            debug!("Found config file at {:?}", test_path);
            let config = parse_config(test_path)?;
            let project_path = ancestor.to_owned();
            return Ok((project_path, config));
        }
    }

    bail!(ErrorKind::ConfigMissing(path.to_string_lossy().to_string()))
}

fn parse_config(path: PathBuf) -> Result<CromConfig, CromError> {
    let contents = read_to_string(&path)?;

    match toml::from_str::<CromConfig>(&contents) {
        Ok(config) => Ok(config),
        Err(e) => bail!(ErrorKind::ConfigInvalid(e.to_string())),
    }
}
