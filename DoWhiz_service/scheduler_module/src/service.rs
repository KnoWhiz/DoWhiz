mod config;
mod email;
mod html;
mod inbound;
mod ingestion;
mod postmark;
mod recipients;
mod scheduler;
mod server;
mod state;
mod workspace;

pub(crate) type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub use crate::thread_state::{bump_thread_state, default_thread_state_path};

pub use config::{ServiceConfig, DEFAULT_INBOUND_BODY_MAX_BYTES};
pub use email::{process_inbound_payload, PostmarkInbound};
pub use scheduler::cancel_pending_thread_tasks;
pub use server::run_server;
pub use workspace::copy_dir_recursive;

pub(crate) use config::{default_employee_config_path, resolve_telegram_bot_token};
