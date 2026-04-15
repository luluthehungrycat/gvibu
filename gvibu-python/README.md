GVIBU Python tooling scaffolding
This is a placeholder Python-based GVIBU tool including a minimal vish CLI surface.

Commands:
- vish.py help
- vish.py init
- vish.py status
- vish.py gvibu <subcommand>

Note: This is a scaffold and will be expanded in later iterations.

## Usage and tests (plan for next iterations)
- Run tests: pytest -q
- Run CLI locally: python3 vish.py gvibu help
- Docker usage: docker run -it --rm gvibu-ai-lab-gvibu_python:latest vish gvibu help
- Version signaling: the Rust surface exposes runtime version via GVIBU_RUST_VERSION; the Python surface can adopt a similar approach in future iterations.
