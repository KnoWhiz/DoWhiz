# Manage plans

- Source URL: https://developers.zoom.us/docs/api/references/manage-plans/
- Snapshot path: docs/api/references/manage-plans
- Generated (UTC): 2026-02-07T22:40:11.513884+00:00

## Frontmatter

```json
{
  "title": "Manage plans"
}
```

## Content

```md

# Manage plans

If you are a partner or if you have a [primary account](/docs/api/ma/), you can manage billing plans for your associated Pro or higher subaccounts. Zoom only allows approved partners to use these APIs to manage billing information for their subaccounts. Use the following APIs to manage plans for an account.

* [Subscribe account to plan](/docs/api/billing/ma/#tag/billing/POST/accounts/\{accountId\}/plans) — add a base plan.
* [Subscribe account to an additional plan](/docs/api/billing/ma/#tag/billing/POST/accounts/\{accountId\}/plans/addons) — add an additional plan.
* [Update a base plan](/docs/api/billing/ma/#tag/billing/PUT/accounts/\{accountId\}/plans/base) — update a base plan.
* [Update an account's additional plan](/docs/api/billing/ma/#tag/billing/PUT/accounts/\{accountId\}/plans/addons) — update an additional plan.
* [Cancel additional plans](/docs/api/billing/ma/#tag/billing/PATCH/accounts/\{accountId\}/plans/addons/status) — specify an additional plan or plans to cancel.
* [Cancel a base plan](/docs/api/billing/ma/#tag/billing/PATCH/accounts/\{accountId\}/plans/base/status) — cancel a base plan.

You can also get [account plan information](/docs/api/billing/ma/#tag/billing/GET/accounts/\{accountId\}/billing/invoices/\{invoiceId\}) and [plan usage](/docs/api/billing/ma/#tag/billing/GET/accounts/\{accountId\}/plans/usage).

## Examples

See the following examples for syntax. See the [Subscribe account to plan](/docs/api/billing/ma/#tag/billing/POST/accounts/\{accountId\}/plans) and [Subscribe account to an additional plan](/docs/api/billing/ma/#tag/billing/POST/accounts/\{accountId\}/plans/addons) schemas and examples for details.

### Subscribe

In the API payload, replace the `type` with the API plan type for that specific plan, such as `monthly` or `plan_pro_three_years`.

```json
...
  "plan_base": {
    "hosts": 20,
    "type": "montly"
  },
...
```

### Subscribe (additional) example

You can follow the syntax above for the [Subscribe account to an additional plan](/docs/api/billing/ma/#tag/billing/POST/accounts/\{accountId\}/plans/addons) API. You can also add an additional plan by sending the plan type using the following format:

```json
{
  "hosts": 18,
  "type": "webinar500_monthly",
  "paid_period_start_date": "2021-12-31",
  "service_effective_date": "2021-12-31"
}
```

## Video SDK Account Plans

*For pricing, see the [Developer pricing](https://zoom.us/pricing/developer) page.*

Primary Accounts can set up accounts with [Video SDK Plans](/docs/api/references/billing-plans#video-sdk-plans) using the APIs above. See the following notes and [Video SDK account: Create subaccounts](/docs/build/account/#create-subaccounts) for details.

- If your primary account is a Zoom Meeting account, you can choose the Video SDK base plan for the accounts you manage.
- The `hosts` and `increasing_hosts` parameters are not required in the `plan_base` object to subscribe an account to a plan or update a base plan.

Primary accounts can use APIs to programmatically manage activities for subaccounts in their organization if both the owner of the primary account and the owner of the subaccount agree. Work with your Zoom account representative to enable this access for the primary account and each subaccount. See [Access or modify subaccount data](/docs/video-sdk/api-request/#access-or-modify-subaccount-data) for details.
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
      "code": "var Component=(()=>{var u=Object.create;var o=Object.defineProperty;var b=Object.getOwnPropertyDescriptor;var g=Object.getOwnPropertyNames;var m=Object.getPrototypeOf,f=Object.prototype.hasOwnProperty;var y=(e,a)=>()=>(a||e((a={exports:{}}).exports,a),a.exports),I=(e,a)=>{for(var i in a)o(e,i,{get:a[i],enumerable:!0})},s=(e,a,i,l)=>{if(a&&typeof a==\"object\"||typeof a==\"function\")for(let c of g(a))!f.call(e,c)&&c!==i&&o(e,c,{get:()=>a[c],enumerable:!(l=b(a,c))||l.enumerable});return e};var x=(e,a,i)=>(i=e!=null?u(m(e)):{},s(a||!e||!e.__esModule?o(i,\"default\",{value:e,enumerable:!0}):i,e)),_=e=>s(o({},\"__esModule\",{value:!0}),e);var d=y((P,r)=>{r.exports=_jsx_runtime});var S={};I(S,{default:()=>p,frontmatter:()=>v});var n=x(d());var{useMDXComponents:t}=MdxJsReact;var v={title:\"Manage plans\"};function h(e){let a={a:\"a\",code:\"code\",em:\"em\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",p:\"p\",pre:\"pre\",ul:\"ul\",...t(),...e.components};return(0,n.jsxs)(n.Fragment,{children:[(0,n.jsx)(a.h1,{id:\"manage-plans\",children:\"Manage plans\"}),`\n`,(0,n.jsxs)(a.p,{children:[\"If you are a partner or if you have a \",(0,n.jsx)(a.a,{href:\"/docs/api/ma/\",children:\"primary account\"}),\", you can manage billing plans for your associated Pro or higher subaccounts. Zoom only allows approved partners to use these APIs to manage billing information for their subaccounts. Use the following APIs to manage plans for an account.\"]}),`\n`,(0,n.jsxs)(a.ul,{children:[`\n`,(0,n.jsxs)(a.li,{children:[(0,n.jsx)(a.a,{href:\"/docs/api/billing/ma/#tag/billing/post/accounts/{accountId}/plans\",children:\"Subscribe account to plan\"}),\" \\u2014 add a base plan.\"]}),`\n`,(0,n.jsxs)(a.li,{children:[(0,n.jsx)(a.a,{href:\"/docs/api/billing/ma/#tag/billing/post/accounts/{accountId}/plans/addons\",children:\"Subscribe account to an additional plan\"}),\" \\u2014 add an additional plan.\"]}),`\n`,(0,n.jsxs)(a.li,{children:[(0,n.jsx)(a.a,{href:\"/docs/api/billing/ma/#tag/billing/put/accounts/{accountId}/plans/base\",children:\"Update a base plan\"}),\" \\u2014 update a base plan.\"]}),`\n`,(0,n.jsxs)(a.li,{children:[(0,n.jsx)(a.a,{href:\"/docs/api/billing/ma/#tag/billing/put/accounts/{accountId}/plans/addons\",children:\"Update an account's additional plan\"}),\" \\u2014 update an additional plan.\"]}),`\n`,(0,n.jsxs)(a.li,{children:[(0,n.jsx)(a.a,{href:\"/docs/api/billing/ma/#tag/billing/patch/accounts/{accountId}/plans/addons/status\",children:\"Cancel additional plans\"}),\" \\u2014 specify an additional plan or plans to cancel.\"]}),`\n`,(0,n.jsxs)(a.li,{children:[(0,n.jsx)(a.a,{href:\"/docs/api/billing/ma/#tag/billing/patch/accounts/{accountId}/plans/base/status\",children:\"Cancel a base plan\"}),\" \\u2014 cancel a base plan.\"]}),`\n`]}),`\n`,(0,n.jsxs)(a.p,{children:[\"You can also get \",(0,n.jsx)(a.a,{href:\"/docs/api/billing/ma/#tag/billing/get/accounts/{accountId}/billing/invoices/{invoiceId}\",children:\"account plan information\"}),\" and \",(0,n.jsx)(a.a,{href:\"/docs/api/billing/ma/#tag/billing/get/accounts/{accountId}/plans/usage\",children:\"plan usage\"}),\".\"]}),`\n`,(0,n.jsx)(a.h2,{id:\"examples\",children:(0,n.jsxs)(a.a,{href:\"#examples\",children:[\"Examples\",(0,n.jsx)(a.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(a.p,{children:[\"See the following examples for syntax. See the \",(0,n.jsx)(a.a,{href:\"/docs/api/billing/ma/#tag/billing/post/accounts/{accountId}/plans\",children:\"Subscribe account to plan\"}),\" and \",(0,n.jsx)(a.a,{href:\"/docs/api/billing/ma/#tag/billing/post/accounts/{accountId}/plans/addons\",children:\"Subscribe account to an additional plan\"}),\" schemas and examples for details.\"]}),`\n`,(0,n.jsx)(a.h3,{id:\"subscribe\",children:(0,n.jsxs)(a.a,{href:\"#subscribe\",children:[\"Subscribe\",(0,n.jsx)(a.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(a.p,{children:[\"In the API payload, replace the \",(0,n.jsx)(a.code,{children:\"type\"}),\" with the API plan type for that specific plan, such as \",(0,n.jsx)(a.code,{children:\"monthly\"}),\" or \",(0,n.jsx)(a.code,{children:\"plan_pro_three_years\"}),\".\"]}),`\n`,(0,n.jsx)(a.pre,{children:(0,n.jsx)(a.code,{className:\"language-json\",children:`...\n  \"plan_base\": {\n    \"hosts\": 20,\n    \"type\": \"montly\"\n  },\n...\n`})}),`\n`,(0,n.jsx)(a.h3,{id:\"subscribe-additional-example\",children:(0,n.jsxs)(a.a,{href:\"#subscribe-additional-example\",children:[\"Subscribe (additional) example\",(0,n.jsx)(a.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(a.p,{children:[\"You can follow the syntax above for the \",(0,n.jsx)(a.a,{href:\"/docs/api/billing/ma/#tag/billing/post/accounts/{accountId}/plans/addons\",children:\"Subscribe account to an additional plan\"}),\" API. You can also add an additional plan by sending the plan type using the following format:\"]}),`\n`,(0,n.jsx)(a.pre,{children:(0,n.jsx)(a.code,{className:\"language-json\",children:`{\n  \"hosts\": 18,\n  \"type\": \"webinar500_monthly\",\n  \"paid_period_start_date\": \"2021-12-31\",\n  \"service_effective_date\": \"2021-12-31\"\n}\n`})}),`\n`,(0,n.jsx)(a.h2,{id:\"video-sdk-account-plans\",children:(0,n.jsxs)(a.a,{href:\"#video-sdk-account-plans\",children:[\"Video SDK Account Plans\",(0,n.jsx)(a.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsx)(a.p,{children:(0,n.jsxs)(a.em,{children:[\"For pricing, see the \",(0,n.jsx)(a.a,{href:\"https://zoom.us/pricing/developer\",children:\"Developer pricing\"}),\" page.\"]})}),`\n`,(0,n.jsxs)(a.p,{children:[\"Primary Accounts can set up accounts with \",(0,n.jsx)(a.a,{href:\"/docs/api/references/billing-plans#video-sdk-plans\",children:\"Video SDK Plans\"}),\" using the APIs above. See the following notes and \",(0,n.jsx)(a.a,{href:\"/docs/build/account/#create-subaccounts\",children:\"Video SDK account: Create subaccounts\"}),\" for details.\"]}),`\n`,(0,n.jsxs)(a.ul,{children:[`\n`,(0,n.jsx)(a.li,{children:\"If your primary account is a Zoom Meeting account, you can choose the Video SDK base plan for the accounts you manage.\"}),`\n`,(0,n.jsxs)(a.li,{children:[\"The \",(0,n.jsx)(a.code,{children:\"hosts\"}),\" and \",(0,n.jsx)(a.code,{children:\"increasing_hosts\"}),\" parameters are not required in the \",(0,n.jsx)(a.code,{children:\"plan_base\"}),\" object to subscribe an account to a plan or update a base plan.\"]}),`\n`]}),`\n`,(0,n.jsxs)(a.p,{children:[\"Primary accounts can use APIs to programmatically manage activities for subaccounts in their organization if both the owner of the primary account and the owner of the subaccount agree. Work with your Zoom account representative to enable this access for the primary account and each subaccount. See \",(0,n.jsx)(a.a,{href:\"/docs/video-sdk/api-request/#access-or-modify-subaccount-data\",children:\"Access or modify subaccount data\"}),\" for details.\"]})]})}function p(e={}){let{wrapper:a}={...t(),...e.components};return a?(0,n.jsx)(a,{...e,children:(0,n.jsx)(h,{...e})}):h(e)}return _(S);})();\n;return Component;",
      "frontmatter": {
        "title": "Manage plans"
      },
      "errors": [],
      "matter": {
        "content": "\n# Manage plans\n\nIf you are a partner or if you have a [primary account](/docs/api/ma/), you can manage billing plans for your associated Pro or higher subaccounts. Zoom only allows approved partners to use these APIs to manage billing information for their subaccounts. Use the following APIs to manage plans for an account.\n\n* [Subscribe account to plan](/docs/api/billing/ma/#tag/billing/POST/accounts/\\{accountId\\}/plans) \u2014 add a base plan.\n* [Subscribe account to an additional plan](/docs/api/billing/ma/#tag/billing/POST/accounts/\\{accountId\\}/plans/addons) \u2014 add an additional plan.\n* [Update a base plan](/docs/api/billing/ma/#tag/billing/PUT/accounts/\\{accountId\\}/plans/base) \u2014 update a base plan.\n* [Update an account's additional plan](/docs/api/billing/ma/#tag/billing/PUT/accounts/\\{accountId\\}/plans/addons) \u2014 update an additional plan.\n* [Cancel additional plans](/docs/api/billing/ma/#tag/billing/PATCH/accounts/\\{accountId\\}/plans/addons/status) \u2014 specify an additional plan or plans to cancel.\n* [Cancel a base plan](/docs/api/billing/ma/#tag/billing/PATCH/accounts/\\{accountId\\}/plans/base/status) \u2014 cancel a base plan.\n\nYou can also get [account plan information](/docs/api/billing/ma/#tag/billing/GET/accounts/\\{accountId\\}/billing/invoices/\\{invoiceId\\}) and [plan usage](/docs/api/billing/ma/#tag/billing/GET/accounts/\\{accountId\\}/plans/usage).\n\n## Examples\n\nSee the following examples for syntax. See the [Subscribe account to plan](/docs/api/billing/ma/#tag/billing/POST/accounts/\\{accountId\\}/plans) and [Subscribe account to an additional plan](/docs/api/billing/ma/#tag/billing/POST/accounts/\\{accountId\\}/plans/addons) schemas and examples for details.\n\n### Subscribe\n\nIn the API payload, replace the `type` with the API plan type for that specific plan, such as `monthly` or `plan_pro_three_years`.\n\n```json\n...\n  \"plan_base\": {\n    \"hosts\": 20,\n    \"type\": \"montly\"\n  },\n...\n```\n\n### Subscribe (additional) example\n\nYou can follow the syntax above for the [Subscribe account to an additional plan](/docs/api/billing/ma/#tag/billing/POST/accounts/\\{accountId\\}/plans/addons) API. You can also add an additional plan by sending the plan type using the following format:\n\n```json\n{\n  \"hosts\": 18,\n  \"type\": \"webinar500_monthly\",\n  \"paid_period_start_date\": \"2021-12-31\",\n  \"service_effective_date\": \"2021-12-31\"\n}\n```\n\n## Video SDK Account Plans\n\n*For pricing, see the [Developer pricing](https://zoom.us/pricing/developer) page.*\n\nPrimary Accounts can set up accounts with [Video SDK Plans](/docs/api/references/billing-plans#video-sdk-plans) using the APIs above. See the following notes and [Video SDK account: Create subaccounts](/docs/build/account/#create-subaccounts) for details.\n\n- If your primary account is a Zoom Meeting account, you can choose the Video SDK base plan for the accounts you manage.\n- The `hosts` and `increasing_hosts` parameters are not required in the `plan_base` object to subscribe an account to a plan or update a base plan.\n\nPrimary accounts can use APIs to programmatically manage activities for subaccounts in their organization if both the owner of the primary account and the owner of the subaccount agree. Work with your Zoom account representative to enable this access for the primary account and each subaccount. See [Access or modify subaccount data](/docs/video-sdk/api-request/#access-or-modify-subaccount-data) for details.\n",
        "data": {
          "title": "Manage plans"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/references/manage-plans.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
