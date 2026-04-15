#!/usr/bin/env python3
import sys
import argparse


def main():
    parser = argparse.ArgumentParser(
        prog="vish", description="GVIBU Python shell (MVP)"
    )
    subparsers = parser.add_subparsers(dest="cmd", help="sub-command help")
    parser_help = subparsers.add_parser("help", help="show help")
    parser_init = subparsers.add_parser("init", help="initialize placeholder")
    parser_status = subparsers.add_parser("status", help="status placeholder")
    parser_gvibu = subparsers.add_parser("gvibu", help="GVIBU surface (placeholder)")
    gvibu_subparsers = parser_gvibu.add_subparsers(dest="gvibu_cmd")
    gvibu_subparsers.add_parser("help", help="gvibu help placeholder")
    gvibu_subparsers.add_parser("init", help="gvibu init placeholder")
    gvibu_subparsers.add_parser("status", help="gvibu status placeholder")
    gvibu_subparsers.add_parser("run", help="gvibu run placeholder")

    args = parser.parse_args()
    if args.cmd == "help" or args.cmd is None:
        print("GVIBU Python shell (vish) - routes commands to GVIBU surface")
        sys.exit(0)
    if args.cmd == "init":
        print("GVIBU init: placeholder for initializing userland integration")
        sys.exit(0)
    if args.cmd == "status":
        print("GVIBU status: MVP skeleton, no real state yet")
        sys.exit(0)
    if args.cmd == "gvibu":
        subcmd = args.gvibu_cmd or "help"
        try:
            # Try local import first
            try:
                from gvibu import gvibu as _gv

                output = _gv.route(
                    subcmd,
                )
            except Exception:
                # Fallback: load gvibu.py directly from filesystem to avoid packaging issues
                import importlib.util, os

                base = os.path.dirname(__file__)
                gvibu_path = os.path.join(base, "gvibu.py")
                spec = importlib.util.spec_from_file_location("gvibu_back", gvibu_path)
                gvibu_back = importlib.util.module_from_spec(spec)
                spec.loader.exec_module(gvibu_back)  # type: ignore
                output = gvibu_back.route(subcmd)
            print(output)
            sys.exit(0)
        except Exception as e:
            print("GVIBU Python surface error:", e)
            sys.exit(1)
    print("Unknown command: {}".format(args.cmd))
    sys.exit(1)


if __name__ == "__main__":
    main()
