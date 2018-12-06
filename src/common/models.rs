use std::collections::HashMap;
use std::io::Error as IoError;
use toml::de::Error as DeTomlError;
use toml::ser::Error as SeTomlError;

#[derive(Debug)]
pub enum CromError {
    IoError(IoError),
    TomlParse(String),
    TomlSave(String),
    UnableToFindConfig(String),
    UserInput,
}

impl From<CromError> for i32 {
    fn from(error: CromError) -> Self {
        match error {
            CromError::IoError(_) => 10,
            CromError::TomlParse(_) => 20,
            CromError::TomlSave(_) => 21,
            CromError::UnableToFindConfig(_) => 30,
            CromError::UserInput => 40
        }
    }
}

impl From<IoError> for CromError {
    fn from(error: IoError) -> Self {
        CromError::IoError(error)
    }
}

impl From<DeTomlError> for CromError {
    fn from(error: DeTomlError) -> Self {
        CromError::TomlParse(error.to_string())
    }
}

impl From<SeTomlError> for CromError {
    fn from(error: SeTomlError) -> Self {
        CromError::TomlSave(error.to_string())
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