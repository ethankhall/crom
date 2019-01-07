use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CromConfig {
    #[serde(flatten)]
    pub projects: HashMap<String, ProjectConfig>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectConfig {
    pub pattern: String,
    pub version_files: Vec<String>,
    pub included_paths: Option<Vec<String>>,
    pub message_template: Option<String>,
}