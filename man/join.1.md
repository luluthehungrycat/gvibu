# join(1) — gvibu

## NAME

**join** — `join` reads FILE1 and FILE2 and outputs lines where the join field matches

## SYNOPSIS

`gvibu join [OPTIONS] [ARGS...]`

## DESCRIPTION

`join` reads FILE1 and FILE2 and outputs lines where the join field matches. The join field is whitespace-delimited. Output consists of the join field followed by the remaining fields from FILE1 and FILE2.

## OPTIONS

- - `-j FIELD`: Join on field number FIELD (default: 1).

## EXIT STATUS

| Code | Meaning                  |
| ---- | ------------------------ |
| 0    | Success                  |
| 1    | Error (missing operand, I/O error) |

## SEE ALSO

gvibu(1)
