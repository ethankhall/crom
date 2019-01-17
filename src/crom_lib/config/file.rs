use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;

use serde::de::{self, Deserialize, Deserializer};

use crate::crom_lib::error::*;

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
    pub types: Vec<VersionType>,
    pub message_template: Option<String>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum VersionType {
    Cargo,
    Property,
    Maven,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ProjectArtifactCompressionFormat {
    ZIP,
    TGZ,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProjectArtifactWrapper {
    pub name: String,
    pub format: ProjectArtifactCompressionFormat,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ProjectArtifactTarget {
    GitHub,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectArtifacts {
    pub paths: HashMap<String, String>,
    pub compress: Option<ProjectArtifactWrapper>,
    pub target: ProjectArtifactTarget,
}

impl FromStr for VersionType {
    type Err = ConfigError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "property" | "properties" => Ok(VersionType::Property),
            "mvn" | "maven" => Ok(VersionType::Maven),
            "cargo" | "rust" => Ok(VersionType::Cargo),
            _ => Err(ConfigError::InvalidVersionType(s.to_string())),
        }
    }
}

impl<'de> Deserialize<'de> for VersionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = VersionType;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: de::Error {
                return VersionType::from_str(v).map_err(|x| de::Error::custom(format!("{:?}", x)));
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}
