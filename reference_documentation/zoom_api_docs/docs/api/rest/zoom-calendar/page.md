# Zoom Calendar API

- Source URL: https://developers.zoom.us/docs/api/rest/zoom-calendar/
- Snapshot path: docs/api/rest/zoom-calendar
- Generated (UTC): 2026-02-07T22:40:11.531372+00:00

## Frontmatter

```json
{
  "title": "Zoom Calendar API"
}
```

## Content

```md

# Zoom Calendar API

You can use Zoom Calendar APIs to integrate Zoom Calendar into third-party applications or services, and build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us). 

### Before you begin
- Learn the [basics](/docs/api/using-zoom-apis/) on how to build with Zoom APIs.

- Use our [Postman collection](https://www.postman.com/zoom-developer) to try [Zoom Calendar APIs](/docs/api/calendar). 

- Learn about [webhooks](/docs/api/webhooks/) and how to get data delivered to your designated URL.


## Use cases
Zoom Calendar APIs empower developers enables you to build custom scheduling solutions for specific industries, such as healthcare, education, and event management. 
- Healthcare providers can use the Zoom Calendar API to integrate medical appointments into patient management systems. 
- Educators can integrate Zoom scheduling features to manage virtual classes, office hours, and tutoring sessions. 
- Event managers can use the Zoom Calendar API to provide virtual event scheduling and management features. 

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
      "code": "var Component=(()=>{var u=Object.create;var a=Object.defineProperty;var m=Object.getOwnPropertyDescriptor;var g=Object.getOwnPropertyNames;var f=Object.getPrototypeOf,b=Object.prototype.hasOwnProperty;var v=(o,e)=>()=>(e||o((e={exports:{}}).exports,e),e.exports),x=(o,e)=>{for(var r in e)a(o,r,{get:e[r],enumerable:!0})},c=(o,e,r,d)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let i of g(e))!b.call(o,i)&&i!==r&&a(o,i,{get:()=>e[i],enumerable:!(d=m(e,i))||d.enumerable});return o};var A=(o,e,r)=>(r=o!=null?u(f(o)):{},c(e||!o||!o.__esModule?a(r,\"default\",{value:o,enumerable:!0}):r,o)),C=o=>c(a({},\"__esModule\",{value:!0}),o);var l=v((y,s)=>{s.exports=_jsx_runtime});var w={};x(w,{default:()=>p,frontmatter:()=>Z});var n=A(l());var{useMDXComponents:t}=MdxJsReact;var Z={title:\"Zoom Calendar API\"};function h(o){let e={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",p:\"p\",strong:\"strong\",ul:\"ul\",...t(),...o.components};return(0,n.jsxs)(n.Fragment,{children:[(0,n.jsx)(e.h1,{id:\"zoom-calendar-api\",children:\"Zoom Calendar API\"}),`\n`,(0,n.jsxs)(e.p,{children:[\"You can use Zoom Calendar APIs to integrate Zoom Calendar into third-party applications or services, and build private services or public applications on the \",(0,n.jsx)(e.a,{href:\"http://marketplace.zoom.us\",children:\"Zoom App Marketplace\"}),\".\"]}),`\n`,(0,n.jsx)(e.h3,{id:\"before-you-begin\",children:(0,n.jsxs)(e.a,{href:\"#before-you-begin\",children:[\"Before you begin\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsxs)(e.li,{children:[`\n`,(0,n.jsxs)(e.p,{children:[\"Learn the \",(0,n.jsx)(e.a,{href:\"/docs/api/using-zoom-apis/\",children:\"basics\"}),\" on how to build with Zoom APIs.\"]}),`\n`]}),`\n`,(0,n.jsxs)(e.li,{children:[`\n`,(0,n.jsxs)(e.p,{children:[\"Use our \",(0,n.jsx)(e.a,{href:\"https://www.postman.com/zoom-developer\",children:\"Postman collection\"}),\" to try \",(0,n.jsx)(e.a,{href:\"/docs/api/calendar\",children:\"Zoom Calendar APIs\"}),\".\"]}),`\n`]}),`\n`,(0,n.jsxs)(e.li,{children:[`\n`,(0,n.jsxs)(e.p,{children:[\"Learn about \",(0,n.jsx)(e.a,{href:\"/docs/api/webhooks/\",children:\"webhooks\"}),\" and how to get data delivered to your designated URL.\"]}),`\n`]}),`\n`]}),`\n`,(0,n.jsx)(e.h2,{id:\"use-cases\",children:(0,n.jsxs)(e.a,{href:\"#use-cases\",children:[\"Use cases\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsx)(e.p,{children:\"Zoom Calendar APIs empower developers enables you to build custom scheduling solutions for specific industries, such as healthcare, education, and event management.\"}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsx)(e.li,{children:\"Healthcare providers can use the Zoom Calendar API to integrate medical appointments into patient management systems.\"}),`\n`,(0,n.jsx)(e.li,{children:\"Educators can integrate Zoom scheduling features to manage virtual classes, office hours, and tutoring sessions.\"}),`\n`,(0,n.jsx)(e.li,{children:\"Event managers can use the Zoom Calendar API to provide virtual event scheduling and management features.\"}),`\n`]}),`\n`,(0,n.jsx)(e.h2,{id:\"endpoints\",children:(0,n.jsxs)(e.a,{href:\"#endpoints\",children:[\"Endpoints\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.p,{children:[\"All endpoints are available through \",(0,n.jsx)(e.code,{children:\"https\"}),\" at \",(0,n.jsx)(e.code,{children:\"api.zoom.us/v2/\"}),\".\"]}),`\n`,(0,n.jsx)(e.h2,{id:\"resources-and-information\",children:(0,n.jsxs)(e.a,{href:\"#resources-and-information\",children:[\"Resources and information\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.a,{href:\"/docs/integrations/oauth/\",children:(0,n.jsx)(e.strong,{children:\"OAuth 2.0 for Zoom\"})}),\" \",(0,n.jsx)(\"br\",{}),`\nHow to get your credentials and create private or public applications`]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Version\"}),\" \",(0,n.jsx)(\"br\",{}),`\n2.0.0`]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Host\"}),\" \",(0,n.jsx)(\"br\",{}),`\n`,(0,n.jsx)(e.code,{children:\"api.zoom.us/v2\"})]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Protocols\"}),\" \",(0,n.jsx)(\"br\",{}),`\n`,(0,n.jsx)(e.code,{children:\"https\"})]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Accepts\"}),\" \",(0,n.jsx)(\"br\",{}),`\n`,(0,n.jsx)(e.code,{children:\"application/json\"}),\", \",(0,n.jsx)(e.code,{children:\"multipart/form-data\"})]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Responds With\"}),(0,n.jsx)(\"br\",{}),`\n`,(0,n.jsx)(e.code,{children:\"application/json\"}),\", \",(0,n.jsx)(e.code,{children:\"application/xml\"})]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Zoom API License and Terms of Use\"}),\" \",(0,n.jsx)(\"br\",{}),`\n`,(0,n.jsx)(e.code,{children:\"https://explore.zoom.us/en/legal/zoom-api-license-and-tou/\"})]}),`\n`]})]})}function p(o={}){let{wrapper:e}={...t(),...o.components};return e?(0,n.jsx)(e,{...o,children:(0,n.jsx)(h,{...o})}):h(o)}return C(w);})();\n;return Component;",
      "frontmatter": {
        "title": "Zoom Calendar API"
      },
      "errors": [],
      "matter": {
        "content": "\n# Zoom Calendar API\n\nYou can use Zoom Calendar APIs to integrate Zoom Calendar into third-party applications or services, and build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us). \n\n### Before you begin\n- Learn the [basics](/docs/api/using-zoom-apis/) on how to build with Zoom APIs.\n\n- Use our [Postman collection](https://www.postman.com/zoom-developer) to try [Zoom Calendar APIs](/docs/api/calendar). \n\n- Learn about [webhooks](/docs/api/webhooks/) and how to get data delivered to your designated URL.\n\n\n## Use cases\nZoom Calendar APIs empower developers enables you to build custom scheduling solutions for specific industries, such as healthcare, education, and event management. \n- Healthcare providers can use the Zoom Calendar API to integrate medical appointments into patient management systems. \n- Educators can integrate Zoom scheduling features to manage virtual classes, office hours, and tutoring sessions. \n- Event managers can use the Zoom Calendar API to provide virtual event scheduling and management features. \n\n## Endpoints\nAll endpoints are available through `https` at `api.zoom.us/v2/`. \n\n## Resources and information\n- [**OAuth 2.0 for Zoom**](/docs/integrations/oauth/) <br/>\nHow to get your credentials and create private or public applications\n- **Version** <br/>\n  2.0.0\n- **Host** <br/>\n `api.zoom.us/v2`\n- **Protocols** <br/>\n  `https`\n- **Accepts** <br/>\n `application/json`, `multipart/form-data`\n- **Responds With**<br/>\n  `application/json`, `application/xml`\n- **Zoom API License and Terms of Use** <br/>\n `https://explore.zoom.us/en/legal/zoom-api-license-and-tou/`",
        "data": {
          "title": "Zoom Calendar API"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/rest/zoom-calendar.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
