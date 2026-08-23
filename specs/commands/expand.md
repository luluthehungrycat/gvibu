# expand

Convert tab characters to spaces.

## Usage

```
expand [OPTION]... [FILE]...
```

With no files, read standard input. With files, process them in order.
Columns reset after each newline. The default tab stop is every eight columns.

## Options

| Flag | Description |
|------|-------------|
| `-t LIST` | Use comma-separated positive tab stops, such as `4,8,12` |
| `-tLIST` | Attached form of `-t` |
| `-i` | Convert only tabs before the first non-tab character on each line |
| `--` | End option parsing before file names |

Custom tab stops must be strictly increasing. After the final explicit stop,
the interval between the final two stops repeats; for one stop, that stop is
the repeated interval.

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | All input was processed |
| 1 | Invalid option, tab stop, input, or output error |
