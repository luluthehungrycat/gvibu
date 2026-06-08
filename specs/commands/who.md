# who Spec

Overview
- Show who is logged in by reading utmp records.

Behavior
- Invocation: `gvibu who`
- Reads /var/run/utmp, /run/utmp, or /var/log/wtmp (first found).
- Prints entries with type USER_PROCESS or LOGIN_PROCESS.
- Format: `USER     LINE         DATE       TIME       (HOST)`
- Skips empty usernames and "LOGIN" entries.
- If no users found, prints "no users logged in".

Exit Codes
- 0: Success
- 1: Cannot find/read utmp file or invalid option

Output Conventions
- stdout: login entries
- stderr: error messages when applicable
