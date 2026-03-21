---
name: google-workspace-cli
description: Use Google Workspace CLI (gws) for Calendar, Tasks, Contacts, Meet, and Drive operations. Use this skill when the user wants to schedule meetings, manage tasks, look up contacts, create video calls, or perform advanced Drive operations.
---

# Google Workspace CLI (gws) Skill

## Overview

`gws` is Google's official CLI for Workspace APIs. It provides direct access to:
- **Calendar** - Create/list/update events, check availability
- **Meet** - Create video meeting rooms
- **Tasks** - Create/list/complete tasks
- **People** - Search contacts
- **Drive** - Upload, download, create folders, move files

## Authentication

The CLI reads the access token from the `GOOGLE_WORKSPACE_CLI_TOKEN` environment variable.

**This is automatically provided** in your workspace environment. The system uses Service Account + Domain-Wide Delegation (DWD) for production, which means:
- No token expiration issues (auto-refresh)
- No 2FA prompts
- No user intervention needed

Just use `gws` commands directly - authentication is handled for you.

## Calendar Operations

### List Events
```bash
gws calendar events list --params '{"calendarId":"primary","timeMin":"2026-03-21T00:00:00Z","timeMax":"2026-03-28T00:00:00Z","singleEvents":true,"orderBy":"startTime"}' --format json
```

### Create Event
```bash
gws calendar events insert --params '{"calendarId":"primary","sendUpdates":"all","conferenceDataVersion":1}' --json '{
  "summary": "Q2 Planning Meeting",
  "description": "Discuss Q2 roadmap",
  "start": {"dateTime": "2026-03-22T14:00:00", "timeZone": "America/Los_Angeles"},
  "end": {"dateTime": "2026-03-22T15:00:00", "timeZone": "America/Los_Angeles"},
  "attendees": [
    {"email": "john@example.com"},
    {"email": "sarah@example.com"}
  ],
  "conferenceData": {
    "createRequest": {
      "requestId": "unique-id-123",
      "conferenceSolutionKey": {"type": "hangoutsMeet"}
    }
  }
}'
```

**Key parameters:**
- `sendUpdates`: "all" | "externalOnly" | "none" - Whether to send invite emails
- `conferenceDataVersion`: Set to 1 to enable Google Meet creation
- `conferenceData.createRequest`: Include this to auto-create a Meet link

### Update Event
```bash
gws calendar events patch --params '{"calendarId":"primary","eventId":"event123","sendUpdates":"all"}' --json '{
  "start": {"dateTime": "2026-03-22T16:00:00", "timeZone": "America/Los_Angeles"},
  "end": {"dateTime": "2026-03-22T17:00:00", "timeZone": "America/Los_Angeles"}
}'
```

### Delete Event
```bash
gws calendar events delete --params '{"calendarId":"primary","eventId":"event123","sendUpdates":"all"}'
```

### Check Free/Busy
```bash
gws calendar freebusy query --json '{
  "timeMin": "2026-03-22T00:00:00Z",
  "timeMax": "2026-03-22T23:59:59Z",
  "items": [{"id": "primary"}]
}'
```

## Google Meet

Meet links are created automatically when you include `conferenceData` in calendar events (see above).

For standalone Meet rooms (without calendar event):
```bash
gws meet spaces create --json '{}'
```

## Tasks Operations

### List Task Lists
```bash
gws tasks tasklists list --format json
```

### List Tasks
```bash
gws tasks tasks list --params '{"tasklist":"@default"}' --format json
```

### Create Task
```bash
gws tasks tasks insert --params '{"tasklist":"@default"}' --json '{
  "title": "Review proposal",
  "notes": "Check budget section",
  "due": "2026-03-28T00:00:00Z"
}'
```

### Complete Task
```bash
gws tasks tasks patch --params '{"tasklist":"@default","task":"task123"}' --json '{
  "status": "completed"
}'
```

## People/Contacts Operations

### Search Contacts
```bash
gws people people searchContacts --params '{"query":"John","readMask":"names,emailAddresses,phoneNumbers"}' --format json
```

### List Connections (all contacts)
```bash
gws people people connections list --params '{"resourceName":"people/me","personFields":"names,emailAddresses,organizations"}' --format json
```

## Drive Operations (Extended)

### List Files
```bash
gws drive files list --params '{"q":"mimeType=\"application/vnd.google-apps.folder\"","fields":"files(id,name,mimeType)"}' --format json
```

### Upload File
```bash
gws drive files create --params '{"uploadType":"multipart"}' --upload /path/to/file.pdf --json '{
  "name": "report.pdf",
  "parents": ["folder_id_here"]
}'
```

### Create Folder
```bash
gws drive files create --json '{
  "name": "Project Alpha",
  "mimeType": "application/vnd.google-apps.folder",
  "parents": ["parent_folder_id"]
}'
```

### Move File
```bash
gws drive files update --params '{"fileId":"file123","addParents":"new_folder_id","removeParents":"old_folder_id"}'
```

### Copy File
```bash
gws drive files copy --params '{"fileId":"file123"}' --json '{
  "name": "Copy of Document",
  "parents": ["destination_folder_id"]
}'
```

### Search Files
```bash
gws drive files list --params '{"q":"name contains \"report\" and modifiedTime > \"2026-03-01T00:00:00\"","fields":"files(id,name,modifiedTime,webViewLink)"}' --format json
```

## Common Patterns

### Create Meeting with Meet Link and Notify Attendees
```bash
gws calendar events insert --params '{"calendarId":"primary","sendUpdates":"all","conferenceDataVersion":1}' --json '{
  "summary": "Team Sync",
  "start": {"dateTime": "2026-03-22T10:00:00", "timeZone": "America/Los_Angeles"},
  "end": {"dateTime": "2026-03-22T10:30:00", "timeZone": "America/Los_Angeles"},
  "attendees": [{"email": "team@example.com"}],
  "conferenceData": {"createRequest": {"requestId": "'$(uuidgen)'", "conferenceSolutionKey": {"type": "hangoutsMeet"}}}
}'
```

### Check Today's Schedule
```bash
TODAY=$(date -u +%Y-%m-%dT00:00:00Z)
TOMORROW=$(date -u -d "+1 day" +%Y-%m-%dT00:00:00Z)
gws calendar events list --params "{\"calendarId\":\"primary\",\"timeMin\":\"$TODAY\",\"timeMax\":\"$TOMORROW\",\"singleEvents\":true,\"orderBy\":\"startTime\"}" --format json
```

## Error Handling

Check exit codes:
- `0` = Success
- `1` = API error (check response body)
- `2` = Auth error (token invalid/expired)
- `3` = Validation error (bad arguments)

## Schema Discovery

To see all available parameters for any API:
```bash
gws schema calendar.events.insert
gws schema drive.files.create
gws schema tasks.tasks.insert
```

## Memory Integration

When user mentions people or teams, save to memo.md:
```markdown
## Contacts
- "Engineering team": alice@company.com, bob@company.com
- "Advisor": prof.zhang@university.edu
```

When user confirms timezone preference:
```markdown
## Preferences
- Timezone: America/Los_Angeles
- Default meeting duration: 30 minutes
```
