# DoWhiz Vision (Code-Aligned)

## 1) Current Baseline (What Exists Today)

As of the current codebase, DoWhiz operates as a multi-channel digital employee runtime with a startup-workspace product shell:

- ingress gateway (`inbound_gateway`) for webhook/event intake
- worker service (`rust_service`) for queue consumption, scheduling, and task execution
- channel coverage: email, slack, discord, sms/twilio, telegram, whatsapp, google docs/sheets/slides comments, bluebubbles/imessage
- task execution via Codex/Claude runner abstraction (`run_task_module`)
- Mongo-backed scheduler/user/index state
- Supabase Postgres-backed account/auth/billing data
- ingestion queue support for Postgres (legacy/optional) and Service Bus (required by gateway flow)
- frontend product routes:
  - `/` landing
  - `/start` founder intake -> startup workspace blueprint
  - `/workspace` workspace home
  - `/dashboard` internal analytics/supporting view
- canonical startup workspace modeling in both frontend and backend:
  - blueprint
  - resource model
  - starter tasks
  - agent roster
  - artifact queue
- provider runtime state endpoint for truthful connected/configured status overlays:
  - `GET /api/workspace/provider-state`

## 2) Product North Star

Help a founder spin up a one-person company workspace and digital founding team quickly, while keeping the same model extensible to small teams (2-5 people) without architectural rewrites.

Core product truth:
- primary object: workspace (not chat)
- primary interaction model: humans + agents operating in one shared system
- channel-native triggers remain first-class surfaces, but are not the product home

## 3) Product Object Model

DoWhiz models resources first, providers second.

First-class workspace resource categories:
- `workspace_home`
- `knowledge_hub_structured`
- `formal_docs`
- `build_system`
- `external_execution`
- `coordination_layer`
- `publish_presence`
- `agent_roster`
- `task_board`
- `artifact_queue`
- `approval_policy`

Mapping rule:
- tools are providers (GitHub, Google Docs, email, Slack/Discord, Notion)
- resources are product objects

This keeps automation truthful:
- connected vs available-not-configured vs planned/manual vs blocked

## 4) Platform Principles

1. Isolation first
- user-scoped workspace, memory, and task ownership must remain hard boundaries.

2. Operational reliability
- queue-driven ingestion + idempotent dedupe + observable retries.

3. Channel-native UX
- users interact from their existing channel; system handles tool complexity internally.
- workspace remains the product operating home post-onboarding.

4. Auditable behavior
- task scheduling, outbound actions, and long-term memory updates should be inspectable.
- artifact queues and approval policy should be visible, not implicit.

5. Truthful automation
- no fake “fully automated” claims where manual provisioning/approval is still required.

## 5) Role Model

Digital employees are role packages, not a single generic bot.

Each role combines:
- behavioral style/persona guidance
- model/runner defaults
- skill/tool policy
- risk/approval posture

Current employee config model already supports this via `employee.toml` fields (`runner`, `model`, `addresses`, role-specific guide files, skills dirs).

## 6) Target Architecture Evolution

### 6.1 Ingress and routing

Continue to strengthen gateway-first ingress:
- deterministic route resolution
- richer dedupe and replay controls
- stronger channel-level validation and anti-loop protection

### 6.2 Startup workspace product layer

Continue keeping startup workspace policy in dedicated scheduler modules:
- `scheduler_module/src/domain/*` for canonical product objects
- `scheduler_module/src/service/startup_workspace/*` for intake/bootstrap/provider-state policy
- `scheduler_module/src/service/workspace.rs` for workspace artifact persistence seams

### 6.3 Execution backends

Maintain dual-mode execution:
- local/docker for development and constrained environments
- Azure ACI backend for staging/production isolation and scalability
- keep execution logic in `run_task_module`; avoid moving product policy into runner monoliths.

### 6.4 Unified memory/account layer

Expand unified account-linked memory so cross-channel continuity improves while preserving per-user isolation and revocation controls.

## 7) Near-Term Priorities

1. Harden startup workspace bootstrap observability
- metrics/events for blueprint validation outcomes, resource-state generation, manual-step blockers, and provider-state resolution.

2. Harden queue and retry observability
- explicit metrics around enqueue/claim/ack/fail and channel-level error classes.

3. Improve test ergonomics and confidence
- keep canonical suite mapping in `reference_documentation/test_plans/DoWhiz_service_tests.md`
- expand startup workspace tests (domain + service layer) and keep runtime path tests green.

4. Tighten deployment coherence
- keep docs/scripts/workflows aligned to single runtime `.env` policy and explicit config-path selection.

5. Reduce operator ambiguity
- make gateway vs worker responsibilities unambiguous across docs and runbooks.

## 8) Long-Term Direction

DoWhiz should evolve into a trustworthy, multi-tenant startup workspace platform where:
- onboarding a new role is configuration-first
- adding a new channel is adapter-first
- scaling execution is backend-policy-first
- governance, artifact review, and human approvals remain first-class, not afterthoughts
