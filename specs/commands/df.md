# df

Report file system space usage.

## Usage

```
df [OPTION]... [FILE]...
```

## Options

| Flag | Description |
|------|-------------|
| `-h` | Print sizes in human-readable format (K, M, G) |
| `-T` | Print filesystem type |

## Arguments

- `FILE`: One or more files or mount points. If none, show all mounted filesystems.

## Exit Codes

- `0`: Success
- `1`: Error (invalid option, cannot read mounts)

## Examples

```bash
df                     # Show all filesystems
df -h                  # Human-readable sizes
df -T /home            # Show filesystem type
df -hT /var/log        # Human-readable with type
```
