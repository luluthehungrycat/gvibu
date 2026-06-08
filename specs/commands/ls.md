# ls

List directory contents.

## Usage

```
ls [OPTION]... [FILE]...
```

## Options

| Flag | Description |
|------|-------------|
| `-l` | Use long listing format |
| `-a` | Show all files (including . and ..) |
| `-A` | Show almost all files (excluding . and ..) |
| `-h` | Human-readable sizes (with -l) |
| `-t` | Sort by modification time, newest first |
| `-r` | Reverse sort order |
| `-S` | Sort by file size, largest first |
| `-1` | List one file per line (default) |

## Arguments

- `FILE`: Files or directories to list. Default: current directory.

## Exit Codes

- `0`: Success
- `1`: Error (invalid option, cannot access path)

## Examples

```bash
ls                  # List current directory
ls -l               # Long listing format
ls -la              # Long listing including hidden files
ls -ltr             # Long listing sorted by time, reversed
ls -lh              # Long listing with human-readable sizes
ls /tmp             # List /tmp directory
```
