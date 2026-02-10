# Notion workspace access and process

This doc describes how to request access to the DoWhiz Notion workspace and the
standard onboarding and maintenance steps. It intentionally avoids listing
private URLs or contacts; admins can update it with specifics.

## When you need access
- Contributors who need to read or modify product, ops, or process docs.
- Contractors or partners who need access to a limited subset of pages.

## Request access
1) Open a GitHub issue using the "Notion workspace access request" template (or
   create one manually).
2) Provide your full name, work email, role/team, access level, specific
   teamspaces/pages, reason for access, and desired start/end dates.
3) Admins confirm scope and send a Notion invite to the requested email.

## Accept and verify
- Accept the invite from Notion.
- Verify you can access the requested teamspaces/pages.
- Set your profile details and enable 2FA if required by workspace policy.

## Access hygiene
- Use the least-privilege level; prefer guest access for external collaborators.
- Share only the pages or teamspaces needed for the task.
- For temporary access, include an end date and remove access when complete.
- Do not share pages outside the workspace without approval.

## Offboarding or access changes
- Open a GitHub issue with the updated scope (downgrade or remove access).
- Admins update access and confirm completion.

## Optional: integration access (automation)
- Request a dedicated internal integration for API access.
- Store tokens in a secret manager; never commit tokens to git.
- Share only the specific pages/databases the integration requires.
