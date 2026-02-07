# Recurrence Object Definitions

- Source URL: https://developers.zoom.us/docs/api/references/recurrence-object-definitions/
- Snapshot path: docs/api/references/recurrence-object-definitions
- Generated (UTC): 2026-02-07T22:40:11.521175+00:00

## Frontmatter

```json
{
  "title": "Recurrence Object Definitions"
}
```

## Content

```md

# Recurrence Object Definitions

## Recurrence Object Keys and Values

| Key | Value | Example |
| --- | ----- | ------- |
| `type` | **Required**. The meeting's recurrence type: <ul><li>`1` &mdash; Daily</li><li>`2` &mdash; Weekly</li><li>`3` &mdash; Monthly</li></ul> | `2` |
| `repeat_interval` | The meeting's repeat interval: <ul><li>For a daily interval, a max of `90` days.</li><li>For a weekly interval, a max of `12` weeks.</li><li>For a monthly interval, a max of `3` months.</li></ul> This value defaults to `1`. | `12` |
| `weekly_days` | **Required only for weekly recurring meetings.** A comma-separated list of the days of the week on which the recurring meeting occurs: <ul><li>`1` &mdash; Sunday</li><li>`2` &mdash; Monday</li><li>`3` &mdash; Tuesday</li><li>`4` &mdash; Wednesday</li><li>`5` &mdash; Thursday</li><li>`6` &mdash; Friday</li><li>`7` &mdash; Saturday</li></ul> This value defaults to `1`. | `1,3` |
| `monthly_day` | **Required only for monthly recurring meetings.** The day of the month on which a recurring meeting occurs, between `1` to `31`. | `15` |
| `monthly_week` | **Required only for monthly recurring meetings.** The week of the month on which a recurring meeting occurs: <ul><li>`-1` The last week of the month.</li><li>`1` &mdash; The first week.</li><li>`2` &mdash; The second week.</li><li>`3` &mdash; The third week.</li><li>`4` &mdash; The fourth week</li></ul>. | `1` |
| `monthly_week_day` | **Required only for monthly recurring meetings.** The day of the week on which to schedule the recurring meeting: <ul><li>`1` &mdash; Sunday</li><li>`2` &mdash; Monday</li><li>`3` &mdash; Tuesday</li><li>`4` &mdash; Wednesday</li><li>`5` &mdash; Thursday</li><li>`6` &mdash; Friday</li><li>`7` &mdash; Saturday</li></ul> | `2` |
| `end_times` | The number of times the recurring meeting will occur before it is canceled, between a value of `1` and `50`. You **cannot** use this key with the `end_date_time` key. To increase this value, contact the [Zoom Support Team](https://support.zoom.us/hc/en-us/articles/201362003). <br /><br /> This value defaults to `1`. | `10` |
| `end_date_time` | The recurring meeting's final ending date and time before it is canceled, in UTC format. You **cannot** use this key with the `end_times` key. | `2012-11-25T12:00:00Z` |
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
      "code": "var Component=(()=>{var m=Object.create;var i=Object.defineProperty;var y=Object.getOwnPropertyDescriptor;var f=Object.getOwnPropertyNames;var g=Object.getPrototypeOf,w=Object.prototype.hasOwnProperty;var T=(n,d)=>()=>(d||n((d={exports:{}}).exports,d),d.exports),k=(n,d)=>{for(var r in d)i(n,r,{get:d[r],enumerable:!0})},t=(n,d,r,h)=>{if(d&&typeof d==\"object\"||typeof d==\"function\")for(let c of f(d))!w.call(n,c)&&c!==r&&i(n,c,{get:()=>d[c],enumerable:!(h=y(d,c))||h.enumerable});return n};var p=(n,d,r)=>(r=n!=null?m(g(n)):{},t(d||!n||!n.__esModule?i(r,\"default\",{value:n,enumerable:!0}):r,n)),_=n=>t(i({},\"__esModule\",{value:!0}),n);var a=T((j,o)=>{o.exports=_jsx_runtime});var x={};k(x,{default:()=>u,frontmatter:()=>b});var e=p(a());var{useMDXComponents:l}=MdxJsReact;var b={title:\"Recurrence Object Definitions\"};function s(n){let d={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",i:\"i\",strong:\"strong\",table:\"table\",tbody:\"tbody\",td:\"td\",th:\"th\",thead:\"thead\",tr:\"tr\",...l(),...n.components};return(0,e.jsxs)(e.Fragment,{children:[(0,e.jsx)(d.h1,{id:\"recurrence-object-definitions\",children:\"Recurrence Object Definitions\"}),`\n`,(0,e.jsx)(d.h2,{id:\"recurrence-object-keys-and-values\",children:(0,e.jsxs)(d.a,{href:\"#recurrence-object-keys-and-values\",children:[\"Recurrence Object Keys and Values\",(0,e.jsx)(d.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,e.jsxs)(d.table,{children:[(0,e.jsx)(d.thead,{children:(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.th,{children:\"Key\"}),(0,e.jsx)(d.th,{children:\"Value\"}),(0,e.jsx)(d.th,{children:\"Example\"})]})}),(0,e.jsxs)(d.tbody,{children:[(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"type\"})}),(0,e.jsxs)(d.td,{children:[(0,e.jsx)(d.strong,{children:\"Required\"}),\". The meeting's recurrence type: \",(0,e.jsxs)(\"ul\",{children:[(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"1\"}),\" \\u2014 Daily\"]}),(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"2\"}),\" \\u2014 Weekly\"]}),(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"3\"}),\" \\u2014 Monthly\"]})]})]}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"2\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"repeat_interval\"})}),(0,e.jsxs)(d.td,{children:[\"The meeting's repeat interval: \",(0,e.jsxs)(\"ul\",{children:[(0,e.jsxs)(\"li\",{children:[\"For a daily interval, a max of \",(0,e.jsx)(d.code,{children:\"90\"}),\" days.\"]}),(0,e.jsxs)(\"li\",{children:[\"For a weekly interval, a max of \",(0,e.jsx)(d.code,{children:\"12\"}),\" weeks.\"]}),(0,e.jsxs)(\"li\",{children:[\"For a monthly interval, a max of \",(0,e.jsx)(d.code,{children:\"3\"}),\" months.\"]})]}),\" This value defaults to \",(0,e.jsx)(d.code,{children:\"1\"}),\".\"]}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"12\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"weekly_days\"})}),(0,e.jsxs)(d.td,{children:[(0,e.jsx)(d.strong,{children:\"Required only for weekly recurring meetings.\"}),\" A comma-separated list of the days of the week on which the recurring meeting occurs: \",(0,e.jsxs)(\"ul\",{children:[(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"1\"}),\" \\u2014 Sunday\"]}),(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"2\"}),\" \\u2014 Monday\"]}),(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"3\"}),\" \\u2014 Tuesday\"]}),(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"4\"}),\" \\u2014 Wednesday\"]}),(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"5\"}),\" \\u2014 Thursday\"]}),(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"6\"}),\" \\u2014 Friday\"]}),(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"7\"}),\" \\u2014 Saturday\"]})]}),\" This value defaults to \",(0,e.jsx)(d.code,{children:\"1\"}),\".\"]}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"1,3\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"monthly_day\"})}),(0,e.jsxs)(d.td,{children:[(0,e.jsx)(d.strong,{children:\"Required only for monthly recurring meetings.\"}),\" The day of the month on which a recurring meeting occurs, between \",(0,e.jsx)(d.code,{children:\"1\"}),\" to \",(0,e.jsx)(d.code,{children:\"31\"}),\".\"]}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"15\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"monthly_week\"})}),(0,e.jsxs)(d.td,{children:[(0,e.jsx)(d.strong,{children:\"Required only for monthly recurring meetings.\"}),\" The week of the month on which a recurring meeting occurs: \",(0,e.jsxs)(\"ul\",{children:[(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"-1\"}),\" The last week of the month.\"]}),(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"1\"}),\" \\u2014 The first week.\"]}),(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"2\"}),\" \\u2014 The second week.\"]}),(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"3\"}),\" \\u2014 The third week.\"]}),(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"4\"}),\" \\u2014 The fourth week\"]})]}),\".\"]}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"1\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"monthly_week_day\"})}),(0,e.jsxs)(d.td,{children:[(0,e.jsx)(d.strong,{children:\"Required only for monthly recurring meetings.\"}),\" The day of the week on which to schedule the recurring meeting: \",(0,e.jsxs)(\"ul\",{children:[(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"1\"}),\" \\u2014 Sunday\"]}),(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"2\"}),\" \\u2014 Monday\"]}),(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"3\"}),\" \\u2014 Tuesday\"]}),(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"4\"}),\" \\u2014 Wednesday\"]}),(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"5\"}),\" \\u2014 Thursday\"]}),(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"6\"}),\" \\u2014 Friday\"]}),(0,e.jsxs)(\"li\",{children:[(0,e.jsx)(d.code,{children:\"7\"}),\" \\u2014 Saturday\"]})]})]}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"2\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"end_times\"})}),(0,e.jsxs)(d.td,{children:[\"The number of times the recurring meeting will occur before it is canceled, between a value of \",(0,e.jsx)(d.code,{children:\"1\"}),\" and \",(0,e.jsx)(d.code,{children:\"50\"}),\". You \",(0,e.jsx)(d.strong,{children:\"cannot\"}),\" use this key with the \",(0,e.jsx)(d.code,{children:\"end_date_time\"}),\" key. To increase this value, contact the \",(0,e.jsx)(d.a,{href:\"https://support.zoom.us/hc/en-us/articles/201362003\",children:\"Zoom Support Team\"}),\". \",(0,e.jsx)(\"br\",{}),(0,e.jsx)(\"br\",{}),\" This value defaults to \",(0,e.jsx)(d.code,{children:\"1\"}),\".\"]}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"10\"})})]}),(0,e.jsxs)(d.tr,{children:[(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"end_date_time\"})}),(0,e.jsxs)(d.td,{children:[\"The recurring meeting's final ending date and time before it is canceled, in UTC format. You \",(0,e.jsx)(d.strong,{children:\"cannot\"}),\" use this key with the \",(0,e.jsx)(d.code,{children:\"end_times\"}),\" key.\"]}),(0,e.jsx)(d.td,{children:(0,e.jsx)(d.code,{children:\"2012-11-25T12:00:00Z\"})})]})]})]})]})}function u(n={}){let{wrapper:d}={...l(),...n.components};return d?(0,e.jsx)(d,{...n,children:(0,e.jsx)(s,{...n})}):s(n)}return _(x);})();\n;return Component;",
      "frontmatter": {
        "title": "Recurrence Object Definitions"
      },
      "errors": [],
      "matter": {
        "content": "\r\n# Recurrence Object Definitions\r\n\r\n## Recurrence Object Keys and Values\r\n\r\n| Key | Value | Example |\r\n| --- | ----- | ------- |\r\n| `type` | **Required**. The meeting's recurrence type: <ul><li>`1` &mdash; Daily</li><li>`2` &mdash; Weekly</li><li>`3` &mdash; Monthly</li></ul> | `2` |\r\n| `repeat_interval` | The meeting's repeat interval: <ul><li>For a daily interval, a max of `90` days.</li><li>For a weekly interval, a max of `12` weeks.</li><li>For a monthly interval, a max of `3` months.</li></ul> This value defaults to `1`. | `12` |\r\n| `weekly_days` | **Required only for weekly recurring meetings.** A comma-separated list of the days of the week on which the recurring meeting occurs: <ul><li>`1` &mdash; Sunday</li><li>`2` &mdash; Monday</li><li>`3` &mdash; Tuesday</li><li>`4` &mdash; Wednesday</li><li>`5` &mdash; Thursday</li><li>`6` &mdash; Friday</li><li>`7` &mdash; Saturday</li></ul> This value defaults to `1`. | `1,3` |\r\n| `monthly_day` | **Required only for monthly recurring meetings.** The day of the month on which a recurring meeting occurs, between `1` to `31`. | `15` |\r\n| `monthly_week` | **Required only for monthly recurring meetings.** The week of the month on which a recurring meeting occurs: <ul><li>`-1` The last week of the month.</li><li>`1` &mdash; The first week.</li><li>`2` &mdash; The second week.</li><li>`3` &mdash; The third week.</li><li>`4` &mdash; The fourth week</li></ul>. | `1` |\r\n| `monthly_week_day` | **Required only for monthly recurring meetings.** The day of the week on which to schedule the recurring meeting: <ul><li>`1` &mdash; Sunday</li><li>`2` &mdash; Monday</li><li>`3` &mdash; Tuesday</li><li>`4` &mdash; Wednesday</li><li>`5` &mdash; Thursday</li><li>`6` &mdash; Friday</li><li>`7` &mdash; Saturday</li></ul> | `2` |\r\n| `end_times` | The number of times the recurring meeting will occur before it is canceled, between a value of `1` and `50`. You **cannot** use this key with the `end_date_time` key. To increase this value, contact the [Zoom Support Team](https://support.zoom.us/hc/en-us/articles/201362003). <br /><br /> This value defaults to `1`. | `10` |\r\n| `end_date_time` | The recurring meeting's final ending date and time before it is canceled, in UTC format. You **cannot** use this key with the `end_times` key. | `2012-11-25T12:00:00Z` |\r\n",
        "data": {
          "title": "Recurrence Object Definitions"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/references/recurrence-object-definitions.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
