"""GVIBU backend module.

Exposes a simple route(cmd) interface that can be wired by a CLI surface.
cmd is expected to be a list of strings representing command tokens.
"""

from typing import Tuple, Dict, Any, List


def _handle_main(cmd: List[str]) -> Tuple[str, Dict[str, Any]]:
    if not cmd or cmd[0] == "help":
        return (
            "GVIBU CLI: available commands: init, status, run; use gvibu.py help for usage.",
            {"usage": ["init", "status", "run"]},
        )
    action = cmd[0]
    if action == "init":
        return ("GVIBU initialized.", {"state": "initialized"})
    if action == "status":
        return ("GVIBU status: OK", {"status": "OK"})
    if action == "run":
        # Minimal demonstration of a run action
        payload = {"ran": True, "result": "success"}
        return ("GVIBU run completed.", payload)
    return (f"Unknown command: {action}", {"error": True, "command": action})


def route(cmd) -> Dict[str, Any]:
    """Public entrypoint for GVIBU CLI integration.

    Accepts a list-like command representation (e.g. sys.argv[1:]). Returns a
    dict with 'message' and optional 'payload' used by the caller.
    """
    if isinstance(cmd, str):
        tokens = cmd.split()
    else:
        try:
            tokens = list(cmd)  # type: ignore
        except TypeError:
            tokens = []

    message, payload = _handle_main(tokens)
    return {"message": message, "payload": payload}
