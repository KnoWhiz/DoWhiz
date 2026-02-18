use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use scheduler_module::adapters::postmark::PostmarkInboundPayload;
use scheduler_module::channel::Channel;
use scheduler_module::employee_config::EmployeeDirectory;
use scheduler_module::ingestion_queue::IngestionQueue;
use scheduler_module::mailbox;

use super::config::GatewayDefaultsConfig;

#[derive(Clone)]
pub(super) struct GatewayConfig {
    pub(super) defaults: GatewayDefaultsConfig,
    pub(super) routes: HashMap<RouteKey, RouteTarget>,
    pub(super) channel_defaults: HashMap<Channel, RouteTarget>,
}

#[derive(Clone)]
pub(super) struct GatewayState {
    pub(super) config: GatewayConfig,
    pub(super) employee_directory: EmployeeDirectory,
    pub(super) address_to_employee: HashMap<String, String>,
    pub(super) queue: Arc<dyn IngestionQueue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct RouteKey {
    pub(super) channel: Channel,
    pub(super) key: String,
}

#[derive(Debug, Clone)]
pub(super) struct RouteTarget {
    pub(super) tenant_id: Option<String>,
    pub(super) employee_id: String,
}

#[derive(Debug, Clone)]
pub(super) struct RouteDecision {
    pub(super) tenant_id: String,
    pub(super) employee_id: String,
}

pub(super) fn build_address_map(directory: &EmployeeDirectory) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for employee in &directory.employees {
        for address in &employee.address_set {
            if let Some(prev) = map.insert(address.clone(), employee.id.clone()) {
                tracing::warn!(
                    "gateway duplicate address mapping: {} ({} -> {})",
                    address,
                    prev,
                    employee.id
                );
            }
        }
    }
    map
}

pub(super) fn find_service_address(
    payload: &PostmarkInboundPayload,
    service_addresses: &HashSet<String>,
) -> Option<String> {
    let candidates = collect_service_address_candidates(payload);
    let mailbox = mailbox::select_inbound_service_mailbox(&candidates, service_addresses);
    mailbox.map(|value| value.address)
}

pub(super) fn collect_service_address_candidates(
    payload: &PostmarkInboundPayload,
) -> Vec<Option<&str>> {
    let mut candidates = Vec::new();
    if let Some(value) = payload.to.as_deref() {
        candidates.push(Some(value));
    }
    if let Some(value) = payload.cc.as_deref() {
        candidates.push(Some(value));
    }
    if let Some(value) = payload.bcc.as_deref() {
        candidates.push(Some(value));
    }
    if let Some(list) = payload.to_full.as_ref() {
        for entry in list {
            candidates.push(Some(entry.email.as_str()));
        }
    }
    if let Some(list) = payload.cc_full.as_ref() {
        for entry in list {
            candidates.push(Some(entry.email.as_str()));
        }
    }
    if let Some(list) = payload.bcc_full.as_ref() {
        for entry in list {
            candidates.push(Some(entry.email.as_str()));
        }
    }
    for header in [
        "X-Original-To",
        "Delivered-To",
        "Envelope-To",
        "X-Envelope-To",
        "X-Forwarded-To",
        "X-Original-Recipient",
        "Original-Recipient",
    ] {
        for value in payload.header_values(header) {
            candidates.push(Some(value));
        }
    }
    candidates
}
