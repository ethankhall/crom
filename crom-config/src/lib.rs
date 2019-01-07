#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

#[macro_export]
macro_rules! s {
    ($x:expr) => {
        $x.to_string()
    };
}

mod version;
mod config;
mod repo;

pub use crate::version::Version;

pub static CONFIG_FILE: &'static str = ".crom.toml";

trait Project {
    fn find_latest_version() -> Version;
}