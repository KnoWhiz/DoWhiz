---
name: human-approval-gate
description: Use when browser/login flow is blocked by CAPTCHA, missing password, OTP, passcode, approval tap, or another user/admin verification step. In run_task/Codex environments, use the blocking MCP tool so the current browser session stays paused on the same page until the human replies.
allowed-tools: Bash(human_approval_gate:*), Bash(python3:*), Bash(cat:*), Bash(test:*), Bash(date:*)
---

# Human Approval Gate (2FA / Login Approval)

## When to Use

Use this skill when any authentication flow is blocked by something the agent genuinely cannot finish alone, for example:
- CAPTCHA after one failed built-in solve attempt
- missing password after checking the workspace `.env`
- OTP / verification code input
- "Approve sign-in on your phone"
- "Tap number on mobile app"
- any out-of-band challenge that requires user/admin action from another device/account

For CAPTCHA/image puzzle/text-recognition steps:
- first use your own multimodal/vision abilities plus browser tooling to inspect the challenge and attempt one direct solve in the page
- if that first attempt fails and you are still blocked on CAPTCHA, open the gate with challenge type `captcha`
- always attach the current browser screenshot so the human can see exactly what blocked you

Only use this skill when the blocker genuinely requires a human outside the current browser session, for example:
- SMS code sent to a phone you cannot access
- email code sent to another person's mailbox
- approval tap / number match on another device
- recovery detail or one-time code that only the user/admin can retrieve
- password not present in the workspace `.env`

When a page offers multiple verification methods, choose SMS verification first by default.

Trigger this skill only after the website has already initiated the challenge. For example:
- click "send code", "text me a code", "email me a code", or similar first
- choose the verification method first if the site asks
- wait until the page is explicitly waiting for the code / tap / approval, then send the human approval email
- for 2FA requests, describe exactly which method is active: SMS, email, authenticator app, or device tap / number match
- for every request type, attach the current browser screenshot(s)

For owner/admin login flows, if account email/username is missing, try known admin identifiers first (`dowhiz@deep-tutor.com` on staging, `oliver@dowhiz.com` on production). Do not use the approval gate only to ask for identifier when these known values are available.

For password requests, check the workspace `.env` first. For Google login, prefer checking `GOOGLE_PASSWORD` before asking a human.

Do not keep retrying login while blocked.

## Required Behavior

1. If the blocker is CAPTCHA/image/text recognition, attempt one built-in solve first. If still blocked, use `--challenge-type captcha`.
2. If the blocker is a missing password, check the workspace `.env` first. If still missing, use `--challenge-type password`.
3. If the blocker is 2FA, trigger the website challenge first and only then use `--challenge-type two_factor`.
4. In run_task/Codex environments, call the MCP tool `dowhiz_human_approval_gate_request_and_wait`. Do not use the shell CLI there.
5. Always attach the current browser screenshot(s).
6. Wait on gate result.
7. Continue only after the first reply is received and inspect that reply yourself.
8. If timeout, stop login attempts and report clearly.
9. If SMS verification is unavailable or fails, switch to another available method and keep the same gate-based wait behavior.

## Scope Rules

- `scope=admin`: when agent logs in an owner/admin account (for example Oliver's own Google/Notion/X account). Send to `admin@dowhiz.com`.
- `scope=user`: when agent logs in an end user's account. Send to that specific user email.
- For `scope=admin`, do not pass a user recipient address.

## Blocking MCP Tool

Preferred in run_task/Codex environments:

- Take the current browser screenshot(s) first.
- Call `dowhiz_human_approval_gate_request_and_wait`.
- That single tool call sends the email, waits for the first same-thread reply or timeout, and returns the full challenge state.
- While that tool call is pending, do not do any other browser or shell actions.

Example parameter shape:

```json
{
  "scope": "admin",
  "challenge_type": "two_factor",
  "page_state": "waiting_for_code_input",
  "two_factor_method": "sms",
  "verification_destination": "phone ending in 9315",
  "account_label": "Oliver Google account",
  "context": "Google has already sent the code and the page is waiting for it.",
  "screenshot": ["work/google-verify.png"],
  "timeout_minutes": 30
}
```

## CLI Quick Start

Manual fallback only. Do not use this shell CLI inside run_task/Codex environments because it bypasses the enforced blocking MCP path.

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
  --challenge-type two_factor \
  --page-state waiting_for_code_input \
  --two-factor-method sms \
  --verification-destination "phone ending in 9315" \
  --account-label "Oliver Google account" \
  --context "Google has already sent the code and the page is waiting for it." \
  --screenshot work/google-verify.png \
  --timeout-minutes 30 \
  --wait
```

For CAPTCHA after one failed solve attempt:

```bash
$HAG_CMD request \
  --scope admin \
  --challenge-type captcha \
  --account-label "Oliver Google account" \
  --context "I already tried one built-in CAPTCHA solve and the page still did not advance." \
  --screenshot work/google-captcha.png \
  --timeout-minutes 30 \
  --wait
```

For password help:

```bash
$HAG_CMD request \
  --scope admin \
  --challenge-type password \
  --page-state waiting_for_password \
  --account-label "Oliver Google account" \
  --password-env-key GOOGLE_PASSWORD \
  --password-lookup-status "Checked workspace .env for GOOGLE_PASSWORD; no value was present." \
  --screenshot work/google-password.png \
  --timeout-minutes 30 \
  --wait
```

For user-owned account:

```bash
$HAG_CMD request \
  --scope user \
  --recipient "user@example.com" \
  --challenge-type two_factor \
  --page-state waiting_for_code_input \
  --two-factor-method email \
  --verification-destination "user@example.com" \
  --account-label "User X account" \
  --screenshot work/user-login.png \
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

- In run_task/Codex environments, the shell CLI is intentionally disabled and the MCP tool is the only supported path.
- Keep waiting in this command; do not run unrelated steps while waiting.
- Reuse the same challenge thread; do not spam multiple requests unless previous one timed out.
- Never include raw credentials in outbound messages.
- Every outbound request should honestly describe the current browser state and include at least one screenshot attachment.
- Every outbound request now also writes a structured send record to `.human_approval_gate/events.jsonl` and emits a `HAG_EVENT ...` line to stderr so prod/staging task logs show attachment filenames and sizes.
- Sender identity priority is: `--from` > `HUMAN_APPROVAL_FROM` > employee mailbox from `employee.toml`/`employee.staging.toml`.
- HAG reply emails (`[HAG:...]` threads) are consumed by the gate flow and are not routed into normal Email->task execution.
