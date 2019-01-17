use std::path::PathBuf;

use crate::crom_lib::error::*;
use crate::crom_lib::Version;

pub struct PomUpdater;

impl PomUpdater {
    pub fn update_version(_root_path: PathBuf, _version: &Version) -> Result<(), ErrorContainer> {
        Err(ErrorContainer::Updater(UpdaterError::Unsupported))
    }
}
