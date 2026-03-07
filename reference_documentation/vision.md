# DoWhiz Vision (Code-Aligned)

## 1) Current Baseline (What Exists Today)

As of current `dev` codebase, DoWhiz already operates as a multi-channel digital employee system with:

- ingress gateway (`inbound_gateway`) for webhook/event intake
- worker service (`rust_service`) for queue consumption, scheduling, and task execution
- channel coverage: email, slack, discord, sms/twilio, telegram, whatsapp, google docs/sheets/slides comments, bluebubbles/imessage
- task execution via Codex/Claude runner abstraction (`run_task_module`)
- Mongo-backed scheduler/user/index state
- Supabase Postgres-backed account/auth/billing data
- ingestion queue support for Postgres (legacy/optional) and Service Bus (required by gateway flow)

## 2) Product North Star

Give each user a dependable digital employee team that can:
- accept work from normal communication channels
- execute tasks with the right toolchain autonomously
- keep context over time (memory + references)
- follow up proactively and safely
- escalate to humans when ambiguity/approval/risk requires it

## 3) Platform Principles

1. Isolation first
- user-scoped workspace, memory, and task ownership must remain hard boundaries.

2. Operational reliability
- queue-driven ingestion + idempotent dedupe + observable retries.

3. Channel-native UX
- users interact from their existing channel; system handles tool complexity internally.

4. Auditable behavior
- task scheduling, outbound actions, and long-term memory updates should be inspectable.

## 4) Role Model

Digital employees are role packages, not a single generic bot.

Each role combines:
- behavioral style/persona guidance
- model/runner defaults
- skill/tool policy
- risk/approval posture

Current employee config model already supports this via `employee.toml` fields (`runner`, `model`, `addresses`, role-specific guide files, skills dirs).

## 5) Target Architecture Evolution

### 5.1 Ingress and routing

Continue to strengthen gateway-first ingress:
- deterministic route resolution
- richer dedupe and replay controls
- stronger channel-level validation and anti-loop protection

### 5.2 Orchestration

Continue evolving scheduler from task runner into policy engine:
- better follow-up planning primitives
- clearer cancellation/reschedule semantics
- stronger concurrency and fairness controls

### 5.3 Execution backends

Maintain dual-mode execution:
- local/docker for development and constrained environments
- Azure ACI backend for staging/production isolation and scalability

### 5.4 Unified memory/account layer

Expand unified account-linked memory so cross-channel continuity improves while preserving per-user isolation and revocation controls.

## 6) Near-Term Priorities

1. Harden queue and retry observability
- explicit metrics around enqueue/claim/ack/fail and channel-level error classes.

2. Improve test ergonomics and confidence
- keep canonical suite mapping in `reference_documentation/test_plans/DoWhiz_service_tests.md`
- close known gaps (queue race, ACI lifecycle, outbound failure injection).

3. Tighten deployment coherence
- keep docs/scripts/workflows aligned to single runtime `.env` policy and explicit config-path selection.

4. Reduce operator ambiguity
- make gateway vs worker responsibilities unambiguous across docs and runbooks.

## 7) Long-Term Direction

DoWhiz should evolve into a trustworthy, multi-tenant digital employee platform where:
- onboarding a new role is configuration-first
- adding a new channel is adapter-first
- scaling execution is backend-policy-first
- governance and audit remain first-class, not afterthoughts
