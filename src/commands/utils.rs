use async_trait::async_trait;
use std::env;

use crate::cli::{UtilityArgs, UtilitySubCommand};
use git2::Repository;

use crate::git_repo;
use crate::CromResult;

pub struct UtilsCommand;

#[async_trait]
impl super::CommandRunner<UtilityArgs> for UtilsCommand {
    async fn run_command(args: UtilityArgs) -> CromResult<i32> {
        match args.sub_command {
            UtilitySubCommand::VerifyNoChanges => {
                let repo = Repository::discover(env::current_dir()?)?;
                git_repo::is_working_repo_clean(&repo).map(|x| if x { 0 } else { 1 })
            }
        }
    }
}
