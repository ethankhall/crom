use mio_httpc::CallBuilder;
use json;

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
        
        let (response_meta, body) = CallBuilder::post(body_text.as_bytes().to_vec())
            .timeout_ms(100)
            .url(&url).unwrap()
            .header("Authorization", &token)
            .header("Accept", "application/vnd.github.v3+json")
            .exec()?;

        if response_meta.status >= 300 {
            error!("Response {} from GitHub was {}", response_meta.status, String::from_utf8(body)?);
            return Err(CromError::UnknownError(s!("Trouble talking to GitHub")));
        } else {
            return Ok(true);
        }
    }
}