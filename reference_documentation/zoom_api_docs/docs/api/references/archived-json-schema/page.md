# Archived JSON schemas

- Source URL: https://developers.zoom.us/docs/api/references/archived-json-schema/
- Snapshot path: docs/api/references/archived-json-schema
- Generated (UTC): 2026-02-07T22:40:11.505825+00:00

## Frontmatter

```json
{
  "title": "Archived JSON schemas",
  "description": "Chat message and subgroup activity JSON schemas."
}
```

## Content

```md

# Archived JSON schemas

Zoom returns the following JSON schemas when downloading chat messages and subgroup activity with the [Get a meeting's archived files](/docs/api/meetings/#tag/archiving/get/past_meetings/{meetingUUID}/archive_files) API.

## In-meeting chat message

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "Archiving Chat",
  "type": "object",
  "properties": {
    "uuid": {
      "type": "string",
      "format": "uuid",
      "description": "The meeting or webinar's universally unique ID (UUID). Each meeting or webinar instance generates its own UUID."
    },
    "messages": {
      "type": "array",
      "description": "A list of archived chat messages.",
      "items": {
        "type": "object",
        "properties": {
          "msg_type": {
            "type": "integer",
            "enum": [1, 2, 3, 4, 5, 6, 7],
            "description": "Message type. 1: public chat, 2: private chat, 3: deleted chat, 4: emoji reaction, 5: nonverbal feedback, 6: edited chat, 7: reply chat."
          },
          "e_time": {
            "type": "string",
            "pattern": "^(\\d{2}:\\d{2}:\\d{2})$",
            "description": "Elapsed time after the meeting or webinar started (HH:MM:SS).",
            "examples": ["00:05:30", "01:23:45"]
          },
          "s_id": {
            "type": "string",
            "description": "The sender's ID. If the user did not choose to display email and node_id, the value is empty."
          },
          "s_name": {
            "type": "string",
            "pattern": "^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$",
            "description": "The sender's name for this message, base64-encoded."
          },
          "s_email": {
            "type": "string",
            "pattern": "^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$",
            "description": "The sender's email for this message, base64-encoded. Empty if hidden."
          },
          "e_archive_time": {
            "type": "string",
            "pattern": "^(\\d{2}:\\d{2}:\\d{2})$",
            "description": "Elapsed time since archiving started (HH:MM:SS).",
            "examples": ["00:05:30", "01:23:45"]
          },
          "msg_id": {
            "type": "string",
            "pattern": "^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$",
            "description": "The text message's unique ID or the unique ID of the deleted/edited message, base64-encoded."
          },
          "msg_extended_type": {
            "type": "string",
            "pattern": "^(\\d{2}){6}$",
            "description": "The format XX XX XX XX XX XX of this message type from the left to right has 2-digit decimal pair numbers which indicates the type of the message. The information for each pair is listed here: 1st XX pair (Sender Location): 01-Meeting, 02-Webinar, 03-Breakout Room, 04-Waiting Room, 05-Backstage; 2nd XX pair (Receiver Location): 01-Meeting, 02-Webinar, 03-Breakout Room, 04-Waiting Room, 05-Backstage; 3rd XX pair (Message Type): 01-Sent, 02-Deleted, 03-Edited; 4th XX pair: 00-Null; 5th XX pair (Receiver): 01- Everyone in meeting, 02-(Webinar)Hosts and Panelists, 03-(Webinar)Attendee CC to Panelists and Hosts, 04-Direct message to User, 05-Waiting Room Participants, 06 - To all hosts (from Waiting Room), 07 - Private group chat; 6th XX pair: 00-Null."
          },
          "r_id": {
            "type": "string",
            "description": "Receiver’s ID. Required when msg_type=2 (private chat)."
          },
          "r_name": {
            "type": "string",
            "pattern": "^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$",
            "description": "Receiver’s name, base64-encoded. Required when msg_type=2."
          },
          "r_email": {
            "type": "string",
            "pattern": "^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$",
            "description": "Receiver’s email, base64-encoded. Required when msg_type=2. Empty if hidden."
          },
          "msg_content": {
            "type": "string",
            "pattern": "^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$",
            "description": "Message content, format depends on msg_type. base64-encoded."
          },
          "original_msg_id": {
            "type": "string",
            "pattern": "^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$",
            "description": "The ID of the original message that was deleted or edited. base64-encoded."
          },
          "parent_msg_id": {
            "type": "string",
            "pattern": "^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$",
            "description": "The ID of the parent message being replied to. base64-encoded."
          },
          "private_group_id": {
            "type": "string",
            "pattern": "^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$",
            "description": "ID of the private chat group. Used when msg_extended_type indicates private group (07). base64-encoded."
          },
          "files": {
            "type": "array",
            "description": "List of files shared in this chat message.",
            "items": {
              "type": "object",
              "properties": {
                "fileId": { "type": "string" },
                "fileName": { "type": "string" },
                "fileSize": { "type": "integer" }
              },
              "required": ["fileId", "fileName", "fileSize"]
            }
          },
          "rich_text": {
            "type": "array",
            "description": "Rich text formatting applied to parts of the message.",
            "items": {
              "type": "object",
              "properties": {
                "start_position": { "type": "integer" },
                "end_position": { "type": "integer" },
                "format_type": {
                  "type": "string",
                  "enum": [
                    "Bold", "Italic", "Strikethrough", "BulletedList", "NumberedList",
                    "Underline", "FontSize", "FontColor", "BackgroundColor", "LeftIndent",
                    "Paragraph", "Quote", "AddLink"
                  ]
                },
                "format_attr": { "type": "string" }
              },
              "required": ["start_position", "end_position", "format_type"]
            }
          }
        },
        "required": [
          "msg_type", "e_time", "e_archive_time", "s_id", "s_name", "s_email"
        ],
        "allOf": [
          {
            "if": { "properties": { "msg_type": { "enum": [1, 2, 3, 6, 7] } } },
            "then": { "required": ["msg_id", "msg_extended_type"] }
          },
          {
            "if": { "properties": { "msg_type": { "const": 2 } } },
            "then": { "required": ["r_id", "r_name", "r_email"] }
          },
          {
            "if": { "properties": { "msg_type": { "enum": [1, 2, 4, 6, 7] } } },
            "then": {
              "properties": {
                "msg_content": {
                  "type": "string",
                  "pattern": "^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$",
                  "description": "The content of this text message in the meeting or webinar or the command of the emoji reaction in the meeting or webinar. base64 encoded."
                }
              },
              "required": ["msg_content"]
            }
          },
          {
            "if": { "properties": { "msg_type": { "const": 5 } } },
            "then": {
              "properties": {
                "msg_content": {
                  "type": "string",
                  "pattern": "^[0-9]+$",
                  "description": "The command of the nonverbal feedback in the meeting or webinar. For example: 1-raising hand, 2-yes, 3-no."
                }
              },
              "required": ["msg_content"]
            }
          },
          {
            "if": { "properties": { "msg_type": { "enum": [3, 6] } } },
            "then": { "required": ["original_msg_id"] }
          },
          {
            "if": { "properties": { "msg_type": { "const": 7 } } },
            "then": { "required": ["parent_msg_id"] }
          }
        ]
      }
    }
  },
  "required": ["uuid", "messages"]
}
```

## Subgroup archiving activity

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "Subgroup Archiving Activity",
  "type": "object",
  "properties": {
    "archiving_start_time": {
      "type": "string",
      "format": "date-time",
      "description": "The archiving starting timestamp in GMT format.",
      "examples": ["2021-04-26T05:23:18Z"]
    },
    "backstage_room": {
      "$ref": "#/$defs/BackstageRoom"
    },
    "private_group": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PrivateGroup"
      }
    }
  },
  "required": ["archiving_start_time"],

  "$defs": {
    "BackstageRoom": {
      "type": "object",
      "properties": {
        "id": {
          "type": "string",
          "description": "The ID of the Webinar backstage room."
        },
        "createTime": {
          "type": "string",
          "pattern": "^(\\d{2}:\\d{2}:\\d{2})$",
          "description": "Elapsed time from archiving start when the backstage room was created (HH:MM:SS).",
          "examples": ["00:00:30"]
        },
        "deleteTime": {
          "type": "string",
          "pattern": "^(\\d{2}:\\d{2}:\\d{2})$",
          "description": "Elapsed time from archiving start when the backstage room was deleted (HH:MM:SS).",
          "examples": ["00:10:30"]
        },
        "actions": {
          "type": "array",
          "items": { "$ref": "#/$defs/Participant" }
        }
      },
      "required": ["id", "createTime", "deleteTime", "actions"]
    },

    "PrivateGroup": {
      "type": "object",
      "properties": {
        "id": {
          "type": "string",
          "description": "The ID of the private chat group."
        },
        "createTime": {
          "type": "string",
          "pattern": "^(\\d{2}:\\d{2}:\\d{2})$",
          "description": "Elapsed time from archiving start when the private chat group was created (HH:MM:SS).",
          "examples": ["00:00:30"]
        },
        "deleteTime": {
          "type": "string",
          "pattern": "^(\\d{2}:\\d{2}:\\d{2})$",
          "description": "Elapsed time from archiving start when the private chat group was deleted (HH:MM:SS).", 
          "examples": ["00:20:30"]
        },
        "actions": {
          "type": "array",
          "items": { "$ref": "#/$defs/Participant" }
        }
      },
      "required": ["id", "createTime", "deleteTime", "actions"]
    },

    "Participant": {
      "type": "object",
      "properties": {
        "email": {
          "type": "string",
          "format": "email",
          "description": "The email of the participant."
        },
        "joinTime": {
          "type": "string",
          "pattern": "^(\\d{2}:\\d{2}:\\d{2})$",
          "description": "Elapsed time from archiving start when the participant joined (HH:MM:SS).",
          "examples": ["00:00:30"]
        },
        "leaveTime": {
          "type": "string",
          "pattern": "^(\\d{2}:\\d{2}:\\d{2})$",
          "description": "Elapsed time from archiving start when the participant left (HH:MM:SS).",
          "examples": ["00:00:35"]
        }
      },
      "required": ["joinTime", "leaveTime"]
    }
  }
}
```
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
      "code": "var Component=(()=>{var g=Object.create;var a=Object.defineProperty;var l=Object.getOwnPropertyDescriptor;var y=Object.getOwnPropertyNames;var f=Object.getPrototypeOf,u=Object.prototype.hasOwnProperty;var _=(n,e)=>()=>(e||n((e={exports:{}}).exports,e),e.exports),v=(n,e)=>{for(var i in e)a(n,i,{get:e[i],enumerable:!0})},p=(n,e,i,o)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let r of y(e))!u.call(n,r)&&r!==i&&a(n,r,{get:()=>e[r],enumerable:!(o=l(e,r))||o.enumerable});return n};var b=(n,e,i)=>(i=n!=null?g(f(n)):{},p(e||!n||!n.__esModule?a(i,\"default\",{value:n,enumerable:!0}):i,n)),A=n=>p(a({},\"__esModule\",{value:!0}),n);var c=_(($,d)=>{d.exports=_jsx_runtime});var Z={};v(Z,{default:()=>h,frontmatter:()=>z});var t=b(c());var{useMDXComponents:s}=MdxJsReact;var z={title:\"Archived JSON schemas\",description:\"Chat message and subgroup activity JSON schemas.\"};function m(n){let e={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",i:\"i\",p:\"p\",pre:\"pre\",...s(),...n.components};return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(e.h1,{id:\"archived-json-schemas\",children:\"Archived JSON schemas\"}),`\n`,(0,t.jsxs)(e.p,{children:[\"Zoom returns the following JSON schemas when downloading chat messages and subgroup activity with the \",(0,t.jsx)(e.a,{href:\"/docs/api/meetings/#tag/archiving/get/past_meetings/{meetingUUID}/archive_files\",children:\"Get a meeting's archived files\"}),\" API.\"]}),`\n`,(0,t.jsx)(e.h2,{id:\"in-meeting-chat-message\",children:(0,t.jsxs)(e.a,{href:\"#in-meeting-chat-message\",children:[\"In-meeting chat message\",(0,t.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,t.jsx)(e.pre,{children:(0,t.jsx)(e.code,{className:\"language-json\",children:`{\n  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n  \"title\": \"Archiving Chat\",\n  \"type\": \"object\",\n  \"properties\": {\n    \"uuid\": {\n      \"type\": \"string\",\n      \"format\": \"uuid\",\n      \"description\": \"The meeting or webinar's universally unique ID (UUID). Each meeting or webinar instance generates its own UUID.\"\n    },\n    \"messages\": {\n      \"type\": \"array\",\n      \"description\": \"A list of archived chat messages.\",\n      \"items\": {\n        \"type\": \"object\",\n        \"properties\": {\n          \"msg_type\": {\n            \"type\": \"integer\",\n            \"enum\": [1, 2, 3, 4, 5, 6, 7],\n            \"description\": \"Message type. 1: public chat, 2: private chat, 3: deleted chat, 4: emoji reaction, 5: nonverbal feedback, 6: edited chat, 7: reply chat.\"\n          },\n          \"e_time\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(\\\\\\\\d{2}:\\\\\\\\d{2}:\\\\\\\\d{2})$\",\n            \"description\": \"Elapsed time after the meeting or webinar started (HH:MM:SS).\",\n            \"examples\": [\"00:05:30\", \"01:23:45\"]\n          },\n          \"s_id\": {\n            \"type\": \"string\",\n            \"description\": \"The sender's ID. If the user did not choose to display email and node_id, the value is empty.\"\n          },\n          \"s_name\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$\",\n            \"description\": \"The sender's name for this message, base64-encoded.\"\n          },\n          \"s_email\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$\",\n            \"description\": \"The sender's email for this message, base64-encoded. Empty if hidden.\"\n          },\n          \"e_archive_time\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(\\\\\\\\d{2}:\\\\\\\\d{2}:\\\\\\\\d{2})$\",\n            \"description\": \"Elapsed time since archiving started (HH:MM:SS).\",\n            \"examples\": [\"00:05:30\", \"01:23:45\"]\n          },\n          \"msg_id\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$\",\n            \"description\": \"The text message's unique ID or the unique ID of the deleted/edited message, base64-encoded.\"\n          },\n          \"msg_extended_type\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(\\\\\\\\d{2}){6}$\",\n            \"description\": \"The format XX XX XX XX XX XX of this message type from the left to right has 2-digit decimal pair numbers which indicates the type of the message. The information for each pair is listed here: 1st XX pair (Sender Location): 01-Meeting, 02-Webinar, 03-Breakout Room, 04-Waiting Room, 05-Backstage; 2nd XX pair (Receiver Location): 01-Meeting, 02-Webinar, 03-Breakout Room, 04-Waiting Room, 05-Backstage; 3rd XX pair (Message Type): 01-Sent, 02-Deleted, 03-Edited; 4th XX pair: 00-Null; 5th XX pair (Receiver): 01- Everyone in meeting, 02-(Webinar)Hosts and Panelists, 03-(Webinar)Attendee CC to Panelists and Hosts, 04-Direct message to User, 05-Waiting Room Participants, 06 - To all hosts (from Waiting Room), 07 - Private group chat; 6th XX pair: 00-Null.\"\n          },\n          \"r_id\": {\n            \"type\": \"string\",\n            \"description\": \"Receiver\\u2019s ID. Required when msg_type=2 (private chat).\"\n          },\n          \"r_name\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$\",\n            \"description\": \"Receiver\\u2019s name, base64-encoded. Required when msg_type=2.\"\n          },\n          \"r_email\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$\",\n            \"description\": \"Receiver\\u2019s email, base64-encoded. Required when msg_type=2. Empty if hidden.\"\n          },\n          \"msg_content\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$\",\n            \"description\": \"Message content, format depends on msg_type. base64-encoded.\"\n          },\n          \"original_msg_id\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$\",\n            \"description\": \"The ID of the original message that was deleted or edited. base64-encoded.\"\n          },\n          \"parent_msg_id\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$\",\n            \"description\": \"The ID of the parent message being replied to. base64-encoded.\"\n          },\n          \"private_group_id\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$\",\n            \"description\": \"ID of the private chat group. Used when msg_extended_type indicates private group (07). base64-encoded.\"\n          },\n          \"files\": {\n            \"type\": \"array\",\n            \"description\": \"List of files shared in this chat message.\",\n            \"items\": {\n              \"type\": \"object\",\n              \"properties\": {\n                \"fileId\": { \"type\": \"string\" },\n                \"fileName\": { \"type\": \"string\" },\n                \"fileSize\": { \"type\": \"integer\" }\n              },\n              \"required\": [\"fileId\", \"fileName\", \"fileSize\"]\n            }\n          },\n          \"rich_text\": {\n            \"type\": \"array\",\n            \"description\": \"Rich text formatting applied to parts of the message.\",\n            \"items\": {\n              \"type\": \"object\",\n              \"properties\": {\n                \"start_position\": { \"type\": \"integer\" },\n                \"end_position\": { \"type\": \"integer\" },\n                \"format_type\": {\n                  \"type\": \"string\",\n                  \"enum\": [\n                    \"Bold\", \"Italic\", \"Strikethrough\", \"BulletedList\", \"NumberedList\",\n                    \"Underline\", \"FontSize\", \"FontColor\", \"BackgroundColor\", \"LeftIndent\",\n                    \"Paragraph\", \"Quote\", \"AddLink\"\n                  ]\n                },\n                \"format_attr\": { \"type\": \"string\" }\n              },\n              \"required\": [\"start_position\", \"end_position\", \"format_type\"]\n            }\n          }\n        },\n        \"required\": [\n          \"msg_type\", \"e_time\", \"e_archive_time\", \"s_id\", \"s_name\", \"s_email\"\n        ],\n        \"allOf\": [\n          {\n            \"if\": { \"properties\": { \"msg_type\": { \"enum\": [1, 2, 3, 6, 7] } } },\n            \"then\": { \"required\": [\"msg_id\", \"msg_extended_type\"] }\n          },\n          {\n            \"if\": { \"properties\": { \"msg_type\": { \"const\": 2 } } },\n            \"then\": { \"required\": [\"r_id\", \"r_name\", \"r_email\"] }\n          },\n          {\n            \"if\": { \"properties\": { \"msg_type\": { \"enum\": [1, 2, 4, 6, 7] } } },\n            \"then\": {\n              \"properties\": {\n                \"msg_content\": {\n                  \"type\": \"string\",\n                  \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$\",\n                  \"description\": \"The content of this text message in the meeting or webinar or the command of the emoji reaction in the meeting or webinar. base64 encoded.\"\n                }\n              },\n              \"required\": [\"msg_content\"]\n            }\n          },\n          {\n            \"if\": { \"properties\": { \"msg_type\": { \"const\": 5 } } },\n            \"then\": {\n              \"properties\": {\n                \"msg_content\": {\n                  \"type\": \"string\",\n                  \"pattern\": \"^[0-9]+$\",\n                  \"description\": \"The command of the nonverbal feedback in the meeting or webinar. For example: 1-raising hand, 2-yes, 3-no.\"\n                }\n              },\n              \"required\": [\"msg_content\"]\n            }\n          },\n          {\n            \"if\": { \"properties\": { \"msg_type\": { \"enum\": [3, 6] } } },\n            \"then\": { \"required\": [\"original_msg_id\"] }\n          },\n          {\n            \"if\": { \"properties\": { \"msg_type\": { \"const\": 7 } } },\n            \"then\": { \"required\": [\"parent_msg_id\"] }\n          }\n        ]\n      }\n    }\n  },\n  \"required\": [\"uuid\", \"messages\"]\n}\n`})}),`\n`,(0,t.jsx)(e.h2,{id:\"subgroup-archiving-activity\",children:(0,t.jsxs)(e.a,{href:\"#subgroup-archiving-activity\",children:[\"Subgroup archiving activity\",(0,t.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,t.jsx)(e.pre,{children:(0,t.jsx)(e.code,{className:\"language-json\",children:`{\n  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n  \"title\": \"Subgroup Archiving Activity\",\n  \"type\": \"object\",\n  \"properties\": {\n    \"archiving_start_time\": {\n      \"type\": \"string\",\n      \"format\": \"date-time\",\n      \"description\": \"The archiving starting timestamp in GMT format.\",\n      \"examples\": [\"2021-04-26T05:23:18Z\"]\n    },\n    \"backstage_room\": {\n      \"$ref\": \"#/$defs/BackstageRoom\"\n    },\n    \"private_group\": {\n      \"type\": \"array\",\n      \"items\": {\n        \"$ref\": \"#/$defs/PrivateGroup\"\n      }\n    }\n  },\n  \"required\": [\"archiving_start_time\"],\n\n  \"$defs\": {\n    \"BackstageRoom\": {\n      \"type\": \"object\",\n      \"properties\": {\n        \"id\": {\n          \"type\": \"string\",\n          \"description\": \"The ID of the Webinar backstage room.\"\n        },\n        \"createTime\": {\n          \"type\": \"string\",\n          \"pattern\": \"^(\\\\\\\\d{2}:\\\\\\\\d{2}:\\\\\\\\d{2})$\",\n          \"description\": \"Elapsed time from archiving start when the backstage room was created (HH:MM:SS).\",\n          \"examples\": [\"00:00:30\"]\n        },\n        \"deleteTime\": {\n          \"type\": \"string\",\n          \"pattern\": \"^(\\\\\\\\d{2}:\\\\\\\\d{2}:\\\\\\\\d{2})$\",\n          \"description\": \"Elapsed time from archiving start when the backstage room was deleted (HH:MM:SS).\",\n          \"examples\": [\"00:10:30\"]\n        },\n        \"actions\": {\n          \"type\": \"array\",\n          \"items\": { \"$ref\": \"#/$defs/Participant\" }\n        }\n      },\n      \"required\": [\"id\", \"createTime\", \"deleteTime\", \"actions\"]\n    },\n\n    \"PrivateGroup\": {\n      \"type\": \"object\",\n      \"properties\": {\n        \"id\": {\n          \"type\": \"string\",\n          \"description\": \"The ID of the private chat group.\"\n        },\n        \"createTime\": {\n          \"type\": \"string\",\n          \"pattern\": \"^(\\\\\\\\d{2}:\\\\\\\\d{2}:\\\\\\\\d{2})$\",\n          \"description\": \"Elapsed time from archiving start when the private chat group was created (HH:MM:SS).\",\n          \"examples\": [\"00:00:30\"]\n        },\n        \"deleteTime\": {\n          \"type\": \"string\",\n          \"pattern\": \"^(\\\\\\\\d{2}:\\\\\\\\d{2}:\\\\\\\\d{2})$\",\n          \"description\": \"Elapsed time from archiving start when the private chat group was deleted (HH:MM:SS).\", \n          \"examples\": [\"00:20:30\"]\n        },\n        \"actions\": {\n          \"type\": \"array\",\n          \"items\": { \"$ref\": \"#/$defs/Participant\" }\n        }\n      },\n      \"required\": [\"id\", \"createTime\", \"deleteTime\", \"actions\"]\n    },\n\n    \"Participant\": {\n      \"type\": \"object\",\n      \"properties\": {\n        \"email\": {\n          \"type\": \"string\",\n          \"format\": \"email\",\n          \"description\": \"The email of the participant.\"\n        },\n        \"joinTime\": {\n          \"type\": \"string\",\n          \"pattern\": \"^(\\\\\\\\d{2}:\\\\\\\\d{2}:\\\\\\\\d{2})$\",\n          \"description\": \"Elapsed time from archiving start when the participant joined (HH:MM:SS).\",\n          \"examples\": [\"00:00:30\"]\n        },\n        \"leaveTime\": {\n          \"type\": \"string\",\n          \"pattern\": \"^(\\\\\\\\d{2}:\\\\\\\\d{2}:\\\\\\\\d{2})$\",\n          \"description\": \"Elapsed time from archiving start when the participant left (HH:MM:SS).\",\n          \"examples\": [\"00:00:35\"]\n        }\n      },\n      \"required\": [\"joinTime\", \"leaveTime\"]\n    }\n  }\n}\n`})})]})}function h(n={}){let{wrapper:e}={...s(),...n.components};return e?(0,t.jsx)(e,{...n,children:(0,t.jsx)(m,{...n})}):m(n)}return A(Z);})();\n;return Component;",
      "frontmatter": {
        "title": "Archived JSON schemas",
        "description": "Chat message and subgroup activity JSON schemas."
      },
      "errors": [],
      "matter": {
        "content": "\n# Archived JSON schemas\n\nZoom returns the following JSON schemas when downloading chat messages and subgroup activity with the [Get a meeting's archived files](/docs/api/meetings/#tag/archiving/get/past_meetings/{meetingUUID}/archive_files) API.\n\n## In-meeting chat message\n\n```json\n{\n  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n  \"title\": \"Archiving Chat\",\n  \"type\": \"object\",\n  \"properties\": {\n    \"uuid\": {\n      \"type\": \"string\",\n      \"format\": \"uuid\",\n      \"description\": \"The meeting or webinar's universally unique ID (UUID). Each meeting or webinar instance generates its own UUID.\"\n    },\n    \"messages\": {\n      \"type\": \"array\",\n      \"description\": \"A list of archived chat messages.\",\n      \"items\": {\n        \"type\": \"object\",\n        \"properties\": {\n          \"msg_type\": {\n            \"type\": \"integer\",\n            \"enum\": [1, 2, 3, 4, 5, 6, 7],\n            \"description\": \"Message type. 1: public chat, 2: private chat, 3: deleted chat, 4: emoji reaction, 5: nonverbal feedback, 6: edited chat, 7: reply chat.\"\n          },\n          \"e_time\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(\\\\d{2}:\\\\d{2}:\\\\d{2})$\",\n            \"description\": \"Elapsed time after the meeting or webinar started (HH:MM:SS).\",\n            \"examples\": [\"00:05:30\", \"01:23:45\"]\n          },\n          \"s_id\": {\n            \"type\": \"string\",\n            \"description\": \"The sender's ID. If the user did not choose to display email and node_id, the value is empty.\"\n          },\n          \"s_name\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$\",\n            \"description\": \"The sender's name for this message, base64-encoded.\"\n          },\n          \"s_email\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$\",\n            \"description\": \"The sender's email for this message, base64-encoded. Empty if hidden.\"\n          },\n          \"e_archive_time\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(\\\\d{2}:\\\\d{2}:\\\\d{2})$\",\n            \"description\": \"Elapsed time since archiving started (HH:MM:SS).\",\n            \"examples\": [\"00:05:30\", \"01:23:45\"]\n          },\n          \"msg_id\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$\",\n            \"description\": \"The text message's unique ID or the unique ID of the deleted/edited message, base64-encoded.\"\n          },\n          \"msg_extended_type\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(\\\\d{2}){6}$\",\n            \"description\": \"The format XX XX XX XX XX XX of this message type from the left to right has 2-digit decimal pair numbers which indicates the type of the message. The information for each pair is listed here: 1st XX pair (Sender Location): 01-Meeting, 02-Webinar, 03-Breakout Room, 04-Waiting Room, 05-Backstage; 2nd XX pair (Receiver Location): 01-Meeting, 02-Webinar, 03-Breakout Room, 04-Waiting Room, 05-Backstage; 3rd XX pair (Message Type): 01-Sent, 02-Deleted, 03-Edited; 4th XX pair: 00-Null; 5th XX pair (Receiver): 01- Everyone in meeting, 02-(Webinar)Hosts and Panelists, 03-(Webinar)Attendee CC to Panelists and Hosts, 04-Direct message to User, 05-Waiting Room Participants, 06 - To all hosts (from Waiting Room), 07 - Private group chat; 6th XX pair: 00-Null.\"\n          },\n          \"r_id\": {\n            \"type\": \"string\",\n            \"description\": \"Receiver\u2019s ID. Required when msg_type=2 (private chat).\"\n          },\n          \"r_name\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$\",\n            \"description\": \"Receiver\u2019s name, base64-encoded. Required when msg_type=2.\"\n          },\n          \"r_email\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$\",\n            \"description\": \"Receiver\u2019s email, base64-encoded. Required when msg_type=2. Empty if hidden.\"\n          },\n          \"msg_content\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$\",\n            \"description\": \"Message content, format depends on msg_type. base64-encoded.\"\n          },\n          \"original_msg_id\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$\",\n            \"description\": \"The ID of the original message that was deleted or edited. base64-encoded.\"\n          },\n          \"parent_msg_id\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$\",\n            \"description\": \"The ID of the parent message being replied to. base64-encoded.\"\n          },\n          \"private_group_id\": {\n            \"type\": \"string\",\n            \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}(?:==)?|[A-Za-z0-9+/]{3}=?)?$\",\n            \"description\": \"ID of the private chat group. Used when msg_extended_type indicates private group (07). base64-encoded.\"\n          },\n          \"files\": {\n            \"type\": \"array\",\n            \"description\": \"List of files shared in this chat message.\",\n            \"items\": {\n              \"type\": \"object\",\n              \"properties\": {\n                \"fileId\": { \"type\": \"string\" },\n                \"fileName\": { \"type\": \"string\" },\n                \"fileSize\": { \"type\": \"integer\" }\n              },\n              \"required\": [\"fileId\", \"fileName\", \"fileSize\"]\n            }\n          },\n          \"rich_text\": {\n            \"type\": \"array\",\n            \"description\": \"Rich text formatting applied to parts of the message.\",\n            \"items\": {\n              \"type\": \"object\",\n              \"properties\": {\n                \"start_position\": { \"type\": \"integer\" },\n                \"end_position\": { \"type\": \"integer\" },\n                \"format_type\": {\n                  \"type\": \"string\",\n                  \"enum\": [\n                    \"Bold\", \"Italic\", \"Strikethrough\", \"BulletedList\", \"NumberedList\",\n                    \"Underline\", \"FontSize\", \"FontColor\", \"BackgroundColor\", \"LeftIndent\",\n                    \"Paragraph\", \"Quote\", \"AddLink\"\n                  ]\n                },\n                \"format_attr\": { \"type\": \"string\" }\n              },\n              \"required\": [\"start_position\", \"end_position\", \"format_type\"]\n            }\n          }\n        },\n        \"required\": [\n          \"msg_type\", \"e_time\", \"e_archive_time\", \"s_id\", \"s_name\", \"s_email\"\n        ],\n        \"allOf\": [\n          {\n            \"if\": { \"properties\": { \"msg_type\": { \"enum\": [1, 2, 3, 6, 7] } } },\n            \"then\": { \"required\": [\"msg_id\", \"msg_extended_type\"] }\n          },\n          {\n            \"if\": { \"properties\": { \"msg_type\": { \"const\": 2 } } },\n            \"then\": { \"required\": [\"r_id\", \"r_name\", \"r_email\"] }\n          },\n          {\n            \"if\": { \"properties\": { \"msg_type\": { \"enum\": [1, 2, 4, 6, 7] } } },\n            \"then\": {\n              \"properties\": {\n                \"msg_content\": {\n                  \"type\": \"string\",\n                  \"pattern\": \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$\",\n                  \"description\": \"The content of this text message in the meeting or webinar or the command of the emoji reaction in the meeting or webinar. base64 encoded.\"\n                }\n              },\n              \"required\": [\"msg_content\"]\n            }\n          },\n          {\n            \"if\": { \"properties\": { \"msg_type\": { \"const\": 5 } } },\n            \"then\": {\n              \"properties\": {\n                \"msg_content\": {\n                  \"type\": \"string\",\n                  \"pattern\": \"^[0-9]+$\",\n                  \"description\": \"The command of the nonverbal feedback in the meeting or webinar. For example: 1-raising hand, 2-yes, 3-no.\"\n                }\n              },\n              \"required\": [\"msg_content\"]\n            }\n          },\n          {\n            \"if\": { \"properties\": { \"msg_type\": { \"enum\": [3, 6] } } },\n            \"then\": { \"required\": [\"original_msg_id\"] }\n          },\n          {\n            \"if\": { \"properties\": { \"msg_type\": { \"const\": 7 } } },\n            \"then\": { \"required\": [\"parent_msg_id\"] }\n          }\n        ]\n      }\n    }\n  },\n  \"required\": [\"uuid\", \"messages\"]\n}\n```\n\n## Subgroup archiving activity\n\n```json\n{\n  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n  \"title\": \"Subgroup Archiving Activity\",\n  \"type\": \"object\",\n  \"properties\": {\n    \"archiving_start_time\": {\n      \"type\": \"string\",\n      \"format\": \"date-time\",\n      \"description\": \"The archiving starting timestamp in GMT format.\",\n      \"examples\": [\"2021-04-26T05:23:18Z\"]\n    },\n    \"backstage_room\": {\n      \"$ref\": \"#/$defs/BackstageRoom\"\n    },\n    \"private_group\": {\n      \"type\": \"array\",\n      \"items\": {\n        \"$ref\": \"#/$defs/PrivateGroup\"\n      }\n    }\n  },\n  \"required\": [\"archiving_start_time\"],\n\n  \"$defs\": {\n    \"BackstageRoom\": {\n      \"type\": \"object\",\n      \"properties\": {\n        \"id\": {\n          \"type\": \"string\",\n          \"description\": \"The ID of the Webinar backstage room.\"\n        },\n        \"createTime\": {\n          \"type\": \"string\",\n          \"pattern\": \"^(\\\\d{2}:\\\\d{2}:\\\\d{2})$\",\n          \"description\": \"Elapsed time from archiving start when the backstage room was created (HH:MM:SS).\",\n          \"examples\": [\"00:00:30\"]\n        },\n        \"deleteTime\": {\n          \"type\": \"string\",\n          \"pattern\": \"^(\\\\d{2}:\\\\d{2}:\\\\d{2})$\",\n          \"description\": \"Elapsed time from archiving start when the backstage room was deleted (HH:MM:SS).\",\n          \"examples\": [\"00:10:30\"]\n        },\n        \"actions\": {\n          \"type\": \"array\",\n          \"items\": { \"$ref\": \"#/$defs/Participant\" }\n        }\n      },\n      \"required\": [\"id\", \"createTime\", \"deleteTime\", \"actions\"]\n    },\n\n    \"PrivateGroup\": {\n      \"type\": \"object\",\n      \"properties\": {\n        \"id\": {\n          \"type\": \"string\",\n          \"description\": \"The ID of the private chat group.\"\n        },\n        \"createTime\": {\n          \"type\": \"string\",\n          \"pattern\": \"^(\\\\d{2}:\\\\d{2}:\\\\d{2})$\",\n          \"description\": \"Elapsed time from archiving start when the private chat group was created (HH:MM:SS).\",\n          \"examples\": [\"00:00:30\"]\n        },\n        \"deleteTime\": {\n          \"type\": \"string\",\n          \"pattern\": \"^(\\\\d{2}:\\\\d{2}:\\\\d{2})$\",\n          \"description\": \"Elapsed time from archiving start when the private chat group was deleted (HH:MM:SS).\", \n          \"examples\": [\"00:20:30\"]\n        },\n        \"actions\": {\n          \"type\": \"array\",\n          \"items\": { \"$ref\": \"#/$defs/Participant\" }\n        }\n      },\n      \"required\": [\"id\", \"createTime\", \"deleteTime\", \"actions\"]\n    },\n\n    \"Participant\": {\n      \"type\": \"object\",\n      \"properties\": {\n        \"email\": {\n          \"type\": \"string\",\n          \"format\": \"email\",\n          \"description\": \"The email of the participant.\"\n        },\n        \"joinTime\": {\n          \"type\": \"string\",\n          \"pattern\": \"^(\\\\d{2}:\\\\d{2}:\\\\d{2})$\",\n          \"description\": \"Elapsed time from archiving start when the participant joined (HH:MM:SS).\",\n          \"examples\": [\"00:00:30\"]\n        },\n        \"leaveTime\": {\n          \"type\": \"string\",\n          \"pattern\": \"^(\\\\d{2}:\\\\d{2}:\\\\d{2})$\",\n          \"description\": \"Elapsed time from archiving start when the participant left (HH:MM:SS).\",\n          \"examples\": [\"00:00:35\"]\n        }\n      },\n      \"required\": [\"joinTime\", \"leaveTime\"]\n    }\n  }\n}\n```\n",
        "data": {
          "title": "Archived JSON schemas",
          "description": "Chat message and subgroup activity JSON schemas."
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/references/archived-json-schema.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
