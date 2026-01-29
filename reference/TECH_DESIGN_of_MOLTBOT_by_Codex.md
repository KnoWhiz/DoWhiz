# TECH_DESIGN

## Scope
This document summarizes the Moltbot architecture with emphasis on:
- Multi-agent framework (routing, isolation, and cross-agent tools)
- Agent personalities (workspace and identity injection)
- Scheduled jobs (cron and heartbeat)

## System overview
- The Gateway is a single long-lived process that owns all messaging surfaces and exposes a WebSocket API to clients (mac app, CLI, web UI) and nodes (macOS/iOS/Android/headless).
- Clients connect over WebSocket, send request frames, and receive request responses plus server-push events (agent, chat, presence, heartbeat, cron, etc.).
- The agent runtime is embedded and derived from p-mono, but session management, tool wiring, and routing are Moltbot-owned.
- A canvas host serves agent-editable HTML and A2UI content for UI surfaces.

## Multi-agent framework

### Agent boundaries and storage
An agent is a fully scoped unit with its own workspace, state directory, and session store:
- Workspace: files like AGENTS.md, SOUL.md, USER.md, IDENTITY.md, etc.
- Agent dir (per-agent state): stores auth profiles and model registry.
  - `~/.clawdbot/agents/<agentId>/agent/auth-profiles.json` is per-agent and not shared.
- Sessions: `~/.clawdbot/agents/<agentId>/sessions/` with JSONL transcripts and a per-agent `sessions.json` store.

Defaults and resolution:
- Default agent id is `main` unless another entry is marked `default: true`.
- Default workspace is `~/clawd` for the default agent; non-default agents default to `~/clawd-<agentId>` unless overridden.
- Default agent dir is `~/.clawdbot/agents/<agentId>/agent` unless overridden.

Important isolation note:
- The workspace is the default cwd, not a hard sandbox. Relative paths resolve inside the workspace, but absolute paths can reach elsewhere unless sandboxing is enabled.

### Routing to agents (bindings)
Inbound messages are routed to agents through deterministic bindings.
- Binding match precedence (most specific wins):
  1) peer match (exact DM/group/channel id)
  2) guild id (Discord)
  3) team id (Slack)
  4) account id match for a channel
  5) channel-level match (`accountId: "*"`)
  6) fallback to default agent (or first agent entry)
- Multi-account channels use `accountId` to identify each login instance and can route each account to different agents.

### Sessions and multi-agent scoping
Direct chats collapse to the agent main key by default, while groups are isolated:
- DM keys (default): `agent:<agentId>:<mainKey>`
- Group keys: `agent:<agentId>:<channel>:group:<id>` (or channel/topic variants)
- Cron keys: `cron:<jobId>`

DM scoping (`session.dmScope`):
- `main` (default) uses the shared main session.
- `per-peer`, `per-channel-peer`, `per-account-channel-peer` isolate DMs.
- `session.identityLinks` can map provider-prefixed peer ids to a canonical identity so the same person shares a session across channels.

### Per-agent config and isolation controls
Each `agents.list[]` entry can override:
- Workspace, agent dir, and model.
- Identity (name/theme/emoji/avatar).
- Heartbeat settings and group chat mention patterns.
- Tool policy (`allow`/`deny`) and sandbox config (mode/scope).
- Subagent settings (model, concurrency, allowlist).

Important details:
- `tools.elevated` is global and sender-based, not per-agent.
- When sandboxing is enabled and `workspaceAccess` is not `rw`, tools operate inside the sandbox workspace under `~/.clawdbot/sandboxes`.

### Skills in multi-agent setups
Skills are resolved per agent with a clear precedence:
- Workspace skills: `<workspace>/skills`
- Shared/managed skills: `~/.clawdbot/skills`
- Bundled skills shipped with the install

### Cross-agent tools and messaging
Moltbot provides session tools that allow cross-session and cross-agent interactions:
- `sessions_list`, `sessions_history`, `sessions_send`, `sessions_spawn`.
- Cross-agent access is gated by `tools.agentToAgent`:
  - Disabled by default.
  - Allowlist (`tools.agentToAgent.allow`) must include target agents.
- `sessions_send` includes a reply-back ping-pong loop (stop with `REPLY_SKIP`), capped by `session.agentToAgent.maxPingPongTurns` (0-5, default 5).
- After the ping-pong loop, an announce step runs on the target agent (reply `ANNOUNCE_SKIP` to suppress delivery).
- Sandboxed sessions default to seeing only spawned sessions unless `agents.defaults.sandbox.sessionToolsVisibility` is set to `all`.

### Sub-agents (background runs)
Sub-agents are isolated agent runs spawned from an existing session:
- Session key: `agent:<agentId>:subagent:<uuid>`
- Tool: `sessions_spawn` (non-blocking, returns immediately)
- `agents.list[].subagents.allowAgents` controls which agent ids can be targeted (default: requester agent only).
- Default tool policy: all tools except session tools
- Cannot spawn sub-agents (no nested fan-out)
- Announce step posts results back to the requester chat (use `ANNOUNCE_SKIP` to suppress)
- Sub-agent auth is resolved by agent id; main agent auth profiles are merged as fallbacks.
- Auto-archive after `agents.defaults.subagents.archiveAfterMinutes` (default 60)
- Dedicated queue lane `subagent` with concurrency `agents.defaults.subagents.maxConcurrent`
- Sub-agent context injects only AGENTS.md and TOOLS.md (no SOUL.md, IDENTITY.md, USER.md, HEARTBEAT.md, BOOTSTRAP.md)

## Agent personalities and identity

### Workspace bootstrap and system prompt
Moltbot builds a custom system prompt on every run and injects workspace files on first turn:
- Injected files: AGENTS.md, SOUL.md, TOOLS.md, IDENTITY.md, USER.md, HEARTBEAT.md, BOOTSTRAP.md (only on brand-new workspaces)
- Large files are truncated per `agents.defaults.bootstrapMaxChars` with a marker.
- Missing files inject a short missing-file marker.

Personality comes primarily from:
- SOUL.md (persona, tone, boundaries)
- AGENTS.md (operating instructions and priorities)
- USER.md (user profile and how to address them)
- IDENTITY.md (name, emoji, theme, vibe)

Sub-agent runs use a minimal prompt and do not inject SOUL.md, IDENTITY.md, USER.md, HEARTBEAT.md, or BOOTSTRAP.md.

### Identity config and UX behavior
Per-agent identity config (`agents.list[].identity`) controls defaults and UX:
- `name`, `theme`, `emoji`, `avatar`
- `identity.emoji` can drive `messages.ackReaction` defaults.
- `identity.name`/`emoji` seed group mention patterns so native mentions map to the correct agent.
- `identity.avatar` must be inside the workspace or be a remote URL/data URI.

### IDENTITY.md parsing and CLI support
The `moltbot agents set-identity` command can:
- Read `IDENTITY.md` from a workspace (`--from-identity`)
- Override identity fields explicitly
- Write values into `agents.list[].identity`

## Scheduling and automation

### Cron scheduler (Gateway)
Cron runs inside the Gateway process and persists jobs so schedules survive restarts.

Core properties:
- Store path: `~/.clawdbot/cron/jobs.json` (configurable via `cron.store`)
- Run history: `~/.clawdbot/cron/runs/<jobId>.jsonl`
- Enable/disable: `cron.enabled` or `CLAWDBOT_SKIP_CRON=1`
- Concurrency: `cron.maxConcurrentRuns` (default 1)

Job structure:
- `schedule`: when to run
- `payload`: what to do
- `sessionTarget`: `main` or `isolated`
- Optional: `agentId`, delivery settings, `deleteAfterRun`
Constraints:
- `sessionTarget: "main"` requires `payload.kind: "systemEvent"`.
- `sessionTarget: "isolated"` requires `payload.kind: "agentTurn"`.

Schedule kinds:
- `at`: one-shot timestamp (ISO accepted; UTC if no timezone)
- `every`: fixed interval (ms)
- `cron`: 5-field cron expression with optional IANA timezone (host timezone if omitted)

Execution modes:
- Main session (`sessionTarget: "main"`): payload is `systemEvent`. The job enqueues a system event and runs on the next heartbeat, or immediately when `wakeMode: "now"` is used.
- Isolated session (`sessionTarget: "isolated"`): payload is `agentTurn`. Each run uses a fresh session id in `cron:<jobId>` and posts a summary back to main (configurable via post-to-main fields). Output can also be delivered to a channel.

Agent selection:
- If `job.agentId` is set and exists, the job runs under that agent; otherwise it falls back to the default agent.

Delivery behavior (isolated jobs):
- `deliver: true` sends output to the last route or the explicit `channel`/`to` target.
- If `to` is set, delivery occurs even when `deliver` is omitted; use `deliver: false` to keep it internal.
- `bestEffortDeliver` avoids failing the job if delivery fails.
- Telegram topics use `-100...:topic:<id>` to target a forum thread.

Interfaces:
- CLI: `moltbot cron add|edit|list|status|remove|run|runs`
- Gateway API: `cron.status`, `cron.list`, `cron.add`, `cron.update`, `cron.remove`, `cron.run`, `cron.runs`
- Agent tool: `cron` with actions `status`, `list`, `add`, `update`, `remove`, `run`, `runs`, and `wake`

### Heartbeat scheduler
Heartbeat runs periodic agent turns in the main session to surface important updates without spamming.

Key behaviors:
- Default interval is 30m (or 1h for Anthropic OAuth/setup-token). Set to `0m` to disable.
- Prompt default: reads HEARTBEAT.md and replies `HEARTBEAT_OK` when nothing needs attention.
- `HEARTBEAT_OK` replies are stripped and suppressed when short (<= `ackMaxChars`).
- `activeHours` can restrict the run window based on configured timezone.
- Per-agent heartbeats: if any `agents.list[]` entry includes a heartbeat block, only those agents run heartbeats.
- Delivery is controlled by `target` (`last`/`none`/explicit channel) and optional `to`.
- Visibility controls per channel/account: `showOk`, `showAlerts`, `useIndicator`.
- If visibility disables all output, heartbeat runs are skipped to save API calls.

### Cron vs heartbeat (daily messages)
- Use cron for exact timing and one-shot reminders (for example, daily messages at a precise time).
- Use heartbeat for periodic, context-aware checks that should be batched in the main session.

## Key config and file paths
- Config: `~/.clawdbot/moltbot.json`
- Agent dir: `~/.clawdbot/agents/<agentId>/agent`
- Sessions: `~/.clawdbot/agents/<agentId>/sessions/`
- Workspace: `~/clawd` (default agent) or `~/clawd-<agentId>` (non-default)
- Cron store: `~/.clawdbot/cron/jobs.json`
- Cron runs: `~/.clawdbot/cron/runs/<jobId>.jsonl`

## References
- docs/concepts/architecture.md
- docs/concepts/agent.md
- docs/concepts/agent-workspace.md
- docs/concepts/system-prompt.md
- docs/concepts/multi-agent.md
- docs/concepts/session.md
- docs/concepts/session-tool.md
- docs/tools/subagents.md
- docs/gateway/configuration.md
- docs/automation/cron-jobs.md
- docs/automation/cron-vs-heartbeat.md
- docs/gateway/heartbeat.md
- docs/cli/agents.md
- src/agents/agent-scope.ts
- src/agents/tools/cron-tool.ts
- src/gateway/server-cron.ts
