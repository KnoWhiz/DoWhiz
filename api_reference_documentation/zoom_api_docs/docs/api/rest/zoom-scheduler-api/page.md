# Zoom Scheduler API

- Source URL: https://developers.zoom.us/docs/api/rest/zoom-scheduler-api/
- Snapshot path: docs/api/rest/zoom-scheduler-api
- Generated (UTC): 2026-02-07T22:40:11.535700+00:00

## Frontmatter

```json
{
  "title": "Zoom Scheduler API"
}
```

## Content

```md

# Zoom Scheduler API

You can use Zoom Scheduler APIs to integrate Zoom Scheduler into third-party applications or services, and build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us). 

### Before you begin
- Learn the [basics](/docs/api/using-zoom-apis/) on how to build with Zoom APIs.

- Use our [Postman Public Workspace](https://www.postman.com/zoom-developer) to try [Zoom Scheduler APIs](/docs/api/scheduler). 

- Learn about [webhooks](/docs/api/webhooks/) and how to get data delivered to your designated URL.


## Use cases
Zoom Scheduler APIs empower developers to create customized solutions that leverage the functionality of Zoom Scheduler. Here are a few common use cases.<br/>

**Bookable schedule**<br/>
Share your availability with attendees through your website, email, text, chat, and other channels. Zoom Scheduler automatically marks you as unavailable when you're booked, which allows attendees to select from your open slots.<br/>

**Integrations**<br/>
Connect your Google, Microsoft 365, or Zoom calendars to allow Zoom Scheduler to automatically read your availability and create calendar events on your behalf.<br/>

**Number of attendees**<br/>
Select from one-to-one, one-to-many, or team scheduling options. For instance, create a bookable schedule for one-to-one interviews or panel interviews.<br/>

**Notifications**<br/>
Customize and send automated confirmations, reminders, and follow-up communication through email and SMS.<br/>

**Attendee information**<br/>
Collect basic identifying information or create custom questions to prepare for appointments during the booking process.<br/>

**Branding**<br/>
Customize your booking links and scheduling pages to reflect your brand.

## Endpoints
All endpoints are available through `https` at `api.zoom.us/v2/`. 

## Resources and information
- [**OAuth 2.0 for Zoom**](/docs/integrations/oauth/) <br/>
How to get your credentials and create private or public applications
- **Version** <br/>
  2.0.0
- **Host** <br/>
 `api.zoom.us/v2`
- **Protocols** <br/>
  `https`
- **Accepts** <br/>
 `application/json`, `multipart/form-data`
- **Responds With**<br/>
  `application/json`, `application/xml`
- **Zoom API License and Terms of Use** <br/>
 `https://explore.zoom.us/en/legal/zoom-api-license-and-tou/`
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
      "code": "var Component=(()=>{var p=Object.create;var t=Object.defineProperty;var m=Object.getOwnPropertyDescriptor;var b=Object.getOwnPropertyNames;var f=Object.getPrototypeOf,g=Object.prototype.hasOwnProperty;var y=(o,n)=>()=>(n||o((n={exports:{}}).exports,n),n.exports),w=(o,n)=>{for(var r in n)t(o,r,{get:n[r],enumerable:!0})},l=(o,n,r,c)=>{if(n&&typeof n==\"object\"||typeof n==\"function\")for(let i of b(n))!g.call(o,i)&&i!==r&&t(o,i,{get:()=>n[i],enumerable:!(c=m(n,i))||c.enumerable});return o};var v=(o,n,r)=>(r=o!=null?p(f(o)):{},l(n||!o||!o.__esModule?t(r,\"default\",{value:o,enumerable:!0}):r,o)),x=o=>l(t({},\"__esModule\",{value:!0}),o);var s=y((A,d)=>{d.exports=_jsx_runtime});var S={};w(S,{default:()=>u,frontmatter:()=>Z});var e=v(s());var{useMDXComponents:a}=MdxJsReact;var Z={title:\"Zoom Scheduler API\"};function h(o){let n={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",p:\"p\",strong:\"strong\",ul:\"ul\",...a(),...o.components};return(0,e.jsxs)(e.Fragment,{children:[(0,e.jsx)(n.h1,{id:\"zoom-scheduler-api\",children:\"Zoom Scheduler API\"}),`\n`,(0,e.jsxs)(n.p,{children:[\"You can use Zoom Scheduler APIs to integrate Zoom Scheduler into third-party applications or services, and build private services or public applications on the \",(0,e.jsx)(n.a,{href:\"http://marketplace.zoom.us\",children:\"Zoom App Marketplace\"}),\".\"]}),`\n`,(0,e.jsx)(n.h3,{id:\"before-you-begin\",children:(0,e.jsxs)(n.a,{href:\"#before-you-begin\",children:[\"Before you begin\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Learn the \",(0,e.jsx)(n.a,{href:\"/docs/api/using-zoom-apis/\",children:\"basics\"}),\" on how to build with Zoom APIs.\"]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Use our \",(0,e.jsx)(n.a,{href:\"https://www.postman.com/zoom-developer\",children:\"Postman Public Workspace\"}),\" to try \",(0,e.jsx)(n.a,{href:\"/docs/api/scheduler\",children:\"Zoom Scheduler APIs\"}),\".\"]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Learn about \",(0,e.jsx)(n.a,{href:\"/docs/api/webhooks/\",children:\"webhooks\"}),\" and how to get data delivered to your designated URL.\"]}),`\n`]}),`\n`]}),`\n`,(0,e.jsx)(n.h2,{id:\"use-cases\",children:(0,e.jsxs)(n.a,{href:\"#use-cases\",children:[\"Use cases\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.p,{children:[\"Zoom Scheduler APIs empower developers to create customized solutions that leverage the functionality of Zoom Scheduler. Here are a few common use cases.\",(0,e.jsx)(\"br\",{})]}),`\n`,(0,e.jsxs)(n.p,{children:[(0,e.jsx)(n.strong,{children:\"Bookable schedule\"}),(0,e.jsx)(\"br\",{}),`\nShare your availability with attendees through your website, email, text, chat, and other channels. Zoom Scheduler automatically marks you as unavailable when you're booked, which allows attendees to select from your open slots.`,(0,e.jsx)(\"br\",{})]}),`\n`,(0,e.jsxs)(n.p,{children:[(0,e.jsx)(n.strong,{children:\"Integrations\"}),(0,e.jsx)(\"br\",{}),`\nConnect your Google, Microsoft 365, or Zoom calendars to allow Zoom Scheduler to automatically read your availability and create calendar events on your behalf.`,(0,e.jsx)(\"br\",{})]}),`\n`,(0,e.jsxs)(n.p,{children:[(0,e.jsx)(n.strong,{children:\"Number of attendees\"}),(0,e.jsx)(\"br\",{}),`\nSelect from one-to-one, one-to-many, or team scheduling options. For instance, create a bookable schedule for one-to-one interviews or panel interviews.`,(0,e.jsx)(\"br\",{})]}),`\n`,(0,e.jsxs)(n.p,{children:[(0,e.jsx)(n.strong,{children:\"Notifications\"}),(0,e.jsx)(\"br\",{}),`\nCustomize and send automated confirmations, reminders, and follow-up communication through email and SMS.`,(0,e.jsx)(\"br\",{})]}),`\n`,(0,e.jsxs)(n.p,{children:[(0,e.jsx)(n.strong,{children:\"Attendee information\"}),(0,e.jsx)(\"br\",{}),`\nCollect basic identifying information or create custom questions to prepare for appointments during the booking process.`,(0,e.jsx)(\"br\",{})]}),`\n`,(0,e.jsxs)(n.p,{children:[(0,e.jsx)(n.strong,{children:\"Branding\"}),(0,e.jsx)(\"br\",{}),`\nCustomize your booking links and scheduling pages to reflect your brand.`]}),`\n`,(0,e.jsx)(n.h2,{id:\"endpoints\",children:(0,e.jsxs)(n.a,{href:\"#endpoints\",children:[\"Endpoints\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.p,{children:[\"All endpoints are available through \",(0,e.jsx)(n.code,{children:\"https\"}),\" at \",(0,e.jsx)(n.code,{children:\"api.zoom.us/v2/\"}),\".\"]}),`\n`,(0,e.jsx)(n.h2,{id:\"resources-and-information\",children:(0,e.jsxs)(n.a,{href:\"#resources-and-information\",children:[\"Resources and information\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.a,{href:\"/docs/integrations/oauth/\",children:(0,e.jsx)(n.strong,{children:\"OAuth 2.0 for Zoom\"})}),\" \",(0,e.jsx)(\"br\",{}),`\nHow to get your credentials and create private or public applications`]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Version\"}),\" \",(0,e.jsx)(\"br\",{}),`\n2.0.0`]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Host\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"api.zoom.us/v2\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Protocols\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"https\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Accepts\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"application/json\"}),\", \",(0,e.jsx)(n.code,{children:\"multipart/form-data\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Responds With\"}),(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"application/json\"}),\", \",(0,e.jsx)(n.code,{children:\"application/xml\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Zoom API License and Terms of Use\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"https://explore.zoom.us/en/legal/zoom-api-license-and-tou/\"})]}),`\n`]})]})}function u(o={}){let{wrapper:n}={...a(),...o.components};return n?(0,e.jsx)(n,{...o,children:(0,e.jsx)(h,{...o})}):h(o)}return x(S);})();\n;return Component;",
      "frontmatter": {
        "title": "Zoom Scheduler API"
      },
      "errors": [],
      "matter": {
        "content": "\n# Zoom Scheduler API\n\nYou can use Zoom Scheduler APIs to integrate Zoom Scheduler into third-party applications or services, and build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us). \n\n### Before you begin\n- Learn the [basics](/docs/api/using-zoom-apis/) on how to build with Zoom APIs.\n\n- Use our [Postman Public Workspace](https://www.postman.com/zoom-developer) to try [Zoom Scheduler APIs](/docs/api/scheduler). \n\n- Learn about [webhooks](/docs/api/webhooks/) and how to get data delivered to your designated URL.\n\n\n## Use cases\nZoom Scheduler APIs empower developers to create customized solutions that leverage the functionality of Zoom Scheduler. Here are a few common use cases.<br/>\n\n**Bookable schedule**<br/>\nShare your availability with attendees through your website, email, text, chat, and other channels. Zoom Scheduler automatically marks you as unavailable when you're booked, which allows attendees to select from your open slots.<br/>\n\n**Integrations**<br/>\nConnect your Google, Microsoft 365, or Zoom calendars to allow Zoom Scheduler to automatically read your availability and create calendar events on your behalf.<br/>\n\n**Number of attendees**<br/>\nSelect from one-to-one, one-to-many, or team scheduling options. For instance, create a bookable schedule for one-to-one interviews or panel interviews.<br/>\n\n**Notifications**<br/>\nCustomize and send automated confirmations, reminders, and follow-up communication through email and SMS.<br/>\n\n**Attendee information**<br/>\nCollect basic identifying information or create custom questions to prepare for appointments during the booking process.<br/>\n\n**Branding**<br/>\nCustomize your booking links and scheduling pages to reflect your brand.\n\n## Endpoints\nAll endpoints are available through `https` at `api.zoom.us/v2/`. \n\n## Resources and information\n- [**OAuth 2.0 for Zoom**](/docs/integrations/oauth/) <br/>\nHow to get your credentials and create private or public applications\n- **Version** <br/>\n  2.0.0\n- **Host** <br/>\n `api.zoom.us/v2`\n- **Protocols** <br/>\n  `https`\n- **Accepts** <br/>\n `application/json`, `multipart/form-data`\n- **Responds With**<br/>\n  `application/json`, `application/xml`\n- **Zoom API License and Terms of Use** <br/>\n `https://explore.zoom.us/en/legal/zoom-api-license-and-tou/`",
        "data": {
          "title": "Zoom Scheduler API"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/rest/zoom-scheduler-api.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
