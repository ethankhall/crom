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

pub static CONFIG_FILE: &str = ".crom.toml";

pub static PACKAGE_JSON: &str = "package.json";
pub static VERSION_PROPERTIES: &str = "version.properties";
pub static CARGO_TOML: &str = "Cargo.toml";

pub trait Project {
    fn find_latest_version(&self, version_mod: VersionModification) -> Version;
    fn update_versions(&self, version: &Version) -> Result<(), CliErrors>;
    fn publish(
        &self,
        version: &Version,
        names: Vec<String>,
        root_artifact_path: Option<PathBuf>,
    ) -> Result<(), CliErrors>;
    fn tag_version(
        &self,
        version: &Version,
        targets: Vec<TagTarget>,
        allow_dirty_repo: bool,
    ) -> Result<(), CliErrors>;
}

pub fn make_project() -> Result<ParsedProjectConfig, CliErrors> {
    ParsedProjectConfig::new()
}

pub fn read_file_to_string(path: &PathBuf) -> Result<String, CliErrors> {
    if !path.exists() {
        return Err(CliErrors::IO(IOError::FileNotFound(path.clone())));
    }
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn client() -> reqwest::Client {
    reqwest::Client::builder()
        .gzip(true)
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap()
}
