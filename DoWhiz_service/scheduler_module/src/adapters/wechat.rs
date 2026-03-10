//! WeChat Work (企业微信) adapter for inbound and outbound messages via qyapi.
//!
//! This module provides adapters for handling WeChat Work messages:
//! - `WeChatInboundAdapter`: Parses WeChat webhook payloads (XML)
//! - `WeChatOutboundAdapter`: Sends messages via WeChat Work API

use serde::{Deserialize, Serialize};
use std::sync::RwLock;
use tracing::info;

use crate::channel::{
    AdapterError, Channel, ChannelMetadata, InboundAdapter, InboundMessage, OutboundAdapter,
    OutboundMessage, SendResult,
};

/// Adapter for parsing WeChat Work webhook payloads.
#[derive(Debug, Clone, Default)]
pub struct WeChatInboundAdapter;

impl WeChatInboundAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl InboundAdapter for WeChatInboundAdapter {
    fn parse(&self, raw_payload: &[u8]) -> Result<InboundMessage, AdapterError> {
        let payload_str = std::str::from_utf8(raw_payload)
            .map_err(|e| AdapterError::ParseError(format!("invalid UTF-8: {}", e)))?;

        // Parse XML payload
        let msg = parse_wechat_xml(payload_str)?;

        // Only handle text messages for now
        if msg.msg_type != "text" {
            return Err(AdapterError::ParseError(format!(
                "unsupported message type: {}",
                msg.msg_type
            )));
        }

        let sender = msg.from_user_name.clone();
        let thread_id = format!("wechat:{}:{}", msg.to_user_name, sender);

        Ok(InboundMessage {
            channel: Channel::WeChat,
            sender: sender.clone(),
            sender_name: None, // WeChat doesn't provide display name in webhook
            recipient: msg.to_user_name.clone(),
            subject: None,
            text_body: Some(msg.content.clone()),
            html_body: None,
            thread_id,
            message_id: Some(msg.msg_id.clone()),
            attachments: vec![],
            reply_to: vec![sender],
            raw_payload: raw_payload.to_vec(),
            metadata: ChannelMetadata {
                wechat_corp_id: Some(msg.to_user_name),
                wechat_user_id: Some(msg.from_user_name),
                wechat_agent_id: Some(msg.agent_id.to_string()),
                ..Default::default()
            },
        })
    }

    fn channel(&self) -> Channel {
        Channel::WeChat
    }
}

/// Adapter for sending messages via WeChat Work API.
#[derive(Debug)]
pub struct WeChatOutboundAdapter {
    pub corp_id: String,
    pub agent_id: String,
    pub secret: String,
    access_token_cache: RwLock<Option<CachedAccessToken>>,
}

#[derive(Debug, Clone)]
struct CachedAccessToken {
    token: String,
    expires_at: std::time::Instant,
}

impl WeChatOutboundAdapter {
    pub fn new(corp_id: String, agent_id: String, secret: String) -> Self {
        Self {
            corp_id,
            agent_id,
            secret,
            access_token_cache: RwLock::new(None),
        }
    }

    pub fn from_env() -> Result<Self, AdapterError> {
        let corp_id = std::env::var("WECHAT_CORP_ID")
            .map_err(|_| AdapterError::ConfigError("WECHAT_CORP_ID not set".to_string()))?;
        let agent_id = std::env::var("WECHAT_AGENT_ID")
            .map_err(|_| AdapterError::ConfigError("WECHAT_AGENT_ID not set".to_string()))?;
        let secret = std::env::var("WECHAT_SECRET")
            .map_err(|_| AdapterError::ConfigError("WECHAT_SECRET not set".to_string()))?;

        Ok(Self::new(corp_id, agent_id, secret))
    }

    /// Get access token, refreshing if expired.
    fn get_access_token(&self) -> Result<String, AdapterError> {
        // Check cache first
        {
            let cache = self.access_token_cache.read().unwrap();
            if let Some(ref cached) = *cache {
                if cached.expires_at > std::time::Instant::now() {
                    return Ok(cached.token.clone());
                }
            }
        }

        // Fetch new token with secrets
        let url = format!(
            "https://qyapi.weixin.qq.com/cgi-bin/gettoken?corpid={}&corpsecret={}",
            self.corp_id, self.secret
        );

        let client = reqwest::blocking::Client::new();
        let response: WeChatAccessTokenResponse = client
            .get(&url)
            .send()
            .map_err(|e| AdapterError::SendError(format!("token request failed: {}", e)))?
            .json()
            .map_err(|e| AdapterError::SendError(format!("token parse failed: {}", e)))?;

        if response.errcode != 0 {
            return Err(AdapterError::SendError(format!(
                "WeChat token error {}: {}",
                response.errcode,
                response.errmsg.unwrap_or_default()
            )));
        }

        let token = response
            .access_token
            .ok_or_else(|| AdapterError::SendError("no access_token in response".to_string()))?;

        // Cache with 110 minute expiry (tokens last 2 hours, refresh early)
        let expires_at =
            std::time::Instant::now() + std::time::Duration::from_secs(110 * 60);
        {
            let mut cache = self.access_token_cache.write().unwrap();
            *cache = Some(CachedAccessToken {
                token: token.clone(),
                expires_at,
            });
        }

        Ok(token)
    }
}

impl OutboundAdapter for WeChatOutboundAdapter {
    fn send(&self, message: &OutboundMessage) -> Result<SendResult, AdapterError> {
        let access_token = self.get_access_token()?;

        let user_id = message
            .to
            .first()
            .ok_or_else(|| AdapterError::ConfigError("no recipient specified".to_string()))?;

        let text = if message.text_body.is_empty() {
            message.html_body.clone()
        } else {
            message.text_body.clone()
        };

        let request = WeChatSendMessageRequest {
            touser: user_id.clone(),
            msgtype: "text".to_string(),
            agentid: self.agent_id.parse().unwrap_or(1),
            text: WeChatTextContent {
                content: text,
            },
        };

        let url = format!(
            "https://qyapi.weixin.qq.com/cgi-bin/message/send?access_token={}",
            access_token
        );

        let client = reqwest::blocking::Client::new();
        let response: WeChatSendResponse = client
            .post(&url)
            .json(&request)
            .send()
            .map_err(|e| AdapterError::SendError(format!("send request failed: {}", e)))?
            .json()
            .map_err(|e| AdapterError::SendError(format!("response parse failed: {}", e)))?;

        if response.errcode != 0 {
            return Ok(SendResult {
                success: false,
                message_id: String::new(),
                submitted_at: String::new(),
                error: Some(format!(
                    "WeChat error {}: {}",
                    response.errcode,
                    response.errmsg.unwrap_or_default()
                )),
            });
        }

        info!("sent WeChat message to user {}", user_id);

        Ok(SendResult {
            success: true,
            message_id: response.msgid.unwrap_or_default(),
            submitted_at: chrono::Utc::now().to_rfc3339(),
            error: None,
        })
    }

    fn channel(&self) -> Channel {
        Channel::WeChat
    }
}

// ============================================================================
// XML Parsing
// ============================================================================

/// Parsed WeChat message from XML.
#[derive(Debug, Clone)]
pub struct WeChatMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_type: String,
    pub content: String,
    pub msg_id: String,
    pub agent_id: i64,
}

/// Parse WeChat XML payload into structured message.
fn parse_wechat_xml(xml: &str) -> Result<WeChatMessage, AdapterError> {
    // Simple XML parsing without external crate
    // WeChat XML format:
    // <xml>
    //   <ToUserName><![CDATA[corp_id]]></ToUserName>
    //   <FromUserName><![CDATA[user_id]]></FromUserName>
    //   <CreateTime>1348831860</CreateTime>
    //   <MsgType><![CDATA[text]]></MsgType>
    //   <Content><![CDATA[message content]]></Content>
    //   <MsgId>1234567890123456</MsgId>
    //   <AgentID>1</AgentID>
    // </xml>

    fn extract_cdata(xml: &str, tag: &str) -> Option<String> {
        let start_tag = format!("<{}>", tag);
        let end_tag = format!("</{}>", tag);

        let start = xml.find(&start_tag)? + start_tag.len();
        let end = xml.find(&end_tag)?;
        let content = &xml[start..end];

        // Handle CDATA
        if content.starts_with("<![CDATA[") && content.ends_with("]]>") {
            Some(content[9..content.len() - 3].to_string())
        } else {
            Some(content.trim().to_string())
        }
    }

    fn extract_value(xml: &str, tag: &str) -> Option<String> {
        let start_tag = format!("<{}>", tag);
        let end_tag = format!("</{}>", tag);

        let start = xml.find(&start_tag)? + start_tag.len();
        let end = xml.find(&end_tag)?;
        Some(xml[start..end].trim().to_string())
    }

    let to_user_name = extract_cdata(xml, "ToUserName")
        .ok_or_else(|| AdapterError::MissingField("ToUserName"))?;
    let from_user_name = extract_cdata(xml, "FromUserName")
        .ok_or_else(|| AdapterError::MissingField("FromUserName"))?;
    let create_time = extract_value(xml, "CreateTime")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);
    let msg_type = extract_cdata(xml, "MsgType")
        .ok_or_else(|| AdapterError::MissingField("MsgType"))?;
    let content = extract_cdata(xml, "Content").unwrap_or_default();
    let msg_id = extract_value(xml, "MsgId").unwrap_or_default();
    let agent_id = extract_value(xml, "AgentID")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);

    Ok(WeChatMessage {
        to_user_name,
        from_user_name,
        create_time,
        msg_type,
        content,
        msg_id,
        agent_id,
    })
}

// ============================================================================
// WeChat API Types
// ============================================================================

#[derive(Debug, Deserialize)]
struct WeChatAccessTokenResponse {
    #[serde(default)]
    errcode: i32,
    errmsg: Option<String>,
    access_token: Option<String>,
    expires_in: Option<i64>,
}

#[derive(Debug, Serialize)]
struct WeChatSendMessageRequest {
    touser: String,
    msgtype: String,
    agentid: i64,
    text: WeChatTextContent,
}

#[derive(Debug, Serialize)]
struct WeChatTextContent {
    content: String,
}

#[derive(Debug, Deserialize)]
struct WeChatSendResponse {
    #[serde(default)]
    errcode: i32,
    errmsg: Option<String>,
    msgid: Option<String>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_text_message() {
        let xml = r#"<xml>
            <ToUserName><![CDATA[ww1234567890]]></ToUserName>
            <FromUserName><![CDATA[zhangsan]]></FromUserName>
            <CreateTime>1348831860</CreateTime>
            <MsgType><![CDATA[text]]></MsgType>
            <Content><![CDATA[Hello from WeChat!]]></Content>
            <MsgId>1234567890123456</MsgId>
            <AgentID>1</AgentID>
        </xml>"#;

        let adapter = WeChatInboundAdapter::new();
        let message = adapter.parse(xml.as_bytes()).unwrap();

        assert_eq!(message.channel, Channel::WeChat);
        assert_eq!(message.sender, "zhangsan");
        assert_eq!(message.text_body, Some("Hello from WeChat!".to_string()));
        assert_eq!(message.metadata.wechat_corp_id, Some("ww1234567890".to_string()));
        assert_eq!(message.metadata.wechat_user_id, Some("zhangsan".to_string()));
        assert_eq!(message.metadata.wechat_agent_id, Some("1".to_string()));
    }

    #[test]
    fn parse_xml_helper() {
        let xml = r#"<xml>
            <ToUserName><![CDATA[corp123]]></ToUserName>
            <FromUserName><![CDATA[user456]]></FromUserName>
            <CreateTime>1600000000</CreateTime>
            <MsgType><![CDATA[text]]></MsgType>
            <Content><![CDATA[Test message]]></Content>
            <MsgId>999</MsgId>
            <AgentID>2</AgentID>
        </xml>"#;

        let msg = parse_wechat_xml(xml).unwrap();
        assert_eq!(msg.to_user_name, "corp123");
        assert_eq!(msg.from_user_name, "user456");
        assert_eq!(msg.create_time, 1600000000);
        assert_eq!(msg.msg_type, "text");
        assert_eq!(msg.content, "Test message");
        assert_eq!(msg.msg_id, "999");
        assert_eq!(msg.agent_id, 2);
    }

    #[test]
    fn reject_non_text_message() {
        let xml = r#"<xml>
            <ToUserName><![CDATA[corp123]]></ToUserName>
            <FromUserName><![CDATA[user456]]></FromUserName>
            <CreateTime>1600000000</CreateTime>
            <MsgType><![CDATA[image]]></MsgType>
            <MsgId>999</MsgId>
            <AgentID>2</AgentID>
        </xml>"#;

        let adapter = WeChatInboundAdapter::new();
        let result = adapter.parse(xml.as_bytes());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unsupported message type"));
    }

    #[test]
    fn reject_voice_message() {
        let xml = r#"<xml>
            <ToUserName><![CDATA[corp123]]></ToUserName>
            <FromUserName><![CDATA[user456]]></FromUserName>
            <CreateTime>1600000000</CreateTime>
            <MsgType><![CDATA[voice]]></MsgType>
            <MsgId>999</MsgId>
            <AgentID>2</AgentID>
        </xml>"#;

        let adapter = WeChatInboundAdapter::new();
        let result = adapter.parse(xml.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn reject_event_message() {
        let xml = r#"<xml>
            <ToUserName><![CDATA[corp123]]></ToUserName>
            <FromUserName><![CDATA[user456]]></FromUserName>
            <CreateTime>1600000000</CreateTime>
            <MsgType><![CDATA[event]]></MsgType>
            <Event><![CDATA[subscribe]]></Event>
            <AgentID>2</AgentID>
        </xml>"#;

        let adapter = WeChatInboundAdapter::new();
        let result = adapter.parse(xml.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn parse_message_with_chinese_content() {
        let xml = r#"<xml>
            <ToUserName><![CDATA[ww企业ID]]></ToUserName>
            <FromUserName><![CDATA[张三]]></FromUserName>
            <CreateTime>1600000000</CreateTime>
            <MsgType><![CDATA[text]]></MsgType>
            <Content><![CDATA[你好，请帮我处理这个任务]]></Content>
            <MsgId>12345</MsgId>
            <AgentID>1000002</AgentID>
        </xml>"#;

        let adapter = WeChatInboundAdapter::new();
        let message = adapter.parse(xml.as_bytes()).unwrap();
        assert_eq!(message.text_body, Some("你好，请帮我处理这个任务".to_string()));
        assert_eq!(message.sender, "张三");
    }

    #[test]
    fn parse_message_with_multiline_content() {
        let xml = r#"<xml>
            <ToUserName><![CDATA[corp123]]></ToUserName>
            <FromUserName><![CDATA[user456]]></FromUserName>
            <CreateTime>1600000000</CreateTime>
            <MsgType><![CDATA[text]]></MsgType>
            <Content><![CDATA[Line 1
Line 2
Line 3]]></Content>
            <MsgId>999</MsgId>
            <AgentID>2</AgentID>
        </xml>"#;

        let adapter = WeChatInboundAdapter::new();
        let message = adapter.parse(xml.as_bytes()).unwrap();
        assert!(message.text_body.as_ref().unwrap().contains("Line 1"));
        assert!(message.text_body.as_ref().unwrap().contains("Line 2"));
        assert!(message.text_body.as_ref().unwrap().contains("Line 3"));
    }

    #[test]
    fn thread_id_format() {
        let xml = r#"<xml>
            <ToUserName><![CDATA[ww12345]]></ToUserName>
            <FromUserName><![CDATA[zhangsan]]></FromUserName>
            <CreateTime>1600000000</CreateTime>
            <MsgType><![CDATA[text]]></MsgType>
            <Content><![CDATA[Test]]></Content>
            <MsgId>999</MsgId>
            <AgentID>1</AgentID>
        </xml>"#;

        let adapter = WeChatInboundAdapter::new();
        let message = adapter.parse(xml.as_bytes()).unwrap();
        assert_eq!(message.thread_id, "wechat:ww12345:zhangsan");
    }

    #[test]
    fn adapter_channel_is_wechat() {
        let adapter = WeChatInboundAdapter::new();
        assert_eq!(adapter.channel(), Channel::WeChat);
    }

    #[test]
    fn parse_invalid_utf8() {
        let invalid_bytes = vec![0xff, 0xfe, 0x00, 0x01];
        let adapter = WeChatInboundAdapter::new();
        let result = adapter.parse(&invalid_bytes);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("UTF-8"));
    }

    #[test]
    fn parse_invalid_xml() {
        let invalid_xml = b"<xml><broken";
        let adapter = WeChatInboundAdapter::new();
        let result = adapter.parse(invalid_xml);
        assert!(result.is_err());
    }

    #[test]
    fn parse_missing_required_fields() {
        let xml = r#"<xml>
            <ToUserName><![CDATA[corp123]]></ToUserName>
        </xml>"#;

        let adapter = WeChatInboundAdapter::new();
        let result = adapter.parse(xml.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn parse_empty_content() {
        let xml = r#"<xml>
            <ToUserName><![CDATA[corp123]]></ToUserName>
            <FromUserName><![CDATA[user456]]></FromUserName>
            <CreateTime>1600000000</CreateTime>
            <MsgType><![CDATA[text]]></MsgType>
            <Content><![CDATA[]]></Content>
            <MsgId>999</MsgId>
            <AgentID>2</AgentID>
        </xml>"#;

        let adapter = WeChatInboundAdapter::new();
        let message = adapter.parse(xml.as_bytes()).unwrap();
        assert_eq!(message.text_body, Some("".to_string()));
    }

    #[test]
    fn metadata_fields_populated() {
        let xml = r#"<xml>
            <ToUserName><![CDATA[ww9876543210]]></ToUserName>
            <FromUserName><![CDATA[lisi]]></FromUserName>
            <CreateTime>1600000000</CreateTime>
            <MsgType><![CDATA[text]]></MsgType>
            <Content><![CDATA[Hello]]></Content>
            <MsgId>55555</MsgId>
            <AgentID>1000005</AgentID>
        </xml>"#;

        let adapter = WeChatInboundAdapter::new();
        let message = adapter.parse(xml.as_bytes()).unwrap();

        assert_eq!(message.metadata.wechat_corp_id, Some("ww9876543210".to_string()));
        assert_eq!(message.metadata.wechat_user_id, Some("lisi".to_string()));
        assert_eq!(message.metadata.wechat_agent_id, Some("1000005".to_string()));
    }

    #[test]
    fn reply_to_is_sender() {
        let xml = r#"<xml>
            <ToUserName><![CDATA[corp]]></ToUserName>
            <FromUserName><![CDATA[sender123]]></FromUserName>
            <CreateTime>1600000000</CreateTime>
            <MsgType><![CDATA[text]]></MsgType>
            <Content><![CDATA[Hi]]></Content>
            <MsgId>1</MsgId>
            <AgentID>1</AgentID>
        </xml>"#;

        let adapter = WeChatInboundAdapter::new();
        let message = adapter.parse(xml.as_bytes()).unwrap();
        assert_eq!(message.reply_to, vec!["sender123".to_string()]);
    }

    #[test]
    fn raw_payload_preserved() {
        let xml = r#"<xml>
            <ToUserName><![CDATA[corp]]></ToUserName>
            <FromUserName><![CDATA[user]]></FromUserName>
            <CreateTime>1600000000</CreateTime>
            <MsgType><![CDATA[text]]></MsgType>
            <Content><![CDATA[Test]]></Content>
            <MsgId>1</MsgId>
            <AgentID>1</AgentID>
        </xml>"#;

        let adapter = WeChatInboundAdapter::new();
        let message = adapter.parse(xml.as_bytes()).unwrap();
        assert_eq!(message.raw_payload, xml.as_bytes());
    }

    // ==================== Outbound Adapter Tests ====================

    #[test]
    fn outbound_adapter_channel_is_wechat() {
        let adapter = WeChatOutboundAdapter::new(
            "corp123".to_string(),
            "1000001".to_string(),
            "secret".to_string(),
        );
        assert_eq!(adapter.channel(), Channel::WeChat);
    }

    #[test]
    fn outbound_adapter_caches_token() {
        let adapter = WeChatOutboundAdapter::new(
            "corp123".to_string(),
            "1000001".to_string(),
            "secret".to_string(),
        );
        // Initially no cached token
        {
            let cache = adapter.access_token_cache.read().unwrap();
            assert!(cache.is_none());
        }
    }

    // ==================== XML Helper Tests ====================

    #[test]
    fn parse_wechat_xml_extracts_all_fields() {
        let xml = r#"<xml>
            <ToUserName><![CDATA[to_corp]]></ToUserName>
            <FromUserName><![CDATA[from_user]]></FromUserName>
            <CreateTime>1234567890</CreateTime>
            <MsgType><![CDATA[text]]></MsgType>
            <Content><![CDATA[Message content here]]></Content>
            <MsgId>9999999</MsgId>
            <AgentID>42</AgentID>
        </xml>"#;

        let msg = parse_wechat_xml(xml).unwrap();
        assert_eq!(msg.to_user_name, "to_corp");
        assert_eq!(msg.from_user_name, "from_user");
        assert_eq!(msg.create_time, 1234567890);
        assert_eq!(msg.msg_type, "text");
        assert_eq!(msg.content, "Message content here");
        assert_eq!(msg.msg_id, "9999999");
        assert_eq!(msg.agent_id, 42);
    }

    #[test]
    fn parse_wechat_xml_handles_special_characters() {
        let xml = r#"<xml>
            <ToUserName><![CDATA[corp]]></ToUserName>
            <FromUserName><![CDATA[user]]></FromUserName>
            <CreateTime>1000</CreateTime>
            <MsgType><![CDATA[text]]></MsgType>
            <Content><![CDATA[Test with <special> & "characters"]]></Content>
            <MsgId>1</MsgId>
            <AgentID>1</AgentID>
        </xml>"#;

        let msg = parse_wechat_xml(xml).unwrap();
        assert_eq!(msg.content, r#"Test with <special> & "characters""#);
    }

    #[test]
    fn parse_wechat_xml_zero_agent_id() {
        let xml = r#"<xml>
            <ToUserName><![CDATA[corp]]></ToUserName>
            <FromUserName><![CDATA[user]]></FromUserName>
            <CreateTime>1000</CreateTime>
            <MsgType><![CDATA[text]]></MsgType>
            <Content><![CDATA[Test]]></Content>
            <MsgId>1</MsgId>
            <AgentID>0</AgentID>
        </xml>"#;

        let msg = parse_wechat_xml(xml).unwrap();
        assert_eq!(msg.agent_id, 0);
    }
}
