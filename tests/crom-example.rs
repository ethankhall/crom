extern crate assert_cmd;
extern crate predicates;

mod lib;

use std::fs::File;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempdir::TempDir;

#[test]
#[cfg(unix)]
fn can_list_current_version() {
    let tmp_dir = TempDir::new("test-dir").expect("temp dir should be created");
    let tmp_dir = tmp_dir.path().to_owned();
    lib::checkout_repo(tmp_dir.clone());

    let mut builder = tmp_dir.to_path_buf();
    builder.push("example-1");

    println!("Finished clone");

    let mut cmd = Command::cargo_bin("crom").unwrap();
    let assert = cmd
        .arg("get")
        .arg("latest")
        .current_dir(builder.clone())
        .assert();

    assert.success().stdout(predicate::str::similar(format!(
        "{}\n",
        lib::CURRENT_VERSION
    )));

    let foo_txt = builder.with_file_name("foo.txt");
    File::create(&foo_txt).expect(&format!("Should be able to create foo file: {:?}", foo_txt));
    lib::add_file(tmp_dir, foo_txt);

    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .current_dir(builder.clone())
        .output()
        .unwrap();

    let mut cmd = Command::cargo_bin("crom").unwrap();
    let assert = cmd
        .arg("get")
        .arg("pre-release")
        .current_dir(builder.clone())
        .assert();

    assert.success().stdout(predicate::str::similar(format!(
        "{}-{}",
        lib::NEXT_VERSION,
        String::from_utf8(output.stdout).unwrap()
    )));
}
