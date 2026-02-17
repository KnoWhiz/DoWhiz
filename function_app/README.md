# DoWhiz Azure Function App

This folder contains the Azure Functions custom handler wrapper for the Rust worker service
(`DoWhiz_service` -> `scheduler_module` -> `rust_service`). The worker does not expose `/postmark/inbound`;
inbound webhooks must go through `inbound_gateway` (see `DoWhiz_service/README.md`).

## Layout

- `host.json`: custom handler config and HTTP routing
- `HttpEntry/function.json`: catch-all HTTP trigger for proxying requests
- `rust_service`: compiled Linux worker binary (created by the build script)
- `inbound_gateway`: optional gateway binary if you swap the handler (build manually)
- `local.settings.example.json`: local-only config template
- `scripts/`: build + local E2E helpers

## Build the Linux binary

From the repo root:

```bash
./function_app/scripts/build_binary.sh
```

Notes:
- Default target is `x86_64-unknown-linux-gnu` (Azure Functions Linux).
- Override with `TARGET=...` or `PROFILE=...` if needed.
- On macOS, use `cross` or a Linux container if native cross-compile fails.

To build the inbound gateway instead:
```bash
cargo build -p scheduler_module --bin inbound_gateway --release --target x86_64-unknown-linux-gnu
cp DoWhiz_service/target/x86_64-unknown-linux-gnu/release/inbound_gateway function_app/inbound_gateway
```
Then update `function_app/host.json` to set `defaultExecutablePath` to `inbound_gateway`.

## Run locally

Prereqs:
- Azure Functions Core Tools (`func`)
- Azurite (if you keep `AzureWebJobsStorage=UseDevelopmentStorage=true`)

Steps:

```bash
cp function_app/local.settings.example.json function_app/local.settings.json
cd function_app
func host start --port 7071
```

Test endpoints:

```bash
curl -fsS http://localhost:7071/health
```

`/postmark/inbound` is only available if you swap the handler to `inbound_gateway` (see above).

## VM Deployment Workflow (Non-Azure Functions)

If you are deploying the Rust service directly on a VM (Nginx + systemd), follow the VM deployment workflow in `DoWhiz_service/README.md`. This repo supports both approaches, but the Azure Functions wrapper is not required for VM deployments.

## Local E2E script

This builds the worker binary, starts Azurite (if needed), runs `func host start`,
then checks `/health`. It will only pass the `/postmark/inbound` check if you
swap the handler to `inbound_gateway`.

```bash
./function_app/scripts/e2e_local.sh
```

Logs land in `function_app/.e2e/`.

## Deploy to Azure

1. Create a **Linux** Function App. For custom handlers, select **.NET Core**
   as the runtime stack.
2. Set app settings:
   - `FUNCTIONS_WORKER_RUNTIME=custom`
   - `AzureWebJobsStorage=<storage connection string>`
   - Use writable paths for DoWhiz state (example below).
3. Build the Linux binary and copy it to `function_app/rust_service`.
   - If you want the Function App to serve inbound webhooks, swap the handler to `inbound_gateway` and provide a `gateway.toml`.
4. Deploy from this folder:

```bash
cd function_app
func azure functionapp publish <app-name>
```

Suggested Azure app settings for writable state:

```
WORKSPACE_ROOT=/home/data/workspaces
SCHEDULER_STATE_PATH=/home/data/state/tasks.db
PROCESSED_IDS_PATH=/home/data/state/postmark_processed_ids.txt
INGESTION_DB_PATH=/home/data/state/ingestion.db
INGESTION_DEDUPE_PATH=/home/data/state/ingestion_processed_ids.txt
USERS_ROOT=/home/data/users
USERS_DB_PATH=/home/data/state/users.db
TASK_INDEX_PATH=/home/data/state/task_index.db
CODEX_DISABLED=1
```

## URLs

`host.json` sets `routePrefix` to empty, so URLs are:

- Health: `https://<app>.azurewebsites.net/health`
- Postmark inbound: `https://<app>.azurewebsites.net/postmark/inbound` (only when using `inbound_gateway`)

If you prefer `/api/...`, remove the `extensions.http.routePrefix` entry.

## Security

`HttpEntry/function.json` uses `authLevel: "anonymous"` for inbound webhooks.
Change to `function` or `admin` if you want to require keys.
