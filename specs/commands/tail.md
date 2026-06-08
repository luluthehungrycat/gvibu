# tail

Output the last part of files.

## Usage

```
tail [OPTION]... [FILE]...
```

## Options

| Flag | Description |
|------|-------------|
| `-n N` | Output the last N lines (default: 10) |
| `-c N` | Output the last N bytes |

## Arguments

- `FILE`: One or more files to read. If `-` or no files, read from stdin.

## Exit Codes

- `0`: Success
- `1`: Error (invalid option, bad argument, file read error)

## Examples

```bash
tail /var/log/syslog     # Last 10 lines
tail -n 20 file.txt      # Last 20 lines
tail -c 100 file.bin     # Last 100 bytes
```
