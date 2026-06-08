# expr

Evaluate expressions.

## Usage

```
expr EXPRESSION
```

## Operators (in precedence order)

| Operator | Description |
|----------|-------------|
| `:` | Regex match |
| `* / %` | Multiplication, division, modulo |
| `+ -` | Addition, subtraction |
| `= != < <= > >=` | Comparison |
| `&` | Logical AND |
| `|` | Logical OR |
| `( )` | Grouping |

## Keywords

| Keyword | Description |
|---------|-------------|
| `length STR` | String length |
| `substr STR POS LEN` | Substring (1-indexed) |
| `index STR CHARS` | Position of first matching character (1-indexed) |
| `match STR REGEX` | Match string against regex |

## Exit Codes

- `0`: Expression is non-null and non-zero
- `1`: Expression is null or zero
- `2`: Invalid expression

## Examples

```bash
expr 1 + 2                    # 3
expr 10 / 3                   # 3
expr 5 = 5                    # 1
expr 5 != 6                   # 1
expr 0 \| 5                   # 5
expr length hello             # 5
expr substr hello 2 3         # ell
expr index hello l            # 3
expr "hello world" : "h.*w"   # hello w
expr \( 1 + 2 \) \* 3         # 9
```
