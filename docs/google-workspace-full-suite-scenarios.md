# Google Workspace Full Suite - User Scenario Document

> **Status**: Draft for Review
> **Branch**: `feature/google-workspace-full-suite`
> **Date**: 2026-03-21

This document describes the target user experience for each new Google Workspace feature. Please review and provide feedback before implementation begins.

---

## 0. Memory-Based Preference Learning

> **Core Principle**: Ask once, confirm once, remember forever.

All user preferences are stored in `memo.md` under the user's memory directory. The agent learns from each interaction and avoids repetitive questions.

### 0.1 Memo Structure for Workspace Features

```markdown
# Memo

## Profile
- Timezone: America/Los_Angeles (confirmed 2026-03-21)
- Default meeting duration: 30 minutes
- Preferred meeting times: 10am-12pm, 2pm-5pm

## Preferences
- Calendar notifications: Email only (no SMS)
- Always add Google Meet to meetings: Yes
- Ask before sending calendar invites: No (send automatically)
- Default file sharing: "Anyone with link can view"

## Contacts
- "Engineering team": alice@company.com, bob@company.com, carol@company.com
- "Design team": david@company.com, eve@company.com
- "Advisor/Supervisor": prof.zhang@university.edu
- "Manager": sarah.manager@company.com

## Projects
- Current project: "Q2 Product Launch"
- Project folder: https://drive.google.com/drive/folders/xxx

## Decisions
- 2026-03-21: User prefers casual tone for agent replies
- 2026-03-20: User wants task reminders 1 day before due date
```

### 0.2 Learning Flow Example

**First Interaction** (Timezone unknown):

> **User**: "Schedule a meeting tomorrow at 2pm"
>
> **Agent**: "I'll schedule that for 2:00 PM. What timezone should I use? (I'll remember this for future meetings)"
>
> **User**: "Pacific time"
>
> **Agent**: "Got it! I've scheduled the meeting for 2:00 PM Pacific (America/Los_Angeles).
> I've saved this as your default timezone. Want me to always use Pacific time, or ask each time?"
>
> **User**: "Always use Pacific"
>
> **Agent**: "Perfect, I'll use Pacific time for all your calendar events going forward."

**Subsequent Interactions** (Timezone remembered):

> **User**: "Schedule a call at 3pm next Monday"
>
> **Agent**: "Done! Scheduled for Monday, March 24 at 3:00 PM Pacific."

*(No timezone question needed)*

### 0.3 Team/Group Resolution Example

**First Mention**:

> **User**: "Share this doc with the engineering team"
>
> **Agent**: "I don't have the engineering team members saved yet. Who should I include?"
>
> **User**: "alice@company.com, bob@company.com, and carol@company.com"
>
> **Agent**: "Shared with all three! I've saved them as your 'engineering team' for future requests."

**Subsequent Mentions**:

> **User**: "Invite the engineering team to a standup tomorrow"
>
> **Agent**: "Created standup meeting with alice@company.com, bob@company.com, carol@company.com for tomorrow at your usual standup time (9:00 AM Pacific)."

### 0.4 Preference Confirmation Pattern

When agent learns something new, confirm with user:

```
Agent: "I notice you often schedule 30-minute meetings. Should I default to 30 minutes
       unless you specify otherwise?"
User: "Yes"
Agent: "Got it, 30 minutes is now your default meeting duration."

[Saved to memo.md: Default meeting duration: 30 minutes]
```

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
1. Find engineering team contacts (from memo.md Contacts section)
2. Create calendar event with Meet link
3. Create Google Drive folder
4. Share folder with attendees
5. Create task for agenda prep

**Agent Reply**:
> All set! Here's what I've created:
>
> **Meeting**: Project Kickoff
> - When: Monday, March 24, 10:00 AM (Pacific)
> - Who: alice@company.com, bob@company.com, carol@company.com
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

### 6.2 Data Analysis Report with Meeting (Full Pipeline)

**User Input**:
> "根据 'Q1 Sales Data' Google Sheet 里的数据生成一份 Google Doc 报告，要包含数据趋势图，生成后把 Doc 发给我，共享给导师，并帮我预约明天下午五点与导师的 meeting 讨论数据内容"

**Agent Behavior (Step by Step)**:

1. **Read Sheet Data**
   - Find "Q1 Sales Data" spreadsheet in user's Drive
   - Read all data using `google-sheets read-spreadsheet`
   - Parse data structure (columns, rows, types)

2. **Generate Charts**
   - Analyze data to determine appropriate chart types
   - Create trend charts using Google Sheets Charts API or generate locally
   - Export charts as images

3. **Create Report Document**
   - Create new Google Doc: "Q1 Sales Data Analysis Report"
   - Insert report structure:
     - Executive Summary
     - Data Overview (key metrics)
     - Trend Analysis (with embedded charts)
     - Conclusions & Recommendations
   - Apply consistent styling

4. **Share Document**
   - Share with user (Editor)
   - Share with "导师" / supervisor from memo.md (Commenter or Editor based on preference)
   - Get shareable link

5. **Schedule Meeting**
   - Resolve "导师" from memo.md Contacts: `prof.zhang@university.edu`
   - Create calendar event: "Q1 Sales Data Discussion"
   - Time: Tomorrow 5:00 PM (user's timezone from memo.md)
   - Add Google Meet link
   - Attach document link to meeting description
   - Send calendar invite

6. **Reply to User**

**Agent Reply**:
> 全部完成！
>
> **报告文档**: Q1 Sales Data Analysis Report
> - 链接: https://docs.google.com/document/d/xxx/edit
> - 包含: 执行摘要、数据概览、趋势图表分析、结论建议
> - 已共享给你 (可编辑) 和 Prof. Zhang (可评论)
>
> **会议已预约**: Q1 Sales Data Discussion
> - 时间: 明天 (3月22日) 下午 5:00 - 6:00 (Pacific)
> - 参与者: prof.zhang@university.edu
> - Google Meet: https://meet.google.com/abc-defg-hij
> - 会议描述中已附上报告链接
>
> 导师会收到日历邀请和文档共享通知。还需要我做什么吗？

**If "导师" Not in Contacts**:

> Agent: "我需要知道你导师的邮箱地址来共享文档和发送会议邀请。"
>
> User: "prof.zhang@university.edu"
>
> Agent: "收到！我已经将 prof.zhang@university.edu 保存为你的'导师'，以后提到导师时会自动使用这个邮箱。"
>
> [Saved to memo.md: "Advisor/Supervisor": prof.zhang@university.edu]

---

### 6.3 Weekly Report Automation

**User Input**:
> "Every Friday at 4pm, compile data from my 'Weekly Metrics' sheet, generate a summary doc, and share it with the team"

**Agent Behavior**:
1. Create recurring task/schedule
2. On trigger:
   - Read latest data from "Weekly Metrics" sheet
   - Generate summary document with charts
   - Share with "team" (from memo.md)
   - Notify user when complete

**Agent Reply**:
> Set up weekly automation:
>
> **Schedule**: Every Friday at 4:00 PM Pacific
> **Actions**:
> 1. Read "Weekly Metrics" spreadsheet
> 2. Generate summary document with latest data
> 3. Share with team: alice@, bob@, carol@ (Editor)
> 4. Send you a confirmation when done
>
> First run will be this Friday. I'll let you know when it's complete!

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
google-drive find-by-name --name="Q1 Sales Data" [--type=spreadsheet|document|presentation]
google-drive get-info <file_id>
```

### google-sheets CLI (extended for reports)
```
google-sheets read-spreadsheet <id>
google-sheets read-values <id> <range>
google-sheets get-metadata <id>
google-sheets create-chart <id> --type=line|bar|pie --data-range="A1:D10" --title="..."
google-sheets export-chart <id> <chart_id> --output=chart.png
google-sheets analyze-data <id> --summary  # Returns key metrics, trends, etc.
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

**Basic Feature Tests**:
1. **Calendar E2E**: Create event -> Verify in calendar -> Update -> Delete
2. **Meet E2E**: Create space -> Get join URL -> Verify accessible
3. **Drive E2E**: Upload file -> Move to folder -> Share -> Download -> Delete
4. **Tasks E2E**: Create list -> Add tasks -> Complete -> Delete

**Memory/Preference Tests**:
5. **Timezone Learning**: First meeting (ask timezone) -> Second meeting (no ask)
6. **Team Resolution**: First share with "team" (ask members) -> Second share (auto-resolve)
7. **Preference Persistence**: Set preference -> Restart service -> Verify preference retained

**Cross-Service Workflow Tests**:
8. **Meeting + Folder + Task**: Create meeting -> Create folder -> Share -> Add task
9. **Full Report Pipeline** (Critical):
   - Input: "Generate report from Sheet X, share with advisor, schedule meeting"
   - Verify:
     - Sheet data read correctly
     - Charts generated and embedded
     - Doc created with proper structure
     - Doc shared with correct permissions
     - Meeting created at correct time
     - Meeting has Meet link
     - Meeting invite sent to advisor
     - All links/references correct

**Error Recovery Tests**:
10. **Partial Workflow Failure**: Simulate Drive API failure mid-workflow -> Verify graceful handling
11. **Missing Permission**: Calendar access revoked -> Proper error message

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

## 9. Design Decisions (Based on Feedback)

### Resolved (Memory-Based Approach)

| Question | Decision |
|----------|----------|
| Timezone | Ask once on first calendar action, save to memo.md, never ask again |
| Team resolution | Learn from first mention ("engineering team" = emails), save to Contacts section |
| Default duration | Observe patterns, suggest default, save preference |
| Error tone | Ask once on first error, save preference |

### Still Need Input

1. **Task reminders**: Should agent proactively remind users of upcoming tasks?
   - Option A: Yes, remind 1 day before due date (save preference in memo.md)
   - Option B: Only when user asks
   - Option C: Ask user preference on first task creation

2. **Drive upload limits**: Max file size for uploads?
   - Suggested: 100MB (Google Drive API limit for simple upload)
   - For larger files: "This file is 250MB. I'll need to use resumable upload which takes longer. Proceed?"

3. **Chart generation for reports**: How should we generate charts from Sheet data?
   - Option A: Use Google Sheets embedded charts (export as image)
   - Option B: Generate locally with Python/matplotlib (more control)
   - Option C: Use Google Charts API
   - **Recommendation**: Option A for simplicity, fallback to B for custom styling

4. **Cross-service workflow atomicity**: If one step fails mid-workflow, should we:
   - Option A: Rollback all completed steps
   - Option B: Keep completed steps, report what failed
   - Option C: Ask user what to do
   - **Recommendation**: Option B (keep progress, let user decide)

---

## 10. Implementation Priority

| Priority | Feature | Estimated Effort | Dependencies |
|----------|---------|------------------|--------------|
| P0 | Memory/Preference integration (memo.md read/write) | 1 day | Existing memory_store |
| P0 | Calendar - Create/List/Update events | 2 days | OAuth scopes |
| P0 | Calendar - Add Meet link | 0.5 day | Calendar done |
| P1 | Meet - Create instant space | 1 day | OAuth scopes |
| P1 | Drive - Upload/Download/Move/Search | 2 days | Existing auth |
| P1 | Drive - Create folder/Find by name | 0.5 day | Drive basics |
| P1 | Sheets - Chart creation & export | 1.5 days | Existing sheets CLI |
| P2 | Tasks - Full CRUD | 1.5 days | OAuth scopes |
| P2 | Contacts - Search/Create (for team resolution) | 1 day | OAuth scopes |
| P3 | Cross-service workflows (report pipeline) | 2 days | All above |
| P3 | E2E test suite | 1 day | All above |

**Total estimated**: ~14 days

### Implementation Order

```
Week 1:
├── Day 1-2: Memory preference system + Calendar basics
├── Day 3: Calendar + Meet integration
├── Day 4-5: Drive full operations

Week 2:
├── Day 1-2: Sheets chart creation + export
├── Day 3: Tasks integration
├── Day 4: Contacts integration (team resolution)
├── Day 5: Cross-service workflow integration

Week 3:
├── Day 1-2: E2E testing with real accounts
├── Day 3: Bug fixes and polish
```

---

## Next Steps

1. [x] Review this document and provide feedback
2. [ ] Confirm remaining design decisions (Section 9)
3. [ ] Provide test Google account credentials
4. [ ] Help with OAuth scope authorization (Calendar, Meet, Drive, Tasks, Contacts)
5. [ ] Begin implementation:
   - Phase 1: Memory preference system + Calendar (P0)
   - Phase 2: Drive + Sheets charts (P1)
   - Phase 3: Tasks + Contacts + Cross-service (P2-P3)
6. [ ] E2E testing with your real account

---

## Appendix: Test Account Requirements

For E2E testing, I'll need access to a Google account with:

- Google Calendar enabled
- Google Drive with some test spreadsheets
- Google Tasks enabled
- Google Contacts with some test entries

**What I need from you**:
1. OAuth credentials (client_id, client_secret) with required scopes
2. Initial refresh_token for the test account
3. A test spreadsheet ID to use for report generation tests
4. A test email address to use as "advisor" for sharing tests

**Privacy Note**: All test data created will be clearly labeled (e.g., "[TEST] Q1 Report") and can be deleted after testing.
