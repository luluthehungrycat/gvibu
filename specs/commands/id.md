# id Spec

Overview
- Print user and group identity information.

Behavior
- Invocation: `gvibu id [options]`
- Default: print uid, gid, and supplementary groups with names.
- -u: print effective user ID.
- -g: print effective group ID.
- -G: print all supplementary group IDs.
- -n: print name instead of numeric ID (used with -u, -g, -G).
- -r: print real ID instead of effective (used with -u, -g, -G).
- -n requires -u, -g, or -G.

Exit Codes
- 0: Success
- 1: Invalid option or -n without -u/-g/-G

Output Conventions
- stdout: identity information
- stderr: error messages when applicable

Implementation Notes
- Uses libc getuid/getgid/etc (Rust) or os.getuid/os.getgid/etc (Python).
- Name resolution via /etc/passwd and /etc/group (Rust) or pwd/grp modules (Python).
