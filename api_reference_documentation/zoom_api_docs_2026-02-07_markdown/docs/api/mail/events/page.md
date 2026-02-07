# Mail Webhooks

- Source URL: https://developers.zoom.us/docs/api/mail/events/
- Snapshot path: docs/api/mail/events
- Generated (UTC): 2026-02-07T22:40:10.837076+00:00

## Frontmatter

```json
{
  "title": "Mail Webhooks",
  "keywords": "zoom, webhook, event, mail",
  "description": "The Zoom Mail webhooks allow developers to receive events for [Zoom Mail](https://developers.zoom.us/docs/zoom-mail/) features.",
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
    "title": "Mail",
    "description": "The Zoom Mail webhooks allow developers to receive events for [Zoom Mail](https://developers.zoom.us/docs/zoom-mail/) features.",
    "version": "1.0.0"
  },
  "paths": {
    "mail.history_event": {
      "post": {
        "tags": [
          "mail"
        ],
        "operationId": "mail.history_event",
        "requestBody": {
          "description": "# mail.history_event\nThe **Mail History Event** triggers whenever there is message related change in the mailbox.  For example, new message delivered, message moved between folders, message removed, and similar items.\n\n## Prerequisites\n* Account must have Workplace Business or higher license type.\n* Account must have enabled Zoom Mail Service and finished Mail Service Onboarding Flow.\n* Mail service End-To-End Encryption is disabled.\n* The requested mailbox has been provisioned.\n* Enabled **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/) with the following configurations:\n  * A valid **Event Notification Endpoint URL**.\n  * The **Mail History Event** subscription enabled under the **Mail** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `mail:read`,`mail:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `email:read:msg`,`email:read:msg:admin`\n\n**Event type**: `mail.history_event`\n",
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
                    "description": "The name of the event."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "A timestamp at which the event occurred in echo milliseconds."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "operator",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user whose mailbox having the event."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The email of the user who triggered the event.  It could be the mailbox owner or the mailbox delegate user."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "email",
                          "history_id"
                        ],
                        "description": "The information of the event.",
                        "properties": {
                          "email": {
                            "type": "string",
                            "description": "The mailbox email of which event is from."
                          },
                          "history_id": {
                            "type": "integer",
                            "description": "History ID of the event that is generated from the message change."
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
                  "value": "{\n  \"event\": \"mail.history_event\",\n  \"event_ts\": 1732336289348,\n  \"payload\": {\n    \"account_id\": \"iZudq0ta9dafad\",\n    \"operator\": \"john.doe@example.com\",\n    \"object\": {\n      \"email\": \"john.doe@example.com\",\n      \"history_id\": 347658\n    }\n  }\n}"
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
                      "description": "The name of the event."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "A timestamp at which the event occurred in echo milliseconds."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "operator",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user whose mailbox having the event."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The email of the user who triggered the event.  It could be the mailbox owner or the mailbox delegate user."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "email",
                            "history_id"
                          ],
                          "description": "The information of the event.",
                          "properties": {
                            "email": {
                              "type": "string",
                              "description": "The mailbox email of which event is from."
                            },
                            "history_id": {
                              "type": "integer",
                              "description": "History ID of the event that is generated from the message change."
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
                    "value": "{\n  \"event\": \"mail.history_event\",\n  \"event_ts\": 1732336289348,\n  \"payload\": {\n    \"account_id\": \"iZudq0ta9dafad\",\n    \"operator\": \"john.doe@example.com\",\n    \"object\": {\n      \"email\": \"john.doe@example.com\",\n      \"history_id\": 347658\n    }\n  }\n}"
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
    "title": "Mail",
    "description": "The Zoom Mail webhooks allow developers to receive events for [Zoom Mail](https://developers.zoom.us/docs/zoom-mail/) features.",
    "version": "1.0.0"
  },
  "webhooks": {
    "mail.history_event": {
      "post": {
        "tags": [
          "mail"
        ],
        "operationId": "mail.history_event",
        "requestBody": {
          "description": "# mail.history_event\nThe **Mail History Event** triggers whenever there is message related change in the mailbox.  For example, new message delivered, message moved between folders, message removed, and similar items.\n\n## Prerequisites\n* Account must have Workplace Business or higher license type.\n* Account must have enabled Zoom Mail Service and finished Mail Service Onboarding Flow.\n* Mail service End-To-End Encryption is disabled.\n* The requested mailbox has been provisioned.\n* Enabled **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/) with the following configurations:\n  * A valid **Event Notification Endpoint URL**.\n  * The **Mail History Event** subscription enabled under the **Mail** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `mail:read`,`mail:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `email:read:msg`,`email:read:msg:admin`\n\n**Event type**: `mail.history_event`\n",
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
                    "description": "The name of the event."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "A timestamp at which the event occurred in echo milliseconds."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "operator",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user whose mailbox having the event."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The email of the user who triggered the event.  It could be the mailbox owner or the mailbox delegate user."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "email",
                          "history_id"
                        ],
                        "description": "The information of the event.",
                        "properties": {
                          "email": {
                            "type": "string",
                            "description": "The mailbox email of which event is from."
                          },
                          "history_id": {
                            "type": "integer",
                            "description": "History ID of the event that is generated from the message change."
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
                  "value": "{\n  \"event\": \"mail.history_event\",\n  \"event_ts\": 1732336289348,\n  \"payload\": {\n    \"account_id\": \"iZudq0ta9dafad\",\n    \"operator\": \"john.doe@example.com\",\n    \"object\": {\n      \"email\": \"john.doe@example.com\",\n      \"history_id\": 347658\n    }\n  }\n}"
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
                      "description": "The name of the event."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "A timestamp at which the event occurred in echo milliseconds."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "operator",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user whose mailbox having the event."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The email of the user who triggered the event.  It could be the mailbox owner or the mailbox delegate user."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "email",
                            "history_id"
                          ],
                          "description": "The information of the event.",
                          "properties": {
                            "email": {
                              "type": "string",
                              "description": "The mailbox email of which event is from."
                            },
                            "history_id": {
                              "type": "integer",
                              "description": "History ID of the event that is generated from the message change."
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
                    "value": "{\n  \"event\": \"mail.history_event\",\n  \"event_ts\": 1732336289348,\n  \"payload\": {\n    \"account_id\": \"iZudq0ta9dafad\",\n    \"operator\": \"john.doe@example.com\",\n    \"object\": {\n      \"email\": \"john.doe@example.com\",\n      \"history_id\": 347658\n    }\n  }\n}"
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
      "mail",
      "events"
    ]
  },
  "downloadPath": "/api-hub/mail/events/webhooks.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": false
  },
  "productMeta": {
    "specType": "adoc",
    "slug": "/docs/api/rest/reference/zoom-mail/events",
    "productName": "Zoom Mail",
    "title": "Zoom API Events - Mail",
    "keywords": "zoom, webhook, event, mail",
    "zoapEndpoint": "webhookByProduct",
    "params": {
      "productName": "Zoom Mail"
    },
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "productDisplayTitle": "Mail",
    "scalarSlug": "/docs/api/mail/events",
    "description": "The Zoom Mail webhooks allow developers to receive events for [Zoom Mail](/docs/zoom-mail) features.",
    "svgname": "_mail"
  },
  "frontmatter": {
    "title": "Mail Webhooks",
    "keywords": "zoom, webhook, event, mail",
    "description": "The Zoom Mail webhooks allow developers to receive events for [Zoom Mail](https://developers.zoom.us/docs/zoom-mail/) features.",
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
        "mail",
        "events"
      ]
    },
    "downloadPath": "/api-hub/mail/events/webhooks.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": false
    },
    "productMeta": {
      "specType": "adoc",
      "slug": "/docs/api/rest/reference/zoom-mail/events",
      "productName": "Zoom Mail",
      "title": "Zoom API Events - Mail",
      "keywords": "zoom, webhook, event, mail",
      "zoapEndpoint": "webhookByProduct",
      "params": {
        "productName": "Zoom Mail"
      },
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "productDisplayTitle": "Mail",
      "scalarSlug": "/docs/api/mail/events",
      "description": "The Zoom Mail webhooks allow developers to receive events for [Zoom Mail](/docs/zoom-mail) features.",
      "svgname": "_mail"
    },
    "spec": "{\"openapi\":\"3.1.0\",\"info\":{\"title\":\"Mail\",\"description\":\"The Zoom Mail webhooks allow developers to receive events for [Zoom Mail](https://developers.zoom.us/docs/zoom-mail/) features.\",\"version\":\"1.0.0\"},\"paths\":{\"mail.history_event\":{\"post\":{\"tags\":[\"mail\"],\"operationId\":\"mail.history_event\",\"requestBody\":{\"description\":\"# mail.history_event\\nThe **Mail History Event** triggers whenever there is message related change in the mailbox.  For example, new message delivered, message moved between folders, message removed, and similar items.\\n\\n## Prerequisites\\n* Account must have Workplace Business or higher license type.\\n* Account must have enabled Zoom Mail Service and finished Mail Service Onboarding Flow.\\n* Mail service End-To-End Encryption is disabled.\\n* The requested mailbox has been provisioned.\\n* Enabled **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/) with the following configurations:\\n  * A valid **Event Notification Endpoint URL**.\\n  * The **Mail History Event** subscription enabled under the **Mail** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `mail:read`,`mail:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `email:read:msg`,`email:read:msg:admin`\\n\\n**Event type**: `mail.history_event`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred in echo milliseconds.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user whose mailbox having the event.\"},\"operator\":{\"type\":\"string\",\"description\":\"The email of the user who triggered the event.  It could be the mailbox owner or the mailbox delegate user.\"},\"object\":{\"type\":\"object\",\"required\":[\"email\",\"history_id\"],\"description\":\"The information of the event.\",\"properties\":{\"email\":{\"type\":\"string\",\"description\":\"The mailbox email of which event is from.\"},\"history_id\":{\"type\":\"integer\",\"description\":\"History ID of the event that is generated from the message change.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"mail.history_event\\\",\\n  \\\"event_ts\\\": 1732336289348,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"iZudq0ta9dafad\\\",\\n    \\\"operator\\\": \\\"john.doe@example.com\\\",\\n    \\\"object\\\": {\\n      \\\"email\\\": \\\"john.doe@example.com\\\",\\n      \\\"history_id\\\": 347658\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred in echo milliseconds.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user whose mailbox having the event.\"},\"operator\":{\"type\":\"string\",\"description\":\"The email of the user who triggered the event.  It could be the mailbox owner or the mailbox delegate user.\"},\"object\":{\"type\":\"object\",\"required\":[\"email\",\"history_id\"],\"description\":\"The information of the event.\",\"properties\":{\"email\":{\"type\":\"string\",\"description\":\"The mailbox email of which event is from.\"},\"history_id\":{\"type\":\"integer\",\"description\":\"History ID of the event that is generated from the message change.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"mail.history_event\\\",\\n  \\\"event_ts\\\": 1732336289348,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"iZudq0ta9dafad\\\",\\n    \\\"operator\\\": \\\"john.doe@example.com\\\",\\n    \\\"object\\\": {\\n      \\\"email\\\": \\\"john.doe@example.com\\\",\\n      \\\"history_id\\\": 347658\\n    }\\n  }\\n}\"}}}}}}}}},\"servers\":[{\"url\":\"https://{your-endpoint-url}\"}]}",
    "frontmatter": {
      "title": "Mail Webhooks",
      "keywords": "zoom, webhook, event, mail",
      "description": "The Zoom Mail webhooks allow developers to receive events for [Zoom Mail](https://developers.zoom.us/docs/zoom-mail/) features.",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
