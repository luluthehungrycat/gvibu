pub mod basename;
pub mod cat;
pub mod dirname;
pub mod head;
pub mod wc;
pub mod echo_cmd;
pub mod false_cmd;
pub mod pwd_cmd;
pub mod true_cmd;
pub mod yes;
pub mod printenv;
pub mod sleep;

pub const BINARY_NAMES: &[&str] = &["gvibu", "gvibu-ref"];

pub struct Command {
    pub names: &'static [&'static str],
    pub run: fn(&[String]) -> i32,
}

pub const COMMANDS: &[Command] = &[
    Command { names: &["true"], run: true_cmd::run },
    Command { names: &["false"], run: false_cmd::run },
    Command { names: &["echo"], run: echo_cmd::run },
    Command { names: &["pwd"], run: pwd_cmd::run },
    Command { names: &["basename"], run: basename::run },
    Command { names: &["dirname"], run: dirname::run },
    Command { names: &["cat"], run: cat::run },
    Command { names: &["wc"], run: wc::run },
    Command { names: &["head"], run: head::run },
    Command { names: &["yes"], run: yes::run },
    Command { names: &["printenv"], run: printenv::run },
    Command { names: &["sleep"], run: sleep::run },
];

pub fn lookup(name: &str) -> Option<&'static Command> {
    COMMANDS.iter().find(|cmd| cmd.names.contains(&name))
}
