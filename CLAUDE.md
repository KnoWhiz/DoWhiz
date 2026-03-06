# CLAUDE.md

`external/` folder contains information about other projects that we can use as reference but we never need to touch the code in it.

By default open RUN_CODEX_E2E=1 and POSTMARK_LIVE_TEST=1 to use real emails address (mini-mouse@dowhiz.com / deep-tutor@deep-tutor.com to send emails to proto@dowhiz.com / oliver@dowhiz.com / maggie@dowhiz.com). For ngrok domain you can use: `https://shayne-laminar-lillian.ngrok-free.dev`

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

DoWhiz is a multi-tenant, email-first digital employee platform. Users send tasks to digital employees via email (and other channels like Slack, Discord, SMS via Twilio, Telegram, WhatsApp, Google Docs comments, iMessage via BlueBubbles), and AI agents (Codex CLI or Claude Code) process and respond. The system emphasizes per-user isolation, role-based agents, and tool-backed execution.

## Build and Development Commands

### Rust Backend (DoWhiz_service)

**IMPORTANT:** Always use `--release` flag when running cargo build/test to prevent debug artifacts from bloating storage:
```bash
cargo build --release                          # Build all modules (release)
cargo test --release                           # Run all tests (release)
cargo test --release -p scheduler_module       # Test specific module
cargo test --release -p scheduler_module --test scheduler_basic  # Single test
cargo clippy --all-targets --all-features     # Lint
cargo fmt --check                             # Format check

# Run HTTP server
cargo run --release -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001
```

### React Website
```bash
cd website
npm install && npm run dev    # Dev server (port 5173)
npm run build                 # Production build -> website/dist/
npm run lint                  # ESLint
```

### Live E2E Tests (require credentials)
```bash
RUST_SERVICE_LIVE_TEST=1 \
POSTMARK_INBOUND_HOOK_URL=https://YOUR-NGROK.ngrok.app \
POSTMARK_TEST_FROM=you@example.com \
cargo test -p scheduler_module --test service_real_email -- --nocapture
```

### Running Employees
```bash
# Worker-only local run (recommended with inbound gateway)
./DoWhiz_service/scripts/run_employee.sh little_bear 9001 --skip-hook --skip-ngrok
./DoWhiz_service/scripts/run_employee.sh mini_mouse 9002 --skip-hook --skip-ngrok

# Docker build and run
docker build -t dowhiz-service .
docker run --rm -p 9001:9001 \
  -e EMPLOYEE_ID=little_bear \
  -v "$PWD/DoWhiz_service/.env:/app/.env:ro" \
  dowhiz-service
```

## Architecture

### Workspace Structure (Cargo workspace)
- **scheduler_module**: Core HTTP worker server (Axum), inbound gateway binary (webhooks + dedupe), scheduler and user/index stores backed by MongoDB, plus account/auth/billing flows backed by Supabase Postgres
- **send_emails_module**: Postmark API wrapper for email delivery
- **run_task_module**: Codex/Claude CLI invocation for task execution
- **website**: React 19 + Vite marketing site

### Data Flow
```
Inbound (Email/Slack/Discord/SMS/Telegram/WhatsApp/Google Docs/iMessage)
    → Ingestion Gateway (dedupe + raw payload storage in Azure Blob)
    → Ingestion Queue (Service Bus for gateway; Postgres optional/legacy)
    → Worker Service (per-employee)
    → Scheduler/User Index (MongoDB) → Task Execution (Codex/Claude) → Outbound Reply
```

### Key Files
| File | Purpose |
|------|---------|
| `scheduler_module/src/service/server.rs` | Worker HTTP server, scheduler loop |
| `scheduler_module/src/bin/inbound_gateway.rs` | Inbound gateway entrypoint (webhooks + dedupe) |
| `scheduler_module/src/lib.rs` | Core Scheduler, TaskKind, Schedule definitions |
| `scheduler_module/src/user_store/mod.rs` | Per-user data management |
| `send_emails_module/src/lib.rs` | Postmark API wrapper |
| `run_task_module/src/lib.rs` | Codex/Claude CLI invocation |
| `scheduler_module/src/adapters/whatsapp.rs` | WhatsApp inbound/outbound adapter |
| `DoWhiz_service/employee.toml` | Employee registry (addresses, runners, models) |

### Runtime State
```
$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/
├── state/
│   ├── tasks.db              # Scope key path for scheduler records in MongoDB
│   ├── users.db              # Scope key path for user records in MongoDB
│   ├── task_index.db         # Scope key path for due-task index records in MongoDB
│   └── postmark_processed_ids.txt
├── workspaces/<message_id>/  # Per-task execution
└── users/<user_id>/
    ├── state/tasks.db        # User-scope key path for per-user scheduler records in MongoDB
    ├── memory/               # Agent context
    └── mail/                 # Email archive
```

### Employees
| ID | Name | Runner | Primary Email |
|----|------|--------|---------------|
| `little_bear` | Oliver | Codex | oliver@dowhiz.com |
| `mini_mouse` | Maggie | Claude | maggie@dowhiz.com |
| `sticky_octopus` | Devin | Codex | devin@dowhiz.com |
| `boiled_egg` | Boiled-Egg | Codex | proto@dowhiz.com |

## Key Concepts

### Agent Architecture Philosophy

**Core Principle: Agents are channel-agnostic.** All inbound messages (email, Slack, Discord, Notion comments, WhatsApp, etc.) are "just messages" to the agent. The agent should be able to:

1. **Be triggered from anywhere** - A comment in Notion is equal to an email or Discord message
2. **Use any tool** - Agent decides what tools to use (Google Drive, Sheets, browser automation, etc.) based on the task
3. **Reply on the same channel** - The only hardcoded logic is: if a user sends a message on one channel, the agent must reply on that channel

**What NOT to hardcode:**
- Task execution logic should NOT be channel-specific
- Agents should NOT be restricted to working "on the same doc" (e.g., Google Docs comment → can create new docs elsewhere)
- Tool availability should NOT depend on the inbound channel

**Channel isolation is ONLY for outbound:**
- `SendReplyTask` structs are channel-specific (different fields per channel)
- `codex.rs` / task execution is channel-agnostic
- Inbound processing normalizes all channels to common `InboundMessage`

**Example autonomous workflow:**
> User sends email: "Do XX research and give me a report"
> Agent should autonomously:
> 1. Search Google Drive for existing data
> 2. Open Google Sheets to analyze data
> 3. Use browser automation for web research
> 4. Draft report in Google Docs
> 5. Send email reply with link to report

This is the **ultimate goal**: digital employees as autonomous as real people, not restricted to hardcoded task kinds.

### Skills-Based Architecture (Target State)

**Convert adapter logic → skill.md instructions.** Instead of hardcoding outbound adapters in Rust, teach agents via skills:

```
┌─────────────────────────────────────────────────────────────┐
│ Current State                                               │
│ Inbound → Gateway → RunTask → Codex → SendReplyTask (Rust)  │
│                                 ↓                           │
│                    Hardcoded adapter per channel            │
├─────────────────────────────────────────────────────────────┤
│ Target State                                                │
│ Inbound → Gateway → RunTask → Codex (with skills)           │
│                                 ↓                           │
│              Agent uses skill.md to send messages           │
│              (Discord skill, Email skill, Notion skill...)  │
└─────────────────────────────────────────────────────────────┘
```

**Two things we need to build:**
1. **Message channels** - Inbound handlers for each channel (email, Slack, Notion, etc.)
2. **Tools/Skills** - What agents have access to (browser-use, Google Workspace CLI, xlsx, etc.)

**Cross-API functionality:**
- Agent receives via Discord → edits Google Sheet → sends summary via Email
- Agent receives via Notion comment → creates Google Doc → replies to Notion
- Sending messages is a SKILL/TOOL, not just adapter code

**Workspace Preparation (Per-User Isolation):**
```
Before mounting agent container:
1. Query all user info by account_id (MongoDB RLS-like policy)
2. Retrieve: linked accounts, channel configs, available tools
3. Mount this context into the workspace
4. Agent operates ONLY on that user's resources
```

**Security constraint:**
- Agent can ONLY access resources belonging to the triggering user
- Cannot "scam" other users (e.g., "delete all docs from person X")
- MongoDB owner scopes enforce isolation

**Skills distribution:**
- Skills directory: https://skills.sh/
- npm packages: `@googleworkspace/cli`, etc.
- Local skills in `.claude/skills/`

### Channel Implementation Checklist

When adding a new channel, verify it follows the architecture:

| Aspect | Current State | Target State |
|--------|--------------|--------------|
| Inbound | Rust adapter → `InboundMessage` | Same (keep) |
| Task execution | `codex.rs` (channel-agnostic) | Same (keep) |
| Outbound | Rust `execute_X_send()` | Agent uses skill.md |
| Cross-channel | Not supported | Agent can send to any channel |

**Notion implementation analysis:**
- ✅ Inbound: `poller.rs` creates `InboundMessage` correctly
- ✅ Multi-workspace: Automatically polls all accessible workspaces
- ⚠️ Outbound: Still uses Rust queue → poller pattern
- ❌ Skill conversion: Browser logic should become a skill.md

**Future work for Notion:**
1. Convert `browser.rs` actions to skill instructions
2. Let agent call browser-use directly via skill
3. Keep `execute_notion_send` as fallback during transition

### Task Kinds
- **SendReply**: Send outbound reply on a channel (email/slack/discord/sms/telegram/whatsapp/google workspace)
- **RunTask**: Invoke Codex/Claude CLI to generate reply
- **Noop**: Testing placeholder

### Schedules
- **Cron**: 6-field format `sec min hour day month weekday` (UTC)
- **OneShot**: Single execution at specific DateTime

### Per-User Isolation
Each user gets isolated workspace/memory/mail directories plus Mongo owner scopes derived from per-user paths. Default concurrency limits are global 200 and per-user 1 (configurable via env).

### Follow-up Scheduling
Agents emit scheduled tasks in stdout:
```
SCHEDULED_TASKS_JSON_BEGIN
[{"type":"send_email","delay_minutes":15,"subject":"...","html_path":"...","to":[...]}]
SCHEDULED_TASKS_JSON_END
```

## Required Environment Variables

Copy `.env.example` to `DoWhiz_service/.env` and configure:
- `POSTMARK_SERVER_TOKEN` - Postmark API key (required)
- `AZURE_OPENAI_API_KEY_BACKUP` - Required for Codex and Claude runners (Foundry config)
- `AZURE_OPENAI_ENDPOINT_BACKUP` - Optional endpoint override for components that use Azure OpenAI directly (Codex runner endpoint is fixed in code)
- `MONGODB_URI` - Required for scheduler/user/index/slack persistence
- `SUPABASE_DB_URL` - Required by `AccountStore` (auth/account-link flows in worker and gateway; billing routes in worker when Stripe is enabled)

Optional:
- `CODEX_DISABLED=1` - Bypass Codex CLI (uses placeholder replies)
- `RUN_TASK_DOCKER_IMAGE` - Enable per-task Docker isolation
- `GITHUB_USERNAME`, `GITHUB_PERSONAL_ACCESS_TOKEN` - GitHub access for agents
- `OPENAI_API_KEY` - Enable message router quick replies
- `INGESTION_QUEUE_BACKEND=servicebus` + `SERVICE_BUS_CONNECTION_STRING` + `SERVICE_BUS_QUEUE_NAME` - Required when running the inbound gateway
- `AZURE_STORAGE_CONTAINER_INGEST` + `AZURE_STORAGE_SAS_TOKEN` (and optionally `AZURE_STORAGE_ACCOUNT` or `AZURE_STORAGE_CONTAINER_SAS_URL`) - Raw payload storage when `RAW_PAYLOAD_STORAGE_BACKEND=azure`
- `SUPABASE_JWT_SECRET` + `SUPABASE_ANON_KEY` + `SUPABASE_PROJECT_URL` - Recommended for token validation in `/auth/*` and `/billing/*`

## Testing Expectations

After completing code changes, you must design targeted, detailed unit tests and end-to-end tests to ensure both new and existing functionality behave as expected. Debug and resolve any issues found during test runs. If certain issues require manual intervention, provide a detailed report and follow-up steps.

## Test Checklist and Report (DoWhiz_service)
- Canonical checklist: `reference_documentation/test_plans/DoWhiz_service_tests.md`
- After any DoWhiz_service change, run all relevant AUTO tests from the checklist.
- For LIVE/MANUAL/PLANNED entries, mark SKIP with a reason unless explicitly run.
- If the user asks so, include the Test Report table from the checklist in your final response (PASS/FAIL/SKIP per Test ID). Otherwise by default summarize the tests results.
- For real end-to-end tests requiring a public ngrok URL, use `https://shayne-laminar-lillian.ngrok-free.dev` and set up the environment accordingly.

## Long-Running Task Workflow

When working on complex tasks that span multiple interactions or require significant changes, follow this structured approach inspired by effective engineering practices.

### Session Startup Protocol

Before making any changes, always orient yourself:

1. **Understand current state**
   - `git status` and `git log --oneline -10` to see recent changes
   - Check for any uncommitted work or work-in-progress
   - Read relevant progress notes if they exist

2. **Verify baseline functionality**
   - Run `cargo build --release` to ensure the codebase compiles
   - For service changes: start the dev server and verify basic functionality works
   - Identify and fix any existing broken state BEFORE adding new features

3. **Review task scope**
   - Read the user's request carefully
   - If the task is complex, decompose it into discrete sub-tasks using TodoWrite
   - Each sub-task should be completable and testable independently

### Incremental Progress Principle

**Work on ONE thing at a time.** This is critical for maintaining code quality:

| DO | DON'T |
|----|-------|
| Complete one feature, test it, commit | Try to implement multiple features at once |
| Fix one bug, verify the fix, commit | Make sweeping changes across many files |
| Refactor one module, ensure tests pass, commit | Refactor while also adding new features |

### Clean State Handoff

Every work session should end with the codebase in a mergeable state:

1. **No half-implemented features** - Either complete the feature or revert to last working state
2. **All tests pass** - Run `cargo test --release` before considering work complete
3. **Meaningful git commits** - Each commit should represent a logical, working unit
4. **Clear progress notes** - If work continues later, document:
   - What was completed
   - What remains to be done
   - Any blockers or decisions needed

### Self-Verification Checklist

Before marking a task complete, verify:

- [ ] Code compiles without warnings (`cargo build --release`)
- [ ] Relevant tests pass (`cargo test --release -p <module>`)
- [ ] New functionality has been tested (unit test or manual verification)
- [ ] No regressions in existing functionality
- [ ] Code follows project conventions (rustfmt, clippy)

### Complex Task Decomposition

For tasks requiring multiple steps, use TodoWrite to create a structured checklist:

```
Example: "Add WhatsApp media support"
├── [ ] Research WhatsApp media API requirements
├── [ ] Add media types to adapter schema
├── [ ] Implement media download handler
├── [ ] Implement media upload handler
├── [ ] Add unit tests for media handling
├── [ ] Test end-to-end with real WhatsApp sandbox
└── [ ] Update documentation
```

Mark each item complete ONLY after:
1. The code change is made
2. The change is tested
3. The change is committed

### Recovery from Bad State

If you discover the codebase is in a broken state:

1. **Don't proceed with new work** - Fix the existing issue first
2. **Use git to understand what changed** - `git diff`, `git log`, `git blame`
3. **Consider reverting** - `git checkout <file>` or `git revert <commit>` if needed
4. **Document the issue** - Note what was broken and how it was fixed

### When Stuck on a Bug

If the same bug persists after 2-3 attempts with different approaches:

1. **Stop and reassess** - Don't keep trying the same fix repeatedly
2. **Use WebSearch** - Search for the error message, library version issues, or known bugs
3. **Check external resources** - Official docs, GitHub issues, Stack Overflow may have solutions
4. **Consider alternative approaches** - Sometimes a different architecture avoids the problem entirely
5. **Ask the user** - If still blocked, clearly explain what was tried and ask for guidance

### Post-Debugging Reflection (IMPORTANT)

**After resolving any significant debugging session, you MUST:**

1. **Identify root cause** - What was the actual issue? (e.g., missing env var, wrong API assumption, format mismatch)

2. **Document the lesson** - Update relevant documentation:
   - **Skill files** (`.claude/skills/*/SKILL.md`) - Add troubleshooting tips, required env vars, format notes
   - **Memory files** (`~/.claude/projects/.../memory/`) - Add project-specific learnings
   - **This file (CLAUDE.md)** - Add gotchas that apply to multiple features

3. **Prevent recurrence** - Ask yourself:
   - Would another agent hit the same issue following current docs?
   - What one line of documentation would have saved hours of debugging?
   - Are there similar patterns elsewhere that need the same fix?

**Examples of lessons worth documenting:**
- `IN_DOCKER=true` required for browser-use in WSL/root (cost: 2+ hours debugging)
- `browser-use state` output uses tabs not spaces (cost: regex debugging)
- Notion `/notifications` URL doesn't work, must click Inbox button (cost: navigation failures)
- Cookie-first auth for OAuth sites to avoid rate limiting (cost: account lockouts)

**Where to document:**
| Type of Lesson | Location |
|----------------|----------|
| Tool-specific (browser-use, etc.) | `.claude/skills/<tool>/SKILL.md` |
| Project-specific patterns | `~/.claude/projects/.../memory/*.md` |
| Cross-cutting concerns | This file (CLAUDE.md) |
| Feature-specific | Inline code comments or feature skill file |

---

## Coding Guidelines

**Contributor principle:** Keep the codebase modular and easy to maintain. If a file grows too large (roughly 500–1000 lines), consider splitting it into smaller, well-defined modules with clear responsibilities.

Use standard Rust formatting (rustfmt) with `snake_case` for functions/modules, `CamelCase` for types.

Tests live in `*/tests/*.rs` for integration and in-module `#[test]` for unit coverage. Live Postmark tests are opt-in via environment variables.

When opening PRs, include a short summary, tests run, and any required env/config changes.

---

## Known Gotchas (Lessons Learned)

This section collects hard-won debugging lessons. **Read before starting work on related features.**

### browser-use CLI

| Issue | Solution | Cost if Ignored |
|-------|----------|-----------------|
| Browser times out after 30s in WSL/root | Set `IN_DOCKER=true` env var | Hours of debugging |
| `browser-use state` output format | Uses tabs for indentation, `[index]<element>` format | Regex parsing failures |
| OAuth sites (Notion, Slack) rate limit | Use cookie import/export, not automated login | Account lockouts |
| Login detection fails | Check for UI text ("New page", "Inbox") not CSS classes | False "not logged in" errors |

### Notion Integration

| Issue | Solution | Cost if Ignored |
|-------|----------|-----------------|
| `/notifications` URL returns 404 | Click Inbox button in sidebar instead | Navigation failures |
| Notification parsing misses items | Regex must handle tabs, various date formats ("Mar 2", "Yesterday") | Missed @mentions |
| Same notification matched twice | Add content-based deduplication (actor + page + mentioned) | Duplicate processing |

### Rust/Cargo

| Issue | Solution | Cost if Ignored |
|-------|----------|-----------------|
| Debug builds bloat storage | Always use `--release` flag | Disk full errors |
| MongoDB not available locally | Code handles gracefully with noop mode | Test failures |

### General Patterns

| Issue | Solution | Cost if Ignored |
|-------|----------|-----------------|
| API format assumptions | Save actual response to file, test parsing offline | Hours of trial-and-error |
| Environment differences | Document required env vars in skill files | Works locally, fails in CI/prod |

---

# Operations & Deployment Notes (Azure VM)

Use `DoWhiz_service/OPERATIONS.md` as the source of truth for VM paths, PM2 commands, deployment runbooks, and troubleshooting.

Current deployment policy:
- Production VM deploy target branch: `main`
- Staging VM deploy target branch: `dev`

Single `.env` split policy:
- Production uses base keys
- Staging uses `STAGING_` keys with `DEPLOY_TARGET=staging`
- Runtime mapping is handled by `DoWhiz_service/scripts/load_env_target.sh`

For exact staging/prod commands, key split table, and rollback:
- `DoWhiz_service/docs/staging_production_deploy.md`

**IMPORTANT:** Never start services manually on prod/staging VMs. Use PM2 commands:
```bash
pm2 restart all              # Restart all services
pm2 list                     # Check service status
pm2 logs dw_gateway          # View gateway logs
pm2 logs dw_worker           # View worker logs
```

Log file locations:
- Current logs: `pm2 logs dw_gateway` / `pm2 logs dw_worker`
- History logs: `~/server/logs/`
- Service Bus grep: `grep -i "service bus\\|enqueue\\|error" /home/azureuser/server/.pm2/logs/dowhiz-inbound-gateway-out.log | tail -50`

**tmux sessions:** You can also check logs via tmux windows opened with `tmux a`.
- **DO NOT close these windows** - they are persistent monitoring sessions
- To detach (leave without closing): `Ctrl-a d` (prefix is Ctrl-a, not default Ctrl-b)

Key env names to verify first:
- `SERVICE_BUS_CONNECTION_STRING` (or `SCALE_OLIVER_SERVICE_BUS_CONNECTION_STRING`)
- `SERVICE_BUS_QUEUE_NAME`
- `INGESTION_QUEUE_BACKEND=servicebus`
- `RAW_PAYLOAD_STORAGE_BACKEND=azure` (recommended for gateway production flow)
- `AZURE_STORAGE_CONTAINER_INGEST`
- `AZURE_STORAGE_SAS_TOKEN` (or `AZURE_STORAGE_CONTAINER_SAS_URL`)
