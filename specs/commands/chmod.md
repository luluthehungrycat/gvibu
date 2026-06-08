# chmod

Change file mode bits.

## Usage

```
chmod [OPTION]... MODE FILE...
```

## Options

| Flag | Description |
|------|-------------|
| `-R` | Recursively change files and directories |
| `-v` | Verbose; output a diagnostic for every file processed |

## Mode

Mode can be specified as:
- Octal number (e.g., `755`, `644`, `0755`)
- Symbolic notation: `[ugoa][-+=][rwx]+` (e.g., `u+x`, `go-w`, `a+r`, `u=rwx`)

## Exit Codes

| Code | Description |
|------|-------------|
| 0    | Success |
| 1    | Error (invalid option, invalid mode, file error) |

## Examples

```
chmod 755 script.sh
chmod -R 644 docs/
chmod u+x file
chmod go-w file
chmod -v a+r file
```
