# Zoom GraphQL overview

- Source URL: https://developers.zoom.us/docs/api/graphql/
- Snapshot path: docs/api/graphql
- Generated (UTC): 2026-02-07T22:40:10.796889+00:00

## Frontmatter

```json
{
  "title": "Zoom GraphQL overview"
}
```

## Content

```md

# Zoom GraphQL overview

<Alert variant="warning">

**Beta**

Zoom GraphQL is in a public beta. See the [GraphQL beta FAQ](/docs/api/graphql/beta-faq) for details.

</Alert>

[GraphQL](https://graphql.org/) is a query language for APIs as well as a runtime environment for filling those queries with data.

## Introduction to GraphQL

GraphQL operations have a single entry point to fetch (query) or modify (mutate) data by describing it with syntax that mirrors its response payload. Here is the Zoom GraphQL endpoint:

`https://api.zoom.us/v3/graphql`

With a GraphQL operation, you can describe the data that you want in your request, rather than sending multiple requests, like with RESTful API endpoints. With GraphQL, you can define the schema with your own custom objects using multiple fields.

## REST APIs: under fetching and over fetching

Traditionally when using Rest APIs, the entities live on a bunch of endpoints on a server. When you make a request, it returns all the entities from that request. There are two drawbacks in this use case, under fetching and over fetching.

**Under fetching**

With REST APIs, you may need to make multiple requests to fetch multiple entities. For example, if you want to get the list of users in your Zoom account and all of their meetings, you'll need to call more than one API endpoint. This is an example of under fetching because you're not getting all the data you need in one response.

**Over fetching**

In some cases, you may only want a small subset of the data entity. With REST APIs, you'd still have to request all of the data from an API. For example, if you only want to receive the user email and user ID from an API that sends all user data, it will send that data and more. This is an example of over fetching as you are getting more data than you need in a response.

## Advantages of GraphQL

GraphQL enables you to create a query to fetch only the data you need. Here's an example that requests a user's profile details, meeting details, and recording details in a single query. The RESTful alternative would be multiple queries to multiple API endpoints.

```json
{
   user(userId:"me", ){

       profile{
           id
           email
           employeeUniqueId
           firstName
           lastName
           role{
               id
               name
           }
       }
       meetings(first:100,meetingType:PREVIOUS_MEETINGS){
           edges{
               id
               topic
               startTime
           }

       }
       recordings(first:100){
           edges{
               uuid
               topic
           }
       }

   }
}
```

## GraphQL operations

GraphQL operations consist mainly of queries and mutations.

### Queries

If you're used to REST APIs, you're probably familiar with the HTTP GET method to fetch data. With GraphQL, you'd use queries, such as the one shown earlier to request user details.

### Mutations

REST APIs use HTTP methods such as DELETE, PUT, PATCH, and POST to modify data. GraphQL uses mutations. For example, the following to create a user:

```
mutation($input:UserCreateInput!){
 		createUser(input:$input)
{
   		id
email
type
firstName
lastName
 	}
}
input: {
   action: "CREATE",
   userInfo: {
     email: "first.last@example.com",
     type: "Licensed",
     firstName: "first",
     lastName: "last"
   }
 }

```

## Entities available

Zoom GraphQL is built on top of the [Zoom API](/docs/api/). You can access the following entities using Zoom GraphQL:

* Chat channels (partial)
* Cloud recording
* Dashboards
* Groups (partial)
* Meetings
* Reports (partial)
* Users
* Webinars

See the [Zoom GraphQL API playground](https://nws.zoom.us/graphql/playground) or [Postman collection](#postman-collection) for details.

<Alert variant="warning">

**Not available for Video SDK**

GraphQL is not available for Video SDK.

</Alert>

## Postman collection

Go to the [Zoom GraphQL Collection](https://www.postman.com/zoom-developer/workspace/zoom-public-workspace/collection/22097587-84c52112-0ae1-4517-a32a-c95a5055b4e3) in Postman to see sample queries and mutations. See [Using Postman to query](/docs/api/graphql/graphql-postman) for details.
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
      "code": "var Component=(()=>{var u=Object.create;var a=Object.defineProperty;var m=Object.getOwnPropertyDescriptor;var f=Object.getOwnPropertyNames;var g=Object.getPrototypeOf,y=Object.prototype.hasOwnProperty;var q=(t,e)=>()=>(e||t((e={exports:{}}).exports,e),e.exports),v=(t,e)=>{for(var i in e)a(t,i,{get:e[i],enumerable:!0})},l=(t,e,i,s)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let r of f(e))!y.call(t,r)&&r!==i&&a(t,r,{get:()=>e[r],enumerable:!(s=m(e,r))||s.enumerable});return t};var w=(t,e,i)=>(i=t!=null?u(g(t)):{},l(e||!t||!t.__esModule?a(i,\"default\",{value:t,enumerable:!0}):i,t)),b=t=>l(a({},\"__esModule\",{value:!0}),t);var d=q((P,h)=>{h.exports=_jsx_runtime});var I={};v(I,{default:()=>p,frontmatter:()=>G});var n=w(d());var{useMDXComponents:o}=MdxJsReact;var G={title:\"Zoom GraphQL overview\"};function c(t){let e={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",p:\"p\",pre:\"pre\",strong:\"strong\",ul:\"ul\",...o(),...t.components},{Alert:i}=e;return i||T(\"Alert\",!0),(0,n.jsxs)(n.Fragment,{children:[(0,n.jsx)(e.h1,{id:\"zoom-graphql-overview\",children:\"Zoom GraphQL overview\"}),`\n`,(0,n.jsxs)(i,{variant:\"warning\",children:[(0,n.jsx)(e.p,{children:(0,n.jsx)(e.strong,{children:\"Beta\"})}),(0,n.jsxs)(e.p,{children:[\"Zoom GraphQL is in a public beta. See the \",(0,n.jsx)(e.a,{href:\"/docs/api/graphql/beta-faq\",children:\"GraphQL beta FAQ\"}),\" for details.\"]})]}),`\n`,(0,n.jsxs)(e.p,{children:[(0,n.jsx)(e.a,{href:\"https://graphql.org/\",children:\"GraphQL\"}),\" is a query language for APIs as well as a runtime environment for filling those queries with data.\"]}),`\n`,(0,n.jsx)(e.h2,{id:\"introduction-to-graphql\",children:(0,n.jsxs)(e.a,{href:\"#introduction-to-graphql\",children:[\"Introduction to GraphQL\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsx)(e.p,{children:\"GraphQL operations have a single entry point to fetch (query) or modify (mutate) data by describing it with syntax that mirrors its response payload. Here is the Zoom GraphQL endpoint:\"}),`\n`,(0,n.jsx)(e.p,{children:(0,n.jsx)(e.code,{children:\"https://api.zoom.us/v3/graphql\"})}),`\n`,(0,n.jsx)(e.p,{children:\"With a GraphQL operation, you can describe the data that you want in your request, rather than sending multiple requests, like with RESTful API endpoints. With GraphQL, you can define the schema with your own custom objects using multiple fields.\"}),`\n`,(0,n.jsx)(e.h2,{id:\"rest-apis-under-fetching-and-over-fetching\",children:(0,n.jsxs)(e.a,{href:\"#rest-apis-under-fetching-and-over-fetching\",children:[\"REST APIs: under fetching and over fetching\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsx)(e.p,{children:\"Traditionally when using Rest APIs, the entities live on a bunch of endpoints on a server. When you make a request, it returns all the entities from that request. There are two drawbacks in this use case, under fetching and over fetching.\"}),`\n`,(0,n.jsx)(e.p,{children:(0,n.jsx)(e.strong,{children:\"Under fetching\"})}),`\n`,(0,n.jsx)(e.p,{children:\"With REST APIs, you may need to make multiple requests to fetch multiple entities. For example, if you want to get the list of users in your Zoom account and all of their meetings, you'll need to call more than one API endpoint. This is an example of under fetching because you're not getting all the data you need in one response.\"}),`\n`,(0,n.jsx)(e.p,{children:(0,n.jsx)(e.strong,{children:\"Over fetching\"})}),`\n`,(0,n.jsx)(e.p,{children:\"In some cases, you may only want a small subset of the data entity. With REST APIs, you'd still have to request all of the data from an API. For example, if you only want to receive the user email and user ID from an API that sends all user data, it will send that data and more. This is an example of over fetching as you are getting more data than you need in a response.\"}),`\n`,(0,n.jsx)(e.h2,{id:\"advantages-of-graphql\",children:(0,n.jsxs)(e.a,{href:\"#advantages-of-graphql\",children:[\"Advantages of GraphQL\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsx)(e.p,{children:\"GraphQL enables you to create a query to fetch only the data you need. Here's an example that requests a user's profile details, meeting details, and recording details in a single query. The RESTful alternative would be multiple queries to multiple API endpoints.\"}),`\n`,(0,n.jsx)(e.pre,{children:(0,n.jsx)(e.code,{className:\"language-json\",children:`{\n   user(userId:\"me\", ){\n\n       profile{\n           id\n           email\n           employeeUniqueId\n           firstName\n           lastName\n           role{\n               id\n               name\n           }\n       }\n       meetings(first:100,meetingType:PREVIOUS_MEETINGS){\n           edges{\n               id\n               topic\n               startTime\n           }\n\n       }\n       recordings(first:100){\n           edges{\n               uuid\n               topic\n           }\n       }\n\n   }\n}\n`})}),`\n`,(0,n.jsx)(e.h2,{id:\"graphql-operations\",children:(0,n.jsxs)(e.a,{href:\"#graphql-operations\",children:[\"GraphQL operations\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsx)(e.p,{children:\"GraphQL operations consist mainly of queries and mutations.\"}),`\n`,(0,n.jsx)(e.h3,{id:\"queries\",children:(0,n.jsxs)(e.a,{href:\"#queries\",children:[\"Queries\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsx)(e.p,{children:\"If you're used to REST APIs, you're probably familiar with the HTTP GET method to fetch data. With GraphQL, you'd use queries, such as the one shown earlier to request user details.\"}),`\n`,(0,n.jsx)(e.h3,{id:\"mutations\",children:(0,n.jsxs)(e.a,{href:\"#mutations\",children:[\"Mutations\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsx)(e.p,{children:\"REST APIs use HTTP methods such as DELETE, PUT, PATCH, and POST to modify data. GraphQL uses mutations. For example, the following to create a user:\"}),`\n`,(0,n.jsx)(e.pre,{children:(0,n.jsx)(e.code,{children:`mutation($input:UserCreateInput!){\n \t\tcreateUser(input:$input)\n{\n   \t\tid\nemail\ntype\nfirstName\nlastName\n \t}\n}\ninput: {\n   action: \"CREATE\",\n   userInfo: {\n     email: \"first.last@example.com\",\n     type: \"Licensed\",\n     firstName: \"first\",\n     lastName: \"last\"\n   }\n }\n\n`})}),`\n`,(0,n.jsx)(e.h2,{id:\"entities-available\",children:(0,n.jsxs)(e.a,{href:\"#entities-available\",children:[\"Entities available\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.p,{children:[\"Zoom GraphQL is built on top of the \",(0,n.jsx)(e.a,{href:\"/docs/api/\",children:\"Zoom API\"}),\". You can access the following entities using Zoom GraphQL:\"]}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsx)(e.li,{children:\"Chat channels (partial)\"}),`\n`,(0,n.jsx)(e.li,{children:\"Cloud recording\"}),`\n`,(0,n.jsx)(e.li,{children:\"Dashboards\"}),`\n`,(0,n.jsx)(e.li,{children:\"Groups (partial)\"}),`\n`,(0,n.jsx)(e.li,{children:\"Meetings\"}),`\n`,(0,n.jsx)(e.li,{children:\"Reports (partial)\"}),`\n`,(0,n.jsx)(e.li,{children:\"Users\"}),`\n`,(0,n.jsx)(e.li,{children:\"Webinars\"}),`\n`]}),`\n`,(0,n.jsxs)(e.p,{children:[\"See the \",(0,n.jsx)(e.a,{href:\"https://nws.zoom.us/graphql/playground\",children:\"Zoom GraphQL API playground\"}),\" or \",(0,n.jsx)(e.a,{href:\"#postman-collection\",children:\"Postman collection\"}),\" for details.\"]}),`\n`,(0,n.jsxs)(i,{variant:\"warning\",children:[(0,n.jsx)(e.p,{children:(0,n.jsx)(e.strong,{children:\"Not available for Video SDK\"})}),(0,n.jsx)(e.p,{children:\"GraphQL is not available for Video SDK.\"})]}),`\n`,(0,n.jsx)(e.h2,{id:\"postman-collection\",children:(0,n.jsxs)(e.a,{href:\"#postman-collection\",children:[\"Postman collection\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.p,{children:[\"Go to the \",(0,n.jsx)(e.a,{href:\"https://www.postman.com/zoom-developer/workspace/zoom-public-workspace/collection/22097587-84c52112-0ae1-4517-a32a-c95a5055b4e3\",children:\"Zoom GraphQL Collection\"}),\" in Postman to see sample queries and mutations. See \",(0,n.jsx)(e.a,{href:\"/docs/api/graphql/graphql-postman\",children:\"Using Postman to query\"}),\" for details.\"]})]})}function p(t={}){let{wrapper:e}={...o(),...t.components};return e?(0,n.jsx)(e,{...t,children:(0,n.jsx)(c,{...t})}):c(t)}function T(t,e){throw new Error(\"Expected \"+(e?\"component\":\"object\")+\" `\"+t+\"` to be defined: you likely forgot to import, pass, or provide it.\")}return b(I);})();\n;return Component;",
      "frontmatter": {
        "title": "Zoom GraphQL overview"
      },
      "errors": [],
      "matter": {
        "content": "\n# Zoom GraphQL overview\n\n<Alert variant=\"warning\">\n\n**Beta**\n\nZoom GraphQL is in a public beta. See the [GraphQL beta FAQ](/docs/api/graphql/beta-faq) for details.\n\n</Alert>\n\n[GraphQL](https://graphql.org/) is a query language for APIs as well as a runtime environment for filling those queries with data.\n\n## Introduction to GraphQL\n\nGraphQL operations have a single entry point to fetch (query) or modify (mutate) data by describing it with syntax that mirrors its response payload. Here is the Zoom GraphQL endpoint:\n\n`https://api.zoom.us/v3/graphql`\n\nWith a GraphQL operation, you can describe the data that you want in your request, rather than sending multiple requests, like with RESTful API endpoints. With GraphQL, you can define the schema with your own custom objects using multiple fields.\n\n## REST APIs: under fetching and over fetching\n\nTraditionally when using Rest APIs, the entities live on a bunch of endpoints on a server. When you make a request, it returns all the entities from that request. There are two drawbacks in this use case, under fetching and over fetching.\n\n**Under fetching**\n\nWith REST APIs, you may need to make multiple requests to fetch multiple entities. For example, if you want to get the list of users in your Zoom account and all of their meetings, you'll need to call more than one API endpoint. This is an example of under fetching because you're not getting all the data you need in one response.\n\n**Over fetching**\n\nIn some cases, you may only want a small subset of the data entity. With REST APIs, you'd still have to request all of the data from an API. For example, if you only want to receive the user email and user ID from an API that sends all user data, it will send that data and more. This is an example of over fetching as you are getting more data than you need in a response.\n\n## Advantages of GraphQL\n\nGraphQL enables you to create a query to fetch only the data you need. Here's an example that requests a user's profile details, meeting details, and recording details in a single query. The RESTful alternative would be multiple queries to multiple API endpoints.\n\n```json\n{\n   user(userId:\"me\", ){\n\n       profile{\n           id\n           email\n           employeeUniqueId\n           firstName\n           lastName\n           role{\n               id\n               name\n           }\n       }\n       meetings(first:100,meetingType:PREVIOUS_MEETINGS){\n           edges{\n               id\n               topic\n               startTime\n           }\n\n       }\n       recordings(first:100){\n           edges{\n               uuid\n               topic\n           }\n       }\n\n   }\n}\n```\n\n## GraphQL operations\n\nGraphQL operations consist mainly of queries and mutations.\n\n### Queries\n\nIf you're used to REST APIs, you're probably familiar with the HTTP GET method to fetch data. With GraphQL, you'd use queries, such as the one shown earlier to request user details.\n\n### Mutations\n\nREST APIs use HTTP methods such as DELETE, PUT, PATCH, and POST to modify data. GraphQL uses mutations. For example, the following to create a user:\n\n```\nmutation($input:UserCreateInput!){\n \t\tcreateUser(input:$input)\n{\n   \t\tid\nemail\ntype\nfirstName\nlastName\n \t}\n}\ninput: {\n   action: \"CREATE\",\n   userInfo: {\n     email: \"first.last@example.com\",\n     type: \"Licensed\",\n     firstName: \"first\",\n     lastName: \"last\"\n   }\n }\n\n```\n\n## Entities available\n\nZoom GraphQL is built on top of the [Zoom API](/docs/api/). You can access the following entities using Zoom GraphQL:\n\n* Chat channels (partial)\n* Cloud recording\n* Dashboards\n* Groups (partial)\n* Meetings\n* Reports (partial)\n* Users\n* Webinars\n\nSee the [Zoom GraphQL API playground](https://nws.zoom.us/graphql/playground) or [Postman collection](#postman-collection) for details.\n\n<Alert variant=\"warning\">\n\n**Not available for Video SDK**\n\nGraphQL is not available for Video SDK.\n\n</Alert>\n\n## Postman collection\n\nGo to the [Zoom GraphQL Collection](https://www.postman.com/zoom-developer/workspace/zoom-public-workspace/collection/22097587-84c52112-0ae1-4517-a32a-c95a5055b4e3) in Postman to see sample queries and mutations. See [Using Postman to query](/docs/api/graphql/graphql-postman) for details.\n",
        "data": {
          "title": "Zoom GraphQL overview"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/graphql/index.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
