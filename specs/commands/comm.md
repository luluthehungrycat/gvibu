# comm

Compare two sorted files line by line.

## Usage

```
comm [OPTION]... FILE1 FILE2
```

## Description

Compare sorted files FILE1 and FILE2 line by line.
With no options, produce three-column output: lines unique to FILE1,
lines unique to FILE2, and lines common to both files.

If FILE is `-`, read from stdin.

## Options

| Flag   | Description                           |
|--------|---------------------------------------|
| `-1`   | Suppress column 1 (lines unique to FILE1) |
| `-2`   | Suppress column 2 (lines unique to FILE2) |
| `-3`   | Suppress column 3 (lines common to both)  |

## Exit Codes

| Code | Meaning             |
|------|---------------------|
| 0    | Success             |
| 1    | Error               |

## Examples

```
comm file1 file2
comm -12 file1 file2  # lines common to both
comm -3 file1 file2   # lines unique to each
```
