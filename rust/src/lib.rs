pub mod commands;

use std::cell::RefCell;

thread_local! {
    static WASM_STDIN: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Replace the stdin buffer used by WASM command dispatch.
pub fn set_wasm_stdin(input: Option<&str>) {
    WASM_STDIN.with(|buffer| *buffer.borrow_mut() = input.map(str::to_owned));
}

/// Return a copy of the current WASM stdin buffer, if one was supplied.
pub fn get_wasm_stdin() -> Option<String> {
    WASM_STDIN.with(|buffer| buffer.borrow().clone())
}

/// Clear buffered stdin after a single command invocation.
pub fn clear_wasm_stdin() {
    set_wasm_stdin(None);
}

#[cfg(test)]
mod wasm_stdin_tests {
    use super::{clear_wasm_stdin, commands, get_wasm_stdin, set_wasm_stdin};

    #[test]
    fn buffered_stdin_is_replaceable_and_clearable_between_calls() {
        let command = commands::lookup("wc").expect("wc is registered");
        let mut first_output = Vec::new();
        set_wasm_stdin(Some("one\n"));
        assert_eq!((command.run)(&mut first_output, &["-l".into()]), 0);
        assert_eq!(String::from_utf8(first_output).unwrap().trim(), "1");

        let mut second_output = Vec::new();
        set_wasm_stdin(Some("one\ntwo\n"));
        assert_eq!((command.run)(&mut second_output, &["-l".into()]), 0);
        assert_eq!(String::from_utf8(second_output).unwrap().trim(), "2");

        clear_wasm_stdin();
        assert_eq!(get_wasm_stdin(), None);
    }

    #[test]
    fn stdin_reading_commands_consume_the_injected_buffer() {
        let cases: &[(&str, &[&str], &str, &str)] = &[
            ("cat", &[], "alpha\n", "alpha\n"),
            ("cat", &["-"], "alpha\n", "alpha\n"),
            ("expand", &[], "\talpha\n", "        alpha\n"),
            ("expand", &["-"], "\tbeta\n", "        beta\n"),
            ("rev", &[], "abc\nxy", "cba\nyx"),
            ("rev", &["-"], "def\n", "fed\n"),
            ("comm", &["-", "-"], "alpha\n", "\t\talpha\n"),
            ("cut", &["-f2"], "a\tb\n", "b\n"),
            ("cut", &["-f2", "-"], "a\tb\n", "b\n"),
            ("fold", &["-w", "2"], "abc\n", "ab\nc\n"),
            ("grep", &["alpha"], "alpha\nbeta\n", "alpha\n"),
            ("grep", &["alpha", "-"], "alpha\nbeta\n", "alpha\n"),
            ("head", &["-n", "1"], "a\nb\n", "a\n"),
            ("head", &["-n", "1", "-"], "a\nb\n", "a\n"),
            ("nl", &[], "alpha\n", "     1\talpha\n"),
            ("sort", &[], "b\na\n", "a\nb\n"),
            ("sort", &["-"], "b\na\n", "a\nb\n"),
            ("tail", &["-n", "1"], "a\nb\n", "b\n"),
            ("tail", &["-n", "1", "-"], "a\nb\n", "b\n"),
            ("tac", &[], "a\nb\n", "b\na\n"),
            ("tee", &[], "alpha\n", "alpha\n"),
            ("tr", &["a-z", "A-Z"], "ab\n", "AB\n"),
            ("uniq", &[], "a\na\n", "a\n"),
        ];
        for (name, args, input, expected) in cases {
            let command = commands::lookup(name).expect("command is registered");
            let mut output = Vec::new();
            set_wasm_stdin(Some(input));
            let args: Vec<String> = args.iter().map(|arg| (*arg).to_owned()).collect();
            assert_eq!((command.run)(&mut output, &args), 0, "command {name}");
            assert_eq!(String::from_utf8(output).unwrap(), *expected, "command {name}");
        }
        let shuf = commands::lookup("shuf").expect("shuf is registered");
        let mut output = Vec::new();
        set_wasm_stdin(Some("a\nb\n"));
        assert_eq!((shuf.run)(&mut output, &[]), 0);
        let mut lines: Vec<_> = String::from_utf8(output).unwrap().lines().map(str::to_owned).collect();
        lines.sort();
        assert_eq!(lines, ["a", "b"]);
        clear_wasm_stdin();
    }
}

/// Write a line to output, returning 0 on BrokenPipe (clean exit) and 1 on other errors.
#[macro_export]
macro_rules! pwriteln {
    ($dst:expr) => {
        if let Err(e) = writeln!($dst) {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                return 0;
            }
            return 1;
        }
    };
    ($dst:expr, $($rest:tt)*) => {
        if let Err(e) = writeln!($dst, $($rest)*) {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                return 0;
            }
            return 1;
        }
    };
}

/// Write to output, returning 0 on BrokenPipe (clean exit) and 1 on other errors.
#[macro_export]
macro_rules! pwrite {
    ($dst:expr, $($rest:tt)*) => {
        if let Err(e) = write!($dst, $($rest)*) {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                return 0;
            }
            return 1;
        }
    };
}
