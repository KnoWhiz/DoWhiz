use std::collections::HashMap;
use std::time::Duration;

use serde::Deserialize;
use serde_json::json;

use super::contracts::{HubspotDispatchReport, ModeAOutboundDispatchOutput};

#[derive(Debug, thiserror::Error)]
pub enum HubspotDispatchError {
    #[error("missing HUBSPOT_ACCESS_TOKEN in environment")]
    MissingAccessToken,
    #[error("hubspot request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("hubspot response parse failed: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("hubspot request failed with status {status}: {body}")]
    UnexpectedStatus { status: u16, body: String },
}

#[derive(Debug, Clone)]
pub struct HubspotModeAExecutor {
    client: reqwest::blocking::Client,
    base_url: String,
    access_token: String,
}

impl HubspotModeAExecutor {
    pub fn from_env() -> Result<Self, HubspotDispatchError> {
        let access_token = std::env::var("HUBSPOT_ACCESS_TOKEN")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .ok_or(HubspotDispatchError::MissingAccessToken)?;
        let base_url = std::env::var("HUBSPOT_API_BASE_URL")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "https://api.hubapi.com".to_string());
        Ok(Self::new(base_url, access_token))
    }

    pub fn new(base_url: String, access_token: String) -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(20))
            .build()
            .expect("hubspot client should build");
        Self {
            client,
            base_url,
            access_token,
        }
    }

    pub fn dispatch_mode_a_drafts(
        &self,
        output: &ModeAOutboundDispatchOutput,
    ) -> HubspotDispatchReport {
        let mut report = HubspotDispatchReport::default();
        let mut contacts_by_email = HashMap::new();

        for task in &output.hubspot_task_drafts {
            report.tasks_attempted += 1;
            match self.create_task(task) {
                Ok(task_id) => {
                    report.tasks_created += 1;
                    let contact_id = resolve_contact_id_cached(
                        self,
                        &task.contact_email,
                        &mut contacts_by_email,
                        &mut report,
                    );
                    if let Some(contact_id) = contact_id {
                        report.associations_attempted += 1;
                        if self
                            .associate_entity_with_contact("tasks", &task_id, &contact_id)
                            .is_ok()
                        {
                            report.associations_created += 1;
                        } else {
                            report.errors.push(format!(
                                "failed to associate HubSpot task {} with contact {}",
                                task_id, task.contact_email
                            ));
                        }
                    }
                }
                Err(err) => {
                    report.errors.push(format!(
                        "failed to create HubSpot task draft {}: {}",
                        task.external_id, err
                    ));
                }
            }
        }

        for communication in &output.hubspot_communication_drafts {
            report.notes_attempted += 1;
            match self.create_note(communication) {
                Ok(note_id) => {
                    report.notes_created += 1;
                    let contact_id = resolve_contact_id_cached(
                        self,
                        &communication.contact_email,
                        &mut contacts_by_email,
                        &mut report,
                    );
                    if let Some(contact_id) = contact_id {
                        report.associations_attempted += 1;
                        if self
                            .associate_entity_with_contact("notes", &note_id, &contact_id)
                            .is_ok()
                        {
                            report.associations_created += 1;
                        } else {
                            report.errors.push(format!(
                                "failed to associate HubSpot note {} with contact {}",
                                note_id, communication.contact_email
                            ));
                        }
                    }
                }
                Err(err) => {
                    report.errors.push(format!(
                        "failed to create HubSpot communication draft {}: {}",
                        communication.external_id, err
                    ));
                }
            }
        }

        report
    }

    fn create_task(
        &self,
        task: &super::contracts::HubspotTaskDraft,
    ) -> Result<String, HubspotDispatchError> {
        let payload = json!({
            "properties": {
                "hs_task_subject": task.subject,
                "hs_task_body": task.body,
                "hs_timestamp": task.due_at.to_rfc3339(),
                "hs_task_status": "NOT_STARTED",
                "hs_task_priority": "MEDIUM"
            }
        });
        let response = self.post_json("/crm/v3/objects/tasks", &payload)?;
        extract_id(response)
    }

    fn create_note(
        &self,
        communication: &super::contracts::HubspotCommunicationDraft,
    ) -> Result<String, HubspotDispatchError> {
        let payload = json!({
            "properties": {
                "hs_note_body": communication.summary,
                "hs_timestamp": communication.scheduled_at.to_rfc3339(),
            }
        });
        let response = self.post_json("/crm/v3/objects/notes", &payload)?;
        extract_id(response)
    }

    fn associate_entity_with_contact(
        &self,
        entity_kind: &str,
        entity_id: &str,
        contact_id: &str,
    ) -> Result<(), HubspotDispatchError> {
        let path = format!(
            "/crm/v4/objects/{}/{}/associations/default/contacts/{}",
            entity_kind, entity_id, contact_id
        );
        self.put_empty(&path)
    }

    fn find_contact_id_by_email(
        &self,
        email: &str,
    ) -> Result<Option<String>, HubspotDispatchError> {
        let payload = json!({
            "filterGroups": [
                {
                    "filters": [
                        {
                            "propertyName": "email",
                            "operator": "EQ",
                            "value": email
                        }
                    ]
                }
            ],
            "limit": 1
        });

        let response = self.post_json("/crm/v3/objects/contacts/search", &payload)?;
        let parsed: ContactSearchResponse = serde_json::from_value(response)?;
        Ok(parsed.results.first().map(|result| result.id.clone()))
    }

    fn post_json(
        &self,
        path: &str,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, HubspotDispatchError> {
        let response = self
            .client
            .post(self.build_url(path))
            .bearer_auth(&self.access_token)
            .json(payload)
            .send()?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().unwrap_or_default();
            return Err(HubspotDispatchError::UnexpectedStatus {
                status: status.as_u16(),
                body,
            });
        }
        Ok(response.json()?)
    }

    fn put_empty(&self, path: &str) -> Result<(), HubspotDispatchError> {
        let response = self
            .client
            .put(self.build_url(path))
            .bearer_auth(&self.access_token)
            .send()?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().unwrap_or_default();
            return Err(HubspotDispatchError::UnexpectedStatus {
                status: status.as_u16(),
                body,
            });
        }
        Ok(())
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url.trim_end_matches('/'), path)
    }
}

fn extract_id(value: serde_json::Value) -> Result<String, HubspotDispatchError> {
    value
        .get("id")
        .and_then(|id| id.as_str())
        .map(|id| id.to_string())
        .ok_or_else(|| HubspotDispatchError::UnexpectedStatus {
            status: 200,
            body: format!("missing id in response payload: {}", value),
        })
}

fn resolve_contact_id_cached(
    executor: &HubspotModeAExecutor,
    email: &str,
    cache: &mut HashMap<String, Option<String>>,
    report: &mut HubspotDispatchReport,
) -> Option<String> {
    if let Some(existing) = cache.get(email) {
        return existing.clone();
    }

    let resolved = match executor.find_contact_id_by_email(email) {
        Ok(value) => value,
        Err(err) => {
            report.errors.push(format!(
                "failed to find HubSpot contact for {}: {}",
                email, err
            ));
            None
        }
    };
    cache.insert(email.to_string(), resolved.clone());
    resolved
}

#[derive(Debug, Deserialize)]
struct ContactSearchResponse {
    results: Vec<ContactSearchResult>,
}

#[derive(Debug, Deserialize)]
struct ContactSearchResult {
    id: String,
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use mockito::{Matcher, Server};

    use super::super::contracts::{
        GtmChannel, HubspotCommunicationDraft, HubspotTaskDraft, ModeAOutboundDispatchOutput,
    };
    use super::*;

    fn mode_a_output() -> ModeAOutboundDispatchOutput {
        ModeAOutboundDispatchOutput {
            approval_queue: Vec::new(),
            manual_send_tasks: Vec::new(),
            hubspot_task_drafts: vec![HubspotTaskDraft {
                external_id: "task_ext_1".to_string(),
                subject: "Follow up on LinkedIn outreach".to_string(),
                body: "Please send the approved DM template manually.".to_string(),
                due_at: Utc::now(),
                contact_email: "alpha@example.com".to_string(),
                owner_team: "sdr_team".to_string(),
            }],
            hubspot_communication_drafts: vec![HubspotCommunicationDraft {
                external_id: "comm_ext_1".to_string(),
                channel: GtmChannel::LinkedinDm,
                contact_email: "alpha@example.com".to_string(),
                scheduled_at: Utc::now(),
                summary: "Planned manual LinkedIn outreach".to_string(),
            }],
        }
    }

    #[test]
    fn dispatch_mode_a_drafts_creates_tasks_and_notes() {
        let mut server = Server::new();
        let _search = server
            .mock("POST", "/crm/v3/objects/contacts/search")
            .match_header("authorization", "Bearer test-token")
            .match_body(Matcher::Regex("alpha@example.com".to_string()))
            .with_status(200)
            .with_body(r#"{"results":[{"id":"201"}]}"#)
            .create();
        let _task_create = server
            .mock("POST", "/crm/v3/objects/tasks")
            .match_header("authorization", "Bearer test-token")
            .with_status(201)
            .with_body(r#"{"id":"301"}"#)
            .create();
        let _task_assoc = server
            .mock(
                "PUT",
                "/crm/v4/objects/tasks/301/associations/default/contacts/201",
            )
            .match_header("authorization", "Bearer test-token")
            .with_status(204)
            .create();
        let _note_create = server
            .mock("POST", "/crm/v3/objects/notes")
            .match_header("authorization", "Bearer test-token")
            .with_status(201)
            .with_body(r#"{"id":"401"}"#)
            .create();
        let _note_assoc = server
            .mock(
                "PUT",
                "/crm/v4/objects/notes/401/associations/default/contacts/201",
            )
            .match_header("authorization", "Bearer test-token")
            .with_status(204)
            .create();

        let executor = HubspotModeAExecutor::new(server.url(), "test-token".to_string());
        let report = executor.dispatch_mode_a_drafts(&mode_a_output());

        assert_eq!(report.tasks_attempted, 1);
        assert_eq!(report.tasks_created, 1);
        assert_eq!(report.notes_attempted, 1);
        assert_eq!(report.notes_created, 1);
        assert_eq!(report.associations_attempted, 2);
        assert_eq!(report.associations_created, 2);
        assert!(report.errors.is_empty());
    }

    #[test]
    fn dispatch_mode_a_drafts_reports_api_errors() {
        let mut server = Server::new();
        let _task_create = server
            .mock("POST", "/crm/v3/objects/tasks")
            .match_header("authorization", "Bearer test-token")
            .with_status(500)
            .with_body(r#"{"message":"server error"}"#)
            .create();
        let _note_create = server
            .mock("POST", "/crm/v3/objects/notes")
            .match_header("authorization", "Bearer test-token")
            .with_status(500)
            .with_body(r#"{"message":"server error"}"#)
            .create();

        let executor = HubspotModeAExecutor::new(server.url(), "test-token".to_string());
        let report = executor.dispatch_mode_a_drafts(&mode_a_output());

        assert_eq!(report.tasks_attempted, 1);
        assert_eq!(report.tasks_created, 0);
        assert_eq!(report.notes_attempted, 1);
        assert_eq!(report.notes_created, 0);
        assert_eq!(report.associations_attempted, 0);
        assert_eq!(report.associations_created, 0);
        assert_eq!(report.errors.len(), 2);
    }
}
