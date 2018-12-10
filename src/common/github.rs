use std::path::PathBuf;
use std::fs::File;

use std::io::prelude::*;
use json::{self, JsonValue};
use hyper::{Client, Chunk, Request};
use hyper::body::Body;
use hyper::rt::{Future, Stream};
use indicatif::{ProgressBar, ProgressStyle};
use hyper::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, CONTENT_LENGTH};
use url::Url;
use mime::Mime;
use mime_guess::guess_mime_type;

use crate::git::*;
use crate::model::*;
use crate::error::*;


pub struct GitHub;

impl GitHub {
    pub fn tag_version(repo: &Repo, version: &Version) -> Result<bool, CromError> {
        let head = repo.get_head_sha()?;
        let (owner, repo) = repo.get_owner_repo_info()?;
        let message = format!("Crom is creating a version {}.", version);

        let token = match std::env::var("GITHUB_TOKEN") {
            Ok(value) => value,
            Err(_) => return Err(CromError::UnknownError(s!("Unable to find GitHub token in GITHUB_TOKEN")))
        };

        let url = format!("https://api.github.com/repos/{owner}/{repo}/release", owner=owner, repo=repo);
        debug!("URL to post to: {}", url);

        let body = object!{
            "tag_name" => version.to_string(),
            "target_commitish" => head,
            "name" => version.to_string(),
            "body" => message,
            "draft" => false,
            "prerelease" => false
        };

        let body_text = body.dump();

        let request = make_post(&url, &token, body_text);

        let https = hyper_rustls::HttpsConnector::new(4);
        let client = Client::builder().build(https);

        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let res = rt.block_on(client.request(request)).unwrap();
        let status = res.status();
        if !status.is_success() {
            let body = match res.into_body().concat2().wait() {
                Ok(body) => String::from_utf8(body.to_vec())?,
                Err(err) => {
                    error!("Unable to access response from GitHub.");
                    return Err(CromError::GitHubError(err.to_string()));
                }
            };

            error!("Response {} from GitHub was {}", status, body);
            return Err(CromError::UnknownError(s!("Trouble talking to GitHub")));
        } else {
            return Ok(true);
        }
    }

    pub fn publish_artifact(repo: &Repo, version: &Version, files: Vec<Artifact>) -> Result<(), CromError> {
        let (owner, repo) = repo.get_owner_repo_info()?;
        let release_url = format!("https://api.github.com/repos/{owner}/{repo}/release/tags/{version}", 
            owner=owner, repo=repo, version=version);

        let spinner = ProgressBar::new_spinner();
        spinner.set_style(ProgressStyle::default_spinner()
            .tick_chars("/|\\- ")
            .template("{spinner:.dim.bold} Processing request to {wide_msg}"));
        spinner.enable_steady_tick(100);
        spinner.tick();
        spinner.set_message(&format!("{}", release_url));

        let token = match std::env::var("GITHUB_TOKEN") {
            Ok(value) => value,
            Err(_) => return Err(CromError::UnknownError(s!("Unable to find GitHub token in GITHUB_TOKEN")))
        };

        let https = hyper_rustls::HttpsConnector::new(4);
        let client = Client::builder().build(https);

        let mut rt = tokio::runtime::Runtime::new().unwrap();

        let request = make_get_request(&release_url, &token);
        let res = rt.block_on(client.request(request)).unwrap();

        let json_body = match res.into_body().concat2().wait() {
            Ok(body) => json::parse(&String::from_utf8(body.to_vec())?)?,
            Err(err) => {
                error!("Unable to access response from GitHub.");
                return Err(CromError::GitHubError(err.to_string()));
            }
        };

        let obj = match json_body {
            JsonValue::Object(obj) => obj,
            _ => {
                error!("GitHub gave back a strange type.");
                return Err(CromError::GitHubError(s!("GitHub gave back a strange type.")));
            }
        };

        let upload_url = obj.get("upload_url").unwrap().as_str().unwrap();

        for artifact in files {
            spinner.set_message(&format!("Uploading {}", artifact.name));
            let request = make_file_upload_request(upload_url, &token, &artifact.name, artifact.file_path)?;
            rt.block_on(client.request(request)).unwrap();
        }

        spinner.finish_and_clear();
  
        return Ok(());
    }
}

fn make_file_upload_request(url: &str, token: &str, expected_name: &str, file_path: PathBuf) -> Result<Request<Body>, CromError> {
    let mut uri = Url::parse(&url).expect("Url to be valid");
    {
            let mut path = uri.path_segments_mut().expect("Cannot get path");
            path.pop();
            path.push("assets");
    }

    {
            let mut query = uri.query_pairs_mut();
            query.clear();
            query.append_pair("name", expected_name);
    }

    let mime: Mime = guess_mime_type(&file_path);

    let mut file = File::open(file_path)?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)?;

    let size = contents.len();
    let (mut sender, body) = Body::channel();
    sender.send_data(Chunk::from(contents));

    return Ok(Request::builder()
        .method("POST")
        .uri(uri.as_str())
        .header(CONTENT_TYPE, mime.to_string())
        .header(AUTHORIZATION, token)
        .header(CONTENT_LENGTH, size)
        .body(body)
        .unwrap());
}

fn make_get_request(url: &str, token: &str) -> Request<Body> {
    return Request::builder()
        .method("GET")
        .uri(url)
        .header(ACCEPT, "application/vnd.github.v3+json")
        .header(AUTHORIZATION, token)
        .body(Body::empty())
        .unwrap();
}

fn make_post(url: &str, token: &str, body_content: String) -> Request<Body> {
    let (mut sender, body) = Body::channel();
    sender.send_data(Chunk::from(body_content));

    return Request::builder()
        .method("POST")
        .uri(url)
        .header(AUTHORIZATION, token)
        .header(ACCEPT, "application/vnd.github.v3+json")
        .body(body)
        .unwrap();
}