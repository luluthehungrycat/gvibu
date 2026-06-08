# nl

Number lines of files.

## Usage

```sh
nl [-v START] [FILE...]
```

## Options

- `-v START`: Starting line number (default: 1).

## Description

`nl` numbers non-empty lines sequentially. Empty lines are output unnumbered.
If no FILE is given, or FILE is `-`, read standard input.

## Exit Codes

| Code | Meaning                  |
| ---- | ------------------------ |
| 0    | Success                  |
| 1    | Error (I/O error, invalid option) |
