FROM rust:1.70-slim AS build
WORKDIR /usr/src/gvibu_rust
COPY gvibu-rust/Cargo.toml ./
COPY gvibu-rust/src ./src
RUN cargo build --release

FROM debian:stable-slim
COPY --from=build /usr/src/gvibu_rust/target/release/gvibu_rust /usr/local/bin/vish
RUN chmod +x /usr/local/bin/vish
CMD ["vish", "help"]
HEALTHCHECK --interval=5s --timeout=2s --start-period=1s CMD ["bash", "-lc", "vish gvibu help >/dev/null 2>&1 || true"]
