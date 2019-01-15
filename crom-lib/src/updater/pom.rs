use std::path::PathBuf;

use crate::error::*;
use crate::Version;

pub struct PomUpdater;

impl PomUpdater {
    pub fn update_version(_root_path: PathBuf, _version: &Version) -> Result<(), ErrorContainer> {
        return Err(ErrorContainer::Updater(UpdaterError::Unsupported));
    }
}
