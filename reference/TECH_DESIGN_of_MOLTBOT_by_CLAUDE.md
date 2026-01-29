# Moltbot Technical Architecture

A comprehensive technical design document covering the multi-agent framework, agent personality system, and scheduled jobs infrastructure.

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Multi-Agent Framework](#2-multi-agent-framework)
3. [Agent Personalities & Bootstrap System](#3-agent-personalities--bootstrap-system)
4. [Scheduled Jobs (Cron)](#4-scheduled-jobs-cron)
5. [Configuration Reference](#5-configuration-reference)
6. [Data Flow & Interactions](#6-data-flow--interactions)
7. [Critical Files Reference](#7-critical-files-reference)

---

## 1. Executive Summary

### 1.1 Architecture Overview

Moltbot is a multi-agent messaging orchestration platform that enables multiple AI agents to operate concurrently, each with distinct personalities, workspaces, and configurations. The system routes inbound messages from various channels (Telegram, Discord, Slack, Signal, etc.) to appropriate agents based on configurable bindings.

### 1.2 Core Concepts

| Concept | Description |
|---------|-------------|
| **Agents** | Isolated autonomous units with their own workspace, identity, model configuration, and state |
| **Sessions** | Conversation contexts identified by unique keys (format: `agent:<agentId>:<sessionKey>`) |
| **Routing** | Message-to-agent binding with hierarchical matching (most specific wins) |
| **Lanes** | Concurrent execution tracks: Main, Cron, Subagent, Nested |
| **Bootstrap** | Workspace files (SOUL.md, AGENTS.md, etc.) loaded into system prompt |

### 1.3 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           MOLTBOT GATEWAY                                │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │
│  │   Telegram  │  │   Discord   │  │    Slack    │  │   Signal    │     │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘     │
│         │                │                │                │             │
│         └────────────────┴───────┬────────┴────────────────┘             │
│                                  ▼                                       │
│                        ┌──────────────────┐                              │
│                        │  Message Router  │                              │
│                        │  (bindings.ts)   │                              │
│                        └────────┬─────────┘                              │
│                                 │                                        │
│         ┌───────────────────────┼───────────────────────┐               │
│         ▼                       ▼                       ▼               │
│  ┌──────────────┐       ┌──────────────┐       ┌──────────────┐        │
│  │   Agent A    │       │   Agent B    │       │   Agent C    │        │
│  │  (main)      │       │  (research)  │       │  (dev)       │        │
│  ├──────────────┤       ├──────────────┤       ├──────────────┤        │
│  │ - Workspace  │       │ - Workspace  │       │ - Workspace  │        │
│  │ - SOUL.md    │       │ - SOUL.md    │       │ - SOUL.md    │        │
│  │ - Sessions   │       │ - Sessions   │       │ - Sessions   │        │
│  │ - Auth       │       │ - Auth       │       │ - Auth       │        │
│  └──────────────┘       └──────────────┘       └──────────────┘        │
│                                                                          │
├──────────────────────────────────────────────────────────────────────────┤
│                         EXECUTION LANES                                   │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐         │
│  │    MAIN    │  │    CRON    │  │  SUBAGENT  │  │   NESTED   │         │
│  │  (max: 4)  │  │ (scheduled)│  │  (max: 8)  │  │  (child)   │         │
│  └────────────┘  └────────────┘  └────────────┘  └────────────┘         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Multi-Agent Framework

### 2.1 Agent Configuration Schema

**File:** [src/config/types.agents.ts](src/config/types.agents.ts)

```typescript
type AgentConfig = {
  id: string;                    // Unique agent identifier
  default?: boolean;             // Mark as default agent
  name?: string;                 // Display name
  workspace?: string;            // Working directory path
  agentDir?: string;             // State directory override
  model?: AgentModelConfig;      // Model selection (string or {primary, fallbacks})
  memorySearch?: MemorySearchConfig;
  humanDelay?: HumanDelayConfig;
  heartbeat?: HeartbeatConfig;
  identity?: IdentityConfig;
  groupChat?: GroupChatConfig;
  subagents?: {
    allowAgents?: string[];      // Allowlist for cross-agent spawning ("*" = any)
    model?: string | { primary?: string; fallbacks?: string[] };
  };
  sandbox?: SandboxConfig;
  tools?: AgentToolsConfig;
};

type AgentsConfig = {
  defaults?: AgentDefaultsConfig;
  list?: AgentConfig[];
};
```

**Configuration Example (`~/.clawdbot/moltbot.json`):**
```json
{
  "agents": {
    "defaults": {
      "model": {
        "primary": "anthropic/claude-sonnet-4-20250514",
        "fallbacks": ["anthropic/claude-3-5-haiku-latest"]
      },
      "maxConcurrent": 4,
      "subagents": {
        "maxConcurrent": 8,
        "archiveAfterMinutes": 60
      }
    },
    "list": [
      {
        "id": "main",
        "default": true,
        "workspace": "~/clawd",
        "identity": {
          "name": "Claude",
          "emoji": "🤖"
        }
      },
      {
        "id": "research",
        "workspace": "~/clawd-research",
        "model": "anthropic/claude-opus-4-20250514",
        "subagents": {
          "allowAgents": ["*"]
        }
      }
    ]
  }
}
```

### 2.2 Agent Scope Resolution

**File:** [src/agents/agent-scope.ts](src/agents/agent-scope.ts)

Key functions for agent resolution:

| Function | Purpose |
|----------|---------|
| `listAgentIds(cfg)` | Returns all configured agent IDs |
| `resolveDefaultAgentId(cfg)` | Returns the default agent ID |
| `resolveAgentConfig(cfg, id)` | Returns config for specific agent |
| `resolveAgentWorkspaceDir(cfg, id)` | Resolves `~/clawd` or `~/clawd-{id}` |
| `resolveAgentDir(cfg, id)` | Resolves `~/.clawdbot/agents/{id}/agent` |

**Resolution Flow:**
```
Session Key: "agent:research:dm:user123"
       │
       ▼
┌─────────────────────┐
│ parseAgentSessionKey │ ──► { agentId: "research", rest: "dm:user123" }
└─────────────────────┘
       │
       ▼
┌─────────────────────┐
│  normalizeAgentId   │ ──► "research" (lowercase, validated)
└─────────────────────┘
       │
       ▼
┌─────────────────────┐
│ resolveAgentConfig  │ ──► { workspace: "~/clawd-research", model: {...} }
└─────────────────────┘
```

**Default Values:**
- Default agent ID: `"main"`
- Default workspace: `~/clawd` (main) or `~/clawd-{agentId}` (others)
- Default state dir: `~/.clawdbot/agents/{agentId}/agent`

### 2.3 Session Key Format

**File:** [src/routing/session-key.ts](src/routing/session-key.ts)

Session keys uniquely identify conversation contexts:

| Pattern | Description |
|---------|-------------|
| `agent:{agentId}:main` | Main session |
| `agent:{agentId}:dm:{peerId}` | DM session (per-peer) |
| `agent:{agentId}:{channel}:dm:{peerId}` | Channel-scoped DM |
| `agent:{agentId}:{channel}:group:{groupId}` | Group session |
| `agent:{agentId}:subagent:{uuid}` | Subagent session |
| `agent:{agentId}:cron:{jobId}` | Cron job session |

### 2.4 Message Routing & Bindings

**File:** [src/routing/bindings.ts](src/routing/bindings.ts)

Bindings route inbound messages to agents using **most-specific-match-wins** logic:

```typescript
type AgentBinding = {
  agentId: string;
  match: {
    channel: string;         // "telegram", "discord", etc.
    accountId?: string;      // Bot account ID
    peer?: {
      kind: "dm" | "group" | "channel";
      id: string;
    };
    guildId?: string;        // Discord server ID
    teamId?: string;         // Slack workspace ID
  };
};
```

**Match Priority (highest to lowest):**
1. Peer match (exact DM/group/channel ID)
2. Guild ID (Discord server)
3. Team ID (Slack workspace)
4. Account ID match
5. Channel-level match
6. Default agent

**Configuration Example:**
```json
{
  "bindings": [
    {
      "agentId": "research",
      "match": {
        "channel": "discord",
        "guildId": "123456789"
      }
    },
    {
      "agentId": "dev",
      "match": {
        "channel": "telegram",
        "peer": { "kind": "dm", "id": "987654321" }
      }
    }
  ]
}
```

### 2.5 Execution Lanes

**File:** [src/process/lanes.ts](src/process/lanes.ts)

Four concurrent execution lanes manage agent runs:

```typescript
const enum CommandLane {
  Main = "main",         // Primary user conversations
  Cron = "cron",         // Scheduled job execution
  Subagent = "subagent", // Spawned sub-agent runs
  Nested = "nested"      // Child/nested agent calls
}
```

**Concurrency Defaults:**

| Lane | Default Max Concurrent | Config Path |
|------|------------------------|-------------|
| Main | 4 | `agents.defaults.maxConcurrent` |
| Subagent | 8 | `agents.defaults.subagents.maxConcurrent` |
| Cron | 1 | `cron.maxConcurrentRuns` |

**Lane Guarantees:**
- Each lane has independent queue and concurrency limits
- Main lane serializes regular agent turns (respecting maxConcurrent)
- Subagent lane allows sub-agents to run in parallel
- All agents share the same process with isolated sessions/state

### 2.6 Sub-Agent Management

**File:** [src/agents/subagent-registry.ts](src/agents/subagent-registry.ts)

Sub-agents are spawned via the `sessions_spawn` tool:

```typescript
type SubagentRunRecord = {
  runId: string;
  childSessionKey: string;       // Session key for spawned agent
  requesterSessionKey: string;   // Calling agent session
  requesterOrigin?: DeliveryContext;
  requesterDisplayKey: string;
  task: string;                  // Task description
  cleanup: "delete" | "keep";    // Session cleanup policy
  label?: string;                // User-provided label
  createdAt: number;
  startedAt?: number;
  endedAt?: number;
  outcome?: SubagentRunOutcome;
  archiveAtMs?: number;          // Auto-archive deadline
  cleanupCompletedAt?: number;
};
```

**Sub-Agent Spawn Flow:**
```
┌──────────────────────────────────────────────────────────────────┐
│                    Sub-Agent Spawn Flow                          │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Parent Session                     Gateway                      │
│       │                                │                         │
│       │ sessions_spawn(task)           │                         │
│       ├───────────────────────────────►│                         │
│       │                                │                         │
│       │   ┌────────────────────────────┤                         │
│       │   │ 1. Validate allowAgents    │                         │
│       │   │ 2. Generate child key      │                         │
│       │   │ 3. Apply model override    │                         │
│       │   │ 4. Build system prompt     │                         │
│       │   └────────────────────────────┤                         │
│       │                                │                         │
│       │   registerSubagentRun()        │                         │
│       │◄───────────────────────────────┤                         │
│       │   { status: "accepted",        │                         │
│       │     childSessionKey, runId }   │                         │
│       │                                │                         │
│       │       ┌────────────────────────┴────┐                    │
│       │       │     Child Agent Run         │                    │
│       │       │  (lane: "subagent")         │                    │
│       │       └────────────────────────┬────┘                    │
│       │                                │                         │
│       │   onAgentEvent(lifecycle:end)  │                         │
│       │◄───────────────────────────────┤                         │
│       │                                │                         │
│       │   runSubagentAnnounceFlow()    │                         │
│       │   (summary posted to parent)   │                         │
│       │                                │                         │
└──────────────────────────────────────────────────────────────────┘
```

**Sub-Agent Lifecycle:**
1. Parent agent calls `sessions_spawn` with task + target agentId
2. Spawned agent runs in isolation with own session
3. Completion triggers `subagent-announce` flow back to requester
4. Sessions auto-archive after configured time (default 60 min)
5. Persistent registry stored at `~/.clawdbot/agents/{agentId}/subagent-runs.jsonl`

### 2.7 Agent Isolation & Security

Per-agent isolation mechanisms:

| Aspect | Isolation |
|--------|-----------|
| **Workspace** | Separate directory per agent (default cwd for tools) |
| **State** | Separate `agentDir` with auth profiles and session store |
| **Auth Profiles** | Per-agent `~/.clawdbot/agents/{id}/agent/auth-profiles.json` |
| **Sessions** | Separate session store per agent |
| **Sandbox** | Optional per-agent Docker container isolation |
| **Tool restrictions** | Per-agent allow/deny tool lists |

---

## 3. Agent Personalities & Bootstrap System

### 3.1 Workspace Bootstrap Files

**File:** [src/agents/workspace.ts](src/agents/workspace.ts)

Each agent workspace contains bootstrap files loaded into the system prompt:

| File | Purpose |
|------|---------|
| `AGENTS.md` | Agent capabilities and instructions |
| `SOUL.md` | **Primary personality definition** |
| `TOOLS.md` | Tool-specific guidance |
| `IDENTITY.md` | Name, emoji, theme, creature, vibe |
| `USER.md` | User preferences and context |
| `HEARTBEAT.md` | Periodic check-in instructions |
| `BOOTSTRAP.md` | Session initialization |
| `MEMORY.md` | Persistent memory notes |

**Default Workspace Paths:**
- Main agent: `~/clawd/`
- Other agents: `~/clawd-{agentId}/`

### 3.2 Bootstrap Loading Flow

**File:** [src/agents/bootstrap-files.ts](src/agents/bootstrap-files.ts)

```
┌─────────────────────────────────────────────────────────────────┐
│                 Bootstrap File Loading                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Workspace Dir (~clawd/)                                       │
│        │                                                        │
│        ├── SOUL.md ──────────────┐                              │
│        ├── AGENTS.md ────────────┤                              │
│        ├── TOOLS.md ─────────────┼──► loadWorkspaceBootstrapFiles()
│        ├── IDENTITY.md ──────────┤       │                      │
│        ├── USER.md ──────────────┤       │                      │
│        ├── HEARTBEAT.md ─────────┤       ▼                      │
│        └── MEMORY.md ────────────┘   WorkspaceBootstrapFile[]   │
│                                          │                      │
│                         ┌────────────────┴───────────────┐      │
│                         ▼                                ▼      │
│               [Main Session]                    [Subagent Session]
│               Load all files                    AGENTS.md + TOOLS.md only
│                         │                                │      │
│                         ▼                                ▼      │
│                    applyBootstrapHookOverrides()                │
│                    (soul-evil, custom hooks)                    │
│                         │                                       │
│                         ▼                                       │
│                    buildBootstrapContextFiles()                 │
│                    (truncate to maxChars: 20000)                │
│                         │                                       │
│                         ▼                                       │
│                    System Prompt Injection                      │
│                    <contextFile name="SOUL.md">...</>           │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Key Functions:**

```typescript
// src/agents/bootstrap-files.ts
async function resolveBootstrapFilesForRun(params: {
  workspaceDir: string;
  config?: MoltbotConfig;
  sessionKey?: string;
  agentId?: string;
}): Promise<WorkspaceBootstrapFile[]>

async function resolveBootstrapContextForRun(params): Promise<{
  bootstrapFiles: WorkspaceBootstrapFile[];
  contextFiles: EmbeddedContextFile[];
}>
```

**Subagent Bootstrap Filtering:**
- Subagents only receive `AGENTS.md` and `TOOLS.md`
- Full personality files (SOUL.md, etc.) are excluded to keep subagents focused

### 3.3 SOUL_EVIL Personality Hook

**File:** [src/hooks/soul-evil.ts](src/hooks/soul-evil.ts)

The `soul-evil` hook enables dynamic personality swapping:

```typescript
type SoulEvilConfig = {
  file?: string;        // Alternate filename (default: SOUL_EVIL.md)
  chance?: number;      // Random chance 0-1 to activate
  purge?: {
    at?: string;        // Daily start time (HH:mm format)
    duration?: string;  // Duration (e.g., "30m", "1h")
  };
};
```

**Activation Logic:**
1. Check if within daily purge window (time-based)
2. If not, roll random chance
3. If activated, swap SOUL.md content with SOUL_EVIL.md

**Configuration Example:**
```json
{
  "hooks": {
    "internal": {
      "entries": {
        "soul-evil": {
          "enabled": true,
          "file": "SOUL_EVIL.md",
          "chance": 0.05,
          "purge": {
            "at": "23:00",
            "duration": "1h"
          }
        }
      }
    }
  }
}
```

**Key Function:**
```typescript
// Performs the SOUL.md swap at bootstrap time
async function applySoulEvilOverride(params: {
  files: WorkspaceBootstrapFile[];
  workspaceDir: string;
  config?: SoulEvilConfig;
  userTimezone?: string;
}): Promise<WorkspaceBootstrapFile[]>
```

### 3.4 Identity Configuration

**File:** [src/agents/identity.ts](src/agents/identity.ts)

Each agent can have distinct identity metadata:

```typescript
type IdentityConfig = {
  name?: string;      // Display name (e.g., "Claude")
  emoji?: string;     // Ack reaction emoji (default: "👀")
  theme?: string;     // UI theme hint
  creature?: string;  // Persona description
  vibe?: string;      // Personality traits (e.g., "sharp", "warm")
  avatar?: string;    // Avatar image URL/path
};
```

**Identity Resolution:**
- `resolveAckReaction()` → `identity.emoji || "👀"`
- `resolveIdentityNamePrefix()` → `"[{name}]"` for message prefix
- `resolveMessagePrefix()` → `"[moltbot]"` fallback

**IDENTITY.md Example:**
```markdown
# Agent Identity

name: Research Assistant
emoji: 🔬
vibe: analytical, thorough, curious
creature: scholarly owl
```

### 3.5 System Prompt Modes

**File:** [src/agents/system-prompt.ts](src/agents/system-prompt.ts)

The system prompt supports different complexity levels:

| Mode | Sections Included | Use Case |
|------|-------------------|----------|
| `"full"` | All sections (Skills, Memory, User, Time, Messaging, Tooling, etc.) | Main agent sessions |
| `"minimal"` | Tooling, Workspace, Runtime only | Subagent sessions |
| `"none"` | Basic identity line only | Minimal personality |

**System Prompt Guidance for SOUL.md:**
> "If SOUL.md is present, embody its persona and tone. Avoid stiff, generic replies; follow its guidance unless higher-priority instructions override it."

---

## 4. Scheduled Jobs (Cron)

### 4.1 Cron Types

**File:** [src/cron/types.ts](src/cron/types.ts)

Three schedule types are supported:

```typescript
type CronSchedule =
  | { kind: "at"; atMs: number }                              // One-shot at timestamp
  | { kind: "every"; everyMs: number; anchorMs?: number }     // Repeating interval
  | { kind: "cron"; expr: string; tz?: string };              // 5-field cron expression
```

| Type | Description | Example |
|------|-------------|---------|
| `at` | One-shot at specific timestamp | `{ kind: "at", atMs: 1735689600000 }` |
| `every` | Recurring interval | `{ kind: "every", everyMs: 1800000 }` (30 min) |
| `cron` | 5-field cron expression | `{ kind: "cron", expr: "0 9 * * *", tz: "America/New_York" }` |

### 4.2 Cron Job Structure

```typescript
type CronJob = {
  id: string;
  agentId?: string;              // Target agent (default if omitted)
  name: string;
  description?: string;
  enabled: boolean;
  deleteAfterRun?: boolean;      // Auto-delete one-shot jobs
  createdAtMs: number;
  updatedAtMs: number;
  schedule: CronSchedule;
  sessionTarget: CronSessionTarget;  // "main" | "isolated"
  wakeMode: CronWakeMode;            // "next-heartbeat" | "now"
  payload: CronPayload;
  isolation?: CronIsolation;
  state: CronJobState;
};

type CronPayload =
  | { kind: "systemEvent"; text: string }
  | {
      kind: "agentTurn";
      message: string;
      model?: string;
      thinking?: string;
      timeoutSeconds?: number;
      deliver?: boolean;
      channel?: ChannelId | "last";
      to?: string;
      bestEffortDeliver?: boolean;
    };

type CronIsolation = {
  postToMainPrefix?: string;           // Summary prefix (default: "Cron")
  postToMainMode?: "summary" | "full"; // What to post back
  postToMainMaxChars?: number;         // Max chars when full (default: 8000)
};

type CronJobState = {
  nextRunAtMs?: number;
  runningAtMs?: number;
  lastRunAtMs?: number;
  lastStatus?: "ok" | "error" | "skipped";
  lastError?: string;
  lastDurationMs?: number;
};
```

### 4.3 Two Execution Modes

| Mode | Session Target | Payload Kind | Description |
|------|---------------|--------------|-------------|
| **Main Session** | `"main"` | `systemEvent` | Inject text into main session as system event |
| **Isolated** | `"isolated"` | `agentTurn` | Full agent run in dedicated `cron:{jobId}` session |

**Main Session Jobs:**
- Text injected via `enqueueSystemEvent()`
- If `wakeMode: "now"`, immediately trigger heartbeat
- Runs in main conversation context

**Isolated Agent Jobs:**
- Runs in fresh `cron:{jobId}` session (no history)
- Can specify alternate model/thinking level
- Optional delivery to messaging channels
- Posts summary back to main session

### 4.4 Cron Execution Flow

**File:** [src/cron/service/timer.ts](src/cron/service/timer.ts)

```
┌─────────────────────────────────────────────────────────────────┐
│                    Cron Execution Flow                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   CronService.start()                                           │
│        │                                                        │
│        ├──► ensureLoaded() ──► Load ~/.clawdbot/cron/jobs.json  │
│        ├──► recomputeNextRuns()                                 │
│        └──► armTimer()                                          │
│                │                                                │
│                ▼                                                │
│   setTimeout(onTimer, delay)                                    │
│                │                                                │
│   ┌────────────┴────────────┐                                   │
│   │       Timer Tick        │                                   │
│   └────────────┬────────────┘                                   │
│                │                                                │
│                ▼                                                │
│   runDueJobs() ──► For each job where now >= nextRunAtMs:       │
│        │                                                        │
│        ├─────────────────────────────────────────┐              │
│        │  sessionTarget == "main"                │              │
│        │  └──► enqueueSystemEvent(text)          │              │
│        │       └──► requestHeartbeatNow()        │              │
│        │            (or wait for next heartbeat) │              │
│        │                                         │              │
│        ├─────────────────────────────────────────┤              │
│        │  sessionTarget == "isolated"            │              │
│        │  └──► runIsolatedAgentJob()             │              │
│        │       ├── Build session key             │              │
│        │       ├── Resolve model/thinking        │              │
│        │       ├── runEmbeddedPiAgent()          │              │
│        │       └── Deliver response (optional)   │              │
│        └─────────────────────────────────────────┘              │
│                │                                                │
│                ▼                                                │
│   finish() ──► Update state, persist, emit event                │
│        │                                                        │
│        ├──► computeJobNextRunAtMs() (if recurring)              │
│        └──► armTimer() (schedule next tick)                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Timer Management:**
- Uses Node.js `setTimeout` with `unref()` to avoid blocking process exit
- Clamps delay to `2^31 - 1` ms to avoid overflow warnings
- Re-arms after each job execution

### 4.5 Cron Storage & Persistence

**Store Path:** `~/.clawdbot/cron/jobs.json`

```typescript
type CronStoreFile = {
  version: 1;
  jobs: CronJob[];
};
```

**Run Logs:** `~/.clawdbot/cron/runs/{jobId}.jsonl`

```typescript
type CronRunLogEntry = {
  ts: number;
  jobId: string;
  action: "finished";
  status: "ok" | "error" | "skipped";
  error?: string;
  summary?: string;
  runAtMs: number;
  durationMs: number;
  nextRunAtMs?: number;
};
```

**Log Limits:**
- Max 2000 lines or 2 MB per job (whichever reached first)
- Auto-prune on append

### 4.6 Cron Configuration Examples

**Daily Report at 9 AM (Main Session):**
```bash
moltbot cron add \
  --name "Daily briefing" \
  --cron "0 9 * * *" \
  --tz "America/Los_Angeles" \
  --session main \
  --system-event "What's on my schedule today?" \
  --wake now
```

**Isolated Task Every 30 Minutes:**
```bash
moltbot cron add \
  --name "Email check" \
  --every 30m \
  --session isolated \
  --message "Check my inbox and summarize new emails" \
  --deliver \
  --channel whatsapp \
  --to "+1234567890" \
  --post-mode full
```

**One-Shot Reminder (Auto-Delete):**
```bash
moltbot cron add \
  --name "Meeting reminder" \
  --at "2025-12-24T15:00:00Z" \
  --system-event "You have a meeting in 5 minutes" \
  --wake now \
  --delete-after-run
```

**JSON Configuration:**
```json
{
  "id": "daily-summary",
  "name": "Daily Summary",
  "enabled": true,
  "schedule": {
    "kind": "cron",
    "expr": "0 9 * * *",
    "tz": "America/New_York"
  },
  "sessionTarget": "isolated",
  "wakeMode": "now",
  "payload": {
    "kind": "agentTurn",
    "message": "Generate daily summary of tasks and priorities",
    "model": "anthropic/claude-sonnet-4-20250514",
    "thinking": "low",
    "deliver": true,
    "channel": "telegram",
    "to": "+1234567890"
  },
  "isolation": {
    "postToMainMode": "summary",
    "postToMainPrefix": "Daily"
  }
}
```

### 4.7 Cron Service API

**File:** [src/cron/service.ts](src/cron/service.ts)

| Method | Description |
|--------|-------------|
| `start()` | Load jobs, arm timer, begin polling |
| `stop()` | Clear timer |
| `status()` | Return `{ enabled, storePath, jobs, nextWakeAtMs }` |
| `list(includeDisabled?)` | List jobs |
| `add(input)` | Create new job |
| `update(id, patch)` | Patch existing job |
| `remove(id)` | Delete job |
| `run(id, mode?)` | Force-run job (`"due"` or `"force"`) |
| `runLogs(jobId, limit?)` | Fetch run history |
| `wake(text, mode)` | Inject wake event into main session |

---

## 5. Configuration Reference

### 5.1 Full Agent Defaults Schema

**File:** [src/config/types.agent-defaults.ts](src/config/types.agent-defaults.ts)

```typescript
type AgentDefaultsConfig = {
  model?: { primary?: string; fallbacks?: string[] };
  imageModel?: { primary?: string; fallbacks?: string[] };
  models?: Record<string, AgentModelEntryConfig>;
  workspace?: string;
  repoRoot?: string;
  skipBootstrap?: boolean;
  bootstrapMaxChars?: number;           // Default: 20000
  userTimezone?: string;                // IANA timezone
  timeFormat?: "auto" | "12" | "24";
  contextTokens?: number;
  thinkingDefault?: ThinkLevel;
  verboseDefault?: VerboseLevel;
  elevatedDefault?: ElevatedLevel;
  timeoutSeconds?: number;
  maxConcurrent?: number;               // Default: 4
  heartbeat?: HeartbeatConfig;
  subagents?: {
    maxConcurrent?: number;             // Default: 8
    archiveAfterMinutes?: number;       // Default: 60
    model?: string | { primary?: string; fallbacks?: string[] };
  };
  sandbox?: SandboxConfig;
  humanDelay?: HumanDelayConfig;
  contextPruning?: AgentContextPruningConfig;
  compaction?: AgentCompactionConfig;
};
```

### 5.2 Key File Paths

```
State Directory: ~/.clawdbot/

├── moltbot.json            → Main configuration
├── sessions.json           → Session store
├── cron/
│   ├── jobs.json           → Cron job definitions
│   └── runs/*.jsonl        → Per-job run logs
├── agents/{agentId}/
│   ├── agent/              → Agent state directory
│   │   └── auth-profiles.json
│   ├── sessions/           → Session transcripts
│   └── subagent-runs.jsonl → Sub-agent registry
└── credentials/            → Channel credentials

Agent Workspace: ~/clawd/ (or ~/clawd-{agentId})

├── SOUL.md                 → Personality
├── SOUL_EVIL.md            → Alternate personality (optional)
├── AGENTS.md               → Capabilities
├── TOOLS.md                → Tool guidance
├── IDENTITY.md             → Identity config
├── USER.md                 → User context
├── HEARTBEAT.md            → Heartbeat instructions
├── BOOTSTRAP.md            → Session initialization
├── MEMORY.md               → Memory notes
└── .git/                   → Auto-initialized git repo
```

### 5.3 Default Values Summary

| Setting | Default | Source |
|---------|---------|--------|
| Agent max concurrent | 4 | `src/config/agent-limits.ts` |
| Subagent max concurrent | 8 | `src/config/agent-limits.ts` |
| Subagent archive after | 60 min | `src/agents/subagent-registry.ts` |
| Bootstrap max chars | 20000 | `src/agents/pi-embedded-helpers.ts` |
| Default agent ID | `"main"` | `src/routing/session-key.ts` |
| Default ack reaction | `"👀"` | `src/agents/identity.ts` |
| Cron run log max | 2MB / 2000 lines | `src/cron/run-log.ts` |
| Cron postToMain max chars | 8000 | `src/cron/types.ts` |

---

## 6. Data Flow & Interactions

### 6.1 Inbound Message Flow

```
┌──────────────────────────────────────────────────────────────────┐
│                      Inbound Message Flow                        │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Channel (Telegram/Discord/etc.)                                 │
│       │                                                          │
│       ▼                                                          │
│  ┌────────────────┐                                              │
│  │ Channel Plugin │  Extract: { channel, accountId, peer,        │
│  └───────┬────────┘            content, attachments }            │
│          │                                                       │
│          ▼                                                       │
│  ┌────────────────┐                                              │
│  │ Message Router │  Evaluate bindings → select agentId          │
│  └───────┬────────┘                                              │
│          │                                                       │
│          ▼                                                       │
│  ┌────────────────┐                                              │
│  │  Session Key   │  Build: agent:{agentId}:{channel}:{peer}     │
│  │   Resolution   │                                              │
│  └───────┬────────┘                                              │
│          │                                                       │
│          ▼                                                       │
│  ┌────────────────┐                                              │
│  │    Lane Gate   │  Check maxConcurrent, queue if full          │
│  └───────┬────────┘                                              │
│          │                                                       │
│          ▼                                                       │
│  ┌────────────────┐                                              │
│  │ Agent Runner   │  Load bootstrap → build prompt →             │
│  │                │  call LLM → stream response                  │
│  └───────┬────────┘                                              │
│          │                                                       │
│          ▼                                                       │
│  ┌────────────────┐                                              │
│  │ Response       │  Route back to originating channel           │
│  │ Delivery       │                                              │
│  └────────────────┘                                              │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

### 6.2 Agent Event System

**File:** [src/infra/agent-events.ts](src/infra/agent-events.ts)

```typescript
type AgentEventPayload = {
  runId: string;
  seq: number;               // Monotonic per-run
  stream: AgentEventStream;  // "lifecycle" | "tool" | "assistant" | "error"
  ts: number;
  data: Record<string, unknown>;
  sessionKey?: string;
};

// Lifecycle events
{ stream: "lifecycle", data: { phase: "start" | "end" | "error" } }

// Tool events
{ stream: "tool", data: { name, args, result } }

// Assistant events
{ stream: "assistant", data: { text, tokens } }
```

**Event Subscription:**
```typescript
const unsubscribe = onAgentEvent((evt) => {
  if (evt.stream === "lifecycle" && evt.data.phase === "end") {
    // Handle agent completion
  }
});
```

---

## 7. Critical Files Reference

| File | Purpose |
|------|---------|
| [src/config/types.agents.ts](src/config/types.agents.ts) | Agent configuration types |
| [src/agents/agent-scope.ts](src/agents/agent-scope.ts) | Agent ID resolution, config lookup |
| [src/routing/bindings.ts](src/routing/bindings.ts) | Message-to-agent routing |
| [src/routing/session-key.ts](src/routing/session-key.ts) | Session key parsing/building |
| [src/process/lanes.ts](src/process/lanes.ts) | Execution lane definitions |
| [src/agents/subagent-registry.ts](src/agents/subagent-registry.ts) | Sub-agent lifecycle management |
| [src/agents/bootstrap-files.ts](src/agents/bootstrap-files.ts) | Bootstrap file loading |
| [src/agents/workspace.ts](src/agents/workspace.ts) | Workspace file definitions |
| [src/agents/system-prompt.ts](src/agents/system-prompt.ts) | System prompt construction |
| [src/agents/identity.ts](src/agents/identity.ts) | Identity resolution |
| [src/hooks/soul-evil.ts](src/hooks/soul-evil.ts) | Personality swap logic |
| [src/hooks/internal-hooks.ts](src/hooks/internal-hooks.ts) | Hook registration/triggering |
| [src/cron/types.ts](src/cron/types.ts) | Cron type definitions |
| [src/cron/service.ts](src/cron/service.ts) | Cron service interface |
| [src/cron/service/timer.ts](src/cron/service/timer.ts) | Timer management, job execution |
| [src/cron/isolated-agent/run.ts](src/cron/isolated-agent/run.ts) | Isolated agent turn execution |
| [src/infra/agent-events.ts](src/infra/agent-events.ts) | Event emission system |

---

## Session Key Examples

```
# Main session for default agent
agent:main:main

# DM session with per-peer scope
agent:main:dm:+1234567890

# Telegram group session
agent:research:telegram:group:chatid123

# Subagent session
agent:main:subagent:550e8400-e29b-41d4-a716-446655440000

# Cron job session
agent:main:cron:daily-summary

# Discord channel-scoped
agent:dev:discord:channel:channelid456
```
