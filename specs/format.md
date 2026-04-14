# Command Specification Format

## Overview

Each command in gvibu has a human-readable specification in `specs/commands/` and machine-readable tests in `shared-tests/cases/`.

## Human-Readable Spec Format

```markdown
# <command-name>

## Synopsis
<command-name> [options] [arguments...]

## Description
<description of what the command does>

## Options
- `-<flag>`: <description>

## Operands
- <operand>: <description>

## Exit Codes
- `0`: <success case>
- `1`: <runtime error>
- `2`: <usage error>

## Examples
    $ <command-name> <args>
    <output>

    $ <command-name> <args>
    <output>
```

## Machine-Readable Test Format

Tests are stored as JSON files in `shared-tests/cases/<command>.json`:

```json
{
  "command": "<command-name>",
  "description": "<command description>",
  "cases": [
    {
      "name": "<test name>",
      "args": ["<arg1>", "<arg2>"],
      "stdin": "<optional stdin input>",
      "stdout": "<expected stdout>",
      "stderr": "<expected stderr>",
      "exit_code": <0|1|2>
    }
  ]
}
```

## Test Case Fields

| Field | Required | Description |
|-------|----------|-------------|
| `name` | Yes | Descriptive test name |
| `args` | Yes | Command-line arguments (array) |
| `stdin` | No | Input to provide on stdin |
| `stdout` | Yes | Expected stdout content |
| `stderr` | Yes | Expected stderr content |
| `exit_code` | Yes | Expected exit code (0, 1, or 2) |

## Writing Specs

1. Start with synopsis - minimal interface
2. Describe behavior - what does it do?
3. Document options - only what you implement
4. List operands - what arguments are expected
5. Define exit codes - when does it succeed/fail?
6. Add examples - show typical usage

## Spec Review Checklist

- [ ] Synopsis matches actual implementation
- [ ] All options documented
- [ ] All exit codes covered
- [ ] Examples are accurate
- [ ] Edge cases considered
- [ ] No overclaiming (don't promise GNU features)
