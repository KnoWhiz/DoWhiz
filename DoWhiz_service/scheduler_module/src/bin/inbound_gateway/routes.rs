use std::collections::HashMap;

use scheduler_module::channel::Channel;
use uuid::Uuid;

use super::config::GatewayRouteConfig;
use super::state::{GatewayState, RouteDecision, RouteKey, RouteTarget};

pub(super) fn resolve_route(
    channel: Channel,
    route_key: &str,
    state: &GatewayState,
) -> Option<RouteDecision> {
    let normalized_key = normalize_route_key(channel, route_key);
    let key = RouteKey {
        channel,
        key: normalized_key.clone(),
    };

    let target = state.config.routes.get(&key).cloned().or_else(|| {
        state
            .config
            .channel_defaults
            .get(&channel)
            .cloned()
            .or_else(|| {
                if channel == Channel::Email {
                    state
                        .address_to_employee
                        .get(&normalized_key)
                        .map(|employee_id| RouteTarget {
                            employee_id: employee_id.clone(),
                            tenant_id: state.config.defaults.tenant_id.clone(),
                        })
                } else {
                    None
                }
            })
            .or_else(|| {
                state
                    .config
                    .defaults
                    .employee_id
                    .as_ref()
                    .map(|employee_id| RouteTarget {
                        employee_id: employee_id.clone(),
                        tenant_id: state.config.defaults.tenant_id.clone(),
                    })
            })
    })?;

    let tenant_id = target
        .tenant_id
        .clone()
        .or_else(|| state.config.defaults.tenant_id.clone())
        .unwrap_or_else(|| "default".to_string());

    Some(RouteDecision {
        tenant_id,
        employee_id: target.employee_id,
    })
}

pub(super) fn build_dedupe_key(
    tenant_id: &str,
    employee_id: &str,
    channel: Channel,
    external_message_id: Option<&str>,
    raw_payload: &[u8],
) -> String {
    let base = if let Some(id) = external_message_id {
        id.to_string()
    } else if !raw_payload.is_empty() {
        format!("{:x}", md5::compute(raw_payload))
    } else {
        Uuid::new_v4().to_string()
    };
    format!("{}:{}:{}:{}", tenant_id, employee_id, channel, base)
}

pub(super) fn normalize_routes(
    routes: &[GatewayRouteConfig],
) -> Result<
    (
        HashMap<RouteKey, RouteTarget>,
        HashMap<Channel, RouteTarget>,
    ),
    String,
> {
    let mut map = HashMap::new();
    let mut defaults = HashMap::new();

    for route in routes {
        let channel: Channel = route
            .channel
            .parse()
            .map_err(|err| format!("invalid route channel {}: {}", route.channel, err))?;
        let key = normalize_route_key(channel, route.key.trim());
        if key.is_empty() {
            return Err("route key cannot be empty".to_string());
        }
        let target = RouteTarget {
            tenant_id: route.tenant_id.clone(),
            employee_id: route.employee_id.clone(),
        };
        if key == "*" {
            defaults.insert(channel, target);
        } else {
            let route_key = RouteKey { channel, key };
            map.insert(route_key, target);
        }
    }

    Ok((map, defaults))
}

pub(super) fn normalize_route_key(channel: Channel, key: &str) -> String {
    let trimmed = key.trim();
    if trimmed == "*" {
        return "*".to_string();
    }
    match channel {
        Channel::Email => normalize_email(trimmed),
        Channel::Sms => normalize_phone_number(trimmed),
        _ => trimmed.to_string(),
    }
}

pub(super) fn normalize_email(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

pub(super) fn normalize_phone_number(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_digit() || *ch == '+')
        .collect::<String>()
}
