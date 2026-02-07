# Rate limits

- Source URL: https://developers.zoom.us/docs/api/rate-limits/
- Snapshot path: docs/api/rate-limits
- Generated (UTC): 2026-02-07T22:40:11.503779+00:00

## Frontmatter

```json
{
  "title": "Rate limits"
}
```

## Content

```md

# Rate limits

Rate limits define the maximum number of requests a single account can make within a given period of time. When you exceed a rate limit, the API request will fail and return a **HTTP 429** status code.

## Rate limit details

- **Rate limits are applied at the account level.** These rate limits are shared by users for all apps installed on the account.
- **Rate limits are applied based on your account plan.** Increased rate limits are available for all Business, Enterprise, Education, and Partner accounts. [See pricing for details](https://zoom.us/pricing).
- **APIs are grouped by Request Type with unique rate limits for each.** The documentation for each API endpoint includes a "Rate Limit Label".
- Some APIs have special rate limits at the **user-level**.
- Some APIs have **limits on concurrent operations** performed on a single resource.

<Alert>
  The Zoom API uses rate limits to ensure that API request traffic is efficiently handled. We may alter these limits as
  needed. See the <a href="/changelog">changelog</a> for details.
</Alert>

## Rate limits by account type

Rate limits are applied based on the account plan, either **Free**, **Pro**, and **Business+** (which includes Business, Education, Enterprise & Partners). [Meeting SDK](/docs/meeting-sdk/) is included in these account types. This outlines the rate limits applied to APIs by rate limit type for each account plan.

| Rate Limit Type         | Free                  | Pro         | Business+   |
| ----------------------- | --------------------- | ----------- | ----------- |
| Light APIs              | 4/second, 6000/day    | 30/second   | 80/second   |
| Medium APIs             | 2/second, 2000/day    | 20/second   | 60/second   |
| Heavy APIs              | 1/second, 1000/day    | 10/second\* | 40/second\* |
| Resource-intensive APIs | 10/minute, 30,000/day | 10/minute\* | 20/minute\* |

\* Subject to combined daily rate limits:

- Pro: 30,000 requests/day for heavy and resource-intensive APIs
- Business+: 60,000 requests/day for heavy and resource-intensive APIs

### Video SDK account rate limits

Video SDK is a [separate account type](/docs/video-sdk/developer-accounts/). _See [Video SDK Plans & Pricing for Developer](https://zoom.us/pricing/developer) for pricing._ Video SDK REST API rate limits are tied to the account's plan type.

- **Video SDK Pay As You Go** *(Deprecated)* - Uses the **Pro** rate limits.
- **Video SDK Annual Prepay Monthly Usage** - Uses the **Pro** rate limits.
- **All other Video SDK plans** (including Video SDK Monthly Prepay Monthly Usage, Video SDK Universal credit, and others) - Uses the **Business+** rate limits.

## User-level rate limits

These are some special user-level rate limits.

### Meeting and Webinar create/update requests
- 100 create/update requests per day (UTC) per user
- Applies to all Meeting/Webinar IDs hosted by the user
- Resets at 00:00 UTC daily

### Registration requests

- 3 requests per day (UTC) for the same registrant in the same meeting/webinar

### Registrant status requests

- 10 requests per day (UTC) for the same registrant in the same meeting/webinar

## Zoom Phone APIs

| Rate Limit Type         | Pro                    | Business+               |
| ----------------------- | ---------------------- | ----------------------- |
| Light APIs              | 20/second              | 40/second               |
| Medium APIs             | 10/second              | 20/second               |
| Heavy APIs              | 5/second, 15,000/day\* | 10/second, 30,000/day\* |
| Resource-intensive APIs | 5/minute, 15,000/day\* | 10/minute, 30,000/day\* |

\* Daily limit shared between heavy and resource-intensive APIs

## Zoom Contact Center APIs

Zoom Contact Center APIs are rate limited by request type and applied based on the account plan:

| Rate Limit Type | Pro                    | Business+               |
| --------------- | ---------------------- | ----------------------- |
| Light APIs      | 20/second              | 40/second               |
| Medium APIs     | 10/second              | 20/second               |
| Heavy APIs      | 5/second, 15,000/day\* | 10/second, 30,000/day\* |

\* Daily limit shared with resource-intensive APIs

## Concurrent request limits

Concurrent requests to a single resource may fail with a 429 HTTP status code. For example:

```
Too many concurrent requests. A request to disassociate this user has already been made.
```

## Error handling best practices

- Handle 429 status codes by implementing a wait before retrying.
- Build retry mechanisms for failed requests.
- Reduce request frequency where possible.
- Cache responses to minimize repetitive requests.
- Use [**webhooks**](/docs/api/webhooks) for data change events instead of polling.

### Error table

This table lists errors you may encounter.

| Limit Type | HTTP Status | Error Message                                                                                                                              |
| ---------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| Per-second | 429         | `You have reached the maximum per-second rate limit for this API. Try again later.`                                                          |
| Daily      | 429         | `You have reached the maximum daily rate limit for this API. Refer to the response header for details on when you can make another request.` |
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
      "code": "var Component=(()=>{var m=Object.create;var d=Object.defineProperty;var p=Object.getOwnPropertyDescriptor;var y=Object.getOwnPropertyNames;var g=Object.getPrototypeOf,f=Object.prototype.hasOwnProperty;var b=(r,e)=>()=>(e||r((e={exports:{}}).exports,e),e.exports),P=(r,e)=>{for(var n in e)d(r,n,{get:e[n],enumerable:!0})},a=(r,e,n,s)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let t of y(e))!f.call(r,t)&&t!==n&&d(r,t,{get:()=>e[t],enumerable:!(s=p(e,t))||s.enumerable});return r};var A=(r,e,n)=>(n=r!=null?m(g(r)):{},a(e||!r||!r.__esModule?d(n,\"default\",{value:r,enumerable:!0}):n,r)),v=r=>a(d({},\"__esModule\",{value:!0}),r);var h=b((D,c)=>{c.exports=_jsx_runtime});var T={};P(T,{default:()=>u,frontmatter:()=>q});var i=A(h());var{useMDXComponents:l}=MdxJsReact;var q={title:\"Rate limits\"};function o(r){let e={a:\"a\",code:\"code\",em:\"em\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",p:\"p\",pre:\"pre\",strong:\"strong\",table:\"table\",tbody:\"tbody\",td:\"td\",th:\"th\",thead:\"thead\",tr:\"tr\",ul:\"ul\",...l(),...r.components},{Alert:n}=e;return n||I(\"Alert\",!0),(0,i.jsxs)(i.Fragment,{children:[(0,i.jsx)(e.h1,{id:\"rate-limits\",children:\"Rate limits\"}),`\n`,(0,i.jsxs)(e.p,{children:[\"Rate limits define the maximum number of requests a single account can make within a given period of time. When you exceed a rate limit, the API request will fail and return a \",(0,i.jsx)(e.strong,{children:\"HTTP 429\"}),\" status code.\"]}),`\n`,(0,i.jsx)(e.h2,{id:\"rate-limit-details\",children:(0,i.jsxs)(e.a,{href:\"#rate-limit-details\",children:[\"Rate limit details\",(0,i.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,i.jsxs)(e.ul,{children:[`\n`,(0,i.jsxs)(e.li,{children:[(0,i.jsx)(e.strong,{children:\"Rate limits are applied at the account level.\"}),\" These rate limits are shared by users for all apps installed on the account.\"]}),`\n`,(0,i.jsxs)(e.li,{children:[(0,i.jsx)(e.strong,{children:\"Rate limits are applied based on your account plan.\"}),\" Increased rate limits are available for all Business, Enterprise, Education, and Partner accounts. \",(0,i.jsx)(e.a,{href:\"https://zoom.us/pricing\",children:\"See pricing for details\"}),\".\"]}),`\n`,(0,i.jsxs)(e.li,{children:[(0,i.jsx)(e.strong,{children:\"APIs are grouped by Request Type with unique rate limits for each.\"}),' The documentation for each API endpoint includes a \"Rate Limit Label\".']}),`\n`,(0,i.jsxs)(e.li,{children:[\"Some APIs have special rate limits at the \",(0,i.jsx)(e.strong,{children:\"user-level\"}),\".\"]}),`\n`,(0,i.jsxs)(e.li,{children:[\"Some APIs have \",(0,i.jsx)(e.strong,{children:\"limits on concurrent operations\"}),\" performed on a single resource.\"]}),`\n`]}),`\n`,(0,i.jsx)(n,{children:(0,i.jsxs)(e.p,{children:[`The Zoom API uses rate limits to ensure that API request traffic is efficiently handled. We may alter these limits as\nneeded. See the `,(0,i.jsx)(\"a\",{href:\"/changelog\",children:\"changelog\"}),\" for details.\"]})}),`\n`,(0,i.jsx)(e.h2,{id:\"rate-limits-by-account-type\",children:(0,i.jsxs)(e.a,{href:\"#rate-limits-by-account-type\",children:[\"Rate limits by account type\",(0,i.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,i.jsxs)(e.p,{children:[\"Rate limits are applied based on the account plan, either \",(0,i.jsx)(e.strong,{children:\"Free\"}),\", \",(0,i.jsx)(e.strong,{children:\"Pro\"}),\", and \",(0,i.jsx)(e.strong,{children:\"Business+\"}),\" (which includes Business, Education, Enterprise & Partners). \",(0,i.jsx)(e.a,{href:\"/docs/meeting-sdk/\",children:\"Meeting SDK\"}),\" is included in these account types. This outlines the rate limits applied to APIs by rate limit type for each account plan.\"]}),`\n`,(0,i.jsxs)(e.table,{children:[(0,i.jsx)(e.thead,{children:(0,i.jsxs)(e.tr,{children:[(0,i.jsx)(e.th,{children:\"Rate Limit Type\"}),(0,i.jsx)(e.th,{children:\"Free\"}),(0,i.jsx)(e.th,{children:\"Pro\"}),(0,i.jsx)(e.th,{children:\"Business+\"})]})}),(0,i.jsxs)(e.tbody,{children:[(0,i.jsxs)(e.tr,{children:[(0,i.jsx)(e.td,{children:\"Light APIs\"}),(0,i.jsx)(e.td,{children:\"4/second, 6000/day\"}),(0,i.jsx)(e.td,{children:\"30/second\"}),(0,i.jsx)(e.td,{children:\"80/second\"})]}),(0,i.jsxs)(e.tr,{children:[(0,i.jsx)(e.td,{children:\"Medium APIs\"}),(0,i.jsx)(e.td,{children:\"2/second, 2000/day\"}),(0,i.jsx)(e.td,{children:\"20/second\"}),(0,i.jsx)(e.td,{children:\"60/second\"})]}),(0,i.jsxs)(e.tr,{children:[(0,i.jsx)(e.td,{children:\"Heavy APIs\"}),(0,i.jsx)(e.td,{children:\"1/second, 1000/day\"}),(0,i.jsx)(e.td,{children:\"10/second*\"}),(0,i.jsx)(e.td,{children:\"40/second*\"})]}),(0,i.jsxs)(e.tr,{children:[(0,i.jsx)(e.td,{children:\"Resource-intensive APIs\"}),(0,i.jsx)(e.td,{children:\"10/minute, 30,000/day\"}),(0,i.jsx)(e.td,{children:\"10/minute*\"}),(0,i.jsx)(e.td,{children:\"20/minute*\"})]})]})]}),`\n`,(0,i.jsx)(e.p,{children:\"* Subject to combined daily rate limits:\"}),`\n`,(0,i.jsxs)(e.ul,{children:[`\n`,(0,i.jsx)(e.li,{children:\"Pro: 30,000 requests/day for heavy and resource-intensive APIs\"}),`\n`,(0,i.jsx)(e.li,{children:\"Business+: 60,000 requests/day for heavy and resource-intensive APIs\"}),`\n`]}),`\n`,(0,i.jsx)(e.h3,{id:\"video-sdk-account-rate-limits\",children:(0,i.jsxs)(e.a,{href:\"#video-sdk-account-rate-limits\",children:[\"Video SDK account rate limits\",(0,i.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,i.jsxs)(e.p,{children:[\"Video SDK is a \",(0,i.jsx)(e.a,{href:\"/docs/video-sdk/developer-accounts/\",children:\"separate account type\"}),\". \",(0,i.jsxs)(e.em,{children:[\"See \",(0,i.jsx)(e.a,{href:\"https://zoom.us/pricing/developer\",children:\"Video SDK Plans & Pricing for Developer\"}),\" for pricing.\"]}),\" Video SDK REST API rate limits are tied to the account's plan type.\"]}),`\n`,(0,i.jsxs)(e.ul,{children:[`\n`,(0,i.jsxs)(e.li,{children:[(0,i.jsx)(e.strong,{children:\"Video SDK Pay As You Go\"}),\" \",(0,i.jsx)(e.em,{children:\"(Deprecated)\"}),\" - Uses the \",(0,i.jsx)(e.strong,{children:\"Pro\"}),\" rate limits.\"]}),`\n`,(0,i.jsxs)(e.li,{children:[(0,i.jsx)(e.strong,{children:\"Video SDK Annual Prepay Monthly Usage\"}),\" - Uses the \",(0,i.jsx)(e.strong,{children:\"Pro\"}),\" rate limits.\"]}),`\n`,(0,i.jsxs)(e.li,{children:[(0,i.jsx)(e.strong,{children:\"All other Video SDK plans\"}),\" (including Video SDK Monthly Prepay Monthly Usage, Video SDK Universal credit, and others) - Uses the \",(0,i.jsx)(e.strong,{children:\"Business+\"}),\" rate limits.\"]}),`\n`]}),`\n`,(0,i.jsx)(e.h2,{id:\"user-level-rate-limits\",children:(0,i.jsxs)(e.a,{href:\"#user-level-rate-limits\",children:[\"User-level rate limits\",(0,i.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,i.jsx)(e.p,{children:\"These are some special user-level rate limits.\"}),`\n`,(0,i.jsx)(e.h3,{id:\"meeting-and-webinar-createupdate-requests\",children:(0,i.jsxs)(e.a,{href:\"#meeting-and-webinar-createupdate-requests\",children:[\"Meeting and Webinar create/update requests\",(0,i.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,i.jsxs)(e.ul,{children:[`\n`,(0,i.jsx)(e.li,{children:\"100 create/update requests per day (UTC) per user\"}),`\n`,(0,i.jsx)(e.li,{children:\"Applies to all Meeting/Webinar IDs hosted by the user\"}),`\n`,(0,i.jsx)(e.li,{children:\"Resets at 00:00 UTC daily\"}),`\n`]}),`\n`,(0,i.jsx)(e.h3,{id:\"registration-requests\",children:(0,i.jsxs)(e.a,{href:\"#registration-requests\",children:[\"Registration requests\",(0,i.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,i.jsxs)(e.ul,{children:[`\n`,(0,i.jsx)(e.li,{children:\"3 requests per day (UTC) for the same registrant in the same meeting/webinar\"}),`\n`]}),`\n`,(0,i.jsx)(e.h3,{id:\"registrant-status-requests\",children:(0,i.jsxs)(e.a,{href:\"#registrant-status-requests\",children:[\"Registrant status requests\",(0,i.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,i.jsxs)(e.ul,{children:[`\n`,(0,i.jsx)(e.li,{children:\"10 requests per day (UTC) for the same registrant in the same meeting/webinar\"}),`\n`]}),`\n`,(0,i.jsx)(e.h2,{id:\"zoom-phone-apis\",children:(0,i.jsxs)(e.a,{href:\"#zoom-phone-apis\",children:[\"Zoom Phone APIs\",(0,i.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,i.jsxs)(e.table,{children:[(0,i.jsx)(e.thead,{children:(0,i.jsxs)(e.tr,{children:[(0,i.jsx)(e.th,{children:\"Rate Limit Type\"}),(0,i.jsx)(e.th,{children:\"Pro\"}),(0,i.jsx)(e.th,{children:\"Business+\"})]})}),(0,i.jsxs)(e.tbody,{children:[(0,i.jsxs)(e.tr,{children:[(0,i.jsx)(e.td,{children:\"Light APIs\"}),(0,i.jsx)(e.td,{children:\"20/second\"}),(0,i.jsx)(e.td,{children:\"40/second\"})]}),(0,i.jsxs)(e.tr,{children:[(0,i.jsx)(e.td,{children:\"Medium APIs\"}),(0,i.jsx)(e.td,{children:\"10/second\"}),(0,i.jsx)(e.td,{children:\"20/second\"})]}),(0,i.jsxs)(e.tr,{children:[(0,i.jsx)(e.td,{children:\"Heavy APIs\"}),(0,i.jsx)(e.td,{children:\"5/second, 15,000/day*\"}),(0,i.jsx)(e.td,{children:\"10/second, 30,000/day*\"})]}),(0,i.jsxs)(e.tr,{children:[(0,i.jsx)(e.td,{children:\"Resource-intensive APIs\"}),(0,i.jsx)(e.td,{children:\"5/minute, 15,000/day*\"}),(0,i.jsx)(e.td,{children:\"10/minute, 30,000/day*\"})]})]})]}),`\n`,(0,i.jsx)(e.p,{children:\"* Daily limit shared between heavy and resource-intensive APIs\"}),`\n`,(0,i.jsx)(e.h2,{id:\"zoom-contact-center-apis\",children:(0,i.jsxs)(e.a,{href:\"#zoom-contact-center-apis\",children:[\"Zoom Contact Center APIs\",(0,i.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,i.jsx)(e.p,{children:\"Zoom Contact Center APIs are rate limited by request type and applied based on the account plan:\"}),`\n`,(0,i.jsxs)(e.table,{children:[(0,i.jsx)(e.thead,{children:(0,i.jsxs)(e.tr,{children:[(0,i.jsx)(e.th,{children:\"Rate Limit Type\"}),(0,i.jsx)(e.th,{children:\"Pro\"}),(0,i.jsx)(e.th,{children:\"Business+\"})]})}),(0,i.jsxs)(e.tbody,{children:[(0,i.jsxs)(e.tr,{children:[(0,i.jsx)(e.td,{children:\"Light APIs\"}),(0,i.jsx)(e.td,{children:\"20/second\"}),(0,i.jsx)(e.td,{children:\"40/second\"})]}),(0,i.jsxs)(e.tr,{children:[(0,i.jsx)(e.td,{children:\"Medium APIs\"}),(0,i.jsx)(e.td,{children:\"10/second\"}),(0,i.jsx)(e.td,{children:\"20/second\"})]}),(0,i.jsxs)(e.tr,{children:[(0,i.jsx)(e.td,{children:\"Heavy APIs\"}),(0,i.jsx)(e.td,{children:\"5/second, 15,000/day*\"}),(0,i.jsx)(e.td,{children:\"10/second, 30,000/day*\"})]})]})]}),`\n`,(0,i.jsx)(e.p,{children:\"* Daily limit shared with resource-intensive APIs\"}),`\n`,(0,i.jsx)(e.h2,{id:\"concurrent-request-limits\",children:(0,i.jsxs)(e.a,{href:\"#concurrent-request-limits\",children:[\"Concurrent request limits\",(0,i.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,i.jsx)(e.p,{children:\"Concurrent requests to a single resource may fail with a 429 HTTP status code. For example:\"}),`\n`,(0,i.jsx)(e.pre,{children:(0,i.jsx)(e.code,{children:`Too many concurrent requests. A request to disassociate this user has already been made.\n`})}),`\n`,(0,i.jsx)(e.h2,{id:\"error-handling-best-practices\",children:(0,i.jsxs)(e.a,{href:\"#error-handling-best-practices\",children:[\"Error handling best practices\",(0,i.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,i.jsxs)(e.ul,{children:[`\n`,(0,i.jsx)(e.li,{children:\"Handle 429 status codes by implementing a wait before retrying.\"}),`\n`,(0,i.jsx)(e.li,{children:\"Build retry mechanisms for failed requests.\"}),`\n`,(0,i.jsx)(e.li,{children:\"Reduce request frequency where possible.\"}),`\n`,(0,i.jsx)(e.li,{children:\"Cache responses to minimize repetitive requests.\"}),`\n`,(0,i.jsxs)(e.li,{children:[\"Use \",(0,i.jsx)(e.a,{href:\"/docs/api/webhooks\",children:(0,i.jsx)(e.strong,{children:\"webhooks\"})}),\" for data change events instead of polling.\"]}),`\n`]}),`\n`,(0,i.jsx)(e.h3,{id:\"error-table\",children:(0,i.jsxs)(e.a,{href:\"#error-table\",children:[\"Error table\",(0,i.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,i.jsx)(e.p,{children:\"This table lists errors you may encounter.\"}),`\n`,(0,i.jsxs)(e.table,{children:[(0,i.jsx)(e.thead,{children:(0,i.jsxs)(e.tr,{children:[(0,i.jsx)(e.th,{children:\"Limit Type\"}),(0,i.jsx)(e.th,{children:\"HTTP Status\"}),(0,i.jsx)(e.th,{children:\"Error Message\"})]})}),(0,i.jsxs)(e.tbody,{children:[(0,i.jsxs)(e.tr,{children:[(0,i.jsx)(e.td,{children:\"Per-second\"}),(0,i.jsx)(e.td,{children:\"429\"}),(0,i.jsx)(e.td,{children:(0,i.jsx)(e.code,{children:\"You have reached the maximum per-second rate limit for this API. Try again later.\"})})]}),(0,i.jsxs)(e.tr,{children:[(0,i.jsx)(e.td,{children:\"Daily\"}),(0,i.jsx)(e.td,{children:\"429\"}),(0,i.jsx)(e.td,{children:(0,i.jsx)(e.code,{children:\"You have reached the maximum daily rate limit for this API. Refer to the response header for details on when you can make another request.\"})})]})]})]})]})}function u(r={}){let{wrapper:e}={...l(),...r.components};return e?(0,i.jsx)(e,{...r,children:(0,i.jsx)(o,{...r})}):o(r)}function I(r,e){throw new Error(\"Expected \"+(e?\"component\":\"object\")+\" `\"+r+\"` to be defined: you likely forgot to import, pass, or provide it.\")}return v(T);})();\n;return Component;",
      "frontmatter": {
        "title": "Rate limits"
      },
      "errors": [],
      "matter": {
        "content": "\n# Rate limits\n\nRate limits define the maximum number of requests a single account can make within a given period of time. When you exceed a rate limit, the API request will fail and return a **HTTP 429** status code.\n\n## Rate limit details\n\n- **Rate limits are applied at the account level.** These rate limits are shared by users for all apps installed on the account.\n- **Rate limits are applied based on your account plan.** Increased rate limits are available for all Business, Enterprise, Education, and Partner accounts. [See pricing for details](https://zoom.us/pricing).\n- **APIs are grouped by Request Type with unique rate limits for each.** The documentation for each API endpoint includes a \"Rate Limit Label\".\n- Some APIs have special rate limits at the **user-level**.\n- Some APIs have **limits on concurrent operations** performed on a single resource.\n\n<Alert>\n  The Zoom API uses rate limits to ensure that API request traffic is efficiently handled. We may alter these limits as\n  needed. See the <a href=\"/changelog\">changelog</a> for details.\n</Alert>\n\n## Rate limits by account type\n\nRate limits are applied based on the account plan, either **Free**, **Pro**, and **Business+** (which includes Business, Education, Enterprise & Partners). [Meeting SDK](/docs/meeting-sdk/) is included in these account types. This outlines the rate limits applied to APIs by rate limit type for each account plan.\n\n| Rate Limit Type         | Free                  | Pro         | Business+   |\n| ----------------------- | --------------------- | ----------- | ----------- |\n| Light APIs              | 4/second, 6000/day    | 30/second   | 80/second   |\n| Medium APIs             | 2/second, 2000/day    | 20/second   | 60/second   |\n| Heavy APIs              | 1/second, 1000/day    | 10/second\\* | 40/second\\* |\n| Resource-intensive APIs | 10/minute, 30,000/day | 10/minute\\* | 20/minute\\* |\n\n\\* Subject to combined daily rate limits:\n\n- Pro: 30,000 requests/day for heavy and resource-intensive APIs\n- Business+: 60,000 requests/day for heavy and resource-intensive APIs\n\n### Video SDK account rate limits\n\nVideo SDK is a [separate account type](/docs/video-sdk/developer-accounts/). _See [Video SDK Plans & Pricing for Developer](https://zoom.us/pricing/developer) for pricing._ Video SDK REST API rate limits are tied to the account's plan type.\n\n- **Video SDK Pay As You Go** *(Deprecated)* - Uses the **Pro** rate limits.\n- **Video SDK Annual Prepay Monthly Usage** - Uses the **Pro** rate limits.\n- **All other Video SDK plans** (including Video SDK Monthly Prepay Monthly Usage, Video SDK Universal credit, and others) - Uses the **Business+** rate limits.\n\n## User-level rate limits\n\nThese are some special user-level rate limits.\n\n### Meeting and Webinar create/update requests\n- 100 create/update requests per day (UTC) per user\n- Applies to all Meeting/Webinar IDs hosted by the user\n- Resets at 00:00 UTC daily\n\n### Registration requests\n\n- 3 requests per day (UTC) for the same registrant in the same meeting/webinar\n\n### Registrant status requests\n\n- 10 requests per day (UTC) for the same registrant in the same meeting/webinar\n\n## Zoom Phone APIs\n\n| Rate Limit Type         | Pro                    | Business+               |\n| ----------------------- | ---------------------- | ----------------------- |\n| Light APIs              | 20/second              | 40/second               |\n| Medium APIs             | 10/second              | 20/second               |\n| Heavy APIs              | 5/second, 15,000/day\\* | 10/second, 30,000/day\\* |\n| Resource-intensive APIs | 5/minute, 15,000/day\\* | 10/minute, 30,000/day\\* |\n\n\\* Daily limit shared between heavy and resource-intensive APIs\n\n## Zoom Contact Center APIs\n\nZoom Contact Center APIs are rate limited by request type and applied based on the account plan:\n\n| Rate Limit Type | Pro                    | Business+               |\n| --------------- | ---------------------- | ----------------------- |\n| Light APIs      | 20/second              | 40/second               |\n| Medium APIs     | 10/second              | 20/second               |\n| Heavy APIs      | 5/second, 15,000/day\\* | 10/second, 30,000/day\\* |\n\n\\* Daily limit shared with resource-intensive APIs\n\n## Concurrent request limits\n\nConcurrent requests to a single resource may fail with a 429 HTTP status code. For example:\n\n```\nToo many concurrent requests. A request to disassociate this user has already been made.\n```\n\n## Error handling best practices\n\n- Handle 429 status codes by implementing a wait before retrying.\n- Build retry mechanisms for failed requests.\n- Reduce request frequency where possible.\n- Cache responses to minimize repetitive requests.\n- Use [**webhooks**](/docs/api/webhooks) for data change events instead of polling.\n\n### Error table\n\nThis table lists errors you may encounter.\n\n| Limit Type | HTTP Status | Error Message                                                                                                                              |\n| ---------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------ |\n| Per-second | 429         | `You have reached the maximum per-second rate limit for this API. Try again later.`                                                          |\n| Daily      | 429         | `You have reached the maximum daily rate limit for this API. Refer to the response header for details on when you can make another request.` |\n",
        "data": {
          "title": "Rate limits"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/rate-limits.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
