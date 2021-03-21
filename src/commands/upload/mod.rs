use async_trait::async_trait;
use error_chain::bail;
use log::{debug, error, log_enabled, trace};
use std::path::PathBuf;

use git2::Repository;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Request;

mod compress;
mod github;

use crate::cli::UploadArgs;
use crate::errors::ErrorKind;
use crate::git_repo;
use crate::models::*;
use crate::version::Version;
use crate::CromResult;

pub struct UploadCommand;

#[async_trait]
impl super::CommandRunner<UploadArgs> for UploadCommand {
    async fn run_command(args: UploadArgs) -> CromResult<i32> {
        let (version, location, config) =
            super::create_version(args.sub_command.make_version_request()).await?;
        let repo = Repository::discover(location.clone())?;
        let remote = git_repo::get_owner_repo_info(&repo)?;
        let (owner, repo) = match &remote {
            git_repo::RepoRemote::GitHub { owner, repo } => (owner, repo),
        };

        let artifact_path = args
            .sub_command
            .artifact_path()
            .map(PathBuf::from)
            .unwrap_or(location);
        let mut artifacts: Vec<ProjectArtifacts> = Vec::new();

        for name in args.sub_command.artifact_names() {
            match config.artifact.get(&name) {
                Some(artifact) => artifacts.push(artifact.clone()),
                None => {
                    error!("Could not find artifact {} in .crom.toml", &name);
                    bail!(ErrorKind::ArtifactMissing(name.to_string()))
                }
            }
        }

        upload_artifacts(
            &owner,
            &repo,
            &version,
            artifacts,
            artifact_path,
            args.sub_command.github_token(),
        )
        .await?;
        Ok(0)
    }
}

#[derive(Debug)]
pub struct ArtifactContainer {
    request: Request,
    name: String,
}

impl ArtifactContainer {
    fn new(request: Request, name: String) -> Self {
        ArtifactContainer { request, name }
    }
}

async fn upload_artifacts(
    owner: &str,
    repo: &str,
    version: &Version,
    artifacts: Vec<ProjectArtifacts>,
    root_artifact_path: PathBuf,
    auth: String,
) -> CromResult<()> {
    let mut upload_requests: Vec<ArtifactContainer> = Vec::new();

    for art in artifacts {
        let res = match art.target {
            ProjectArtifactTarget::GitHub => {
                let client = github::GithubClient::new(&owner, &repo, &auth);
                client
                    .make_upload_request(version, art, root_artifact_path.clone())
                    .await
            }
        };

        match res {
            Err(e) => {
                error!("Error while building upload request: {:?}", e);
                bail!(ErrorKind::GitHubError("Upload Failed".to_string()))
            }
            Ok(bodys) => upload_requests.extend(bodys),
        }
    }

    do_request(upload_requests).await
}

async fn do_request(requests: Vec<ArtifactContainer>) -> CromResult<()> {
    let spinner = ProgressBar::new(requests.len() as u64);
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("/|\\- ")
            .template("{spinner:.dim.bold} [{pos:>7}/{len:7}] Processing request to {wide_msg}"),
    );

    if !log_enabled!(log::Level::Trace) {
        spinner.enable_steady_tick(100);
    }

    for request in requests {
        if let Err(e) = do_transfer(request).await {
            spinner.finish_and_clear();
            return Err(e);
        }
        spinner.inc(1);
    }

    spinner.finish_and_clear();

    Ok(())
}

async fn do_transfer(container: ArtifactContainer) -> CromResult<()> {
    trace!("Request: {:?}", container);

    let res = match crate::http::client().execute(container.request).await {
        Ok(res) => res,
        Err(err) => {
            let err_string = err.to_string();
            debug!("Hyper error: {:?}", err);
            error!("Failed to make request for {}", container.name);
            bail!(ErrorKind::GitHubError(format!(
                "Unknown communication error when talking to GitHub: Error {}",
                err_string,
            )))
        }
    };

    let status = res.status();
    if !status.is_success() {
        if let Ok(body_text) = res.text().await {
            debug!("Failed Upload: {}", body_text);
        }

        error!("Failed to upload to {}", container.name);
        bail!(ErrorKind::GitHubError(format!(
            "Failed Upload to {}",
            container.name
        )));
    }

    Ok(())
}
