# Chat API

- Source URL: https://developers.zoom.us/docs/api/rest/chat-api/
- Snapshot path: docs/api/rest/chat-api
- Generated (UTC): 2026-02-07T22:40:11.522574+00:00

## Frontmatter

```json
{
  "title": "Chat API"
}
```

## Content

```md

# Zoom Chat API

Developers can use the Zoom Chat API to access chat and chat channel data to build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us). 

### Before you begin
- Learn the [basics](/docs/api/using-zoom-apis/) on how to build with Zoom APIs.

- Use our [Postman Public Workspace](https://www.postman.com/zoom-developer) to try [Zoom Team Chat APIs](/docs/api/team-chat). You must have a Pro or a higher [plan](https://zoom.us/pricing).

- Learn about [webhooks](/docs/api/webhooks/) and how to get data delivered to your designated URL.


## Use cases
Team Chat APIs facilitate communication and collaboration for teams or groups. 

This table describes some common use cases for Zoom Chat APIs. Note that specific use cases can vary based on the capabilities of the Team Chat API or the platform you use. 

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
       <p> Integrating with third-party apps </p>
      </td>
      <td>
        
         Users can connect with other tools and applications to consolidate information and workflows.
        
        <br />
      </td>
    </tr>
    <tr>
      <td>
        <p> Integrating chatbots </p>
       
      </td>
      <td>
        Users can add chatbots to automate repetitive tasks, answer common queries, and improve the efficiency of  workflows.
      </td>
    </tr>
    <tr>
      <td>
      <p>  Automating notifications and alerts </p>
        <br />
      </td>
      <td>
        Users can send automated notifications and alerts to inform the team about import updates.
      </td>
    </tr>
    <tr>
      <td>
       <p>  Sharing files </p>
      </td>
      <td>
        Users can share files, images, and other content to enhance the workflow among team members.
      </td>
    </tr>
    <tr>
      <td>
        <p> Managing collaborative tasks   </p>
      </td>
      <td>
        Users can discuss tasks, assign responsibilities, and track progress within the chat environment.
      </td>
    </tr>
     <tr>
      <td>
        <p> Communicating in real-time</p>
      </td>
      <td>
        Enable real-time messaging and chat to  communicate with team members to collaborate efficiently.

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
      "code": "var Component=(()=>{var m=Object.create;var o=Object.defineProperty;var u=Object.getOwnPropertyDescriptor;var f=Object.getOwnPropertyNames;var g=Object.getPrototypeOf,b=Object.prototype.hasOwnProperty;var w=(i,n)=>()=>(n||i((n={exports:{}}).exports,n),n.exports),v=(i,n)=>{for(var r in n)o(i,r,{get:n[r],enumerable:!0})},d=(i,n,r,c)=>{if(n&&typeof n==\"object\"||typeof n==\"function\")for(let t of f(n))!b.call(i,t)&&t!==r&&o(i,t,{get:()=>n[t],enumerable:!(c=u(n,t))||c.enumerable});return i};var A=(i,n,r)=>(r=i!=null?m(g(i)):{},d(n||!i||!i.__esModule?o(r,\"default\",{value:i,enumerable:!0}):r,i)),x=i=>d(o({},\"__esModule\",{value:!0}),i);var s=w((k,h)=>{h.exports=_jsx_runtime});var C={};v(C,{default:()=>p,frontmatter:()=>y});var e=A(s());var{useMDXComponents:a}=MdxJsReact;var y={title:\"Chat API\"};function l(i){let n={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",p:\"p\",strong:\"strong\",ul:\"ul\",...a(),...i.components};return(0,e.jsxs)(e.Fragment,{children:[(0,e.jsx)(n.h1,{id:\"zoom-chat-api\",children:\"Zoom Chat API\"}),`\n`,(0,e.jsxs)(n.p,{children:[\"Developers can use the Zoom Chat API to access chat and chat channel data to build private services or public applications on the \",(0,e.jsx)(n.a,{href:\"http://marketplace.zoom.us\",children:\"Zoom App Marketplace\"}),\".\"]}),`\n`,(0,e.jsx)(n.h3,{id:\"before-you-begin\",children:(0,e.jsxs)(n.a,{href:\"#before-you-begin\",children:[\"Before you begin\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Learn the \",(0,e.jsx)(n.a,{href:\"/docs/api/using-zoom-apis/\",children:\"basics\"}),\" on how to build with Zoom APIs.\"]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Use our \",(0,e.jsx)(n.a,{href:\"https://www.postman.com/zoom-developer\",children:\"Postman Public Workspace\"}),\" to try \",(0,e.jsx)(n.a,{href:\"/docs/api/team-chat\",children:\"Zoom Team Chat APIs\"}),\". You must have a Pro or a higher \",(0,e.jsx)(n.a,{href:\"https://zoom.us/pricing\",children:\"plan\"}),\".\"]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Learn about \",(0,e.jsx)(n.a,{href:\"/docs/api/webhooks/\",children:\"webhooks\"}),\" and how to get data delivered to your designated URL.\"]}),`\n`]}),`\n`]}),`\n`,(0,e.jsx)(n.h2,{id:\"use-cases\",children:(0,e.jsxs)(n.a,{href:\"#use-cases\",children:[\"Use cases\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"Team Chat APIs facilitate communication and collaboration for teams or groups.\"}),`\n`,(0,e.jsx)(n.p,{children:\"This table describes some common use cases for Zoom Chat APIs. Note that specific use cases can vary based on the capabilities of the Team Chat API or the platform you use.\"}),`\n`,(0,e.jsxs)(\"table\",{children:[(0,e.jsx)(\"thead\",{children:(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"th\",{children:(0,e.jsx)(\"p\",{children:(0,e.jsx)(n.strong,{children:\"Use case\"})})}),(0,e.jsx)(\"th\",{children:(0,e.jsx)(\"p\",{children:(0,e.jsx)(n.strong,{children:\"Outcome\"})})})]})}),(0,e.jsxs)(\"tbody\",{children:[(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"td\",{children:(0,e.jsx)(\"p\",{children:\" Integrating with third-party apps \"})}),(0,e.jsxs)(\"td\",{children:[(0,e.jsx)(n.p,{children:\"Users can connect with other tools and applications to consolidate information and workflows.\"}),(0,e.jsx)(\"br\",{})]})]}),(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"td\",{children:(0,e.jsx)(\"p\",{children:\" Integrating chatbots \"})}),(0,e.jsx)(\"td\",{children:(0,e.jsx)(n.p,{children:\"Users can add chatbots to automate repetitive tasks, answer common queries, and improve the efficiency of  workflows.\"})})]}),(0,e.jsxs)(\"tr\",{children:[(0,e.jsxs)(\"td\",{children:[(0,e.jsx)(\"p\",{children:\"  Automating notifications and alerts \"}),(0,e.jsx)(\"br\",{})]}),(0,e.jsx)(\"td\",{children:(0,e.jsx)(n.p,{children:\"Users can send automated notifications and alerts to inform the team about import updates.\"})})]}),(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"td\",{children:(0,e.jsx)(\"p\",{children:\"  Sharing files \"})}),(0,e.jsx)(\"td\",{children:(0,e.jsx)(n.p,{children:\"Users can share files, images, and other content to enhance the workflow among team members.\"})})]}),(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"td\",{children:(0,e.jsx)(\"p\",{children:\" Managing collaborative tasks   \"})}),(0,e.jsx)(\"td\",{children:(0,e.jsx)(n.p,{children:\"Users can discuss tasks, assign responsibilities, and track progress within the chat environment.\"})})]}),(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"td\",{children:(0,e.jsx)(\"p\",{children:\" Communicating in real-time\"})}),(0,e.jsx)(\"td\",{children:(0,e.jsx)(n.p,{children:\"Enable real-time messaging and chat to  communicate with team members to collaborate efficiently.\"})})]})]})]}),`\n`,(0,e.jsx)(n.h2,{id:\"endpoints\",children:(0,e.jsxs)(n.a,{href:\"#endpoints\",children:[\"Endpoints\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.p,{children:[\"All endpoints are available through \",(0,e.jsx)(n.code,{children:\"https\"}),\" at \",(0,e.jsx)(n.code,{children:\"api.zoom.us/v2/\"}),\".\"]}),`\n`,(0,e.jsx)(n.h2,{id:\"resources-and-information\",children:(0,e.jsxs)(n.a,{href:\"#resources-and-information\",children:[\"Resources and information\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.a,{href:\"/docs/integrations/oauth/\",children:(0,e.jsx)(n.strong,{children:\"OAuth 2.0 for Zoom\"})}),\" \",(0,e.jsx)(\"br\",{}),`\nHow to get your credentials and create private or public applications`]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Version\"}),\" \",(0,e.jsx)(\"br\",{}),`\n2.0.0`]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Host\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"api.zoom.us/v2\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Protocols\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"https\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Accepts\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"application/json\"}),\", \",(0,e.jsx)(n.code,{children:\"multipart/form-data\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Responds With\"}),(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"application/json\"}),\", \",(0,e.jsx)(n.code,{children:\"application/xml\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Zoom API License and Terms of Use\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"https://explore.zoom.us/en/legal/zoom-api-license-and-tou/\"})]}),`\n`]})]})}function p(i={}){let{wrapper:n}={...a(),...i.components};return n?(0,e.jsx)(n,{...i,children:(0,e.jsx)(l,{...i})}):l(i)}return x(C);})();\n;return Component;",
      "frontmatter": {
        "title": "Chat API"
      },
      "errors": [],
      "matter": {
        "content": "\n# Zoom Chat API\n\nDevelopers can use the Zoom Chat API to access chat and chat channel data to build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us). \n\n### Before you begin\n- Learn the [basics](/docs/api/using-zoom-apis/) on how to build with Zoom APIs.\n\n- Use our [Postman Public Workspace](https://www.postman.com/zoom-developer) to try [Zoom Team Chat APIs](/docs/api/team-chat). You must have a Pro or a higher [plan](https://zoom.us/pricing).\n\n- Learn about [webhooks](/docs/api/webhooks/) and how to get data delivered to your designated URL.\n\n\n## Use cases\nTeam Chat APIs facilitate communication and collaboration for teams or groups. \n\nThis table describes some common use cases for Zoom Chat APIs. Note that specific use cases can vary based on the capabilities of the Team Chat API or the platform you use. \n\n<table>\n  <thead>\n    <tr>\n      <th><p>**Use case**</p></th>\n      <th><p>**Outcome**</p></th>\n    </tr>\n  </thead>\n  <tbody>\n    <tr>\n      <td>\n       <p> Integrating with third-party apps </p>\n      </td>\n      <td>\n        \n         Users can connect with other tools and applications to consolidate information and workflows.\n        \n        <br />\n      </td>\n    </tr>\n    <tr>\n      <td>\n        <p> Integrating chatbots </p>\n       \n      </td>\n      <td>\n        Users can add chatbots to automate repetitive tasks, answer common queries, and improve the efficiency of  workflows.\n      </td>\n    </tr>\n    <tr>\n      <td>\n      <p>  Automating notifications and alerts </p>\n        <br />\n      </td>\n      <td>\n        Users can send automated notifications and alerts to inform the team about import updates.\n      </td>\n    </tr>\n    <tr>\n      <td>\n       <p>  Sharing files </p>\n      </td>\n      <td>\n        Users can share files, images, and other content to enhance the workflow among team members.\n      </td>\n    </tr>\n    <tr>\n      <td>\n        <p> Managing collaborative tasks   </p>\n      </td>\n      <td>\n        Users can discuss tasks, assign responsibilities, and track progress within the chat environment.\n      </td>\n    </tr>\n     <tr>\n      <td>\n        <p> Communicating in real-time</p>\n      </td>\n      <td>\n        Enable real-time messaging and chat to  communicate with team members to collaborate efficiently.\n\n      </td>\n    </tr>\n  </tbody>\n</table>\n\n## Endpoints\nAll endpoints are available through `https` at `api.zoom.us/v2/`.  \n\n## Resources and information\n- [**OAuth 2.0 for Zoom**](/docs/integrations/oauth/) <br/>\nHow to get your credentials and create private or public applications\n- **Version** <br/>\n  2.0.0\n- **Host** <br/>\n `api.zoom.us/v2`\n- **Protocols** <br/>\n  `https`\n- **Accepts** <br/>\n `application/json`, `multipart/form-data`\n- **Responds With**<br/>\n  `application/json`, `application/xml`\n- **Zoom API License and Terms of Use** <br/>\n `https://explore.zoom.us/en/legal/zoom-api-license-and-tou/`\n  \n\n\n\n\n\n\n\n  \n\n\n\n\n\n\n\n\n",
        "data": {
          "title": "Chat API"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/rest/chat-api.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
