# Legacy HIPAA Business Associate Agreement Accounts

- Source URL: https://developers.zoom.us/docs/api/references/legacy-business-associate-agreements/
- Snapshot path: docs/api/references/legacy-business-associate-agreements
- Generated (UTC): 2026-02-07T22:40:11.512148+00:00

## Frontmatter

```json
{
  "title": "Legacy HIPAA Business Associate Agreement Accounts"
}
```

## Content

```md

# Legacy HIPAA Business Associate Agreement Accounts

If your account has **not** signed the updated November 2020 [HIPAA business associate agreement (BAA)](https://support.zoom.us/hc/en-us/articles/207652183-HIPAA-Business-Associate-Agreement-BAA), some Zoom APIs will **not** return users' [Protected Health Information (PHI)](https://www.hhs.gov/answers/hipaa/what-is-phi/index.html).

Users who sign the updated (November 2020) HIPAA business associate agreement are **not** restricted.

> **Note:** For users who migrate from a legacy HIPAA BAA to the updated BAA, any historical data under the previous (legacy) BAA will remain hidden **except** participant email addresses.

Legacy HIPAA business associate agreements are considered those which were signed **prior** to November 2020. Restrictions under this signed BAA include:

* No PHI exposed via meeting reports or meeting/webinar Dashboard-related APIs.
* Disabled and hidden cloud recording feature.
* Enhanced encryption is enabled and **cannot** be disabled.
* In meeting chats **cannot** be copied or saved.
* The **Require Encryption for 3rd Part Endpoints (H.323/SIP)** is enabled and **cannot** be disabled.

Under the legacy BAA **without** a data processing addendum, reports containing PHI will behave as follows:

* Meeting participant reports will **not** display users' PHI. However, webinar attendee reports will display users' PHI.
* [Dashboard](/docs/api/accounts/#tag/dashboards) API responses for meeting **and** webinar participants will **not** display users' PHI. This also includes Dashboard CSV exports.

For information on how to sign a new BAA or sign a data processing addendum, [contact Zoom Sales](https://zoom.us/contactsales).

## Legacy BAAs and API responses

An account that calls a BAA-restricted API under the legacy BAA **without** a signed data processing addendum cannot view the user's following information:

* Usernames.
* IP addresses.
* The user's location.
* The user's email address.

Users that sign a data processing addendum are given limited access to users' PHI. However, they still cannot view the following information:

* The user's location.
* The user's IP address.

### APIs

The following APIs **do not** return user PHI under the legacy BAA without a signed data processing addendum:

### Dashboard APIs

* [List meeting participants](/docs/api/accounts/#tag/dashboards/GET/metrics/meetings/\{meetingId\}/participants)
* [Get meeting participant QoS](/docs/api/accounts/#tag/dashboards/GET/metrics/meetings/\{meetingId\}/participants/\{participantId\}/qos)
* [List meeting participant QoS](/docs/api/accounts/#tag/dashboards/GET/metrics/meetings/\{meetingId\}/participants/qos)
* [Get webinar participants](/docs/api/accounts/#tag/dashboards/GET/metrics/webinars/\{webinarId\}/participants)
* [Get webinar participant QoS](/docs/api/accounts/#tag/dashboards/GET/metrics/webinars/\{webinarId\}/participants/\{participantId\}/qos)
* [List webinar participant QoS](/docs/api/accounts/#tag/dashboards/GET/metrics/webinars/\{webinarId\}/participants/qos)

### Reports APIs

* [Get meeting participant reports](/docs/api/meetings/#tag/reports/GET/report/meetings/\{meetingId\}/participants)
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
      "code": "var Component=(()=>{var g=Object.create;var a=Object.defineProperty;var u=Object.getOwnPropertyDescriptor;var m=Object.getOwnPropertyNames;var A=Object.getPrototypeOf,b=Object.prototype.hasOwnProperty;var w=(i,e)=>()=>(e||i((e={exports:{}}).exports,e),e.exports),I=(i,e)=>{for(var r in e)a(i,r,{get:e[r],enumerable:!0})},o=(i,e,r,d)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let s of m(e))!b.call(i,s)&&s!==r&&a(i,s,{get:()=>e[s],enumerable:!(d=u(e,s))||d.enumerable});return i};var f=(i,e,r)=>(r=i!=null?g(A(i)):{},o(e||!i||!i.__esModule?a(r,\"default\",{value:i,enumerable:!0}):r,i)),P=i=>o(a({},\"__esModule\",{value:!0}),i);var h=w((B,c)=>{c.exports=_jsx_runtime});var H={};I(H,{default:()=>p,frontmatter:()=>y});var n=f(h());var{useMDXComponents:t}=MdxJsReact;var y={title:\"Legacy HIPAA Business Associate Agreement Accounts\"};function l(i){let e={a:\"a\",blockquote:\"blockquote\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",p:\"p\",strong:\"strong\",ul:\"ul\",...t(),...i.components};return(0,n.jsxs)(n.Fragment,{children:[(0,n.jsx)(e.h1,{id:\"legacy-hipaa-business-associate-agreement-accounts\",children:\"Legacy HIPAA Business Associate Agreement Accounts\"}),`\n`,(0,n.jsxs)(e.p,{children:[\"If your account has \",(0,n.jsx)(e.strong,{children:\"not\"}),\" signed the updated November 2020 \",(0,n.jsx)(e.a,{href:\"https://support.zoom.us/hc/en-us/articles/207652183-HIPAA-Business-Associate-Agreement-BAA\",children:\"HIPAA business associate agreement (BAA)\"}),\", some Zoom APIs will \",(0,n.jsx)(e.strong,{children:\"not\"}),\" return users' \",(0,n.jsx)(e.a,{href:\"https://www.hhs.gov/answers/hipaa/what-is-phi/index.html\",children:\"Protected Health Information (PHI)\"}),\".\"]}),`\n`,(0,n.jsxs)(e.p,{children:[\"Users who sign the updated (November 2020) HIPAA business associate agreement are \",(0,n.jsx)(e.strong,{children:\"not\"}),\" restricted.\"]}),`\n`,(0,n.jsxs)(e.blockquote,{children:[`\n`,(0,n.jsxs)(e.p,{children:[(0,n.jsx)(e.strong,{children:\"Note:\"}),\" For users who migrate from a legacy HIPAA BAA to the updated BAA, any historical data under the previous (legacy) BAA will remain hidden \",(0,n.jsx)(e.strong,{children:\"except\"}),\" participant email addresses.\"]}),`\n`]}),`\n`,(0,n.jsxs)(e.p,{children:[\"Legacy HIPAA business associate agreements are considered those which were signed \",(0,n.jsx)(e.strong,{children:\"prior\"}),\" to November 2020. Restrictions under this signed BAA include:\"]}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsx)(e.li,{children:\"No PHI exposed via meeting reports or meeting/webinar Dashboard-related APIs.\"}),`\n`,(0,n.jsx)(e.li,{children:\"Disabled and hidden cloud recording feature.\"}),`\n`,(0,n.jsxs)(e.li,{children:[\"Enhanced encryption is enabled and \",(0,n.jsx)(e.strong,{children:\"cannot\"}),\" be disabled.\"]}),`\n`,(0,n.jsxs)(e.li,{children:[\"In meeting chats \",(0,n.jsx)(e.strong,{children:\"cannot\"}),\" be copied or saved.\"]}),`\n`,(0,n.jsxs)(e.li,{children:[\"The \",(0,n.jsx)(e.strong,{children:\"Require Encryption for 3rd Part Endpoints (H.323/SIP)\"}),\" is enabled and \",(0,n.jsx)(e.strong,{children:\"cannot\"}),\" be disabled.\"]}),`\n`]}),`\n`,(0,n.jsxs)(e.p,{children:[\"Under the legacy BAA \",(0,n.jsx)(e.strong,{children:\"without\"}),\" a data processing addendum, reports containing PHI will behave as follows:\"]}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsxs)(e.li,{children:[\"Meeting participant reports will \",(0,n.jsx)(e.strong,{children:\"not\"}),\" display users' PHI. However, webinar attendee reports will display users' PHI.\"]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.a,{href:\"/docs/api/accounts/#tag/dashboards\",children:\"Dashboard\"}),\" API responses for meeting \",(0,n.jsx)(e.strong,{children:\"and\"}),\" webinar participants will \",(0,n.jsx)(e.strong,{children:\"not\"}),\" display users' PHI. This also includes Dashboard CSV exports.\"]}),`\n`]}),`\n`,(0,n.jsxs)(e.p,{children:[\"For information on how to sign a new BAA or sign a data processing addendum, \",(0,n.jsx)(e.a,{href:\"https://zoom.us/contactsales\",children:\"contact Zoom Sales\"}),\".\"]}),`\n`,(0,n.jsx)(e.h2,{id:\"legacy-baas-and-api-responses\",children:(0,n.jsxs)(e.a,{href:\"#legacy-baas-and-api-responses\",children:[\"Legacy BAAs and API responses\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.p,{children:[\"An account that calls a BAA-restricted API under the legacy BAA \",(0,n.jsx)(e.strong,{children:\"without\"}),\" a signed data processing addendum cannot view the user's following information:\"]}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsx)(e.li,{children:\"Usernames.\"}),`\n`,(0,n.jsx)(e.li,{children:\"IP addresses.\"}),`\n`,(0,n.jsx)(e.li,{children:\"The user's location.\"}),`\n`,(0,n.jsx)(e.li,{children:\"The user's email address.\"}),`\n`]}),`\n`,(0,n.jsx)(e.p,{children:\"Users that sign a data processing addendum are given limited access to users' PHI. However, they still cannot view the following information:\"}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsx)(e.li,{children:\"The user's location.\"}),`\n`,(0,n.jsx)(e.li,{children:\"The user's IP address.\"}),`\n`]}),`\n`,(0,n.jsx)(e.h3,{id:\"apis\",children:(0,n.jsxs)(e.a,{href:\"#apis\",children:[\"APIs\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.p,{children:[\"The following APIs \",(0,n.jsx)(e.strong,{children:\"do not\"}),\" return user PHI under the legacy BAA without a signed data processing addendum:\"]}),`\n`,(0,n.jsx)(e.h3,{id:\"dashboard-apis\",children:(0,n.jsxs)(e.a,{href:\"#dashboard-apis\",children:[\"Dashboard APIs\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsx)(e.li,{children:(0,n.jsx)(e.a,{href:\"/docs/api/accounts/#tag/dashboards/get/metrics/meetings/{meetingId}/participants\",children:\"List meeting participants\"})}),`\n`,(0,n.jsx)(e.li,{children:(0,n.jsx)(e.a,{href:\"/docs/api/accounts/#tag/dashboards/get/metrics/meetings/{meetingId}/participants/{participantId}/qos\",children:\"Get meeting participant QoS\"})}),`\n`,(0,n.jsx)(e.li,{children:(0,n.jsx)(e.a,{href:\"/docs/api/accounts/#tag/dashboards/get/metrics/meetings/{meetingId}/participants/qos\",children:\"List meeting participant QoS\"})}),`\n`,(0,n.jsx)(e.li,{children:(0,n.jsx)(e.a,{href:\"/docs/api/accounts/#tag/dashboards/get/metrics/webinars/{webinarId}/participants\",children:\"Get webinar participants\"})}),`\n`,(0,n.jsx)(e.li,{children:(0,n.jsx)(e.a,{href:\"/docs/api/accounts/#tag/dashboards/get/metrics/webinars/{webinarId}/participants/{participantId}/qos\",children:\"Get webinar participant QoS\"})}),`\n`,(0,n.jsx)(e.li,{children:(0,n.jsx)(e.a,{href:\"/docs/api/accounts/#tag/dashboards/get/metrics/webinars/{webinarId}/participants/qos\",children:\"List webinar participant QoS\"})}),`\n`]}),`\n`,(0,n.jsx)(e.h3,{id:\"reports-apis\",children:(0,n.jsxs)(e.a,{href:\"#reports-apis\",children:[\"Reports APIs\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsx)(e.li,{children:(0,n.jsx)(e.a,{href:\"/docs/api/meetings/#tag/reports/get/report/meetings/{meetingId}/participants\",children:\"Get meeting participant reports\"})}),`\n`]})]})}function p(i={}){let{wrapper:e}={...t(),...i.components};return e?(0,n.jsx)(e,{...i,children:(0,n.jsx)(l,{...i})}):l(i)}return P(H);})();\n;return Component;",
      "frontmatter": {
        "title": "Legacy HIPAA Business Associate Agreement Accounts"
      },
      "errors": [],
      "matter": {
        "content": "\n# Legacy HIPAA Business Associate Agreement Accounts\n\nIf your account has **not** signed the updated November 2020 [HIPAA business associate agreement (BAA)](https://support.zoom.us/hc/en-us/articles/207652183-HIPAA-Business-Associate-Agreement-BAA), some Zoom APIs will **not** return users' [Protected Health Information (PHI)](https://www.hhs.gov/answers/hipaa/what-is-phi/index.html).\n\nUsers who sign the updated (November 2020) HIPAA business associate agreement are **not** restricted.\n\n> **Note:** For users who migrate from a legacy HIPAA BAA to the updated BAA, any historical data under the previous (legacy) BAA will remain hidden **except** participant email addresses.\n\nLegacy HIPAA business associate agreements are considered those which were signed **prior** to November 2020. Restrictions under this signed BAA include:\n\n* No PHI exposed via meeting reports or meeting/webinar Dashboard-related APIs.\n* Disabled and hidden cloud recording feature.\n* Enhanced encryption is enabled and **cannot** be disabled.\n* In meeting chats **cannot** be copied or saved.\n* The **Require Encryption for 3rd Part Endpoints (H.323/SIP)** is enabled and **cannot** be disabled.\n\nUnder the legacy BAA **without** a data processing addendum, reports containing PHI will behave as follows:\n\n* Meeting participant reports will **not** display users' PHI. However, webinar attendee reports will display users' PHI.\n* [Dashboard](/docs/api/accounts/#tag/dashboards) API responses for meeting **and** webinar participants will **not** display users' PHI. This also includes Dashboard CSV exports.\n\nFor information on how to sign a new BAA or sign a data processing addendum, [contact Zoom Sales](https://zoom.us/contactsales).\n\n## Legacy BAAs and API responses\n\nAn account that calls a BAA-restricted API under the legacy BAA **without** a signed data processing addendum cannot view the user's following information:\n\n* Usernames.\n* IP addresses.\n* The user's location.\n* The user's email address.\n\nUsers that sign a data processing addendum are given limited access to users' PHI. However, they still cannot view the following information:\n\n* The user's location.\n* The user's IP address.\n\n### APIs\n\nThe following APIs **do not** return user PHI under the legacy BAA without a signed data processing addendum:\n\n### Dashboard APIs\n\n* [List meeting participants](/docs/api/accounts/#tag/dashboards/GET/metrics/meetings/\\{meetingId\\}/participants)\n* [Get meeting participant QoS](/docs/api/accounts/#tag/dashboards/GET/metrics/meetings/\\{meetingId\\}/participants/\\{participantId\\}/qos)\n* [List meeting participant QoS](/docs/api/accounts/#tag/dashboards/GET/metrics/meetings/\\{meetingId\\}/participants/qos)\n* [Get webinar participants](/docs/api/accounts/#tag/dashboards/GET/metrics/webinars/\\{webinarId\\}/participants)\n* [Get webinar participant QoS](/docs/api/accounts/#tag/dashboards/GET/metrics/webinars/\\{webinarId\\}/participants/\\{participantId\\}/qos)\n* [List webinar participant QoS](/docs/api/accounts/#tag/dashboards/GET/metrics/webinars/\\{webinarId\\}/participants/qos)\n\n### Reports APIs\n\n* [Get meeting participant reports](/docs/api/meetings/#tag/reports/GET/report/meetings/\\{meetingId\\}/participants)\n",
        "data": {
          "title": "Legacy HIPAA Business Associate Agreement Accounts"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/references/legacy-business-associate-agreements.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
