# syntax=docker/dockerfile:1.6

FROM rust:1.93-bookworm AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libsqlite3-dev \
    libssl-dev \
    pkg-config \
  && rm -rf /var/lib/apt/lists/*

COPY DoWhiz_service/ DoWhiz_service/

RUN cargo build --locked -p scheduler_module --bin rust_service --release \
  --manifest-path DoWhiz_service/Cargo.toml

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libsqlite3-0 \
    libssl3 \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN useradd -r -u 10001 -g nogroup app && \
  mkdir -p \
    /app/.workspace/run_task/state \
    /app/.workspace/run_task/users \
    /app/.workspace/run_task/workspaces && \
  chown -R app:nogroup /app

COPY --from=builder /app/DoWhiz_service/target/release/rust_service /app/rust_service

USER app

ENV RUST_SERVICE_HOST=0.0.0.0
ENV RUST_SERVICE_PORT=9001
ENV WORKSPACE_ROOT=/app/.workspace/run_task/workspaces
ENV SCHEDULER_STATE_PATH=/app/.workspace/run_task/state/tasks.db
ENV PROCESSED_IDS_PATH=/app/.workspace/run_task/state/postmark_processed_ids.txt
ENV USERS_ROOT=/app/.workspace/run_task/users
ENV USERS_DB_PATH=/app/.workspace/run_task/state/users.db
ENV TASK_INDEX_PATH=/app/.workspace/run_task/state/task_index.db
ENV CODEX_DISABLED=1

EXPOSE 9001

ENTRYPOINT ["/app/rust_service"]
CMD ["--host", "0.0.0.0", "--port", "9001"]
