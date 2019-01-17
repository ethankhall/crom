use std::io::Error as IoError;

use crate::crom_lib::ErrorContainer;
use git2::Error as GitError;
use ini::ini::ParseError as IniError;
use toml::de::Error as DeTomlError;
use toml::ser::Error as SeTomlError;

#[derive(Debug)]
pub enum CromError {
    UnknownError(String),
    IoError(IoError),
    PropertyLoad(String),
    PropertySave(String),
    TomlParse(String),
    TomlSave(String),
    PomLoad,
    PomSave,
    JsonLoad(String),
    UnableToFindConfig(String),
    GitError(String),
    GitTagNotFound,
    GitWorkspaceNotClean,
    GitRemoteUnkown(String),
    GitHubError(String),
    GitHubTokenMissing,
    UserInput,
    VersionFileNotFound,
    VersionFileFormatUnknown(String),
    ConfigError(String),
    ProjectNameNeeded,
    SharedError(ErrorContainer),
}

impl From<ErrorContainer> for CromError {
    fn from(e: ErrorContainer) -> CromError {
        CromError::SharedError(e)
    }
}

impl From<CromError> for i32 {
    fn from(error: CromError) -> Self {
        match error {
            CromError::UnknownError(_) => 9,
            CromError::IoError(_) => 10,
            CromError::TomlParse(_) => 20,
            CromError::TomlSave(_) => 21,
            CromError::PropertyLoad(_) => 22,
            CromError::PropertySave(_) => 23,
            CromError::PomLoad => 24,
            CromError::PomSave => 25,
            CromError::JsonLoad(_) => 26,
            CromError::UnableToFindConfig(_) => 30,
            CromError::ProjectNameNeeded => 31,
            CromError::VersionFileNotFound => 32,
            CromError::VersionFileFormatUnknown(_) => 33,
            CromError::GitError(_) => 40,
            CromError::GitTagNotFound => 41,
            CromError::GitWorkspaceNotClean => 42,
            CromError::GitRemoteUnkown(_) => 43,
            CromError::GitHubError(_) => 44,
            CromError::GitHubTokenMissing => 45,
            CromError::UserInput => 50,
            CromError::ConfigError(_) => 51,
            CromError::SharedError(_) => 52,
        }
    }
}

impl From<json::Error> for CromError {
    fn from(error: json::Error) -> Self {
        debug!("Error reading JONS: {}", error);
        CromError::JsonLoad(error.to_string())
    }
}

impl From<xmltree::Error> for CromError {
    fn from(error: xmltree::Error) -> Self {
        debug!("Error writing POM file: {}", error);
        CromError::PomSave
    }
}

impl From<xmltree::ParseError> for CromError {
    fn from(error: xmltree::ParseError) -> Self {
        debug!("Error loading POM file: {}", error);
        CromError::PomLoad
    }
}

impl From<std::string::FromUtf8Error> for CromError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        CromError::PropertySave(error.utf8_error().to_string())
    }
}

impl From<IniError> for CromError {
    fn from(error: IniError) -> Self {
        CromError::PropertyLoad(error.to_string())
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
