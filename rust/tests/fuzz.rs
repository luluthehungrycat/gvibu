/// Property-based fuzz tests for gvibu commands using proptest.
/// These tests verify that commands never panic and maintain certain invariants
/// when given random inputs across a wide range of values.
use proptest::prelude::*;
use std::process::Command;
fn safe_string() -> impl Strategy<Value = String> {
    any::<String>().prop_filter("safe process argument", |value| !value.contains('\0'))
}

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
    let mut stdin_pipe = child.stdin.take().unwrap();
    let _ = stdin_pipe.write_all(stdin.as_bytes());
    drop(stdin_pipe);
    let output = child.wait_with_output().expect("failed to read output");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

// ---------------------------------------------------------------------------
// true / false — must always return the right exit code
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_true_any_args(ref args in proptest::collection::vec(safe_string(), 0..10)) {
        let mut cmd_args = vec!["true"];
        cmd_args.extend(args.iter().map(|s| s.as_str()));
        let (code, _, _) = run(&cmd_args);
        assert_eq!(code, 0, "true should always exit 0");
    }
}

proptest! {
    #[test]
    fn fuzz_false_any_args(ref args in proptest::collection::vec(safe_string(), 0..10)) {
        let mut cmd_args = vec!["false"];
        cmd_args.extend(args.iter().map(|s| s.as_str()));
        let (code, _, _) = run(&cmd_args);
        assert_eq!(code, 1, "false should always exit 1");
    }
}

// ---------------------------------------------------------------------------
// echo — never crashes with any args
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_echo_no_crash(ref args in proptest::collection::vec(safe_string(), 0..10)) {
        let mut cmd_args = vec!["echo"];
        cmd_args.extend(args.iter().map(|s| s.as_str()));
        let (code, out, _) = run(&cmd_args);
        assert_eq!(code, 0, "echo should never crash");
        if !args.is_empty() && !args[0].starts_with('-') {
            assert!(out.ends_with('\n'), "echo output should end with newline");
        }
    }
}

// ---------------------------------------------------------------------------
// basename — never crashes on any path string
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_basename_no_crash(ref path in safe_string()) {
        let (code, _, _) = run(&["basename", path]);
        assert!(code == 0 || code == 1, "basename should only exit 0 or 1, got {}", code);
    }
}

proptest! {
    #[test]
    fn fuzz_basename_with_suffix(ref path in safe_string(), ref suffix in safe_string()) {
        let (code, _, _) = run(&["basename", path, suffix]);
        assert!(code == 0 || code == 1, "basename with suffix should only exit 0 or 1");
    }
}

// ---------------------------------------------------------------------------
// dirname — never crashes on any path string
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_dirname_no_crash(ref path in safe_string()) {
        let (code, _, _) = run(&["dirname", path]);
        assert!(code == 0 || code == 2, "dirname should only exit 0 or 2, got {}", code);
    }
}

// ---------------------------------------------------------------------------
// sort — never crashes, output is sorted, unique output has no dupes
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_sort_no_crash(ref lines in proptest::collection::vec(safe_string(), 0..30)) {
        let input = lines.join("\n");
        let (code, out, _) = run_with_stdin(&["sort"], &input);
        assert_eq!(code, 0, "sort should not crash");
        let out_lines: Vec<&str> = out.lines().collect();
        assert!(out_lines.len() <= lines.len() || lines.is_empty(),
            "sort output lines ({}) should not exceed input lines ({})",
            out_lines.len(), lines.len());
    }
}

proptest! {
    #[test]
    fn fuzz_sort_output_is_sorted(ref lines in proptest::collection::vec("[ -~]{0,20}", 0..20)) {
        let input = lines.join("\n");
        let (code, out, _) = run_with_stdin(&["sort"], &input);
        assert_eq!(code, 0);
        let out_lines: Vec<String> = out.lines().map(|s| s.to_string()).collect();
        let mut sorted = out_lines.clone();
        sorted.sort();
        assert_eq!(out_lines, sorted, "sort output should be sorted");
    }
}

proptest! {
    #[test]
    fn fuzz_sort_unique(ref lines in proptest::collection::vec("[ -~]{0,10}", 0..20)) {
        let input = lines.join("\n");
        let (code, out, _) = run_with_stdin(&["sort", "-u"], &input);
        assert_eq!(code, 0);
        let out_lines: Vec<&str> = out.lines().collect();
        let mut dedup = out_lines.clone();
        dedup.sort();
        dedup.dedup();
        assert_eq!(out_lines.len(), dedup.len(),
            "sort -u output should have no duplicates");
    }
}

proptest! {
    #[test]
    fn fuzz_sort_reverse(ref lines in proptest::collection::vec("[ -~]{0,10}", 0..20)) {
        let input = lines.join("\n");
        let (code, out, _) = run_with_stdin(&["sort", "-r"], &input);
        assert_eq!(code, 0);
        let out_lines: Vec<String> = out.lines().map(|s| s.to_string()).collect();
        let mut reverse_sorted = out_lines.clone();
        reverse_sorted.sort_by(|a, b| b.cmp(a));
        assert_eq!(out_lines, reverse_sorted, "sort -r output should be reverse sorted");
    }
}

// ---------------------------------------------------------------------------
// test — never panics, only returns 0 or 1
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_test_no_crash(ref args in proptest::collection::vec(safe_string(), 1..8)) {
        let mut cmd_args = vec!["test"];
        cmd_args.extend(args.iter().map(|s| s.as_str()));
        let (code, _, _) = run(&cmd_args);
        assert!(code == 0 || code == 1, "test should only exit 0 or 1, got {}", code);
    }
}

// ---------------------------------------------------------------------------
// cut — never crashes
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_cut_field(ref lines in proptest::collection::vec(safe_string(), 0..10)) {
        let input = lines.join("\n");
        let (code, _, _) = run_with_stdin(&["cut", "-f1"], &input);
        assert!(code == 0 || code == 1, "cut should not crash, got {}", code);
    }
}

proptest! {
    #[test]
    fn fuzz_cut_delimiter(ref lines in proptest::collection::vec(safe_string(), 0..10)) {
        let input = lines.join("\n");
        let (code, _, _) = run_with_stdin(&["cut", "-d,", "-f1"], &input);
        assert!(code == 0 || code == 1, "cut should not crash with delimiter");
    }
}

// ---------------------------------------------------------------------------
// tr — never crashes
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_tr_basic(ref input in safe_string(), ref set1 in safe_string(), ref set2 in safe_string()) {
        let (code, _, _) = run_with_stdin(&["tr", set1, set2], input);
        assert!(code == 0 || code == 1, "tr should not crash, got {}", code);
    }
}

// ---------------------------------------------------------------------------
// wc — never crashes, counts are consistent
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_wc_no_crash(ref content in ".{0,200}") {
        let (code, _, _) = run_with_stdin(&["wc"], content);
        assert_eq!(code, 0, "wc should not crash");
    }
}

proptest! {
    #[test]
    fn fuzz_wc_counts_consistent(ref content in "[ -~]{0,200}") {
        let (code, out, _) = run_with_stdin(&["wc"], content);
        assert_eq!(code, 0);
        let parts: Vec<&str> = out.split_whitespace().collect();
        assert_eq!(parts.len(), 4, "wc should output 4 values, got {:?}", parts);
        let bytes: usize = parts[2].parse().unwrap_or(0);
        let chars: usize = parts[3].parse().unwrap_or(0);
        assert_eq!(bytes, chars,
            "For ASCII content, bytes ({}) should equal chars ({}) for {:?}",
            bytes, chars, content);
    }
}

// ---------------------------------------------------------------------------
// uniq — never crashes, output never exceeds input
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_uniq_no_crash(ref lines in proptest::collection::vec(safe_string(), 0..15)) {
        let input = lines.join("\n");
        let (code, out, _) = run_with_stdin(&["uniq"], &input);
        assert_eq!(code, 0, "uniq should not crash");
        let out_lines: Vec<&str> = out.lines().collect();
        assert!(out_lines.len() <= lines.len(),
            "uniq output lines ({}) should not exceed input lines ({})",
            out_lines.len(), lines.len());
    }
}

// ---------------------------------------------------------------------------
// seq — never crashes with reasonable numbers
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_seq_two_args(a in -20i32..=20, b in -20i32..=20) {
        let (code, _, _) = run(&["seq", &a.to_string(), &b.to_string()]);
        assert!(code == 0 || code == 1,
            "seq should only exit 0 or 1, got {} for seq {} {}", code, a, b);
    }
}

proptest! {
    #[test]
    fn fuzz_seq_three_args(a in -10i32..=10, step in -5i32..=5, b in -10i32..=10) {
        if step == 0 { return Ok(()); } // skip zero step, handled separately
        let (code, _, _) = run(&["seq", &a.to_string(), &step.to_string(), &b.to_string()]);
        assert!(code == 0 || code == 1,
            "seq should only exit 0 or 1, got {} for seq {} {} {}", code, a, step, b);
    }
}

// ---------------------------------------------------------------------------
// head — never crashes
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_head_no_crash(ref content in ".{0,100}", n in 0usize..30) {
        let (code, _, _) = run_with_stdin(&["head", "-n", &n.to_string()], content);
        assert_eq!(code, 0, "head should not crash");
    }
}

proptest! {
    #[test]
    fn fuzz_head_c_flag(ref content in ".{0,100}", n in 0usize..30) {
        let (code, _, _) = run_with_stdin(&["head", "-c", &n.to_string()], content);
        assert_eq!(code, 0, "head -c should not crash");
    }
}

// ---------------------------------------------------------------------------
// yes — never crashes (limited iterations via timeout)
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_yes_no_crash(ref args in proptest::collection::vec("[ -~]{0,20}", 0..5)) {
        let mut cmd_args = vec!["yes"];
        cmd_args.extend(args.iter().map(|s| s.as_str()));
        let mut cmd = Command::new(gvibu_bin());
        cmd.args(&cmd_args);
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());
        let mut child = cmd.spawn().expect("failed to spawn gvibu");
        std::thread::sleep(std::time::Duration::from_millis(50));
        let _ = child.kill();
        let output = child.wait_with_output().expect("failed to wait");
        let out = String::from_utf8_lossy(&output.stdout);
        assert!(!out.is_empty(), "yes should produce output");
        if args.is_empty() {
            assert!(out.starts_with("y\n"), "default yes should output 'y'");
        }
    }
}

// ---------------------------------------------------------------------------
// cat — never crashes
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_cat_stable(ref content in ".{0,200}") {
        // Reading stdin and immediately outputting should preserve content
        let (code, out, _) = run_with_stdin(&["cat"], content);
        assert_eq!(code, 0, "cat should not crash");
        assert_eq!(&out, content, "cat stdin should preserve content exactly");
    }
}

// ---------------------------------------------------------------------------
// printf — never crashes with random format strings and args
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_printf_no_crash(fmt in safe_string(), ref args in proptest::collection::vec("[\x20-\x7e]{0,10}", 0..4)) {
        let mut cmd_args = vec!["printf"];
        // Prevent format strings starting with '-' from being eaten as flags
        if fmt.starts_with('-') {
            cmd_args.push("--");
        }
        cmd_args.push(fmt.as_str());
        cmd_args.extend(args.iter().map(|s| s.as_str()));
        let (code, _, _) = run(&cmd_args);
        assert!(code == 0 || code == 1,
            "printf should only exit 0 or 1, got {} for fmt={:?} args={:?}", code, fmt, args);
    }
}

proptest! {
    #[test]
    fn fuzz_printf_s_specifier(ref text in "[\x20-\x7e]{0,50}") {
        let (code, out, _) = run(&["printf", "%s", text]);
        assert_eq!(code, 0, "printf %%s should succeed");
        assert_eq!(out, *text, "printf %%s should reproduce string exactly");
    }
}

proptest! {
    #[test]
    fn fuzz_printf_d_specifier(n in -1000i32..=1000) {
        let (code, out, _) = run(&["printf", "%d", &n.to_string()]);
        assert_eq!(code, 0, "printf %%d should succeed");
        assert_eq!(out.trim_end(), n.to_string(), "printf %%d should output number");
    }
}

// ---------------------------------------------------------------------------
// date — never crashes with any format string
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_date_no_crash(fmt in safe_string()) {
        let arg = format!("+{}", fmt);
        let (code, _, _) = run(&["date", &arg]);
        assert!(code == 0 || code == 1,
            "date should only exit 0 or 1, got {} for +{:?}", code, fmt);
    }
}

proptest! {
    #[test]
    fn fuzz_date_flag_no_crash(flag in proptest::prop_oneof!["-u", "-R", "-I"]) {
        let (code, _, _) = run(&["date", &*flag]);
        assert_eq!(code, 0, "date {} should succeed", flag);
    }
}

// ---------------------------------------------------------------------------
// expr — arithmetic and string operation invariants
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_expr_no_crash(ref args in proptest::collection::vec("[\x20-\x7e]{1,15}", 1..5)) {
        let mut cmd_args = vec!["expr"];
        cmd_args.extend(args.iter().map(|s| s.as_str()));
        let (code, _, _) = run(&cmd_args);
        assert!(code == 0 || code == 1 || code == 2,
            "expr should exit 0, 1, or 2, got {} for {:?}", code, args);
    }
}

proptest! {
    #[test]
    fn fuzz_expr_addition_commutes(a in -100i32..=100, b in -100i32..=100) {
        let (code1, out1, _) = run(&["expr", &a.to_string(), "+", &b.to_string()]);
        let (code2, out2, _) = run(&["expr", &b.to_string(), "+", &a.to_string()]);
        assert_eq!(code1, code2, "addition should be commutative");
        if code1 == 0 {
            assert_eq!(out1.trim(), out2.trim(), "a+b should equal b+a for a={} b={}", a, b);
        }
    }
}

proptest! {
    #[test]
    fn fuzz_expr_multiplication_commutes(a in -50i32..=50, b in -50i32..=50) {
        let (code1, out1, _) = run(&["expr", &a.to_string(), "*", &b.to_string()]);
        let (code2, out2, _) = run(&["expr", &b.to_string(), "*", &a.to_string()]);
        assert_eq!(code1, code2, "multiplication should be commutative");
        if code1 == 0 {
            assert_eq!(out1.trim(), out2.trim(), "a*b should equal b*a for a={} b={}", a, b);
        }
    }
}

// ---------------------------------------------------------------------------
// cp — never crashes, copy preserves content
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_cp_preserves_content(ref content in ".{0,200}") {
        // Use a unique temp dir that gets cleaned up
        let dir = std::env::temp_dir().join(format!("gvibu_fuzz_cp_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let src = dir.join("src.txt");
        let dst = dir.join("dst.txt");
        let _ = std::fs::write(&src, content);
        let src_str = src.to_str().expect("valid utf8 path");
        let dst_str = dst.to_str().expect("valid utf8 path");
        let (code, _, _) = run(&["cp", src_str, dst_str]);
        assert_eq!(code, 0, "cp should succeed");
        let copied = std::fs::read_to_string(&dst).unwrap_or_default();
        assert_eq!(&copied, content, "cp should preserve content exactly");
        let _ = std::fs::remove_dir_all(&dir);
    }
}

proptest! {
    #[test]
    fn fuzz_cp_no_crash(ref args in proptest::collection::vec("[\x20-\x7e]{1,20}", 1..4)) {
        let mut cmd_args = vec!["cp"];
        cmd_args.extend(args.iter().map(|s| s.as_str()));
        let (code, _, _) = run(&cmd_args);
        assert!(code == 0 || code == 1,
            "cp should only exit 0 or 1, got {}", code);
    }
}

// ---------------------------------------------------------------------------
// split — never crashes, split+concat restores original
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn fuzz_split_concat_roundtrip(ref content in ".{0,200}") {
        // Only test non-empty content with a trailing newline (typical file)
        if content.is_empty() || !content.ends_with('\n') {
            return Ok(());
        }
        let dir = std::env::temp_dir().join(format!("gvibu_fuzz_split_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);

        // Write input file in temp dir
        let input_file = dir.join("input.txt");
        let _ = std::fs::write(&input_file, content);
        let input_str = input_file.to_str().expect("valid utf8");

        // Run split
        let (code, _, _) = run(&["split", "-l", "50", input_str]);
        assert_eq!(code, 0, "split should succeed");

        // Concatenate split parts
        let mut combined = String::new();
        let mut entries: Vec<_> = std::fs::read_dir(&dir).unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let n = e.file_name();
                let n = n.to_str().unwrap_or("");
                n != "input.txt" && !n.starts_with('.')
            })
            .collect();
        entries.sort_by_key(|e| e.file_name());
        for entry in &entries {
            if let Ok(c) = std::fs::read_to_string(entry.path()) {
                combined.push_str(&c);
            }
        }

        assert_eq!(&combined, content, "split parts should concatenate to original");
        let _ = std::fs::remove_dir_all(&dir);
    }
}

proptest! {
    #[test]
    fn fuzz_split_no_crash(ref content in ".{0,100}") {
        let dir = std::env::temp_dir().join(format!("gvibu_fuzz_split_nc_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let input_file = dir.join("in.txt");
        let _ = std::fs::write(&input_file, content);
        let input_str = input_file.to_str().expect("valid utf8");

        let (code, _, _) = run(&["split", input_str]);
        assert!(code == 0 || code == 1,
            "split should only exit 0 or 1, got {}", code);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
