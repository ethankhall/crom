use hyper::client::HttpConnector;
use hyper::rt::{Future, Stream};
use hyper::{Body, Client, Request};
use hyper_rustls::HttpsConnector;
use indicatif::{ProgressBar, ProgressStyle};

use crate::crom_lib::config::file::*;
use crate::crom_lib::error::*;
use crate::crom_lib::repo::*;
use crate::crom_lib::version::Version;

mod compress;
mod github;

#[derive(Debug)]
pub struct ArtifactContainer {
    request: Request<Body>,
    name: String,
}

impl ArtifactContainer {
    pub fn new(request: Request<Body>, name: String) -> Self {
        ArtifactContainer { request, name }
    }
}

pub fn upload_artifacts(
    details: &RepoDetails,
    version: &Version,
    artifacts: Vec<ProjectArtifacts>,
) -> Result<(), ErrorContainer> {
    let mut upload_requests: Vec<ArtifactContainer> = Vec::new();

    for art in artifacts {
        let res = match art.target {
            ProjectArtifactTarget::GitHub => github::make_upload_request(details, version, art),
        };

        match res {
            Err(e) => {
                error!("Error while uploading artifact: {:?}", e);
                return Err(ErrorContainer::Artifact(ArtifactError::FailedUpload));
            }
            Ok(bodys) => upload_requests.extend(bodys),
        }
    }

    return do_request(upload_requests);
}

fn do_request(requests: Vec<ArtifactContainer>) -> Result<(), ErrorContainer> {
    let https = hyper_rustls::HttpsConnector::new(4);
    let client = Client::builder().build(https);

    let mut rt = tokio::runtime::Runtime::new().unwrap();

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
        if let Err(e) = do_transfer(request, &mut rt, &client) {
            spinner.finish_and_clear();
            return Err(e);
        }
        spinner.inc(1);
    }

    spinner.finish_and_clear();

    return Ok(());
}

fn do_transfer(
    container: ArtifactContainer,
    rt: &mut tokio::runtime::Runtime,
    client: &Client<HttpsConnector<HttpConnector>>,
) -> Result<(), ErrorContainer> {
    trace!("Request: {:?}", container);
    let res = rt.block_on(client.request(container.request)).unwrap();
    let status = res.status();
    if !status.is_success() {
        if let Ok(body) = res.into_body().concat2().wait() {
            let body_text = &String::from_utf8(body.to_vec())?;
            debug!("Failed Upload: {}", body_text);
        }
        error!("Failed to upload to {}", container.name);
        return Err(ErrorContainer::GitHub(GitHubError::UploadFailed(format!(
            "Failed Upload to {}",
            container.name
        ))));
    }

    return Ok(());
}
