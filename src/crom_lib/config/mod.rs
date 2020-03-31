use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use toml;

pub(crate) mod file;
mod parser;

use self::file::*;

use crate::crom_lib::error::*;
use crate::crom_lib::repo::*;
use crate::crom_lib::updater::*;
use crate::crom_lib::version::*;

#[derive(Debug)]
pub struct ParsedProjectConfig {
    pub project_config: Rc<file::ProjectConfig>,
    pub artifacts: HashMap<String, ProjectArtifacts>,
    pub project_path: PathBuf,
    pub repo_details: RepoDetails,
}

pub fn build_default_config(version_format: &str) -> String {
    let project_config = ProjectConfig {
        pattern: s!(version_format),
        cargo: None,
        property: None,
        maven: None,
        package_json: None,
        version_py: None,
        message_template: None,
    };
    let crom_lib = CromConfig {
        project: project_config,
        artifact: HashMap::new(),
    };

    toml::to_string_pretty(&crom_lib).expect("That toml should be serializer.")
}

impl ParsedProjectConfig {
    pub fn find_latest_version(&self, modification: VersionModification) -> Version {
        match self.repo_details.known_versions.last() {
            Some(v) => get_latest_version(&self.repo_details, v.clone(), modification),
            None => VersionMatcher::new(&self.project_config.pattern).build_default_version(),
        }
    }

    pub fn update_versions(&self, version: &Version) -> Result<(), ErrorContainer> {
        let mut updators: Vec<&dyn UpdateVersion> = Vec::new();
        if let Some(cargo) = &self.project_config.cargo {
            updators.push(cargo);
        }

        if let Some(property) = &self.project_config.property {
            updators.push(property);
        }

        if let Some(maven) = &self.project_config.maven {
            updators.push(maven);
        }

        if let Some(package_json) = &self.project_config.package_json {
            updators.push(package_json);
        }

        if let Some(version_py) = &self.project_config.version_py {
            updators.push(version_py);
        }

        update(self.project_path.clone(), version, updators)
    }

    pub fn publish(
        &self,
        version: &Version,
        names: Vec<String>,
        root_artifact_path: Option<PathBuf>,
        auth: &Option<String>
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
            auth
        )
    }

    pub fn tag_version(
        &self,
        version: &Version,
        targets: Vec<TagTarget>,
        allow_dirty_repo: bool,
        auth: &Option<String>
    ) -> Result<(), ErrorContainer> {
        if !self.repo_details.is_workspace_clean {
            if allow_dirty_repo {
                warn!("Skipping check for workspace changes.");
            } else {
                return Err(ErrorContainer::State(StateError::RepoNotClean));
            }
        }

        let message = make_message(self.project_config.message_template.clone(), version)?;

        crate::crom_lib::repo::tag_repo(&self.repo_details, version, &message, targets, auth)?;

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
