# grep

Print lines matching a pattern.

## Usage

```
grep [OPTION]... PATTERN [FILE]...
```

## Options

| Flag | Description |
|------|-------------|
| `-i`, `--ignore-case` | Ignore case distinctions |
| `-r`, `-R`, `--recursive` | Read all files under directories recursively |
| `-v`, `--invert-match` | Select non-matching lines |
| `-c`, `--count` | Print count of matching lines |
| `-n`, `--line-number` | Print line numbers with output |
| `-l`, `--files-with-matches` | Print only file names with matches |

## Arguments

- `PATTERN`: Regular expression pattern to match
- `FILE`: One or more files to search. If none, read from stdin.

## Exit Codes

- `0`: Match found
- `1`: No match found
- `2`: Error (invalid option, pattern, or file)

## Examples

```bash
grep foo file.txt           # Find "foo" in file
grep -i bar file.txt        # Case-insensitive search
grep -r foo /path/          # Recursive search
grep -c baz file.txt        # Count matches
grep -n hello file.txt      # Show line numbers
grep -l error logs/         # Files containing "error"
grep -v comment file.txt    # Lines NOT containing "comment"
```
