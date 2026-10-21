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
| `-F` | Append `/`, `*`, or `@` indicators to directories, executables, or symlinks |
| `-d` | List directory arguments themselves instead of their contents |
| `-R` | List subdirectories recursively |
| `-i` | Show inode numbers |
| `--color[=WHEN]` | Color names (`always`, `never`, or `auto`; bare `--color` means `auto`) |

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
ls -F --color=auto  # Add file indicators and color when stdout is a terminal
ls -d /tmp          # List the directory entry itself
ls -R /tmp          # List subdirectories recursively
ls /tmp             # List /tmp directory
```
