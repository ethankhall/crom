use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use toml;
use toml_edit::{value, Document};

use crate::error::*;
use crate::model::*;
use super::*;

pub struct CargoUpdater;

impl CargoUpdater {
    pub fn update_version(path: PathBuf, version: &Version) -> Result<(), CromError> {
        let text = read_file_to_string(&path)?;
        let mut doc = text.parse::<Document>().expect("invalid doc");
        doc["package"]["version"] = value(version.to_string());

        let toml_string = doc.to_string();
        let pretty_toml = toml::to_string_pretty(&toml_string)?;
        let toml_bytes = pretty_toml.as_bytes();

        let mut file = File::create(path)?;
        file.write_all(toml_bytes)?;
        Ok(())
    }
}