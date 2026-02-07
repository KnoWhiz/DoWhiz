# Number Management API

- Source URL: https://developers.zoom.us/docs/api/rest/number-management/
- Snapshot path: docs/api/rest/number-management
- Generated (UTC): 2026-02-07T22:40:11.527068+00:00

## Frontmatter

```json
{
  "title": "Number Management API"
}
```

## Content

```md

# Number Management API

### Before you begin
- Learn the [basics](/docs/api/using-zoom-apis/) on how to build with Zoom APIs.
- Learn about [webhooks](/docs/api/webhooks/) and how to get data delivered to your designated URL.

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
      "code": "var Component=(()=>{var u=Object.create;var t=Object.defineProperty;var m=Object.getOwnPropertyDescriptor;var g=Object.getOwnPropertyNames;var b=Object.getPrototypeOf,f=Object.prototype.hasOwnProperty;var x=(o,n)=>()=>(n||o((n={exports:{}}).exports,n),n.exports),M=(o,n)=>{for(var r in n)t(o,r,{get:n[r],enumerable:!0})},a=(o,n,r,c)=>{if(n&&typeof n==\"object\"||typeof n==\"function\")for(let i of g(n))!f.call(o,i)&&i!==r&&t(o,i,{get:()=>n[i],enumerable:!(c=m(n,i))||c.enumerable});return o};var _=(o,n,r)=>(r=o!=null?u(b(o)):{},a(n||!o||!o.__esModule?t(r,\"default\",{value:o,enumerable:!0}):r,o)),j=o=>a(t({},\"__esModule\",{value:!0}),o);var h=x((y,l)=>{l.exports=_jsx_runtime});var A={};M(A,{default:()=>p,frontmatter:()=>w});var e=_(h());var{useMDXComponents:d}=MdxJsReact;var w={title:\"Number Management API\"};function s(o){let n={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",p:\"p\",strong:\"strong\",ul:\"ul\",...d(),...o.components};return(0,e.jsxs)(e.Fragment,{children:[(0,e.jsx)(n.h1,{id:\"number-management-api\",children:\"Number Management API\"}),`\n`,(0,e.jsx)(n.h3,{id:\"before-you-begin\",children:(0,e.jsxs)(n.a,{href:\"#before-you-begin\",children:[\"Before you begin\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsxs)(n.li,{children:[\"Learn the \",(0,e.jsx)(n.a,{href:\"/docs/api/using-zoom-apis/\",children:\"basics\"}),\" on how to build with Zoom APIs.\"]}),`\n`,(0,e.jsxs)(n.li,{children:[\"Learn about \",(0,e.jsx)(n.a,{href:\"/docs/api/webhooks/\",children:\"webhooks\"}),\" and how to get data delivered to your designated URL.\"]}),`\n`]}),`\n`,(0,e.jsx)(n.h2,{id:\"endpoints\",children:(0,e.jsxs)(n.a,{href:\"#endpoints\",children:[\"Endpoints\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.p,{children:[\"All endpoints are available through \",(0,e.jsx)(n.code,{children:\"https\"}),\" at \",(0,e.jsx)(n.code,{children:\"api.zoom.us/v2/\"}),\".\"]}),`\n`,(0,e.jsx)(n.h2,{id:\"resources-and-information\",children:(0,e.jsxs)(n.a,{href:\"#resources-and-information\",children:[\"Resources and information\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.a,{href:\"/docs/integrations/oauth/\",children:(0,e.jsx)(n.strong,{children:\"OAuth 2.0 for Zoom\"})}),\" \",(0,e.jsx)(\"br\",{}),`\nHow to get your credentials and create private or public applications`]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Version\"}),\" \",(0,e.jsx)(\"br\",{}),`\n2.0.0`]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Host\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"api.zoom.us/v2\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Protocols\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"https\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Accepts\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"application/json\"}),\", \",(0,e.jsx)(n.code,{children:\"multipart/form-data\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Responds With\"}),(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"application/json\"}),\", \",(0,e.jsx)(n.code,{children:\"application/xml\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Zoom API License and Terms of Use\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"https://explore.zoom.us/en/legal/zoom-api-license-and-tou/\"})]}),`\n`]})]})}function p(o={}){let{wrapper:n}={...d(),...o.components};return n?(0,e.jsx)(n,{...o,children:(0,e.jsx)(s,{...o})}):s(o)}return j(A);})();\n;return Component;",
      "frontmatter": {
        "title": "Number Management API"
      },
      "errors": [],
      "matter": {
        "content": "\n# Number Management API\n\n### Before you begin\n- Learn the [basics](/docs/api/using-zoom-apis/) on how to build with Zoom APIs.\n- Learn about [webhooks](/docs/api/webhooks/) and how to get data delivered to your designated URL.\n\n## Endpoints\nAll endpoints are available through `https` at `api.zoom.us/v2/`.\n\n## Resources and information\n- [**OAuth 2.0 for Zoom**](/docs/integrations/oauth/) <br/>\nHow to get your credentials and create private or public applications\n- **Version** <br/>\n  2.0.0\n- **Host** <br/>\n `api.zoom.us/v2`\n- **Protocols** <br/>\n  `https`\n- **Accepts** <br/>\n `application/json`, `multipart/form-data`\n- **Responds With**<br/>\n  `application/json`, `application/xml`\n- **Zoom API License and Terms of Use** <br/>\n `https://explore.zoom.us/en/legal/zoom-api-license-and-tou/`\n",
        "data": {
          "title": "Number Management API"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/rest/number-management.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
