# Zoom Virtual Agent API

- Source URL: https://developers.zoom.us/docs/api/rest/zoom-virtual-agent-api/
- Snapshot path: docs/api/rest/zoom-virtual-agent-api
- Generated (UTC): 2026-02-07T22:40:11.536684+00:00

## Frontmatter

```json
{
  "title": "Zoom Virtual Agent API"
}
```

## Content

```md

# Zoom Virtual Agent API

Zoom Virtual Agent is an AI-powered chatbot that delivers customer support, responds to inquiries, and executes tasks in a conversational style. You can use Zoom Virtual Agent APIs to integrate Zoom Virtual Agent into third-party applications or services, and build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us).

### Before you begin
- Learn the [basics](/docs/api/using-zoom-apis/) on how to build with Zoom APIs.

- Use our [Postman Public Workspace](https://www.postman.com/zoom-developer) to try [Zoom Virtual Agent APIs](/docs/api/virtual-agent). You must have a Pro or a higher plan.

- Learn about [webhooks](/docs/api/webhooks/) and how to get data delivered to your designated URL.


## Use cases
Zoom Virtual Agent APIs empower developers to create customized solutions that leverage the functionality of Zoom Virtual Agent. 

This table describes some common use cases for Zoom Virtual Agent  APIs.

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
       <p> Delivering technical support </p>
      </td>
      <td>        
         Users can get instant assistance for common issues during meetings or through chat, including troubleshooting and resource links.    
        <br />
      </td>
    </tr>
    <tr>
      <td>
        <p> Automating workflows </p>
       
      </td>
      <td>
        Users can automate repetitive tasks, such as send reminders, assign action items, or schedule follow-up meetings based on predefined criteria.
      </td>
    </tr>
    <tr>
      <td>
      <p>  Scheduling meetings </p>
        <br />
      </td>
      <td>
        Users can automate tasks, such as schedule meetings, check availability, and reserve conference rooms in Zoom.
      </td>
    </tr>
    <tr>
      <td>
       <p>  Accessing information </p>
      </td>
      <td>
        Users can quickly retrieve meeting agendas, participant lists, or previous recordings in the Zoom interface.
      </td>
    </tr>
    <tr>
      <td>
        <p>  Integrating with external systems  </p>
      </td>
      <td>
        Users can integrate with other business systems, such as CRM or project management tools, to perform actions directly in Zoom.

      </td>
    </tr>
     <tr>
      <td>
        <p> Managing events</p>
      </td>
      <td>
        Users can automate event tasks, such as registration, attendee tracking, reminders, or follow-up emails for webinars or large events.
      </td>
    </tr>
  </tbody>
</table>

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
      "code": "var Component=(()=>{var u=Object.create;var o=Object.defineProperty;var m=Object.getOwnPropertyDescriptor;var g=Object.getOwnPropertyNames;var f=Object.getPrototypeOf,b=Object.prototype.hasOwnProperty;var v=(r,n)=>()=>(n||r((n={exports:{}}).exports,n),n.exports),A=(r,n)=>{for(var i in n)o(r,i,{get:n[i],enumerable:!0})},a=(r,n,i,c)=>{if(n&&typeof n==\"object\"||typeof n==\"function\")for(let t of g(n))!b.call(r,t)&&t!==i&&o(r,t,{get:()=>n[t],enumerable:!(c=m(n,t))||c.enumerable});return r};var w=(r,n,i)=>(i=r!=null?u(f(r)):{},a(n||!r||!r.__esModule?o(i,\"default\",{value:r,enumerable:!0}):i,r)),y=r=>a(o({},\"__esModule\",{value:!0}),r);var l=v((P,d)=>{d.exports=_jsx_runtime});var x={};A(x,{default:()=>p,frontmatter:()=>Z});var e=w(l());var{useMDXComponents:s}=MdxJsReact;var Z={title:\"Zoom Virtual Agent API\"};function h(r){let n={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",p:\"p\",strong:\"strong\",ul:\"ul\",...s(),...r.components};return(0,e.jsxs)(e.Fragment,{children:[(0,e.jsx)(n.h1,{id:\"zoom-virtual-agent-api\",children:\"Zoom Virtual Agent API\"}),`\n`,(0,e.jsxs)(n.p,{children:[\"Zoom Virtual Agent is an AI-powered chatbot that delivers customer support, responds to inquiries, and executes tasks in a conversational style. You can use Zoom Virtual Agent APIs to integrate Zoom Virtual Agent into third-party applications or services, and build private services or public applications on the \",(0,e.jsx)(n.a,{href:\"http://marketplace.zoom.us\",children:\"Zoom App Marketplace\"}),\".\"]}),`\n`,(0,e.jsx)(n.h3,{id:\"before-you-begin\",children:(0,e.jsxs)(n.a,{href:\"#before-you-begin\",children:[\"Before you begin\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Learn the \",(0,e.jsx)(n.a,{href:\"/docs/api/using-zoom-apis/\",children:\"basics\"}),\" on how to build with Zoom APIs.\"]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Use our \",(0,e.jsx)(n.a,{href:\"https://www.postman.com/zoom-developer\",children:\"Postman Public Workspace\"}),\" to try \",(0,e.jsx)(n.a,{href:\"/docs/api/virtual-agent\",children:\"Zoom Virtual Agent APIs\"}),\". You must have a Pro or a higher plan.\"]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Learn about \",(0,e.jsx)(n.a,{href:\"/docs/api/webhooks/\",children:\"webhooks\"}),\" and how to get data delivered to your designated URL.\"]}),`\n`]}),`\n`]}),`\n`,(0,e.jsx)(n.h2,{id:\"use-cases\",children:(0,e.jsxs)(n.a,{href:\"#use-cases\",children:[\"Use cases\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"Zoom Virtual Agent APIs empower developers to create customized solutions that leverage the functionality of Zoom Virtual Agent.\"}),`\n`,(0,e.jsx)(n.p,{children:\"This table describes some common use cases for Zoom Virtual Agent  APIs.\"}),`\n`,(0,e.jsxs)(\"table\",{children:[(0,e.jsx)(\"thead\",{children:(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"th\",{children:(0,e.jsx)(\"p\",{children:(0,e.jsx)(n.strong,{children:\"Use case\"})})}),(0,e.jsx)(\"th\",{children:(0,e.jsx)(\"p\",{children:(0,e.jsx)(n.strong,{children:\"Outcome\"})})})]})}),(0,e.jsxs)(\"tbody\",{children:[(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"td\",{children:(0,e.jsx)(\"p\",{children:\" Delivering technical support \"})}),(0,e.jsxs)(\"td\",{children:[(0,e.jsx)(n.p,{children:\"Users can get instant assistance for common issues during meetings or through chat, including troubleshooting and resource links.\"}),(0,e.jsx)(\"br\",{})]})]}),(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"td\",{children:(0,e.jsx)(\"p\",{children:\" Automating workflows \"})}),(0,e.jsx)(\"td\",{children:(0,e.jsx)(n.p,{children:\"Users can automate repetitive tasks, such as send reminders, assign action items, or schedule follow-up meetings based on predefined criteria.\"})})]}),(0,e.jsxs)(\"tr\",{children:[(0,e.jsxs)(\"td\",{children:[(0,e.jsx)(\"p\",{children:\"  Scheduling meetings \"}),(0,e.jsx)(\"br\",{})]}),(0,e.jsx)(\"td\",{children:(0,e.jsx)(n.p,{children:\"Users can automate tasks, such as schedule meetings, check availability, and reserve conference rooms in Zoom.\"})})]}),(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"td\",{children:(0,e.jsx)(\"p\",{children:\"  Accessing information \"})}),(0,e.jsx)(\"td\",{children:(0,e.jsx)(n.p,{children:\"Users can quickly retrieve meeting agendas, participant lists, or previous recordings in the Zoom interface.\"})})]}),(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"td\",{children:(0,e.jsx)(\"p\",{children:\"  Integrating with external systems  \"})}),(0,e.jsx)(\"td\",{children:(0,e.jsx)(n.p,{children:\"Users can integrate with other business systems, such as CRM or project management tools, to perform actions directly in Zoom.\"})})]}),(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"td\",{children:(0,e.jsx)(\"p\",{children:\" Managing events\"})}),(0,e.jsx)(\"td\",{children:(0,e.jsx)(n.p,{children:\"Users can automate event tasks, such as registration, attendee tracking, reminders, or follow-up emails for webinars or large events.\"})})]})]})]}),`\n`,(0,e.jsx)(n.h2,{id:\"endpoints\",children:(0,e.jsxs)(n.a,{href:\"#endpoints\",children:[\"Endpoints\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.p,{children:[\"All endpoints are available through \",(0,e.jsx)(n.code,{children:\"https\"}),\" at \",(0,e.jsx)(n.code,{children:\"api.zoom.us/v2/\"}),\".\"]}),`\n`,(0,e.jsx)(n.h2,{id:\"resources-and-information\",children:(0,e.jsxs)(n.a,{href:\"#resources-and-information\",children:[\"Resources and information\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.a,{href:\"/docs/integrations/oauth/\",children:(0,e.jsx)(n.strong,{children:\"OAuth 2.0 for Zoom\"})}),\" \",(0,e.jsx)(\"br\",{}),`\nHow to get your credentials and create private or public applications`]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Version\"}),\" \",(0,e.jsx)(\"br\",{}),`\n2.0.0`]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Host\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"api.zoom.us/v2\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Protocols\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"https\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Accepts\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"application/json\"}),\", \",(0,e.jsx)(n.code,{children:\"multipart/form-data\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Responds With\"}),(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"application/json\"}),\", \",(0,e.jsx)(n.code,{children:\"application/xml\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Zoom API License and Terms of Use\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"https://explore.zoom.us/en/legal/zoom-api-license-and-tou/\"})]}),`\n`]})]})}function p(r={}){let{wrapper:n}={...s(),...r.components};return n?(0,e.jsx)(n,{...r,children:(0,e.jsx)(h,{...r})}):h(r)}return y(x);})();\n;return Component;",
      "frontmatter": {
        "title": "Zoom Virtual Agent API"
      },
      "errors": [],
      "matter": {
        "content": "\n# Zoom Virtual Agent API\n\nZoom Virtual Agent is an AI-powered chatbot that delivers customer support, responds to inquiries, and executes tasks in a conversational style. You can use Zoom Virtual Agent APIs to integrate Zoom Virtual Agent into third-party applications or services, and build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us).\n\n### Before you begin\n- Learn the [basics](/docs/api/using-zoom-apis/) on how to build with Zoom APIs.\n\n- Use our [Postman Public Workspace](https://www.postman.com/zoom-developer) to try [Zoom Virtual Agent APIs](/docs/api/virtual-agent). You must have a Pro or a higher plan.\n\n- Learn about [webhooks](/docs/api/webhooks/) and how to get data delivered to your designated URL.\n\n\n## Use cases\nZoom Virtual Agent APIs empower developers to create customized solutions that leverage the functionality of Zoom Virtual Agent. \n\nThis table describes some common use cases for Zoom Virtual Agent  APIs.\n\n<table>\n  <thead>\n    <tr>\n      <th><p>**Use case**</p></th>\n      <th><p>**Outcome**</p></th>\n    </tr>\n  </thead>\n  <tbody>\n    <tr>\n      <td>\n       <p> Delivering technical support </p>\n      </td>\n      <td>        \n         Users can get instant assistance for common issues during meetings or through chat, including troubleshooting and resource links.    \n        <br />\n      </td>\n    </tr>\n    <tr>\n      <td>\n        <p> Automating workflows </p>\n       \n      </td>\n      <td>\n        Users can automate repetitive tasks, such as send reminders, assign action items, or schedule follow-up meetings based on predefined criteria.\n      </td>\n    </tr>\n    <tr>\n      <td>\n      <p>  Scheduling meetings </p>\n        <br />\n      </td>\n      <td>\n        Users can automate tasks, such as schedule meetings, check availability, and reserve conference rooms in Zoom.\n      </td>\n    </tr>\n    <tr>\n      <td>\n       <p>  Accessing information </p>\n      </td>\n      <td>\n        Users can quickly retrieve meeting agendas, participant lists, or previous recordings in the Zoom interface.\n      </td>\n    </tr>\n    <tr>\n      <td>\n        <p>  Integrating with external systems  </p>\n      </td>\n      <td>\n        Users can integrate with other business systems, such as CRM or project management tools, to perform actions directly in Zoom.\n\n      </td>\n    </tr>\n     <tr>\n      <td>\n        <p> Managing events</p>\n      </td>\n      <td>\n        Users can automate event tasks, such as registration, attendee tracking, reminders, or follow-up emails for webinars or large events.\n      </td>\n    </tr>\n  </tbody>\n</table>\n\n## Endpoints\nAll endpoints are available through `https` at `api.zoom.us/v2/`. \n\n## Resources and information\n- [**OAuth 2.0 for Zoom**](/docs/integrations/oauth/) <br/>\nHow to get your credentials and create private or public applications\n- **Version** <br/>\n  2.0.0\n- **Host** <br/>\n `api.zoom.us/v2`\n- **Protocols** <br/>\n  `https`\n- **Accepts** <br/>\n `application/json`, `multipart/form-data`\n- **Responds With**<br/>\n  `application/json`, `application/xml`\n- **Zoom API License and Terms of Use** <br/>\n `https://explore.zoom.us/en/legal/zoom-api-license-and-tou/`\n  \n\n\n\n\n\n\n\n  \n\n\n\n\n\n\n\n\n",
        "data": {
          "title": "Zoom Virtual Agent API"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/rest/zoom-virtual-agent-api.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
