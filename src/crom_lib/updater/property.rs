use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use ini::Ini;

use super::UpdateVersion;
use super::*;
use crate::crom_lib::config::file::PropertyFileConfig;
use crate::crom_lib::{read_file_to_string, Version};

impl UpdateVersion for PropertyFileConfig {
    fn update_version(&self, root_path: PathBuf, version: &Version) -> Result<(), ErrorContainer> {
        let mut path = root_path.clone();
        path.push(self.path.clone());

        let text = read_file_to_string(&path)?;

        let mut conf: Ini = Ini::load_from_str(&text)?;

        conf.with_section(None::<String>)
            .set("version", version.to_string());

        let mut version_file_buffer = Vec::new();
        conf.write_to(&mut version_file_buffer).unwrap();

        let version_text = String::from_utf8(version_file_buffer)?;

        let mut file = File::create(path)?;
        file.write_all(version_text.as_bytes())?;
        Ok(())
    }
}
