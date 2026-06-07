# syntax=docker/dockerfile:1

# ------------------------------------------------------------------------------
# Builder stage
# ------------------------------------------------------------------------------
FROM rust:1-slim AS builder

WORKDIR /usr/src/proximityd

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libbluetooth-dev \
    libdbus-1-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first for layer caching
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build release binary
RUN cargo build --release --locked

# ------------------------------------------------------------------------------
# Runtime stage
# ------------------------------------------------------------------------------
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies only
RUN apt-get update && apt-get install -y --no-install-recommends \
    libbluetooth3 \
    dbus \
    procps \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean \
    && rm -rf /var/cache/apt/archives /var/lib/dpkg /var/lib/apt

# Create non-root proximityd user with a home directory for config storage
RUN groupadd --gid 1000 proximityd \
    && useradd --uid 1000 --gid proximityd --shell /bin/false --create-home proximityd \
    && mkdir -p /home/proximityd/.config/proximityd \
    && chown -R proximityd:proximityd /home/proximityd/.config

# Copy binary from builder
COPY --from=builder /usr/src/proximityd/target/release/proximityd /usr/local/bin/proximityd
RUN chmod +x /usr/local/bin/proximityd

# Switch to non-root user
USER proximityd

# Health check: verify proximityd process is running
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD pgrep -x proximityd > /dev/null || exit 1

# Default to daemon mode
CMD ["proximityd", "--daemon"]
