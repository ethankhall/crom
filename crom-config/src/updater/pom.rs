use std::fs::File;
use std::path::PathBuf;

use super::*;
use crate::error::*;
use crate::Version;

pub struct PomUpdater;

impl PomUpdater {
    pub fn update_version(root_path: PathBuf, version: &Version) -> Result<(), ErrorContainer> {
        return Err(ErrorContainer::Updater(UpdaterError::Unsupported));
    }
}
