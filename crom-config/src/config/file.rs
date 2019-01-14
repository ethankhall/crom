use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CromConfig {
    pub project: ProjectConfig,
    pub artifact: HashMap<String, ProjectArtifacts>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectConfig {
    pub pattern: String,
    pub version_files: Vec<VersionFormat>,
    pub message_template: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum VersionFormat {
    Cargo,
    Property,
    Maven
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ProjectArtifactCompressionFormat {
    ZIP,
    TGZ
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProjectArtifactWrapper {
    pub name: String,
    pub format: ProjectArtifactCompressionFormat
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ProjectArtifactTarget {
    GitHub
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectArtifacts {
    pub paths: HashMap<String, String>,
    pub compress: Option<ProjectArtifactWrapper>,
    pub target: ProjectArtifactTarget
}