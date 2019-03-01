extern crate assert_cmd;
extern crate predicates;

mod shared;

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
    shared::checkout_repo(tmp_dir.clone());

    let mut builder = tmp_dir.to_path_buf();
    builder.push("example-1");

    println!("Finished clone");

    let mut cmd = Command::main_binary().unwrap();
    let assert = cmd
        .arg("get")
        .arg("current-version")
        .current_dir(builder.clone())
        .assert();

    assert.success().stdout(predicate::str::similar(format!(
        "{}\n",
        shared::CURRENT_VERSION
    )));

    let foo_txt = builder.with_file_name("foo.txt");
    File::create(&foo_txt).expect(&format!("Should be able to create foo file: {:?}", foo_txt));
    shared::add_file(tmp_dir, foo_txt);

    let mut cmd = Command::main_binary().unwrap();
    let assert = cmd
        .arg("get")
        .arg("current-version")
        .current_dir(builder.clone())
        .assert();

    assert.success().stdout(predicate::str::similar(format!(
        "{}-SNAPSHOT\n",
        shared::NEXT_VERSION
    )));
}
