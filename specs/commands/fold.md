# fold

Wrap each input line to fit in specified width.

## Usage

```
fold [OPTION]...
```

## Description

Reads lines from stdin and wraps them to fit within a specified width.
File arguments are not supported — use shell redirection.

## Options

| Flag      | Description                                      |
|-----------|--------------------------------------------------|
| `-w N`    | Set width to N (default: 80)                     |
| `-s`      | Break at spaces rather than at exact column      |

## Exit Codes

| Code | Meaning                                              |
|------|------------------------------------------------------|
| 0    | Success                                              |
| 1    | Error (invalid option, width, file argument)          |

## Examples

```
echo "hello world this is a long line" | fold -w 10
echo "word1 word2 word3" | fold -s -w 10
```
