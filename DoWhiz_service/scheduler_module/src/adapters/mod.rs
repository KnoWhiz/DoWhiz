//! Channel adapters for different messaging platforms.
//!
//! This module contains implementations of the InboundAdapter and OutboundAdapter
//! traits for various messaging platforms.

pub mod bluebubbles;
pub mod discord;
pub mod google_docs;
pub mod postmark;
pub mod slack;
pub mod telegram;

pub use bluebubbles::{
    send_quick_bluebubbles_response, BlueBubblesInboundAdapter, BlueBubblesOutboundAdapter,
    BlueBubblesWebhook,
};
pub use discord::{DiscordInboundAdapter, DiscordOutboundAdapter};
pub use google_docs::{
    contains_employee_mention, extract_employee_name, format_edit_proposal, GoogleDocsComment,
    GoogleDocsInboundAdapter, GoogleDocsOutboundAdapter,
};
pub use postmark::{PostmarkInboundAdapter, PostmarkOutboundAdapter};
pub use slack::{
    is_url_verification, SlackChallengeResponse, SlackEventWrapper, SlackInboundAdapter,
    SlackMessageEvent, SlackOutboundAdapter, SlackUrlVerification,
};
pub use telegram::{
    send_quick_telegram_response, TelegramInboundAdapter, TelegramOutboundAdapter, TelegramUpdate,
};
