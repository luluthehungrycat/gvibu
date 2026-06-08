/// date: print or set the system date and time.
use std::io::Write;

use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc, Weekday};

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut utc = false;
    let mut rfc2822 = false;
    let mut iso8601 = false;
    let mut format_str: Option<String> = None;

    for arg in args {
        let a = arg.as_str();
        match a {
            "-u" | "--utc" | "--universal" => utc = true,
            "-R" | "--rfc-2822" | "--rfc-email" => rfc2822 = true,
            "-I" | "--iso-8601" => iso8601 = true,
            "--" => break,
            s if s.starts_with('+') => {
                format_str = Some(s[1..].to_string());
            }
            s if s.starts_with('-') && s.len() > 1 => {
                eprintln!("date: invalid option: {}", s);
                return 1;
            }
            _ => {
                eprintln!("date: extra operand '{}'", a);
                return 1;
            }
        }
    }

    let now: DateTime<Local> = Local::now();
    let now_utc = Utc::now();

    let dt: DateTime<Utc> = if utc { now_utc } else { now.into() };

    if rfc2822 {
        // RFC 2822 format: Mon, 08 Jun 2026 12:34:56 +0000
        let rfc_str = dt.format("%a, %d %b %Y %H:%M:%S %z").to_string();
        pwriteln!(w, "{}", rfc_str);
        return 0;
    }

    if iso8601 {
        // ISO 8601: 2026-06-08
        let iso_str = dt.format("%Y-%m-%d").to_string();
        pwriteln!(w, "{}", iso_str);
        return 0;
    }

    if let Some(fmt) = format_str {
        // Custom format string with % specifiers
        let out = format_custom(&dt, &fmt);
        pwriteln!(w, "{}", out);
        return 0;
    }

    // Default format: Mon Jun  8 12:34:56 UTC 2026
    let local = if utc { now_utc.with_timezone(&Local) } else { now };
    let default_fmt = local.format("%a %b %e %H:%M:%S %Z %Y").to_string();
    pwriteln!(w, "{}", default_fmt);
    0
}

fn format_custom(dt: &DateTime<Utc>, fmt: &str) -> String {
    let mut result = String::new();
    let bytes = fmt.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 1 < bytes.len() {
            let spec = bytes[i + 1] as char;
            let replacement = match spec {
                'Y' => format!("{:04}", dt.year()),
                'y' => format!("{:02}", dt.year() % 100),
                'm' => format!("{:02}", dt.month()),
                'd' => format!("{:02}", dt.day()),
                'H' => format!("{:02}", dt.hour()),
                'I' => {
                    let h = dt.hour12().1;
                    format!("{:02}", h)
                }
                'M' => format!("{:02}", dt.minute()),
                'S' => format!("{:02}", dt.second()),
                's' => format!("{}", dt.timestamp()),
                'A' => dt.format("%A").to_string(),
                'a' => dt.format("%a").to_string(),
                'B' => dt.format("%B").to_string(),
                'b' => dt.format("%b").to_string(),
                'Z' => dt.format("%Z").to_string(),
                'z' => dt.format("%z").to_string(),
                'p' => dt.format("%p").to_string(),
                'j' => format!("{:03}", dt.ordinal()),
                'U' => format!("{:02}", (dt.ordinal() - dt.weekday().num_days_from_sunday() as u32 + 6) / 7),
                'W' => format!("{:02}", (dt.ordinal() - dt.weekday().num_days_from_monday() as u32 + 6) / 7),
                'w' => format!("{}", dt.weekday().num_days_from_sunday()),
                'u' => format!("{}", dt.weekday().number_from_monday()),
                'V' => {
                    // ISO week number
                    let iso_week = dt.iso_week();
                    format!("{:02}", iso_week.week())
                }
                'C' => format!("{:02}", dt.year() / 100),
                'D' => dt.format("%m/%d/%y").to_string(),
                'F' => dt.format("%Y-%m-%d").to_string(),
                'T' => dt.format("%H:%M:%S").to_string(),
                'r' => dt.format("%I:%M:%S %p").to_string(),
                'R' => dt.format("%H:%M").to_string(),
                '%' => "%".to_string(),
                'n' => "\n".to_string(),
                't' => "\t".to_string(),
                _ => format!("%{}", spec),
            };
            result.push_str(&replacement);
            i += 2;
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }

    #[test]
    fn test_date_extra_operand() {
        assert_eq!(run(&mut std::io::sink(), &["foo".into()]), 1);
    }

    #[test]
    fn test_date_default() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &[]), 0);
        let out = String::from_utf8_lossy(&buf);
        assert!(out.contains("2026"), "default output should contain year, got: {}", out);
    }

    #[test]
    fn test_date_utc_flag() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["-u".into()]), 0);
        let out = String::from_utf8_lossy(&buf);
        assert!(out.contains("UTC") || out.contains("0000"), "utc output should contain UTC or +0000, got: {}", out);
    }

    #[test]
    fn test_date_custom_format() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["+%Y-%m-%d".into()]), 0);
        let out = String::from_utf8_lossy(&buf).trim().to_string();
        // Should look like 2026-06-08
        assert_eq!(out.len(), 10, "format should be YYYY-MM-DD, got: {}", out);
        assert_eq!(out.chars().filter(|&c| c == '-').count(), 2);
    }

    #[test]
    fn test_date_rfc2822() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["-R".into()]), 0);
        let out = String::from_utf8_lossy(&buf);
        // RFC 2822 format: Mon, 08 Jun 2026 12:34:56 +0000
        assert!(out.contains("2026"), "rfc2822 should contain year, got: {}", out);
        assert!(out.contains(':'), "rfc2822 should contain time, got: {}", out);
    }

    #[test]
    fn test_date_iso8601() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["-I".into()]), 0);
        let out = String::from_utf8_lossy(&buf).trim().to_string();
        assert_eq!(out.len(), 10, "ISO 8601 should be YYYY-MM-DD, got: {}", out);
    }

    #[test]
    fn test_date_epoch_seconds() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["+%s".into()]), 0);
        let out = String::from_utf8_lossy(&buf).trim().to_string();
        assert!(out.parse::<i64>().is_ok(), "epoch seconds should be numeric, got: {}", out);
        // 2026 epoch should be > 1700000000
        assert!(out.len() >= 10, "epoch seconds should be 10+ digits");
    }

    #[test]
    fn test_format_custom() {
        let dt = Utc.with_ymd_and_hms(2026, 6, 8, 12, 34, 56).unwrap();
        assert_eq!(format_custom(&dt, "%Y-%m-%d"), "2026-06-08");
        assert_eq!(format_custom(&dt, "%H:%M:%S"), "12:34:56");
        assert_eq!(format_custom(&dt, "%A"), "Monday");
        assert_eq!(format_custom(&dt, "%a"), "Mon");
        assert_eq!(format_custom(&dt, "%B"), "June");
        assert_eq!(format_custom(&dt, "%b"), "Jun");
        assert_eq!(format_custom(&dt, "%%"), "%");
        assert_eq!(format_custom(&dt, "%s"), "1788928496");
        assert_eq!(format_custom(&dt, "%j"), "159");
        assert_eq!(format_custom(&dt, "hello world"), "hello world");
    }
}
