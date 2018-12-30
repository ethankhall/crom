use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use toml_edit::{value, Document};

use crate::error::*;
use crate::model::*;
use super::*;

pub struct CargoUpdater;

impl CargoUpdater {
    pub fn update_version(path: PathBuf, version: &Version) -> Result<(), CromError> {
        let text = read_file_to_string(&path)?;
        let mut doc = text.parse::<Document>().expect("invalid doc");
        let mut version_str = s!(version);

        if version_str.starts_with("v") {
            version_str = version_str.replacen("v", "", 1);
        }

        doc["package"]["version"] = value(version_str);

        let toml_string = doc.to_string();
        let toml_bytes = toml_string.as_bytes();

        let mut file = File::create(path)?;
        file.write_all(toml_bytes)?;
        Ok(())
    }
}