//! Channel adapters for different messaging platforms.
//!
//! This module contains implementations of the InboundAdapter and OutboundAdapter
//! traits for various messaging platforms.

pub mod bluebubbles;
pub mod discord;
pub mod google_common;
pub mod google_docs;
pub mod google_sheets;
pub mod google_slides;
pub mod postmark;
pub mod slack;
pub mod telegram;
pub mod whatsapp;

pub use bluebubbles::{
    send_quick_bluebubbles_response, BlueBubblesInboundAdapter, BlueBubblesOutboundAdapter,
    BlueBubblesWebhook,
};
pub use discord::{DiscordInboundAdapter, DiscordOutboundAdapter};
pub use google_common::{
    ActionableComment, GoogleComment, GoogleCommentsClient, GoogleFileType,
};
pub use google_docs::{
    contains_employee_mention, extract_employee_name, format_edit_proposal, GoogleDocsComment,
    GoogleDocsInboundAdapter, GoogleDocsOutboundAdapter,
};
pub use google_sheets::{GoogleSheetsInboundAdapter, GoogleSheetsOutboundAdapter};
pub use google_slides::{GoogleSlidesInboundAdapter, GoogleSlidesOutboundAdapter};
pub use postmark::{PostmarkInboundAdapter, PostmarkOutboundAdapter};
pub use slack::{
    is_url_verification, SlackChallengeResponse, SlackEventWrapper, SlackInboundAdapter,
    SlackMessageEvent, SlackOutboundAdapter, SlackUrlVerification,
};
pub use telegram::{
    send_quick_telegram_response, TelegramInboundAdapter, TelegramOutboundAdapter, TelegramUpdate,
};
pub use whatsapp::{
    send_quick_whatsapp_response, WhatsAppInboundAdapter, WhatsAppOutboundAdapter, WhatsAppWebhook,
};
