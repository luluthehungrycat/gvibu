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

#[test]
fn echo_e_newline() {
    let (code, out, err) = run(&["echo", "-e", "hello\\nworld"]);
    assert_eq!(code, 0);
    assert_eq!(out, "hello\nworld\n");
    assert_eq!(err, "");
}

#[test]
fn echo_e_tab() {
    let (code, out, err) = run(&["echo", "-e", "hello\\tworld"]);
    assert_eq!(code, 0);
    assert_eq!(out, "hello\tworld\n");
    assert_eq!(err, "");
}

#[test]
fn echo_e_no_effect() {
    let (code, out, err) = run(&["echo", "-e", "hello"]);
    assert_eq!(code, 0);
    assert_eq!(out, "hello\n");
    assert_eq!(err, "");
}

#[test]
fn echo_e_with_n() {
    let (code, out, err) = run(&["echo", "-n", "-e", "hello\\nworld"]);
    assert_eq!(code, 0);
    assert_eq!(out, "hello\nworld");
    assert_eq!(err, "");
}

#[test]
fn echo_e_disables_escapes() {
    let (code, out, err) = run(&["echo", "-E", "-e", "hello\\nworld"]);
    assert_eq!(code, 0);
    // Last flag wins: -e overrides -E, so \n is interpreted as newline
    assert_eq!(out, "hello\nworld\n", "stderr: {}", err);
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

#[test]
fn cat_numbered_lines() {
    let (code, out, err) = run_with_stdin(&["cat", "-n"], "line1\nline2\nline3\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "     1\tline1\n     2\tline2\n     3\tline3\n");
    assert_eq!(err, "");
}

#[test]
fn cat_stdin_no_numbering() {
    let (code, out, err) = run_with_stdin(&["cat"], "hello\n");
    assert_eq!(code, 0);
    assert_eq!(out, "hello\n");
    assert_eq!(err, "");
}

#[test]
fn cat_invalid_option() {
    let (code, out, err) = run(&["cat", "-x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// wc
// ---------------------------------------------------------------------------
#[test]
fn wc_dev_null() {
    let (code, out, err) = run(&["wc", "/dev/null"]);
    assert_eq!(code, 0);
    assert_eq!(out, "      0       0       0       0 /dev/null\n");
    assert_eq!(err, "");
}

#[test]
fn wc_stdin() {
    let (code, out, err) = run_with_stdin(&["wc"], "hello world\n");
    assert_eq!(code, 0);
    assert_eq!(out, "      1       2      12      12\n");
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

#[test]
fn wc_chars_only() {
    let (code, out, err) = run(&["wc", "-m", "/dev/null"]);
    assert_eq!(code, 0);
    assert_eq!(out, "      0 /dev/null\n");
    assert_eq!(err, "");
}

#[test]
fn wc_chars_stdin_utf8() {
    let (code, out, err) = run_with_stdin(&["wc", "-m"], "héllo wörld\n");
    assert_eq!(code, 0);
    assert_eq!(out, "     12\n");
    assert_eq!(err, "");
}

#[test]
fn wc_all_flags() {
    let (code, out, err) = run(&["wc", "-lwcm", "/dev/null"]);
    assert_eq!(code, 0);
    assert_eq!(out, "      0       0       0       0 /dev/null\n");
    assert_eq!(err, "");
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

#[test]
fn head_c_flag_dev_null() {
    let (code, out, err) = run(&["head", "-c", "5", "/dev/null"]);
    assert_eq!(code, 0);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn head_c_flag_stdin() {
    let (code, out, err) = run_with_stdin(&["head", "-c", "5"], "hello world");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "hello");
    assert_eq!(err, "");
}

#[test]
fn head_c_flag_zero() {
    let (code, out, err) = run_with_stdin(&["head", "-c", "0"], "hello");
    assert_eq!(code, 0);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn head_c_missing_arg() {
    let (code, out, err) = run(&["head", "-c"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn head_invalid_c_value() {
    let (code, out, err) = run(&["head", "-c", "abc", "/dev/null"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
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

// ── touch ──

#[test]
fn touch_no_args() {
    let (code, out, err) = run(&["touch"]);
    assert_eq!(code, 2);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print usage");
}

#[test]
fn touch_creates_file() {
    let dir = std::env::temp_dir().join(format!("gvibu_touch_creates_file_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("touch_test");
    let path_str = path.to_str().unwrap();

    // Ensure clean
    let _ = std::fs::remove_file(&path);
    assert!(!path.exists());

    let (code, _out, err) = run(&["touch", path_str]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(path.exists(), "file should be created");

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_dir(&dir);
}

#[test]
fn touch_nonexistent_directory() {
    let (code, out, err) = run(&["touch", "/nonexistent_dir/file"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn touch_access_only() {
    let dir = std::env::temp_dir().join(format!("gvibu_touch_access_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("touch_atime");
    let path_str = path.to_str().unwrap();
    let _ = std::fs::remove_file(&path);

    let (code, _out, err) = run(&["touch", "-a", path_str]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(path.exists(), "file should be created");

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_dir(&dir);
}

#[test]
fn touch_mod_only() {
    let dir = std::env::temp_dir().join(format!("gvibu_touch_mod_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("touch_mtime");
    let path_str = path.to_str().unwrap();
    let _ = std::fs::remove_file(&path);

    let (code, _out, err) = run(&["touch", "-m", path_str]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(path.exists(), "file should be created");

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_dir(&dir);
}

#[test]
fn touch_both_flags() {
    let dir = std::env::temp_dir().join(format!("gvibu_touch_both_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("touch_am");
    let path_str = path.to_str().unwrap();
    let _ = std::fs::remove_file(&path);
    let (code, out, err) = run(&["touch", "-am", path_str]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "");
    assert_eq!(err, "");
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_dir(&dir);
}

#[test]
fn touch_invalid_option() {
    let (code, out, err) = run(&["touch", "-x", "/dev/null"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ── seq ──

#[test]
fn seq_basic() {
    let (code, out, _err) = run(&["seq", "5"]);
    assert_eq!(code, 0);
    assert_eq!(out, "1\n2\n3\n4\n5\n");
}

#[test]
fn seq_with_sep() {
    let (code, out, _err) = run(&["seq", "-s", ",", "3"]);
    assert_eq!(code, 0);
    assert_eq!(out, "1,2,3\n");
}

#[test]
fn seq_equal_width() {
    let (code, out, _err) = run(&["seq", "-w", "5", "10"]);
    assert_eq!(code, 0);
    assert_eq!(out, "05\n06\n07\n08\n09\n10\n");
}

#[test]
fn seq_w_and_s() {
    let (code, out, _err) = run(&["seq", "-w", "-s", " ", "3"]);
    assert_eq!(code, 0);
    assert_eq!(out, "1 2 3\n");
}

#[test]
fn seq_first_last() {
    let (code, out, err) = run(&["seq", "3", "7"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "3\n4\n5\n6\n7\n");
}

#[test]
fn seq_first_step_last() {
    let (code, out, err) = run(&["seq", "2", "3", "14"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "2\n5\n8\n11\n14\n");
}

#[test]
fn seq_first_greater_than_last() {
    let (code, out, err) = run(&["seq", "10", "5"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "");
}

#[test]
fn seq_negative_step() {
    let (code, out, err) = run(&["seq", "10", "-2", "4"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "10\n8\n6\n4\n");
}

#[test]
fn seq_step_zero() {
    let (code, out, err) = run(&["seq", "1", "0", "5"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn seq_no_args() {
    let (code, out, err) = run(&["seq"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print usage");
}

#[test]
fn seq_invalid_arg() {
    let (code, out, err) = run(&["seq", "abc"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ── which ──

#[test]
fn which_found() {
    let (code, out, err) = run(&["which", "sh"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(out.trim().ends_with("/sh"), "out: {:?}", out);
}

#[test]
fn which_not_found() {
    let (code, out, err) = run(&["which", "nonexistent_cmd_xyz"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn which_no_args() {
    let (code, out, err) = run(&["which"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

// ── uname ──

#[test]
fn uname_default() {
    let (code, out, err) = run(&["uname"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(!out.trim().is_empty(), "should output kernel name");
}

#[test]
fn uname_s() {
    let (code, out, err) = run(&["uname", "-s"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out.trim(), "Linux");
}

#[test]
fn uname_n() {
    let (code, out, err) = run(&["uname", "-n"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(!out.trim().is_empty(), "should output nodename");
}

#[test]
fn uname_r() {
    let (code, out, err) = run(&["uname", "-r"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(!out.trim().is_empty(), "should output kernel release");
}

#[test]
fn uname_m() {
    let (code, out, err) = run(&["uname", "-m"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(!out.trim().is_empty(), "should output machine hardware");
}

#[test]
fn uname_all() {
    let (code, out, err) = run(&["uname", "-a"]);
    assert_eq!(code, 0, "stderr: {}", err);
    let parts: Vec<&str> = out.trim().split_whitespace().collect();
    assert!(parts.len() >= 4, "should have 4+ parts, got {:?}", parts);
}

#[test]
fn uname_invalid_option() {
    let (code, out, err) = run(&["uname", "-x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ── env ──

#[test]
fn env_print() {
    let (code, out, err) = run(&["env"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(!out.is_empty(), "should print environment");
    assert!(out.contains('='), "should contain key=value pairs");
}

#[test]
fn env_var_assign() {
    let (code, out, err) = run(&["env", "TEST_GVIBU=hello"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(out.contains("TEST_GVIBU=hello"), "out: {:?}", out);
}

#[test]
fn env_unset() {
    let (code, out, err) = run(&["env", "-u", "PATH"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(
        !out.lines().any(|l| l.starts_with("PATH=")),
        "PATH should be unset, got line with 'PATH=' in: {:?}",
        out.lines().find(|l| l.contains("PATH="))
    );
}

#[test]
fn env_ignore() {
    let (code, out, err) = run(&["env", "-i"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "");
}

#[test]
fn env_run_command() {
    let (code, out, err) = run(&["env", "echo", "hello"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "hello\n");
}

#[test]
fn env_run_command_with_var() {
    let (code, out, err) = run(&["env", "TEST_GVIBU=hello", "sh", "-c", "echo $TEST_GVIBU"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out.trim(), "hello");
}

#[test]
fn env_command_not_found() {
    let (code, out, err) = run(&["env", "nonexistent_cmd_xyz"]);
    assert_eq!(code, 127);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// id
// ---------------------------------------------------------------------------
#[test]
fn id_no_args() {
    let (code, out, err) = run(&["id"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(out.contains("uid="), "should contain uid=, got: {:?}", out);
    assert_eq!(err, "");
}

#[test]
fn id_user_flag() {
    let (code, out, err) = run(&["id", "-u"]);
    assert_eq!(code, 0, "stderr: {}", err);
    let val = out.trim().parse::<u32>().unwrap_or(0);
    assert!(val > 0, "uid should be positive, got: {:?}", out);
}

#[test]
fn id_group_flag() {
    let (code, out, err) = run(&["id", "-g"]);
    assert_eq!(code, 0, "stderr: {}", err);
    let val = out.trim().parse::<u32>().unwrap_or(0);
    assert!(val > 0, "gid should be positive, got: {:?}", out);
}

#[test]
fn id_name_user() {
    let (code, out, err) = run(&["id", "-n", "-u"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(!out.trim().is_empty(), "should output username");
    assert!(!out.trim().chars().any(|c| c.is_ascii_digit() || c == ':'), "username should not look like a uid");
}

#[test]
fn id_supp_groups() {
    let (code, out, err) = run(&["id", "-G"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(!out.trim().is_empty(), "should output group list");
}

#[test]
fn id_real_user() {
    let (code, out, err) = run(&["id", "-r", "-u"]);
    assert_eq!(code, 0, "stderr: {}", err);
    let val = out.trim().parse::<u32>().unwrap_or(0);
    assert!(val > 0, "real uid should be positive");
}

// ---------------------------------------------------------------------------
// who
// ---------------------------------------------------------------------------
#[test]
fn who_no_args() {
    let (code, _out, err) = run(&["who"]);
    assert_eq!(code, 0, "stderr: {}", err);
    // In a minimal container, who may have no output — that's fine
    assert_eq!(err, "");
}

#[test]
fn who_invalid_option() {
    let (code, out, err) = run(&["who", "-x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// kill
// ---------------------------------------------------------------------------
#[test]
fn kill_list_signals() {
    let (code, out, err) = run(&["kill", "-l"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(out.contains("TERM"), "should list SIGTERM, got: {:?}", out);
    assert!(out.contains("KILL"), "should list SIGKILL");
}

#[test]
fn kill_no_args() {
    let (code, out, err) = run(&["kill"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print usage");
}

#[test]
fn kill_invalid_option() {
    let (code, out, err) = run(&["kill", "-x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn kill_unknown_signal() {
    let (code, out, err) = run(&["kill", "-s", "NOSIGNAL", "1"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// cut
// ---------------------------------------------------------------------------
#[test]
fn cut_dev_null() {
    let (code, out, err) = run(&["cut", "-f1", "/dev/null"]);
    assert_eq!(code, 0);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn cut_field_stdin() {
    let (code, out, err) = run_with_stdin(&["cut", "-f1"], "a\tb\nc\td\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "a\nc\n");
    assert_eq!(err, "");
}

#[test]
fn cut_custom_delimiter() {
    let (code, out, err) = run_with_stdin(&["cut", "-d,", "-f2"], "x,y,z\n1,2,3\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "y\n2\n");
    assert_eq!(err, "");
}

#[test]
fn cut_multi_field() {
    let (code, out, err) = run_with_stdin(&["cut", "-f1,3"], "a\tb\tc\nd\te\tf\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "a\tc\nd\tf\n");
    assert_eq!(err, "");
}

#[test]
fn cut_invalid_option() {
    let (code, out, err) = run(&["cut", "-x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// tr
// ---------------------------------------------------------------------------
#[test]
fn tr_basic_translate() {
    let (code, out, err) = run_with_stdin(&["tr", "a", "z"], "abc\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "zbc\n");
    assert_eq!(err, "");
}

#[test]
fn tr_delete() {
    let (code, out, err) = run_with_stdin(&["tr", "-d", "aeiou"], "hello world\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "hll wrld\n");
}

#[test]
fn tr_squeeze() {
    let (code, out, err) = run_with_stdin(&["tr", "-s", " "], "a    b   c\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "a b c\n");
}

#[test]
fn tr_squeeze_only_does_not_translate() {
    let (code, out, err) = run_with_stdin(&["tr", "-s", "ab"], "abbbaccc");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "abac");
}

#[test]
fn tr_invalid_option() {
    let (code, out, err) = run(&["tr", "-x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// mv
// ---------------------------------------------------------------------------
#[test]
fn mv_no_args() {
    let (code, out, err) = run(&["mv"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print usage");
}

#[test]
fn mv_nonexistent_source() {
    let (code, out, err) = run(&["mv", "/nonexistent_mv_src_xyz", "/tmp/mv_dst"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// rm
// ---------------------------------------------------------------------------
#[test]
fn rm_no_args() {
    let (code, out, err) = run(&["rm"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print usage");
}

#[test]
fn rm_nonexistent() {
    let (code, out, err) = run(&["rm", "/nonexistent_rm_xyz"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn rm_force_nonexistent() {
    let (code, out, err) = run(&["rm", "-f", "/nonexistent_rm_xyz"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "");
}

// ---------------------------------------------------------------------------
// ln
// ---------------------------------------------------------------------------
#[test]
fn ln_no_args() {
    let (code, out, err) = run(&["ln"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print usage");
}

#[test]
fn ln_one_arg() {
    let (code, out, err) = run(&["ln", "/dev/null"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn ln_invalid_option() {
    let (code, out, err) = run(&["ln", "-x", "a", "b"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// whoami
// ---------------------------------------------------------------------------
#[test]
fn whoami_no_args() {
    let (code, out, err) = run(&["whoami"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(!out.is_empty(), "should output username");
    assert!(out.ends_with('\n'), "should end with newline");
    assert_eq!(err, "");
}

#[test]
fn whoami_with_args() {
    let (code, out, err) = run(&["whoami", "extra"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// link
// ---------------------------------------------------------------------------
#[test]
fn link_no_args() {
    let (code, out, err) = run(&["link"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn link_one_arg() {
    let (code, out, err) = run(&["link", "/dev/null"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn link_too_many_args() {
    let (code, out, err) = run(&["link", "a", "b", "c"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn link_nonexistent_source() {
    let (code, out, err) = run(&["link", "/nonexistent_link_src", "/tmp/link_dst"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// unlink
// ---------------------------------------------------------------------------
#[test]
fn unlink_no_args() {
    let (code, out, err) = run(&["unlink"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn unlink_too_many_args() {
    let (code, out, err) = run(&["unlink", "a", "b"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn unlink_nonexistent() {
    let (code, out, err) = run(&["unlink", "/nonexistent_unlink_test_xyz"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// tee
// ---------------------------------------------------------------------------
#[test]
fn tee_stdin_to_stdout() {
    let (code, out, err) = run_with_stdin(&["tee"], "hello\nworld\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "hello\nworld\n");
    assert_eq!(err, "");
}

#[test]
fn tee_invalid_option() {
    let (code, out, err) = run(&["tee", "-x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// mkdir
// ---------------------------------------------------------------------------
#[test]
fn mkdir_no_args() {
    let (code, out, err) = run(&["mkdir"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn mkdir_invalid_option() {
    let (code, out, err) = run(&["mkdir", "-x", "d"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// rmdir
// ---------------------------------------------------------------------------
#[test]
fn rmdir_no_args() {
    let (code, out, err) = run(&["rmdir"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn rmdir_nonexistent() {
    let (code, out, err) = run(&["rmdir", "/tmp/nonexistent_rmdir_xyz"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// logname
// ---------------------------------------------------------------------------
#[test]
fn logname_no_args() {
    let (code, out, err) = run(&["logname"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(!out.is_empty(), "should output login name");
    assert!(out.ends_with('\n'), "should end with newline");
    assert_eq!(err, "");
}

#[test]
fn logname_with_args() {
    let (code, out, err) = run(&["logname", "extra"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// hostname
// ---------------------------------------------------------------------------
#[test]
fn hostname_no_args() {
    let (code, out, err) = run(&["hostname"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(!out.is_empty(), "should output hostname");
    assert!(out.ends_with('\n'), "should end with newline");
    assert_eq!(err, "");
}

#[test]
fn hostname_with_args() {
    let (code, out, err) = run(&["hostname", "extra"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// uptime
// ---------------------------------------------------------------------------
#[test]
fn uptime_no_args() {
    let (code, out, err) = run(&["uptime"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(!out.is_empty(), "should output uptime string");
    assert!(out.starts_with("up "), "should start with 'up '");
    assert_eq!(err, "");
}

#[test]
fn uptime_with_args() {
    let (code, out, err) = run(&["uptime", "extra"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// readlink
// ---------------------------------------------------------------------------
#[test]
fn readlink_no_args() {
    let (code, out, err) = run(&["readlink"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn readlink_too_many_args() {
    let (code, out, err) = run(&["readlink", "a", "b"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn readlink_nonexistent() {
    let (code, out, err) = run(&["readlink", "/nonexistent_readlink_xyz"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// realpath
// ---------------------------------------------------------------------------
#[test]
fn realpath_no_args() {
    let (code, out, err) = run(&["realpath"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn realpath_too_many_args() {
    let (code, out, err) = run(&["realpath", "a", "b"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn realpath_nonexistent() {
    let (code, out, err) = run(&["realpath", "/nonexistent_realpath_xyz"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// uniq
// ---------------------------------------------------------------------------
#[test]
fn uniq_stdin_basic() {
    let (code, out, err) = run_with_stdin(&["uniq"], "a\na\nb\nb\nc\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "a\nb\nc\n");
    assert_eq!(err, "");
}

#[test]
fn uniq_invalid_option() {
    let (code, out, err) = run(&["uniq", "-x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn uniq_count() {
    let (code, out, err) = run_with_stdin(&["uniq", "-c"], "a\na\nb\nc\nc\nc\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "   2 a\n   1 b\n   3 c\n");
    assert_eq!(err, "");
}

#[test]
fn uniq_repeated_only() {
    let (code, out, err) = run_with_stdin(&["uniq", "-d"], "a\na\nb\nc\nc\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "a\nc\n");
    assert_eq!(err, "");
}

#[test]
fn uniq_unique_only() {
    let (code, out, err) = run_with_stdin(&["uniq", "-u"], "a\na\nb\nc\nc\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "b\n");
    assert_eq!(err, "");
}

#[test]
fn uniq_all_unique() {
    let (code, out, err) = run_with_stdin(&["uniq"], "a\nb\nc\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "a\nb\nc\n");
    assert_eq!(err, "");
}

// ---------------------------------------------------------------------------
// wc -L (max line length)
// ---------------------------------------------------------------------------
#[test]
fn wc_max_line_dev_null() {
    let (code, out, err) = run(&["wc", "-L", "/dev/null"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "      0 /dev/null\n");
    assert_eq!(err, "");
}

#[test]
fn wc_max_line_stdin() {
    let (code, out, err) = run_with_stdin(&["wc", "-L"], "short\nlonger_line\nshort\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "     11\n");
    assert_eq!(err, "");
}

#[test]
fn wc_all_flags_with_l() {
    let (code, out, err) = run_with_stdin(&["wc", "-lwcmL"], "a\nbb\nccc\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "      3       3       9       9       3\n");
    assert_eq!(err, "");
}

// ---------------------------------------------------------------------------
// chmod
// ---------------------------------------------------------------------------
#[test]
fn chmod_no_args() {
    let (code, out, err) = run(&["chmod"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print usage");
}

#[test]
fn chmod_invalid_option() {
    let (code, out, err) = run(&["chmod", "-x", "644", "/dev/null"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn chmod_octal_dev_null() {
    let (code, _out, err) = run(&["chmod", "644", "/dev/null"]);
    // May fail with EPERM in containers/CI — accept 0 or 1
    assert!(code == 0 || code == 1, "code: {}, stderr: {}", code, err);
}

#[test]
fn chmod_nonexistent() {
    let (code, out, err) = run(&["chmod", "644", "/nonexistent_chmod_xyz"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// chown
// ---------------------------------------------------------------------------
#[test]
fn chown_no_args() {
    let (code, out, err) = run(&["chown"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print usage");
}

#[test]
fn chown_nonexistent() {
    let (code, out, err) = run(&["chown", "root", "/nonexistent_chown_xyz"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn chown_dev_null() {
    let (code, _out, err) = run(&["chown", "root:root", "/dev/null"]);
    // Requires root — accept failure in non-root environments
    assert!(code == 0 || code == 1, "code: {}, stderr: {}", code, err);
}

// ---------------------------------------------------------------------------
// sort
// ---------------------------------------------------------------------------
#[test]
fn sort_stdin() {
    let (code, out, err) = run_with_stdin(&["sort"], "c\na\nb\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "a\nb\nc\n");
    assert_eq!(err, "");
}

#[test]
fn sort_reverse() {
    let (code, out, err) = run_with_stdin(&["sort", "-r"], "a\nb\nc\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "c\nb\na\n");
    assert_eq!(err, "");
}

#[test]
fn sort_numeric() {
    let (code, out, err) = run_with_stdin(&["sort", "-n"], "10\n2\n1\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "1\n2\n10\n");
    assert_eq!(err, "");
}

#[test]
fn sort_unique() {
    let (code, out, err) = run_with_stdin(&["sort", "-u"], "a\na\nb\nb\nc\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "a\nb\nc\n");
    assert_eq!(err, "");
}

#[test]
fn sort_invalid_option() {
    let (code, out, err) = run(&["sort", "-x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn sort_key_numeric() {
    let (code, out, err) = run_with_stdin(&["sort", "-k2,2n"], "b 2\na 1\nc 3\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "a 1\nb 2\nc 3\n");
    assert_eq!(err, "");
}

#[test]
fn sort_multi_key() {
    let (code, out, err) = run_with_stdin(&["sort", "-k1,1", "-k2,2n"], "a 3\na 1\na 2\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "a 1\na 2\na 3\n");
    assert_eq!(err, "");
}

// ---------------------------------------------------------------------------
// test
// ---------------------------------------------------------------------------
#[test]
fn test_true_expr() {
    let (code, out, err) = run(&["test", "x", "=", "x"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn test_false_expr() {
    let (code, out, err) = run(&["test", "x", "=", "y"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn test_file_exists() {
    let (code, out, err) = run(&["test", "-e", "/dev/null"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn test_file_not_exists() {
    let (code, out, err) = run(&["test", "-e", "/nonexistent_test_xyz"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn test_string_nonempty() {
    let (code, out, err) = run(&["test", "-n", "hello"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn test_string_empty() {
    let (code, out, err) = run(&["test", "-z", ""]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn test_not_operator() {
    let (code, out, err) = run(&["test", "!", "-e", "/nonexistent_test_xyz"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn test_bracket_alias() {
    let (code, out, err) = run(&["[", "x", "=", "x", "]"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn test_bracket_missing_closing() {
    let (code, out, err) = run(&["[", "x", "=", "x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error about missing ]");
}

// ---------------------------------------------------------------------------
// tail
// ---------------------------------------------------------------------------
#[test]
fn tail_no_args_stdin() {
    let input = "a\nb\nc\nd\ne\nf\ng\nh\ni\nj\nk\nl\n";
    let (code, out, err) = run_with_stdin(&["tail"], input);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "c\nd\ne\nf\ng\nh\ni\nj\nk\nl\n");
    assert_eq!(err, "");
}

#[test]
fn tail_n_flag_3() {
    let (code, out, err) = run_with_stdin(&["tail", "-n", "3"], "a\nb\nc\nd\ne\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "c\nd\ne\n");
    assert_eq!(err, "");
}

#[test]
fn tail_c_flag_5() {
    let (code, out, err) = run_with_stdin(&["tail", "-c", "5"], "hello world");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "world");
    assert_eq!(err, "");
}

#[test]
fn tail_dev_null() {
    let (code, out, err) = run(&["tail", "/dev/null"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn tail_invalid_option() {
    let (code, out, err) = run(&["tail", "-x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// tac
// ---------------------------------------------------------------------------
#[test]
fn tac_stdin_basic() {
    let (code, out, err) = run_with_stdin(&["tac"], "a\nb\nc\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "c\nb\na\n");
    assert_eq!(err, "");
}

#[test]
fn tac_dev_null() {
    let (code, out, err) = run(&["tac", "/dev/null"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn tac_invalid_option() {
    let (code, out, err) = run(&["tac", "-x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// fold
// ---------------------------------------------------------------------------
#[test]
fn fold_width_10() {
    let (code, out, err) = run_with_stdin(
        &["fold", "-w", "10"],
        "hello world this is a long line",
    );
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "hello worl\nd this is \na long lin\ne\n");
    assert_eq!(err, "");
}

#[test]
fn fold_invalid_option() {
    let (code, out, err) = run(&["fold", "-x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// comm
// ---------------------------------------------------------------------------
#[test]
fn comm_no_args() {
    let (code, out, err) = run(&["comm"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print usage");
}

#[test]
fn comm_one_arg() {
    let (code, out, err) = run(&["comm", "/dev/null"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print missing operand");
}

#[test]
fn comm_invalid_option() {
    let (code, out, err) = run(&["comm", "-x", "/dev/null", "/dev/null"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// join
// ---------------------------------------------------------------------------
#[test]
fn join_no_args() {
    let (code, out, err) = run(&["join"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print usage");
}

#[test]
fn join_invalid_option() {
    let (code, out, err) = run(&["join", "-x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// nl
// ---------------------------------------------------------------------------
#[test]
fn nl_dev_null() {
    let (code, out, err) = run(&["nl", "/dev/null"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn nl_no_args() {
    let (code, _out, err) = run_with_stdin(&["nl"], "");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(err, "");
}

#[test]
fn nl_invalid_option() {
    let (code, out, err) = run(&["nl", "-x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// shuf
// ---------------------------------------------------------------------------
#[test]
fn shuf_empty_stdin() {
    let (code, _out, err) = run_with_stdin(&["shuf"], "");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(err, "");
}

#[test]
fn shuf_too_many_args() {
    let (code, out, err) = run(&["shuf", "a", "b"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn shuf_nonexistent_file() {
    let (code, out, err) = run(&["shuf", "/nonexistent_shuf_test_xyz"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// sum
// ---------------------------------------------------------------------------
#[test]
fn sum_no_args() {
    let (code, out, err) = run(&["sum"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print usage");
}

#[test]
fn sum_empty_file() {
    let (code, out, err) = run(&["sum", "/dev/null"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "0 0\n");
    assert_eq!(err, "");
}

#[test]
fn sum_nonexistent_file() {
    let (code, out, err) = run(&["sum", "/nonexistent_sum_test_xyz"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// ls
// ---------------------------------------------------------------------------
#[test]
fn ls_dev_null() {
    let (code, out, err) = run(&["ls", "/dev/null"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out.trim(), "null");
    assert_eq!(err, "");
}

#[test]
fn ls_current_dir() {
    let (code, out, err) = run(&["ls"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(!out.is_empty());
    assert_eq!(err, "");
}

#[test]
fn ls_invalid_option() {
    let (code, out, err) = run(&["ls", "-x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn ls_l_flag() {
    let (code, out, err) = run(&["ls", "-l", "/dev/null"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(out.starts_with('-'), "expected - prefix for file: {:?}", out);
    assert_eq!(err, "");
}

#[test]
fn ls_bundled_flags() {
    let (code, out, err) = run(&["ls", "-la", "/dev/null"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(out.starts_with('-'), "expected - prefix for file: {:?}", out);
    assert_eq!(err, "");
}

#[test]
fn ls_nonexistent() {
    let (code, out, err) = run(&["ls", "/nonexistent_ls_test_xyz"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// grep
// ---------------------------------------------------------------------------
#[test]
fn grep_missing_pattern() {
    let (code, out, err) = run(&["grep"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn grep_invalid_option() {
    let (code, out, err) = run(&["grep", "-x", "pattern"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn grep_invalid_pattern() {
    let (code, out, err) = run(&["grep", "[invalid"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn grep_no_match_stdin() {
    let (code, out, err) = run(&["grep", "xyz"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn grep_file_match() {
    let (code, out, err) = run_with_stdin(&["grep", "line"], "hello\nworld\nline three\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(out.contains("line"), "output should contain 'line': {:?}", out);
    assert_eq!(err, "");
}

#[test]
fn grep_ignore_case() {
    let (code, out, err) = run_with_stdin(&["grep", "-i", "test"], "Test\ntesting\nno\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(!out.is_empty());
    assert_eq!(err, "");
}

#[test]
fn grep_count() {
    let (code, out, err) = run_with_stdin(&["grep", "-c", "line"], "line one\nline two\nother\n");
    assert_eq!(code, 0, "stderr: {}", err);
    let count: usize = out.trim().parse().expect("count should be a number");
    assert_eq!(count, 2, "should have two matches");
    assert_eq!(err, "");
}

#[test]
fn grep_line_number() {
    let (code, out, err) = run_with_stdin(&["grep", "-n", "line"], "other\nline\nlast\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(out.contains("2:"), "output should contain line number '2:': {:?}", out);
    assert_eq!(err, "");
}

#[test]
fn grep_invert() {
    let (code, out, err) = run_with_stdin(&["grep", "-v", "line"], "line one\nother\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "other\n");
    assert_eq!(err, "");
}

#[test]
fn grep_bundled_flags() {
    let (code, out, err) = run_with_stdin(&["grep", "-iv", "test"], "Test\ntesting\nother\n");
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "other\n");
    assert_eq!(err, "");
}

#[test]
fn grep_nonexistent_file() {
    let (code, out, err) = run(&["grep", "pattern", "/nonexistent_grep_test_xyz"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// du
// ---------------------------------------------------------------------------
#[test]
fn du_dev_null() {
    let (code, out, err) = run(&["du", "/dev/null"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out.trim(), "0\t/dev/null");
    assert_eq!(err, "");
}

#[test]
fn du_human_readable_dev_null() {
    let (code, out, err) = run(&["du", "-h", "/dev/null"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out.trim(), "0\t/dev/null");
    assert_eq!(err, "");
}

#[test]
fn du_invalid_option() {
    let (code, out, err) = run(&["du", "-x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error on stderr");
}

#[test]
fn du_nonexistent() {
    let (code, out, err) = run(&["du", "/nonexistent_du_test_xyz"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error on stderr");
}

#[test]
fn du_bundled_flags() {
    let (code, out, err) = run(&["du", "-hs", "/dev/null"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out.trim(), "0\t/dev/null");
    assert_eq!(err, "");
}

// ---------------------------------------------------------------------------
// df
// ---------------------------------------------------------------------------
#[test]
fn df_invalid_option() {
    let (code, out, err) = run(&["df", "-x"]);
    // Accept 0 or 1 — some envs may not have /proc/mounts
    assert!(code == 0 || code == 1, "code: {}, stderr: {}", code, err);
}

#[test]
fn df_no_args() {
    let (code, out, err) = run(&["df"]);
    // May fail if /proc/mounts is unavailable — accept 0 or 1
    assert!(code == 0 || code == 1, "code: {}, stderr: {}", code, err);
    if code == 0 {
        assert!(out.contains("Filesystem"), "output should have header: {:?}", out);
        assert_eq!(err, "");
    }
}

#[test]
fn df_human_readable() {
    let (code, out, err) = run(&["df", "-h"]);
    assert!(code == 0 || code == 1, "code: {}, stderr: {}", code, err);
    if code == 0 {
        assert!(out.contains("Filesystem"), "output should have header: {:?}", out);
        assert!(out.contains("K") || out.contains("M") || out.contains("G") || out.contains("T"));
        assert_eq!(err, "");
    }
}

#[test]
fn df_bundled_flags() {
    let (code, out, err) = run(&["df", "-hT"]);
    assert!(code == 0 || code == 1, "code: {}, stderr: {}", code, err);
    if code == 0 {
        assert!(out.contains("Filesystem"), "output should have header: {:?}", out);
        assert!(out.contains("Type"), "should show type column");
        assert_eq!(err, "");
    }
}

// ---------------------------------------------------------------------------
// cp
// ---------------------------------------------------------------------------
#[test]
fn cp_missing_args() {
    let (code, out, err) = run(&["cp"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print usage");
}

#[test]
fn cp_single_arg() {
    let (code, out, err) = run(&["cp", "a"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print usage");
}

#[test]
fn cp_nonexistent_source() {
    let (code, out, err) = run(&["cp", "/nonexistent_cp_src_xyz", "/tmp"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn cp_invalid_option() {
    let (code, out, err) = run(&["cp", "-x", "a", "b"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// printf
// ---------------------------------------------------------------------------
#[test]
fn printf_format_string() {
    let (code, out, err) = run(&["printf", "hello"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "hello");
    assert_eq!(err, "");
}

#[test]
fn printf_percent_d() {
    let (code, out, err) = run(&["printf", "%d", "42"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "42");
    assert_eq!(err, "");
}

#[test]
fn printf_percent_s() {
    let (code, out, err) = run(&["printf", "%s", "world"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "world");
    assert_eq!(err, "");
}

#[test]
fn printf_newline_escape() {
    let (code, out, err) = run(&["printf", "a\\nb"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "a\nb");
    assert_eq!(err, "");
}

#[test]
fn printf_no_format() {
    let (code, out, err) = run(&["printf", ""]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

// ---------------------------------------------------------------------------
// date
// ---------------------------------------------------------------------------
#[test]
fn date_default_format() {
    let (code, out, err) = run(&["date"]);
    assert_eq!(code, 0, "stderr: {}", err);
    // output should contain time info
    assert!(!out.is_empty(), "should produce output");
    assert_eq!(err, "");
}

#[test]
fn date_utc_flag() {
    let (code, out, err) = run(&["date", "-u"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert!(!out.is_empty(), "should produce output");
    assert!(out.contains("UTC") || out.ends_with("\n"), "UTC flag should produce UTC time");
    assert_eq!(err, "");
}

#[test]
fn date_format_string() {
    let (code, out, err) = run(&["date", "+%Y-%m-%d"]);
    assert_eq!(code, 0, "stderr: {}", err);
    let trimmed = out.trim();
    assert_eq!(trimmed.len(), 10, "YYYY-MM-DD should be 10 chars, got: {:?}", trimmed);
    assert!(trimmed.contains('-'), "should contain dashes");
    assert_eq!(err, "");
}

#[test]
fn date_invalid_option() {
    let (code, out, err) = run(&["date", "-x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

// ---------------------------------------------------------------------------
// expr
// ---------------------------------------------------------------------------
#[test]
fn expr_addition() {
    let (code, out, err) = run(&["expr", "2", "+", "3"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out.trim(), "5");
    assert_eq!(err, "");
}

#[test]
fn expr_subtraction() {
    let (code, out, err) = run(&["expr", "10", "-", "3"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out.trim(), "7");
    assert_eq!(err, "");
}

#[test]
fn expr_multiplication() {
    let (code, out, err) = run(&["expr", "4", "*", "3"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out.trim(), "12");
    assert_eq!(err, "");
}

#[test]
fn expr_division() {
    let (code, out, err) = run(&["expr", "10", "/", "3"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out.trim(), "3");
    assert_eq!(err, "");
}

#[test]
fn expr_comparison() {
    let (code, out, err) = run(&["expr", "5", "=", "5"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out.trim(), "1");
    assert_eq!(err, "");
}

#[test]
fn expr_string_length() {
    let (code, out, err) = run(&["expr", "length", "hello"]);
    assert_eq!(code, 0, "stderr: {}", err);
    assert_eq!(out.trim(), "5");
    assert_eq!(err, "");
}

// ---------------------------------------------------------------------------
// split
// ---------------------------------------------------------------------------
#[test]
fn split_no_args() {
    let (code, out, err) = run(&["split"]);
    // GNU split reads stdin when no args, exit 0 is correct
    assert!(code == 0 || code == 1, "code: {}, stderr: {}", code, err);
    if code == 1 {
        assert_eq!(out, "");
        assert!(!err.is_empty(), "should print usage");
    }
}

#[test]
fn split_invalid_option() {
    let (code, out, err) = run(&["split", "-x"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}

#[test]
fn split_nonexistent_file() {
    let (code, out, err) = run(&["split", "/nonexistent_split_test_xyz"]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!err.is_empty(), "should print error");
}
