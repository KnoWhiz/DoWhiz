# Error Codes

- Source URL: https://developers.zoom.us/docs/api/errors/
- Snapshot path: docs/api/errors
- Generated (UTC): 2026-02-07T22:40:10.756453+00:00

## Frontmatter

```json
{
  "title": "Error Codes"
}
```

## Content

```md

# Error Codes

The Zoom API uses HTTP Status codes to reflect a successful or unsuccesful request. 2XX status codes represent a successful request, 4XX/5XX status codes represent an error took place. If you receive an error status code, check the body for an error code and message.

| Status Code | Description          | Most Likely Cause                                                                                                                          |
| ----------- | -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| 2XX         | Successful Request   |                                                                                                                                            |
| 400         | Bad Request          | Invalid/missing data                                                                                                                       |
| 401         | Unauthorized         | Invalid/missing credentials                                                                                                                |
| 403         | Forbidden            | User does not have permission or has not authorized [shared access permissions](/docs/api/using-zoom-apis#shared-access-permissions). |
| 404         | Not Found            | The resource doesn't exist; invalid or non-existent user ID, for example                                                                   |
| 409         | Conflict             | Trying to overwrite a resource, for example when creating a user with an email that already exists                                         |
| 429         | Too Many Requests    | Hit an API rate limit                                                                                                                      |
| 4700        | Invalid Access Token | Invalid access token, does not contain scopes. The user is not authorized                                                                  |

### Error response example

```json
{
  "code": 300,
  "message": "Request Body should be a valid JSON object."
}
```

### Error response when sending invalid fields

```json
{
  "code": 300,
  "message": "Validation Failed.",
  "errors": [
    {
      "field": "user_info.email",
      "message": "Invalid field."
    },
    {
      "field": "user_info.type",
      "message": "Invalid field."
    }
  ]
}
```
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
      "code": "var Component=(()=>{var p=Object.create;var s=Object.defineProperty;var m=Object.getOwnPropertyDescriptor;var f=Object.getOwnPropertyNames;var g=Object.getPrototypeOf,x=Object.prototype.hasOwnProperty;var v=(n,e)=>()=>(e||n((e={exports:{}}).exports,e),e.exports),y=(n,e)=>{for(var d in e)s(n,d,{get:e[d],enumerable:!0})},c=(n,e,d,o)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let t of f(e))!x.call(n,t)&&t!==d&&s(n,t,{get:()=>e[t],enumerable:!(o=m(e,t))||o.enumerable});return n};var X=(n,e,d)=>(d=n!=null?p(g(n)):{},c(e||!n||!n.__esModule?s(d,\"default\",{value:n,enumerable:!0}):d,n)),b=n=>c(s({},\"__esModule\",{value:!0}),n);var l=v((j,a)=>{a.exports=_jsx_runtime});var I={};y(I,{default:()=>u,frontmatter:()=>C});var r=X(l());var{useMDXComponents:i}=MdxJsReact;var C={title:\"Error Codes\"};function h(n){let e={a:\"a\",code:\"code\",h1:\"h1\",h3:\"h3\",i:\"i\",p:\"p\",pre:\"pre\",table:\"table\",tbody:\"tbody\",td:\"td\",th:\"th\",thead:\"thead\",tr:\"tr\",...i(),...n.components};return(0,r.jsxs)(r.Fragment,{children:[(0,r.jsx)(e.h1,{id:\"error-codes\",children:\"Error Codes\"}),`\n`,(0,r.jsx)(e.p,{children:\"The Zoom API uses HTTP Status codes to reflect a successful or unsuccesful request. 2XX status codes represent a successful request, 4XX/5XX status codes represent an error took place. If you receive an error status code, check the body for an error code and message.\"}),`\n`,(0,r.jsxs)(e.table,{children:[(0,r.jsx)(e.thead,{children:(0,r.jsxs)(e.tr,{children:[(0,r.jsx)(e.th,{children:\"Status Code\"}),(0,r.jsx)(e.th,{children:\"Description\"}),(0,r.jsx)(e.th,{children:\"Most Likely Cause\"})]})}),(0,r.jsxs)(e.tbody,{children:[(0,r.jsxs)(e.tr,{children:[(0,r.jsx)(e.td,{children:\"2XX\"}),(0,r.jsx)(e.td,{children:\"Successful Request\"}),(0,r.jsx)(e.td,{})]}),(0,r.jsxs)(e.tr,{children:[(0,r.jsx)(e.td,{children:\"400\"}),(0,r.jsx)(e.td,{children:\"Bad Request\"}),(0,r.jsx)(e.td,{children:\"Invalid/missing data\"})]}),(0,r.jsxs)(e.tr,{children:[(0,r.jsx)(e.td,{children:\"401\"}),(0,r.jsx)(e.td,{children:\"Unauthorized\"}),(0,r.jsx)(e.td,{children:\"Invalid/missing credentials\"})]}),(0,r.jsxs)(e.tr,{children:[(0,r.jsx)(e.td,{children:\"403\"}),(0,r.jsx)(e.td,{children:\"Forbidden\"}),(0,r.jsxs)(e.td,{children:[\"User does not have permission or has not authorized \",(0,r.jsx)(e.a,{href:\"/docs/api/using-zoom-apis#shared-access-permissions\",children:\"shared access permissions\"}),\".\"]})]}),(0,r.jsxs)(e.tr,{children:[(0,r.jsx)(e.td,{children:\"404\"}),(0,r.jsx)(e.td,{children:\"Not Found\"}),(0,r.jsx)(e.td,{children:\"The resource doesn't exist; invalid or non-existent user ID, for example\"})]}),(0,r.jsxs)(e.tr,{children:[(0,r.jsx)(e.td,{children:\"409\"}),(0,r.jsx)(e.td,{children:\"Conflict\"}),(0,r.jsx)(e.td,{children:\"Trying to overwrite a resource, for example when creating a user with an email that already exists\"})]}),(0,r.jsxs)(e.tr,{children:[(0,r.jsx)(e.td,{children:\"429\"}),(0,r.jsx)(e.td,{children:\"Too Many Requests\"}),(0,r.jsx)(e.td,{children:\"Hit an API rate limit\"})]}),(0,r.jsxs)(e.tr,{children:[(0,r.jsx)(e.td,{children:\"4700\"}),(0,r.jsx)(e.td,{children:\"Invalid Access Token\"}),(0,r.jsx)(e.td,{children:\"Invalid access token, does not contain scopes. The user is not authorized\"})]})]})]}),`\n`,(0,r.jsx)(e.h3,{id:\"error-response-example\",children:(0,r.jsxs)(e.a,{href:\"#error-response-example\",children:[\"Error response example\",(0,r.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,r.jsx)(e.pre,{children:(0,r.jsx)(e.code,{className:\"language-json\",children:`{\n  \"code\": 300,\n  \"message\": \"Request Body should be a valid JSON object.\"\n}\n`})}),`\n`,(0,r.jsx)(e.h3,{id:\"error-response-when-sending-invalid-fields\",children:(0,r.jsxs)(e.a,{href:\"#error-response-when-sending-invalid-fields\",children:[\"Error response when sending invalid fields\",(0,r.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,r.jsx)(e.pre,{children:(0,r.jsx)(e.code,{className:\"language-json\",children:`{\n  \"code\": 300,\n  \"message\": \"Validation Failed.\",\n  \"errors\": [\n    {\n      \"field\": \"user_info.email\",\n      \"message\": \"Invalid field.\"\n    },\n    {\n      \"field\": \"user_info.type\",\n      \"message\": \"Invalid field.\"\n    }\n  ]\n}\n`})})]})}function u(n={}){let{wrapper:e}={...i(),...n.components};return e?(0,r.jsx)(e,{...n,children:(0,r.jsx)(h,{...n})}):h(n)}return b(I);})();\n;return Component;",
      "frontmatter": {
        "title": "Error Codes"
      },
      "errors": [],
      "matter": {
        "content": "\n# Error Codes\n\nThe Zoom API uses HTTP Status codes to reflect a successful or unsuccesful request. 2XX status codes represent a successful request, 4XX/5XX status codes represent an error took place. If you receive an error status code, check the body for an error code and message.\n\n| Status Code | Description          | Most Likely Cause                                                                                                                          |\n| ----------- | -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |\n| 2XX         | Successful Request   |                                                                                                                                            |\n| 400         | Bad Request          | Invalid/missing data                                                                                                                       |\n| 401         | Unauthorized         | Invalid/missing credentials                                                                                                                |\n| 403         | Forbidden            | User does not have permission or has not authorized [shared access permissions](/docs/api/using-zoom-apis#shared-access-permissions). |\n| 404         | Not Found            | The resource doesn't exist; invalid or non-existent user ID, for example                                                                   |\n| 409         | Conflict             | Trying to overwrite a resource, for example when creating a user with an email that already exists                                         |\n| 429         | Too Many Requests    | Hit an API rate limit                                                                                                                      |\n| 4700        | Invalid Access Token | Invalid access token, does not contain scopes. The user is not authorized                                                                  |\n\n### Error response example\n\n```json\n{\n  \"code\": 300,\n  \"message\": \"Request Body should be a valid JSON object.\"\n}\n```\n\n### Error response when sending invalid fields\n\n```json\n{\n  \"code\": 300,\n  \"message\": \"Validation Failed.\",\n  \"errors\": [\n    {\n      \"field\": \"user_info.email\",\n      \"message\": \"Invalid field.\"\n    },\n    {\n      \"field\": \"user_info.type\",\n      \"message\": \"Invalid field.\"\n    }\n  ]\n}\n```\n",
        "data": {
          "title": "Error Codes"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/errors.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
