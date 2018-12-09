use std::io::Error as IoError;

use toml::de::Error as DeTomlError;
use toml::ser::Error as SeTomlError;
use git2::Error as GitError;

#[derive(Debug)]
pub enum CromError {
    IoError(IoError),
    TomlParse(String),
    TomlSave(String),
    UnableToFindConfig(String),
    GitError(String),
    GitTagNotFound,
    GitWorkspaceNotClean,
    UserInput,
    ConfigError(String),
    ProjectNameNeeded,
}

impl From<CromError> for i32 {
    fn from(error: CromError) -> Self {
        match error {
            CromError::IoError(_) => 10,
            CromError::TomlParse(_) => 20,
            CromError::TomlSave(_) => 21,
            CromError::UnableToFindConfig(_) => 30,
            CromError::ProjectNameNeeded => 31,
            CromError::GitError(_) => 40,
            CromError::GitTagNotFound => 41,
            CromError::GitWorkspaceNotClean => 42,
            CromError::UserInput => 50,
            CromError::ConfigError(_) => 51,
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