use std::path::PathBuf;

pub mod cargo;
pub mod package_json;
pub mod pom;
pub mod property;
pub mod version_py;

use crate::crom_lib::error::*;
use crate::crom_lib::version::Version;

pub trait UpdateVersion {
    fn update_version(&self, root_path: PathBuf, version: &Version) -> Result<(), CliErrors>;
}

pub fn update(
    root_path: PathBuf,
    version: &Version,
    version_updators: Vec<&dyn UpdateVersion>,
) -> Result<(), CliErrors> {
    for updator in version_updators {
        let result = updator.update_version(root_path.clone(), &version);

        match result {
            Ok(()) => {}
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
