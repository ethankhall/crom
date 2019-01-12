use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::convert::From;
use std::error::Error;

use ini::ini::ParseError as IniError;

pub mod cargo;
pub mod pom;
pub mod property;

pub use self::cargo::CargoUpdater;
pub use self::property::PropertyUpdater;

#[derive(Debug)]
pub enum UpdaterError {
    IOError(String),
    PropertySave(String),
    PropertyLoad(String),
    Unsupported
}

pub fn read_file_to_string(path: &PathBuf) -> Result<String, UpdaterError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

impl From<std::io::Error> for UpdaterError {
    fn from(e: std::io::Error) -> UpdaterError {
        UpdaterError::IOError(e.description().to_string())
    }
}

impl From<std::string::FromUtf8Error> for UpdaterError {
    fn from(error: std::string::FromUtf8Error) -> UpdaterError {
        UpdaterError::PropertySave(error.utf8_error().to_string())
    }
}

impl From<IniError> for UpdaterError {
    fn from(error: IniError) -> UpdaterError {
        UpdaterError::PropertyLoad(error.to_string())
    }
}