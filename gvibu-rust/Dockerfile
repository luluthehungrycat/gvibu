FROM rust:1.70-slim AS builder
WORKDIR /build
# Copy source trees required to build the vish binary.
# vish/Cargo.toml depends on the gvibu crate (../rust).
COPY rust ./rust
COPY vish ./vish
# Build the vish release binary
RUN cargo build --release --manifest-path vish/Cargo.toml

FROM debian:stable-slim
COPY --from=builder /build/vish/target/release/vish /usr/local/bin/vish
RUN chmod +x /usr/local/bin/vish
CMD ["vish"]
HEALTHCHECK --interval=5s --timeout=2s --start-period=1s CMD ["bash", "-lc", "vish help >/dev/null 2>&1 || true"]
