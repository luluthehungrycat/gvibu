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
  - `-v`: Print kernel version
  - `-m` or `--machine`: Print machine hardware name
  - `-o`: Print operating system
  - `-a` or `--all`: Print all information in order (`-s -n -r -v -m -o`)

Exit Codes
- 0: Success
- 1: Runtime error (invalid option)

Output Conventions
- stdout: requested system information, space-separated for multiple flags
- stderr: error messages for invalid options

Implementation Notes
- Python: use `os.uname()` and `platform.uname()`.
- Rust: read Linux kernel fields from `/proc/sys/kernel` and use the POSIX `uname` system call elsewhere.
- Platform-dependent output; parity tester must use wildcards or skip string comparisons.
