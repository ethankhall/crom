use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::convert::From;

pub mod cargo;
pub mod pom;
pub mod property;

use crate::error::*;

use self::cargo::CargoUpdater;
use self::property::PropertyUpdater;
use self::pom::PomUpdater;

use crate::config::file::VersionFormat;
use crate::version::Version;
use crate::error::*;

pub fn update(root_path: PathBuf, version: &Version, formats: Vec<VersionFormat>) -> Result<(), ErrorContainer> {
    for format in formats {
        let path = root_path.clone();
        let result = match format {
            VersionFormat::Cargo => CargoUpdater::update_version(path, &version),
            VersionFormat::Property => PomUpdater::update_version(path, &version),
            VersionFormat::Maven => PomUpdater::update_version(path, &version),
        };

        match result {
            Ok(()) => {},
            Err(e) => {
                return Err(e)
            }
        }
    }

    return Ok(());
}