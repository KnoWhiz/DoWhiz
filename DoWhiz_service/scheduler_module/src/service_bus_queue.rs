use std::collections::HashMap;
use std::env;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use azure_core::{auth::Secret, error::Error as AzureError, HttpClient};
use azure_messaging_servicebus::prelude::QueueClient;
use azure_messaging_servicebus::service_bus::{
    PeekLockResponse, SendMessageOptions, SettableBrokerProperties,
};
use tokio::runtime::Runtime;
use tracing::{debug, warn};
use uuid::Uuid;

use crate::env_alias::var_with_scale_oliver;
use crate::ingestion::IngestionEnvelope;
use crate::ingestion_queue::{EnqueueResult, IngestionQueue, IngestionQueueError, QueuedEnvelope};

#[derive(Debug, Clone)]
pub struct ServiceBusConfig {
    pub namespace: String,
    pub policy_name: String,
    pub policy_key: String,
    pub queue_name: String,
    pub peek_lock_timeout: Duration,
    pub lock_renew_interval: Duration,
}

pub struct ServiceBusIngestionQueue {
    http_client: Arc<dyn HttpClient>,
    namespace: String,
    policy_name: String,
    policy_key: String,
    queue_name: String,
    peek_lock_timeout: Duration,
    lock_renew_interval: Duration,
    runtime: Option<Runtime>,
    clients: Mutex<HashMap<String, QueueClient>>,
    pending: Mutex<HashMap<Uuid, PendingLock>>,
}

struct PendingLock {
    response: Arc<PeekLockResponse>,
    renewer: Option<LockRenewer>,
}

impl PendingLock {
    fn new(response: Arc<PeekLockResponse>, renewer: LockRenewer) -> Self {
        Self {
            response,
            renewer: Some(renewer),
        }
    }

    fn stop_renewer(&mut self) {
        if let Some(renewer) = self.renewer.take() {
            renewer.stop();
        }
    }
}

struct LockRenewer {
    stop_tx: mpsc::Sender<()>,
    handle: thread::JoinHandle<()>,
}

impl LockRenewer {
    fn stop(self) {
        let _ = self.stop_tx.send(());
        let _ = self.handle.join();
    }
}

impl ServiceBusIngestionQueue {
    pub fn from_env() -> Result<Self, IngestionQueueError> {
        let config = resolve_service_bus_config_from_env()?;
        Self::new(config)
    }

    pub fn new(config: ServiceBusConfig) -> Result<Self, IngestionQueueError> {
        let http_client = azure_core::new_http_client();
        let runtime =
            Runtime::new().map_err(|err| IngestionQueueError::ServiceBus(err.to_string()))?;
        Ok(Self {
            http_client,
            namespace: config.namespace,
            policy_name: config.policy_name,
            policy_key: config.policy_key,
            queue_name: config.queue_name,
            peek_lock_timeout: config.peek_lock_timeout,
            lock_renew_interval: config.lock_renew_interval,
            runtime: Some(runtime),
            clients: Mutex::new(HashMap::new()),
            pending: Mutex::new(HashMap::new()),
        })
    }

    fn runtime(&self) -> Result<&Runtime, IngestionQueueError> {
        self.runtime.as_ref().ok_or_else(|| {
            IngestionQueueError::ServiceBus("service bus runtime dropped".to_string())
        })
    }

    fn client_for_queue(&self, queue_name: &str) -> Result<QueueClient, IngestionQueueError> {
        let mut guard = self.clients.lock().map_err(|_| {
            IngestionQueueError::ServiceBus("queue client lock poisoned".to_string())
        })?;
        if let Some(client) = guard.get(queue_name) {
            return Ok(client.clone());
        }
        let client = QueueClient::new(
            self.http_client.clone(),
            self.namespace.clone(),
            queue_name.to_string(),
            self.policy_name.clone(),
            Secret::new(self.policy_key.clone()),
        )
        .map_err(|err| IngestionQueueError::ServiceBus(err.to_string()))?;
        guard.insert(queue_name.to_string(), client.clone());
        Ok(client)
    }

    pub fn enqueue(
        &self,
        envelope: &IngestionEnvelope,
    ) -> Result<EnqueueResult, IngestionQueueError> {
        let client = self.client_for_queue(&self.queue_name)?;
        let payload_json = serde_json::to_string(envelope).map_err(IngestionQueueError::Json)?;
        let mut broker = SettableBrokerProperties::default();
        broker.message_id = Some(envelope.dedupe_key.clone());
        let options = SendMessageOptions {
            content_type: Some("application/json".to_string()),
            broker_properties: Some(broker),
            custom_properties: None,
        };
        self.runtime()?
            .block_on(client.send_message(&payload_json, Some(options)))
            .map_err(map_service_bus_error)?;
        Ok(EnqueueResult { inserted: true })
    }

    pub fn claim_next(
        &self,
        employee_id: &str,
    ) -> Result<Option<QueuedEnvelope>, IngestionQueueError> {
        let client = self.client_for_queue(&self.queue_name)?;
        let response = self
            .runtime()?
            .block_on(client.peek_lock_message2(Some(self.peek_lock_timeout)))
            .map_err(map_service_bus_error)?;
        if *response.status() == azure_core::StatusCode::NoContent {
            return Ok(None);
        }
        if *response.status() != azure_core::StatusCode::Ok
            && *response.status() != azure_core::StatusCode::Created
        {
            return Err(IngestionQueueError::ServiceBus(format!(
                "unexpected service bus status {}",
                response.status()
            )));
        }
        let body = response.body();
        let envelope: IngestionEnvelope =
            serde_json::from_str(&body).map_err(IngestionQueueError::Json)?;
        if envelope.employee_id != employee_id {
            self.runtime()?
                .block_on(response.unlock_message())
                .map_err(map_service_bus_error)?;
            return Ok(None);
        }
        let handle_id = Uuid::new_v4();
        let response = Arc::new(response);
        let renewer = match self.spawn_lock_renewer(handle_id, response.clone()) {
            Ok(renewer) => renewer,
            Err(err) => {
                self.runtime()?
                    .block_on(response.unlock_message())
                    .map_err(map_service_bus_error)?;
                return Err(err);
            }
        };
        let mut pending = self
            .pending
            .lock()
            .map_err(|_| IngestionQueueError::ServiceBus("pending lock poisoned".to_string()))?;
        pending.insert(handle_id, PendingLock::new(response, renewer));
        Ok(Some(QueuedEnvelope {
            id: handle_id,
            envelope,
        }))
    }

    pub fn mark_done(&self, id: &Uuid) -> Result<(), IngestionQueueError> {
        let mut pending = self.take_pending(id)?;
        pending.stop_renewer();
        self.runtime()?
            .block_on(pending.response.delete_message())
            .map_err(map_service_bus_error)?;
        Ok(())
    }

    pub fn mark_failed(&self, id: &Uuid, _error: &str) -> Result<(), IngestionQueueError> {
        let mut pending = self.take_pending(id)?;
        pending.stop_renewer();
        self.runtime()?
            .block_on(pending.response.unlock_message())
            .map_err(map_service_bus_error)?;
        Ok(())
    }

    fn take_pending(&self, id: &Uuid) -> Result<PendingLock, IngestionQueueError> {
        let mut pending = self
            .pending
            .lock()
            .map_err(|_| IngestionQueueError::ServiceBus("pending lock poisoned".to_string()))?;
        pending
            .remove(id)
            .ok_or_else(|| IngestionQueueError::ServiceBus("missing pending lock".to_string()))
    }

    fn spawn_lock_renewer(
        &self,
        handle_id: Uuid,
        response: Arc<PeekLockResponse>,
    ) -> Result<LockRenewer, IngestionQueueError> {
        let runtime_handle = self.runtime()?.handle().clone();
        let renew_interval = self.lock_renew_interval;
        let queue_name = self.queue_name.clone();
        let message_id = response
            .broker_properties()
            .map(|properties| properties.message_id);
        let (stop_tx, stop_rx) = mpsc::channel::<()>();
        let thread_name = format!("sb-lock-renew-{}", &handle_id.to_string()[..8]);

        let handle = thread::Builder::new()
            .name(thread_name)
            .spawn(move || {
                debug!(
                    "service bus lock renewer started queue={} handle_id={} message_id={:?} interval_secs={}",
                    queue_name,
                    handle_id,
                    message_id,
                    renew_interval.as_secs()
                );
                loop {
                    match stop_rx.recv_timeout(renew_interval) {
                        Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                        Err(mpsc::RecvTimeoutError::Timeout) => {
                            match runtime_handle.block_on(response.renew_message_lock()) {
                                Ok(()) => {
                                    debug!(
                                        "service bus lock renewed queue={} handle_id={} message_id={:?}",
                                        queue_name, handle_id, message_id
                                    );
                                }
                                Err(err) => {
                                    warn!(
                                        "service bus lock renew failed queue={} handle_id={} message_id={:?}: {}",
                                        queue_name, handle_id, message_id, err
                                    );
                                }
                            }
                        }
                    }
                }
                debug!(
                    "service bus lock renewer stopped queue={} handle_id={} message_id={:?}",
                    queue_name, handle_id, message_id
                );
            })
            .map_err(|err| {
                IngestionQueueError::ServiceBus(format!(
                    "failed to spawn service bus lock renewer: {}",
                    err
                ))
            })?;

        Ok(LockRenewer { stop_tx, handle })
    }

    fn stop_all_pending_renewers(&self) {
        let Ok(mut pending) = self.pending.lock() else {
            return;
        };
        for (_, mut lock) in pending.drain() {
            lock.stop_renewer();
        }
    }
}

impl IngestionQueue for ServiceBusIngestionQueue {
    fn enqueue(&self, envelope: &IngestionEnvelope) -> Result<EnqueueResult, IngestionQueueError> {
        ServiceBusIngestionQueue::enqueue(self, envelope)
    }

    fn claim_next(&self, employee_id: &str) -> Result<Option<QueuedEnvelope>, IngestionQueueError> {
        ServiceBusIngestionQueue::claim_next(self, employee_id)
    }

    fn mark_done(&self, id: &Uuid) -> Result<(), IngestionQueueError> {
        ServiceBusIngestionQueue::mark_done(self, id)
    }

    fn mark_failed(&self, id: &Uuid, error: &str) -> Result<(), IngestionQueueError> {
        ServiceBusIngestionQueue::mark_failed(self, id, error)
    }
}

impl Drop for ServiceBusIngestionQueue {
    fn drop(&mut self) {
        self.stop_all_pending_renewers();
        if let Some(runtime) = self.runtime.take() {
            runtime.shutdown_background();
        }
    }
}

fn map_service_bus_error(err: AzureError) -> IngestionQueueError {
    IngestionQueueError::ServiceBus(err.to_string())
}

pub fn resolve_service_bus_config_from_env() -> Result<ServiceBusConfig, IngestionQueueError> {
    let timeout_secs = resolve_i64_env("SERVICE_BUS_PEEK_LOCK_TIMEOUT_SECS", 30);
    let renew_secs = resolve_i64_env(
        "SERVICE_BUS_LOCK_RENEW_INTERVAL_SECS",
        default_lock_renew_interval_secs(timeout_secs),
    );
    let lock_renew_interval_secs = clamp_lock_renew_interval_secs(renew_secs);

    if let Some(conn_str) = var_with_scale_oliver("SERVICE_BUS_CONNECTION_STRING") {
        let parts = parse_service_bus_connection_string(&conn_str)?;
        let queue_name = var_with_scale_oliver("SERVICE_BUS_QUEUE_NAME")
            .or(parts.entity_path)
            .ok_or_else(|| {
                IngestionQueueError::Config(
                    "missing SCALE_OLIVER_SERVICE_BUS_QUEUE_NAME/SERVICE_BUS_QUEUE_NAME"
                        .to_string(),
                )
            })?;
        return Ok(ServiceBusConfig {
            namespace: parts.namespace,
            policy_name: parts.policy_name,
            policy_key: parts.policy_key,
            queue_name,
            peek_lock_timeout: Duration::from_secs(timeout_secs as u64),
            lock_renew_interval: Duration::from_secs(lock_renew_interval_secs as u64),
        });
    }

    let namespace = var_with_scale_oliver("SERVICE_BUS_NAMESPACE").ok_or_else(|| {
        IngestionQueueError::Config(
            "missing SCALE_OLIVER_SERVICE_BUS_NAMESPACE/SERVICE_BUS_NAMESPACE".to_string(),
        )
    })?;
    let policy_name = var_with_scale_oliver("SERVICE_BUS_POLICY_NAME").ok_or_else(|| {
        IngestionQueueError::Config(
            "missing SCALE_OLIVER_SERVICE_BUS_POLICY_NAME/SERVICE_BUS_POLICY_NAME".to_string(),
        )
    })?;
    let policy_key = var_with_scale_oliver("SERVICE_BUS_POLICY_KEY").ok_or_else(|| {
        IngestionQueueError::Config(
            "missing SCALE_OLIVER_SERVICE_BUS_POLICY_KEY/SERVICE_BUS_POLICY_KEY".to_string(),
        )
    })?;
    let queue_name = var_with_scale_oliver("SERVICE_BUS_QUEUE_NAME").ok_or_else(|| {
        IngestionQueueError::Config(
            "missing SCALE_OLIVER_SERVICE_BUS_QUEUE_NAME/SERVICE_BUS_QUEUE_NAME".to_string(),
        )
    })?;
    Ok(ServiceBusConfig {
        namespace,
        policy_name,
        policy_key,
        queue_name,
        peek_lock_timeout: Duration::from_secs(timeout_secs as u64),
        lock_renew_interval: Duration::from_secs(lock_renew_interval_secs as u64),
    })
}

struct ParsedConnectionString {
    namespace: String,
    policy_name: String,
    policy_key: String,
    entity_path: Option<String>,
}

fn parse_service_bus_connection_string(
    conn_str: &str,
) -> Result<ParsedConnectionString, IngestionQueueError> {
    let mut namespace = None;
    let mut policy_name = None;
    let mut policy_key = None;
    let mut entity_path = None;
    for part in conn_str.split(';') {
        let mut iter = part.splitn(2, '=');
        let key = iter.next().unwrap_or("").trim();
        let value = iter.next().unwrap_or("").trim();
        match key {
            "Endpoint" => {
                if let Some(value) = value.strip_prefix("sb://") {
                    let value = value.trim_end_matches('/');
                    let ns = value.split('.').next().unwrap_or("").to_string();
                    if !ns.is_empty() {
                        namespace = Some(ns);
                    }
                }
            }
            "SharedAccessKeyName" => {
                if !value.is_empty() {
                    policy_name = Some(value.to_string());
                }
            }
            "SharedAccessKey" => {
                if !value.is_empty() {
                    policy_key = Some(value.to_string());
                }
            }
            "EntityPath" => {
                if !value.is_empty() {
                    entity_path = Some(value.to_string());
                }
            }
            _ => {}
        }
    }

    let namespace = namespace.ok_or_else(|| {
        IngestionQueueError::Config("missing namespace in connection string".to_string())
    })?;
    let policy_name = policy_name.ok_or_else(|| {
        IngestionQueueError::Config("missing policy name in connection string".to_string())
    })?;
    let policy_key = policy_key.ok_or_else(|| {
        IngestionQueueError::Config("missing policy key in connection string".to_string())
    })?;

    Ok(ParsedConnectionString {
        namespace,
        policy_name,
        policy_key,
        entity_path,
    })
}

fn resolve_i64_env(key: &str, default_value: i64) -> i64 {
    env::var(key)
        .ok()
        .and_then(|value| value.parse::<i64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default_value)
}

fn default_lock_renew_interval_secs(peek_lock_timeout_secs: i64) -> i64 {
    clamp_lock_renew_interval_secs(peek_lock_timeout_secs / 2)
}

fn clamp_lock_renew_interval_secs(interval_secs: i64) -> i64 {
    interval_secs.clamp(5, 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_lock_renew_interval_is_half_peek_lock_timeout() {
        assert_eq!(default_lock_renew_interval_secs(30), 15);
        assert_eq!(default_lock_renew_interval_secs(120), 60);
    }

    #[test]
    fn clamp_lock_renew_interval_enforces_bounds() {
        assert_eq!(clamp_lock_renew_interval_secs(1), 5);
        assert_eq!(clamp_lock_renew_interval_secs(5), 5);
        assert_eq!(clamp_lock_renew_interval_secs(22), 22);
        assert_eq!(clamp_lock_renew_interval_secs(500), 60);
    }

    #[test]
    fn parse_connection_string_extracts_required_parts() {
        let parsed = parse_service_bus_connection_string(
            "Endpoint=sb://my-namespace.servicebus.windows.net/;\
             SharedAccessKeyName=my-policy;\
             SharedAccessKey=top-secret;\
             EntityPath=ingestion-little_bear",
        )
        .expect("parse connection string");

        assert_eq!(parsed.namespace, "my-namespace");
        assert_eq!(parsed.policy_name, "my-policy");
        assert_eq!(parsed.policy_key, "top-secret");
        assert_eq!(parsed.entity_path.as_deref(), Some("ingestion-little_bear"));
    }
}
