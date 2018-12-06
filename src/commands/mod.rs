use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use toml;

use super::models::*;

pub fn write_default_config<P: AsRef<Path>>(default_format: &str, dest: P) -> Result<(), CromError> {
    let project = ProjectConfig { pattern: format!("{}", default_format), version_files: vec![], included_paths: None };
    let mut projects_map: HashMap<String, ProjectConfig> = HashMap::new();
    projects_map.insert(String::from("default"), project);

    let crom_config = CromConfig { projects: projects_map };

    let mut file = File::create(dest)?;
    file.write_all(toml::to_string_pretty(&crom_config)?.as_bytes())?;
    Ok(())
}