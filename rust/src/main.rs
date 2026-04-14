mod commands;

use std::env;
use std::process;

fn get_command_name() -> String {
    let argv0 = env::args().next().unwrap_or_default();
    let argv0_basename = std::path::Path::new(&argv0)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();
    
    let args: Vec<String> = env::args().collect();
    
    if commands::TRUE.contains(&argv0_basename.as_str()) {
        return argv0_basename;
    }
    if commands::FALSE.contains(&argv0_basename.as_str()) {
        return argv0_basename;
    }
    if commands::ECHO.contains(&argv0_basename.as_str()) {
        return argv0_basename;
    }
    if commands::PWD.contains(&argv0_basename.as_str()) {
        return argv0_basename;
    }
    
    if args.len() > 1 {
        let cmd = &args[1];
        if commands::TRUE.contains(&cmd.as_str()) {
            return cmd.clone();
        }
        if commands::FALSE.contains(&cmd.as_str()) {
            return cmd.clone();
        }
        if commands::ECHO.contains(&cmd.as_str()) {
            return cmd.clone();
        }
        if commands::PWD.contains(&cmd.as_str()) {
            return cmd.clone();
        }
    }
    
    argv0_basename
}

fn main() {
    let cmd_name = get_command_name();
    let args: Vec<String> = env::args().collect();
    let argv0 = env::args().next().unwrap_or_default();
    let argv0_basename = std::path::Path::new(&argv0)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();
    
    let run_args: Vec<String> = if commands::TRUE.contains(&argv0_basename.as_str()) 
        || commands::FALSE.contains(&argv0_basename.as_str())
        || commands::ECHO.contains(&argv0_basename.as_str())
        || commands::PWD.contains(&argv0_basename.as_str())
    {
        args[1..].to_vec()
    } else if commands::TRUE.contains(&cmd_name.as_str())
        || commands::FALSE.contains(&cmd_name.as_str())
        || commands::ECHO.contains(&cmd_name.as_str())
        || commands::PWD.contains(&cmd_name.as_str())
    {
        args[2..].to_vec()
    } else {
        args[1..].to_vec()
    };
    
    let exit_code = if commands::TRUE.contains(&cmd_name.as_str()) {
        commands::true_cmd::run(&run_args)
    } else if commands::FALSE.contains(&cmd_name.as_str()) {
        commands::false_cmd::run(&run_args)
    } else if commands::ECHO.contains(&cmd_name.as_str()) {
        commands::echo_cmd::run(&run_args)
    } else if commands::PWD.contains(&cmd_name.as_str()) {
        commands::pwd_cmd::run(&run_args)
    } else {
        eprintln!("gvibu: {}: command not found", cmd_name);
        1
    };
    
    process::exit(exit_code);
}
