use std::path::PathBuf;
use std::rc::Rc;
use std::collections::HashMap;

use indicatif::{ProgressBar, ProgressStyle};
use toml;

pub(crate) mod file;
mod parser;

use self::file::*;

use crate::version::*;
use crate::Project;
use crate::repo::*;
use crate::error::*;

pub struct ParsedProjectConfig {
    pub project_config: Rc<file::ProjectConfig>,
    pub project_path: PathBuf,
    pub repo_details: RepoDetails
}

pub fn build_default_config(version_format: &str) -> String {
    let project_config = ProjectConfig { pattern: s!(version_format), version_files: vec![], message_template: None };
    let crom_config = CromConfig { project: project_config, artifact: HashMap::new() };

    return toml::to_string_pretty(&crom_config).expect("That toml should be serializer.");
}

impl Project for ParsedProjectConfig {
    fn find_latest_version(&self, modification: VersionModification) -> Version {
        match self.repo_details.known_versions.last() {
            Some(v) => get_latest_version(&self.repo_details, v.clone(), modification),
            None => {
                VersionMatcher::new(&self.project_config.pattern).build_default_version()   
            }
        }
    }

    fn update_versions(&self, version: &Version) -> Result<(), ErrorContainer> {
        return crate::updater::update(self.project_path.clone(), version, self.project_config.version_files.clone());
    }

    fn publish(&self, version: &Version, names: Vec<String>) -> Result<(), ErrorContainer> {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("/|\\- ")
                .template("{spinner:.dim.bold} Processing request to {wide_msg}"),
        );

        if !log_enabled!(log::Level::Trace) {
            spinner.enable_steady_tick(100);
            spinner.tick();
        }

        //TODO...

        spinner.finish_and_clear();
        return Ok(());
    }

    fn tag_version(&self, version: &Version, targets: Vec<TagTarget>, allow_dirty_repo: bool) -> Result<(), ErrorContainer> {
        if !self.repo_details.is_workspace_clean {
            if allow_dirty_repo {
                warn!("Skipping check for workspace changes.");
            } else {
                return Err(ErrorContainer::State(StateError::RepoNotClean));
            }
        }

        let message = make_message(self.project_config.message_template.clone(), version)?;

        crate::repo::tag_repo(&self.repo_details, version, &message, targets)?;

        return Ok(());
    }
}

pub fn make_message(message_template: Option<String>, version: &Version) -> Result<String, ErrorContainer> {
    let template = message_template.clone().unwrap_or(s!("Crom is creating a version {version}."));

    if !template.contains("{version}") {
        return Err(ErrorContainer::Config(ConfigError::MissingVersionDefinition))
    }

    Ok(template.replace("{version}", &version.to_string()))
}

fn get_latest_version(
    repo_details: &RepoDetails,
    latest_version: Version,
    modification: VersionModification,
) -> Version {
    let return_version = match modification {
        VersionModification::NoneOrSnapshot => match repo_details.is_version_head(&latest_version) {
            true => latest_version,
            false => latest_version.next_snapshot()
        },
        VersionModification::None => match repo_details.is_version_head(&latest_version) {
            true => latest_version,
            false => latest_version.self_without_snapshot()
        },
        VersionModification::NoneOrNext => match repo_details.is_version_head(&latest_version) {
            true => latest_version,
            false => latest_version.next_version()
        },
        VersionModification::OneMore => latest_version.next_version(),
    };

    return return_version;
}
