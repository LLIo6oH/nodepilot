# syntax=docker/dockerfile:1

FROM rust:1.86-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates docker.io \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /usr/local/bin/nodepilot-backend

ENV DATABASE_URL=sqlite:///data/nodepilot.sqlite
ENV WORKSPACES_DIR=/workspaces
ENV RUNTIME_MODE=docker
ENV RUNTIME_IMAGE=nodepilot-runtime:latest
ENV RUNTIME_WORKSPACES_VOLUME=nodepilot-workspaces

VOLUME ["/data", "/workspaces"]

EXPOSE 8080

CMD ["nodepilot-backend"]
