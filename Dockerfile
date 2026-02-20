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

RUN cargo build --locked -p scheduler_module --bin rust_service --bin inbound_fanout --bin inbound_gateway --bin google-docs --release \
  --manifest-path DoWhiz_service/Cargo.toml

FROM debian:bookworm-slim AS runtime

# Work around occasional Debian keyring signature issues in some build environments.
RUN mkdir -p /tmp/apt-cache/partial \
  && apt-get update -o Acquire::AllowInsecureRepositories=true \
    -o Acquire::AllowDowngradeToInsecureRepositories=true \
  && apt-get install -y --no-install-recommends --allow-unauthenticated \
    -o Dir::Cache::Archives=/tmp/apt-cache \
    debian-archive-keyring \
    ca-certificates \
  && rm -rf /var/lib/apt/lists/* /tmp/apt-cache

RUN if [ -f /etc/apt/sources.list ]; then \
      sed -i 's|http://deb.debian.org|https://deb.debian.org|g' /etc/apt/sources.list; \
    fi \
  && if [ -f /etc/apt/sources.list.d/debian.sources ]; then \
      sed -i 's|http://deb.debian.org|https://deb.debian.org|g' /etc/apt/sources.list.d/debian.sources; \
    fi \
  && mkdir -p /tmp/apt-cache/partial \
  && apt-get update && apt-get install -y --no-install-recommends \
    -o Dir::Cache::Archives=/tmp/apt-cache \
    libsqlite3-0 \
    libssl3 \
    python3 \
    python3-pip \
    python3-venv \
    python-is-python3 \
    curl \
    git \
    gh \
    pandoc \
    libreoffice \
    poppler-utils \
    qpdf \
    tesseract-ocr \
    libmagic1 \
    fonts-dejavu \
  && rm -rf /var/lib/apt/lists/* /tmp/apt-cache

# Install Node.js 20.x (LTS)
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
  && apt-get install -y --no-install-recommends nodejs \
  && rm -rf /var/lib/apt/lists/*

# Install global npm packages (playwright-cli, Codex CLI, Claude CLI, doc/pptx tooling, pnpm)
RUN npm install -g @playwright/cli@latest @openai/codex@latest @anthropic-ai/claude-code@latest docx pptxgenjs pnpm

# Install Playwright browsers (Chromium only to save space)
ENV PLAYWRIGHT_BROWSERS_PATH=/app/.cache/ms-playwright
RUN mkdir -p /app/.cache/ms-playwright \
  && npx playwright install --with-deps chromium

# Install cloudflared for browser-use tunnels (remote-browser skill)
RUN arch="$(dpkg --print-architecture)" \
  && case "$arch" in \
       amd64) pkg="cloudflared-linux-amd64.deb" ;; \
       arm64) pkg="cloudflared-linux-arm64.deb" ;; \
       *) echo "Unsupported architecture: $arch" >&2; exit 1 ;; \
     esac \
  && curl -fsSL "https://github.com/cloudflare/cloudflared/releases/latest/download/${pkg}" -o /tmp/cloudflared.deb \
  && dpkg -i /tmp/cloudflared.deb \
  && rm /tmp/cloudflared.deb

# Install Python packages required by skills
RUN python3 -m pip install --break-system-packages --no-cache-dir \
    browser-use \
    playwright \
    python-docx \
    pdf2image \
    reportlab \
    pdfplumber \
    pypdf \
    pytesseract \
    openpyxl \
    pandas \
    matplotlib \
    pillow \
    imageio \
    numpy \
    "markitdown[pptx]"

# Ensure Playwright's Chromium binary satisfies the chrome channel lookup.
RUN chromium_path="$(ls -d /app/.cache/ms-playwright/chromium-*/chrome-linux/chrome | head -n1)" \
  && mkdir -p /opt/google/chrome \
  && ln -s "$chromium_path" /opt/google/chrome/chrome

WORKDIR /app

RUN useradd -r -u 10001 -g nogroup app && \
  mkdir -p \
    /app/.workspace/run_task/state \
    /app/.workspace/run_task/users \
    /app/.workspace/run_task/workspaces && \
  chown -R app:nogroup /app

COPY --from=builder /app/DoWhiz_service/target/release/rust_service /app/rust_service
COPY --from=builder /app/DoWhiz_service/target/release/inbound_fanout /app/inbound_fanout
COPY --from=builder /app/DoWhiz_service/target/release/inbound_gateway /app/inbound_gateway
COPY --from=builder /app/DoWhiz_service/target/release/google-docs /app/bin/google-docs

# Copy employee configuration and personas
COPY DoWhiz_service/employee.toml /app/DoWhiz_service/employee.toml
COPY DoWhiz_service/employees/ /app/DoWhiz_service/employees/

# Copy skills directory for Codex
COPY DoWhiz_service/skills/ /app/DoWhiz_service/skills/

RUN chown -R app:nogroup /app/DoWhiz_service /app/bin

USER app

ENV RUST_SERVICE_HOST=0.0.0.0
ENV RUST_SERVICE_PORT=9001
ENV HOME=/app
ENV WORKSPACE_ROOT=/app/.workspace/run_task/workspaces
ENV SCHEDULER_STATE_PATH=/app/.workspace/run_task/state/tasks.db
ENV PROCESSED_IDS_PATH=/app/.workspace/run_task/state/postmark_processed_ids.txt
ENV USERS_ROOT=/app/.workspace/run_task/users
ENV USERS_DB_PATH=/app/.workspace/run_task/state/users.db
ENV TASK_INDEX_PATH=/app/.workspace/run_task/state/task_index.db
ENV PLAYWRIGHT_BROWSERS_PATH=/app/.cache/ms-playwright
ENV DOWHIZ_BIN_DIR=/app/bin
ENV PATH=/app/bin:$PATH

EXPOSE 9001
EXPOSE 9100

ENTRYPOINT ["/app/rust_service"]
