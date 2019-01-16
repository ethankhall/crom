use hyper::rt::{Future, Stream};
use hyper::Client;

use super::*;
use crate::crom_lib::http::*;

pub fn tag_version(
    details: &RepoDetails,
    version: &Version,
    message: &str,
) -> Result<(), ErrorContainer> {
    let head = format!("{}", details.head_ref);
    let (owner, repo) = match &details.remote {
        RepoRemote::GitHub(owner, repo) => (owner, repo),
    };

    let url = format!(
        "{api_server}/repos/{owner}/{repo}/releases",
        api_server = get_github_api(),
        owner = owner,
        repo = repo
    );

    debug!("URL to post to: {}", url);

    let body = object! {
        "tag_name" => version.to_string(),
        "target_commitish" => head,
        "name" => version.to_string(),
        "body" => message.to_string(),
        "draft" => false,
        "prerelease" => false
    };

    let body_text = body.dump();

    let request = make_post(&url, make_github_auth_headers()?, body_text)?;

    trace!("Request {:?}", &request);

    let https = hyper_rustls::HttpsConnector::new(4);
    let client = Client::builder().build(https);

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(client.request(request)).unwrap();
    let status = res.status();
    if !status.is_success() {
        let body = match res.into_body().concat2().wait() {
            Ok(body) => String::from_utf8(body.to_vec())?,
            Err(err) => {
                error!("Unable to access response from GitHub. Status was {}", status);
                return Err(ErrorContainer::GitHub(GitHubError::AccessError(
                    err.to_string(),
                )));
            }
        };

        error!("Response {} from GitHub ({}) was {}", status, url, body);
        return Err(ErrorContainer::GitHub(
            GitHubError::UnkownCommunicationError(s!("Trouble talking to GitHub")),
        ));
    } else {
        return Ok(());
    }
}
