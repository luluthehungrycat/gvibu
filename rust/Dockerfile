FROM rust:1.75-slim AS builder

WORKDIR /app

COPY Cargo.toml ./
COPY src/ ./src/

RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /app/target/release/gvibu /usr/local/bin/gvibu

CMD ["gvibu"]
