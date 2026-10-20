FROM rust:stable-slim AS builder
WORKDIR /build
# `vish` dispatches through the main gvibu Rust library.
COPY rust ./rust
COPY vish ./vish
RUN cargo build --release --manifest-path vish/Cargo.toml

FROM debian:stable-slim
COPY --from=builder /build/vish/target/release/vish /usr/local/bin/vish
RUN chmod +x /usr/local/bin/vish
CMD ["vish"]
HEALTHCHECK --interval=5s --timeout=2s --start-period=1s CMD ["vish", "help"]
