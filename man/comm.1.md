# comm(1) — gvibu

## NAME

**comm** — Compare sorted files FILE1 and FILE2 line by line

## SYNOPSIS

`gvibu comm [OPTIONS] [ARGS...]`

## DESCRIPTION

Compare sorted files FILE1 and FILE2 line by line. With no options, produce three-column output: lines unique to FILE1, lines unique to FILE2, and lines common to both files. If FILE is `-`, read from stdin.

## EXIT STATUS

| Code | Meaning             |
|------|---------------------|
| 0    | Success             |
| 1    | Error               |

## SEE ALSO

gvibu(1)
