mod bluebubbles;
mod discord;
mod discord_context;
mod google_workspace;
mod notion;
mod notion_email;
mod quick_responses;
mod slack;
mod sms;
mod telegram;
mod wechat;
mod whatsapp;

pub(super) use bluebubbles::process_bluebubbles_event;
pub(crate) use discord::hydrate_discord_attachments;
pub(crate) use discord::persist_discord_ingest_context;
pub(super) use discord::process_discord_inbound_message;
pub(crate) use discord_context::build_discord_message_text_with_quote;
pub(crate) use discord_context::build_discord_router_context;
pub(crate) use discord_context::hydrate_discord_context_files;
pub(super) use google_workspace::process_google_workspace_message;
pub(super) use notion::process_notion_message;
pub(super) use notion_email::process_notion_email;
pub(super) use quick_responses::{
    try_quick_response_bluebubbles, try_quick_response_discord,
    try_quick_response_google_workspace, try_quick_response_slack, try_quick_response_telegram,
    try_quick_response_wechat, try_quick_response_whatsapp,
};
pub(super) use slack::process_slack_event;
pub(super) use sms::process_sms_message;
pub(super) use telegram::process_telegram_event;
pub(super) use wechat::process_wechat_event;
pub(super) use whatsapp::process_whatsapp_event;
