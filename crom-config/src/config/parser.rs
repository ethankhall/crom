use std::env;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use super::*;
use super::file::*;

impl ParsedProjectConfig {
    fn new(project_name: &str) -> Result<Self, ConfigError> {
        let (path, config) = find_and_parse_config()?;

        let project_config = match config.projects.get(project_name) {
            Some(config) => config.clone(),
            None => {
                return Err(ConfigError::ProjectNameNotDefined(project_name.to_string()));
            }
        };

        let project_config = Rc::new(project_config);

        Ok(ParsedProjectConfig { project_config, project_path: path })
    }
}

fn find_and_parse_config() -> Result<(PathBuf, CromConfig), ConfigError> {
    let path = env::current_dir()?;
    for ancestor in path.ancestors() {
        let test_path = ancestor.join(CONFIG_FILE);
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