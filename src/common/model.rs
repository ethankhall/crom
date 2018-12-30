use std::fmt::{Display, Formatter, self};
use std::path::PathBuf;

use crate::error::*;

pub struct Artifact {
    pub name: String,
    pub file_path: PathBuf
}

#[derive(Debug, Clone)]
pub struct Version {
    parts: Vec<VersionComponent>,
    is_snapshot: bool,
    is_only_static: bool
}

impl Version {
    fn new(parts: Vec<VersionComponent>, snapshot: bool) -> Version {
        let has_dynamic_version = parts.clone().into_iter().any(|x| {
            match x {
                VersionComponent::Changing(_) => true,
                VersionComponent::Static(_) => false
            }
        });

        return Version { parts: parts, is_snapshot: snapshot, is_only_static: !has_dynamic_version }
    }

    pub fn next_version(&self) -> Version {
        if self.is_only_static {
            warn!("Attempting to bump a static only version!");
        }

        let parts: Vec<VersionComponent> = self.parts.clone().into_iter().map(|x| {
            match x {
                VersionComponent::Static(part) => VersionComponent::Static(part),
                VersionComponent::Changing(part) => VersionComponent::Changing(part + 1)
            }
        }).collect();

        return Version::new(parts, false);
    }

    pub fn self_without_snapshot(&self) -> Version {
        return Version::new(self.parts.clone(), false);
    }

    pub fn next_snapshot(&self) -> Version {
        let mut next_version = self.next_version();
        next_version.is_snapshot = true;
        return next_version;
    }
}

impl From<String> for Version {
    fn from(input: String) -> Self {
        return Version::new(vec![VersionComponent::Static(input)], false);
    }
}

#[derive(Debug, Clone, PartialEq)]
enum VersionComponent {
    Static(String),
    Changing(i32)
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let parts: Vec<String> = self.parts.clone().into_iter().map(|x| {
            match x {
                VersionComponent::Static(part) => part,
                VersionComponent::Changing(part) => format!("{}", part)
            }
        }).collect();

        let joined = parts.join(".");
        match self.is_snapshot {
            false => write!(f, "{}", joined),
            true => write!(f, "{}-SNAPSHOT", joined)
        }
    }
}

#[derive(Debug)]
enum VersionMatchComponent {
    Static(String),
    Dynamic
}

#[derive(Debug)]
pub struct VersionMatcher {
    pattern: Vec<VersionMatchComponent>
}

impl VersionMatcher {
    pub fn new(pattern: String) -> Result<Self, CromError> {
        let split: Vec<&str> = pattern.split(".").collect();
        let parts: Vec<VersionMatchComponent> = split.into_iter().map(|x| {
            return match x {
                "%d" => VersionMatchComponent::Dynamic,
                _ => VersionMatchComponent::Static(x.to_string())
            };
        }).collect();

        return Ok(VersionMatcher{ pattern: parts });
    }

    pub fn build_default_version(&self, default: i32) -> Version {
        let pattern = &self.pattern;
        let parts: Vec<VersionComponent> = pattern.into_iter().map(|x| {
            match x {
                VersionMatchComponent::Static(v) => VersionComponent::Static(s!(v)),
                VersionMatchComponent::Dynamic => VersionComponent::Changing(default)
            }
        }).collect();

        return Version::new(parts, false);
    }

    pub fn match_version(&self, input: String) -> Option<Version> {
        let split: Vec<&str> = input.split(".").collect();

        if split.len() != self.pattern.len() {
            return None;
        }

        let mut version_parts: Vec<VersionComponent> = Vec::new();

        for i in 0..split.len() {
            let pattern_part = self.pattern.get(i).unwrap();
            let split_part = split.get(i).unwrap();

            match pattern_part {
                VersionMatchComponent::Static(value) => {
                    if value != split_part { 
                        return None 
                    } else { 
                        version_parts.push(VersionComponent::Static(s!(value)));
                    }
                },
                VersionMatchComponent::Dynamic => {
                    let parsed = match split_part.parse::<i32>() {
                        Err(_) => return None,
                        Ok(v) => v
                    };

                    version_parts.push(VersionComponent::Changing(parsed));
                }
            }
        }

        return Some(Version::new(version_parts, false));
    }
}

#[test]
fn parse_semver() {
    let matcher = VersionMatcher::new(s!("1.2.%d")).unwrap();

    assert_eq!(s!("1.2.3"), matcher.match_version(s!("1.2.3")).unwrap().to_string());
    assert_ne!(s!("2.2.3"), matcher.match_version(s!("1.2.3")).unwrap().to_string());
    assert!(matcher.match_version(s!("1.2")).is_none());
    assert!(matcher.match_version(s!("1.2.3.4")).is_none());

    let matcher = VersionMatcher::new(s!("a.b.%d")).unwrap();
    
    assert_eq!(s!("a.b.3"), matcher.match_version(s!("a.b.3")).unwrap().to_string());
}