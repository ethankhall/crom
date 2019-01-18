use std::convert::From;
use std::error::Error;
use std::path::PathBuf;

use ini::ini::ParseError as IniError;

#[derive(Debug, PartialEq)]
pub enum ErrorContainer {
    Repo(RepoError),
    Version(VersionError),
    Config(ConfigError),
    Updater(UpdaterError),
    GitHub(GitHubError),
    IOError(String),
    State(StateError),
    Artifact(ArtifactError),
    Compress(CompressError),
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
    Unsupported,
    CargoTomlNotValid(String),
}

impl From<zip::result::ZipError> for ErrorContainer {
    fn from(err: zip::result::ZipError) -> ErrorContainer {
        ErrorContainer::Compress(CompressError::ZipFailure(err.description().to_string()))
    }
}

impl From<IniError> for ErrorContainer {
    fn from(error: IniError) -> ErrorContainer {
        ErrorContainer::Updater(UpdaterError::PropertyLoad(error.to_string()))
    }
}

impl From<std::io::Error> for ErrorContainer {
    fn from(e: std::io::Error) -> ErrorContainer {
        ErrorContainer::IOError(e.description().to_string())
    }
}

impl From<std::string::FromUtf8Error> for ErrorContainer {
    fn from(error: std::string::FromUtf8Error) -> ErrorContainer {
        ErrorContainer::IOError(error.utf8_error().to_string())
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
