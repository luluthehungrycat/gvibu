# tac

Concatenate and write files in reverse.

## Usage

```
tac [FILE]...
```

## Arguments

- `FILE`: One or more files to read in reverse. If `-` or no files, read from stdin.

## Exit Codes

- `0`: Success
- `1`: Error (invalid option, file read error)

## Examples

```bash
tac file.txt                     # Reverse file content
echo -e "a\\nb\\nc" | tac         # Output "c\\nb\\na"
```
