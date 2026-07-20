# nl(1) — gvibu

## NAME

**nl** — `nl` numbers non-empty lines sequentially

## SYNOPSIS

`gvibu nl [OPTIONS] [ARGS...]`

## DESCRIPTION

`nl` numbers non-empty lines sequentially. Empty lines are output unnumbered. If no FILE is given, or FILE is `-`, read standard input.

## OPTIONS

- - `-v START`: Starting line number (default: 1).

## EXIT STATUS

| Code | Meaning                  |
| ---- | ------------------------ |
| 0    | Success                  |
| 1    | Error (I/O error, invalid option) |

## SEE ALSO

gvibu(1)
