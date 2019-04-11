use serde_json::{self, Value};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use super::UpdateVersion;
use crate::crom_lib::config::file::NodeConfig;
use crate::crom_lib::error::*;
use crate::crom_lib::{read_file_to_string, Version};

impl UpdateVersion for NodeConfig {
    fn update_version(&self, root_path: PathBuf, version: &Version) -> Result<(), ErrorContainer> {
        let mut path = root_path.clone();
        path.push(crate::PACKAGE_JSON);

        let text = read_file_to_string(&path)?;

        let mut json: Value = serde_json::from_str(&text)?;

        json["version"] = Value::String(version.to_string());

        let text = serde_json::to_string(&json)?;

        let mut file = File::create(path)?;
        file.write_all(text.as_bytes())?;
        Ok(())
    }
}
