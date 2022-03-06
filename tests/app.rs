use assert_cmd::Command;
const PROGRAM_NAME: &str = "sire";

#[test]
fn show_usage() {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PROGRAM_NAME)
            .expect("missing program")
            .arg(flag)
            .assert()
            .stdout(predicates::str::contains("USAGE"));
    }
}

#[test]
fn missing_paarams() {
    Command::cargo_bin(PROGRAM_NAME)
        .expect("missing program")
        .assert()
        .failure();
}

#[test]
fn basic_success() {
    Command::cargo_bin(PROGRAM_NAME)
        .expect("missing program")
        .args(["--source-path", "."])
        .assert()
        .stdout(predicates::str::contains("Hello, world!"));
}
