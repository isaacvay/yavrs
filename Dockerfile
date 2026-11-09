# ==============================================================================
# Multi-stage production Dockerfile for Yav.rs Website
# Produces an ultra-lean, hardened, non-root container image (<40MB)
# ==============================================================================

# ------------------------------------------------------------------------------
# 1. Builder stage: compile optimized release binary
# ------------------------------------------------------------------------------
FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace dependencies if compiling alongside yav.rs crates
COPY ../yav.rs /yav.rs
COPY . /app

# Compile binary with full release profile (LTO fat, strip, panic=abort)
RUN cargo build --release --bin website

# ------------------------------------------------------------------------------
# 2. Runtime stage: minimal debian runtime with non-root security context
# ------------------------------------------------------------------------------
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install ca-certificates and curl for healthcheck
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create unprivileged application user
RUN useradd -ms /bin/bash -u 10001 appuser

# Copy release binary and static assets
COPY --from=builder /app/target/release/website /app/website
COPY --from=builder /app/public /app/public
COPY --from=builder /app/package.yavml /app/package.yavml

# Ensure permissions
RUN chown -R appuser:appuser /app

USER appuser

# Environment variables
ENV RUST_LOG=info
ENV HOST=0.0.0.0
ENV PORT=3000

EXPOSE 3000

# Docker/K8s liveness health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://127.0.0.1:3000/healthz || exit 1

ENTRYPOINT ["/app/website"]
