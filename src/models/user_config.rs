use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::version::VersionMatcher;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct CromConfig {
    #[serde(flatten)]
    pub project: ProjectConfig,

    #[serde(default)]
    pub artifact: HashMap<String, ProjectArtifacts>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectConfig {
    pub pattern: String,
    pub cargo: Option<CargoConfig>,
    pub property: Option<PropertyFileConfig>,
    pub maven: Option<MavenConfig>,
    #[serde(rename = "node")]
    pub package_json: Option<NodeConfig>,
    #[serde(rename = "python")]
    pub version_py: Option<VersionPyConfig>,
    pub message_template: Option<String>,
}

impl CromConfig {
    pub fn create_default(pattern: String, message_template: String) -> Self {
        let project_config = ProjectConfig {
            pattern,
            message_template: Some(message_template),
            cargo: None,
            property: None,
            maven: None,
            package_json: None,
            version_py: None,
        };

        CromConfig {
            project: project_config,
            artifact: HashMap::new(),
        }
    }

    pub fn create_version_matcher(&self) -> VersionMatcher {
        VersionMatcher::new(&self.project.pattern)
    }
}

#[derive(Serialize, Debug, PartialEq, Clone, Deserialize)]
pub struct VersionPyConfig {
    pub path: String,
}

#[derive(Serialize, Debug, PartialEq, Clone, Deserialize)]
pub struct NodeConfig {
    #[serde(default = "default_none_path")]
    #[serde(alias = "path")]
    pub directory: Option<String>,
}

#[derive(Serialize, Debug, PartialEq, Clone, Deserialize)]
pub struct CargoConfig {
    #[serde(default = "default_none_path")]
    #[serde(alias = "path")]
    pub directory: Option<String>,
}

#[derive(Serialize, Debug, PartialEq, Clone, Deserialize)]
pub struct MavenConfig {}

#[derive(Serialize, Debug, PartialEq, Clone, Deserialize)]
pub struct PropertyFileConfig {
    #[serde(default = "default_propery_file_path")]
    pub path: String,
}

fn default_none_path() -> Option<String> {
    None
}

fn default_propery_file_path() -> String {
    s!(crate::statics::VERSION_PROPERTIES)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ProjectArtifactCompressionFormat {
    #[serde(alias = "zip", alias = "ZIP")]
    Zip,
    #[serde(alias = "tgz", alias = "TGZ", alias = "tar.gz")]
    Tgz,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProjectArtifactWrapper {
    pub name: String,
    pub format: ProjectArtifactCompressionFormat,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ProjectArtifactTarget {
    #[serde(alias = "github")]
    GitHub,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectArtifacts {
    pub paths: HashMap<String, String>,
    pub compress: Option<ProjectArtifactWrapper>,
    pub target: ProjectArtifactTarget,
}

#[test]
fn verify_config_parse() {
    let example_text = "
pattern = 'v0.1.%d'
message-template = \"Created {version} for release.\"

[cargo]
[maven]
[node]
[python]
path = \"path/to/version.py\"
[property]
path = \"path/to/property-file.properties\"
";

    let config = toml::from_str::<CromConfig>(example_text).unwrap();
    println!("config: {:?}", config);
    assert_eq!(Some(CargoConfig { directory: None }), config.project.cargo);
    assert_eq!(Some(MavenConfig {}), config.project.maven);
    assert_eq!(
        Some(NodeConfig { directory: None }),
        config.project.package_json
    );
    assert_eq!(
        Some(VersionPyConfig {
            path: s!("path/to/version.py"),
        }),
        config.project.version_py
    );
    assert_eq!(
        Some(PropertyFileConfig {
            path: s!("path/to/property-file.properties"),
        }),
        config.project.property
    );
}
