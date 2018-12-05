use std::collections::HashMap;
use std::io::Error as IoError;
use toml::de::Error as TomlError;

#[derive(Debug)]
pub enum CromError {
    UnableToOpenFile(IoError),
    TomlParse(TomlError)
}

impl From<IoError> for CromError {
    fn from(error: IoError) -> Self {
        CromError::UnableToOpenFile(error)
    }
}

impl From<TomlError> for CromError {
    fn from(error: TomlError) -> Self {
        CromError::TomlParse(error)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CromConfig {
    #[serde(flatten)]
    pub projects: HashMap<String, ProjectConfig>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ProjectConfig {
    pub pattern: String,
    pub version_files: Vec<String>,
    pub included_paths: Option<Vec<String>>
}