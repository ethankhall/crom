use std::env;

use super::file::*;
use super::*;
use crate::crom_lib::*;

impl ParsedProjectConfig {
    pub fn new() -> Result<Self, CliErrors> {
        let (path, config) = find_and_parse_config()?;

        debug!("Parsed config: {:?}", config);
        debug!("Root path: {:?}", path);

        let project_config = config.project;

        let project_config = Rc::new(project_config);

        let matcher = VersionMatcher::new(&project_config.pattern);
        let repo_details = RepoDetails::new(&path, matcher)?;

        let cfg = ParsedProjectConfig {
            project_config,
            project_path: path,
            repo_details,
            artifacts: config.artifact,
        };

        debug!("Config: {:?}", cfg);
        Ok(cfg)
    }
}

fn find_and_parse_config() -> Result<(PathBuf, CromConfig), CliErrors> {
    let path = env::current_dir()?;
    for ancestor in path.ancestors() {
        let test_path = ancestor.join(crate::crom_lib::CONFIG_FILE);
        if test_path.exists() {
            debug!("Found config file at {:?}", test_path);
            let config = parse_config(test_path)?;
            let project_path = ancestor.to_owned();
            return Ok((project_path, config));
        }
    }

    Err(CliErrors::Config(ConfigError::UnableToFindConfig(path)))
}

fn parse_config(path: PathBuf) -> Result<CromConfig, CliErrors> {
    let contents = read_file_to_string(&path)?;

    match toml::from_str::<CromConfig>(&contents) {
        Ok(config) => Ok(config),
        Err(e) => Err(CliErrors::Config(ConfigError::ParseError(e.to_string()))),
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::IoError(err.to_string())
    }
}
