use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use toml_edit::{value, Document};

use super::*;
use crate::crom_lib::{read_file_to_string, Version};

pub struct CargoUpdater;

impl CargoUpdater {
    pub fn update_version(root_path: PathBuf, version: &Version) -> Result<(), ErrorContainer> {
        let mut path = root_path.clone();

        path.push(crate::crom_lib::CARGO_TOML);

        let text = read_file_to_string(&path)?;
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
                                return Err(ErrorContainer::Updater(
                                    UpdaterError::CargoTomlNotValid(s!(
                                        "Cargo.toml for workspace.members was not a string."
                                    )),
                                ));
                            }
                        };

                        let mut my_path = root_path.clone();
                        my_path.push(name);
                        my_path.push(crate::crom_lib::CARGO_TOML);
                        update_version_in_cargo_toml(my_path, version)?;
                    }
                }
                None => {
                    return Err(ErrorContainer::Updater(UpdaterError::CargoTomlNotValid(
                        s!("Cargo.toml for workspace was missing members."),
                    )));
                }
            }

            Ok(())
        }
    }
}

fn update_version_in_cargo_toml(path: PathBuf, version: &Version) -> Result<(), ErrorContainer> {
    let text = read_file_to_string(&path)?;
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
