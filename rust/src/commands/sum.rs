/// sum: compute BSD 16-bit checksum and block count.
use std::fs;
use std::io::Write;
use crate::pwriteln;

const BLOCK_SIZE: usize = 1024;

fn bsd_checksum(data: &[u8]) -> u16 {
    let mut checksum = 0u16;
    for &byte in data {
        let carry = (checksum >> 1) | (checksum & 1) << 15;
        checksum = carry.wrapping_add(byte as u16);
    }
    checksum
}

fn block_count(len: usize) -> usize {
    (len + BLOCK_SIZE - 1) / BLOCK_SIZE
}

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let files: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    if files.is_empty() {
        eprintln!("sum: missing operand");
        return 1;
    }

    for fname in files {
        match fs::read(fname) {
            Ok(data) => {
                let cksum = bsd_checksum(&data);
                let blocks = block_count(data.len());
                pwriteln!(w, "{} {}", cksum, blocks);
            }
            Err(e) => {
                eprintln!("sum: {}: {}", fname, e);
                return 1;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsd_checksum_empty() {
        assert_eq!(bsd_checksum(b""), 0);
    }

    #[test]
    fn test_bsd_checksum_simple() {
        // b"abc" -> 97+98+99 with rotations
        let cksum = bsd_checksum(b"abc");
        assert_ne!(cksum, 0);
    }

    #[test]
    fn test_block_count() {
        assert_eq!(block_count(0), 0);
        assert_eq!(block_count(1), 1);
        assert_eq!(block_count(1024), 1);
        assert_eq!(block_count(1025), 2);
    }

    #[test]
    fn test_sum_nonexistent() {
        let mut out: Vec<u8> = Vec::new();
        let code = run(&mut out, &["/nonexistent_sum_file_xyz".into()]);
        assert_eq!(code, 1);
    }

    #[test]
    fn test_sum_no_args() {
        let mut out: Vec<u8> = Vec::new();
        let code = run(&mut out, &[]);
        assert_eq!(code, 1);
    }
}
