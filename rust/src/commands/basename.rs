use std::ffi::OsString;

pub fn run(args: &[OsString]) -> i32 {
    if args.is_empty() || (args.len() == 1 && args[0].to_string_lossy().is_empty()) {
        eprintln!("basename: usage: basename NAME [SUFFIX]");
        return 1;
    }

    let mut name = args[0].to_string_lossy().trim_matches('/').to_string();
    // If there are suffix args, remove if present
    if args.len() > 1 {
        let suffix = args[1].to_string_lossy().into_owned();
        if !suffix.is_empty() && name.ends_with(&suffix) {
            name.truncate(name.len() - suffix.len());
        }
    }
    println!("{}", name);
    return if args.len() > 1 { 1 } else { 0 };
}
