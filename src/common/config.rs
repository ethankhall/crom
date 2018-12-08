use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::env;
use std::collections::HashMap;

use regex::Regex;
use toml;

use crate::error::CromError;

pub fn find_and_parse_config() -> Result<(PathBuf, CromConfig), CromError> {
    let path = env::current_dir()?;
    for ancestor in path.ancestors() {
        let test_path = ancestor.join(".crom.toml");
        if test_path.exists() {
            return Ok((test_path.to_owned(), parse_config(test_path)?));
        }
    }

    return Err(CromError::UnableToFindConfig(format!("{:?}", path)));
}

fn parse_config<P: AsRef<Path>>(path: P) -> Result<CromConfig, CromError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: CromConfig = toml::from_str(&contents)?;
    return Ok(config);
}


#[derive(Serialize, Deserialize)]
pub struct CromConfig {
    #[serde(flatten)]
    pub projects: HashMap<String, ProjectConfig>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ProjectConfig {
    pub pattern: String,
    pub version_files: Vec<String>,
    pub included_paths: Option<Vec<String>>
}

impl ProjectConfig {
    pub fn build_version(&self) -> Version {
        let split: Vec<&str> = self.pattern.split(".").collect();
        let parts: Vec<VersionPart> = split.into_iter().map(|x| {
            return match x {
                "%d" => VersionPart::Dynamic,
                _ => VersionPart::Pinned(x.to_string())
            };
        }).collect();

        return Version { parts: parts };
    }
}

#[derive(Debug, Clone)]
pub struct Version {
    pub parts: Vec<VersionPart>
}

#[derive(Debug, Clone)]
pub enum VersionPart {
    Pinned(String),
    Dynamic
}

impl Version {
    pub fn to_regex(&self) -> Result<Regex, CromError> {
        let regex_string: Vec<String> = self.parts.clone().into_iter().map(|x| {
            match x {
                VersionPart::Pinned(value) => value,
                VersionPart::Dynamic => "(?P<sub>\\d+)".to_string()
            }
        }).collect();
        
        let regex_string = regex_string.join(".");

        return Ok(Regex::new(&regex_string)?);
    }

    pub fn make_version_number(&self, part: i32) -> String {
        let vec: Vec<String> = self.parts.clone().into_iter().map(|x| {
            match x {
                VersionPart::Pinned(value) => value,
                VersionPart::Dynamic => format!("{}", part)
            }
        }).collect();
        
        return vec.join(".");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parse_config_example1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources");
        d.push("test");
        d.push("config-1.toml");

        match parse_config(d) {
            Ok(config) => {
                assert!(config.projects.contains_key("default"));
                let project_config = ProjectConfig { pattern: String::from("1.2.3.%d"), version_files: vec![String::from("foo/bar")], included_paths:None };
                assert_eq!(&project_config, config.projects.get("default").unwrap());

                let version: Version = project_config.build_version();
                let regex = version.to_regex().unwrap();
                assert!(regex.is_match("1.2.3.4"));
                assert!(!regex.is_match("1.2.3"));
                assert!(!regex.is_match("2.2.3.4"));
            },
            Err(err) => {
                assert!(false, format!("{:?}", err));
            }
        }
    }
}