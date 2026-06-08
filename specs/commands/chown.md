# chown

Change file owner and group.

## Usage

```
chown [OPTION]... [OWNER][:][GROUP] FILE...
```

## Options

| Flag | Description |
|------|-------------|
| `-R` | Recursively change ownership |
| `-v` | Verbose; output a diagnostic for every file processed |

## Owner Specification

The owner can be specified as:
- Username (e.g., `root`)
- Numeric UID (e.g., `0`)
- `USER:GROUP` (e.g., `root:root`)
- `USER:` (change only user)
- `:GROUP` (change only group)

## Exit Codes

| Code | Description |
|------|-------------|
| 0    | Success |
| 1    | Error (invalid option, invalid owner, file error) |

## Examples

```
chown root file
chown root:staff file
chown :staff file
chown -R root:root dir/
chown -v 1000:1000 file
```
