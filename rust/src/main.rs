use gvibu::commands;
use std::env;
use std::io;
use std::process;

fn resolve_command<'a>(argv0_basename: &'a str, args: &'a [String]) -> Option<(&'a str, &'a [String])> {
    // Symlink mode: argv0 basename is the command name
    if commands::lookup(argv0_basename).is_some() {
        return Some((argv0_basename, &args[1..]));
    }

    // Subcommand mode: argv0 is the binary name, args[1] is the command
    if commands::BINARY_NAMES.contains(&argv0_basename.as_ref()) && args.len() > 1 {
        let cmd_name = &args[1];
        if commands::lookup(cmd_name).is_some() {
            return Some((cmd_name.as_str(), &args[2..]));
        }
    }

    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let argv0 = &args[0];
    let argv0_basename = std::path::Path::new(argv0)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();

    let (cmd_name, run_args) = match resolve_command(&argv0_basename, &args) {
        Some(result) => result,
        None => {
            eprintln!("gvibu: {}: command not found", argv0_basename);
            process::exit(1);
        }
    };

    if let Some(cmd) = commands::lookup(cmd_name) {
        let exit_code = (cmd.run)(&mut io::stdout(), run_args);
        process::exit(exit_code);
    } else {
        eprintln!("gvibu: {}: command not found", cmd_name);
        process::exit(1);
    }
}
