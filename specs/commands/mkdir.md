# mkdir Spec

Overview
- Create directories.

Behavior
- Invocation: `gvibu mkdir [OPTION]... DIRECTORY...`
- Options:
  - `-p`: create parent directories as needed; no error if existing
- Creates each specified directory.
- With `-p`, creates parent components as needed and silently skips existing directories.
- Without `-p`, fails if the directory (or any parent) does not exist.

Exit Codes
- 0: All requested directories created successfully
- 1: Any directory creation failed

Output Conventions
- stdout: nothing
- stderr: error messages when applicable
