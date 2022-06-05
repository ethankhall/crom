use async_trait::async_trait;
use error_chain::bail;

use log::{error, info, trace};

use crate::cli::TagArgs;
use git2::Repository;
use log::debug;

use crate::errors::ErrorKind;
use crate::git_repo;
use crate::version::Version;
use crate::CromResult;

pub struct TagCommand;

#[async_trait]
impl super::CommandRunner<TagArgs> for TagCommand {
    async fn run_command(args: TagArgs) -> CromResult<i32> {
        let (version, location, config) =
            super::create_version(args.sub_command.make_version_request()).await?;
        let repo = Repository::discover(location)?;
        let message = make_message(config.project.message_template, &version);

        if args.sub_command.target_github() {
            let github_token = args
                .sub_command
                .github_token()
                .as_ref()
                .expect("Clap it, but no github token provided.");
            let head = git_repo::get_head_sha(&repo)?;
            let remote = git_repo::get_owner_repo_info(&repo)?;
            let (owner, repo) = match &remote {
                git_repo::RepoRemote::GitHub { owner, repo } => (owner, repo),
            };
            tag_github(&head, owner, repo, &version, &message, github_token).await?;
        }

        if args.sub_command.target_local() {
            tag_local(&repo, &version, &message)?;
            info!("Created local tag {}", version);
        }

        Ok(0)
    }
}

fn make_message(message_template: Option<String>, version: &Version) -> String {
    let template = message_template.unwrap_or_else(|| s!("Crom is creating a version {version}."));

    template.replace("{version}", &version.to_string())
}

pub async fn tag_github(
    head: &str,
    owner: &str,
    repo: &str,
    version: &Version,
    message: &str,
    auth: &str,
) -> CromResult<()> {
    use crate::http::*;

    let url = format!(
        "{api_server}/repos/{owner}/{repo}/releases",
        api_server = get_github_api(),
        owner = owner,
        repo = repo
    );

    debug!("URL to post to: {}", url);

    let body = json::object! {
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
    let res = client().execute(request).await.unwrap();
    let status = res.status();
    if !status.is_success() {
        let body = match res.text().await {
            Ok(body) => body,
            Err(err) => {
                error!(
                    "Unable to access response from GitHub. Status was {}",
                    status
                );
                bail!(ErrorKind::GitHubError(err.to_string()))
            }
        };

        error!("Response {} from GitHub ({}) was {}", status, url, body);
        bail!(ErrorKind::GitHubError(s!("Trouble talking to GitHub")))
    } else {
        Ok(())
    }
}

fn tag_local(repo: &Repository, version: &Version, message: &str) -> CromResult<()> {
    use git2::*;

    let head = git_repo::get_head_sha(repo)?;
    let sig = git2::Signature::now("crom", "cli@crom.tech")?;

    let head_obj = repo.find_object(Oid::from_str(&head)?, Some(ObjectType::Commit))?;

    match repo.tag(&format!("{}", version), &head_obj, &sig, message, false) {
        Ok(_) => Ok(()),
        Err(e) => bail!(ErrorKind::UnableToTag(e.to_string())),
    }
}
