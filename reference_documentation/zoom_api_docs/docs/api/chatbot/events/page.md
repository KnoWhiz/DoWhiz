# Chatbot Webhooks

- Source URL: https://developers.zoom.us/docs/api/chatbot/events/
- Snapshot path: docs/api/chatbot/events
- Generated (UTC): 2026-02-07T22:40:10.500225+00:00

## Frontmatter

```json
{
  "title": "Chatbot Webhooks",
  "keywords": "zoom, marketplace, chatbot, chat, bot, webhook, event,  interactive message, button, message, action",
  "description": "The chatbot webhooks allow developers to receive events for [chatbot](https://developers.zoom.us/docs/team-chat-apps/) features.  \r\n\r\nNote regarding JSON payloads for guest users:\r\nBe aware that guest users (users not signed in to Zoom or who are members of a different account) are assigned temporary IDs that are valid only for the duration of that meeting or chat channel. Data for guest users in JSON payloads is associated with these temporary IDs.\r\n\r\n",
  "skipQualtrics": true
}
```

## Content

No MDX content found for this page.

## OpenAPI

```json
{
  "openapi": "3.1.0",
  "info": {
    "title": "Chatbot",
    "description": "The chatbot webhooks allow developers to receive events for [chatbot](https://developers.zoom.us/docs/team-chat-apps/) features.  \r\n\r\nNote regarding JSON payloads for guest users:\r\nBe aware that guest users (users not signed in to Zoom or who are members of a different account) are assigned temporary IDs that are valid only for the duration of that meeting or chat channel. Data for guest users in JSON payloads is associated with these temporary IDs.\r\n\r\n",
    "version": "1.0.0"
  },
  "paths": {
    "team_chat.link_shared": {
      "post": {
        "tags": [
          "team_chat"
        ],
        "operationId": "team_chat.link_shared",
        "requestBody": {
          "description": "# team_chat.link_shared\nThe **Chat message link shared** \u00a0event triggers when a user sends a chat message to a contact or a chat channel, and when the app is installed for the user. This message must contain the link. The link's domain is registered by the app in Marketplace. A POST request with a mention message is sent to the [Bot Endpoint URL](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-chatbot-app#set-features) specified in the app's dashboard.\n## Prerequisites\n* A Pro or a higher [plan](https://zoom.us/pricing).\n\n\n**Event type**: `team_chat.link_shared`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "required": [
                  "event",
                  "event_ts",
                  "response_url",
                  "payload"
                ],
                "properties": {
                  "event": {
                    "type": "string",
                    "description": "The name of the event.",
                    "enum": [
                      "team_chat.link_shared"
                    ]
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "A timestamp at which the event occurred."
                  },
                  "response_url": {
                    "type": "string",
                    "description": "A callback uri for app to response preview result."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "object",
                      "operator",
                      "operator_member_id",
                      "operator_id",
                      "by_external_user"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who sent the app mention message."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user who sent the app mention message."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user who sent the app mention message."
                      },
                      "operator_member_id": {
                        "type": "string",
                        "description": "The member ID of the user who sent the app mention message."
                      },
                      "by_external_user": {
                        "type": "boolean",
                        "description": "Whether this action was performed by an external user."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the app mention message.",
                        "required": [
                          "type",
                          "link",
                          "trigger_id",
                          "timestamp",
                          "date_time"
                        ],
                        "properties": {
                          "message_id": {
                            "type": "string",
                            "description": "The message's ID. If the preview is provided in an unfurl for a posted message, rather than a link inside the message composer, we will simply ignore this property."
                          },
                          "type": {
                            "type": "string",
                            "description": "The type of message: \n* to_contact \u2014 The message was sent to a contact. \n* to_channel \u2014 The message was sent to a chat channel.",
                            "enum": [
                              "to_contact",
                              "to_channel"
                            ]
                          },
                          "channel_id": {
                            "type": "string",
                            "description": "The chat channel's ID."
                          },
                          "channel_name": {
                            "type": "string",
                            "description": "The chat channel's name."
                          },
                          "contact_id": {
                            "type": "string",
                            "description": "The message contact's user ID. If the user ID does not belong to the same account as the current user, this field will be empty."
                          },
                          "contact_member_id": {
                            "type": "string",
                            "description": "The message contact's member ID."
                          },
                          "trigger_id": {
                            "type": "string",
                            "description": "The unique identification of this webook event, app can use this trigger_id to invoke openAPI"
                          },
                          "link": {
                            "type": "string",
                            "description": "The link content in message."
                          },
                          "reply_main_message_id": {
                            "type": "string",
                            "description": "The ID of the parent message in the chat thread.It's only available if the message is a reply to a main message in a thread."
                          },
                          "date_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The date and time at which the message was sent, in yyyy-MM-dd'T'HH:mm:ss'Z' format."
                          },
                          "timestamp": {
                            "type": "integer",
                            "format": "int64",
                            "description": "The timestamp for when the message was sent."
                          }
                        },
                        "additionalProperties": false
                      }
                    },
                    "additionalProperties": false
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"team_chat.link_shared\",\n  \"event_ts\": 1696914490200,\n  \"response_url\": \"https://api.zoom.us/v2/im/chat/users/exampleUserId/unfurls/exampleTriggerId\",\n  \"payload\": {\n    \"account_id\": \"exampleAccountId\",\n    \"operator_id\": \"exampleuserId\",\n    \"operator\": \"example@zoom.us\",\n    \"operator_member_id\": \"R4VM29Oj0fVM2hhEmSKVM2hhezJTezJTKVM2hezJT2hezJTSK\",\n    \"by_external_user\": false,\n    \"object\": {\n      \"message_id\": \"5DD614F4-DD19-4C39-8D03-93C4D0A0C839\",\n      \"reply_main_message_id\": \"5DD614F4-DD19-4C39-8D03-93C4D0A0C838\",\n      \"type\": \"to_channel\",\n      \"channel_id\": \"xxxxxxx\",\n      \"channel_name\": \"Team Chat\",\n      \"link\": \"https://sg.com/1234w\",\n      \"trigger_id\": \"exampleTriggerId\",\n      \"date_time\": \"2023-10-10T05:08:10Z\",\n      \"timestamp\": 1696914490157\n    }\n  }\n}"
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "required": [
                    "event",
                    "event_ts",
                    "response_url",
                    "payload"
                  ],
                  "properties": {
                    "event": {
                      "type": "string",
                      "description": "The name of the event.",
                      "enum": [
                        "team_chat.link_shared"
                      ]
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "A timestamp at which the event occurred."
                    },
                    "response_url": {
                      "type": "string",
                      "description": "A callback uri for app to response preview result."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "object",
                        "operator",
                        "operator_member_id",
                        "operator_id",
                        "by_external_user"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who sent the app mention message."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user who sent the app mention message."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user who sent the app mention message."
                        },
                        "operator_member_id": {
                          "type": "string",
                          "description": "The member ID of the user who sent the app mention message."
                        },
                        "by_external_user": {
                          "type": "boolean",
                          "description": "Whether this action was performed by an external user."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the app mention message.",
                          "required": [
                            "type",
                            "link",
                            "trigger_id",
                            "timestamp",
                            "date_time"
                          ],
                          "properties": {
                            "message_id": {
                              "type": "string",
                              "description": "The message's ID. If the preview is provided in an unfurl for a posted message, rather than a link inside the message composer, we will simply ignore this property."
                            },
                            "type": {
                              "type": "string",
                              "description": "The type of message: \n* to_contact \u2014 The message was sent to a contact. \n* to_channel \u2014 The message was sent to a chat channel.",
                              "enum": [
                                "to_contact",
                                "to_channel"
                              ]
                            },
                            "channel_id": {
                              "type": "string",
                              "description": "The chat channel's ID."
                            },
                            "channel_name": {
                              "type": "string",
                              "description": "The chat channel's name."
                            },
                            "contact_id": {
                              "type": "string",
                              "description": "The message contact's user ID. If the user ID does not belong to the same account as the current user, this field will be empty."
                            },
                            "contact_member_id": {
                              "type": "string",
                              "description": "The message contact's member ID."
                            },
                            "trigger_id": {
                              "type": "string",
                              "description": "The unique identification of this webook event, app can use this trigger_id to invoke openAPI"
                            },
                            "link": {
                              "type": "string",
                              "description": "The link content in message."
                            },
                            "reply_main_message_id": {
                              "type": "string",
                              "description": "The ID of the parent message in the chat thread.It's only available if the message is a reply to a main message in a thread."
                            },
                            "date_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The date and time at which the message was sent, in yyyy-MM-dd'T'HH:mm:ss'Z' format."
                            },
                            "timestamp": {
                              "type": "integer",
                              "format": "int64",
                              "description": "The timestamp for when the message was sent."
                            }
                          },
                          "additionalProperties": false
                        }
                      },
                      "additionalProperties": false
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"team_chat.link_shared\",\n  \"event_ts\": 1696914490200,\n  \"response_url\": \"https://api.zoom.us/v2/im/chat/users/exampleUserId/unfurls/exampleTriggerId\",\n  \"payload\": {\n    \"account_id\": \"exampleAccountId\",\n    \"operator_id\": \"exampleuserId\",\n    \"operator\": \"example@zoom.us\",\n    \"operator_member_id\": \"R4VM29Oj0fVM2hhEmSKVM2hhezJTezJTKVM2hezJT2hezJTSK\",\n    \"by_external_user\": false,\n    \"object\": {\n      \"message_id\": \"5DD614F4-DD19-4C39-8D03-93C4D0A0C839\",\n      \"reply_main_message_id\": \"5DD614F4-DD19-4C39-8D03-93C4D0A0C838\",\n      \"type\": \"to_channel\",\n      \"channel_id\": \"xxxxxxx\",\n      \"channel_name\": \"Team Chat\",\n      \"link\": \"https://sg.com/1234w\",\n      \"trigger_id\": \"exampleTriggerId\",\n      \"date_time\": \"2023-10-10T05:08:10Z\",\n      \"timestamp\": 1696914490157\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "interactive_message_fields_editable": {
      "post": {
        "tags": [
          "interactive_message_fields_editable"
        ],
        "operationId": "interactive_message_fields_editable",
        "requestBody": {
          "description": "# interactive_message_fields_editable\n\nAfter a user edits the text of a Chatbot [message with Editable Text](https://marketplace.zoom.us/docs/guides/chatbots/customizing-messages/message-with-editable-text), the `interactive_message_editable` event is invoked and a POST request with data about the event is sent to the [Bot Endpoint URL](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-chatbot-app#set-features) specified in the app's dashboard.\n\nThe `editItem` object displays the changes in edited text with `origin` and `target` values.\n\n\n\n**Event type**: `interactive_message_fields_editable`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "event": {
                    "type": "string",
                    "description": "The name of the event."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "The timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format."
                  },
                  "callback_url": {
                    "type": "string",
                    "description": "Use this endpoint to send a one-time asynchronous response within 30 minutes of receiving the event. <br> The response payload should follow the same structure as the [Send Chatbot Messages](https://developers.zoom.us/docs/api/chatbot/#tag/chatbot-messages/POST/im/chat/messages) API with required property **content** and optional properties *reply_to*, *visible_to_user* and *is_markdown_support*. <br> Include the **callback_token** value from the event as **Bearer token in the Authorization header** of your HTTP POST request."
                  },
                  "callback_token": {
                    "type": "string",
                    "description": "Use the generated token value with the **callback_url** value to send a one-time asynchronous response. The token only lasts for 30 mins. Include the **callback_token** as a **Bearer token in the Authorization header** of your HTTP request. \n Example: \n\n \ncurl --request POST \\\n  --url '{callback_url}' \\\n  --header 'authorization: Bearer {callback_token}' \\\n  --header 'content-type: application/json'\\\n  --data-raw '{\"content\":{\"head\":{\"text\":\"Response Header\"},\"body\":[{\"type\":\"message\",\"text\":\"This is an asynchronous chatbot response.\"}]}}'"
                  },
                  "payload": {
                    "type": "object",
                    "properties": {
                      "accountId": {
                        "type": "string",
                        "description": "The account ID of the user sending the command."
                      },
                      "channelName": {
                        "type": "string",
                        "description": "The channel in which the message was sent. "
                      },
                      "fieldEditItem": {
                        "type": "object",
                        "description": "The item that the user edits.",
                        "properties": {
                          "currentValue": {
                            "type": "string"
                          },
                          "key": {
                            "type": "string"
                          },
                          "newValue": {
                            "type": "string"
                          }
                        }
                      },
                      "messageId": {
                        "type": "string",
                        "description": "The unique ID of the message."
                      },
                      "original": {
                        "type": "object",
                        "description": "The original message sent to the user.",
                        "properties": {
                          "head": {
                            "type": "object",
                            "properties": {
                              "sub_head": {
                                "type": "object",
                                "properties": {
                                  "text": {
                                    "type": "string"
                                  }
                                }
                              },
                              "text": {
                                "type": "string"
                              }
                            }
                          },
                          "body": {
                            "type": "array",
                            "items": {
                              "type": "object",
                              "properties": {
                                "type": {
                                  "type": "string"
                                },
                                "items": {
                                  "type": "array",
                                  "items": {
                                    "type": "object",
                                    "properties": {
                                      "editable": {
                                        "type": "string"
                                      },
                                      "value": {
                                        "type": "string"
                                      },
                                      "key": {
                                        "type": "string"
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      },
                      "robotJid": {
                        "type": "string",
                        "description": "The unique credential of the Chatbot."
                      },
                      "timestamp": {
                        "type": "string",
                        "description": "The time command message was sent."
                      },
                      "toJid": {
                        "type": "string",
                        "description": "The JID of the channel or user the message was sent to."
                      },
                      "userId": {
                        "type": "string",
                        "description": "The user who sends the command."
                      },
                      "userJid": {
                        "type": "string",
                        "description": "The JID of user who is sending the command."
                      },
                      "userName": {
                        "type": "string",
                        "description": "The user's display name."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"interactive_message_fields_editable\",\n  \"event_ts\": 1560891222018,\n  \"callback_url\": \"https://api.zoom.us/v2/im/chat/callback/bKFS3ybORiWbJx6VM6Qe1A\",\n  \"callback_token\": \"eyJzdiI6IjAwMDAwMSIsInR5cCI6IkpXVCIsImFsZyI6IkVTMjU2In0.eyJhdWQiOiJib3QiLCJ1aWQiOiJWV2ZQa1NENVJvMk5xRWUyR1VMa2F3IiwiY2xpZW50SWQiOiJRQmZVaEFyUlNHVVFVclM2ZDFXVnciLCJpc3MiOiJ6b29tIiwiZXhwIjoxNzYwMzYzNzc5LCJpYXQiOjE3NjAzMzM3NzksImFpZCI6InQ4dURPZWtYUkx5b3ExR0M4UFBaTVEifQ.wBeNSNeYLQYJD1Lp0O0EnnfyCYNzF03tEwFin0-hRV7UfECLmHsW7NnQ9EEPGxOokDKnQb-pRPd_iBX4JKfg_Q\",\n  \"payload\": {\n    \"accountId\": \"gVcjZnWWRLWvv_GtyGuaxg\",\n    \"channelName\": \"Jill Chill\",\n    \"fieldEditItem\": {\n      \"currentValue\": \"\",\n      \"key\": \"Title\",\n      \"newValue\": \"Software Engineer\"\n    },\n    \"messageId\": \"20190618205323327_EZjIqCQ_aw1\",\n    \"original\": {\n      \"head\": {\n        \"sub_head\": {\n          \"text\": \"I am a sub header\"\n        },\n        \"text\": \"I am a header\"\n      },\n      \"body\": [\n        {\n          \"type\": \"fields\",\n          \"items\": [\n            {\n              \"editable\": true,\n              \"value\": \" \",\n              \"key\": \"Name\"\n            },\n            {\n              \"editable\": true,\n              \"value\": \" \",\n              \"key\": \"Title\"\n            },\n            {\n              \"editable\": false,\n              \"value\": \"Example Company\",\n              \"key\": \"Company\"\n            }\n          ]\n        }\n      ]\n    },\n    \"robotJid\": \"v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"timestamp\": 1560891222018,\n    \"toJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"userId\": \"KdYKjnimT4KPd8KKdQt9FQ\",\n    \"userJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com\",\n    \"userName\": \"Jill Chill\"\n  }\n}"
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "event": {
                      "type": "string",
                      "description": "The name of the event."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "The timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format."
                    },
                    "callback_url": {
                      "type": "string",
                      "description": "Use this endpoint to send a one-time asynchronous response within 30 minutes of receiving the event. <br> The response payload should follow the same structure as the [Send Chatbot Messages](https://developers.zoom.us/docs/api/chatbot/#tag/chatbot-messages/POST/im/chat/messages) API with required property **content** and optional properties *reply_to*, *visible_to_user* and *is_markdown_support*. <br> Include the **callback_token** value from the event as **Bearer token in the Authorization header** of your HTTP POST request."
                    },
                    "callback_token": {
                      "type": "string",
                      "description": "Use the generated token value with the **callback_url** value to send a one-time asynchronous response. The token only lasts for 30 mins. Include the **callback_token** as a **Bearer token in the Authorization header** of your HTTP request. \n Example: \n\n \ncurl --request POST \\\n  --url '{callback_url}' \\\n  --header 'authorization: Bearer {callback_token}' \\\n  --header 'content-type: application/json'\\\n  --data-raw '{\"content\":{\"head\":{\"text\":\"Response Header\"},\"body\":[{\"type\":\"message\",\"text\":\"This is an asynchronous chatbot response.\"}]}}'"
                    },
                    "payload": {
                      "type": "object",
                      "properties": {
                        "accountId": {
                          "type": "string",
                          "description": "The account ID of the user sending the command."
                        },
                        "channelName": {
                          "type": "string",
                          "description": "The channel in which the message was sent. "
                        },
                        "fieldEditItem": {
                          "type": "object",
                          "description": "The item that the user edits.",
                          "properties": {
                            "currentValue": {
                              "type": "string"
                            },
                            "key": {
                              "type": "string"
                            },
                            "newValue": {
                              "type": "string"
                            }
                          }
                        },
                        "messageId": {
                          "type": "string",
                          "description": "The unique ID of the message."
                        },
                        "original": {
                          "type": "object",
                          "description": "The original message sent to the user.",
                          "properties": {
                            "head": {
                              "type": "object",
                              "properties": {
                                "sub_head": {
                                  "type": "object",
                                  "properties": {
                                    "text": {
                                      "type": "string"
                                    }
                                  }
                                },
                                "text": {
                                  "type": "string"
                                }
                              }
                            },
                            "body": {
                              "type": "array",
                              "items": {
                                "type": "object",
                                "properties": {
                                  "type": {
                                    "type": "string"
                                  },
                                  "items": {
                                    "type": "array",
                                    "items": {
                                      "type": "object",
                                      "properties": {
                                        "editable": {
                                          "type": "string"
                                        },
                                        "value": {
                                          "type": "string"
                                        },
                                        "key": {
                                          "type": "string"
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        },
                        "robotJid": {
                          "type": "string",
                          "description": "The unique credential of the Chatbot."
                        },
                        "timestamp": {
                          "type": "string",
                          "description": "The time command message was sent."
                        },
                        "toJid": {
                          "type": "string",
                          "description": "The JID of the channel or user the message was sent to."
                        },
                        "userId": {
                          "type": "string",
                          "description": "The user who sends the command."
                        },
                        "userJid": {
                          "type": "string",
                          "description": "The JID of user who is sending the command."
                        },
                        "userName": {
                          "type": "string",
                          "description": "The user's display name."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"interactive_message_fields_editable\",\n  \"event_ts\": 1560891222018,\n  \"callback_url\": \"https://api.zoom.us/v2/im/chat/callback/bKFS3ybORiWbJx6VM6Qe1A\",\n  \"callback_token\": \"eyJzdiI6IjAwMDAwMSIsInR5cCI6IkpXVCIsImFsZyI6IkVTMjU2In0.eyJhdWQiOiJib3QiLCJ1aWQiOiJWV2ZQa1NENVJvMk5xRWUyR1VMa2F3IiwiY2xpZW50SWQiOiJRQmZVaEFyUlNHVVFVclM2ZDFXVnciLCJpc3MiOiJ6b29tIiwiZXhwIjoxNzYwMzYzNzc5LCJpYXQiOjE3NjAzMzM3NzksImFpZCI6InQ4dURPZWtYUkx5b3ExR0M4UFBaTVEifQ.wBeNSNeYLQYJD1Lp0O0EnnfyCYNzF03tEwFin0-hRV7UfECLmHsW7NnQ9EEPGxOokDKnQb-pRPd_iBX4JKfg_Q\",\n  \"payload\": {\n    \"accountId\": \"gVcjZnWWRLWvv_GtyGuaxg\",\n    \"channelName\": \"Jill Chill\",\n    \"fieldEditItem\": {\n      \"currentValue\": \"\",\n      \"key\": \"Title\",\n      \"newValue\": \"Software Engineer\"\n    },\n    \"messageId\": \"20190618205323327_EZjIqCQ_aw1\",\n    \"original\": {\n      \"head\": {\n        \"sub_head\": {\n          \"text\": \"I am a sub header\"\n        },\n        \"text\": \"I am a header\"\n      },\n      \"body\": [\n        {\n          \"type\": \"fields\",\n          \"items\": [\n            {\n              \"editable\": true,\n              \"value\": \" \",\n              \"key\": \"Name\"\n            },\n            {\n              \"editable\": true,\n              \"value\": \" \",\n              \"key\": \"Title\"\n            },\n            {\n              \"editable\": false,\n              \"value\": \"Example Company\",\n              \"key\": \"Company\"\n            }\n          ]\n        }\n      ]\n    },\n    \"robotJid\": \"v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"timestamp\": 1560891222018,\n    \"toJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"userId\": \"KdYKjnimT4KPd8KKdQt9FQ\",\n    \"userJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com\",\n    \"userName\": \"Jill Chill\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "bot_notification": {
      "post": {
        "tags": [
          "bot_notification"
        ],
        "operationId": "bot_notification",
        "requestBody": {
          "description": "# bot_notification\n\nAfter a user enters a [User Command](https://marketplace.zoom.us/docs/guides/chatbots/sending-messages#user-commands), a slash command unique to the application, Chatbots send a POST request associated with the `bot_notification` event to the [Bot Endpoint URL](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-chatbot-app#set-features) with a JSON payload containing all text entered following the command. This payload contains information on the user and event including the account, channel, timestamp, and user name.\n\nThe `cmd` value reflects the text entered by the user after the slash command. The `userName` is the string value of the user's displayed name, and `channelName` is the string value of the channel in which the command was sent.\n\n\n\n**Event type**: `bot_notification`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "event": {
                    "type": "string",
                    "description": "Name of the event."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "Timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format."
                  },
                  "payload": {
                    "type": "object",
                    "properties": {
                      "accountId": {
                        "type": "string",
                        "description": "The account ID of the user sending the command."
                      },
                      "channelName": {
                        "type": "string",
                        "description": "Channel in which the message was sent. "
                      },
                      "cmd": {
                        "type": "string",
                        "description": "Slash command entered by the user."
                      },
                      "name": {
                        "type": "string",
                        "description": "Chatbot Name."
                      },
                      "robotJid": {
                        "type": "string",
                        "description": "Unique credential of the Chatbot."
                      },
                      "timestamp": {
                        "type": "integer",
                        "format": "date-time",
                        "description": "Time command message was sent."
                      },
                      "toJid": {
                        "type": "string",
                        "description": "JID of the channel or user the message was sent to."
                      },
                      "userId": {
                        "type": "string",
                        "description": "User sending the command."
                      },
                      "userJid": {
                        "type": "string",
                        "description": "JID of user who is sending the command."
                      },
                      "userName": {
                        "type": "string",
                        "description": "User's display name."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"bot_notification\",\n  \"event_ts\": 1560796234686,\n  \"payload\": {\n      \"accountId\": \"asgVcjZnWWRLWvv_GtyGuaxg\",\n      \"channelName\": \"Photos\",\n      \"cmd\": \"island\",\n      \"name\": \"Unsplash\",\n      \"robotJid\": \"v10r4uxexurcasg-pwh8hyh7sg@example.com\",\n      \"timestamp\": 1560796234686,\n      \"toJid\": \"b1c841dc7b0b4as69287e6be05c7f93f25@conference.example.com\",\n      \"userId\": \"KdYKjnimT4asKPd8KKdQt9FQ\",\n      \"userJid\": \"kdykjnimtas4kpd8kkdqt9fq@example.com\",\n      \"userName\": \"Jill Chill\"\n      }\n}"
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "event": {
                      "type": "string",
                      "description": "Name of the event."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "Timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format."
                    },
                    "payload": {
                      "type": "object",
                      "properties": {
                        "accountId": {
                          "type": "string",
                          "description": "The account ID of the user sending the command."
                        },
                        "channelName": {
                          "type": "string",
                          "description": "Channel in which the message was sent. "
                        },
                        "cmd": {
                          "type": "string",
                          "description": "Slash command entered by the user."
                        },
                        "name": {
                          "type": "string",
                          "description": "Chatbot Name."
                        },
                        "robotJid": {
                          "type": "string",
                          "description": "Unique credential of the Chatbot."
                        },
                        "timestamp": {
                          "type": "integer",
                          "format": "date-time",
                          "description": "Time command message was sent."
                        },
                        "toJid": {
                          "type": "string",
                          "description": "JID of the channel or user the message was sent to."
                        },
                        "userId": {
                          "type": "string",
                          "description": "User sending the command."
                        },
                        "userJid": {
                          "type": "string",
                          "description": "JID of user who is sending the command."
                        },
                        "userName": {
                          "type": "string",
                          "description": "User's display name."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"bot_notification\",\n  \"event_ts\": 1560796234686,\n  \"payload\": {\n      \"accountId\": \"asgVcjZnWWRLWvv_GtyGuaxg\",\n      \"channelName\": \"Photos\",\n      \"cmd\": \"island\",\n      \"name\": \"Unsplash\",\n      \"robotJid\": \"v10r4uxexurcasg-pwh8hyh7sg@example.com\",\n      \"timestamp\": 1560796234686,\n      \"toJid\": \"b1c841dc7b0b4as69287e6be05c7f93f25@conference.example.com\",\n      \"userId\": \"KdYKjnimT4asKPd8KKdQt9FQ\",\n      \"userJid\": \"kdykjnimtas4kpd8kkdqt9fq@example.com\",\n      \"userName\": \"Jill Chill\"\n      }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "interactive_message_actions": {
      "post": {
        "tags": [
          "interactive_message_actions"
        ],
        "operationId": "interactive_message_actions",
        "requestBody": {
          "description": "# interactive_message_actions\n\nAfter a user clicks a [button in a Chatbot message](https://marketplace.zoom.us/docs/guides/chatbots/customizing-messages/message-with-buttons), the `interactive_message_actions` event is invoked and a POST request with data on the selected option is sent to the [Bot Endpoint URL](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-chatbot-app#set-features) specified in the app's dashboard.\n\nThe `actionItem` object displays the value chosen by the user from the `items` array of the original message from the Chatbot.\n\n\n\n**Event type**: `interactive_message_actions`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "event": {
                    "type": "string",
                    "description": "The name of the event."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "The timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format."
                  },
                  "callback_url": {
                    "type": "string",
                    "description": "Use this endpoint to send a one-time asynchronous response within 30 minutes of receiving the event. <br> The response payload should follow the same structure as the [Send Chatbot Messages](https://developers.zoom.us/docs/api/chatbot/#tag/chatbot-messages/POST/im/chat/messages) API with required property **content** and optional properties *reply_to*, *visible_to_user* and *is_markdown_support*. <br> Include the **callback_token** value from the event as **Bearer token in the Authorization header** of your HTTP POST request."
                  },
                  "callback_token": {
                    "type": "string",
                    "description": "Use the generated token value with the **callback_url** value to send a one-time asynchronous response. The token only lasts for 30 mins. Include the **callback_token** as a **Bearer token in the Authorization header** of your HTTP request. \n Example: \n\n \ncurl --request POST \\\n  --url '{callback_url}' \\\n  --header 'authorization: Bearer {callback_token}' \\\n  --header 'content-type: application/json'\\\n  --data-raw '{\"content\":{\"head\":{\"text\":\"Response Header\"},\"body\":[{\"type\":\"message\",\"text\":\"This is an asynchronous chatbot response.\"}]}}'"
                  },
                  "payload": {
                    "type": "object",
                    "properties": {
                      "accountId": {
                        "type": "string",
                        "description": "The account ID of the user sending the command."
                      },
                      "actionItem": {
                        "type": "object",
                        "description": "Action taken by the user.",
                        "properties": {
                          "text": {
                            "type": "string"
                          },
                          "value": {
                            "type": "string"
                          }
                        }
                      },
                      "channelName": {
                        "type": "string",
                        "description": "Channel in which the message was sent."
                      },
                      "messageId": {
                        "type": "string",
                        "description": "Unique ID of the message."
                      },
                      "original": {
                        "type": "object",
                        "description": "Original message sent to the user.",
                        "properties": {
                          "head": {
                            "type": "object",
                            "properties": {
                              "sub_head": {
                                "type": "object",
                                "properties": {
                                  "text": {
                                    "type": "string"
                                  }
                                }
                              },
                              "text": {
                                "type": "string"
                              }
                            }
                          },
                          "body": {
                            "type": "array",
                            "items": {
                              "type": "object",
                              "properties": {
                                "type": {
                                  "type": "string"
                                },
                                "items": {
                                  "type": "array",
                                  "items": {
                                    "type": "object",
                                    "properties": {
                                      "style": {
                                        "type": "string"
                                      },
                                      "text": {
                                        "type": "string"
                                      },
                                      "value": {
                                        "type": "string"
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      },
                      "robotJid": {
                        "type": "string",
                        "description": "Unique credential of the Chatbot."
                      },
                      "timestamp": {
                        "type": "integer",
                        "description": "Time command message was sent."
                      },
                      "toJid": {
                        "type": "string",
                        "description": "JID of the channel or user the message was sent to."
                      },
                      "userId": {
                        "type": "string",
                        "description": "User sending the command."
                      },
                      "userJid": {
                        "type": "string",
                        "description": "JID of user who is sending the command."
                      },
                      "userName": {
                        "type": "string",
                        "description": "User's display name."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"interactive_message_actions\",\n  \"event_ts\": 1560891332219,\n  \"callback_url\": \"https://api.zoom.us/v2/im/chat/callback/bKFS3ybORiWbJx6VM6Qe1A\",\n  \"callback_token\": \"eyJzdiI6IjAwMDAwMSIsInR5cCI6IkpXVCIsImFsZyI6IkVTMjU2In0.eyJhdWQiOiJib3QiLCJ1aWQiOiJWV2ZQa1NENVJvMk5xRWUyR1VMa2F3IiwiY2xpZW50SWQiOiJRQmZVaEFyUlNHVVFVclM2ZDFXVnciLCJpc3MiOiJ6b29tIiwiZXhwIjoxNzYwMzYzNzc5LCJpYXQiOjE3NjAzMzM3NzksImFpZCI6InQ4dURPZWtYUkx5b3ExR0M4UFBaTVEifQ.wBeNSNeYLQYJD1Lp0O0EnnfyCYNzF03tEwFin0-hRV7UfECLmHsW7NnQ9EEPGxOokDKnQb-pRPd_iBX4JKfg_Q\",\n  \"payload\": {\n    \"accountId\": \"gVcjZnWWRLWvv_GtyGuaxg\",\n    \"actionItem\": {\n      \"text\": \"Add\",\n      \"value\": \"add\"\n    },\n    \"channelName\": \"Jill Chill\",\n    \"messageId\": \"20190618205516496_ynICmz8_aw1\",\n    \"original\": {\n      \"head\": {\n        \"sub_head\": {\n          \"text\": \"I am a sub header\"\n        },\n        \"text\": \"I am a header\"\n      },\n      \"body\": [\n        {\n          \"type\": \"actions\",\n          \"items\": [\n            {\n              \"style\": \"Primary\",\n              \"text\": \"Add\",\n              \"value\": \"add\"\n            },\n            {\n              \"style\": \"Default\",\n              \"text\": \"Update\",\n              \"value\": \"update\"\n            },\n            {\n              \"style\": \"Danger\",\n              \"text\": \"Delete\",\n              \"value\": \"delete\"\n            },\n            {\n              \"style\": \"Disabled\",\n              \"text\": \"Disabled\",\n              \"value\": \"disable\"\n            }\n          ]\n        }\n      ]\n    },\n    \"robotJid\": \"v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"timestamp\": 1560891332219,\n    \"toJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"userId\": \"KdYKjnimT4KPd8KKdQt9FQ\",\n    \"userJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com\",\n    \"userName\": \"Jill Chill\"\n  }\n}"
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "event": {
                      "type": "string",
                      "description": "The name of the event."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "The timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format."
                    },
                    "callback_url": {
                      "type": "string",
                      "description": "Use this endpoint to send a one-time asynchronous response within 30 minutes of receiving the event. <br> The response payload should follow the same structure as the [Send Chatbot Messages](https://developers.zoom.us/docs/api/chatbot/#tag/chatbot-messages/POST/im/chat/messages) API with required property **content** and optional properties *reply_to*, *visible_to_user* and *is_markdown_support*. <br> Include the **callback_token** value from the event as **Bearer token in the Authorization header** of your HTTP POST request."
                    },
                    "callback_token": {
                      "type": "string",
                      "description": "Use the generated token value with the **callback_url** value to send a one-time asynchronous response. The token only lasts for 30 mins. Include the **callback_token** as a **Bearer token in the Authorization header** of your HTTP request. \n Example: \n\n \ncurl --request POST \\\n  --url '{callback_url}' \\\n  --header 'authorization: Bearer {callback_token}' \\\n  --header 'content-type: application/json'\\\n  --data-raw '{\"content\":{\"head\":{\"text\":\"Response Header\"},\"body\":[{\"type\":\"message\",\"text\":\"This is an asynchronous chatbot response.\"}]}}'"
                    },
                    "payload": {
                      "type": "object",
                      "properties": {
                        "accountId": {
                          "type": "string",
                          "description": "The account ID of the user sending the command."
                        },
                        "actionItem": {
                          "type": "object",
                          "description": "Action taken by the user.",
                          "properties": {
                            "text": {
                              "type": "string"
                            },
                            "value": {
                              "type": "string"
                            }
                          }
                        },
                        "channelName": {
                          "type": "string",
                          "description": "Channel in which the message was sent."
                        },
                        "messageId": {
                          "type": "string",
                          "description": "Unique ID of the message."
                        },
                        "original": {
                          "type": "object",
                          "description": "Original message sent to the user.",
                          "properties": {
                            "head": {
                              "type": "object",
                              "properties": {
                                "sub_head": {
                                  "type": "object",
                                  "properties": {
                                    "text": {
                                      "type": "string"
                                    }
                                  }
                                },
                                "text": {
                                  "type": "string"
                                }
                              }
                            },
                            "body": {
                              "type": "array",
                              "items": {
                                "type": "object",
                                "properties": {
                                  "type": {
                                    "type": "string"
                                  },
                                  "items": {
                                    "type": "array",
                                    "items": {
                                      "type": "object",
                                      "properties": {
                                        "style": {
                                          "type": "string"
                                        },
                                        "text": {
                                          "type": "string"
                                        },
                                        "value": {
                                          "type": "string"
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        },
                        "robotJid": {
                          "type": "string",
                          "description": "Unique credential of the Chatbot."
                        },
                        "timestamp": {
                          "type": "integer",
                          "description": "Time command message was sent."
                        },
                        "toJid": {
                          "type": "string",
                          "description": "JID of the channel or user the message was sent to."
                        },
                        "userId": {
                          "type": "string",
                          "description": "User sending the command."
                        },
                        "userJid": {
                          "type": "string",
                          "description": "JID of user who is sending the command."
                        },
                        "userName": {
                          "type": "string",
                          "description": "User's display name."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"interactive_message_actions\",\n  \"event_ts\": 1560891332219,\n  \"callback_url\": \"https://api.zoom.us/v2/im/chat/callback/bKFS3ybORiWbJx6VM6Qe1A\",\n  \"callback_token\": \"eyJzdiI6IjAwMDAwMSIsInR5cCI6IkpXVCIsImFsZyI6IkVTMjU2In0.eyJhdWQiOiJib3QiLCJ1aWQiOiJWV2ZQa1NENVJvMk5xRWUyR1VMa2F3IiwiY2xpZW50SWQiOiJRQmZVaEFyUlNHVVFVclM2ZDFXVnciLCJpc3MiOiJ6b29tIiwiZXhwIjoxNzYwMzYzNzc5LCJpYXQiOjE3NjAzMzM3NzksImFpZCI6InQ4dURPZWtYUkx5b3ExR0M4UFBaTVEifQ.wBeNSNeYLQYJD1Lp0O0EnnfyCYNzF03tEwFin0-hRV7UfECLmHsW7NnQ9EEPGxOokDKnQb-pRPd_iBX4JKfg_Q\",\n  \"payload\": {\n    \"accountId\": \"gVcjZnWWRLWvv_GtyGuaxg\",\n    \"actionItem\": {\n      \"text\": \"Add\",\n      \"value\": \"add\"\n    },\n    \"channelName\": \"Jill Chill\",\n    \"messageId\": \"20190618205516496_ynICmz8_aw1\",\n    \"original\": {\n      \"head\": {\n        \"sub_head\": {\n          \"text\": \"I am a sub header\"\n        },\n        \"text\": \"I am a header\"\n      },\n      \"body\": [\n        {\n          \"type\": \"actions\",\n          \"items\": [\n            {\n              \"style\": \"Primary\",\n              \"text\": \"Add\",\n              \"value\": \"add\"\n            },\n            {\n              \"style\": \"Default\",\n              \"text\": \"Update\",\n              \"value\": \"update\"\n            },\n            {\n              \"style\": \"Danger\",\n              \"text\": \"Delete\",\n              \"value\": \"delete\"\n            },\n            {\n              \"style\": \"Disabled\",\n              \"text\": \"Disabled\",\n              \"value\": \"disable\"\n            }\n          ]\n        }\n      ]\n    },\n    \"robotJid\": \"v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"timestamp\": 1560891332219,\n    \"toJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"userId\": \"KdYKjnimT4KPd8KKdQt9FQ\",\n    \"userJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com\",\n    \"userName\": \"Jill Chill\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "interactive_message_editable": {
      "post": {
        "tags": [
          "interactive_message_editable"
        ],
        "operationId": "interactive_message_editable",
        "requestBody": {
          "description": "# interactive_message_editable\n\nAfter a user edits the text of a Chatbot [message with Editable Text](https://marketplace.zoom.us/docs/guides/chatbots/customizing-messages/message-with-editable-text), the `interactive_message_editable` event is invoked and a POST request with data about the event is sent to the [Bot Endpoint URL](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-chatbot-app#set-features) specfied in the app's dashboard.\n\nThe `editItem` object displays the changes in edited text with `origin` and `target` values.\n\n\n\n**Event type**: `interactive_message_editable`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "event": {
                    "type": "string",
                    "description": "The name of the event."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "The timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format."
                  },
                  "callback_url": {
                    "type": "string",
                    "description": "Use this endpoint to send a one-time asynchronous response within 30 minutes of receiving the event. <br> The response payload should follow the same structure as the [Send Chatbot Messages](https://developers.zoom.us/docs/api/chatbot/#tag/chatbot-messages/POST/im/chat/messages) API with required property **content** and optional properties *reply_to*, *visible_to_user* and *is_markdown_support*. <br> Include the **callback_token** value from the event as **Bearer token in the Authorization header** of your HTTP POST request."
                  },
                  "callback_token": {
                    "type": "string",
                    "description": "Use the generated token value with the **callback_url** value to send a one-time asynchronous response. The token only lasts for 30 mins. Include the **callback_token** as a **Bearer token in the Authorization header** of your HTTP request. \n Example: \n\n \ncurl --request POST \\\n  --url '{callback_url}' \\\n  --header 'authorization: Bearer {callback_token}' \\\n  --header 'content-type: application/json'\\\n  --data-raw '{\"content\":{\"head\":{\"text\":\"Response Header\"},\"body\":[{\"type\":\"message\",\"text\":\"This is an asynchronous chatbot response.\"}]}}'"
                  },
                  "payload": {
                    "type": "object",
                    "properties": {
                      "accountId": {
                        "type": "string",
                        "description": "The account ID of the user editing the message."
                      },
                      "channelName": {
                        "type": "string",
                        "description": "The channel in which the message was edited."
                      },
                      "editItem": {
                        "type": "object",
                        "description": "The object containing original and edited value.",
                        "properties": {
                          "origin": {
                            "type": "string"
                          },
                          "target": {
                            "type": "string"
                          }
                        }
                      },
                      "messageId": {
                        "type": "string",
                        "description": "The unique ID of the message."
                      },
                      "original": {
                        "type": "object",
                        "description": "The original message sent to the user.",
                        "properties": {
                          "head": {
                            "type": "object",
                            "properties": {
                              "sub_head": {
                                "type": "object",
                                "properties": {
                                  "text": {
                                    "type": "string"
                                  }
                                }
                              },
                              "text": {
                                "type": "string"
                              }
                            }
                          },
                          "body": {
                            "type": "array",
                            "items": {
                              "type": "object",
                              "properties": {
                                "": {
                                  "type": "object",
                                  "properties": {
                                    "editable": {
                                      "type": "boolean"
                                    },
                                    "text": {
                                      "type": "string"
                                    },
                                    "type": {
                                      "type": "string"
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      },
                      "robotJid": {
                        "type": "string",
                        "description": "The unique credential of the Chatbot."
                      },
                      "timestamp": {
                        "type": "integer",
                        "format": "date-time",
                        "description": "The time command message was sent."
                      },
                      "toJid": {
                        "type": "string",
                        "description": "The JID of the channel or user the message was sent to."
                      },
                      "userId": {
                        "type": "string",
                        "description": "The user who sends the command."
                      },
                      "userJid": {
                        "type": "string",
                        "description": "The JID of user who is sending the command."
                      },
                      "userName": {
                        "type": "string",
                        "description": "The user's display name."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"interactive_message_editable\",\n  \"event_ts\": 1560891135776,\n  \"callback_url\": \"https://api.zoom.us/v2/im/chat/callback/bKFS3ybORiWbJx6VM6Qe1A\",\n  \"callback_token\": \"eyJzdiI6IjAwMDAwMSIsInR5cCI6IkpXVCIsImFsZyI6IkVTMjU2In0.eyJhdWQiOiJib3QiLCJ1aWQiOiJWV2ZQa1NENVJvMk5xRWUyR1VMa2F3IiwiY2xpZW50SWQiOiJRQmZVaEFyUlNHVVFVclM2ZDFXVnciLCJpc3MiOiJ6b29tIiwiZXhwIjoxNzYwMzYzNzc5LCJpYXQiOjE3NjAzMzM3NzksImFpZCI6InQ4dURPZWtYUkx5b3ExR0M4UFBaTVEifQ.wBeNSNeYLQYJD1Lp0O0EnnfyCYNzF03tEwFin0-hRV7UfECLmHsW7NnQ9EEPGxOokDKnQb-pRPd_iBX4JKfg_Q\",\n  \"payload\": {\n    \"accountId\": \"gVcjZnWWRLWvv_GtyGuaxg\",\n    \"channelName\": \"Jill Chill\",\n    \"editItem\": {\n      \"origin\": \"I am a message with editable text\",\n      \"target\": \"I am a message with editable text that has been edited\"\n    },\n    \"messageId\": \"20190618205205291_tI1CUa6_aw1\",\n    \"original\": {\n      \"head\": {\n        \"sub_head\": {\n          \"text\": \"I am a sub header\"\n        },\n        \"text\": \"I am a header\"\n      },\n      \"body\": [\n        {\n          \"editable\": true,\n          \"text\": \"I am a message with editable text\",\n          \"type\": \"message\"\n        }\n      ]\n    },\n    \"robotJid\": \"v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"timestamp\": 1560891135776,\n    \"toJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"userId\": \"KdYKjnimT4KPd8KKdQt9FQ\",\n    \"userJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com\",\n    \"userName\": \"Jill Chill\"\n  }\n}"
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "event": {
                      "type": "string",
                      "description": "The name of the event."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "The timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format."
                    },
                    "callback_url": {
                      "type": "string",
                      "description": "Use this endpoint to send a one-time asynchronous response within 30 minutes of receiving the event. <br> The response payload should follow the same structure as the [Send Chatbot Messages](https://developers.zoom.us/docs/api/chatbot/#tag/chatbot-messages/POST/im/chat/messages) API with required property **content** and optional properties *reply_to*, *visible_to_user* and *is_markdown_support*. <br> Include the **callback_token** value from the event as **Bearer token in the Authorization header** of your HTTP POST request."
                    },
                    "callback_token": {
                      "type": "string",
                      "description": "Use the generated token value with the **callback_url** value to send a one-time asynchronous response. The token only lasts for 30 mins. Include the **callback_token** as a **Bearer token in the Authorization header** of your HTTP request. \n Example: \n\n \ncurl --request POST \\\n  --url '{callback_url}' \\\n  --header 'authorization: Bearer {callback_token}' \\\n  --header 'content-type: application/json'\\\n  --data-raw '{\"content\":{\"head\":{\"text\":\"Response Header\"},\"body\":[{\"type\":\"message\",\"text\":\"This is an asynchronous chatbot response.\"}]}}'"
                    },
                    "payload": {
                      "type": "object",
                      "properties": {
                        "accountId": {
                          "type": "string",
                          "description": "The account ID of the user editing the message."
                        },
                        "channelName": {
                          "type": "string",
                          "description": "The channel in which the message was edited."
                        },
                        "editItem": {
                          "type": "object",
                          "description": "The object containing original and edited value.",
                          "properties": {
                            "origin": {
                              "type": "string"
                            },
                            "target": {
                              "type": "string"
                            }
                          }
                        },
                        "messageId": {
                          "type": "string",
                          "description": "The unique ID of the message."
                        },
                        "original": {
                          "type": "object",
                          "description": "The original message sent to the user.",
                          "properties": {
                            "head": {
                              "type": "object",
                              "properties": {
                                "sub_head": {
                                  "type": "object",
                                  "properties": {
                                    "text": {
                                      "type": "string"
                                    }
                                  }
                                },
                                "text": {
                                  "type": "string"
                                }
                              }
                            },
                            "body": {
                              "type": "array",
                              "items": {
                                "type": "object",
                                "properties": {
                                  "": {
                                    "type": "object",
                                    "properties": {
                                      "editable": {
                                        "type": "boolean"
                                      },
                                      "text": {
                                        "type": "string"
                                      },
                                      "type": {
                                        "type": "string"
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        },
                        "robotJid": {
                          "type": "string",
                          "description": "The unique credential of the Chatbot."
                        },
                        "timestamp": {
                          "type": "integer",
                          "format": "date-time",
                          "description": "The time command message was sent."
                        },
                        "toJid": {
                          "type": "string",
                          "description": "The JID of the channel or user the message was sent to."
                        },
                        "userId": {
                          "type": "string",
                          "description": "The user who sends the command."
                        },
                        "userJid": {
                          "type": "string",
                          "description": "The JID of user who is sending the command."
                        },
                        "userName": {
                          "type": "string",
                          "description": "The user's display name."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"interactive_message_editable\",\n  \"event_ts\": 1560891135776,\n  \"callback_url\": \"https://api.zoom.us/v2/im/chat/callback/bKFS3ybORiWbJx6VM6Qe1A\",\n  \"callback_token\": \"eyJzdiI6IjAwMDAwMSIsInR5cCI6IkpXVCIsImFsZyI6IkVTMjU2In0.eyJhdWQiOiJib3QiLCJ1aWQiOiJWV2ZQa1NENVJvMk5xRWUyR1VMa2F3IiwiY2xpZW50SWQiOiJRQmZVaEFyUlNHVVFVclM2ZDFXVnciLCJpc3MiOiJ6b29tIiwiZXhwIjoxNzYwMzYzNzc5LCJpYXQiOjE3NjAzMzM3NzksImFpZCI6InQ4dURPZWtYUkx5b3ExR0M4UFBaTVEifQ.wBeNSNeYLQYJD1Lp0O0EnnfyCYNzF03tEwFin0-hRV7UfECLmHsW7NnQ9EEPGxOokDKnQb-pRPd_iBX4JKfg_Q\",\n  \"payload\": {\n    \"accountId\": \"gVcjZnWWRLWvv_GtyGuaxg\",\n    \"channelName\": \"Jill Chill\",\n    \"editItem\": {\n      \"origin\": \"I am a message with editable text\",\n      \"target\": \"I am a message with editable text that has been edited\"\n    },\n    \"messageId\": \"20190618205205291_tI1CUa6_aw1\",\n    \"original\": {\n      \"head\": {\n        \"sub_head\": {\n          \"text\": \"I am a sub header\"\n        },\n        \"text\": \"I am a header\"\n      },\n      \"body\": [\n        {\n          \"editable\": true,\n          \"text\": \"I am a message with editable text\",\n          \"type\": \"message\"\n        }\n      ]\n    },\n    \"robotJid\": \"v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"timestamp\": 1560891135776,\n    \"toJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"userId\": \"KdYKjnimT4KPd8KKdQt9FQ\",\n    \"userJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com\",\n    \"userName\": \"Jill Chill\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "bot_installed": {
      "post": {
        "tags": [
          "bot_installed"
        ],
        "operationId": "bot_installed",
        "requestBody": {
          "description": "# bot_installed\nTo configure a dynamic welcome message:\n1. Go to your Chatbot's settings and navigate to the **Features** page.\n2. Check **Your App** for the welcome message.\n3. Save your settings.\n\nAfter your Chatbot is authorized, Zoom sends an HTTP POST request to your respective Bot Endpoint URL with the following request body:\n\n**Event type**: `bot_installed`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "event": {
                    "type": "string",
                    "description": "Name of the event."
                  },
                  "payload": {
                    "type": "object",
                    "properties": {
                      "accountId": {
                        "type": "string",
                        "description": "The account ID of the user sending the command."
                      },
                      "robotJid": {
                        "type": "string",
                        "description": "Unique credential of the Chatbot."
                      },
                      "timestamp": {
                        "type": "integer",
                        "format": "date-time",
                        "description": "Time command message was sent."
                      },
                      "userId": {
                        "type": "string",
                        "description": "User sending the command."
                      },
                      "userJid": {
                        "type": "string",
                        "description": "JID of user who is sending the command."
                      },
                      "userName": {
                        "type": "string",
                        "description": "User's display name."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"bot_installed\",\n  \"payload\": {\n    \"accountId\": \"CDSCdcsdv\",\n    \"robotJid\": \"csvfdvfd@xmpp.zoom.us\",\n    \"timestamp\": 1542796725586,\n    \"userId\": \"fvdfvfdvfd\",\n    \"userJid\": \"fvdfvfdvfd@xmpp.zoom.us\",\n    \"userName\": \"UserName\"\n  }\n}"
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "event": {
                      "type": "string",
                      "description": "Name of the event."
                    },
                    "payload": {
                      "type": "object",
                      "properties": {
                        "accountId": {
                          "type": "string",
                          "description": "The account ID of the user sending the command."
                        },
                        "robotJid": {
                          "type": "string",
                          "description": "Unique credential of the Chatbot."
                        },
                        "timestamp": {
                          "type": "integer",
                          "format": "date-time",
                          "description": "Time command message was sent."
                        },
                        "userId": {
                          "type": "string",
                          "description": "User sending the command."
                        },
                        "userJid": {
                          "type": "string",
                          "description": "JID of user who is sending the command."
                        },
                        "userName": {
                          "type": "string",
                          "description": "User's display name."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"bot_installed\",\n  \"payload\": {\n    \"accountId\": \"CDSCdcsdv\",\n    \"robotJid\": \"csvfdvfd@xmpp.zoom.us\",\n    \"timestamp\": 1542796725586,\n    \"userId\": \"fvdfvfdvfd\",\n    \"userJid\": \"fvdfvfdvfd@xmpp.zoom.us\",\n    \"userName\": \"UserName\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "team_chat.app_mention": {
      "post": {
        "tags": [
          "team_chat"
        ],
        "operationId": "team_chat.app_mention",
        "requestBody": {
          "description": "# team_chat.app_mention\nThis event triggers when a user mentions a specific app in a chat channel. A POST request with the mention message is sent to the [Bot Endpoint URL](https://developers.zoom.us/docs/team-chat-apps/team-chat-subscriptions/) specified in the app's dashboard.\n## Prerequisites\n* A Pro or a higher [plan](https://zoom.us/pricing).\n\n**Event type**: `team_chat.app_mention`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "required": [
                  "event",
                  "event_ts",
                  "payload"
                ],
                "properties": {
                  "event": {
                    "type": "string",
                    "description": "The name of the event.",
                    "enum": [
                      "team_chat.app_mention"
                    ]
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "A timestamp at which the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "operator",
                      "operator_id",
                      "object",
                      "operator_member_id",
                      "by_external_user"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who sent the app mention message. Will be empty when by_external_user is true"
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user who sent the app mention message. Will be empty when by_external_user is true"
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user who sent the app mention message. Will be empty when by_external_user is true"
                      },
                      "operator_member_id": {
                        "type": "string",
                        "description": "The member ID of the user who sent the app mention message."
                      },
                      "by_external_user": {
                        "type": "boolean",
                        "description": "Whether this action was performed by an external user."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the app mention message.",
                        "required": [
                          "message_id",
                          "type",
                          "message",
                          "timestamp",
                          "date_time"
                        ],
                        "properties": {
                          "message_id": {
                            "type": "string",
                            "description": "The message's ID."
                          },
                          "type": {
                            "type": "string",
                            "description": "The type of message: \n* to_channel \u2014 The message was sent to a chat channel.",
                            "enum": [
                              "to_channel"
                            ]
                          },
                          "channel_id": {
                            "type": "string",
                            "description": "The chat channel's ID."
                          },
                          "channel_name": {
                            "type": "string",
                            "description": "The chat channel's name."
                          },
                          "message": {
                            "type": "string",
                            "description": "The sent message's content."
                          },
                          "files": {
                            "type": "array",
                            "description": "The attachments file info in sent message's content.",
                            "items": {
                              "type": "object",
                              "required": [
                                "OS_file_type",
                                "file_id",
                                "file_message_type",
                                "file_name",
                                "file_size"
                              ],
                              "properties": {
                                "OS_file_type": {
                                  "type": "string",
                                  "description": "The attachments file extension type"
                                },
                                "file_id": {
                                  "type": "string",
                                  "description": "The attachments file's universally unique identifier (UUID), in base64-encoded format."
                                },
                                "file_message_type": {
                                  "type": "string",
                                  "description": "The attachments file message type",
                                  "enum": [
                                    "file",
                                    "image",
                                    "audio",
                                    "audio v2",
                                    "code snippet",
                                    "screen shot"
                                  ]
                                },
                                "file_name": {
                                  "type": "string",
                                  "description": "The attachments file's name."
                                },
                                "file_size": {
                                  "type": "integer",
                                  "description": "The attachments file's size, in bytes."
                                }
                              },
                              "additionalProperties": false
                            }
                          },
                          "rich_text": {
                            "type": "array",
                            "description": "The [rich text formatting](https://support.zoom.us/hc/en-us/articles/115004789183-Replying-to-and-managing-chat-messages)object. Use this object to include rich text info in sent message's content.",
                            "items": {
                              "type": "object",
                              "required": [
                                "start_position",
                                "end_position",
                                "format_type"
                              ],
                              "properties": {
                                "start_position": {
                                  "type": "integer",
                                  "description": "The start position of the rich text in the message string. \n\nFor example if the first character at the beginning of the message is bold, the value for this field will be 0."
                                },
                                "end_position": {
                                  "type": "integer",
                                  "description": "The end position of the rich text."
                                },
                                "format_type": {
                                  "type": "string",
                                  "description": "The type of rich text.",
                                  "enum": [
                                    "Bold",
                                    "Italic",
                                    "Strikethrough",
                                    "BulletedList",
                                    "NumberedList",
                                    "Underline",
                                    "FontSize",
                                    "FontColor",
                                    "BackgroundColor",
                                    "LeftIndent",
                                    "Paragraph",
                                    "Quote",
                                    "AddLink"
                                  ]
                                },
                                "format_attr": {
                                  "type": "string",
                                  "description": "This field is non-null if the value of the format_type field is listed below:\n\n1.FontSize value: s for small, m for medium, or l for large font size. \n\n2.FontColor and BackgroundColor value: only supports RGB value. For example: FFC0CB \n\n3.LeftIndent value: a positive pixel length.  \n\n4.Paragraph value: h1 for Heading 1, h2 for Heading 2, or h3 for Heading 3. \n\n5.AddLink value: must be a valid URL, with an http or https prefix. For example: https://example.com"
                                }
                              },
                              "additionalProperties": false
                            },
                            "reply_main_message_id": {
                              "type": "string",
                              "description": "The ID of the parent message in the chat thread.It's only available if the message is a reply to a main message in a thread."
                            },
                            "date_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The date and time at which the message was sent, in yyyy-MM-dd'T'HH:mm:ss'Z' format."
                            },
                            "timestamp": {
                              "type": "integer",
                              "format": "int64",
                              "description": "The timestamp for when the message was sent."
                            }
                          }
                        },
                        "additionalProperties": false
                      }
                    },
                    "additionalProperties": false
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"team_chat.app_mention\",\n  \"event_ts\": 1696914490200,\n  \"payload\": {\n    \"account_id\": \"asgVcjZnWWRLWvv_GtyGuaxg\",\n    \"operator\": \"example@zoom.us\",\n    \"operator_member_id\": \"R4VM29Oj0fVM2hhEmSKVM2hhezJTezJTKVM2hezJT2hezJTSK\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"by_external_user\": true,\n    \"object\": {\n      \"message_id\": \"5DD614F4-DD19-4C39-8D03-93C4D0A0C839\",\n      \"type\": \"to_channel\",\n      \"channel_id\": \"8AmQ0WOoQ1O3a84yYFc2xQ\",\n      \"channel_name\": \"Team Chat\",\n      \"message\": \"hello\",\n      \"files\": [\n        {\n          \"file_id\": \"56bed9orRnOSEnBoX_HQvA\",\n          \"file_name\": \"Screenshot 2023-12-22 at 11.21.38 AM.png\",\n          \"file_size\": 23025,\n          \"OS_file_type\": \"PNG\",\n          \"file_message_type\": \"image\"\n        }\n      ],\n      \"rich_text\": [\n        {\n          \"start_position\": 0,\n          \"end_position\": 1,\n          \"format_type\": \"Bold\"\n        },\n        {\n          \"start_position\": 1,\n          \"end_position\": 2,\n          \"format_type\": \"BulletedList\"\n        },\n        {\n          \"start_position\": 2,\n          \"end_position\": 3,\n          \"format_type\": \"AddLink\",\n          \"format_attr\": \"https://www.example.com/templates/daily-stand-up\"\n        }\n      ],\n      \"reply_main_message_id\": \"5DD614F4-DD19-4C39-8D03-93C4D0A0C838\",\n      \"date_time\": \"2023-10-10T05:08:10Z\",\n      \"timestamp\": 1696914490157\n    }\n  }\n}"
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "required": [
                    "event",
                    "event_ts",
                    "payload"
                  ],
                  "properties": {
                    "event": {
                      "type": "string",
                      "description": "The name of the event.",
                      "enum": [
                        "team_chat.app_mention"
                      ]
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "A timestamp at which the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "operator",
                        "operator_id",
                        "object",
                        "operator_member_id",
                        "by_external_user"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who sent the app mention message. Will be empty when by_external_user is true"
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user who sent the app mention message. Will be empty when by_external_user is true"
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user who sent the app mention message. Will be empty when by_external_user is true"
                        },
                        "operator_member_id": {
                          "type": "string",
                          "description": "The member ID of the user who sent the app mention message."
                        },
                        "by_external_user": {
                          "type": "boolean",
                          "description": "Whether this action was performed by an external user."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the app mention message.",
                          "required": [
                            "message_id",
                            "type",
                            "message",
                            "timestamp",
                            "date_time"
                          ],
                          "properties": {
                            "message_id": {
                              "type": "string",
                              "description": "The message's ID."
                            },
                            "type": {
                              "type": "string",
                              "description": "The type of message: \n* to_channel \u2014 The message was sent to a chat channel.",
                              "enum": [
                                "to_channel"
                              ]
                            },
                            "channel_id": {
                              "type": "string",
                              "description": "The chat channel's ID."
                            },
                            "channel_name": {
                              "type": "string",
                              "description": "The chat channel's name."
                            },
                            "message": {
                              "type": "string",
                              "description": "The sent message's content."
                            },
                            "files": {
                              "type": "array",
                              "description": "The attachments file info in sent message's content.",
                              "items": {
                                "type": "object",
                                "required": [
                                  "OS_file_type",
                                  "file_id",
                                  "file_message_type",
                                  "file_name",
                                  "file_size"
                                ],
                                "properties": {
                                  "OS_file_type": {
                                    "type": "string",
                                    "description": "The attachments file extension type"
                                  },
                                  "file_id": {
                                    "type": "string",
                                    "description": "The attachments file's universally unique identifier (UUID), in base64-encoded format."
                                  },
                                  "file_message_type": {
                                    "type": "string",
                                    "description": "The attachments file message type",
                                    "enum": [
                                      "file",
                                      "image",
                                      "audio",
                                      "audio v2",
                                      "code snippet",
                                      "screen shot"
                                    ]
                                  },
                                  "file_name": {
                                    "type": "string",
                                    "description": "The attachments file's name."
                                  },
                                  "file_size": {
                                    "type": "integer",
                                    "description": "The attachments file's size, in bytes."
                                  }
                                },
                                "additionalProperties": false
                              }
                            },
                            "rich_text": {
                              "type": "array",
                              "description": "The [rich text formatting](https://support.zoom.us/hc/en-us/articles/115004789183-Replying-to-and-managing-chat-messages)object. Use this object to include rich text info in sent message's content.",
                              "items": {
                                "type": "object",
                                "required": [
                                  "start_position",
                                  "end_position",
                                  "format_type"
                                ],
                                "properties": {
                                  "start_position": {
                                    "type": "integer",
                                    "description": "The start position of the rich text in the message string. \n\nFor example if the first character at the beginning of the message is bold, the value for this field will be 0."
                                  },
                                  "end_position": {
                                    "type": "integer",
                                    "description": "The end position of the rich text."
                                  },
                                  "format_type": {
                                    "type": "string",
                                    "description": "The type of rich text.",
                                    "enum": [
                                      "Bold",
                                      "Italic",
                                      "Strikethrough",
                                      "BulletedList",
                                      "NumberedList",
                                      "Underline",
                                      "FontSize",
                                      "FontColor",
                                      "BackgroundColor",
                                      "LeftIndent",
                                      "Paragraph",
                                      "Quote",
                                      "AddLink"
                                    ]
                                  },
                                  "format_attr": {
                                    "type": "string",
                                    "description": "This field is non-null if the value of the format_type field is listed below:\n\n1.FontSize value: s for small, m for medium, or l for large font size. \n\n2.FontColor and BackgroundColor value: only supports RGB value. For example: FFC0CB \n\n3.LeftIndent value: a positive pixel length.  \n\n4.Paragraph value: h1 for Heading 1, h2 for Heading 2, or h3 for Heading 3. \n\n5.AddLink value: must be a valid URL, with an http or https prefix. For example: https://example.com"
                                  }
                                },
                                "additionalProperties": false
                              },
                              "reply_main_message_id": {
                                "type": "string",
                                "description": "The ID of the parent message in the chat thread.It's only available if the message is a reply to a main message in a thread."
                              },
                              "date_time": {
                                "type": "string",
                                "format": "date-time",
                                "description": "The date and time at which the message was sent, in yyyy-MM-dd'T'HH:mm:ss'Z' format."
                              },
                              "timestamp": {
                                "type": "integer",
                                "format": "int64",
                                "description": "The timestamp for when the message was sent."
                              }
                            }
                          },
                          "additionalProperties": false
                        }
                      },
                      "additionalProperties": false
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"team_chat.app_mention\",\n  \"event_ts\": 1696914490200,\n  \"payload\": {\n    \"account_id\": \"asgVcjZnWWRLWvv_GtyGuaxg\",\n    \"operator\": \"example@zoom.us\",\n    \"operator_member_id\": \"R4VM29Oj0fVM2hhEmSKVM2hhezJTezJTKVM2hezJT2hezJTSK\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"by_external_user\": true,\n    \"object\": {\n      \"message_id\": \"5DD614F4-DD19-4C39-8D03-93C4D0A0C839\",\n      \"type\": \"to_channel\",\n      \"channel_id\": \"8AmQ0WOoQ1O3a84yYFc2xQ\",\n      \"channel_name\": \"Team Chat\",\n      \"message\": \"hello\",\n      \"files\": [\n        {\n          \"file_id\": \"56bed9orRnOSEnBoX_HQvA\",\n          \"file_name\": \"Screenshot 2023-12-22 at 11.21.38 AM.png\",\n          \"file_size\": 23025,\n          \"OS_file_type\": \"PNG\",\n          \"file_message_type\": \"image\"\n        }\n      ],\n      \"rich_text\": [\n        {\n          \"start_position\": 0,\n          \"end_position\": 1,\n          \"format_type\": \"Bold\"\n        },\n        {\n          \"start_position\": 1,\n          \"end_position\": 2,\n          \"format_type\": \"BulletedList\"\n        },\n        {\n          \"start_position\": 2,\n          \"end_position\": 3,\n          \"format_type\": \"AddLink\",\n          \"format_attr\": \"https://www.example.com/templates/daily-stand-up\"\n        }\n      ],\n      \"reply_main_message_id\": \"5DD614F4-DD19-4C39-8D03-93C4D0A0C838\",\n      \"date_time\": \"2023-10-10T05:08:10Z\",\n      \"timestamp\": 1696914490157\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "interactive_message_select": {
      "post": {
        "tags": [
          "interactive_message_select"
        ],
        "operationId": "interactive_message_select",
        "requestBody": {
          "description": "# interactive_message_select\n\nAfter a user selects an option in a Chatbot [message with a Dropdown](https://marketplace.zoom.us/docs/guides/chatbots/customizing-messages/message-with-dropdown), the `interactive_message_select` event is invoked. The Chatbot sends a POST request associated with this event along with data on the selected option to the [Bot Endpoint URL](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-chatbot-app#set-features) specified in the app's dashboard.\n\nThe `selectedItems` array displays the value chosen by the user from the  `selected_items` array in the original message sent by the Chatbot.\n\n\n\n**Event type**: `interactive_message_select`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "event": {
                    "type": "string",
                    "description": "Name of the event."
                  },
                  "payload": {
                    "type": "object",
                    "properties": {
                      "accountId": {
                        "type": "string",
                        "description": "The account ID of the user sending the command."
                      },
                      "channelName": {
                        "type": "string",
                        "description": "Channel in which the message was sent. "
                      },
                      "messageId": {
                        "type": "string",
                        "description": "Unique ID of the message."
                      },
                      "original": {
                        "type": "object",
                        "description": "Original message sent to user.",
                        "properties": {
                          "head": {
                            "type": "object",
                            "properties": {
                              "sub_head": {
                                "type": "object",
                                "properties": {
                                  "text": {
                                    "type": "string"
                                  }
                                }
                              },
                              "text": {
                                "type": "string"
                              }
                            }
                          },
                          "body": {
                            "type": "array",
                            "items": {
                              "type": "object",
                              "properties": {
                                "select_items": {
                                  "type": "array",
                                  "items": {
                                    "type": "object",
                                    "properties": {
                                      "text": {
                                        "type": "string"
                                      },
                                      "value": {
                                        "type": "string"
                                      }
                                    }
                                  }
                                },
                                "text": {
                                  "type": "string"
                                },
                                "type": {
                                  "type": "string"
                                }
                              }
                            }
                          }
                        }
                      },
                      "robotJid": {
                        "type": "string",
                        "description": "Unique credential of the Chatbot."
                      },
                      "selectedItems": {
                        "type": "array",
                        "description": "Value of item selected by the user.",
                        "items": {
                          "type": "object",
                          "properties": {
                            "value": {
                              "type": "string"
                            }
                          }
                        }
                      },
                      "timestamp": {
                        "type": "integer",
                        "format": "date-time",
                        "description": "Time command message was sent."
                      },
                      "toJid": {
                        "type": "string",
                        "description": "JID of the channel or user the message was sent to."
                      },
                      "userId": {
                        "type": "string",
                        "description": "User sending the command."
                      },
                      "userJid": {
                        "type": "string",
                        "description": "JID of user who is sending the command."
                      },
                      "userName": {
                        "type": "string",
                        "description": "User's display name."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"interactive_message_select\",\n  \"event_ts\": 1560891284464,\n  \"payload\": {\n    \"accountId\": \"gVcjZnWWRLWvv_GtyGuaxg\",\n    \"channelName\": \"Jill Chill\",\n    \"messageId\": \"20190618205440879_sRHVqf4_aw1\",\n    \"original\": {\n      \"head\": {\n        \"sub_head\": {\n          \"text\": \"I am a sub header\"\n        },\n        \"text\": \"I am a header\"\n      },\n      \"body\": [\n        {\n          \"select_items\": [\n            {\n              \"text\": \"Lions\",\n              \"value\": \"lions\"\n            },\n            {\n              \"text\": \"Tigers\",\n              \"value\": \"tigers\"\n            },\n            {\n              \"text\": \"Bears\",\n              \"value\": \"bears\"\n            }\n          ],\n          \"text\": \"Animals\",\n          \"type\": \"select\"\n        }\n      ]\n    },\n    \"robotJid\": \"v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"selectedItems\": [\n      {\n        \"value\": \"lions\"\n      }\n    ],\n    \"timestamp\": 1560891284464,\n    \"toJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"userId\": \"KdYKjnimT4KPd8KKdQt9FQ\",\n    \"userJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com\",\n    \"userName\": \"Jill Chill\"\n  }\n}"
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "event": {
                      "type": "string",
                      "description": "Name of the event."
                    },
                    "payload": {
                      "type": "object",
                      "properties": {
                        "accountId": {
                          "type": "string",
                          "description": "The account ID of the user sending the command."
                        },
                        "channelName": {
                          "type": "string",
                          "description": "Channel in which the message was sent. "
                        },
                        "messageId": {
                          "type": "string",
                          "description": "Unique ID of the message."
                        },
                        "original": {
                          "type": "object",
                          "description": "Original message sent to user.",
                          "properties": {
                            "head": {
                              "type": "object",
                              "properties": {
                                "sub_head": {
                                  "type": "object",
                                  "properties": {
                                    "text": {
                                      "type": "string"
                                    }
                                  }
                                },
                                "text": {
                                  "type": "string"
                                }
                              }
                            },
                            "body": {
                              "type": "array",
                              "items": {
                                "type": "object",
                                "properties": {
                                  "select_items": {
                                    "type": "array",
                                    "items": {
                                      "type": "object",
                                      "properties": {
                                        "text": {
                                          "type": "string"
                                        },
                                        "value": {
                                          "type": "string"
                                        }
                                      }
                                    }
                                  },
                                  "text": {
                                    "type": "string"
                                  },
                                  "type": {
                                    "type": "string"
                                  }
                                }
                              }
                            }
                          }
                        },
                        "robotJid": {
                          "type": "string",
                          "description": "Unique credential of the Chatbot."
                        },
                        "selectedItems": {
                          "type": "array",
                          "description": "Value of item selected by the user.",
                          "items": {
                            "type": "object",
                            "properties": {
                              "value": {
                                "type": "string"
                              }
                            }
                          }
                        },
                        "timestamp": {
                          "type": "integer",
                          "format": "date-time",
                          "description": "Time command message was sent."
                        },
                        "toJid": {
                          "type": "string",
                          "description": "JID of the channel or user the message was sent to."
                        },
                        "userId": {
                          "type": "string",
                          "description": "User sending the command."
                        },
                        "userJid": {
                          "type": "string",
                          "description": "JID of user who is sending the command."
                        },
                        "userName": {
                          "type": "string",
                          "description": "User's display name."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"interactive_message_select\",\n  \"event_ts\": 1560891284464,\n  \"payload\": {\n    \"accountId\": \"gVcjZnWWRLWvv_GtyGuaxg\",\n    \"channelName\": \"Jill Chill\",\n    \"messageId\": \"20190618205440879_sRHVqf4_aw1\",\n    \"original\": {\n      \"head\": {\n        \"sub_head\": {\n          \"text\": \"I am a sub header\"\n        },\n        \"text\": \"I am a header\"\n      },\n      \"body\": [\n        {\n          \"select_items\": [\n            {\n              \"text\": \"Lions\",\n              \"value\": \"lions\"\n            },\n            {\n              \"text\": \"Tigers\",\n              \"value\": \"tigers\"\n            },\n            {\n              \"text\": \"Bears\",\n              \"value\": \"bears\"\n            }\n          ],\n          \"text\": \"Animals\",\n          \"type\": \"select\"\n        }\n      ]\n    },\n    \"robotJid\": \"v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"selectedItems\": [\n      {\n        \"value\": \"lions\"\n      }\n    ],\n    \"timestamp\": 1560891284464,\n    \"toJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"userId\": \"KdYKjnimT4KPd8KKdQt9FQ\",\n    \"userJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com\",\n    \"userName\": \"Jill Chill\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    }
  },
  "servers": [
    {
      "url": "https://{your-endpoint-url}"
    }
  ]
}
```

## Downloads

### webhooks.json
```json
{
  "openapi": "3.1.0",
  "info": {
    "title": "Chatbot",
    "description": "The chatbot webhooks allow developers to receive events for [chatbot](https://developers.zoom.us/docs/team-chat-apps/) features.  \r\n\r\nNote regarding JSON payloads for guest users:\r\nBe aware that guest users (users not signed in to Zoom or who are members of a different account) are assigned temporary IDs that are valid only for the duration of that meeting or chat channel. Data for guest users in JSON payloads is associated with these temporary IDs.\r\n\r\n",
    "version": "1.0.0"
  },
  "webhooks": {
    "team_chat.link_shared": {
      "post": {
        "tags": [
          "team_chat"
        ],
        "operationId": "team_chat.link_shared",
        "requestBody": {
          "description": "# team_chat.link_shared\nThe **Chat message link shared** \u00a0event triggers when a user sends a chat message to a contact or a chat channel, and when the app is installed for the user. This message must contain the link. The link's domain is registered by the app in Marketplace. A POST request with a mention message is sent to the [Bot Endpoint URL](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-chatbot-app#set-features) specified in the app's dashboard.\n## Prerequisites\n* A Pro or a higher [plan](https://zoom.us/pricing).\n\n\n**Event type**: `team_chat.link_shared`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "required": [
                  "event",
                  "event_ts",
                  "response_url",
                  "payload"
                ],
                "properties": {
                  "event": {
                    "type": "string",
                    "description": "The name of the event.",
                    "enum": [
                      "team_chat.link_shared"
                    ]
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "A timestamp at which the event occurred."
                  },
                  "response_url": {
                    "type": "string",
                    "description": "A callback uri for app to response preview result."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "object",
                      "operator",
                      "operator_member_id",
                      "operator_id",
                      "by_external_user"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who sent the app mention message."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user who sent the app mention message."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user who sent the app mention message."
                      },
                      "operator_member_id": {
                        "type": "string",
                        "description": "The member ID of the user who sent the app mention message."
                      },
                      "by_external_user": {
                        "type": "boolean",
                        "description": "Whether this action was performed by an external user."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the app mention message.",
                        "required": [
                          "type",
                          "link",
                          "trigger_id",
                          "timestamp",
                          "date_time"
                        ],
                        "properties": {
                          "message_id": {
                            "type": "string",
                            "description": "The message's ID. If the preview is provided in an unfurl for a posted message, rather than a link inside the message composer, we will simply ignore this property."
                          },
                          "type": {
                            "type": "string",
                            "description": "The type of message: \n* to_contact \u2014 The message was sent to a contact. \n* to_channel \u2014 The message was sent to a chat channel.",
                            "enum": [
                              "to_contact",
                              "to_channel"
                            ]
                          },
                          "channel_id": {
                            "type": "string",
                            "description": "The chat channel's ID."
                          },
                          "channel_name": {
                            "type": "string",
                            "description": "The chat channel's name."
                          },
                          "contact_id": {
                            "type": "string",
                            "description": "The message contact's user ID. If the user ID does not belong to the same account as the current user, this field will be empty."
                          },
                          "contact_member_id": {
                            "type": "string",
                            "description": "The message contact's member ID."
                          },
                          "trigger_id": {
                            "type": "string",
                            "description": "The unique identification of this webook event, app can use this trigger_id to invoke openAPI"
                          },
                          "link": {
                            "type": "string",
                            "description": "The link content in message."
                          },
                          "reply_main_message_id": {
                            "type": "string",
                            "description": "The ID of the parent message in the chat thread.It's only available if the message is a reply to a main message in a thread."
                          },
                          "date_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The date and time at which the message was sent, in yyyy-MM-dd'T'HH:mm:ss'Z' format."
                          },
                          "timestamp": {
                            "type": "integer",
                            "format": "int64",
                            "description": "The timestamp for when the message was sent."
                          }
                        },
                        "additionalProperties": false
                      }
                    },
                    "additionalProperties": false
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"team_chat.link_shared\",\n  \"event_ts\": 1696914490200,\n  \"response_url\": \"https://api.zoom.us/v2/im/chat/users/exampleUserId/unfurls/exampleTriggerId\",\n  \"payload\": {\n    \"account_id\": \"exampleAccountId\",\n    \"operator_id\": \"exampleuserId\",\n    \"operator\": \"example@zoom.us\",\n    \"operator_member_id\": \"R4VM29Oj0fVM2hhEmSKVM2hhezJTezJTKVM2hezJT2hezJTSK\",\n    \"by_external_user\": false,\n    \"object\": {\n      \"message_id\": \"5DD614F4-DD19-4C39-8D03-93C4D0A0C839\",\n      \"reply_main_message_id\": \"5DD614F4-DD19-4C39-8D03-93C4D0A0C838\",\n      \"type\": \"to_channel\",\n      \"channel_id\": \"xxxxxxx\",\n      \"channel_name\": \"Team Chat\",\n      \"link\": \"https://sg.com/1234w\",\n      \"trigger_id\": \"exampleTriggerId\",\n      \"date_time\": \"2023-10-10T05:08:10Z\",\n      \"timestamp\": 1696914490157\n    }\n  }\n}"
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "required": [
                    "event",
                    "event_ts",
                    "response_url",
                    "payload"
                  ],
                  "properties": {
                    "event": {
                      "type": "string",
                      "description": "The name of the event.",
                      "enum": [
                        "team_chat.link_shared"
                      ]
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "A timestamp at which the event occurred."
                    },
                    "response_url": {
                      "type": "string",
                      "description": "A callback uri for app to response preview result."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "object",
                        "operator",
                        "operator_member_id",
                        "operator_id",
                        "by_external_user"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who sent the app mention message."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user who sent the app mention message."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user who sent the app mention message."
                        },
                        "operator_member_id": {
                          "type": "string",
                          "description": "The member ID of the user who sent the app mention message."
                        },
                        "by_external_user": {
                          "type": "boolean",
                          "description": "Whether this action was performed by an external user."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the app mention message.",
                          "required": [
                            "type",
                            "link",
                            "trigger_id",
                            "timestamp",
                            "date_time"
                          ],
                          "properties": {
                            "message_id": {
                              "type": "string",
                              "description": "The message's ID. If the preview is provided in an unfurl for a posted message, rather than a link inside the message composer, we will simply ignore this property."
                            },
                            "type": {
                              "type": "string",
                              "description": "The type of message: \n* to_contact \u2014 The message was sent to a contact. \n* to_channel \u2014 The message was sent to a chat channel.",
                              "enum": [
                                "to_contact",
                                "to_channel"
                              ]
                            },
                            "channel_id": {
                              "type": "string",
                              "description": "The chat channel's ID."
                            },
                            "channel_name": {
                              "type": "string",
                              "description": "The chat channel's name."
                            },
                            "contact_id": {
                              "type": "string",
                              "description": "The message contact's user ID. If the user ID does not belong to the same account as the current user, this field will be empty."
                            },
                            "contact_member_id": {
                              "type": "string",
                              "description": "The message contact's member ID."
                            },
                            "trigger_id": {
                              "type": "string",
                              "description": "The unique identification of this webook event, app can use this trigger_id to invoke openAPI"
                            },
                            "link": {
                              "type": "string",
                              "description": "The link content in message."
                            },
                            "reply_main_message_id": {
                              "type": "string",
                              "description": "The ID of the parent message in the chat thread.It's only available if the message is a reply to a main message in a thread."
                            },
                            "date_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The date and time at which the message was sent, in yyyy-MM-dd'T'HH:mm:ss'Z' format."
                            },
                            "timestamp": {
                              "type": "integer",
                              "format": "int64",
                              "description": "The timestamp for when the message was sent."
                            }
                          },
                          "additionalProperties": false
                        }
                      },
                      "additionalProperties": false
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"team_chat.link_shared\",\n  \"event_ts\": 1696914490200,\n  \"response_url\": \"https://api.zoom.us/v2/im/chat/users/exampleUserId/unfurls/exampleTriggerId\",\n  \"payload\": {\n    \"account_id\": \"exampleAccountId\",\n    \"operator_id\": \"exampleuserId\",\n    \"operator\": \"example@zoom.us\",\n    \"operator_member_id\": \"R4VM29Oj0fVM2hhEmSKVM2hhezJTezJTKVM2hezJT2hezJTSK\",\n    \"by_external_user\": false,\n    \"object\": {\n      \"message_id\": \"5DD614F4-DD19-4C39-8D03-93C4D0A0C839\",\n      \"reply_main_message_id\": \"5DD614F4-DD19-4C39-8D03-93C4D0A0C838\",\n      \"type\": \"to_channel\",\n      \"channel_id\": \"xxxxxxx\",\n      \"channel_name\": \"Team Chat\",\n      \"link\": \"https://sg.com/1234w\",\n      \"trigger_id\": \"exampleTriggerId\",\n      \"date_time\": \"2023-10-10T05:08:10Z\",\n      \"timestamp\": 1696914490157\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "interactive_message_fields_editable": {
      "post": {
        "tags": [
          "interactive_message_fields_editable"
        ],
        "operationId": "interactive_message_fields_editable",
        "requestBody": {
          "description": "# interactive_message_fields_editable\n\nAfter a user edits the text of a Chatbot [message with Editable Text](https://marketplace.zoom.us/docs/guides/chatbots/customizing-messages/message-with-editable-text), the `interactive_message_editable` event is invoked and a POST request with data about the event is sent to the [Bot Endpoint URL](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-chatbot-app#set-features) specified in the app's dashboard.\n\nThe `editItem` object displays the changes in edited text with `origin` and `target` values.\n\n\n\n**Event type**: `interactive_message_fields_editable`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "event": {
                    "type": "string",
                    "description": "The name of the event."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "The timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format."
                  },
                  "callback_url": {
                    "type": "string",
                    "description": "Use this endpoint to send a one-time asynchronous response within 30 minutes of receiving the event. <br> The response payload should follow the same structure as the [Send Chatbot Messages](https://developers.zoom.us/docs/api/chatbot/#tag/chatbot-messages/POST/im/chat/messages) API with required property **content** and optional properties *reply_to*, *visible_to_user* and *is_markdown_support*. <br> Include the **callback_token** value from the event as **Bearer token in the Authorization header** of your HTTP POST request."
                  },
                  "callback_token": {
                    "type": "string",
                    "description": "Use the generated token value with the **callback_url** value to send a one-time asynchronous response. The token only lasts for 30 mins. Include the **callback_token** as a **Bearer token in the Authorization header** of your HTTP request. \n Example: \n\n \ncurl --request POST \\\n  --url '{callback_url}' \\\n  --header 'authorization: Bearer {callback_token}' \\\n  --header 'content-type: application/json'\\\n  --data-raw '{\"content\":{\"head\":{\"text\":\"Response Header\"},\"body\":[{\"type\":\"message\",\"text\":\"This is an asynchronous chatbot response.\"}]}}'"
                  },
                  "payload": {
                    "type": "object",
                    "properties": {
                      "accountId": {
                        "type": "string",
                        "description": "The account ID of the user sending the command."
                      },
                      "channelName": {
                        "type": "string",
                        "description": "The channel in which the message was sent. "
                      },
                      "fieldEditItem": {
                        "type": "object",
                        "description": "The item that the user edits.",
                        "properties": {
                          "currentValue": {
                            "type": "string"
                          },
                          "key": {
                            "type": "string"
                          },
                          "newValue": {
                            "type": "string"
                          }
                        }
                      },
                      "messageId": {
                        "type": "string",
                        "description": "The unique ID of the message."
                      },
                      "original": {
                        "type": "object",
                        "description": "The original message sent to the user.",
                        "properties": {
                          "head": {
                            "type": "object",
                            "properties": {
                              "sub_head": {
                                "type": "object",
                                "properties": {
                                  "text": {
                                    "type": "string"
                                  }
                                }
                              },
                              "text": {
                                "type": "string"
                              }
                            }
                          },
                          "body": {
                            "type": "array",
                            "items": {
                              "type": "object",
                              "properties": {
                                "type": {
                                  "type": "string"
                                },
                                "items": {
                                  "type": "array",
                                  "items": {
                                    "type": "object",
                                    "properties": {
                                      "editable": {
                                        "type": "string"
                                      },
                                      "value": {
                                        "type": "string"
                                      },
                                      "key": {
                                        "type": "string"
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      },
                      "robotJid": {
                        "type": "string",
                        "description": "The unique credential of the Chatbot."
                      },
                      "timestamp": {
                        "type": "string",
                        "description": "The time command message was sent."
                      },
                      "toJid": {
                        "type": "string",
                        "description": "The JID of the channel or user the message was sent to."
                      },
                      "userId": {
                        "type": "string",
                        "description": "The user who sends the command."
                      },
                      "userJid": {
                        "type": "string",
                        "description": "The JID of user who is sending the command."
                      },
                      "userName": {
                        "type": "string",
                        "description": "The user's display name."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"interactive_message_fields_editable\",\n  \"event_ts\": 1560891222018,\n  \"callback_url\": \"https://api.zoom.us/v2/im/chat/callback/bKFS3ybORiWbJx6VM6Qe1A\",\n  \"callback_token\": \"eyJzdiI6IjAwMDAwMSIsInR5cCI6IkpXVCIsImFsZyI6IkVTMjU2In0.eyJhdWQiOiJib3QiLCJ1aWQiOiJWV2ZQa1NENVJvMk5xRWUyR1VMa2F3IiwiY2xpZW50SWQiOiJRQmZVaEFyUlNHVVFVclM2ZDFXVnciLCJpc3MiOiJ6b29tIiwiZXhwIjoxNzYwMzYzNzc5LCJpYXQiOjE3NjAzMzM3NzksImFpZCI6InQ4dURPZWtYUkx5b3ExR0M4UFBaTVEifQ.wBeNSNeYLQYJD1Lp0O0EnnfyCYNzF03tEwFin0-hRV7UfECLmHsW7NnQ9EEPGxOokDKnQb-pRPd_iBX4JKfg_Q\",\n  \"payload\": {\n    \"accountId\": \"gVcjZnWWRLWvv_GtyGuaxg\",\n    \"channelName\": \"Jill Chill\",\n    \"fieldEditItem\": {\n      \"currentValue\": \"\",\n      \"key\": \"Title\",\n      \"newValue\": \"Software Engineer\"\n    },\n    \"messageId\": \"20190618205323327_EZjIqCQ_aw1\",\n    \"original\": {\n      \"head\": {\n        \"sub_head\": {\n          \"text\": \"I am a sub header\"\n        },\n        \"text\": \"I am a header\"\n      },\n      \"body\": [\n        {\n          \"type\": \"fields\",\n          \"items\": [\n            {\n              \"editable\": true,\n              \"value\": \" \",\n              \"key\": \"Name\"\n            },\n            {\n              \"editable\": true,\n              \"value\": \" \",\n              \"key\": \"Title\"\n            },\n            {\n              \"editable\": false,\n              \"value\": \"Example Company\",\n              \"key\": \"Company\"\n            }\n          ]\n        }\n      ]\n    },\n    \"robotJid\": \"v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"timestamp\": 1560891222018,\n    \"toJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"userId\": \"KdYKjnimT4KPd8KKdQt9FQ\",\n    \"userJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com\",\n    \"userName\": \"Jill Chill\"\n  }\n}"
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "event": {
                      "type": "string",
                      "description": "The name of the event."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "The timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format."
                    },
                    "callback_url": {
                      "type": "string",
                      "description": "Use this endpoint to send a one-time asynchronous response within 30 minutes of receiving the event. <br> The response payload should follow the same structure as the [Send Chatbot Messages](https://developers.zoom.us/docs/api/chatbot/#tag/chatbot-messages/POST/im/chat/messages) API with required property **content** and optional properties *reply_to*, *visible_to_user* and *is_markdown_support*. <br> Include the **callback_token** value from the event as **Bearer token in the Authorization header** of your HTTP POST request."
                    },
                    "callback_token": {
                      "type": "string",
                      "description": "Use the generated token value with the **callback_url** value to send a one-time asynchronous response. The token only lasts for 30 mins. Include the **callback_token** as a **Bearer token in the Authorization header** of your HTTP request. \n Example: \n\n \ncurl --request POST \\\n  --url '{callback_url}' \\\n  --header 'authorization: Bearer {callback_token}' \\\n  --header 'content-type: application/json'\\\n  --data-raw '{\"content\":{\"head\":{\"text\":\"Response Header\"},\"body\":[{\"type\":\"message\",\"text\":\"This is an asynchronous chatbot response.\"}]}}'"
                    },
                    "payload": {
                      "type": "object",
                      "properties": {
                        "accountId": {
                          "type": "string",
                          "description": "The account ID of the user sending the command."
                        },
                        "channelName": {
                          "type": "string",
                          "description": "The channel in which the message was sent. "
                        },
                        "fieldEditItem": {
                          "type": "object",
                          "description": "The item that the user edits.",
                          "properties": {
                            "currentValue": {
                              "type": "string"
                            },
                            "key": {
                              "type": "string"
                            },
                            "newValue": {
                              "type": "string"
                            }
                          }
                        },
                        "messageId": {
                          "type": "string",
                          "description": "The unique ID of the message."
                        },
                        "original": {
                          "type": "object",
                          "description": "The original message sent to the user.",
                          "properties": {
                            "head": {
                              "type": "object",
                              "properties": {
                                "sub_head": {
                                  "type": "object",
                                  "properties": {
                                    "text": {
                                      "type": "string"
                                    }
                                  }
                                },
                                "text": {
                                  "type": "string"
                                }
                              }
                            },
                            "body": {
                              "type": "array",
                              "items": {
                                "type": "object",
                                "properties": {
                                  "type": {
                                    "type": "string"
                                  },
                                  "items": {
                                    "type": "array",
                                    "items": {
                                      "type": "object",
                                      "properties": {
                                        "editable": {
                                          "type": "string"
                                        },
                                        "value": {
                                          "type": "string"
                                        },
                                        "key": {
                                          "type": "string"
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        },
                        "robotJid": {
                          "type": "string",
                          "description": "The unique credential of the Chatbot."
                        },
                        "timestamp": {
                          "type": "string",
                          "description": "The time command message was sent."
                        },
                        "toJid": {
                          "type": "string",
                          "description": "The JID of the channel or user the message was sent to."
                        },
                        "userId": {
                          "type": "string",
                          "description": "The user who sends the command."
                        },
                        "userJid": {
                          "type": "string",
                          "description": "The JID of user who is sending the command."
                        },
                        "userName": {
                          "type": "string",
                          "description": "The user's display name."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"interactive_message_fields_editable\",\n  \"event_ts\": 1560891222018,\n  \"callback_url\": \"https://api.zoom.us/v2/im/chat/callback/bKFS3ybORiWbJx6VM6Qe1A\",\n  \"callback_token\": \"eyJzdiI6IjAwMDAwMSIsInR5cCI6IkpXVCIsImFsZyI6IkVTMjU2In0.eyJhdWQiOiJib3QiLCJ1aWQiOiJWV2ZQa1NENVJvMk5xRWUyR1VMa2F3IiwiY2xpZW50SWQiOiJRQmZVaEFyUlNHVVFVclM2ZDFXVnciLCJpc3MiOiJ6b29tIiwiZXhwIjoxNzYwMzYzNzc5LCJpYXQiOjE3NjAzMzM3NzksImFpZCI6InQ4dURPZWtYUkx5b3ExR0M4UFBaTVEifQ.wBeNSNeYLQYJD1Lp0O0EnnfyCYNzF03tEwFin0-hRV7UfECLmHsW7NnQ9EEPGxOokDKnQb-pRPd_iBX4JKfg_Q\",\n  \"payload\": {\n    \"accountId\": \"gVcjZnWWRLWvv_GtyGuaxg\",\n    \"channelName\": \"Jill Chill\",\n    \"fieldEditItem\": {\n      \"currentValue\": \"\",\n      \"key\": \"Title\",\n      \"newValue\": \"Software Engineer\"\n    },\n    \"messageId\": \"20190618205323327_EZjIqCQ_aw1\",\n    \"original\": {\n      \"head\": {\n        \"sub_head\": {\n          \"text\": \"I am a sub header\"\n        },\n        \"text\": \"I am a header\"\n      },\n      \"body\": [\n        {\n          \"type\": \"fields\",\n          \"items\": [\n            {\n              \"editable\": true,\n              \"value\": \" \",\n              \"key\": \"Name\"\n            },\n            {\n              \"editable\": true,\n              \"value\": \" \",\n              \"key\": \"Title\"\n            },\n            {\n              \"editable\": false,\n              \"value\": \"Example Company\",\n              \"key\": \"Company\"\n            }\n          ]\n        }\n      ]\n    },\n    \"robotJid\": \"v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"timestamp\": 1560891222018,\n    \"toJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"userId\": \"KdYKjnimT4KPd8KKdQt9FQ\",\n    \"userJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com\",\n    \"userName\": \"Jill Chill\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "bot_notification": {
      "post": {
        "tags": [
          "bot_notification"
        ],
        "operationId": "bot_notification",
        "requestBody": {
          "description": "# bot_notification\n\nAfter a user enters a [User Command](https://marketplace.zoom.us/docs/guides/chatbots/sending-messages#user-commands), a slash command unique to the application, Chatbots send a POST request associated with the `bot_notification` event to the [Bot Endpoint URL](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-chatbot-app#set-features) with a JSON payload containing all text entered following the command. This payload contains information on the user and event including the account, channel, timestamp, and user name.\n\nThe `cmd` value reflects the text entered by the user after the slash command. The `userName` is the string value of the user's displayed name, and `channelName` is the string value of the channel in which the command was sent.\n\n\n\n**Event type**: `bot_notification`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "event": {
                    "type": "string",
                    "description": "Name of the event."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "Timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format."
                  },
                  "payload": {
                    "type": "object",
                    "properties": {
                      "accountId": {
                        "type": "string",
                        "description": "The account ID of the user sending the command."
                      },
                      "channelName": {
                        "type": "string",
                        "description": "Channel in which the message was sent. "
                      },
                      "cmd": {
                        "type": "string",
                        "description": "Slash command entered by the user."
                      },
                      "name": {
                        "type": "string",
                        "description": "Chatbot Name."
                      },
                      "robotJid": {
                        "type": "string",
                        "description": "Unique credential of the Chatbot."
                      },
                      "timestamp": {
                        "type": "integer",
                        "format": "date-time",
                        "description": "Time command message was sent."
                      },
                      "toJid": {
                        "type": "string",
                        "description": "JID of the channel or user the message was sent to."
                      },
                      "userId": {
                        "type": "string",
                        "description": "User sending the command."
                      },
                      "userJid": {
                        "type": "string",
                        "description": "JID of user who is sending the command."
                      },
                      "userName": {
                        "type": "string",
                        "description": "User's display name."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"bot_notification\",\n  \"event_ts\": 1560796234686,\n  \"payload\": {\n      \"accountId\": \"asgVcjZnWWRLWvv_GtyGuaxg\",\n      \"channelName\": \"Photos\",\n      \"cmd\": \"island\",\n      \"name\": \"Unsplash\",\n      \"robotJid\": \"v10r4uxexurcasg-pwh8hyh7sg@example.com\",\n      \"timestamp\": 1560796234686,\n      \"toJid\": \"b1c841dc7b0b4as69287e6be05c7f93f25@conference.example.com\",\n      \"userId\": \"KdYKjnimT4asKPd8KKdQt9FQ\",\n      \"userJid\": \"kdykjnimtas4kpd8kkdqt9fq@example.com\",\n      \"userName\": \"Jill Chill\"\n      }\n}"
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "event": {
                      "type": "string",
                      "description": "Name of the event."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "Timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format."
                    },
                    "payload": {
                      "type": "object",
                      "properties": {
                        "accountId": {
                          "type": "string",
                          "description": "The account ID of the user sending the command."
                        },
                        "channelName": {
                          "type": "string",
                          "description": "Channel in which the message was sent. "
                        },
                        "cmd": {
                          "type": "string",
                          "description": "Slash command entered by the user."
                        },
                        "name": {
                          "type": "string",
                          "description": "Chatbot Name."
                        },
                        "robotJid": {
                          "type": "string",
                          "description": "Unique credential of the Chatbot."
                        },
                        "timestamp": {
                          "type": "integer",
                          "format": "date-time",
                          "description": "Time command message was sent."
                        },
                        "toJid": {
                          "type": "string",
                          "description": "JID of the channel or user the message was sent to."
                        },
                        "userId": {
                          "type": "string",
                          "description": "User sending the command."
                        },
                        "userJid": {
                          "type": "string",
                          "description": "JID of user who is sending the command."
                        },
                        "userName": {
                          "type": "string",
                          "description": "User's display name."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"bot_notification\",\n  \"event_ts\": 1560796234686,\n  \"payload\": {\n      \"accountId\": \"asgVcjZnWWRLWvv_GtyGuaxg\",\n      \"channelName\": \"Photos\",\n      \"cmd\": \"island\",\n      \"name\": \"Unsplash\",\n      \"robotJid\": \"v10r4uxexurcasg-pwh8hyh7sg@example.com\",\n      \"timestamp\": 1560796234686,\n      \"toJid\": \"b1c841dc7b0b4as69287e6be05c7f93f25@conference.example.com\",\n      \"userId\": \"KdYKjnimT4asKPd8KKdQt9FQ\",\n      \"userJid\": \"kdykjnimtas4kpd8kkdqt9fq@example.com\",\n      \"userName\": \"Jill Chill\"\n      }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "interactive_message_actions": {
      "post": {
        "tags": [
          "interactive_message_actions"
        ],
        "operationId": "interactive_message_actions",
        "requestBody": {
          "description": "# interactive_message_actions\n\nAfter a user clicks a [button in a Chatbot message](https://marketplace.zoom.us/docs/guides/chatbots/customizing-messages/message-with-buttons), the `interactive_message_actions` event is invoked and a POST request with data on the selected option is sent to the [Bot Endpoint URL](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-chatbot-app#set-features) specified in the app's dashboard.\n\nThe `actionItem` object displays the value chosen by the user from the `items` array of the original message from the Chatbot.\n\n\n\n**Event type**: `interactive_message_actions`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "event": {
                    "type": "string",
                    "description": "The name of the event."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "The timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format."
                  },
                  "callback_url": {
                    "type": "string",
                    "description": "Use this endpoint to send a one-time asynchronous response within 30 minutes of receiving the event. <br> The response payload should follow the same structure as the [Send Chatbot Messages](https://developers.zoom.us/docs/api/chatbot/#tag/chatbot-messages/POST/im/chat/messages) API with required property **content** and optional properties *reply_to*, *visible_to_user* and *is_markdown_support*. <br> Include the **callback_token** value from the event as **Bearer token in the Authorization header** of your HTTP POST request."
                  },
                  "callback_token": {
                    "type": "string",
                    "description": "Use the generated token value with the **callback_url** value to send a one-time asynchronous response. The token only lasts for 30 mins. Include the **callback_token** as a **Bearer token in the Authorization header** of your HTTP request. \n Example: \n\n \ncurl --request POST \\\n  --url '{callback_url}' \\\n  --header 'authorization: Bearer {callback_token}' \\\n  --header 'content-type: application/json'\\\n  --data-raw '{\"content\":{\"head\":{\"text\":\"Response Header\"},\"body\":[{\"type\":\"message\",\"text\":\"This is an asynchronous chatbot response.\"}]}}'"
                  },
                  "payload": {
                    "type": "object",
                    "properties": {
                      "accountId": {
                        "type": "string",
                        "description": "The account ID of the user sending the command."
                      },
                      "actionItem": {
                        "type": "object",
                        "description": "Action taken by the user.",
                        "properties": {
                          "text": {
                            "type": "string"
                          },
                          "value": {
                            "type": "string"
                          }
                        }
                      },
                      "channelName": {
                        "type": "string",
                        "description": "Channel in which the message was sent."
                      },
                      "messageId": {
                        "type": "string",
                        "description": "Unique ID of the message."
                      },
                      "original": {
                        "type": "object",
                        "description": "Original message sent to the user.",
                        "properties": {
                          "head": {
                            "type": "object",
                            "properties": {
                              "sub_head": {
                                "type": "object",
                                "properties": {
                                  "text": {
                                    "type": "string"
                                  }
                                }
                              },
                              "text": {
                                "type": "string"
                              }
                            }
                          },
                          "body": {
                            "type": "array",
                            "items": {
                              "type": "object",
                              "properties": {
                                "type": {
                                  "type": "string"
                                },
                                "items": {
                                  "type": "array",
                                  "items": {
                                    "type": "object",
                                    "properties": {
                                      "style": {
                                        "type": "string"
                                      },
                                      "text": {
                                        "type": "string"
                                      },
                                      "value": {
                                        "type": "string"
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      },
                      "robotJid": {
                        "type": "string",
                        "description": "Unique credential of the Chatbot."
                      },
                      "timestamp": {
                        "type": "integer",
                        "description": "Time command message was sent."
                      },
                      "toJid": {
                        "type": "string",
                        "description": "JID of the channel or user the message was sent to."
                      },
                      "userId": {
                        "type": "string",
                        "description": "User sending the command."
                      },
                      "userJid": {
                        "type": "string",
                        "description": "JID of user who is sending the command."
                      },
                      "userName": {
                        "type": "string",
                        "description": "User's display name."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"interactive_message_actions\",\n  \"event_ts\": 1560891332219,\n  \"callback_url\": \"https://api.zoom.us/v2/im/chat/callback/bKFS3ybORiWbJx6VM6Qe1A\",\n  \"callback_token\": \"eyJzdiI6IjAwMDAwMSIsInR5cCI6IkpXVCIsImFsZyI6IkVTMjU2In0.eyJhdWQiOiJib3QiLCJ1aWQiOiJWV2ZQa1NENVJvMk5xRWUyR1VMa2F3IiwiY2xpZW50SWQiOiJRQmZVaEFyUlNHVVFVclM2ZDFXVnciLCJpc3MiOiJ6b29tIiwiZXhwIjoxNzYwMzYzNzc5LCJpYXQiOjE3NjAzMzM3NzksImFpZCI6InQ4dURPZWtYUkx5b3ExR0M4UFBaTVEifQ.wBeNSNeYLQYJD1Lp0O0EnnfyCYNzF03tEwFin0-hRV7UfECLmHsW7NnQ9EEPGxOokDKnQb-pRPd_iBX4JKfg_Q\",\n  \"payload\": {\n    \"accountId\": \"gVcjZnWWRLWvv_GtyGuaxg\",\n    \"actionItem\": {\n      \"text\": \"Add\",\n      \"value\": \"add\"\n    },\n    \"channelName\": \"Jill Chill\",\n    \"messageId\": \"20190618205516496_ynICmz8_aw1\",\n    \"original\": {\n      \"head\": {\n        \"sub_head\": {\n          \"text\": \"I am a sub header\"\n        },\n        \"text\": \"I am a header\"\n      },\n      \"body\": [\n        {\n          \"type\": \"actions\",\n          \"items\": [\n            {\n              \"style\": \"Primary\",\n              \"text\": \"Add\",\n              \"value\": \"add\"\n            },\n            {\n              \"style\": \"Default\",\n              \"text\": \"Update\",\n              \"value\": \"update\"\n            },\n            {\n              \"style\": \"Danger\",\n              \"text\": \"Delete\",\n              \"value\": \"delete\"\n            },\n            {\n              \"style\": \"Disabled\",\n              \"text\": \"Disabled\",\n              \"value\": \"disable\"\n            }\n          ]\n        }\n      ]\n    },\n    \"robotJid\": \"v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"timestamp\": 1560891332219,\n    \"toJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"userId\": \"KdYKjnimT4KPd8KKdQt9FQ\",\n    \"userJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com\",\n    \"userName\": \"Jill Chill\"\n  }\n}"
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "event": {
                      "type": "string",
                      "description": "The name of the event."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "The timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format."
                    },
                    "callback_url": {
                      "type": "string",
                      "description": "Use this endpoint to send a one-time asynchronous response within 30 minutes of receiving the event. <br> The response payload should follow the same structure as the [Send Chatbot Messages](https://developers.zoom.us/docs/api/chatbot/#tag/chatbot-messages/POST/im/chat/messages) API with required property **content** and optional properties *reply_to*, *visible_to_user* and *is_markdown_support*. <br> Include the **callback_token** value from the event as **Bearer token in the Authorization header** of your HTTP POST request."
                    },
                    "callback_token": {
                      "type": "string",
                      "description": "Use the generated token value with the **callback_url** value to send a one-time asynchronous response. The token only lasts for 30 mins. Include the **callback_token** as a **Bearer token in the Authorization header** of your HTTP request. \n Example: \n\n \ncurl --request POST \\\n  --url '{callback_url}' \\\n  --header 'authorization: Bearer {callback_token}' \\\n  --header 'content-type: application/json'\\\n  --data-raw '{\"content\":{\"head\":{\"text\":\"Response Header\"},\"body\":[{\"type\":\"message\",\"text\":\"This is an asynchronous chatbot response.\"}]}}'"
                    },
                    "payload": {
                      "type": "object",
                      "properties": {
                        "accountId": {
                          "type": "string",
                          "description": "The account ID of the user sending the command."
                        },
                        "actionItem": {
                          "type": "object",
                          "description": "Action taken by the user.",
                          "properties": {
                            "text": {
                              "type": "string"
                            },
                            "value": {
                              "type": "string"
                            }
                          }
                        },
                        "channelName": {
                          "type": "string",
                          "description": "Channel in which the message was sent."
                        },
                        "messageId": {
                          "type": "string",
                          "description": "Unique ID of the message."
                        },
                        "original": {
                          "type": "object",
                          "description": "Original message sent to the user.",
                          "properties": {
                            "head": {
                              "type": "object",
                              "properties": {
                                "sub_head": {
                                  "type": "object",
                                  "properties": {
                                    "text": {
                                      "type": "string"
                                    }
                                  }
                                },
                                "text": {
                                  "type": "string"
                                }
                              }
                            },
                            "body": {
                              "type": "array",
                              "items": {
                                "type": "object",
                                "properties": {
                                  "type": {
                                    "type": "string"
                                  },
                                  "items": {
                                    "type": "array",
                                    "items": {
                                      "type": "object",
                                      "properties": {
                                        "style": {
                                          "type": "string"
                                        },
                                        "text": {
                                          "type": "string"
                                        },
                                        "value": {
                                          "type": "string"
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        },
                        "robotJid": {
                          "type": "string",
                          "description": "Unique credential of the Chatbot."
                        },
                        "timestamp": {
                          "type": "integer",
                          "description": "Time command message was sent."
                        },
                        "toJid": {
                          "type": "string",
                          "description": "JID of the channel or user the message was sent to."
                        },
                        "userId": {
                          "type": "string",
                          "description": "User sending the command."
                        },
                        "userJid": {
                          "type": "string",
                          "description": "JID of user who is sending the command."
                        },
                        "userName": {
                          "type": "string",
                          "description": "User's display name."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"interactive_message_actions\",\n  \"event_ts\": 1560891332219,\n  \"callback_url\": \"https://api.zoom.us/v2/im/chat/callback/bKFS3ybORiWbJx6VM6Qe1A\",\n  \"callback_token\": \"eyJzdiI6IjAwMDAwMSIsInR5cCI6IkpXVCIsImFsZyI6IkVTMjU2In0.eyJhdWQiOiJib3QiLCJ1aWQiOiJWV2ZQa1NENVJvMk5xRWUyR1VMa2F3IiwiY2xpZW50SWQiOiJRQmZVaEFyUlNHVVFVclM2ZDFXVnciLCJpc3MiOiJ6b29tIiwiZXhwIjoxNzYwMzYzNzc5LCJpYXQiOjE3NjAzMzM3NzksImFpZCI6InQ4dURPZWtYUkx5b3ExR0M4UFBaTVEifQ.wBeNSNeYLQYJD1Lp0O0EnnfyCYNzF03tEwFin0-hRV7UfECLmHsW7NnQ9EEPGxOokDKnQb-pRPd_iBX4JKfg_Q\",\n  \"payload\": {\n    \"accountId\": \"gVcjZnWWRLWvv_GtyGuaxg\",\n    \"actionItem\": {\n      \"text\": \"Add\",\n      \"value\": \"add\"\n    },\n    \"channelName\": \"Jill Chill\",\n    \"messageId\": \"20190618205516496_ynICmz8_aw1\",\n    \"original\": {\n      \"head\": {\n        \"sub_head\": {\n          \"text\": \"I am a sub header\"\n        },\n        \"text\": \"I am a header\"\n      },\n      \"body\": [\n        {\n          \"type\": \"actions\",\n          \"items\": [\n            {\n              \"style\": \"Primary\",\n              \"text\": \"Add\",\n              \"value\": \"add\"\n            },\n            {\n              \"style\": \"Default\",\n              \"text\": \"Update\",\n              \"value\": \"update\"\n            },\n            {\n              \"style\": \"Danger\",\n              \"text\": \"Delete\",\n              \"value\": \"delete\"\n            },\n            {\n              \"style\": \"Disabled\",\n              \"text\": \"Disabled\",\n              \"value\": \"disable\"\n            }\n          ]\n        }\n      ]\n    },\n    \"robotJid\": \"v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"timestamp\": 1560891332219,\n    \"toJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"userId\": \"KdYKjnimT4KPd8KKdQt9FQ\",\n    \"userJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com\",\n    \"userName\": \"Jill Chill\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "interactive_message_editable": {
      "post": {
        "tags": [
          "interactive_message_editable"
        ],
        "operationId": "interactive_message_editable",
        "requestBody": {
          "description": "# interactive_message_editable\n\nAfter a user edits the text of a Chatbot [message with Editable Text](https://marketplace.zoom.us/docs/guides/chatbots/customizing-messages/message-with-editable-text), the `interactive_message_editable` event is invoked and a POST request with data about the event is sent to the [Bot Endpoint URL](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-chatbot-app#set-features) specfied in the app's dashboard.\n\nThe `editItem` object displays the changes in edited text with `origin` and `target` values.\n\n\n\n**Event type**: `interactive_message_editable`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "event": {
                    "type": "string",
                    "description": "The name of the event."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "The timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format."
                  },
                  "callback_url": {
                    "type": "string",
                    "description": "Use this endpoint to send a one-time asynchronous response within 30 minutes of receiving the event. <br> The response payload should follow the same structure as the [Send Chatbot Messages](https://developers.zoom.us/docs/api/chatbot/#tag/chatbot-messages/POST/im/chat/messages) API with required property **content** and optional properties *reply_to*, *visible_to_user* and *is_markdown_support*. <br> Include the **callback_token** value from the event as **Bearer token in the Authorization header** of your HTTP POST request."
                  },
                  "callback_token": {
                    "type": "string",
                    "description": "Use the generated token value with the **callback_url** value to send a one-time asynchronous response. The token only lasts for 30 mins. Include the **callback_token** as a **Bearer token in the Authorization header** of your HTTP request. \n Example: \n\n \ncurl --request POST \\\n  --url '{callback_url}' \\\n  --header 'authorization: Bearer {callback_token}' \\\n  --header 'content-type: application/json'\\\n  --data-raw '{\"content\":{\"head\":{\"text\":\"Response Header\"},\"body\":[{\"type\":\"message\",\"text\":\"This is an asynchronous chatbot response.\"}]}}'"
                  },
                  "payload": {
                    "type": "object",
                    "properties": {
                      "accountId": {
                        "type": "string",
                        "description": "The account ID of the user editing the message."
                      },
                      "channelName": {
                        "type": "string",
                        "description": "The channel in which the message was edited."
                      },
                      "editItem": {
                        "type": "object",
                        "description": "The object containing original and edited value.",
                        "properties": {
                          "origin": {
                            "type": "string"
                          },
                          "target": {
                            "type": "string"
                          }
                        }
                      },
                      "messageId": {
                        "type": "string",
                        "description": "The unique ID of the message."
                      },
                      "original": {
                        "type": "object",
                        "description": "The original message sent to the user.",
                        "properties": {
                          "head": {
                            "type": "object",
                            "properties": {
                              "sub_head": {
                                "type": "object",
                                "properties": {
                                  "text": {
                                    "type": "string"
                                  }
                                }
                              },
                              "text": {
                                "type": "string"
                              }
                            }
                          },
                          "body": {
                            "type": "array",
                            "items": {
                              "type": "object",
                              "properties": {
                                "": {
                                  "type": "object",
                                  "properties": {
                                    "editable": {
                                      "type": "boolean"
                                    },
                                    "text": {
                                      "type": "string"
                                    },
                                    "type": {
                                      "type": "string"
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      },
                      "robotJid": {
                        "type": "string",
                        "description": "The unique credential of the Chatbot."
                      },
                      "timestamp": {
                        "type": "integer",
                        "format": "date-time",
                        "description": "The time command message was sent."
                      },
                      "toJid": {
                        "type": "string",
                        "description": "The JID of the channel or user the message was sent to."
                      },
                      "userId": {
                        "type": "string",
                        "description": "The user who sends the command."
                      },
                      "userJid": {
                        "type": "string",
                        "description": "The JID of user who is sending the command."
                      },
                      "userName": {
                        "type": "string",
                        "description": "The user's display name."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"interactive_message_editable\",\n  \"event_ts\": 1560891135776,\n  \"callback_url\": \"https://api.zoom.us/v2/im/chat/callback/bKFS3ybORiWbJx6VM6Qe1A\",\n  \"callback_token\": \"eyJzdiI6IjAwMDAwMSIsInR5cCI6IkpXVCIsImFsZyI6IkVTMjU2In0.eyJhdWQiOiJib3QiLCJ1aWQiOiJWV2ZQa1NENVJvMk5xRWUyR1VMa2F3IiwiY2xpZW50SWQiOiJRQmZVaEFyUlNHVVFVclM2ZDFXVnciLCJpc3MiOiJ6b29tIiwiZXhwIjoxNzYwMzYzNzc5LCJpYXQiOjE3NjAzMzM3NzksImFpZCI6InQ4dURPZWtYUkx5b3ExR0M4UFBaTVEifQ.wBeNSNeYLQYJD1Lp0O0EnnfyCYNzF03tEwFin0-hRV7UfECLmHsW7NnQ9EEPGxOokDKnQb-pRPd_iBX4JKfg_Q\",\n  \"payload\": {\n    \"accountId\": \"gVcjZnWWRLWvv_GtyGuaxg\",\n    \"channelName\": \"Jill Chill\",\n    \"editItem\": {\n      \"origin\": \"I am a message with editable text\",\n      \"target\": \"I am a message with editable text that has been edited\"\n    },\n    \"messageId\": \"20190618205205291_tI1CUa6_aw1\",\n    \"original\": {\n      \"head\": {\n        \"sub_head\": {\n          \"text\": \"I am a sub header\"\n        },\n        \"text\": \"I am a header\"\n      },\n      \"body\": [\n        {\n          \"editable\": true,\n          \"text\": \"I am a message with editable text\",\n          \"type\": \"message\"\n        }\n      ]\n    },\n    \"robotJid\": \"v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"timestamp\": 1560891135776,\n    \"toJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"userId\": \"KdYKjnimT4KPd8KKdQt9FQ\",\n    \"userJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com\",\n    \"userName\": \"Jill Chill\"\n  }\n}"
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "event": {
                      "type": "string",
                      "description": "The name of the event."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "The timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format."
                    },
                    "callback_url": {
                      "type": "string",
                      "description": "Use this endpoint to send a one-time asynchronous response within 30 minutes of receiving the event. <br> The response payload should follow the same structure as the [Send Chatbot Messages](https://developers.zoom.us/docs/api/chatbot/#tag/chatbot-messages/POST/im/chat/messages) API with required property **content** and optional properties *reply_to*, *visible_to_user* and *is_markdown_support*. <br> Include the **callback_token** value from the event as **Bearer token in the Authorization header** of your HTTP POST request."
                    },
                    "callback_token": {
                      "type": "string",
                      "description": "Use the generated token value with the **callback_url** value to send a one-time asynchronous response. The token only lasts for 30 mins. Include the **callback_token** as a **Bearer token in the Authorization header** of your HTTP request. \n Example: \n\n \ncurl --request POST \\\n  --url '{callback_url}' \\\n  --header 'authorization: Bearer {callback_token}' \\\n  --header 'content-type: application/json'\\\n  --data-raw '{\"content\":{\"head\":{\"text\":\"Response Header\"},\"body\":[{\"type\":\"message\",\"text\":\"This is an asynchronous chatbot response.\"}]}}'"
                    },
                    "payload": {
                      "type": "object",
                      "properties": {
                        "accountId": {
                          "type": "string",
                          "description": "The account ID of the user editing the message."
                        },
                        "channelName": {
                          "type": "string",
                          "description": "The channel in which the message was edited."
                        },
                        "editItem": {
                          "type": "object",
                          "description": "The object containing original and edited value.",
                          "properties": {
                            "origin": {
                              "type": "string"
                            },
                            "target": {
                              "type": "string"
                            }
                          }
                        },
                        "messageId": {
                          "type": "string",
                          "description": "The unique ID of the message."
                        },
                        "original": {
                          "type": "object",
                          "description": "The original message sent to the user.",
                          "properties": {
                            "head": {
                              "type": "object",
                              "properties": {
                                "sub_head": {
                                  "type": "object",
                                  "properties": {
                                    "text": {
                                      "type": "string"
                                    }
                                  }
                                },
                                "text": {
                                  "type": "string"
                                }
                              }
                            },
                            "body": {
                              "type": "array",
                              "items": {
                                "type": "object",
                                "properties": {
                                  "": {
                                    "type": "object",
                                    "properties": {
                                      "editable": {
                                        "type": "boolean"
                                      },
                                      "text": {
                                        "type": "string"
                                      },
                                      "type": {
                                        "type": "string"
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        },
                        "robotJid": {
                          "type": "string",
                          "description": "The unique credential of the Chatbot."
                        },
                        "timestamp": {
                          "type": "integer",
                          "format": "date-time",
                          "description": "The time command message was sent."
                        },
                        "toJid": {
                          "type": "string",
                          "description": "The JID of the channel or user the message was sent to."
                        },
                        "userId": {
                          "type": "string",
                          "description": "The user who sends the command."
                        },
                        "userJid": {
                          "type": "string",
                          "description": "The JID of user who is sending the command."
                        },
                        "userName": {
                          "type": "string",
                          "description": "The user's display name."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"interactive_message_editable\",\n  \"event_ts\": 1560891135776,\n  \"callback_url\": \"https://api.zoom.us/v2/im/chat/callback/bKFS3ybORiWbJx6VM6Qe1A\",\n  \"callback_token\": \"eyJzdiI6IjAwMDAwMSIsInR5cCI6IkpXVCIsImFsZyI6IkVTMjU2In0.eyJhdWQiOiJib3QiLCJ1aWQiOiJWV2ZQa1NENVJvMk5xRWUyR1VMa2F3IiwiY2xpZW50SWQiOiJRQmZVaEFyUlNHVVFVclM2ZDFXVnciLCJpc3MiOiJ6b29tIiwiZXhwIjoxNzYwMzYzNzc5LCJpYXQiOjE3NjAzMzM3NzksImFpZCI6InQ4dURPZWtYUkx5b3ExR0M4UFBaTVEifQ.wBeNSNeYLQYJD1Lp0O0EnnfyCYNzF03tEwFin0-hRV7UfECLmHsW7NnQ9EEPGxOokDKnQb-pRPd_iBX4JKfg_Q\",\n  \"payload\": {\n    \"accountId\": \"gVcjZnWWRLWvv_GtyGuaxg\",\n    \"channelName\": \"Jill Chill\",\n    \"editItem\": {\n      \"origin\": \"I am a message with editable text\",\n      \"target\": \"I am a message with editable text that has been edited\"\n    },\n    \"messageId\": \"20190618205205291_tI1CUa6_aw1\",\n    \"original\": {\n      \"head\": {\n        \"sub_head\": {\n          \"text\": \"I am a sub header\"\n        },\n        \"text\": \"I am a header\"\n      },\n      \"body\": [\n        {\n          \"editable\": true,\n          \"text\": \"I am a message with editable text\",\n          \"type\": \"message\"\n        }\n      ]\n    },\n    \"robotJid\": \"v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"timestamp\": 1560891135776,\n    \"toJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"userId\": \"KdYKjnimT4KPd8KKdQt9FQ\",\n    \"userJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com\",\n    \"userName\": \"Jill Chill\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "bot_installed": {
      "post": {
        "tags": [
          "bot_installed"
        ],
        "operationId": "bot_installed",
        "requestBody": {
          "description": "# bot_installed\nTo configure a dynamic welcome message:\n1. Go to your Chatbot's settings and navigate to the **Features** page.\n2. Check **Your App** for the welcome message.\n3. Save your settings.\n\nAfter your Chatbot is authorized, Zoom sends an HTTP POST request to your respective Bot Endpoint URL with the following request body:\n\n**Event type**: `bot_installed`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "event": {
                    "type": "string",
                    "description": "Name of the event."
                  },
                  "payload": {
                    "type": "object",
                    "properties": {
                      "accountId": {
                        "type": "string",
                        "description": "The account ID of the user sending the command."
                      },
                      "robotJid": {
                        "type": "string",
                        "description": "Unique credential of the Chatbot."
                      },
                      "timestamp": {
                        "type": "integer",
                        "format": "date-time",
                        "description": "Time command message was sent."
                      },
                      "userId": {
                        "type": "string",
                        "description": "User sending the command."
                      },
                      "userJid": {
                        "type": "string",
                        "description": "JID of user who is sending the command."
                      },
                      "userName": {
                        "type": "string",
                        "description": "User's display name."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"bot_installed\",\n  \"payload\": {\n    \"accountId\": \"CDSCdcsdv\",\n    \"robotJid\": \"csvfdvfd@xmpp.zoom.us\",\n    \"timestamp\": 1542796725586,\n    \"userId\": \"fvdfvfdvfd\",\n    \"userJid\": \"fvdfvfdvfd@xmpp.zoom.us\",\n    \"userName\": \"UserName\"\n  }\n}"
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "event": {
                      "type": "string",
                      "description": "Name of the event."
                    },
                    "payload": {
                      "type": "object",
                      "properties": {
                        "accountId": {
                          "type": "string",
                          "description": "The account ID of the user sending the command."
                        },
                        "robotJid": {
                          "type": "string",
                          "description": "Unique credential of the Chatbot."
                        },
                        "timestamp": {
                          "type": "integer",
                          "format": "date-time",
                          "description": "Time command message was sent."
                        },
                        "userId": {
                          "type": "string",
                          "description": "User sending the command."
                        },
                        "userJid": {
                          "type": "string",
                          "description": "JID of user who is sending the command."
                        },
                        "userName": {
                          "type": "string",
                          "description": "User's display name."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"bot_installed\",\n  \"payload\": {\n    \"accountId\": \"CDSCdcsdv\",\n    \"robotJid\": \"csvfdvfd@xmpp.zoom.us\",\n    \"timestamp\": 1542796725586,\n    \"userId\": \"fvdfvfdvfd\",\n    \"userJid\": \"fvdfvfdvfd@xmpp.zoom.us\",\n    \"userName\": \"UserName\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "team_chat.app_mention": {
      "post": {
        "tags": [
          "team_chat"
        ],
        "operationId": "team_chat.app_mention",
        "requestBody": {
          "description": "# team_chat.app_mention\nThis event triggers when a user mentions a specific app in a chat channel. A POST request with the mention message is sent to the [Bot Endpoint URL](https://developers.zoom.us/docs/team-chat-apps/team-chat-subscriptions/) specified in the app's dashboard.\n## Prerequisites\n* A Pro or a higher [plan](https://zoom.us/pricing).\n\n**Event type**: `team_chat.app_mention`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "required": [
                  "event",
                  "event_ts",
                  "payload"
                ],
                "properties": {
                  "event": {
                    "type": "string",
                    "description": "The name of the event.",
                    "enum": [
                      "team_chat.app_mention"
                    ]
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "A timestamp at which the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "operator",
                      "operator_id",
                      "object",
                      "operator_member_id",
                      "by_external_user"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who sent the app mention message. Will be empty when by_external_user is true"
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user who sent the app mention message. Will be empty when by_external_user is true"
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user who sent the app mention message. Will be empty when by_external_user is true"
                      },
                      "operator_member_id": {
                        "type": "string",
                        "description": "The member ID of the user who sent the app mention message."
                      },
                      "by_external_user": {
                        "type": "boolean",
                        "description": "Whether this action was performed by an external user."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the app mention message.",
                        "required": [
                          "message_id",
                          "type",
                          "message",
                          "timestamp",
                          "date_time"
                        ],
                        "properties": {
                          "message_id": {
                            "type": "string",
                            "description": "The message's ID."
                          },
                          "type": {
                            "type": "string",
                            "description": "The type of message: \n* to_channel \u2014 The message was sent to a chat channel.",
                            "enum": [
                              "to_channel"
                            ]
                          },
                          "channel_id": {
                            "type": "string",
                            "description": "The chat channel's ID."
                          },
                          "channel_name": {
                            "type": "string",
                            "description": "The chat channel's name."
                          },
                          "message": {
                            "type": "string",
                            "description": "The sent message's content."
                          },
                          "files": {
                            "type": "array",
                            "description": "The attachments file info in sent message's content.",
                            "items": {
                              "type": "object",
                              "required": [
                                "OS_file_type",
                                "file_id",
                                "file_message_type",
                                "file_name",
                                "file_size"
                              ],
                              "properties": {
                                "OS_file_type": {
                                  "type": "string",
                                  "description": "The attachments file extension type"
                                },
                                "file_id": {
                                  "type": "string",
                                  "description": "The attachments file's universally unique identifier (UUID), in base64-encoded format."
                                },
                                "file_message_type": {
                                  "type": "string",
                                  "description": "The attachments file message type",
                                  "enum": [
                                    "file",
                                    "image",
                                    "audio",
                                    "audio v2",
                                    "code snippet",
                                    "screen shot"
                                  ]
                                },
                                "file_name": {
                                  "type": "string",
                                  "description": "The attachments file's name."
                                },
                                "file_size": {
                                  "type": "integer",
                                  "description": "The attachments file's size, in bytes."
                                }
                              },
                              "additionalProperties": false
                            }
                          },
                          "rich_text": {
                            "type": "array",
                            "description": "The [rich text formatting](https://support.zoom.us/hc/en-us/articles/115004789183-Replying-to-and-managing-chat-messages)object. Use this object to include rich text info in sent message's content.",
                            "items": {
                              "type": "object",
                              "required": [
                                "start_position",
                                "end_position",
                                "format_type"
                              ],
                              "properties": {
                                "start_position": {
                                  "type": "integer",
                                  "description": "The start position of the rich text in the message string. \n\nFor example if the first character at the beginning of the message is bold, the value for this field will be 0."
                                },
                                "end_position": {
                                  "type": "integer",
                                  "description": "The end position of the rich text."
                                },
                                "format_type": {
                                  "type": "string",
                                  "description": "The type of rich text.",
                                  "enum": [
                                    "Bold",
                                    "Italic",
                                    "Strikethrough",
                                    "BulletedList",
                                    "NumberedList",
                                    "Underline",
                                    "FontSize",
                                    "FontColor",
                                    "BackgroundColor",
                                    "LeftIndent",
                                    "Paragraph",
                                    "Quote",
                                    "AddLink"
                                  ]
                                },
                                "format_attr": {
                                  "type": "string",
                                  "description": "This field is non-null if the value of the format_type field is listed below:\n\n1.FontSize value: s for small, m for medium, or l for large font size. \n\n2.FontColor and BackgroundColor value: only supports RGB value. For example: FFC0CB \n\n3.LeftIndent value: a positive pixel length.  \n\n4.Paragraph value: h1 for Heading 1, h2 for Heading 2, or h3 for Heading 3. \n\n5.AddLink value: must be a valid URL, with an http or https prefix. For example: https://example.com"
                                }
                              },
                              "additionalProperties": false
                            },
                            "reply_main_message_id": {
                              "type": "string",
                              "description": "The ID of the parent message in the chat thread.It's only available if the message is a reply to a main message in a thread."
                            },
                            "date_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The date and time at which the message was sent, in yyyy-MM-dd'T'HH:mm:ss'Z' format."
                            },
                            "timestamp": {
                              "type": "integer",
                              "format": "int64",
                              "description": "The timestamp for when the message was sent."
                            }
                          }
                        },
                        "additionalProperties": false
                      }
                    },
                    "additionalProperties": false
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"team_chat.app_mention\",\n  \"event_ts\": 1696914490200,\n  \"payload\": {\n    \"account_id\": \"asgVcjZnWWRLWvv_GtyGuaxg\",\n    \"operator\": \"example@zoom.us\",\n    \"operator_member_id\": \"R4VM29Oj0fVM2hhEmSKVM2hhezJTezJTKVM2hezJT2hezJTSK\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"by_external_user\": true,\n    \"object\": {\n      \"message_id\": \"5DD614F4-DD19-4C39-8D03-93C4D0A0C839\",\n      \"type\": \"to_channel\",\n      \"channel_id\": \"8AmQ0WOoQ1O3a84yYFc2xQ\",\n      \"channel_name\": \"Team Chat\",\n      \"message\": \"hello\",\n      \"files\": [\n        {\n          \"file_id\": \"56bed9orRnOSEnBoX_HQvA\",\n          \"file_name\": \"Screenshot 2023-12-22 at 11.21.38 AM.png\",\n          \"file_size\": 23025,\n          \"OS_file_type\": \"PNG\",\n          \"file_message_type\": \"image\"\n        }\n      ],\n      \"rich_text\": [\n        {\n          \"start_position\": 0,\n          \"end_position\": 1,\n          \"format_type\": \"Bold\"\n        },\n        {\n          \"start_position\": 1,\n          \"end_position\": 2,\n          \"format_type\": \"BulletedList\"\n        },\n        {\n          \"start_position\": 2,\n          \"end_position\": 3,\n          \"format_type\": \"AddLink\",\n          \"format_attr\": \"https://www.example.com/templates/daily-stand-up\"\n        }\n      ],\n      \"reply_main_message_id\": \"5DD614F4-DD19-4C39-8D03-93C4D0A0C838\",\n      \"date_time\": \"2023-10-10T05:08:10Z\",\n      \"timestamp\": 1696914490157\n    }\n  }\n}"
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "required": [
                    "event",
                    "event_ts",
                    "payload"
                  ],
                  "properties": {
                    "event": {
                      "type": "string",
                      "description": "The name of the event.",
                      "enum": [
                        "team_chat.app_mention"
                      ]
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "A timestamp at which the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "operator",
                        "operator_id",
                        "object",
                        "operator_member_id",
                        "by_external_user"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who sent the app mention message. Will be empty when by_external_user is true"
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user who sent the app mention message. Will be empty when by_external_user is true"
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user who sent the app mention message. Will be empty when by_external_user is true"
                        },
                        "operator_member_id": {
                          "type": "string",
                          "description": "The member ID of the user who sent the app mention message."
                        },
                        "by_external_user": {
                          "type": "boolean",
                          "description": "Whether this action was performed by an external user."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the app mention message.",
                          "required": [
                            "message_id",
                            "type",
                            "message",
                            "timestamp",
                            "date_time"
                          ],
                          "properties": {
                            "message_id": {
                              "type": "string",
                              "description": "The message's ID."
                            },
                            "type": {
                              "type": "string",
                              "description": "The type of message: \n* to_channel \u2014 The message was sent to a chat channel.",
                              "enum": [
                                "to_channel"
                              ]
                            },
                            "channel_id": {
                              "type": "string",
                              "description": "The chat channel's ID."
                            },
                            "channel_name": {
                              "type": "string",
                              "description": "The chat channel's name."
                            },
                            "message": {
                              "type": "string",
                              "description": "The sent message's content."
                            },
                            "files": {
                              "type": "array",
                              "description": "The attachments file info in sent message's content.",
                              "items": {
                                "type": "object",
                                "required": [
                                  "OS_file_type",
                                  "file_id",
                                  "file_message_type",
                                  "file_name",
                                  "file_size"
                                ],
                                "properties": {
                                  "OS_file_type": {
                                    "type": "string",
                                    "description": "The attachments file extension type"
                                  },
                                  "file_id": {
                                    "type": "string",
                                    "description": "The attachments file's universally unique identifier (UUID), in base64-encoded format."
                                  },
                                  "file_message_type": {
                                    "type": "string",
                                    "description": "The attachments file message type",
                                    "enum": [
                                      "file",
                                      "image",
                                      "audio",
                                      "audio v2",
                                      "code snippet",
                                      "screen shot"
                                    ]
                                  },
                                  "file_name": {
                                    "type": "string",
                                    "description": "The attachments file's name."
                                  },
                                  "file_size": {
                                    "type": "integer",
                                    "description": "The attachments file's size, in bytes."
                                  }
                                },
                                "additionalProperties": false
                              }
                            },
                            "rich_text": {
                              "type": "array",
                              "description": "The [rich text formatting](https://support.zoom.us/hc/en-us/articles/115004789183-Replying-to-and-managing-chat-messages)object. Use this object to include rich text info in sent message's content.",
                              "items": {
                                "type": "object",
                                "required": [
                                  "start_position",
                                  "end_position",
                                  "format_type"
                                ],
                                "properties": {
                                  "start_position": {
                                    "type": "integer",
                                    "description": "The start position of the rich text in the message string. \n\nFor example if the first character at the beginning of the message is bold, the value for this field will be 0."
                                  },
                                  "end_position": {
                                    "type": "integer",
                                    "description": "The end position of the rich text."
                                  },
                                  "format_type": {
                                    "type": "string",
                                    "description": "The type of rich text.",
                                    "enum": [
                                      "Bold",
                                      "Italic",
                                      "Strikethrough",
                                      "BulletedList",
                                      "NumberedList",
                                      "Underline",
                                      "FontSize",
                                      "FontColor",
                                      "BackgroundColor",
                                      "LeftIndent",
                                      "Paragraph",
                                      "Quote",
                                      "AddLink"
                                    ]
                                  },
                                  "format_attr": {
                                    "type": "string",
                                    "description": "This field is non-null if the value of the format_type field is listed below:\n\n1.FontSize value: s for small, m for medium, or l for large font size. \n\n2.FontColor and BackgroundColor value: only supports RGB value. For example: FFC0CB \n\n3.LeftIndent value: a positive pixel length.  \n\n4.Paragraph value: h1 for Heading 1, h2 for Heading 2, or h3 for Heading 3. \n\n5.AddLink value: must be a valid URL, with an http or https prefix. For example: https://example.com"
                                  }
                                },
                                "additionalProperties": false
                              },
                              "reply_main_message_id": {
                                "type": "string",
                                "description": "The ID of the parent message in the chat thread.It's only available if the message is a reply to a main message in a thread."
                              },
                              "date_time": {
                                "type": "string",
                                "format": "date-time",
                                "description": "The date and time at which the message was sent, in yyyy-MM-dd'T'HH:mm:ss'Z' format."
                              },
                              "timestamp": {
                                "type": "integer",
                                "format": "int64",
                                "description": "The timestamp for when the message was sent."
                              }
                            }
                          },
                          "additionalProperties": false
                        }
                      },
                      "additionalProperties": false
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"team_chat.app_mention\",\n  \"event_ts\": 1696914490200,\n  \"payload\": {\n    \"account_id\": \"asgVcjZnWWRLWvv_GtyGuaxg\",\n    \"operator\": \"example@zoom.us\",\n    \"operator_member_id\": \"R4VM29Oj0fVM2hhEmSKVM2hhezJTezJTKVM2hezJT2hezJTSK\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"by_external_user\": true,\n    \"object\": {\n      \"message_id\": \"5DD614F4-DD19-4C39-8D03-93C4D0A0C839\",\n      \"type\": \"to_channel\",\n      \"channel_id\": \"8AmQ0WOoQ1O3a84yYFc2xQ\",\n      \"channel_name\": \"Team Chat\",\n      \"message\": \"hello\",\n      \"files\": [\n        {\n          \"file_id\": \"56bed9orRnOSEnBoX_HQvA\",\n          \"file_name\": \"Screenshot 2023-12-22 at 11.21.38 AM.png\",\n          \"file_size\": 23025,\n          \"OS_file_type\": \"PNG\",\n          \"file_message_type\": \"image\"\n        }\n      ],\n      \"rich_text\": [\n        {\n          \"start_position\": 0,\n          \"end_position\": 1,\n          \"format_type\": \"Bold\"\n        },\n        {\n          \"start_position\": 1,\n          \"end_position\": 2,\n          \"format_type\": \"BulletedList\"\n        },\n        {\n          \"start_position\": 2,\n          \"end_position\": 3,\n          \"format_type\": \"AddLink\",\n          \"format_attr\": \"https://www.example.com/templates/daily-stand-up\"\n        }\n      ],\n      \"reply_main_message_id\": \"5DD614F4-DD19-4C39-8D03-93C4D0A0C838\",\n      \"date_time\": \"2023-10-10T05:08:10Z\",\n      \"timestamp\": 1696914490157\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "interactive_message_select": {
      "post": {
        "tags": [
          "interactive_message_select"
        ],
        "operationId": "interactive_message_select",
        "requestBody": {
          "description": "# interactive_message_select\n\nAfter a user selects an option in a Chatbot [message with a Dropdown](https://marketplace.zoom.us/docs/guides/chatbots/customizing-messages/message-with-dropdown), the `interactive_message_select` event is invoked. The Chatbot sends a POST request associated with this event along with data on the selected option to the [Bot Endpoint URL](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-chatbot-app#set-features) specified in the app's dashboard.\n\nThe `selectedItems` array displays the value chosen by the user from the  `selected_items` array in the original message sent by the Chatbot.\n\n\n\n**Event type**: `interactive_message_select`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "event": {
                    "type": "string",
                    "description": "Name of the event."
                  },
                  "payload": {
                    "type": "object",
                    "properties": {
                      "accountId": {
                        "type": "string",
                        "description": "The account ID of the user sending the command."
                      },
                      "channelName": {
                        "type": "string",
                        "description": "Channel in which the message was sent. "
                      },
                      "messageId": {
                        "type": "string",
                        "description": "Unique ID of the message."
                      },
                      "original": {
                        "type": "object",
                        "description": "Original message sent to user.",
                        "properties": {
                          "head": {
                            "type": "object",
                            "properties": {
                              "sub_head": {
                                "type": "object",
                                "properties": {
                                  "text": {
                                    "type": "string"
                                  }
                                }
                              },
                              "text": {
                                "type": "string"
                              }
                            }
                          },
                          "body": {
                            "type": "array",
                            "items": {
                              "type": "object",
                              "properties": {
                                "select_items": {
                                  "type": "array",
                                  "items": {
                                    "type": "object",
                                    "properties": {
                                      "text": {
                                        "type": "string"
                                      },
                                      "value": {
                                        "type": "string"
                                      }
                                    }
                                  }
                                },
                                "text": {
                                  "type": "string"
                                },
                                "type": {
                                  "type": "string"
                                }
                              }
                            }
                          }
                        }
                      },
                      "robotJid": {
                        "type": "string",
                        "description": "Unique credential of the Chatbot."
                      },
                      "selectedItems": {
                        "type": "array",
                        "description": "Value of item selected by the user.",
                        "items": {
                          "type": "object",
                          "properties": {
                            "value": {
                              "type": "string"
                            }
                          }
                        }
                      },
                      "timestamp": {
                        "type": "integer",
                        "format": "date-time",
                        "description": "Time command message was sent."
                      },
                      "toJid": {
                        "type": "string",
                        "description": "JID of the channel or user the message was sent to."
                      },
                      "userId": {
                        "type": "string",
                        "description": "User sending the command."
                      },
                      "userJid": {
                        "type": "string",
                        "description": "JID of user who is sending the command."
                      },
                      "userName": {
                        "type": "string",
                        "description": "User's display name."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"interactive_message_select\",\n  \"event_ts\": 1560891284464,\n  \"payload\": {\n    \"accountId\": \"gVcjZnWWRLWvv_GtyGuaxg\",\n    \"channelName\": \"Jill Chill\",\n    \"messageId\": \"20190618205440879_sRHVqf4_aw1\",\n    \"original\": {\n      \"head\": {\n        \"sub_head\": {\n          \"text\": \"I am a sub header\"\n        },\n        \"text\": \"I am a header\"\n      },\n      \"body\": [\n        {\n          \"select_items\": [\n            {\n              \"text\": \"Lions\",\n              \"value\": \"lions\"\n            },\n            {\n              \"text\": \"Tigers\",\n              \"value\": \"tigers\"\n            },\n            {\n              \"text\": \"Bears\",\n              \"value\": \"bears\"\n            }\n          ],\n          \"text\": \"Animals\",\n          \"type\": \"select\"\n        }\n      ]\n    },\n    \"robotJid\": \"v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"selectedItems\": [\n      {\n        \"value\": \"lions\"\n      }\n    ],\n    \"timestamp\": 1560891284464,\n    \"toJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"userId\": \"KdYKjnimT4KPd8KKdQt9FQ\",\n    \"userJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com\",\n    \"userName\": \"Jill Chill\"\n  }\n}"
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "event": {
                      "type": "string",
                      "description": "Name of the event."
                    },
                    "payload": {
                      "type": "object",
                      "properties": {
                        "accountId": {
                          "type": "string",
                          "description": "The account ID of the user sending the command."
                        },
                        "channelName": {
                          "type": "string",
                          "description": "Channel in which the message was sent. "
                        },
                        "messageId": {
                          "type": "string",
                          "description": "Unique ID of the message."
                        },
                        "original": {
                          "type": "object",
                          "description": "Original message sent to user.",
                          "properties": {
                            "head": {
                              "type": "object",
                              "properties": {
                                "sub_head": {
                                  "type": "object",
                                  "properties": {
                                    "text": {
                                      "type": "string"
                                    }
                                  }
                                },
                                "text": {
                                  "type": "string"
                                }
                              }
                            },
                            "body": {
                              "type": "array",
                              "items": {
                                "type": "object",
                                "properties": {
                                  "select_items": {
                                    "type": "array",
                                    "items": {
                                      "type": "object",
                                      "properties": {
                                        "text": {
                                          "type": "string"
                                        },
                                        "value": {
                                          "type": "string"
                                        }
                                      }
                                    }
                                  },
                                  "text": {
                                    "type": "string"
                                  },
                                  "type": {
                                    "type": "string"
                                  }
                                }
                              }
                            }
                          }
                        },
                        "robotJid": {
                          "type": "string",
                          "description": "Unique credential of the Chatbot."
                        },
                        "selectedItems": {
                          "type": "array",
                          "description": "Value of item selected by the user.",
                          "items": {
                            "type": "object",
                            "properties": {
                              "value": {
                                "type": "string"
                              }
                            }
                          }
                        },
                        "timestamp": {
                          "type": "integer",
                          "format": "date-time",
                          "description": "Time command message was sent."
                        },
                        "toJid": {
                          "type": "string",
                          "description": "JID of the channel or user the message was sent to."
                        },
                        "userId": {
                          "type": "string",
                          "description": "User sending the command."
                        },
                        "userJid": {
                          "type": "string",
                          "description": "JID of user who is sending the command."
                        },
                        "userName": {
                          "type": "string",
                          "description": "User's display name."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"interactive_message_select\",\n  \"event_ts\": 1560891284464,\n  \"payload\": {\n    \"accountId\": \"gVcjZnWWRLWvv_GtyGuaxg\",\n    \"channelName\": \"Jill Chill\",\n    \"messageId\": \"20190618205440879_sRHVqf4_aw1\",\n    \"original\": {\n      \"head\": {\n        \"sub_head\": {\n          \"text\": \"I am a sub header\"\n        },\n        \"text\": \"I am a header\"\n      },\n      \"body\": [\n        {\n          \"select_items\": [\n            {\n              \"text\": \"Lions\",\n              \"value\": \"lions\"\n            },\n            {\n              \"text\": \"Tigers\",\n              \"value\": \"tigers\"\n            },\n            {\n              \"text\": \"Bears\",\n              \"value\": \"bears\"\n            }\n          ],\n          \"text\": \"Animals\",\n          \"type\": \"select\"\n        }\n      ]\n    },\n    \"robotJid\": \"v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"selectedItems\": [\n      {\n        \"value\": \"lions\"\n      }\n    ],\n    \"timestamp\": 1560891284464,\n    \"toJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\",\n    \"userId\": \"KdYKjnimT4KPd8KKdQt9FQ\",\n    \"userJid\": \"kdykjnimt4kpd8kkdqt9fq@example.com\",\n    \"userName\": \"Jill Chill\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
```


## Metadata

```json
{
  "params": {
    "slug": [
      "chatbot",
      "events"
    ]
  },
  "downloadPath": "/api-hub/chatbot/events/webhooks.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": false
  },
  "productMeta": {
    "specType": "adoc",
    "slug": "/docs/api/rest/reference/chatbot/events",
    "title": "Zoom API Events - Chatbot",
    "description": "",
    "keywords": "zoom, marketplace, chatbot, chat, bot, webhook, event,  interactive message, button, message, action",
    "zoapEndpoint": "webhookByProduct",
    "params": {
      "productName": "Chat Bot"
    },
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "productDisplayTitle": "Chatbot",
    "scalarSlug": "/docs/api/chatbot/events",
    "svgname": "_chat"
  },
  "frontmatter": {
    "title": "Chatbot Webhooks",
    "keywords": "zoom, marketplace, chatbot, chat, bot, webhook, event,  interactive message, button, message, action",
    "description": "The chatbot webhooks allow developers to receive events for [chatbot](https://developers.zoom.us/docs/team-chat-apps/) features.  \r\n\r\nNote regarding JSON payloads for guest users:\r\nBe aware that guest users (users not signed in to Zoom or who are members of a different account) are assigned temporary IDs that are valid only for the duration of that meeting or chat channel. Data for guest users in JSON payloads is associated with these temporary IDs.\r\n\r\n",
    "skipQualtrics": true
  },
  "isScalarApiRef": true
}
```

## Raw Next.js page data

```json
{
  "pageProps": {
    "params": {
      "slug": [
        "chatbot",
        "events"
      ]
    },
    "downloadPath": "/api-hub/chatbot/events/webhooks.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": false
    },
    "productMeta": {
      "specType": "adoc",
      "slug": "/docs/api/rest/reference/chatbot/events",
      "title": "Zoom API Events - Chatbot",
      "description": "",
      "keywords": "zoom, marketplace, chatbot, chat, bot, webhook, event,  interactive message, button, message, action",
      "zoapEndpoint": "webhookByProduct",
      "params": {
        "productName": "Chat Bot"
      },
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "productDisplayTitle": "Chatbot",
      "scalarSlug": "/docs/api/chatbot/events",
      "svgname": "_chat"
    },
    "spec": "{\"openapi\":\"3.1.0\",\"info\":{\"title\":\"Chatbot\",\"description\":\"The chatbot webhooks allow developers to receive events for [chatbot](https://developers.zoom.us/docs/team-chat-apps/) features.  \\r\\n\\r\\nNote regarding JSON payloads for guest users:\\r\\nBe aware that guest users (users not signed in to Zoom or who are members of a different account) are assigned temporary IDs that are valid only for the duration of that meeting or chat channel. Data for guest users in JSON payloads is associated with these temporary IDs.\\r\\n\\r\\n\",\"version\":\"1.0.0\"},\"paths\":{\"team_chat.link_shared\":{\"post\":{\"tags\":[\"team_chat\"],\"operationId\":\"team_chat.link_shared\",\"requestBody\":{\"description\":\"# team_chat.link_shared\\nThe **Chat message link shared** \u00a0event triggers when a user sends a chat message to a contact or a chat channel, and when the app is installed for the user. This message must contain the link. The link's domain is registered by the app in Marketplace. A POST request with a mention message is sent to the [Bot Endpoint URL](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-chatbot-app#set-features) specified in the app's dashboard.\\n## Prerequisites\\n* A Pro or a higher [plan](https://zoom.us/pricing).\\n\\n\\n**Event type**: `team_chat.link_shared`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"response_url\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\",\"enum\":[\"team_chat.link_shared\"]},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"response_url\":{\"type\":\"string\",\"description\":\"A callback uri for app to response preview result.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\",\"operator\",\"operator_member_id\",\"operator_id\",\"by_external_user\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who sent the app mention message.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user who sent the app mention message.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user who sent the app mention message.\"},\"operator_member_id\":{\"type\":\"string\",\"description\":\"The member ID of the user who sent the app mention message.\"},\"by_external_user\":{\"type\":\"boolean\",\"description\":\"Whether this action was performed by an external user.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the app mention message.\",\"required\":[\"type\",\"link\",\"trigger_id\",\"timestamp\",\"date_time\"],\"properties\":{\"message_id\":{\"type\":\"string\",\"description\":\"The message's ID. If the preview is provided in an unfurl for a posted message, rather than a link inside the message composer, we will simply ignore this property.\"},\"type\":{\"type\":\"string\",\"description\":\"The type of message: \\n* to_contact \u2014 The message was sent to a contact. \\n* to_channel \u2014 The message was sent to a chat channel.\",\"enum\":[\"to_contact\",\"to_channel\"]},\"channel_id\":{\"type\":\"string\",\"description\":\"The chat channel's ID.\"},\"channel_name\":{\"type\":\"string\",\"description\":\"The chat channel's name.\"},\"contact_id\":{\"type\":\"string\",\"description\":\"The message contact's user ID. If the user ID does not belong to the same account as the current user, this field will be empty.\"},\"contact_member_id\":{\"type\":\"string\",\"description\":\"The message contact's member ID.\"},\"trigger_id\":{\"type\":\"string\",\"description\":\"The unique identification of this webook event, app can use this trigger_id to invoke openAPI\"},\"link\":{\"type\":\"string\",\"description\":\"The link content in message.\"},\"reply_main_message_id\":{\"type\":\"string\",\"description\":\"The ID of the parent message in the chat thread.It's only available if the message is a reply to a main message in a thread.\"},\"date_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The date and time at which the message was sent, in yyyy-MM-dd'T'HH:mm:ss'Z' format.\"},\"timestamp\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp for when the message was sent.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"team_chat.link_shared\\\",\\n  \\\"event_ts\\\": 1696914490200,\\n  \\\"response_url\\\": \\\"https://api.zoom.us/v2/im/chat/users/exampleUserId/unfurls/exampleTriggerId\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"exampleAccountId\\\",\\n    \\\"operator_id\\\": \\\"exampleuserId\\\",\\n    \\\"operator\\\": \\\"example@zoom.us\\\",\\n    \\\"operator_member_id\\\": \\\"R4VM29Oj0fVM2hhEmSKVM2hhezJTezJTKVM2hezJT2hezJTSK\\\",\\n    \\\"by_external_user\\\": false,\\n    \\\"object\\\": {\\n      \\\"message_id\\\": \\\"5DD614F4-DD19-4C39-8D03-93C4D0A0C839\\\",\\n      \\\"reply_main_message_id\\\": \\\"5DD614F4-DD19-4C39-8D03-93C4D0A0C838\\\",\\n      \\\"type\\\": \\\"to_channel\\\",\\n      \\\"channel_id\\\": \\\"xxxxxxx\\\",\\n      \\\"channel_name\\\": \\\"Team Chat\\\",\\n      \\\"link\\\": \\\"https://sg.com/1234w\\\",\\n      \\\"trigger_id\\\": \\\"exampleTriggerId\\\",\\n      \\\"date_time\\\": \\\"2023-10-10T05:08:10Z\\\",\\n      \\\"timestamp\\\": 1696914490157\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"response_url\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\",\"enum\":[\"team_chat.link_shared\"]},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"response_url\":{\"type\":\"string\",\"description\":\"A callback uri for app to response preview result.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\",\"operator\",\"operator_member_id\",\"operator_id\",\"by_external_user\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who sent the app mention message.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user who sent the app mention message.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user who sent the app mention message.\"},\"operator_member_id\":{\"type\":\"string\",\"description\":\"The member ID of the user who sent the app mention message.\"},\"by_external_user\":{\"type\":\"boolean\",\"description\":\"Whether this action was performed by an external user.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the app mention message.\",\"required\":[\"type\",\"link\",\"trigger_id\",\"timestamp\",\"date_time\"],\"properties\":{\"message_id\":{\"type\":\"string\",\"description\":\"The message's ID. If the preview is provided in an unfurl for a posted message, rather than a link inside the message composer, we will simply ignore this property.\"},\"type\":{\"type\":\"string\",\"description\":\"The type of message: \\n* to_contact \u2014 The message was sent to a contact. \\n* to_channel \u2014 The message was sent to a chat channel.\",\"enum\":[\"to_contact\",\"to_channel\"]},\"channel_id\":{\"type\":\"string\",\"description\":\"The chat channel's ID.\"},\"channel_name\":{\"type\":\"string\",\"description\":\"The chat channel's name.\"},\"contact_id\":{\"type\":\"string\",\"description\":\"The message contact's user ID. If the user ID does not belong to the same account as the current user, this field will be empty.\"},\"contact_member_id\":{\"type\":\"string\",\"description\":\"The message contact's member ID.\"},\"trigger_id\":{\"type\":\"string\",\"description\":\"The unique identification of this webook event, app can use this trigger_id to invoke openAPI\"},\"link\":{\"type\":\"string\",\"description\":\"The link content in message.\"},\"reply_main_message_id\":{\"type\":\"string\",\"description\":\"The ID of the parent message in the chat thread.It's only available if the message is a reply to a main message in a thread.\"},\"date_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The date and time at which the message was sent, in yyyy-MM-dd'T'HH:mm:ss'Z' format.\"},\"timestamp\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp for when the message was sent.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"team_chat.link_shared\\\",\\n  \\\"event_ts\\\": 1696914490200,\\n  \\\"response_url\\\": \\\"https://api.zoom.us/v2/im/chat/users/exampleUserId/unfurls/exampleTriggerId\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"exampleAccountId\\\",\\n    \\\"operator_id\\\": \\\"exampleuserId\\\",\\n    \\\"operator\\\": \\\"example@zoom.us\\\",\\n    \\\"operator_member_id\\\": \\\"R4VM29Oj0fVM2hhEmSKVM2hhezJTezJTKVM2hezJT2hezJTSK\\\",\\n    \\\"by_external_user\\\": false,\\n    \\\"object\\\": {\\n      \\\"message_id\\\": \\\"5DD614F4-DD19-4C39-8D03-93C4D0A0C839\\\",\\n      \\\"reply_main_message_id\\\": \\\"5DD614F4-DD19-4C39-8D03-93C4D0A0C838\\\",\\n      \\\"type\\\": \\\"to_channel\\\",\\n      \\\"channel_id\\\": \\\"xxxxxxx\\\",\\n      \\\"channel_name\\\": \\\"Team Chat\\\",\\n      \\\"link\\\": \\\"https://sg.com/1234w\\\",\\n      \\\"trigger_id\\\": \\\"exampleTriggerId\\\",\\n      \\\"date_time\\\": \\\"2023-10-10T05:08:10Z\\\",\\n      \\\"timestamp\\\": 1696914490157\\n    }\\n  }\\n}\"}}}}}}}},\"interactive_message_fields_editable\":{\"post\":{\"tags\":[\"interactive_message_fields_editable\"],\"operationId\":\"interactive_message_fields_editable\",\"requestBody\":{\"description\":\"# interactive_message_fields_editable\\n\\nAfter a user edits the text of a Chatbot [message with Editable Text](https://marketplace.zoom.us/docs/guides/chatbots/customizing-messages/message-with-editable-text), the `interactive_message_editable` event is invoked and a POST request with data about the event is sent to the [Bot Endpoint URL](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-chatbot-app#set-features) specified in the app's dashboard.\\n\\nThe `editItem` object displays the changes in edited text with `origin` and `target` values.\\n\\n\\n\\n**Event type**: `interactive_message_fields_editable`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format.\"},\"callback_url\":{\"type\":\"string\",\"description\":\"Use this endpoint to send a one-time asynchronous response within 30 minutes of receiving the event. <br> The response payload should follow the same structure as the [Send Chatbot Messages](https://developers.zoom.us/docs/api/chatbot/#tag/chatbot-messages/POST/im/chat/messages) API with required property **content** and optional properties *reply_to*, *visible_to_user* and *is_markdown_support*. <br> Include the **callback_token** value from the event as **Bearer token in the Authorization header** of your HTTP POST request.\"},\"callback_token\":{\"type\":\"string\",\"description\":\"Use the generated token value with the **callback_url** value to send a one-time asynchronous response. The token only lasts for 30 mins. Include the **callback_token** as a **Bearer token in the Authorization header** of your HTTP request. \\n Example: \\n\\n \\ncurl --request POST \\\\\\n  --url '{callback_url}' \\\\\\n  --header 'authorization: Bearer {callback_token}' \\\\\\n  --header 'content-type: application/json'\\\\\\n  --data-raw '{\\\"content\\\":{\\\"head\\\":{\\\"text\\\":\\\"Response Header\\\"},\\\"body\\\":[{\\\"type\\\":\\\"message\\\",\\\"text\\\":\\\"This is an asynchronous chatbot response.\\\"}]}}'\"},\"payload\":{\"type\":\"object\",\"properties\":{\"accountId\":{\"type\":\"string\",\"description\":\"The account ID of the user sending the command.\"},\"channelName\":{\"type\":\"string\",\"description\":\"The channel in which the message was sent. \"},\"fieldEditItem\":{\"type\":\"object\",\"description\":\"The item that the user edits.\",\"properties\":{\"currentValue\":{\"type\":\"string\"},\"key\":{\"type\":\"string\"},\"newValue\":{\"type\":\"string\"}}},\"messageId\":{\"type\":\"string\",\"description\":\"The unique ID of the message.\"},\"original\":{\"type\":\"object\",\"description\":\"The original message sent to the user.\",\"properties\":{\"head\":{\"type\":\"object\",\"properties\":{\"sub_head\":{\"type\":\"object\",\"properties\":{\"text\":{\"type\":\"string\"}}},\"text\":{\"type\":\"string\"}}},\"body\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"type\":{\"type\":\"string\"},\"items\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"editable\":{\"type\":\"string\"},\"value\":{\"type\":\"string\"},\"key\":{\"type\":\"string\"}}}}}}}}},\"robotJid\":{\"type\":\"string\",\"description\":\"The unique credential of the Chatbot.\"},\"timestamp\":{\"type\":\"string\",\"description\":\"The time command message was sent.\"},\"toJid\":{\"type\":\"string\",\"description\":\"The JID of the channel or user the message was sent to.\"},\"userId\":{\"type\":\"string\",\"description\":\"The user who sends the command.\"},\"userJid\":{\"type\":\"string\",\"description\":\"The JID of user who is sending the command.\"},\"userName\":{\"type\":\"string\",\"description\":\"The user's display name.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"interactive_message_fields_editable\\\",\\n  \\\"event_ts\\\": 1560891222018,\\n  \\\"callback_url\\\": \\\"https://api.zoom.us/v2/im/chat/callback/bKFS3ybORiWbJx6VM6Qe1A\\\",\\n  \\\"callback_token\\\": \\\"eyJzdiI6IjAwMDAwMSIsInR5cCI6IkpXVCIsImFsZyI6IkVTMjU2In0.eyJhdWQiOiJib3QiLCJ1aWQiOiJWV2ZQa1NENVJvMk5xRWUyR1VMa2F3IiwiY2xpZW50SWQiOiJRQmZVaEFyUlNHVVFVclM2ZDFXVnciLCJpc3MiOiJ6b29tIiwiZXhwIjoxNzYwMzYzNzc5LCJpYXQiOjE3NjAzMzM3NzksImFpZCI6InQ4dURPZWtYUkx5b3ExR0M4UFBaTVEifQ.wBeNSNeYLQYJD1Lp0O0EnnfyCYNzF03tEwFin0-hRV7UfECLmHsW7NnQ9EEPGxOokDKnQb-pRPd_iBX4JKfg_Q\\\",\\n  \\\"payload\\\": {\\n    \\\"accountId\\\": \\\"gVcjZnWWRLWvv_GtyGuaxg\\\",\\n    \\\"channelName\\\": \\\"Jill Chill\\\",\\n    \\\"fieldEditItem\\\": {\\n      \\\"currentValue\\\": \\\"\\\",\\n      \\\"key\\\": \\\"Title\\\",\\n      \\\"newValue\\\": \\\"Software Engineer\\\"\\n    },\\n    \\\"messageId\\\": \\\"20190618205323327_EZjIqCQ_aw1\\\",\\n    \\\"original\\\": {\\n      \\\"head\\\": {\\n        \\\"sub_head\\\": {\\n          \\\"text\\\": \\\"I am a sub header\\\"\\n        },\\n        \\\"text\\\": \\\"I am a header\\\"\\n      },\\n      \\\"body\\\": [\\n        {\\n          \\\"type\\\": \\\"fields\\\",\\n          \\\"items\\\": [\\n            {\\n              \\\"editable\\\": true,\\n              \\\"value\\\": \\\" \\\",\\n              \\\"key\\\": \\\"Name\\\"\\n            },\\n            {\\n              \\\"editable\\\": true,\\n              \\\"value\\\": \\\" \\\",\\n              \\\"key\\\": \\\"Title\\\"\\n            },\\n            {\\n              \\\"editable\\\": false,\\n              \\\"value\\\": \\\"Example Company\\\",\\n              \\\"key\\\": \\\"Company\\\"\\n            }\\n          ]\\n        }\\n      ]\\n    },\\n    \\\"robotJid\\\": \\\"v1m0yn1imztuogsxjje8fdew@example.com\\\",\\n    \\\"timestamp\\\": 1560891222018,\\n    \\\"toJid\\\": \\\"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\\\",\\n    \\\"userId\\\": \\\"KdYKjnimT4KPd8KKdQt9FQ\\\",\\n    \\\"userJid\\\": \\\"kdykjnimt4kpd8kkdqt9fq@example.com\\\",\\n    \\\"userName\\\": \\\"Jill Chill\\\"\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format.\"},\"callback_url\":{\"type\":\"string\",\"description\":\"Use this endpoint to send a one-time asynchronous response within 30 minutes of receiving the event. <br> The response payload should follow the same structure as the [Send Chatbot Messages](https://developers.zoom.us/docs/api/chatbot/#tag/chatbot-messages/POST/im/chat/messages) API with required property **content** and optional properties *reply_to*, *visible_to_user* and *is_markdown_support*. <br> Include the **callback_token** value from the event as **Bearer token in the Authorization header** of your HTTP POST request.\"},\"callback_token\":{\"type\":\"string\",\"description\":\"Use the generated token value with the **callback_url** value to send a one-time asynchronous response. The token only lasts for 30 mins. Include the **callback_token** as a **Bearer token in the Authorization header** of your HTTP request. \\n Example: \\n\\n \\ncurl --request POST \\\\\\n  --url '{callback_url}' \\\\\\n  --header 'authorization: Bearer {callback_token}' \\\\\\n  --header 'content-type: application/json'\\\\\\n  --data-raw '{\\\"content\\\":{\\\"head\\\":{\\\"text\\\":\\\"Response Header\\\"},\\\"body\\\":[{\\\"type\\\":\\\"message\\\",\\\"text\\\":\\\"This is an asynchronous chatbot response.\\\"}]}}'\"},\"payload\":{\"type\":\"object\",\"properties\":{\"accountId\":{\"type\":\"string\",\"description\":\"The account ID of the user sending the command.\"},\"channelName\":{\"type\":\"string\",\"description\":\"The channel in which the message was sent. \"},\"fieldEditItem\":{\"type\":\"object\",\"description\":\"The item that the user edits.\",\"properties\":{\"currentValue\":{\"type\":\"string\"},\"key\":{\"type\":\"string\"},\"newValue\":{\"type\":\"string\"}}},\"messageId\":{\"type\":\"string\",\"description\":\"The unique ID of the message.\"},\"original\":{\"type\":\"object\",\"description\":\"The original message sent to the user.\",\"properties\":{\"head\":{\"type\":\"object\",\"properties\":{\"sub_head\":{\"type\":\"object\",\"properties\":{\"text\":{\"type\":\"string\"}}},\"text\":{\"type\":\"string\"}}},\"body\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"type\":{\"type\":\"string\"},\"items\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"editable\":{\"type\":\"string\"},\"value\":{\"type\":\"string\"},\"key\":{\"type\":\"string\"}}}}}}}}},\"robotJid\":{\"type\":\"string\",\"description\":\"The unique credential of the Chatbot.\"},\"timestamp\":{\"type\":\"string\",\"description\":\"The time command message was sent.\"},\"toJid\":{\"type\":\"string\",\"description\":\"The JID of the channel or user the message was sent to.\"},\"userId\":{\"type\":\"string\",\"description\":\"The user who sends the command.\"},\"userJid\":{\"type\":\"string\",\"description\":\"The JID of user who is sending the command.\"},\"userName\":{\"type\":\"string\",\"description\":\"The user's display name.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"interactive_message_fields_editable\\\",\\n  \\\"event_ts\\\": 1560891222018,\\n  \\\"callback_url\\\": \\\"https://api.zoom.us/v2/im/chat/callback/bKFS3ybORiWbJx6VM6Qe1A\\\",\\n  \\\"callback_token\\\": \\\"eyJzdiI6IjAwMDAwMSIsInR5cCI6IkpXVCIsImFsZyI6IkVTMjU2In0.eyJhdWQiOiJib3QiLCJ1aWQiOiJWV2ZQa1NENVJvMk5xRWUyR1VMa2F3IiwiY2xpZW50SWQiOiJRQmZVaEFyUlNHVVFVclM2ZDFXVnciLCJpc3MiOiJ6b29tIiwiZXhwIjoxNzYwMzYzNzc5LCJpYXQiOjE3NjAzMzM3NzksImFpZCI6InQ4dURPZWtYUkx5b3ExR0M4UFBaTVEifQ.wBeNSNeYLQYJD1Lp0O0EnnfyCYNzF03tEwFin0-hRV7UfECLmHsW7NnQ9EEPGxOokDKnQb-pRPd_iBX4JKfg_Q\\\",\\n  \\\"payload\\\": {\\n    \\\"accountId\\\": \\\"gVcjZnWWRLWvv_GtyGuaxg\\\",\\n    \\\"channelName\\\": \\\"Jill Chill\\\",\\n    \\\"fieldEditItem\\\": {\\n      \\\"currentValue\\\": \\\"\\\",\\n      \\\"key\\\": \\\"Title\\\",\\n      \\\"newValue\\\": \\\"Software Engineer\\\"\\n    },\\n    \\\"messageId\\\": \\\"20190618205323327_EZjIqCQ_aw1\\\",\\n    \\\"original\\\": {\\n      \\\"head\\\": {\\n        \\\"sub_head\\\": {\\n          \\\"text\\\": \\\"I am a sub header\\\"\\n        },\\n        \\\"text\\\": \\\"I am a header\\\"\\n      },\\n      \\\"body\\\": [\\n        {\\n          \\\"type\\\": \\\"fields\\\",\\n          \\\"items\\\": [\\n            {\\n              \\\"editable\\\": true,\\n              \\\"value\\\": \\\" \\\",\\n              \\\"key\\\": \\\"Name\\\"\\n            },\\n            {\\n              \\\"editable\\\": true,\\n              \\\"value\\\": \\\" \\\",\\n              \\\"key\\\": \\\"Title\\\"\\n            },\\n            {\\n              \\\"editable\\\": false,\\n              \\\"value\\\": \\\"Example Company\\\",\\n              \\\"key\\\": \\\"Company\\\"\\n            }\\n          ]\\n        }\\n      ]\\n    },\\n    \\\"robotJid\\\": \\\"v1m0yn1imztuogsxjje8fdew@example.com\\\",\\n    \\\"timestamp\\\": 1560891222018,\\n    \\\"toJid\\\": \\\"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\\\",\\n    \\\"userId\\\": \\\"KdYKjnimT4KPd8KKdQt9FQ\\\",\\n    \\\"userJid\\\": \\\"kdykjnimt4kpd8kkdqt9fq@example.com\\\",\\n    \\\"userName\\\": \\\"Jill Chill\\\"\\n  }\\n}\"}}}}}}}},\"bot_notification\":{\"post\":{\"tags\":[\"bot_notification\"],\"operationId\":\"bot_notification\",\"requestBody\":{\"description\":\"# bot_notification\\n\\nAfter a user enters a [User Command](https://marketplace.zoom.us/docs/guides/chatbots/sending-messages#user-commands), a slash command unique to the application, Chatbots send a POST request associated with the `bot_notification` event to the [Bot Endpoint URL](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-chatbot-app#set-features) with a JSON payload containing all text entered following the command. This payload contains information on the user and event including the account, channel, timestamp, and user name.\\n\\nThe `cmd` value reflects the text entered by the user after the slash command. The `userName` is the string value of the user's displayed name, and `channelName` is the string value of the channel in which the command was sent.\\n\\n\\n\\n**Event type**: `bot_notification`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"Name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"Timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"accountId\":{\"type\":\"string\",\"description\":\"The account ID of the user sending the command.\"},\"channelName\":{\"type\":\"string\",\"description\":\"Channel in which the message was sent. \"},\"cmd\":{\"type\":\"string\",\"description\":\"Slash command entered by the user.\"},\"name\":{\"type\":\"string\",\"description\":\"Chatbot Name.\"},\"robotJid\":{\"type\":\"string\",\"description\":\"Unique credential of the Chatbot.\"},\"timestamp\":{\"type\":\"integer\",\"format\":\"date-time\",\"description\":\"Time command message was sent.\"},\"toJid\":{\"type\":\"string\",\"description\":\"JID of the channel or user the message was sent to.\"},\"userId\":{\"type\":\"string\",\"description\":\"User sending the command.\"},\"userJid\":{\"type\":\"string\",\"description\":\"JID of user who is sending the command.\"},\"userName\":{\"type\":\"string\",\"description\":\"User's display name.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"bot_notification\\\",\\n  \\\"event_ts\\\": 1560796234686,\\n  \\\"payload\\\": {\\n      \\\"accountId\\\": \\\"asgVcjZnWWRLWvv_GtyGuaxg\\\",\\n      \\\"channelName\\\": \\\"Photos\\\",\\n      \\\"cmd\\\": \\\"island\\\",\\n      \\\"name\\\": \\\"Unsplash\\\",\\n      \\\"robotJid\\\": \\\"v10r4uxexurcasg-pwh8hyh7sg@example.com\\\",\\n      \\\"timestamp\\\": 1560796234686,\\n      \\\"toJid\\\": \\\"b1c841dc7b0b4as69287e6be05c7f93f25@conference.example.com\\\",\\n      \\\"userId\\\": \\\"KdYKjnimT4asKPd8KKdQt9FQ\\\",\\n      \\\"userJid\\\": \\\"kdykjnimtas4kpd8kkdqt9fq@example.com\\\",\\n      \\\"userName\\\": \\\"Jill Chill\\\"\\n      }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"Name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"Timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"accountId\":{\"type\":\"string\",\"description\":\"The account ID of the user sending the command.\"},\"channelName\":{\"type\":\"string\",\"description\":\"Channel in which the message was sent. \"},\"cmd\":{\"type\":\"string\",\"description\":\"Slash command entered by the user.\"},\"name\":{\"type\":\"string\",\"description\":\"Chatbot Name.\"},\"robotJid\":{\"type\":\"string\",\"description\":\"Unique credential of the Chatbot.\"},\"timestamp\":{\"type\":\"integer\",\"format\":\"date-time\",\"description\":\"Time command message was sent.\"},\"toJid\":{\"type\":\"string\",\"description\":\"JID of the channel or user the message was sent to.\"},\"userId\":{\"type\":\"string\",\"description\":\"User sending the command.\"},\"userJid\":{\"type\":\"string\",\"description\":\"JID of user who is sending the command.\"},\"userName\":{\"type\":\"string\",\"description\":\"User's display name.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"bot_notification\\\",\\n  \\\"event_ts\\\": 1560796234686,\\n  \\\"payload\\\": {\\n      \\\"accountId\\\": \\\"asgVcjZnWWRLWvv_GtyGuaxg\\\",\\n      \\\"channelName\\\": \\\"Photos\\\",\\n      \\\"cmd\\\": \\\"island\\\",\\n      \\\"name\\\": \\\"Unsplash\\\",\\n      \\\"robotJid\\\": \\\"v10r4uxexurcasg-pwh8hyh7sg@example.com\\\",\\n      \\\"timestamp\\\": 1560796234686,\\n      \\\"toJid\\\": \\\"b1c841dc7b0b4as69287e6be05c7f93f25@conference.example.com\\\",\\n      \\\"userId\\\": \\\"KdYKjnimT4asKPd8KKdQt9FQ\\\",\\n      \\\"userJid\\\": \\\"kdykjnimtas4kpd8kkdqt9fq@example.com\\\",\\n      \\\"userName\\\": \\\"Jill Chill\\\"\\n      }\\n}\"}}}}}}}},\"interactive_message_actions\":{\"post\":{\"tags\":[\"interactive_message_actions\"],\"operationId\":\"interactive_message_actions\",\"requestBody\":{\"description\":\"# interactive_message_actions\\n\\nAfter a user clicks a [button in a Chatbot message](https://marketplace.zoom.us/docs/guides/chatbots/customizing-messages/message-with-buttons), the `interactive_message_actions` event is invoked and a POST request with data on the selected option is sent to the [Bot Endpoint URL](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-chatbot-app#set-features) specified in the app's dashboard.\\n\\nThe `actionItem` object displays the value chosen by the user from the `items` array of the original message from the Chatbot.\\n\\n\\n\\n**Event type**: `interactive_message_actions`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format.\"},\"callback_url\":{\"type\":\"string\",\"description\":\"Use this endpoint to send a one-time asynchronous response within 30 minutes of receiving the event. <br> The response payload should follow the same structure as the [Send Chatbot Messages](https://developers.zoom.us/docs/api/chatbot/#tag/chatbot-messages/POST/im/chat/messages) API with required property **content** and optional properties *reply_to*, *visible_to_user* and *is_markdown_support*. <br> Include the **callback_token** value from the event as **Bearer token in the Authorization header** of your HTTP POST request.\"},\"callback_token\":{\"type\":\"string\",\"description\":\"Use the generated token value with the **callback_url** value to send a one-time asynchronous response. The token only lasts for 30 mins. Include the **callback_token** as a **Bearer token in the Authorization header** of your HTTP request. \\n Example: \\n\\n \\ncurl --request POST \\\\\\n  --url '{callback_url}' \\\\\\n  --header 'authorization: Bearer {callback_token}' \\\\\\n  --header 'content-type: application/json'\\\\\\n  --data-raw '{\\\"content\\\":{\\\"head\\\":{\\\"text\\\":\\\"Response Header\\\"},\\\"body\\\":[{\\\"type\\\":\\\"message\\\",\\\"text\\\":\\\"This is an asynchronous chatbot response.\\\"}]}}'\"},\"payload\":{\"type\":\"object\",\"properties\":{\"accountId\":{\"type\":\"string\",\"description\":\"The account ID of the user sending the command.\"},\"actionItem\":{\"type\":\"object\",\"description\":\"Action taken by the user.\",\"properties\":{\"text\":{\"type\":\"string\"},\"value\":{\"type\":\"string\"}}},\"channelName\":{\"type\":\"string\",\"description\":\"Channel in which the message was sent.\"},\"messageId\":{\"type\":\"string\",\"description\":\"Unique ID of the message.\"},\"original\":{\"type\":\"object\",\"description\":\"Original message sent to the user.\",\"properties\":{\"head\":{\"type\":\"object\",\"properties\":{\"sub_head\":{\"type\":\"object\",\"properties\":{\"text\":{\"type\":\"string\"}}},\"text\":{\"type\":\"string\"}}},\"body\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"type\":{\"type\":\"string\"},\"items\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"style\":{\"type\":\"string\"},\"text\":{\"type\":\"string\"},\"value\":{\"type\":\"string\"}}}}}}}}},\"robotJid\":{\"type\":\"string\",\"description\":\"Unique credential of the Chatbot.\"},\"timestamp\":{\"type\":\"integer\",\"description\":\"Time command message was sent.\"},\"toJid\":{\"type\":\"string\",\"description\":\"JID of the channel or user the message was sent to.\"},\"userId\":{\"type\":\"string\",\"description\":\"User sending the command.\"},\"userJid\":{\"type\":\"string\",\"description\":\"JID of user who is sending the command.\"},\"userName\":{\"type\":\"string\",\"description\":\"User's display name.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"interactive_message_actions\\\",\\n  \\\"event_ts\\\": 1560891332219,\\n  \\\"callback_url\\\": \\\"https://api.zoom.us/v2/im/chat/callback/bKFS3ybORiWbJx6VM6Qe1A\\\",\\n  \\\"callback_token\\\": \\\"eyJzdiI6IjAwMDAwMSIsInR5cCI6IkpXVCIsImFsZyI6IkVTMjU2In0.eyJhdWQiOiJib3QiLCJ1aWQiOiJWV2ZQa1NENVJvMk5xRWUyR1VMa2F3IiwiY2xpZW50SWQiOiJRQmZVaEFyUlNHVVFVclM2ZDFXVnciLCJpc3MiOiJ6b29tIiwiZXhwIjoxNzYwMzYzNzc5LCJpYXQiOjE3NjAzMzM3NzksImFpZCI6InQ4dURPZWtYUkx5b3ExR0M4UFBaTVEifQ.wBeNSNeYLQYJD1Lp0O0EnnfyCYNzF03tEwFin0-hRV7UfECLmHsW7NnQ9EEPGxOokDKnQb-pRPd_iBX4JKfg_Q\\\",\\n  \\\"payload\\\": {\\n    \\\"accountId\\\": \\\"gVcjZnWWRLWvv_GtyGuaxg\\\",\\n    \\\"actionItem\\\": {\\n      \\\"text\\\": \\\"Add\\\",\\n      \\\"value\\\": \\\"add\\\"\\n    },\\n    \\\"channelName\\\": \\\"Jill Chill\\\",\\n    \\\"messageId\\\": \\\"20190618205516496_ynICmz8_aw1\\\",\\n    \\\"original\\\": {\\n      \\\"head\\\": {\\n        \\\"sub_head\\\": {\\n          \\\"text\\\": \\\"I am a sub header\\\"\\n        },\\n        \\\"text\\\": \\\"I am a header\\\"\\n      },\\n      \\\"body\\\": [\\n        {\\n          \\\"type\\\": \\\"actions\\\",\\n          \\\"items\\\": [\\n            {\\n              \\\"style\\\": \\\"Primary\\\",\\n              \\\"text\\\": \\\"Add\\\",\\n              \\\"value\\\": \\\"add\\\"\\n            },\\n            {\\n              \\\"style\\\": \\\"Default\\\",\\n              \\\"text\\\": \\\"Update\\\",\\n              \\\"value\\\": \\\"update\\\"\\n            },\\n            {\\n              \\\"style\\\": \\\"Danger\\\",\\n              \\\"text\\\": \\\"Delete\\\",\\n              \\\"value\\\": \\\"delete\\\"\\n            },\\n            {\\n              \\\"style\\\": \\\"Disabled\\\",\\n              \\\"text\\\": \\\"Disabled\\\",\\n              \\\"value\\\": \\\"disable\\\"\\n            }\\n          ]\\n        }\\n      ]\\n    },\\n    \\\"robotJid\\\": \\\"v1m0yn1imztuogsxjje8fdew@example.com\\\",\\n    \\\"timestamp\\\": 1560891332219,\\n    \\\"toJid\\\": \\\"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\\\",\\n    \\\"userId\\\": \\\"KdYKjnimT4KPd8KKdQt9FQ\\\",\\n    \\\"userJid\\\": \\\"kdykjnimt4kpd8kkdqt9fq@example.com\\\",\\n    \\\"userName\\\": \\\"Jill Chill\\\"\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format.\"},\"callback_url\":{\"type\":\"string\",\"description\":\"Use this endpoint to send a one-time asynchronous response within 30 minutes of receiving the event. <br> The response payload should follow the same structure as the [Send Chatbot Messages](https://developers.zoom.us/docs/api/chatbot/#tag/chatbot-messages/POST/im/chat/messages) API with required property **content** and optional properties *reply_to*, *visible_to_user* and *is_markdown_support*. <br> Include the **callback_token** value from the event as **Bearer token in the Authorization header** of your HTTP POST request.\"},\"callback_token\":{\"type\":\"string\",\"description\":\"Use the generated token value with the **callback_url** value to send a one-time asynchronous response. The token only lasts for 30 mins. Include the **callback_token** as a **Bearer token in the Authorization header** of your HTTP request. \\n Example: \\n\\n \\ncurl --request POST \\\\\\n  --url '{callback_url}' \\\\\\n  --header 'authorization: Bearer {callback_token}' \\\\\\n  --header 'content-type: application/json'\\\\\\n  --data-raw '{\\\"content\\\":{\\\"head\\\":{\\\"text\\\":\\\"Response Header\\\"},\\\"body\\\":[{\\\"type\\\":\\\"message\\\",\\\"text\\\":\\\"This is an asynchronous chatbot response.\\\"}]}}'\"},\"payload\":{\"type\":\"object\",\"properties\":{\"accountId\":{\"type\":\"string\",\"description\":\"The account ID of the user sending the command.\"},\"actionItem\":{\"type\":\"object\",\"description\":\"Action taken by the user.\",\"properties\":{\"text\":{\"type\":\"string\"},\"value\":{\"type\":\"string\"}}},\"channelName\":{\"type\":\"string\",\"description\":\"Channel in which the message was sent.\"},\"messageId\":{\"type\":\"string\",\"description\":\"Unique ID of the message.\"},\"original\":{\"type\":\"object\",\"description\":\"Original message sent to the user.\",\"properties\":{\"head\":{\"type\":\"object\",\"properties\":{\"sub_head\":{\"type\":\"object\",\"properties\":{\"text\":{\"type\":\"string\"}}},\"text\":{\"type\":\"string\"}}},\"body\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"type\":{\"type\":\"string\"},\"items\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"style\":{\"type\":\"string\"},\"text\":{\"type\":\"string\"},\"value\":{\"type\":\"string\"}}}}}}}}},\"robotJid\":{\"type\":\"string\",\"description\":\"Unique credential of the Chatbot.\"},\"timestamp\":{\"type\":\"integer\",\"description\":\"Time command message was sent.\"},\"toJid\":{\"type\":\"string\",\"description\":\"JID of the channel or user the message was sent to.\"},\"userId\":{\"type\":\"string\",\"description\":\"User sending the command.\"},\"userJid\":{\"type\":\"string\",\"description\":\"JID of user who is sending the command.\"},\"userName\":{\"type\":\"string\",\"description\":\"User's display name.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"interactive_message_actions\\\",\\n  \\\"event_ts\\\": 1560891332219,\\n  \\\"callback_url\\\": \\\"https://api.zoom.us/v2/im/chat/callback/bKFS3ybORiWbJx6VM6Qe1A\\\",\\n  \\\"callback_token\\\": \\\"eyJzdiI6IjAwMDAwMSIsInR5cCI6IkpXVCIsImFsZyI6IkVTMjU2In0.eyJhdWQiOiJib3QiLCJ1aWQiOiJWV2ZQa1NENVJvMk5xRWUyR1VMa2F3IiwiY2xpZW50SWQiOiJRQmZVaEFyUlNHVVFVclM2ZDFXVnciLCJpc3MiOiJ6b29tIiwiZXhwIjoxNzYwMzYzNzc5LCJpYXQiOjE3NjAzMzM3NzksImFpZCI6InQ4dURPZWtYUkx5b3ExR0M4UFBaTVEifQ.wBeNSNeYLQYJD1Lp0O0EnnfyCYNzF03tEwFin0-hRV7UfECLmHsW7NnQ9EEPGxOokDKnQb-pRPd_iBX4JKfg_Q\\\",\\n  \\\"payload\\\": {\\n    \\\"accountId\\\": \\\"gVcjZnWWRLWvv_GtyGuaxg\\\",\\n    \\\"actionItem\\\": {\\n      \\\"text\\\": \\\"Add\\\",\\n      \\\"value\\\": \\\"add\\\"\\n    },\\n    \\\"channelName\\\": \\\"Jill Chill\\\",\\n    \\\"messageId\\\": \\\"20190618205516496_ynICmz8_aw1\\\",\\n    \\\"original\\\": {\\n      \\\"head\\\": {\\n        \\\"sub_head\\\": {\\n          \\\"text\\\": \\\"I am a sub header\\\"\\n        },\\n        \\\"text\\\": \\\"I am a header\\\"\\n      },\\n      \\\"body\\\": [\\n        {\\n          \\\"type\\\": \\\"actions\\\",\\n          \\\"items\\\": [\\n            {\\n              \\\"style\\\": \\\"Primary\\\",\\n              \\\"text\\\": \\\"Add\\\",\\n              \\\"value\\\": \\\"add\\\"\\n            },\\n            {\\n              \\\"style\\\": \\\"Default\\\",\\n              \\\"text\\\": \\\"Update\\\",\\n              \\\"value\\\": \\\"update\\\"\\n            },\\n            {\\n              \\\"style\\\": \\\"Danger\\\",\\n              \\\"text\\\": \\\"Delete\\\",\\n              \\\"value\\\": \\\"delete\\\"\\n            },\\n            {\\n              \\\"style\\\": \\\"Disabled\\\",\\n              \\\"text\\\": \\\"Disabled\\\",\\n              \\\"value\\\": \\\"disable\\\"\\n            }\\n          ]\\n        }\\n      ]\\n    },\\n    \\\"robotJid\\\": \\\"v1m0yn1imztuogsxjje8fdew@example.com\\\",\\n    \\\"timestamp\\\": 1560891332219,\\n    \\\"toJid\\\": \\\"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\\\",\\n    \\\"userId\\\": \\\"KdYKjnimT4KPd8KKdQt9FQ\\\",\\n    \\\"userJid\\\": \\\"kdykjnimt4kpd8kkdqt9fq@example.com\\\",\\n    \\\"userName\\\": \\\"Jill Chill\\\"\\n  }\\n}\"}}}}}}}},\"interactive_message_editable\":{\"post\":{\"tags\":[\"interactive_message_editable\"],\"operationId\":\"interactive_message_editable\",\"requestBody\":{\"description\":\"# interactive_message_editable\\n\\nAfter a user edits the text of a Chatbot [message with Editable Text](https://marketplace.zoom.us/docs/guides/chatbots/customizing-messages/message-with-editable-text), the `interactive_message_editable` event is invoked and a POST request with data about the event is sent to the [Bot Endpoint URL](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-chatbot-app#set-features) specfied in the app's dashboard.\\n\\nThe `editItem` object displays the changes in edited text with `origin` and `target` values.\\n\\n\\n\\n**Event type**: `interactive_message_editable`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format.\"},\"callback_url\":{\"type\":\"string\",\"description\":\"Use this endpoint to send a one-time asynchronous response within 30 minutes of receiving the event. <br> The response payload should follow the same structure as the [Send Chatbot Messages](https://developers.zoom.us/docs/api/chatbot/#tag/chatbot-messages/POST/im/chat/messages) API with required property **content** and optional properties *reply_to*, *visible_to_user* and *is_markdown_support*. <br> Include the **callback_token** value from the event as **Bearer token in the Authorization header** of your HTTP POST request.\"},\"callback_token\":{\"type\":\"string\",\"description\":\"Use the generated token value with the **callback_url** value to send a one-time asynchronous response. The token only lasts for 30 mins. Include the **callback_token** as a **Bearer token in the Authorization header** of your HTTP request. \\n Example: \\n\\n \\ncurl --request POST \\\\\\n  --url '{callback_url}' \\\\\\n  --header 'authorization: Bearer {callback_token}' \\\\\\n  --header 'content-type: application/json'\\\\\\n  --data-raw '{\\\"content\\\":{\\\"head\\\":{\\\"text\\\":\\\"Response Header\\\"},\\\"body\\\":[{\\\"type\\\":\\\"message\\\",\\\"text\\\":\\\"This is an asynchronous chatbot response.\\\"}]}}'\"},\"payload\":{\"type\":\"object\",\"properties\":{\"accountId\":{\"type\":\"string\",\"description\":\"The account ID of the user editing the message.\"},\"channelName\":{\"type\":\"string\",\"description\":\"The channel in which the message was edited.\"},\"editItem\":{\"type\":\"object\",\"description\":\"The object containing original and edited value.\",\"properties\":{\"origin\":{\"type\":\"string\"},\"target\":{\"type\":\"string\"}}},\"messageId\":{\"type\":\"string\",\"description\":\"The unique ID of the message.\"},\"original\":{\"type\":\"object\",\"description\":\"The original message sent to the user.\",\"properties\":{\"head\":{\"type\":\"object\",\"properties\":{\"sub_head\":{\"type\":\"object\",\"properties\":{\"text\":{\"type\":\"string\"}}},\"text\":{\"type\":\"string\"}}},\"body\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"\":{\"type\":\"object\",\"properties\":{\"editable\":{\"type\":\"boolean\"},\"text\":{\"type\":\"string\"},\"type\":{\"type\":\"string\"}}}}}}}},\"robotJid\":{\"type\":\"string\",\"description\":\"The unique credential of the Chatbot.\"},\"timestamp\":{\"type\":\"integer\",\"format\":\"date-time\",\"description\":\"The time command message was sent.\"},\"toJid\":{\"type\":\"string\",\"description\":\"The JID of the channel or user the message was sent to.\"},\"userId\":{\"type\":\"string\",\"description\":\"The user who sends the command.\"},\"userJid\":{\"type\":\"string\",\"description\":\"The JID of user who is sending the command.\"},\"userName\":{\"type\":\"string\",\"description\":\"The user's display name.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"interactive_message_editable\\\",\\n  \\\"event_ts\\\": 1560891135776,\\n  \\\"callback_url\\\": \\\"https://api.zoom.us/v2/im/chat/callback/bKFS3ybORiWbJx6VM6Qe1A\\\",\\n  \\\"callback_token\\\": \\\"eyJzdiI6IjAwMDAwMSIsInR5cCI6IkpXVCIsImFsZyI6IkVTMjU2In0.eyJhdWQiOiJib3QiLCJ1aWQiOiJWV2ZQa1NENVJvMk5xRWUyR1VMa2F3IiwiY2xpZW50SWQiOiJRQmZVaEFyUlNHVVFVclM2ZDFXVnciLCJpc3MiOiJ6b29tIiwiZXhwIjoxNzYwMzYzNzc5LCJpYXQiOjE3NjAzMzM3NzksImFpZCI6InQ4dURPZWtYUkx5b3ExR0M4UFBaTVEifQ.wBeNSNeYLQYJD1Lp0O0EnnfyCYNzF03tEwFin0-hRV7UfECLmHsW7NnQ9EEPGxOokDKnQb-pRPd_iBX4JKfg_Q\\\",\\n  \\\"payload\\\": {\\n    \\\"accountId\\\": \\\"gVcjZnWWRLWvv_GtyGuaxg\\\",\\n    \\\"channelName\\\": \\\"Jill Chill\\\",\\n    \\\"editItem\\\": {\\n      \\\"origin\\\": \\\"I am a message with editable text\\\",\\n      \\\"target\\\": \\\"I am a message with editable text that has been edited\\\"\\n    },\\n    \\\"messageId\\\": \\\"20190618205205291_tI1CUa6_aw1\\\",\\n    \\\"original\\\": {\\n      \\\"head\\\": {\\n        \\\"sub_head\\\": {\\n          \\\"text\\\": \\\"I am a sub header\\\"\\n        },\\n        \\\"text\\\": \\\"I am a header\\\"\\n      },\\n      \\\"body\\\": [\\n        {\\n          \\\"editable\\\": true,\\n          \\\"text\\\": \\\"I am a message with editable text\\\",\\n          \\\"type\\\": \\\"message\\\"\\n        }\\n      ]\\n    },\\n    \\\"robotJid\\\": \\\"v1m0yn1imztuogsxjje8fdew@example.com\\\",\\n    \\\"timestamp\\\": 1560891135776,\\n    \\\"toJid\\\": \\\"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\\\",\\n    \\\"userId\\\": \\\"KdYKjnimT4KPd8KKdQt9FQ\\\",\\n    \\\"userJid\\\": \\\"kdykjnimt4kpd8kkdqt9fq@example.com\\\",\\n    \\\"userName\\\": \\\"Jill Chill\\\"\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp (in milliseconds since epcoch) at which the event occured. The value of this field is returned in long(int64) format.\"},\"callback_url\":{\"type\":\"string\",\"description\":\"Use this endpoint to send a one-time asynchronous response within 30 minutes of receiving the event. <br> The response payload should follow the same structure as the [Send Chatbot Messages](https://developers.zoom.us/docs/api/chatbot/#tag/chatbot-messages/POST/im/chat/messages) API with required property **content** and optional properties *reply_to*, *visible_to_user* and *is_markdown_support*. <br> Include the **callback_token** value from the event as **Bearer token in the Authorization header** of your HTTP POST request.\"},\"callback_token\":{\"type\":\"string\",\"description\":\"Use the generated token value with the **callback_url** value to send a one-time asynchronous response. The token only lasts for 30 mins. Include the **callback_token** as a **Bearer token in the Authorization header** of your HTTP request. \\n Example: \\n\\n \\ncurl --request POST \\\\\\n  --url '{callback_url}' \\\\\\n  --header 'authorization: Bearer {callback_token}' \\\\\\n  --header 'content-type: application/json'\\\\\\n  --data-raw '{\\\"content\\\":{\\\"head\\\":{\\\"text\\\":\\\"Response Header\\\"},\\\"body\\\":[{\\\"type\\\":\\\"message\\\",\\\"text\\\":\\\"This is an asynchronous chatbot response.\\\"}]}}'\"},\"payload\":{\"type\":\"object\",\"properties\":{\"accountId\":{\"type\":\"string\",\"description\":\"The account ID of the user editing the message.\"},\"channelName\":{\"type\":\"string\",\"description\":\"The channel in which the message was edited.\"},\"editItem\":{\"type\":\"object\",\"description\":\"The object containing original and edited value.\",\"properties\":{\"origin\":{\"type\":\"string\"},\"target\":{\"type\":\"string\"}}},\"messageId\":{\"type\":\"string\",\"description\":\"The unique ID of the message.\"},\"original\":{\"type\":\"object\",\"description\":\"The original message sent to the user.\",\"properties\":{\"head\":{\"type\":\"object\",\"properties\":{\"sub_head\":{\"type\":\"object\",\"properties\":{\"text\":{\"type\":\"string\"}}},\"text\":{\"type\":\"string\"}}},\"body\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"\":{\"type\":\"object\",\"properties\":{\"editable\":{\"type\":\"boolean\"},\"text\":{\"type\":\"string\"},\"type\":{\"type\":\"string\"}}}}}}}},\"robotJid\":{\"type\":\"string\",\"description\":\"The unique credential of the Chatbot.\"},\"timestamp\":{\"type\":\"integer\",\"format\":\"date-time\",\"description\":\"The time command message was sent.\"},\"toJid\":{\"type\":\"string\",\"description\":\"The JID of the channel or user the message was sent to.\"},\"userId\":{\"type\":\"string\",\"description\":\"The user who sends the command.\"},\"userJid\":{\"type\":\"string\",\"description\":\"The JID of user who is sending the command.\"},\"userName\":{\"type\":\"string\",\"description\":\"The user's display name.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"interactive_message_editable\\\",\\n  \\\"event_ts\\\": 1560891135776,\\n  \\\"callback_url\\\": \\\"https://api.zoom.us/v2/im/chat/callback/bKFS3ybORiWbJx6VM6Qe1A\\\",\\n  \\\"callback_token\\\": \\\"eyJzdiI6IjAwMDAwMSIsInR5cCI6IkpXVCIsImFsZyI6IkVTMjU2In0.eyJhdWQiOiJib3QiLCJ1aWQiOiJWV2ZQa1NENVJvMk5xRWUyR1VMa2F3IiwiY2xpZW50SWQiOiJRQmZVaEFyUlNHVVFVclM2ZDFXVnciLCJpc3MiOiJ6b29tIiwiZXhwIjoxNzYwMzYzNzc5LCJpYXQiOjE3NjAzMzM3NzksImFpZCI6InQ4dURPZWtYUkx5b3ExR0M4UFBaTVEifQ.wBeNSNeYLQYJD1Lp0O0EnnfyCYNzF03tEwFin0-hRV7UfECLmHsW7NnQ9EEPGxOokDKnQb-pRPd_iBX4JKfg_Q\\\",\\n  \\\"payload\\\": {\\n    \\\"accountId\\\": \\\"gVcjZnWWRLWvv_GtyGuaxg\\\",\\n    \\\"channelName\\\": \\\"Jill Chill\\\",\\n    \\\"editItem\\\": {\\n      \\\"origin\\\": \\\"I am a message with editable text\\\",\\n      \\\"target\\\": \\\"I am a message with editable text that has been edited\\\"\\n    },\\n    \\\"messageId\\\": \\\"20190618205205291_tI1CUa6_aw1\\\",\\n    \\\"original\\\": {\\n      \\\"head\\\": {\\n        \\\"sub_head\\\": {\\n          \\\"text\\\": \\\"I am a sub header\\\"\\n        },\\n        \\\"text\\\": \\\"I am a header\\\"\\n      },\\n      \\\"body\\\": [\\n        {\\n          \\\"editable\\\": true,\\n          \\\"text\\\": \\\"I am a message with editable text\\\",\\n          \\\"type\\\": \\\"message\\\"\\n        }\\n      ]\\n    },\\n    \\\"robotJid\\\": \\\"v1m0yn1imztuogsxjje8fdew@example.com\\\",\\n    \\\"timestamp\\\": 1560891135776,\\n    \\\"toJid\\\": \\\"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\\\",\\n    \\\"userId\\\": \\\"KdYKjnimT4KPd8KKdQt9FQ\\\",\\n    \\\"userJid\\\": \\\"kdykjnimt4kpd8kkdqt9fq@example.com\\\",\\n    \\\"userName\\\": \\\"Jill Chill\\\"\\n  }\\n}\"}}}}}}}},\"bot_installed\":{\"post\":{\"tags\":[\"bot_installed\"],\"operationId\":\"bot_installed\",\"requestBody\":{\"description\":\"# bot_installed\\nTo configure a dynamic welcome message:\\n1. Go to your Chatbot's settings and navigate to the **Features** page.\\n2. Check **Your App** for the welcome message.\\n3. Save your settings.\\n\\nAfter your Chatbot is authorized, Zoom sends an HTTP POST request to your respective Bot Endpoint URL with the following request body:\\n\\n**Event type**: `bot_installed`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"Name of the event.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"accountId\":{\"type\":\"string\",\"description\":\"The account ID of the user sending the command.\"},\"robotJid\":{\"type\":\"string\",\"description\":\"Unique credential of the Chatbot.\"},\"timestamp\":{\"type\":\"integer\",\"format\":\"date-time\",\"description\":\"Time command message was sent.\"},\"userId\":{\"type\":\"string\",\"description\":\"User sending the command.\"},\"userJid\":{\"type\":\"string\",\"description\":\"JID of user who is sending the command.\"},\"userName\":{\"type\":\"string\",\"description\":\"User's display name.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"bot_installed\\\",\\n  \\\"payload\\\": {\\n    \\\"accountId\\\": \\\"CDSCdcsdv\\\",\\n    \\\"robotJid\\\": \\\"csvfdvfd@xmpp.zoom.us\\\",\\n    \\\"timestamp\\\": 1542796725586,\\n    \\\"userId\\\": \\\"fvdfvfdvfd\\\",\\n    \\\"userJid\\\": \\\"fvdfvfdvfd@xmpp.zoom.us\\\",\\n    \\\"userName\\\": \\\"UserName\\\"\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"Name of the event.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"accountId\":{\"type\":\"string\",\"description\":\"The account ID of the user sending the command.\"},\"robotJid\":{\"type\":\"string\",\"description\":\"Unique credential of the Chatbot.\"},\"timestamp\":{\"type\":\"integer\",\"format\":\"date-time\",\"description\":\"Time command message was sent.\"},\"userId\":{\"type\":\"string\",\"description\":\"User sending the command.\"},\"userJid\":{\"type\":\"string\",\"description\":\"JID of user who is sending the command.\"},\"userName\":{\"type\":\"string\",\"description\":\"User's display name.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"bot_installed\\\",\\n  \\\"payload\\\": {\\n    \\\"accountId\\\": \\\"CDSCdcsdv\\\",\\n    \\\"robotJid\\\": \\\"csvfdvfd@xmpp.zoom.us\\\",\\n    \\\"timestamp\\\": 1542796725586,\\n    \\\"userId\\\": \\\"fvdfvfdvfd\\\",\\n    \\\"userJid\\\": \\\"fvdfvfdvfd@xmpp.zoom.us\\\",\\n    \\\"userName\\\": \\\"UserName\\\"\\n  }\\n}\"}}}}}}}},\"team_chat.app_mention\":{\"post\":{\"tags\":[\"team_chat\"],\"operationId\":\"team_chat.app_mention\",\"requestBody\":{\"description\":\"# team_chat.app_mention\\nThis event triggers when a user mentions a specific app in a chat channel. A POST request with the mention message is sent to the [Bot Endpoint URL](https://developers.zoom.us/docs/team-chat-apps/team-chat-subscriptions/) specified in the app's dashboard.\\n## Prerequisites\\n* A Pro or a higher [plan](https://zoom.us/pricing).\\n\\n**Event type**: `team_chat.app_mention`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\",\"enum\":[\"team_chat.app_mention\"]},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\",\"operator_member_id\",\"by_external_user\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who sent the app mention message. Will be empty when by_external_user is true\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user who sent the app mention message. Will be empty when by_external_user is true\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user who sent the app mention message. Will be empty when by_external_user is true\"},\"operator_member_id\":{\"type\":\"string\",\"description\":\"The member ID of the user who sent the app mention message.\"},\"by_external_user\":{\"type\":\"boolean\",\"description\":\"Whether this action was performed by an external user.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the app mention message.\",\"required\":[\"message_id\",\"type\",\"message\",\"timestamp\",\"date_time\"],\"properties\":{\"message_id\":{\"type\":\"string\",\"description\":\"The message's ID.\"},\"type\":{\"type\":\"string\",\"description\":\"The type of message: \\n* to_channel \u2014 The message was sent to a chat channel.\",\"enum\":[\"to_channel\"]},\"channel_id\":{\"type\":\"string\",\"description\":\"The chat channel's ID.\"},\"channel_name\":{\"type\":\"string\",\"description\":\"The chat channel's name.\"},\"message\":{\"type\":\"string\",\"description\":\"The sent message's content.\"},\"files\":{\"type\":\"array\",\"description\":\"The attachments file info in sent message's content.\",\"items\":{\"type\":\"object\",\"required\":[\"OS_file_type\",\"file_id\",\"file_message_type\",\"file_name\",\"file_size\"],\"properties\":{\"OS_file_type\":{\"type\":\"string\",\"description\":\"The attachments file extension type\"},\"file_id\":{\"type\":\"string\",\"description\":\"The attachments file's universally unique identifier (UUID), in base64-encoded format.\"},\"file_message_type\":{\"type\":\"string\",\"description\":\"The attachments file message type\",\"enum\":[\"file\",\"image\",\"audio\",\"audio v2\",\"code snippet\",\"screen shot\"]},\"file_name\":{\"type\":\"string\",\"description\":\"The attachments file's name.\"},\"file_size\":{\"type\":\"integer\",\"description\":\"The attachments file's size, in bytes.\"}},\"additionalProperties\":false}},\"rich_text\":{\"type\":\"array\",\"description\":\"The [rich text formatting](https://support.zoom.us/hc/en-us/articles/115004789183-Replying-to-and-managing-chat-messages)object. Use this object to include rich text info in sent message's content.\",\"items\":{\"type\":\"object\",\"required\":[\"start_position\",\"end_position\",\"format_type\"],\"properties\":{\"start_position\":{\"type\":\"integer\",\"description\":\"The start position of the rich text in the message string. \\n\\nFor example if the first character at the beginning of the message is bold, the value for this field will be 0.\"},\"end_position\":{\"type\":\"integer\",\"description\":\"The end position of the rich text.\"},\"format_type\":{\"type\":\"string\",\"description\":\"The type of rich text.\",\"enum\":[\"Bold\",\"Italic\",\"Strikethrough\",\"BulletedList\",\"NumberedList\",\"Underline\",\"FontSize\",\"FontColor\",\"BackgroundColor\",\"LeftIndent\",\"Paragraph\",\"Quote\",\"AddLink\"]},\"format_attr\":{\"type\":\"string\",\"description\":\"This field is non-null if the value of the format_type field is listed below:\\n\\n1.FontSize value: s for small, m for medium, or l for large font size. \\n\\n2.FontColor and BackgroundColor value: only supports RGB value. For example: FFC0CB \\n\\n3.LeftIndent value: a positive pixel length.  \\n\\n4.Paragraph value: h1 for Heading 1, h2 for Heading 2, or h3 for Heading 3. \\n\\n5.AddLink value: must be a valid URL, with an http or https prefix. For example: https://example.com\"}},\"additionalProperties\":false},\"reply_main_message_id\":{\"type\":\"string\",\"description\":\"The ID of the parent message in the chat thread.It's only available if the message is a reply to a main message in a thread.\"},\"date_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The date and time at which the message was sent, in yyyy-MM-dd'T'HH:mm:ss'Z' format.\"},\"timestamp\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp for when the message was sent.\"}}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"team_chat.app_mention\\\",\\n  \\\"event_ts\\\": 1696914490200,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"asgVcjZnWWRLWvv_GtyGuaxg\\\",\\n    \\\"operator\\\": \\\"example@zoom.us\\\",\\n    \\\"operator_member_id\\\": \\\"R4VM29Oj0fVM2hhEmSKVM2hhezJTezJTKVM2hezJT2hezJTSK\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"by_external_user\\\": true,\\n    \\\"object\\\": {\\n      \\\"message_id\\\": \\\"5DD614F4-DD19-4C39-8D03-93C4D0A0C839\\\",\\n      \\\"type\\\": \\\"to_channel\\\",\\n      \\\"channel_id\\\": \\\"8AmQ0WOoQ1O3a84yYFc2xQ\\\",\\n      \\\"channel_name\\\": \\\"Team Chat\\\",\\n      \\\"message\\\": \\\"hello\\\",\\n      \\\"files\\\": [\\n        {\\n          \\\"file_id\\\": \\\"56bed9orRnOSEnBoX_HQvA\\\",\\n          \\\"file_name\\\": \\\"Screenshot 2023-12-22 at 11.21.38 AM.png\\\",\\n          \\\"file_size\\\": 23025,\\n          \\\"OS_file_type\\\": \\\"PNG\\\",\\n          \\\"file_message_type\\\": \\\"image\\\"\\n        }\\n      ],\\n      \\\"rich_text\\\": [\\n        {\\n          \\\"start_position\\\": 0,\\n          \\\"end_position\\\": 1,\\n          \\\"format_type\\\": \\\"Bold\\\"\\n        },\\n        {\\n          \\\"start_position\\\": 1,\\n          \\\"end_position\\\": 2,\\n          \\\"format_type\\\": \\\"BulletedList\\\"\\n        },\\n        {\\n          \\\"start_position\\\": 2,\\n          \\\"end_position\\\": 3,\\n          \\\"format_type\\\": \\\"AddLink\\\",\\n          \\\"format_attr\\\": \\\"https://www.example.com/templates/daily-stand-up\\\"\\n        }\\n      ],\\n      \\\"reply_main_message_id\\\": \\\"5DD614F4-DD19-4C39-8D03-93C4D0A0C838\\\",\\n      \\\"date_time\\\": \\\"2023-10-10T05:08:10Z\\\",\\n      \\\"timestamp\\\": 1696914490157\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\",\"enum\":[\"team_chat.app_mention\"]},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\",\"operator_member_id\",\"by_external_user\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who sent the app mention message. Will be empty when by_external_user is true\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user who sent the app mention message. Will be empty when by_external_user is true\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user who sent the app mention message. Will be empty when by_external_user is true\"},\"operator_member_id\":{\"type\":\"string\",\"description\":\"The member ID of the user who sent the app mention message.\"},\"by_external_user\":{\"type\":\"boolean\",\"description\":\"Whether this action was performed by an external user.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the app mention message.\",\"required\":[\"message_id\",\"type\",\"message\",\"timestamp\",\"date_time\"],\"properties\":{\"message_id\":{\"type\":\"string\",\"description\":\"The message's ID.\"},\"type\":{\"type\":\"string\",\"description\":\"The type of message: \\n* to_channel \u2014 The message was sent to a chat channel.\",\"enum\":[\"to_channel\"]},\"channel_id\":{\"type\":\"string\",\"description\":\"The chat channel's ID.\"},\"channel_name\":{\"type\":\"string\",\"description\":\"The chat channel's name.\"},\"message\":{\"type\":\"string\",\"description\":\"The sent message's content.\"},\"files\":{\"type\":\"array\",\"description\":\"The attachments file info in sent message's content.\",\"items\":{\"type\":\"object\",\"required\":[\"OS_file_type\",\"file_id\",\"file_message_type\",\"file_name\",\"file_size\"],\"properties\":{\"OS_file_type\":{\"type\":\"string\",\"description\":\"The attachments file extension type\"},\"file_id\":{\"type\":\"string\",\"description\":\"The attachments file's universally unique identifier (UUID), in base64-encoded format.\"},\"file_message_type\":{\"type\":\"string\",\"description\":\"The attachments file message type\",\"enum\":[\"file\",\"image\",\"audio\",\"audio v2\",\"code snippet\",\"screen shot\"]},\"file_name\":{\"type\":\"string\",\"description\":\"The attachments file's name.\"},\"file_size\":{\"type\":\"integer\",\"description\":\"The attachments file's size, in bytes.\"}},\"additionalProperties\":false}},\"rich_text\":{\"type\":\"array\",\"description\":\"The [rich text formatting](https://support.zoom.us/hc/en-us/articles/115004789183-Replying-to-and-managing-chat-messages)object. Use this object to include rich text info in sent message's content.\",\"items\":{\"type\":\"object\",\"required\":[\"start_position\",\"end_position\",\"format_type\"],\"properties\":{\"start_position\":{\"type\":\"integer\",\"description\":\"The start position of the rich text in the message string. \\n\\nFor example if the first character at the beginning of the message is bold, the value for this field will be 0.\"},\"end_position\":{\"type\":\"integer\",\"description\":\"The end position of the rich text.\"},\"format_type\":{\"type\":\"string\",\"description\":\"The type of rich text.\",\"enum\":[\"Bold\",\"Italic\",\"Strikethrough\",\"BulletedList\",\"NumberedList\",\"Underline\",\"FontSize\",\"FontColor\",\"BackgroundColor\",\"LeftIndent\",\"Paragraph\",\"Quote\",\"AddLink\"]},\"format_attr\":{\"type\":\"string\",\"description\":\"This field is non-null if the value of the format_type field is listed below:\\n\\n1.FontSize value: s for small, m for medium, or l for large font size. \\n\\n2.FontColor and BackgroundColor value: only supports RGB value. For example: FFC0CB \\n\\n3.LeftIndent value: a positive pixel length.  \\n\\n4.Paragraph value: h1 for Heading 1, h2 for Heading 2, or h3 for Heading 3. \\n\\n5.AddLink value: must be a valid URL, with an http or https prefix. For example: https://example.com\"}},\"additionalProperties\":false},\"reply_main_message_id\":{\"type\":\"string\",\"description\":\"The ID of the parent message in the chat thread.It's only available if the message is a reply to a main message in a thread.\"},\"date_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The date and time at which the message was sent, in yyyy-MM-dd'T'HH:mm:ss'Z' format.\"},\"timestamp\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp for when the message was sent.\"}}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"team_chat.app_mention\\\",\\n  \\\"event_ts\\\": 1696914490200,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"asgVcjZnWWRLWvv_GtyGuaxg\\\",\\n    \\\"operator\\\": \\\"example@zoom.us\\\",\\n    \\\"operator_member_id\\\": \\\"R4VM29Oj0fVM2hhEmSKVM2hhezJTezJTKVM2hezJT2hezJTSK\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"by_external_user\\\": true,\\n    \\\"object\\\": {\\n      \\\"message_id\\\": \\\"5DD614F4-DD19-4C39-8D03-93C4D0A0C839\\\",\\n      \\\"type\\\": \\\"to_channel\\\",\\n      \\\"channel_id\\\": \\\"8AmQ0WOoQ1O3a84yYFc2xQ\\\",\\n      \\\"channel_name\\\": \\\"Team Chat\\\",\\n      \\\"message\\\": \\\"hello\\\",\\n      \\\"files\\\": [\\n        {\\n          \\\"file_id\\\": \\\"56bed9orRnOSEnBoX_HQvA\\\",\\n          \\\"file_name\\\": \\\"Screenshot 2023-12-22 at 11.21.38 AM.png\\\",\\n          \\\"file_size\\\": 23025,\\n          \\\"OS_file_type\\\": \\\"PNG\\\",\\n          \\\"file_message_type\\\": \\\"image\\\"\\n        }\\n      ],\\n      \\\"rich_text\\\": [\\n        {\\n          \\\"start_position\\\": 0,\\n          \\\"end_position\\\": 1,\\n          \\\"format_type\\\": \\\"Bold\\\"\\n        },\\n        {\\n          \\\"start_position\\\": 1,\\n          \\\"end_position\\\": 2,\\n          \\\"format_type\\\": \\\"BulletedList\\\"\\n        },\\n        {\\n          \\\"start_position\\\": 2,\\n          \\\"end_position\\\": 3,\\n          \\\"format_type\\\": \\\"AddLink\\\",\\n          \\\"format_attr\\\": \\\"https://www.example.com/templates/daily-stand-up\\\"\\n        }\\n      ],\\n      \\\"reply_main_message_id\\\": \\\"5DD614F4-DD19-4C39-8D03-93C4D0A0C838\\\",\\n      \\\"date_time\\\": \\\"2023-10-10T05:08:10Z\\\",\\n      \\\"timestamp\\\": 1696914490157\\n    }\\n  }\\n}\"}}}}}}}},\"interactive_message_select\":{\"post\":{\"tags\":[\"interactive_message_select\"],\"operationId\":\"interactive_message_select\",\"requestBody\":{\"description\":\"# interactive_message_select\\n\\nAfter a user selects an option in a Chatbot [message with a Dropdown](https://marketplace.zoom.us/docs/guides/chatbots/customizing-messages/message-with-dropdown), the `interactive_message_select` event is invoked. The Chatbot sends a POST request associated with this event along with data on the selected option to the [Bot Endpoint URL](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-chatbot-app#set-features) specified in the app's dashboard.\\n\\nThe `selectedItems` array displays the value chosen by the user from the  `selected_items` array in the original message sent by the Chatbot.\\n\\n\\n\\n**Event type**: `interactive_message_select`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"Name of the event.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"accountId\":{\"type\":\"string\",\"description\":\"The account ID of the user sending the command.\"},\"channelName\":{\"type\":\"string\",\"description\":\"Channel in which the message was sent. \"},\"messageId\":{\"type\":\"string\",\"description\":\"Unique ID of the message.\"},\"original\":{\"type\":\"object\",\"description\":\"Original message sent to user.\",\"properties\":{\"head\":{\"type\":\"object\",\"properties\":{\"sub_head\":{\"type\":\"object\",\"properties\":{\"text\":{\"type\":\"string\"}}},\"text\":{\"type\":\"string\"}}},\"body\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"select_items\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"text\":{\"type\":\"string\"},\"value\":{\"type\":\"string\"}}}},\"text\":{\"type\":\"string\"},\"type\":{\"type\":\"string\"}}}}}},\"robotJid\":{\"type\":\"string\",\"description\":\"Unique credential of the Chatbot.\"},\"selectedItems\":{\"type\":\"array\",\"description\":\"Value of item selected by the user.\",\"items\":{\"type\":\"object\",\"properties\":{\"value\":{\"type\":\"string\"}}}},\"timestamp\":{\"type\":\"integer\",\"format\":\"date-time\",\"description\":\"Time command message was sent.\"},\"toJid\":{\"type\":\"string\",\"description\":\"JID of the channel or user the message was sent to.\"},\"userId\":{\"type\":\"string\",\"description\":\"User sending the command.\"},\"userJid\":{\"type\":\"string\",\"description\":\"JID of user who is sending the command.\"},\"userName\":{\"type\":\"string\",\"description\":\"User's display name.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"interactive_message_select\\\",\\n  \\\"event_ts\\\": 1560891284464,\\n  \\\"payload\\\": {\\n    \\\"accountId\\\": \\\"gVcjZnWWRLWvv_GtyGuaxg\\\",\\n    \\\"channelName\\\": \\\"Jill Chill\\\",\\n    \\\"messageId\\\": \\\"20190618205440879_sRHVqf4_aw1\\\",\\n    \\\"original\\\": {\\n      \\\"head\\\": {\\n        \\\"sub_head\\\": {\\n          \\\"text\\\": \\\"I am a sub header\\\"\\n        },\\n        \\\"text\\\": \\\"I am a header\\\"\\n      },\\n      \\\"body\\\": [\\n        {\\n          \\\"select_items\\\": [\\n            {\\n              \\\"text\\\": \\\"Lions\\\",\\n              \\\"value\\\": \\\"lions\\\"\\n            },\\n            {\\n              \\\"text\\\": \\\"Tigers\\\",\\n              \\\"value\\\": \\\"tigers\\\"\\n            },\\n            {\\n              \\\"text\\\": \\\"Bears\\\",\\n              \\\"value\\\": \\\"bears\\\"\\n            }\\n          ],\\n          \\\"text\\\": \\\"Animals\\\",\\n          \\\"type\\\": \\\"select\\\"\\n        }\\n      ]\\n    },\\n    \\\"robotJid\\\": \\\"v1m0yn1imztuogsxjje8fdew@example.com\\\",\\n    \\\"selectedItems\\\": [\\n      {\\n        \\\"value\\\": \\\"lions\\\"\\n      }\\n    ],\\n    \\\"timestamp\\\": 1560891284464,\\n    \\\"toJid\\\": \\\"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\\\",\\n    \\\"userId\\\": \\\"KdYKjnimT4KPd8KKdQt9FQ\\\",\\n    \\\"userJid\\\": \\\"kdykjnimt4kpd8kkdqt9fq@example.com\\\",\\n    \\\"userName\\\": \\\"Jill Chill\\\"\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"Name of the event.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"accountId\":{\"type\":\"string\",\"description\":\"The account ID of the user sending the command.\"},\"channelName\":{\"type\":\"string\",\"description\":\"Channel in which the message was sent. \"},\"messageId\":{\"type\":\"string\",\"description\":\"Unique ID of the message.\"},\"original\":{\"type\":\"object\",\"description\":\"Original message sent to user.\",\"properties\":{\"head\":{\"type\":\"object\",\"properties\":{\"sub_head\":{\"type\":\"object\",\"properties\":{\"text\":{\"type\":\"string\"}}},\"text\":{\"type\":\"string\"}}},\"body\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"select_items\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"text\":{\"type\":\"string\"},\"value\":{\"type\":\"string\"}}}},\"text\":{\"type\":\"string\"},\"type\":{\"type\":\"string\"}}}}}},\"robotJid\":{\"type\":\"string\",\"description\":\"Unique credential of the Chatbot.\"},\"selectedItems\":{\"type\":\"array\",\"description\":\"Value of item selected by the user.\",\"items\":{\"type\":\"object\",\"properties\":{\"value\":{\"type\":\"string\"}}}},\"timestamp\":{\"type\":\"integer\",\"format\":\"date-time\",\"description\":\"Time command message was sent.\"},\"toJid\":{\"type\":\"string\",\"description\":\"JID of the channel or user the message was sent to.\"},\"userId\":{\"type\":\"string\",\"description\":\"User sending the command.\"},\"userJid\":{\"type\":\"string\",\"description\":\"JID of user who is sending the command.\"},\"userName\":{\"type\":\"string\",\"description\":\"User's display name.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"interactive_message_select\\\",\\n  \\\"event_ts\\\": 1560891284464,\\n  \\\"payload\\\": {\\n    \\\"accountId\\\": \\\"gVcjZnWWRLWvv_GtyGuaxg\\\",\\n    \\\"channelName\\\": \\\"Jill Chill\\\",\\n    \\\"messageId\\\": \\\"20190618205440879_sRHVqf4_aw1\\\",\\n    \\\"original\\\": {\\n      \\\"head\\\": {\\n        \\\"sub_head\\\": {\\n          \\\"text\\\": \\\"I am a sub header\\\"\\n        },\\n        \\\"text\\\": \\\"I am a header\\\"\\n      },\\n      \\\"body\\\": [\\n        {\\n          \\\"select_items\\\": [\\n            {\\n              \\\"text\\\": \\\"Lions\\\",\\n              \\\"value\\\": \\\"lions\\\"\\n            },\\n            {\\n              \\\"text\\\": \\\"Tigers\\\",\\n              \\\"value\\\": \\\"tigers\\\"\\n            },\\n            {\\n              \\\"text\\\": \\\"Bears\\\",\\n              \\\"value\\\": \\\"bears\\\"\\n            }\\n          ],\\n          \\\"text\\\": \\\"Animals\\\",\\n          \\\"type\\\": \\\"select\\\"\\n        }\\n      ]\\n    },\\n    \\\"robotJid\\\": \\\"v1m0yn1imztuogsxjje8fdew@example.com\\\",\\n    \\\"selectedItems\\\": [\\n      {\\n        \\\"value\\\": \\\"lions\\\"\\n      }\\n    ],\\n    \\\"timestamp\\\": 1560891284464,\\n    \\\"toJid\\\": \\\"kdykjnimt4kpd8kkdqt9fq@example.com/robot_v1m0yn1imztuogsxjje8fdew@example.com\\\",\\n    \\\"userId\\\": \\\"KdYKjnimT4KPd8KKdQt9FQ\\\",\\n    \\\"userJid\\\": \\\"kdykjnimt4kpd8kkdqt9fq@example.com\\\",\\n    \\\"userName\\\": \\\"Jill Chill\\\"\\n  }\\n}\"}}}}}}}}},\"servers\":[{\"url\":\"https://{your-endpoint-url}\"}]}",
    "frontmatter": {
      "title": "Chatbot Webhooks",
      "keywords": "zoom, marketplace, chatbot, chat, bot, webhook, event,  interactive message, button, message, action",
      "description": "The chatbot webhooks allow developers to receive events for [chatbot](https://developers.zoom.us/docs/team-chat-apps/) features.  \r\n\r\nNote regarding JSON payloads for guest users:\r\nBe aware that guest users (users not signed in to Zoom or who are members of a different account) are assigned temporary IDs that are valid only for the duration of that meeting or chat channel. Data for guest users in JSON payloads is associated with these temporary IDs.\r\n\r\n",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
