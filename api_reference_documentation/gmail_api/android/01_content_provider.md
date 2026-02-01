# Android Content Provider for Gmail

## Overview

The Android Gmail app provides a content provider that third-party developers can leverage to retrieve label information including name and unread count, and receive updates as data changes. An app or widget could display the unread count of a specific account's inbox, for example.

**Compatibility Note:** Support extends to versions after 2.3.6 (Froyo/Gingerbread) and 4.0.5 (Honeycomb/ICS).

Before querying, call the `GmailContract.canReadLabels(Context)` method to verify the user's Gmail app version supports these operations.

## Finding a Valid Gmail Account

Applications must first identify an email address associated with a valid Gmail account. Using the `GET_ACCOUNTS` permission, the `AccountManager` class can supply this information:

```java
final String ACCOUNT_TYPE_GOOGLE = "com.google";
final String[] FEATURES_MAIL = {
        "service_mail"
};
AccountManager.get(this).getAccountsByTypeAndFeatures(ACCOUNT_TYPE_GOOGLE, FEATURES_MAIL,
        new AccountManagerCallback() {
            @Override
            public void run(AccountManagerFuture future) {
                Account[] accounts = null;
                try {
                    accounts = future.getResult();
                    if (accounts != null && accounts.length > 0) {
                        String selectedAccount = accounts[0].name;
                        queryLabels(selectedAccount);
                    }
                } catch (OperationCanceledException oce) {
                    // TODO: handle exception
                } catch (IOException ioe) {
                    // TODO: handle exception
                } catch (AuthenticatorException ae) {
                    // TODO: handle exception
                }
            }
        }, null /* handler */);
```

## Querying the Content Provider

With an email address selected, construct a `ContentProvider` URI for querying. The `GmailContract.java` class facilitates URI construction and column definition.

Query the URI directly or use a `CursorLoader` to obtain label information:

```java
Cursor labelsCursor = getContentResolver().query(
    GmailContract.Labels.getLabelsUri(selectedAccount),
    null, null, null, null);
```

The returned cursor contains data for all account labels. You can persist the URI value in the `GmailContract.Labels.URI` column to monitor individual label changes.

### Identifying Pre-defined Labels

Pre-defined label names vary by locale, so use the `GmailContract.Labels.CANONICAL_NAME` column instead of the `NAME` column:

```java
if (labelsCursor != null) {
    final String inboxCanonicalName =
        GmailContract.Labels.LabelCanonicalName.CANONICAL_NAME_INBOX;
    final int canonicalNameIndex =
        labelsCursor.getColumnIndexOrThrow(GmailContract.Labels.CANONICAL_NAME);
    while (labelsCursor.moveToNext()) {
        if (inboxCanonicalName.equals(labelsCursor.getString(canonicalNameIndex))) {
            // this row corresponds to the Inbox
        }
    }
}
```

## Additional Resources

- **Content provider basics:** See the Android developer documentation for foundational concepts
- **Sample app:** A downloadable sample demonstrates this content provider in action
