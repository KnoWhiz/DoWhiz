# DoWhiz Azure Function App

This folder contains the Azure Functions custom handler wrapper for the Rust service
(`DoWhiz_service` -> `scheduler_module` -> `rust_service`).

## Layout

- `host.json`: custom handler config and HTTP routing
- `HttpEntry/function.json`: catch-all HTTP trigger for proxying requests
- `rust_service`: compiled Linux binary (created by the build script)
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
curl -fsS -X POST -H "Content-Type: application/json" \
  -d '{"From":"agent@dowhiz.com","To":"test@example.com","Subject":"Local"}' \
  http://localhost:7071/postmark/inbound
```

## Local E2E script

This builds the binary, starts Azurite (if needed), runs `func host start`,
then checks `/health` and `/postmark/inbound`.

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
USERS_ROOT=/home/data/users
USERS_DB_PATH=/home/data/state/users.db
TASK_INDEX_PATH=/home/data/state/task_index.db
CODEX_DISABLED=1
```

## URLs

`host.json` sets `routePrefix` to empty, so URLs are:

- Health: `https://<app>.azurewebsites.net/health`
- Postmark inbound: `https://<app>.azurewebsites.net/postmark/inbound`

If you prefer `/api/...`, remove the `extensions.http.routePrefix` entry.

## Security

`HttpEntry/function.json` uses `authLevel: "anonymous"` for inbound webhooks.
Change to `function` or `admin` if you want to require keys.
