# Get started with GraphQL

- Source URL: https://developers.zoom.us/docs/api/graphql/get-started/
- Snapshot path: docs/api/graphql/get-started
- Generated (UTC): 2026-02-07T22:40:10.797633+00:00

## Frontmatter

```json
{
  "title": "Get started with GraphQL"
}
```

## Content

```md

# Get started with GraphQL

<Alert variant="warning">

**Beta**

Zoom GraphQL is in a public beta. 
* See the [GraphQL beta FAQ](/docs/api/graphql/beta-faq) for details.
* **Get started using the [Zoom GraphQL API playground](https://nws.zoom.us/graphql/playground).** 
* See the [Zoom GraphQL API playground](/blog/graphql-api-playground) blog post for details.

</Alert>

<Alert variant="primary">

**GraphQL endpoint**

Zoom GraphQL is available at the following endpoint: `https://api.zoom.us/v3/graphql`

</Alert>

## Make your first GraphQL query

Follow these steps to make your first GraphQL query in Postman, to get the `uuid`, `id`, and `joinUrl` for your previous meetings.

### Before you start

1. Log on to your Marketplace account (be sure to use an account that you've used for some meetings in the past).
2. Create a [Server-to-Server OAuth](/docs/internal-apps/create) app type.
3. Add the scope to **View all user meetings**: `meeting:read:admin`

### Create an authorization token

4. In the Postman **Params** tab, add the following keys and values:

Key | Value
--- | ---
`grant_type` | `account_credentials`
`account_id` | Your **Account ID**, shown in the App credentials tab for your app in Marketplace (use a [Postman variable](https://learning.postman.com/docs/sending-requests/variables/) for this and other credentials).

5. In the Postman **Authorization** tab, enter your app's **Client ID** as the Username and **Client secret** as the Password.
6. Choose `POST`, enter request URL `https://zoom.us/oauth/token` and click **Send**.
7. Copy your **access token**.

### Query meetings

8. Create a new request and click the Postman **Authorization** tab.
9. In Type, choose **Bearer Token**.
10. Enter your **access token** (use a [variable](https://learning.postman.com/docs/sending-requests/variables/)).
11. Click **Body**, choose **GraphQL**, and enter the following test query:

```json
{
	meetings (meetingType: PREVIOUS_MEETINGS) {
		edges {
			uuid
			id
			joinUrl
		}
	}
}
```

12. Choose `POST`, enter request URL `https://zoom.us/v3/graphql` and click **Send**.
12. If you've had previous meetings on this account, you should get a response like the following:

```json
{
	"data": {
		"meetings": {
			"edges": [
				{
					"uuid": "AAABBBCCCDDDEEEFFFGGGH==",
					"id": 11111111111,
					"joinUrl": "https://link.example.com/a/11111111111"
				},
				{
					"uuid": "1112223334445556667778==",
					"id": 22222222222,
					"joinUrl": "https://link.example.com/a/22222222222?pwd=000000000000000000000000000000.0"
				},
				{
					"uuid": "+++===+++===+++===+++w+==",
					"id": 33333333333,
					"joinUrl": "https://link.example.com/a/33333333333"
				}
			]
		}
	}
}
```

## Learn more

* See **Using GraphQL** for more details about authentication, queries, mutations, and pagination.
* See **Postman samples** for information about the Zoom GraphQL public workspace on Postman and how to add the Zoom GraphQL schema to Postman.
* See **Scopes** and **Rate limits** for tables showing these affect GraphQL queries and mutations.
* Use the [Zoom GraphQL API playground](https://nws.zoom.us/graphql/playground) to send queries and mutations directly in your web browser. Access the GraphQL reference documentation and autocomplete JSON code directly in the playground. See the [Zoom Dev article](https://dev.to/zoom/the-zoom-graphql-api-playground-your-new-favorite-development-tool-59n5) to learn more.
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
      "code": "var Component=(()=>{var u=Object.create;var o=Object.defineProperty;var m=Object.getOwnPropertyDescriptor;var g=Object.getOwnPropertyNames;var f=Object.getPrototypeOf,y=Object.prototype.hasOwnProperty;var b=(n,e)=>()=>(e||n((e={exports:{}}).exports,e),e.exports),q=(n,e)=>{for(var r in e)o(n,r,{get:e[r],enumerable:!0})},l=(n,e,r,d)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let i of g(e))!y.call(n,i)&&i!==r&&o(n,i,{get:()=>e[i],enumerable:!(d=m(e,i))||d.enumerable});return n};var w=(n,e,r)=>(r=n!=null?u(f(n)):{},l(e||!n||!n.__esModule?o(r,\"default\",{value:n,enumerable:!0}):r,n)),k=n=>l(o({},\"__esModule\",{value:!0}),n);var c=b((C,h)=>{h.exports=_jsx_runtime});var L={};q(L,{default:()=>p,frontmatter:()=>G});var t=w(c());var{useMDXComponents:a}=MdxJsReact;var G={title:\"Get started with GraphQL\"};function s(n){let e={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",ol:\"ol\",p:\"p\",pre:\"pre\",strong:\"strong\",table:\"table\",tbody:\"tbody\",td:\"td\",th:\"th\",thead:\"thead\",tr:\"tr\",ul:\"ul\",...a(),...n.components},{Alert:r}=e;return r||v(\"Alert\",!0),(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(e.h1,{id:\"get-started-with-graphql\",children:\"Get started with GraphQL\"}),`\n`,(0,t.jsxs)(r,{variant:\"warning\",children:[(0,t.jsx)(e.p,{children:(0,t.jsx)(e.strong,{children:\"Beta\"})}),(0,t.jsx)(e.p,{children:\"Zoom GraphQL is in a public beta.\"}),(0,t.jsxs)(e.ul,{children:[`\n`,(0,t.jsxs)(e.li,{children:[\"See the \",(0,t.jsx)(e.a,{href:\"/docs/api/graphql/beta-faq\",children:\"GraphQL beta FAQ\"}),\" for details.\"]}),`\n`,(0,t.jsx)(e.li,{children:(0,t.jsxs)(e.strong,{children:[\"Get started using the \",(0,t.jsx)(e.a,{href:\"https://nws.zoom.us/graphql/playground\",children:\"Zoom GraphQL API playground\"}),\".\"]})}),`\n`,(0,t.jsxs)(e.li,{children:[\"See the \",(0,t.jsx)(e.a,{href:\"/blog/graphql-api-playground\",children:\"Zoom GraphQL API playground\"}),\" blog post for details.\"]}),`\n`]})]}),`\n`,(0,t.jsxs)(r,{variant:\"primary\",children:[(0,t.jsx)(e.p,{children:(0,t.jsx)(e.strong,{children:\"GraphQL endpoint\"})}),(0,t.jsxs)(e.p,{children:[\"Zoom GraphQL is available at the following endpoint: \",(0,t.jsx)(e.code,{children:\"https://api.zoom.us/v3/graphql\"})]})]}),`\n`,(0,t.jsx)(e.h2,{id:\"make-your-first-graphql-query\",children:(0,t.jsxs)(e.a,{href:\"#make-your-first-graphql-query\",children:[\"Make your first GraphQL query\",(0,t.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,t.jsxs)(e.p,{children:[\"Follow these steps to make your first GraphQL query in Postman, to get the \",(0,t.jsx)(e.code,{children:\"uuid\"}),\", \",(0,t.jsx)(e.code,{children:\"id\"}),\", and \",(0,t.jsx)(e.code,{children:\"joinUrl\"}),\" for your previous meetings.\"]}),`\n`,(0,t.jsx)(e.h3,{id:\"before-you-start\",children:(0,t.jsxs)(e.a,{href:\"#before-you-start\",children:[\"Before you start\",(0,t.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,t.jsxs)(e.ol,{children:[`\n`,(0,t.jsx)(e.li,{children:\"Log on to your Marketplace account (be sure to use an account that you've used for some meetings in the past).\"}),`\n`,(0,t.jsxs)(e.li,{children:[\"Create a \",(0,t.jsx)(e.a,{href:\"/docs/internal-apps/create\",children:\"Server-to-Server OAuth\"}),\" app type.\"]}),`\n`,(0,t.jsxs)(e.li,{children:[\"Add the scope to \",(0,t.jsx)(e.strong,{children:\"View all user meetings\"}),\": \",(0,t.jsx)(e.code,{children:\"meeting:read:admin\"})]}),`\n`]}),`\n`,(0,t.jsx)(e.h3,{id:\"create-an-authorization-token\",children:(0,t.jsxs)(e.a,{href:\"#create-an-authorization-token\",children:[\"Create an authorization token\",(0,t.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,t.jsxs)(e.ol,{start:\"4\",children:[`\n`,(0,t.jsxs)(e.li,{children:[\"In the Postman \",(0,t.jsx)(e.strong,{children:\"Params\"}),\" tab, add the following keys and values:\"]}),`\n`]}),`\n`,(0,t.jsxs)(e.table,{children:[(0,t.jsx)(e.thead,{children:(0,t.jsxs)(e.tr,{children:[(0,t.jsx)(e.th,{children:\"Key\"}),(0,t.jsx)(e.th,{children:\"Value\"})]})}),(0,t.jsxs)(e.tbody,{children:[(0,t.jsxs)(e.tr,{children:[(0,t.jsx)(e.td,{children:(0,t.jsx)(e.code,{children:\"grant_type\"})}),(0,t.jsx)(e.td,{children:(0,t.jsx)(e.code,{children:\"account_credentials\"})})]}),(0,t.jsxs)(e.tr,{children:[(0,t.jsx)(e.td,{children:(0,t.jsx)(e.code,{children:\"account_id\"})}),(0,t.jsxs)(e.td,{children:[\"Your \",(0,t.jsx)(e.strong,{children:\"Account ID\"}),\", shown in the App credentials tab for your app in Marketplace (use a \",(0,t.jsx)(e.a,{href:\"https://learning.postman.com/docs/sending-requests/variables/\",children:\"Postman variable\"}),\" for this and other credentials).\"]})]})]})]}),`\n`,(0,t.jsxs)(e.ol,{start:\"5\",children:[`\n`,(0,t.jsxs)(e.li,{children:[\"In the Postman \",(0,t.jsx)(e.strong,{children:\"Authorization\"}),\" tab, enter your app's \",(0,t.jsx)(e.strong,{children:\"Client ID\"}),\" as the Username and \",(0,t.jsx)(e.strong,{children:\"Client secret\"}),\" as the Password.\"]}),`\n`,(0,t.jsxs)(e.li,{children:[\"Choose \",(0,t.jsx)(e.code,{children:\"POST\"}),\", enter request URL \",(0,t.jsx)(e.code,{children:\"https://zoom.us/oauth/token\"}),\" and click \",(0,t.jsx)(e.strong,{children:\"Send\"}),\".\"]}),`\n`,(0,t.jsxs)(e.li,{children:[\"Copy your \",(0,t.jsx)(e.strong,{children:\"access token\"}),\".\"]}),`\n`]}),`\n`,(0,t.jsx)(e.h3,{id:\"query-meetings\",children:(0,t.jsxs)(e.a,{href:\"#query-meetings\",children:[\"Query meetings\",(0,t.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,t.jsxs)(e.ol,{start:\"8\",children:[`\n`,(0,t.jsxs)(e.li,{children:[\"Create a new request and click the Postman \",(0,t.jsx)(e.strong,{children:\"Authorization\"}),\" tab.\"]}),`\n`,(0,t.jsxs)(e.li,{children:[\"In Type, choose \",(0,t.jsx)(e.strong,{children:\"Bearer Token\"}),\".\"]}),`\n`,(0,t.jsxs)(e.li,{children:[\"Enter your \",(0,t.jsx)(e.strong,{children:\"access token\"}),\" (use a \",(0,t.jsx)(e.a,{href:\"https://learning.postman.com/docs/sending-requests/variables/\",children:\"variable\"}),\").\"]}),`\n`,(0,t.jsxs)(e.li,{children:[\"Click \",(0,t.jsx)(e.strong,{children:\"Body\"}),\", choose \",(0,t.jsx)(e.strong,{children:\"GraphQL\"}),\", and enter the following test query:\"]}),`\n`]}),`\n`,(0,t.jsx)(e.pre,{children:(0,t.jsx)(e.code,{className:\"language-json\",children:`{\n\tmeetings (meetingType: PREVIOUS_MEETINGS) {\n\t\tedges {\n\t\t\tuuid\n\t\t\tid\n\t\t\tjoinUrl\n\t\t}\n\t}\n}\n`})}),`\n`,(0,t.jsxs)(e.ol,{start:\"12\",children:[`\n`,(0,t.jsxs)(e.li,{children:[\"Choose \",(0,t.jsx)(e.code,{children:\"POST\"}),\", enter request URL \",(0,t.jsx)(e.code,{children:\"https://zoom.us/v3/graphql\"}),\" and click \",(0,t.jsx)(e.strong,{children:\"Send\"}),\".\"]}),`\n`,(0,t.jsx)(e.li,{children:\"If you've had previous meetings on this account, you should get a response like the following:\"}),`\n`]}),`\n`,(0,t.jsx)(e.pre,{children:(0,t.jsx)(e.code,{className:\"language-json\",children:`{\n\t\"data\": {\n\t\t\"meetings\": {\n\t\t\t\"edges\": [\n\t\t\t\t{\n\t\t\t\t\t\"uuid\": \"AAABBBCCCDDDEEEFFFGGGH==\",\n\t\t\t\t\t\"id\": 11111111111,\n\t\t\t\t\t\"joinUrl\": \"https://link.example.com/a/11111111111\"\n\t\t\t\t},\n\t\t\t\t{\n\t\t\t\t\t\"uuid\": \"1112223334445556667778==\",\n\t\t\t\t\t\"id\": 22222222222,\n\t\t\t\t\t\"joinUrl\": \"https://link.example.com/a/22222222222?pwd=000000000000000000000000000000.0\"\n\t\t\t\t},\n\t\t\t\t{\n\t\t\t\t\t\"uuid\": \"+++===+++===+++===+++w+==\",\n\t\t\t\t\t\"id\": 33333333333,\n\t\t\t\t\t\"joinUrl\": \"https://link.example.com/a/33333333333\"\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t}\n}\n`})}),`\n`,(0,t.jsx)(e.h2,{id:\"learn-more\",children:(0,t.jsxs)(e.a,{href:\"#learn-more\",children:[\"Learn more\",(0,t.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,t.jsxs)(e.ul,{children:[`\n`,(0,t.jsxs)(e.li,{children:[\"See \",(0,t.jsx)(e.strong,{children:\"Using GraphQL\"}),\" for more details about authentication, queries, mutations, and pagination.\"]}),`\n`,(0,t.jsxs)(e.li,{children:[\"See \",(0,t.jsx)(e.strong,{children:\"Postman samples\"}),\" for information about the Zoom GraphQL public workspace on Postman and how to add the Zoom GraphQL schema to Postman.\"]}),`\n`,(0,t.jsxs)(e.li,{children:[\"See \",(0,t.jsx)(e.strong,{children:\"Scopes\"}),\" and \",(0,t.jsx)(e.strong,{children:\"Rate limits\"}),\" for tables showing these affect GraphQL queries and mutations.\"]}),`\n`,(0,t.jsxs)(e.li,{children:[\"Use the \",(0,t.jsx)(e.a,{href:\"https://nws.zoom.us/graphql/playground\",children:\"Zoom GraphQL API playground\"}),\" to send queries and mutations directly in your web browser. Access the GraphQL reference documentation and autocomplete JSON code directly in the playground. See the \",(0,t.jsx)(e.a,{href:\"https://dev.to/zoom/the-zoom-graphql-api-playground-your-new-favorite-development-tool-59n5\",children:\"Zoom Dev article\"}),\" to learn more.\"]}),`\n`]})]})}function p(n={}){let{wrapper:e}={...a(),...n.components};return e?(0,t.jsx)(e,{...n,children:(0,t.jsx)(s,{...n})}):s(n)}function v(n,e){throw new Error(\"Expected \"+(e?\"component\":\"object\")+\" `\"+n+\"` to be defined: you likely forgot to import, pass, or provide it.\")}return k(L);})();\n;return Component;",
      "frontmatter": {
        "title": "Get started with GraphQL"
      },
      "errors": [],
      "matter": {
        "content": "\n# Get started with GraphQL\n\n<Alert variant=\"warning\">\n\n**Beta**\n\nZoom GraphQL is in a public beta. \n* See the [GraphQL beta FAQ](/docs/api/graphql/beta-faq) for details.\n* **Get started using the [Zoom GraphQL API playground](https://nws.zoom.us/graphql/playground).** \n* See the [Zoom GraphQL API playground](/blog/graphql-api-playground) blog post for details.\n\n</Alert>\n\n<Alert variant=\"primary\">\n\n**GraphQL endpoint**\n\nZoom GraphQL is available at the following endpoint: `https://api.zoom.us/v3/graphql`\n\n</Alert>\n\n## Make your first GraphQL query\n\nFollow these steps to make your first GraphQL query in Postman, to get the `uuid`, `id`, and `joinUrl` for your previous meetings.\n\n### Before you start\n\n1. Log on to your Marketplace account (be sure to use an account that you've used for some meetings in the past).\n2. Create a [Server-to-Server OAuth](/docs/internal-apps/create) app type.\n3. Add the scope to **View all user meetings**: `meeting:read:admin`\n\n### Create an authorization token\n\n4. In the Postman **Params** tab, add the following keys and values:\n\nKey | Value\n--- | ---\n`grant_type` | `account_credentials`\n`account_id` | Your **Account ID**, shown in the App credentials tab for your app in Marketplace (use a [Postman variable](https://learning.postman.com/docs/sending-requests/variables/) for this and other credentials).\n\n5. In the Postman **Authorization** tab, enter your app's **Client ID** as the Username and **Client secret** as the Password.\n6. Choose `POST`, enter request URL `https://zoom.us/oauth/token` and click **Send**.\n7. Copy your **access token**.\n\n### Query meetings\n\n8. Create a new request and click the Postman **Authorization** tab.\n9. In Type, choose **Bearer Token**.\n10. Enter your **access token** (use a [variable](https://learning.postman.com/docs/sending-requests/variables/)).\n11. Click **Body**, choose **GraphQL**, and enter the following test query:\n\n```json\n{\n\tmeetings (meetingType: PREVIOUS_MEETINGS) {\n\t\tedges {\n\t\t\tuuid\n\t\t\tid\n\t\t\tjoinUrl\n\t\t}\n\t}\n}\n```\n\n12. Choose `POST`, enter request URL `https://zoom.us/v3/graphql` and click **Send**.\n12. If you've had previous meetings on this account, you should get a response like the following:\n\n```json\n{\n\t\"data\": {\n\t\t\"meetings\": {\n\t\t\t\"edges\": [\n\t\t\t\t{\n\t\t\t\t\t\"uuid\": \"AAABBBCCCDDDEEEFFFGGGH==\",\n\t\t\t\t\t\"id\": 11111111111,\n\t\t\t\t\t\"joinUrl\": \"https://link.example.com/a/11111111111\"\n\t\t\t\t},\n\t\t\t\t{\n\t\t\t\t\t\"uuid\": \"1112223334445556667778==\",\n\t\t\t\t\t\"id\": 22222222222,\n\t\t\t\t\t\"joinUrl\": \"https://link.example.com/a/22222222222?pwd=000000000000000000000000000000.0\"\n\t\t\t\t},\n\t\t\t\t{\n\t\t\t\t\t\"uuid\": \"+++===+++===+++===+++w+==\",\n\t\t\t\t\t\"id\": 33333333333,\n\t\t\t\t\t\"joinUrl\": \"https://link.example.com/a/33333333333\"\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t}\n}\n```\n\n## Learn more\n\n* See **Using GraphQL** for more details about authentication, queries, mutations, and pagination.\n* See **Postman samples** for information about the Zoom GraphQL public workspace on Postman and how to add the Zoom GraphQL schema to Postman.\n* See **Scopes** and **Rate limits** for tables showing these affect GraphQL queries and mutations.\n* Use the [Zoom GraphQL API playground](https://nws.zoom.us/graphql/playground) to send queries and mutations directly in your web browser. Access the GraphQL reference documentation and autocomplete JSON code directly in the playground. See the [Zoom Dev article](https://dev.to/zoom/the-zoom-graphql-api-playground-your-new-favorite-development-tool-59n5) to learn more.\n",
        "data": {
          "title": "Get started with GraphQL"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/graphql/get-started.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
