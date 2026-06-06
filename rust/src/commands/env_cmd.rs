/// env: print or set environment variables.
use std::collections::BTreeMap;
use std::env;
use std::process;

pub fn run(args: &[String]) -> i32 {
    let mut ignore_env = false;
    let mut unset_vars: Vec<String> = Vec::new();
    let mut set_vars: BTreeMap<String, String> = BTreeMap::new();
    let mut cmd_and_args: Vec<String> = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        if arg == "-i" || arg == "--ignore-environment" {
            ignore_env = true;
        } else if arg == "-u" || arg == "--unset" {
            i += 1;
            if i >= args.len() {
                eprintln!("env: option requires an argument: -u");
                return 1;
            }
            unset_vars.push(args[i].clone());
        } else {
            break;
        }
        i += 1;
    }

    // Collect VAR=VAL assignments and [COMMAND [ARG]...]
    let mut assignments_done = false;
    for arg in args[i..].iter() {
        if !assignments_done && arg.contains('=') && !arg.starts_with('-') {
            if let Some(pos) = arg.find('=') {
                let key = &arg[..pos];
                let val = &arg[pos + 1..];
                set_vars.insert(key.to_string(), val.to_string());
            }
        } else {
            assignments_done = true;
            cmd_and_args.push(arg.clone());
        }
    }

    // Build environment
    let env_map: BTreeMap<String, String> = if ignore_env {
        BTreeMap::new()
    } else {
        env::vars().collect()
    };

    let mut final_env: BTreeMap<String, String> = env_map;
    for (key, val) in &set_vars {
        final_env.insert(key.clone(), val.clone());
    }
    for key in &unset_vars {
        final_env.remove(key);
    }

    if cmd_and_args.is_empty() {
        for (key, value) in &final_env {
            println!("{}={}", key, value);
        }
        return 0;
    }

    // Run command
    let cmd_name = &cmd_and_args[0];
    let cmd_rest: Vec<&str> = cmd_and_args[1..].iter().map(|s| s.as_str()).collect();

    // Convert env BTreeMap to Vec<(&str, &str)>
    let env_vec: Vec<(&str, &str)> = final_env.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();

    match process::Command::new(cmd_name)
        .args(&cmd_rest)
        .env_clear()
        .envs(env_vec.iter().copied())
        .status()
    {
        Ok(status) => {
            if let Some(code) = status.code() {
                code
            } else {
                1
            }
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("env: {}: No such file or directory", cmd_name);
                127
            } else if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("env: {}: Permission denied", cmd_name);
                126
            } else {
                eprintln!("env: {}", e);
                1
            }
        }
    }
}
