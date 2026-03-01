use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use scheduler_module::adapters::discord::DiscordInboundAdapter;
use scheduler_module::channel::Channel;
use tracing::{error, info, warn};

use super::handlers::build_envelope;
use super::state::{GatewayConfig, GatewayState, RouteDecision};

/// Configuration for a single employee's Discord bot.
struct EmployeeDiscordConfig {
    employee_id: String,
    token: String,
    bot_user_id: Option<u64>,
}

/// Known employee Discord configurations.
/// Each entry: (employee_id, token_env_key, user_id_env_key)
const EMPLOYEE_DISCORD_CONFIGS: &[(&str, &str, &str)] = &[
    (
        "boiled_egg",
        "BOILED_EGG_DISCORD_BOT_TOKEN",
        "BOILED_EGG_DISCORD_BOT_USER_ID",
    ),
    (
        "little_bear",
        "LITTLE_BEAR_DISCORD_BOT_TOKEN",
        "LITTLE_BEAR_DISCORD_BOT_USER_ID",
    ),
];

/// Collect all valid employee Discord configurations from environment.
fn collect_employee_discord_configs() -> Vec<EmployeeDiscordConfig> {
    let mut configs = Vec::new();

    for (employee_id, token_key, user_id_key) in EMPLOYEE_DISCORD_CONFIGS {
        if let Ok(token) = env::var(token_key) {
            if !token.trim().is_empty() {
                let bot_user_id = env::var(user_id_key)
                    .ok()
                    .and_then(|v| v.parse::<u64>().ok());
                configs.push(EmployeeDiscordConfig {
                    employee_id: employee_id.to_string(),
                    token,
                    bot_user_id,
                });
                info!(
                    "discord gateway: found config for employee {} (bot_user_id={:?})",
                    employee_id, bot_user_id
                );
            }
        }
    }

    // Fallback to legacy single-bot config if no employee configs found
    if configs.is_empty() {
        if let Ok(token) = env::var("DISCORD_BOT_TOKEN") {
            if !token.trim().is_empty() {
                let bot_user_id = env::var("DISCORD_BOT_USER_ID")
                    .ok()
                    .and_then(|v| v.parse::<u64>().ok());
                let default_employee =
                    env::var("EMPLOYEE_ID").unwrap_or_else(|_| "boiled_egg".to_string());
                configs.push(EmployeeDiscordConfig {
                    employee_id: default_employee.clone(),
                    token,
                    bot_user_id,
                });
                info!(
                    "discord gateway: using legacy config for employee {}",
                    default_employee
                );
            }
        }
    }

    configs
}

fn has_discord_routes(config: &GatewayConfig) -> bool {
    config.channel_defaults.contains_key(&Channel::Discord)
        || config
            .routes
            .keys()
            .any(|route| route.channel == Channel::Discord)
}

fn should_enqueue_discord_message(
    is_direct_message: bool,
    is_mention: bool,
    is_reply_to_bot: bool,
) -> bool {
    is_direct_message || is_mention || is_reply_to_bot
}

pub(super) async fn spawn_discord_gateway(state: Arc<GatewayState>) {
    if !has_discord_routes(&state.config) {
        info!("discord gateway: no Discord routes configured, skipping");
        return;
    }

    let configs = collect_employee_discord_configs();

    if configs.is_empty() {
        info!("discord gateway: no Discord bot tokens configured, skipping");
        return;
    }

    let default_tenant_id = state
        .config
        .defaults
        .tenant_id
        .clone()
        .unwrap_or_else(|| "default".to_string());

    for config in configs {
        let employee_id = config.employee_id.clone();
        let token = config.token.clone();
        let tenant_id = default_tenant_id.clone();

        let mut bot_user_ids = HashSet::new();
        if let Some(id) = config.bot_user_id {
            bot_user_ids.insert(id);
        }

        let handler_state = DiscordIngressState {
            state: state.clone(),
            adapter: DiscordInboundAdapter::new(bot_user_ids.clone()),
            bot_user_ids,
            employee_id: employee_id.clone(),
            tenant_id,
        };

        tokio::spawn(async move {
            info!(
                "discord gateway: starting client for employee {}",
                employee_id
            );
            if let Err(err) = run_discord_gateway(token, handler_state).await {
                error!(
                    "discord gateway error for employee {}: {}",
                    employee_id, err
                );
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::config::GatewayDefaultsConfig;
    use crate::state::{GatewayConfig, RouteKey, RouteTarget};

    fn mk_config(routes: Vec<RouteKey>, defaults: Vec<Channel>) -> GatewayConfig {
        let mut route_map: HashMap<RouteKey, RouteTarget> = HashMap::new();
        for route in routes {
            route_map.insert(
                route,
                RouteTarget {
                    tenant_id: Some("staging".to_string()),
                    employee_id: "little_bear".to_string(),
                },
            );
        }

        let mut channel_defaults: HashMap<Channel, RouteTarget> = HashMap::new();
        for channel in defaults {
            channel_defaults.insert(
                channel,
                RouteTarget {
                    tenant_id: Some("staging".to_string()),
                    employee_id: "little_bear".to_string(),
                },
            );
        }

        GatewayConfig {
            defaults: GatewayDefaultsConfig::default(),
            routes: route_map,
            channel_defaults,
        }
    }

    #[test]
    fn has_discord_routes_false_when_not_configured() {
        let config = mk_config(vec![], vec![]);
        assert!(!has_discord_routes(&config));
    }

    #[test]
    fn has_discord_routes_true_when_default_exists() {
        let config = mk_config(vec![], vec![Channel::Discord]);
        assert!(has_discord_routes(&config));
    }

    #[test]
    fn has_discord_routes_true_when_explicit_route_exists() {
        let config = mk_config(
            vec![RouteKey {
                channel: Channel::Discord,
                key: "U12345".to_string(),
            }],
            vec![],
        );

        assert!(has_discord_routes(&config));
    }

    #[test]
    fn should_enqueue_discord_message_allows_direct_messages_without_mentions() {
        assert!(should_enqueue_discord_message(true, false, false));
    }

    #[test]
    fn should_enqueue_discord_message_requires_signal_in_guild_channels() {
        assert!(!should_enqueue_discord_message(false, false, false));
        assert!(should_enqueue_discord_message(false, true, false));
        assert!(should_enqueue_discord_message(false, false, true));
    }
}

struct DiscordIngressState {
    state: Arc<GatewayState>,
    adapter: DiscordInboundAdapter,
    bot_user_ids: HashSet<u64>,
    employee_id: String,
    tenant_id: String,
}

struct DiscordIngressHandler {
    inner: Arc<DiscordIngressState>,
}

#[serenity::async_trait]
impl serenity::all::EventHandler for DiscordIngressHandler {
    async fn ready(&self, _ctx: serenity::all::Context, ready: serenity::all::Ready) {
        info!("Discord bot connected as {}", ready.user.name);
    }

    async fn message(&self, _ctx: serenity::all::Context, msg: serenity::all::Message) {
        let inbound = match self.inner.adapter.from_serenity_message(&msg) {
            Ok(message) => message,
            Err(err) => {
                if !err.to_string().contains("ignoring bot") {
                    warn!("gateway discord parse error: {}", err);
                }
                return;
            }
        };

        let is_mention = msg
            .mentions
            .iter()
            .any(|u| self.inner.bot_user_ids.contains(&u.id.get()));
        let is_reply_to_bot = msg
            .referenced_message
            .as_ref()
            .map(|ref_msg| self.inner.bot_user_ids.contains(&ref_msg.author.id.get()))
            .unwrap_or(false);
        let is_direct_message = msg.guild_id.is_none();

        // In DMs, users typically send plain text without mentioning the bot.
        // Mention/reply gating is only needed in guild channels.
        if !should_enqueue_discord_message(is_direct_message, is_mention, is_reply_to_bot) {
            return;
        }

        // Use the employee_id from this handler's state (each bot client knows its employee)
        let route = RouteDecision {
            tenant_id: self.inner.tenant_id.clone(),
            employee_id: self.inner.employee_id.clone(),
        };

        info!(
            "discord gateway routing message to employee={} (dm={}, mention={}, reply_to_bot={})",
            self.inner.employee_id,
            is_direct_message,
            is_mention,
            is_reply_to_bot
        );

        let external_message_id = inbound.message_id.clone();
        let envelope = match build_envelope(
            route,
            Channel::Discord,
            external_message_id,
            &inbound,
            &inbound.raw_payload,
        )
        .await
        {
            Ok(envelope) => envelope,
            Err(err) => {
                error!("gateway failed to store raw payload: {}", err);
                return;
            }
        };

        let envelope_id = envelope.envelope_id;
        let dedupe_key = envelope.dedupe_key.clone();
        let queue = self.inner.state.queue.clone();
        match tokio::task::spawn_blocking(move || queue.enqueue(&envelope)).await {
            Ok(Ok(result)) => {
                if result.inserted {
                    info!("gateway enqueued discord message {}", envelope_id);
                } else {
                    info!("gateway duplicate discord message {}", dedupe_key);
                }
            }
            Ok(Err(err)) => {
                error!("gateway discord enqueue error: {}", err);
            }
            Err(err) => {
                error!("gateway discord enqueue join error: {}", err);
            }
        }
    }
}

async fn run_discord_gateway(
    token: String,
    state: DiscordIngressState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let intents = serenity::all::GatewayIntents::GUILD_MESSAGES
        | serenity::all::GatewayIntents::DIRECT_MESSAGES
        | serenity::all::GatewayIntents::MESSAGE_CONTENT;

    let handler = DiscordIngressHandler {
        inner: Arc::new(state),
    };

    let mut client = serenity::Client::builder(&token, intents)
        .event_handler(handler)
        .await?;

    info!("Starting Discord Gateway client...");
    client.start().await?;
    Ok(())
}
