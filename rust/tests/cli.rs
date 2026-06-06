/// Integration tests for the gvibu multicall binary.
/// Tests run the compiled binary via subprocess, exercising both
/// subcommand mode (`gvibu <command> [args]`) and symlink-like dispatch.

use std::process::Command;

/// Path to the gvibu binary. Cargo sets CARGO_BIN_EXE_gvibu during test runs.
fn gvibu_bin() -> &'static str {
    env!("CARGO_BIN_EXE_gvibu")
}

fn run(args: &[&str]) -> (i32, String, String) {
    let output = Command::new(gvibu_bin())
        .args(args)
        .output()
        .expect("failed to run gvibu");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

fn run_with_timeout(args: &[&str], timeout_ms: u64) -> (Option<i32>, String, String) {
    let mut cmd = Command::new(gvibu_bin());
    cmd.args(args);
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());
    let mut child = cmd.spawn().expect("failed to spawn gvibu");

    std::thread::sleep(std::time::Duration::from_millis(timeout_ms));
    let _ = child.kill();

    let output = child.wait_with_output().expect("failed to wait");
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

fn run_with_stdin(args: &[&str], stdin: &str) -> (i32, String, String) {
    let mut cmd = Command::new(gvibu_bin());
    cmd.args(args);
    let mut child = cmd
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to run gvibu");
    use std::io::Write;
    child
        .stdin
        .take()
        .unwrap()
        .write_all(stdin.as_bytes())
        .expect("failed to write stdin");
    let output = child.wait_with_output().expect("failed to read output");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

// ---------------------------------------------------------------------------
// true
// ---------------------------------------------------------------------------
#[test]
fn true_no_args() {
    let (code, out, err) = run(&["true"]);
    assert_eq!(code, 0, "true should exit 0; stderr: {err}");
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn true_with_args() {
    let (code, out, err) = run(&["true", "a", "b"]);
    assert_eq!(code, 0);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

// ---------------------------------------------------------------------------
// false
// ---------------------------------------------------------------------------
#[test]
fn false_no_args() {
    let (code, out, err) = run(&["false"]);
    assert_eq!(code, 1, "false should exit 1; stderr: {err}");
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn false_with_args() {
    let (code, out, err) = run(&["false", "foo", "bar"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

// ---------------------------------------------------------------------------
// echo
// ---------------------------------------------------------------------------
#[test]
fn echo_no_args() {
    let (code, out, err) = run(&["echo"]);
    assert_eq!(code, 0);
    assert_eq!(out, "\n");
    assert_eq!(err, "");
}

#[test]
fn echo_with_args() {
    let (code, out, err) = run(&["echo", "hello", "world"]);
    assert_eq!(code, 0);
    assert_eq!(out, "hello world\n");
    assert_eq!(err, "");
}

#[test]
fn echo_no_newline() {
    let (code, out, err) = run(&["echo", "-n", "hello"]);
    assert_eq!(code, 0);
    assert_eq!(out, "hello");
    assert_eq!(err, "");
}

#[test]
fn echo_dash_n_not_first() {
    let (code, out, err) = run(&["echo", "hello", "-n", "world"]);
    assert_eq!(code, 0);
    assert_eq!(out, "hello -n world\n");
    assert_eq!(err, "");
}

// ---------------------------------------------------------------------------
// pwd
// ---------------------------------------------------------------------------
#[test]
fn pwd_no_args() {
    let (code, out, err) = run(&["pwd"]);
    assert_eq!(code, 0);
    assert!(!out.is_empty(), "pwd should print something");
    assert!(out.ends_with('\n'), "pwd should end with newline");
    assert_eq!(err, "");
}

#[test]
fn pwd_with_args() {
    let (code, out, err) = run(&["pwd", "extra"]);
    assert_eq!(code, 2, "pwd with args should exit 2 (usage error)");
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print usage error");
}

// ---------------------------------------------------------------------------
// basename
// ---------------------------------------------------------------------------
#[test]
fn basename_no_args() {
    let (code, out, err) = run(&["basename"]);
    assert_eq!(code, 1, "basename without args should exit 1");
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print usage error");
}

#[test]
fn basename_with_suffix() {
    let (code, out, err) = run(&["basename", "/usr/local/bin/testfile.txt", ".txt"]);
    assert_eq!(code, 0);
    assert_eq!(out, "testfile\n");
    assert_eq!(err, "");
}

#[test]
fn basename_no_suffix() {
    let (code, out, err) = run(&["basename", "/usr/local/bin/testfile.txt"]);
    assert_eq!(code, 0);
    assert_eq!(out, "testfile.txt\n");
    assert_eq!(err, "");
}

#[test]
fn basename_trailing_slash() {
    let (code, out, err) = run(&["basename", "/a/b/c/"]);
    assert_eq!(code, 0);
    assert_eq!(out, "c\n");
    assert_eq!(err, "");
}

#[test]
fn basename_root_path() {
    let (code, out, err) = run(&["basename", "/"]);
    assert_eq!(code, 0);
    assert_eq!(out, "\n");
    assert_eq!(err, "");
}

#[test]
fn basename_suffix_equals_basename() {
    let (code, out, err) = run(&["basename", "foo", "foo"]);
    assert_eq!(code, 0);
    assert_eq!(out, "\n");
    assert_eq!(err, "");
}

// ---------------------------------------------------------------------------
// dirname
// ---------------------------------------------------------------------------
#[test]
fn dirname_no_args() {
    let (code, out, err) = run(&["dirname"]);
    assert_eq!(code, 2, "dirname without args should exit 2");
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print usage error");
}

#[test]
fn dirname_normal_path() {
    let (code, out, err) = run(&["dirname", "/usr/local/bin"]);
    assert_eq!(code, 0);
    assert_eq!(out, "/usr/local\n");
    assert_eq!(err, "");
}

#[test]
fn dirname_deep_path() {
    let (code, out, err) = run(&["dirname", "/a/b/c/file.txt"]);
    assert_eq!(code, 0);
    assert_eq!(out, "/a/b/c\n");
    assert_eq!(err, "");
}

#[test]
fn dirname_root_parent() {
    let (code, out, err) = run(&["dirname", "/usr"]);
    assert_eq!(code, 0);
    assert_eq!(out, "/\n");
    assert_eq!(err, "");
}

#[test]
fn dirname_root_itself() {
    let (code, out, err) = run(&["dirname", "/"]);
    assert_eq!(code, 0);
    assert_eq!(out, "/\n");
    assert_eq!(err, "");
}

#[test]
fn dirname_relative_path() {
    let (code, out, err) = run(&["dirname", "a/b"]);
    assert_eq!(code, 0);
    assert_eq!(out, "a\n");
    assert_eq!(err, "");
}

#[test]
fn dirname_simple_name() {
    let (code, out, err) = run(&["dirname", "foo"]);
    assert_eq!(code, 0);
    assert_eq!(out, ".\n");
    assert_eq!(err, "");
}

// ---------------------------------------------------------------------------
// cat
// ---------------------------------------------------------------------------
#[test]
fn cat_dev_null() {
    let (code, out, err) = run(&["cat", "/dev/null"]);
    assert_eq!(code, 0);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn cat_stdin_hello() {
    let (code, out, err) = run_with_stdin(&["cat"], "hello\n");
    assert_eq!(code, 0);
    assert_eq!(out, "hello\n");
    assert_eq!(err, "");
}

#[test]
fn cat_nonexistent_file() {
    let (code, out, err) = run(&["cat", "/nonexistent_test_file_xyz"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(err.contains("cat:"), "stderr should mention cat");
}

// ---------------------------------------------------------------------------
// wc
// ---------------------------------------------------------------------------
#[test]
fn wc_dev_null() {
    let (code, out, err) = run(&["wc", "/dev/null"]);
    assert_eq!(code, 0);
    assert_eq!(out, "      0       0       0 /dev/null\n");
    assert_eq!(err, "");
}

#[test]
fn wc_stdin() {
    let (code, out, err) = run_with_stdin(&["wc"], "hello world\n");
    assert_eq!(code, 0);
    assert_eq!(out, "      1       2      12\n");
    assert_eq!(err, "");
}

#[test]
fn wc_lines_only() {
    let (code, out, err) = run(&["wc", "-l", "/dev/null"]);
    assert_eq!(code, 0);
    assert_eq!(out, "      0 /dev/null\n");
    assert_eq!(err, "");
}

#[test]
fn wc_combined_flags() {
    let (code, out, err) = run(&["wc", "-lw", "/dev/null"]);
    assert_eq!(code, 0);
    assert_eq!(out, "      0       0 /dev/null\n");
    assert_eq!(err, "");
}

#[test]
fn wc_multi_file() {
    let (code, out, _err) = run(&["wc", "/dev/null", "/dev/null"]);
    assert_eq!(code, 0);
    assert!(out.contains("total"), "should include total line");
}

// ---------------------------------------------------------------------------
// head
// ---------------------------------------------------------------------------
#[test]
fn head_dev_null() {
    let (code, out, err) = run(&["head", "/dev/null"]);
    assert_eq!(code, 0);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn head_stdin_n1() {
    let (code, out, err) = run_with_stdin(&["head", "-n", "1"], "line1\nline2\n");
    assert_eq!(code, 0);
    assert_eq!(out, "line1\n");
    assert_eq!(err, "");
}

#[test]
fn head_nonexistent_file() {
    let (code, out, err) = run(&["head", "/nonexistent_test_file_xyz"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(err.contains("head:"), "stderr should mention head");
}

#[test]
fn head_multi_file_dev_null() {
    let (code, out, err) = run(&["head", "/dev/null", "/dev/null"]);
    assert_eq!(code, 0);
    assert!(out.contains("==>"), "multi-file output should have headers");
    assert_eq!(err, "");
}

// ---------------------------------------------------------------------------
// yes
// ---------------------------------------------------------------------------
#[test]
fn yes_default_first_line() {
    let (code, out, err) = run_with_timeout(&["yes"], 100);
    assert_eq!(code, None, "yes should be killed, not exit on its own");
    assert!(out.starts_with("y\n"), "first line should be 'y'");
    assert_eq!(err, "");
}

#[test]
fn yes_with_args_first_line() {
    let (code, out, err) = run_with_timeout(&["yes", "hello", "world"], 100);
    assert_eq!(code, None, "yes should be killed, not exit on its own");
    assert!(out.starts_with("hello world\n"), "first line should be 'hello world'");
    assert_eq!(err, "");
}

// ---------------------------------------------------------------------------
// printenv
// ---------------------------------------------------------------------------
#[test]
fn printenv_known_var() {
    let (code, out, err) = run(&["printenv", "PATH"]);
    assert_eq!(code, 0);
    assert!(!out.is_empty(), "PATH should be set");
    assert!(out.ends_with('\n'));
    assert_eq!(err, "");
}

#[test]
fn printenv_unknown_var() {
    let (code, out, err) = run(&["printenv", "__GVIBU_NONEXISTENT_VAR_XYZ__"]);
    assert_eq!(code, 0);
    assert_eq!(out, "\n");
    assert_eq!(err, "");
}

#[test]
fn printenv_all() {
    let (code, out, err) = run(&["printenv"]);
    assert_eq!(code, 0);
    assert!(!out.is_empty(), "should print environment");
    assert!(out.contains("PATH="), "PATH should be in output");
    assert_eq!(err, "");
}

// ---------------------------------------------------------------------------
// sleep
// ---------------------------------------------------------------------------
#[test]
fn sleep_zero() {
    let (code, out, err) = run(&["sleep", "0"]);
    assert_eq!(code, 0);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn sleep_no_args() {
    let (code, out, err) = run(&["sleep"]);
    assert_eq!(code, 2);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print usage");
}

#[test]
fn sleep_invalid_number() {
    let (code, out, err) = run(&["sleep", "abc"]);
    assert_eq!(code, 2);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn sleep_negative() {
    let (code, out, err) = run(&["sleep", "-5"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}
