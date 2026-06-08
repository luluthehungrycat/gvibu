/// du: estimate file space usage.
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut human = false;
    let mut summary = false;
    let mut paths: Vec<&str> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" => human = true,
            "-s" => summary = true,
            "--" => { i += 1; break; }
            arg if arg.starts_with('-') && arg.len() > 1 => {
                // Handle bundled flags: -hs, -sh
                for c in arg[1..].chars() {
                    match c {
                        'h' => human = true,
                        's' => summary = true,
                        _ => {
                            pwriteln!(w, "du: invalid option: -{}", c);
                            return 1;
                        }
                    }
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

    for path_str in paths {
        let path = Path::new(path_str);
        match du_walk(path, summary) {
            Ok(entries) => {
                for (size, p) in &entries {
                    let display = if *p == "." { "." } else { p };
                    if human {
                        pwriteln!(w, "{}\t{}", human_size(*size), display);
                    } else {
                        pwriteln!(w, "{}\t{}", size, display);
                    }
                }
                if let Some((total, _)) = entries.last() {
                    grand_total += total;
                }
            }
            Err(e) => {
                pwriteln!(w, "du: {}: {}", path_str, e);
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

fn du_walk(path: &Path, summary: bool) -> Result<Vec<(u64, String)>, String> {
    let meta = fs::symlink_metadata(path).map_err(|e| format!("{}", e))?;

    if meta.is_dir() && !summary {
        let mut entries: Vec<(u64, String)> = Vec::new();
        let dir_entries = fs::read_dir(path).map_err(|e| format!("{}", e))?;

        for entry in dir_entries {
            let entry = entry.map_err(|e| format!("{}", e))?;
            let sub_path = entry.path();
            let sub_meta = fs::symlink_metadata(&sub_path).map_err(|e| format!("{}", e))?;

            if sub_meta.is_dir() {
                let sub_entries = du_walk(&sub_path, false)?;
                entries.extend(sub_entries);
            } else {
                entries.push((sub_meta.len(), sub_path.to_string_lossy().to_string()));
            }
        }

        // Sort by path for deterministic output
        entries.sort_by(|a, b| a.1.cmp(&b.1));

        let dir_size: u64 = entries.iter().map(|(s, _)| s).sum();
        entries.push((dir_size, path.to_string_lossy().to_string()));
        Ok(entries)
    } else {
        let size = meta.len();
        Ok(vec![(size, path.to_string_lossy().to_string())])
    }
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
        assert_eq!(run(&mut std::io::sink(), &["-h", "/dev/null".into()]), 0);
    }

    #[test]
    fn test_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }

    #[test]
    fn test_bundled_flags() {
        assert_eq!(run(&mut std::io::sink(), &["-hs", "/dev/null".into()]), 0);
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
}
