#[derive(Debug, Clone, PartialEq)]
pub enum VersionComponent {
    Static(String),
    Changing(i32),
}

#[derive(Debug, Clone)]
pub struct Version {
    parts: Vec<VersionComponent>,
    is_snapshot: bool,
    is_only_static: bool,
}

#[derive(Debug)]
pub struct VersionMatcher {
    pattern: Vec<VersionComponent>,
}

pub enum CromVersionError {

}

mod version;
mod version_parser;