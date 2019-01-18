use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

mod artifact;
mod config;
mod error;
mod http;
mod repo;
mod updater;
mod version;

pub use config::{build_default_config, ParsedProjectConfig};
pub use error::*;
pub use repo::TagTarget;
pub use version::{Version, VersionModification};

pub static CONFIG_FILE: &'static str = ".crom.toml";

pub static VERSION_PROPERTIES: &'static str = "version.properties";
pub static CARGO_TOML: &'static str = "Cargo.toml";

pub trait Project {
    fn find_latest_version(&self, version_mod: VersionModification) -> Version;
    fn update_versions(&self, version: &Version) -> Result<(), ErrorContainer>;
    fn publish(&self, version: &Version, names: Vec<String>) -> Result<(), ErrorContainer>;
    fn tag_version(
        &self,
        version: &Version,
        targets: Vec<TagTarget>,
        allow_dirty_repo: bool,
    ) -> Result<(), ErrorContainer>;
}

pub fn make_project() -> Result<impl Project, ErrorContainer> {
    ParsedProjectConfig::new()
}

pub fn read_file_to_string(path: &PathBuf) -> Result<String, ErrorContainer> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
