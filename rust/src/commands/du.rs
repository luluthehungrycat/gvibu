/// du: estimate file space usage.
use std::fs;
use std::io::Write;
use crate::pwriteln;
use std::path::Path;

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut human = false;
    let mut summary = false;
    let mut max_depth: Option<usize> = None;
    let mut paths: Vec<&str> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" => human = true,
            "-s" => summary = true,
            "-d" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<usize>() {
                        Ok(d) => max_depth = Some(d),
                        Err(_) => {
                            pwriteln!(w, "du: invalid max depth: {}", args[i + 1]);
                            return 1;
                        }
                    }
                    i += 1;
                } else {
                    pwriteln!(w, "du: -d requires an argument");
                    return 1;
                }
            }
            arg if arg.starts_with("--max-depth=") => {
                let value = &arg[12..]; // "--max-depth=" is 12 chars
                match value.parse::<usize>() {
                    Ok(d) => max_depth = Some(d),
                    Err(_) => {
                        eprintln!("du: invalid max depth: {}", value);
                        return 1;
                    }
                }
            }
            "--max-depth" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<usize>() {
                        Ok(d) => max_depth = Some(d),
                        Err(_) => {
                            eprintln!("du: invalid max depth: {}", args[i + 1]);
                            return 1;
                        }
                    }
                    i += 1;
                } else {
                    eprintln!("du: --max-depth requires an argument");
                    return 1;
                }
            }
            "--" => { break; }
            arg if arg.starts_with('-') && arg.len() > 1 => {
                // Handle bundled flags: -hs, -sh
                let mut j = 1;
                while j < arg.len() {
                    let c = arg.chars().nth(j).unwrap();
                    match c {
                        'h' => human = true,
                        's' => summary = true,
                        '0'..='9' => {
                            // Handle -dN or --max-depth=N
                            let num_str: String = arg.chars().skip(j).collect();
                            if let Ok(d) = num_str.parse::<usize>() {
                                max_depth = Some(d);
                                break;
                            } else {
                                eprintln!("du: invalid option: -{}", c);
                                return 1;
                            }
                        }
                        _ => {
                            eprintln!("du: invalid option: -{}", c);
                            return 1;
                        }
                    }
                    j += 1;
                }
            }
            p => paths.push(p),
        }
        i += 1;
    }

    if paths.is_empty() {
        paths.push(".");
    }

    let mut grand_total: u64 = 0;
    let mut exit_code = 0;

    for path_str in &paths {
        let path = Path::new(path_str);
        match du_walk(path, summary, max_depth) {
            Ok(entries) => {
                for (size, p, depth) in &entries {
                    let display = if *p == "." { "." } else { p };
                    if human {
                        pwriteln!(w, "{}\t{}", human_size(*size), display);
                    } else {
                        pwriteln!(w, "{}\t{}", size, display);
                    }
                }
                if let Some((total, _, _)) = entries.last() {
                    grand_total += total;
                }
            }
            Err(e) => {
                eprintln!("du: {}: {}", path_str, e);
                exit_code = 1;
            }
        }
    }

    if grand_total > 0 && paths.len() > 1 {
        if human {
            pwriteln!(w, "{}\ttotal", human_size(grand_total));
        } else {
            pwriteln!(w, "{}\ttotal", grand_total);
        }
    }

    exit_code
}

fn du_walk(path: &Path, summary: bool, max_depth: Option<usize>) -> Result<Vec<(u64, String, usize)>, String> {
    let meta = fs::symlink_metadata(path).map_err(|e| format!("{}", e))?;

    if meta.is_dir() && !summary {
        let mut entries: Vec<(u64, String, usize)> = Vec::new();
        let dir_entries = fs::read_dir(path).map_err(|e| format!("{}", e))?;

        for entry in dir_entries {
            let entry = entry.map_err(|e| format!("{}", e))?;
            let sub_path = entry.path();
            let sub_meta = fs::symlink_metadata(&sub_path).map_err(|e| format!("{}", e))?;

            // Skip symlinks to avoid counting them multiple times
            if sub_meta.file_type().is_symlink() {
                continue;
            }

            if sub_meta.is_dir() {
                // Check depth limit
                let current_depth = count_path_depth(&sub_path, path);
                if let Some(max) = max_depth {
                    if current_depth > max {
                        continue;
                    }
                }
                let sub_entries = du_walk(&sub_path, false, max_depth)?;
                entries.extend(sub_entries);
            } else {
                entries.push((sub_meta.len(), sub_path.to_string_lossy().to_string(), 0));
            }
        }

        // Sort by path for deterministic output
        entries.sort_by(|a, b| a.1.cmp(&b.1));

        let dir_size: u64 = entries.iter().map(|(s, _, _)| s).sum();
        let depth = count_path_depth(path, path);
        entries.push((dir_size, path.to_string_lossy().to_string(), depth));
        Ok(entries)
    } else {
        let size = meta.len();
        Ok(vec![(size, path.to_string_lossy().to_string(), 0)])
    }
}

fn count_path_depth(path: &Path, base: &Path) -> usize {
    let path_str = path.to_string_lossy();
    let base_str = base.to_string_lossy();
    
    if path_str == base_str {
        return 0;
    }
    
    path_str.split('/').count().saturating_sub(base_str.split('/').count())
}

fn human_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["", "K", "M", "G", "T", "P"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    if unit_idx == 0 {
        format!("{}", bytes)
    } else if size >= 100.0 {
        format!("{:.0}{}", size, UNITS[unit_idx])
    } else if size >= 10.0 {
        format!("{:.1}{}", size, UNITS[unit_idx])
    } else {
        format!("{:.1}{}", size, UNITS[unit_idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dev_null() {
        assert_eq!(run(&mut std::io::sink(), &["/dev/null".into()]), 0);
    }

    #[test]
    fn test_human_readable_dev_null() {
        assert_eq!(run(&mut std::io::sink(), &["-h".into(), "/dev/null".into()]), 0);
    }

    #[test]
    fn test_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }

    #[test]
    fn test_bundled_flags() {
        assert_eq!(run(&mut std::io::sink(), &["-hs".into(), "/dev/null".into()]), 0);
    }

    #[test]
    fn test_human_size_zero() {
        assert_eq!(human_size(0), "0");
    }

    #[test]
    fn test_human_size_bytes() {
        assert_eq!(human_size(500), "500");
    }

    #[test]
    fn test_human_size_kb() {
        assert_eq!(human_size(2048), "2.0K");
    }

    #[test]
    fn test_human_size_mb() {
        assert_eq!(human_size(1048576), "1.0M");
    }

    #[test]
    fn test_human_size_gb() {
        let gb = 1073741824u64;
        let result = human_size(gb);
        assert_eq!(result, "1.0G");
    }

    #[test]
    fn test_max_depth_flag() {
        assert_eq!(run(&mut std::io::sink(), &["--max-depth=1".into(), "/dev/null".into()]), 0);
    }

    #[test]
    fn test_count_path_depth() {
        let path = Path::new("/tmp/test");
        let base = Path::new("/tmp");
        assert_eq!(count_path_depth(path, base), 1);
        assert_eq!(count_path_depth(base, base), 0);
    }
}
