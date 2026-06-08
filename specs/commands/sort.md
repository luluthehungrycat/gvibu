# sort

Sort lines of text files.

## Usage

```
sort [OPTION]... [FILE]...
```

## Options

| Flag | Description |
|------|-------------|
| `-r` | Reverse the sort order |
| `-n` | Compare according to numeric value |
| `-u` | Output only the first of consecutive equal lines |
| `-f` | Fold case; ignore case when comparing |
| `-k` | Sort via key: `POS1[,POS2]` where POS is `F[.C][opts]`, F=field, C=char offset. Per-key modifiers: `n` (numeric), `r` (reverse). May be specified multiple times; later keys break ties. |

When no FILE is given, read from standard input.

## Exit Codes

| Code | Description |
|------|-------------|
| 0    | Success |
| 1    | Error (invalid option, file read error) |

## Examples

```
sort file.txt
sort -r file.txt
sort -n numbers.txt
sort -u file.txt
sort -rn file.txt
echo -e "c\na\nb" | sort
echo -e "b 2\na 1\nb 1" | sort -k2,2n
echo -e "a 3\na 1\na 2" | sort -k1,1 -k2,2n
```
