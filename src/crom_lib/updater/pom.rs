use std::path::PathBuf;
use std::process::*;

use crate::crom_lib::error::*;
use crate::crom_lib::Version;

pub struct PomUpdater;

impl PomUpdater {
    pub fn update_version(root_path: PathBuf, version: &Version) -> Result<(), ErrorContainer> {
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
                return Err(ErrorContainer::Updater(
                    UpdaterError::UnableToStartMavenProcess(e.to_string()),
                ));
            }
        };

        let ecode = match child.wait() {
            Ok(code) => code,
            Err(e) => {
                return Err(ErrorContainer::Updater(
                    UpdaterError::UnableToStartMavenProcess(e.to_string()),
                ));
            }
        };

        if !ecode.success() {
            Err(ErrorContainer::Updater(
                UpdaterError::MavenVersionSetFailed(ecode.code().unwrap_or(1)),
            ))
        } else {
            Ok(())
        }
    }
}
