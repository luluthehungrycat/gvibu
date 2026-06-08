# cp

Copy files and directories.

## Usage

```
cp [OPTION]... SOURCE... DEST
```

## Options

| Flag | Description |
|------|-------------|
| `-i`, `--interactive` | Prompt before overwrite |
| `-f`, `--force` | Remove destination file first if it exists |
| `-v`, `--verbose` | Print copied paths |
| `-n`, `--no-clobber` | Do not overwrite existing files |
| `-u`, `--update` | Copy only when source is newer than destination |
| `-r`, `-R`, `--recursive` | Copy directories recursively |

## Arguments

- `SOURCE`: One or more source files or directories.
- `DEST`: Destination path. If multiple sources are given, DEST must be an existing directory.

## Exit Codes

- `0`: Success
- `1`: Error (invalid option, missing operand, copy failure)

## Examples

```bash
cp file.txt /tmp/                    # Copy file to directory
cp -r dir/ /tmp/                     # Copy directory recursively
cp -i file.txt dest.txt              # Prompt before overwrite
cp -v file.txt /tmp/                 # Verbose copy
cp -n file.txt dest.txt              # Don't overwrite existing
cp -u source.txt dest.txt            # Only copy if source is newer
cp -f source.txt dest.txt            # Force overwrite
```
