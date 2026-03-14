# syntax=docker/dockerfile:1.6

FROM rust:1.93-bookworm AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    pkg-config \
  && rm -rf /var/lib/apt/lists/*

COPY DoWhiz_service/ DoWhiz_service/

RUN cargo build --locked -p scheduler_module --bin rust_service --bin inbound_fanout --bin inbound_gateway --bin google-docs --release \
  --manifest-path DoWhiz_service/Cargo.toml

ARG BASE_IMAGE
FROM ${BASE_IMAGE} AS runtime

WORKDIR /app

COPY --from=builder /app/DoWhiz_service/target/release/rust_service /app/rust_service
COPY --from=builder /app/DoWhiz_service/target/release/inbound_fanout /app/inbound_fanout
COPY --from=builder /app/DoWhiz_service/target/release/inbound_gateway /app/inbound_gateway
COPY --from=builder /app/DoWhiz_service/target/release/google-docs /app/bin/google-docs
COPY DoWhiz_service/bin/ /app/bin/

# Copy employee configuration and personas
COPY DoWhiz_service/employee.toml /app/DoWhiz_service/employee.toml
COPY DoWhiz_service/employees/ /app/DoWhiz_service/employees/

# Copy skills directory for Codex
COPY DoWhiz_service/skills/ /app/DoWhiz_service/skills/

RUN chmod +x /app/bin/human_approval_gate /app/bin/human_approval_gate_mcp || true
RUN ln -sf /app/bin/human_approval_gate /usr/local/bin/human_approval_gate
RUN ln -sf /app/bin/human_approval_gate_mcp /usr/local/bin/human_approval_gate_mcp
RUN chown -R app:nogroup /app/DoWhiz_service /app/bin /app/rust_service /app/inbound_fanout /app/inbound_gateway

USER app

ENTRYPOINT ["/app/rust_service"]
