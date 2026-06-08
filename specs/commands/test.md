# test / [

Evaluate expression; return 0 (true) or 1 (false).

## Usage

```
test EXPR
[ EXPR ]
```

## Unary Operators

| Operator | Description |
|----------|-------------|
| `-n` STRING | String is not empty |
| `-z` STRING | String is empty |
| `-f` FILE | File exists and is a regular file |
| `-d` FILE | File exists and is a directory |
| `-e` FILE | File exists |
| `-x` FILE | File exists and is executable |
| `-w` FILE | File exists and is writable |
| `-r` FILE | File exists and is readable |
| `-s` FILE | File exists and has size > 0 |

## Binary Operators

| Operator | Description |
|----------|-------------|
| STRING = STRING | Strings are equal |
| STRING != STRING | Strings are not equal |
| INT -eq INT | Integers are equal |
| INT -ne INT | Integers are not equal |
| INT -lt INT | Left is less than right |
| INT -le INT | Left is less than or equal to right |
| INT -gt INT | Left is greater than right |
| INT -ge INT | Left is greater than or equal to right |

## Other Operators

| Operator | Description |
|----------|-------------|
| ! EXPR | Logical NOT |

## Exit Codes

| Code | Description |
|------|-------------|
| 0    | Expression is true |
| 1    | Expression is false or empty |
| 2    | Error (invalid expression) |

## Examples

```
test "hello"
test -n "hello"
test -z ""
test -f /dev/null
test -d /
test 5 -eq 5
test "a" = "a"
test ! ""
[ "hello" ]
[ -f /dev/null ]
[ 5 -gt 3 ]
```
