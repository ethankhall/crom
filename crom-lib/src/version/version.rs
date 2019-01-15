use std::fmt::{Display, Formatter};

use super::*;

impl Version {
    pub fn new(parts: Vec<VersionComponent>, snapshot: bool) -> Version {
        let has_dynamic_version = parts.clone().into_iter().any(|x| match x {
            VersionComponent::Changing(_) => true,
            VersionComponent::Static(_) => false,
        });

        return Version {
            parts: parts,
            is_snapshot: snapshot,
            is_only_static: !has_dynamic_version,
        };
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
        match self.is_snapshot {
            false => write!(f, "{}", joined),
            true => write!(f, "{}-SNAPSHOT", joined),
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
