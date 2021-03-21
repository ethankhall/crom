use async_trait::async_trait;

use crate::cli::GetArgs;
use crate::CromResult;

pub struct GetCommand;

#[async_trait]
impl super::CommandRunner<GetArgs> for GetCommand {
    async fn run_command(args: GetArgs) -> CromResult<i32> {
        let (version, _, _) =
            super::create_version(args.sub_command.make_version_request()).await?;

        println!("{}", version);
        Ok(0)
    }
}
