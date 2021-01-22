use error_chain::error_chain;

error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        Toml(::toml::ser::Error);
        GitRepo(::git2::Error);
        Regex(::regex::Error);
        Reqwest(::reqwest::Error);
        Json(::serde_json::Error);
        Ini(::ini::ini::ParseError);
        StringUtf8Error(::std::string::FromUtf8Error);
        Zip(::zip::result::ZipError);
    }

    errors {
        UserError(t: String) {
            description("User input not understood")
            display("User input '{}' not understood.", t)
        }

        ConfigMissing(t: String) {
            description("Unable to find .crom.toml")
            display("Unable to find .crom.toml in {} or it's parents.", t)
        }

        ConfigInvalid(t: String) {
            description("File .crom.toml was not valid.")
            display("There was an error when reading .crom.toml. Error: '{}'", t)
        }

        UnableToTag(t: String) {
            description("Unable to create tag.")
            display("Unable to create tag. Error: '{}'", t)
        }

        UnknownGitRemotes(t: String) {
            description("Unable to find git remote")
            display("Unable to determine git repo from remote: {}", t)
        }

        FileNotFound(t: ::std::path::PathBuf) {
            description("File not found")
            display("File not found: {:?}", t)
        }

        HeaderError(t: String) {
            description("Error while building headers.")
            display("Error while building headers: Error: '{}'", t)
        }

        GitHubError(t: String) {
            description("Error while talking to GitHub")
            display("Error while talking to GitHub: Error: '{}'", t)
        }

        ArtifactMissing(t: String) {
            description("Artifact was not found")
            display("Artifact {} was not found.", t)
        }

        InvalidToml(t: String) {
            description("Invalid Toml")
            display("{}", t)
        }

        Maven(t: String) {
            description("Error when executing Maven")
            display("{}", t)
        }

        CompressionError(t: String) {
            description("Error when compressing artifacts")
            display("{}", t)
        }
    }
}
