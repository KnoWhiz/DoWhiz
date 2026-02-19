use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use azure_core::{auth::Secret, error::Error as AzureError, HttpClient};
use azure_messaging_servicebus::prelude::QueueClient;
use azure_messaging_servicebus::service_bus::{
    PeekLockResponse, SendMessageOptions, SettableBrokerProperties,
};
use tokio::runtime::Runtime;
use uuid::Uuid;

use crate::ingestion::IngestionEnvelope;
use crate::ingestion_queue::{EnqueueResult, IngestionQueue, IngestionQueueError, QueuedEnvelope};

#[derive(Debug, Clone)]
pub struct ServiceBusConfig {
    pub namespace: String,
    pub policy_name: String,
    pub policy_key: String,
    pub queue_name: String,
    pub queue_per_employee: bool,
    pub peek_lock_timeout: Duration,
}

pub struct ServiceBusIngestionQueue {
    http_client: Arc<dyn HttpClient>,
    namespace: String,
    policy_name: String,
    policy_key: String,
    queue_name: String,
    queue_per_employee: bool,
    peek_lock_timeout: Duration,
    runtime: Option<Runtime>,
    clients: Mutex<HashMap<String, QueueClient>>,
    pending: Mutex<HashMap<Uuid, PeekLockResponse>>,
}

impl ServiceBusIngestionQueue {
    pub fn from_env() -> Result<Self, IngestionQueueError> {
        let config = resolve_service_bus_config_from_env()?;
        Self::new(config)
    }

    pub fn new(config: ServiceBusConfig) -> Result<Self, IngestionQueueError> {
        let http_client = azure_core::new_http_client();
        let runtime = Runtime::new()
            .map_err(|err| IngestionQueueError::ServiceBus(err.to_string()))?;
        Ok(Self {
            http_client,
            namespace: config.namespace,
            policy_name: config.policy_name,
            policy_key: config.policy_key,
            queue_name: config.queue_name,
            queue_per_employee: config.queue_per_employee,
            peek_lock_timeout: config.peek_lock_timeout,
            runtime: Some(runtime),
            clients: Mutex::new(HashMap::new()),
            pending: Mutex::new(HashMap::new()),
        })
    }

    fn runtime(&self) -> Result<&Runtime, IngestionQueueError> {
        self.runtime
            .as_ref()
            .ok_or_else(|| IngestionQueueError::ServiceBus("service bus runtime dropped".to_string()))
    }

    fn queue_name_for_employee(&self, employee_id: &str) -> String {
        if self.queue_per_employee {
            format!("{}-{}", self.queue_name, employee_id)
        } else {
            self.queue_name.clone()
        }
    }

    fn client_for_queue(&self, queue_name: &str) -> Result<QueueClient, IngestionQueueError> {
        let mut guard = self
            .clients
            .lock()
            .map_err(|_| IngestionQueueError::ServiceBus("queue client lock poisoned".to_string()))?;
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

    pub fn enqueue(&self, envelope: &IngestionEnvelope) -> Result<EnqueueResult, IngestionQueueError> {
        let queue_name = self.queue_name_for_employee(&envelope.employee_id);
        let client = self.client_for_queue(&queue_name)?;
        let payload_json =
            serde_json::to_string(envelope).map_err(IngestionQueueError::Json)?;
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

    pub fn claim_next(&self, employee_id: &str) -> Result<Option<QueuedEnvelope>, IngestionQueueError> {
        let queue_name = self.queue_name_for_employee(employee_id);
        let client = self.client_for_queue(&queue_name)?;
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
        let handle_id = Uuid::new_v4();
        let mut pending = self.pending.lock().map_err(|_| {
            IngestionQueueError::ServiceBus("pending lock poisoned".to_string())
        })?;
        pending.insert(handle_id, response);
        Ok(Some(QueuedEnvelope {
            id: handle_id,
            envelope,
        }))
    }

    pub fn mark_done(&self, id: &Uuid) -> Result<(), IngestionQueueError> {
        let response = self.take_pending(id)?;
        self.runtime()?
            .block_on(response.delete_message())
            .map_err(map_service_bus_error)?;
        Ok(())
    }

    pub fn mark_failed(&self, id: &Uuid, _error: &str) -> Result<(), IngestionQueueError> {
        let response = self.take_pending(id)?;
        self.runtime()?
            .block_on(response.unlock_message())
            .map_err(map_service_bus_error)?;
        Ok(())
    }

    fn take_pending(&self, id: &Uuid) -> Result<PeekLockResponse, IngestionQueueError> {
        let mut pending = self
            .pending
            .lock()
            .map_err(|_| IngestionQueueError::ServiceBus("pending lock poisoned".to_string()))?;
        pending
            .remove(id)
            .ok_or_else(|| IngestionQueueError::ServiceBus("missing pending lock".to_string()))
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
        if let Some(runtime) = self.runtime.take() {
            runtime.shutdown_background();
        }
    }
}

fn map_service_bus_error(err: AzureError) -> IngestionQueueError {
    IngestionQueueError::ServiceBus(err.to_string())
}

pub fn resolve_service_bus_config_from_env() -> Result<ServiceBusConfig, IngestionQueueError> {
    if let Ok(conn_str) = env::var("SERVICE_BUS_CONNECTION_STRING") {
        let parts = parse_service_bus_connection_string(&conn_str)?;
        let queue_name = env::var("SERVICE_BUS_QUEUE_NAME")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .or(parts.entity_path)
            .ok_or_else(|| IngestionQueueError::Config("missing SERVICE_BUS_QUEUE_NAME".to_string()))?;
        let queue_per_employee = resolve_bool_env("SERVICE_BUS_QUEUE_PER_EMPLOYEE", true);
        let timeout_secs = resolve_i64_env("SERVICE_BUS_PEEK_LOCK_TIMEOUT_SECS", 30);
        return Ok(ServiceBusConfig {
            namespace: parts.namespace,
            policy_name: parts.policy_name,
            policy_key: parts.policy_key,
            queue_name,
            queue_per_employee,
            peek_lock_timeout: Duration::from_secs(timeout_secs as u64),
        });
    }

    let namespace = env::var("SERVICE_BUS_NAMESPACE")
        .map_err(|_| IngestionQueueError::Config("missing SERVICE_BUS_NAMESPACE".to_string()))?;
    let policy_name = env::var("SERVICE_BUS_POLICY_NAME")
        .map_err(|_| IngestionQueueError::Config("missing SERVICE_BUS_POLICY_NAME".to_string()))?;
    let policy_key = env::var("SERVICE_BUS_POLICY_KEY")
        .map_err(|_| IngestionQueueError::Config("missing SERVICE_BUS_POLICY_KEY".to_string()))?;
    let queue_name = env::var("SERVICE_BUS_QUEUE_NAME")
        .map_err(|_| IngestionQueueError::Config("missing SERVICE_BUS_QUEUE_NAME".to_string()))?;
    let queue_per_employee = resolve_bool_env("SERVICE_BUS_QUEUE_PER_EMPLOYEE", true);
    let timeout_secs = resolve_i64_env("SERVICE_BUS_PEEK_LOCK_TIMEOUT_SECS", 30);
    Ok(ServiceBusConfig {
        namespace,
        policy_name,
        policy_key,
        queue_name,
        queue_per_employee,
        peek_lock_timeout: Duration::from_secs(timeout_secs as u64),
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

fn resolve_bool_env(key: &str, default_value: bool) -> bool {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .map(|value| matches!(value.as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(default_value)
}
