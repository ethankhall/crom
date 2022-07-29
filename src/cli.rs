use clap::{ArgEnum, ArgGroup, Parser};
use log::LevelFilter;

#[derive(Parser, Debug)]
#[clap(group = ArgGroup::new("logging"))]
pub struct LoggingOpts {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences), global(true), group = "logging")]
    pub verbose: u64,

    /// Enable warn logging
    #[clap(short, long, global(true), group = "logging")]
    pub warn: bool,

    /// Disable everything but error logging
    #[clap(short, long, global(true), group = "logging")]
    pub error: bool,
}

impl LoggingOpts {
    pub fn to_level_filter(&self) -> LevelFilter {
        if self.error {
            LevelFilter::Error
        } else if self.warn {
            LevelFilter::Warn
        } else if self.verbose == 0 {
            LevelFilter::Info
        } else if self.verbose == 1 {
            LevelFilter::Debug
        } else {
            LevelFilter::Trace
        }
    }
}

pub enum VersionRequest {
    Custom(String),
    Latest,
    NextRelease,
    PreRelease,
}

#[derive(Parser, Debug)]
#[clap(author, about, version)]
pub struct Opts {
    #[clap(subcommand)]
    pub sub_command: SubCommand,
    #[clap(flatten)]
    pub logging_opts: LoggingOpts,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    Init(InitArgs),
    Get(GetArgs),
    #[clap(alias = "write")]
    WriteVersion(WriteArgs),
    #[clap(name = "util", alias = "utility", alias = "utilities")]
    Utility(UtilityArgs),
    #[clap(name = "gh")]
    GitHub(GitHubCli)
}

/// Bootstrap a project
#[derive(Parser, ArgEnum, Debug, Clone)]
pub enum InitBumper {
    #[clap(name = "semver")]
    SemanticVersion,
    Atomic,
}

/// Create a .crom.toml file in the working directory.
#[derive(Parser, Debug)]
pub struct InitArgs {
    /// What logic should the project use to set versions?
    #[clap(arg_enum)]
    pub bumper: InitBumper,
}

/// Retrieve information from the current repo
#[derive(Parser, Debug)]
pub struct GetArgs {
    #[clap(subcommand)]
    pub sub_command: GetSubCommand,
}

#[derive(Parser, Debug)]
pub enum GetSubCommand {
    /// Get the latest version based on the git history.
    #[clap(alias = "latest-version")]
    Latest,

    /// Get a beta version of the repository.
    ///
    /// If you use the 'semver' bumper, a semver compatible version
    /// will be emitted. The git hash will be included, allowing multiple
    /// pre-release versions to be created.
    ///
    /// If you use the 'atomic' bumper, this will always return the
    /// next version.
    #[clap(alias = "snapshot-version")]
    PreRelease,

    /// Get the next version of the repository
    ///
    /// If you use the 'semver' bumper, the version will follow the
    /// pattern that you are using.
    ///
    /// If you use the 'atomic' bumper, the version will be the next
    /// integer.
    #[clap(alias = "next-release-version")]
    NextRelease,
}

impl GetSubCommand {
    pub fn make_version_request(&self) -> VersionRequest {
        match self {
            GetSubCommand::Latest => VersionRequest::Latest,
            GetSubCommand::PreRelease => VersionRequest::PreRelease,
            GetSubCommand::NextRelease => VersionRequest::NextRelease,
        }
    }
}

/// Write version into defined sources.
///
/// You mush specify the locations that need to be updated in the
/// `.crom.toml` file.
#[derive(Parser, Debug)]
pub struct WriteArgs {
    #[clap(subcommand)]
    pub sub_command: WriteSubCommand,
}

#[derive(Parser, Debug)]
pub enum WriteSubCommand {
    /// Write the latest version
    ///
    /// See [get latest] for how the value is computed.
    Latest,

    /// Write the pre-release version
    ///
    /// See [get pre-release] for how the value is computed.
    PreRelease,

    /// Write the next-release version
    ///
    /// See [get next-release] for how the value is computed.
    NextRelease,

    /// Write the custom version
    ///
    /// Version is required arg so crom knows what to write.
    Custom(WriteSubCommandArgsCustom),
}

impl WriteSubCommand {
    pub fn make_version_request(&self) -> VersionRequest {
        match self {
            WriteSubCommand::Custom(args) => VersionRequest::Custom(args.version.clone()),
            WriteSubCommand::PreRelease => VersionRequest::PreRelease,
            WriteSubCommand::NextRelease => VersionRequest::NextRelease,
            WriteSubCommand::Latest => VersionRequest::Latest,
        }
    }
}

#[derive(Parser, Debug)]
pub struct WriteSubCommandArgsCustom {
    /// The custom version to be written.
    pub version: String,
}

/// Utility that are useful during CI.
#[derive(Parser, Debug)]
pub struct UtilityArgs {
    #[clap(subcommand)]
    pub sub_command: UtilitySubCommand,
}

#[derive(Parser, Debug)]
pub enum UtilitySubCommand {
    /// Verify repo has no tracked changes
    ///
    /// If there are any changes to tracked files, the CLI will return
    /// with a non-zero exit code.
    VerifyNoChanges,
}

/// Execute the official GitHub CLI
#[derive(Parser, Debug)]
#[clap(allow_missing_positional = true, disable_help_flag = true, disable_help_subcommand = true, allow_hyphen_values = true)]
pub struct GitHubCli {
    #[clap(multiple_values = true)]
    pub args: Vec<String>
}