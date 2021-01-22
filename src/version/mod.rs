#[derive(Debug, Clone, Eq)]
pub enum VersionComponent {
    Static(String),
    Changing(i32),
}

#[derive(Debug, Clone, Eq)]
pub struct Version {
    parts: Vec<VersionComponent>,
    is_only_static: bool,
    pre_release: Option<String>,
}

#[derive(Debug)]
pub struct VersionMatcher {
    pattern: Vec<VersionComponent>,
}

mod version_impl;
mod version_parser;
