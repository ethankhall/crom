extern crate assert_cmd;
extern crate mockito;
extern crate predicates;

mod lib;

use std::process::Command;

use assert_cmd::prelude::*;
use mockito::mock;
use predicates::prelude::*;
use tempdir::TempDir;

#[cfg(unix)]
#[tokio::test]
async fn can_tag_version() {
    let mock = mock("POST", "/repos/ethankhall/crom-examples/releases")
        .match_header("accept", "application/vnd.github.v3+json")
        .match_header("authorization", "bearer ABC123")
        .with_status(201)
        .with_body("{\"test\": true}")
        .create();

    let tmp_dir = TempDir::new("test-dir").expect("temp dir should be created");
    let tmp_dir = tmp_dir.path().to_owned();
    lib::checkout_repo(tmp_dir.clone());

    let mut builder = tmp_dir;
    builder.push("example-1");

    println!("Finished clone");

    let mut cmd = Command::cargo_bin("crom").unwrap();
    let assert = cmd
        .arg("tag")
        .arg("next-release")
        .arg("--github")
        .arg("--local")
        .arg("-vvvv")
        .env("GITHUB_API_SERVER", mockito::server_url())
        .env("GITHUB_TOKEN", "ABC123")
        .current_dir(builder.clone())
        .assert();

    println!(
        "{}",
        std::str::from_utf8(&assert.success().get_output().stdout).unwrap()
    );

    mock.assert();

    let mut cmd = Command::cargo_bin("crom").unwrap();
    let assert = cmd
        .arg("get")
        .arg("latest")
        .current_dir(builder.clone())
        .assert();

    assert
        .success()
        .stdout(predicate::str::similar(format!("{}\n", lib::NEXT_VERSION)));
}
