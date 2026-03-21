# Google Workspace Full Suite - User Scenario Document

> **Status**: Draft for Review
> **Branch**: `feature/google-workspace-full-suite`
> **Date**: 2026-03-21

This document describes the target user experience for each new Google Workspace feature. Please review and provide feedback before implementation begins.

---

## 1. Google Calendar

### 1.1 Create Meeting/Event

**User Input** (any channel: Email, Slack, Discord, Notion, etc.):
> "Help me schedule a meeting with john@example.com and sarah@example.com tomorrow at 2pm for 1 hour to discuss Q2 planning"

**Agent Behavior**:
1. Parse: attendees, time, duration, title/purpose
2. Create calendar event via Google Calendar API
3. Send invitations to attendees (Google handles this)
4. Reply to user on same channel with confirmation:

**Agent Reply**:
> Meeting scheduled!
>
> **Q2 Planning Discussion**
> - Date: March 22, 2026, 2:00 PM - 3:00 PM (your timezone)
> - Attendees: john@example.com, sarah@example.com
> - Calendar link: [View in Google Calendar](https://calendar.google.com/...)
>
> Invitations have been sent to all attendees.

**Edge Cases**:
- Time conflict detection: "You have a conflict at 2pm. Would you like me to schedule at 3pm instead?"
- Missing info: "What time would you like to schedule this meeting?"
- Timezone handling: Use user's configured timezone or ask

---

### 1.2 Check Availability / Free-Busy Query

**User Input**:
> "When am I free tomorrow afternoon?"

**Agent Reply**:
> Tomorrow (March 22) afternoon availability:
> - 1:00 PM - 2:00 PM: Free
> - 2:00 PM - 3:00 PM: Q2 Planning (busy)
> - 3:00 PM - 5:00 PM: Free

---

### 1.3 List Upcoming Events

**User Input**:
> "What's on my calendar this week?"

**Agent Reply**:
> Your schedule for this week:
>
> **Monday, March 24**
> - 9:00 AM - 10:00 AM: Team Standup
> - 2:00 PM - 3:00 PM: 1:1 with Manager
>
> **Tuesday, March 25**
> - 10:00 AM - 11:30 AM: Product Review
>
> [No events Wednesday-Friday]

---

### 1.4 Modify/Cancel Event

**User Input**:
> "Move my 2pm meeting tomorrow to 4pm"

**Agent Behavior**:
1. Find the event at 2pm tomorrow
2. Update to 4pm
3. Google Calendar sends update notifications to attendees

**Agent Reply**:
> Done! Moved "Q2 Planning Discussion" from 2:00 PM to 4:00 PM.
> All attendees have been notified of the change.

---

### 1.5 Add Google Meet to Event

**User Input**:
> "Add a video call link to my 2pm meeting"

**Agent Reply**:
> Added Google Meet to "Q2 Planning Discussion"
> - Meet link: https://meet.google.com/abc-defg-hij
>
> All attendees will see the link in their calendar invite.

---

## 2. Google Meet

### 2.1 Create Instant Meeting

**User Input**:
> "Create a Google Meet for a quick call right now"

**Agent Reply**:
> Created Google Meet room:
> - Join link: https://meet.google.com/xyz-abcd-efg
> - No expiration (available until ended)
>
> Share this link with anyone you want to join.

---

### 2.2 Schedule Meeting with Meet Link

**User Input**:
> "Schedule a video call with the design team next Monday at 10am"

**Agent Behavior**:
1. Create calendar event
2. Automatically add Google Meet conferencing
3. Invite attendees (if design team members are known/configured)

**Agent Reply**:
> Scheduled: Design Team Video Call
> - Monday, March 24, 10:00 AM - 11:00 AM
> - Google Meet: https://meet.google.com/abc-defg-hij
> - Attendees: [design-team@company.com or individual members]
>
> Calendar invites sent with Meet link included.

---

## 3. Google Drive (Full Operations)

### 3.1 Upload File

**User Input** (with attachment):
> "Upload this report to my Google Drive in the Q2 Reports folder"

**Agent Behavior**:
1. Receive file attachment
2. Find or create "Q2 Reports" folder
3. Upload file to that folder

**Agent Reply**:
> Uploaded "quarterly-report.pdf" to Google Drive
> - Location: My Drive / Q2 Reports / quarterly-report.pdf
> - Link: https://drive.google.com/file/d/xxx/view
> - Sharing: Private (only you can access)
>
> Would you like me to share it with anyone?

---

### 3.2 Create Folder

**User Input**:
> "Create a new folder called 'Project Alpha' in my Drive"

**Agent Reply**:
> Created folder "Project Alpha"
> - Location: My Drive / Project Alpha
> - Link: https://drive.google.com/drive/folders/xxx

---

### 3.3 Move/Copy Files

**User Input**:
> "Move the Q1 report to the Archive folder"

**Agent Reply**:
> Moved "Q1-Report.pdf" from "Reports" to "Archive"
> - New location: My Drive / Archive / Q1-Report.pdf

---

### 3.4 Search Files

**User Input**:
> "Find all spreadsheets I edited last week"

**Agent Reply**:
> Found 3 spreadsheets edited last week:
>
> 1. Budget-2026.xlsx (modified Mar 18)
> 2. Team-Schedule.gsheet (modified Mar 17)
> 3. Expense-Tracker.gsheet (modified Mar 15)
>
> Want me to open or share any of these?

---

### 3.5 Share with Specific Permissions

**User Input**:
> "Share the Project Alpha folder with the engineering team as editors"

**Agent Behavior**:
1. Resolve "engineering team" to email addresses (from contacts or config)
2. Apply editor permissions

**Agent Reply**:
> Shared "Project Alpha" folder:
> - alice@company.com: Editor
> - bob@company.com: Editor
> - carol@company.com: Editor
>
> They'll receive email notifications with the link.

---

## 4. Google Tasks

### 4.1 Create Task

**User Input**:
> "Remind me to review the proposal by Friday"

**Agent Reply**:
> Created task: "Review the proposal"
> - Due: Friday, March 28
> - List: My Tasks
>
> I'll remind you before the deadline.

---

### 4.2 List Tasks

**User Input**:
> "What tasks do I have pending?"

**Agent Reply**:
> Your pending tasks:
>
> **Due Today**
> - [ ] Send invoice to client
>
> **Due This Week**
> - [ ] Review the proposal (Due: Mar 28)
> - [ ] Prepare presentation slides (Due: Mar 27)
>
> **No Due Date**
> - [ ] Update documentation

---

### 4.3 Complete Task

**User Input**:
> "Mark the invoice task as done"

**Agent Reply**:
> Marked as complete: "Send invoice to client"
>
> Remaining tasks due today: 0

---

### 4.4 Create Task List

**User Input**:
> "Create a new task list for the website redesign project"

**Agent Reply**:
> Created task list: "Website Redesign"
>
> You can now add tasks to this list. Example:
> "Add task 'Create wireframes' to Website Redesign list"

---

## 5. Google People/Contacts

### 5.1 Find Contact

**User Input**:
> "What's John Smith's email?"

**Agent Reply**:
> Found contact: John Smith
> - Email: john.smith@example.com
> - Phone: +1 555-123-4567
> - Company: Acme Corp

---

### 5.2 Create Contact

**User Input**:
> "Add a new contact: Lisa Chen, lisa@startup.io, Product Manager at TechCo"

**Agent Reply**:
> Added contact: Lisa Chen
> - Email: lisa@startup.io
> - Title: Product Manager
> - Company: TechCo

---

### 5.3 Search Contacts

**User Input**:
> "Who do I know at Google?"

**Agent Reply**:
> Contacts at Google:
>
> 1. Sarah Johnson - Engineering Manager
>    sarah.j@google.com
>
> 2. Mike Lee - Product Designer
>    mike.lee@google.com

---

## 6. Cross-Feature Workflows

### 6.1 Complete Meeting Setup

**User Input**:
> "Set up a project kickoff meeting with the engineering team next week, create a shared folder for project docs, and add a task to prepare the agenda"

**Agent Behavior**:
1. Find engineering team contacts
2. Create calendar event with Meet link
3. Create Google Drive folder
4. Share folder with attendees
5. Create task for agenda prep

**Agent Reply**:
> All set! Here's what I've created:
>
> **Meeting**: Project Kickoff
> - When: Monday, March 24, 10:00 AM
> - Who: engineering-team@company.com
> - Where: https://meet.google.com/abc-defg-hij
>
> **Shared Folder**: Project Kickoff Docs
> - Link: https://drive.google.com/drive/folders/xxx
> - Shared with: All meeting attendees (Editor access)
>
> **Task**: Prepare project kickoff agenda
> - Due: Sunday, March 23 (day before meeting)
>
> Anything else you'd like me to add?

---

## 7. CLI Commands (For Agent Tools)

Each feature will have a corresponding CLI for agent use:

### google-calendar CLI
```
google-calendar list-events [--start=DATE] [--end=DATE]
google-calendar get-event <event_id>
google-calendar create-event --title="..." --start="..." --end="..." [--attendees="a@x.com,b@y.com"] [--meet]
google-calendar update-event <event_id> [--title=...] [--start=...] [--end=...]
google-calendar delete-event <event_id>
google-calendar freebusy --start="..." --end="..." [--attendees="..."]
```

### google-meet CLI
```
google-meet create-space [--title="..."]
google-meet get-space <space_id>
```

### google-drive CLI (extended)
```
google-drive list [--folder=ID] [--query="..."]
google-drive upload <local_path> [--folder=ID] [--name="..."]
google-drive download <file_id> <local_path>
google-drive create-folder --name="..." [--parent=ID]
google-drive move <file_id> --to=FOLDER_ID
google-drive copy <file_id> [--name="..."] [--to=FOLDER_ID]
google-drive delete <file_id>
google-drive search --query="..."
```

### google-tasks CLI
```
google-tasks list-tasklists
google-tasks list-tasks [--tasklist=ID]
google-tasks create-task --title="..." [--due=DATE] [--tasklist=ID] [--notes="..."]
google-tasks complete-task <task_id> [--tasklist=ID]
google-tasks delete-task <task_id> [--tasklist=ID]
google-tasks create-tasklist --title="..."
```

### google-contacts CLI
```
google-contacts search --query="..."
google-contacts get <contact_id>
google-contacts create --name="..." [--email="..."] [--phone="..."] [--company="..."]
google-contacts update <contact_id> [--name=...] [--email=...] [--phone=...]
google-contacts delete <contact_id>
```

---

## 8. Testing Strategy

### 8.1 Unit Tests
- API request/response parsing
- Date/time handling across timezones
- Attendee email validation
- Error handling

### 8.2 Integration Tests (with real API)
```bash
# Environment variables needed:
GOOGLE_CALENDAR_TEST=1
GOOGLE_MEET_TEST=1
GOOGLE_DRIVE_TEST=1
GOOGLE_TASKS_TEST=1
GOOGLE_CONTACTS_TEST=1
```

### 8.3 E2E Test Scenarios
1. **Calendar E2E**: Create event -> Verify in calendar -> Update -> Delete
2. **Meet E2E**: Create space -> Get join URL -> Verify accessible
3. **Drive E2E**: Upload file -> Move to folder -> Share -> Download -> Delete
4. **Tasks E2E**: Create list -> Add tasks -> Complete -> Delete
5. **Cross-service E2E**: Create meeting with attendees, folder, and tasks

### 8.4 Required OAuth Scopes
```
https://www.googleapis.com/auth/calendar
https://www.googleapis.com/auth/calendar.events
https://www.googleapis.com/auth/meetings.space.created
https://www.googleapis.com/auth/drive
https://www.googleapis.com/auth/tasks
https://www.googleapis.com/auth/contacts
```

---

## 9. Questions for Review

1. **Calendar timezone**: Should we default to user's Google account timezone, or require explicit timezone in requests?

2. **Meeting attendees**: How should we resolve "engineering team" to actual email addresses?
   - Option A: Pre-configured groups in employee.toml
   - Option B: Query Google Groups
   - Option C: Query contacts for matching company/title

3. **Task reminders**: Should the agent proactively remind users of upcoming tasks, or only respond when asked?

4. **Drive upload limits**: Should we set a file size limit for uploads through the agent?

5. **Contact privacy**: Should we limit contact search to user's own contacts, or include directory (if Google Workspace admin)?

6. **Error messages**: Preferred tone for errors?
   - Formal: "Unable to create event. Calendar access denied."
   - Casual: "Hmm, I can't access your calendar. Can you check permissions?"

---

## 10. Implementation Priority

| Priority | Feature | Estimated Effort | Dependencies |
|----------|---------|------------------|--------------|
| P0 | Calendar - Create/List/Update events | 2 days | OAuth scopes |
| P0 | Calendar - Add Meet link | 0.5 day | Calendar done |
| P1 | Meet - Create instant space | 1 day | OAuth scopes |
| P1 | Drive - Upload/Download/Move | 2 days | Existing auth |
| P1 | Drive - Create folder/Search | 1 day | Drive basics |
| P2 | Tasks - Full CRUD | 1.5 days | OAuth scopes |
| P2 | Contacts - Search/Create | 1 day | OAuth scopes |
| P3 | Cross-service workflows | 1 day | All above |

**Total estimated**: ~10 days

---

## Next Steps

1. [ ] Review this document and provide feedback
2. [ ] Confirm OAuth scope additions are acceptable
3. [ ] Provide test account credentials / help with OAuth setup
4. [ ] Begin implementation starting with Calendar (P0)
