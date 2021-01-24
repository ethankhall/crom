use clap::{ArgGroup, Clap};
use log::LevelFilter;

#[derive(Clap, Debug)]
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

#[derive(Clap, Debug)]
#[clap(author, about, version)]
pub struct Opts {
    #[clap(subcommand)]
    pub sub_command: SubCommand,
    #[clap(flatten)]
    pub logging_opts: LoggingOpts,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    Init(InitArgs),
    Get(GetArgs),
    Tag(TagArgs),
    #[clap(alias = "write")]
    WriteVersion(WriteArgs),
    #[clap(alias = "upload-artifact", alias = "upload")]
    UploadArtifacts(UploadArgs),
    #[clap(name = "util", alias = "utility", alias = "utilities")]
    Utility(UtilityArgs),
}

/// Bootstrap a project
#[derive(Clap, Debug)]
pub enum InitBumper {
    #[clap(name = "semver")]
    SemanticVersion,
    Atomic,
}

/// Create a .crom.toml file in the working directory.
#[derive(Clap, Debug)]
pub struct InitArgs {
    /// What logic should the project use to set versions?
    #[clap(arg_enum)]
    pub bumper: InitBumper,
}

/// Retrieve information from the current repo
#[derive(Clap, Debug)]
pub struct GetArgs {
    #[clap(subcommand)]
    pub sub_command: GetSubCommand,
}

#[derive(Clap, Debug)]
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
            GetSubCommand::NextRelease => VersionRequest::NextRelease
        }
    }
}

/// Tags the current repo with a new version
#[derive(Clap, Debug)]
pub struct TagArgs {
    #[clap(subcommand)]
    pub sub_command: TagSubCommand,
}

#[derive(Clap, Debug)]
pub enum TagSubCommand {
    /// Create a tag with custom values.
    Custom(TagSubCommandCustomArgs),

    /// Create a pre-release tag.
    ///
    /// This will be based on the bumper parameters. See [get pre-release]
    /// for how the value is computed.
    ///
    /// This is intended to be used when you publish versions with every build,
    /// or branch so that the pre-release version can be used. The general workflow
    /// is intended to be `crom tag pre-release`, then if you need to update version
    /// number in config (Crom.toml, package.json, etc) run `crom pre-version latest`.
    /// If you need to upload artifacts, you can then use `crom upload-artifact pre-release`
    PreRelease(TagSubCommandArgs),

    /// Create a tag based on the next version
    ///
    /// This will be based on the bumper parameters. See [get next-release]
    /// for how the value is computed.
    ///
    /// This is intended to be used when releasing a version. The general workflow
    /// is intended to be `crom tag next-release`, then if you need to update version
    /// number in config (Crom.toml, package.json, etc) run `crom write-version latest`.
    /// If you need to upload artifacts, you can then use `crom upload-artifact latest`
    NextRelease(TagSubCommandArgs),
}

impl TagSubCommand {

    pub fn make_version_request(&self) -> VersionRequest {
        match self {
            TagSubCommand::Custom(args) => VersionRequest::Custom(args.version.clone()),
            TagSubCommand::PreRelease(_) => VersionRequest::PreRelease,
            TagSubCommand::NextRelease(_) => VersionRequest::NextRelease
        }
    }
    
    pub fn github_token(&self) -> &Option<String> {
        match self {
            TagSubCommand::NextRelease(args) => &args.github_token,
            TagSubCommand::PreRelease(args) => &args.github_token,
            TagSubCommand::Custom(args) => &args.github_token,
        }
    }

    pub fn target_github(&self) -> bool {
        match self {
            TagSubCommand::NextRelease(args) => args.github,
            TagSubCommand::PreRelease(args) => args.github,
            TagSubCommand::Custom(args) => args.github,
        }
    }

    pub fn target_local(&self) -> bool {
        match self {
            TagSubCommand::NextRelease(args) => args.local,
            TagSubCommand::PreRelease(args) => args.local,
            TagSubCommand::Custom(args) => args.local,
        }
    }
}

#[derive(Clap, Debug)]
#[clap(group = ArgGroup::new("target").required(true).multiple(true))]
pub struct TagSubCommandArgs {
    /// Token to be used when talking to GitHub
    #[clap(long, env = "GITHUB_TOKEN")]
    pub github_token: Option<String>,

    /// Should the tag be created on GitHub
    #[clap(short, long, group = "target", requires = "github-token")]
    pub github: bool,

    /// Should the tag be created locally?
    #[clap(short, long, group = "target")]
    pub local: bool,
}

#[derive(Clap, Debug)]

#[clap(group = ArgGroup::new("target").required(true).multiple(true))]
pub struct TagSubCommandCustomArgs {
    /// Token to be used when talking to GitHub
    #[clap(long, env = "GITHUB_TOKEN")]
    pub github_token: Option<String>,

    /// Should the tag be created on GitHub
    #[clap(short, long, group = "target", requires = "github-token")]
    pub github: bool,

    /// Should the tag be created locally?
    #[clap(short, long, group = "target")]
    pub local: bool,

    /// The custom version to be created.
    pub version: String,
}

/// Write version into defined sources.
///
/// You mush specify the locations that need to be updated in the
/// `.crom.toml` file.
#[derive(Clap, Debug)]
pub struct WriteArgs {
    #[clap(subcommand)]
    pub sub_command: WriteSubCommand,
}

#[derive(Clap, Debug)]
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
            WriteSubCommand::Latest => VersionRequest::Latest
        }
    }
}

#[derive(Clap, Debug)]
pub struct WriteSubCommandArgsCustom {
    /// The custom version to be written.
    pub version: String,
}

/// Upload artifacts to GitHub.
///
/// You mush specify the artifacts you want to upload. These are defined
/// in `.crom.toml`
#[derive(Clap, Debug)]
pub struct UploadArgs {
    #[clap(subcommand)]
    pub sub_command: UploadSubCommand,
}

#[derive(Clap, Debug)]
pub enum UploadSubCommand {
    /// Upload a latest version
    ///
    /// See [get latest] for how the value is computed.
    ///
    /// You will need to specify which artifacts you want to upload.
    Latest(UploadSubCommandArgs),

    /// Update a pre-release version
    ///
    /// See [get pre-release] for how the value is computed.
    ///
    /// You will need to specify which artifacts you want to upload.
    PreRelease(UploadSubCommandArgs),

    /// Upload a custom version
    ///
    /// Version is required arg so crom knows what to write.
    ///
    /// You will need to specify which artifacts you want to upload.
    Custom(UploadSubCommandArgsCustom),
}

impl UploadSubCommand {
    pub fn make_version_request(&self) -> VersionRequest {
        match self {
            UploadSubCommand::Custom(args) => VersionRequest::Custom(args.version.clone()),
            UploadSubCommand::PreRelease(_) => VersionRequest::PreRelease,
            UploadSubCommand::Latest(_) => VersionRequest::Latest,
        }
    }

    pub fn github_token(&self) -> String {
        match self {
            UploadSubCommand::Custom(args) => args.github_token.clone(),
            UploadSubCommand::PreRelease(args) => args.github_token.clone(),
            UploadSubCommand::Latest(args) => args.github_token.clone(),
        }
    }

    pub fn artifact_path(&self) -> Option<String> {
        match self {
            UploadSubCommand::Custom(args) => args.artifact_path.clone(),
            UploadSubCommand::PreRelease(args) => args.artifact_path.clone(),
            UploadSubCommand::Latest(args) => args.artifact_path.clone(),
        }
    }

    pub fn artifact_names(&self) -> Vec<String> {
        match self {
            UploadSubCommand::Custom(args) => args.names.clone(),
            UploadSubCommand::PreRelease(args) => args.names.clone(),
            UploadSubCommand::Latest(args) => args.names.clone(),
        }
    }
}

#[derive(Clap, Debug)]
pub struct UploadSubCommandArgs {
    /// Where are the artifacts located?
    ///
    /// By default, crom will look in your working directory.
    /// But most of the time this is wrong, and you should specify
    /// the path to search in.
    #[clap(long)]
    pub artifact_path: Option<String>,

    /// Token to be used when talking to GitHub
    #[clap(long, env = "GITHUB_TOKEN")]
    pub github_token: String,

    /// The artifacts that need to be uploaded.
    ///
    /// These names are defined in the `.crom.toml`
    #[clap(required = true, min_values = 1)]
    pub names: Vec<String>,
}

#[derive(Clap, Debug)]
pub struct UploadSubCommandArgsCustom {
    /// Where are the artifacts located?
    ///
    /// By default, crom will look in your working directory.
    /// But most of the time this is wrong, and you should specify
    /// the path to search in.
    #[clap(long)]
    pub artifact_path: Option<String>,

    /// Token to be used when talking to GitHub
    #[clap(long, env = "GITHUB_TOKEN")]
    pub github_token: String,

    /// The artifacts that need to be uploaded.
    ///
    /// These names are defined in the `.crom.toml`
    #[clap(required = true, min_values = 1)]
    pub names: Vec<String>,

    /// The custom version to be written.
    pub version: String,
}

/// Utility that are useful during CI.
#[derive(Clap, Debug)]
pub struct UtilityArgs {
    #[clap(subcommand)]
    pub sub_command: UtilitySubCommand,
}

#[derive(Clap, Debug)]
pub enum UtilitySubCommand {
    /// Verify repo has no tracked changes
    ///
    /// If there are any changes to tracked files, the CLI will return
    /// with a non-zero exit code.
    VerifyNoChanges,
}
