use std::collections::HashMap;
use std::io::Error as IoError;
use toml::de::Error as DeTomlError;
use toml::ser::Error as SeTomlError;
use git2::Error as GitError;
use regex::Regex;

#[derive(Debug)]
pub enum CromError {
    IoError(IoError),
    TomlParse(String),
    TomlSave(String),
    UnableToFindConfig(String),
    GitError(String),
    UserInput,
    ConfigError(String)
}

impl From<CromError> for i32 {
    fn from(error: CromError) -> Self {
        match error {
            CromError::IoError(_) => 10,
            CromError::TomlParse(_) => 20,
            CromError::TomlSave(_) => 21,
            CromError::UnableToFindConfig(_) => 30,
            CromError::GitError(_) => 35,
            CromError::UserInput => 40,
            CromError::ConfigError(_) => 41,
        }
    }
}

impl From<GitError> for CromError {
    fn from(error: GitError) -> Self {
        CromError::GitError(error.to_string())
    }
}

impl From<IoError> for CromError {
    fn from(error: IoError) -> Self {
        CromError::IoError(error)
    }
}

impl From<DeTomlError> for CromError {
    fn from(error: DeTomlError) -> Self {
        CromError::TomlParse(error.to_string())
    }
}

impl From<SeTomlError> for CromError {
    fn from(error: SeTomlError) -> Self {
        CromError::TomlSave(error.to_string())
    }
}

impl From<regex::Error> for CromError {
    fn from(error: regex::Error) -> Self {
        CromError::ConfigError(error.to_string())
    }
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

}

#[derive(Debug)]
pub struct Version {
    pub pattern: String,
    pub parts: Vec<VersionPart>
}

#[derive(Debug)]
pub enum VersionPart {
    Pinned(String),
    Wild
}

impl Version {
    pub fn to_regex(&self) -> Result<Regex, CromError> {
        let regex_string = self.pattern.replace(".", "\\.").replace("%d", "(?P<sub>\\d+)");

        return Ok(Regex::new(&regex_string)?);
    }

    pub fn make_version_number(self, part: i32) -> String {
        let vec: Vec<String> = self.parts.into_iter().map(|x| {
            match x {
                VersionPart::Pinned(value) => value,
                VersionPart::Wild => format!("{}", part)
            }
        }).collect();
        
        return vec.join(".");
    }
}

impl From<ProjectConfig> for Version {
    fn from(config: ProjectConfig) -> Self {
        let split: Vec<&str> = config.pattern.split(".").collect();
        let parts: Vec<VersionPart> = split.into_iter().map(|x| {
            return match x {
                "%d" => VersionPart::Wild,
                _ => VersionPart::Pinned(x.to_string())
            };
        }).collect();

        return Version { pattern: config.pattern, parts: parts };
    }
}