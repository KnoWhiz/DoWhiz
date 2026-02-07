# Zoom Webinars Plus & Events API

- Source URL: https://developers.zoom.us/docs/api/rest/zoom-events-api/
- Snapshot path: docs/api/rest/zoom-events-api
- Generated (UTC): 2026-02-07T22:40:11.533231+00:00

## Frontmatter

```json
{
  "title": "Zoom Webinars Plus & Events API"
}
```

## Content

```md

# Zoom Webinars Plus & Events API

The Zoom Webinars Plus & Events API lets developers access information from Zoom. Use this API to build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us/). Learn how to get your credentials and create private or public apps in our [Authorization guide](/docs/integrations/oauth/).

The API accepts `application/json` and `multipart/form-data`, and responds with `application/json` and `application/xml`. All endpoints are available at https://api.zoom.us/v2/.

To use this API, you must have
* Pro, Business, Enterprise, or Education account.
* Zoom Session license, Events Unlimited license or Zoom Events Pay Per Attendee license.
* Stripe or PayPal Business account is required to create paid events.
* Zoom desktop client for Windows, macOS, or Linux: Global minimum version or higher.
* Zoom mobile app for Android or iOS: Global minimum version or higher.

**Permissions:** The Webinars Plus & Events API Management permission grants host privileges to all hubs via the API. The Account Owner and Admin have this permission by default. You can also choose to grant this permission to other members.

**Note**: For access to the latest Zoom Events and Webinar features, we strongly recommend hosts to update to the latest version of the Zoom desktop client/mobile application.

Some webhooks may require a higher account level.

## More resources
Documentation: [Endpoints](/docs/api/events/), [Webhooks](/docs/api/events/events), [Release notes](/changelog/events/), [Postman hub](https://www.postman.com/zoom-developer/workspace/zoom-public-workspace/collection/22097587-16eaf86b-15d2-4ee3-add8-096af38b3d9b), [Terms of Service](https://explore.zoom.us/en/legal/zoom-api-license-and-tou/), [Plans](https://zoom.us/pricing/events).

## Make your first API calls

The Webinars Plus & Events API offers valuable functionality for events management.

### Use Case:

* Simplify events registration and attendance tracking processes.
* Enhance events engagement through personalized experiences.

### Endpoints:

* `GET/v2/zoom_events/events/{eventId}/registrants` endpoint, which allows you to access registrant information, like names and email addresses.

This API call enables you to effectively manage event attendance effectiveness.  You can find more information and interactively test these calls in [Postman](https://www.postman.com/zoom-developer/workspace/zoom-public-workspace/collection/22097587-16eaf86b-15d2-4ee3-add8-096af38b3d9b).
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
      "code": "var Component=(()=>{var m=Object.create;var t=Object.defineProperty;var u=Object.getOwnPropertyDescriptor;var f=Object.getOwnPropertyNames;var v=Object.getPrototypeOf,b=Object.prototype.hasOwnProperty;var g=(o,e)=>()=>(e||o((e={exports:{}}).exports,e),e.exports),P=(o,e)=>{for(var i in e)t(o,i,{get:e[i],enumerable:!0})},l=(o,e,i,a)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let r of f(e))!b.call(o,r)&&r!==i&&t(o,r,{get:()=>e[r],enumerable:!(a=u(e,r))||a.enumerable});return o};var w=(o,e,i)=>(i=o!=null?m(v(o)):{},l(e||!o||!o.__esModule?t(i,\"default\",{value:o,enumerable:!0}):i,o)),A=o=>l(t({},\"__esModule\",{value:!0}),o);var d=g((z,c)=>{c.exports=_jsx_runtime});var k={};P(k,{default:()=>p,frontmatter:()=>y});var n=w(d());var{useMDXComponents:s}=MdxJsReact;var y={title:\"Zoom Webinars Plus & Events API\"};function h(o){let e={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",p:\"p\",strong:\"strong\",ul:\"ul\",...s(),...o.components};return(0,n.jsxs)(n.Fragment,{children:[(0,n.jsx)(e.h1,{id:\"zoom-webinars-plus--events-api\",children:\"Zoom Webinars Plus & Events API\"}),`\n`,(0,n.jsxs)(e.p,{children:[\"The Zoom Webinars Plus & Events API lets developers access information from Zoom. Use this API to build private services or public applications on the \",(0,n.jsx)(e.a,{href:\"http://marketplace.zoom.us/\",children:\"Zoom App Marketplace\"}),\". Learn how to get your credentials and create private or public apps in our \",(0,n.jsx)(e.a,{href:\"/docs/integrations/oauth/\",children:\"Authorization guide\"}),\".\"]}),`\n`,(0,n.jsxs)(e.p,{children:[\"The API accepts \",(0,n.jsx)(e.code,{children:\"application/json\"}),\" and \",(0,n.jsx)(e.code,{children:\"multipart/form-data\"}),\", and responds with \",(0,n.jsx)(e.code,{children:\"application/json\"}),\" and \",(0,n.jsx)(e.code,{children:\"application/xml\"}),\". All endpoints are available at \",(0,n.jsx)(e.a,{href:\"https://api.zoom.us/v2/\",children:\"https://api.zoom.us/v2/\"}),\".\"]}),`\n`,(0,n.jsx)(e.p,{children:\"To use this API, you must have\"}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsx)(e.li,{children:\"Pro, Business, Enterprise, or Education account.\"}),`\n`,(0,n.jsx)(e.li,{children:\"Zoom Session license, Events Unlimited license or Zoom Events Pay Per Attendee license.\"}),`\n`,(0,n.jsx)(e.li,{children:\"Stripe or PayPal Business account is required to create paid events.\"}),`\n`,(0,n.jsx)(e.li,{children:\"Zoom desktop client for Windows, macOS, or Linux: Global minimum version or higher.\"}),`\n`,(0,n.jsx)(e.li,{children:\"Zoom mobile app for Android or iOS: Global minimum version or higher.\"}),`\n`]}),`\n`,(0,n.jsxs)(e.p,{children:[(0,n.jsx)(e.strong,{children:\"Permissions:\"}),\" The Webinars Plus & Events API Management permission grants host privileges to all hubs via the API. The Account Owner and Admin have this permission by default. You can also choose to grant this permission to other members.\"]}),`\n`,(0,n.jsxs)(e.p,{children:[(0,n.jsx)(e.strong,{children:\"Note\"}),\": For access to the latest Zoom Events and Webinar features, we strongly recommend hosts to update to the latest version of the Zoom desktop client/mobile application.\"]}),`\n`,(0,n.jsx)(e.p,{children:\"Some webhooks may require a higher account level.\"}),`\n`,(0,n.jsx)(e.h2,{id:\"more-resources\",children:(0,n.jsxs)(e.a,{href:\"#more-resources\",children:[\"More resources\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.p,{children:[\"Documentation: \",(0,n.jsx)(e.a,{href:\"/docs/api/events/\",children:\"Endpoints\"}),\", \",(0,n.jsx)(e.a,{href:\"/docs/api/events/events\",children:\"Webhooks\"}),\", \",(0,n.jsx)(e.a,{href:\"/changelog/events/\",children:\"Release notes\"}),\", \",(0,n.jsx)(e.a,{href:\"https://www.postman.com/zoom-developer/workspace/zoom-public-workspace/collection/22097587-16eaf86b-15d2-4ee3-add8-096af38b3d9b\",children:\"Postman hub\"}),\", \",(0,n.jsx)(e.a,{href:\"https://explore.zoom.us/en/legal/zoom-api-license-and-tou/\",children:\"Terms of Service\"}),\", \",(0,n.jsx)(e.a,{href:\"https://zoom.us/pricing/events\",children:\"Plans\"}),\".\"]}),`\n`,(0,n.jsx)(e.h2,{id:\"make-your-first-api-calls\",children:(0,n.jsxs)(e.a,{href:\"#make-your-first-api-calls\",children:[\"Make your first API calls\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsx)(e.p,{children:\"The Webinars Plus & Events API offers valuable functionality for events management.\"}),`\n`,(0,n.jsx)(e.h3,{id:\"use-case\",children:(0,n.jsxs)(e.a,{href:\"#use-case\",children:[\"Use Case:\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsx)(e.li,{children:\"Simplify events registration and attendance tracking processes.\"}),`\n`,(0,n.jsx)(e.li,{children:\"Enhance events engagement through personalized experiences.\"}),`\n`]}),`\n`,(0,n.jsx)(e.h3,{id:\"endpoints\",children:(0,n.jsxs)(e.a,{href:\"#endpoints\",children:[\"Endpoints:\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.code,{children:\"GET/v2/zoom_events/events/{eventId}/registrants\"}),\" endpoint, which allows you to access registrant information, like names and email addresses.\"]}),`\n`]}),`\n`,(0,n.jsxs)(e.p,{children:[\"This API call enables you to effectively manage event attendance effectiveness.  You can find more information and interactively test these calls in \",(0,n.jsx)(e.a,{href:\"https://www.postman.com/zoom-developer/workspace/zoom-public-workspace/collection/22097587-16eaf86b-15d2-4ee3-add8-096af38b3d9b\",children:\"Postman\"}),\".\"]})]})}function p(o={}){let{wrapper:e}={...s(),...o.components};return e?(0,n.jsx)(e,{...o,children:(0,n.jsx)(h,{...o})}):h(o)}return A(k);})();\n;return Component;",
      "frontmatter": {
        "title": "Zoom Webinars Plus & Events API"
      },
      "errors": [],
      "matter": {
        "content": "\n# Zoom Webinars Plus & Events API\n\nThe Zoom Webinars Plus & Events API lets developers access information from Zoom. Use this API to build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us/). Learn how to get your credentials and create private or public apps in our [Authorization guide](/docs/integrations/oauth/).\n\nThe API accepts `application/json` and `multipart/form-data`, and responds with `application/json` and `application/xml`. All endpoints are available at https://api.zoom.us/v2/.\n\nTo use this API, you must have\n* Pro, Business, Enterprise, or Education account.\n* Zoom Session license, Events Unlimited license or Zoom Events Pay Per Attendee license.\n* Stripe or PayPal Business account is required to create paid events.\n* Zoom desktop client for Windows, macOS, or Linux: Global minimum version or higher.\n* Zoom mobile app for Android or iOS: Global minimum version or higher.\n\n**Permissions:** The Webinars Plus & Events API Management permission grants host privileges to all hubs via the API. The Account Owner and Admin have this permission by default. You can also choose to grant this permission to other members.\n\n**Note**: For access to the latest Zoom Events and Webinar features, we strongly recommend hosts to update to the latest version of the Zoom desktop client/mobile application.\n\nSome webhooks may require a higher account level.\n\n## More resources\nDocumentation: [Endpoints](/docs/api/events/), [Webhooks](/docs/api/events/events), [Release notes](/changelog/events/), [Postman hub](https://www.postman.com/zoom-developer/workspace/zoom-public-workspace/collection/22097587-16eaf86b-15d2-4ee3-add8-096af38b3d9b), [Terms of Service](https://explore.zoom.us/en/legal/zoom-api-license-and-tou/), [Plans](https://zoom.us/pricing/events).\n\n## Make your first API calls\n\nThe Webinars Plus & Events API offers valuable functionality for events management.\n\n### Use Case:\n\n* Simplify events registration and attendance tracking processes.\n* Enhance events engagement through personalized experiences.\n\n### Endpoints:\n\n* `GET/v2/zoom_events/events/{eventId}/registrants` endpoint, which allows you to access registrant information, like names and email addresses.\n\nThis API call enables you to effectively manage event attendance effectiveness.  You can find more information and interactively test these calls in [Postman](https://www.postman.com/zoom-developer/workspace/zoom-public-workspace/collection/22097587-16eaf86b-15d2-4ee3-add8-096af38b3d9b).\n",
        "data": {
          "title": "Zoom Webinars Plus & Events API"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/rest/zoom-events-api.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
