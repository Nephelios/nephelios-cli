use assert_cmd::Command;

/// Tests the CLI help command output.
///
/// Verifies that the help command executes successfully and
/// contains the expected help text.
///
/// # Returns
///
/// * `()` - The test passes if the assertions are successful
/// * Panics if the assertions fail
#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("nephelios-cli").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("Nephelios CLI tool"));
}
