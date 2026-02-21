//! Google Sheets adapter for collaborative spreadsheet editing via comments.
//!
//! This module provides adapters for handling messages via Google Sheets comments:
//! - `GoogleSheetsInboundAdapter`: Polls for comments mentioning the employee
//! - `GoogleSheetsOutboundAdapter`: Posts replies and applies edits to spreadsheets

mod inbound;
mod outbound;

pub use inbound::GoogleSheetsInboundAdapter;
pub use outbound::GoogleSheetsOutboundAdapter;
