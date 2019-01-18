use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub const CURRENT_VERSION: &str = "v0.1.4";
pub const NEXT_SNAPSHOT_VERSION: &str = "v0.1.5-SNAPSHOT";
pub const NEXT_VERSION: &str = "v0.1.5";

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

pub fn copy_resource<T: Into<String>>(source_name: T, dest: PathBuf) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("resources");
    path.push(source_name.into());

    fs::copy(path, &dest).expect("copying file from test dir");
}
