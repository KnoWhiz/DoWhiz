//! Google Docs adapter for collaborative document editing via comments.
//!
//! This module provides adapters for handling messages via Google Docs comments:
//! - `GoogleDocsInboundAdapter`: Polls for comments mentioning the employee
//! - `GoogleDocsOutboundAdapter`: Posts replies and applies edits to documents

mod formatting;
mod inbound;
mod mentions;
mod models;
mod outbound;

pub use formatting::format_edit_proposal;
pub use inbound::GoogleDocsInboundAdapter;
pub use mentions::{contains_employee_mention, extract_employee_name};
pub use models::{ActionableComment, GoogleDocsComment};
pub use outbound::GoogleDocsOutboundAdapter;
