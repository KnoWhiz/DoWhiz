---
name: human-approval-gate
description: Use when browser/login flow asks for OTP, passcode, approval tap, or any user/admin verification step. Sends a request email and blocks until the first reply or timeout using the human_approval_gate CLI.
allowed-tools: Bash(human_approval_gate:*), Bash(python3:*), Bash(cat:*), Bash(test:*), Bash(date:*)
---

# Human Approval Gate (2FA / Login Approval)

## When to Use

Use this skill immediately when any authentication flow is blocked by human verification, for example:
- OTP / verification code input
- "Approve sign-in on your phone"
- "Tap number on mobile app"
- any out-of-band challenge that requires user/admin action from another device/account

Do NOT use this skill for CAPTCHA/image puzzle/text-recognition steps. First use your own multimodal/vision abilities plus browser tooling to inspect the challenge, solve it directly in the page, and continue yourself.

Only use this skill when the blocker genuinely requires a human outside the current browser session, for example:
- SMS code sent to a phone you cannot access
- email code sent to another person's mailbox
- approval tap / number match on another device
- recovery detail or one-time code that only the user/admin can retrieve

When a page offers multiple verification methods, choose SMS verification first by default.

Trigger this skill only after the website has already initiated the challenge. For example:
- click "send code", "text me a code", "email me a code", or similar first
- choose the verification method first if the site asks
- wait until the page is explicitly waiting for the code / tap / approval, then send the human approval email

For owner/admin login flows, if account email/username is missing, try known admin identifiers first (`dowhiz@deep-tutor.com` on staging, `oliver@dowhiz.com` on production). Do not use the approval gate only to ask for identifier when these known values are available.

Do not keep retrying login while blocked.

## Required Behavior

1. If the blocker is CAPTCHA/image/text recognition, solve it yourself in-browser first instead of opening the gate.
2. Trigger gate request right away once the page is explicitly waiting for human-only input.
3. Wait on gate result.
4. Continue only after the first reply is received and inspect that reply yourself.
5. If timeout, stop login attempts and report clearly.
6. If SMS verification is unavailable or fails, switch to another available method and keep the same gate-based wait behavior.

## Scope Rules

- `scope=admin`: when agent logs in an owner/admin account (for example Oliver's own Google/Notion/X account). Send to `admin@dowhiz.com`.
- `scope=user`: when agent logs in an end user's account. Send to that specific user email.
- For `scope=admin`, do not pass a user recipient address.

## CLI Quick Start

CLI fallback (if command not found on PATH):

```bash
if command -v human_approval_gate >/dev/null 2>&1; then
  HAG_CMD="human_approval_gate"
elif [ -x /app/bin/human_approval_gate ]; then
  HAG_CMD="/app/bin/human_approval_gate"
else
  HAG_CMD="python3 .agents/skills/human-approval-gate/scripts/human_approval_gate.py"
fi
```

### A) Create request and wait (preferred)

```bash
$HAG_CMD request \
  --scope admin \
  --account-label "Oliver Google account" \
  --action-text "Please reply in this thread with the verification code or approval result shown by Google" \
  --context "Agent is on Google verification page" \
  --timeout-minutes 30 \
  --wait
```

For user-owned account:

```bash
$HAG_CMD request \
  --scope user \
  --recipient "user@example.com" \
  --account-label "User X account" \
  --action-text "Please reply in this thread with the code or any instructions shown during login" \
  --timeout-minutes 30 \
  --wait
```

### B) Split mode (request then wait later)

```bash
$HAG_CMD request --scope admin --wait-timeout-minutes 30
$HAG_CMD wait --challenge-id "<challenge_id>" --timeout-minutes 30
```

### C) Check status

```bash
$HAG_CMD status --challenge-id "<challenge_id>" --refresh
```

## Return States

The command returns JSON with `status`:
- `replied`: inspect `reply` and continue only if the reply contains what the page needs
- `timeout`: stop login flow and tell user/admin to restart verification
- `pending`: still waiting
- `error`: command/runtime issue

## Reply Handling

No rigid reply format is required. Ask the recipient to reply in the same thread with the verification code, approval result, or other information shown by the site. The CLI returns the full reply details to the agent, and the agent decides how to interpret them.

## Important Notes

- Keep waiting in this command; do not run unrelated steps while waiting.
- Reuse the same challenge thread; do not spam multiple requests unless previous one timed out.
- Never include raw credentials in outbound messages.
- Sender identity priority is: `--from` > `HUMAN_APPROVAL_FROM` > employee mailbox from `employee.toml`/`employee.staging.toml`.
- HAG reply emails (`[HAG:...]` threads) are consumed by the gate flow and are not routed into normal Email->task execution.
