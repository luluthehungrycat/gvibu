# join

Join lines of two files on a common field.

## Usage

```sh
join [-j FIELD] FILE1 FILE2
```

## Options

- `-j FIELD`: Join on field number FIELD (default: 1).

## Description

`join` reads FILE1 and FILE2 and outputs lines where the join field matches.
The join field is whitespace-delimited. Output consists of the join field
followed by the remaining fields from FILE1 and FILE2.

## Exit Codes

| Code | Meaning                  |
| ---- | ------------------------ |
| 0    | Success                  |
| 1    | Error (missing operand, I/O error) |
