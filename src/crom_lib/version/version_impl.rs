use std::cmp::{min, Ord, Ordering};
use std::fmt::{Display, Formatter};

use super::*;

impl Ord for Version {
    fn cmp(&self, other: &Version) -> Ordering {
        let self_str = self.to_string();
        let other_str = other.to_string();

        let self_parts: Vec<&str> = self_str.split('.').collect();
        let other_parts: Vec<&str> = other_str.split('.').collect();

        let end = min(self_parts.len(), other_parts.len());
        for i in 0..end {
            let other_part = other_parts[i];
            let self_part = self_parts[i];

            match other_part.cmp(&self_part) {
                Ordering::Equal => continue,
                Ordering::Less => return Ordering::Greater,
                Ordering::Greater => return Ordering::Less,
            }
        }

        if other.parts.len() == self.parts.len() {
            Ordering::Equal
        } else if other.parts.len() > self.parts.len() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for Version {}

impl Version {
    pub fn new(parts: Vec<VersionComponent>, snapshot: bool) -> Version {
        let has_dynamic_version = parts.clone().into_iter().any(|x| match x {
            VersionComponent::Changing(_) => true,
            VersionComponent::Static(_) => false,
        });

        Version {
            parts,
            is_snapshot: snapshot,
            is_only_static: !has_dynamic_version,
        }
    }

    pub fn next_version(&self) -> Version {
        if self.is_only_static {
            warn!("Attempting to bump a static only version!");
        }

        let parts: Vec<VersionComponent> = self
            .parts
            .clone()
            .into_iter()
            .map(|x| match x {
                VersionComponent::Static(part) => VersionComponent::Static(part),
                VersionComponent::Changing(part) => VersionComponent::Changing(part + 1),
            })
            .collect();

        Version::new(parts, false)
    }

    pub fn self_without_snapshot(&self) -> Version {
        Version::new(self.parts.clone(), false)
    }

    pub fn next_snapshot(&self) -> Version {
        let mut next_version = self.next_version();
        next_version.is_snapshot = true;
        next_version
    }
}

impl From<String> for Version {
    fn from(input: String) -> Self {
        Version::new(vec![VersionComponent::Static(input)], false)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let parts: Vec<String> = self
            .parts
            .clone()
            .into_iter()
            .map(|x| match x {
                VersionComponent::Static(part) => part,
                VersionComponent::Changing(part) => format!("{}", part),
            })
            .collect();

        let joined = parts.join(".");
        if self.is_snapshot {
            write!(f, "{}-SNAPSHOT", joined)
        } else {
            write!(f, "{}", joined)
        }
    }
}

#[test]
fn test_next_version() {
    let matcher = VersionMatcher::new("1.2.3.%d");
    let version = matcher.match_version(s!("1.2.3.5")).unwrap();

    assert_eq!("1.2.3.5", version.to_string());
    assert_eq!("1.2.3.6", version.next_version().to_string());
}

#[test]
fn test_version_comparison() {
    let matcher = VersionMatcher::new("1.2.%d");

    let version_3 = matcher.match_version(s!("1.2.3")).unwrap();
    let version_4 = matcher.match_version(s!("1.2.4")).unwrap();

    let matcher_dot = VersionMatcher::new("1.2.3.%d");
    let version_3_3 = matcher_dot.match_version(s!("1.2.3.3")).unwrap();

    assert!(version_3 < version_4);
    assert!(version_3 < version_3_3);
    assert!(version_3_3 < version_4);
}
