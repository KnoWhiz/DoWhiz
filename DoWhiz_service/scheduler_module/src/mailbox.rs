use crate::user_store::extract_emails;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceMailbox {
    pub address: String,
    pub display_name: Option<String>,
}

impl ServiceMailbox {
    pub fn formatted(&self) -> String {
        match self
            .display_name
            .as_deref()
            .map(str::trim)
            .filter(|name| !name.is_empty())
        {
            Some(name) => format!("{} <{}>", name, self.address),
            None => self.address.clone(),
        }
    }
}

pub fn is_service_address(address: &str, service_addresses: &HashSet<String>) -> bool {
    service_addresses.contains(&normalize_address(address))
}

pub fn select_inbound_service_mailbox(
    raws: &[Option<&str>],
    service_addresses: &HashSet<String>,
) -> Option<ServiceMailbox> {
    for raw in raws.iter().copied().flatten() {
        for email in extract_emails(raw) {
            if is_service_address(&email, service_addresses) {
                return Some(ServiceMailbox {
                    address: email.clone(),
                    display_name: display_name_for_address(raw, &email),
                });
            }
        }
    }
    None
}

fn normalize_address(address: &str) -> String {
    address.trim().to_ascii_lowercase()
}

fn display_name_for_address(raw: &str, address: &str) -> Option<String> {
    let address_lower = address.to_ascii_lowercase();
    let segment = raw
        .split(',')
        .find(|segment| segment.to_ascii_lowercase().contains(&address_lower))
        .unwrap_or(raw);
    if !segment.contains('<') {
        return None;
    }
    let name = segment
        .split('<')
        .next()
        .unwrap_or("")
        .trim()
        .trim_matches('"')
        .trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn service_addresses() -> HashSet<String> {
        ["oliver@dowhiz.com", "mini-mouse@dowhiz.com"]
            .iter()
            .map(|value| value.to_string())
            .collect()
    }

    #[test]
    fn selects_service_mailbox_with_display_name() {
        let raws = &[Some("Oliver Bear <oliver@dowhiz.com>")];
        let mailbox = select_inbound_service_mailbox(raws, &service_addresses())
            .expect("mailbox");
        assert_eq!(mailbox.address, "oliver@dowhiz.com");
        assert_eq!(mailbox.display_name.as_deref(), Some("Oliver Bear"));
        assert_eq!(mailbox.formatted(), "Oliver Bear <oliver@dowhiz.com>");
    }

    #[test]
    fn selects_service_mailbox_without_display_name() {
        let raws = &[Some("mini-mouse@dowhiz.com")];
        let mailbox = select_inbound_service_mailbox(raws, &service_addresses())
            .expect("mailbox");
        assert_eq!(mailbox.address, "mini-mouse@dowhiz.com");
        assert_eq!(mailbox.display_name, None);
        assert_eq!(mailbox.formatted(), "mini-mouse@dowhiz.com");
    }

    #[test]
    fn rejects_non_service_address() {
        let raws = &[Some("user@example.com")];
        let mailbox = select_inbound_service_mailbox(raws, &service_addresses());
        assert!(mailbox.is_none());
    }
}
