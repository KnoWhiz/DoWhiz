# Zoom Clips API

- Source URL: https://developers.zoom.us/docs/api/rest/zoom-clips-api/
- Snapshot path: docs/api/rest/zoom-clips-api
- Generated (UTC): 2026-02-07T22:40:11.531985+00:00

## Frontmatter

```json
{
  "title": "Zoom Clips API"
}
```

## Content

```md

# Zoom Clips API
You can use Zoom Clips APIs to integrate Zoom Clips into third-party applications or services, and build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us/). 

To learn how to obtain credentials and develop private or public applications, see [OAuth 2.0 for Zoom](/docs/integrations/oauth/).

### Before you begin
- Learn the [basics](/docs/api/using-zoom-apis/) on how to build with Zoom APIs.

- Use our [Postman Public Workspace](https://www.postman.com/zoom-developer) to try [Zoom Clips APIs](/docs/api/clips/). You must have a [Zoom Clips add on]( https://zoom.us/pricing).
## Use cases
Zoom Clips APIs empower developers to create customized solutions that leverage the functionality of Zoom Clips. 

This table describes some common use cases for Zoom Clips APIs.

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
       <p> Use the Zoom Clips uploading API to migrate videos to Zoom Clips with one click. </p>
      </td>
      <td>
        
         Users can migrate videos from other platforms or a local computer to Zoom Clips.
        
        <br />
      </td>
    </tr>
    <tr>
      <td>
        <p> Auto upload video from a third-party application to Zoom Clips to leverage features, such as sharing, edition, reactions, and statistics. </p>
       
      </td>
      <td>
        Users can automatically sync video to Zoom Clips.
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
      "code": "var Component=(()=>{var m=Object.create;var t=Object.defineProperty;var u=Object.getOwnPropertyDescriptor;var f=Object.getOwnPropertyNames;var g=Object.getPrototypeOf,b=Object.prototype.hasOwnProperty;var v=(o,n)=>()=>(n||o((n={exports:{}}).exports,n),n.exports),C=(o,n)=>{for(var i in n)t(o,i,{get:n[i],enumerable:!0})},s=(o,n,i,l)=>{if(n&&typeof n==\"object\"||typeof n==\"function\")for(let r of f(n))!b.call(o,r)&&r!==i&&t(o,r,{get:()=>n[r],enumerable:!(l=u(n,r))||l.enumerable});return o};var Z=(o,n,i)=>(i=o!=null?m(g(o)):{},s(n||!o||!o.__esModule?t(i,\"default\",{value:o,enumerable:!0}):i,o)),A=o=>s(t({},\"__esModule\",{value:!0}),o);var a=v((w,d)=>{d.exports=_jsx_runtime});var y={};C(y,{default:()=>p,frontmatter:()=>x});var e=Z(a());var{useMDXComponents:c}=MdxJsReact;var x={title:\"Zoom Clips API\"};function h(o){let n={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",p:\"p\",strong:\"strong\",ul:\"ul\",...c(),...o.components};return(0,e.jsxs)(e.Fragment,{children:[(0,e.jsx)(n.h1,{id:\"zoom-clips-api\",children:\"Zoom Clips API\"}),`\n`,(0,e.jsxs)(n.p,{children:[\"You can use Zoom Clips APIs to integrate Zoom Clips into third-party applications or services, and build private services or public applications on the \",(0,e.jsx)(n.a,{href:\"http://marketplace.zoom.us/\",children:\"Zoom App Marketplace\"}),\".\"]}),`\n`,(0,e.jsxs)(n.p,{children:[\"To learn how to obtain credentials and develop private or public applications, see \",(0,e.jsx)(n.a,{href:\"/docs/integrations/oauth/\",children:\"OAuth 2.0 for Zoom\"}),\".\"]}),`\n`,(0,e.jsx)(n.h3,{id:\"before-you-begin\",children:(0,e.jsxs)(n.a,{href:\"#before-you-begin\",children:[\"Before you begin\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Learn the \",(0,e.jsx)(n.a,{href:\"/docs/api/using-zoom-apis/\",children:\"basics\"}),\" on how to build with Zoom APIs.\"]}),`\n`]}),`\n`,(0,e.jsxs)(n.li,{children:[`\n`,(0,e.jsxs)(n.p,{children:[\"Use our \",(0,e.jsx)(n.a,{href:\"https://www.postman.com/zoom-developer\",children:\"Postman Public Workspace\"}),\" to try \",(0,e.jsx)(n.a,{href:\"/docs/api/clips/\",children:\"Zoom Clips APIs\"}),\". You must have a \",(0,e.jsx)(n.a,{href:\"https://zoom.us/pricing\",children:\"Zoom Clips add on\"}),\".\"]}),`\n`]}),`\n`]}),`\n`,(0,e.jsx)(n.h2,{id:\"use-cases\",children:(0,e.jsxs)(n.a,{href:\"#use-cases\",children:[\"Use cases\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsx)(n.p,{children:\"Zoom Clips APIs empower developers to create customized solutions that leverage the functionality of Zoom Clips.\"}),`\n`,(0,e.jsx)(n.p,{children:\"This table describes some common use cases for Zoom Clips APIs.\"}),`\n`,(0,e.jsxs)(\"table\",{children:[(0,e.jsx)(\"thead\",{children:(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"th\",{children:(0,e.jsx)(\"p\",{children:(0,e.jsx)(n.strong,{children:\"Use case\"})})}),(0,e.jsx)(\"th\",{children:(0,e.jsx)(\"p\",{children:(0,e.jsx)(n.strong,{children:\"Outcome\"})})})]})}),(0,e.jsxs)(\"tbody\",{children:[(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"td\",{children:(0,e.jsx)(\"p\",{children:\" Use the Zoom Clips uploading API to migrate videos to Zoom Clips with one click. \"})}),(0,e.jsxs)(\"td\",{children:[(0,e.jsx)(n.p,{children:\"Users can migrate videos from other platforms or a local computer to Zoom Clips.\"}),(0,e.jsx)(\"br\",{})]})]}),(0,e.jsxs)(\"tr\",{children:[(0,e.jsx)(\"td\",{children:(0,e.jsx)(\"p\",{children:\" Auto upload video from a third-party application to Zoom Clips to leverage features, such as sharing, edition, reactions, and statistics. \"})}),(0,e.jsx)(\"td\",{children:(0,e.jsx)(n.p,{children:\"Users can automatically sync video to Zoom Clips.\"})})]})]})]}),`\n`,(0,e.jsx)(n.h2,{id:\"endpoints\",children:(0,e.jsxs)(n.a,{href:\"#endpoints\",children:[\"Endpoints\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.p,{children:[\"All endpoints are available through \",(0,e.jsx)(n.code,{children:\"https\"}),\" at \",(0,e.jsx)(n.code,{children:\"api.zoom.us/v2/\"}),\".\"]}),`\n`,(0,e.jsx)(n.h2,{id:\"resources-and-information\",children:(0,e.jsxs)(n.a,{href:\"#resources-and-information\",children:[\"Resources and information\",(0,e.jsx)(n.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(n.ul,{children:[`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.a,{href:\"/docs/integrations/oauth/\",children:(0,e.jsx)(n.strong,{children:\"OAuth 2.0 for Zoom\"})}),\" \",(0,e.jsx)(\"br\",{}),`\nHow to get your credentials and create private or public applications`]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Version\"}),\" \",(0,e.jsx)(\"br\",{}),`\n2.0.0`]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Host\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"api.zoom.us/v2\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Protocols\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"https\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Accepts\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"application/json\"}),\", \",(0,e.jsx)(n.code,{children:\"multipart/form-data\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Responds With\"}),(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"application/json\"}),\", \",(0,e.jsx)(n.code,{children:\"application/xml\"})]}),`\n`,(0,e.jsxs)(n.li,{children:[(0,e.jsx)(n.strong,{children:\"Zoom API License and Terms of Use\"}),\" \",(0,e.jsx)(\"br\",{}),`\n`,(0,e.jsx)(n.code,{children:\"https://explore.zoom.us/en/legal/zoom-api-license-and-tou/\"})]}),`\n`]})]})}function p(o={}){let{wrapper:n}={...c(),...o.components};return n?(0,e.jsx)(n,{...o,children:(0,e.jsx)(h,{...o})}):h(o)}return A(y);})();\n;return Component;",
      "frontmatter": {
        "title": "Zoom Clips API"
      },
      "errors": [],
      "matter": {
        "content": "\n# Zoom Clips API\nYou can use Zoom Clips APIs to integrate Zoom Clips into third-party applications or services, and build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us/). \n\nTo learn how to obtain credentials and develop private or public applications, see [OAuth 2.0 for Zoom](/docs/integrations/oauth/).\n\n### Before you begin\n- Learn the [basics](/docs/api/using-zoom-apis/) on how to build with Zoom APIs.\n\n- Use our [Postman Public Workspace](https://www.postman.com/zoom-developer) to try [Zoom Clips APIs](/docs/api/clips/). You must have a [Zoom Clips add on]( https://zoom.us/pricing).\n## Use cases\nZoom Clips APIs empower developers to create customized solutions that leverage the functionality of Zoom Clips. \n\nThis table describes some common use cases for Zoom Clips APIs.\n\n<table>\n  <thead>\n    <tr>\n      <th><p>**Use case**</p></th>\n      <th><p>**Outcome**</p></th>\n    </tr>\n  </thead>\n  <tbody>\n    <tr>\n      <td>\n       <p> Use the Zoom Clips uploading API to migrate videos to Zoom Clips with one click. </p>\n      </td>\n      <td>\n        \n         Users can migrate videos from other platforms or a local computer to Zoom Clips.\n        \n        <br />\n      </td>\n    </tr>\n    <tr>\n      <td>\n        <p> Auto upload video from a third-party application to Zoom Clips to leverage features, such as sharing, edition, reactions, and statistics. </p>\n       \n      </td>\n      <td>\n        Users can automatically sync video to Zoom Clips.\n      </td>\n    </tr>  \n  </tbody>\n</table>\n\n\n## Endpoints\nAll endpoints are available through `https` at `api.zoom.us/v2/`. \n\n## Resources and information\n- [**OAuth 2.0 for Zoom**](/docs/integrations/oauth/) <br/>\nHow to get your credentials and create private or public applications\n- **Version** <br/>\n  2.0.0\n- **Host** <br/>\n `api.zoom.us/v2`\n- **Protocols** <br/>\n  `https`\n- **Accepts** <br/>\n `application/json`, `multipart/form-data`\n- **Responds With**<br/>\n  `application/json`, `application/xml`\n- **Zoom API License and Terms of Use** <br/>\n `https://explore.zoom.us/en/legal/zoom-api-license-and-tou/`",
        "data": {
          "title": "Zoom Clips API"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/rest/zoom-clips-api.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
