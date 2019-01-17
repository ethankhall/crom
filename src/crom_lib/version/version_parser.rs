use super::*;

impl VersionMatcher {
    pub fn new(pattern: &str) -> Self {
        let split: Vec<&str> = pattern.split('.').collect();
        let parts: Vec<VersionComponent> = split
            .into_iter()
            .map(|x| match x {
                "%d" => VersionComponent::Changing(0),
                _ => VersionComponent::Static(x.to_string()),
            })
            .collect();

        VersionMatcher { pattern: parts }
    }

    pub fn build_default_version(&self) -> Version {
        Version::new(self.pattern.clone(), false)
    }

    pub fn match_version(&self, input: String) -> Option<Version> {
        let split: Vec<&str> = input.split('.').collect();

        if split.len() != self.pattern.len() {
            return None;
        }

        let mut version_parts: Vec<VersionComponent> = Vec::new();

        for (i, split_part) in split.iter().enumerate() {
            let pattern_part = &self.pattern[i];

            match pattern_part {
                VersionComponent::Static(value) => {
                    version_parts.push(VersionComponent::Static(value.to_string()));
                }
                VersionComponent::Changing(_) => {
                    let parsed = match split_part.parse::<i32>() {
                        Err(_) => return None,
                        Ok(v) => v,
                    };

                    version_parts.push(VersionComponent::Changing(parsed));
                }
            }
        }

        Some(Version::new(version_parts, false))
    }
}

#[test]
fn parse_semver() {
    let matcher = VersionMatcher::new("1.2.%d");

    assert_eq!(
        s!("1.2.3"),
        matcher.match_version(s!("1.2.3")).unwrap().to_string()
    );
    assert_ne!(
        s!("2.2.3"),
        matcher.match_version(s!("1.2.3")).unwrap().to_string()
    );
    assert!(matcher.match_version(s!("1.2")).is_none());
    assert!(matcher.match_version(s!("1.2.3.4")).is_none());

    let matcher = VersionMatcher::new("a.b.%d");

    assert_eq!(
        s!("a.b.3"),
        matcher.match_version(s!("a.b.3")).unwrap().to_string()
    );
}
