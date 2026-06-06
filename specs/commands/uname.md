# uname Spec

Overview
- Print system information.

Behavior
- Invocation: `gvibu uname [OPTION]...`
- With no flags, print the kernel name (equivalent to `-s`).
- Options:
  - `-s` or `--kernel-name`: Print kernel name
  - `-n` or `--nodename`: Print network node hostname
  - `-r` or `--kernel-release`: Print kernel release
  - `-m` or `--machine`: Print machine hardware name
  - `-a` or `--all`: Print all information in order

Exit Codes
- 0: Success
- 1: Runtime error (invalid option)

Output Conventions
- stdout: requested system information, space-separated for multiple flags
- stderr: error messages for invalid options

Implementation Notes
- Python: use `os.uname()` and `platform.uname()`.
- Rust: use `std::process::Command` for `uname` or `std::env::consts`.
- Platform-dependent output; parity tester must use wildcards or skip string comparisons.
