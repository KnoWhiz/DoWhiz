# Zoom Rooms API

- Source URL: https://developers.zoom.us/docs/api/rest/zoom-rooms-api/
- Snapshot path: docs/api/rest/zoom-rooms-api
- Generated (UTC): 2026-02-07T22:40:11.535170+00:00

## Frontmatter

```json
{
  "title": "Zoom Rooms API"
}
```

## Content

```md

# Zoom Rooms API

The Zoom Rooms API lets developers access information from Zoom. Use this API to build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us/). Learn how to get your credentials and create private or public apps in our [Authorization guide](/docs/integrations/oauth/).

The API accepts `application/json` and `multipart/form-data`, and responds with `application/json` and `application/xml`. All endpoints are available at https://api.zoom.us/v2/.

To use this API, you must have:

* Zoom Rooms license.
* Role with access to Zoom Rooms on the web portal.

## More resources

Documentation: [Endpoints](/docs/api/rooms/), [Webhooks](/docs/api/rooms/events), [Release notes](/changelog/zoom-rooms/), [Postman hub](https://www.postman.com/zoom-developer/workspace/zoom-public-workspace/collection/22097587-d24ad564-1750-4c7f-a524-687e96729ccb), [Terms of Service](https://explore.zoom.us/en/legal/zoom-api-license-and-tou/), [Plans](https://zoom.us/pricing/zoom-rooms)

## Make your first API calls

The Zoom Rooms API facilitates seamless control of Zoom meeting rooms.

### Use Case:

* Get information about room details.
* Create tailored meeting experiences in Zoom Rooms.
* Streamline the management of multiple Zoom Rooms.

### Endpoints:

* `GET/v2/rooms/{roomId}` endpoint which provides access to room-specific data such as name, location and more.

By utilizing the Zoom Rooms API, you can streamline room management and enhance user experiences.  You can find more information and interactively test this call using [Postman](https://www.postman.com/zoom-developer/workspace/zoom-public-workspace/collection/22097587-d24ad564-1750-4c7f-a524-687e96729ccb).
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
      "code": "var Component=(()=>{var p=Object.create;var a=Object.defineProperty;var u=Object.getOwnPropertyDescriptor;var f=Object.getOwnPropertyNames;var v=Object.getPrototypeOf,w=Object.prototype.hasOwnProperty;var g=(n,e)=>()=>(e||n((e={exports:{}}).exports,e),e.exports),x=(n,e)=>{for(var i in e)a(n,i,{get:e[i],enumerable:!0})},c=(n,e,i,s)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let r of f(e))!w.call(n,r)&&r!==i&&a(n,r,{get:()=>e[r],enumerable:!(s=u(e,r))||s.enumerable});return n};var z=(n,e,i)=>(i=n!=null?p(v(n)):{},c(e||!n||!n.__esModule?a(i,\"default\",{value:n,enumerable:!0}):i,n)),b=n=>c(a({},\"__esModule\",{value:!0}),n);var d=g((Z,l)=>{l.exports=_jsx_runtime});var P={};x(P,{default:()=>m,frontmatter:()=>A});var o=z(d());var{useMDXComponents:t}=MdxJsReact;var A={title:\"Zoom Rooms API\"};function h(n){let e={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",p:\"p\",ul:\"ul\",...t(),...n.components};return(0,o.jsxs)(o.Fragment,{children:[(0,o.jsx)(e.h1,{id:\"zoom-rooms-api\",children:\"Zoom Rooms API\"}),`\n`,(0,o.jsxs)(e.p,{children:[\"The Zoom Rooms API lets developers access information from Zoom. Use this API to build private services or public applications on the \",(0,o.jsx)(e.a,{href:\"http://marketplace.zoom.us/\",children:\"Zoom App Marketplace\"}),\". Learn how to get your credentials and create private or public apps in our \",(0,o.jsx)(e.a,{href:\"/docs/integrations/oauth/\",children:\"Authorization guide\"}),\".\"]}),`\n`,(0,o.jsxs)(e.p,{children:[\"The API accepts \",(0,o.jsx)(e.code,{children:\"application/json\"}),\" and \",(0,o.jsx)(e.code,{children:\"multipart/form-data\"}),\", and responds with \",(0,o.jsx)(e.code,{children:\"application/json\"}),\" and \",(0,o.jsx)(e.code,{children:\"application/xml\"}),\". All endpoints are available at \",(0,o.jsx)(e.a,{href:\"https://api.zoom.us/v2/\",children:\"https://api.zoom.us/v2/\"}),\".\"]}),`\n`,(0,o.jsx)(e.p,{children:\"To use this API, you must have:\"}),`\n`,(0,o.jsxs)(e.ul,{children:[`\n`,(0,o.jsx)(e.li,{children:\"Zoom Rooms license.\"}),`\n`,(0,o.jsx)(e.li,{children:\"Role with access to Zoom Rooms on the web portal.\"}),`\n`]}),`\n`,(0,o.jsx)(e.h2,{id:\"more-resources\",children:(0,o.jsxs)(e.a,{href:\"#more-resources\",children:[\"More resources\",(0,o.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,o.jsxs)(e.p,{children:[\"Documentation: \",(0,o.jsx)(e.a,{href:\"/docs/api/rooms/\",children:\"Endpoints\"}),\", \",(0,o.jsx)(e.a,{href:\"/docs/api/rooms/events\",children:\"Webhooks\"}),\", \",(0,o.jsx)(e.a,{href:\"/changelog/zoom-rooms/\",children:\"Release notes\"}),\", \",(0,o.jsx)(e.a,{href:\"https://www.postman.com/zoom-developer/workspace/zoom-public-workspace/collection/22097587-d24ad564-1750-4c7f-a524-687e96729ccb\",children:\"Postman hub\"}),\", \",(0,o.jsx)(e.a,{href:\"https://explore.zoom.us/en/legal/zoom-api-license-and-tou/\",children:\"Terms of Service\"}),\", \",(0,o.jsx)(e.a,{href:\"https://zoom.us/pricing/zoom-rooms\",children:\"Plans\"})]}),`\n`,(0,o.jsx)(e.h2,{id:\"make-your-first-api-calls\",children:(0,o.jsxs)(e.a,{href:\"#make-your-first-api-calls\",children:[\"Make your first API calls\",(0,o.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,o.jsx)(e.p,{children:\"The Zoom Rooms API facilitates seamless control of Zoom meeting rooms.\"}),`\n`,(0,o.jsx)(e.h3,{id:\"use-case\",children:(0,o.jsxs)(e.a,{href:\"#use-case\",children:[\"Use Case:\",(0,o.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,o.jsxs)(e.ul,{children:[`\n`,(0,o.jsx)(e.li,{children:\"Get information about room details.\"}),`\n`,(0,o.jsx)(e.li,{children:\"Create tailored meeting experiences in Zoom Rooms.\"}),`\n`,(0,o.jsx)(e.li,{children:\"Streamline the management of multiple Zoom Rooms.\"}),`\n`]}),`\n`,(0,o.jsx)(e.h3,{id:\"endpoints\",children:(0,o.jsxs)(e.a,{href:\"#endpoints\",children:[\"Endpoints:\",(0,o.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,o.jsxs)(e.ul,{children:[`\n`,(0,o.jsxs)(e.li,{children:[(0,o.jsx)(e.code,{children:\"GET/v2/rooms/{roomId}\"}),\" endpoint which provides access to room-specific data such as name, location and more.\"]}),`\n`]}),`\n`,(0,o.jsxs)(e.p,{children:[\"By utilizing the Zoom Rooms API, you can streamline room management and enhance user experiences.  You can find more information and interactively test this call using \",(0,o.jsx)(e.a,{href:\"https://www.postman.com/zoom-developer/workspace/zoom-public-workspace/collection/22097587-d24ad564-1750-4c7f-a524-687e96729ccb\",children:\"Postman\"}),\".\"]})]})}function m(n={}){let{wrapper:e}={...t(),...n.components};return e?(0,o.jsx)(e,{...n,children:(0,o.jsx)(h,{...n})}):h(n)}return b(P);})();\n;return Component;",
      "frontmatter": {
        "title": "Zoom Rooms API"
      },
      "errors": [],
      "matter": {
        "content": "\n# Zoom Rooms API\n\nThe Zoom Rooms API lets developers access information from Zoom. Use this API to build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us/). Learn how to get your credentials and create private or public apps in our [Authorization guide](/docs/integrations/oauth/).\n\nThe API accepts `application/json` and `multipart/form-data`, and responds with `application/json` and `application/xml`. All endpoints are available at https://api.zoom.us/v2/.\n\nTo use this API, you must have:\n\n* Zoom Rooms license.\n* Role with access to Zoom Rooms on the web portal.\n\n## More resources\n\nDocumentation: [Endpoints](/docs/api/rooms/), [Webhooks](/docs/api/rooms/events), [Release notes](/changelog/zoom-rooms/), [Postman hub](https://www.postman.com/zoom-developer/workspace/zoom-public-workspace/collection/22097587-d24ad564-1750-4c7f-a524-687e96729ccb), [Terms of Service](https://explore.zoom.us/en/legal/zoom-api-license-and-tou/), [Plans](https://zoom.us/pricing/zoom-rooms)\n\n## Make your first API calls\n\nThe Zoom Rooms API facilitates seamless control of Zoom meeting rooms.\n\n### Use Case:\n\n* Get information about room details.\n* Create tailored meeting experiences in Zoom Rooms.\n* Streamline the management of multiple Zoom Rooms.\n\n### Endpoints:\n\n* `GET/v2/rooms/{roomId}` endpoint which provides access to room-specific data such as name, location and more.\n\nBy utilizing the Zoom Rooms API, you can streamline room management and enhance user experiences.  You can find more information and interactively test this call using [Postman](https://www.postman.com/zoom-developer/workspace/zoom-public-workspace/collection/22097587-d24ad564-1750-4c7f-a524-687e96729ccb).\n",
        "data": {
          "title": "Zoom Rooms API"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/rest/zoom-rooms-api.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
