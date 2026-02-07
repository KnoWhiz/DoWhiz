# Using webhooks

- Source URL: https://developers.zoom.us/docs/api/webhooks/
- Snapshot path: docs/api/webhooks
- Generated (UTC): 2026-02-07T22:40:11.975157+00:00

## Frontmatter

```json
{
  "title": "Using webhooks",
  "resources": [
    {
      "title": "Zoom Platform 101 Zoom Webhooks",
      "url": "https://youtu.be/oAnr1ezW8dY?si=AVlihxis-DIS4L5W"
    },
    {
      "title": "Server-to-Server apps",
      "url": "https://developers.zoom.us/docs/internal-apps/"
    },
    {
      "title": "General OAuth apps",
      "url": "https://developers.zoom.us/docs/integrations/"
    },
    {
      "title": "Build platform apps",
      "url": "https://developers.zoom.us/docs/build/"
    },
    {
      "title": "Updating an app",
      "url": "https://developers.zoom.us/docs/distribute/published-apps/updating/"
    }
  ]
}
```

## Content

```md

# Using webhooks

Instead of making repeated calls to pull data frequently from the Zoom API, you can use webhooks for Zoom to send events to your server. Zoom sends webhook events as HTTP POST requests with a JSON body to your app's specified endpoint. 

For an introduction and explanation of webhooks, see [Zoom Platform 101: Zoom Webhooks](https://youtu.be/oAnr1ezW8dY?si=AVlihxis-DIS4L5W).  

<Alert>

**Now available**

[Zoom Rivet](/docs/rivet/javascript/get-started/#initialize-the-client), Zoom's API library that handles webhooks for you.

</Alert>

## Workplace

<div class="api_ref_tile_container">
<Link class="api_ref_tile" href="/docs/api/meetings/events">
  <SvgIcon svgname="_meetings" />
  <h3>Meetings</h3>
</Link>
<Link className="api_ref_tile" href="/docs/api/rtms/events">
    <SvgIcon svgname="_rtms" />
    <h3>RTMS</h3>
  </Link>
<Link class="api_ref_tile" href="/docs/api/team-chat/events">
  <SvgIcon svgname="_chat" />
  <h3>Team Chat</h3>
</Link>
<Link class="api_ref_tile" href="/docs/api/phone/events">
  <SvgIcon svgname="_zoomphone" />
  <h3>Phone</h3>
</Link>
<Link class="api_ref_tile" href="/docs/api/mail/events">
  <SvgIcon svgname="_mail" />
  <h3>Mail</h3>
</Link>
<Link class="api_ref_tile" href="/docs/api/calendar/events">
  <SvgIcon svgname="_calendar" />
  <h3>Calendar</h3>
</Link>
<Link class="api_ref_tile" href="/docs/api/rooms/events">
  <SvgIcon svgname="_zoomrooms" />
  <h3>Rooms</h3>
</Link>
<Link class="api_ref_tile" href="/docs/api/whiteboard/events">
  <SvgIcon svgname="_whiteboard" />
  <h3>Whiteboard</h3>
</Link>
<Link class="api_ref_tile" href="/docs/api/chatbot/events">
  <SvgIcon svgname="_chat" />
  <h3>Chatbot</h3>
</Link>
<Link class="api_ref_tile" href="/docs/api/scheduler/events">
  <SvgIcon svgname="_zoomscheduler" />
  <h3>Scheduler</h3>
</Link>

</div>

## Business services

<div class="api_ref_tile_container">
  <Link class="api_ref_tile" href="/docs/api/contact-center/events">
    <SvgIcon svgname="contactCenter" />
    <h3>Contact Center</h3>
  </Link>
  <Link class="api_ref_tile" href="/docs/api/events/events">
    <SvgIcon svgname="_zoomevents" />
    <h3>Events</h3>
  </Link>
  <Link class="api_ref_tile" href="/docs/api/iq/events">
    <SvgIcon svgname="zra" />
    <h3>Revenue Accelerator</h3>
  </Link>
  <Link class="api_ref_tile" href="/docs/api/number-management/events">
  <SvgIcon svgname="number-management" />
  <h3>Number Management</h3>
</Link>
      <Link class="api_ref_tile" href="/docs/api/node/events">
  <SvgIcon svgname="zoom-node" />
    <h3>Node</h3>
  </Link>
        <Link class="api_ref_tile" href="/docs/api/quality-management/events">
  <SvgIcon svgname="qualityManagement" />
    <h3>Quality Management</h3>
  </Link>
  <Link className="api_ref_tile" href="/docs/api/healthcare/events">
    <SvgIcon svgname="clinical-notes" />
    <h3>Healthcare</h3>
  </Link>
  <Link className="api_ref_tile" href="/docs/api/video-management/events">
    <SvgIcon svgname="video-management" />
    <h3>Video Management</h3>
  </Link>
</div>

## Accounts

<div class="api_ref_tile_container">
  <Link class="api_ref_tile" href="/docs/api/users/events">
    <SvgIcon svgname="users" />
    <h3>Users</h3>
  </Link>
  <Link class="api_ref_tile" href="/docs/api/accounts/events">
    <SvgIcon svgname="accounts" />
    <h3>Accounts</h3>
  </Link>
  <Link class="api_ref_tile" href="/docs/api/qss/events">
    <SvgIcon svgname="qss" />
    <h3>QSS</h3>
  </Link>
</div>

## Build platform

<div class="api_ref_tile_container">
  <Link class="api_ref_tile" href="/docs/api/video-sdk/events">
    <SvgIcon svgname="_video-sdks" />
    <h3>Video SDK</h3>
  </Link>
    <Link class="api_ref_tile" href="/docs/api/cobrowse-sdk/events">
    <SvgIcon svgname="cobrowse-sdk" />
    <h3>Cobrowse SDK</h3>
  </Link>
</div>

## Marketplace

<div class="api_ref_tile_container">
  <Link class="api_ref_tile" href="/docs/api/marketplace/events">
    <SvgIcon svgname="marketplace" />
    <h3>Apps</h3>
  </Link>
</div>

## Configure webhooks via build flow  
You configure webhooks by subscribing to specific events in the app build flow. 

For instructions to configure your webhook subscriptions using Zoom APIs, see [Configure webhooks via Zoom API](/#Configure-webhooks-via-Zoom-API).    

1. Go to your app's **Access** page.   
  a. Login to the [Zoom App Marketplace](https://marketplace.zoom.us/) and click **Manage**.    
  b. On the **Created Apps** page, open your app, and go to the **Features** → **Access**.   
2. Under the **General Features** section, enable **Event Subscriptions** and **Add New Event Subscription**.  

    <Image src="/img/add-new-subscription2.png" width={500} /> 

    NOTE: You can subscribe to as many events as needed for each event subscription, up to a maximum of 20 event subscriptions per app. Event subscriptions can also have duplicate events. For example, one event subscription could have *Meetings* and *User Events*, and a second event subscription can have *Meetings* and *Recordings* events.   
3. On the **Event Types** screen, select the specific events and then **Done**.  
    <Image src="/img/add-events.png" width={500} />  
4. Set your **Event notification endpoint URL** for each event subscription in your app and **Save**. Be sure that it satisfies the [webhook endpoint requirements](#webhook-endpoint-url-requirements).  
     <Image src="/img/save-subscription.png" width={500} />  


### Webhook endpoint URL requirements

To receive webhooks in development and production environments, the **Event notification endpoint URL** that you specify for each event subscription must:

- Be a publicly accessible `https` endpoint url that supports TLSv1.2+ with a valid certificate chain issued by a [Certificate Authority (CA)](https://en.wikipedia.org/wiki/Certificate_authority).
- Be a fully qualified domain name (FQDN).
- Be able to accept HTTP POST requests containing JSON payloads.
- Be able to respond with a `200` or `204` HTTP Status Code.

### Webhook structure

Webhooks events are POST requests sent to your endpoint URL and contain request headers and a request body of the event type, event timestamp, and event details within the payload object.

**Example request headers:**

```json
{
  "host": "example.com",
  "user-agent": "Zoom Marketplace/1.0a",
  "content-length": "110",
  "authorization": "Wk9PTV9BVVRIT1JJWkFUSU9O",
  "content-type": "application/json; charset=utf-8",
  "traceparent": "Wk9PTV9UUkFDRVBBUkVOVA",
  "x-forwarded-for": "{X_FORWARDED_FOR}",
  "x-forwarded-host": "example.com",
  "x-forwarded-proto": "https",
  "x-zm-request-timestamp": "1739923528",
  "x-zm-signature": "v0=WF9aT09NX1NJR05BVFVSRQ",
  "x-zm-request-id": "6009d653_d487_445d_8406_42b654974899",
  "accept-encoding": "gzip"
}
```

**Example request body:**

```json
{
  "event": "meeting.started",
  "event_ts": 1626230691572,
  "payload": {
    "account_id": "Wk9PTV9BQ0NPVU5UX0lE",
    "object": {
      "id": "1234567890",
      "uuid": "Wk9PTV9NRUVUSU5HX0lE",
      "host_id": "Wk9PTV9VU0VSX0lE",
      "topic": "My Meeting",
      "type": 8,
      "start_time": "2021-07-13T21:44:51Z",
      "timezone": "America/Los_Angeles",
      "duration": 60
    }
  }
}
```

### Zoom IP addresses

We **strongly** recommend that you [Verify webhook events](#verify-webhook-events) instead of creating an allow list of Zoom IP addresses because Zoom may update the [IP ranges](https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0060548) used at any time.

## Verify webhook events

Zoom offers two ways to verify the authenticity of a webhook, to ensure developers that the request came from Zoom:  


<Tabs>
  <Tab eventKey="verifyWithZoomsHeader" title="Verify with Zoom's header">


### Verify with Zoom's header

Zoom includes a webhook secret token on the **Add Feature** page for your app. Zoom uses the value of the secret token to hash the webhook data, which it sends in the `x-zm-signature` webhook request header.

To verify that Zoom sent a webhook request, use the webhook secret token and the webhook request body to create a signature to compare with the `x-zm-signature` header value sent by Zoom.

**Example request headers:**  

```json  
{
  "host": "example.com",
  "user-agent": "Zoom Marketplace/1.0a",
  "content-length": "110",
  "authorization": "Wk9PTV9BVVRIT1JJWkFUSU9O",
  "content-type": "application/json; charset=utf-8",
  "traceparent": "Wk9PTV9UUkFDRVBBUkVOVA",
  "x-forwarded-for": "{X_FORWARDED_FOR}",
  "x-forwarded-host": "example.com",
  "x-forwarded-proto": "https",
  "x-zm-request-timestamp": "1739923528",
  "x-zm-signature": "v0=WF9aT09NX1NJR05BVFVSRQ",
  "accept-encoding": "gzip"
}
```

**Create your signature string** to compare with the Zoom webhook `x-zm-signature` header value:

1. Receive the webhook request.  

   The webhook request body includes the following properties:

   Key | Value Type | Value Description
   --- | --- | ---
   `event` | string |The type of the webhook event
   `payload` | object | Contains the event webhook details
   `event_ts` | number | The timestamp of the webhook event.

   **Example request body:**  

   ```json
   {
     "event": "session.started",
     "payload": {
       "account_id": "{ACCOUNT_ID}",
       "object": {
         "start_time": "2022-07-27T16:56:34Z",
         "id":"{SESSION_ID}"
       }
     },
     "event_ts": 1658940994914
   }
   ```

2. Construct the message.  

   After receiving a webhook request, construct the message string with `"v0"`, the webhook request header `x-zm-request-timestamp` value, and the webhook request body. Separate each section with a colon `:`. For example:

   `v0:{WEBHOOK_REQUEST_HEADER_X-ZM-REQUEST-TIMESTAMP_VALUE}:{WEBHOOK_REQUEST_BODY}`

   Example string with printed values:

   ```
   v0:1658940994:{"event":"session.started","payload":{"account_id":"{ACCOUNT_ID}","object":{"start_time":"2022-07-27T16:56:34Z","id":"{SESSION_ID}"}},"event_ts":1658940994914}
   ```

3. Hash the message.  
  
   Once you have constructed the message string, create a [HMAC SHA-256](https://datatracker.ietf.org/doc/html/rfc4634) hash. Set your webhook's secret token as the secret/salt, and the `message` you constructed as the string to hash. Output in `hex` format.

4. Create the signature.  
  
   Then, create the signature by prepending `v0=` to the hashed message. For example:

   `v0={HASHED_MESSAGE}`

   Example string with printed values:

   ```
   v0=a05d830fa017433bc47887f835a00b9ff33d3882f22f63a2986a8es270341
   ```

5. Compare the signature.  
  
   Compare the `signature` you created with the Zoom webhook request header `x-zm-signature` value. If it matches, the webhook request came from Zoom.

   **Here is an example in Node.js:**

   ```js
   const crypto = require('crypto')

   const message = `v0:${request.headers['x-zm-request-timestamp']}:${JSON.stringify(request.body)}`

   const hashForVerify = crypto.createHmac('sha256', ZOOM_WEBHOOK_SECRET_TOKEN).update(message).digest('hex')

   const signature = `v0=${hashForVerify}`

   if (request.headers['x-zm-signature'] === signature) {
     // Webhook request came from Zoom
   } else {
     // Webhook request did not come from Zoom
   }
   ```

   See the [Webhook sample app for an example implementation](https://github.com/zoom/webhook-sample-node.js/blob/master/index.js#L31).

Learn more about [HMAC Auth for webhooks](https://webhooks.fyi/security/hmac).

  </Tab>
  <Tab eventKey="verifyWithYourHeader" title="Verify with your own header">


### Verify with your own header

Zoom offers the following options to verify webhooks with your own header:

- [Basic Authentication](#basic-authentication)
- [Token Authentication (OAuth)](#token-authentication-oauth)  
- [Custom Header](#custom-header)

#### Basic authentication

1. Choose **Basic Authentication**. Specify a username and password.

   <Image src="/img/webhooks_basic_auth.jpg" alt="Zoom webhook basic auth" width={500} />

2. When a webhook is triggered, Zoom makes a POST request with the following request headers, including the `authorization` value being your username and password, base64encoded with a colon `:` in between:

   ```json
   {
     "host": "example.com",
     "user-agent": "Zoom Marketplace/1.0a",
     "content-length": "320",
     "authorization": "Basic Q2xpZW50SUQ6Q2xpZW50U2VjcmV0",
     "clientid": "{CLIENT_ID}",
     "content-type": "application/json; charset-utf-8",
     "x-forwarded-for": "{X_FORWARDED_FOR}",
     "x-forwarded-proto": "https",
     "x-zm-request-timestamp": "1674780079",
     "x-zm-signature": "v0={HASHED_WEBHOOK_SECRET_TOKEN}",
     "x-zm-trackingid": "{X_ZM_TRACKINGID}",
     "accept-encoding": "gzip"
   }
   ```

Learn more about [Basic Auth for webhooks](https://webhooks.fyi/security/shared-secret).

#### Token authentication (OAuth)  

1. Choose the "Token Authentication" option. Specify an OAuth token endpoint, and your OAuth server's Client ID and Client Secret.

   `https://example.com/oauth/token?grant_type=client_credentials`

   <Image src="/img/webhooks_token_auth.jpg" alt="Zoom webhook oauth" width={500} />

2. When a webhook is triggered, Zoom makes a POST request to retrieve an access_token from your OAuth server, with the following request headers, including the  `authorization` value being your Client ID and Client Secret, base64encoded, with a colon `:` in between:

   ```json
   {
     "host": "example.com",
     "user-agent": "Zoom Marketplace/1.0a",
     "content-length": "0",
     "authorization": "Basic Q2xpZW50SUQ6Q2xpZW50U2VjcmV0",
     "content-type": "application/x-www-form-urlencoded",
     "x-forwarded-for": "{X_FORWARDED_FOR}",
     "x-forwarded-proto": "https",
     "accept-encoding": "gzip"
   }
   ```

3. After you validate the authorization header, respond to the request with a `200` status code and the following JSON. **expires_in** must be equal to or greater than 3599.

   ```json
   {
     "access_token": "{ACCESS_TOKEN}",
     "token_type": "bearer",
     "expires_in": "3599"
   }
   ```

4. Zoom sends the webhook request to your server with the authorization bearer header set to the value of your access_token:

   ```json
   {
     "host": "example.com",
     "user-agent": "Zoom Marketplace/1.0a",
     "content-length": "110",
     "authorization": "Bearer {ACCESS_TOKEN}",
     "content-type": "application/json; charset=utf-8",
     "x-forwarded-for": "{X_FORWARDED_FOR}",
     "x-forwarded-proto": "https",
     "x-zm-request-timestamp": "1674847087",
     "x-zm-signature": "v0={HASHED_WEBHOOK_SECRET_TOKEN}",
     "accept-encoding": "gzip"
   }
   ```

Learn more about [OAuth for webhooks](https://webhooks.fyi/security/jwt-jwk-oauth2).

#### Custom header

1. Choose **Custom Header**. Specify a key and value.

   <Image src="/img/webhook-custom-header.png" alt="Zoom webhook custom header" width={500} />

2. When an event triggers a webhook, Zoom makes a POST request with the following request headers, including your custom header property (in this example `x-my-custom-key`):

   ```json
   {
     "host": "example.com",
     "user-agent": "Zoom Marketplace/1.0a",
     "content-length": "110",
     "content-type": "application/json; charset=utf-8",
     "x-forwarded-for": "{X_FORWARDED_FOR}",
     "x-forwarded-proto": "https",
     "x-my-custom-key": "my-custom-value",
     "x-zm-request-timestamp": "1674847087",
     "x-zm-signature": "v0={HASHED_WEBHOOK_SECRET_TOKEN}",
     "accept-encoding": "gzip"
   }
   ```

   <Alert>

   **Removing the x- Prefix in custom header keys**

   If you want to remove the `x-` prefix in the custom header key, please reach out to [Zoom Support](https://support.zoom.us) or post on our [Developer Forum](https://devforum.zoom.us/).
   
   </Alert>

Learn more about [One Time Verification for webhooks](https://webhooks.fyi/security/one-time-verification-challenge).

  </Tab>
</Tabs>

## Validate your webhook endpoint

Zoom requires that you manually trigger webhook validation when you add a new webhook or make changes to an existing one. Subsequently, Zoom  automatically revalidates webhooks every 72 hours.

Zoom uses a challenge-response check (CRC) for webhook validation. When a CRC occurs, Zoom makes a POST request to your endpoint with a challenge request body. After your endpoint receives the request, your app needs to respond with the challenge response within 3 seconds.

To trigger the initial CRC validation, click **Validate** under the **Event Notification Endpoint URL** on the **Feature** page for your app. See [Revalidation](#revalidation) for revalidation details.

<Image src="/img/webhook-validate.png" alt="Zoom webhook validate" width={500}/>

<Alert>

**Note**

You won't be able to save your changes until after Zoom validates your endpoint.

</Alert>

### Implement the challenge-response check flow

1. Receive the challenge (webhook request body).

   The webhook request body includes the following properties:

   Key | Value Type | Value Description
   --- | --- | ---
   `event` | string |The type of the webhook event, which will be `"endpoint.url_validation"`
   `payload` | object | Contains a property with the `plainToken` value, the string to hash.
   `event_ts` | number | The timestamp of the webhook event.

  
   **Example request headers:**  
   ```json  
   {
     "host": "example.com",
     "user-agent": "Zoom Marketplace/1.0a",
     "content-length": "110",
     "authorization": "Wk9PTV9BVVRIT1JJWkFUSU9O",
     "content-type": "application/json; charset=utf-8",
     "traceparent": "Wk9PTV9UUkFDRVBBUkVOVA",
     "x-forwarded-for": "{X_FORWARDED_FOR}",
     "x-forwarded-host": "example.com",
     "x-forwarded-proto": "https",
     "x-zm-request-timestamp": "1739923528",
     "x-zm-signature": "v0=WF9aT09NX1NJR05BVFVSRQ",
     "accept-encoding": "gzip"
   }
   ```  
   
   **Example request body:**  

   ```json
   {
     "payload": {
       "plainToken": "qgg8vlvZRS6UYooatFL8Aw"
     },
     "event_ts": 1654503849680,
     "event": "endpoint.url_validation"
   }
   ```

2. Hash the plainToken.

   Once you receive the request body, create a [HMAC SHA-256](https://datatracker.ietf.org/doc/html/rfc4634) hash. Set your webhook's secret token as the secret (salt), and the `plainToken` value as the string to hash. Output in `hex` format.

3. Create the response JSON object.
  
   Create a JSON object with a key of `"plainToken"` with a value of the `plainToken` from the request body, and a key of `"encryptedToken"`  with a value of the hashed `plainToken`. For example:

   ```json
   {
     "plainToken": "qgg8vlvZRS6UYooatFL8Aw",
     "encryptedToken": "23a89b634c017e5364a1c8d9c8ea909b60dd5599e2bb04bb1558d9c3a121faa5"
   }
   ```

4. Respond with the response JSON object.

   Respond with the response JSON within 3 seconds with a `200` or `204` HTTP response code.

   When Zoom successfully validates the challenge response from your webhook endpoint URL, you'll see a validated message under the **Event Notification Endpoint URL**. If validation fails, you'll see a failed to validate message. Once you successfully complete validation, click **Save**.

   <Image src="/img/webhook-validate-success.png" alt="Zoom webhook validate success" width={500} />

   **Here is an example in Node.js:**

   ```js
   const crypto = require('crypto')

   // Webhook request event type is a challenge-response check
   if(request.body.event === 'endpoint.url_validation') {
     const hashForValidate = crypto.createHmac('sha256', ZOOM_WEBHOOK_SECRET_TOKEN).update(request.body.payload.plainToken).digest('hex')

     response.status(200)
     response.json({
       "plainToken": request.body.payload.plainToken,
       "encryptedToken": hashForValidate
     })
   }
   ```

   See the [Webhook sample app for an example implementation](https://github.com/zoom/webhook-sample-node.js/blob/master/index.js#L34).

### Revalidation

Your production and development webhook URLs will be revalidated periodically every 72 hours. Zoom will send notification emails to the account owner associated with your apps if the URL fails the revalidation, following this schedule:

1. First notification email after a total of 2 consecutive failed revalidations.
2. Second notification email after a total of 4 consecutive failed revalidations.

If the revalidation check fails 6 times in a row, Zoom will stop sending webhook events and disable the event subscription. To fix the issue to receive webhook events again, go to your apps webhook endpoint URL configuration, reenable webhooks, update your endpoint URL if needed, click validate to revalidate your endpoint URL, and finally, click save.

If you are still having trouble, see the [Webhook sample app for an example implementation](https://github.com/zoom/webhook-sample-node.js/blob/master/index.js#L34) or review the [webhook endpoint URL requirements](/docs/api/webhooks/#webhook-endpoint-url-requirements).

---  

## Configure webhooks via Zoom API   
For existing apps, you can use the Zoom API to create and manage event subscriptions. This allows you to programmatically:  
- Subscribe users or accounts to specific webhook events based on their preference, actions or regions.  
- Change which events you are subscribed to without manually reconfiguring the app in the build flow.  
- Easily manage webhook subscriptions per customer when building multi-tenant applications.  

<Alert>

**Note**  

You can’t use the API method to modify event subscriptions created through the build flow.    

</Alert>  

The following is an example of creating an event subscription via API for a general app type. You can use the same general steps for other subscription management actions:    
 

* **Create an Event Subscription**      
  <a href="https://developers.zoom.us/docs/api/marketplace/#tag/app/post/marketplace/app/event_subscription">`POST https://api.zoom.us/v2/marketplace/app/event_subscription`</a>  
   

* **Subscribe to an Event Subscription**    
  <a href="https://developers.zoom.us/docs/api/marketplace/#tag/app/patch/marketplace/app/event_subscription/{eventSubscriptionId}">`PATCH https://api.zoom.us/v2/marketplace/app/event_subscription/{eventSubscriptionId}`</a>  

* **Unsubscribe from an Event Subscription**    
  <a href="https://developers.zoom.us/docs/api/marketplace/#tag/app/delete/marketplace/app/event_subscription">`DELETE https://api.zoom.us/v2/marketplace/app/event_subscription`</a>    

* **Delete an Event Subscription**  
  <a href="https://developers.zoom.us/docs/api/marketplace/#tag/app/delete/marketplace/app/event_subscription/{eventSubscriptionId}">`DELETE https://api.zoom.us/v2/marketplace/app/event_subscription/{eventSubscriptionId}`</a>

* **Get User or Account Event Subscriptions**  
   <a href="https://developers.zoom.us/docs/api/marketplace/#tag/app/get/marketplace/app/event_subscription">`GET https://api.zoom.us/v2/marketplace/app/event_subscription`</a>  
    
### Prerequisite:  
* You have an existing app.  
* In your app, you have already selected the scopes required by your event subscriptions. See [Scopes](#scopes).  

### Step 1: Request an access token  
[Request an access token](/docs/integrations/oauth/#request-an-access-token) to call APIs for your own Zoom account by sending a POST request to `https://zoom.us/oauth/token`.  

Include a basic authorization header. The basic authorization header is your Client ID and Client Secret with a colon : in between, Base64 Encoded.  


```bash  
-- request POST https://zoom.us/oauth/token?grant_type=client_credentials
-- header "Authorization": "Basic <YOUR_BASE64_ENCODED_CREDENTIALS>" 
```

<Alert variant="warning">

**Note** 

Webhook subscriptions are app-owned resources; you need to use the grant type appropriate to the app type to obtain the access token.  
* For General OAuth apps, use the [client credentials](/docs/integrations/oauth/#request-an-access-token) grant type.  
* For Server-to-Server OAuth apps, use the [account credentials](/docs/integrations/oauth/#request-an-access-token) grant type.

</Alert>

This returns an `access_token` with the required scopes to manage webhook subscriptions. Use this token as a bearer token in the authorization header of subsequent API calls.   

```json  
{
    "access_token": "eyJzyNzA2ZDlkMzk0MDgzTk0MzAzMjUsImlzcy...A",
    "token_type": "bearer",
    "expires_in": 3600,
    "scope": "marketplace:delete:event_subscription marketplace:read:list_event_subscriptions marketplace:update:client_secret marketplace:update:event_subscription marketplace:write:event_subscription marketplace:write:websocket_connection",
    "api_url": "https://api.zoom.us"
}

```

### Step 2: Create an event subscription 

Use the POST method for the `event_subscription` endpoint.  
```bash
POST https://api.zoom.us/v2/marketplace/app/event_subscription 
```  

Include the authorization in the request header.
```bash
{ "Authorization": "Bearer <access_token>" }
```

Example request body:  
```json  
{
  "events": [
    "meeting.created"
  ],
  "event_subscription_name": "Example Event Subscription",
  "event_webhook_url": "https://www.example.com",
  "user_ids": [
    "_8KG7DeoRU2xIsDSY9ed2Q,90KG7DeoRU2xIsDSY9edwe"
  ],
  "subscription_scope": "user",
  "account_id": "pvg3UAgpRlyTDW-9sIpKcw"
}

```  
On successful creation, Zoom provides an event_subscription_id.

```json  
{
  "event_subscription_id": "0ZAaJY4dQ52BbwI9PArBLQ"
}
```  
With this event_subscription_id you can then subscribe to, unsubscribe from, or delete event subscriptions.  

For more information, see the [Developer blog](/blog/webhook-management-with-zoom-api/). 

## Scopes  
Scopes control your app's access to the Zoom platform including the event information requested by your subscriptions.  

Zoom has automatically applied the scopes for event subscriptions to your app: (such as creating, updating, or deleting subscriptions). For any other events you want your app to receive, you must add the respective scopes to your app via the app build flow.   

For example:  
* To subscribe to the `meeting.participant_joined` event, your app must request the `meeting:read:participant` or `meeting:read:participant:admin`.  
* To receive `recording.completed`, you'll need `cloud_recording:read:recording` or `cloud_recording:read:recording:admin`.  

If you haven't added the appropriate scopes to the app, on using the event subscription endpoints you will receive an error. "The event is not allowed for the app."  

For more information, see [Selecting scope](/docs/build-flow/create-oauth-apps/#step-4-select-the-scopes-zoom-api-methods).




---  

## Notification delivery

Your webhook endpoint URL should respond with either a `200` or a `204` HTTP status code within **three seconds** after receiving a webhook request in order for Zoom to consider the notification was successfully delivered.

### Unsuccessful delivery

Zoom attempts to deliver the webhook **three times** for response codes >=`500` or codes in `-1`, `-2`, `-3`, `-4`, `-5`, `-7`, `-8`, `-10`, `-13`, `-14`, `-15`. (See **Enum Descriptions** below.)  

- The first attempt is sent **5 minutes** after the initial delivery attempt.
- A second attempt is sent **20 minutes** after the first attempt.
- A third attempt is sent **60 minutes** after the second attempt.

Zoom considers response codes >=`200` or < `300` as successfully delivered after any retry attempt.  

If Zoom does **not** receive a response code >=`200` or < `300` after three attempts, then Zoom will send no further webhooks for that event.  

Zoom does not retry redirect codes >=`300` and < `400`.  
Zoom does not retry client error codes >=`400` and < `500`, such as unauthenticated, resource not found, rate limiting, etc., because retrying would still fail or further increase the server load.  

**Enum Descriptions**  
- `ReadTimeOut(-1)` Socket time out.
- `ConnectionRefused(-2)` Connection refused.    
- `Fallback(-3)` Hystrix fallback on error.    
- `Resilience4jTimeout(-4)` Resilience time out.    
- `AsyncError(-5)` Failed to execute job via async mode.    
- `RemoteError(-7)` Remote error.    
- `Delegate(-8)` Delegate tasks to a separate server for processing if in delegate white list.  
- `NeedInstantRetry(-10)` Retry webhook invoke instantly when connection lease request times out.    
- `DNSResolveTimeout(-13)` Zoom DNS resolve timeout exception.    
- `CanNotGetAccessToken(-14)` Delay retry error code if developer enabled Oauth client credential header in the subscription, but Zoom cannot get access_token from tokenUrl by clientId & clientSecret.  
- `CanNotGetCustomHeaderSettingInNoCacheTopic(-15)` Delay retry error code if developer enabled custom header in the subscription, but Zoom cannot get custom header setting when handling msg from no cache topic.   
  


<Alert> 

**Troubleshooting missing webhook events**

If you have successfully validated your webhook endpoint, but you are not receiving certain events you have selected, toggle the webhook feature on and off and try again.

</Alert>

Use [Get webhook logs​](/docs/api/marketplace/#tag/app/GET/marketplace/apps/{appId}/webhook_logs) to get a list of webhooks sent to an app.

### Delivery latency

The `event_ts` response is included in the payload of all event notifications. This represents the timestamp for when the associated event occurred.

You can determine the latency in webhook delivery by calculating the difference between the delivery timestamp and the value of the `event_ts`.
```

## OpenAPI

No OpenAPI spec found for this page.

## Downloads

No download files found.

## Metadata

```json
{
  "sidebarName": "api"
}
```

## Raw Next.js page data

```json
{
  "pageProps": {
    "mainContent": {
      "code": "var Component=(()=>{var b=Object.create;var s=Object.defineProperty;var v=Object.getOwnPropertyDescriptor;var k=Object.getOwnPropertyNames;var y=Object.getPrototypeOf,w=Object.prototype.hasOwnProperty;var _=(i,n)=>()=>(n||i((n={exports:{}}).exports,n),n.exports),S=(i,n)=>{for(var o in n)s(i,o,{get:n[o],enumerable:!0})},p=(i,n,o,a)=>{if(n&&typeof n==\"object\"||typeof n==\"function\")for(let t of k(n))!w.call(i,t)&&t!==o&&s(i,t,{get:()=>n[t],enumerable:!(a=v(n,t))||a.enumerable});return i};var T=(i,n,o)=>(o=i!=null?b(y(i)):{},p(n||!i||!i.__esModule?s(o,\"default\",{value:i,enumerable:!0}):o,i)),x=i=>p(s({},\"__esModule\",{value:!0}),i);var m=_((A,u)=>{u.exports=_jsx_runtime});var q={};S(q,{default:()=>g,frontmatter:()=>E});var e=T(m());var{useMDXComponents:d}=MdxJsReact;var E={title:\"Using webhooks\",resources:[{title:\"Zoom Platform 101 Zoom Webhooks\",url:\"https://youtu.be/oAnr1ezW8dY?si=AVlihxis-DIS4L5W\"},{title:\"Server-to-Server apps\",url:\"https://developers.zoom.us/docs/internal-apps/\"},{title:\"General OAuth apps\",url:\"https://developers.zoom.us/docs/integrations/\"},{title:\"Build platform apps\",url:\"https://developers.zoom.us/docs/build/\"},{title:\"Updating an app\",url:\"https://developers.zoom.us/docs/distribute/published-apps/updating/\"}]};function f(i){let n={a:\"a\",br:\"br\",code:\"code\",em:\"em\",h1:\"h1\",h2:\"h2\",h3:\"h3\",h4:\"h4\",hr:\"hr\",i:\"i\",li:\"li\",ol:\"ol\",p:\"p\",pre:\"pre\",strong:\"strong\",table:\"table\",tbody:\"tbody\",td:\"td\",th:\"th\",thead:\"thead\",tr:\"tr\",ul:\"ul\",...d(),...i.components},{Alert:o,Image:a,Link:t,SvgIcon:r,Tab:h,Tabs:l}=n;return o||c(\"Alert\",!0),a||c(\"Image\",!0),t||c(\"Link\",!0),r||c(\"SvgIcon\",!0),h||c(\"Tab\",!0),l||c(\"Tabs\",!0),(0,e.jsxs)(e.Fragment,{children:[(0,e.jsx)(n.h1,{id:\"using-webhooks\",children:\"Using webhooks\"}),`\n`,(0,e.jsx)(n.p,{children:\"Instead of making repeated calls to pull data frequently from the Zoom API, you can use webhooks for Zoom to send events to your server. Zoom sends webhook events as HTTP POST requests with a JSON body to your app's specified endpoint.\"}),`\n`,(0,e.jsxs)(n.p,{children:[\"For an introduction and explanation of webhooks, see \",(0,e.jsx)(n.a,{href:\"https://youtu.be/oAnr1ezW8dY?si=AVlihxis-DIS4L5W\",children:\"Zoom Platform 101: Zoom Webhooks\"}),\".\"]}),`\n`,(0,e.jsxs)(o,{children:[(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Now available\"})}),(0,e.jsxs)(n.p,{children:[(0,e.jsx)(n.a,{href:\"/docs/rivet/javascript/get-started/#initialize-the-client\",children:\"Zoom Rivet\"}),\", Zoom's API library that handles webhooks for you.\"]})]}),`\n`,(0,e.jsx)(n.h2,{id:\"workplace\",children:(0,e.jsxs)(n.a,{href:\"#workplace\",children:[\"Workplace\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(\"div\",{class:\"api_ref_tile_container\",children:[(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/meetings/events\",children:[(0,e.jsx)(r,{svgname:\"_meetings\"}),(0,e.jsx)(\"h3\",{children:\"Meetings\"})]}),(0,e.jsxs)(t,{className:\"api_ref_tile\",href:\"/docs/api/rtms/events\",children:[(0,e.jsx)(r,{svgname:\"_rtms\"}),(0,e.jsx)(\"h3\",{children:\"RTMS\"})]}),(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/team-chat/events\",children:[(0,e.jsx)(r,{svgname:\"_chat\"}),(0,e.jsx)(\"h3\",{children:\"Team Chat\"})]}),(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/phone/events\",children:[(0,e.jsx)(r,{svgname:\"_zoomphone\"}),(0,e.jsx)(\"h3\",{children:\"Phone\"})]}),(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/mail/events\",children:[(0,e.jsx)(r,{svgname:\"_mail\"}),(0,e.jsx)(\"h3\",{children:\"Mail\"})]}),(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/calendar/events\",children:[(0,e.jsx)(r,{svgname:\"_calendar\"}),(0,e.jsx)(\"h3\",{children:\"Calendar\"})]}),(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/rooms/events\",children:[(0,e.jsx)(r,{svgname:\"_zoomrooms\"}),(0,e.jsx)(\"h3\",{children:\"Rooms\"})]}),(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/whiteboard/events\",children:[(0,e.jsx)(r,{svgname:\"_whiteboard\"}),(0,e.jsx)(\"h3\",{children:\"Whiteboard\"})]}),(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/chatbot/events\",children:[(0,e.jsx)(r,{svgname:\"_chat\"}),(0,e.jsx)(\"h3\",{children:\"Chatbot\"})]}),(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/scheduler/events\",children:[(0,e.jsx)(r,{svgname:\"_zoomscheduler\"}),(0,e.jsx)(\"h3\",{children:\"Scheduler\"})]})]}),`\n`,(0,e.jsx)(n.h2,{id:\"business-services\",children:(0,e.jsxs)(n.a,{href:\"#business-services\",children:[\"Business services\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(\"div\",{class:\"api_ref_tile_container\",children:[(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/contact-center/events\",children:[(0,e.jsx)(r,{svgname:\"contactCenter\"}),(0,e.jsx)(\"h3\",{children:\"Contact Center\"})]}),(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/events/events\",children:[(0,e.jsx)(r,{svgname:\"_zoomevents\"}),(0,e.jsx)(\"h3\",{children:\"Events\"})]}),(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/iq/events\",children:[(0,e.jsx)(r,{svgname:\"zra\"}),(0,e.jsx)(\"h3\",{children:\"Revenue Accelerator\"})]}),(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/number-management/events\",children:[(0,e.jsx)(r,{svgname:\"number-management\"}),(0,e.jsx)(\"h3\",{children:\"Number Management\"})]}),(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/node/events\",children:[(0,e.jsx)(r,{svgname:\"zoom-node\"}),(0,e.jsx)(\"h3\",{children:\"Node\"})]}),(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/quality-management/events\",children:[(0,e.jsx)(r,{svgname:\"qualityManagement\"}),(0,e.jsx)(\"h3\",{children:\"Quality Management\"})]}),(0,e.jsxs)(t,{className:\"api_ref_tile\",href:\"/docs/api/healthcare/events\",children:[(0,e.jsx)(r,{svgname:\"clinical-notes\"}),(0,e.jsx)(\"h3\",{children:\"Healthcare\"})]}),(0,e.jsxs)(t,{className:\"api_ref_tile\",href:\"/docs/api/video-management/events\",children:[(0,e.jsx)(r,{svgname:\"video-management\"}),(0,e.jsx)(\"h3\",{children:\"Video Management\"})]})]}),`\n`,(0,e.jsx)(n.h2,{id:\"accounts\",children:(0,e.jsxs)(n.a,{href:\"#accounts\",children:[\"Accounts\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(\"div\",{class:\"api_ref_tile_container\",children:[(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/users/events\",children:[(0,e.jsx)(r,{svgname:\"users\"}),(0,e.jsx)(\"h3\",{children:\"Users\"})]}),(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/accounts/events\",children:[(0,e.jsx)(r,{svgname:\"accounts\"}),(0,e.jsx)(\"h3\",{children:\"Accounts\"})]}),(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/qss/events\",children:[(0,e.jsx)(r,{svgname:\"qss\"}),(0,e.jsx)(\"h3\",{children:\"QSS\"})]})]}),`\n`,(0,e.jsx)(n.h2,{id:\"build-platform\",children:(0,e.jsxs)(n.a,{href:\"#build-platform\",children:[\"Build platform\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(\"div\",{class:\"api_ref_tile_container\",children:[(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/video-sdk/events\",children:[(0,e.jsx)(r,{svgname:\"_video-sdks\"}),(0,e.jsx)(\"h3\",{children:\"Video SDK\"})]}),(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/cobrowse-sdk/events\",children:[(0,e.jsx)(r,{svgname:\"cobrowse-sdk\"}),(0,e.jsx)(\"h3\",{children:\"Cobrowse SDK\"})]})]}),`\n`,(0,e.jsx)(n.h2,{id:\"marketplace\",children:(0,e.jsxs)(n.a,{href:\"#marketplace\",children:[\"Marketplace\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(\"div\",{class:\"api_ref_tile_container\",children:(0,e.jsxs)(t,{class:\"api_ref_tile\",href:\"/docs/api/marketplace/events\",children:[(0,e.jsx)(r,{svgname:\"marketplace\"}),(0,e.jsx)(\"h3\",{children:\"Apps\"})]})}),`\n`,(0,e.jsx)(n.h2,{id:\"configure-webhooks-via-build-flow\",children:(0,e.jsxs)(n.a,{href:\"#configure-webhooks-via-build-flow\",children:[\"Configure webhooks via build flow\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"You configure webhooks by subscribing to specific events in the app build flow.\"}),`\n`,(0,e.jsxs)(n.p,{children:[\"For instructions to configure your webhook subscriptions using Zoom APIs, see \",(0,e.jsx)(n.a,{href:\"/#Configure-webhooks-via-Zoom-API\",children:\"Configure webhooks via Zoom API\"}),\".\"]}),`\n`,(0,e.jsxs)(n.ol,{children:[`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Go to your app's \",(0,e.jsx)(n.strong,{children:\"Access\"}),\" page.\",(0,e.jsx)(n.br,{}),`\n`,\"a. Login to the \",(0,e.jsx)(n.a,{href:\"https://marketplace.zoom.us/\",children:\"Zoom App Marketplace\"}),\" and click \",(0,e.jsx)(n.strong,{children:\"Manage\"}),\".\",(0,e.jsx)(n.br,{}),`\n`,\"b. On the \",(0,e.jsx)(n.strong,{children:\"Created Apps\"}),\" page, open your app, and go to the \",(0,e.jsx)(n.strong,{children:\"Features\"}),\" \\u2192 \",(0,e.jsx)(n.strong,{children:\"Access\"}),\".\"]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Under the \",(0,e.jsx)(n.strong,{children:\"General Features\"}),\" section, enable \",(0,e.jsx)(n.strong,{children:\"Event Subscriptions\"}),\" and \",(0,e.jsx)(n.strong,{children:\"Add New Event Subscription\"}),\".\"]}),`\n`,(0,e.jsx)(a,{src:\"/img/add-new-subscription2.png\",width:500}),`\n`,(0,e.jsxs)(n.p,{children:[\"NOTE: You can subscribe to as many events as needed for each event subscription, up to a maximum of 20 event subscriptions per app. Event subscriptions can also have duplicate events. For example, one event subscription could have \",(0,e.jsx)(n.em,{children:\"Meetings\"}),\" and \",(0,e.jsx)(n.em,{children:\"User Events\"}),\", and a second event subscription can have \",(0,e.jsx)(n.em,{children:\"Meetings\"}),\" and \",(0,e.jsx)(n.em,{children:\"Recordings\"}),\" events.\"]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"On the \",(0,e.jsx)(n.strong,{children:\"Event Types\"}),\" screen, select the specific events and then \",(0,e.jsx)(n.strong,{children:\"Done\"}),\".\"]}),`\n`,(0,e.jsx)(a,{src:\"/img/add-events.png\",width:500}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Set your \",(0,e.jsx)(n.strong,{children:\"Event notification endpoint URL\"}),\" for each event subscription in your app and \",(0,e.jsx)(n.strong,{children:\"Save\"}),\". Be sure that it satisfies the \",(0,e.jsx)(n.a,{href:\"#webhook-endpoint-url-requirements\",children:\"webhook endpoint requirements\"}),\".\"]}),`\n`,(0,e.jsx)(a,{src:\"/img/save-subscription.png\",width:500}),`\n`]}),`\n`]}),`\n`,(0,e.jsx)(n.h3,{id:\"webhook-endpoint-url-requirements\",children:(0,e.jsxs)(n.a,{href:\"#webhook-endpoint-url-requirements\",children:[\"Webhook endpoint URL requirements\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.p,{children:[\"To receive webhooks in development and production environments, the \",(0,e.jsx)(n.strong,{children:\"Event notification endpoint URL\"}),\" that you specify for each event subscription must:\"]}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsxs)(n.li,{children:[\"Be a publicly accessible \",(0,e.jsx)(n.code,{children:\"https\"}),\" endpoint url that supports TLSv1.2+ with a valid certificate chain issued by a \",(0,e.jsx)(n.a,{href:\"https://en.wikipedia.org/wiki/Certificate_authority\",children:\"Certificate Authority (CA)\"}),\".\"]}),`\n`,(0,e.jsx)(n.li,{children:\"Be a fully qualified domain name (FQDN).\"}),`\n`,(0,e.jsx)(n.li,{children:\"Be able to accept HTTP POST requests containing JSON payloads.\"}),`\n`,(0,e.jsxs)(n.li,{children:[\"Be able to respond with a \",(0,e.jsx)(n.code,{children:\"200\"}),\" or \",(0,e.jsx)(n.code,{children:\"204\"}),\" HTTP Status Code.\"]}),`\n`]}),`\n`,(0,e.jsx)(n.h3,{id:\"webhook-structure\",children:(0,e.jsxs)(n.a,{href:\"#webhook-structure\",children:[\"Webhook structure\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"Webhooks events are POST requests sent to your endpoint URL and contain request headers and a request body of the event type, event timestamp, and event details within the payload object.\"}),`\n`,(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Example request headers:\"})}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n  \"host\": \"example.com\",\n  \"user-agent\": \"Zoom Marketplace/1.0a\",\n  \"content-length\": \"110\",\n  \"authorization\": \"Wk9PTV9BVVRIT1JJWkFUSU9O\",\n  \"content-type\": \"application/json; charset=utf-8\",\n  \"traceparent\": \"Wk9PTV9UUkFDRVBBUkVOVA\",\n  \"x-forwarded-for\": \"{X_FORWARDED_FOR}\",\n  \"x-forwarded-host\": \"example.com\",\n  \"x-forwarded-proto\": \"https\",\n  \"x-zm-request-timestamp\": \"1739923528\",\n  \"x-zm-signature\": \"v0=WF9aT09NX1NJR05BVFVSRQ\",\n  \"x-zm-request-id\": \"6009d653_d487_445d_8406_42b654974899\",\n  \"accept-encoding\": \"gzip\"\n}\n`})}),`\n`,(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Example request body:\"})}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n  \"event\": \"meeting.started\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"Wk9PTV9BQ0NPVU5UX0lE\",\n    \"object\": {\n      \"id\": \"1234567890\",\n      \"uuid\": \"Wk9PTV9NRUVUSU5HX0lE\",\n      \"host_id\": \"Wk9PTV9VU0VSX0lE\",\n      \"topic\": \"My Meeting\",\n      \"type\": 8,\n      \"start_time\": \"2021-07-13T21:44:51Z\",\n      \"timezone\": \"America/Los_Angeles\",\n      \"duration\": 60\n    }\n  }\n}\n`})}),`\n`,(0,e.jsx)(n.h3,{id:\"zoom-ip-addresses\",children:(0,e.jsxs)(n.a,{href:\"#zoom-ip-addresses\",children:[\"Zoom IP addresses\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.p,{children:[\"We \",(0,e.jsx)(n.strong,{children:\"strongly\"}),\" recommend that you \",(0,e.jsx)(n.a,{href:\"#verify-webhook-events\",children:\"Verify webhook events\"}),\" instead of creating an allow list of Zoom IP addresses because Zoom may update the \",(0,e.jsx)(n.a,{href:\"https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0060548\",children:\"IP ranges\"}),\" used at any time.\"]}),`\n`,(0,e.jsx)(n.h2,{id:\"verify-webhook-events\",children:(0,e.jsxs)(n.a,{href:\"#verify-webhook-events\",children:[\"Verify webhook events\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"Zoom offers two ways to verify the authenticity of a webhook, to ensure developers that the request came from Zoom:\"}),`\n`,(0,e.jsxs)(l,{children:[(0,e.jsxs)(h,{eventKey:\"verifyWithZoomsHeader\",title:\"Verify with Zoom's header\",children:[(0,e.jsx)(n.h3,{id:\"verify-with-zooms-header\",children:(0,e.jsxs)(n.a,{href:\"#verify-with-zooms-header\",children:[\"Verify with Zoom's header\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),(0,e.jsxs)(n.p,{children:[\"Zoom includes a webhook secret token on the \",(0,e.jsx)(n.strong,{children:\"Add Feature\"}),\" page for your app. Zoom uses the value of the secret token to hash the webhook data, which it sends in the \",(0,e.jsx)(n.code,{children:\"x-zm-signature\"}),\" webhook request header.\"]}),(0,e.jsxs)(n.p,{children:[\"To verify that Zoom sent a webhook request, use the webhook secret token and the webhook request body to create a signature to compare with the \",(0,e.jsx)(n.code,{children:\"x-zm-signature\"}),\" header value sent by Zoom.\"]}),(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Example request headers:\"})}),(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n  \"host\": \"example.com\",\n  \"user-agent\": \"Zoom Marketplace/1.0a\",\n  \"content-length\": \"110\",\n  \"authorization\": \"Wk9PTV9BVVRIT1JJWkFUSU9O\",\n  \"content-type\": \"application/json; charset=utf-8\",\n  \"traceparent\": \"Wk9PTV9UUkFDRVBBUkVOVA\",\n  \"x-forwarded-for\": \"{X_FORWARDED_FOR}\",\n  \"x-forwarded-host\": \"example.com\",\n  \"x-forwarded-proto\": \"https\",\n  \"x-zm-request-timestamp\": \"1739923528\",\n  \"x-zm-signature\": \"v0=WF9aT09NX1NJR05BVFVSRQ\",\n  \"accept-encoding\": \"gzip\"\n}\n`})}),(0,e.jsxs)(n.p,{children:[(0,e.jsx)(n.strong,{children:\"Create your signature string\"}),\" to compare with the Zoom webhook \",(0,e.jsx)(n.code,{children:\"x-zm-signature\"}),\" header value:\"]}),(0,e.jsxs)(n.ol,{children:[`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsx)(n.p,{children:\"Receive the webhook request.\"}),`\n`,(0,e.jsx)(n.p,{children:\"The webhook request body includes the following properties:\"}),`\n`,(0,e.jsxs)(n.table,{children:[(0,e.jsx)(n.thead,{children:(0,e.jsxs)(n.tr,{children:[(0,e.jsx)(n.th,{children:\"Key\"}),(0,e.jsx)(n.th,{children:\"Value Type\"}),(0,e.jsx)(n.th,{children:\"Value Description\"})]})}),(0,e.jsxs)(n.tbody,{children:[(0,e.jsxs)(n.tr,{children:[(0,e.jsx)(n.td,{children:(0,e.jsx)(n.code,{children:\"event\"})}),(0,e.jsx)(n.td,{children:\"string\"}),(0,e.jsx)(n.td,{children:\"The type of the webhook event\"})]}),(0,e.jsxs)(n.tr,{children:[(0,e.jsx)(n.td,{children:(0,e.jsx)(n.code,{children:\"payload\"})}),(0,e.jsx)(n.td,{children:\"object\"}),(0,e.jsx)(n.td,{children:\"Contains the event webhook details\"})]}),(0,e.jsxs)(n.tr,{children:[(0,e.jsx)(n.td,{children:(0,e.jsx)(n.code,{children:\"event_ts\"})}),(0,e.jsx)(n.td,{children:\"number\"}),(0,e.jsx)(n.td,{children:\"The timestamp of the webhook event.\"})]})]})]}),`\n`,(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Example request body:\"})}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n  \"event\": \"session.started\",\n  \"payload\": {\n    \"account_id\": \"{ACCOUNT_ID}\",\n    \"object\": {\n      \"start_time\": \"2022-07-27T16:56:34Z\",\n      \"id\":\"{SESSION_ID}\"\n    }\n  },\n  \"event_ts\": 1658940994914\n}\n`})}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsx)(n.p,{children:\"Construct the message.\"}),`\n`,(0,e.jsxs)(n.p,{children:[\"After receiving a webhook request, construct the message string with \",(0,e.jsx)(n.code,{children:'\"v0\"'}),\", the webhook request header \",(0,e.jsx)(n.code,{children:\"x-zm-request-timestamp\"}),\" value, and the webhook request body. Separate each section with a colon \",(0,e.jsx)(n.code,{children:\":\"}),\". For example:\"]}),`\n`,(0,e.jsx)(n.p,{children:(0,e.jsx)(n.code,{children:\"v0:{WEBHOOK_REQUEST_HEADER_X-ZM-REQUEST-TIMESTAMP_VALUE}:{WEBHOOK_REQUEST_BODY}\"})}),`\n`,(0,e.jsx)(n.p,{children:\"Example string with printed values:\"}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{children:`v0:1658940994:{\"event\":\"session.started\",\"payload\":{\"account_id\":\"{ACCOUNT_ID}\",\"object\":{\"start_time\":\"2022-07-27T16:56:34Z\",\"id\":\"{SESSION_ID}\"}},\"event_ts\":1658940994914}\n`})}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsx)(n.p,{children:\"Hash the message.\"}),`\n`,(0,e.jsxs)(n.p,{children:[\"Once you have constructed the message string, create a \",(0,e.jsx)(n.a,{href:\"https://datatracker.ietf.org/doc/html/rfc4634\",children:\"HMAC SHA-256\"}),\" hash. Set your webhook's secret token as the secret/salt, and the \",(0,e.jsx)(n.code,{children:\"message\"}),\" you constructed as the string to hash. Output in \",(0,e.jsx)(n.code,{children:\"hex\"}),\" format.\"]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsx)(n.p,{children:\"Create the signature.\"}),`\n`,(0,e.jsxs)(n.p,{children:[\"Then, create the signature by prepending \",(0,e.jsx)(n.code,{children:\"v0=\"}),\" to the hashed message. For example:\"]}),`\n`,(0,e.jsx)(n.p,{children:(0,e.jsx)(n.code,{children:\"v0={HASHED_MESSAGE}\"})}),`\n`,(0,e.jsx)(n.p,{children:\"Example string with printed values:\"}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{children:`v0=a05d830fa017433bc47887f835a00b9ff33d3882f22f63a2986a8es270341\n`})}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsx)(n.p,{children:\"Compare the signature.\"}),`\n`,(0,e.jsxs)(n.p,{children:[\"Compare the \",(0,e.jsx)(n.code,{children:\"signature\"}),\" you created with the Zoom webhook request header \",(0,e.jsx)(n.code,{children:\"x-zm-signature\"}),\" value. If it matches, the webhook request came from Zoom.\"]}),`\n`,(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Here is an example in Node.js:\"})}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-js\",children:`const crypto = require('crypto')\n\nconst message = \\`v0:\\${request.headers['x-zm-request-timestamp']}:\\${JSON.stringify(request.body)}\\`\n\nconst hashForVerify = crypto.createHmac('sha256', ZOOM_WEBHOOK_SECRET_TOKEN).update(message).digest('hex')\n\nconst signature = \\`v0=\\${hashForVerify}\\`\n\nif (request.headers['x-zm-signature'] === signature) {\n  // Webhook request came from Zoom\n} else {\n  // Webhook request did not come from Zoom\n}\n`})}),`\n`,(0,e.jsxs)(n.p,{children:[\"See the \",(0,e.jsx)(n.a,{href:\"https://github.com/zoom/webhook-sample-node.js/blob/master/index.js#L31\",children:\"Webhook sample app for an example implementation\"}),\".\"]}),`\n`]}),`\n`]}),(0,e.jsxs)(n.p,{children:[\"Learn more about \",(0,e.jsx)(n.a,{href:\"https://webhooks.fyi/security/hmac\",children:\"HMAC Auth for webhooks\"}),\".\"]})]}),(0,e.jsxs)(h,{eventKey:\"verifyWithYourHeader\",title:\"Verify with your own header\",children:[(0,e.jsx)(n.h3,{id:\"verify-with-your-own-header\",children:(0,e.jsxs)(n.a,{href:\"#verify-with-your-own-header\",children:[\"Verify with your own header\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),(0,e.jsx)(n.p,{children:\"Zoom offers the following options to verify webhooks with your own header:\"}),(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsx)(n.li,{children:(0,e.jsx)(n.a,{href:\"#basic-authentication\",children:\"Basic Authentication\"})}),`\n`,(0,e.jsx)(n.li,{children:(0,e.jsx)(n.a,{href:\"#token-authentication-oauth\",children:\"Token Authentication (OAuth)\"})}),`\n`,(0,e.jsx)(n.li,{children:(0,e.jsx)(n.a,{href:\"#custom-header\",children:\"Custom Header\"})}),`\n`]}),(0,e.jsx)(n.h4,{id:\"basic-authentication\",children:(0,e.jsxs)(n.a,{href:\"#basic-authentication\",children:[\"Basic authentication\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),(0,e.jsxs)(n.ol,{children:[`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Choose \",(0,e.jsx)(n.strong,{children:\"Basic Authentication\"}),\". Specify a username and password.\"]}),`\n`,(0,e.jsx)(a,{src:\"/img/webhooks_basic_auth.jpg\",alt:\"Zoom webhook basic auth\",width:500}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"When a webhook is triggered, Zoom makes a POST request with the following request headers, including the \",(0,e.jsx)(n.code,{children:\"authorization\"}),\" value being your username and password, base64encoded with a colon \",(0,e.jsx)(n.code,{children:\":\"}),\" in between:\"]}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n  \"host\": \"example.com\",\n  \"user-agent\": \"Zoom Marketplace/1.0a\",\n  \"content-length\": \"320\",\n  \"authorization\": \"Basic Q2xpZW50SUQ6Q2xpZW50U2VjcmV0\",\n  \"clientid\": \"{CLIENT_ID}\",\n  \"content-type\": \"application/json; charset-utf-8\",\n  \"x-forwarded-for\": \"{X_FORWARDED_FOR}\",\n  \"x-forwarded-proto\": \"https\",\n  \"x-zm-request-timestamp\": \"1674780079\",\n  \"x-zm-signature\": \"v0={HASHED_WEBHOOK_SECRET_TOKEN}\",\n  \"x-zm-trackingid\": \"{X_ZM_TRACKINGID}\",\n  \"accept-encoding\": \"gzip\"\n}\n`})}),`\n`]}),`\n`]}),(0,e.jsxs)(n.p,{children:[\"Learn more about \",(0,e.jsx)(n.a,{href:\"https://webhooks.fyi/security/shared-secret\",children:\"Basic Auth for webhooks\"}),\".\"]}),(0,e.jsx)(n.h4,{id:\"token-authentication-oauth\",children:(0,e.jsxs)(n.a,{href:\"#token-authentication-oauth\",children:[\"Token authentication (OAuth)\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),(0,e.jsxs)(n.ol,{children:[`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsx)(n.p,{children:`Choose the \"Token Authentication\" option. Specify an OAuth token endpoint, and your OAuth server's Client ID and Client Secret.`}),`\n`,(0,e.jsx)(n.p,{children:(0,e.jsx)(n.code,{children:\"https://example.com/oauth/token?grant_type=client_credentials\"})}),`\n`,(0,e.jsx)(a,{src:\"/img/webhooks_token_auth.jpg\",alt:\"Zoom webhook oauth\",width:500}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"When a webhook is triggered, Zoom makes a POST request to retrieve an access_token from your OAuth server, with the following request headers, including the  \",(0,e.jsx)(n.code,{children:\"authorization\"}),\" value being your Client ID and Client Secret, base64encoded, with a colon \",(0,e.jsx)(n.code,{children:\":\"}),\" in between:\"]}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n  \"host\": \"example.com\",\n  \"user-agent\": \"Zoom Marketplace/1.0a\",\n  \"content-length\": \"0\",\n  \"authorization\": \"Basic Q2xpZW50SUQ6Q2xpZW50U2VjcmV0\",\n  \"content-type\": \"application/x-www-form-urlencoded\",\n  \"x-forwarded-for\": \"{X_FORWARDED_FOR}\",\n  \"x-forwarded-proto\": \"https\",\n  \"accept-encoding\": \"gzip\"\n}\n`})}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"After you validate the authorization header, respond to the request with a \",(0,e.jsx)(n.code,{children:\"200\"}),\" status code and the following JSON. \",(0,e.jsx)(n.strong,{children:\"expires_in\"}),\" must be equal to or greater than 3599.\"]}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n  \"access_token\": \"{ACCESS_TOKEN}\",\n  \"token_type\": \"bearer\",\n  \"expires_in\": \"3599\"\n}\n`})}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsx)(n.p,{children:\"Zoom sends the webhook request to your server with the authorization bearer header set to the value of your access_token:\"}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n  \"host\": \"example.com\",\n  \"user-agent\": \"Zoom Marketplace/1.0a\",\n  \"content-length\": \"110\",\n  \"authorization\": \"Bearer {ACCESS_TOKEN}\",\n  \"content-type\": \"application/json; charset=utf-8\",\n  \"x-forwarded-for\": \"{X_FORWARDED_FOR}\",\n  \"x-forwarded-proto\": \"https\",\n  \"x-zm-request-timestamp\": \"1674847087\",\n  \"x-zm-signature\": \"v0={HASHED_WEBHOOK_SECRET_TOKEN}\",\n  \"accept-encoding\": \"gzip\"\n}\n`})}),`\n`]}),`\n`]}),(0,e.jsxs)(n.p,{children:[\"Learn more about \",(0,e.jsx)(n.a,{href:\"https://webhooks.fyi/security/jwt-jwk-oauth2\",children:\"OAuth for webhooks\"}),\".\"]}),(0,e.jsx)(n.h4,{id:\"custom-header\",children:(0,e.jsxs)(n.a,{href:\"#custom-header\",children:[\"Custom header\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),(0,e.jsxs)(n.ol,{children:[`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Choose \",(0,e.jsx)(n.strong,{children:\"Custom Header\"}),\". Specify a key and value.\"]}),`\n`,(0,e.jsx)(a,{src:\"/img/webhook-custom-header.png\",alt:\"Zoom webhook custom header\",width:500}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"When an event triggers a webhook, Zoom makes a POST request with the following request headers, including your custom header property (in this example \",(0,e.jsx)(n.code,{children:\"x-my-custom-key\"}),\"):\"]}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n  \"host\": \"example.com\",\n  \"user-agent\": \"Zoom Marketplace/1.0a\",\n  \"content-length\": \"110\",\n  \"content-type\": \"application/json; charset=utf-8\",\n  \"x-forwarded-for\": \"{X_FORWARDED_FOR}\",\n  \"x-forwarded-proto\": \"https\",\n  \"x-my-custom-key\": \"my-custom-value\",\n  \"x-zm-request-timestamp\": \"1674847087\",\n  \"x-zm-signature\": \"v0={HASHED_WEBHOOK_SECRET_TOKEN}\",\n  \"accept-encoding\": \"gzip\"\n}\n`})}),`\n`,(0,e.jsxs)(o,{children:[(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Removing the x- Prefix in custom header keys\"})}),(0,e.jsxs)(n.p,{children:[\"If you want to remove the \",(0,e.jsx)(n.code,{children:\"x-\"}),\" prefix in the custom header key, please reach out to \",(0,e.jsx)(n.a,{href:\"https://support.zoom.us\",children:\"Zoom Support\"}),\" or post on our \",(0,e.jsx)(n.a,{href:\"https://devforum.zoom.us/\",children:\"Developer Forum\"}),\".\"]})]}),`\n`]}),`\n`]}),(0,e.jsxs)(n.p,{children:[\"Learn more about \",(0,e.jsx)(n.a,{href:\"https://webhooks.fyi/security/one-time-verification-challenge\",children:\"One Time Verification for webhooks\"}),\".\"]})]})]}),`\n`,(0,e.jsx)(n.h2,{id:\"validate-your-webhook-endpoint\",children:(0,e.jsxs)(n.a,{href:\"#validate-your-webhook-endpoint\",children:[\"Validate your webhook endpoint\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"Zoom requires that you manually trigger webhook validation when you add a new webhook or make changes to an existing one. Subsequently, Zoom  automatically revalidates webhooks every 72 hours.\"}),`\n`,(0,e.jsx)(n.p,{children:\"Zoom uses a challenge-response check (CRC) for webhook validation. When a CRC occurs, Zoom makes a POST request to your endpoint with a challenge request body. After your endpoint receives the request, your app needs to respond with the challenge response within 3 seconds.\"}),`\n`,(0,e.jsxs)(n.p,{children:[\"To trigger the initial CRC validation, click \",(0,e.jsx)(n.strong,{children:\"Validate\"}),\" under the \",(0,e.jsx)(n.strong,{children:\"Event Notification Endpoint URL\"}),\" on the \",(0,e.jsx)(n.strong,{children:\"Feature\"}),\" page for your app. See \",(0,e.jsx)(n.a,{href:\"#revalidation\",children:\"Revalidation\"}),\" for revalidation details.\"]}),`\n`,(0,e.jsx)(a,{src:\"/img/webhook-validate.png\",alt:\"Zoom webhook validate\",width:500}),`\n`,(0,e.jsxs)(o,{children:[(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Note\"})}),(0,e.jsx)(n.p,{children:\"You won't be able to save your changes until after Zoom validates your endpoint.\"})]}),`\n`,(0,e.jsx)(n.h3,{id:\"implement-the-challenge-response-check-flow\",children:(0,e.jsxs)(n.a,{href:\"#implement-the-challenge-response-check-flow\",children:[\"Implement the challenge-response check flow\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.ol,{children:[`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsx)(n.p,{children:\"Receive the challenge (webhook request body).\"}),`\n`,(0,e.jsx)(n.p,{children:\"The webhook request body includes the following properties:\"}),`\n`,(0,e.jsxs)(n.table,{children:[(0,e.jsx)(n.thead,{children:(0,e.jsxs)(n.tr,{children:[(0,e.jsx)(n.th,{children:\"Key\"}),(0,e.jsx)(n.th,{children:\"Value Type\"}),(0,e.jsx)(n.th,{children:\"Value Description\"})]})}),(0,e.jsxs)(n.tbody,{children:[(0,e.jsxs)(n.tr,{children:[(0,e.jsx)(n.td,{children:(0,e.jsx)(n.code,{children:\"event\"})}),(0,e.jsx)(n.td,{children:\"string\"}),(0,e.jsxs)(n.td,{children:[\"The type of the webhook event, which will be \",(0,e.jsx)(n.code,{children:'\"endpoint.url_validation\"'})]})]}),(0,e.jsxs)(n.tr,{children:[(0,e.jsx)(n.td,{children:(0,e.jsx)(n.code,{children:\"payload\"})}),(0,e.jsx)(n.td,{children:\"object\"}),(0,e.jsxs)(n.td,{children:[\"Contains a property with the \",(0,e.jsx)(n.code,{children:\"plainToken\"}),\" value, the string to hash.\"]})]}),(0,e.jsxs)(n.tr,{children:[(0,e.jsx)(n.td,{children:(0,e.jsx)(n.code,{children:\"event_ts\"})}),(0,e.jsx)(n.td,{children:\"number\"}),(0,e.jsx)(n.td,{children:\"The timestamp of the webhook event.\"})]})]})]}),`\n`,(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Example request headers:\"})}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n  \"host\": \"example.com\",\n  \"user-agent\": \"Zoom Marketplace/1.0a\",\n  \"content-length\": \"110\",\n  \"authorization\": \"Wk9PTV9BVVRIT1JJWkFUSU9O\",\n  \"content-type\": \"application/json; charset=utf-8\",\n  \"traceparent\": \"Wk9PTV9UUkFDRVBBUkVOVA\",\n  \"x-forwarded-for\": \"{X_FORWARDED_FOR}\",\n  \"x-forwarded-host\": \"example.com\",\n  \"x-forwarded-proto\": \"https\",\n  \"x-zm-request-timestamp\": \"1739923528\",\n  \"x-zm-signature\": \"v0=WF9aT09NX1NJR05BVFVSRQ\",\n  \"accept-encoding\": \"gzip\"\n}\n`})}),`\n`,(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Example request body:\"})}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n  \"payload\": {\n    \"plainToken\": \"qgg8vlvZRS6UYooatFL8Aw\"\n  },\n  \"event_ts\": 1654503849680,\n  \"event\": \"endpoint.url_validation\"\n}\n`})}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsx)(n.p,{children:\"Hash the plainToken.\"}),`\n`,(0,e.jsxs)(n.p,{children:[\"Once you receive the request body, create a \",(0,e.jsx)(n.a,{href:\"https://datatracker.ietf.org/doc/html/rfc4634\",children:\"HMAC SHA-256\"}),\" hash. Set your webhook's secret token as the secret (salt), and the \",(0,e.jsx)(n.code,{children:\"plainToken\"}),\" value as the string to hash. Output in \",(0,e.jsx)(n.code,{children:\"hex\"}),\" format.\"]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsx)(n.p,{children:\"Create the response JSON object.\"}),`\n`,(0,e.jsxs)(n.p,{children:[\"Create a JSON object with a key of \",(0,e.jsx)(n.code,{children:'\"plainToken\"'}),\" with a value of the \",(0,e.jsx)(n.code,{children:\"plainToken\"}),\" from the request body, and a key of \",(0,e.jsx)(n.code,{children:'\"encryptedToken\"'}),\"  with a value of the hashed \",(0,e.jsx)(n.code,{children:\"plainToken\"}),\". For example:\"]}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n  \"plainToken\": \"qgg8vlvZRS6UYooatFL8Aw\",\n  \"encryptedToken\": \"23a89b634c017e5364a1c8d9c8ea909b60dd5599e2bb04bb1558d9c3a121faa5\"\n}\n`})}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsx)(n.p,{children:\"Respond with the response JSON object.\"}),`\n`,(0,e.jsxs)(n.p,{children:[\"Respond with the response JSON within 3 seconds with a \",(0,e.jsx)(n.code,{children:\"200\"}),\" or \",(0,e.jsx)(n.code,{children:\"204\"}),\" HTTP response code.\"]}),`\n`,(0,e.jsxs)(n.p,{children:[\"When Zoom successfully validates the challenge response from your webhook endpoint URL, you'll see a validated message under the \",(0,e.jsx)(n.strong,{children:\"Event Notification Endpoint URL\"}),\". If validation fails, you'll see a failed to validate message. Once you successfully complete validation, click \",(0,e.jsx)(n.strong,{children:\"Save\"}),\".\"]}),`\n`,(0,e.jsx)(a,{src:\"/img/webhook-validate-success.png\",alt:\"Zoom webhook validate success\",width:500}),`\n`,(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Here is an example in Node.js:\"})}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-js\",children:`const crypto = require('crypto')\n\n// Webhook request event type is a challenge-response check\nif(request.body.event === 'endpoint.url_validation') {\n  const hashForValidate = crypto.createHmac('sha256', ZOOM_WEBHOOK_SECRET_TOKEN).update(request.body.payload.plainToken).digest('hex')\n\n  response.status(200)\n  response.json({\n    \"plainToken\": request.body.payload.plainToken,\n    \"encryptedToken\": hashForValidate\n  })\n}\n`})}),`\n`,(0,e.jsxs)(n.p,{children:[\"See the \",(0,e.jsx)(n.a,{href:\"https://github.com/zoom/webhook-sample-node.js/blob/master/index.js#L34\",children:\"Webhook sample app for an example implementation\"}),\".\"]}),`\n`]}),`\n`]}),`\n`,(0,e.jsx)(n.h3,{id:\"revalidation\",children:(0,e.jsxs)(n.a,{href:\"#revalidation\",children:[\"Revalidation\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"Your production and development webhook URLs will be revalidated periodically every 72 hours. Zoom will send notification emails to the account owner associated with your apps if the URL fails the revalidation, following this schedule:\"}),`\n`,(0,e.jsxs)(n.ol,{children:[`\n`,(0,e.jsx)(n.li,{children:\"First notification email after a total of 2 consecutive failed revalidations.\"}),`\n`,(0,e.jsx)(n.li,{children:\"Second notification email after a total of 4 consecutive failed revalidations.\"}),`\n`]}),`\n`,(0,e.jsx)(n.p,{children:\"If the revalidation check fails 6 times in a row, Zoom will stop sending webhook events and disable the event subscription. To fix the issue to receive webhook events again, go to your apps webhook endpoint URL configuration, reenable webhooks, update your endpoint URL if needed, click validate to revalidate your endpoint URL, and finally, click save.\"}),`\n`,(0,e.jsxs)(n.p,{children:[\"If you are still having trouble, see the \",(0,e.jsx)(n.a,{href:\"https://github.com/zoom/webhook-sample-node.js/blob/master/index.js#L34\",children:\"Webhook sample app for an example implementation\"}),\" or review the \",(0,e.jsx)(n.a,{href:\"/docs/api/webhooks/#webhook-endpoint-url-requirements\",children:\"webhook endpoint URL requirements\"}),\".\"]}),`\n`,(0,e.jsx)(n.hr,{}),`\n`,(0,e.jsx)(n.h2,{id:\"configure-webhooks-via-zoom-api\",children:(0,e.jsxs)(n.a,{href:\"#configure-webhooks-via-zoom-api\",children:[\"Configure webhooks via Zoom API\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"For existing apps, you can use the Zoom API to create and manage event subscriptions. This allows you to programmatically:\"}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsx)(n.li,{children:\"Subscribe users or accounts to specific webhook events based on their preference, actions or regions.\"}),`\n`,(0,e.jsx)(n.li,{children:\"Change which events you are subscribed to without manually reconfiguring the app in the build flow.\"}),`\n`,(0,e.jsx)(n.li,{children:\"Easily manage webhook subscriptions per customer when building multi-tenant applications.\"}),`\n`]}),`\n`,(0,e.jsxs)(o,{children:[(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Note\"})}),(0,e.jsx)(n.p,{children:\"You can\\u2019t use the API method to modify event subscriptions created through the build flow.\"})]}),`\n`,(0,e.jsx)(n.p,{children:\"The following is an example of creating an event subscription via API for a general app type. You can use the same general steps for other subscription management actions:\"}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[(0,e.jsx)(n.strong,{children:\"Create an Event Subscription\"}),(0,e.jsx)(n.br,{}),`\n`,(0,e.jsx)(\"a\",{href:\"https://developers.zoom.us/docs/api/marketplace/#tag/app/post/marketplace/app/event_subscription\",children:(0,e.jsx)(n.code,{children:\"POST https://api.zoom.us/v2/marketplace/app/event_subscription\"})})]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[(0,e.jsx)(n.strong,{children:\"Subscribe to an Event Subscription\"}),(0,e.jsx)(n.br,{}),`\n`,(0,e.jsx)(\"a\",{href:\"https://developers.zoom.us/docs/api/marketplace/#tag/app/patch/marketplace/app/event_subscription/{eventSubscriptionId}\",children:(0,e.jsx)(n.code,{children:\"PATCH https://api.zoom.us/v2/marketplace/app/event_subscription/{eventSubscriptionId}\"})})]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[(0,e.jsx)(n.strong,{children:\"Unsubscribe from an Event Subscription\"}),(0,e.jsx)(n.br,{}),`\n`,(0,e.jsx)(\"a\",{href:\"https://developers.zoom.us/docs/api/marketplace/#tag/app/delete/marketplace/app/event_subscription\",children:(0,e.jsx)(n.code,{children:\"DELETE https://api.zoom.us/v2/marketplace/app/event_subscription\"})})]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[(0,e.jsx)(n.strong,{children:\"Delete an Event Subscription\"}),(0,e.jsx)(n.br,{}),`\n`,(0,e.jsx)(\"a\",{href:\"https://developers.zoom.us/docs/api/marketplace/#tag/app/delete/marketplace/app/event_subscription/{eventSubscriptionId}\",children:(0,e.jsx)(n.code,{children:\"DELETE https://api.zoom.us/v2/marketplace/app/event_subscription/{eventSubscriptionId}\"})})]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[(0,e.jsx)(n.strong,{children:\"Get User or Account Event Subscriptions\"}),(0,e.jsx)(n.br,{}),`\n`,(0,e.jsx)(\"a\",{href:\"https://developers.zoom.us/docs/api/marketplace/#tag/app/get/marketplace/app/event_subscription\",children:(0,e.jsx)(n.code,{children:\"GET https://api.zoom.us/v2/marketplace/app/event_subscription\"})})]}),`\n`]}),`\n`]}),`\n`,(0,e.jsx)(n.h3,{id:\"prerequisite\",children:(0,e.jsxs)(n.a,{href:\"#prerequisite\",children:[\"Prerequisite:\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsx)(n.li,{children:\"You have an existing app.\"}),`\n`,(0,e.jsxs)(n.li,{children:[\"In your app, you have already selected the scopes required by your event subscriptions. See \",(0,e.jsx)(n.a,{href:\"#scopes\",children:\"Scopes\"}),\".\"]}),`\n`]}),`\n`,(0,e.jsx)(n.h3,{id:\"step-1-request-an-access-token\",children:(0,e.jsxs)(n.a,{href:\"#step-1-request-an-access-token\",children:[\"Step 1: Request an access token\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.p,{children:[(0,e.jsx)(n.a,{href:\"/docs/integrations/oauth/#request-an-access-token\",children:\"Request an access token\"}),\" to call APIs for your own Zoom account by sending a POST request to \",(0,e.jsx)(n.code,{children:\"https://zoom.us/oauth/token\"}),\".\"]}),`\n`,(0,e.jsx)(n.p,{children:\"Include a basic authorization header. The basic authorization header is your Client ID and Client Secret with a colon : in between, Base64 Encoded.\"}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-bash\",children:`-- request POST https://zoom.us/oauth/token?grant_type=client_credentials\n-- header \"Authorization\": \"Basic <YOUR_BASE64_ENCODED_CREDENTIALS>\" \n`})}),`\n`,(0,e.jsxs)(o,{variant:\"warning\",children:[(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Note\"})}),(0,e.jsx)(n.p,{children:\"Webhook subscriptions are app-owned resources; you need to use the grant type appropriate to the app type to obtain the access token.\"}),(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsxs)(n.li,{children:[\"For General OAuth apps, use the \",(0,e.jsx)(n.a,{href:\"/docs/integrations/oauth/#request-an-access-token\",children:\"client credentials\"}),\" grant type.\"]}),`\n`,(0,e.jsxs)(n.li,{children:[\"For Server-to-Server OAuth apps, use the \",(0,e.jsx)(n.a,{href:\"/docs/integrations/oauth/#request-an-access-token\",children:\"account credentials\"}),\" grant type.\"]}),`\n`]})]}),`\n`,(0,e.jsxs)(n.p,{children:[\"This returns an \",(0,e.jsx)(n.code,{children:\"access_token\"}),\" with the required scopes to manage webhook subscriptions. Use this token as a bearer token in the authorization header of subsequent API calls.\"]}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n    \"access_token\": \"eyJzyNzA2ZDlkMzk0MDgzTk0MzAzMjUsImlzcy...A\",\n    \"token_type\": \"bearer\",\n    \"expires_in\": 3600,\n    \"scope\": \"marketplace:delete:event_subscription marketplace:read:list_event_subscriptions marketplace:update:client_secret marketplace:update:event_subscription marketplace:write:event_subscription marketplace:write:websocket_connection\",\n    \"api_url\": \"https://api.zoom.us\"\n}\n\n`})}),`\n`,(0,e.jsx)(n.h3,{id:\"step-2-create-an-event-subscription\",children:(0,e.jsxs)(n.a,{href:\"#step-2-create-an-event-subscription\",children:[\"Step 2: Create an event subscription\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.p,{children:[\"Use the POST method for the \",(0,e.jsx)(n.code,{children:\"event_subscription\"}),\" endpoint.\"]}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-bash\",children:`POST https://api.zoom.us/v2/marketplace/app/event_subscription \n`})}),`\n`,(0,e.jsx)(n.p,{children:\"Include the authorization in the request header.\"}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-bash\",children:`{ \"Authorization\": \"Bearer <access_token>\" }\n`})}),`\n`,(0,e.jsx)(n.p,{children:\"Example request body:\"}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n  \"events\": [\n    \"meeting.created\"\n  ],\n  \"event_subscription_name\": \"Example Event Subscription\",\n  \"event_webhook_url\": \"https://www.example.com\",\n  \"user_ids\": [\n    \"_8KG7DeoRU2xIsDSY9ed2Q,90KG7DeoRU2xIsDSY9edwe\"\n  ],\n  \"subscription_scope\": \"user\",\n  \"account_id\": \"pvg3UAgpRlyTDW-9sIpKcw\"\n}\n\n`})}),`\n`,(0,e.jsx)(n.p,{children:\"On successful creation, Zoom provides an event_subscription_id.\"}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n  \"event_subscription_id\": \"0ZAaJY4dQ52BbwI9PArBLQ\"\n}\n`})}),`\n`,(0,e.jsx)(n.p,{children:\"With this event_subscription_id you can then subscribe to, unsubscribe from, or delete event subscriptions.\"}),`\n`,(0,e.jsxs)(n.p,{children:[\"For more information, see the \",(0,e.jsx)(n.a,{href:\"/blog/webhook-management-with-zoom-api/\",children:\"Developer blog\"}),\".\"]}),`\n`,(0,e.jsx)(n.h2,{id:\"scopes\",children:(0,e.jsxs)(n.a,{href:\"#scopes\",children:[\"Scopes\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"Scopes control your app's access to the Zoom platform including the event information requested by your subscriptions.\"}),`\n`,(0,e.jsx)(n.p,{children:\"Zoom has automatically applied the scopes for event subscriptions to your app: (such as creating, updating, or deleting subscriptions). For any other events you want your app to receive, you must add the respective scopes to your app via the app build flow.\"}),`\n`,(0,e.jsx)(n.p,{children:\"For example:\"}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsxs)(n.li,{children:[\"To subscribe to the \",(0,e.jsx)(n.code,{children:\"meeting.participant_joined\"}),\" event, your app must request the \",(0,e.jsx)(n.code,{children:\"meeting:read:participant\"}),\" or \",(0,e.jsx)(n.code,{children:\"meeting:read:participant:admin\"}),\".\"]}),`\n`,(0,e.jsxs)(n.li,{children:[\"To receive \",(0,e.jsx)(n.code,{children:\"recording.completed\"}),\", you'll need \",(0,e.jsx)(n.code,{children:\"cloud_recording:read:recording\"}),\" or \",(0,e.jsx)(n.code,{children:\"cloud_recording:read:recording:admin\"}),\".\"]}),`\n`]}),`\n`,(0,e.jsx)(n.p,{children:`If you haven't added the appropriate scopes to the app, on using the event subscription endpoints you will receive an error. \"The event is not allowed for the app.\"`}),`\n`,(0,e.jsxs)(n.p,{children:[\"For more information, see \",(0,e.jsx)(n.a,{href:\"/docs/build-flow/create-oauth-apps/#step-4-select-the-scopes-zoom-api-methods\",children:\"Selecting scope\"}),\".\"]}),`\n`,(0,e.jsx)(n.hr,{}),`\n`,(0,e.jsx)(n.h2,{id:\"notification-delivery\",children:(0,e.jsxs)(n.a,{href:\"#notification-delivery\",children:[\"Notification delivery\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.p,{children:[\"Your webhook endpoint URL should respond with either a \",(0,e.jsx)(n.code,{children:\"200\"}),\" or a \",(0,e.jsx)(n.code,{children:\"204\"}),\" HTTP status code within \",(0,e.jsx)(n.strong,{children:\"three seconds\"}),\" after receiving a webhook request in order for Zoom to consider the notification was successfully delivered.\"]}),`\n`,(0,e.jsx)(n.h3,{id:\"unsuccessful-delivery\",children:(0,e.jsxs)(n.a,{href:\"#unsuccessful-delivery\",children:[\"Unsuccessful delivery\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.p,{children:[\"Zoom attempts to deliver the webhook \",(0,e.jsx)(n.strong,{children:\"three times\"}),\" for response codes >=\",(0,e.jsx)(n.code,{children:\"500\"}),\" or codes in \",(0,e.jsx)(n.code,{children:\"-1\"}),\", \",(0,e.jsx)(n.code,{children:\"-2\"}),\", \",(0,e.jsx)(n.code,{children:\"-3\"}),\", \",(0,e.jsx)(n.code,{children:\"-4\"}),\", \",(0,e.jsx)(n.code,{children:\"-5\"}),\", \",(0,e.jsx)(n.code,{children:\"-7\"}),\", \",(0,e.jsx)(n.code,{children:\"-8\"}),\", \",(0,e.jsx)(n.code,{children:\"-10\"}),\", \",(0,e.jsx)(n.code,{children:\"-13\"}),\", \",(0,e.jsx)(n.code,{children:\"-14\"}),\", \",(0,e.jsx)(n.code,{children:\"-15\"}),\". (See \",(0,e.jsx)(n.strong,{children:\"Enum Descriptions\"}),\" below.)\"]}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsxs)(n.li,{children:[\"The first attempt is sent \",(0,e.jsx)(n.strong,{children:\"5 minutes\"}),\" after the initial delivery attempt.\"]}),`\n`,(0,e.jsxs)(n.li,{children:[\"A second attempt is sent \",(0,e.jsx)(n.strong,{children:\"20 minutes\"}),\" after the first attempt.\"]}),`\n`,(0,e.jsxs)(n.li,{children:[\"A third attempt is sent \",(0,e.jsx)(n.strong,{children:\"60 minutes\"}),\" after the second attempt.\"]}),`\n`]}),`\n`,(0,e.jsxs)(n.p,{children:[\"Zoom considers response codes >=\",(0,e.jsx)(n.code,{children:\"200\"}),\" or < \",(0,e.jsx)(n.code,{children:\"300\"}),\" as successfully delivered after any retry attempt.\"]}),`\n`,(0,e.jsxs)(n.p,{children:[\"If Zoom does \",(0,e.jsx)(n.strong,{children:\"not\"}),\" receive a response code >=\",(0,e.jsx)(n.code,{children:\"200\"}),\" or < \",(0,e.jsx)(n.code,{children:\"300\"}),\" after three attempts, then Zoom will send no further webhooks for that event.\"]}),`\n`,(0,e.jsxs)(n.p,{children:[\"Zoom does not retry redirect codes >=\",(0,e.jsx)(n.code,{children:\"300\"}),\" and < \",(0,e.jsx)(n.code,{children:\"400\"}),\".\",(0,e.jsx)(n.br,{}),`\n`,\"Zoom does not retry client error codes >=\",(0,e.jsx)(n.code,{children:\"400\"}),\" and < \",(0,e.jsx)(n.code,{children:\"500\"}),\", such as unauthenticated, resource not found, rate limiting, etc., because retrying would still fail or further increase the server load.\"]}),`\n`,(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Enum Descriptions\"})}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.code,{children:\"ReadTimeOut(-1)\"}),\" Socket time out.\"]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.code,{children:\"ConnectionRefused(-2)\"}),\" Connection refused.\"]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.code,{children:\"Fallback(-3)\"}),\" Hystrix fallback on error.\"]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.code,{children:\"Resilience4jTimeout(-4)\"}),\" Resilience time out.\"]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.code,{children:\"AsyncError(-5)\"}),\" Failed to execute job via async mode.\"]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.code,{children:\"RemoteError(-7)\"}),\" Remote error.\"]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.code,{children:\"Delegate(-8)\"}),\" Delegate tasks to a separate server for processing if in delegate white list.\"]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.code,{children:\"NeedInstantRetry(-10)\"}),\" Retry webhook invoke instantly when connection lease request times out.\"]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.code,{children:\"DNSResolveTimeout(-13)\"}),\" Zoom DNS resolve timeout exception.\"]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.code,{children:\"CanNotGetAccessToken(-14)\"}),\" Delay retry error code if developer enabled Oauth client credential header in the subscription, but Zoom cannot get access_token from tokenUrl by clientId & clientSecret.\"]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.code,{children:\"CanNotGetCustomHeaderSettingInNoCacheTopic(-15)\"}),\" Delay retry error code if developer enabled custom header in the subscription, but Zoom cannot get custom header setting when handling msg from no cache topic.\"]}),`\n`]}),`\n`,(0,e.jsxs)(o,{children:[(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Troubleshooting missing webhook events\"})}),(0,e.jsx)(n.p,{children:\"If you have successfully validated your webhook endpoint, but you are not receiving certain events you have selected, toggle the webhook feature on and off and try again.\"})]}),`\n`,(0,e.jsxs)(n.p,{children:[\"Use \",(0,e.jsx)(n.a,{href:\"/docs/api/marketplace/#tag/app/get/marketplace/apps/{appId}/webhook_logs\",children:\"Get webhook logs\\u200B\"}),\" to get a list of webhooks sent to an app.\"]}),`\n`,(0,e.jsx)(n.h3,{id:\"delivery-latency\",children:(0,e.jsxs)(n.a,{href:\"#delivery-latency\",children:[\"Delivery latency\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.p,{children:[\"The \",(0,e.jsx)(n.code,{children:\"event_ts\"}),\" response is included in the payload of all event notifications. This represents the timestamp for when the associated event occurred.\"]}),`\n`,(0,e.jsxs)(n.p,{children:[\"You can determine the latency in webhook delivery by calculating the difference between the delivery timestamp and the value of the \",(0,e.jsx)(n.code,{children:\"event_ts\"}),\".\"]})]})}function g(i={}){let{wrapper:n}={...d(),...i.components};return n?(0,e.jsx)(n,{...i,children:(0,e.jsx)(f,{...i})}):f(i)}function c(i,n){throw new Error(\"Expected \"+(n?\"component\":\"object\")+\" `\"+i+\"` to be defined: you likely forgot to import, pass, or provide it.\")}return x(q);})();\n;return Component;",
      "frontmatter": {
        "title": "Using webhooks",
        "resources": [
          {
            "title": "Zoom Platform 101 Zoom Webhooks",
            "url": "https://youtu.be/oAnr1ezW8dY?si=AVlihxis-DIS4L5W"
          },
          {
            "title": "Server-to-Server apps",
            "url": "https://developers.zoom.us/docs/internal-apps/"
          },
          {
            "title": "General OAuth apps",
            "url": "https://developers.zoom.us/docs/integrations/"
          },
          {
            "title": "Build platform apps",
            "url": "https://developers.zoom.us/docs/build/"
          },
          {
            "title": "Updating an app",
            "url": "https://developers.zoom.us/docs/distribute/published-apps/updating/"
          }
        ]
      },
      "errors": [],
      "matter": {
        "content": "\n# Using webhooks\n\nInstead of making repeated calls to pull data frequently from the Zoom API, you can use webhooks for Zoom to send events to your server. Zoom sends webhook events as HTTP POST requests with a JSON body to your app's specified endpoint. \n\nFor an introduction and explanation of webhooks, see [Zoom Platform 101: Zoom Webhooks](https://youtu.be/oAnr1ezW8dY?si=AVlihxis-DIS4L5W).  \n\n<Alert>\n\n**Now available**\n\n[Zoom Rivet](/docs/rivet/javascript/get-started/#initialize-the-client), Zoom's API library that handles webhooks for you.\n\n</Alert>\n\n## Workplace\n\n<div class=\"api_ref_tile_container\">\n<Link class=\"api_ref_tile\" href=\"/docs/api/meetings/events\">\n  <SvgIcon svgname=\"_meetings\" />\n  <h3>Meetings</h3>\n</Link>\n<Link className=\"api_ref_tile\" href=\"/docs/api/rtms/events\">\n    <SvgIcon svgname=\"_rtms\" />\n    <h3>RTMS</h3>\n  </Link>\n<Link class=\"api_ref_tile\" href=\"/docs/api/team-chat/events\">\n  <SvgIcon svgname=\"_chat\" />\n  <h3>Team Chat</h3>\n</Link>\n<Link class=\"api_ref_tile\" href=\"/docs/api/phone/events\">\n  <SvgIcon svgname=\"_zoomphone\" />\n  <h3>Phone</h3>\n</Link>\n<Link class=\"api_ref_tile\" href=\"/docs/api/mail/events\">\n  <SvgIcon svgname=\"_mail\" />\n  <h3>Mail</h3>\n</Link>\n<Link class=\"api_ref_tile\" href=\"/docs/api/calendar/events\">\n  <SvgIcon svgname=\"_calendar\" />\n  <h3>Calendar</h3>\n</Link>\n<Link class=\"api_ref_tile\" href=\"/docs/api/rooms/events\">\n  <SvgIcon svgname=\"_zoomrooms\" />\n  <h3>Rooms</h3>\n</Link>\n<Link class=\"api_ref_tile\" href=\"/docs/api/whiteboard/events\">\n  <SvgIcon svgname=\"_whiteboard\" />\n  <h3>Whiteboard</h3>\n</Link>\n<Link class=\"api_ref_tile\" href=\"/docs/api/chatbot/events\">\n  <SvgIcon svgname=\"_chat\" />\n  <h3>Chatbot</h3>\n</Link>\n<Link class=\"api_ref_tile\" href=\"/docs/api/scheduler/events\">\n  <SvgIcon svgname=\"_zoomscheduler\" />\n  <h3>Scheduler</h3>\n</Link>\n\n</div>\n\n## Business services\n\n<div class=\"api_ref_tile_container\">\n  <Link class=\"api_ref_tile\" href=\"/docs/api/contact-center/events\">\n    <SvgIcon svgname=\"contactCenter\" />\n    <h3>Contact Center</h3>\n  </Link>\n  <Link class=\"api_ref_tile\" href=\"/docs/api/events/events\">\n    <SvgIcon svgname=\"_zoomevents\" />\n    <h3>Events</h3>\n  </Link>\n  <Link class=\"api_ref_tile\" href=\"/docs/api/iq/events\">\n    <SvgIcon svgname=\"zra\" />\n    <h3>Revenue Accelerator</h3>\n  </Link>\n  <Link class=\"api_ref_tile\" href=\"/docs/api/number-management/events\">\n  <SvgIcon svgname=\"number-management\" />\n  <h3>Number Management</h3>\n</Link>\n      <Link class=\"api_ref_tile\" href=\"/docs/api/node/events\">\n  <SvgIcon svgname=\"zoom-node\" />\n    <h3>Node</h3>\n  </Link>\n        <Link class=\"api_ref_tile\" href=\"/docs/api/quality-management/events\">\n  <SvgIcon svgname=\"qualityManagement\" />\n    <h3>Quality Management</h3>\n  </Link>\n  <Link className=\"api_ref_tile\" href=\"/docs/api/healthcare/events\">\n    <SvgIcon svgname=\"clinical-notes\" />\n    <h3>Healthcare</h3>\n  </Link>\n  <Link className=\"api_ref_tile\" href=\"/docs/api/video-management/events\">\n    <SvgIcon svgname=\"video-management\" />\n    <h3>Video Management</h3>\n  </Link>\n</div>\n\n## Accounts\n\n<div class=\"api_ref_tile_container\">\n  <Link class=\"api_ref_tile\" href=\"/docs/api/users/events\">\n    <SvgIcon svgname=\"users\" />\n    <h3>Users</h3>\n  </Link>\n  <Link class=\"api_ref_tile\" href=\"/docs/api/accounts/events\">\n    <SvgIcon svgname=\"accounts\" />\n    <h3>Accounts</h3>\n  </Link>\n  <Link class=\"api_ref_tile\" href=\"/docs/api/qss/events\">\n    <SvgIcon svgname=\"qss\" />\n    <h3>QSS</h3>\n  </Link>\n</div>\n\n## Build platform\n\n<div class=\"api_ref_tile_container\">\n  <Link class=\"api_ref_tile\" href=\"/docs/api/video-sdk/events\">\n    <SvgIcon svgname=\"_video-sdks\" />\n    <h3>Video SDK</h3>\n  </Link>\n    <Link class=\"api_ref_tile\" href=\"/docs/api/cobrowse-sdk/events\">\n    <SvgIcon svgname=\"cobrowse-sdk\" />\n    <h3>Cobrowse SDK</h3>\n  </Link>\n</div>\n\n## Marketplace\n\n<div class=\"api_ref_tile_container\">\n  <Link class=\"api_ref_tile\" href=\"/docs/api/marketplace/events\">\n    <SvgIcon svgname=\"marketplace\" />\n    <h3>Apps</h3>\n  </Link>\n</div>\n\n## Configure webhooks via build flow  \nYou configure webhooks by subscribing to specific events in the app build flow. \n\nFor instructions to configure your webhook subscriptions using Zoom APIs, see [Configure webhooks via Zoom API](/#Configure-webhooks-via-Zoom-API).    \n\n1. Go to your app's **Access** page.   \n  a. Login to the [Zoom App Marketplace](https://marketplace.zoom.us/) and click **Manage**.    \n  b. On the **Created Apps** page, open your app, and go to the **Features** \u2192 **Access**.   \n2. Under the **General Features** section, enable **Event Subscriptions** and **Add New Event Subscription**.  \n\n    <Image src=\"/img/add-new-subscription2.png\" width={500} /> \n\n    NOTE: You can subscribe to as many events as needed for each event subscription, up to a maximum of 20 event subscriptions per app. Event subscriptions can also have duplicate events. For example, one event subscription could have *Meetings* and *User Events*, and a second event subscription can have *Meetings* and *Recordings* events.   \n3. On the **Event Types** screen, select the specific events and then **Done**.  \n    <Image src=\"/img/add-events.png\" width={500} />  \n4. Set your **Event notification endpoint URL** for each event subscription in your app and **Save**. Be sure that it satisfies the [webhook endpoint requirements](#webhook-endpoint-url-requirements).  \n     <Image src=\"/img/save-subscription.png\" width={500} />  \n\n\n### Webhook endpoint URL requirements\n\nTo receive webhooks in development and production environments, the **Event notification endpoint URL** that you specify for each event subscription must:\n\n- Be a publicly accessible `https` endpoint url that supports TLSv1.2+ with a valid certificate chain issued by a [Certificate Authority (CA)](https://en.wikipedia.org/wiki/Certificate_authority).\n- Be a fully qualified domain name (FQDN).\n- Be able to accept HTTP POST requests containing JSON payloads.\n- Be able to respond with a `200` or `204` HTTP Status Code.\n\n### Webhook structure\n\nWebhooks events are POST requests sent to your endpoint URL and contain request headers and a request body of the event type, event timestamp, and event details within the payload object.\n\n**Example request headers:**\n\n```json\n{\n  \"host\": \"example.com\",\n  \"user-agent\": \"Zoom Marketplace/1.0a\",\n  \"content-length\": \"110\",\n  \"authorization\": \"Wk9PTV9BVVRIT1JJWkFUSU9O\",\n  \"content-type\": \"application/json; charset=utf-8\",\n  \"traceparent\": \"Wk9PTV9UUkFDRVBBUkVOVA\",\n  \"x-forwarded-for\": \"{X_FORWARDED_FOR}\",\n  \"x-forwarded-host\": \"example.com\",\n  \"x-forwarded-proto\": \"https\",\n  \"x-zm-request-timestamp\": \"1739923528\",\n  \"x-zm-signature\": \"v0=WF9aT09NX1NJR05BVFVSRQ\",\n  \"x-zm-request-id\": \"6009d653_d487_445d_8406_42b654974899\",\n  \"accept-encoding\": \"gzip\"\n}\n```\n\n**Example request body:**\n\n```json\n{\n  \"event\": \"meeting.started\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"Wk9PTV9BQ0NPVU5UX0lE\",\n    \"object\": {\n      \"id\": \"1234567890\",\n      \"uuid\": \"Wk9PTV9NRUVUSU5HX0lE\",\n      \"host_id\": \"Wk9PTV9VU0VSX0lE\",\n      \"topic\": \"My Meeting\",\n      \"type\": 8,\n      \"start_time\": \"2021-07-13T21:44:51Z\",\n      \"timezone\": \"America/Los_Angeles\",\n      \"duration\": 60\n    }\n  }\n}\n```\n\n### Zoom IP addresses\n\nWe **strongly** recommend that you [Verify webhook events](#verify-webhook-events) instead of creating an allow list of Zoom IP addresses because Zoom may update the [IP ranges](https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0060548) used at any time.\n\n## Verify webhook events\n\nZoom offers two ways to verify the authenticity of a webhook, to ensure developers that the request came from Zoom:  \n\n\n<Tabs>\n  <Tab eventKey=\"verifyWithZoomsHeader\" title=\"Verify with Zoom's header\">\n\n\n### Verify with Zoom's header\n\nZoom includes a webhook secret token on the **Add Feature** page for your app. Zoom uses the value of the secret token to hash the webhook data, which it sends in the `x-zm-signature` webhook request header.\n\nTo verify that Zoom sent a webhook request, use the webhook secret token and the webhook request body to create a signature to compare with the `x-zm-signature` header value sent by Zoom.\n\n**Example request headers:**  \n\n```json  \n{\n  \"host\": \"example.com\",\n  \"user-agent\": \"Zoom Marketplace/1.0a\",\n  \"content-length\": \"110\",\n  \"authorization\": \"Wk9PTV9BVVRIT1JJWkFUSU9O\",\n  \"content-type\": \"application/json; charset=utf-8\",\n  \"traceparent\": \"Wk9PTV9UUkFDRVBBUkVOVA\",\n  \"x-forwarded-for\": \"{X_FORWARDED_FOR}\",\n  \"x-forwarded-host\": \"example.com\",\n  \"x-forwarded-proto\": \"https\",\n  \"x-zm-request-timestamp\": \"1739923528\",\n  \"x-zm-signature\": \"v0=WF9aT09NX1NJR05BVFVSRQ\",\n  \"accept-encoding\": \"gzip\"\n}\n```\n\n**Create your signature string** to compare with the Zoom webhook `x-zm-signature` header value:\n\n1. Receive the webhook request.  \n\n   The webhook request body includes the following properties:\n\n   Key | Value Type | Value Description\n   --- | --- | ---\n   `event` | string |The type of the webhook event\n   `payload` | object | Contains the event webhook details\n   `event_ts` | number | The timestamp of the webhook event.\n\n   **Example request body:**  \n\n   ```json\n   {\n     \"event\": \"session.started\",\n     \"payload\": {\n       \"account_id\": \"{ACCOUNT_ID}\",\n       \"object\": {\n         \"start_time\": \"2022-07-27T16:56:34Z\",\n         \"id\":\"{SESSION_ID}\"\n       }\n     },\n     \"event_ts\": 1658940994914\n   }\n   ```\n\n2. Construct the message.  \n\n   After receiving a webhook request, construct the message string with `\"v0\"`, the webhook request header `x-zm-request-timestamp` value, and the webhook request body. Separate each section with a colon `:`. For example:\n\n   `v0:{WEBHOOK_REQUEST_HEADER_X-ZM-REQUEST-TIMESTAMP_VALUE}:{WEBHOOK_REQUEST_BODY}`\n\n   Example string with printed values:\n\n   ```\n   v0:1658940994:{\"event\":\"session.started\",\"payload\":{\"account_id\":\"{ACCOUNT_ID}\",\"object\":{\"start_time\":\"2022-07-27T16:56:34Z\",\"id\":\"{SESSION_ID}\"}},\"event_ts\":1658940994914}\n   ```\n\n3. Hash the message.  \n  \n   Once you have constructed the message string, create a [HMAC SHA-256](https://datatracker.ietf.org/doc/html/rfc4634) hash. Set your webhook's secret token as the secret/salt, and the `message` you constructed as the string to hash. Output in `hex` format.\n\n4. Create the signature.  \n  \n   Then, create the signature by prepending `v0=` to the hashed message. For example:\n\n   `v0={HASHED_MESSAGE}`\n\n   Example string with printed values:\n\n   ```\n   v0=a05d830fa017433bc47887f835a00b9ff33d3882f22f63a2986a8es270341\n   ```\n\n5. Compare the signature.  \n  \n   Compare the `signature` you created with the Zoom webhook request header `x-zm-signature` value. If it matches, the webhook request came from Zoom.\n\n   **Here is an example in Node.js:**\n\n   ```js\n   const crypto = require('crypto')\n\n   const message = `v0:${request.headers['x-zm-request-timestamp']}:${JSON.stringify(request.body)}`\n\n   const hashForVerify = crypto.createHmac('sha256', ZOOM_WEBHOOK_SECRET_TOKEN).update(message).digest('hex')\n\n   const signature = `v0=${hashForVerify}`\n\n   if (request.headers['x-zm-signature'] === signature) {\n     // Webhook request came from Zoom\n   } else {\n     // Webhook request did not come from Zoom\n   }\n   ```\n\n   See the [Webhook sample app for an example implementation](https://github.com/zoom/webhook-sample-node.js/blob/master/index.js#L31).\n\nLearn more about [HMAC Auth for webhooks](https://webhooks.fyi/security/hmac).\n\n  </Tab>\n  <Tab eventKey=\"verifyWithYourHeader\" title=\"Verify with your own header\">\n\n\n### Verify with your own header\n\nZoom offers the following options to verify webhooks with your own header:\n\n- [Basic Authentication](#basic-authentication)\n- [Token Authentication (OAuth)](#token-authentication-oauth)  \n- [Custom Header](#custom-header)\n\n#### Basic authentication\n\n1. Choose **Basic Authentication**. Specify a username and password.\n\n   <Image src=\"/img/webhooks_basic_auth.jpg\" alt=\"Zoom webhook basic auth\" width={500} />\n\n2. When a webhook is triggered, Zoom makes a POST request with the following request headers, including the `authorization` value being your username and password, base64encoded with a colon `:` in between:\n\n   ```json\n   {\n     \"host\": \"example.com\",\n     \"user-agent\": \"Zoom Marketplace/1.0a\",\n     \"content-length\": \"320\",\n     \"authorization\": \"Basic Q2xpZW50SUQ6Q2xpZW50U2VjcmV0\",\n     \"clientid\": \"{CLIENT_ID}\",\n     \"content-type\": \"application/json; charset-utf-8\",\n     \"x-forwarded-for\": \"{X_FORWARDED_FOR}\",\n     \"x-forwarded-proto\": \"https\",\n     \"x-zm-request-timestamp\": \"1674780079\",\n     \"x-zm-signature\": \"v0={HASHED_WEBHOOK_SECRET_TOKEN}\",\n     \"x-zm-trackingid\": \"{X_ZM_TRACKINGID}\",\n     \"accept-encoding\": \"gzip\"\n   }\n   ```\n\nLearn more about [Basic Auth for webhooks](https://webhooks.fyi/security/shared-secret).\n\n#### Token authentication (OAuth)  \n\n1. Choose the \"Token Authentication\" option. Specify an OAuth token endpoint, and your OAuth server's Client ID and Client Secret.\n\n   `https://example.com/oauth/token?grant_type=client_credentials`\n\n   <Image src=\"/img/webhooks_token_auth.jpg\" alt=\"Zoom webhook oauth\" width={500} />\n\n2. When a webhook is triggered, Zoom makes a POST request to retrieve an access_token from your OAuth server, with the following request headers, including the  `authorization` value being your Client ID and Client Secret, base64encoded, with a colon `:` in between:\n\n   ```json\n   {\n     \"host\": \"example.com\",\n     \"user-agent\": \"Zoom Marketplace/1.0a\",\n     \"content-length\": \"0\",\n     \"authorization\": \"Basic Q2xpZW50SUQ6Q2xpZW50U2VjcmV0\",\n     \"content-type\": \"application/x-www-form-urlencoded\",\n     \"x-forwarded-for\": \"{X_FORWARDED_FOR}\",\n     \"x-forwarded-proto\": \"https\",\n     \"accept-encoding\": \"gzip\"\n   }\n   ```\n\n3. After you validate the authorization header, respond to the request with a `200` status code and the following JSON. **expires_in** must be equal to or greater than 3599.\n\n   ```json\n   {\n     \"access_token\": \"{ACCESS_TOKEN}\",\n     \"token_type\": \"bearer\",\n     \"expires_in\": \"3599\"\n   }\n   ```\n\n4. Zoom sends the webhook request to your server with the authorization bearer header set to the value of your access_token:\n\n   ```json\n   {\n     \"host\": \"example.com\",\n     \"user-agent\": \"Zoom Marketplace/1.0a\",\n     \"content-length\": \"110\",\n     \"authorization\": \"Bearer {ACCESS_TOKEN}\",\n     \"content-type\": \"application/json; charset=utf-8\",\n     \"x-forwarded-for\": \"{X_FORWARDED_FOR}\",\n     \"x-forwarded-proto\": \"https\",\n     \"x-zm-request-timestamp\": \"1674847087\",\n     \"x-zm-signature\": \"v0={HASHED_WEBHOOK_SECRET_TOKEN}\",\n     \"accept-encoding\": \"gzip\"\n   }\n   ```\n\nLearn more about [OAuth for webhooks](https://webhooks.fyi/security/jwt-jwk-oauth2).\n\n#### Custom header\n\n1. Choose **Custom Header**. Specify a key and value.\n\n   <Image src=\"/img/webhook-custom-header.png\" alt=\"Zoom webhook custom header\" width={500} />\n\n2. When an event triggers a webhook, Zoom makes a POST request with the following request headers, including your custom header property (in this example `x-my-custom-key`):\n\n   ```json\n   {\n     \"host\": \"example.com\",\n     \"user-agent\": \"Zoom Marketplace/1.0a\",\n     \"content-length\": \"110\",\n     \"content-type\": \"application/json; charset=utf-8\",\n     \"x-forwarded-for\": \"{X_FORWARDED_FOR}\",\n     \"x-forwarded-proto\": \"https\",\n     \"x-my-custom-key\": \"my-custom-value\",\n     \"x-zm-request-timestamp\": \"1674847087\",\n     \"x-zm-signature\": \"v0={HASHED_WEBHOOK_SECRET_TOKEN}\",\n     \"accept-encoding\": \"gzip\"\n   }\n   ```\n\n   <Alert>\n\n   **Removing the x- Prefix in custom header keys**\n\n   If you want to remove the `x-` prefix in the custom header key, please reach out to [Zoom Support](https://support.zoom.us) or post on our [Developer Forum](https://devforum.zoom.us/).\n   \n   </Alert>\n\nLearn more about [One Time Verification for webhooks](https://webhooks.fyi/security/one-time-verification-challenge).\n\n  </Tab>\n</Tabs>\n\n## Validate your webhook endpoint\n\nZoom requires that you manually trigger webhook validation when you add a new webhook or make changes to an existing one. Subsequently, Zoom  automatically revalidates webhooks every 72 hours.\n\nZoom uses a challenge-response check (CRC) for webhook validation. When a CRC occurs, Zoom makes a POST request to your endpoint with a challenge request body. After your endpoint receives the request, your app needs to respond with the challenge response within 3 seconds.\n\nTo trigger the initial CRC validation, click **Validate** under the **Event Notification Endpoint URL** on the **Feature** page for your app. See [Revalidation](#revalidation) for revalidation details.\n\n<Image src=\"/img/webhook-validate.png\" alt=\"Zoom webhook validate\" width={500}/>\n\n<Alert>\n\n**Note**\n\nYou won't be able to save your changes until after Zoom validates your endpoint.\n\n</Alert>\n\n### Implement the challenge-response check flow\n\n1. Receive the challenge (webhook request body).\n\n   The webhook request body includes the following properties:\n\n   Key | Value Type | Value Description\n   --- | --- | ---\n   `event` | string |The type of the webhook event, which will be `\"endpoint.url_validation\"`\n   `payload` | object | Contains a property with the `plainToken` value, the string to hash.\n   `event_ts` | number | The timestamp of the webhook event.\n\n  \n   **Example request headers:**  \n   ```json  \n   {\n     \"host\": \"example.com\",\n     \"user-agent\": \"Zoom Marketplace/1.0a\",\n     \"content-length\": \"110\",\n     \"authorization\": \"Wk9PTV9BVVRIT1JJWkFUSU9O\",\n     \"content-type\": \"application/json; charset=utf-8\",\n     \"traceparent\": \"Wk9PTV9UUkFDRVBBUkVOVA\",\n     \"x-forwarded-for\": \"{X_FORWARDED_FOR}\",\n     \"x-forwarded-host\": \"example.com\",\n     \"x-forwarded-proto\": \"https\",\n     \"x-zm-request-timestamp\": \"1739923528\",\n     \"x-zm-signature\": \"v0=WF9aT09NX1NJR05BVFVSRQ\",\n     \"accept-encoding\": \"gzip\"\n   }\n   ```  \n   \n   **Example request body:**  \n\n   ```json\n   {\n     \"payload\": {\n       \"plainToken\": \"qgg8vlvZRS6UYooatFL8Aw\"\n     },\n     \"event_ts\": 1654503849680,\n     \"event\": \"endpoint.url_validation\"\n   }\n   ```\n\n2. Hash the plainToken.\n\n   Once you receive the request body, create a [HMAC SHA-256](https://datatracker.ietf.org/doc/html/rfc4634) hash. Set your webhook's secret token as the secret (salt), and the `plainToken` value as the string to hash. Output in `hex` format.\n\n3. Create the response JSON object.\n  \n   Create a JSON object with a key of `\"plainToken\"` with a value of the `plainToken` from the request body, and a key of `\"encryptedToken\"`  with a value of the hashed `plainToken`. For example:\n\n   ```json\n   {\n     \"plainToken\": \"qgg8vlvZRS6UYooatFL8Aw\",\n     \"encryptedToken\": \"23a89b634c017e5364a1c8d9c8ea909b60dd5599e2bb04bb1558d9c3a121faa5\"\n   }\n   ```\n\n4. Respond with the response JSON object.\n\n   Respond with the response JSON within 3 seconds with a `200` or `204` HTTP response code.\n\n   When Zoom successfully validates the challenge response from your webhook endpoint URL, you'll see a validated message under the **Event Notification Endpoint URL**. If validation fails, you'll see a failed to validate message. Once you successfully complete validation, click **Save**.\n\n   <Image src=\"/img/webhook-validate-success.png\" alt=\"Zoom webhook validate success\" width={500} />\n\n   **Here is an example in Node.js:**\n\n   ```js\n   const crypto = require('crypto')\n\n   // Webhook request event type is a challenge-response check\n   if(request.body.event === 'endpoint.url_validation') {\n     const hashForValidate = crypto.createHmac('sha256', ZOOM_WEBHOOK_SECRET_TOKEN).update(request.body.payload.plainToken).digest('hex')\n\n     response.status(200)\n     response.json({\n       \"plainToken\": request.body.payload.plainToken,\n       \"encryptedToken\": hashForValidate\n     })\n   }\n   ```\n\n   See the [Webhook sample app for an example implementation](https://github.com/zoom/webhook-sample-node.js/blob/master/index.js#L34).\n\n### Revalidation\n\nYour production and development webhook URLs will be revalidated periodically every 72 hours. Zoom will send notification emails to the account owner associated with your apps if the URL fails the revalidation, following this schedule:\n\n1. First notification email after a total of 2 consecutive failed revalidations.\n2. Second notification email after a total of 4 consecutive failed revalidations.\n\nIf the revalidation check fails 6 times in a row, Zoom will stop sending webhook events and disable the event subscription. To fix the issue to receive webhook events again, go to your apps webhook endpoint URL configuration, reenable webhooks, update your endpoint URL if needed, click validate to revalidate your endpoint URL, and finally, click save.\n\nIf you are still having trouble, see the [Webhook sample app for an example implementation](https://github.com/zoom/webhook-sample-node.js/blob/master/index.js#L34) or review the [webhook endpoint URL requirements](/docs/api/webhooks/#webhook-endpoint-url-requirements).\n\n---  \n\n## Configure webhooks via Zoom API   \nFor existing apps, you can use the Zoom API to create and manage event subscriptions. This allows you to programmatically:  \n- Subscribe users or accounts to specific webhook events based on their preference, actions or regions.  \n- Change which events you are subscribed to without manually reconfiguring the app in the build flow.  \n- Easily manage webhook subscriptions per customer when building multi-tenant applications.  \n\n<Alert>\n\n**Note**  \n\nYou can\u2019t use the API method to modify event subscriptions created through the build flow.    \n\n</Alert>  \n\nThe following is an example of creating an event subscription via API for a general app type. You can use the same general steps for other subscription management actions:    \n \n\n* **Create an Event Subscription**      \n  <a href=\"https://developers.zoom.us/docs/api/marketplace/#tag/app/post/marketplace/app/event_subscription\">`POST https://api.zoom.us/v2/marketplace/app/event_subscription`</a>  \n   \n\n* **Subscribe to an Event Subscription**    \n  <a href=\"https://developers.zoom.us/docs/api/marketplace/#tag/app/patch/marketplace/app/event_subscription/{eventSubscriptionId}\">`PATCH https://api.zoom.us/v2/marketplace/app/event_subscription/{eventSubscriptionId}`</a>  \n\n* **Unsubscribe from an Event Subscription**    \n  <a href=\"https://developers.zoom.us/docs/api/marketplace/#tag/app/delete/marketplace/app/event_subscription\">`DELETE https://api.zoom.us/v2/marketplace/app/event_subscription`</a>    \n\n* **Delete an Event Subscription**  \n  <a href=\"https://developers.zoom.us/docs/api/marketplace/#tag/app/delete/marketplace/app/event_subscription/{eventSubscriptionId}\">`DELETE https://api.zoom.us/v2/marketplace/app/event_subscription/{eventSubscriptionId}`</a>\n\n* **Get User or Account Event Subscriptions**  \n   <a href=\"https://developers.zoom.us/docs/api/marketplace/#tag/app/get/marketplace/app/event_subscription\">`GET https://api.zoom.us/v2/marketplace/app/event_subscription`</a>  \n    \n### Prerequisite:  \n* You have an existing app.  \n* In your app, you have already selected the scopes required by your event subscriptions. See [Scopes](#scopes).  \n\n### Step 1: Request an access token  \n[Request an access token](/docs/integrations/oauth/#request-an-access-token) to call APIs for your own Zoom account by sending a POST request to `https://zoom.us/oauth/token`.  \n\nInclude a basic authorization header. The basic authorization header is your Client ID and Client Secret with a colon : in between, Base64 Encoded.  \n\n\n```bash  \n-- request POST https://zoom.us/oauth/token?grant_type=client_credentials\n-- header \"Authorization\": \"Basic <YOUR_BASE64_ENCODED_CREDENTIALS>\" \n```\n\n<Alert variant=\"warning\">\n\n**Note** \n\nWebhook subscriptions are app-owned resources; you need to use the grant type appropriate to the app type to obtain the access token.  \n* For General OAuth apps, use the [client credentials](/docs/integrations/oauth/#request-an-access-token) grant type.  \n* For Server-to-Server OAuth apps, use the [account credentials](/docs/integrations/oauth/#request-an-access-token) grant type.\n\n</Alert>\n\nThis returns an `access_token` with the required scopes to manage webhook subscriptions. Use this token as a bearer token in the authorization header of subsequent API calls.   \n\n```json  \n{\n    \"access_token\": \"eyJzyNzA2ZDlkMzk0MDgzTk0MzAzMjUsImlzcy...A\",\n    \"token_type\": \"bearer\",\n    \"expires_in\": 3600,\n    \"scope\": \"marketplace:delete:event_subscription marketplace:read:list_event_subscriptions marketplace:update:client_secret marketplace:update:event_subscription marketplace:write:event_subscription marketplace:write:websocket_connection\",\n    \"api_url\": \"https://api.zoom.us\"\n}\n\n```\n\n### Step 2: Create an event subscription \n\nUse the POST method for the `event_subscription` endpoint.  \n```bash\nPOST https://api.zoom.us/v2/marketplace/app/event_subscription \n```  \n\nInclude the authorization in the request header.\n```bash\n{ \"Authorization\": \"Bearer <access_token>\" }\n```\n\nExample request body:  \n```json  \n{\n  \"events\": [\n    \"meeting.created\"\n  ],\n  \"event_subscription_name\": \"Example Event Subscription\",\n  \"event_webhook_url\": \"https://www.example.com\",\n  \"user_ids\": [\n    \"_8KG7DeoRU2xIsDSY9ed2Q,90KG7DeoRU2xIsDSY9edwe\"\n  ],\n  \"subscription_scope\": \"user\",\n  \"account_id\": \"pvg3UAgpRlyTDW-9sIpKcw\"\n}\n\n```  \nOn successful creation, Zoom provides an event_subscription_id.\n\n```json  \n{\n  \"event_subscription_id\": \"0ZAaJY4dQ52BbwI9PArBLQ\"\n}\n```  \nWith this event_subscription_id you can then subscribe to, unsubscribe from, or delete event subscriptions.  \n\nFor more information, see the [Developer blog](/blog/webhook-management-with-zoom-api/). \n\n## Scopes  \nScopes control your app's access to the Zoom platform including the event information requested by your subscriptions.  \n\nZoom has automatically applied the scopes for event subscriptions to your app: (such as creating, updating, or deleting subscriptions). For any other events you want your app to receive, you must add the respective scopes to your app via the app build flow.   \n\nFor example:  \n* To subscribe to the `meeting.participant_joined` event, your app must request the `meeting:read:participant` or `meeting:read:participant:admin`.  \n* To receive `recording.completed`, you'll need `cloud_recording:read:recording` or `cloud_recording:read:recording:admin`.  \n\nIf you haven't added the appropriate scopes to the app, on using the event subscription endpoints you will receive an error. \"The event is not allowed for the app.\"  \n\nFor more information, see [Selecting scope](/docs/build-flow/create-oauth-apps/#step-4-select-the-scopes-zoom-api-methods).\n\n\n\n\n---  \n\n## Notification delivery\n\nYour webhook endpoint URL should respond with either a `200` or a `204` HTTP status code within **three seconds** after receiving a webhook request in order for Zoom to consider the notification was successfully delivered.\n\n### Unsuccessful delivery\n\nZoom attempts to deliver the webhook **three times** for response codes >=`500` or codes in `-1`, `-2`, `-3`, `-4`, `-5`, `-7`, `-8`, `-10`, `-13`, `-14`, `-15`. (See **Enum Descriptions** below.)  \n\n- The first attempt is sent **5 minutes** after the initial delivery attempt.\n- A second attempt is sent **20 minutes** after the first attempt.\n- A third attempt is sent **60 minutes** after the second attempt.\n\nZoom considers response codes >=`200` or < `300` as successfully delivered after any retry attempt.  \n\nIf Zoom does **not** receive a response code >=`200` or < `300` after three attempts, then Zoom will send no further webhooks for that event.  \n\nZoom does not retry redirect codes >=`300` and < `400`.  \nZoom does not retry client error codes >=`400` and < `500`, such as unauthenticated, resource not found, rate limiting, etc., because retrying would still fail or further increase the server load.  \n\n**Enum Descriptions**  \n- `ReadTimeOut(-1)` Socket time out.\n- `ConnectionRefused(-2)` Connection refused.    \n- `Fallback(-3)` Hystrix fallback on error.    \n- `Resilience4jTimeout(-4)` Resilience time out.    \n- `AsyncError(-5)` Failed to execute job via async mode.    \n- `RemoteError(-7)` Remote error.    \n- `Delegate(-8)` Delegate tasks to a separate server for processing if in delegate white list.  \n- `NeedInstantRetry(-10)` Retry webhook invoke instantly when connection lease request times out.    \n- `DNSResolveTimeout(-13)` Zoom DNS resolve timeout exception.    \n- `CanNotGetAccessToken(-14)` Delay retry error code if developer enabled Oauth client credential header in the subscription, but Zoom cannot get access_token from tokenUrl by clientId & clientSecret.  \n- `CanNotGetCustomHeaderSettingInNoCacheTopic(-15)` Delay retry error code if developer enabled custom header in the subscription, but Zoom cannot get custom header setting when handling msg from no cache topic.   \n  \n\n\n<Alert> \n\n**Troubleshooting missing webhook events**\n\nIf you have successfully validated your webhook endpoint, but you are not receiving certain events you have selected, toggle the webhook feature on and off and try again.\n\n</Alert>\n\nUse [Get webhook logs\u200b](/docs/api/marketplace/#tag/app/GET/marketplace/apps/{appId}/webhook_logs) to get a list of webhooks sent to an app.\n\n### Delivery latency\n\nThe `event_ts` response is included in the payload of all event notifications. This represents the timestamp for when the associated event occurred.\n\nYou can determine the latency in webhook delivery by calculating the difference between the delivery timestamp and the value of the `event_ts`.\n\n",
        "data": {
          "title": "Using webhooks",
          "resources": [
            {
              "title": "Zoom Platform 101 Zoom Webhooks",
              "url": "https://youtu.be/oAnr1ezW8dY?si=AVlihxis-DIS4L5W"
            },
            {
              "title": "Server-to-Server apps",
              "url": "https://developers.zoom.us/docs/internal-apps/"
            },
            {
              "title": "General OAuth apps",
              "url": "https://developers.zoom.us/docs/integrations/"
            },
            {
              "title": "Build platform apps",
              "url": "https://developers.zoom.us/docs/build/"
            },
            {
              "title": "Updating an app",
              "url": "https://developers.zoom.us/docs/distribute/published-apps/updating/"
            }
          ]
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/webhooks.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
