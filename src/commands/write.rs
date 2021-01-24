use async_trait::async_trait;
use std::path::PathBuf;
use error_chain::bail;

use std::fs::File;
use std::io::prelude::*;
use std::fs::read_to_string;
use std::process::*;

use toml_edit::{value, Document};
use serde_json::{self, Value};
use ini::Ini;

use crate::cli::{WriteArgs};
use crate::version::Version;
use crate::CromResult;
use crate::errors::ErrorKind;
use crate::models::{CargoConfig, NodeConfig, MavenConfig, PropertyFileConfig, VersionPyConfig};

pub struct WriteCommand;

#[async_trait]
impl super::CommandRunner<WriteArgs> for WriteCommand {
    async fn run_command(args: WriteArgs) -> CromResult<i32> {
        let (version, location, config) = super::create_version(args.sub_command.make_version_request()).await?;

        if let Some(project) = config.project.cargo {
            project.update_version(location.clone(), &version)?;
        }

        if let Some(project) = config.project.property {
            project.update_version(location.clone(), &version)?;
        }

        if let Some(project) = config.project.maven {
            project.update_version(location.clone(), &version)?;
        }

        if let Some(project) = config.project.package_json {
            project.update_version(location.clone(), &version)?;
        }

        if let Some(project) = config.project.version_py {
            project.update_version(location, &version)?;
        }

        Ok(0)
    }
}

trait UpdateVersion {
    fn update_version(&self, root_path: PathBuf, version: &Version) -> CromResult<()>;
}

impl UpdateVersion for CargoConfig {
    fn update_version(&self, root_path: PathBuf, version: &Version) -> CromResult<()> {
        let mut path = root_path.clone();

        if let Some(dir) = &self.directory {
            path.push(dir);
        }

        path.push(crate::statics::CARGO_TOML);

        let text = read_to_string(&path)?;
        let doc = text.parse::<Document>().expect("invalid doc");

        if doc["workspace"].is_none() {
            update_version_in_cargo_toml(path, version)
        } else {
            match doc["workspace"]["members"].as_array() {
                Some(arr) => {
                    for item in arr.iter() {
                        let name = match item.as_str() {
                            Some(s) => s,
                            None => {
                                bail!(ErrorKind::InvalidToml(
                                    "Cargo.toml for workspace.members was not a string.".to_string()
                                ))
                            }
                        };

                        let mut my_path = root_path.clone();
                        my_path.push(name);
                        my_path.push(crate::statics::CARGO_TOML);
                        update_version_in_cargo_toml(my_path, version)?;
                    }
                }
                None => {
                    bail!(ErrorKind::InvalidToml(
                        "Cargo.toml for workspace was missing members.".to_string()
                    ))
                }
            }

            Ok(())
        }
    }
}

fn update_version_in_cargo_toml(path: PathBuf, version: &Version) -> CromResult<()> {
    let text = read_to_string(&path)?;
    let mut doc = text.parse::<Document>().expect("invalid doc");

    let mut version_str = s!(version);

    if version_str.starts_with('v') {
        version_str = version_str.replacen('v', "", 1);
    }

    doc["package"]["version"] = value(version_str);

    let toml_string = doc.to_string();
    let toml_bytes = toml_string.as_bytes();

    let mut file = File::create(path)?;
    file.write_all(toml_bytes)?;

    Ok(())
}

impl UpdateVersion for NodeConfig {
    fn update_version(&self, root_path: PathBuf, version: &Version) -> CromResult<()> {
        let mut path = root_path;

        if let Some(dir) = &self.directory {
            path.push(dir);
        }

        path.push(crate::statics::PACKAGE_JSON);

        let text = read_to_string(&path)?;

        let mut json: Value = serde_json::from_str(&text)?;

        json["version"] = Value::String(version.to_string());

        let text = serde_json::to_string_pretty(&json)?;

        let mut file = File::create(path)?;
        file.write_all(text.as_bytes())?;
        Ok(())
    }
}


impl UpdateVersion for MavenConfig {
    fn update_version(&self, root_path: PathBuf, version: &Version) -> CromResult<()> {
        let spawn = Command::new("mvn")
            .current_dir(root_path)
            .args(&[
                "versions:set",
                &format!("-DnewVersion={}", version),
                "-DprocessAllModules=true",
            ])
            .spawn();

        let mut child = match spawn {
            Ok(child) => child,
            Err(e) => {
                bail!(ErrorKind::Maven(
                    e.to_string(),
                ))
                        }
        };

        let ecode = match child.wait() {
            Ok(code) => code,
            Err(e) => {
                bail!(ErrorKind::Maven(
                    e.to_string(),
                ))
            }
        };

        if !ecode.success() {
            bail!(ErrorKind::Maven(
                "Maven wasn't able to set version".to_string()
            ))
        } else {
            Ok(())
        }
    }
}

impl UpdateVersion for PropertyFileConfig {
    fn update_version(&self, root_path: PathBuf, version: &Version) -> CromResult<()> {
        let mut path = root_path;
        path.push(self.path.clone());

        let text = read_to_string(&path)?;

        let mut conf: Ini = Ini::load_from_str(&text)?;

        conf.with_section(None::<String>)
            .set("version", version.to_string());

        let mut version_file_buffer = Vec::new();
        conf.write_to(&mut version_file_buffer).unwrap();

        let version_text = String::from_utf8(version_file_buffer)?;

        let mut file = File::create(path)?;
        file.write_all(version_text.as_bytes())?;
        Ok(())
    }
}

impl UpdateVersion for VersionPyConfig {
    fn update_version(&self, root_path: PathBuf, version: &Version) -> CromResult<()> {
        let mut path = root_path;
        path.push(self.path.clone());

        let version_text = format!("__version__ = \"{}\"", version);

        let mut file = File::create(path)?;
        file.write_all(version_text.as_bytes())?;
        Ok(())
    }
}
