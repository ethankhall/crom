use super::*;
use crate::crom_lib::http::*;
use json::object;

pub async fn tag_version(
    details: &RepoDetails,
    version: &Version,
    message: &str,
    auth: &Option<String>
) -> Result<(), CliErrors> {
    let head = details.head_ref.to_string();
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

    let request = make_post(&url, make_github_auth_headers(auth)?, body_text)?;

    trace!("Request {:?}", &request);
    let res = crate::crom_lib::client().execute(request).await.unwrap();
    let status = res.status();
    if !status.is_success() {
        let body = match res.text().await {
            Ok(body) => body,
            Err(err) => {
                error!(
                    "Unable to access response from GitHub. Status was {}",
                    status
                );
                return Err(CliErrors::GitHub(GitHubError::AccessError(
                    err.to_string(),
                )));
            }
        };

        error!("Response {} from GitHub ({}) was {}", status, url, body);
        Err(CliErrors::GitHub(
            GitHubError::UnkownCommunicationError(s!("Trouble talking to GitHub")),
        ))
    } else {
        Ok(())
    }
}
