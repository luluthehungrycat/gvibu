/// date: print or set the system date and time.
use std::io::Write;
use crate::pwriteln;

use chrono::{DateTime, Datelike, Local, Timelike, Utc, Duration, NaiveDate};

/// Parse a date string and return a DateTime<Utc>
fn parse_date_string(date_str: &str, reference: DateTime<Utc>) -> Result<DateTime<Utc>, String> {
    let date_str = date_str.trim();
    
    // Handle special keywords
    match date_str.to_lowercase().as_str() {
        "now" | "today" => Ok(reference),
        "yesterday" => Ok(reference - Duration::days(1)),
        "tomorrow" => Ok(reference + Duration::days(1)),
        _ => {
            // Try parsing as ISO 8601 date (YYYY-MM-DD)
            if let Ok(naive_date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                return Ok(naive_date.and_hms_opt(0, 0, 0).expect("0,0,0 is always valid NaiveTime").and_local_timezone(Utc).single().expect("Utc always succeeds"));
            }
            
            // Try parsing as full ISO 8601 datetime
            if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
                return Ok(dt.with_timezone(&Utc));
            }
            
            // Try parsing with various common formats
            let formats = [
                "%Y-%m-%d %H:%M:%S",
                "%Y-%m-%d %H:%M",
                "%Y/%m/%d",
                "%d %b %Y",
                "%b %d, %Y",
                "%B %d, %Y",
                "%d %B %Y",
            ];
            
            for fmt in formats.iter() {
                if let Ok(naive_dt) = NaiveDate::parse_from_str(date_str, fmt) {
                    return Ok(naive_dt.and_hms_opt(0, 0, 0).expect("0,0,0 is always valid NaiveTime").and_local_timezone(Utc).single().expect("Utc always succeeds"));
                }
            }
            
            // Try DateTime parsing with chrono's flexible parser
            if let Ok(dt) = DateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S %z") {
                return Ok(dt.with_timezone(&Utc));
            }
            
            // Handle relative dates like "+1 day", "-2 weeks", etc.
            if date_str.starts_with('+') || date_str.starts_with('-') {
                return parse_relative_date(date_str, reference);
            }
            
            Err(format!("date: invalid date '{}'", date_str))
        }
    }
}

/// Parse relative date strings like "+1 day", "-2 weeks", etc.
fn parse_relative_date(date_str: &str, reference: DateTime<Utc>) -> Result<DateTime<Utc>, String> {
    let date_str = date_str.trim();
    
    // Parse the sign and value
    let (sign, rest) = if date_str.starts_with('+') {
        (1, &date_str[1..])
    } else if date_str.starts_with('-') {
        (-1, &date_str[1..])
    } else {
        return Err(format!("date: invalid relative date '{}'", date_str));
    };
    
    let parts: Vec<&str> = rest.trim().split_whitespace().collect();
    if parts.len() != 2 {
        return Err(format!("date: invalid relative date format '{}'", date_str));
    }
    
    let value: i64 = parts[0].parse().map_err(|_| format!("date: invalid number in '{}'", date_str))?;
    let unit = parts[1].to_lowercase();
    
    let multiplier = match unit.as_str() {
        "second" | "seconds" | "sec" | "s" => 1,
        "minute" | "minutes" | "min" | "m" => 60,
        "hour" | "hours" | "hr" | "h" => 3600,
        "day" | "days" | "d" => 86400,
        "week" | "weeks" | "w" => 604800,
        "month" | "months" | "mon" => {
            // For months, approximate as 30 days
            let days = value * sign * 30;
            return Ok(reference + Duration::days(days));
        }
        "year" | "years" | "yr" | "y" => {
            // For years, approximate as 365 days
            let days = value * sign * 365;
            return Ok(reference + Duration::days(days));
        }
        _ => return Err(format!("date: unknown time unit '{}'", unit)),
    };
    
    let total_seconds = value * multiplier * sign;
    Ok(reference + Duration::seconds(total_seconds))
}

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut utc = false;
    let mut rfc2822 = false;
    let mut iso8601 = false;
    let mut format_str: Option<String> = None;
    let mut date_str: Option<String> = None;
    let mut parsing_date_arg = false;

    let mut i = 0;
    while i < args.len() {
        let arg = args[i].as_str();
        
        if parsing_date_arg {
            date_str = Some(arg.to_string());
            parsing_date_arg = false;
            i += 1;
            continue;
        }
        
        match arg {
            "-d" | "--date" => {
                parsing_date_arg = true;
                i += 1;
                continue;
            }
            "-u" | "--utc" | "--universal" => utc = true,
            "-R" | "--rfc-2822" | "--rfc-email" => rfc2822 = true,
            "-I" | "--iso-8601" => iso8601 = true,
            "--" => {
                i += 1;
                break;
            }
            s if s.starts_with('+') => {
                format_str = Some(s[1..].to_string());
            }
            s if s.starts_with('-') && s.len() > 1 => {
                eprintln!("date: invalid option: {}", s);
                return 1;
            }
            _ => {
                // If we encounter a non-option argument and we're not parsing date,
                // it's an extra operand
                eprintln!("date: extra operand '{}'", arg);
                return 1;
            }
        }
        i += 1;
    }

    // Check if -d flag was provided without a date argument
    if parsing_date_arg {
        eprintln!("date: option requires an argument -- 'd'");
        return 1;
    }

    let now: DateTime<Local> = Local::now();
    let now_utc = Utc::now();
    let reference_dt = if utc { now_utc } else { now.into() };

    // Parse the date string if provided
    let dt: DateTime<Utc> = if let Some(date_input) = date_str {
        match parse_date_string(&date_input, reference_dt) {
            Ok(parsed_dt) => parsed_dt,
            Err(e) => {
                eprintln!("{}", e);
                return 1;
            }
        }
    } else {
        if utc { now_utc } else { now.into() }
    };

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
    let default_fmt = if utc {
        dt.format("%a %b %e %H:%M:%S %Z %Y").to_string()
    } else {
        // Convert to local timezone for display
        let local_dt: DateTime<Local> = dt.into();
        local_dt.format("%a %b %e %H:%M:%S %Z %Y").to_string()
    };
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
    use chrono::TimeZone;

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
    fn test_date_with_d_flag_iso() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["-d".into(), "2024-01-01".into()]), 0);
        let out = String::from_utf8_lossy(&buf).trim().to_string();
        assert!(out.contains("2024") && out.contains("01"), "should contain 2024-01, got: {}", out);
    }

    #[test]
    fn test_date_with_d_flag_yesterday() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["-d".into(), "yesterday".into(), "+%Y-%m-%d".into()]), 0);
        let out = String::from_utf8_lossy(&buf).trim().to_string();
        // Should be a valid date in YYYY-MM-DD format
        assert_eq!(out.len(), 10, "yesterday should be YYYY-MM-DD, got: {}", out);
        assert_eq!(out.chars().filter(|&c| c == '-').count(), 2);
    }

    #[test]
    fn test_date_with_d_flag_tomorrow() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["-d".into(), "tomorrow".into(), "+%Y-%m-%d".into()]), 0);
        let out = String::from_utf8_lossy(&buf).trim().to_string();
        // Should be a valid date in YYYY-MM-DD format
        assert_eq!(out.len(), 10, "tomorrow should be YYYY-MM-DD, got: {}", out);
        assert_eq!(out.chars().filter(|&c| c == '-').count(), 2);
    }

    #[test]
    fn test_date_with_d_flag_now() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["-d".into(), "now".into()]), 0);
        let out = String::from_utf8_lossy(&buf);
        assert!(out.contains("2026"), "now should contain current year, got: {}", out);
    }

    #[test]
    fn test_date_with_d_flag_invalid() {
        assert_eq!(run(&mut std::io::sink(), &["-d".into(), "invalid-date".into()]), 1);
    }

    #[test]
    fn test_date_with_d_flag_missing_arg() {
        assert_eq!(run(&mut std::io::sink(), &["-d".into()]), 1);
    }

    #[test]
    fn test_date_with_date_flag() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["--date".into(), "2024-12-25".into(), "+%Y-%m-%d".into()]), 0);
        let out = String::from_utf8_lossy(&buf).trim().to_string();
        assert_eq!(out, "2024-12-25");
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
        assert_eq!(format_custom(&dt, "%s"), dt.timestamp().to_string());
        assert_eq!(format_custom(&dt, "%j"), "159");
        assert_eq!(format_custom(&dt, "hello world"), "hello world");
    }
}
