use assert_cmd::Command;

#[test]
fn report_contains_totals_and_details() {
    let input = r#"
{"line":1,"timestamp":"2024-03-10T12:00:01Z","level":"INFO","message":"Starting service"}
{"line":2,"timestamp":"2024-03-10T12:00:03Z","level":"WARN","message":"Slow response from /api/orders","fields":{"duration_ms":650}}
{"line":3,"timestamp":"2024-03-10T12:00:10Z","level":"ERROR","message":"Timeout while calling payment gateway","fields":{"duration_ms":1200}}
{"line":4,"timestamp":"2024-03-10T12:00:30Z","level":"ERROR","message":"Payment failed"}
{"line":5,"timestamp":"2024-03-10T12:00:32Z","level":"ERROR","message":"Timeout while calling payment gateway"}
{"line":6,"timestamp":"2024-03-10T12:00:33Z","level":"ERROR","message":"Timeout while calling payment gateway"}
{"line":7,"timestamp":"2024-03-10T12:05:33Z","level":"INFO","message":"Background job started"}
{"line":8,"timestamp":"2024-03-10T12:05:34Z","level":"INFO","message":"Completed request","fields":{"duration_ms":1200}}
{"line":9,"timestamp":"2024-03-10T12:05:35Z","level":"INFO","message":"/api/slow","fields":{"duration_ms":2000}}
{"line":10,"timestamp":"2024-03-10T12:05:37Z","level":"ERROR","message":"Timeout while calling payment gateway"}
"#;

    let assert = Command::cargo_bin("frictiondetect")
        .unwrap()
        .arg("analyse")
        .write_stdin(input)
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert!(stdout.contains("FrictionDetect Report"));
    assert!(stdout.contains("Lines analysed: 10"));
    assert!(stdout.contains("Errors: 5"));
    assert!(stdout.contains("Warnings: 1"));
    assert!(stdout.contains("Slow operations: 4"));
    assert!(stdout.contains("Long gaps: 1"));
    assert!(stdout.contains("Repeated failures: 1"));
    assert!(stdout.contains("[1200 ms]   2024-03-10T12:05:34Z   Completed request"));
}

#[test]
fn standalone_jsonl_without_line_numbers_defaults_to_one_based_index() {
    let input = concat!(
        r#"{"timestamp":"2024-03-10T12:00:01Z","level":"INFO","message":"Boot","fields":null}"#,
        "\n",
        r#"{"timestamp":"2024-03-10T12:00:02Z","level":"ERROR","message":"DB failure","fields":{"duration_ms":900}}"#,
        "\n",
    );

    let assert = Command::cargo_bin("frictiondetect")
        .unwrap()
        .arg("analyse")
        .write_stdin(input)
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert!(stdout.contains("Lines analysed: 2"));
    assert!(stdout.contains("Errors: 1"));
    assert!(stdout.contains("Slow operations: 1"));
    assert!(stdout.contains("[line 2]  2024-03-10T12:00:02Z  DB failure"));
}

#[test]
fn analyse_file_fixture_and_missing_file_error() {
    let assert = Command::cargo_bin("frictiondetect")
        .unwrap()
        .args(["analyse", "--file", "tests/fixtures/sample1.jsonl"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(stdout.contains("Lines analysed: 10"));
    assert!(stdout.contains("Errors: 5"));
    assert!(stdout.contains("Long gaps: 1"));

    Command::cargo_bin("frictiondetect")
        .unwrap()
        .args(["analyse", "--file", "tests/fixtures/does-not-exist.jsonl"])
        .assert()
        .failure();
}

#[test]
fn empty_stdin_produces_zero_counts() {
    let assert = Command::cargo_bin("frictiondetect")
        .unwrap()
        .arg("analyse")
        .write_stdin("\n   \n")
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(stdout.contains("Lines analysed: 0"));
    assert!(stdout.contains("Errors: 0"));
}

#[test]
fn malformed_json_aborts_with_error() {
    let input = "{\"line\":1,\"level\":\"INFO\",\"message\":\"ok\"}\nnot valid json\n";

    Command::cargo_bin("frictiondetect")
        .unwrap()
        .arg("analyse")
        .write_stdin(input)
        .assert()
        .failure();
}

#[test]
fn about_strips_escape_codes_when_piped() {
    let assert = Command::cargo_bin("frictiondetect")
        .unwrap()
        .arg("about")
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(stdout.contains("FrictionDetect"));
    assert!(stdout.contains("Mohammed Usman"));
    assert!(!stdout.contains("\x1b["));
    assert!(!stdout.contains("\x1b]8;;"));
}

#[test]
fn fd_alias_and_invalid_subcommand() {
    Command::cargo_bin("fd")
        .unwrap()
        .arg("--version")
        .assert()
        .success();

    let assert = Command::cargo_bin("frictiondetect")
        .unwrap()
        .arg("not-a-command")
        .assert()
        .failure();

    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(!stderr.is_empty());
}