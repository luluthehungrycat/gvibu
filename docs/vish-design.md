# vish — GVIBU Shell Design

## Overview

vish (GVIBU Interactive SHell) is a thin REPL that dispatches to the 58 gvibu coreutils commands. Implemented in both Rust and Python with identical behavior for test parity.

## User Interface

### Modes
- **REPL mode** (default, no args): interactive prompt `gvibu> `, reads commands via `readline()`/`input()`, dispatches until `exit` or Ctrl+D
- **Subcommand mode** (`gvibu <cmd> [args]`): single-command dispatch, prints output to stdout, exits with the command's exit code

### Prompt
- Rust: `gvibu> ` (no trailing space in prompt, space is from readline)
- Python: `gvibu> ` (same)

### Exit
- Command: `exit` or `quit` — exits with code 0
- Ctrl+D (EOF) — exits with code 0
- Ctrl+C (SIGINT) — prints `^C`, stays in REPL (does not exit)

## Built-in Commands (handled before command lookup)

| Command | Rust Output | Python Output |
|---------|-------------|---------------|
| `help` | Prints usage text with available built-ins | Same |
| `version` | Prints `gvibu v<CARGO_PKG_VERSION>` | Prints `gvibu v<from gvibu_ref.__version__>` |
| `commands` | Lists all 58 command names, one per line | Same |
| `exit` / `quit` | Exits with code 0 | Same |

### Help text
```
gvibu — universal coreutils
Built-in commands: help, version, commands, exit, quit
Type a command name to run it, or one of the built-ins above.
```

### Error output format
- Unknown command: `<command>: command not found` to stderr, exit 1
- Command failure: `<command>: <error message>` to stderr, exit from command's return code
- Empty input: ignored (prints new prompt)

## Command Dispatch

### Dispatch flow
1. Read line from stdin
2. Trim whitespace; skip empty lines
3. Check for built-in commands (exit, quit, help, version, commands)
4. Parse into command name + args (space-split, no shell metacharacter handling)
5. Call `gvibu::commands::lookup(name)` (Rust) or `gvibu_ref.main.route(name, args)` (Python)
6. Print output; propagate exit code

### Rust implementation
- Binary: `vish/src/main.rs`, crate dependency on `gvibu`
- Uses `gvibu::commands::lookup()` (now `HashMap`-based, O(1))
- `std::io::stdin().read_line()` loop
- No shebang line handling

### Python implementation
- Script: `gvibu-python/vish.py`, uses `python-ref/gvibu_ref/main.py` for dispatch
- `input()` loop with `sys.stdin.isatty()` check for prompt display
- Non-interactive mode: read lines from stdin until EOF

## Things NOT supported
- Pipes, redirects
- Environment variable expansion (`$VAR`)
- Command history (beyond what readline provides)
- Tab completion
- Job control
- Scripting / shebang execution
- POSIX shell compatibility

## Test Migration Plan

### Existing tests that need updating

#### `tests/test_vish_cli.py` (tests bash script `./gvibu-linux/vish`)
Currently tests the bash `gvibu-linux/vish` script with subcommand mode. The new vish will replace the bash script. Tests update:
- `test_help()`: Change expected output from `"GVIBU minimal shell"` to the new help text
- `test_gvibu_help()`: Remove — `gvibu help` is now just `help` (help command)
- `test_init()`: Remove — `init` is not a built-in, will dispatch to gvibu `init` command or fail
- `test_status()`: Remove — same as init
- `test_unknown()`: Check for `not found` instead of `Unknown command`

#### `tests/test_vish_py.py` (tests Python vish.py)
Currently tests old argparse MVP. Tests update:
- `test_help_py()`: Change expected to new help text
- `test_init_py()` / `test_status_py()`: Remove or update to test `true`/`false`/`echo`

#### `tests/test_vish_integration_py.py` (cross-language parity)
- `test_integration_help()`: Update expected output
- `test_integration_gvibu_init()`: Replace with `echo hello` parity test

### New tests needed
- Rust vish REPL: echo, true/false exit codes, unknown command, exit, help output, version, commands listing
- Python vish REPL: same parity set
- Cross-language: echo, true, false produce same output in both

## Output format guarantee

All commands produce output via the existing command's `run(w: &mut dyn Write, args: &[String]) -> i32` interface. The REPL captures the `Write` buffer and prints it to stdout. No output manipulation by the REPL.
