use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use scheduler_module::adapters::discord::DiscordInboundAdapter;
use scheduler_module::channel::Channel;
use tracing::{error, info, warn};

use super::handlers::build_envelope;
use super::routes::resolve_route;
use super::state::GatewayState;

pub(super) async fn spawn_discord_gateway(state: Arc<GatewayState>) {
    let token = match env::var("DISCORD_BOT_TOKEN") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => return,
    };

    let bot_user_id = env::var("DISCORD_BOT_USER_ID")
        .ok()
        .and_then(|value| value.parse::<u64>().ok());

    let mut bot_user_ids = HashSet::new();
    if let Some(id) = bot_user_id {
        bot_user_ids.insert(id);
    }

    let handler_state = DiscordIngressState {
        state: state.clone(),
        adapter: DiscordInboundAdapter::new(bot_user_ids.clone()),
        bot_user_ids,
    };

    tokio::spawn(async move {
        if let Err(err) = run_discord_gateway(token, handler_state).await {
            error!("discord gateway error: {}", err);
        }
    });
}

struct DiscordIngressState {
    state: Arc<GatewayState>,
    adapter: DiscordInboundAdapter,
    bot_user_ids: HashSet<u64>,
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

        if !is_mention && !is_reply_to_bot {
            return;
        }

        let guild_id = inbound
            .metadata
            .discord_guild_id
            .map(|value| value.to_string())
            .unwrap_or_else(|| "dm".to_string());

        let route_key = guild_id.clone();
        let Some(route) = resolve_route(Channel::Discord, &route_key, &self.inner.state) else {
            info!("gateway no route for discord guild_id={}", route_key);
            return;
        };

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
