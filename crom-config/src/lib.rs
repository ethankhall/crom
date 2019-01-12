#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

#[macro_export]
macro_rules! s {
    ($x:expr) => {
        $x.to_string()
    };
}

mod version;
mod config;
mod repo;
mod error;
mod artifact;
mod updater;

pub use crate::version::{Version, VersionModification, VersionError};
pub use crate::config::{ConfigError, ParsedProjectConfig};
pub use crate::repo::{RepoError, TagTarget};
pub use crate::error::SharedError;

pub static CONFIG_FILE: &'static str = ".crom.toml";

pub trait Project {
    fn find_latest_version(&self, version_mod: VersionModification) -> Version;
    fn update_versions(&self, version: &Version) -> Result<(), SharedError>;
    fn publish(&self, version: &Version, names: Vec<String>) -> Result<(), SharedError>;
    fn tag_version(&self, version: &Version, targets: Vec<TagTarget>, allow_dirty_repo: bool) -> Result<(), SharedError>;
}

pub fn make_project() -> Result<impl Project, error::SharedError> {
    ParsedProjectConfig::new()
}