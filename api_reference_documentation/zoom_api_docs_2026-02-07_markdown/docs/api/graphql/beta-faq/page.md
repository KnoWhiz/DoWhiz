# Zoom GraphQL beta FAQ

- Source URL: https://developers.zoom.us/docs/api/graphql/beta-faq/
- Snapshot path: docs/api/graphql/beta-faq
- Generated (UTC): 2026-02-07T22:40:10.797268+00:00

## Frontmatter

```json
{
  "title": "Zoom GraphQL beta FAQ"
}
```

## Content

```md

# Zoom GraphQL beta FAQ

<Alert variant="warning">

**Beta**

Zoom GraphQL is in a public beta.

</Alert>

Here are some answers to Frequently Asked Questions about the GraphQL public beta.

## What is GraphQL?

GraphQL is a query language and server-side runtime for application programming interfaces (APIs) that prioritizes giving clients exactly the data they request and no more.

GraphQL is designed to make APIs fast, flexible, and developer-friendly. It can even be deployed within an integrated development environment (IDE) known as GraphiQL.

## What is the difference between GraphQL and REST API?

GraphQL lets developers construct requests that pull data from multiple data sources in a single API call.

### REST API

REST stands for [Representational state transfer](https://en.wikipedia.org/wiki/Representational_state_transfer). It's used to describe Web applications, which transfer the application's state to the client by traversing through links. The REST architectural style includes some guiding constraints. Systems which follow these constraints are called RESTful.

A RESTful API call is the process of a client application submitting a request to an API endpoint, the API server acting on that request, such as to retrieve data, and the server responding back to the client with a successful response that includes any data requested, or an error.

### GraphQL

[GraphQL](https://graphql.org/) is a query language for APIs as well as a runtime environment for filling those queries with data. A GraphQL schema is made up of object types, which define which kind of object you can request and the fields it contains. As queries come in, GraphQL validates the queries against the schema. GraphQL then executes the validated queries.

## What is the goal of this beta?

We want to get feedback from our beta customers on the complete experience, such as getting started, Developer documentation, the app build flow, getting your app running, troubleshooting your app, and more.

## How can I sign up for the beta?

You don't need to sign up. The GraphQL beta is available for your account. You can start using it now!

## Can I use GraphQL with a free Zoom account?

No, you must have a Zoom Pro account or higher to use GraphQL.

## Is GraphQL account level or user-managed?

Both. Depending on your access token scope and also the objects that you request. Graph QL uses the same authentication mechanism as our Rest APIs.

## Can I use GraphQL outside of my Zoom Account?

For example, if you use GraphQL in your app and publish it, can users activate your app in their account and use GraphQL calls?

No, GraphQL are “bound” to the account that creates them, and are not intended for multi-tenancy like Zoom Marketplace Apps.

## What app types can I use with GraphQL?

OAuth and Server-to-Server OAuth app types.

## Is GraphQL available for the Video SDK API?

No, but this is something we are exploring. If you are interested in using GraphQL for the Video SDK API, please let us know.
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
      "code": "var Component=(()=>{var u=Object.create;var i=Object.defineProperty;var f=Object.getOwnPropertyDescriptor;var g=Object.getOwnPropertyNames;var m=Object.getPrototypeOf,w=Object.prototype.hasOwnProperty;var b=(t,e)=>()=>(e||t((e={exports:{}}).exports,e),e.exports),y=(t,e)=>{for(var n in e)i(t,n,{get:e[n],enumerable:!0})},s=(t,e,n,o)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let r of g(e))!w.call(t,r)&&r!==n&&i(t,r,{get:()=>e[r],enumerable:!(o=f(e,r))||o.enumerable});return t};var q=(t,e,n)=>(n=t!=null?u(m(t)):{},s(e||!t||!t.__esModule?i(n,\"default\",{value:t,enumerable:!0}):n,t)),v=t=>s(i({},\"__esModule\",{value:!0}),t);var d=b((I,c)=>{c.exports=_jsx_runtime});var G={};y(G,{default:()=>p,frontmatter:()=>Q});var a=q(d());var{useMDXComponents:h}=MdxJsReact;var Q={title:\"Zoom GraphQL beta FAQ\"};function l(t){let e={a:\"a\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",p:\"p\",strong:\"strong\",...h(),...t.components},{Alert:n}=e;return n||L(\"Alert\",!0),(0,a.jsxs)(a.Fragment,{children:[(0,a.jsx)(e.h1,{id:\"zoom-graphql-beta-faq\",children:\"Zoom GraphQL beta FAQ\"}),`\n`,(0,a.jsxs)(n,{variant:\"warning\",children:[(0,a.jsx)(e.p,{children:(0,a.jsx)(e.strong,{children:\"Beta\"})}),(0,a.jsx)(e.p,{children:\"Zoom GraphQL is in a public beta.\"})]}),`\n`,(0,a.jsx)(e.p,{children:\"Here are some answers to Frequently Asked Questions about the GraphQL public beta.\"}),`\n`,(0,a.jsx)(e.h2,{id:\"what-is-graphql\",children:(0,a.jsxs)(e.a,{href:\"#what-is-graphql\",children:[\"What is GraphQL?\",(0,a.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,a.jsx)(e.p,{children:\"GraphQL is a query language and server-side runtime for application programming interfaces (APIs) that prioritizes giving clients exactly the data they request and no more.\"}),`\n`,(0,a.jsx)(e.p,{children:\"GraphQL is designed to make APIs fast, flexible, and developer-friendly. It can even be deployed within an integrated development environment (IDE) known as GraphiQL.\"}),`\n`,(0,a.jsx)(e.h2,{id:\"what-is-the-difference-between-graphql-and-rest-api\",children:(0,a.jsxs)(e.a,{href:\"#what-is-the-difference-between-graphql-and-rest-api\",children:[\"What is the difference between GraphQL and REST API?\",(0,a.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,a.jsx)(e.p,{children:\"GraphQL lets developers construct requests that pull data from multiple data sources in a single API call.\"}),`\n`,(0,a.jsx)(e.h3,{id:\"rest-api\",children:(0,a.jsxs)(e.a,{href:\"#rest-api\",children:[\"REST API\",(0,a.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,a.jsxs)(e.p,{children:[\"REST stands for \",(0,a.jsx)(e.a,{href:\"https://en.wikipedia.org/wiki/Representational_state_transfer\",children:\"Representational state transfer\"}),\". It's used to describe Web applications, which transfer the application's state to the client by traversing through links. The REST architectural style includes some guiding constraints. Systems which follow these constraints are called RESTful.\"]}),`\n`,(0,a.jsx)(e.p,{children:\"A RESTful API call is the process of a client application submitting a request to an API endpoint, the API server acting on that request, such as to retrieve data, and the server responding back to the client with a successful response that includes any data requested, or an error.\"}),`\n`,(0,a.jsx)(e.h3,{id:\"graphql\",children:(0,a.jsxs)(e.a,{href:\"#graphql\",children:[\"GraphQL\",(0,a.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,a.jsxs)(e.p,{children:[(0,a.jsx)(e.a,{href:\"https://graphql.org/\",children:\"GraphQL\"}),\" is a query language for APIs as well as a runtime environment for filling those queries with data. A GraphQL schema is made up of object types, which define which kind of object you can request and the fields it contains. As queries come in, GraphQL validates the queries against the schema. GraphQL then executes the validated queries.\"]}),`\n`,(0,a.jsx)(e.h2,{id:\"what-is-the-goal-of-this-beta\",children:(0,a.jsxs)(e.a,{href:\"#what-is-the-goal-of-this-beta\",children:[\"What is the goal of this beta?\",(0,a.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,a.jsx)(e.p,{children:\"We want to get feedback from our beta customers on the complete experience, such as getting started, Developer documentation, the app build flow, getting your app running, troubleshooting your app, and more.\"}),`\n`,(0,a.jsx)(e.h2,{id:\"how-can-i-sign-up-for-the-beta\",children:(0,a.jsxs)(e.a,{href:\"#how-can-i-sign-up-for-the-beta\",children:[\"How can I sign up for the beta?\",(0,a.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,a.jsx)(e.p,{children:\"You don't need to sign up. The GraphQL beta is available for your account. You can start using it now!\"}),`\n`,(0,a.jsx)(e.h2,{id:\"can-i-use-graphql-with-a-free-zoom-account\",children:(0,a.jsxs)(e.a,{href:\"#can-i-use-graphql-with-a-free-zoom-account\",children:[\"Can I use GraphQL with a free Zoom account?\",(0,a.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,a.jsx)(e.p,{children:\"No, you must have a Zoom Pro account or higher to use GraphQL.\"}),`\n`,(0,a.jsx)(e.h2,{id:\"is-graphql-account-level-or-user-managed\",children:(0,a.jsxs)(e.a,{href:\"#is-graphql-account-level-or-user-managed\",children:[\"Is GraphQL account level or user-managed?\",(0,a.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,a.jsx)(e.p,{children:\"Both. Depending on your access token scope and also the objects that you request. Graph QL uses the same authentication mechanism as our Rest APIs.\"}),`\n`,(0,a.jsx)(e.h2,{id:\"can-i-use-graphql-outside-of-my-zoom-account\",children:(0,a.jsxs)(e.a,{href:\"#can-i-use-graphql-outside-of-my-zoom-account\",children:[\"Can I use GraphQL outside of my Zoom Account?\",(0,a.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,a.jsx)(e.p,{children:\"For example, if you use GraphQL in your app and publish it, can users activate your app in their account and use GraphQL calls?\"}),`\n`,(0,a.jsx)(e.p,{children:\"No, GraphQL are \\u201Cbound\\u201D to the account that creates them, and are not intended for multi-tenancy like Zoom Marketplace Apps.\"}),`\n`,(0,a.jsx)(e.h2,{id:\"what-app-types-can-i-use-with-graphql\",children:(0,a.jsxs)(e.a,{href:\"#what-app-types-can-i-use-with-graphql\",children:[\"What app types can I use with GraphQL?\",(0,a.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,a.jsx)(e.p,{children:\"OAuth and Server-to-Server OAuth app types.\"}),`\n`,(0,a.jsx)(e.h2,{id:\"is-graphql-available-for-the-video-sdk-api\",children:(0,a.jsxs)(e.a,{href:\"#is-graphql-available-for-the-video-sdk-api\",children:[\"Is GraphQL available for the Video SDK API?\",(0,a.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,a.jsx)(e.p,{children:\"No, but this is something we are exploring. If you are interested in using GraphQL for the Video SDK API, please let us know.\"})]})}function p(t={}){let{wrapper:e}={...h(),...t.components};return e?(0,a.jsx)(e,{...t,children:(0,a.jsx)(l,{...t})}):l(t)}function L(t,e){throw new Error(\"Expected \"+(e?\"component\":\"object\")+\" `\"+t+\"` to be defined: you likely forgot to import, pass, or provide it.\")}return v(G);})();\n;return Component;",
      "frontmatter": {
        "title": "Zoom GraphQL beta FAQ"
      },
      "errors": [],
      "matter": {
        "content": "\n# Zoom GraphQL beta FAQ\n\n<Alert variant=\"warning\">\n\n**Beta**\n\nZoom GraphQL is in a public beta.\n\n</Alert>\n\nHere are some answers to Frequently Asked Questions about the GraphQL public beta.\n\n## What is GraphQL?\n\nGraphQL is a query language and server-side runtime for application programming interfaces (APIs) that prioritizes giving clients exactly the data they request and no more.\n\nGraphQL is designed to make APIs fast, flexible, and developer-friendly. It can even be deployed within an integrated development environment (IDE) known as GraphiQL.\n\n## What is the difference between GraphQL and REST API?\n\nGraphQL lets developers construct requests that pull data from multiple data sources in a single API call.\n\n### REST API\n\nREST stands for [Representational state transfer](https://en.wikipedia.org/wiki/Representational_state_transfer). It's used to describe Web applications, which transfer the application's state to the client by traversing through links. The REST architectural style includes some guiding constraints. Systems which follow these constraints are called RESTful.\n\nA RESTful API call is the process of a client application submitting a request to an API endpoint, the API server acting on that request, such as to retrieve data, and the server responding back to the client with a successful response that includes any data requested, or an error.\n\n### GraphQL\n\n[GraphQL](https://graphql.org/) is a query language for APIs as well as a runtime environment for filling those queries with data. A GraphQL schema is made up of object types, which define which kind of object you can request and the fields it contains. As queries come in, GraphQL validates the queries against the schema. GraphQL then executes the validated queries.\n\n## What is the goal of this beta?\n\nWe want to get feedback from our beta customers on the complete experience, such as getting started, Developer documentation, the app build flow, getting your app running, troubleshooting your app, and more.\n\n## How can I sign up for the beta?\n\nYou don't need to sign up. The GraphQL beta is available for your account. You can start using it now!\n\n## Can I use GraphQL with a free Zoom account?\n\nNo, you must have a Zoom Pro account or higher to use GraphQL.\n\n## Is GraphQL account level or user-managed?\n\nBoth. Depending on your access token scope and also the objects that you request. Graph QL uses the same authentication mechanism as our Rest APIs.\n\n## Can I use GraphQL outside of my Zoom Account?\n\nFor example, if you use GraphQL in your app and publish it, can users activate your app in their account and use GraphQL calls?\n\nNo, GraphQL are \u201cbound\u201d to the account that creates them, and are not intended for multi-tenancy like Zoom Marketplace Apps.\n\n## What app types can I use with GraphQL?\n\nOAuth and Server-to-Server OAuth app types.\n\n## Is GraphQL available for the Video SDK API?\n\nNo, but this is something we are exploring. If you are interested in using GraphQL for the Video SDK API, please let us know.\n",
        "data": {
          "title": "Zoom GraphQL beta FAQ"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/graphql/beta-faq.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
