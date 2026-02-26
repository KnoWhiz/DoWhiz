# Conference Room Connector (CRC)

- Source URL: https://developers.zoom.us/docs/api/rest/crc-api/
- Snapshot path: docs/api/rest/crc-api
- Generated (UTC): 2026-02-07T22:40:11.524745+00:00

## Frontmatter

```json
{
  "title": "Conference Room Connector (CRC)"
}
```

## Content

```md
# Conference Room Connector API
The Conference Room Connector API is an interface that allows developers to integrate and extend the functionality of a conference room connector service.  [Zoom App Marketplace](http://marketplace.zoom.us/). Learn how to get your credentials and create private or public apps in our [Authorization guide](/docs/integrations/oauth/).

The API accepts `application/json` and `multipart/form-data`, and responds with `application/json` and `application/xml`. All endpoints are available at https://api.zoom.us/v2/.

To use this API, you must have:

* You must be subscribed to the H.323/SIP Room Connector, which requires:
  * Pro, Business, Education, or Enterprise account
Login to [http://zoom.us/signin](http://zoom.us/signin).
  * Go to [Billing](https://zoom.us/billing) and subscribe to H.323/SIP Room Connector or [contact sales](https://zoom.us/contactsales).
* Only **Licensed** users can schedule meetings with Room Connector.
* See [Room Connector Support Devices](https://support.zoom.us/hc/en-us/articles/202445433-Room-Connector-Supported-Devices).

## More resources

Documentation: [Endpoints](/docs/api/crc/#tag/api-connector), [Release notes](/changelog/crc/), [Terms of Service](https://explore.zoom.us/en/legal/zoom-api-license-and-tou/), [Plans](https://zoom.us/pricing/zoom-rooms)
## The Conference Room Connector API

### Use Case:

* Integration with Room Systems.
* An H.323 or SIP device can make a video call to a Room Connector to join a Zoom cloud meeting.
* A Room Connector can also call out to an H.323 or SIP device to join a Zoom cloud meeting.
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
      "code": "var Component=(()=>{var u=Object.create;var c=Object.defineProperty;var m=Object.getOwnPropertyDescriptor;var f=Object.getOwnPropertyNames;var g=Object.getPrototypeOf,C=Object.prototype.hasOwnProperty;var R=(o,e)=>()=>(e||o((e={exports:{}}).exports,e),e.exports),v=(o,e)=>{for(var r in e)c(o,r,{get:e[r],enumerable:!0})},s=(o,e,r,a)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let t of f(e))!C.call(o,t)&&t!==r&&c(o,t,{get:()=>e[t],enumerable:!(a=m(e,t))||a.enumerable});return o};var x=(o,e,r)=>(r=o!=null?u(g(o)):{},s(e||!o||!o.__esModule?c(r,\"default\",{value:o,enumerable:!0}):r,o)),z=o=>s(c({},\"__esModule\",{value:!0}),o);var l=R((j,h)=>{h.exports=_jsx_runtime});var A={};v(A,{default:()=>p,frontmatter:()=>P});var n=x(l());var{useMDXComponents:i}=MdxJsReact;var P={title:\"Conference Room Connector (CRC)\"};function d(o){let e={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",p:\"p\",strong:\"strong\",ul:\"ul\",...i(),...o.components};return(0,n.jsxs)(n.Fragment,{children:[(0,n.jsx)(e.h1,{id:\"conference-room-connector-api\",children:\"Conference Room Connector API\"}),`\n`,(0,n.jsxs)(e.p,{children:[\"The Conference Room Connector API is an interface that allows developers to integrate and extend the functionality of a conference room connector service.  \",(0,n.jsx)(e.a,{href:\"http://marketplace.zoom.us/\",children:\"Zoom App Marketplace\"}),\". Learn how to get your credentials and create private or public apps in our \",(0,n.jsx)(e.a,{href:\"/docs/integrations/oauth/\",children:\"Authorization guide\"}),\".\"]}),`\n`,(0,n.jsxs)(e.p,{children:[\"The API accepts \",(0,n.jsx)(e.code,{children:\"application/json\"}),\" and \",(0,n.jsx)(e.code,{children:\"multipart/form-data\"}),\", and responds with \",(0,n.jsx)(e.code,{children:\"application/json\"}),\" and \",(0,n.jsx)(e.code,{children:\"application/xml\"}),\". All endpoints are available at \",(0,n.jsx)(e.a,{href:\"https://api.zoom.us/v2/\",children:\"https://api.zoom.us/v2/\"}),\".\"]}),`\n`,(0,n.jsx)(e.p,{children:\"To use this API, you must have:\"}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsxs)(e.li,{children:[\"You must be subscribed to the H.323/SIP Room Connector, which requires:\",`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsxs)(e.li,{children:[`Pro, Business, Education, or Enterprise account\nLogin to `,(0,n.jsx)(e.a,{href:\"http://zoom.us/signin\",children:\"http://zoom.us/signin\"}),\".\"]}),`\n`,(0,n.jsxs)(e.li,{children:[\"Go to \",(0,n.jsx)(e.a,{href:\"https://zoom.us/billing\",children:\"Billing\"}),\" and subscribe to H.323/SIP Room Connector or \",(0,n.jsx)(e.a,{href:\"https://zoom.us/contactsales\",children:\"contact sales\"}),\".\"]}),`\n`]}),`\n`]}),`\n`,(0,n.jsxs)(e.li,{children:[\"Only \",(0,n.jsx)(e.strong,{children:\"Licensed\"}),\" users can schedule meetings with Room Connector.\"]}),`\n`,(0,n.jsxs)(e.li,{children:[\"See \",(0,n.jsx)(e.a,{href:\"https://support.zoom.us/hc/en-us/articles/202445433-Room-Connector-Supported-Devices\",children:\"Room Connector Support Devices\"}),\".\"]}),`\n`]}),`\n`,(0,n.jsx)(e.h2,{id:\"more-resources\",children:(0,n.jsxs)(e.a,{href:\"#more-resources\",children:[\"More resources\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.p,{children:[\"Documentation: \",(0,n.jsx)(e.a,{href:\"/docs/api/crc/#tag/api-connector\",children:\"Endpoints\"}),\", \",(0,n.jsx)(e.a,{href:\"/changelog/crc/\",children:\"Release notes\"}),\", \",(0,n.jsx)(e.a,{href:\"https://explore.zoom.us/en/legal/zoom-api-license-and-tou/\",children:\"Terms of Service\"}),\", \",(0,n.jsx)(e.a,{href:\"https://zoom.us/pricing/zoom-rooms\",children:\"Plans\"})]}),`\n`,(0,n.jsx)(e.h2,{id:\"the-conference-room-connector-api\",children:(0,n.jsxs)(e.a,{href:\"#the-conference-room-connector-api\",children:[\"The Conference Room Connector API\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsx)(e.h3,{id:\"use-case\",children:(0,n.jsxs)(e.a,{href:\"#use-case\",children:[\"Use Case:\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsx)(e.li,{children:\"Integration with Room Systems.\"}),`\n`,(0,n.jsx)(e.li,{children:\"An H.323 or SIP device can make a video call to a Room Connector to join a Zoom cloud meeting.\"}),`\n`,(0,n.jsx)(e.li,{children:\"A Room Connector can also call out to an H.323 or SIP device to join a Zoom cloud meeting.\"}),`\n`]})]})}function p(o={}){let{wrapper:e}={...i(),...o.components};return e?(0,n.jsx)(e,{...o,children:(0,n.jsx)(d,{...o})}):d(o)}return z(A);})();\n;return Component;",
      "frontmatter": {
        "title": "Conference Room Connector (CRC)"
      },
      "errors": [],
      "matter": {
        "content": "# Conference Room Connector API\nThe Conference Room Connector API is an interface that allows developers to integrate and extend the functionality of a conference room connector service.  [Zoom App Marketplace](http://marketplace.zoom.us/). Learn how to get your credentials and create private or public apps in our [Authorization guide](/docs/integrations/oauth/).\n\nThe API accepts `application/json` and `multipart/form-data`, and responds with `application/json` and `application/xml`. All endpoints are available at https://api.zoom.us/v2/.\n\nTo use this API, you must have:\n\n* You must be subscribed to the H.323/SIP Room Connector, which requires:\n  * Pro, Business, Education, or Enterprise account\nLogin to [http://zoom.us/signin](http://zoom.us/signin).\n  * Go to [Billing](https://zoom.us/billing) and subscribe to H.323/SIP Room Connector or [contact sales](https://zoom.us/contactsales).\n* Only **Licensed** users can schedule meetings with Room Connector.\n* See [Room Connector Support Devices](https://support.zoom.us/hc/en-us/articles/202445433-Room-Connector-Supported-Devices).\n\n## More resources\n\nDocumentation: [Endpoints](/docs/api/crc/#tag/api-connector), [Release notes](/changelog/crc/), [Terms of Service](https://explore.zoom.us/en/legal/zoom-api-license-and-tou/), [Plans](https://zoom.us/pricing/zoom-rooms)\n## The Conference Room Connector API\n\n### Use Case:\n\n* Integration with Room Systems.\n* An H.323 or SIP device can make a video call to a Room Connector to join a Zoom cloud meeting.\n* A Room Connector can also call out to an H.323 or SIP device to join a Zoom cloud meeting.\n\n\n",
        "data": {
          "title": "Conference Room Connector (CRC)"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/rest/crc-api.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
