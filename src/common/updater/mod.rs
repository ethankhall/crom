use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub mod cargo;
pub mod property;
pub mod pom;

use crate::error::*;

pub use self::cargo::CargoUpdater;
pub use self::property::PropertyUpdater;

pub fn read_file_to_string(path: &PathBuf) -> Result<String, CromError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}
