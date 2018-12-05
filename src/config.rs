use std::fs::File;
use std::io::prelude::*;

use toml;

use super::models::{CromConfig, CromError};

fn parse_config(path: &str) -> Result<CromConfig, CromError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: CromConfig = toml::from_str(&contents)?;
    return Ok(config);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;
    use super::super::models::ProjectConfig;

    #[test]
    fn parse_config_example1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources");
        d.push("test");
        d.push("config-1.toml");

        match parse_config(d.to_str().unwrap()) {
            Ok(config) => {
                assert!(config.projects.contains_key("default"));
                let project_config = ProjectConfig { pattern: String::from("1.2.3.%d"), version_files: vec![String::from("foo/bar")], included_paths:None };
                assert_eq!(&project_config, config.projects.get("default").unwrap());
            },
            Err(err) => {
                assert!(false, format!("{:?}", err));
            }
        }
    }
}