FROM rust:1.75-slim AS builder
WORKDIR /build
COPY rust/Cargo.toml ./
COPY rust/src/ ./src/
RUN cargo build --release

FROM debian:stable-slim
COPY --from=builder /build/target/release/gvibu /usr/local/bin/gvibu
RUN set -e && \
    for cmd in true false echo pwd basename dirname cat wc head yes printenv sleep touch seq which uname env whoami link unlink tee mkdir rmdir hostname logname readlink realpath uniq uptime id who kill cut tr mv rm ln chmod chown sort grep ls cp printf date expr split tail tac fold comm join nl shuf sum du df test '['; do \
        ln -s /usr/local/bin/gvibu "/usr/local/bin/$cmd"; \
    done
CMD ["true"]
