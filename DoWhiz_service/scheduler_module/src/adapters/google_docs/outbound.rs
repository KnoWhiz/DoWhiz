use tracing::{error, info};

use crate::channel::{AdapterError, Channel, OutboundAdapter, OutboundMessage, SendResult};
use crate::google_auth::GoogleAuth;

use super::models::CommentReply;

/// Adapter for posting replies to Google Docs comments.
#[derive(Debug, Clone)]
pub struct GoogleDocsOutboundAdapter {
    /// Google authentication
    auth: GoogleAuth,
}

impl GoogleDocsOutboundAdapter {
    pub fn new(auth: GoogleAuth) -> Self {
        Self { auth }
    }

    /// Post a reply to a comment.
    pub fn reply_to_comment(
        &self,
        document_id: &str,
        comment_id: &str,
        reply_content: &str,
    ) -> Result<CommentReply, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        // Google Drive API v3 requires 'fields' parameter to specify response fields
        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/comments/{}/replies?fields=id,content,createdTime,author",
            document_id, comment_id
        );

        let payload = serde_json::json!({
            "content": reply_content
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
                "Failed to reply to comment {} on {}: {} - {}",
                comment_id, document_id, status, body
            );
            return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
        }

        let reply: CommentReply = response
            .json()
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        info!(
            "Posted reply {} to comment {} on document {}",
            reply.id, comment_id, document_id
        );

        Ok(reply)
    }

    /// Read document content (for context when processing comments).
    pub fn read_document_content(&self, document_id: &str) -> Result<String, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        // Export document as plain text
        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/export?mimeType=text/plain",
            document_id
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!("Failed to read document {}: {} - {}", document_id, status, body);
            return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
        }

        response
            .text()
            .map_err(|e| AdapterError::ParseError(e.to_string()))
    }

    /// Apply an edit to the document (direct edit, not suggestion mode).
    /// Note: Google Docs API does not support creating suggestions programmatically.
    pub fn apply_document_edit(
        &self,
        document_id: &str,
        requests: Vec<serde_json::Value>,
    ) -> Result<(), AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        let url = format!(
            "https://docs.googleapis.com/v1/documents/{}:batchUpdate",
            document_id
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
            error!("Failed to apply edit to {}: {} - {}", document_id, status, body);
            return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
        }

        info!("Applied edit to document {}", document_id);
        Ok(())
    }

    /// Get document structure to find text positions.
    /// Returns the document body content with start/end indices.
    pub fn get_document_structure(
        &self,
        document_id: &str,
    ) -> Result<serde_json::Value, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();
        let url = format!(
            "https://docs.googleapis.com/v1/documents/{}",
            document_id
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!("Failed to get document structure {}: {} - {}", document_id, status, body);
            return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
        }

        response
            .json()
            .map_err(|e| AdapterError::ParseError(e.to_string()))
    }

    /// Find text in document and return its start and end indices.
    /// Returns (start_index, end_index) or None if not found.
    pub fn find_text_position(
        &self,
        document_id: &str,
        search_text: &str,
    ) -> Result<Option<(i64, i64)>, AdapterError> {
        let doc = self.get_document_structure(document_id)?;

        // Extract body content
        let body = doc.get("body").and_then(|b| b.get("content"));
        if body.is_none() {
            return Ok(None);
        }

        let content = body.unwrap().as_array().ok_or_else(|| {
            AdapterError::ParseError("Invalid document structure".to_string())
        })?;

        // Build full text and track positions
        let mut full_text = String::new();
        let mut text_positions: Vec<(usize, i64)> = Vec::new(); // (string_pos, doc_index)

        for element in content {
            if let Some(paragraph) = element.get("paragraph") {
                if let Some(elements) = paragraph.get("elements").and_then(|e| e.as_array()) {
                    for elem in elements {
                        if let Some(text_run) = elem.get("textRun") {
                            if let Some(content_text) = text_run.get("content").and_then(|c| c.as_str()) {
                                let start_idx = elem.get("startIndex").and_then(|i| i.as_i64()).unwrap_or(0);
                                text_positions.push((full_text.len(), start_idx));
                                full_text.push_str(content_text);
                            }
                        }
                    }
                }
            }
        }

        // Find the search text in full_text
        if let Some(string_pos) = full_text.find(search_text) {
            // Convert string position to document index
            let mut doc_start_idx = 0i64;
            for (str_pos, doc_idx) in &text_positions {
                if *str_pos <= string_pos {
                    doc_start_idx = *doc_idx + (string_pos - str_pos) as i64;
                }
            }
            let doc_end_idx = doc_start_idx + search_text.len() as i64;
            return Ok(Some((doc_start_idx, doc_end_idx)));
        }

        Ok(None)
    }

    /// Mark text for deletion with red color and strikethrough.
    /// Used in suggesting mode to show text that will be removed.
    pub fn mark_deletion(
        &self,
        document_id: &str,
        text_to_mark: &str,
    ) -> Result<(), AdapterError> {
        let position = self.find_text_position(document_id, text_to_mark)?;

        let (start_idx, end_idx) = position.ok_or_else(|| {
            AdapterError::SendError(format!("Text not found in document: '{}'", text_to_mark))
        })?;

        // Apply red color and strikethrough
        let requests = vec![
            serde_json::json!({
                "updateTextStyle": {
                    "range": {
                        "startIndex": start_idx,
                        "endIndex": end_idx
                    },
                    "textStyle": {
                        "foregroundColor": {
                            "color": {
                                "rgbColor": {
                                    "red": 1.0,
                                    "green": 0.0,
                                    "blue": 0.0
                                }
                            }
                        },
                        "strikethrough": true
                    },
                    "fields": "foregroundColor,strikethrough"
                }
            })
        ];

        self.apply_document_edit(document_id, requests)?;
        info!("Marked deletion '{}' at indices {}-{}", text_to_mark, start_idx, end_idx);
        Ok(())
    }

    /// Insert new text with blue color (suggesting mode).
    /// The text is inserted after the specified anchor text.
    pub fn insert_suggestion(
        &self,
        document_id: &str,
        after_text: &str,
        new_text: &str,
    ) -> Result<(), AdapterError> {
        let position = self.find_text_position(document_id, after_text)?;

        let (_, end_idx) = position.ok_or_else(|| {
            AdapterError::SendError(format!("Anchor text not found: '{}'", after_text))
        })?;

        // Insert text and make it blue (explicitly remove strikethrough in case anchor has it)
        let requests = vec![
            serde_json::json!({
                "insertText": {
                    "location": {
                        "index": end_idx
                    },
                    "text": new_text
                }
            }),
            serde_json::json!({
                "updateTextStyle": {
                    "range": {
                        "startIndex": end_idx,
                        "endIndex": end_idx + new_text.chars().count() as i64
                    },
                    "textStyle": {
                        "foregroundColor": {
                            "color": {
                                "rgbColor": {
                                    "red": 0.0,
                                    "green": 0.0,
                                    "blue": 1.0
                                }
                            }
                        },
                        "strikethrough": false
                    },
                    "fields": "foregroundColor,strikethrough"
                }
            })
        ];

        self.apply_document_edit(document_id, requests)?;
        info!("Inserted suggestion '{}' after '{}'", new_text, after_text);
        Ok(())
    }

    /// Replace text with revision marks (suggesting mode).
    /// Old text gets red + strikethrough, new text gets blue.
    pub fn suggest_replace(
        &self,
        document_id: &str,
        old_text: &str,
        new_text: &str,
    ) -> Result<(), AdapterError> {
        let position = self.find_text_position(document_id, old_text)?;

        let (start_idx, end_idx) = position.ok_or_else(|| {
            AdapterError::SendError(format!("Text to replace not found: '{}'", old_text))
        })?;

        // First, mark old text as deleted (red + strikethrough)
        // Then insert new text (blue) right after the old text
        let requests = vec![
            // Mark old text as deleted
            serde_json::json!({
                "updateTextStyle": {
                    "range": {
                        "startIndex": start_idx,
                        "endIndex": end_idx
                    },
                    "textStyle": {
                        "foregroundColor": {
                            "color": {
                                "rgbColor": {
                                    "red": 1.0,
                                    "green": 0.0,
                                    "blue": 0.0
                                }
                            }
                        },
                        "strikethrough": true
                    },
                    "fields": "foregroundColor,strikethrough"
                }
            }),
            // Insert new text right after old text
            serde_json::json!({
                "insertText": {
                    "location": {
                        "index": end_idx
                    },
                    "text": new_text
                }
            }),
            // Make new text blue (and explicitly remove strikethrough since it may inherit from previous text)
            serde_json::json!({
                "updateTextStyle": {
                    "range": {
                        "startIndex": end_idx,
                        "endIndex": end_idx + new_text.chars().count() as i64
                    },
                    "textStyle": {
                        "foregroundColor": {
                            "color": {
                                "rgbColor": {
                                    "red": 0.0,
                                    "green": 0.0,
                                    "blue": 1.0
                                }
                            }
                        },
                        "strikethrough": false
                    },
                    "fields": "foregroundColor,strikethrough"
                }
            })
        ];

        self.apply_document_edit(document_id, requests)?;
        info!("Suggested replacement: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }

    /// Apply all suggestions in the document.
    /// Deletes all red strikethrough text and converts blue text to black.
    pub fn apply_suggestions(&self, document_id: &str) -> Result<(), AdapterError> {
        let doc = self.get_document_structure(document_id)?;

        let body = doc.get("body").and_then(|b| b.get("content"));
        if body.is_none() {
            return Ok(());
        }

        let content = body.unwrap().as_array().ok_or_else(|| {
            AdapterError::ParseError("Invalid document structure".to_string())
        })?;

        // Collect ranges to delete (red strikethrough) and ranges to normalize (blue)
        let mut ranges_to_delete: Vec<(i64, i64)> = Vec::new();
        let mut ranges_to_normalize: Vec<(i64, i64)> = Vec::new();

        for element in content {
            if let Some(paragraph) = element.get("paragraph") {
                if let Some(elements) = paragraph.get("elements").and_then(|e| e.as_array()) {
                    for elem in elements {
                        if let Some(text_run) = elem.get("textRun") {
                            let start_idx = elem.get("startIndex").and_then(|i| i.as_i64()).unwrap_or(0);
                            let end_idx = elem.get("endIndex").and_then(|i| i.as_i64()).unwrap_or(0);

                            if let Some(text_style) = text_run.get("textStyle") {
                                // Check for red strikethrough (deletion markers)
                                let is_strikethrough = text_style.get("strikethrough")
                                    .and_then(|s| s.as_bool())
                                    .unwrap_or(false);
                                let is_red = text_style.get("foregroundColor")
                                    .and_then(|fc| fc.get("color"))
                                    .and_then(|c| c.get("rgbColor"))
                                    .map(|rgb| {
                                        let r = rgb.get("red").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let g = rgb.get("green").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let b = rgb.get("blue").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        r > 0.8 && g < 0.2 && b < 0.2 // Check if red
                                    })
                                    .unwrap_or(false);

                                if is_strikethrough && is_red {
                                    ranges_to_delete.push((start_idx, end_idx));
                                    continue;
                                }

                                // Check for blue text (addition markers)
                                let is_blue = text_style.get("foregroundColor")
                                    .and_then(|fc| fc.get("color"))
                                    .and_then(|c| c.get("rgbColor"))
                                    .map(|rgb| {
                                        let r = rgb.get("red").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let g = rgb.get("green").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let b = rgb.get("blue").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        r < 0.2 && g < 0.2 && b > 0.8 // Check if blue
                                    })
                                    .unwrap_or(false);

                                if is_blue {
                                    ranges_to_normalize.push((start_idx, end_idx));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Build requests: delete red text (in reverse order) then normalize blue text
        let mut requests: Vec<serde_json::Value> = Vec::new();

        // Sort ranges_to_delete in reverse order (to avoid index shifting issues)
        let mut sorted_delete = ranges_to_delete.clone();
        sorted_delete.sort_by(|a, b| b.0.cmp(&a.0));

        for (start_idx, end_idx) in sorted_delete {
            requests.push(serde_json::json!({
                "deleteContentRange": {
                    "range": {
                        "startIndex": start_idx,
                        "endIndex": end_idx
                    }
                }
            }));
        }

        // Normalize blue text to black (remove color)
        let ranges_to_normalize_len = ranges_to_normalize.len();
        for (start_idx, end_idx) in ranges_to_normalize {
            // Adjust indices based on deletions that occurred before this range
            let mut adjusted_start = start_idx;
            let mut adjusted_end = end_idx;
            for (del_start, del_end) in &ranges_to_delete {
                if *del_end <= start_idx {
                    let deleted_length = del_end - del_start;
                    adjusted_start -= deleted_length;
                    adjusted_end -= deleted_length;
                }
            }

            requests.push(serde_json::json!({
                "updateTextStyle": {
                    "range": {
                        "startIndex": adjusted_start,
                        "endIndex": adjusted_end
                    },
                    "textStyle": {
                        "foregroundColor": {}  // Reset to default (black)
                    },
                    "fields": "foregroundColor"
                }
            }));
        }

        if !requests.is_empty() {
            self.apply_document_edit(document_id, requests)?;
            info!("Applied suggestions: deleted {} ranges, normalized {} ranges",
                  ranges_to_delete.len(), ranges_to_normalize_len);
        }

        Ok(())
    }

    /// Discard all suggestions in the document.
    /// Removes blue text and restores red strikethrough text to normal.
    pub fn discard_suggestions(&self, document_id: &str) -> Result<(), AdapterError> {
        let doc = self.get_document_structure(document_id)?;

        let body = doc.get("body").and_then(|b| b.get("content"));
        if body.is_none() {
            return Ok(());
        }

        let content = body.unwrap().as_array().ok_or_else(|| {
            AdapterError::ParseError("Invalid document structure".to_string())
        })?;

        // Collect ranges to delete (blue text) and ranges to restore (red strikethrough)
        let mut ranges_to_delete: Vec<(i64, i64)> = Vec::new();
        let mut ranges_to_restore: Vec<(i64, i64)> = Vec::new();

        for element in content {
            if let Some(paragraph) = element.get("paragraph") {
                if let Some(elements) = paragraph.get("elements").and_then(|e| e.as_array()) {
                    for elem in elements {
                        if let Some(text_run) = elem.get("textRun") {
                            let start_idx = elem.get("startIndex").and_then(|i| i.as_i64()).unwrap_or(0);
                            let end_idx = elem.get("endIndex").and_then(|i| i.as_i64()).unwrap_or(0);

                            if let Some(text_style) = text_run.get("textStyle") {
                                // Check for blue text (to be deleted)
                                let is_blue = text_style.get("foregroundColor")
                                    .and_then(|fc| fc.get("color"))
                                    .and_then(|c| c.get("rgbColor"))
                                    .map(|rgb| {
                                        let r = rgb.get("red").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let g = rgb.get("green").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let b = rgb.get("blue").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        r < 0.2 && g < 0.2 && b > 0.8
                                    })
                                    .unwrap_or(false);

                                if is_blue {
                                    ranges_to_delete.push((start_idx, end_idx));
                                    continue;
                                }

                                // Check for red strikethrough (to be restored)
                                let is_strikethrough = text_style.get("strikethrough")
                                    .and_then(|s| s.as_bool())
                                    .unwrap_or(false);
                                let is_red = text_style.get("foregroundColor")
                                    .and_then(|fc| fc.get("color"))
                                    .and_then(|c| c.get("rgbColor"))
                                    .map(|rgb| {
                                        let r = rgb.get("red").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let g = rgb.get("green").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let b = rgb.get("blue").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        r > 0.8 && g < 0.2 && b < 0.2
                                    })
                                    .unwrap_or(false);

                                if is_strikethrough && is_red {
                                    ranges_to_restore.push((start_idx, end_idx));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Build requests: delete blue text (in reverse order) then restore red text
        let mut requests: Vec<serde_json::Value> = Vec::new();

        // Sort ranges_to_delete in reverse order
        let mut sorted_delete = ranges_to_delete.clone();
        sorted_delete.sort_by(|a, b| b.0.cmp(&a.0));

        for (start_idx, end_idx) in sorted_delete {
            requests.push(serde_json::json!({
                "deleteContentRange": {
                    "range": {
                        "startIndex": start_idx,
                        "endIndex": end_idx
                    }
                }
            }));
        }

        // Restore red text to normal (remove color and strikethrough)
        let ranges_to_restore_len = ranges_to_restore.len();
        for (start_idx, end_idx) in ranges_to_restore {
            // Adjust indices based on deletions
            let mut adjusted_start = start_idx;
            let mut adjusted_end = end_idx;
            for (del_start, del_end) in &ranges_to_delete {
                if *del_end <= start_idx {
                    let deleted_length = del_end - del_start;
                    adjusted_start -= deleted_length;
                    adjusted_end -= deleted_length;
                }
            }

            requests.push(serde_json::json!({
                "updateTextStyle": {
                    "range": {
                        "startIndex": adjusted_start,
                        "endIndex": adjusted_end
                    },
                    "textStyle": {
                        "foregroundColor": {},  // Reset to default
                        "strikethrough": false
                    },
                    "fields": "foregroundColor,strikethrough"
                }
            }));
        }

        if !requests.is_empty() {
            self.apply_document_edit(document_id, requests)?;
            info!("Discarded suggestions: deleted {} ranges, restored {} ranges",
                  ranges_to_delete.len(), ranges_to_restore_len);
        }

        Ok(())
    }
}

impl OutboundAdapter for GoogleDocsOutboundAdapter {
    fn send(&self, message: &OutboundMessage) -> Result<SendResult, AdapterError> {
        let document_id = message
            .metadata
            .google_docs_document_id
            .as_ref()
            .ok_or_else(|| AdapterError::ConfigError("Missing document ID".to_string()))?;

        let comment_id = message
            .metadata
            .google_docs_comment_id
            .as_ref()
            .ok_or_else(|| AdapterError::ConfigError("Missing comment ID".to_string()))?;

        // Use text_body as the reply content
        let reply_content = if !message.text_body.is_empty() {
            &message.text_body
        } else {
            &message.html_body
        };

        let reply = self.reply_to_comment(document_id, comment_id, reply_content)?;

        Ok(SendResult {
            success: true,
            message_id: reply.id,
            submitted_at: reply.created_time.unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
            error: None,
        })
    }

    fn channel(&self) -> Channel {
        Channel::GoogleDocs
    }
}

