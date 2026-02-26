# Zoom Phone API

- Source URL: https://developers.zoom.us/docs/api/rest/zoom-phone-api/
- Snapshot path: docs/api/rest/zoom-phone-api
- Generated (UTC): 2026-02-07T22:40:11.534495+00:00

## Frontmatter

```json
{
  "title": "Zoom Phone API"
}
```

## Content

```md

# Zoom Phone API

You can use Zoom Phone APIs to integrate Zoom Phone into third-party applications or services, and build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us). 

### Before you begin
- Learn the [basics](/docs/api/using-zoom-apis/) on how to build with Zoom APIs.

- Use our [Postman Public Workspace](https://www.postman.com/zoom-developer) to try [Zoom Phone APIs](/docs/api/phone). You must have a Pro or a higher plan with a [Zoom Phone plan](https://zoom.us/pricing/zoom-phone).

- Learn about [webhooks](/docs/api/webhooks/) and how to get data delivered to your designated URL.


## Use cases
Zoom Phone APIs empower developers to create customized solutions that leverage the functionality of Zoom Phone. 

This table describes some common use cases for Zoom Phone APIs.

<table>
  <thead>
    <tr>
      <th><p>**Use case**</p></th>
      <th><p>**Outcome**</p></th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>
       <p> Integrating Zoom Phone features into existing applications or services. </p>
      </td>
      <td>
        
         Users can make and receive calls, manage voicemails, and perform other tasks in third-party applications or services.
        
        <br />
      </td>
    </tr>
    <tr>
      <td>
        <p> Customizing user interfaces with Zoom Phone features. </p>
       
      </td>
      <td>
        Users can experience Zoom Phone features in their specific business environment.
      </td>
    </tr>
    <tr>
      <td>
      <p>  Implementing user authentication and authorization processes. </p>
        <br />
      </td>
      <td>
        Users can enjoy secure access to phone features and data.
      </td>
    </tr>
    <tr>
      <td>
       <p>  Automating business phone workflows with Zoom Phone APIs. </p>
      </td>
      <td>
        Businesses can automatically log call data, initiate actions based on call events, or integrate phone functionality into their workflow processes.
      </td>
    </tr>
    <tr>
      <td>
        <p> Accessing call data and generating customized reports and analytics.  </p>
      </td>
      <td>
        Businesses can analyze communication patterns and monitor call performance to make informed decisions.
      </td>
    </tr>
     <tr>
      <td>
        <p> Integrating Customer Relationship Management (CRM) systems with Zoom Phone to connect between phone communication and customer data.</p>
      </td>
      <td>
        Users gain greater context and clarity during business calls.

      </td>
    </tr>
  </tbody>
</table>

## Endpoints
All endpoints are available through `https` at `api.zoom.us/v2/`. For instance, `https://api.zoom.us/v2/users/` returns all users on an account. If the phone account is not set up, you receive this `403` error response: `Zoom Phone has not been enabled for this account`.

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
      "code": "var Component=(()=>{var u=Object.create;var t=Object.defineProperty;var m=Object.getOwnPropertyDescriptor;var g=Object.getOwnPropertyNames;var f=Object.getPrototypeOf,b=Object.prototype.hasOwnProperty;var P=(o,n)=>()=>(n||o((n={exports:{}}).exports,n),n.exports),v=(o,n)=>{for(var i in n)t(o,i,{get:n[i],enumerable:!0})},a=(o,n,i,s)=>{if(n&&typeof n==\"object\"||typeof n==\"function\")for(let r of g(n))!b.call(o,r)&&r!==i&&t(o,r,{get:()=>n[r],enumerable:!(s=m(n,r))||s.enumerable});return o};var w=(o,n,i)=>(i=o!=null?u(f(o)):{},a(n||!o||!o.__esModule?t(i,\"default\",{value:o,enumerable:!0}):i,o)),y=o=>a(t({},\"__esModule\",{value:!0}),o);var h=P((A,d)=>{d.exports=_jsx_runtime});var x={};v(x,{default:()=>p,frontmatter:()=>Z});var e=w(h());var{useMDXComponents:c}=MdxJsReact;var Z={title:\"Zoom Phone API\"};function l(o){let n={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",p:\"p\",strong:\"strong\",ul:\"ul\",...c(),...o.components};return(0,e.jsxs)(e.Fragment,{children:[(0,e.jsx)(n.h1,{id:\"zoom-phone-api\",children:\"Zoom Phone API\"}),`\n`,(0,e.jsxs)(n.p,{children:[\"You can use Zoom Phone APIs to integrate Zoom Phone into third-party applications or services, and build private services or public applications on the \",(0,e.jsx)(n.a,{href:\"http://marketplace.zoom.us\",children:\"Zoom App Marketplace\"}),\".\"]}),`\n`,(0,e.jsx)(n.h3,{id:\"before-you-begin\",children:(0,e.jsxs)(n.a,{href:\"#before-you-begin\",children:[\"Before you begin\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Learn the \",(0,e.jsx)(n.a,{href:\"/docs/api/using-zoom-apis/\",children:\"basics\"}),\" on how to build with Zoom APIs.\"]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Use our \",(0,e.jsx)(n.a,{href:\"https://www.postman.com/zoom-developer\",children:\"Postman Public Workspace\"}),\" to try \",(0,e.jsx)(n.a,{href:\"/docs/api/phone\",children:\"Zoom Phone APIs\"}),\". You must have a Pro or a higher plan with a \",(0,e.jsx)(n.a,{href:\"https://zoom.us/pricing/zoom-phone\",children:\"Zoom Phone plan\"}),\".\"]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Learn about \",(0,e.jsx)(n.a,{href:\"/docs/api/webhooks/\",children:\"webhooks\"}),\" and how to get data delivered to your designated URL.\"]}),`\n`]}),`\n`]}),`\n`,(0,e.jsx)(n.h2,{id:\"use-cases\",children:(0,e.jsxs)(n.a,{href:\"#use-cases\",children:[\"Use cases\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"Zoom Phone APIs empower developers to create customized solutions that leverage the functionality of Zoom Phone.\"}),`\n`,(0,e.jsx)(n.p,{children:\"This table describes some common use cases for Zoom Phone APIs.\"}),`\n`,(0,e.jsxs)(\"table\",{children:[(0,e.jsx)(\"thead\",{children:(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"th\",{children:(0,e.jsx)(\"p\",{children:(0,e.jsx)(n.strong,{children:\"Use case\"})})}),(0,e.jsx)(\"th\",{children:(0,e.jsx)(\"p\",{children:(0,e.jsx)(n.strong,{children:\"Outcome\"})})})]})}),(0,e.jsxs)(\"tbody\",{children:[(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"td\",{children:(0,e.jsx)(\"p\",{children:\" Integrating Zoom Phone features into existing applications or services. \"})}),(0,e.jsxs)(\"td\",{children:[(0,e.jsx)(n.p,{children:\"Users can make and receive calls, manage voicemails, and perform other tasks in third-party applications or services.\"}),(0,e.jsx)(\"br\",{})]})]}),(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"td\",{children:(0,e.jsx)(\"p\",{children:\" Customizing user interfaces with Zoom Phone features. \"})}),(0,e.jsx)(\"td\",{children:(0,e.jsx)(n.p,{children:\"Users can experience Zoom Phone features in their specific business environment.\"})})]}),(0,e.jsxs)(\"tr\",{children:[(0,e.jsxs)(\"td\",{children:[(0,e.jsx)(\"p\",{children:\"  Implementing user authentication and authorization processes. \"}),(0,e.jsx)(\"br\",{})]}),(0,e.jsx)(\"td\",{children:(0,e.jsx)(n.p,{children:\"Users can enjoy secure access to phone features and data.\"})})]}),(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"td\",{children:(0,e.jsx)(\"p\",{children:\"  Automating business phone workflows with Zoom Phone APIs. \"})}),(0,e.jsx)(\"td\",{children:(0,e.jsx)(n.p,{children:\"Businesses can automatically log call data, initiate actions based on call events, or integrate phone functionality into their workflow processes.\"})})]}),(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"td\",{children:(0,e.jsx)(\"p\",{children:\" Accessing call data and generating customized reports and analytics.  \"})}),(0,e.jsx)(\"td\",{children:(0,e.jsx)(n.p,{children:\"Businesses can analyze communication patterns and monitor call performance to make informed decisions.\"})})]}),(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"td\",{children:(0,e.jsx)(\"p\",{children:\" Integrating Customer Relationship Management (CRM) systems with Zoom Phone to connect between phone communication and customer data.\"})}),(0,e.jsx)(\"td\",{children:(0,e.jsx)(n.p,{children:\"Users gain greater context and clarity during business calls.\"})})]})]})]}),`\n`,(0,e.jsx)(n.h2,{id:\"endpoints\",children:(0,e.jsxs)(n.a,{href:\"#endpoints\",children:[\"Endpoints\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.p,{children:[\"All endpoints are available through \",(0,e.jsx)(n.code,{children:\"https\"}),\" at \",(0,e.jsx)(n.code,{children:\"api.zoom.us/v2/\"}),\". For instance, \",(0,e.jsx)(n.code,{children:\"https://api.zoom.us/v2/users/\"}),\" returns all users on an account. If the phone account is not set up, you receive this \",(0,e.jsx)(n.code,{children:\"403\"}),\" error response: \",(0,e.jsx)(n.code,{children:\"Zoom Phone has not been enabled for this account\"}),\".\"]}),`\n`,(0,e.jsx)(n.h2,{id:\"resources-and-information\",children:(0,e.jsxs)(n.a,{href:\"#resources-and-information\",children:[\"Resources and information\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.a,{href:\"/docs/integrations/oauth/\",children:(0,e.jsx)(n.strong,{children:\"OAuth 2.0 for Zoom\"})}),\" \",(0,e.jsx)(\"br\",{}),`\nHow to get your credentials and create private or public applications`]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Version\"}),\" \",(0,e.jsx)(\"br\",{}),`\n2.0.0`]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Host\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"api.zoom.us/v2\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Protocols\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"https\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Accepts\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"application/json\"}),\", \",(0,e.jsx)(n.code,{children:\"multipart/form-data\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Responds With\"}),(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"application/json\"}),\", \",(0,e.jsx)(n.code,{children:\"application/xml\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Zoom API License and Terms of Use\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"https://explore.zoom.us/en/legal/zoom-api-license-and-tou/\"})]}),`\n`]})]})}function p(o={}){let{wrapper:n}={...c(),...o.components};return n?(0,e.jsx)(n,{...o,children:(0,e.jsx)(l,{...o})}):l(o)}return y(x);})();\n;return Component;",
      "frontmatter": {
        "title": "Zoom Phone API"
      },
      "errors": [],
      "matter": {
        "content": "\n# Zoom Phone API\n\nYou can use Zoom Phone APIs to integrate Zoom Phone into third-party applications or services, and build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us). \n\n### Before you begin\n- Learn the [basics](/docs/api/using-zoom-apis/) on how to build with Zoom APIs.\n\n- Use our [Postman Public Workspace](https://www.postman.com/zoom-developer) to try [Zoom Phone APIs](/docs/api/phone). You must have a Pro or a higher plan with a [Zoom Phone plan](https://zoom.us/pricing/zoom-phone).\n\n- Learn about [webhooks](/docs/api/webhooks/) and how to get data delivered to your designated URL.\n\n\n## Use cases\nZoom Phone APIs empower developers to create customized solutions that leverage the functionality of Zoom Phone. \n\nThis table describes some common use cases for Zoom Phone APIs.\n\n<table>\n  <thead>\n    <tr>\n      <th><p>**Use case**</p></th>\n      <th><p>**Outcome**</p></th>\n    </tr>\n  </thead>\n  <tbody>\n    <tr>\n      <td>\n       <p> Integrating Zoom Phone features into existing applications or services. </p>\n      </td>\n      <td>\n        \n         Users can make and receive calls, manage voicemails, and perform other tasks in third-party applications or services.\n        \n        <br />\n      </td>\n    </tr>\n    <tr>\n      <td>\n        <p> Customizing user interfaces with Zoom Phone features. </p>\n       \n      </td>\n      <td>\n        Users can experience Zoom Phone features in their specific business environment.\n      </td>\n    </tr>\n    <tr>\n      <td>\n      <p>  Implementing user authentication and authorization processes. </p>\n        <br />\n      </td>\n      <td>\n        Users can enjoy secure access to phone features and data.\n      </td>\n    </tr>\n    <tr>\n      <td>\n       <p>  Automating business phone workflows with Zoom Phone APIs. </p>\n      </td>\n      <td>\n        Businesses can automatically log call data, initiate actions based on call events, or integrate phone functionality into their workflow processes.\n      </td>\n    </tr>\n    <tr>\n      <td>\n        <p> Accessing call data and generating customized reports and analytics.  </p>\n      </td>\n      <td>\n        Businesses can analyze communication patterns and monitor call performance to make informed decisions.\n      </td>\n    </tr>\n     <tr>\n      <td>\n        <p> Integrating Customer Relationship Management (CRM) systems with Zoom Phone to connect between phone communication and customer data.</p>\n      </td>\n      <td>\n        Users gain greater context and clarity during business calls.\n\n      </td>\n    </tr>\n  </tbody>\n</table>\n\n## Endpoints\nAll endpoints are available through `https` at `api.zoom.us/v2/`. For instance, `https://api.zoom.us/v2/users/` returns all users on an account. If the phone account is not set up, you receive this `403` error response: `Zoom Phone has not been enabled for this account`.\n\n## Resources and information\n- [**OAuth 2.0 for Zoom**](/docs/integrations/oauth/) <br/>\nHow to get your credentials and create private or public applications\n- **Version** <br/>\n  2.0.0\n- **Host** <br/>\n `api.zoom.us/v2`\n- **Protocols** <br/>\n  `https`\n- **Accepts** <br/>\n `application/json`, `multipart/form-data`\n- **Responds With**<br/>\n  `application/json`, `application/xml`\n- **Zoom API License and Terms of Use** <br/>\n `https://explore.zoom.us/en/legal/zoom-api-license-and-tou/`\n  \n\n\n\n\n\n\n\n  \n\n\n\n\n\n\n\n\n",
        "data": {
          "title": "Zoom Phone API"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/rest/zoom-phone-api.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
