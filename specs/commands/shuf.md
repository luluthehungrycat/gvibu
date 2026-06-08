# shuf

## Usage
```
shuf [FILE]
```

## Description
Randomly permute the lines from stdin or a file.

## Options
None.

## Exit Codes
- `0`: Success
- `1`: File not found or error

## Examples
```bash
echo -e "a\nb\nc" | shuf
shuf /etc/hosts
```
