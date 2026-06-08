# split

Split a file into pieces.

## Usage

```
split [OPTION]... [FILE [PREFIX]]
```

## Options

| Flag | Description |
|------|-------------|
| `-l N` | Split into files with N lines each (default: 1000) |
| `-b N` | Split into files with N bytes each |
| `-d` | Use numeric suffixes (00, 01, ...) instead of alphabetic (aa, ab, ...) |
| `-a N` | Suffix length (default: 2) |

## Size Suffixes for -b

| Suffix | Multiplier |
|--------|------------|
| `b` | 512 |
| `k` | 1024 |
| `m` | 1024^2 |
| `g` | 1024^3 |

## Arguments

- `FILE`: Input file (default: stdin if not specified or `-`)
- `PREFIX`: Output file prefix (default: `x`)

## Exit Codes

- `0`: Success
- `1`: Error

## Examples

```bash
split file.txt                    # Creates xaa, xab, ... (1000 lines each)
split -l 50 file.txt              # 50 lines per file
split -b 1k file.txt              # 1 KB per file
split -d file.txt                 # x00, x01, ... (numeric suffixes)
split -a 3 file.txt out           # outaaa, outaab, ... (3-char suffixes)
cat file.txt | split              # Split stdin
```
