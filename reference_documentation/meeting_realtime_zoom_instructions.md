# Zoom Meeting + Realtime Voice Agent — Implementation Instructions

## 1) Objective
Build a Zoom meeting workflow where:
- Oliver can schedule meetings via email.
- Oliver joins Zoom with **real-time voice** using **Azure OpenAI gpt-realtime** (no STT→LLM→TTS roundtrip).
- Meeting history is archived similarly to email history (file-based, **no new DB**).
- After the meeting ends, Oliver automatically produces meeting notes and emails all participants.

---

## 2) Key Constraints
- **Azure OpenAI only** (not OpenAI direct):
  - Use `AZURE_OPENAI_API_KEY_BACKUP` and `AZURE_OPENAI_ENDPOINT_BACKUP` from `.env`.
  - Deployment name is **`gpt-realtime`** (fixed).
- **No new database**. Store meeting history as files, modeled after existing email archives.
- **Use a Zoom skill** rather than SchedulerAction outputs.  
  The agent must call the skill to create/update/cancel meetings, and must write the Zoom link into `reply_email_draft.html`.
- **Meeting notes recipients** default to **all Zoom participants**.

---

## 3) Architecture Overview
**Components**
1) **zoom_meeting skill** (agent side)  
   Guides the agent to call a local CLI for Zoom operations.
2) **meeting_ctl CLI** (Rust)  
   Centralized entry point for Zoom API calls, bot-user pool allocation, meeting history archiving, join scheduling, and post-meeting notes scheduling.
3) **meeting_worker** (service/daemon)  
   Joins Zoom meetings, runs **Azure Realtime gpt-realtime** for live voice, writes transcripts/notes into meeting history, and schedules notes tasks.
4) **scheduler_module**  
   Add `JoinMeeting` task type to run `meeting_worker` at scheduled time.

---

## 4) Meeting History Storage (File-Based)
Create a meeting history directory per user, parallel to email archive style:

```
/Users/bingran_you/Documents/GitHub_MacBook/DoWhiz/DoWhiz_service/.workspace/run_task/users/<user_id>/mail/meetings/YYYY/MM/<meeting_id>/
```

Recommended contents:
- `metadata.json`  
  Meeting metadata (topic, meeting_id, start_at, duration, join_url, host user id, bot user id, timezone)
- `participants.json`  
  Participant emails (for post-meeting notes recipients)
- `transcript.json` (optional)
- `audio.raw` (optional)
- `notes.md` (final summary)
- `screens/` (optional screenshots or key frames if screen capture is available)

---

## 5) Zoom Integration (based on local docs)
Use the Zoom API docs snapshot at:
`/Users/bingran_you/Documents/GitHub_MacBook/DoWhiz/api_reference_documentation/zoom_api_docs`

Key endpoints:
- Create meeting: `POST /users/{userId}/meetings`
- Update meeting: `PATCH /meetings/{meetingId}`
- Delete meeting: `DELETE /meetings/{meetingId}`
- Participants report: `GET /report/meetings/{meetingId}/participants`

Authentication: Zoom Server-to-Server OAuth.

Meeting settings for "anyone can join":
- `join_before_host = true`
- `waiting_room = false`
- `meeting_authentication = false`
- `password` omitted (if Zoom auto-generates, include in email)

---

## 6) zoom_meeting Skill
Location:  
`/Users/bingran_you/Documents/GitHub_MacBook/DoWhiz/DoWhiz_service/skills/zoom_meeting/SKILL.md`

Rules:
- The agent **must** use this skill for any meeting creation/update/cancel.
- The skill can only invoke `meeting_ctl`; no direct API calls.
- The skill must ensure the final meeting link is written into `reply_email_draft.html`.

Expected CLI usage:
```
meeting_ctl create \
  --user-id <user_id> \
  --topic "<topic>" \
  --start "<RFC3339|now>" \
  --duration <minutes> \
  --attendees "<email1,email2,...>" \
  --join-now | --schedule-join
```

---

## 7) meeting_ctl CLI (Rust)
Create new crate:
`/Users/bingran_you/Documents/GitHub_MacBook/DoWhiz/DoWhiz_service/meeting_module`

CLI responsibilities:
- Create/update/cancel Zoom meeting.
- Allocate bot user.
- Write meeting history.
- Generate calendar invite (ICS).
- Schedule join (via scheduler_module) if future meeting.
- Schedule notes task after meeting.

Suggested subcommands:
1. `create`  
   Input: user_id, topic, start_time, duration, attendees, join_now/schedule_join  
   Output JSON: meeting_id, join_url, start_time, duration, bot_user_id, meeting_archive_path, ics_path

2. `prepare-workspace`  
   Creates a dedicated workspace for meeting-worker personalization.

3. `schedule-notes`  
   Creates RunTaskTask for meeting notes with participants as recipients.

---

## 8) meeting_worker (Realtime Voice Agent)
Requirements:
- Must use Azure Realtime **gpt-realtime** with `.env` credentials.
- Must create a workspace for **current user** to load personalized memory and references.
- Joins Zoom meeting (Meeting SDK or Web client).
- Feeds Zoom audio into Azure Realtime and injects audio responses back into Zoom.

Workspace setup:
```
/Users/bingran_you/Documents/GitHub_MacBook/DoWhiz/DoWhiz_service/.workspace/run_task/users/<user_id>/workspaces/meeting_<meeting_id>/
```

The worker must:
- Load `memory/*.md` for personalization.
- Optionally load `references/past_emails` for context.
- Write transcripts to meeting history.
- On meeting end, call `meeting_ctl schedule-notes`.

Optional screen visibility:
- If SDK/RTMS provides screen frames, summarize frames into text context for the Realtime session.

---

## 9) Scheduler Integration
Add `TaskKind::JoinMeeting` in:
`/Users/bingran_you/Documents/GitHub_MacBook/DoWhiz/DoWhiz_service/scheduler_module/src/lib.rs`

Behavior:
- At scheduled time, run meeting_worker with `meeting_id`.
- meeting_worker handles realtime conversation and outputs meeting records.

Meeting notes:
- meeting_worker calls `meeting_ctl schedule-notes`.
- `schedule-notes` creates a RunTaskTask with:
  - recipients = all participants (from participants.json or report API)
  - instructions to read meeting history directory.

---

## 10) Meeting Notes Task (RunTask)
Create a dedicated workspace for the notes task:
```
/Users/bingran_you/Documents/GitHub_MacBook/DoWhiz/DoWhiz_service/.workspace/run_task/users/<user_id>/workspaces/meeting_notes_<meeting_id>/
```

In `incoming_email/email.html`, explicitly include:
1) “Share meeting notes with: <participants list>”
2) “Analyze meeting history at: <meeting history path>”

The agent should produce `reply_email_draft.html` and send the notes email to participants.

---

## 11) Default Recipients
Meeting notes are sent to **all participants**:
1. `GET /report/meetings/{meetingId}/participants` (if available)
2. Fallback to invitees list from meeting creation.

---

## 12) Testing / Validation
- Create meeting → join → end → notes email.
- Scheduled meeting → join at time → end → notes email.
- Verify meeting history is archived under `mail/meetings/`.

---

## 13) Deliverables Summary
Required new/updated paths:
- ` /Users/bingran_you/Documents/GitHub_MacBook/DoWhiz/DoWhiz_service/skills/zoom_meeting/SKILL.md`
- ` /Users/bingran_you/Documents/GitHub_MacBook/DoWhiz/DoWhiz_service/meeting_module`
- ` /Users/bingran_you/Documents/GitHub_MacBook/DoWhiz/DoWhiz_service/meeting_module/src/bin/meeting_ctl.rs`
- ` /Users/bingran_you/Documents/GitHub_MacBook/DoWhiz/DoWhiz_service/scheduler_module/src/lib.rs`
- ` /Users/bingran_you/Documents/GitHub_MacBook/DoWhiz/DoWhiz_service/run_task_module/run_task.rs`

---

## 14) Notes
- All Zoom operations must be performed via `meeting_ctl` to keep behavior consistent and auditable.
- Realtime audio must use Azure deployment `gpt-realtime`.
- Meeting archives are file-based; no DB schema changes required.

Go bears!
