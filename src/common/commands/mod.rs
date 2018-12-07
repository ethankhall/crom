pub mod init {
    use std::path::Path;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::prelude::*;

    use toml;

    use crate::models::*;

    pub fn write_default_config<P: AsRef<Path>>(default_format: &str, dest: P) -> Result<(), CromError> {
        let project = ProjectConfig { pattern: format!("{}", default_format), version_files: vec![], included_paths: None };
        let mut projects_map: HashMap<String, ProjectConfig> = HashMap::new();
        projects_map.insert(String::from("default"), project);

        let crom_config = CromConfig { projects: projects_map };

        let mut file = File::create(dest)?;
        file.write_all(toml::to_string_pretty(&crom_config)?.as_bytes())?;
        Ok(())
    }
}

pub mod get {
    use std::path::PathBuf;
    use crate::models::*;
    use crate::git::Repo;

    static DEFAULT_VERSION: &'static i32 = &1;

    pub fn get_current_version(root_path: PathBuf, config: ProjectConfig) -> Result<String, CromError> {
        let repo = Repo::new(root_path)?;
        let version: Version = config.into();
        let regex = version.to_regex()?;

        let mut wildcard_version: Vec<i32> = repo.get_tags()?.into_iter()
            .filter(|tag| regex.is_match(tag))
            .map(|tag| regex.captures(&tag).unwrap()["sub"].to_string())
            .map(|part| part.parse::<i32>().unwrap())
            .collect();
        
        wildcard_version.sort();
        let part = wildcard_version.last().unwrap_or_else(|| DEFAULT_VERSION);
        return Ok(version.make_version_number(*part));
    }
}