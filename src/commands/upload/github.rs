use error_chain::bail;
use std::collections::HashMap;
use std::path::PathBuf;

use json::{self, JsonValue};
use reqwest::{Request, Response};
use url::Url;

use log::{debug, error, log_enabled, trace};

use crate::errors::ErrorKind;
use crate::http::*;
use crate::models::*;
use crate::version::Version;
use crate::CromResult;

use super::ArtifactContainer;

pub struct GithubClient {
    auth: String,
    owner: String,
    repo: String,
}

impl<'a> GithubClient {
    pub fn new(owner: &str, repo: &str, auth: &str) -> Self {
        GithubClient {
            auth: auth.to_string(),
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }

    pub async fn make_upload_request(
        &self,
        version: &Version,
        artifacts: ProjectArtifacts,
        root_path: PathBuf,
    ) -> CromResult<Vec<ArtifactContainer>> {
        let release_url = format!(
            "{api}/repos/{owner}/{repo}/releases/tags/{version}",
            api = get_github_api(),
            owner = self.owner,
            repo = self.repo,
            version = version
        );

        debug!("Release URL: {}", release_url);

        let request = make_get_request(&release_url, make_github_auth_headers(&self.auth)?)?;
        let res = client().execute(request).await.unwrap();
        let upload_url = extract_upload_url(res).await?;

        match artifacts.compress {
            Some(compression) => {
                self.compress_artifact(&upload_url, root_path, &artifacts.paths, &compression)
            }
            None => self.build_artifact_containers(&upload_url, root_path, &artifacts.paths),
        }
    }

    fn compress_artifact(
        &self,
        upload_url: &Url,
        root_path: PathBuf,
        artifacts: &HashMap<String, String>,
        compresion: &ProjectArtifactWrapper,
    ) -> CromResult<Vec<ArtifactContainer>> {
        let compressed_name = compresion.name.to_string();
        let file = tempfile::NamedTempFile::new()?;

        super::compress::compress_files(&file, root_path, &artifacts, &compresion.format)?;
        let request =
            self.build_request(upload_url, &compressed_name, file.path().to_path_buf())?;
        file.close()?;

        let container = ArtifactContainer::new(request, compressed_name);
        Ok(vec![container])
    }

    fn build_artifact_containers(
        &self,
        upload_url: &Url,
        root_path: PathBuf,
        artifacts: &HashMap<String, String>,
    ) -> CromResult<Vec<ArtifactContainer>> {
        let mut upload_requests = Vec::new();

        for (name, art_path) in artifacts {
            let mut path = root_path.clone();
            path.push(art_path);

            let request = self.build_request(upload_url, &name, path)?;
            upload_requests.push(ArtifactContainer::new(request, name.to_string()));
        }

        Ok(upload_requests)
    }

    fn build_request(
        &self,
        upload_url: &Url,
        file_name: &str,
        file: PathBuf,
    ) -> CromResult<Request> {
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

        make_file_upload_request(&uri, file, make_github_auth_headers(&self.auth)?)
    }
}

async fn extract_upload_url(res: Response) -> CromResult<Url> {
    let body_text = match res.text().await {
        Ok(text) => text,
        Err(err) => {
            error!("Unable to access response from GitHub.");
            bail!(ErrorKind::GitHubError(err.to_string(),))
        }
    };

    let json_body = match json::parse(&body_text) {
        Ok(value) => value,
        Err(err) => {
            debug!("Body was: {}", body_text);
            bail!(ErrorKind::GitHubError(err.to_string().to_lowercase(),))
        }
    };

    let obj = match json_body {
        JsonValue::Object(obj) => obj,
        _ => {
            error!("GitHub gave back a strange type.");
            bail!(ErrorKind::GitHubError(s!(
                "GitHub gave back a strange type."
            ),))
        }
    };

    if log_enabled!(log::Level::Trace) {
        trace!("Json Response: {}", obj.dump());
    }

    let upload_url = obj.get("upload_url").unwrap().as_str().unwrap();
    match Url::parse(upload_url) {
        Ok(it) => Ok(it),
        Err(e) => bail!(ErrorKind::GitHubError(e.to_string(),)),
    }
}
