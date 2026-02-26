# Quality of Service Subscription (QSS) API

- Source URL: https://developers.zoom.us/docs/api/rest/qss-api/
- Snapshot path: docs/api/rest/qss-api
- Generated (UTC): 2026-02-07T22:40:11.528777+00:00

## Frontmatter

```json
{
  "title": "Quality of Service Subscription (QSS) API"
}
```

## Content

```md

# Quality of Service Subscription (QSS) API

[Zoom Quality of Service Subscription (QSS)](https://explore.zoom.us/en/qss/) is an add-on product that enables Developers to receive details about the network traffic in near real-time for users, hosts, and participants in their meetings, webinars, and phone calls. This data can be used to proactively troubleshoot and identify issues as quickly as possible. [Contact Zoom Sales](https://explore.zoom.us/en/contactsales/) to add this feature.

## Requirements

In addition to having the QSS product on your account, at least one user on the account must have Developer permissions enabled to be able to create an application that can subscribe to QSS events. This user should create a Zoom app to receive notifications.

## Version: 2.0.0

* **Host:** api.zoom.us/v2
* **Protocols:** `https`
* **Accepts:** `application/json`, `multipart/form-data`
* **Responds With:** `application/json`, `application/xml`
* **Contact URL:** https://developer.zoom.us/
* **Terms Of Service**: https://zoom.us/docs/en-us/zoom_api_license_and_tou.html

## To enable QSS event notifications

On the **Feature** page for the Marketplace app, enable **Event Subscription** and add the **Event notification endpoint URL** to send the webhooks. Click **Add events** and select **QSS**. Select the QSS events that you’d like to receive.

See a list of events and their details in [QSS Event reference](/docs/api/qss/events). See [Using webhooks](/docs/api/webhooks) for details about configuring webhooks for event notifications.

## Use QSS APIs

Use QSS APIs to build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us). Note that you must subscribe to the **QSS summary event** to receive data from the QoS summary APIs.

See the [QSS API reference](/docs/api/qss) and the [QSS Master account API reference](/docs/api/qss/ma) for details.

To learn how to get your credentials and create private or public applications, see [OAuth with Zoom](/docs/integrations/oauth/). See [Using Zoom APIs](/docs/api/using-zoom-apis) for additional information about using Zoom APIs.

See [Master Account API](/docs/api/ma) to learn more about how to use the Master Accounts API.

All endpoints are available via `https` and are located at `api.zoom.us/v2/`.
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
      "code": "var Component=(()=>{var u=Object.create;var r=Object.defineProperty;var m=Object.getOwnPropertyDescriptor;var f=Object.getOwnPropertyNames;var S=Object.getPrototypeOf,v=Object.prototype.hasOwnProperty;var b=(t,e)=>()=>(e||t((e={exports:{}}).exports,e),e.exports),g=(t,e)=>{for(var o in e)r(t,o,{get:e[o],enumerable:!0})},c=(t,e,o,a)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let i of f(e))!v.call(t,i)&&i!==o&&r(t,i,{get:()=>e[i],enumerable:!(a=m(e,i))||a.enumerable});return t};var A=(t,e,o)=>(o=t!=null?u(S(t)):{},c(e||!t||!t.__esModule?r(o,\"default\",{value:t,enumerable:!0}):o,t)),Q=t=>c(r({},\"__esModule\",{value:!0}),t);var h=b((y,d)=>{d.exports=_jsx_runtime});var q={};g(q,{default:()=>p,frontmatter:()=>_});var n=A(h());var{useMDXComponents:s}=MdxJsReact;var _={title:\"Quality of Service Subscription (QSS) API\"};function l(t){let e={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",i:\"i\",li:\"li\",p:\"p\",strong:\"strong\",ul:\"ul\",...s(),...t.components};return(0,n.jsxs)(n.Fragment,{children:[(0,n.jsx)(e.h1,{id:\"quality-of-service-subscription-qss-api\",children:\"Quality of Service Subscription (QSS) API\"}),`\n`,(0,n.jsxs)(e.p,{children:[(0,n.jsx)(e.a,{href:\"https://explore.zoom.us/en/qss/\",children:\"Zoom Quality of Service Subscription (QSS)\"}),\" is an add-on product that enables Developers to receive details about the network traffic in near real-time for users, hosts, and participants in their meetings, webinars, and phone calls. This data can be used to proactively troubleshoot and identify issues as quickly as possible. \",(0,n.jsx)(e.a,{href:\"https://explore.zoom.us/en/contactsales/\",children:\"Contact Zoom Sales\"}),\" to add this feature.\"]}),`\n`,(0,n.jsx)(e.h2,{id:\"requirements\",children:(0,n.jsxs)(e.a,{href:\"#requirements\",children:[\"Requirements\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsx)(e.p,{children:\"In addition to having the QSS product on your account, at least one user on the account must have Developer permissions enabled to be able to create an application that can subscribe to QSS events. This user should create a Zoom app to receive notifications.\"}),`\n`,(0,n.jsx)(e.h2,{id:\"version-200\",children:(0,n.jsxs)(e.a,{href:\"#version-200\",children:[\"Version: 2.0.0\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Host:\"}),\" api.zoom.us/v2\"]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Protocols:\"}),\" \",(0,n.jsx)(e.code,{children:\"https\"})]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Accepts:\"}),\" \",(0,n.jsx)(e.code,{children:\"application/json\"}),\", \",(0,n.jsx)(e.code,{children:\"multipart/form-data\"})]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Responds With:\"}),\" \",(0,n.jsx)(e.code,{children:\"application/json\"}),\", \",(0,n.jsx)(e.code,{children:\"application/xml\"})]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Contact URL:\"}),\" \",(0,n.jsx)(e.a,{href:\"https://developer.zoom.us/\",children:\"https://developer.zoom.us/\"})]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Terms Of Service\"}),\": \",(0,n.jsx)(e.a,{href:\"https://zoom.us/docs/en-us/zoom_api_license_and_tou.html\",children:\"https://zoom.us/docs/en-us/zoom_api_license_and_tou.html\"})]}),`\n`]}),`\n`,(0,n.jsx)(e.h2,{id:\"to-enable-qss-event-notifications\",children:(0,n.jsxs)(e.a,{href:\"#to-enable-qss-event-notifications\",children:[\"To enable QSS event notifications\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.p,{children:[\"On the \",(0,n.jsx)(e.strong,{children:\"Feature\"}),\" page for the Marketplace app, enable \",(0,n.jsx)(e.strong,{children:\"Event Subscription\"}),\" and add the \",(0,n.jsx)(e.strong,{children:\"Event notification endpoint URL\"}),\" to send the webhooks. Click \",(0,n.jsx)(e.strong,{children:\"Add events\"}),\" and select \",(0,n.jsx)(e.strong,{children:\"QSS\"}),\". Select the QSS events that you\\u2019d like to receive.\"]}),`\n`,(0,n.jsxs)(e.p,{children:[\"See a list of events and their details in \",(0,n.jsx)(e.a,{href:\"/docs/api/qss/events\",children:\"QSS Event reference\"}),\". See \",(0,n.jsx)(e.a,{href:\"/docs/api/webhooks\",children:\"Using webhooks\"}),\" for details about configuring webhooks for event notifications.\"]}),`\n`,(0,n.jsx)(e.h2,{id:\"use-qss-apis\",children:(0,n.jsxs)(e.a,{href:\"#use-qss-apis\",children:[\"Use QSS APIs\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.p,{children:[\"Use QSS APIs to build private services or public applications on the \",(0,n.jsx)(e.a,{href:\"http://marketplace.zoom.us\",children:\"Zoom App Marketplace\"}),\". Note that you must subscribe to the \",(0,n.jsx)(e.strong,{children:\"QSS summary event\"}),\" to receive data from the QoS summary APIs.\"]}),`\n`,(0,n.jsxs)(e.p,{children:[\"See the \",(0,n.jsx)(e.a,{href:\"/docs/api/qss\",children:\"QSS API reference\"}),\" and the \",(0,n.jsx)(e.a,{href:\"/docs/api/qss/ma\",children:\"QSS Master account API reference\"}),\" for details.\"]}),`\n`,(0,n.jsxs)(e.p,{children:[\"To learn how to get your credentials and create private or public applications, see \",(0,n.jsx)(e.a,{href:\"/docs/integrations/oauth/\",children:\"OAuth with Zoom\"}),\". See \",(0,n.jsx)(e.a,{href:\"/docs/api/using-zoom-apis\",children:\"Using Zoom APIs\"}),\" for additional information about using Zoom APIs.\"]}),`\n`,(0,n.jsxs)(e.p,{children:[\"See \",(0,n.jsx)(e.a,{href:\"/docs/api/ma\",children:\"Master Account API\"}),\" to learn more about how to use the Master Accounts API.\"]}),`\n`,(0,n.jsxs)(e.p,{children:[\"All endpoints are available via \",(0,n.jsx)(e.code,{children:\"https\"}),\" and are located at \",(0,n.jsx)(e.code,{children:\"api.zoom.us/v2/\"}),\".\"]})]})}function p(t={}){let{wrapper:e}={...s(),...t.components};return e?(0,n.jsx)(e,{...t,children:(0,n.jsx)(l,{...t})}):l(t)}return Q(q);})();\n;return Component;",
      "frontmatter": {
        "title": "Quality of Service Subscription (QSS) API"
      },
      "errors": [],
      "matter": {
        "content": "\n# Quality of Service Subscription (QSS) API\n\n[Zoom Quality of Service Subscription (QSS)](https://explore.zoom.us/en/qss/) is an add-on product that enables Developers to receive details about the network traffic in near real-time for users, hosts, and participants in their meetings, webinars, and phone calls. This data can be used to proactively troubleshoot and identify issues as quickly as possible. [Contact Zoom Sales](https://explore.zoom.us/en/contactsales/) to add this feature.\n\n## Requirements\n\nIn addition to having the QSS product on your account, at least one user on the account must have Developer permissions enabled to be able to create an application that can subscribe to QSS events. This user should create a Zoom app to receive notifications.\n\n## Version: 2.0.0\n\n* **Host:** api.zoom.us/v2\n* **Protocols:** `https`\n* **Accepts:** `application/json`, `multipart/form-data`\n* **Responds With:** `application/json`, `application/xml`\n* **Contact URL:** https://developer.zoom.us/\n* **Terms Of Service**: https://zoom.us/docs/en-us/zoom_api_license_and_tou.html\n\n## To enable QSS event notifications\n\nOn the **Feature** page for the Marketplace app, enable **Event Subscription** and add the **Event notification endpoint URL** to send the webhooks. Click **Add events** and select **QSS**. Select the QSS events that you\u2019d like to receive.\n\nSee a list of events and their details in [QSS Event reference](/docs/api/qss/events). See [Using webhooks](/docs/api/webhooks) for details about configuring webhooks for event notifications.\n\n## Use QSS APIs\n\nUse QSS APIs to build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us). Note that you must subscribe to the **QSS summary event** to receive data from the QoS summary APIs.\n\nSee the [QSS API reference](/docs/api/qss) and the [QSS Master account API reference](/docs/api/qss/ma) for details.\n\nTo learn how to get your credentials and create private or public applications, see [OAuth with Zoom](/docs/integrations/oauth/). See [Using Zoom APIs](/docs/api/using-zoom-apis) for additional information about using Zoom APIs.\n\nSee [Master Account API](/docs/api/ma) to learn more about how to use the Master Accounts API.\n\nAll endpoints are available via `https` and are located at `api.zoom.us/v2/`.",
        "data": {
          "title": "Quality of Service Subscription (QSS) API"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/rest/qss-api.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
