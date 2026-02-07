# Using GraphQL

- Source URL: https://developers.zoom.us/docs/api/graphql/using-graphql/
- Snapshot path: docs/api/graphql/using-graphql
- Generated (UTC): 2026-02-07T22:40:10.800036+00:00

## Frontmatter

```json
{
  "title": "Using GraphQL"
}
```

## Content

```md

# Using GraphQL

<Alert variant="warning">

**Beta**

Zoom GraphQL is in a public beta.

</Alert>

## Zoom GraphQL endpoint

GraphQL only requires one endpoint for all queries and mutations. Here is the Zoom GraphQL endpoint:

`https://api.zoom.us/v3/graphql`

## Authentication

Authenticate using OAuth. Create an [OAuth](/docs/integrations/create/) app or a [Server-to-Server OAuth](/docs/internal-apps/create/) app to get credentials and see [OAuth with Zoom](/docs/integrations/oauth/) for details.

You must have the required [scopes](/docs/api/graphql/graphql-scopes) to access the data in your query or mutation.

## Pagination

Use the `cursor` value to get the next page token, then send it to get the next page of data. See the following samples and the [GraphQL Pagination documentation](https://graphql.org/learn/pagination/) for details.

**Request**

```json
{
  meetings(userId:"me",first:10,meetingType:PREVIOUS_MEETINGS){
    edges {
    id
    }
    pageInfo{cursor}
  }
}
```

**Response**
```json
{
    "data": {
        "meetings": {
            "edges": [
                {
                    "id": 93989529046
                },
                {
                    "id": 92631809138
                },
                {
                    "id": 92849539085
                },
                {
                    "id": 94292260890
                },
                {
                    "id": 99070971365
                },
                {
                    "id": 92260053040
                },
                {
                    "id": 92260053040
                },
                {
                    "id": 92260053040
                },
                {
                    "id": 92260053040
                },
                {
                    "id": 92260053040
                }
            ],
            "pageInfo": {
                "cursor": "mSrvrBJZhdALywcIEb1LmpEMDJ6whNfoSJ2"
            }
        }
    }
}
```

The `cursor` value is the token for the next page. Enter it using the after argument in your next request.

**Request for the next page of values**

```json
{
  meetings(userId:"me",first:10,meetingType:PREVIOUS_MEETINGS,after:"DsitdziACFmWHu3nTYiGtaGEjB4P6fsve02"){
    edges {
    id
    }
    pageInfo{cursor}
  }
}
```

**Response**

```json
{
    "data": {
        "meetings": {
            "edges": [
                {
                    "id": 92260053040
                },
                {
                    "id": 92260053040
                }
            ],
            "pageInfo": {
                "cursor": ""
            }
        }
    }
}
```

If there are no more pages, the cursor value will be empty.

## Resources

For details about rate limits and scopes, see:

* [Rate limits](/docs/api/graphql/graphql-ratelimits)
* [Scopes](/docs/api/graphql/graphql-scopes)
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
      "code": "var Component=(()=>{var u=Object.create;var a=Object.defineProperty;var g=Object.getOwnPropertyDescriptor;var m=Object.getOwnPropertyNames;var f=Object.getPrototypeOf,q=Object.prototype.hasOwnProperty;var x=(r,n)=>()=>(n||r((n={exports:{}}).exports,n),n.exports),v=(r,n)=>{for(var i in n)a(r,i,{get:n[i],enumerable:!0})},s=(r,n,i,d)=>{if(n&&typeof n==\"object\"||typeof n==\"function\")for(let t of m(n))!q.call(r,t)&&t!==i&&a(r,t,{get:()=>n[t],enumerable:!(d=g(n,t))||d.enumerable});return r};var E=(r,n,i)=>(i=r!=null?u(f(r)):{},s(n||!r||!r.__esModule?a(i,\"default\",{value:r,enumerable:!0}):i,r)),I=r=>s(a({},\"__esModule\",{value:!0}),r);var c=x((A,h)=>{h.exports=_jsx_runtime});var G={};v(G,{default:()=>p,frontmatter:()=>j});var e=E(c());var{useMDXComponents:o}=MdxJsReact;var j={title:\"Using GraphQL\"};function l(r){let n={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",i:\"i\",li:\"li\",p:\"p\",pre:\"pre\",strong:\"strong\",ul:\"ul\",...o(),...r.components},{Alert:i}=n;return i||y(\"Alert\",!0),(0,e.jsxs)(e.Fragment,{children:[(0,e.jsx)(n.h1,{id:\"using-graphql\",children:\"Using GraphQL\"}),`\n`,(0,e.jsxs)(i,{variant:\"warning\",children:[(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Beta\"})}),(0,e.jsx)(n.p,{children:\"Zoom GraphQL is in a public beta.\"})]}),`\n`,(0,e.jsx)(n.h2,{id:\"zoom-graphql-endpoint\",children:(0,e.jsxs)(n.a,{href:\"#zoom-graphql-endpoint\",children:[\"Zoom GraphQL endpoint\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"GraphQL only requires one endpoint for all queries and mutations. Here is the Zoom GraphQL endpoint:\"}),`\n`,(0,e.jsx)(n.p,{children:(0,e.jsx)(n.code,{children:\"https://api.zoom.us/v3/graphql\"})}),`\n`,(0,e.jsx)(n.h2,{id:\"authentication\",children:(0,e.jsxs)(n.a,{href:\"#authentication\",children:[\"Authentication\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.p,{children:[\"Authenticate using OAuth. Create an \",(0,e.jsx)(n.a,{href:\"/docs/integrations/create/\",children:\"OAuth\"}),\" app or a \",(0,e.jsx)(n.a,{href:\"/docs/internal-apps/create/\",children:\"Server-to-Server OAuth\"}),\" app to get credentials and see \",(0,e.jsx)(n.a,{href:\"/docs/integrations/oauth/\",children:\"OAuth with Zoom\"}),\" for details.\"]}),`\n`,(0,e.jsxs)(n.p,{children:[\"You must have the required \",(0,e.jsx)(n.a,{href:\"/docs/api/graphql/graphql-scopes\",children:\"scopes\"}),\" to access the data in your query or mutation.\"]}),`\n`,(0,e.jsx)(n.h2,{id:\"pagination\",children:(0,e.jsxs)(n.a,{href:\"#pagination\",children:[\"Pagination\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.p,{children:[\"Use the \",(0,e.jsx)(n.code,{children:\"cursor\"}),\" value to get the next page token, then send it to get the next page of data. See the following samples and the \",(0,e.jsx)(n.a,{href:\"https://graphql.org/learn/pagination/\",children:\"GraphQL Pagination documentation\"}),\" for details.\"]}),`\n`,(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Request\"})}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n  meetings(userId:\"me\",first:10,meetingType:PREVIOUS_MEETINGS){\n    edges {\n    id\n    }\n    pageInfo{cursor}\n  }\n}\n`})}),`\n`,(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Response\"})}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n    \"data\": {\n        \"meetings\": {\n            \"edges\": [\n                {\n                    \"id\": 93989529046\n                },\n                {\n                    \"id\": 92631809138\n                },\n                {\n                    \"id\": 92849539085\n                },\n                {\n                    \"id\": 94292260890\n                },\n                {\n                    \"id\": 99070971365\n                },\n                {\n                    \"id\": 92260053040\n                },\n                {\n                    \"id\": 92260053040\n                },\n                {\n                    \"id\": 92260053040\n                },\n                {\n                    \"id\": 92260053040\n                },\n                {\n                    \"id\": 92260053040\n                }\n            ],\n            \"pageInfo\": {\n                \"cursor\": \"mSrvrBJZhdALywcIEb1LmpEMDJ6whNfoSJ2\"\n            }\n        }\n    }\n}\n`})}),`\n`,(0,e.jsxs)(n.p,{children:[\"The \",(0,e.jsx)(n.code,{children:\"cursor\"}),\" value is the token for the next page. Enter it using the after argument in your next request.\"]}),`\n`,(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Request for the next page of values\"})}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n  meetings(userId:\"me\",first:10,meetingType:PREVIOUS_MEETINGS,after:\"DsitdziACFmWHu3nTYiGtaGEjB4P6fsve02\"){\n    edges {\n    id\n    }\n    pageInfo{cursor}\n  }\n}\n`})}),`\n`,(0,e.jsx)(n.p,{children:(0,e.jsx)(n.strong,{children:\"Response\"})}),`\n`,(0,e.jsx)(n.pre,{children:(0,e.jsx)(n.code,{className:\"language-json\",children:`{\n    \"data\": {\n        \"meetings\": {\n            \"edges\": [\n                {\n                    \"id\": 92260053040\n                },\n                {\n                    \"id\": 92260053040\n                }\n            ],\n            \"pageInfo\": {\n                \"cursor\": \"\"\n            }\n        }\n    }\n}\n`})}),`\n`,(0,e.jsx)(n.p,{children:\"If there are no more pages, the cursor value will be empty.\"}),`\n`,(0,e.jsx)(n.h2,{id:\"resources\",children:(0,e.jsxs)(n.a,{href:\"#resources\",children:[\"Resources\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"For details about rate limits and scopes, see:\"}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsx)(n.li,{children:(0,e.jsx)(n.a,{href:\"/docs/api/graphql/graphql-ratelimits\",children:\"Rate limits\"})}),`\n`,(0,e.jsx)(n.li,{children:(0,e.jsx)(n.a,{href:\"/docs/api/graphql/graphql-scopes\",children:\"Scopes\"})}),`\n`]})]})}function p(r={}){let{wrapper:n}={...o(),...r.components};return n?(0,e.jsx)(n,{...r,children:(0,e.jsx)(l,{...r})}):l(r)}function y(r,n){throw new Error(\"Expected \"+(n?\"component\":\"object\")+\" `\"+r+\"` to be defined: you likely forgot to import, pass, or provide it.\")}return I(G);})();\n;return Component;",
      "frontmatter": {
        "title": "Using GraphQL"
      },
      "errors": [],
      "matter": {
        "content": "\n# Using GraphQL\n\n<Alert variant=\"warning\">\n\n**Beta**\n\nZoom GraphQL is in a public beta.\n\n</Alert>\n\n## Zoom GraphQL endpoint\n\nGraphQL only requires one endpoint for all queries and mutations. Here is the Zoom GraphQL endpoint:\n\n`https://api.zoom.us/v3/graphql`\n\n## Authentication\n\nAuthenticate using OAuth. Create an [OAuth](/docs/integrations/create/) app or a [Server-to-Server OAuth](/docs/internal-apps/create/) app to get credentials and see [OAuth with Zoom](/docs/integrations/oauth/) for details.\n\nYou must have the required [scopes](/docs/api/graphql/graphql-scopes) to access the data in your query or mutation.\n\n## Pagination\n\nUse the `cursor` value to get the next page token, then send it to get the next page of data. See the following samples and the [GraphQL Pagination documentation](https://graphql.org/learn/pagination/) for details.\n\n**Request**\n\n```json\n{\n  meetings(userId:\"me\",first:10,meetingType:PREVIOUS_MEETINGS){\n    edges {\n    id\n    }\n    pageInfo{cursor}\n  }\n}\n```\n\n**Response**\n```json\n{\n    \"data\": {\n        \"meetings\": {\n            \"edges\": [\n                {\n                    \"id\": 93989529046\n                },\n                {\n                    \"id\": 92631809138\n                },\n                {\n                    \"id\": 92849539085\n                },\n                {\n                    \"id\": 94292260890\n                },\n                {\n                    \"id\": 99070971365\n                },\n                {\n                    \"id\": 92260053040\n                },\n                {\n                    \"id\": 92260053040\n                },\n                {\n                    \"id\": 92260053040\n                },\n                {\n                    \"id\": 92260053040\n                },\n                {\n                    \"id\": 92260053040\n                }\n            ],\n            \"pageInfo\": {\n                \"cursor\": \"mSrvrBJZhdALywcIEb1LmpEMDJ6whNfoSJ2\"\n            }\n        }\n    }\n}\n```\n\nThe `cursor` value is the token for the next page. Enter it using the after argument in your next request.\n\n**Request for the next page of values**\n\n```json\n{\n  meetings(userId:\"me\",first:10,meetingType:PREVIOUS_MEETINGS,after:\"DsitdziACFmWHu3nTYiGtaGEjB4P6fsve02\"){\n    edges {\n    id\n    }\n    pageInfo{cursor}\n  }\n}\n```\n\n**Response**\n\n```json\n{\n    \"data\": {\n        \"meetings\": {\n            \"edges\": [\n                {\n                    \"id\": 92260053040\n                },\n                {\n                    \"id\": 92260053040\n                }\n            ],\n            \"pageInfo\": {\n                \"cursor\": \"\"\n            }\n        }\n    }\n}\n```\n\nIf there are no more pages, the cursor value will be empty.\n\n## Resources\n\nFor details about rate limits and scopes, see:\n\n* [Rate limits](/docs/api/graphql/graphql-ratelimits)\n* [Scopes](/docs/api/graphql/graphql-scopes)",
        "data": {
          "title": "Using GraphQL"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/graphql/using-graphql.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
