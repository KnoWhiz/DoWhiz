# Google Drive Push Notifications 生产环境配置清单

## 概述

启用 Google Drive Push Notifications 后，当 Google Docs/Sheets/Slides 有新评论时，Google 会主动推送通知到我们的 webhook，而不是依赖 15 秒轮询，实现近实时响应。

## 当前状态

- [x] 代码已实现 (`feature/google-drive-push-notifications` 分支)
- [x] Caddy 路由已配置 (`/webhooks/*` → `localhost:9100`)
- [ ] **待配置**: VM 上的 `.env` 环境变量
- [ ] **待验证**: Google Cloud Console 域名验证
- [ ] **待测试**: Webhook 端点可访问性

---

## 配置步骤

### 步骤 1: 更新 VM 上的 .env 文件

SSH 到 Azure VM 后执行：

```bash
cd /home/azureuser/server/DoWhiz_service

# 编辑 .env 文件，添加或修改以下两行：
GOOGLE_DRIVE_PUSH_ENABLED=true
GOOGLE_DRIVE_WEBHOOK_URL=https://api.production1.dowhiz.com/webhooks/google-drive-changes
```

### 步骤 2: 验证 Google Cloud Console 域名

1. 打开 [Google Cloud Console - Domain Verification](https://console.cloud.google.com/apis/credentials/domainverification)
2. 确认 `dowhiz.com` 已经在验证列表中
3. 如果没有，需要通过 Google Search Console 验证域名所有权

**注意**: OAuth Authorized Domains（在 OAuth consent screen 里）和 Domain Verification 是不同的配置。Push notifications 需要 Domain Verification。

### 步骤 3: 确认 inbound-gateway 端口

检查 inbound-gateway 是否运行在端口 9100：

```bash
export HOME=/home/azureuser/server
pm2 list

# 检查 inbound-gateway 的启动参数
pm2 show dowhiz-inbound-gateway
```

当前 Caddy 配置：
```
api.production1.dowhiz.com {
  handle /service/* {
    reverse_proxy localhost:9001   # rust_service worker
  }
  reverse_proxy localhost:9100     # inbound-gateway (包括 /webhooks/*)
}
```

如果 inbound-gateway 运行在其他端口，需要调整 Caddy 或启动参数。

### 步骤 4: 重启服务

```bash
export HOME=/home/azureuser/server
pm2 restart dowhiz-inbound-gateway
```

### 步骤 5: 测试 Webhook 端点

```bash
# 测试端点是否可访问
curl -X POST https://api.production1.dowhiz.com/webhooks/google-drive-changes \
  -H "Content-Type: application/json" \
  -H "X-Goog-Channel-ID: test-channel" \
  -H "X-Goog-Resource-ID: test-resource" \
  -H "X-Goog-Resource-State: sync"

# 期望返回: {"status":"ok"}
```

---

## 工作原理

```
Google Drive 文件有新评论
         ↓
Google 发送 POST 到 webhook URL
         ↓
https://api.production1.dowhiz.com/webhooks/google-drive-changes
         ↓
Caddy 反向代理 → localhost:9100
         ↓
inbound_gateway 的 handle_google_drive_webhook() 处理
         ↓
触发立即轮询该文件的评论
         ↓
评论入队到 Service Bus → Worker 处理
```

---

## 相关代码文件

| 文件 | 说明 |
|------|------|
| `scheduler_module/src/google_drive_changes.rs` | Push notification 核心逻辑 |
| `scheduler_module/src/bin/inbound_gateway/google_drive_webhook.rs` | Webhook 处理器 |
| `scheduler_module/src/bin/inbound_gateway.rs` | 路由注册 (`/webhooks/google-drive-changes`) |

---

## 环境变量说明

| 变量 | 说明 | 示例值 |
|------|------|--------|
| `GOOGLE_DRIVE_PUSH_ENABLED` | 是否启用 push notifications | `true` |
| `GOOGLE_DRIVE_WEBHOOK_URL` | Webhook 完整 URL | `https://api.production1.dowhiz.com/webhooks/google-drive-changes` |
| `GOOGLE_DRIVE_CHANNEL_EXPIRATION_SECS` | Channel 过期时间（可选） | `3600` (默认 1 小时) |

---

## 故障排查

### 问题 1: Webhook 收不到通知

检查：
1. 域名是否在 Google Cloud Console 验证过
2. Webhook URL 是否可以公网访问
3. Caddy 是否正确路由到 inbound-gateway

```bash
# 检查 inbound-gateway 日志
tail -100 /home/azureuser/server/.pm2/logs/dowhiz-inbound-gateway-out.log | grep -i "drive\|webhook"
```

### 问题 2: Channel 过期

Push notification channel 默认 1 小时过期，系统会自动续期。如果续期失败，检查：
1. Google API 配额是否用完
2. Service account 权限是否正确

---

## 参考链接

- [Google Drive API - Push Notifications](https://developers.google.com/drive/api/guides/push)
- [Google Cloud Console - Domain Verification](https://console.cloud.google.com/apis/credentials/domainverification)
