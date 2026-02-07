# SCIM2 API

- Source URL: https://developers.zoom.us/docs/api/rest/scim-api/
- Snapshot path: docs/api/rest/scim-api
- Generated (UTC): 2026-02-07T22:40:11.529349+00:00

## Frontmatter

```json
{
  "title": "SCIM2 API"
}
```

## Content

```md

# SCIM2 API

- **Version:** 1.0
- **Host:** `api.zoom.us`
- **Protocols:** `https`
- **Accepts:** `application/json`
- **Responds With:** `application/json`

More Info

- **[Contact Us by Email](mailto:developersupport@zoom.us)**
- **Contact URL:** &lt;https://developer.zoom.us>
- **Terms Of Service**: &lt;https://zoom.us/terms>
- **License:** [MIT](https://opensource.org/licenses/MIT)

SCIM2 (System for Cross-domain Identity Management) is a specification designed to automate the provisioning of user and group identities across various cloud-based applications and services using SSO services and Identity Providers. The Zoom SCIM2 API provides support for user and group provisioning through the **User** and **Group** resources. See [rate limits](/docs/api/rate-limits/) for SCIM2 API endpoint rate limits.

SCIM uses HTTP methods compatible with the REST architecture style. The payloads require schema definitions, attributes, and values for requests. With the SCIM API, you can use additional features such as Attribute Filters and Search criteria. 

While using the SCIM APIs, set the value of the **Accept** header as “application/scim+json”.

**Prerequisites**

- Zoom Owner or Admin privileges
- Business or Education account with approved [Vanity URL](https://support.zoom.us/hc/en-us/articles/215062646-Guidelines-for-Vanity-URL-Requests)
- [Single Sign-On](https://support.zoom.us/hc/en-us/articles/201363003-Getting-Started-with-SSO) enabled

## Authentication

The SCIM2 API supports OAuth authentication. To use Oauth 2.0, create an [OAuth app](/docs/integrations/create/) or a [server-to-server OAuth app](/docs/internal-apps/create/) on the Zoom App Marketplace and select `SCIM2` as the scope for your app. Next, [generate an OAuth token](/docs/integrations/oauth/) for your app and start making SCIM2 API calls.
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
      "code": "var Component=(()=>{var u=Object.create;var o=Object.defineProperty;var m=Object.getOwnPropertyDescriptor;var g=Object.getOwnPropertyNames;var f=Object.getPrototypeOf,I=Object.prototype.hasOwnProperty;var S=(t,e)=>()=>(e||t((e={exports:{}}).exports,e),e.exports),M=(t,e)=>{for(var r in e)o(t,r,{get:e[r],enumerable:!0})},a=(t,e,r,c)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let i of g(e))!I.call(t,i)&&i!==r&&o(t,i,{get:()=>e[i],enumerable:!(c=m(e,i))||c.enumerable});return t};var C=(t,e,r)=>(r=t!=null?u(f(t)):{},a(e||!t||!t.__esModule?o(r,\"default\",{value:t,enumerable:!0}):r,t)),v=t=>a(o({},\"__esModule\",{value:!0}),t);var d=S((P,h)=>{h.exports=_jsx_runtime});var x={};M(x,{default:()=>p,frontmatter:()=>A});var n=C(d());var{useMDXComponents:s}=MdxJsReact;var A={title:\"SCIM2 API\"};function l(t){let e={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",i:\"i\",li:\"li\",p:\"p\",strong:\"strong\",ul:\"ul\",...s(),...t.components};return(0,n.jsxs)(n.Fragment,{children:[(0,n.jsx)(e.h1,{id:\"scim2-api\",children:\"SCIM2 API\"}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Version:\"}),\" 1.0\"]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Host:\"}),\" \",(0,n.jsx)(e.code,{children:\"api.zoom.us\"})]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Protocols:\"}),\" \",(0,n.jsx)(e.code,{children:\"https\"})]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Accepts:\"}),\" \",(0,n.jsx)(e.code,{children:\"application/json\"})]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Responds With:\"}),\" \",(0,n.jsx)(e.code,{children:\"application/json\"})]}),`\n`]}),`\n`,(0,n.jsx)(e.p,{children:\"More Info\"}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsx)(e.li,{children:(0,n.jsx)(e.strong,{children:(0,n.jsx)(e.a,{href:\"mailto:developersupport@zoom.us\",children:\"Contact Us by Email\"})})}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Contact URL:\"}),\" <\",(0,n.jsx)(e.a,{href:\"https://developer.zoom.us%3E\",children:\"https://developer.zoom.us>\"})]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"Terms Of Service\"}),\": <\",(0,n.jsx)(e.a,{href:\"https://zoom.us/terms%3E\",children:\"https://zoom.us/terms>\"})]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.strong,{children:\"License:\"}),\" \",(0,n.jsx)(e.a,{href:\"https://opensource.org/licenses/MIT\",children:\"MIT\"})]}),`\n`]}),`\n`,(0,n.jsxs)(e.p,{children:[\"SCIM2 (System for Cross-domain Identity Management) is a specification designed to automate the provisioning of user and group identities across various cloud-based applications and services using SSO services and Identity Providers. The Zoom SCIM2 API provides support for user and group provisioning through the \",(0,n.jsx)(e.strong,{children:\"User\"}),\" and \",(0,n.jsx)(e.strong,{children:\"Group\"}),\" resources. See \",(0,n.jsx)(e.a,{href:\"/docs/api/rate-limits/\",children:\"rate limits\"}),\" for SCIM2 API endpoint rate limits.\"]}),`\n`,(0,n.jsx)(e.p,{children:\"SCIM uses HTTP methods compatible with the REST architecture style. The payloads require schema definitions, attributes, and values for requests. With the SCIM API, you can use additional features such as Attribute Filters and Search criteria.\"}),`\n`,(0,n.jsxs)(e.p,{children:[\"While using the SCIM APIs, set the value of the \",(0,n.jsx)(e.strong,{children:\"Accept\"}),\" header as \\u201Capplication/scim+json\\u201D.\"]}),`\n`,(0,n.jsx)(e.p,{children:(0,n.jsx)(e.strong,{children:\"Prerequisites\"})}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsx)(e.li,{children:\"Zoom Owner or Admin privileges\"}),`\n`,(0,n.jsxs)(e.li,{children:[\"Business or Education account with approved \",(0,n.jsx)(e.a,{href:\"https://support.zoom.us/hc/en-us/articles/215062646-Guidelines-for-Vanity-URL-Requests\",children:\"Vanity URL\"})]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.a,{href:\"https://support.zoom.us/hc/en-us/articles/201363003-Getting-Started-with-SSO\",children:\"Single Sign-On\"}),\" enabled\"]}),`\n`]}),`\n`,(0,n.jsx)(e.h2,{id:\"authentication\",children:(0,n.jsxs)(e.a,{href:\"#authentication\",children:[\"Authentication\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.p,{children:[\"The SCIM2 API supports OAuth authentication. To use Oauth 2.0, create an \",(0,n.jsx)(e.a,{href:\"/docs/integrations/create/\",children:\"OAuth app\"}),\" or a \",(0,n.jsx)(e.a,{href:\"/docs/internal-apps/create/\",children:\"server-to-server OAuth app\"}),\" on the Zoom App Marketplace and select \",(0,n.jsx)(e.code,{children:\"SCIM2\"}),\" as the scope for your app. Next, \",(0,n.jsx)(e.a,{href:\"/docs/integrations/oauth/\",children:\"generate an OAuth token\"}),\" for your app and start making SCIM2 API calls.\"]})]})}function p(t={}){let{wrapper:e}={...s(),...t.components};return e?(0,n.jsx)(e,{...t,children:(0,n.jsx)(l,{...t})}):l(t)}return v(x);})();\n;return Component;",
      "frontmatter": {
        "title": "SCIM2 API"
      },
      "errors": [],
      "matter": {
        "content": "\n# SCIM2 API\n\n- **Version:** 1.0\n- **Host:** `api.zoom.us`\n- **Protocols:** `https`\n- **Accepts:** `application/json`\n- **Responds With:** `application/json`\n\nMore Info\n\n- **[Contact Us by Email](mailto:developersupport@zoom.us)**\n- **Contact URL:** &lt;https://developer.zoom.us>\n- **Terms Of Service**: &lt;https://zoom.us/terms>\n- **License:** [MIT](https://opensource.org/licenses/MIT)\n\nSCIM2 (System for Cross-domain Identity Management) is a specification designed to automate the provisioning of user and group identities across various cloud-based applications and services using SSO services and Identity Providers. The Zoom SCIM2 API provides support for user and group provisioning through the **User** and **Group** resources. See [rate limits](/docs/api/rate-limits/) for SCIM2 API endpoint rate limits.\n\nSCIM uses HTTP methods compatible with the REST architecture style. The payloads require schema definitions, attributes, and values for requests. With the SCIM API, you can use additional features such as Attribute Filters and Search criteria. \n\nWhile using the SCIM APIs, set the value of the **Accept** header as \u201capplication/scim+json\u201d.\n\n**Prerequisites**\n\n- Zoom Owner or Admin privileges\n- Business or Education account with approved [Vanity URL](https://support.zoom.us/hc/en-us/articles/215062646-Guidelines-for-Vanity-URL-Requests)\n- [Single Sign-On](https://support.zoom.us/hc/en-us/articles/201363003-Getting-Started-with-SSO) enabled\n\n## Authentication\n\nThe SCIM2 API supports OAuth authentication. To use Oauth 2.0, create an [OAuth app](/docs/integrations/create/) or a [server-to-server OAuth app](/docs/internal-apps/create/) on the Zoom App Marketplace and select `SCIM2` as the scope for your app. Next, [generate an OAuth token](/docs/integrations/oauth/) for your app and start making SCIM2 API calls.\n",
        "data": {
          "title": "SCIM2 API"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/rest/scim-api.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
