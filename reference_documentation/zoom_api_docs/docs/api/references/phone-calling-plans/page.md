# Zoom Phone calling plans

- Source URL: https://developers.zoom.us/docs/api/references/phone-calling-plans/
- Snapshot path: docs/api/references/phone-calling-plans
- Generated (UTC): 2026-02-07T22:40:11.517307+00:00

## Frontmatter

```json
{
  "title": "Zoom Phone calling plans"
}
```

## Content

```md

# Zoom Phone calling plans

Use the [List calling plans](/docs/api/phone/ma/#tag/phone-plans/GET/accounts/\{accountId\}/phone/calling_plans) API to list the plans available in the account.

Use the [Assign calling plan to a user](/docs/api/phone/#tag/users/POST/phone/users/\{userId\}/calling_plans) API to assign a calling plan to a user in the account.

See the following table for a list of Zoom Phone calling plans and codes.

## Zoom Phone calling plans and codes

| Zoom Phone plan name | Plan code |
| ------------- | --------- |
| `NO_FEATURE_PACKAGE` | `1` |
| `INTERNATIONAL_TOLL_NUMBER` | `3` |
| `INTERNATIONAL_TOLL_FREE_NUMBER` | `4` |
| `BYOC_NUMBER` | `5` |
| `BETA_NUMBER` | `6` |
| `METERED_PLAN_US_CA` | `100` |
| `METERED_PLAN_AU_NZ` | `101` |
| `METERED_PLAN_GB_IE` | `102` |
| `METERED_EURA` | `103` |
| `METERED_EURB` | `104` |
| `METERED_JP` | `107`
| `UNLIMITED_PLAN_US_CA` | `200` |
| `UNLIMITED_PLAN_AU_NZ` | `201` |
| `UNLIMITED_PLAN_GB_IE` | `202` |
| `UNLIMITED_EURA` | `203` |
| `UNLIMITED_EURB` | `204` |
| `UNLIMITED_JP` | `207` |
| `US_CA_NUMBER` | `300` |
| `AU_NZ_NUMBER` | `301` |
| `GB_IE_NUMBER` | `302` |
| `EURA_NUMBER` | `303` |
| `EURB_NUMBER` | `304` |
| `JP_NUMBER` | `307` |
| `US_CA_TOLLFREE_NUMBER` | `400` |
| `AU_TOLLFREE_NUMBER` | `401` |
| `GB_IE_TOLLFREE_NUMBER` | `402` |
| `NZ_TOLLFREE_NUMBER` | `403` |
| `GLOBAL_TOLLFREE_NUMBER` | `404` |
| `BETA` | `600` |
| `UNLIMITED_DOMESTIC_SELECT` | `1000` |
| `METERED_GLOBAL_SELECT` | `1001` |
| `UNLIMITED_DOMESTIC_SELECT_NUMBER` | `2000` |
| `ZP_PRO` | `3000` |
| `BASIC` | `3010` |
| `ZP_COMMON_AREA` | `3040` |
| `RESERVED_PLAN` | `3098` |
| `BASIC_MIGRATED` | `3099` |
| `INTERNATIONAL_SELECT_ADDON` | `4000` |
| `ZP_PREMIUM_ADDON, ZP_POWER_USER, ZP_POWER_PACK` | `4010` |
| `PREMIUM_NUMBER` | `5000` |
| `METERED_US_CA_NUMBER_INCLUDED` | `30000` |
| `METERED_AU_NZ_NUMBER_INCLUDED` | `30001` |
| `METERED_GB_IE_NUMBER_INCLUDED` | `30002` |
| `METERED_EURA_NUMBER_INCLUDED` | `30003` |
| `METERED_EURB_NUMBER_INCLUDED` | `30004` |
| `METERED_JP_NUMBER_INCLUDED` | `30007` |
| `UNLIMITED_US_CA_NUMBER_INCLUDED` | `31000` |
| `UNLIMITED_AU_NZ_NUMBER_INCLUDED` | `31001` |
| `UNLIMITED_GB_IE_NUMBER_INCLUDED` | `31002` |
| `UNLIMITED_EURA_NUMBER_INCLUDED` | `31003` |
| `UNLIMITED_EURB_NUMBER_INCLUDED` | `31004` |
| `UNLIMITED_JP_NUMBER_INCLUDED` | `31007` |
| `UNLIMITED_DOMESTIC_SELECT_NUMBER_INCLUDED` | `31005` |
| `METERED_GLOBAL_SELECT_NUMBER_INCLUDED` | `31006` |
| `MEETINGS_PRO_UNLIMITED_US_CA` | `40200` |
| `MEETINGS_PRO_UNLIMITED_AU_NZ` | `40201` |
| `MEETINGS_PRO_UNLIMITED_GB_IE` | `40202` |
| `MEETINGS_PRO_UNLIMITED_JP` | `40207` |
| `MEETINGS_PRO_GLOBAL_SELECT` | `41000` |
| `MEETINGS_PRO_PN_PRO` | `43000` |
| `MEETINGS_BUS_UNLIMITED_US_CA` | `50200` |
| `MEETINGS_BUS_UNLIMITED_AU_NZ` | `50201` |
| `MEETINGS_BUS_UNLIMITED_GB_IE` | `50202` |
| `MEETINGS_BUS_UNLIMITED_JP` | `50207` |
| `MEETINGS_BUS_GLOBAL_SELECT` | `51000` |
| `MEETINGS_BUS_PN_PRO` | `53000` |
| `MEETINGS_ENT_UNLIMITED_US_CA` | `60200` |
| `MEETINGS_ENT_UNLIMITED_AU_NZ` | `60201` |
| `MEETINGS_ENT_UNLIMITED_GB_IE` | `60202` |
| `MEETINGS_ENT_UNLIMITED_JP` | `60207` |
| `MEETINGS_ENT_GLOBAL_SELECT` | `61000` |
| `MEETINGS_ENT_PN_PRO` | `63000` |
| `MEETINGS_US_CA_NUMBER_INCLUDED` | `70200` |
| `MEETINGS_AU_NZ_NUMBER_INCLUDED` | `70201` |
| `MEETINGS_GB_IE_NUMBER_INCLUDED` | `70202` |
| `MEETINGS_JP_NUMBER_INCLUDED` | `70207` |
| `MEETINGS_GLOBAL_SELECT_NUMBER_INCLUDED` | `71000` |
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
      "code": "var Component=(()=>{var I=Object.create;var l=Object.defineProperty;var U=Object.getOwnPropertyDescriptor;var M=Object.getOwnPropertyNames;var T=Object.getPrototypeOf,L=Object.prototype.hasOwnProperty;var D=(c,d)=>()=>(d||c((d={exports:{}}).exports,d),d.exports),R=(c,d)=>{for(var r in d)l(c,r,{get:d[r],enumerable:!0})},t=(c,d,r,h)=>{if(d&&typeof d==\"object\"||typeof d==\"function\")for(let n of M(d))!L.call(c,n)&&n!==r&&l(c,n,{get:()=>d[n],enumerable:!(h=U(d,n))||h.enumerable});return c};var a=(c,d,r)=>(r=c!=null?I(T(c)):{},t(d||!c||!c.__esModule?l(r,\"default\",{value:c,enumerable:!0}):r,c)),B=c=>t(l({},\"__esModule\",{value:!0}),c);var o=D((C,E)=>{E.exports=_jsx_runtime});var s={};R(s,{default:()=>N,frontmatter:()=>A});var e=a(o());var{useMDXComponents:i}=MdxJsReact;var A={title:\"Zoom Phone calling plans\"};function _(c){let d={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",i:\"i\",p:\"p\",table:\"table\",tbody:\"tbody\",td:\"td\",th:\"th\",thead:\"thead\",tr:\"tr\",...i(),...c.components};return(0,e.jsxs)(e.Fragment,{children:[(0,e.jsx)(d.h1,{id:\"zoom-phone-calling-plans\",children:\"Zoom Phone calling plans\"}),`\n`,(0,e.jsxs)(d.p,{children:[\"Use the \",(0,e.jsx)(d.a,{href:\"/docs/api/phone/ma/#tag/phone-plans/get/accounts/{accountId}/phone/calling_plans\",children:\"List calling plans\"}),\" API to list the plans available in the account.\"]}),`\n`,(0,e.jsxs)(d.p,{children:[\"Use the \",(0,e.jsx)(d.a,{href:\"/docs/api/phone/#tag/users/post/phone/users/{userId}/calling_plans\",children:\"Assign calling plan to a user\"}),\" API to assign a calling plan to a user in the account.\"]}),`\n`,(0,e.jsx)(d.p,{children:\"See the following table for a list of Zoom Phone calling plans and codes.\"}),`\n`,(0,e.jsx)(d.h2,{id:\"zoom-phone-calling-plans-and-codes\",children:(0,e.jsxs)(d.a,{href:\"#zoom-phone-calling-plans-and-codes\",children:[\"Zoom Phone calling plans and codes\",(0,e.jsx)(d.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(d.table,{children:[(0,e.jsx)(d.thead,{children:(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.th,{children:\"Zoom Phone plan name\"}),(0,e.jsx)(d.th,{children:\"Plan code\"})]})}),(0,e.jsxs)(d.tbody,{children:[(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"NO_FEATURE_PACKAGE\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"1\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"INTERNATIONAL_TOLL_NUMBER\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"3\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"INTERNATIONAL_TOLL_FREE_NUMBER\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"4\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"BYOC_NUMBER\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"5\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"BETA_NUMBER\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"6\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"METERED_PLAN_US_CA\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"100\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"METERED_PLAN_AU_NZ\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"101\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"METERED_PLAN_GB_IE\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"102\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"METERED_EURA\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"103\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"METERED_EURB\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"104\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"METERED_JP\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"107\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"UNLIMITED_PLAN_US_CA\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"200\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"UNLIMITED_PLAN_AU_NZ\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"201\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"UNLIMITED_PLAN_GB_IE\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"202\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"UNLIMITED_EURA\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"203\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"UNLIMITED_EURB\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"204\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"UNLIMITED_JP\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"207\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"US_CA_NUMBER\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"300\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"AU_NZ_NUMBER\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"301\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"GB_IE_NUMBER\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"302\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"EURA_NUMBER\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"303\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"EURB_NUMBER\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"304\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"JP_NUMBER\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"307\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"US_CA_TOLLFREE_NUMBER\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"400\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"AU_TOLLFREE_NUMBER\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"401\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"GB_IE_TOLLFREE_NUMBER\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"402\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"NZ_TOLLFREE_NUMBER\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"403\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"GLOBAL_TOLLFREE_NUMBER\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"404\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"BETA\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"600\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"UNLIMITED_DOMESTIC_SELECT\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"1000\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"METERED_GLOBAL_SELECT\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"1001\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"UNLIMITED_DOMESTIC_SELECT_NUMBER\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"2000\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"ZP_PRO\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"3000\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"BASIC\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"3010\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"ZP_COMMON_AREA\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"3040\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"RESERVED_PLAN\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"3098\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"BASIC_MIGRATED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"3099\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"INTERNATIONAL_SELECT_ADDON\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"4000\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"ZP_PREMIUM_ADDON, ZP_POWER_USER, ZP_POWER_PACK\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"4010\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"PREMIUM_NUMBER\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"5000\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"METERED_US_CA_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"30000\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"METERED_AU_NZ_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"30001\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"METERED_GB_IE_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"30002\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"METERED_EURA_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"30003\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"METERED_EURB_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"30004\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"METERED_JP_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"30007\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"UNLIMITED_US_CA_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"31000\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"UNLIMITED_AU_NZ_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"31001\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"UNLIMITED_GB_IE_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"31002\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"UNLIMITED_EURA_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"31003\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"UNLIMITED_EURB_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"31004\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"UNLIMITED_JP_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"31007\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"UNLIMITED_DOMESTIC_SELECT_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"31005\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"METERED_GLOBAL_SELECT_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"31006\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_PRO_UNLIMITED_US_CA\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"40200\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_PRO_UNLIMITED_AU_NZ\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"40201\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_PRO_UNLIMITED_GB_IE\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"40202\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_PRO_UNLIMITED_JP\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"40207\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_PRO_GLOBAL_SELECT\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"41000\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_PRO_PN_PRO\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"43000\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_BUS_UNLIMITED_US_CA\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"50200\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_BUS_UNLIMITED_AU_NZ\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"50201\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_BUS_UNLIMITED_GB_IE\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"50202\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_BUS_UNLIMITED_JP\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"50207\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_BUS_GLOBAL_SELECT\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"51000\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_BUS_PN_PRO\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"53000\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_ENT_UNLIMITED_US_CA\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"60200\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_ENT_UNLIMITED_AU_NZ\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"60201\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_ENT_UNLIMITED_GB_IE\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"60202\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_ENT_UNLIMITED_JP\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"60207\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_ENT_GLOBAL_SELECT\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"61000\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_ENT_PN_PRO\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"63000\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_US_CA_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"70200\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_AU_NZ_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"70201\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_GB_IE_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"70202\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_JP_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"70207\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"MEETINGS_GLOBAL_SELECT_NUMBER_INCLUDED\"})}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"71000\"})})]})]})]})]})}function N(c={}){let{wrapper:d}={...i(),...c.components};return d?(0,e.jsx)(d,{...c,children:(0,e.jsx)(_,{...c})}):_(c)}return B(s);})();\n;return Component;",
      "frontmatter": {
        "title": "Zoom Phone calling plans"
      },
      "errors": [],
      "matter": {
        "content": "\r\n# Zoom Phone calling plans\r\n\r\nUse the [List calling plans](/docs/api/phone/ma/#tag/phone-plans/GET/accounts/\\{accountId\\}/phone/calling_plans) API to list the plans available in the account.\r\n\r\nUse the [Assign calling plan to a user](/docs/api/phone/#tag/users/POST/phone/users/\\{userId\\}/calling_plans) API to assign a calling plan to a user in the account.\r\n\r\nSee the following table for a list of Zoom Phone calling plans and codes.\r\n\r\n## Zoom Phone calling plans and codes\r\n\r\n| Zoom Phone plan name | Plan code |\r\n| ------------- | --------- |\r\n| `NO_FEATURE_PACKAGE` | `1` |\r\n| `INTERNATIONAL_TOLL_NUMBER` | `3` |\r\n| `INTERNATIONAL_TOLL_FREE_NUMBER` | `4` |\r\n| `BYOC_NUMBER` | `5` |\r\n| `BETA_NUMBER` | `6` |\r\n| `METERED_PLAN_US_CA` | `100` |\r\n| `METERED_PLAN_AU_NZ` | `101` |\r\n| `METERED_PLAN_GB_IE` | `102` |\r\n| `METERED_EURA` | `103` |\r\n| `METERED_EURB` | `104` |\r\n| `METERED_JP` | `107`\r\n| `UNLIMITED_PLAN_US_CA` | `200` |\r\n| `UNLIMITED_PLAN_AU_NZ` | `201` |\r\n| `UNLIMITED_PLAN_GB_IE` | `202` |\r\n| `UNLIMITED_EURA` | `203` |\r\n| `UNLIMITED_EURB` | `204` |\r\n| `UNLIMITED_JP` | `207` |\r\n| `US_CA_NUMBER` | `300` |\r\n| `AU_NZ_NUMBER` | `301` |\r\n| `GB_IE_NUMBER` | `302` |\r\n| `EURA_NUMBER` | `303` |\r\n| `EURB_NUMBER` | `304` |\r\n| `JP_NUMBER` | `307` |\r\n| `US_CA_TOLLFREE_NUMBER` | `400` |\r\n| `AU_TOLLFREE_NUMBER` | `401` |\r\n| `GB_IE_TOLLFREE_NUMBER` | `402` |\r\n| `NZ_TOLLFREE_NUMBER` | `403` |\r\n| `GLOBAL_TOLLFREE_NUMBER` | `404` |\r\n| `BETA` | `600` |\r\n| `UNLIMITED_DOMESTIC_SELECT` | `1000` |\r\n| `METERED_GLOBAL_SELECT` | `1001` |\r\n| `UNLIMITED_DOMESTIC_SELECT_NUMBER` | `2000` |\r\n| `ZP_PRO` | `3000` |\r\n| `BASIC` | `3010` |\r\n| `ZP_COMMON_AREA` | `3040` |\r\n| `RESERVED_PLAN` | `3098` |\r\n| `BASIC_MIGRATED` | `3099` |\r\n| `INTERNATIONAL_SELECT_ADDON` | `4000` |\r\n| `ZP_PREMIUM_ADDON, ZP_POWER_USER, ZP_POWER_PACK` | `4010` |\r\n| `PREMIUM_NUMBER` | `5000` |\r\n| `METERED_US_CA_NUMBER_INCLUDED` | `30000` |\r\n| `METERED_AU_NZ_NUMBER_INCLUDED` | `30001` |\r\n| `METERED_GB_IE_NUMBER_INCLUDED` | `30002` |\r\n| `METERED_EURA_NUMBER_INCLUDED` | `30003` |\r\n| `METERED_EURB_NUMBER_INCLUDED` | `30004` |\r\n| `METERED_JP_NUMBER_INCLUDED` | `30007` |\r\n| `UNLIMITED_US_CA_NUMBER_INCLUDED` | `31000` |\r\n| `UNLIMITED_AU_NZ_NUMBER_INCLUDED` | `31001` |\r\n| `UNLIMITED_GB_IE_NUMBER_INCLUDED` | `31002` |\r\n| `UNLIMITED_EURA_NUMBER_INCLUDED` | `31003` |\r\n| `UNLIMITED_EURB_NUMBER_INCLUDED` | `31004` |\r\n| `UNLIMITED_JP_NUMBER_INCLUDED` | `31007` |\r\n| `UNLIMITED_DOMESTIC_SELECT_NUMBER_INCLUDED` | `31005` |\r\n| `METERED_GLOBAL_SELECT_NUMBER_INCLUDED` | `31006` |\r\n| `MEETINGS_PRO_UNLIMITED_US_CA` | `40200` |\r\n| `MEETINGS_PRO_UNLIMITED_AU_NZ` | `40201` |\r\n| `MEETINGS_PRO_UNLIMITED_GB_IE` | `40202` |\r\n| `MEETINGS_PRO_UNLIMITED_JP` | `40207` |\r\n| `MEETINGS_PRO_GLOBAL_SELECT` | `41000` |\r\n| `MEETINGS_PRO_PN_PRO` | `43000` |\r\n| `MEETINGS_BUS_UNLIMITED_US_CA` | `50200` |\r\n| `MEETINGS_BUS_UNLIMITED_AU_NZ` | `50201` |\r\n| `MEETINGS_BUS_UNLIMITED_GB_IE` | `50202` |\r\n| `MEETINGS_BUS_UNLIMITED_JP` | `50207` |\r\n| `MEETINGS_BUS_GLOBAL_SELECT` | `51000` |\r\n| `MEETINGS_BUS_PN_PRO` | `53000` |\r\n| `MEETINGS_ENT_UNLIMITED_US_CA` | `60200` |\r\n| `MEETINGS_ENT_UNLIMITED_AU_NZ` | `60201` |\r\n| `MEETINGS_ENT_UNLIMITED_GB_IE` | `60202` |\r\n| `MEETINGS_ENT_UNLIMITED_JP` | `60207` |\r\n| `MEETINGS_ENT_GLOBAL_SELECT` | `61000` |\r\n| `MEETINGS_ENT_PN_PRO` | `63000` |\r\n| `MEETINGS_US_CA_NUMBER_INCLUDED` | `70200` |\r\n| `MEETINGS_AU_NZ_NUMBER_INCLUDED` | `70201` |\r\n| `MEETINGS_GB_IE_NUMBER_INCLUDED` | `70202` |\r\n| `MEETINGS_JP_NUMBER_INCLUDED` | `70207` |\r\n| `MEETINGS_GLOBAL_SELECT_NUMBER_INCLUDED` | `71000` |",
        "data": {
          "title": "Zoom Phone calling plans"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/references/phone-calling-plans.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
