use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use super::UpdateVersion;
use super::*;
use crate::crom_lib::config::file::VersionPyConfig;
use crate::crom_lib::Version;

impl UpdateVersion for VersionPyConfig {
    fn update_version(&self, root_path: PathBuf, version: &Version) -> Result<(), CliErrors> {
        let mut path = root_path;
        path.push(self.path.clone());

        let version_text = format!("__version__ = \"{}\"", version);

        let mut file = File::create(path)?;
        file.write_all(version_text.as_bytes())?;
        Ok(())
    }
}
