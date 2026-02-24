> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Sandbox lifecycle webhooks

Webhooks provide a way for notifications to be delivered to an external web server whenever certain sandbox lifecycle events occur.
This allows you to receive real-time updates about sandbox creation, updates, and termination without having to poll the API.
All webhook requests require authentication using your team [API key](/docs/api-key#where-to-find-api-key).

## Webhook management

### Register webhook

Register a new webhook to receive sandbox lifecycle events.
The webhook will be registered for the team ID associated with your API key.
All events specified during webhook creation will be sent to URL provided during registration with [following payload](#webhook-payload).

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Register a new webhook
  const resp = await fetch(
    'https://api.e2b.app/events/webhooks',
    {
      method: 'POST',
      headers: {
        'X-API-Key': E2B_API_KEY,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        name: 'My Sandbox Webhook',
        url: 'https://your-webhook-endpoint.com/webhook',
        enabled: true,
        events: ['sandbox.lifecycle.created', 'sandbox.lifecycle.updated', 'sandbox.lifecycle.killed'],
        signatureSecret: 'secret-for-event-signature-verification'
      }),
    }
  )

  if (resp.status === 201) {
    console.log('Webhook registered successfully')
  }
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import requests

  # Register a new webhook
  resp = requests.post(
      "https://api.e2b.app/events/webhooks",
      headers={
          "X-API-Key": E2B_API_KEY,
          "Content-Type": "application/json",
      },
      json={
          "name": "My Sandbox Webhook",
          "url": "https://your-webhook-endpoint.com/webhook",
          "enabled": true,
          "events": ["sandbox.lifecycle.created", "sandbox.lifecycle.updated", "sandbox.lifecycle.killed"],
          "signatureSecret": "secret-for-event-signature-verification"
      }
  )

  if resp.status_code == 201:
      print("Webhook registered successfully")
  ```
</CodeGroup>

### List webhooks

List all registered webhooks for your team.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // List webhooks
  const resp = await fetch(
    'https://api.e2b.app/events/webhooks',
    {
      method: 'GET',
      headers: {
        'X-API-Key': E2B_API_KEY
      },
    },
  )

  if (resp.status === 200) {
    console.log('Webhooks listed successfully')
    console.log(await resp.json())
  }
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import requests

  # List webhooks
  resp = requests.get(
      "https://api.e2b.app/events/webhooks",
      headers={
          "X-API-Key": E2B_API_KEY
      },
  )

  if resp.status_code == 200:
      print("Webhooks listed successfully")
      print(response.json())
  ```
</CodeGroup>

### Get webhook configuration

Retrieve the current webhook configuration for your team.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Get webhook configuration
  const resp = await fetch(
    `https://api.e2b.app/events/webhooks/${webhookID}`,
    {
      method: 'GET',
      headers: {
        'X-API-Key': E2B_API_KEY,
      },
    }
  )
  const webhookConfig = await resp.json()
  console.log(webhookConfig)
  // {
  //   "id": "<webhook-id>",
  //   "teamID": "<your-team-id>",
  //   "name": "My Sandbox Webhook",
  //   "createdAt": "2025-08-06T21:00:00Z",
  //   "enabled": true,
  //   "url": "https://your-webhook-endpoint.com/webhook",
  //   "events": ["sandbox.lifecycle.created", "sandbox.lifecycle.killed"]
  // }

  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import requests

  # Get webhook configuration
  resp = requests.get(
      "https://api.e2b.app/events/webhooks/{webhookID}",
      headers={
          "X-API-Key": E2B_API_KEY,
      }
  )

  webhook_config = resp.json()
  print(webhook_config)
  # {
  #   "id": "<webhook-id>",
  #   "teamID": "<your-team-id>",
  #   "name": "My Sandbox Webhook",
  #   "createdAt": "2025-08-06T21:00:00Z",
  #   "enabled": true,
  #   "url": "https://your-webhook-endpoint.com/webhook",
  #   "events": ["sandbox.lifecycle.created", "sandbox.lifecycle.killed"]
  # }
  ```
</CodeGroup>

### Update webhook configuration

Update an existing webhook configuration. The update will replace the previous configuration fields with provided fields.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Update webhook configuration
  const resp = await fetch(
    `https://api.e2b.app/events/webhooks/${webhookID}`,
    {
      method: 'PATCH',
      headers: {
        'X-API-Key': E2B_API_KEY,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        url: 'https://your-updated-webhook-endpoint.com/webhook',
        enabled: false,
        events: ['sandbox.lifecycle.created']
      }),
    }
  )

  if (resp.status === 200) {
    console.log('Webhook updated successfully')
  }
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import requests

  # Update webhook configuration
  resp = requests.patch(
      "https://api.e2b.app/events/webhooks/{webhookID}",
      headers={
          "X-API-Key": E2B_API_KEY,
          "Content-Type": "application/json",
      },
      json={
          "url": "https://your-updated-webhook-endpoint.com/webhook",
          "enabled": False,
          "events": ["sandbox.lifecycle.created"]
      }
  )

  if resp.status_code == 200:
      print("Webhook updated successfully")
  ```
</CodeGroup>

### Delete webhook

Unregister the webhook.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Delete webhook configuration
  const resp = await fetch(
    `https://api.e2b.app/events/webhooks/${webhookID}`,
    {
      method: 'DELETE',
      headers: {
        'X-API-Key': E2B_API_KEY,
      },
    }
  )

  if (resp.status === 200) {
    console.log('Webhook deleted successfully')
  }
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import requests

  # Delete webhook configuration
  resp = requests.delete(
      "https://api.e2b.app/events/webhooks/{webhookID}",
      headers={
          "X-API-Key": E2B_API_KEY,
      }
  )

  if resp.status_code == 200:
      print("Webhook deleted successfully")
  ```
</CodeGroup>

## Webhook payload

When a webhook is triggered, your endpoint will receive a POST request with a JSON payload containing the sandbox event data.
The payload structure matches the event format from the API:

```json  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
{
  "version": "v1",
  "id": "<event-id>",
  "type": "<sandbox-event-type>",
  "eventData": {
    "sandbox_metadata": {
      "<custom-key>": "<custom-value>"
    }
  },
  "sandboxBuildId": "<template-build-id>",
  "sandboxExecutionId": "<sandbox-unique-execution-id>",
  "sandboxId": "<your-sandbox-id>",
  "sandboxTeamId": "<your-team-id>",
  "sandboxTemplateId": "<your-template-id>",
  "timestamp": "2025-08-06T20:59:24Z"
}
```

# Webhook verification

To ensure the authenticity of webhook requests, each request includes a signature in the `e2b-signature` header.
You can verify the signature using the signature secret you provided when registering the webhook.
This confirms that the request originated from E2B and has not been tampered with.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  function verifyWebhookSignature(secret : string, payload : string, payloadSignature : string) {
      const expectedSignatureRaw = crypto.createHash('sha256').update(secret + payload).digest('base64');
      const expectedSignature = expectedSignatureRaw.replace(/=+$/, '');
      return  expectedSignature == payloadSignature
  }

  const payloadValid = verifyWebhookSignature(secret, webhookBodyRaw, webhookSignatureHeader)
  if (payloadValid) {
      console.log("Payload signature is valid")
  } else {
      console.log("Payload signature is INVALID")
  }
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import hashlib
  import base64

  def verify_webhook_signature(secret: str, payload: str, payload_signature: str) -> bool:
      hash_bytes = hashlib.sha256((secret + payload).encode('utf-8')).digest()
      expected_signature = base64.b64encode(hash_bytes).decode('utf-8')
      expected_signature = expected_signature.rstrip('=')

      return expected_signature == payload_signature

  if verify_webhook_signature(secret, webhook_body_raw, webhook_signature_header):
      print("Payload signature is valid")
  else:
      print("Payload signature is INVALID")
  ```

  ```go Golang theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import (
      "crypto/sha256"
      "encoding/base64"
      "fmt"
      "strings"
  )

  func verifyWebhookSignature(secret, payload, payloadSignature string) bool {
      hash := sha256.Sum256([]byte(secret + payload))
      expectedSignature := base64.StdEncoding.EncodeToString(hash[:])

      expectedSignature = strings.TrimRight(expectedSignature, "=")

      return expectedSignature == payloadSignature
  }

  if verifyWebhookSignature(secret, webhookBodyString, webhookSignatureHeaderString) {
      fmt.Println("Payload signature is valid")
  } else {
      fmt.Println("Payload signature is INVALID")
  }
  ```
</CodeGroup>

## Webhook request headers

When webhooks is send, we are adding headers to help you verify the authenticity of the request and make debugging easier.

* `e2b-webhook-id` - Webhook ID that triggered the event
* `e2b-delivery-id` - Unique ID for the delivery attempt
* `e2b-signature-version` - Currently always `v1`, reserved for future use
* `e2b-signature` - Signature for verifying the request authenticity\`

## Available event types

The following event types can be subscribed to via webhooks, they are used as the `type` field in the [payload](#webhook-payload).

* `sandbox.lifecycle.created` - Sandbox creation
* `sandbox.lifecycle.killed` - Sandbox termination
* `sandbox.lifecycle.updated` - Sandbox configuration updates
* `sandbox.lifecycle.paused` - Sandbox pausing
* `sandbox.lifecycle.resumed` - Sandbox resuming
