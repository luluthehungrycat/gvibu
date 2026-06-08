/// tr: translate or delete characters.
/// Fully streaming: O(1) memory per input chunk.
use std::collections::HashSet;
use std::io::{self, Read, Write};

fn expand_set(s: &str) -> Vec<u8> {
    let mut result = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if i + 2 < bytes.len() && bytes[i + 1] == b'-' && bytes[i] < bytes[i + 2] {
            for c in bytes[i]..=bytes[i + 2] {
                result.push(c);
            }
            i += 3;
        } else if bytes[i] == b'\\' && i + 1 < bytes.len() {
            match bytes[i + 1] {
                b'n' => result.push(b'\n'),
                b't' => result.push(b'\t'),
                b'r' => result.push(b'\r'),
                b'\\' => result.push(b'\\'),
                b'0' => result.push(b'\0'),
                c => result.push(c),
            }
            i += 2;
        } else {
            result.push(bytes[i]);
            i += 1;
        }
    }
    // Deduplicate but preserve order
    let mut seen = HashSet::new();
    result.retain(|c| seen.insert(*c));
    result
}

/// Expand a set that may include complement (everything except listed chars).
fn build_char_set(s: &str, complement: bool) -> Vec<u8> {
    if complement {
        let excluded: HashSet<u8> = s.bytes().collect();
        let mut all: Vec<u8> = (0..=127).filter(|c| !excluded.contains(c)).collect();
        for c in 0x80u8..=0xFFu8 {
            if !excluded.contains(&c) {
                all.push(c);
            }
        }
        all
    } else {
        expand_set(s)
    }
}

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let mut delete = false;
    let mut squeeze = false;
    let mut complement = false;
    let mut sets: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--" { i += 1; break; }
        if arg == "-d" {
            delete = true;
        } else if arg == "-s" {
            squeeze = true;
        } else if arg == "-c" || arg == "-C" {
            complement = true;
        } else if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("tr: invalid option: {}", arg);
            return 1;
        } else {
            sets.push(arg.clone());
        }
        i += 1;
    }

    if delete && sets.len() < 1 {
        eprintln!("tr: missing operand");
        return 1;
    }
    if !delete && sets.len() < 2 {
        eprintln!("tr: missing operand");
        return 1;
    }

    let set1_str = if sets.is_empty() { "" } else { &sets[0] };
    let set2_str = if sets.len() < 2 { "" } else { &sets[1] };

    let set1 = build_char_set(set1_str, complement);

    let set2: Vec<u8> = if delete {
        Vec::new()
    } else {
        let expanded = expand_set(set2_str);
        if expanded.is_empty() {
            if let Some(&last) = set1.last() {
                vec![last; set1.len()]
            } else {
                Vec::new()
            }
        } else {
            expanded.iter().cycle().take(set1.len()).copied().collect()
        }
    };

    // Build translation map: [256] of Option<u8>
    let mut translate_map: [Option<u8>; 256] = [None; 256];
    if !delete {
        for (i, &c) in set1.iter().enumerate() {
            if i < set2.len() {
                translate_map[c as usize] = Some(set2[i]);
            } else if let Some(&last) = set2.last() {
                translate_map[c as usize] = Some(last);
            }
        }
    }

    // Delete set as byte lookup table (faster than HashSet)
    let mut delete_set: [bool; 256] = [false; 256];
    if delete {
        for &c in &set1 {
            delete_set[c as usize] = true;
        }
    }

    // Stream stdin in chunks
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut in_buf = [0u8; 8192];
    let mut out_buf = [0u8; 8192];
    let mut out_pos = 0;
    let mut prev: Option<u8> = None;

    loop {
        let n = match handle.read(&mut in_buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => return 1,
        };

        for &byte in &in_buf[..n] {
            // Delete mode
            if delete_set[byte as usize] {
                continue;
            }

            // Translate mode
            let translated = if !delete {
                translate_map[byte as usize].unwrap_or(byte)
            } else {
                byte
            };

            // Squeeze mode
            if squeeze {
                if Some(translated) == prev {
                    continue;
                }
                prev = Some(translated);
            }

            // Buffer output
            out_buf[out_pos] = translated;
            out_pos += 1;

            if out_pos == 8192 {
                if let Err(e) = stdout.write_all(&out_buf) {
                    if e.kind() == std::io::ErrorKind::BrokenPipe {
                        return 0;
                    }
                    return 1;
                }
                out_pos = 0;
            }
        }
    }

    // Flush remaining output
    if out_pos > 0 {
        if let Err(e) = stdout.write_all(&out_buf[..out_pos]) {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                return 0;
            }
            return 1;
        }
    }

    let _ = stdout.flush();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tr_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_tr_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }

    #[test]
    fn test_expand_set_range() {
        let result = expand_set("a-z");
        assert_eq!(result.len(), 26);
        assert_eq!(result[0], b'a');
        assert_eq!(result[25], b'z');
    }

    #[test]
    fn test_expand_set_literal() {
        let result = expand_set("abc");
        assert!(result.contains(&b'a'));
        assert!(result.contains(&b'b'));
        assert!(result.contains(&b'c'));
    }
}
