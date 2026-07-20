# Task 11: vish design doc + test migration plan

## Done
- Created `docs/vish-design.md` with:
  - REPL mode (gvibu> prompt) and subcommand mode
  - Built-in commands: help, version, commands, exit/quit
  - Command dispatch flow (Rust: gvibu::commands::lookup; Python: gvibu_ref.main.route)
  - Error output format: `<cmd>: command not found`
  - Explicit list of things NOT supported
  - Test migration plan for all 3 test files
- Updated `tests/test_vish_cli.py` — replaced bash-script tests with Rust vish subcommand tests
- Updated `tests/test_vish_py.py` — replaced argparse-MVP tests with echo/true/false/unknown
- Updated `tests/test_vish_integration_py.py` — added cross-language parity tests
