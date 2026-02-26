# Zoom Whiteboard API

- Source URL: https://developers.zoom.us/docs/api/rest/whiteboard/
- Snapshot path: docs/api/rest/whiteboard
- Generated (UTC): 2026-02-07T22:40:11.530804+00:00

## Frontmatter

```json
{
  "title": "Zoom Whiteboard API"
}
```

## Content

```md

# Zoom Whiteboard API

The Zoom Whiteboards API lets developers access information from Zoom. Use this API to build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us/). Learn how to get your credentials and create private or public apps in our [Authorization guide](/docs/integrations/oauth/). 
The API accepts `application/json` and `multipart/form-data`, and `responds with application/json` and `application/xml`. All endpoints are available at `https://api.zoom.us/v2/`.
To use this API, you must have (list the plan(s)) and (the credentials). Some webhooks may require a higher account level.

## More resources

* [Methods](/docs/api/whiteboard)
* [Webhooks](/docs/api/whiteboard/events)
* [Terms of Service](https://explore.zoom.us/en/legal/zoom-api-license-and-tou/)
* [Plans](https://zoom.us/pricing/whiteboard)

## Make your first API calls

The Zoom Whiteboard API empowers interactive collaboration by enabling integration with whiteboard features.
    
### Use Case:

* Create a Zoom Whiteboard user.
* Facilitate brainstorming sessions with a remote team member.
* Conduct interactive training or educational sessions online.

### Endpoints:

Use  `POST/v2/whiteboards/{whiteboardId}/collaborator` endpoint, offering a way to initiate collaborative drawing and note-taking experiences for users.
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
      "code": "var Component=(()=>{var u=Object.create;var a=Object.defineProperty;var m=Object.getOwnPropertyDescriptor;var f=Object.getOwnPropertyNames;var b=Object.getPrototypeOf,w=Object.prototype.hasOwnProperty;var g=(n,e)=>()=>(e||n((e={exports:{}}).exports,e),e.exports),v=(n,e)=>{for(var o in e)a(n,o,{get:e[o],enumerable:!0})},d=(n,e,o,s)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let r of f(e))!w.call(n,r)&&r!==o&&a(n,r,{get:()=>e[r],enumerable:!(s=m(e,r))||s.enumerable});return n};var x=(n,e,o)=>(o=n!=null?u(b(n)):{},d(e||!n||!n.__esModule?a(o,\"default\",{value:n,enumerable:!0}):o,n)),A=n=>d(a({},\"__esModule\",{value:!0}),n);var c=g((I,h)=>{h.exports=_jsx_runtime});var P={};v(P,{default:()=>p,frontmatter:()=>M});var i=x(c());var{useMDXComponents:t}=MdxJsReact;var M={title:\"Zoom Whiteboard API\"};function l(n){let e={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",p:\"p\",ul:\"ul\",...t(),...n.components};return(0,i.jsxs)(i.Fragment,{children:[(0,i.jsx)(e.h1,{id:\"zoom-whiteboard-api\",children:\"Zoom Whiteboard API\"}),`\n`,(0,i.jsxs)(e.p,{children:[\"The Zoom Whiteboards API lets developers access information from Zoom. Use this API to build private services or public applications on the \",(0,i.jsx)(e.a,{href:\"http://marketplace.zoom.us/\",children:\"Zoom App Marketplace\"}),\". Learn how to get your credentials and create private or public apps in our \",(0,i.jsx)(e.a,{href:\"/docs/integrations/oauth/\",children:\"Authorization guide\"}),`.\nThe API accepts `,(0,i.jsx)(e.code,{children:\"application/json\"}),\" and \",(0,i.jsx)(e.code,{children:\"multipart/form-data\"}),\", and \",(0,i.jsx)(e.code,{children:\"responds with application/json\"}),\" and \",(0,i.jsx)(e.code,{children:\"application/xml\"}),\". All endpoints are available at \",(0,i.jsx)(e.code,{children:\"https://api.zoom.us/v2/\"}),`.\nTo use this API, you must have (list the plan(s)) and (the credentials). Some webhooks may require a higher account level.`]}),`\n`,(0,i.jsx)(e.h2,{id:\"more-resources\",children:(0,i.jsxs)(e.a,{href:\"#more-resources\",children:[\"More resources\",(0,i.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,i.jsxs)(e.ul,{children:[`\n`,(0,i.jsx)(e.li,{children:(0,i.jsx)(e.a,{href:\"/docs/api/whiteboard\",children:\"Methods\"})}),`\n`,(0,i.jsx)(e.li,{children:(0,i.jsx)(e.a,{href:\"/docs/api/whiteboard/events\",children:\"Webhooks\"})}),`\n`,(0,i.jsx)(e.li,{children:(0,i.jsx)(e.a,{href:\"https://explore.zoom.us/en/legal/zoom-api-license-and-tou/\",children:\"Terms of Service\"})}),`\n`,(0,i.jsx)(e.li,{children:(0,i.jsx)(e.a,{href:\"https://zoom.us/pricing/whiteboard\",children:\"Plans\"})}),`\n`]}),`\n`,(0,i.jsx)(e.h2,{id:\"make-your-first-api-calls\",children:(0,i.jsxs)(e.a,{href:\"#make-your-first-api-calls\",children:[\"Make your first API calls\",(0,i.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,i.jsx)(e.p,{children:\"The Zoom Whiteboard API empowers interactive collaboration by enabling integration with whiteboard features.\"}),`\n`,(0,i.jsx)(e.h3,{id:\"use-case\",children:(0,i.jsxs)(e.a,{href:\"#use-case\",children:[\"Use Case:\",(0,i.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,i.jsxs)(e.ul,{children:[`\n`,(0,i.jsx)(e.li,{children:\"Create a Zoom Whiteboard user.\"}),`\n`,(0,i.jsx)(e.li,{children:\"Facilitate brainstorming sessions with a remote team member.\"}),`\n`,(0,i.jsx)(e.li,{children:\"Conduct interactive training or educational sessions online.\"}),`\n`]}),`\n`,(0,i.jsx)(e.h3,{id:\"endpoints\",children:(0,i.jsxs)(e.a,{href:\"#endpoints\",children:[\"Endpoints:\",(0,i.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,i.jsxs)(e.p,{children:[\"Use  \",(0,i.jsx)(e.code,{children:\"POST/v2/whiteboards/{whiteboardId}/collaborator\"}),\" endpoint, offering a way to initiate collaborative drawing and note-taking experiences for users.\"]})]})}function p(n={}){let{wrapper:e}={...t(),...n.components};return e?(0,i.jsx)(e,{...n,children:(0,i.jsx)(l,{...n})}):l(n)}return A(P);})();\n;return Component;",
      "frontmatter": {
        "title": "Zoom Whiteboard API"
      },
      "errors": [],
      "matter": {
        "content": "\n# Zoom Whiteboard API\n\nThe Zoom Whiteboards API lets developers access information from Zoom. Use this API to build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us/). Learn how to get your credentials and create private or public apps in our [Authorization guide](/docs/integrations/oauth/). \nThe API accepts `application/json` and `multipart/form-data`, and `responds with application/json` and `application/xml`. All endpoints are available at `https://api.zoom.us/v2/`.\nTo use this API, you must have (list the plan(s)) and (the credentials). Some webhooks may require a higher account level.\n\n## More resources\n\n* [Methods](/docs/api/whiteboard)\n* [Webhooks](/docs/api/whiteboard/events)\n* [Terms of Service](https://explore.zoom.us/en/legal/zoom-api-license-and-tou/)\n* [Plans](https://zoom.us/pricing/whiteboard)\n\n## Make your first API calls\n\nThe Zoom Whiteboard API empowers interactive collaboration by enabling integration with whiteboard features.\n    \n### Use Case:\n\n* Create a Zoom Whiteboard user.\n* Facilitate brainstorming sessions with a remote team member.\n* Conduct interactive training or educational sessions online.\n\n### Endpoints:\n\nUse  `POST/v2/whiteboards/{whiteboardId}/collaborator` endpoint, offering a way to initiate collaborative drawing and note-taking experiences for users.\n",
        "data": {
          "title": "Zoom Whiteboard API"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/rest/whiteboard.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
