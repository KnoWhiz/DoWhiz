# DoWhiz Analytics Dashboard

## Overview

This implementation adds a production-oriented internal analytics system that tracks the DoWhiz journey from first touch to paid state.

- Dashboard route: `/dashboard`
- API endpoints:
  - `POST /analytics/track`
  - `GET /analytics/dashboard`
- Access control: admin-only (`ANALYTICS_ADMIN_EMAILS` allowlist, validated from Supabase JWT email claim)
- Payment/subscription truth: backend/Stripe webhook events (`payment_succeeded`, `subscription_activated`)

## Architecture

### Data model and storage

Analytics events are stored in `analytics_events` (Postgres via existing `AccountStore`):

- Core columns: `event_name`, `event_timestamp`, `source`
- Identity columns: `account_id`, `auth_user_id`, `anonymous_id`, `session_id`, `workspace_id`, `org_id`
- Context columns: UTM/referrer/page/device/env/app version
- Event payload: `properties_json`
- Dedupe key: `(event_name, event_key)` unique when `event_key` is present

Schema bootstrap and indexes are created at service startup (`ensure_analytics_schema`) to avoid extra infra dependencies.

### Identity stitching

Dashboard aggregation resolves identities in this priority:

1. `account_id`
2. `auth_user_id`
3. `anonymous_id`

Anonymous-to-known stitching is inferred when events share `anonymous_id` and later include a known account/user identity.

### Data sources

- Client-side intent + acquisition events: website landing + auth page (`/auth/index.html`)
- Backend product and onboarding events: auth/linking and scheduler execution paths
- Trusted monetization events: billing checkout creation + Stripe webhook fulfillment
- Backfill safety:
  - `signup_completed` and `workspace_created` can be backfilled from `accounts`
  - `payment_succeeded` and `subscription_activated` can be backfilled from `payments`

## Dashboard Structure

`/dashboard` includes:

1. Executive KPI row
2. Ordered funnel (visit -> CTA -> signup -> activation -> checkout -> paid)
3. Acquisition breakdown (UTM/referrer/device/landing variant)
4. Activation breakdown (auth/workspace/channel/task + rates)
5. Monetization summary (intent -> checkout -> paid + plan mix)
6. Retention/cohorts (D1/D7/D30, repeat-value, DAU/WAU/MAU stickiness)
7. Reliability section (task success, error rates, failure reasons, latency rows)
8. Metric definition and taxonomy tables for trust/clarity

## Event Taxonomy

### A. Acquisition / Site

Implemented now:

- `landing_page_view`
- `signup_page_view`
- `primary_cta_click`
- `secondary_cta_click`

Deferred:

- `pricing_page_view`
- `demo_or_waitlist_cta_click`

### B. Signup / Auth

Implemented now:

- `signup_started`
- `signup_completed` (backend)
- `login_started`
- `login_completed`
- `first_authenticated_session`
- `auth_error`

### C. Onboarding / Activation

Implemented now:

- `workspace_created` (backend)
- `channel_connect_started`
- `channel_connect_pending`
- `channel_connect_succeeded`
- `channel_connect_failed`
- `first_channel_or_tool_connected`
- `first_agent_or_workflow_created` (mapped from first task start)

### D. Core Usage

Implemented now:

- `task_started`
- `first_task_started`
- `task_succeeded`
- `first_task_succeeded`
- `second_successful_task`
- `task_failed`

### E. Upgrade / Monetization

Implemented now:

- `upgrade_viewed_or_paywall_seen`
- `upgrade_clicked`
- `checkout_started` (backend)
- `checkout_abandoned`
- `checkout_error`
- `payment_succeeded` (Stripe webhook)
- `subscription_activated` (Stripe webhook fulfillment)

Deferred:

- `subscription_renewed`
- `subscription_failed`
- `subscription_canceled`
- `trial_started`
- `trial_converted`

### F. Retention / Engagement

Deferred:

- `active_day`
- `active_week`
- `session_started`
- `session_ended`

### G. Reliability / Performance

Implemented now:

- `checkout_error`
- `task_failed`

Deferred:

- `api_error`
- `integration_error`
- `webhook_error`
- `latency_metric_logged`

## Must-have Funnel Mapping

Implemented ordered funnel steps:

1. `landing_page_view`
2. `primary_cta_click`
3. `signup_started`
4. `signup_completed`
5. `first_authenticated_session`
6. `workspace_created`
7. `first_channel_or_tool_connected`
8. `first_agent_or_workflow_created`
9. `first_task_started`
10. `first_task_succeeded`
11. `second_successful_task`
12. `upgrade_viewed_or_paywall_seen`
13. `checkout_started`
14. `payment_succeeded`
15. `subscription_activated`

## Metric Definitions

Implemented formulas:

1. Visit-to-signup conversion = `signup_completed identities / landing_page_view identities`
2. Signup-to-activation conversion = `first_task_succeeded identities / signup_completed identities`
3. Activation-to-paid conversion = `payment_succeeded identities / first_task_succeeded identities`
4. Overall visitor-to-paid conversion = `payment_succeeded identities / landing_page_view identities`
5. Activation rate = `first_task_succeeded identities / signup_completed identities`
6. Repeat-value rate = `second_successful_task within 7d of first_task_succeeded / first_task_succeeded identities`
7. Time to first value = `median(signup_completed -> first_task_succeeded) in hours`
8. Checkout abandon rate = `checkout_abandoned / checkout_started` (fallback to `(checkout_started - payment_succeeded) / checkout_started`)
9. Trial-to-paid rate = `payment_succeeded / trial_started` when trial events exist
10. D1 / D7 / D30 retention = usage in day-N window from signup cohort / eligible cohort
11. Task success rate = `task_succeeded / (task_succeeded + task_failed)`
12. Workspace activation rate = workspace cohort reaching first task success (documented in metric definitions)

## Route and SEO Constraints

- `/dashboard` has no public nav entry added on landing page.
- Dashboard page sets `meta[name="robots"] = noindex, nofollow` at runtime.
- Dashboard route is not listed in static sitemap.

## Assumptions and Current Limits

- DoWhiz monetization is currently one-time credit purchase; paid-state metrics map to payment + activated credits.
- Reliability percentages rely on currently implemented error events; adding `api_error` and `latency_metric_logged` will improve precision.
- Cohort and retention logic is identity/event-window based; no dedicated materialized cohort table yet.
- Rust compile verification in this environment requires accepting local Xcode license before full `cargo check` can complete.

## Follow-up Recommendations

1. Emit `api_error`, `integration_error`, and `latency_metric_logged` from high-traffic API paths.
2. Add explicit `trial_started/trial_converted` if trial plans are introduced.
3. Add dashboard export endpoint (CSV) for weekly growth reviews.
4. Add automated tests for `analytics/dashboard` aggregation edge cases and dedupe semantics.
