use std::fs::File;
use std::path::PathBuf;

use super::*;
use crate::error::*;
use crate::Version;

pub struct PomUpdater;

impl PomUpdater {
    pub fn update_version(path: PathBuf, version: &Version) -> Result<(), UpdaterError> {
        return Err(UpdaterError::Unsupported);
    }
}
