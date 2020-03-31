use std::path::PathBuf;

use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Request;

use crate::crom_lib::config::file::*;
use crate::crom_lib::error::*;
use crate::crom_lib::repo::*;
use crate::crom_lib::version::Version;

mod compress;
mod github;

#[derive(Debug)]
pub struct ArtifactContainer {
    request: Request,
    name: String,
}

impl ArtifactContainer {
    pub fn new(request: Request, name: String) -> Self {
        ArtifactContainer { request, name }
    }
}

pub fn upload_artifacts(
    details: &RepoDetails,
    version: &Version,
    artifacts: Vec<ProjectArtifacts>,
    root_artifact_path: Option<PathBuf>,
    auth: &Option<String>
) -> Result<(), CliErrors> {
    let mut upload_requests: Vec<ArtifactContainer> = Vec::new();

    for art in artifacts {
        let res = match art.target {
            ProjectArtifactTarget::GitHub => {
                let client = github::GithubClient::new(auth, details);
                client.make_upload_request(version, art, root_artifact_path.clone())
            }
        };

        match res {
            Err(e) => {
                error!("Error while building upload request: {:?}", e);
                return Err(CliErrors::Artifact(ArtifactError::FailedUpload));
            }
            Ok(bodys) => upload_requests.extend(bodys),
        }
    }

    do_request(upload_requests)
}

fn do_request(requests: Vec<ArtifactContainer>) -> Result<(), CliErrors> {
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
        if let Err(e) = do_transfer(request) {
            spinner.finish_and_clear();
            return Err(e);
        }
        spinner.inc(1);
    }

    spinner.finish_and_clear();

    Ok(())
}

fn do_transfer(container: ArtifactContainer) -> Result<(), CliErrors> {
    trace!("Request: {:?}", container);

    let mut res = match crate::crom_lib::client().execute(container.request) {
        Ok(res) => res,
        Err(err) => {
            let err_string = err.to_string();
            if let Some(ref e) = err.get_ref() {
                debug!("Hyper error: {:?}", e)
            }
            error!("Failed to make request for {}", container.name);
            return Err(CliErrors::GitHub(
                GitHubError::UnkownCommunicationError(err_string),
            ));
        }
    };

    let status = res.status();
    if !status.is_success() {
        if let Ok(body_text) = res.text() {
            debug!("Failed Upload: {}", body_text);
        }

        error!("Failed to upload to {}", container.name);
        return Err(CliErrors::GitHub(GitHubError::UploadFailed(format!(
            "Failed Upload to {}",
            container.name
        ))));
    }

    Ok(())
}
