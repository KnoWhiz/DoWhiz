# DoWhiz - Long-term Vision for the Digital Employee Platform

## 1. North Star Vision
Let every user have a "digital employee team" that collaborates like real colleagues via email/collab docs/team tools, can execute tasks independently, keep following up, proactively sync progress, and escalate to humans when needed (clarify / approval / escalation).

Core experience:
- The user only needs to send a task to a digital employee's email; it can execute, produce results, and return them via the appropriate channel.
- Digital employees autonomously choose the best toolchain in the background (LLM, code, docs, editing, data retrieval); the user does not need to care about details.
- Each user has a **fully isolated** memory and data management system that is auditable, revocable, and portable.

## 2. Product Form and Role Model
Digital employees are not a single bot, but **role-based positions**:
- Oliver: all-round assistant for daily work.
- Mini-Mouse: all-round assistant for daily work.
- Alice (Researcher): deep research, routine paper monitoring, survey/brief production.
- Bob (TPM): task scheduling, progress tracking, cross-team reminders, meeting coordination.
- Eve (Docs/Notion/Overleaf collaborator): document organizing, comment responses, version updates (after being added to a document's access, modify based on Overleaf comments and leave comments).

"Role as a strategy package":
- Each role corresponds to a set of goals, tool strategies, default models, risk levels, and approval thresholds.
- Roles can use permissions granted by the user (such as access/edit permissions for different Notion or Google workspaces), but act only within the authorized scope (least privilege).

## 3. Design Principles
1) **Isolation first**: user data, memory, credentials, and execution environments are isolated from each other.
2) **Minimal interruption**: complete when possible; ask precise questions when unclear.
3) **Tool neutral**: tools are replaceable and upgradable; users do not need to know the details.
4) **Controllable long-term memory**: memory is files/structured storage (in the future, users can view, edit, delete).

## 4. Core Abstractions Referencing OpenClaw (Reusable Design Ideas)
OpenClaw's architecture has mature abstractions for "multi-channel messaging + agent execution + task queues." Key borrowable points:

- **Gateway (long-running daemon)**: maintains all message channel connections and exposes a unified control interface.
- **Agent Loop (task execution loop)**: standardizes the execution path of "context assembly -> reasoning -> tool calls -> output -> record."
- **Command Queue (Queue)**: serial per session, global rate limiting, ensures concurrency safety.
- **Multi-agent Routing**: multiple agentIds in the same process with independent workspaces/sessions/credentials.
- **Workspace + Memory as files**: memory sources are readable/writable files for audit and rollback.
- **Hooks / Plugins**: insert security/audit/automation logic before and after execution.

> These abstractions align highly with the core requirements of the "digital employee platform" and can serve as base design references.

## 5. Target System Architecture (Multi-tenant SaaS)

### 5.1 Logical Layers
1) **Ingress Layer (Channel Ingress)**
   - Email (priority)
   - Docs/Notion comment events
   - Later: Slack / Zoom / Calendar
   - Unified event format (Inbound Event)

2) **Control Plane**
   - User/org/subscription management
   - Role catalog and permission policies
   - Connector configuration and credential vaulting (Vault)

3) **Task Orchestrator**
   - Task categories: sign-up / unpaid / irrelevant / direct reply / needs execution
   - Task state machine: queued -> in-progress -> done / archived
   - Scheduled and recurring tasks (daily / weekly / monthly / yearly: cron-like)

4) **Execution Layer (Agent Runtime)**
   - Task context assembly
   - Role strategy and tool selection
   - Tool execution, output, and return

5) **Storage Layer (Data & Memory)**
   - Azure Blob: user-private memory + artifacts
   - DB (Postgres): metadata, tasks, index, permissions, audit logs

6) **Observability Layer (Observability)**
   - Task logs, cost, failure reasons
   - Audit and replay for external write operations

### 5.2 Execution Flow (Email Ingress Example)
```
Email -> Ingress -> Triage -> Task Queue -> Agent Runtime -> Output
Scheduler ^                                           v Scheduler
```

- Triage categories:
  a) need to create account and reply
  b) account balance insufficient -> reply to prompt recharge
  c) irrelevant email -> ignore
  d) execute task locally and reply directly (e.g., need more info, from user's direct task request)
  e) execute task directly in third-party services (e.g., modify documents based on Google Doc/Slides/Overleaf comments) + reply to comments + email report (not direct email reply, because the received email is a machine notification from a third-party app)

- After entering the execution layer:
  1) load user history + current thread + attachments
  2) role strategy decision: clarify or execute directly
  3) execute toolchain, generate results/attachments/external document updates
  4) return via email or specified channel
  5) if recurring task, write into Scheduler

## 6. Multi-tenant Isolation Strategy (Key Issue)
Question: **How to ensure Alice only accesses current user data?**

Hard boundaries:
- **Data isolation**: each user has an independent Azure Blob Storage folder.
- **Execution isolation**:
  - single container, multiple users, but each run mounts only that user's workspace (temporary directory).
- **Runtime constraints**:
  - when accessing storage/search/index, only allow the workspace (temporary directory) for that user_id; scope enforced.
  - any cross-user resource access is rejected and audited.

Reference from OpenClaw:
- enable sandbox (containerized tool execution) when stronger isolation is needed.

## 7. "Next Step" Design After Alice Gets Shared Permissions
Question: **After the user shares Notion/Docs with Alice, how does Alice move on?**

For each access permission type, maintain an abstract workflow layer with security protections to ensure only the information the user has granted is loaded when preparing the workspace.

## 8. Recurring Tasks and Task Management
- Each user has an independent Scheduler (supports cron-like).
- Task entities have a lifecycle:
  - create, pause, cancel, retry on failure
- New tasks generated by recurring tasks still go through the unified queue and permission checks.
- Support "automatic de-noising": identical tasks in a short time window are merged or summarized (if the agent is still executing and has not sent a reply, new information can be delivered as new instructions to the agent, then reply once after completion).

---
