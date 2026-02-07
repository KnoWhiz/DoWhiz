# Privileges and Role Management

- Source URL: https://developers.zoom.us/docs/api/references/privileges/
- Snapshot path: docs/api/references/privileges
- Generated (UTC): 2026-02-07T22:40:11.519486+00:00

## Frontmatter

```json
{
  "title": "Privileges and Role Management"
}
```

## Content

```md

# Privileges and Role Management

A role can have one or more privileges assigned to it by either an account owner or someone with role management privilege. Role-based access control enables your account to have additional user roles. User roles can have a set of permissions that allows access only to the pages a user needs to view or edit.

The table below shows a list of privileges / permissions that can be set for a role:

| Permissions | Description |
| ----------- | ----------- |
| `User:Read`, `User:Edit` | View or edit user information, including the assignment of users to roles. |
| `Role:Read`, `Role:Edit` | View existing roles or create and modify user roles. |
| `Group:Read`, `Group:Edit` | View or edit Groups. All users in a group inherit permissions from the group role. |
| `AccountProfile:Read`, `AccountProfile:Edit` | View or edit settings in the account profile. |
| `AccountSetting:Read`, `AccountSetting:Edit` | View or edit settings that affect all users in the account. |
| `WebinarSetting:Read`, `WebinarSetting:Edit` | View or edit settings that affect all webinars scheduled by users in the account. |
| `Recording:Read`, `Recording:Edit` | View or edit information about recordings of meetings hosted by users in account. |
| `RecordingContent:Read` | View, download, play, and share links to any recording content in the account. |
| `SubAccount:Read`, `SubAccount:Edit` | View or edit settings for sub accounts. |
| `ZoomRooms:Read`, `ZoomRooms:Edit` | View or edit account-level settings for Zoom Rooms. |
| `DigitalSignage:Read`, `DigitalSignage:Edit` | View or edit digital signage for the account. |
| `IMChatHistory:Read` | View archived chat messages in the account. |
| `IMGroups:Read`, `IMGroups:Edit` | View or edit IM groups for the account. |
| `IMSetting:Read,`, `IMSetting:Edit` | View or edit IM settings that affect all users in the account. |
| `IMChatBot:Edit` | Send message by IM bot. |
| `BillingSubscription:Read`, `BillingSubscription:Edit` | View your current plan or purchase additional subscriptions and add-ons. |
| `BillingInformation:Read`, `BillingInformation:Edit` | View your invoice history or edit your billing information. |
| `Dashboard:Read` | View usage statistics for the account. |
| `UsageReport:Read` | View detailed usage reports for the account. |
| `UserActivitiesReport:Read` | View detailed user activity reports that can be used for audits. |
| `ScheduleTrackingFields:Read`, `ScheduleTrackingFields:Edit` | View or edit fields that can be used to analyze meeting usage. |
| `PbxAdmin:Read`, `PbxAdmin:Edit` | View or manage Zoom Phone settings for the account. |
| `ZoomDevelopers:Read`, `ZoomDevelopers:Edit` | Get developer privilege to view app settings or build and submit apps on the [Zoom App Marketplace.](https://marketplace.zoom.us/) |
| `RoomConnector:Read`, `RoomConnector:Edit` | View or manage H.323 devices / SIP room connector. |
| `SubAccount:Read`, `SubAccount:Edit` | View or manage [Meeting Connector](https://support.zoom.us/hc/en-us/sections/200305473-Meeting-Connector?flash_digest=a750f74cbf8803f8d5c4c1fded0519fbe01b02a9). |
| `CrossHybrid:Read`, `CrossHybrid:Edit` | View proxy Zone Controller settings or set up and deploy [Meeting Connector with Zone Controllers.](https://support.zoom.us/hc/en-us/articles/360000093183-Deploying-Meeting-Connector-with-Multiple-Zones) |
| `LyncConnector:Read`, `LyncConnector:Edit` | View settings for or manage Skype for Business (Lync) connector. |
| `ThirdPartyConference:Read`, `ThirdPartyConference:Edit` | View or manage settings for Telephony Service Provider. |
| `Branding:Read`, `Branding:Edit` | View or customize styling of your landing page when you have an approved vanity URL. |
| `SingleSignOn:Read`, `SingleSignOn:Edit` | View or edit Single Sign-On settings. |
| `Integration:Read`, `Integration:Edit` | View or manage app integrations for an account. |
| `MarketPlace:Read`, `MarketPlace:Edit` | View or manage app pre-approval admin settings in the Marketplace. |
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
      "code": "var Component=(()=>{var u=Object.create;var t=Object.defineProperty;var p=Object.getOwnPropertyDescriptor;var m=Object.getOwnPropertyNames;var f=Object.getPrototypeOf,R=Object.prototype.hasOwnProperty;var w=(i,e)=>()=>(e||i((e={exports:{}}).exports,e),e.exports),b=(i,e)=>{for(var n in e)t(i,n,{get:e[n],enumerable:!0})},l=(i,e,n,o)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let r of m(e))!R.call(i,r)&&r!==n&&t(i,r,{get:()=>e[r],enumerable:!(o=p(e,r))||o.enumerable});return i};var y=(i,e,n)=>(n=i!=null?u(f(i)):{},l(e||!i||!i.__esModule?t(n,\"default\",{value:i,enumerable:!0}):n,i)),V=i=>l(t({},\"__esModule\",{value:!0}),i);var a=w((S,h)=>{h.exports=_jsx_runtime});var E={};b(E,{default:()=>g,frontmatter:()=>v});var d=y(a());var{useMDXComponents:c}=MdxJsReact;var v={title:\"Privileges and Role Management\"};function s(i){let e={a:\"a\",code:\"code\",h1:\"h1\",p:\"p\",table:\"table\",tbody:\"tbody\",td:\"td\",th:\"th\",thead:\"thead\",tr:\"tr\",...c(),...i.components};return(0,d.jsxs)(d.Fragment,{children:[(0,d.jsx)(e.h1,{id:\"privileges-and-role-management\",children:\"Privileges and Role Management\"}),`\n`,(0,d.jsx)(e.p,{children:\"A role can have one or more privileges assigned to it by either an account owner or someone with role management privilege. Role-based access control enables your account to have additional user roles. User roles can have a set of permissions that allows access only to the pages a user needs to view or edit.\"}),`\n`,(0,d.jsx)(e.p,{children:\"The table below shows a list of privileges / permissions that can be set for a role:\"}),`\n`,(0,d.jsxs)(e.table,{children:[(0,d.jsx)(e.thead,{children:(0,d.jsxs)(e.tr,{children:[(0,d.jsx)(e.th,{children:\"Permissions\"}),(0,d.jsx)(e.th,{children:\"Description\"})]})}),(0,d.jsxs)(e.tbody,{children:[(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"User:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"User:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or edit user information, including the assignment of users to roles.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"Role:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"Role:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View existing roles or create and modify user roles.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"Group:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"Group:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or edit Groups. All users in a group inherit permissions from the group role.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"AccountProfile:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"AccountProfile:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or edit settings in the account profile.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"AccountSetting:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"AccountSetting:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or edit settings that affect all users in the account.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"WebinarSetting:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"WebinarSetting:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or edit settings that affect all webinars scheduled by users in the account.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"Recording:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"Recording:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or edit information about recordings of meetings hosted by users in account.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsx)(e.td,{children:(0,d.jsx)(e.code,{children:\"RecordingContent:Read\"})}),(0,d.jsx)(e.td,{children:\"View, download, play, and share links to any recording content in the account.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"SubAccount:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"SubAccount:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or edit settings for sub accounts.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"ZoomRooms:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"ZoomRooms:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or edit account-level settings for Zoom Rooms.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"DigitalSignage:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"DigitalSignage:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or edit digital signage for the account.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsx)(e.td,{children:(0,d.jsx)(e.code,{children:\"IMChatHistory:Read\"})}),(0,d.jsx)(e.td,{children:\"View archived chat messages in the account.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"IMGroups:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"IMGroups:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or edit IM groups for the account.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"IMSetting:Read,\"}),\", \",(0,d.jsx)(e.code,{children:\"IMSetting:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or edit IM settings that affect all users in the account.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsx)(e.td,{children:(0,d.jsx)(e.code,{children:\"IMChatBot:Edit\"})}),(0,d.jsx)(e.td,{children:\"Send message by IM bot.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"BillingSubscription:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"BillingSubscription:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View your current plan or purchase additional subscriptions and add-ons.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"BillingInformation:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"BillingInformation:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View your invoice history or edit your billing information.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsx)(e.td,{children:(0,d.jsx)(e.code,{children:\"Dashboard:Read\"})}),(0,d.jsx)(e.td,{children:\"View usage statistics for the account.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsx)(e.td,{children:(0,d.jsx)(e.code,{children:\"UsageReport:Read\"})}),(0,d.jsx)(e.td,{children:\"View detailed usage reports for the account.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsx)(e.td,{children:(0,d.jsx)(e.code,{children:\"UserActivitiesReport:Read\"})}),(0,d.jsx)(e.td,{children:\"View detailed user activity reports that can be used for audits.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"ScheduleTrackingFields:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"ScheduleTrackingFields:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or edit fields that can be used to analyze meeting usage.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"PbxAdmin:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"PbxAdmin:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or manage Zoom Phone settings for the account.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"ZoomDevelopers:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"ZoomDevelopers:Edit\"})]}),(0,d.jsxs)(e.td,{children:[\"Get developer privilege to view app settings or build and submit apps on the \",(0,d.jsx)(e.a,{href:\"https://marketplace.zoom.us/\",children:\"Zoom App Marketplace.\"})]})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"RoomConnector:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"RoomConnector:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or manage H.323 devices / SIP room connector.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"SubAccount:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"SubAccount:Edit\"})]}),(0,d.jsxs)(e.td,{children:[\"View or manage \",(0,d.jsx)(e.a,{href:\"https://support.zoom.us/hc/en-us/sections/200305473-Meeting-Connector?flash_digest=a750f74cbf8803f8d5c4c1fded0519fbe01b02a9\",children:\"Meeting Connector\"}),\".\"]})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"CrossHybrid:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"CrossHybrid:Edit\"})]}),(0,d.jsxs)(e.td,{children:[\"View proxy Zone Controller settings or set up and deploy \",(0,d.jsx)(e.a,{href:\"https://support.zoom.us/hc/en-us/articles/360000093183-Deploying-Meeting-Connector-with-Multiple-Zones\",children:\"Meeting Connector with Zone Controllers.\"})]})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"LyncConnector:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"LyncConnector:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View settings for or manage Skype for Business (Lync) connector.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"ThirdPartyConference:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"ThirdPartyConference:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or manage settings for Telephony Service Provider.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"Branding:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"Branding:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or customize styling of your landing page when you have an approved vanity URL.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"SingleSignOn:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"SingleSignOn:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or edit Single Sign-On settings.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"Integration:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"Integration:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or manage app integrations for an account.\"})]}),(0,d.jsxs)(e.tr,{children:[(0,d.jsxs)(e.td,{children:[(0,d.jsx)(e.code,{children:\"MarketPlace:Read\"}),\", \",(0,d.jsx)(e.code,{children:\"MarketPlace:Edit\"})]}),(0,d.jsx)(e.td,{children:\"View or manage app pre-approval admin settings in the Marketplace.\"})]})]})]})]})}function g(i={}){let{wrapper:e}={...c(),...i.components};return e?(0,d.jsx)(e,{...i,children:(0,d.jsx)(s,{...i})}):s(i)}return V(E);})();\n;return Component;",
      "frontmatter": {
        "title": "Privileges and Role Management"
      },
      "errors": [],
      "matter": {
        "content": "\r\n# Privileges and Role Management\r\n\r\nA role can have one or more privileges assigned to it by either an account owner or someone with role management privilege. Role-based access control enables your account to have additional user roles. User roles can have a set of permissions that allows access only to the pages a user needs to view or edit.\r\n\r\nThe table below shows a list of privileges / permissions that can be set for a role:\r\n\r\n| Permissions | Description |\r\n| ----------- | ----------- |\r\n| `User:Read`, `User:Edit` | View or edit user information, including the assignment of users to roles. |\r\n| `Role:Read`, `Role:Edit` | View existing roles or create and modify user roles. |\r\n| `Group:Read`, `Group:Edit` | View or edit Groups. All users in a group inherit permissions from the group role. |\r\n| `AccountProfile:Read`, `AccountProfile:Edit` | View or edit settings in the account profile. |\r\n| `AccountSetting:Read`, `AccountSetting:Edit` | View or edit settings that affect all users in the account. |\r\n| `WebinarSetting:Read`, `WebinarSetting:Edit` | View or edit settings that affect all webinars scheduled by users in the account. |\r\n| `Recording:Read`, `Recording:Edit` | View or edit information about recordings of meetings hosted by users in account. |\r\n| `RecordingContent:Read` | View, download, play, and share links to any recording content in the account. |\r\n| `SubAccount:Read`, `SubAccount:Edit` | View or edit settings for sub accounts. |\r\n| `ZoomRooms:Read`, `ZoomRooms:Edit` | View or edit account-level settings for Zoom Rooms. |\r\n| `DigitalSignage:Read`, `DigitalSignage:Edit` | View or edit digital signage for the account. |\r\n| `IMChatHistory:Read` | View archived chat messages in the account. |\r\n| `IMGroups:Read`, `IMGroups:Edit` | View or edit IM groups for the account. |\r\n| `IMSetting:Read,`, `IMSetting:Edit` | View or edit IM settings that affect all users in the account. |\r\n| `IMChatBot:Edit` | Send message by IM bot. |\r\n| `BillingSubscription:Read`, `BillingSubscription:Edit` | View your current plan or purchase additional subscriptions and add-ons. |\r\n| `BillingInformation:Read`, `BillingInformation:Edit` | View your invoice history or edit your billing information. |\r\n| `Dashboard:Read` | View usage statistics for the account. |\r\n| `UsageReport:Read` | View detailed usage reports for the account. |\r\n| `UserActivitiesReport:Read` | View detailed user activity reports that can be used for audits. |\r\n| `ScheduleTrackingFields:Read`, `ScheduleTrackingFields:Edit` | View or edit fields that can be used to analyze meeting usage. |\r\n| `PbxAdmin:Read`, `PbxAdmin:Edit` | View or manage Zoom Phone settings for the account. |\r\n| `ZoomDevelopers:Read`, `ZoomDevelopers:Edit` | Get developer privilege to view app settings or build and submit apps on the [Zoom App Marketplace.](https://marketplace.zoom.us/) |\r\n| `RoomConnector:Read`, `RoomConnector:Edit` | View or manage H.323 devices / SIP room connector. |\r\n| `SubAccount:Read`, `SubAccount:Edit` | View or manage [Meeting Connector](https://support.zoom.us/hc/en-us/sections/200305473-Meeting-Connector?flash_digest=a750f74cbf8803f8d5c4c1fded0519fbe01b02a9). |\r\n| `CrossHybrid:Read`, `CrossHybrid:Edit` | View proxy Zone Controller settings or set up and deploy [Meeting Connector with Zone Controllers.](https://support.zoom.us/hc/en-us/articles/360000093183-Deploying-Meeting-Connector-with-Multiple-Zones) |\r\n| `LyncConnector:Read`, `LyncConnector:Edit` | View settings for or manage Skype for Business (Lync) connector. |\r\n| `ThirdPartyConference:Read`, `ThirdPartyConference:Edit` | View or manage settings for Telephony Service Provider. |\r\n| `Branding:Read`, `Branding:Edit` | View or customize styling of your landing page when you have an approved vanity URL. |\r\n| `SingleSignOn:Read`, `SingleSignOn:Edit` | View or edit Single Sign-On settings. |\r\n| `Integration:Read`, `Integration:Edit` | View or manage app integrations for an account. |\r\n| `MarketPlace:Read`, `MarketPlace:Edit` | View or manage app pre-approval admin settings in the Marketplace. |",
        "data": {
          "title": "Privileges and Role Management"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/references/privileges.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
