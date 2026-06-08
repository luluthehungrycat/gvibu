# date

Print or set the system date and time.

## Usage

```
date [OPTION]... [+FORMAT]
```

## Options

| Flag | Description |
|------|-------------|
| `-u`, `--utc`, `--universal` | Print or set Coordinated Universal Time |
| `-R`, `--rfc-2822`, `--rfc-email` | Output date and time in RFC 2822 format |
| `-I`, `--iso-8601` | Output date in ISO 8601 format (YYYY-MM-DD) |

## Format Specifiers

| Specifier | Description |
|-----------|-------------|
| `%Y` | Year (4 digits) |
| `%y` | Year (2 digits) |
| `%m` | Month (01-12) |
| `%d` | Day of month (01-31) |
| `%H` | Hour (00-23) |
| `%I` | Hour (01-12) |
| `%M` | Minute (00-59) |
| `%S` | Second (00-59) |
| `%s` | Seconds since epoch |
| `%A` | Weekday full name |
| `%a` | Weekday abbreviated |
| `%B` | Month full name |
| `%b` | Month abbreviated |
| `%Z` | Timezone name |
| `%z` | Timezone offset |
| `%p` | AM/PM |
| `%j` | Day of year (001-366) |
| `%V` | ISO week number (01-53) |
| `%D` | Date (mm/dd/yy) |
| `%F` | Date (YYYY-MM-DD) |
| `%T` | Time (HH:MM:SS) |
| `%r` | 12-hour time (HH:MM:SS AM/PM) |
| `%R` | 24-hour time (HH:MM) |
| `%%` | Literal percent |
| `%n` | Newline |
| `%t` | Tab |

## Exit Codes

- `0`: Success
- `1`: Error (invalid option, extra operand)

## Examples

```bash
date                          # Mon Jun  8 12:34:56 UTC 2026
date -u                       # UTC time
date -R                       # RFC 2822 format
date -I                       # ISO 8601 date
date "+%Y-%m-%d"              # 2026-06-08
date "+%Y-%m-%d %H:%M:%S"     # 2026-06-08 12:34:56
date "+%s"                    # Epoch seconds
```
