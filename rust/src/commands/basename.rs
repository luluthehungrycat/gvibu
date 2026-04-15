use std::string::ToString;

pub fn run(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("basename: usage: basename NAME [SUFFIX]");
        return 1;
    }
    let mut name = args[0].trim_matches('/').to_string();
    // Extract final path component (basename) first
    if let Some(pos) = name.rfind('/') {
        name = name[(pos + 1)..].to_string();
    }
    // If there are suffix args, remove if present
    if args.len() > 1 {
        let suffix = &args[1];
        if !suffix.is_empty() && name.ends_with(suffix) {
            name.truncate(name.len() - suffix.len());
        }
    }
    println!("{}", name);
    return if args.len() > 1 { 1 } else { 0 };
}
