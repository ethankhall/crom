use std::convert::From;
use std::path::PathBuf;

use ini::ini::ParseError as IniError;

use error_enum::{ErrorContainer, ErrorEnum, PrettyError};

#[derive(Debug, PartialEq, Eq, ErrorContainer)]
pub enum CliErrors {
    Repo(RepoError),
    Config(ConfigError),
    Updater(UpdaterError),
    GitHub(GitHubError),
    IO(IOError),
    UserInput(UserError),
    State(StateError),
    Artifact(ArtifactError),
    Compress(CompressError),
}

#[derive(Debug, PartialEq, Eq, ErrorEnum)]
#[error_enum(prefix = "USER")]
pub enum UserError {
    #[error_enum(description = "Unknown")]
    Unknown,
}

#[derive(Debug, PartialEq, Eq, ErrorEnum)]
#[error_enum(prefix = "IOE")]
pub enum IOError {
    #[error_enum(description = "Unknown")]
    Unknown(String),
    #[error_enum(description = "File not Found")]
    FileNotFound(PathBuf),
    #[error_enum(description = "Serialization Error")]
    SeralizationError(String),
}

#[derive(Debug, PartialEq, Eq, ErrorEnum)]
#[error_enum(prefix = "GZ")]
pub enum CompressError {
    #[error_enum(description = "Zip Failure")]
    ZipFailure(String),
    #[error_enum(description = "Zip Filename Error")]
    ZipFileNameErr(String),
    #[error_enum(description = "Artifact Not Found")]
    UnableToFindArtifact(String),
}

#[derive(Debug, PartialEq, Eq, ErrorEnum)]
#[error_enum(prefix = "ARTIFACT")]
pub enum ArtifactError {
    #[error_enum(description = "Upload Failed")]
    FailedUpload,
}

#[derive(Debug, PartialEq, Eq, ErrorEnum)]
#[error_enum(prefix = "STATE")]
pub enum StateError {
    #[error_enum(description = "Repo is not clean")]
    RepoNotClean,
    #[error_enum(description = "Artifact Not Found")]
    ArtifactNotFound(String),
}

#[derive(Debug, PartialEq, Eq, ErrorEnum)]
#[error_enum(prefix = "GITHUB")]
pub enum GitHubError {
    #[error_enum(description = "Access error")]
    AccessError(String),
    #[error_enum(description = "Connection Error")]
    UnkownCommunicationError(String),
    #[cfg(not(test))]
    #[error_enum(description = "Token Missing")]
    TokenMissing,
    #[error_enum(description = "Upload Fialed")]
    UploadFailed(String),
    #[error_enum(description = "Unable to Upload")]
    UnableToGetUploadUrl(String),
    #[error_enum(description = "Invalid Header")]
    InvalidHeader(String),
}

#[derive(Debug, PartialEq, Eq, ErrorEnum)]
#[error_enum(prefix = "CFG")]
pub enum ConfigError {
    #[error_enum(description = "Unable to find Config")]
    UnableToFindConfig(PathBuf),
    #[error_enum(description = "IO Error")]
    IoError(String),
    #[error_enum(description = "Parse Error")]
    ParseError(String),
    #[error_enum(description = "Missing version definition")]
    MissingVersionDefinition,
}

#[derive(Debug, PartialEq, Eq, ErrorEnum)]
#[error_enum(prefix = "REPO")]
pub enum RepoError {
    #[error_enum(description = "Git Error")]
    GitError(String),
    #[error_enum(description = "Unknow Git Remote")]
    GitRemoteUnkown(String),
    #[error_enum(description = "Regex Error")]
    RegexError(String),
    #[error_enum(description = "Unable to tag repo")]
    UnableToTagRepo(String),
}

#[derive(Debug, PartialEq, Eq, ErrorEnum)]
#[error_enum(prefix = "UPDATE")]
pub enum UpdaterError {
    #[error_enum(description = "Property Load Error")]
    PropertyLoad(String),
    #[error_enum(description = "Unable to start Maven process")]
    UnableToStartMavenProcess(String),
    #[error_enum(description = "Unable to set Maven version")]
    MavenVersionSetFailed(i32),
    #[error_enum(description = "Cargo.toml not valid")]
    CargoTomlNotValid(String),
}

impl From<zip::result::ZipError> for CliErrors {
    fn from(err: zip::result::ZipError) -> CliErrors {
        CliErrors::Compress(CompressError::ZipFailure(err.to_string()))
    }
}

impl From<IniError> for CliErrors {
    fn from(error: IniError) -> CliErrors {
        CliErrors::Updater(UpdaterError::PropertyLoad(error.to_string()))
    }
}

impl From<std::io::Error> for CliErrors {
    fn from(e: std::io::Error) -> CliErrors {
        CliErrors::IO(IOError::Unknown(e.to_string()))
    }
}

impl From<std::string::FromUtf8Error> for CliErrors {
    fn from(error: std::string::FromUtf8Error) -> CliErrors {
        CliErrors::IO(IOError::Unknown(error.utf8_error().to_string()))
    }
}

impl From<serde_json::Error> for CliErrors {
    fn from(err: serde_json::Error) -> CliErrors {
        CliErrors::IO(IOError::SeralizationError(err.to_string()))
    }
}

impl From<reqwest::header::InvalidHeaderValue> for CliErrors {
    fn from(err: reqwest::header::InvalidHeaderValue) -> CliErrors {
        CliErrors::GitHub(GitHubError::InvalidHeader(err.to_string()))
    }
}

impl From<reqwest::header::InvalidHeaderName> for CliErrors {
    fn from(err: reqwest::header::InvalidHeaderName) -> CliErrors {
        CliErrors::GitHub(GitHubError::InvalidHeader(err.to_string()))
    }
}
