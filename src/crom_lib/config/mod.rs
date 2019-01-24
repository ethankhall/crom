use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use toml;

pub(crate) mod file;
mod parser;

use self::file::*;

use crate::crom_lib::error::*;
use crate::crom_lib::repo::*;
use crate::crom_lib::version::*;
use crate::crom_lib::Project;

pub struct ParsedProjectConfig {
    pub project_config: Rc<file::ProjectConfig>,
    pub artifacts: HashMap<String, ProjectArtifacts>,
    pub project_path: PathBuf,
    pub repo_details: RepoDetails,
}

pub fn build_default_config(version_format: &str) -> String {
    let project_config = ProjectConfig {
        pattern: s!(version_format),
        types: vec![],
        message_template: None,
    };
    let crom_lib = CromConfig {
        project: project_config,
        artifact: HashMap::new(),
    };

    toml::to_string_pretty(&crom_lib).expect("That toml should be serializer.")
}

impl Project for ParsedProjectConfig {
    fn find_latest_version(&self, modification: VersionModification) -> Version {
        match self.repo_details.known_versions.last() {
            Some(v) => get_latest_version(&self.repo_details, v.clone(), modification),
            None => VersionMatcher::new(&self.project_config.pattern).build_default_version(),
        }
    }

    fn update_versions(&self, version: &Version) -> Result<(), ErrorContainer> {
        crate::crom_lib::updater::update(
            self.project_path.clone(),
            version,
            self.project_config.types.clone(),
        )
    }

    fn publish(
        &self,
        version: &Version,
        names: Vec<String>,
        root_artifact_path: Option<PathBuf>,
    ) -> Result<(), ErrorContainer> {
        let mut artifacts: Vec<ProjectArtifacts> = Vec::new();

        for name in names {
            match self.artifacts.get(&name) {
                Some(value) => artifacts.push(value.clone()),
                None => return Err(ErrorContainer::State(StateError::ArtifactNotFound(name))),
            }
        }

        if artifacts.is_empty() {
            return Err(ErrorContainer::State(StateError::ArtifactNotFound(s!(
                "No aritifact defined"
            ))));
        }

        debug!("Artifacts to upload: {:?}", artifacts);

        crate::crom_lib::artifact::upload_artifacts(
            &self.repo_details,
            version,
            artifacts,
            root_artifact_path,
        )
    }

    fn tag_version(
        &self,
        version: &Version,
        targets: Vec<TagTarget>,
        allow_dirty_repo: bool,
    ) -> Result<(), ErrorContainer> {
        if !self.repo_details.is_workspace_clean {
            if allow_dirty_repo {
                warn!("Skipping check for workspace changes.");
            } else {
                return Err(ErrorContainer::State(StateError::RepoNotClean));
            }
        }

        let message = make_message(self.project_config.message_template.clone(), version)?;

        crate::crom_lib::repo::tag_repo(&self.repo_details, version, &message, targets)?;

        Ok(())
    }
}

pub fn make_message(
    message_template: Option<String>,
    version: &Version,
) -> Result<String, ErrorContainer> {
    let template = message_template
        .clone()
        .unwrap_or_else(|| s!("Crom is creating a version {version}."));

    if !template.contains("{version}") {
        return Err(ErrorContainer::Config(
            ConfigError::MissingVersionDefinition,
        ));
    }

    Ok(template.replace("{version}", &version.to_string()))
}

fn get_latest_version(
    repo_details: &RepoDetails,
    latest_version: Version,
    modification: VersionModification,
) -> Version {
    match modification {
        VersionModification::NoneOrSnapshot => {
            if repo_details.is_version_head(&latest_version) {
                latest_version
            } else {
                latest_version.next_snapshot()
            }
        }
        VersionModification::None => {
            if repo_details.is_version_head(&latest_version) {
                latest_version
            } else {
                latest_version.self_without_snapshot()
            }
        }
        VersionModification::NoneOrNext => {
            if repo_details.is_version_head(&latest_version) {
                latest_version
            } else {
                latest_version.next_version()
            }
        }
        VersionModification::OneMore => latest_version.next_version(),
    }
}
