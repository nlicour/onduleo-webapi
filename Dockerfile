# --- Build ---
FROM rust AS builder
WORKDIR /build

# Cache des dépendances : on ne re-télécharge que si Cargo.* change
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release && rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release

# --- Runtime ---
FROM debian:slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -r app && useradd -r -g app app
COPY --from=builder /build/target/release/api /usr/local/bin/api

USER app
EXPOSE 8080
ENV RUST_LOG=api=info,tower_http=info

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s \
    CMD curl -fsS http://localhost:8080/healthz || exit 1

ENTRYPOINT ["/usr/local/bin/api"]
