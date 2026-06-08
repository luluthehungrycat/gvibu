# du

Estimate file space usage.

## Usage

```
du [OPTION]... [FILE]...
```

## Options

| Flag | Description |
|------|-------------|
| `-h` | Print sizes in human-readable format (K, M, G) |
| `-s` | Display only a total for each argument (summary) |

## Arguments

- `FILE`: One or more files or directories. Defaults to current directory.

## Exit Codes

- `0`: Success
- `1`: Error (invalid option, file read error)

## Examples

```bash
du                     # Disk usage of current directory
du -h /home            # Human-readable sizes
du -s /var/log         # Summary only
du -sh /usr            # Human-readable summary
```
