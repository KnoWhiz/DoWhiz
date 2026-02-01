# Managing Filters

## Overview

Filters enable automated message handling through advanced filtering rules. They can automatically apply or remove labels or forward emails to verified aliases based on incoming message attributes.

## Matching Criteria

Messages can be filtered by various properties using Gmail's advanced search syntax. Common filter patterns include:

| Filter | Matches |
|--------|---------|
| `criteria.from='sender@example.com'` | Messages from specified sender |
| `criteria.size=10485760` with `criteria.sizeComparison='larger'` | Messages exceeding 10MB |
| `criteria.hasAttachment=true` | Messages containing attachments |
| `criteria.subject='[People with Pets]'` | Specific subject line strings |
| `criteria.query='"my important project"'` | Messages containing exact phrases |
| `criteria.negatedQuery='"secret knock"'` | Messages excluding specific phrases |

**Note:** When multiple criteria exist, a message must satisfy all conditions for filter application.

## Actions

Filters apply actions to matching messages. Common actions include:

| Action | Effect |
|--------|--------|
| `action.removeLabelIds=['INBOX']` | Archive messages |
| `action.removeLabelIds=['UNREAD']` | Mark as read |
| `action.removeLabelIds=['SPAM']` | Prevent spam marking |
| `action.removeLabelIds=['IMPORTANT']` | Prevent importance flagging |
| `action.addLabelIds=['IMPORTANT']` | Mark as important |
| `action.addLabelIds=['TRASH']` | Delete messages |
| `action.addLabelIds=['STARRED']` | Mark as starred |
| `action.addLabelIds=['<user label id>']` | Apply custom labels |

## Code Examples

### Java Implementation

```java
Filter filter = new Filter()
    .setCriteria(new FilterCriteria()
        .setFrom("gduser2@workspacesamples.dev"))
    .setAction(new FilterAction()
        .setAddLabelIds(Arrays.asList(labelId))
        .setRemoveLabelIds(Arrays.asList("INBOX")));

Filter result = service.users().settings().filters().create("me", filter).execute();
```

### Python Implementation

```python
filter_content = {
    "criteria": {"from": "gsuder1@workspacesamples.dev"},
    "action": {
        "addLabelIds": [label_name],
        "removeLabelIds": ["INBOX"],
    },
}

result = (
    service.users()
    .settings()
    .filters()
    .create(userId="me", body=filter_content)
    .execute()
)
```

## API Reference

Access comprehensive documentation for create, list, get, and delete operations via the Filters reference.
