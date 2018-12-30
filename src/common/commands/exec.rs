use std::path::{PathBuf, Path};
use clap::ArgMatches;

use crate::error::*;
use crate::git::*;
use super::*;
use crate::updater::*;
use crate::github::*;

pub fn exec_update_version(args: &ArgMatches) -> Result<i32, CromError> {
    let (root_path, configs) = crate::config::find_and_parse_config()?;

    let project_name = args.value_of("project").unwrap_or("default");
    let project_config = match configs.projects.get(project_name) {
        Some(config) => config,
        None => {
            return Err(CromError::ConfigError(format!("Unable to find project {}", project_name)));
        }
    };

    let modifier = match args.is_present("no_snapshot")  {
        true => VersionModification::None,
        false => VersionModification::NoneOrSnapshot
    };

    let repo = Repo::new(root_path.clone())?;
    let latest_version = match args.value_of("override_version") {
        Some(version) => Version::from(s!(version)),
        None => get_latest_version(&repo, &project_config, modifier)?
    };

    if project_config.version_files.is_empty() {
        return Err(CromError::ConfigError(s!("No version files defined")));
    }

    for version_path in &project_config.version_files {
        let mut file_path = root_path.clone();
        file_path.push(Path::new(&version_path));
        if !file_path.exists() {
            error!("Unable to find {:?}", file_path.into_os_string());
            return Err(CromError::VersionFileNotFound);
        }

        update_version(file_path, &latest_version)?;
    }

    return Ok(0);
}

fn update_version(path: PathBuf, version: &Version) -> Result<(), CromError> {
    return match path.file_name() {
        Some(name) => {
            match name.to_str() {
                Some("version.properties") => PropertyUpdater::update_version(path, version),
                Some("pom.xml") => {Ok(())},
                Some("Cargo.toml") => CargoUpdater::update_version(path, version),
                Some(unknown) => Err(CromError::VersionFileFormatUnknown(s!(unknown))),
                None => Err(CromError::UnknownError(format!("Unable to get filename from {:?}", name)))
            }
        },
        None => Err(CromError::UnknownError(format!("Unable to get filename from {:?}", path)))
    }
}

impl From<&str> for Artifact {
    fn from(input: &str) -> Self {
        return if input.contains("=") {
            let split: Vec<&str> = input.split("=").collect();
            Artifact { name: s!(split[0]), file_path: PathBuf::from(split[1]) }
        } else {
            let path = PathBuf::from(input);
            let file_name = path.file_name();
            Artifact { name: s!(file_name.and_then(|x| x.to_str()).unwrap()), file_path: path }
        }
    }
}

pub fn exec_upload_artifacts(args: &ArgMatches) -> Result<i32, CromError> {  
    let files = args.values_of("FILE");
    let artifacts: Vec<Artifact> = files.unwrap().into_iter().map(|f| Artifact::from(f) ).collect();
    
    for artifact in &artifacts {
        if !artifact.file_path.exists() {
            return Err(CromError::UnknownError(format!("Unable to find {:?}", artifact.file_path)));
        }
    }

    let (root_path, configs) = crate::config::find_and_parse_config()?;

    let project_name = args.value_of("project").unwrap_or("default");
    let project_config = match configs.projects.get(project_name) {
        Some(config) => config,
        None => {
            return Err(CromError::ConfigError(format!("Unable to find project {}", project_name)));
        }
    };

    let repo = Repo::new(root_path)?;

    let versions = project_config.get_current_versions(&repo)?;
    let version = match versions.last() {
        Some(v) => v,
        None => {
            return Err(CromError::GitTagNotFound);
        }
    };

    GitHub::publish_artifact(&repo, &version, artifacts)?;
    return Ok(0);
}

pub fn exec_claim_version(args: &ArgMatches) -> Result<i32, CromError> {
    let (root_path, configs) = crate::config::find_and_parse_config()?;

    let project_name = args.value_of("project").unwrap_or("default");
    let project_config = match configs.projects.get(project_name) {
        Some(config) => config,
        None => {
            return Err(CromError::ConfigError(format!("Unable to find project {}", project_name)));
        }
    };

    let repo = Repo::new(root_path)?;

    if !args.is_present("ignore_changes") {
        match repo.is_working_repo_clean() {
            Ok(true) => {},
            Ok(false) => return Err(CromError::GitWorkspaceNotClean),
            Err(err) => {
                debug!("Error working with git repo: {:?}", err);
                return Err(err);
            }
        }
    } else {
        warn!("Skipping check for workspace changes.");
    }

    let versions = project_config.get_current_versions(&repo)?;

    let version = match versions.last() {
        Some(v) => v,
        None => {
            return Err(CromError::GitTagNotFound);
        }
    };

    let version = version.next_version();

    for source in args.values_of("source").unwrap() {
        let tag_result = match source {
            "local" => repo.tag_version(&version),
            "github" => GitHub::tag_version(&repo, &version),
            _ => unreachable!()
        };
        
        match tag_result {
            Ok(true) => {},
            Ok(false) => return Ok(1),
            Err(err) => return Err(err)
        };
    }

    info!("Created tag {}", version);
    Ok(0)
}