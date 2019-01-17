use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

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

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum ProjectArtifactCompressionFormat {
    ZIP,
    TGZ,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProjectArtifactWrapper {
    pub name: String,
    pub format: ProjectArtifactCompressionFormat,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
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

#[test]
#[allow(non_snake_case)]
fn VersionType_from_string() {
    assert_eq!(Ok(VersionType::Property), VersionType::from_str("property"));
    assert_eq!(
        Ok(VersionType::Property),
        VersionType::from_str("properties")
    );
    assert_eq!(Ok(VersionType::Maven), VersionType::from_str("mvn"));
    assert_eq!(Ok(VersionType::Maven), VersionType::from_str("maven"));
    assert_eq!(Ok(VersionType::Cargo), VersionType::from_str("cargo"));
    assert_eq!(Ok(VersionType::Cargo), VersionType::from_str("rust"));
    assert!(VersionType::from_str("else").is_err());
}

impl FromStr for ProjectArtifactCompressionFormat {
    type Err = ConfigError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tar.gz" | "tgz" => Ok(ProjectArtifactCompressionFormat::TGZ),
            "zip" => Ok(ProjectArtifactCompressionFormat::ZIP),
            _ => Err(ConfigError::InvalidCompressionType(s.to_string())),
        }
    }
}

#[test]
#[allow(non_snake_case)]
fn ProjectArtifactCompressionFormat_from_string() {
    assert_eq!(
        Ok(ProjectArtifactCompressionFormat::TGZ),
        ProjectArtifactCompressionFormat::from_str("tar.gz")
    );
    assert_eq!(
        Ok(ProjectArtifactCompressionFormat::TGZ),
        ProjectArtifactCompressionFormat::from_str("tgz")
    );
    assert_eq!(
        Ok(ProjectArtifactCompressionFormat::ZIP),
        ProjectArtifactCompressionFormat::from_str("zip")
    );
    assert!(VersionType::from_str("rar").is_err());
}

impl FromStr for ProjectArtifactTarget {
    type Err = ConfigError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "github" => Ok(ProjectArtifactTarget::GitHub),
            _ => Err(ConfigError::InvalidArtifactTarget(s.to_string())),
        }
    }
}

#[test]
#[allow(non_snake_case)]
fn ProjectArtifactTarget_from_string() {
    assert_eq!(
        Ok(ProjectArtifactTarget::GitHub),
        ProjectArtifactTarget::from_str("github")
    );
    assert_eq!(
        Ok(ProjectArtifactTarget::GitHub),
        ProjectArtifactTarget::from_str("GitHUB")
    );
    assert!(VersionType::from_str("ftp").is_err());
}

// Used in a type.
macro_rules! deseralize_from_string {
    ($A:tt, $x:expr) => {
        impl<'de> Deserialize<'de> for $A {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> de::Visitor<'de> for Visitor {
                    type Value = $A;

                    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, $x)
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        $A::from_str(v).map_err(|x| de::Error::custom(format!("{:?}", x)))
                    }
                }

                deserializer.deserialize_any(Visitor)
            }
        }
    };
}

deseralize_from_string!(VersionType, "property, properties, mvn, maven, cargo, rust");
deseralize_from_string!(
    ProjectArtifactCompressionFormat,
    "property, properties, mvn, maven, cargo, rust"
);
deseralize_from_string!(ProjectArtifactTarget, "github");
