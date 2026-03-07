//! Outbound adapter for Google Slides.

use tracing::{error, info};

use crate::channel::{AdapterError, Channel, OutboundAdapter, OutboundMessage, SendResult};
use crate::google_auth::GoogleAuth;

use super::super::google_common::{CommentReply, GoogleCommentsClient};
use super::super::google_docs::contains_employee_mention;
use std::collections::HashSet;

/// Adapter for posting replies to Google Slides comments and editing presentations.
#[derive(Debug, Clone)]
pub struct GoogleSlidesOutboundAdapter {
    auth: GoogleAuth,
    comments_client: GoogleCommentsClient,
}

impl GoogleSlidesOutboundAdapter {
    pub fn new(auth: GoogleAuth) -> Self {
        let comments_client =
            GoogleCommentsClient::new(auth.clone(), HashSet::new(), contains_employee_mention);
        Self {
            auth,
            comments_client,
        }
    }

    /// Post a reply to a comment.
    pub fn reply_to_comment(
        &self,
        presentation_id: &str,
        comment_id: &str,
        reply_content: &str,
    ) -> Result<CommentReply, AdapterError> {
        self.comments_client
            .reply_to_comment(presentation_id, comment_id, reply_content)
    }

    /// Read presentation content as plain text.
    pub fn read_presentation_content(&self, presentation_id: &str) -> Result<String, AdapterError> {
        self.comments_client
            .export_file_content(presentation_id, "text/plain")
    }

    /// Batch update presentation (for adding slides, shapes, text, etc.).
    pub fn batch_update(
        &self,
        presentation_id: &str,
        requests: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        let url = format!(
            "https://slides.googleapis.com/v1/presentations/{}:batchUpdate",
            presentation_id
        );

        let payload = serde_json::json!({
            "requests": requests
        });

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!(
                "Failed to batch update presentation {}: {} - {}",
                presentation_id, status, body
            );
            return Err(AdapterError::SendError(format!(
                "HTTP {}: {}",
                status, body
            )));
        }

        info!("Applied batch update to presentation {}", presentation_id);

        response
            .json()
            .map_err(|e| AdapterError::ParseError(e.to_string()))
    }

    /// Insert text into a shape/text box.
    pub fn insert_text(
        &self,
        presentation_id: &str,
        object_id: &str,
        text: &str,
        insertion_index: Option<i32>,
    ) -> Result<(), AdapterError> {
        let request = if let Some(index) = insertion_index {
            serde_json::json!({
                "insertText": {
                    "objectId": object_id,
                    "text": text,
                    "insertionIndex": index
                }
            })
        } else {
            serde_json::json!({
                "insertText": {
                    "objectId": object_id,
                    "text": text
                }
            })
        };

        self.batch_update(presentation_id, vec![request])?;
        Ok(())
    }

    /// Replace all instances of text in the presentation.
    pub fn replace_all_text(
        &self,
        presentation_id: &str,
        find_text: &str,
        replace_text: &str,
        match_case: bool,
    ) -> Result<(), AdapterError> {
        let request = serde_json::json!({
            "replaceAllText": {
                "containsText": {
                    "text": find_text,
                    "matchCase": match_case
                },
                "replaceText": replace_text
            }
        });

        self.batch_update(presentation_id, vec![request])?;
        info!(
            "Replaced all '{}' with '{}' in presentation {}",
            find_text, replace_text, presentation_id
        );
        Ok(())
    }

    /// Create a new slide.
    pub fn create_slide(
        &self,
        presentation_id: &str,
        object_id: Option<&str>,
        insertion_index: Option<i32>,
        layout: Option<&str>,
    ) -> Result<String, AdapterError> {
        let mut request = serde_json::json!({
            "createSlide": {}
        });

        if let Some(id) = object_id {
            request["createSlide"]["objectId"] = serde_json::json!(id);
        }
        if let Some(index) = insertion_index {
            request["createSlide"]["insertionIndex"] = serde_json::json!(index);
        }
        if let Some(layout_id) = layout {
            request["createSlide"]["slideLayoutReference"] = serde_json::json!({
                "predefinedLayout": layout_id
            });
        }

        let response = self.batch_update(presentation_id, vec![request])?;

        // Extract the created slide ID from response
        let slide_id = response
            .get("replies")
            .and_then(|r| r.as_array())
            .and_then(|arr| arr.first())
            .and_then(|reply| reply.get("createSlide"))
            .and_then(|cs| cs.get("objectId"))
            .and_then(|id| id.as_str())
            .unwrap_or("unknown")
            .to_string();

        info!(
            "Created slide {} in presentation {}",
            slide_id, presentation_id
        );
        Ok(slide_id)
    }

    /// Delete a slide.
    pub fn delete_slide(
        &self,
        presentation_id: &str,
        slide_object_id: &str,
    ) -> Result<(), AdapterError> {
        let request = serde_json::json!({
            "deleteObject": {
                "objectId": slide_object_id
            }
        });

        self.batch_update(presentation_id, vec![request])?;
        info!(
            "Deleted slide {} from presentation {}",
            slide_object_id, presentation_id
        );
        Ok(())
    }

    /// Insert an image into a slide.
    ///
    /// The image URL must be publicly accessible. For private images, first upload
    /// to Google Cloud Storage or Azure Blob and use a signed URL.
    ///
    /// Constraints:
    /// - Image must be < 50MB
    /// - Image must be < 25 megapixels
    /// - Supported formats: PNG, JPEG, GIF
    pub fn insert_image(
        &self,
        presentation_id: &str,
        image_url: &str,
        page_id: &str,
        x_pt: f64,
        y_pt: f64,
        width_pt: Option<f64>,
        height_pt: Option<f64>,
    ) -> Result<String, AdapterError> {
        // Generate a unique object ID for the image
        let object_id = format!(
            "img_{}",
            uuid::Uuid::new_v4().to_string().replace('-', "")[..12].to_string()
        );

        // Convert points to EMU (English Metric Units) - Google Slides uses EMU
        // 1 point = 914400 / 72 EMU = 12700 EMU
        let emu_per_pt = 12700.0;

        let translate_x = (x_pt * emu_per_pt) as i64;
        let translate_y = (y_pt * emu_per_pt) as i64;

        let mut request = serde_json::json!({
            "createImage": {
                "objectId": object_id,
                "url": image_url,
                "elementProperties": {
                    "pageObjectId": page_id,
                    "transform": {
                        "scaleX": 1.0,
                        "scaleY": 1.0,
                        "translateX": translate_x,
                        "translateY": translate_y,
                        "unit": "EMU"
                    }
                }
            }
        });

        // Add size if specified
        if let (Some(w), Some(h)) = (width_pt, height_pt) {
            let width_emu = (w * emu_per_pt) as i64;
            let height_emu = (h * emu_per_pt) as i64;
            request["createImage"]["elementProperties"]["size"] = serde_json::json!({
                "width": { "magnitude": width_emu, "unit": "EMU" },
                "height": { "magnitude": height_emu, "unit": "EMU" }
            });
        }

        let response = self.batch_update(presentation_id, vec![request])?;

        // Extract the created image ID from response
        let created_id = response
            .get("replies")
            .and_then(|r| r.as_array())
            .and_then(|arr| arr.first())
            .and_then(|reply| reply.get("createImage"))
            .and_then(|ci| ci.get("objectId"))
            .and_then(|id| id.as_str())
            .unwrap_or(&object_id)
            .to_string();

        info!(
            "Inserted image {} into slide {} of presentation {}",
            created_id, page_id, presentation_id
        );

        Ok(created_id)
    }

    /// Create a new presentation.
    ///
    /// Returns the presentation ID of the newly created presentation.
    pub fn create_presentation(&self, title: &str) -> Result<String, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        let url = "https://slides.googleapis.com/v1/presentations";

        let payload = serde_json::json!({
            "title": title
        });

        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!("Failed to create presentation '{}': {} - {}", title, status, body);
            return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
        }

        let json: serde_json::Value = response
            .json()
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let presentation_id = json
            .get("presentationId")
            .and_then(|id| id.as_str())
            .ok_or_else(|| AdapterError::ParseError("Missing presentationId in response".to_string()))?
            .to_string();

        info!("Created new presentation '{}' with ID {}", title, presentation_id);

        Ok(presentation_id)
    }

    /// Get presentation metadata.
    pub fn get_presentation(
        &self,
        presentation_id: &str,
    ) -> Result<serde_json::Value, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        let url = format!(
            "https://slides.googleapis.com/v1/presentations/{}",
            presentation_id
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!(
                "Failed to get presentation {}: {} - {}",
                presentation_id, status, body
            );
            return Err(AdapterError::SendError(format!(
                "HTTP {}: {}",
                status, body
            )));
        }

        response
            .json()
            .map_err(|e| AdapterError::ParseError(e.to_string()))
    }
}

impl OutboundAdapter for GoogleSlidesOutboundAdapter {
    fn send(&self, message: &OutboundMessage) -> Result<SendResult, AdapterError> {
        let presentation_id = message
            .metadata
            .google_slides_presentation_id
            .as_ref()
            .ok_or_else(|| AdapterError::ConfigError("Missing presentation ID".to_string()))?;

        let comment_id = message
            .metadata
            .google_slides_comment_id
            .as_ref()
            .ok_or_else(|| AdapterError::ConfigError("Missing comment ID".to_string()))?;

        let reply_content = if !message.text_body.is_empty() {
            &message.text_body
        } else {
            &message.html_body
        };

        let reply = self.reply_to_comment(presentation_id, comment_id, reply_content)?;

        Ok(SendResult {
            success: true,
            message_id: reply.id,
            submitted_at: reply
                .created_time
                .unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
            error: None,
        })
    }

    fn channel(&self) -> Channel {
        Channel::GoogleSlides
    }
}
