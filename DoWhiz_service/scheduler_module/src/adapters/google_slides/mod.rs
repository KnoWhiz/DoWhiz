//! Google Slides adapter for collaborative presentation editing via comments.
//!
//! This module provides adapters for handling messages via Google Slides comments:
//! - `GoogleSlidesInboundAdapter`: Polls for comments mentioning the employee
//! - `GoogleSlidesOutboundAdapter`: Posts replies and applies edits to presentations

mod inbound;
mod outbound;

pub use inbound::GoogleSlidesInboundAdapter;
pub use outbound::GoogleSlidesOutboundAdapter;
