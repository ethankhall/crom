use std::fs::File;
use std::path::PathBuf;

use xmltree::{Element, EmitterConfig};

use super::*;
use crate::error::*;
use crate::model::*;

pub struct PomUpdater;

impl PomUpdater {
    pub fn update_version(path: PathBuf, version: &Version) -> Result<(), CromError> {
        let text = read_file_to_string(&path)?;
        let mut elements = Element::parse(text.as_bytes())?;

        if let Some(version_node) = elements.get_mut_child("version") {
            version_node.text = Some(version.to_string());
            let config = EmitterConfig::new()
                .perform_indent(true)
                .normalize_empty_elements(false);

            elements.write_with_config(File::create(path)?, config)?;
        }
        return Ok(());
    }
}
