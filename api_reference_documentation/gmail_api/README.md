# Gmail API Documentation

This documentation is a comprehensive reference for the Gmail API and related Google email services. It has been organized to be easily navigable and understandable by both humans and LLMs.

**Source:** https://developers.google.com/workspace/gmail/api/guides

---

## Table of Contents

### 1. Authentication & Authorization
- [01_scopes.md](auth/01_scopes.md) - Choose Gmail API Scopes
- [02_web_server_authorization.md](auth/02_web_server_authorization.md) - Implement Server-Side Authorization

### 2. Quickstarts
Step-by-step guides to get started with the Gmail API in different programming languages:
- [01_javascript.md](quickstarts/01_javascript.md) - JavaScript Quickstart
- [02_java.md](quickstarts/02_java.md) - Java Quickstart
- [03_python.md](quickstarts/03_python.md) - Python Quickstart
- [04_apps_script.md](quickstarts/04_apps_script.md) - Google Apps Script Quickstart
- [05_go.md](quickstarts/05_go.md) - Go Quickstart
- [06_nodejs.md](quickstarts/06_nodejs.md) - Node.js Quickstart

### 3. Create & Send Mail
- [01_drafts.md](create_send_mail/01_drafts.md) - Working with Drafts
- [02_sending.md](create_send_mail/02_sending.md) - Sending Email
- [03_uploads.md](create_send_mail/03_uploads.md) - Uploading Attachments

### 4. Manage Mailboxes
- [01_threads.md](manage_mailboxes/01_threads.md) - Managing Threads
- [02_labels.md](manage_mailboxes/02_labels.md) - Manage Labels
- [03_filtering.md](manage_mailboxes/03_filtering.md) - Searching for Messages
- [04_list_messages.md](manage_mailboxes/04_list_messages.md) - List Gmail Messages
- [05_sync.md](manage_mailboxes/05_sync.md) - Synchronizing Clients with Gmail
- [06_push_notifications.md](manage_mailboxes/06_push_notifications.md) - Push Notifications

### 5. Manage Settings
- [01_aliases_signatures.md](manage_settings/01_aliases_signatures.md) - Managing Aliases and Signatures
- [02_forwarding.md](manage_settings/02_forwarding.md) - Managing Forwarding
- [03_filters.md](manage_settings/03_filters.md) - Managing Filters
- [04_vacation.md](manage_settings/04_vacation.md) - Managing Vacation Settings
- [05_smime_certs.md](manage_settings/05_smime_certs.md) - Managing S/MIME Certificates
- [06_pop_imap.md](manage_settings/06_pop_imap.md) - Managing POP and IMAP
- [07_delegates.md](manage_settings/07_delegates.md) - Managing Delegates
- [08_language.md](manage_settings/08_language.md) - Manage Language Settings
- [09_inbox_feed.md](manage_settings/09_inbox_feed.md) - Gmail Inbox Feed

### 6. Best Practices
- [01_batch.md](best_practices/01_batch.md) - Batching Requests
- [02_performance.md](best_practices/02_performance.md) - Performance Tips
- [03_handle_errors.md](best_practices/03_handle_errors.md) - Resolve Errors

### 7. Troubleshooting
- [01_auth_troubleshooting.md](troubleshooting/01_auth_troubleshooting.md) - Troubleshoot Authentication & Authorization Issues

### 8. Migration
- [01_migrate_from_email_settings.md](migration/01_migrate_from_email_settings.md) - Migrating from the Email Settings API

### 9. IMAP for Gmail
- [01_imap_smtp_overview.md](imap/01_imap_smtp_overview.md) - IMAP, POP, and SMTP Documentation
- [02_xoauth2_protocol.md](imap/02_xoauth2_protocol.md) - OAuth 2.0 Mechanism for Gmail
- [03_xoauth2_libraries.md](imap/03_xoauth2_libraries.md) - OAuth 2.0 Libraries and Samples
- [04_imap_extensions.md](imap/04_imap_extensions.md) - IMAP Extensions for Gmail

### 10. Postmaster Tools API
- [01_overview.md](postmaster_tools/01_overview.md) - Postmaster Tools API Overview
- [02_quickstart_java.md](postmaster_tools/02_quickstart_java.md) - Java Quickstart
- [03_quickstart_python.md](postmaster_tools/03_quickstart_python.md) - Python Quickstart
- [04_setup.md](postmaster_tools/04_setup.md) - Set up the API
- [05_verify_domain.md](postmaster_tools/05_verify_domain.md) - Verify Authentication Domain
- [06_retrieve_metrics.md](postmaster_tools/06_retrieve_metrics.md) - Retrieve Email Metrics
- [07_troubleshooting.md](postmaster_tools/07_troubleshooting.md) - Troubleshoot Authentication Issues

### 11. AMP for Gmail
- [01_overview.md](amp_for_gmail/01_overview.md) - AMP for Gmail Documentation
- [02_authenticating_requests.md](amp_for_gmail/02_authenticating_requests.md) - Authenticating Requests
- [03_security_requirements.md](amp_for_gmail/03_security_requirements.md) - Security Requirements
- [04_testing.md](amp_for_gmail/04_testing.md) - Test Your AMP Emails
- [05_debugging.md](amp_for_gmail/05_debugging.md) - Debug Dynamic Emails
- [06_register.md](amp_for_gmail/06_register.md) - Register with Google
- [07_supported_platforms.md](amp_for_gmail/07_supported_platforms.md) - Supported Platforms
- [08_tips.md](amp_for_gmail/08_tips.md) - Tips and Known Limitations

### 12. Email Design & Markup
- [01_css_support.md](email_design_markup/01_css_support.md) - CSS Support for Gmail
- [02_markup_overview.md](email_design_markup/02_markup_overview.md) - Gmail Email Markup Overview
- [03_getting_started.md](email_design_markup/03_getting_started.md) - Getting Started with Markup
- [04_actions_overview.md](email_design_markup/04_actions_overview.md) - What Are Actions?
- [05_highlights.md](email_design_markup/05_highlights.md) - What Are Highlights?
- [06_declaring_actions.md](email_design_markup/06_declaring_actions.md) - Declare Actions
- [07_handling_action_requests.md](email_design_markup/07_handling_action_requests.md) - Handle Action Requests
- [08_securing_actions.md](email_design_markup/08_securing_actions.md) - Securing Actions
- [09_limited_use_access_tokens.md](email_design_markup/09_limited_use_access_tokens.md) - Limited Use Access Tokens
- [10_verifying_bearer_tokens.md](email_design_markup/10_verifying_bearer_tokens.md) - Verify Bearer Tokens
- [11_apps_script_tutorial.md](email_design_markup/11_apps_script_tutorial.md) - Apps Script Tutorial
- [12_testing_schemas.md](email_design_markup/12_testing_schemas.md) - Test Your Schemas
- [13_registering_with_google.md](email_design_markup/13_registering_with_google.md) - Register with Google

### 13. Email Promotions
- [01_overview.md](email_promotions/01_overview.md) - Promotions Tab Overview
- [02_annotate_emails.md](email_promotions/02_annotate_emails.md) - Annotate Emails
- [03_best_practices.md](email_promotions/03_best_practices.md) - Best Practices
- [04_troubleshooting.md](email_promotions/04_troubleshooting.md) - Troubleshooting
- [05_faq.md](email_promotions/05_faq.md) - FAQ

### 14. Email Reactions
- [01_format.md](email_reactions/01_format.md) - Email Reactions Format
- [02_examples.md](email_reactions/02_examples.md) - Email Reaction Examples

### 15. Android
- [01_content_provider.md](android/01_content_provider.md) - Android Content Provider for Gmail

### 16. Reference & Support
- [01_rest_api.md](reference/01_rest_api.md) - Gmail API REST Reference
- [02_samples.md](reference/02_samples.md) - Gmail API Samples
- [03_support.md](reference/03_support.md) - Find Support

---

## Directory Structure

```
gmail_api/
├── README.md                    # This index file
├── auth/                        # Authentication & Authorization
├── quickstarts/                 # Language-specific quickstart guides
├── create_send_mail/            # Creating and sending emails
├── manage_mailboxes/            # Managing threads, labels, messages
├── manage_settings/             # Account settings management
├── best_practices/              # Performance and optimization
├── troubleshooting/             # Common issues and solutions
├── migration/                   # Migration from legacy APIs
├── imap/                        # IMAP/POP/SMTP protocols
├── postmaster_tools/            # Email delivery monitoring
├── amp_for_gmail/               # Dynamic email with AMP
├── email_design_markup/         # Schema.org markup for emails
├── email_promotions/            # Promotions tab features
├── email_reactions/             # Emoji reactions in email
├── android/                     # Android-specific integration
└── reference/                   # API reference and support
```

---

## Quick Reference

### Common Scopes

| Scope | Description |
|-------|-------------|
| `gmail.readonly` | Read all resources and metadata |
| `gmail.send` | Send messages only |
| `gmail.compose` | Create, update, delete drafts; send messages |
| `gmail.modify` | All read/write except permanent deletion |
| `gmail.labels` | Create, read, update, delete labels |

### API Endpoints

- **Service Endpoint:** `https://gmail.googleapis.com`
- **Discovery Document:** `https://gmail.googleapis.com/$discovery/rest?version=v1`

### Server Settings

| Service | Server | Port |
|---------|--------|------|
| IMAP | imap.gmail.com | 993 (SSL) |
| POP | pop.gmail.com | 995 (SSL) |
| SMTP | smtp.gmail.com | 465 (SSL) / 587 (TLS) |

---

*Documentation captured from Google Developers documentation. For the most up-to-date information, visit https://developers.google.com/workspace/gmail*
