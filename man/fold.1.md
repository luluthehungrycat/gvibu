# fold(1) — gvibu

## NAME

**fold** — Reads lines from stdin and wraps them to fit within a specified width

## SYNOPSIS

`gvibu fold [OPTIONS] [ARGS...]`

## DESCRIPTION

Reads lines from stdin and wraps them to fit within a specified width. File arguments are not supported — use shell redirection.

## EXIT STATUS

| Code | Meaning                                              |
|------|------------------------------------------------------|
| 0    | Success                                              |
| 1    | Error (invalid option, width, file argument)          |

## SEE ALSO

gvibu(1)
