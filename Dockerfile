# ABOUTME: Multi-stage Docker build for dravr-riviere-server and dravr-riviere-mcp binaries
# ABOUTME: Lightweight runtime without development dependencies

FROM rust:1-bookworm AS builder
WORKDIR /build
COPY . .
RUN cargo build --release -p dravr-riviere-server -p dravr-riviere-mcp

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --create-home --shell /bin/bash riviere

COPY --from=builder /build/target/release/dravr-riviere-server /usr/local/bin/
COPY --from=builder /build/target/release/dravr-riviere-mcp /usr/local/bin/

USER riviere
WORKDIR /home/riviere

EXPOSE 3000
ENTRYPOINT ["dravr-riviere-server"]
CMD ["--host", "0.0.0.0"]
