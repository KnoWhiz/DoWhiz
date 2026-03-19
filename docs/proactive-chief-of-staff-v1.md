# DoWhiz Proactive Chief of Staff V1

- Status: Implemented in V1
- Last updated: 2026-03-19
- Audience: Product, design, frontend, backend
- Scope: V1 product spec for a proactive, low-annoyance recommendation layer

## 1. Summary

DoWhiz should evolve from a mostly reactive task executor into a more proactive operator that helps users answer "what should happen next?" without becoming noisy or controlling.

V1 introduces a lightweight "Chief of Staff" layer that surfaces one high-confidence recommendation at a time. The system should suggest the next best move based on current workspace state, task history, connected tools, and obvious blockers.

The product principle for V1 is:

1. Suggest first.
2. Act only with approval when needed.
3. Use one recommendation at a time.
4. Prefer helpfulness over frequency.

## 2. Problem Statement

Today, DoWhiz works best when the user already knows exactly what they want and can phrase it as a clear request.

Current gaps:

1. After onboarding, the product becomes more reflective than directive.
2. The user must often decide the next step alone.
3. The dashboard "Next Steps" experience is still mostly checklist-oriented and static.
4. The agent does not yet consistently notice stalled momentum, obvious blockers, or strong continuation opportunities.

This creates a product gap:

- Users can ask DoWhiz to do work.
- DoWhiz does not yet reliably help users decide what work matters next.

## 3. Goals

V1 goals:

1. Help users reach first value faster after signup and workspace setup.
2. Reduce "what should I do next?" friction.
3. Increase continuation after successful tasks.
4. Surface blockers before users have to investigate manually.
5. Keep the experience calm, useful, and easy to dismiss.

## 4. Non-Goals

V1 will not:

1. Turn every agent into a proactive chatterbox.
2. Send broad autonomous outbound messages by default.
3. Infer speculative strategy from weak signals.
4. Take sensitive external actions without approval.
5. Solve long-term planning or full autonomous project management in one release.

## 5. Product Concept

V1 introduces a single product behavior:

- A "Chief of Staff" recommendation layer that proposes one next move at a time.

This should be treated as an orchestration behavior, not a separate generic bot persona. The user should experience it as an intelligent, well-timed recommendation system across the dashboard and task completion flows.

Key properties:

1. One recommendation at a time.
2. Backed by concrete product state.
3. Easy to accept, dismiss, or defer.
4. Scoped to the user's current workspace and recent work.

## 6. Current Product Baseline

Relevant current implementation anchors:

1. Conversational founder intake already exists in `website/src/pages/StartupIntakePage.jsx` and `DoWhiz_service/scheduler_module/src/service/startup_workspace/intake_chat.rs`.
2. Workspace recommendations are conceptually modeled in `DoWhiz_service/scheduler_module/src/service/startup_workspace/workspace_home.rs`.
3. Live account-level task summaries already expose `request_summary` in `DoWhiz_service/scheduler_module/src/scheduler/store/mongo.rs`.
4. Dashboard task and workspace UX currently lives in `website/public/auth/index.html`.
5. Provider connection/runtime state already exists via `GET /api/workspace/provider-state` in `DoWhiz_service/scheduler_module/src/service/auth.rs`.
6. Follow-up scheduling hooks already exist in `DoWhiz_service/scheduler_module/src/scheduler/actions.rs`.

The key product observation is that the backend already has meaningful state signals, but the user-facing "what next?" experience is still relatively static.

## 7. Primary User Experience

### 7.1 Dashboard recommendation card

Add a new card at the top of Team Workspace in the unified dashboard.

The card should contain:

1. A single recommended action title.
2. A short "why now" explanation.
3. A short "what this unlocks" explanation.
4. A primary action.
5. A way to see alternatives.
6. A way to dismiss or defer the recommendation.

Recommended controls:

1. `Do it`
2. `Show 2 options`
3. `Not now`
4. `Refresh`
5. `Why am I seeing this?`

Refresh cadence:

1. Auto-refresh at most once per user's local calendar day, on the first dashboard open that day.
2. Do not silently reshuffle the recommendation in the background during routine task polling.
3. Always offer a manual `Refresh` control so the user can explicitly ask for a fresh read.
4. If the user chooses `Not now`, hide the current recommendation for the rest of the local day unless they manually refresh.

### 7.2 Post-task continuation suggestion

After a successful task, append a single optional next-step suggestion in the response.

Examples:

1. "Helpful next step: I can turn this into assigned tasks."
2. "Helpful next step: I can draft the follow-up email."
3. "Helpful next step: I can turn this into a Google Doc brief."

This should be short, non-blocking, and contextual.

### 7.3 Outbound nudges

Outbound proactive nudges should be deferred until after the dashboard card and post-task suggestion prove valuable.

They are explicitly not part of the initial V1 launch scope.

## 8. Trigger Classes

V1 should ship a narrow trigger set with strong confidence and low ambiguity.

### 8.1 Setup blocker

Trigger when the workspace blueprint or user intent clearly implies a required integration or surface, but the integration is not linked.

Examples:

1. GitHub is part of the requested workflow, but no GitHub account is connected.
2. Slack or Discord is expected for coordination, but neither is linked.
3. Formal docs workflow is expected, but Google Docs or equivalent is unavailable.

### 8.2 First-value gap

Trigger when the user has completed meaningful setup steps, but still has no successful task after a defined time window.

Examples:

1. Founder intake complete, but no first task started.
2. Channel linked, but no successful task completed.

### 8.3 Active blocker

Trigger when the user's current work is blocked by a known state.

Examples:

1. Latest task failed.
2. A task is waiting on a required approval.
3. A manual setup step blocks progress.

### 8.4 Continuation opportunity

Trigger immediately after a successful task when there is a strong adjacent action with low ambiguity.

Examples:

1. Meeting summary completed -> suggest converting it into assigned tasks.
2. Draft completed -> suggest sending or sharing it.
3. Research completed -> suggest synthesizing into a brief or plan.

### 8.5 Idle workspace

Trigger when the workspace has enough setup to be useful, but there has been no meaningful activity for a defined idle period.

This should be lower priority than blockers and first-value triggers.

## 9. Priority and Ranking

V1 ranking should remain intentionally simple and interpretable.

Priority order:

1. Active blocker
2. First-value gap
3. Setup blocker
4. Continuation opportunity
5. Idle workspace

Within a priority class, prefer recommendations that:

1. Have the highest confidence.
2. Unlock the most value with the least user effort.
3. Require the fewest assumptions.
4. Avoid sensitive external action.

If two recommendations are close, prefer the one that moves the user toward first successful task completion faster.

## 10. Decision Inputs

V1 recommendations should only rely on high-confidence structured signals.

Initial input families:

1. Workspace blueprint
2. Connected identifiers and provider runtime state
3. Task execution status
4. Request summaries from recent tasks
5. Starter-task and approval-state concepts already modeled in startup workspace planning

Initial signal examples:

1. No GitHub connection but build-system workflow requested.
2. No successful task yet after signup or channel connection.
3. Most recent task failed.
4. Most recent task succeeded and has a known continuation pattern.
5. Workspace appears configured enough for action but has gone idle.

## 11. UX Copy Guidelines

All recommendation copy should follow these rules:

1. Lead with the action, not the diagnosis.
2. Explain why the recommendation is showing now.
3. Explain the expected outcome in plain language.
4. Keep copy calm and concrete.
5. Avoid broad motivational language or fake urgency.

Recommended template:

- Title: imperative action
- Why now: state-based reason
- Outcome: concrete benefit

Example:

- Title: Connect GitHub
- Why now: Your workspace says code delivery matters, but no repository is linked yet.
- Outcome: This unlocks implementation and review workflows for future tasks.

## 12. Guardrails

Guardrails are a core part of the product, not an afterthought.

### 12.1 Recommendation volume

1. Show only one proactive recommendation at a time.
2. Show only one continuation suggestion after a completed task.
3. Do not send the same recommendation across multiple channels.

### 12.2 Approval posture

1. Suggest before acting.
2. Keep external or sensitive actions approval-based.
3. Prefer draft generation over direct sending when ambiguity exists.

### 12.3 Confidence threshold

1. Do not recommend actions based on weak inference.
2. Use only recommendation classes with strong state evidence in V1.
3. If the system cannot explain why a recommendation is showing, it should not show it.

### 12.4 User control

1. Every proactive recommendation must support `Not now`.
2. The user should be able to reduce proactivity level in settings.
3. The user should be able to dismiss repeated recommendations.

### 12.5 Cooldown and repetition

1. Do not repeat the same recommendation for 7 days unless the underlying state changes.
2. If the user dismisses two proactive suggestions in a row, automatically downgrade them to a more conservative proactivity mode until new meaningful state change occurs.
3. Routine dashboard polling should not trigger a fresh recommendation fetch; recommendation cadence should feel like a daily brief, not a live ticker.

## 13. Proactivity Settings

V1 should support a small, explicit user preference model.

Recommended settings:

1. `Off`
2. `Minimal`
3. `Helpful`
4. `Hands-on`

Recommended behavior:

1. `Off`: no proactive suggestions except hard blockers inside the product UI.
2. `Minimal`: dashboard recommendation card only.
3. `Helpful`: dashboard card plus post-task continuation suggestion.
4. `Hands-on`: reserved for later expansion, not necessarily fully implemented in V1.

## 14. Annoyance Budget

V1 should enforce a product-level annoyance budget.

Initial limits:

1. Maximum one proactive dashboard card visible at a time.
2. Maximum one post-task suggestion per successful task.
3. Maximum one outbound proactive nudge per workspace every 72 hours after outbound nudges are eventually enabled.
4. No outbound proactive nudges for new users before first value.

## 15. Rollout Plan

### Phase 1

Ship the dashboard recommendation card only.

Goal:

1. Validate whether users engage with state-driven next-step recommendations.
2. Measure accept, dismiss, and ignore behavior safely.

### Phase 2

Add post-task continuation suggestions.

Goal:

1. Increase continuation after successful tasks.
2. Capture higher-intent follow-on behavior in context.

### Phase 3

Evaluate outbound nudges only after earlier phases prove useful and low-annoyance.

## 16. Metrics

Primary success metrics:

1. Recommendation accept rate
2. Recommendation dismiss rate
3. Time to first successful task
4. First-task success rate after intake
5. Continuation rate after successful tasks
6. Reactivation rate for idle workspaces
7. Proactivity opt-down rate

Diagnostic metrics:

1. Trigger type frequency
2. Recommendation type acceptance by trigger class
3. Repeat recommendation suppression rate
4. Recommendation-to-task-start conversion
5. Recommendation-to-task-success conversion

## 17. Initial Implementation Notes

Suggested first technical step:

1. Create a recommendation API that consolidates workspace blueprint state, provider state, recent account tasks, and existing workspace planning logic into a single recommendation payload for the dashboard.

Suggested first UI step:

1. Replace the static-feeling "Next Steps" block in `website/public/auth/index.html` with a recommendation-driven component model while preserving manual setup visibility elsewhere on the page.

Suggested first analytics step:

1. Track recommendation shown, accepted, dismissed, deferred, and ignored events.

## 18. Open Questions

These should be resolved before full implementation:

1. Should continuation suggestions be hardcoded by task type at first, or inferred from recent `request_summary` patterns?
2. Which settings surface should own proactivity controls: Team Workspace, Settings, or both?
3. Should approval-related recommendations be handled by the same recommendation layer or a distinct approval inbox pattern?

## 19. Decision

Proceed with a narrow V1 that focuses on:

1. One recommendation at a time
2. High-confidence state-driven triggers
3. Dashboard-first rollout
4. Strong cooldown and dismissal controls

The core design thesis is:

DoWhiz should not become more talkative first. It should become more situationally aware first.
