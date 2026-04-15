GVIBU Rust tooling scaffolding
This is a placeholder Rust implementation of GVIBU and the vish shell surface.

## Usage and tests
- Build: cargo build --release
- Run: ./target/release/gvibu_rust gvibu help
- Version signaling: set GVIBU_RUST_VERSION at runtime to surface version
- Tests: cargo test
- Docker usage: see gvibu-rust/Dockerfile healthcheck; you can run with -e GVIBU_RUST_VERSION
