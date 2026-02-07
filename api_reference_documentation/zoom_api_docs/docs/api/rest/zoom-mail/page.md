# Zoom Mail API

- Source URL: https://developers.zoom.us/docs/api/rest/zoom-mail/
- Snapshot path: docs/api/rest/zoom-mail
- Generated (UTC): 2026-02-07T22:40:11.533775+00:00

## Frontmatter

```json
{
  "title": "Zoom Mail API"
}
```

## Content

```md

# Zoom Mail API

The Zoom Mail API lets developers access information from Zoom. Use this API to build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us/). Learn how to get your credentials and create private or public apps in our [Authorization guide](/docs/integrations/oauth/).
The API accepts `application/json` and `multipart/form-data`, and `responds with application/json` and `application/xml`. All endpoints are available at `https://api.zoom.us/v2/`.


To use this API, you must have:

* Zoom Workplace Pro or Standard Pro, Zoom Workplace Business, or Zoom Workplace Enterprise account
* Account owner or admin privileges


**Note**: Pro accounts have the Zoom Mail Services enabled by default. For Business and Enterprise accounts, the Zoom Mail Services are disabled by default.


## More resources

* [Methods](/docs/api/mail)
* [Terms of Service](https://explore.zoom.us/en/legal/zoom-api-license-and-tou/)
* [Plans](https://zoom.us/pricing/)

## Make your first API calls

The Zoom Mail API grant developers access to retrieve information from Zoom, including mailbox data such as threads, messages, and labels.

### Use Case:

* Integrate email.

* Ensure data security with encryption.

* Personalize email experience.



### Endpoints:

Use  `POST /emails/mailboxes/{email}/messages` endpoint, a way that directly inserts a message into this mailbox.

This API call enables you to effectively manage event attendance effectiveness. You can find more information and interactively test these calls in [Postman](https://www.postman.com/zoom-developer/workspace/zoom-public-workspace/collection/33172429-276e0718-2867-44fb-8c7d-5e15c2cf6364).

For more information on using Postman click [here](/docs/api/#get-started-in-postman).
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
      "code": "var Component=(()=>{var m=Object.create;var a=Object.defineProperty;var u=Object.getOwnPropertyDescriptor;var f=Object.getOwnPropertyNames;var g=Object.getPrototypeOf,v=Object.prototype.hasOwnProperty;var x=(i,e)=>()=>(e||i((e={exports:{}}).exports,e),e.exports),P=(i,e)=>{for(var o in e)a(i,o,{get:e[o],enumerable:!0})},c=(i,e,o,s)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let r of f(e))!v.call(i,r)&&r!==o&&a(i,r,{get:()=>e[r],enumerable:!(s=u(e,r))||s.enumerable});return i};var M=(i,e,o)=>(o=i!=null?m(g(i)):{},c(e||!i||!i.__esModule?a(o,\"default\",{value:i,enumerable:!0}):o,i)),b=i=>c(a({},\"__esModule\",{value:!0}),i);var d=x((k,l)=>{l.exports=_jsx_runtime});var A={};P(A,{default:()=>p,frontmatter:()=>y});var n=M(d());var{useMDXComponents:t}=MdxJsReact;var y={title:\"Zoom Mail API\"};function h(i){let e={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",p:\"p\",strong:\"strong\",ul:\"ul\",...t(),...i.components};return(0,n.jsxs)(n.Fragment,{children:[(0,n.jsx)(e.h1,{id:\"zoom-mail-api\",children:\"Zoom Mail API\"}),`\n`,(0,n.jsxs)(e.p,{children:[\"The Zoom Mail API lets developers access information from Zoom. Use this API to build private services or public applications on the \",(0,n.jsx)(e.a,{href:\"http://marketplace.zoom.us/\",children:\"Zoom App Marketplace\"}),\". Learn how to get your credentials and create private or public apps in our \",(0,n.jsx)(e.a,{href:\"/docs/integrations/oauth/\",children:\"Authorization guide\"}),`.\nThe API accepts `,(0,n.jsx)(e.code,{children:\"application/json\"}),\" and \",(0,n.jsx)(e.code,{children:\"multipart/form-data\"}),\", and \",(0,n.jsx)(e.code,{children:\"responds with application/json\"}),\" and \",(0,n.jsx)(e.code,{children:\"application/xml\"}),\". All endpoints are available at \",(0,n.jsx)(e.code,{children:\"https://api.zoom.us/v2/\"}),\".\"]}),`\n`,(0,n.jsx)(e.p,{children:\"To use this API, you must have:\"}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsx)(e.li,{children:\"Zoom Workplace Pro or Standard Pro, Zoom Workplace Business, or Zoom Workplace Enterprise account\"}),`\n`,(0,n.jsx)(e.li,{children:\"Account owner or admin privileges\"}),`\n`]}),`\n`,(0,n.jsxs)(e.p,{children:[(0,n.jsx)(e.strong,{children:\"Note\"}),\": Pro accounts have the Zoom Mail Services enabled by default. For Business and Enterprise accounts, the Zoom Mail Services are disabled by default.\"]}),`\n`,(0,n.jsx)(e.h2,{id:\"more-resources\",children:(0,n.jsxs)(e.a,{href:\"#more-resources\",children:[\"More resources\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsx)(e.li,{children:(0,n.jsx)(e.a,{href:\"/docs/api/mail\",children:\"Methods\"})}),`\n`,(0,n.jsx)(e.li,{children:(0,n.jsx)(e.a,{href:\"https://explore.zoom.us/en/legal/zoom-api-license-and-tou/\",children:\"Terms of Service\"})}),`\n`,(0,n.jsx)(e.li,{children:(0,n.jsx)(e.a,{href:\"https://zoom.us/pricing/\",children:\"Plans\"})}),`\n`]}),`\n`,(0,n.jsx)(e.h2,{id:\"make-your-first-api-calls\",children:(0,n.jsxs)(e.a,{href:\"#make-your-first-api-calls\",children:[\"Make your first API calls\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsx)(e.p,{children:\"The Zoom Mail API grant developers access to retrieve information from Zoom, including mailbox data such as threads, messages, and labels.\"}),`\n`,(0,n.jsx)(e.h3,{id:\"use-case\",children:(0,n.jsxs)(e.a,{href:\"#use-case\",children:[\"Use Case:\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsxs)(e.li,{children:[`\n`,(0,n.jsx)(e.p,{children:\"Integrate email.\"}),`\n`]}),`\n`,(0,n.jsxs)(e.li,{children:[`\n`,(0,n.jsx)(e.p,{children:\"Ensure data security with encryption.\"}),`\n`]}),`\n`,(0,n.jsxs)(e.li,{children:[`\n`,(0,n.jsx)(e.p,{children:\"Personalize email experience.\"}),`\n`]}),`\n`]}),`\n`,(0,n.jsx)(e.h3,{id:\"endpoints\",children:(0,n.jsxs)(e.a,{href:\"#endpoints\",children:[\"Endpoints:\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.p,{children:[\"Use  \",(0,n.jsx)(e.code,{children:\"POST /emails/mailboxes/{email}/messages\"}),\" endpoint, a way that directly inserts a message into this mailbox.\"]}),`\n`,(0,n.jsxs)(e.p,{children:[\"This API call enables you to effectively manage event attendance effectiveness. You can find more information and interactively test these calls in \",(0,n.jsx)(e.a,{href:\"https://www.postman.com/zoom-developer/workspace/zoom-public-workspace/collection/33172429-276e0718-2867-44fb-8c7d-5e15c2cf6364\",children:\"Postman\"}),\".\"]}),`\n`,(0,n.jsxs)(e.p,{children:[\"For more information on using Postman click \",(0,n.jsx)(e.a,{href:\"/docs/api/#get-started-in-postman\",children:\"here\"}),\".\"]})]})}function p(i={}){let{wrapper:e}={...t(),...i.components};return e?(0,n.jsx)(e,{...i,children:(0,n.jsx)(h,{...i})}):h(i)}return b(A);})();\n;return Component;",
      "frontmatter": {
        "title": "Zoom Mail API"
      },
      "errors": [],
      "matter": {
        "content": "\n# Zoom Mail API\n\nThe Zoom Mail API lets developers access information from Zoom. Use this API to build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us/). Learn how to get your credentials and create private or public apps in our [Authorization guide](/docs/integrations/oauth/).\nThe API accepts `application/json` and `multipart/form-data`, and `responds with application/json` and `application/xml`. All endpoints are available at `https://api.zoom.us/v2/`.\n\n\nTo use this API, you must have:\n\n* Zoom Workplace Pro or Standard Pro, Zoom Workplace Business, or Zoom Workplace Enterprise account\n* Account owner or admin privileges\n\n\n**Note**: Pro accounts have the Zoom Mail Services enabled by default. For Business and Enterprise accounts, the Zoom Mail Services are disabled by default.\n\n\n## More resources\n\n* [Methods](/docs/api/mail)\n* [Terms of Service](https://explore.zoom.us/en/legal/zoom-api-license-and-tou/)\n* [Plans](https://zoom.us/pricing/)\n\n## Make your first API calls\n\nThe Zoom Mail API grant developers access to retrieve information from Zoom, including mailbox data such as threads, messages, and labels.\n\n### Use Case:\n\n* Integrate email.\n\n* Ensure data security with encryption.\n\n* Personalize email experience.\n\n\n\n### Endpoints:\n\nUse  `POST /emails/mailboxes/{email}/messages` endpoint, a way that directly inserts a message into this mailbox.\n\nThis API call enables you to effectively manage event attendance effectiveness. You can find more information and interactively test these calls in [Postman](https://www.postman.com/zoom-developer/workspace/zoom-public-workspace/collection/33172429-276e0718-2867-44fb-8c7d-5e15c2cf6364).\n\nFor more information on using Postman click [here](/docs/api/#get-started-in-postman).",
        "data": {
          "title": "Zoom Mail API"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/rest/zoom-mail.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
