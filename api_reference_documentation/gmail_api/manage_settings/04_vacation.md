# Managing Vacation Settings

## Overview

The Gmail API enables developers to configure scheduled auto-reply functionality for user accounts through the Settings resource. This feature allows applications to manage vacation responses programmatically.

## Auto-Reply Configuration

### Key Capabilities

Auto-reply requires specific elements to function:
- A response subject line and message body (HTML or plain text format)
- Optional time-based restrictions (start and end dates)
- Optional domain restrictions (replies only to same domain contacts)

The feature can operate indefinitely or within a defined timeframe. Once an `endTime` is configured, the auto-reply automatically disables upon reaching that timestamp.

### Implementation Example (Java)

```java
import com.google.api.services.gmail.model.VacationSettings;

VacationSettings vacationSettings = new VacationSettings()
    .setEnableAutoReply(true)
    .setResponseBodyHtml(
        "I am on vacation and will reply when I am back in the office. Thanks!")
    .setRestrictToDomain(true)
    .setStartTime(LocalDateTime.now()
        .toEpochSecond(ZoneOffset.from(ZonedDateTime.now())) * 1000)
    .setEndTime(LocalDateTime.now().plusDays(7)
        .toEpochSecond(ZoneOffset.from(ZonedDateTime.now())) * 1000);

VacationSettings response = service.users().settings()
    .updateVacation("me", vacationSettings).execute();
```

### Implementation Example (Python)

```python
vacation_settings = {
    "enableAutoReply": True,
    "responseBodyHtml": (
        "I am on vacation and will reply when I am "
        "back in the office. Thanks!"
    ),
    "restrictToDomain": True,
    "startTime": long(start_time),
    "endTime": long(end_time),
}

response = (
    service.users()
    .settings()
    .updateVacation(userId="me", body=vacation_settings)
    .execute()
)
```

## Disabling Auto-Reply

To deactivate auto-reply, update the resource settings and set `enableAutoReply` to `false`.

## Related Resources

- Settings API Reference
- Get Vacation Settings
- Update Vacation Settings
