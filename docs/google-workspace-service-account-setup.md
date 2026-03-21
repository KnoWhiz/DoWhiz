# Google Workspace Service Account 配置指南

完整的 copy-paste 步骤，配置 Service Account + Domain-Wide Delegation。

---

## 第一步：创建 Service Account (Google Cloud Console)

### 1.1 进入 Service Accounts 页面

打开: https://console.cloud.google.com/iam-admin/serviceaccounts

选择你的项目（如果没有，先创建一个）。

### 1.2 创建 Service Account

1. 点击 **+ CREATE SERVICE ACCOUNT**
2. 填写:
   - Service account name: `dowhiz-digital-employee`
   - Service account ID: `dowhiz-digital-employee` (自动生成)
   - Description: `DoWhiz digital employee for Google Workspace operations`
3. 点击 **CREATE AND CONTINUE**
4. 跳过 "Grant this service account access to project" → 点击 **CONTINUE**
5. 跳过 "Grant users access to this service account" → 点击 **DONE**

### 1.3 启用 Domain-Wide Delegation

1. 在 Service Accounts 列表中，点击刚创建的 `dowhiz-digital-employee`
2. 点击 **DETAILS** 标签
3. 展开 **Advanced settings**
4. 勾选 **Enable Google Workspace Domain-wide Delegation**
5. 点击 **SAVE**
6. **记录 Client ID** (数字，如 `123456789012345678901`)

### 1.4 创建 JSON Key

1. 点击 **KEYS** 标签
2. 点击 **ADD KEY** → **Create new key**
3. 选择 **JSON** 格式
4. 点击 **CREATE**
5. JSON 文件会自动下载，**安全保存**

---

## 第二步：启用 Google APIs (Google Cloud Console)

打开: https://console.cloud.google.com/apis/library

搜索并启用以下 API（点击每个 → **ENABLE**）:

- Google Calendar API
- Google Tasks API
- People API
- Google Drive API
- Google Docs API
- Google Sheets API
- Google Slides API

---

## 第三步：授权 Domain-Wide Delegation (Google Workspace Admin Console)

### 3.1 进入 API Controls

打开: https://admin.google.com/ac/owl/domainwidedelegation

或者: Google Admin Console → Security → Access and data control → API controls → Manage Domain Wide Delegation

### 3.2 添加授权

1. 点击 **Add new**
2. **Client ID**: 粘贴第 1.3 步记录的 Client ID
3. **OAuth scopes**: 复制粘贴以下内容（一行，逗号分隔）:

```
https://www.googleapis.com/auth/calendar,https://www.googleapis.com/auth/calendar.events,https://www.googleapis.com/auth/tasks,https://www.googleapis.com/auth/contacts.readonly,https://www.googleapis.com/auth/documents,https://www.googleapis.com/auth/drive,https://www.googleapis.com/auth/spreadsheets,https://www.googleapis.com/auth/presentations
```

4. 点击 **AUTHORIZE**

---

## 第四步：配置环境变量

### 4.1 上传 Service Account JSON 到服务器

将下载的 JSON 文件上传到两个 VM:

```bash
# Staging VM
scp /path/to/service-account.json dowhizstaging:~/server/DoWhiz/DoWhiz_service/google-service-account.json

# Production VM
scp /path/to/service-account.json dowhizprod1:~/server/DoWhiz/DoWhiz_service/google-service-account.json
```

### 4.2 更新 GitHub Secrets

#### ENV_COMMON (共享配置)

在 GitHub → Settings → Secrets and variables → Actions 中，编辑 `ENV_COMMON`，添加:

```bash
# Google Workspace Service Account
GOOGLE_SERVICE_ACCOUNT_JSON="/home/azureuser/server/DoWhiz/DoWhiz_service/google-service-account.json"
```

#### ENV_STAGING (Staging 专用)

编辑 `ENV_STAGING`，添加:

```bash
# Staging: Boiled Egg 使用 dowhiz@deep-tutor.com 的 Google Workspace
GOOGLE_SERVICE_ACCOUNT_SUBJECT_BOILED_EGG="dowhiz@deep-tutor.com"
GOOGLE_SERVICE_ACCOUNT_SUBJECT="dowhiz@deep-tutor.com"
```

#### ENV_PROD (Production 专用)

编辑 `ENV_PROD`，添加:

```bash
# Production: Little Bear 使用 oliver@dowhiz.com 的 Google Workspace
GOOGLE_SERVICE_ACCOUNT_SUBJECT_LITTLE_BEAR="oliver@dowhiz.com"
GOOGLE_SERVICE_ACCOUNT_SUBJECT="oliver@dowhiz.com"
```

---

## 第五步：验证配置

### 5.1 SSH 到 VM 验证

```bash
# SSH 到 staging
ssh dowhizstaging

# 检查 JSON 文件存在
ls -la ~/server/DoWhiz/DoWhiz_service/google-service-account.json

# 检查环境变量 (重新部署后)
source ~/server/DoWhiz/DoWhiz_service/.env
echo "SA JSON: $GOOGLE_SERVICE_ACCOUNT_JSON"
echo "Subject: $GOOGLE_SERVICE_ACCOUNT_SUBJECT"
```

### 5.2 测试 gws CLI

```bash
# 安装 gws CLI (如果没有)
npm install -g @anthropic-ai/gws

# 生成 token 并测试
cd ~/server/DoWhiz/DoWhiz_service
source .env

# 使用 curl 测试 token 生成 (需要 jq)
# 或者直接重启服务让 agent 测试
pm2 restart all
```

---

## 故障排除

### "Delegation denied" 错误

- 确认 Domain-Wide Delegation 已在 Admin Console 授权
- 确认 Client ID 正确（是 Service Account 的 Client ID，不是 Project ID）
- 确认 scopes 完全匹配（复制上面的完整字符串）

### "User not found" 错误

- 确认 `GOOGLE_SERVICE_ACCOUNT_SUBJECT` 邮箱在 Google Workspace 域内
- 确认该用户账号处于活跃状态

### "API not enabled" 错误

- 返回第二步，确认所有 API 都已启用

### JSON 文件读取失败

- 确认文件路径正确
- 确认文件权限: `chmod 600 google-service-account.json`

---

## 完整 Scopes 列表

| Scope | 用途 |
|-------|------|
| `calendar` | 读写日历 |
| `calendar.events` | 管理日历事件 |
| `tasks` | 管理任务 |
| `contacts.readonly` | 查询联系人邮箱 |
| `documents` | 创建/编辑 Google Docs |
| `drive` | 访问 Google Drive |
| `spreadsheets` | 创建/编辑 Google Sheets |
| `presentations` | 创建/编辑 Google Slides |
