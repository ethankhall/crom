use std::collections::HashMap;
use std::path::PathBuf;

use json::{self, JsonValue};
use reqwest::{Request, Response};
use url::Url;

use crate::crom_lib::config::file::*;
use crate::crom_lib::error::*;
use crate::crom_lib::http::*;
use crate::crom_lib::repo::*;
use crate::crom_lib::version::Version;

use super::ArtifactContainer;

pub fn make_upload_request(
    details: &RepoDetails,
    version: &Version,
    artifacts: ProjectArtifacts,
    root_artifact_path: Option<PathBuf>,
) -> Result<Vec<ArtifactContainer>, ErrorContainer> {
    let (owner, repo) = match &details.remote {
        RepoRemote::GitHub(owner, repo) => (owner, repo),
    };

    let release_url = format!(
        "{api}/repos/{owner}/{repo}/releases/tags/{version}",
        api = get_github_api(),
        owner = owner,
        repo = repo,
        version = version
    );

    debug!("Release URL: {}", release_url);

    let request = make_get_request(&release_url, make_github_auth_headers()?)?;
    let mut res = crate::crom_lib::client().execute(request).unwrap();
    let upload_url = extract_upload_url(&mut res)?;

    let root_path = root_artifact_path.unwrap_or_else(|| details.path.clone());

    match artifacts.compress {
        Some(compression) => {
            compress_artifact(&upload_url, root_path, &artifacts.paths, &compression)
        }
        None => build_artifact_containers(&upload_url, root_path, &artifacts.paths),
    }
}

fn compress_artifact(
    upload_url: &Url,
    root_path: PathBuf,
    artifacts: &HashMap<String, String>,
    compresion: &ProjectArtifactWrapper,
) -> Result<Vec<ArtifactContainer>, ErrorContainer> {
    let compressed_name = compresion.name.to_string();
    let file = tempfile::NamedTempFile::new()?;

    super::compress::compress_files(&file, root_path, &artifacts, &compresion.format)?;
    let request = build_request(upload_url, &compressed_name, file.path().to_path_buf())?;
    file.close()?;

    let container = ArtifactContainer::new(request, compressed_name);
    Ok(vec![container])
}

fn build_artifact_containers(
    upload_url: &Url,
    root_path: PathBuf,
    artifacts: &HashMap<String, String>,
) -> Result<Vec<ArtifactContainer>, ErrorContainer> {
    let mut upload_requests = Vec::new();

    for (name, art_path) in artifacts {
        let mut path = root_path.clone();
        path.push(art_path);

        let request = build_request(upload_url, &name, path)?;
        upload_requests.push(ArtifactContainer::new(request, name.to_string()));
    }

    Ok(upload_requests)
}

fn build_request(
    upload_url: &Url,
    file_name: &str,
    file: PathBuf,
) -> Result<Request, ErrorContainer> {
    let mut uri = upload_url.clone();
    {
        let mut path = uri.path_segments_mut().expect("Cannot get path");
        path.pop();
        path.push("assets");
    }

    {
        let mut query = uri.query_pairs_mut();
        query.clear();
        query.append_pair("name", file_name);
    }

    make_file_upload_request(&uri, file, make_github_auth_headers()?)
}

fn extract_upload_url(res: &mut Response) -> Result<Url, ErrorContainer> {
    let body_text = match res.text() {
        Ok(text) => text,
        Err(err) => {
            error!("Unable to access response from GitHub.");
            return Err(ErrorContainer::GitHub(
                GitHubError::UnkownCommunicationError(err.to_string()),
            ));
        }
    };

    let json_body = match json::parse(&body_text) {
        Ok(value) => value,
        Err(err) => {
            debug!("Body was: {}", body_text);
            return Err(ErrorContainer::GitHub(
                GitHubError::UnkownCommunicationError(err.to_string().to_lowercase()),
            ));
        }
    };

    let obj = match json_body {
        JsonValue::Object(obj) => obj,
        _ => {
            error!("GitHub gave back a strange type.");
            return Err(ErrorContainer::GitHub(
                GitHubError::UnkownCommunicationError(s!("GitHub gave back a strange type.")),
            ));
        }
    };

    if log_enabled!(log::Level::Trace) {
        trace!("Json Response: {}", obj.dump());
    }

    let upload_url = obj.get("upload_url").unwrap().as_str().unwrap();
    match Url::parse(upload_url) {
        Ok(it) => Ok(it),
        Err(e) => Err(ErrorContainer::GitHub(GitHubError::UnableToGetUploadUrl(
            e.to_string(),
        ))),
    }
}
