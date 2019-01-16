use std::path::PathBuf;

pub mod cargo;
pub mod pom;
pub mod property;

use crate::crom_lib::error::*;

use self::cargo::CargoUpdater;
use self::pom::PomUpdater;
use self::property::PropertyUpdater;

use crate::crom_lib::config::file::VersionType;
use crate::crom_lib::version::Version;

pub fn update(
    root_path: PathBuf,
    version: &Version,
    formats: Vec<VersionType>,
) -> Result<(), ErrorContainer> {
    for format in formats {
        let path = root_path.clone();
        let result = match format {
            VersionType::Cargo => CargoUpdater::update_version(path, &version),
            VersionType::Property => PropertyUpdater::update_version(path, &version),
            VersionType::Maven => PomUpdater::update_version(path, &version),
        };

        match result {
            Ok(()) => {}
            Err(e) => return Err(e),
        }
    }

    return Ok(());
}
