use crate::user_store::extract_emails;

const LITTLE_BEAR_ADDRESSES: &[&str] = &[
    "little-bear@dowhiz.com",
    "agent@dowhiz.com",
    "oliver@dowhiz.com",
];
const MINI_MOUSE_ADDRESSES: &[&str] = &["mini-mouse@dowhiz.com", "maggie@dowhiz.com"];

const LITTLE_BEAR_SOUL_BLOCK: &str = r#"<SOUL>
Your name is Oliver, a little bear, who is cute and smart and capable. You always get task done.
Go bears!
</SOUL>
"#;

const MINI_MOUSE_SOUL_BLOCK: &str = r#"<SOUL>
Your name is Mini-Mouse, a tiny mouse, who is curious and quick and capable. You always get task done.
Go mice!
</SOUL>
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspacePersona {
    LittleBear,
    MiniMouse,
}

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

pub fn persona_for_address(address: Option<&str>) -> WorkspacePersona {
    match address {
        Some(value) if is_mini_mouse_address(value) => WorkspacePersona::MiniMouse,
        _ => WorkspacePersona::LittleBear,
    }
}

pub fn soul_block(persona: WorkspacePersona) -> &'static str {
    match persona {
        WorkspacePersona::LittleBear => LITTLE_BEAR_SOUL_BLOCK,
        WorkspacePersona::MiniMouse => MINI_MOUSE_SOUL_BLOCK,
    }
}

pub fn is_service_address(address: &str) -> bool {
    is_little_bear_address(address) || is_mini_mouse_address(address)
}

pub fn select_inbound_service_address(raws: &[Option<&str>]) -> Option<String> {
    select_inbound_service_mailbox(raws).map(|mailbox| mailbox.address)
}

pub fn select_inbound_service_mailbox(raws: &[Option<&str>]) -> Option<ServiceMailbox> {
    for raw in raws.iter().copied().flatten() {
        for email in extract_emails(raw) {
            if is_service_address(&email) {
                return Some(ServiceMailbox {
                    address: email.clone(),
                    display_name: display_name_for_address(raw, &email),
                });
            }
        }
    }
    None
}

fn is_little_bear_address(address: &str) -> bool {
    LITTLE_BEAR_ADDRESSES
        .iter()
        .any(|candidate| candidate.eq_ignore_ascii_case(address))
}

fn is_mini_mouse_address(address: &str) -> bool {
    MINI_MOUSE_ADDRESSES
        .iter()
        .any(|candidate| candidate.eq_ignore_ascii_case(address))
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

    #[test]
    fn selects_service_mailbox_with_display_name() {
        let raws = &[Some("Oliver Bear <oliver@dowhiz.com>")];
        let mailbox = select_inbound_service_mailbox(raws).expect("mailbox");
        assert_eq!(mailbox.address, "oliver@dowhiz.com");
        assert_eq!(mailbox.display_name.as_deref(), Some("Oliver Bear"));
        assert_eq!(mailbox.formatted(), "Oliver Bear <oliver@dowhiz.com>");
    }

    #[test]
    fn selects_service_mailbox_without_display_name() {
        let raws = &[Some("mini-mouse@dowhiz.com")];
        let mailbox = select_inbound_service_mailbox(raws).expect("mailbox");
        assert_eq!(mailbox.address, "mini-mouse@dowhiz.com");
        assert_eq!(mailbox.display_name, None);
        assert_eq!(mailbox.formatted(), "mini-mouse@dowhiz.com");
    }

    #[test]
    fn persona_for_address_defaults_to_little_bear() {
        assert_eq!(persona_for_address(None), WorkspacePersona::LittleBear);
        assert_eq!(
            persona_for_address(Some("little-bear@dowhiz.com")),
            WorkspacePersona::LittleBear
        );
        assert_eq!(
            persona_for_address(Some("mini-mouse@dowhiz.com")),
            WorkspacePersona::MiniMouse
        );
    }
}
