use std::{env, fs};

use assert_cmd::Command;

#[test]
fn success() {
    let mut path = env::current_dir().unwrap();
    path.push("tests");
    path.push("test_project");

    let mut cmd = Command::cargo_bin("sire").unwrap();
    cmd.args(&["-s", path.as_os_str().to_str().unwrap(), "-d", "/tmp/test"]);
    cmd.assert().success();

    fs::remove_dir_all("/tmp/test").expect("Cannot remove test project artifacts");
}
