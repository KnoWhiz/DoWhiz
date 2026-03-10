# MongoDB Staging Isolation E2E (2026-03-03)

## Scope

Validate Mongo-backed staging flow for:

- inbound email -> run_task -> immediate reply
- 5-minute follow-up reminder execution
- user isolation across two distinct senders

## Environment

- Branch: `oliver/mongodb-azure-refactor`
- Staging VM: `dowhizstaging`
- Deploy target: `staging`
- Storage backend: `mongo` (runtime keys in current policy are unprefixed: `STORAGE_BACKEND`, `MONGODB_URI`, `MONGODB_DATABASE`)
- Codex execution backend: Azure ACI (staging-targeted keys)

Note:
- This file is a dated E2E evidence snapshot from 2026-03-03.
- Current deployment policy uses unprefixed runtime `.env` keys; CI/CD merges environment-specific secret sets before writing `.env`.

## Senders / Users

- Sender A: `mini-mouse@dowhiz.com`
  - Resolved user_id: `5d4d8167-a72d-46f6-9c05-74521cc80748`
- Sender B: `deep-tutor@deep-tutor.com`
  - Resolved user_id: `9e9d4691-1080-44a3-9eaa-1be565e8adbe`

Both were observed in staging worker logs as inbound requesters and mapped to distinct user IDs.

## E2E Evidence

### User A

- `resolved inbound requester ... identifier=mini-mouse@dowhiz.com`
- run_task success:
  - `task_id=40dd8985-6b2a-41a0-9241-84f68858ec71`
- follow-up scheduled:
  - `task_id=0df7ac45-4bea-44ed-a37c-0e96bab637dd delay_seconds=300`
- follow-up completed:
  - `scheduler task completed task_id=0df7ac45-4bea-44ed-a37c-0e96bab637dd ... status=success`

### User B

- `resolved inbound requester ... identifier=deep-tutor@deep-tutor.com`
- run_task success:
  - `task_id=87d06658-a78e-4200-a791-b4b0f8ba1c81`
- follow-up scheduled:
  - `task_id=18f36d2a-c58f-4e5e-afac-48169a8ed92a delay_seconds=300`
- follow-up completed:
  - `scheduler task completed task_id=18f36d2a-c58f-4e5e-afac-48169a8ed92a ... status=success`

## Mongo Isolation Checks

Queried staging database (`dowhiz_staging_little_bear`) with PyMongo 3.13:

- `users`:
  - `mini-mouse@dowhiz.com -> 5d4d8167-a72d-46f6-9c05-74521cc80748`
  - `deep-tutor@deep-tutor.com -> 9e9d4691-1080-44a3-9eaa-1be565e8adbe`
- `tasks` / `task_executions`:
  - owner-scoped records exist for both user IDs
- cross-user contamination check:
  - scanned `task_json` for references to the other user's `/users/<user_id>/` path
  - result: `CROSS_VIOLATIONS = 0`

Conclusion: user-scoped task ownership and execution isolation are working in staging with MongoDB.
