use std::path::PathBuf;
use std::process::Command;

pub const CURRENT_VERSION: &str = "v0.1.6";
pub const NEXT_VERSION: &str = "v0.1.7";

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

pub fn add_file(root: PathBuf, file: PathBuf) {
    let mut child = Command::new("git")
        .arg("add")
        .arg(file.to_str().unwrap())
        .current_dir(root)
        .spawn()
        .expect("failed to execute process");

    let ecode = child.wait().expect("failed to wait on child");

    assert!(ecode.success());
}
