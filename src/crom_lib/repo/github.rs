use super::*;
use crate::crom_lib::http::*;

pub fn tag_version(
    details: &RepoDetails,
    version: &Version,
    message: &str,
) -> Result<(), ErrorContainer> {
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

    let request = make_post(&url, make_github_auth_headers()?, body_text)?;

    trace!("Request {:?}", &request);
    let mut res = crate::crom_lib::client().execute(request).unwrap();
    let status = res.status();
    if !status.is_success() {
        let body = match res.text() {
            Ok(body) => body,
            Err(err) => {
                error!(
                    "Unable to access response from GitHub. Status was {}",
                    status
                );
                return Err(ErrorContainer::GitHub(GitHubError::AccessError(
                    err.to_string(),
                )));
            }
        };

        error!("Response {} from GitHub ({}) was {}", status, url, body);
        Err(ErrorContainer::GitHub(
            GitHubError::UnkownCommunicationError(s!("Trouble talking to GitHub")),
        ))
    } else {
        Ok(())
    }
}
