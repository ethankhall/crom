use std::path::PathBuf;
use std::process::Command;

pub const CURRENT_VERSION: &str = "v0.1.5";
pub const NEXT_SNAPSHOT_VERSION: &str = "v0.1.6-SNAPSHOT";
pub const NEXT_VERSION: &str = "v0.1.6";

pub fn checkout_repo(path: PathBuf) {
    println!("Cloning from test repo");

    let mut child = Command::new("git")
        .arg("clone")
        .arg("https://github.com/ethankhall/crom-examples.git")
        .arg(path.to_str().unwrap())
        .spawn()
        .expect("failed to execute process");

    let ecode = child.wait().expect("failed to wait on child");

    assert!(ecode.success());
}
