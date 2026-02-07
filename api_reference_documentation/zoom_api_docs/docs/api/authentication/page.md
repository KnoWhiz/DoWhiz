# Authentication

- Source URL: https://developers.zoom.us/docs/api/authentication/
- Snapshot path: docs/api/authentication
- Generated (UTC): 2026-02-07T22:40:10.460296+00:00

## Frontmatter

```json
{
  "title": "Authentication",
  "hideToc": true
}
```

## Content

```md

# Authentication

<TwoColumn>
<LeftColumn>

Requests to the Zoom API are authenticated through a Bearer token in an Authorization header.

</LeftColumn>

<RightColumn>
<CodeBlock header="cURL" theme="dark">

```
curl -H "Authorization: Bearer YOUR_ACCESS_TOKEN" https://api.zoom.us/v2/users/me
```

</CodeBlock>
</RightColumn>
</TwoColumn>

## Choosing the right authentication method

- Use **OAuth 2.0 for apps that need user authorization** and for all apps that publish on the Zoom App Marketplace.
- Use **server-to-server OAuth for backend services on your own account** that don't require user interaction.

## OAuth 2.0 for user-based authentication
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
      "code": "var Component=(()=>{var f=Object.create;var h=Object.defineProperty;var g=Object.getOwnPropertyDescriptor;var C=Object.getOwnPropertyNames;var x=Object.getPrototypeOf,_=Object.prototype.hasOwnProperty;var A=(n,e)=>()=>(e||n((e={exports:{}}).exports,e),e.exports),M=(n,e)=>{for(var o in e)h(n,o,{get:e[o],enumerable:!0})},s=(n,e,o,i)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let r of C(e))!_.call(n,r)&&r!==o&&h(n,r,{get:()=>e[r],enumerable:!(i=g(e,r))||i.enumerable});return n};var b=(n,e,o)=>(o=n!=null?f(x(n)):{},s(e||!n||!n.__esModule?h(o,\"default\",{value:n,enumerable:!0}):o,n)),k=n=>s(h({},\"__esModule\",{value:!0}),n);var l=A((w,d)=>{d.exports=_jsx_runtime});var j={};M(j,{default:()=>p,frontmatter:()=>R});var t=b(l());var{useMDXComponents:c}=MdxJsReact;var R={title:\"Authentication\",hideToc:!0};function m(n){let e={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",i:\"i\",li:\"li\",p:\"p\",pre:\"pre\",strong:\"strong\",ul:\"ul\",...c(),...n.components},{CodeBlock:o,LeftColumn:i,RightColumn:r,TwoColumn:u}=e;return o||a(\"CodeBlock\",!0),i||a(\"LeftColumn\",!0),r||a(\"RightColumn\",!0),u||a(\"TwoColumn\",!0),(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(e.h1,{id:\"authentication\",children:\"Authentication\"}),`\n`,(0,t.jsxs)(u,{children:[(0,t.jsx)(i,{children:(0,t.jsx)(e.p,{children:\"Requests to the Zoom API are authenticated through a Bearer token in an Authorization header.\"})}),(0,t.jsx)(r,{children:(0,t.jsx)(o,{header:\"cURL\",theme:\"dark\",children:(0,t.jsx)(e.pre,{children:(0,t.jsx)(e.code,{children:`curl -H \"Authorization: Bearer YOUR_ACCESS_TOKEN\" https://api.zoom.us/v2/users/me\n`})})})})]}),`\n`,(0,t.jsx)(e.h2,{id:\"choosing-the-right-authentication-method\",children:(0,t.jsxs)(e.a,{href:\"#choosing-the-right-authentication-method\",children:[\"Choosing the right authentication method\",(0,t.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,t.jsxs)(e.ul,{children:[`\n`,(0,t.jsxs)(e.li,{children:[\"Use \",(0,t.jsx)(e.strong,{children:\"OAuth 2.0 for apps that need user authorization\"}),\" and for all apps that publish on the Zoom App Marketplace.\"]}),`\n`,(0,t.jsxs)(e.li,{children:[\"Use \",(0,t.jsx)(e.strong,{children:\"server-to-server OAuth for backend services on your own account\"}),\" that don't require user interaction.\"]}),`\n`]}),`\n`,(0,t.jsx)(e.h2,{id:\"oauth-20-for-user-based-authentication\",children:(0,t.jsxs)(e.a,{href:\"#oauth-20-for-user-based-authentication\",children:[\"OAuth 2.0 for user-based authentication\",(0,t.jsx)(e.i,{\"aria-hidden\":\"true\"})]})})]})}function p(n={}){let{wrapper:e}={...c(),...n.components};return e?(0,t.jsx)(e,{...n,children:(0,t.jsx)(m,{...n})}):m(n)}function a(n,e){throw new Error(\"Expected \"+(e?\"component\":\"object\")+\" `\"+n+\"` to be defined: you likely forgot to import, pass, or provide it.\")}return k(j);})();\n;return Component;",
      "frontmatter": {
        "title": "Authentication",
        "hideToc": true
      },
      "errors": [],
      "matter": {
        "content": "\n# Authentication\n\n<TwoColumn>\n<LeftColumn>\n\nRequests to the Zoom API are authenticated through a Bearer token in an Authorization header.\n\n</LeftColumn>\n\n<RightColumn>\n<CodeBlock header=\"cURL\" theme=\"dark\">\n\n```\ncurl -H \"Authorization: Bearer YOUR_ACCESS_TOKEN\" https://api.zoom.us/v2/users/me\n```\n\n</CodeBlock>\n</RightColumn>\n</TwoColumn>\n\n## Choosing the right authentication method\n\n- Use **OAuth 2.0 for apps that need user authorization** and for all apps that publish on the Zoom App Marketplace.\n- Use **server-to-server OAuth for backend services on your own account** that don't require user interaction.\n\n## OAuth 2.0 for user-based authentication\n",
        "data": {
          "title": "Authentication",
          "hideToc": true
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/authentication.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
