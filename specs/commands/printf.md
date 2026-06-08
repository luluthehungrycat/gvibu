# printf

Format and print data.

## Usage

```
printf FORMAT [ARGUMENT]...
```

## Options

No options. The first argument is the format string, remaining arguments are values
for format specifiers in the format string.

## Format Specifiers

| Specifier | Description |
|-----------|-------------|
| `%s` | String |
| `%d`, `%i` | Signed decimal integer |
| `%u` | Unsigned decimal integer |
| `%o` | Octal integer |
| `%x` | Hexadecimal (lowercase) |
| `%X` | Hexadecimal (uppercase) |
| `%c` | Character |
| `%f`, `%e`, `%E`, `%g`, `%G` | Floating point |
| `%%` | Literal percent sign |

Width can be specified as `%Ns` for minimum field width.

## Escape Sequences

| Sequence | Description |
|----------|-------------|
| `\n` | Newline |
| `\t` | Tab |
| `\r` | Carriage return |
| `\\` | Backslash |
| `\"` | Double quote |
| `\0NNN` | Octal character code |

## Arguments

- `FORMAT`: Format string with specifiers and escapes.
- `ARGUMENT`: Values for format specifiers. If fewer than needed, arguments are recycled (GNU behavior).

## Exit Codes

- `0`: Success (even if format string has issues)
- `1`: Error (missing operand)

## Examples

```bash
printf "hello %s\\n" world     # "hello world"
printf "%d %x\\n" 255 255      # "255 ff"
printf "%s\\n" a b c           # "a" "b" "c" (one per line)
printf "\\0101\\n"              # "A" (octal 101)
printf "%10s\\n" hi             # "        hi"
```
