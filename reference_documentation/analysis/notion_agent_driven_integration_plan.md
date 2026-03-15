# Notion Agent-Driven Integration Plan

> Status: **ALL PHASES IMPLEMENTED**
> Created: 2026-03-09
> Updated: 2026-03-10
> Branch: `cross-channel-capabilities`

## Implementation Status

- [x] Phase 1: Agent-Driven Inbox Detection (Core) - **COMPLETED**
- [x] Phase 2: Notion API Integration (Operations) - **COMPLETED**
- [x] Phase 3: Complete InboundMessage Flow - **COMPLETED**

### Implemented Files

```
notion_browser/
├── agent_detector.rs  # LLM-based inbox analysis (Phase 1)
├── api_client.rs      # Notion API client for pages/comments (Phase 2)
├── browser.rs         # browser-use CLI wrapper (existing)
├── models.rs          # Data structures (existing)
├── oauth_store.rs     # OAuth token storage in MongoDB (Phase 2)
├── operations.rs      # Hybrid API/browser operations (Phase 2)
├── parser.rs          # Legacy regex parsing (existing)
├── poller.rs          # Polling loop with detection mode switch (Phase 1+3)
└── store.rs           # MongoDB deduplication (existing)
```

### Key Features

1. **Detection Mode Switch**: `NOTION_DETECTION_MODE=agent_driven|hardcoded`
2. **LLM-based Analysis**: Uses Claude Haiku via Azure Foundry or direct Anthropic API
3. **OAuth Token Storage**: MongoDB-backed token persistence per workspace
4. **Hybrid Operations**: API-first with browser fallback for page reading
5. **Queue Integration**: `enqueue_message()` properly sends to ServiceBus

## 1. 背景与问题

### 1.1 当前实现

位置: `DoWhiz_service/scheduler_module/src/notion_browser/`

```
notion_browser/
├── agent_detector.rs  # LLM-based inbox screenshot analysis
├── api_client.rs      # Notion REST API client
├── browser.rs         # browser-use CLI wrapper
├── models.rs          # Data structures
├── oauth_store.rs     # MongoDB OAuth token storage
├── operations.rs      # Hybrid API/browser operations layer
├── parser.rs          # Legacy regex parsing utilities
├── poller.rs          # Polling loop with detection mode switch
└── store.rs           # MongoDB notification deduplication
```

**问题:**
1. **硬编码 regex patterns** - Notion UI 变化就会失效
2. **Inbox 解析脆弱** - 多种日期格式、元素布局变化
3. **Onboarding popup** - 意外弹窗阻断流程
4. **维护成本高** - 每次 UI 变化都需要调试修复

### 1.2 API 方案不可行的原因

| Notion API 能力 | 状态 |
|----------------|------|
| 读取页面内容 | 可以 |
| 读取页面 comments | 可以 |
| 回复 comment thread | 可以 |
| **读取 Inbox/通知** | **不支持** |
| **获取 @mention 通知** | **不支持** |

结论: **必须用浏览器检测 @mentions**，但可以用 API 执行操作。

### 1.3 复杂度分析

| 方案 | 时间复杂度 | 每次轮询 |
|-----|-----------|---------|
| Browser Inbox 检测 | O(1) | 检查 1 个 Inbox |
| API 轮询所有页面 | O(W×P) | W 工作区 × P 页面 |

Browser Inbox 方案天然 scalable，因为 Notion Inbox 已聚合所有通知。

## 2. 目标架构

```
┌─────────────────────────────────────────────────────────────────┐
│                    Notion Integration v2                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  检测层 (Agent-Driven Browser)                           │    │
│  │  ├─ browser-use 打开 Inbox                               │    │
│  │  ├─ 截图 → LLM (Haiku) 分析                              │    │
│  │  ├─ 输出结构化 mentions 列表                              │    │
│  │  └─ 自动处理弹窗、UI 变化                                 │    │
│  └─────────────────────────────────────────────────────────┘    │
│                              │                                   │
│                              ▼                                   │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  消息队列 (Service Bus)                                   │    │
│  │  └─ InboundMessage { channel: Notion, ... }              │    │
│  └─────────────────────────────────────────────────────────┘    │
│                              │                                   │
│                              ▼                                   │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  Worker (Agent 执行任务)                                  │    │
│  │  ├─ 读取页面内容 (API 优先，browser fallback)             │    │
│  │  ├─ 执行用户请求                                          │    │
│  │  └─ 回复 comment (API 优先)                               │    │
│  └─────────────────────────────────────────────────────────┘    │
│                                                                  │
├─────────────────────────────────────────────────────────────────┤
│  辅助组件                                                        │
│  ├─ OAuth Token Store (MongoDB) - 存储各工作区授权              │
│  ├─ Notion API Client - 封装 API 调用                           │
│  └─ Browser Session Manager - 管理 cookie/登录状态              │
└─────────────────────────────────────────────────────────────────┘
```

## 3. 实现计划

### Phase 1: Agent-Driven Inbox 检测 (核心)

**目标:** 用 LLM 替代硬编码 regex 解析 Inbox

#### 3.1.1 新增文件

```
notion_browser/
├── agent_detector.rs    # 新: LLM-based Inbox 分析
├── screenshot.rs        # 新: 截图管理
├── browser.rs           # 保留: browser-use wrapper
├── poller.rs            # 修改: 使用 agent_detector
├── store.rs             # 保留
└── models.rs            # 扩展
```

#### 3.1.2 Agent Detector 设计

```rust
// agent_detector.rs

pub struct AgentDetector {
    llm_client: AnthropicClient,  // Claude Haiku
    model: String,                 // claude-haiku-4-5-20251001
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectedMention {
    pub workspace_name: String,
    pub page_title: String,
    pub page_url: Option<String>,
    pub mentioner: String,
    pub snippet: String,
    pub timestamp: String,
    pub element_index: Option<u32>,  // 用于后续点击
}

impl AgentDetector {
    /// 分析 Inbox 截图，提取 mentions
    pub async fn analyze_inbox_screenshot(
        &self,
        screenshot_path: &Path,
        browser_state: &str,  // browser-use state 输出
    ) -> Result<Vec<DetectedMention>, NotionError> {
        let prompt = self.build_analysis_prompt(browser_state);
        let response = self.llm_client
            .messages()
            .create(MessagesRequest {
                model: &self.model,
                max_tokens: 2000,
                messages: vec![
                    Message {
                        role: Role::User,
                        content: vec![
                            ContentBlock::Image {
                                source: ImageSource::Base64 {
                                    media_type: "image/png".into(),
                                    data: base64_encode(screenshot_path)?,
                                },
                            },
                            ContentBlock::Text { text: prompt },
                        ],
                    },
                ],
                ..Default::default()
            })
            .await?;

        self.parse_llm_response(&response)
    }

    fn build_analysis_prompt(&self, browser_state: &str) -> String {
        format!(r#"
You are analyzing a Notion Inbox screenshot to detect @mentions.

The browser state shows interactive elements:
```
{browser_state}
```

Extract ALL notification items that mention the current user. For each mention, provide:
- workspace_name: The workspace this notification is from
- page_title: The page where the mention occurred
- mentioner: Who mentioned the user
- snippet: The text content of the mention
- timestamp: When it occurred (e.g., "2d", "Yesterday", "Mar 5")
- element_index: The clickable element index from browser state (if identifiable)

Respond in JSON format:
```json
{{
  "mentions": [
    {{
      "workspace_name": "...",
      "page_title": "...",
      "mentioner": "...",
      "snippet": "...",
      "timestamp": "...",
      "element_index": 42
    }}
  ],
  "has_more": false,
  "scroll_needed": false
}}
```

If no mentions are visible, return empty array.
If scrolling might reveal more, set scroll_needed: true.
"#, browser_state = browser_state)
    }
}
```

#### 3.1.3 Poller 集成

```rust
// poller.rs 修改

impl NotionBrowserPoller {
    async fn check_inbox_with_agent(&mut self) -> Result<Vec<NotionNotification>, NotionError> {
        // 1. 导航到 Inbox
        self.browser.navigate_to_inbox().await?;

        // 2. 截图
        let screenshot_path = self.browser.take_screenshot("/tmp/notion_inbox.png").await?;

        // 3. 获取 browser state
        let state = self.browser.get_state().await?;

        // 4. Agent 分析
        let mentions = self.agent_detector
            .analyze_inbox_screenshot(&screenshot_path, &state)
            .await?;

        // 5. 处理 scroll_needed
        // ... 如果需要滚动，重复步骤 2-4

        // 6. 转换为 NotionNotification
        Ok(mentions.into_iter().map(|m| m.into()).collect())
    }
}
```

#### 3.1.4 处理 UI 异常 (弹窗等)

```rust
// agent_detector.rs

impl AgentDetector {
    /// 检测并处理意外 UI 状态
    pub async fn handle_unexpected_ui(
        &self,
        screenshot_path: &Path,
        browser_state: &str,
    ) -> Result<UiAction, NotionError> {
        let prompt = r#"
Analyze this Notion screenshot. Is there any popup, modal, or overlay blocking the main content?

If yes, identify how to dismiss it:
- Look for "X" close buttons, "Skip", "Maybe later", "Got it" buttons
- Identify the element index to click

Respond in JSON:
```json
{
  "blocked": true/false,
  "blocker_type": "onboarding_popup" | "modal" | "tooltip" | null,
  "dismiss_action": {
    "type": "click" | "press_escape" | "none",
    "element_index": 42
  }
}
```
"#;
        // ... LLM 调用并解析响应
    }
}

pub enum UiAction {
    None,
    Click(u32),
    PressEscape,
    Refresh,
}
```

### Phase 2: Notion API 集成 (操作层)

**目标:** 用 API 执行页面读写，提高可靠性

#### 3.2.1 Public Integration 设置

1. 在 Notion Developer Portal 创建 Public Integration
2. 配置 OAuth redirect URI: `https://dowhiz.com/oauth/notion/callback`
3. 申请权限:
   - Read content
   - Update content
   - Read comments
   - Insert comments

#### 3.2.2 OAuth Token 存储

```rust
// notion_api/oauth_store.rs

pub struct NotionOAuthStore {
    collection: Collection<NotionOAuthToken>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionOAuthToken {
    pub workspace_id: String,
    pub workspace_name: String,
    pub access_token: String,  // encrypted
    pub bot_id: String,
    pub owner_user_id: String,
    pub created_at: DateTime<Utc>,
    // Notion tokens don't expire, but we track for audit
}

impl NotionOAuthStore {
    pub async fn get_token(&self, workspace_id: &str) -> Option<String>;
    pub async fn store_token(&self, token: NotionOAuthToken) -> Result<()>;
    pub async fn revoke_token(&self, workspace_id: &str) -> Result<()>;
}
```

#### 3.2.3 Notion API Client

```rust
// notion_api/client.rs

pub struct NotionApiClient {
    http_client: reqwest::Client,
    oauth_store: NotionOAuthStore,
}

impl NotionApiClient {
    /// 读取页面内容
    pub async fn get_page_content(
        &self,
        workspace_id: &str,
        page_id: &str,
    ) -> Result<PageContent, NotionApiError> {
        let token = self.oauth_store.get_token(workspace_id).await
            .ok_or(NotionApiError::NoAuthorization)?;

        // GET https://api.notion.com/v1/blocks/{page_id}/children
        // ...
    }

    /// 读取页面 comments
    pub async fn get_comments(
        &self,
        workspace_id: &str,
        page_id: &str,
    ) -> Result<Vec<Comment>, NotionApiError>;

    /// 回复 comment thread
    pub async fn reply_to_comment(
        &self,
        workspace_id: &str,
        discussion_id: &str,
        content: &str,
    ) -> Result<Comment, NotionApiError>;

    /// 更新页面 block
    pub async fn update_block(
        &self,
        workspace_id: &str,
        block_id: &str,
        content: BlockContent,
    ) -> Result<(), NotionApiError>;
}
```

#### 3.2.4 混合操作策略

```rust
// notion_operations.rs

pub struct NotionOperations {
    api_client: NotionApiClient,
    browser: NotionBrowser,
}

impl NotionOperations {
    /// 读取页面 - API 优先
    pub async fn read_page(&self, workspace_id: &str, page_id: &str) -> Result<PageContent> {
        match self.api_client.get_page_content(workspace_id, page_id).await {
            Ok(content) => Ok(content),
            Err(NotionApiError::NoAuthorization) => {
                info!("No API token for workspace, falling back to browser");
                self.browser.read_page_content(page_id).await
            }
            Err(e) => Err(e.into()),
        }
    }

    /// 回复 comment - API 优先
    pub async fn reply_comment(
        &self,
        workspace_id: &str,
        discussion_id: &str,
        content: &str,
    ) -> Result<()> {
        match self.api_client.reply_to_comment(workspace_id, discussion_id, content).await {
            Ok(_) => Ok(()),
            Err(NotionApiError::NoAuthorization) => {
                self.browser.reply_to_comment_ui(discussion_id, content).await
            }
            Err(e) => Err(e.into()),
        }
    }
}
```

### Phase 3: 完善 InboundMessage 流程

**目标:** 修复 `enqueue_message()` TODO，完成端到端流程

#### 3.3.1 当前 TODO

```rust
// poller.rs:enqueue_message()
async fn enqueue_message(&self, message: InboundMessage) -> Result<(), NotionError> {
    info!("Enqueueing Notion message from {} about {}", ...);
    // TODO: Integrate with actual queue
    let _ = self.queue;
    Ok(())
}
```

#### 3.3.2 实现

```rust
async fn enqueue_message(&self, message: InboundMessage) -> Result<(), NotionError> {
    let Some(ref queue) = self.queue else {
        warn!("No queue configured, message will not be processed");
        return Ok(());
    };

    let envelope = IngestionEnvelope {
        id: Uuid::new_v4().to_string(),
        employee_id: self.config.employee_id.clone(),
        channel: "notion".to_string(),
        timestamp: Utc::now(),
        payload: serde_json::to_value(&message)?,
    };

    queue.send(&envelope).await
        .map_err(|e| NotionError::QueueError(e.to_string()))?;

    info!(
        "Enqueued Notion message: {} mentioned by {} on page {}",
        envelope.id,
        message.sender_name.as_deref().unwrap_or("unknown"),
        message.metadata.get("page_title").unwrap_or(&serde_json::Value::Null)
    );

    Ok(())
}
```

## 4. 成本估算

### 4.1 LLM 推理成本 (Agent-Driven 检测)

| 参数 | 值 |
|-----|---|
| 模型 | Claude Haiku |
| 每次截图分析 tokens | ~2000 input + ~500 output |
| 轮询间隔 | 45 秒 |
| 每小时调用次数 | 80 |
| 每日调用次数 | 1920 |

**月成本估算:**
```
Input:  1920 × 30 × 2000 tokens = 115.2M tokens × $0.25/M = $28.80
Output: 1920 × 30 × 500 tokens  = 28.8M tokens × $1.25/M  = $36.00
Vision: 1920 × 30 × 1000 tokens = 57.6M tokens × $0.25/M  = $14.40
────────────────────────────────────────────────────────────────
Total: ~$80/month
```

### 4.2 对比: 人工维护成本

- 每次 Notion UI 变化需要 1-4 小时调试
- 假设每月 2 次 UI 变化
- 工程师时间成本: 4-16 小时/月

**结论:** Agent-driven 方案成本合理，且更可靠。

## 5. 测试计划

### 5.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_agent_detector_parses_mentions() {
        let detector = AgentDetector::new_mock();
        let screenshot = load_test_screenshot("inbox_with_mentions.png");
        let state = load_test_state("inbox_state.txt");

        let mentions = detector.analyze_inbox_screenshot(&screenshot, &state).await.unwrap();

        assert_eq!(mentions.len(), 3);
        assert_eq!(mentions[0].mentioner, "Oliver Liu");
    }

    #[tokio::test]
    async fn test_agent_handles_popup() {
        let detector = AgentDetector::new_mock();
        let screenshot = load_test_screenshot("inbox_with_onboarding_popup.png");

        let action = detector.handle_unexpected_ui(&screenshot, "").await.unwrap();

        assert!(matches!(action, UiAction::Click(_)));
    }
}
```

### 5.2 Integration Tests

```rust
#[tokio::test]
#[ignore = "requires browser and LLM"]
async fn test_e2e_mention_detection() {
    // 1. 启动 browser session
    // 2. 导航到 Inbox
    // 3. Agent 分析
    // 4. 验证检测到的 mentions
}
```

### 5.3 Manual E2E Test Checklist

- [ ] 从其他账号 @mention Oliver
- [ ] 等待 poller 检测
- [ ] 验证 InboundMessage 入队
- [ ] 验证 Worker 处理
- [ ] 验证回复 comment

## 6. 迁移计划

### 6.1 Phase 1 完成标准

- [ ] `AgentDetector` 实现完成
- [ ] 单元测试覆盖主要场景
- [ ] Poller 集成 agent detector
- [ ] 本地 E2E 测试通过

### 6.2 Phase 2 完成标准

- [ ] Public Integration 创建并通过审核
- [ ] OAuth 流程实现
- [ ] Token 存储集成
- [ ] API client 实现
- [ ] 混合操作策略测试

### 6.3 Phase 3 完成标准

- [ ] `enqueue_message()` 完成
- [ ] Service Bus 集成测试
- [ ] Worker 处理 Notion 消息
- [ ] 完整 E2E 流程验证

## 7. 回滚计划

保留现有 hardcoded 实现作为 fallback:

```rust
pub struct NotionBrowserPoller {
    detection_mode: DetectionMode,
    // ...
}

pub enum DetectionMode {
    AgentDriven,   // 新方案
    Hardcoded,     // 现有方案，作为 fallback
}

impl NotionBrowserPoller {
    async fn check_inbox(&mut self) -> Result<Vec<NotionNotification>> {
        match self.detection_mode {
            DetectionMode::AgentDriven => self.check_inbox_with_agent().await,
            DetectionMode::Hardcoded => self.check_inbox_hardcoded().await,
        }
    }
}
```

环境变量控制:
```bash
NOTION_DETECTION_MODE=agent_driven  # 或 hardcoded
```

## 8. 开放问题

1. **Public Integration 审核时间** - Notion 安全审核可能需要数周
2. **多账号支持** - 未来是否需要支持多个 Oliver 账号?
3. **通知去重** - 跨 session 的通知 ID 稳定性?
4. **Rate limiting** - LLM 调用的 rate limit 策略?

## 9. 参考资料

- [Notion API Authorization](https://developers.notion.com/docs/authorization)
- [Notion Comments API](https://developers.notion.com/docs/working-with-comments)
- [browser-use CLI Documentation](https://github.com/anthropics/browser-use)
- 项目内 Notion 实现: `DoWhiz_service/scheduler_module/src/notion_browser/`
- 内存笔记: `/root/.claude/projects/-mnt-d-dev-DoWhiz/memory/notion_browser_notes.md`
