#![deny(clippy::all)]
// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_export]
macro_rules! s {
    ($x:expr) => {
        $x.to_string()
    };
}

mod cli;
mod commands;
mod errors;
mod git_repo;
mod logging;
mod models;
mod statics;
mod version;

use clap::Parser;
use dotenv::dotenv;
use human_panic::setup_panic;
use log::error;
use std::process;
use std::process::Command;

pub type CromResult<T> = Result<T, crate::errors::Error>;

use crate::cli::*;

#[tokio::main]
async fn main() {
    setup_panic!();
    dotenv().ok();

    let opt = Opts::parse();

    logging::configure_logging(&opt.logging_opts);

    let result: CromResult<i32> = match opt.sub_command {
        SubCommand::Init(args) => crate::commands::run_init(args).await,
        SubCommand::Get(args) => crate::commands::run_get(args).await,
        SubCommand::WriteVersion(args) => crate::commands::run_write(args).await,
        SubCommand::Utility(args) => crate::commands::run_utils(args).await,
        SubCommand::GitHub(gh) => run_gh(gh)
    };

    let exit_code = match result {
        Ok(code) => code,
        Err(err) => {
            error!("{:?}", err);
            1
        }
    };

    process::exit(exit_code);
}

fn run_gh(gh: cli::GitHubCli) -> CromResult<i32> {
    let exit_status = Command::new("gh").args(gh.args).spawn()?.wait()?;
    Ok(exit_status.code().unwrap_or(1))
}