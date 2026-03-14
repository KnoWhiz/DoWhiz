#!/usr/bin/env python3
"""Blocking MCP wrapper for the human approval gate."""

import argparse
from typing import Any, Dict, List, Literal, Optional

from mcp.server.fastmcp import FastMCP
from pydantic import BaseModel, ConfigDict, Field

import human_approval_gate as hag


mcp = FastMCP("human_approval_gate_mcp")


class BlockingHumanApprovalGateInput(BaseModel):
    """Input payload for the blocking human approval gate tool."""

    model_config = ConfigDict(str_strip_whitespace=True, extra="forbid")

    scope: Literal["admin", "user"] = Field(
        default="user",
        description="Who should receive the HAG email.",
    )
    challenge_type: Literal["captcha", "password", "two_factor"] = Field(
        default="two_factor",
        description="Type of blocker currently shown in the browser.",
    )
    screenshot: List[str] = Field(
        ...,
        min_length=1,
        description="Absolute or workspace-relative browser screenshot paths to attach.",
    )
    page_state: str = Field(
        default="",
        description="Explicit page state. Required for two_factor challenges.",
    )
    account_label: str = Field(
        default="",
        description="Human-readable account context, for example 'Oliver Google account'.",
    )
    context: str = Field(
        default="",
        description="Additional honest context about the current blocker.",
    )
    action_text: str = Field(
        default="",
        description="Optional custom ask shown in the HAG email.",
    )
    challenge_id: str = Field(
        default="",
        description="Optional existing challenge id. Leave blank to generate one.",
    )
    recipient: str = Field(
        default="",
        description="Explicit recipient for user scope. Leave blank for admin scope.",
    )
    user_email: str = Field(
        default="",
        description="Fallback user email for user scope when recipient is omitted.",
    )
    from_address: str = Field(
        default="",
        description="Optional sender override.",
    )
    reply_to: str = Field(
        default="",
        description="Optional reply-to override.",
    )
    expected_reply_from: str = Field(
        default="",
        description="Optional email address expected to reply in-thread.",
    )
    two_factor_method: str = Field(
        default="",
        description="For two_factor only: sms, email, auth_app, device_tap, or other.",
    )
    verification_destination: str = Field(
        default="",
        description="Masked phone/email/device label shown by the site.",
    )
    password_env_key: str = Field(
        default=hag.DEFAULT_PASSWORD_ENV_KEY,
        description="Env key that was checked before asking a human for password help.",
    )
    password_lookup_status: str = Field(
        default="",
        description="Honest description of what password lookup was already attempted.",
    )
    timeout_minutes: int = Field(
        default=hag.DEFAULT_TIMEOUT_MINUTES,
        ge=1,
        description="Max wait window communicated to the human recipient.",
    )
    wait_timeout_minutes: Optional[int] = Field(
        default=None,
        ge=1,
        description="Optional shorter local timeout for this tool call.",
    )
    poll_interval_seconds: int = Field(
        default=hag.DEFAULT_POLL_SECONDS,
        ge=1,
        description="Polling interval while waiting for the reply.",
    )
    state_dir: str = Field(
        default=hag.STATE_DIR_DEFAULT,
        description="Challenge state directory inside the current workspace.",
    )
    api_base: str = Field(
        default_factory=lambda: hag.get_env_first("POSTMARK_API_BASE_URL") or hag.API_BASE_DEFAULT,
        description="Postmark API base URL override.",
    )
    dry_run: bool = Field(
        default=False,
        description="If true, writes challenge state without sending email or waiting.",
    )


def build_request_namespace(params: BlockingHumanApprovalGateInput) -> argparse.Namespace:
    return argparse.Namespace(
        api_base=params.api_base,
        state_dir=params.state_dir,
        token="",
        scope=params.scope,
        challenge_type=params.challenge_type,
        challenge_id=params.challenge_id,
        recipient=params.recipient,
        user_email=params.user_email,
        from_address=params.from_address,
        reply_to=params.reply_to,
        expected_reply_from=params.expected_reply_from,
        account_label=params.account_label,
        action_text=params.action_text,
        context=params.context,
        page_state=params.page_state,
        two_factor_method=params.two_factor_method,
        verification_destination=params.verification_destination,
        password_env_key=params.password_env_key,
        password_lookup_status=params.password_lookup_status,
        screenshot=list(params.screenshot),
        timeout_minutes=params.timeout_minutes,
        wait=not params.dry_run,
        wait_timeout_minutes=params.wait_timeout_minutes,
        poll_interval_seconds=params.poll_interval_seconds,
        dry_run=params.dry_run,
    )


def execute_blocking_hag_request(params: BlockingHumanApprovalGateInput) -> Dict[str, Any]:
    state, _exit_code = hag.execute_request(build_request_namespace(params))
    return state


@mcp.tool(
    name=hag.HAG_BLOCKING_MCP_TOOL_NAME,
    annotations={
        "title": "DoWhiz Human Approval Gate",
        "readOnlyHint": False,
        "destructiveHint": False,
        "idempotentHint": False,
        "openWorldHint": True,
    },
)
def dowhiz_human_approval_gate_request_and_wait(
    params: BlockingHumanApprovalGateInput,
) -> Dict[str, Any]:
    """Send a HAG email with screenshots and block until reply or timeout.

    Use this only after the website has already reached the exact blocker screen.
    The call is synchronous: while it is waiting, Codex should not continue with
    other browser or shell actions.
    """

    return execute_blocking_hag_request(params)


if __name__ == "__main__":
    mcp.run()
