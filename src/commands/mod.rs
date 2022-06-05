use crate::git_repo;
use async_trait::async_trait;
use error_chain::bail;
use log::{debug, error};
use std::path::PathBuf;

mod get;
mod init;
mod utils;
mod write;

use crate::cli::VersionRequest;
use crate::errors::ErrorKind;
use crate::models::CromConfig;
use crate::version::Version;
use crate::CromResult;

#[async_trait]
trait CommandRunner<T>
where
    T: Sized,
{
    async fn run_command(arg: T) -> CromResult<i32>;
}

pub async fn run_init(args: crate::cli::InitArgs) -> CromResult<i32> {
    init::InitCommand::run_command(args).await
}

pub async fn run_get(args: crate::cli::GetArgs) -> CromResult<i32> {
    get::GetCommand::run_command(args).await
}

pub async fn run_utils(args: crate::cli::UtilityArgs) -> CromResult<i32> {
    utils::UtilsCommand::run_command(args).await
}

pub async fn run_write(args: crate::cli::WriteArgs) -> CromResult<i32> {
    write::WriteCommand::run_command(args).await
}

pub fn are_you_sure(default: bool) -> CromResult<bool> {
    use std::io::Write;

    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    match input.trim().to_lowercase().as_str() {
        "y" => Ok(default),
        "n" => Ok(!default),
        input => {
            error!("Didn't understand. Please try again.");
            bail!(ErrorKind::UserError(input.to_string()))
        }
    }
}

async fn create_version(request: VersionRequest) -> CromResult<(Version, PathBuf, CromConfig)> {
    use git2::Repository;

    let (location, config) = crate::models::find_project_config().await?;
    debug!("Parsed config: {:?}", config);

    let repo = Repository::discover(location.clone())?;
    let matcher = config.create_version_matcher();
    let mut versions: Vec<Version> = git_repo::get_tags(&repo, &matcher)?;
    versions.sort();

    debug!("Found the following tags: {:?}", &versions);

    let default_version = matcher.build_default_version();
    let latest_version = versions.last().unwrap_or(&default_version);

    let mut head = git_repo::get_head_sha(&repo)?;
    head.truncate(7);

    let version = build_version(request, head, latest_version);

    Ok((version, location, config))
}

fn build_version(request: VersionRequest, head: String, latest_version: &Version) -> Version {
    match &request {
        VersionRequest::Custom(version) => Version::from(version.clone()),
        VersionRequest::PreRelease => latest_version.next_version(Some(head)),
        VersionRequest::NextRelease => latest_version.next_version(None),
        VersionRequest::Latest => latest_version.clone(),
    }
}

#[test]
fn test_latest_release() {
    use crate::version::VersionMatcher;

    let matcher = VersionMatcher::new("1.2.%d");
    let latest_version = matcher.match_version(s!("1.2.3")).unwrap();

    let version = build_version(
        VersionRequest::Latest,
        "abc123".to_string(),
        &latest_version,
    );
    assert_eq!(s!("1.2.3"), version.to_string());
}

#[test]
fn test_next_release() {
    use crate::version::VersionMatcher;

    let matcher = VersionMatcher::new("1.2.%d");
    let latest_version = matcher.match_version(s!("1.2.3")).unwrap();

    let version = build_version(
        VersionRequest::NextRelease,
        "abc123".to_string(),
        &latest_version,
    );
    assert_eq!(s!("1.2.4"), version.to_string());
}

#[test]
fn test_custom_version() {
    use crate::version::VersionMatcher;

    let matcher = VersionMatcher::new("1.2.%d");
    let latest_version = matcher.match_version(s!("1.2.3")).unwrap();

    let version = build_version(
        VersionRequest::Custom(s!("4.5.3")),
        "abc123".to_string(),
        &latest_version,
    );
    assert_eq!(s!("4.5.3"), version.to_string());
}

#[test]
fn test_pre_release() {
    use crate::version::VersionMatcher;

    let matcher = VersionMatcher::new("1.2.%d");
    let latest_version = matcher.match_version(s!("1.2.3")).unwrap();

    let version = build_version(
        VersionRequest::PreRelease,
        "abc123".to_string(),
        &latest_version,
    );
    assert_eq!(s!("1.2.4-abc123"), version.to_string());
}
