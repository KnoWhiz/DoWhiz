---
name: human-approval-gate
description: Use when browser/login flow asks for OTP, passcode, approval tap, or any user/admin verification step. Sends a request email and blocks until reply or timeout using the human_approval_gate CLI.
allowed-tools: Bash(human_approval_gate:*), Bash(python3:*), Bash(cat:*), Bash(test:*), Bash(date:*)
---

# Human Approval Gate (2FA / Login Approval)

## When to Use

Use this skill immediately when any authentication flow is blocked by human verification, for example:
- OTP / verification code input
- "Approve sign-in on your phone"
- "Tap number on mobile app"
- any out-of-band challenge that requires user/admin action from another device/account

Do NOT use this skill for CAPTCHA/image puzzle/text-recognition steps. Solve CAPTCHA directly in browser first.

When a page offers multiple verification methods, choose SMS verification first by default.

Do not keep retrying login while blocked.

## Required Behavior

1. Trigger gate request right away.
2. Wait on gate result.
3. Continue only if approved.
4. If timeout/rejected, stop login attempts and report clearly.
5. If SMS verification is unavailable or fails, switch to another available method and keep the same gate-based wait behavior.

## Scope Rules

- `scope=admin`: when agent logs in an owner/admin account (for example Oliver's own Google/Notion/X account). Send to `admin@dowhiz.com`.
- `scope=user`: when agent logs in an end user's account. Send to that specific user email.
- For `scope=admin`, do not pass a user recipient address.

## CLI Quick Start

### A) Create request and wait (preferred)

```bash
human_approval_gate request \
  --scope admin \
  --account-label "Oliver Google account" \
  --action-text "Please approve Google sign-in and send the OTP code if shown" \
  --context "Agent is on Google verification page" \
  --timeout-minutes 30 \
  --wait
```

For user-owned account:

```bash
human_approval_gate request \
  --scope user \
  --recipient "user@example.com" \
  --account-label "User X account" \
  --action-text "Please reply CODE: <code>" \
  --timeout-minutes 30 \
  --wait
```

### B) Split mode (request then wait later)

```bash
human_approval_gate request --scope admin --wait-timeout-minutes 30
human_approval_gate wait --challenge-id "<challenge_id>" --timeout-minutes 30
```

### C) Check status

```bash
human_approval_gate status --challenge-id "<challenge_id>" --refresh
```

## Return States

The command returns JSON with `status`:
- `approved`: continue login flow (use `resolution.code` when present)
- `rejected`: stop login flow
- `timeout`: stop login flow and tell user/admin to restart verification
- `pending`: still waiting
- `error`: command/runtime issue

## Reply Format Expected in Email

Tell recipient to reply in the same thread with one of:
- `CODE: 123456`
- `APPROVED`
- `DENIED`

The gate also parses simple natural replies (approved/denied keywords), but explicit format is preferred.

## Important Notes

- Keep waiting in this command; do not run unrelated steps while waiting.
- Reuse the same challenge thread; do not spam multiple requests unless previous one timed out.
- Never include raw credentials in outbound messages.
- Sender identity priority is: `--from` > `HUMAN_APPROVAL_FROM` > employee mailbox from `employee.toml`/`employee.staging.toml` > `POSTMARK_FROM_EMAIL` > `POSTMARK_TEST_FROM`.
