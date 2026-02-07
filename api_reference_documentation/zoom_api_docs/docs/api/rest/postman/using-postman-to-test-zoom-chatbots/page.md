# Using Postman to Test Zoom Chat Apps

- Source URL: https://developers.zoom.us/docs/api/rest/postman/using-postman-to-test-zoom-chatbots/
- Snapshot path: docs/api/rest/postman/using-postman-to-test-zoom-chatbots
- Generated (UTC): 2026-02-07T22:40:11.528167+00:00

## Frontmatter

```json
{
  "title": "Using Postman to Test Zoom Chat Apps"
}
```

## Content

```md

# Using Postman to Test Zoom apps

<Alert>

In this article the terms "chatbot" and "chat app" are the same.

</Alert>


## Send Zoom Chat apps messages in 3 easy steps!

1. [Create a chat app on Zoom](/docs/team-chat/build-a-team-chat-app)
2. Install the Zoom Chatbot Postman Collection
3. Send Zoom Chatbot messages

## STEP 1. Create a Chatbot app on Zoom

Once you have [created a Chatbot app on the Zoom App Marketplace](/docs/team-chat/build-a-team-chat-app), go to the Local Test page by clicking on the **Local Test** link on the left side menu. Click the green **Install** button.

<Image
  src="/img/1562090869803.png"
  alt="Local Test Page"
/>

Then click the blue **Authorize** button to authorize our chatbot and install it on our Zoom Client.

<Image src="/img/1562090881329.png" alt="Postman access request" />

After you click Authorize, you will be taken to the redirect URL you provided, **keep this page open**, we will need the **code** in the URL for **STEP 2**.

> **IMPORTANT**: Save the **code** in the URL. We will need it in the **STEP 2**.

<Image src="/img/1562090892848.png" alt="Access code in URL" />

You should now see your Chatbot in Zoom Chat!

<Image src="/img/1562090898896.png" alt="Postman bot" />

## STEP 2. Use the Zoom Chatbot Postman Collection

Navigate to the [Zoom Developer Postman Workspace](https://www.postman.com/zoom-developer) and click the Zoom Meeting API Collection. Under the **Collections** tab on the left, you should see the Zoom Chatbot App Collection.

This will allow you to send Chatbot messages with one click!

<Image src="/img/zoom-chatbot-app-collection.png" alt="Zoom Chatbot Postman Collection" />

To configure the **Zoom Chatbot** collection to use your Zoom Chatbot credentials, hover over the collection and click the 3 dots. Then click **View Details**.

<Image src="/img/zoom-chatbot-app-collection-view-details.png" alt="Zoom Chatbot Postman Collection View Details" />

On the **Variables** tab in Postman, enter the **code** from the url you saved earlier into the **authorization_code** field in the **Current Value** column. Also add your Zoom Development **Client ID**, **Client Secret**, and **Redirect URL for OAuth** (all three found on your **App Credentials** page), and the Zoom Development **Bot JID** (found on your **Features** page) into the respective fields in the **Current Value** column.

<Image src="/img/zoom-chatbot-app-collection-variables.png" alt="Zoom Chatbot Postman Collection View Variables" />

Then click save (CMD S on macOS or CTRL S on Windows).

## STEP 3. Send Zoom Chatbot messages

Before you can call the Zoom API to send chat app messages, you have to get an `access_token`. Expand the collection on the left and click the **Authorize Chatbot** request. Then click the blue **Send** button.

<Alert variant="primary">

**Note** 

You may have to fork the Zoom Chatbot App Collection to your own Workspace to click the blue **Send** button.

</Alert>

This sends a POST request to [Zoom's authorize Chatbot endpoint](/docs/team-chat/installation-and-authentication).

<Image src="/img/zoom-chatbot-app-collection-access-token.png" alt="Zoom Chatbot Postman Collection View Access Token" />

Now you're ready to send Chatbot messages! Click any of the other requests on the left and click the blue Send button. This sends Chatbot messages via [Zoom's POST /im/chat/messages endpoint](/docs/team-chat/send-edit-and-delete-messages). Try them all out!

<Image src="/img/zoom-chatbot-app-collection-send-message.png" alt="Zoom Chatbot Postman Collection View Send Message" />

Here's what the **Message with Styled Text** looks like in Zoom Chat.

<Image src="/img/1562090959855.png" />

If you'd like to learn more about the different Zoom Chatbot messages, see our guide on [Customizing Messages](/docs/team-chat/customizing-messages).
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
      "code": "var Component=(()=>{var g=Object.create;var s=Object.defineProperty;var u=Object.getOwnPropertyDescriptor;var b=Object.getOwnPropertyNames;var C=Object.getPrototypeOf,f=Object.prototype.hasOwnProperty;var k=(o,e)=>()=>(e||o((e={exports:{}}).exports,e),e.exports),y=(o,e)=>{for(var a in e)s(o,a,{get:e[a],enumerable:!0})},c=(o,e,a,n)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let i of b(e))!f.call(o,i)&&i!==a&&s(o,i,{get:()=>e[i],enumerable:!(n=u(e,i))||n.enumerable});return o};var Z=(o,e,a)=>(a=o!=null?g(C(o)):{},c(e||!o||!o.__esModule?s(a,\"default\",{value:o,enumerable:!0}):a,o)),T=o=>c(s({},\"__esModule\",{value:!0}),o);var h=k((v,l)=>{l.exports=_jsx_runtime});var P={};y(P,{default:()=>p,frontmatter:()=>w});var t=Z(h());var{useMDXComponents:r}=MdxJsReact;var w={title:\"Using Postman to Test Zoom Chat Apps\"};function d(o){let e={a:\"a\",blockquote:\"blockquote\",code:\"code\",h1:\"h1\",h2:\"h2\",i:\"i\",li:\"li\",ol:\"ol\",p:\"p\",strong:\"strong\",...r(),...o.components},{Alert:a,Image:n}=e;return a||m(\"Alert\",!0),n||m(\"Image\",!0),(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(e.h1,{id:\"using-postman-to-test-zoom-apps\",children:\"Using Postman to Test Zoom apps\"}),`\n`,(0,t.jsx)(a,{children:(0,t.jsx)(e.p,{children:'In this article the terms \"chatbot\" and \"chat app\" are the same.'})}),`\n`,(0,t.jsx)(e.h2,{id:\"send-zoom-chat-apps-messages-in-3-easy-steps\",children:(0,t.jsxs)(e.a,{href:\"#send-zoom-chat-apps-messages-in-3-easy-steps\",children:[\"Send Zoom Chat apps messages in 3 easy steps!\",(0,t.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,t.jsxs)(e.ol,{children:[`\n`,(0,t.jsx)(e.li,{children:(0,t.jsx)(e.a,{href:\"/docs/team-chat/build-a-team-chat-app\",children:\"Create a chat app on Zoom\"})}),`\n`,(0,t.jsx)(e.li,{children:\"Install the Zoom Chatbot Postman Collection\"}),`\n`,(0,t.jsx)(e.li,{children:\"Send Zoom Chatbot messages\"}),`\n`]}),`\n`,(0,t.jsx)(e.h2,{id:\"step-1-create-a-chatbot-app-on-zoom\",children:(0,t.jsxs)(e.a,{href:\"#step-1-create-a-chatbot-app-on-zoom\",children:[\"STEP 1. Create a Chatbot app on Zoom\",(0,t.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,t.jsxs)(e.p,{children:[\"Once you have \",(0,t.jsx)(e.a,{href:\"/docs/team-chat/build-a-team-chat-app\",children:\"created a Chatbot app on the Zoom App Marketplace\"}),\", go to the Local Test page by clicking on the \",(0,t.jsx)(e.strong,{children:\"Local Test\"}),\" link on the left side menu. Click the green \",(0,t.jsx)(e.strong,{children:\"Install\"}),\" button.\"]}),`\n`,(0,t.jsx)(n,{src:\"/img/1562090869803.png\",alt:\"Local Test Page\"}),`\n`,(0,t.jsxs)(e.p,{children:[\"Then click the blue \",(0,t.jsx)(e.strong,{children:\"Authorize\"}),\" button to authorize our chatbot and install it on our Zoom Client.\"]}),`\n`,(0,t.jsx)(n,{src:\"/img/1562090881329.png\",alt:\"Postman access request\"}),`\n`,(0,t.jsxs)(e.p,{children:[\"After you click Authorize, you will be taken to the redirect URL you provided, \",(0,t.jsx)(e.strong,{children:\"keep this page open\"}),\", we will need the \",(0,t.jsx)(e.strong,{children:\"code\"}),\" in the URL for \",(0,t.jsx)(e.strong,{children:\"STEP 2\"}),\".\"]}),`\n`,(0,t.jsxs)(e.blockquote,{children:[`\n`,(0,t.jsxs)(e.p,{children:[(0,t.jsx)(e.strong,{children:\"IMPORTANT\"}),\": Save the \",(0,t.jsx)(e.strong,{children:\"code\"}),\" in the URL. We will need it in the \",(0,t.jsx)(e.strong,{children:\"STEP 2\"}),\".\"]}),`\n`]}),`\n`,(0,t.jsx)(n,{src:\"/img/1562090892848.png\",alt:\"Access code in URL\"}),`\n`,(0,t.jsx)(e.p,{children:\"You should now see your Chatbot in Zoom Chat!\"}),`\n`,(0,t.jsx)(n,{src:\"/img/1562090898896.png\",alt:\"Postman bot\"}),`\n`,(0,t.jsx)(e.h2,{id:\"step-2-use-the-zoom-chatbot-postman-collection\",children:(0,t.jsxs)(e.a,{href:\"#step-2-use-the-zoom-chatbot-postman-collection\",children:[\"STEP 2. Use the Zoom Chatbot Postman Collection\",(0,t.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,t.jsxs)(e.p,{children:[\"Navigate to the \",(0,t.jsx)(e.a,{href:\"https://www.postman.com/zoom-developer\",children:\"Zoom Developer Postman Workspace\"}),\" and click the Zoom Meeting API Collection. Under the \",(0,t.jsx)(e.strong,{children:\"Collections\"}),\" tab on the left, you should see the Zoom Chatbot App Collection.\"]}),`\n`,(0,t.jsx)(e.p,{children:\"This will allow you to send Chatbot messages with one click!\"}),`\n`,(0,t.jsx)(n,{src:\"/img/zoom-chatbot-app-collection.png\",alt:\"Zoom Chatbot Postman Collection\"}),`\n`,(0,t.jsxs)(e.p,{children:[\"To configure the \",(0,t.jsx)(e.strong,{children:\"Zoom Chatbot\"}),\" collection to use your Zoom Chatbot credentials, hover over the collection and click the 3 dots. Then click \",(0,t.jsx)(e.strong,{children:\"View Details\"}),\".\"]}),`\n`,(0,t.jsx)(n,{src:\"/img/zoom-chatbot-app-collection-view-details.png\",alt:\"Zoom Chatbot Postman Collection View Details\"}),`\n`,(0,t.jsxs)(e.p,{children:[\"On the \",(0,t.jsx)(e.strong,{children:\"Variables\"}),\" tab in Postman, enter the \",(0,t.jsx)(e.strong,{children:\"code\"}),\" from the url you saved earlier into the \",(0,t.jsx)(e.strong,{children:\"authorization_code\"}),\" field in the \",(0,t.jsx)(e.strong,{children:\"Current Value\"}),\" column. Also add your Zoom Development \",(0,t.jsx)(e.strong,{children:\"Client ID\"}),\", \",(0,t.jsx)(e.strong,{children:\"Client Secret\"}),\", and \",(0,t.jsx)(e.strong,{children:\"Redirect URL for OAuth\"}),\" (all three found on your \",(0,t.jsx)(e.strong,{children:\"App Credentials\"}),\" page), and the Zoom Development \",(0,t.jsx)(e.strong,{children:\"Bot JID\"}),\" (found on your \",(0,t.jsx)(e.strong,{children:\"Features\"}),\" page) into the respective fields in the \",(0,t.jsx)(e.strong,{children:\"Current Value\"}),\" column.\"]}),`\n`,(0,t.jsx)(n,{src:\"/img/zoom-chatbot-app-collection-variables.png\",alt:\"Zoom Chatbot Postman Collection View Variables\"}),`\n`,(0,t.jsx)(e.p,{children:\"Then click save (CMD S on macOS or CTRL S on Windows).\"}),`\n`,(0,t.jsx)(e.h2,{id:\"step-3-send-zoom-chatbot-messages\",children:(0,t.jsxs)(e.a,{href:\"#step-3-send-zoom-chatbot-messages\",children:[\"STEP 3. Send Zoom Chatbot messages\",(0,t.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,t.jsxs)(e.p,{children:[\"Before you can call the Zoom API to send chat app messages, you have to get an \",(0,t.jsx)(e.code,{children:\"access_token\"}),\". Expand the collection on the left and click the \",(0,t.jsx)(e.strong,{children:\"Authorize Chatbot\"}),\" request. Then click the blue \",(0,t.jsx)(e.strong,{children:\"Send\"}),\" button.\"]}),`\n`,(0,t.jsxs)(a,{variant:\"primary\",children:[(0,t.jsx)(e.p,{children:(0,t.jsx)(e.strong,{children:\"Note\"})}),(0,t.jsxs)(e.p,{children:[\"You may have to fork the Zoom Chatbot App Collection to your own Workspace to click the blue \",(0,t.jsx)(e.strong,{children:\"Send\"}),\" button.\"]})]}),`\n`,(0,t.jsxs)(e.p,{children:[\"This sends a POST request to \",(0,t.jsx)(e.a,{href:\"/docs/team-chat/installation-and-authentication\",children:\"Zoom's authorize Chatbot endpoint\"}),\".\"]}),`\n`,(0,t.jsx)(n,{src:\"/img/zoom-chatbot-app-collection-access-token.png\",alt:\"Zoom Chatbot Postman Collection View Access Token\"}),`\n`,(0,t.jsxs)(e.p,{children:[\"Now you're ready to send Chatbot messages! Click any of the other requests on the left and click the blue Send button. This sends Chatbot messages via \",(0,t.jsx)(e.a,{href:\"/docs/team-chat/send-edit-and-delete-messages\",children:\"Zoom's POST /im/chat/messages endpoint\"}),\". Try them all out!\"]}),`\n`,(0,t.jsx)(n,{src:\"/img/zoom-chatbot-app-collection-send-message.png\",alt:\"Zoom Chatbot Postman Collection View Send Message\"}),`\n`,(0,t.jsxs)(e.p,{children:[\"Here's what the \",(0,t.jsx)(e.strong,{children:\"Message with Styled Text\"}),\" looks like in Zoom Chat.\"]}),`\n`,(0,t.jsx)(n,{src:\"/img/1562090959855.png\"}),`\n`,(0,t.jsxs)(e.p,{children:[\"If you'd like to learn more about the different Zoom Chatbot messages, see our guide on \",(0,t.jsx)(e.a,{href:\"/docs/team-chat/customizing-messages\",children:\"Customizing Messages\"}),\".\"]})]})}function p(o={}){let{wrapper:e}={...r(),...o.components};return e?(0,t.jsx)(e,{...o,children:(0,t.jsx)(d,{...o})}):d(o)}function m(o,e){throw new Error(\"Expected \"+(e?\"component\":\"object\")+\" `\"+o+\"` to be defined: you likely forgot to import, pass, or provide it.\")}return T(P);})();\n;return Component;",
      "frontmatter": {
        "title": "Using Postman to Test Zoom Chat Apps"
      },
      "errors": [],
      "matter": {
        "content": "\n# Using Postman to Test Zoom apps\n\n<Alert>\n\nIn this article the terms \"chatbot\" and \"chat app\" are the same.\n\n</Alert>\n\n\n## Send Zoom Chat apps messages in 3 easy steps!\n\n1. [Create a chat app on Zoom](/docs/team-chat/build-a-team-chat-app)\n2. Install the Zoom Chatbot Postman Collection\n3. Send Zoom Chatbot messages\n\n## STEP 1. Create a Chatbot app on Zoom\n\nOnce you have [created a Chatbot app on the Zoom App Marketplace](/docs/team-chat/build-a-team-chat-app), go to the Local Test page by clicking on the **Local Test** link on the left side menu. Click the green **Install** button.\n\n<Image\n  src=\"/img/1562090869803.png\"\n  alt=\"Local Test Page\"\n/>\n\nThen click the blue **Authorize** button to authorize our chatbot and install it on our Zoom Client.\n\n<Image src=\"/img/1562090881329.png\" alt=\"Postman access request\" />\n\nAfter you click Authorize, you will be taken to the redirect URL you provided, **keep this page open**, we will need the **code** in the URL for **STEP 2**.\n\n> **IMPORTANT**: Save the **code** in the URL. We will need it in the **STEP 2**.\n\n<Image src=\"/img/1562090892848.png\" alt=\"Access code in URL\" />\n\nYou should now see your Chatbot in Zoom Chat!\n\n<Image src=\"/img/1562090898896.png\" alt=\"Postman bot\" />\n\n## STEP 2. Use the Zoom Chatbot Postman Collection\n\nNavigate to the [Zoom Developer Postman Workspace](https://www.postman.com/zoom-developer) and click the Zoom Meeting API Collection. Under the **Collections** tab on the left, you should see the Zoom Chatbot App Collection.\n\nThis will allow you to send Chatbot messages with one click!\n\n<Image src=\"/img/zoom-chatbot-app-collection.png\" alt=\"Zoom Chatbot Postman Collection\" />\n\nTo configure the **Zoom Chatbot** collection to use your Zoom Chatbot credentials, hover over the collection and click the 3 dots. Then click **View Details**.\n\n<Image src=\"/img/zoom-chatbot-app-collection-view-details.png\" alt=\"Zoom Chatbot Postman Collection View Details\" />\n\nOn the **Variables** tab in Postman, enter the **code** from the url you saved earlier into the **authorization_code** field in the **Current Value** column. Also add your Zoom Development **Client ID**, **Client Secret**, and **Redirect URL for OAuth** (all three found on your **App Credentials** page), and the Zoom Development **Bot JID** (found on your **Features** page) into the respective fields in the **Current Value** column.\n\n<Image src=\"/img/zoom-chatbot-app-collection-variables.png\" alt=\"Zoom Chatbot Postman Collection View Variables\" />\n\nThen click save (CMD S on macOS or CTRL S on Windows).\n\n## STEP 3. Send Zoom Chatbot messages\n\nBefore you can call the Zoom API to send chat app messages, you have to get an `access_token`. Expand the collection on the left and click the **Authorize Chatbot** request. Then click the blue **Send** button.\n\n<Alert variant=\"primary\">\n\n**Note** \n\nYou may have to fork the Zoom Chatbot App Collection to your own Workspace to click the blue **Send** button.\n\n</Alert>\n\nThis sends a POST request to [Zoom's authorize Chatbot endpoint](/docs/team-chat/installation-and-authentication).\n\n<Image src=\"/img/zoom-chatbot-app-collection-access-token.png\" alt=\"Zoom Chatbot Postman Collection View Access Token\" />\n\nNow you're ready to send Chatbot messages! Click any of the other requests on the left and click the blue Send button. This sends Chatbot messages via [Zoom's POST /im/chat/messages endpoint](/docs/team-chat/send-edit-and-delete-messages). Try them all out!\n\n<Image src=\"/img/zoom-chatbot-app-collection-send-message.png\" alt=\"Zoom Chatbot Postman Collection View Send Message\" />\n\nHere's what the **Message with Styled Text** looks like in Zoom Chat.\n\n<Image src=\"/img/1562090959855.png\" />\n\nIf you'd like to learn more about the different Zoom Chatbot messages, see our guide on [Customizing Messages](/docs/team-chat/customizing-messages).\n",
        "data": {
          "title": "Using Postman to Test Zoom Chat Apps"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/rest/postman/using-postman-to-test-zoom-chatbots.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
