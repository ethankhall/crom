use std::convert::From;
use crate::repo::RepoError;
use crate::version::VersionError;
use crate::config::ConfigError;

#[derive(Debug)]
pub enum SharedError {
    Repo(RepoError),
    Version(VersionError),
    Config(ConfigError)
}

impl From<RepoError> for SharedError {
    fn from(err: RepoError) -> SharedError {
        SharedError::Repo(err)
    }
}

impl From<VersionError> for SharedError {
    fn from(err: VersionError) -> SharedError {
        SharedError::Version(err)
    }
}

impl From<ConfigError> for SharedError {
    fn from(err: ConfigError) -> SharedError {
        SharedError::Config(err)
    }
}