use gvibu::commands;
use std::env;
use std::io::{self, BufRead, IsTerminal, Write};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
gvibu \u{2014} universal coreutils
Built-in commands: help, version, commands, exit, quit
Type a command name to run it, or one of the built-ins above.
";

static INTERRUPTED: AtomicBool = AtomicBool::new(false);

/// What a built-in tells the caller to do next.
enum BuiltinAction {
    /// Stay in the REPL / continue with the next command.
    Continue,
    /// Terminate the process with the given exit code.
    Exit(i32),
}

fn print_help() {
    println!("{}", HELP_TEXT);
}

fn print_version() {
    println!("gvibu v{}", VERSION);
}

fn list_commands() {
    for cmd in commands::COMMANDS {
        if let Some(name) = cmd.names.first() {
            println!("{}", name);
        }
    }
}

fn dispatch(name: &str, args: &[String]) -> i32 {
    match commands::lookup(name) {
        Some(cmd) => (cmd.run)(&mut io::stdout(), args),
        None => {
            eprintln!("{}: command not found", name);
            1
        }
    }
}

fn run_builtin(name: &str) -> Option<BuiltinAction> {
    match name {
        "exit" | "quit" => Some(BuiltinAction::Exit(0)),
        "help" => {
            print_help();
            Some(BuiltinAction::Continue)
        }
        "version" => {
            print_version();
            Some(BuiltinAction::Continue)
        }
        "commands" => {
            list_commands();
            Some(BuiltinAction::Continue)
        }
        _ => None,
    }
}

#[cfg(unix)]
mod signal_impl {
    use super::INTERRUPTED;
    use std::sync::atomic::Ordering;

    pub extern "C" fn handle_sigint(_sig: i32) {
        INTERRUPTED.store(true, Ordering::SeqCst);
        let msg: &[u8] = b"^C\n";
        // SAFETY: fd 2 is stderr; write(2) is async-signal-safe.
        unsafe {
            libc::write(2, msg.as_ptr().cast(), msg.len());
        }
    }

    pub fn install() {
        // SAFETY: handler is a valid extern "C" fn; we never restore the
        // previous disposition (SIG_DFL), which is acceptable for a REPL.
        unsafe {
            libc::signal(libc::SIGINT, handle_sigint as *const () as libc::sighandler_t);
        }
    }
}

#[cfg(not(unix))]
mod signal_impl {
    pub fn install() {}
}

/// Run a non-built-in command in the REPL. The exit code is intentionally
/// discarded — a non-zero result from a piped command must not close the
/// interactive loop.
fn run_user_command(name: &str, args: Vec<String>) {
    let _ = dispatch(name, &args);
}

fn run_repl() {
    signal_impl::install();

    let stdin = io::stdin();
    let is_tty = stdin.is_terminal();
    let mut stdout = io::stdout();
    let mut locked = stdin.lock();

    loop {
        INTERRUPTED.store(false, Ordering::SeqCst);

        if is_tty {
            print!("gvibu> ");
            let _ = stdout.flush();
        }

        let mut line = String::new();
        match locked.read_line(&mut line) {
            Ok(0) => {
                if is_tty {
                    println!();
                }
                process::exit(0);
            }
            Ok(_) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                let mut parts = trimmed.split_whitespace();
                let name = match parts.next() {
                    Some(n) => n,
                    None => continue,
                };
                let cmd_args: Vec<String> = parts.map(String::from).collect();

                match run_builtin(name) {
                    Some(BuiltinAction::Exit(code)) => process::exit(code),
                    Some(BuiltinAction::Continue) => continue,
                    None => run_user_command(name, cmd_args),
                }
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) => process::exit(0),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let name = args[1].as_str();
        let cmd_args: Vec<String> = args[2..].to_vec();

        if let Some(action) = run_builtin(name) {
            match action {
                BuiltinAction::Exit(code) => process::exit(code),
                BuiltinAction::Continue => process::exit(0),
            }
        }

        process::exit(dispatch(name, &cmd_args));
    }

    run_repl();
}
