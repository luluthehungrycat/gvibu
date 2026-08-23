# rev

Reverse the characters in each input line.

## Usage

```
rev [FILE]...
```

With no files, read standard input. With files, process them in order. Preserve
line endings and preserve a final line without a newline.

## Options

| Flag | Description |
|------|-------------|
| `--` | End option parsing before file names |

Unknown options return status `1` and write a diagnostic to stderr.

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | All input was processed |
| 1 | Invalid option, input, or output error |
