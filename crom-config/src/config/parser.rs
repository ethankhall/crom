use std::env;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use super::*;
use super::file::*;
use crate::error::ErrorContainer;

impl ParsedProjectConfig {
    pub fn new() -> Result<Self, ErrorContainer> {
        let (path, config) = find_and_parse_config()?;

        let project_config = config.project;

        let project_config = Rc::new(project_config);

        let matcher = VersionMatcher::new(&project_config.pattern);
        let repo_details = RepoDetails::new(&path, matcher)?;

        Ok(ParsedProjectConfig { project_config, project_path: path, repo_details })
    }
}

fn find_and_parse_config() -> Result<(PathBuf, CromConfig), ConfigError> {
    let path = env::current_dir()?;
    for ancestor in path.ancestors() {
        let test_path = ancestor.join(crate::CONFIG_FILE);
        if test_path.exists() {
            let config = parse_config(test_path)?;
            let project_path = ancestor.to_owned();
            return Ok((project_path, config));
        }
    }

    return Err(ConfigError::UnableToFindConfig(path));
}

fn parse_config<P: AsRef<Path>>(path: P) -> Result<CromConfig, ConfigError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: CromConfig = toml::from_str(&contents)?;
    return Ok(config);
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::IoError(err.description().to_string())
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::IoError(err.description().to_string())
    }
}