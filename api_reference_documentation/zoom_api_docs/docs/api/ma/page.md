# Primary (master) account APIs

- Source URL: https://developers.zoom.us/docs/api/ma/
- Snapshot path: docs/api/ma
- Generated (UTC): 2026-02-07T22:40:10.824797+00:00

## Frontmatter

```json
{
  "title": "Primary (master) account APIs"
}
```

## Content

```md

# Primary (master) account APIs

Primary accounts (formerly referred to as master accounts) are Zoom accounts that manage one or more sub accounts. The terms primary and master may be used interchangeably throughout the documentation.

For example:

- A university may use a primary account to manage separate sub accounts for each department.
- An enterprise might use a primary account to manage business units that operate under different regulatory requirements.
- Zoom [reseller and distributor technology partners](https://partner.zoom.com/) can also be primary account holders.

Primary accounts have access to primary/master account APIs, which allow them to programmatically manage activities related to their sub accounts.

<Alert variant="warning">

**Primary account API access limitations**

Primary account APIs can only be used to manage their **own** sub accounts. They **cannot** be used to access or manage other primary accounts.

</Alert>

## Tiered account structures

Zoom partners, such as resellers and distributors, can use primary accounts to provision **two-tiered** or **three-tiered** account structures:

* **Two-tiered** - A single primary account manages multiple sub accounts directly.
* **Three-tiered** - A top-level primary account manages one or more **second-level** primary accounts. Each second-level account can, in turn, provision and manage its own sub accounts.

<Image src="/img/1636145489712.png" />

***For enrollment or business queries regarding reseller and distributor technology partner programs, email [**partner-success@zoom.us**](mailto:partner-success@zoom.us).***

## Using primary/master account APIs

Primary account API endpoints are prepended with `/accounts/{accountId}`. The request and response formats mirror those of standard Zoom APIs.

For example, a regular Zoom account uses the [GET /users](/docs/api/users/#tag/users/GET/users) endpoint to list all of its users. In contrast, a primary account lists users in a sub account using the [List users (`GET /accounts/{accountId}/users`)](/docs/api/users/ma/#tag/users/get/accounts/{accountId}/users) endpoint.

To call primary account API endpoints, the primary account must authenticate using one of the following:

* An [OAuth token](/docs/integrations/oauth/) associated with a user under the primary account
* A [Server-to-Server OAuth token](/docs/internal-apps/s2s-oauth/)

The primary account must have the **Manage the sub account** role enabled to access and manage sub account data via API. See [Using role management](https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0064983) and [Managing Zoom sub accounts](https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0066764) for details.

To get a sub account's `accountId`, use the [Get account settings](/docs/api/accounts/#tag/accounts/get/accounts/{accountId}/settings) API and pass `me` as the `accountId` value.

### Related References

* Zoom primary account APIs - In the sidebar, under **Master accounts**, click **References**, and choose a product to explore supported endpoints.
* [Phone primary account API](/docs/api/phone/ma)
* [Video SDK primary account API](/docs/api/video-sdk/ma)
* [Video SDK Account Plans](/docs/api/references/manage-plans//#video-sdk-account-plans)
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
      "code": "var Component=(()=>{var p=Object.create;var o=Object.defineProperty;var g=Object.getOwnPropertyDescriptor;var y=Object.getOwnPropertyNames;var f=Object.getPrototypeOf,b=Object.prototype.hasOwnProperty;var A=(n,e)=>()=>(e||n((e={exports:{}}).exports,e),e.exports),I=(n,e)=>{for(var t in e)o(n,t,{get:e[t],enumerable:!0})},i=(n,e,t,a)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let c of y(e))!b.call(n,c)&&c!==t&&o(n,c,{get:()=>e[c],enumerable:!(a=g(e,c))||a.enumerable});return n};var P=(n,e,t)=>(t=n!=null?p(f(n)):{},i(e||!n||!n.__esModule?o(t,\"default\",{value:n,enumerable:!0}):t,n)),v=n=>i(o({},\"__esModule\",{value:!0}),n);var u=A((T,d)=>{d.exports=_jsx_runtime});var w={};I(w,{default:()=>m,frontmatter:()=>x});var r=P(u());var{useMDXComponents:s}=MdxJsReact;var x={title:\"Primary (master) account APIs\"};function l(n){let e={a:\"a\",code:\"code\",em:\"em\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",p:\"p\",strong:\"strong\",ul:\"ul\",...s(),...n.components},{Alert:t,Image:a}=e;return t||h(\"Alert\",!0),a||h(\"Image\",!0),(0,r.jsxs)(r.Fragment,{children:[(0,r.jsx)(e.h1,{id:\"primary-master-account-apis\",children:\"Primary (master) account APIs\"}),`\n`,(0,r.jsx)(e.p,{children:\"Primary accounts (formerly referred to as master accounts) are Zoom accounts that manage one or more sub accounts. The terms primary and master may be used interchangeably throughout the documentation.\"}),`\n`,(0,r.jsx)(e.p,{children:\"For example:\"}),`\n`,(0,r.jsxs)(e.ul,{children:[`\n`,(0,r.jsx)(e.li,{children:\"A university may use a primary account to manage separate sub accounts for each department.\"}),`\n`,(0,r.jsx)(e.li,{children:\"An enterprise might use a primary account to manage business units that operate under different regulatory requirements.\"}),`\n`,(0,r.jsxs)(e.li,{children:[\"Zoom \",(0,r.jsx)(e.a,{href:\"https://partner.zoom.com/\",children:\"reseller and distributor technology partners\"}),\" can also be primary account holders.\"]}),`\n`]}),`\n`,(0,r.jsx)(e.p,{children:\"Primary accounts have access to primary/master account APIs, which allow them to programmatically manage activities related to their sub accounts.\"}),`\n`,(0,r.jsxs)(t,{variant:\"warning\",children:[(0,r.jsx)(e.p,{children:(0,r.jsx)(e.strong,{children:\"Primary account API access limitations\"})}),(0,r.jsxs)(e.p,{children:[\"Primary account APIs can only be used to manage their \",(0,r.jsx)(e.strong,{children:\"own\"}),\" sub accounts. They \",(0,r.jsx)(e.strong,{children:\"cannot\"}),\" be used to access or manage other primary accounts.\"]})]}),`\n`,(0,r.jsx)(e.h2,{id:\"tiered-account-structures\",children:(0,r.jsxs)(e.a,{href:\"#tiered-account-structures\",children:[\"Tiered account structures\",(0,r.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,r.jsxs)(e.p,{children:[\"Zoom partners, such as resellers and distributors, can use primary accounts to provision \",(0,r.jsx)(e.strong,{children:\"two-tiered\"}),\" or \",(0,r.jsx)(e.strong,{children:\"three-tiered\"}),\" account structures:\"]}),`\n`,(0,r.jsxs)(e.ul,{children:[`\n`,(0,r.jsxs)(e.li,{children:[(0,r.jsx)(e.strong,{children:\"Two-tiered\"}),\" - A single primary account manages multiple sub accounts directly.\"]}),`\n`,(0,r.jsxs)(e.li,{children:[(0,r.jsx)(e.strong,{children:\"Three-tiered\"}),\" - A top-level primary account manages one or more \",(0,r.jsx)(e.strong,{children:\"second-level\"}),\" primary accounts. Each second-level account can, in turn, provision and manage its own sub accounts.\"]}),`\n`]}),`\n`,(0,r.jsx)(a,{src:\"/img/1636145489712.png\"}),`\n`,(0,r.jsx)(e.p,{children:(0,r.jsx)(e.em,{children:(0,r.jsxs)(e.strong,{children:[\"For enrollment or business queries regarding reseller and distributor technology partner programs, email \",(0,r.jsx)(e.a,{href:\"mailto:partner-success@zoom.us\",children:(0,r.jsx)(e.strong,{children:\"partner-success@zoom.us\"})}),\".\"]})})}),`\n`,(0,r.jsx)(e.h2,{id:\"using-primarymaster-account-apis\",children:(0,r.jsxs)(e.a,{href:\"#using-primarymaster-account-apis\",children:[\"Using primary/master account APIs\",(0,r.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,r.jsxs)(e.p,{children:[\"Primary account API endpoints are prepended with \",(0,r.jsx)(e.code,{children:\"/accounts/{accountId}\"}),\". The request and response formats mirror those of standard Zoom APIs.\"]}),`\n`,(0,r.jsxs)(e.p,{children:[\"For example, a regular Zoom account uses the \",(0,r.jsx)(e.a,{href:\"/docs/api/users/#tag/users/get/users\",children:\"GET /users\"}),\" endpoint to list all of its users. In contrast, a primary account lists users in a sub account using the \",(0,r.jsxs)(e.a,{href:\"/docs/api/users/ma/#tag/users/get/accounts/{accountId}/users\",children:[\"List users (\",(0,r.jsx)(e.code,{children:\"GET /accounts/{accountId}/users\"}),\")\"]}),\" endpoint.\"]}),`\n`,(0,r.jsx)(e.p,{children:\"To call primary account API endpoints, the primary account must authenticate using one of the following:\"}),`\n`,(0,r.jsxs)(e.ul,{children:[`\n`,(0,r.jsxs)(e.li,{children:[\"An \",(0,r.jsx)(e.a,{href:\"/docs/integrations/oauth/\",children:\"OAuth token\"}),\" associated with a user under the primary account\"]}),`\n`,(0,r.jsxs)(e.li,{children:[\"A \",(0,r.jsx)(e.a,{href:\"/docs/internal-apps/s2s-oauth/\",children:\"Server-to-Server OAuth token\"})]}),`\n`]}),`\n`,(0,r.jsxs)(e.p,{children:[\"The primary account must have the \",(0,r.jsx)(e.strong,{children:\"Manage the sub account\"}),\" role enabled to access and manage sub account data via API. See \",(0,r.jsx)(e.a,{href:\"https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0064983\",children:\"Using role management\"}),\" and \",(0,r.jsx)(e.a,{href:\"https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0066764\",children:\"Managing Zoom sub accounts\"}),\" for details.\"]}),`\n`,(0,r.jsxs)(e.p,{children:[\"To get a sub account's \",(0,r.jsx)(e.code,{children:\"accountId\"}),\", use the \",(0,r.jsx)(e.a,{href:\"/docs/api/accounts/#tag/accounts/get/accounts/{accountId}/settings\",children:\"Get account settings\"}),\" API and pass \",(0,r.jsx)(e.code,{children:\"me\"}),\" as the \",(0,r.jsx)(e.code,{children:\"accountId\"}),\" value.\"]}),`\n`,(0,r.jsx)(e.h3,{id:\"related-references\",children:(0,r.jsxs)(e.a,{href:\"#related-references\",children:[\"Related References\",(0,r.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,r.jsxs)(e.ul,{children:[`\n`,(0,r.jsxs)(e.li,{children:[\"Zoom primary account APIs - In the sidebar, under \",(0,r.jsx)(e.strong,{children:\"Master accounts\"}),\", click \",(0,r.jsx)(e.strong,{children:\"References\"}),\", and choose a product to explore supported endpoints.\"]}),`\n`,(0,r.jsx)(e.li,{children:(0,r.jsx)(e.a,{href:\"/docs/api/phone/ma\",children:\"Phone primary account API\"})}),`\n`,(0,r.jsx)(e.li,{children:(0,r.jsx)(e.a,{href:\"/docs/api/video-sdk/ma\",children:\"Video SDK primary account API\"})}),`\n`,(0,r.jsx)(e.li,{children:(0,r.jsx)(e.a,{href:\"/docs/api/references/manage-plans//#video-sdk-account-plans\",children:\"Video SDK Account Plans\"})}),`\n`]})]})}function m(n={}){let{wrapper:e}={...s(),...n.components};return e?(0,r.jsx)(e,{...n,children:(0,r.jsx)(l,{...n})}):l(n)}function h(n,e){throw new Error(\"Expected \"+(e?\"component\":\"object\")+\" `\"+n+\"` to be defined: you likely forgot to import, pass, or provide it.\")}return v(w);})();\n;return Component;",
      "frontmatter": {
        "title": "Primary (master) account APIs"
      },
      "errors": [],
      "matter": {
        "content": "\n# Primary (master) account APIs\n\nPrimary accounts (formerly referred to as master accounts) are Zoom accounts that manage one or more sub accounts. The terms primary and master may be used interchangeably throughout the documentation.\n\nFor example:\n\n- A university may use a primary account to manage separate sub accounts for each department.\n- An enterprise might use a primary account to manage business units that operate under different regulatory requirements.\n- Zoom [reseller and distributor technology partners](https://partner.zoom.com/) can also be primary account holders.\n\nPrimary accounts have access to primary/master account APIs, which allow them to programmatically manage activities related to their sub accounts.\n\n<Alert variant=\"warning\">\n\n**Primary account API access limitations**\n\nPrimary account APIs can only be used to manage their **own** sub accounts. They **cannot** be used to access or manage other primary accounts.\n\n</Alert>\n\n## Tiered account structures\n\nZoom partners, such as resellers and distributors, can use primary accounts to provision **two-tiered** or **three-tiered** account structures:\n\n* **Two-tiered** - A single primary account manages multiple sub accounts directly.\n* **Three-tiered** - A top-level primary account manages one or more **second-level** primary accounts. Each second-level account can, in turn, provision and manage its own sub accounts.\n\n<Image src=\"/img/1636145489712.png\" />\n\n***For enrollment or business queries regarding reseller and distributor technology partner programs, email [**partner-success@zoom.us**](mailto:partner-success@zoom.us).***\n\n## Using primary/master account APIs\n\nPrimary account API endpoints are prepended with `/accounts/{accountId}`. The request and response formats mirror those of standard Zoom APIs.\n\nFor example, a regular Zoom account uses the [GET /users](/docs/api/users/#tag/users/GET/users) endpoint to list all of its users. In contrast, a primary account lists users in a sub account using the [List users (`GET /accounts/{accountId}/users`)](/docs/api/users/ma/#tag/users/get/accounts/{accountId}/users) endpoint.\n\nTo call primary account API endpoints, the primary account must authenticate using one of the following:\n\n* An [OAuth token](/docs/integrations/oauth/) associated with a user under the primary account\n* A [Server-to-Server OAuth token](/docs/internal-apps/s2s-oauth/)\n\nThe primary account must have the **Manage the sub account** role enabled to access and manage sub account data via API. See [Using role management](https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0064983) and [Managing Zoom sub accounts](https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0066764) for details.\n\nTo get a sub account's `accountId`, use the [Get account settings](/docs/api/accounts/#tag/accounts/get/accounts/{accountId}/settings) API and pass `me` as the `accountId` value.\n\n### Related References\n\n* Zoom primary account APIs - In the sidebar, under **Master accounts**, click **References**, and choose a product to explore supported endpoints.\n* [Phone primary account API](/docs/api/phone/ma)\n* [Video SDK primary account API](/docs/api/video-sdk/ma)\n* [Video SDK Account Plans](/docs/api/references/manage-plans//#video-sdk-account-plans)\n",
        "data": {
          "title": "Primary (master) account APIs"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/ma.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
