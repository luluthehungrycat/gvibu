def route(cmd, *args):
    if cmd == "help":
        # Include explicit marker for OpenCode tests to search for
        return "GVIBU command surface: available subcommands: help, init, status, run"
    if cmd == "init":
        return "GVIBU Python init: placeholder for initializing userland integration"
    if cmd == "status":
        return "GVIBU Python status: MVP surface placeholder"
    if cmd == "run":
        return "GVIBU Python run: placeholder for execution"
    return f"Unknown GVIBU Python subcommand: {cmd}"
