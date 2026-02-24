> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Monitor sandbox lifecycle events

The lifecycle API provides RESTful endpoints to request the latest sandbox lifecycle events. This allows you to track when sandboxes are created, paused, resumed, updated, or killed, along with metadata.
All requests require authentication using your team [API key](/docs/api-key#where-to-find-api-key).

Query Parameters:

* `offset` (optional): Number of events to skip (default: 0, min: 0)
* `limit` (optional): Number of events to return (default: 10, min: 1, max: 100)
* `orderAsc` (optional): Sort order - true for ascending, false for descending (default: false)

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sbx = await Sandbox.create()

  // Get the latest events for a specific sandbox
  const resp1 = await fetch(
    `https://api.e2b.app/events/sandboxes/${sbx.id}`,
    {
      method: 'GET',
      headers: {
        'X-API-Key': E2B_API_KEY,
      },
    }
  )
  const sandboxEvents = await resp1.json()

  // Get the latest 10 events for all sandboxes associated with the team
  const resp2 = await fetch(
    'https://api.e2b.app/events/sandboxes?limit=10',
    {
      method: 'GET',
      headers: {
        'X-API-Key': E2B_API_KEY,
      },
    }
  )
  const teamSandboxEvents = await resp2.json()

  console.log(teamSandboxEvents)

  // [
  //   {
  //     "version": "v1",
  //     "id": "f5911677-cb60-498f-afed-f68143b3cc59",
  //     "type": "sandbox.lifecycle.killed",
  //     "eventData": null,
  //     "sandboxBuildId": "a979a14b-bdcc-49e6-bc04-1189fc9fe7c2",
  //     "sandboxExecutionId": "1dae9e1c-9957-4ce7-a236-a99d5779aadf",
  //     "sandboxId": "${SANDBOX_ID}",
  //     "sandboxTeamId": "460355b3-4f64-48f9-9a16-4442817f79f5",
  //     "sandboxTemplateId": "rki5dems9wqfm4r03t7g",
  //     "timestamp": "2025-08-06T20:59:36Z"
  //   },
  //   {
  //     "version": "v1",
  //     "id": "30b09e11-9ba2-42db-9cf6-d21f0f43a234",
  //     "type": "sandbox.lifecycle.updated",
  //     "eventData": {
  //       "set_timeout": "2025-08-06T20:59:59Z"
  //     },
  //     "sandboxBuildId": "a979a14b-bdcc-49e6-bc04-1189fc9fe7c2",
  //     "sandboxExecutionId": "1dae9e1c-9957-4ce7-a236-a99d5779aadf",
  //     "sandboxId": "${SANDBOX_ID}",
  //     "sandboxTeamId": "460355b3-4f64-48f9-9a16-4442817f79f5",
  //     "sandboxTemplateId": "rki5dems9wqfm4r03t7g",
  //     "timestamp": "2025-08-06T20:59:29Z"
  //   },
  //   [...]
  //   {
  //     "version": "v1",
  //     "id": "0568572b-a2ac-4e5f-85fa-fae90905f556",
  //     "type": "sandbox.lifecycle.created",
  //     "eventData": null,
  //     "sandboxBuildId": "a979a14b-bdcc-49e6-bc04-1189fc9fe7c2",
  //     "sandboxExecutionId": "1dae9e1c-9957-4ce7-a236-a99d5779aadf",
  //     "sandboxId": "${SANDBOX_ID}",
  //     "sandboxTeamId": "460355b3-4f64-48f9-9a16-4442817f79f5",
  //     "sandboxTemplateId": "rki5dems9wqfm4r03t7g",
  //     "timestamp": "2025-08-06T20:59:24Z"
  //   }
  // ]
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import requests
  from e2b_code_interpreter import Sandbox

  sbx = Sandbox.create()

  # Get the latest events for a specific sandbox
  resp1 = requests.get(
      f"https://api.e2b.app/events/sandboxes/{sbx.sandbox_id}",
      headers={
          "X-API-Key": E2B_API_KEY,
      }
  )
  sandbox_events = resp1.json()

  # Get the latest 10 events for all sandboxes associated with the team
  resp2 = requests.get(
      "https://api.e2b.app/events/sandboxes?limit=10",
      headers={
          "X-API-Key": E2B_API_KEY,
      }
  )
  team_sandbox_events = resp2.json()

  print(team_sandbox_events)

  # [
  #   {
  #     "version": "v1",
  #     "id": "0568572b-a2ac-4e5f-85fa-fae90905f556",
  #     "type": "sandbox.lifecycle.killed",
  #     "eventData": null,
  #     "sandboxBuildId": "a979a14b-bdcc-49e6-bc04-1189fc9fe7c2",
  #     "sandboxExecutionId": "1dae9e1c-9957-4ce7-a236-a99d5779aadf",
  #     "sandboxId": "${SANDBOX_ID}",
  #     "sandboxTeamId": "460355b3-4f64-48f9-9a16-4442817f79f5",
  #     "sandboxTemplateId": "rki5dems9wqfm4r03t7g",
  #     "timestamp": "2025-08-06T20:59:36Z"
  #   },
  #   {
  #     "version": "v1",
  #     "id": "e7013704-2c51-4dd2-9f6c-388c91460149",
  #     "type": "sandbox.lifecycle.updated",
  #     "eventData": {
  #       "set_timeout": "2025-08-06T20:59:59Z"
  #     },
  #     "sandboxBuildId": "a979a14b-bdcc-49e6-bc04-1189fc9fe7c2",
  #     "sandboxExecutionId": "1dae9e1c-9957-4ce7-a236-a99d5779aadf",
  #     "sandboxId": "${SANDBOX_ID}",
  #     "sandboxTeamId": "460355b3-4f64-48f9-9a16-4442817f79f5",
  #     "sandboxTemplateId": "rki5dems9wqfm4r03t7g",
  #     "timestamp": "2025-08-06T20:59:29Z"
  #   },
  #   [...]
  #   {
  #     "version": "v1",
  #     "id": "f29ef778-2743-4c97-a802-7ba67f84ce24",
  #     "type": "sandbox.lifecycle.created",
  #     "eventData": null,
  #     "sandboxBuildId": "a979a14b-bdcc-49e6-bc04-1189fc9fe7c2",
  #     "sandboxExecutionId": "1dae9e1c-9957-4ce7-a236-a99d5779aadf",
  #     "sandboxId": "${SANDBOX_ID}",
  #     "sandboxTeamId": "460355b3-4f64-48f9-9a16-4442817f79f5",
  #     "sandboxTemplateId": "rki5dems9wqfm4r03t7g",
  #     "timestamp": "2025-08-06T20:59:24Z"
  #   }
  # ]
  ```
</CodeGroup>
