use assert_cmd::Command;

#[test]
fn scaffolding_counts_lines() {
    let mut cmd = Command::cargo_bin("frictionlens").unwrap();

    cmd.write_stdin("{\"raw\":\"hello\"}\n{\"raw\":\"world\"}\n")
        .assert()
        .success()
        .stdout("FrictionLens scaffolding report:\nLines processed: 2\n");
}
