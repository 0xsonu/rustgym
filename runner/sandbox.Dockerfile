# Sandbox image for executing user code in RustGym
# This image is used by the runner service to spawn isolated containers
# for compiling and testing user-submitted Rust code.

FROM rust:1.78-slim

# Install minimal build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Pre-cache common crates by building a dummy project
# This speeds up user code compilation significantly
WORKDIR /tmp/warmup

RUN USER=root cargo init --name warmup . \
    && echo '[dev-dependencies]' >> Cargo.toml \
    && cargo build --release 2>/dev/null || true \
    && cargo test --no-run 2>/dev/null || true \
    && rm -rf /tmp/warmup

# Set the working directory for user code
WORKDIR /workspace

# No CMD — the runner service provides the command at container creation time
