use std::path::PathBuf;
use std::process::*;

use super::UpdateVersion;
use crate::crom_lib::config::file::MavenConfig;
use crate::crom_lib::error::*;
use crate::crom_lib::Version;

impl UpdateVersion for MavenConfig {
    fn update_version(&self, root_path: PathBuf, version: &Version) -> Result<(), CliErrors> {
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
                return Err(CliErrors::Updater(UpdaterError::UnableToStartMavenProcess(
                    e.to_string(),
                )));
            }
        };

        let ecode = match child.wait() {
            Ok(code) => code,
            Err(e) => {
                return Err(CliErrors::Updater(UpdaterError::UnableToStartMavenProcess(
                    e.to_string(),
                )));
            }
        };

        if !ecode.success() {
            Err(CliErrors::Updater(UpdaterError::MavenVersionSetFailed(
                ecode.code().unwrap_or(1),
            )))
        } else {
            Ok(())
        }
    }
}
