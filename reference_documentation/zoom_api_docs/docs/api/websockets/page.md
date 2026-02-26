# Using WebSockets

- Source URL: https://developers.zoom.us/docs/api/websockets/
- Snapshot path: docs/api/websockets
- Generated (UTC): 2026-02-07T22:40:11.976385+00:00

## Frontmatter

```json
{
  "title": "Using WebSockets"
}
```

## Content

```md

# Using WebSockets

The WebSocket protocol offers a full-duplex communication channel over a single TCP connection (see [rfc6455](https://datatracker.ietf.org/doc/html/rfc6455) for details). Zoom WebSocket events allow Developers to make a single, secured HTTP request to Zoom while keeping a single connection open to transmit all event data to the client subscribing to the event.

Zoom delivers events when the WebSocket connection is open. When the connection is closed, Zoom does not deliver events. Any events that occur when the connection is closed will not be delivered. Once the connection is open again, Zoom will deliver events that occurred after the connection was reopened.

## Workplace

<div class="api_ref_tile_container">
<Link class="api_ref_tile" href="/docs/api/meetings/events">
  <SvgIcon svgname="_meetings" />
  <h3>Meetings</h3>
</Link>
<Link class="api_ref_tile" href="/docs/api/team-chat/events">
  <SvgIcon svgname="_chat" />
  <h3>Team Chat</h3>
</Link>
<Link class="api_ref_tile" href="/docs/api/phone/events">
  <SvgIcon svgname="_zoomphone" />
  <h3>Phone</h3>
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

## Marketplace

<div class="api_ref_tile_container">
  <Link class="api_ref_tile" href="/docs/api/marketplace/events">
    <SvgIcon svgname="marketplace" />
    <h3>Apps</h3>
  </Link>
</div>

## Add WebSocket events

1. Go to the **Feature** section of your server-to-server OAuth or general app and toggle **Event subscription** to add an event.

2. Choose the **WebSocket** method.

3. Add a name for this event subscription.

4. (Master account only) If you have a master account with several sub-accounts, you can choose whether to send event notifications to **All users in the account** or **All users in the account and subaccounts**. If you don't have a master account, use the default, **All users in the account**.

<Image src="/img/s2s-websocket01.png" class="ui image" alt="Server-to-Server add feature WebSocket event" />

5. Add the events that you'd like to send.

6. Click **Save**. You'll see the endpoint URL, which you can copy and use later (see [Using WebSockets](#using-websockets) below for details).

<Image src="/img/s2s-websocket02-endpoint.png" class="ui image" alt="Server-to-Server WebSocket event added" />

7. Click **Continue**.

## Using WebSockets

Follow the steps below to use WebSockets.

1. **Get an access token.** Server-to-server apps and general apps use the `client_credentials` grant type.  
  
   Use the [`client_credentials` grant type](https://www.oauth.com/oauth2-servers/access-tokens/client-credentials/) and provide your client ID and secret in exchange for your access token. You should Base64 encode your client ID and secret and put them in the authorization header.  
   
        ```
        curl -X POST -H 'Authorization: Basic Base64Encoder(clientId:clientSecret)' https://zoom.us/oauth/token?grant_type=client_credentials
        ```

   The response will be the access token, a bearer `token_type` that expires in an hour, with the scopes that you chose in your app settings screen:

    ```
    {
        "Access_token": String,
        "Token_type": "bearer",
        "Expire_in": long,
        "scope" : [String]
    }
    ```

2. Copy the **WebSocket EndPoint URL** from the **Event subscription** page in your app (see [Add WebSocket events](#add-websocket-events) above for details).

3. Append the access token to the end of this URL and use it to open a connection to the server:

    ```
    wss://ws.zoom.us/ws?subscriptionId=ZhPGtK84QbapRWXzEBW7MA&access_token=access_token
    ```
* For example, in Postman:

<Image src="/img/websocket-req01.png" class="ui image" alt="Postman open WebSocket request" />

4. Send a heartbeat to the server every 30 seconds to keep the connection alive:

    ```
    {
        "module": "heartbeat"
    }
    ```

* In Postman:

<Image src="/img/websocket-req02.png" class="ui image" alt="Postman WebSocket heartbeat" />

5. Trigger the event to receive a response through your open WebSocket. Depending on the event that you subscribed to, you can trigger it through API, Web, or the Zoom app.

* In Postman:

<Image src="/img/websocket-req03.png" class="ui image" alt="Postman WebSocket response" />

## WebSockets FAQ

Here are some common questions and answers about WebSockets.

### What is a WebSocket?

The WebSocket protocol offers a full-duplex communication channel over a single TCP connection (see [rfc6455](https://datatracker.ietf.org/doc/html/rfc6455) for details).

WebSocket events allow customers' developers to make a single, secured HTTP request to Zoom while keeping a single connection open to transmit all event data to the client subscribing to the event.

### What is the difference between API calls, webhooks, and WebSockets?

Here are the descriptions of API calls, webhooks, and WebSockets that demonstrate the differences.

#### API calls

An API call is the process of a client application submitting a request to an API endpoint, the API server acting on that request, such as to retrieve data, and the server responding back to the client with a successful response that includes any data requested, or an error.

<Image src="/img/1570826762485.png" alt="The Zoom authorization process" />

#### Webhooks

Webhooks are near real-time information about specific events in your Zoom account sent by Zoom via HTTP POST requests, one for each event. Event types include account events, app events, meeting events, and more. See [Using Webhooks](/docs/api/webhooks) for details.

#### WebSockets

A WebSocket is a two-way interactive communication session between the user's browser and a server. When this communication channel is opened, you can send messages to a server and receive event-driven responses without having to poll the server for a reply.

<Image src="/img/websocket-flow.png" alt="The Zoom authorization process" />

### Are WebSockets account level or user-managed?

Account level.

### Can I use WebSockets outside of my Zoom Account?

No, WebSockets are "bound" to the account that creates them, and are not intended for multi-tenancy like Zoom Marketplace Apps.

### What app types can I use with WebSockets?

Both Server-to-Server OAuth and general apps can use WebSockets.
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
      "code": "var Component=(()=>{var m=Object.create;var o=Object.defineProperty;var v=Object.getOwnPropertyDescriptor;var f=Object.getOwnPropertyNames;var b=Object.getPrototypeOf,g=Object.prototype.hasOwnProperty;var k=(t,n)=>()=>(n||t((n={exports:{}}).exports,n),n.exports),w=(t,n)=>{for(var s in n)o(t,s,{get:n[s],enumerable:!0})},h=(t,n,s,i)=>{if(n&&typeof n==\"object\"||typeof n==\"function\")for(let r of f(n))!g.call(t,r)&&r!==s&&o(t,r,{get:()=>n[r],enumerable:!(i=v(n,r))||i.enumerable});return t};var _=(t,n,s)=>(s=t!=null?m(b(t)):{},h(n||!t||!t.__esModule?o(s,\"default\",{value:t,enumerable:!0}):s,t)),S=t=>h(o({},\"__esModule\",{value:!0}),t);var d=k((P,l)=>{l.exports=_jsx_runtime});var y={};w(y,{default:()=>u,frontmatter:()=>W});var e=_(d());var{useMDXComponents:c}=MdxJsReact;var W={title:\"Using WebSockets\"};function p(t){let n={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",h3:\"h3\",h4:\"h4\",i:\"i\",li:\"li\",ol:\"ol\",p:\"p\",pre:\"pre\",strong:\"strong\",ul:\"ul\",...c(),...t.components},{Image:s,Link:i,SvgIcon:r}=n;return s||a(\"Image\",!0),i||a(\"Link\",!0),r||a(\"SvgIcon\",!0),(0,e.jsxs)(e.Fragment,{children:[(0,e.jsx)(n.h1,{id:\"using-websockets\",children:\"Using WebSockets\"}),`\n`,(0,e.jsxs)(n.p,{children:[\"The WebSocket protocol offers a full-duplex communication channel over a single TCP connection (see \",(0,e.jsx)(n.a,{href:\"https://datatracker.ietf.org/doc/html/rfc6455\",children:\"rfc6455\"}),\" for details). Zoom WebSocket events allow Developers to make a single, secured HTTP request to Zoom while keeping a single connection open to transmit all event data to the client subscribing to the event.\"]}),`\n`,(0,e.jsx)(n.p,{children:\"Zoom delivers events when the WebSocket connection is open. When the connection is closed, Zoom does not deliver events. Any events that occur when the connection is closed will not be delivered. Once the connection is open again, Zoom will deliver events that occurred after the connection was reopened.\"}),`\n`,(0,e.jsx)(n.h2,{id:\"workplace\",children:(0,e.jsxs)(n.a,{href:\"#workplace\",children:[\"Workplace\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(\"div\",{class:\"api_ref_tile_container\",children:[(0,e.jsxs)(i,{class:\"api_ref_tile\",href:\"/docs/api/meetings/events\",children:[(0,e.jsx)(r,{svgname:\"_meetings\"}),(0,e.jsx)(\"h3\",{children:\"Meetings\"})]}),(0,e.jsxs)(i,{class:\"api_ref_tile\",href:\"/docs/api/team-chat/events\",children:[(0,e.jsx)(r,{svgname:\"_chat\"}),(0,e.jsx)(\"h3\",{children:\"Team Chat\"})]}),(0,e.jsxs)(i,{class:\"api_ref_tile\",href:\"/docs/api/phone/events\",children:[(0,e.jsx)(r,{svgname:\"_zoomphone\"}),(0,e.jsx)(\"h3\",{children:\"Phone\"})]}),(0,e.jsxs)(i,{class:\"api_ref_tile\",href:\"/docs/api/rooms/events\",children:[(0,e.jsx)(r,{svgname:\"_zoomrooms\"}),(0,e.jsx)(\"h3\",{children:\"Rooms\"})]}),(0,e.jsxs)(i,{class:\"api_ref_tile\",href:\"/docs/api/whiteboard/events\",children:[(0,e.jsx)(r,{svgname:\"_whiteboard\"}),(0,e.jsx)(\"h3\",{children:\"Whiteboard\"})]}),(0,e.jsxs)(i,{class:\"api_ref_tile\",href:\"/docs/api/chatbot/events\",children:[(0,e.jsx)(r,{svgname:\"_chat\"}),(0,e.jsx)(\"h3\",{children:\"Chatbot\"})]})]}),`\n`,(0,e.jsx)(n.h2,{id:\"business-services\",children:(0,e.jsxs)(n.a,{href:\"#business-services\",children:[\"Business services\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(\"div\",{class:\"api_ref_tile_container\",children:[(0,e.jsxs)(i,{class:\"api_ref_tile\",href:\"/docs/api/contact-center/events\",children:[(0,e.jsx)(r,{svgname:\"contactCenter\"}),(0,e.jsx)(\"h3\",{children:\"Contact Center\"})]}),(0,e.jsxs)(i,{class:\"api_ref_tile\",href:\"/docs/api/events/events\",children:[(0,e.jsx)(r,{svgname:\"_zoomevents\"}),(0,e.jsx)(\"h3\",{children:\"Events\"})]}),(0,e.jsxs)(i,{class:\"api_ref_tile\",href:\"/docs/api/iq/events\",children:[(0,e.jsx)(r,{svgname:\"zra\"}),(0,e.jsx)(\"h3\",{children:\"Revenue Accelerator\"})]}),(0,e.jsxs)(i,{class:\"api_ref_tile\",href:\"/docs/api/number-management/events\",children:[(0,e.jsx)(r,{svgname:\"number-management\"}),(0,e.jsx)(\"h3\",{children:\"Number Management\"})]}),(0,e.jsxs)(i,{class:\"api_ref_tile\",href:\"/docs/api/node/events\",children:[(0,e.jsx)(r,{svgname:\"zoom-node\"}),(0,e.jsx)(\"h3\",{children:\"Node\"})]})]}),`\n`,(0,e.jsx)(n.h2,{id:\"accounts\",children:(0,e.jsxs)(n.a,{href:\"#accounts\",children:[\"Accounts\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(\"div\",{class:\"api_ref_tile_container\",children:[(0,e.jsxs)(i,{class:\"api_ref_tile\",href:\"/docs/api/users/events\",children:[(0,e.jsx)(r,{svgname:\"users\"}),(0,e.jsx)(\"h3\",{children:\"Users\"})]}),(0,e.jsxs)(i,{class:\"api_ref_tile\",href:\"/docs/api/accounts/events\",children:[(0,e.jsx)(r,{svgname:\"accounts\"}),(0,e.jsx)(\"h3\",{children:\"Accounts\"})]}),(0,e.jsxs)(i,{class:\"api_ref_tile\",href:\"/docs/api/qss/events\",children:[(0,e.jsx)(r,{svgname:\"qss\"}),(0,e.jsx)(\"h3\",{children:\"QSS\"})]})]}),`\n`,(0,e.jsx)(n.h2,{id:\"marketplace\",children:(0,e.jsxs)(n.a,{href:\"#marketplace\",children:[\"Marketplace\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(\"div\",{class:\"api_ref_tile_container\",children:(0,e.jsxs)(i,{class:\"api_ref_tile\",href:\"/docs/api/marketplace/events\",children:[(0,e.jsx)(r,{svgname:\"marketplace\"}),(0,e.jsx)(\"h3\",{children:\"Apps\"})]})}),`\n`,(0,e.jsx)(n.h2,{id:\"add-websocket-events\",children:(0,e.jsxs)(n.a,{href:\"#add-websocket-events\",children:[\"Add WebSocket events\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.ol,{children:[`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Go to the \",(0,e.jsx)(n.strong,{children:\"Feature\"}),\" section of your server-to-server OAuth or general app and toggle \",(0,e.jsx)(n.strong,{children:\"Event subscription\"}),\" to add an event.\"]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Choose the \",(0,e.jsx)(n.strong,{children:\"WebSocket\"}),\" method.\"]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsx)(n.p,{children:\"Add a name for this event subscription.\"}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"(Master account only) If you have a master account with several sub-accounts, you can choose whether to send event notifications to \",(0,e.jsx)(n.strong,{children:\"All users in the account\"}),\" or \",(0,e.jsx)(n.strong,{children:\"All users in the account and subaccounts\"}),\". If you don't have a master account, use the default, \",(0,e.jsx)(n.strong,{children:\"All users in the account\"}),\".\"]}),`\n`]}),`\n`]}),`\n`,(0,e.jsx)(s,{src:\"/img/s2s-websocket01.png\",class:\"ui image\",alt:\"Server-to-Server add feature WebSocket event\"}),`\n`,(0,e.jsxs)(n.ol,{start:\"5\",children:[`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsx)(n.p,{children:\"Add the events that you'd like to send.\"}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Click \",(0,e.jsx)(n.strong,{children:\"Save\"}),\". You'll see the endpoint URL, which you can copy and use later (see \",(0,e.jsx)(n.a,{href:\"#using-websockets\",children:\"Using WebSockets\"}),\" below for details).\"]}),`\n`]}),`\n`]}),`\n`,(0,e.jsx)(s,{src:\"/img/s2s-websocket02-endpoint.png\",class:\"ui image\",alt:\"Server-to-Server WebSocket event added\"}),`\n`,(0,e.jsxs)(n.ol,{start:\"7\",children:[`\n`,(0,e.jsxs)(n.li,{children:[\"Click \",(0,e.jsx)(n.strong,{children:\"Continue\"}),\".\"]}),`\n`]}),`\n`,(0,e.jsx)(n.h2,{id:\"using-websockets-1\",children:(0,e.jsxs)(n.a,{href:\"#using-websockets-1\",children:[\"Using WebSockets\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"Follow the steps below to use WebSockets.\"}),`\n`,(0,e.jsxs)(n.ol,{children:[`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[(0,e.jsx)(n.strong,{children:\"Get an access token.\"}),\" Server-to-server apps and general apps use the \",(0,e.jsx)(n.code,{children:\"client_credentials\"}),\" grant type.\"]}),`\n`,(0,e.jsxs)(n.p,{children:[\"Use the \",(0,e.jsxs)(n.a,{href:\"https://www.oauth.com/oauth2-servers/access-tokens/client-credentials/\",children:[(0,e.jsx)(n.code,{children:\"client_credentials\"}),\" grant type\"]}),\" and provide your client ID and secret in exchange for your access token. You should Base64 encode your client ID and secret and put them in the authorization header.\"]}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{children:`curl -X POST -H 'Authorization: Basic Base64Encoder(clientId:clientSecret)' https://zoom.us/oauth/token?grant_type=client_credentials\n`})}),`\n`,(0,e.jsxs)(n.p,{children:[\"The response will be the access token, a bearer \",(0,e.jsx)(n.code,{children:\"token_type\"}),\" that expires in an hour, with the scopes that you chose in your app settings screen:\"]}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{children:`{\n    \"Access_token\": String,\n    \"Token_type\": \"bearer\",\n    \"Expire_in\": long,\n    \"scope\" : [String]\n}\n`})}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Copy the \",(0,e.jsx)(n.strong,{children:\"WebSocket EndPoint URL\"}),\" from the \",(0,e.jsx)(n.strong,{children:\"Event subscription\"}),\" page in your app (see \",(0,e.jsx)(n.a,{href:\"#add-websocket-events\",children:\"Add WebSocket events\"}),\" above for details).\"]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsx)(n.p,{children:\"Append the access token to the end of this URL and use it to open a connection to the server:\"}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{children:`wss://ws.zoom.us/ws?subscriptionId=ZhPGtK84QbapRWXzEBW7MA&access_token=access_token\n`})}),`\n`]}),`\n`]}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsx)(n.li,{children:\"For example, in Postman:\"}),`\n`]}),`\n`,(0,e.jsx)(s,{src:\"/img/websocket-req01.png\",class:\"ui image\",alt:\"Postman open WebSocket request\"}),`\n`,(0,e.jsxs)(n.ol,{start:\"4\",children:[`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsx)(n.p,{children:\"Send a heartbeat to the server every 30 seconds to keep the connection alive:\"}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{children:`{\n    \"module\": \"heartbeat\"\n}\n`})}),`\n`]}),`\n`]}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsx)(n.li,{children:\"In Postman:\"}),`\n`]}),`\n`,(0,e.jsx)(s,{src:\"/img/websocket-req02.png\",class:\"ui image\",alt:\"Postman WebSocket heartbeat\"}),`\n`,(0,e.jsxs)(n.ol,{start:\"5\",children:[`\n`,(0,e.jsx)(n.li,{children:\"Trigger the event to receive a response through your open WebSocket. Depending on the event that you subscribed to, you can trigger it through API, Web, or the Zoom app.\"}),`\n`]}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsx)(n.li,{children:\"In Postman:\"}),`\n`]}),`\n`,(0,e.jsx)(s,{src:\"/img/websocket-req03.png\",class:\"ui image\",alt:\"Postman WebSocket response\"}),`\n`,(0,e.jsx)(n.h2,{id:\"websockets-faq\",children:(0,e.jsxs)(n.a,{href:\"#websockets-faq\",children:[\"WebSockets FAQ\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"Here are some common questions and answers about WebSockets.\"}),`\n`,(0,e.jsx)(n.h3,{id:\"what-is-a-websocket\",children:(0,e.jsxs)(n.a,{href:\"#what-is-a-websocket\",children:[\"What is a WebSocket?\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.p,{children:[\"The WebSocket protocol offers a full-duplex communication channel over a single TCP connection (see \",(0,e.jsx)(n.a,{href:\"https://datatracker.ietf.org/doc/html/rfc6455\",children:\"rfc6455\"}),\" for details).\"]}),`\n`,(0,e.jsx)(n.p,{children:\"WebSocket events allow customers' developers to make a single, secured HTTP request to Zoom while keeping a single connection open to transmit all event data to the client subscribing to the event.\"}),`\n`,(0,e.jsx)(n.h3,{id:\"what-is-the-difference-between-api-calls-webhooks-and-websockets\",children:(0,e.jsxs)(n.a,{href:\"#what-is-the-difference-between-api-calls-webhooks-and-websockets\",children:[\"What is the difference between API calls, webhooks, and WebSockets?\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"Here are the descriptions of API calls, webhooks, and WebSockets that demonstrate the differences.\"}),`\n`,(0,e.jsx)(n.h4,{id:\"api-calls\",children:(0,e.jsxs)(n.a,{href:\"#api-calls\",children:[\"API calls\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"An API call is the process of a client application submitting a request to an API endpoint, the API server acting on that request, such as to retrieve data, and the server responding back to the client with a successful response that includes any data requested, or an error.\"}),`\n`,(0,e.jsx)(s,{src:\"/img/1570826762485.png\",alt:\"The Zoom authorization process\"}),`\n`,(0,e.jsx)(n.h4,{id:\"webhooks\",children:(0,e.jsxs)(n.a,{href:\"#webhooks\",children:[\"Webhooks\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.p,{children:[\"Webhooks are near real-time information about specific events in your Zoom account sent by Zoom via HTTP POST requests, one for each event. Event types include account events, app events, meeting events, and more. See \",(0,e.jsx)(n.a,{href:\"/docs/api/webhooks\",children:\"Using Webhooks\"}),\" for details.\"]}),`\n`,(0,e.jsx)(n.h4,{id:\"websockets\",children:(0,e.jsxs)(n.a,{href:\"#websockets\",children:[\"WebSockets\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"A WebSocket is a two-way interactive communication session between the user's browser and a server. When this communication channel is opened, you can send messages to a server and receive event-driven responses without having to poll the server for a reply.\"}),`\n`,(0,e.jsx)(s,{src:\"/img/websocket-flow.png\",alt:\"The Zoom authorization process\"}),`\n`,(0,e.jsx)(n.h3,{id:\"are-websockets-account-level-or-user-managed\",children:(0,e.jsxs)(n.a,{href:\"#are-websockets-account-level-or-user-managed\",children:[\"Are WebSockets account level or user-managed?\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"Account level.\"}),`\n`,(0,e.jsx)(n.h3,{id:\"can-i-use-websockets-outside-of-my-zoom-account\",children:(0,e.jsxs)(n.a,{href:\"#can-i-use-websockets-outside-of-my-zoom-account\",children:[\"Can I use WebSockets outside of my Zoom Account?\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:'No, WebSockets are \"bound\" to the account that creates them, and are not intended for multi-tenancy like Zoom Marketplace Apps.'}),`\n`,(0,e.jsx)(n.h3,{id:\"what-app-types-can-i-use-with-websockets\",children:(0,e.jsxs)(n.a,{href:\"#what-app-types-can-i-use-with-websockets\",children:[\"What app types can I use with WebSockets?\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"Both Server-to-Server OAuth and general apps can use WebSockets.\"})]})}function u(t={}){let{wrapper:n}={...c(),...t.components};return n?(0,e.jsx)(n,{...t,children:(0,e.jsx)(p,{...t})}):p(t)}function a(t,n){throw new Error(\"Expected \"+(n?\"component\":\"object\")+\" `\"+t+\"` to be defined: you likely forgot to import, pass, or provide it.\")}return S(y);})();\n;return Component;",
      "frontmatter": {
        "title": "Using WebSockets"
      },
      "errors": [],
      "matter": {
        "content": "\n# Using WebSockets\n\nThe WebSocket protocol offers a full-duplex communication channel over a single TCP connection (see [rfc6455](https://datatracker.ietf.org/doc/html/rfc6455) for details). Zoom WebSocket events allow Developers to make a single, secured HTTP request to Zoom while keeping a single connection open to transmit all event data to the client subscribing to the event.\n\nZoom delivers events when the WebSocket connection is open. When the connection is closed, Zoom does not deliver events. Any events that occur when the connection is closed will not be delivered. Once the connection is open again, Zoom will deliver events that occurred after the connection was reopened.\n\n## Workplace\n\n<div class=\"api_ref_tile_container\">\n<Link class=\"api_ref_tile\" href=\"/docs/api/meetings/events\">\n  <SvgIcon svgname=\"_meetings\" />\n  <h3>Meetings</h3>\n</Link>\n<Link class=\"api_ref_tile\" href=\"/docs/api/team-chat/events\">\n  <SvgIcon svgname=\"_chat\" />\n  <h3>Team Chat</h3>\n</Link>\n<Link class=\"api_ref_tile\" href=\"/docs/api/phone/events\">\n  <SvgIcon svgname=\"_zoomphone\" />\n  <h3>Phone</h3>\n</Link>\n<Link class=\"api_ref_tile\" href=\"/docs/api/rooms/events\">\n  <SvgIcon svgname=\"_zoomrooms\" />\n  <h3>Rooms</h3>\n</Link>\n<Link class=\"api_ref_tile\" href=\"/docs/api/whiteboard/events\">\n  <SvgIcon svgname=\"_whiteboard\" />\n  <h3>Whiteboard</h3>\n</Link>\n<Link class=\"api_ref_tile\" href=\"/docs/api/chatbot/events\">\n  <SvgIcon svgname=\"_chat\" />\n  <h3>Chatbot</h3>\n</Link>\n</div>\n\n## Business services\n\n<div class=\"api_ref_tile_container\">\n  <Link class=\"api_ref_tile\" href=\"/docs/api/contact-center/events\">\n    <SvgIcon svgname=\"contactCenter\" />\n    <h3>Contact Center</h3>\n  </Link>\n  <Link class=\"api_ref_tile\" href=\"/docs/api/events/events\">\n    <SvgIcon svgname=\"_zoomevents\" />\n    <h3>Events</h3>\n  </Link>\n  <Link class=\"api_ref_tile\" href=\"/docs/api/iq/events\">\n    <SvgIcon svgname=\"zra\" />\n    <h3>Revenue Accelerator</h3>\n  </Link>\n  <Link class=\"api_ref_tile\" href=\"/docs/api/number-management/events\">\n  <SvgIcon svgname=\"number-management\" />\n  <h3>Number Management</h3>\n</Link>\n      <Link class=\"api_ref_tile\" href=\"/docs/api/node/events\">\n  <SvgIcon svgname=\"zoom-node\" />\n    <h3>Node</h3>\n  </Link>\n</div>\n\n## Accounts\n\n<div class=\"api_ref_tile_container\">\n  <Link class=\"api_ref_tile\" href=\"/docs/api/users/events\">\n    <SvgIcon svgname=\"users\" />\n    <h3>Users</h3>\n  </Link>\n  <Link class=\"api_ref_tile\" href=\"/docs/api/accounts/events\">\n    <SvgIcon svgname=\"accounts\" />\n    <h3>Accounts</h3>\n  </Link>\n  <Link class=\"api_ref_tile\" href=\"/docs/api/qss/events\">\n    <SvgIcon svgname=\"qss\" />\n    <h3>QSS</h3>\n  </Link>\n</div>\n\n## Marketplace\n\n<div class=\"api_ref_tile_container\">\n  <Link class=\"api_ref_tile\" href=\"/docs/api/marketplace/events\">\n    <SvgIcon svgname=\"marketplace\" />\n    <h3>Apps</h3>\n  </Link>\n</div>\n\n## Add WebSocket events\n\n1. Go to the **Feature** section of your server-to-server OAuth or general app and toggle **Event subscription** to add an event.\n\n2. Choose the **WebSocket** method.\n\n3. Add a name for this event subscription.\n\n4. (Master account only) If you have a master account with several sub-accounts, you can choose whether to send event notifications to **All users in the account** or **All users in the account and subaccounts**. If you don't have a master account, use the default, **All users in the account**.\n\n<Image src=\"/img/s2s-websocket01.png\" class=\"ui image\" alt=\"Server-to-Server add feature WebSocket event\" />\n\n5. Add the events that you'd like to send.\n\n6. Click **Save**. You'll see the endpoint URL, which you can copy and use later (see [Using WebSockets](#using-websockets) below for details).\n\n<Image src=\"/img/s2s-websocket02-endpoint.png\" class=\"ui image\" alt=\"Server-to-Server WebSocket event added\" />\n\n7. Click **Continue**.\n\n## Using WebSockets\n\nFollow the steps below to use WebSockets.\n\n1. **Get an access token.** Server-to-server apps and general apps use the `client_credentials` grant type.  \n  \n   Use the [`client_credentials` grant type](https://www.oauth.com/oauth2-servers/access-tokens/client-credentials/) and provide your client ID and secret in exchange for your access token. You should Base64 encode your client ID and secret and put them in the authorization header.  \n   \n        ```\n        curl -X POST -H 'Authorization: Basic Base64Encoder(clientId:clientSecret)' https://zoom.us/oauth/token?grant_type=client_credentials\n        ```\n\n   The response will be the access token, a bearer `token_type` that expires in an hour, with the scopes that you chose in your app settings screen:\n\n    ```\n    {\n        \"Access_token\": String,\n        \"Token_type\": \"bearer\",\n        \"Expire_in\": long,\n        \"scope\" : [String]\n    }\n    ```\n\n2. Copy the **WebSocket EndPoint URL** from the **Event subscription** page in your app (see [Add WebSocket events](#add-websocket-events) above for details).\n\n3. Append the access token to the end of this URL and use it to open a connection to the server:\n\n    ```\n    wss://ws.zoom.us/ws?subscriptionId=ZhPGtK84QbapRWXzEBW7MA&access_token=access_token\n    ```\n* For example, in Postman:\n\n<Image src=\"/img/websocket-req01.png\" class=\"ui image\" alt=\"Postman open WebSocket request\" />\n\n4. Send a heartbeat to the server every 30 seconds to keep the connection alive:\n\n    ```\n    {\n        \"module\": \"heartbeat\"\n    }\n    ```\n\n* In Postman:\n\n<Image src=\"/img/websocket-req02.png\" class=\"ui image\" alt=\"Postman WebSocket heartbeat\" />\n\n5. Trigger the event to receive a response through your open WebSocket. Depending on the event that you subscribed to, you can trigger it through API, Web, or the Zoom app.\n\n* In Postman:\n\n<Image src=\"/img/websocket-req03.png\" class=\"ui image\" alt=\"Postman WebSocket response\" />\n\n## WebSockets FAQ\n\nHere are some common questions and answers about WebSockets.\n\n### What is a WebSocket?\n\nThe WebSocket protocol offers a full-duplex communication channel over a single TCP connection (see [rfc6455](https://datatracker.ietf.org/doc/html/rfc6455) for details).\n\nWebSocket events allow customers' developers to make a single, secured HTTP request to Zoom while keeping a single connection open to transmit all event data to the client subscribing to the event.\n\n### What is the difference between API calls, webhooks, and WebSockets?\n\nHere are the descriptions of API calls, webhooks, and WebSockets that demonstrate the differences.\n\n#### API calls\n\nAn API call is the process of a client application submitting a request to an API endpoint, the API server acting on that request, such as to retrieve data, and the server responding back to the client with a successful response that includes any data requested, or an error.\n\n<Image src=\"/img/1570826762485.png\" alt=\"The Zoom authorization process\" />\n\n#### Webhooks\n\nWebhooks are near real-time information about specific events in your Zoom account sent by Zoom via HTTP POST requests, one for each event. Event types include account events, app events, meeting events, and more. See [Using Webhooks](/docs/api/webhooks) for details.\n\n#### WebSockets\n\nA WebSocket is a two-way interactive communication session between the user's browser and a server. When this communication channel is opened, you can send messages to a server and receive event-driven responses without having to poll the server for a reply.\n\n<Image src=\"/img/websocket-flow.png\" alt=\"The Zoom authorization process\" />\n\n### Are WebSockets account level or user-managed?\n\nAccount level.\n\n### Can I use WebSockets outside of my Zoom Account?\n\nNo, WebSockets are \"bound\" to the account that creates them, and are not intended for multi-tenancy like Zoom Marketplace Apps.\n\n### What app types can I use with WebSockets?\n\nBoth Server-to-Server OAuth and general apps can use WebSockets.\n",
        "data": {
          "title": "Using WebSockets"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/websockets.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
