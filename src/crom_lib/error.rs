use std::convert::From;
use std::path::PathBuf;

use ini::ini::ParseError as IniError;

#[derive(Debug, PartialEq)]
pub enum ErrorContainer {
    Repo(RepoError),
    Version(VersionError),
    Config(ConfigError),
    Updater(UpdaterError),
    GitHub(GitHubError),
    IO(IOError),
    UserInput,
    State(StateError),
    Artifact(ArtifactError),
    Compress(CompressError),
}

impl From<ErrorContainer> for i32 {
    fn from(error: ErrorContainer) -> Self {
        match error {
            ErrorContainer::Repo(_) => 10,
            ErrorContainer::Version(_) => 11,
            ErrorContainer::Config(_) => 12,
            ErrorContainer::Updater(_) => 13,
            ErrorContainer::GitHub(_) => 13,
            ErrorContainer::IO(_) => 15,
            ErrorContainer::UserInput => 16,
            ErrorContainer::State(_) => 17,
            ErrorContainer::Artifact(_) => 18,
            ErrorContainer::Compress(_) => 19,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum IOError {
    Unknown(String),
    FileNotFound(PathBuf),
    SeralizationError(String),
}

#[derive(Debug, PartialEq)]
pub enum CompressError {
    ZipFailure(String),
    ZipFileNameErr(String),
    UnableToFindArtifact(String),
    UnableToCompressArtifacts(String),
}

#[derive(Debug, PartialEq)]
pub enum ArtifactError {
    FailedUpload,
}

#[derive(Debug, PartialEq)]
pub enum StateError {
    RepoNotClean,
    ArtifactNotFound(String),
}

#[derive(Debug, PartialEq)]
pub enum GitHubError {
    AccessError(String),
    UnkownCommunicationError(String),
    TokenMissing,
    TokenInvalid(String),
    UploadFailed(String),
    UnableToGetUploadUrl(String),
}

#[derive(Debug, PartialEq)]
pub enum VersionError {}

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    UnableToFindConfig(PathBuf),
    IoError(String),
    ProjectNameNotDefined(String),
    ParseError(String),
    InvalidVersionType(String),
    MissingVersionDefinition,
    InvalidCompressionType(String),
    InvalidArtifactTarget(String),
}

#[derive(Debug, PartialEq)]
pub enum RepoError {
    GitError(String),
    GitRemoteUnkown(String),
    RegexError(String),
    UnableToTagRepo(String),
}

#[derive(Debug, PartialEq)]
pub enum UpdaterError {
    IOError(String),
    PropertySave(String),
    PropertyLoad(String),
    UnableToUpdateConfig,
    UnableToStartMavenProcess(String),
    MavenVersionSetFailed(i32),
    Unsupported,
    CargoTomlNotValid(String),
}

impl From<zip::result::ZipError> for ErrorContainer {
    fn from(err: zip::result::ZipError) -> ErrorContainer {
        ErrorContainer::Compress(CompressError::ZipFailure(err.to_string()))
    }
}

impl From<IniError> for ErrorContainer {
    fn from(error: IniError) -> ErrorContainer {
        ErrorContainer::Updater(UpdaterError::PropertyLoad(error.to_string()))
    }
}

impl From<std::io::Error> for ErrorContainer {
    fn from(e: std::io::Error) -> ErrorContainer {
        ErrorContainer::IO(IOError::Unknown(e.to_string()))
    }
}

impl From<std::string::FromUtf8Error> for ErrorContainer {
    fn from(error: std::string::FromUtf8Error) -> ErrorContainer {
        ErrorContainer::IO(IOError::Unknown(error.utf8_error().to_string()))
    }
}

impl From<RepoError> for ErrorContainer {
    fn from(err: RepoError) -> ErrorContainer {
        ErrorContainer::Repo(err)
    }
}

impl From<VersionError> for ErrorContainer {
    fn from(err: VersionError) -> ErrorContainer {
        ErrorContainer::Version(err)
    }
}

impl From<ConfigError> for ErrorContainer {
    fn from(err: ConfigError) -> ErrorContainer {
        ErrorContainer::Config(err)
    }
}

impl From<UpdaterError> for ErrorContainer {
    fn from(err: UpdaterError) -> ErrorContainer {
        ErrorContainer::Updater(err)
    }
}

impl From<serde_json::Error> for ErrorContainer {
    fn from(err: serde_json::Error) -> ErrorContainer {
        ErrorContainer::IO(IOError::SeralizationError(err.to_string()))
    }
}
