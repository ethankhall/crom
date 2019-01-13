use std::fs::File;
use std::path::PathBuf;
use std::error::Error;

use hyper::header::{ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT};
use hyper::rt::{Future, Stream};
use hyper::{Client, Response, Body};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

use indicatif::{ProgressBar, ProgressStyle};
use json::{self, JsonValue};
use mime::Mime;
use mime_guess::guess_mime_type;
use std::io::prelude::*;
use url::Url;

use crate::repo::*;
use crate::error::*;
use crate::config::file::*;
use crate::version::Version;
use crate::http::*;

pub fn publish_artifact(
    details: &RepoDetails,
    version: &Version,
    artifacts: ProjectArtifacts,
    spinner: &ProgressBar
) -> Result<(), ErrorContainer> {
    
    let (owner, repo) = match &details.remote {
        RepoRemote::GitHub(owner, repo) => (owner, repo)
    };

    let release_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/releases/tags/{version}",
        owner = owner,
        repo = repo,
        version = version
    );

    spinner.set_message(&format!("{}", release_url));

    let https = HttpsConnector::new(4);
    let client: Client<HttpsConnector<HttpConnector>> = Client::builder().build(https);

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    debug!("Release URL: {}", release_url);

    let request = make_get_request(&release_url, make_github_auth_headers()?)?;
    let res = rt.block_on(client.request(request)).unwrap();
    let upload_url = extract_upload_url(res)?;

    match artifacts.compress {
        Some(compression) => unimplemented!(),
        None => upload_each_artifact(&upload_url, details.path.clone(), artifacts, spinner)?
    }

    return Ok(());
}

fn upload_each_artifact(upload_url: &Url,
    root_path: PathBuf, artifacts: ProjectArtifacts, 
    spinner:& ProgressBar) -> Result<(), ErrorContainer> {   

    let https = HttpsConnector::new(4);
    let client: Client<HttpsConnector<HttpConnector>> = Client::builder().build(https);

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    for (name, art_path) in artifacts.paths {
        let mut path = root_path.clone();
        path.push(art_path);

        spinner.set_message(&format!("Uploading {}", name));
        let request = make_file_upload_request(upload_url, path, make_github_auth_headers()?)?;
        trace!("Request for GitHub: {:?}", request);
        let res = rt.block_on(client.request(request)).unwrap();
        let status = res.status();
        if !status.is_success() {
            if let Ok(body) = res.into_body().concat2().wait() {
                let body_text = &String::from_utf8(body.to_vec())?;
                debug!("Failed Upload: {}", body_text);
            }
            error!("Failed to upload {} to GitHub", name);
            return Err(ErrorContainer::GitHub(GitHubError::UploadFailed(format!(
                "Unable to upload {}",
                name
            ))));
        }
    }

    return Ok(());
}

fn extract_upload_url(res: Response<Body>) -> Result<Url, ErrorContainer> {
    
    let json_body = match res.into_body().concat2().wait() {
        Ok(body) => {
            let body_text = &String::from_utf8(body.to_vec())?;
            match json::parse(body_text) {
                Ok(value) => value,
                Err(err) => {
                    debug!("Body was: {}", body_text);
                    return Err(ErrorContainer::GitHub(GitHubError::UnkownCommunicationError(err.description().to_lowercase())));
                }
            }
        }
        Err(err) => {
            error!("Unable to access response from GitHub.");
            return Err(ErrorContainer::GitHub(GitHubError::UnkownCommunicationError(err.to_string())));
        }
    };

    let obj = match json_body {
        JsonValue::Object(obj) => obj,
        _ => {
            error!("GitHub gave back a strange type.");
            return Err(ErrorContainer::GitHub(GitHubError::UnkownCommunicationError(s!(
                "GitHub gave back a strange type."
            ))));
        }
    };

    if log_enabled!(log::Level::Trace) {
        trace!("Json Response: {}", obj.dump());
    }

    let upload_url = obj.get("upload_url").unwrap().as_str().unwrap();
    match Url::parse(upload_url) {
        Ok(it) => Ok(it),
        Err(e) => Err(ErrorContainer::GitHub(GitHubError::UnableToGetUploadUrl(e.description().to_string())))
    }
}