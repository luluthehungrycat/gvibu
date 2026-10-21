use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn help_unknown_command_and_bracket_alias_work() {
    let help = Command::new(env!("CARGO_BIN_EXE_vish"))
        .arg("help")
        .output()
        .unwrap();
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("Built-in commands:"));

    let unknown = Command::new(env!("CARGO_BIN_EXE_vish"))
        .arg("missing-command")
        .output()
        .unwrap();
    assert_eq!(unknown.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&unknown.stderr).contains("command not found"));

    let bracket = Command::new(env!("CARGO_BIN_EXE_vish"))
        .args(["[", "-n", "x", "]"])
        .output()
        .unwrap();
    assert!(bracket.status.success());
}

#[test]
fn repl_eof_and_exit_stop_cleanly() {
    for input in ["", "help\n", "exit\necho should-not-run\n", "quit\necho should-not-run\n"] {
        let mut child = Command::new(env!("CARGO_BIN_EXE_vish"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
        let result = child.wait_with_output().unwrap();
        assert!(result.status.success());
        assert!(!String::from_utf8_lossy(&result.stdout).contains("should-not-run"));
    }
}
