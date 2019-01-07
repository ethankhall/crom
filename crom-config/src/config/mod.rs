use std::path::PathBuf;
use std::rc::Rc;

pub static CONFIG_FILE: &'static str = ".crom.toml";

mod file;
mod parser;

use crate::version::*;

pub enum ConfigError {
    UnableToFindConfig(PathBuf),
    IoError(String),
    ProjectNameNotDefined(String)
}

pub struct ParsedProjectConfig {
    pub project_config: Rc<file::ProjectConfig>,
    pub project_path: PathBuf
}