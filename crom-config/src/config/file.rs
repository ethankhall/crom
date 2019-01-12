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
    pub version_files: Vec<String>,
    pub included_paths: Option<Vec<String>>,
    pub message_template: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ProjectArtifactWrapper {
    ZIP,
    TGZ
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ProjectArtifactTarget {
    GitHub
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectArtifacts {
    paths: HashMap<String, String>,
    artifact_name: String,
    wrap_format: Option<ProjectArtifactWrapper>,
    target: ProjectArtifactTarget

}