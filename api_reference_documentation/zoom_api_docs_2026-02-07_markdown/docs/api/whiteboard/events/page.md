# Whiteboard Webhooks

- Source URL: https://developers.zoom.us/docs/api/whiteboard/events/
- Snapshot path: docs/api/whiteboard/events
- Generated (UTC): 2026-02-07T22:40:11.985765+00:00

## Frontmatter

```json
{
  "title": "Whiteboard Webhooks",
  "keywords": "zoom, marketplace, webhook, event, create, update, delete, real-time, data, summary, qos, quality, service",
  "description": "The Zoom Whiteboard webhooks allow developers to receive events for [Zoom Whiteboard](https://developers.zoom.us/docs/zoom-whiteboard/) features.",
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
    "title": "Whiteboard",
    "description": "The Zoom Whiteboard webhooks allow developers to receive events for [Zoom Whiteboard](https://developers.zoom.us/docs/zoom-whiteboard/) features.",
    "version": "1.0.0"
  },
  "paths": {
    "whiteboard.shared": {
      "post": {
        "tags": [
          "whiteboard"
        ],
        "operationId": "whiteboard.shared",
        "requestBody": {
          "description": "# whiteboard.shared\n\nThe **Whiteboard was shared** event is triggered every time a whiteboard is shared with a Zoom user or a team chat channel by one of your whiteboard collaborators.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Whiteboard was shared** subscription enabled under the **Whiteboard** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read`,`whiteboard:read:admin`,`whiteboard:read`,`whiteboard:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:list_collaborators`,`whiteboard:read:list_collaborators:admin`\n\n**Event type**: `whiteboard.shared`\n",
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
                    "description": "A timestamp at which the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "operator_collaborator_id",
                      "object"
                    ],
                    "properties": {
                      "operator_id": {
                        "type": "string",
                        "description": "nullable The Zoom user ID of the operator who added users or team chat channels to the whiteboard. It should be null for external user(the user is external to the whiteboard's organization)."
                      },
                      "operator_collaborator_id": {
                        "type": "string",
                        "description": "not nullable The unique ID of the collaborator who added users or team chat channels to the whiteboard. Each collaborator will have a ID in a whiteboard."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the whiteboard.",
                        "required": [
                          "whiteboard_id",
                          "collaborators"
                        ],
                        "properties": {
                          "whiteboard_id": {
                            "type": "string",
                            "description": "The whiteboard ID."
                          },
                          "meeting_id": {
                            "type": "string",
                            "description": "The meeting ID. This response only returns the whiteboard is shared in a meeting."
                          },
                          "meeting_uuid": {
                            "type": "string",
                            "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID. This response only returns the whiteboard is shared in a meeting."
                          },
                          "collaborators": {
                            "type": "array",
                            "description": "Information about the whiteboard collaborators.",
                            "items": {
                              "anyOf": [
                                {
                                  "type": "object",
                                  "required": [
                                    "collaborator_id",
                                    "collaborator_type",
                                    "role"
                                  ],
                                  "properties": {
                                    "collaborator_id": {
                                      "type": "string",
                                      "description": "The collaborator's unique ID."
                                    },
                                    "collaborator_name": {
                                      "type": "string",
                                      "description": "The collaborator's display name."
                                    },
                                    "user_id": {
                                      "type": "string",
                                      "description": "The collaborator's user ID. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. "
                                    },
                                    "email": {
                                      "type": "string",
                                      "format": "email",
                                      "description": "User email. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. See [Email address display rules](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#email-address) for details."
                                    },
                                    "channel_id": {
                                      "type": "string",
                                      "description": "The team chat channel ID you want to share with. This response only returns if the collaborator_type is 2. Otherwise it should be null."
                                    },
                                    "collaborator_type": {
                                      "type": "integer",
                                      "description": "Collaborator type. \nEnum: \n*0 - SAME_ACCOUNT_USER \n*1 - EXTERNAL_USER \n*2 - CHANNEL \n",
                                      "enum": [
                                        0,
                                        1,
                                        2
                                      ],
                                      "x-enum-descriptions": [
                                        "SAME_ACCOUNT_USER",
                                        "EXTERNAL_USER",
                                        "CHANNEL"
                                      ]
                                    },
                                    "role": {
                                      "type": "integer",
                                      "description": "Collaborator permission. \nEnum: \n*0 - OWNER \n*1 - CO_OWNER \n*2 - EDITOR \n*3 - COMMENTER \n*4 - VIEWER \n.",
                                      "enum": [
                                        0,
                                        1,
                                        2,
                                        3,
                                        4
                                      ],
                                      "x-enum-descriptions": [
                                        "OWNER",
                                        "CO_OWNER",
                                        "EDITOR",
                                        "COMMENTER",
                                        "VIEWER"
                                      ]
                                    }
                                  },
                                  "additionalProperties": false
                                }
                              ]
                            },
                            "additionalProperties": false
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
                  "value": "{\n  \"event\": \"whiteboard.shared\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operator_collaborator_id\": \"Cvftg6789087654443lkhk\",\n    \"object\": {\n      \"whiteboard_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"meeting_id\": \"1234567890\",\n      \"meeting_ uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n      \"collaborators\": [\n        {\n          \"collaborator_id\": \"x4g6hy789k0o878k\",\n          \"collaborator_name\": \"Jack\",\n          \"email\": \"user@example.com\",\n          \"user_id\": \"er54t67y8ui9o0p0\",\n          \"collaborator_type\": 0,\n          \"role\": 3\n        },\n        {\n          \"collaborator_id\": \"x4g6hy789k0o878k\",\n          \"collaborator_name\": \"ext***@exa***.com\",\n          \"collaborator_type\": 1,\n          \"role\": 3\n        },\n        {\n          \"collaborator_id\": \"df4g6hy789k0o878k\",\n          \"channel_id\": \"example@conference.xmpp.zoom.us\",\n          \"collaborator_name\": \"Channel A\",\n          \"collaborator_type\": 2,\n          \"role\": 3\n        }\n      ]\n    }\n  }\n}"
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
                      "description": "A timestamp at which the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "operator_collaborator_id",
                        "object"
                      ],
                      "properties": {
                        "operator_id": {
                          "type": "string",
                          "description": "nullable The Zoom user ID of the operator who added users or team chat channels to the whiteboard. It should be null for external user(the user is external to the whiteboard's organization)."
                        },
                        "operator_collaborator_id": {
                          "type": "string",
                          "description": "not nullable The unique ID of the collaborator who added users or team chat channels to the whiteboard. Each collaborator will have a ID in a whiteboard."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the whiteboard.",
                          "required": [
                            "whiteboard_id",
                            "collaborators"
                          ],
                          "properties": {
                            "whiteboard_id": {
                              "type": "string",
                              "description": "The whiteboard ID."
                            },
                            "meeting_id": {
                              "type": "string",
                              "description": "The meeting ID. This response only returns the whiteboard is shared in a meeting."
                            },
                            "meeting_uuid": {
                              "type": "string",
                              "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID. This response only returns the whiteboard is shared in a meeting."
                            },
                            "collaborators": {
                              "type": "array",
                              "description": "Information about the whiteboard collaborators.",
                              "items": {
                                "anyOf": [
                                  {
                                    "type": "object",
                                    "required": [
                                      "collaborator_id",
                                      "collaborator_type",
                                      "role"
                                    ],
                                    "properties": {
                                      "collaborator_id": {
                                        "type": "string",
                                        "description": "The collaborator's unique ID."
                                      },
                                      "collaborator_name": {
                                        "type": "string",
                                        "description": "The collaborator's display name."
                                      },
                                      "user_id": {
                                        "type": "string",
                                        "description": "The collaborator's user ID. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. "
                                      },
                                      "email": {
                                        "type": "string",
                                        "format": "email",
                                        "description": "User email. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. See [Email address display rules](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#email-address) for details."
                                      },
                                      "channel_id": {
                                        "type": "string",
                                        "description": "The team chat channel ID you want to share with. This response only returns if the collaborator_type is 2. Otherwise it should be null."
                                      },
                                      "collaborator_type": {
                                        "type": "integer",
                                        "description": "Collaborator type. \nEnum: \n*0 - SAME_ACCOUNT_USER \n*1 - EXTERNAL_USER \n*2 - CHANNEL \n",
                                        "enum": [
                                          0,
                                          1,
                                          2
                                        ],
                                        "x-enum-descriptions": [
                                          "SAME_ACCOUNT_USER",
                                          "EXTERNAL_USER",
                                          "CHANNEL"
                                        ]
                                      },
                                      "role": {
                                        "type": "integer",
                                        "description": "Collaborator permission. \nEnum: \n*0 - OWNER \n*1 - CO_OWNER \n*2 - EDITOR \n*3 - COMMENTER \n*4 - VIEWER \n.",
                                        "enum": [
                                          0,
                                          1,
                                          2,
                                          3,
                                          4
                                        ],
                                        "x-enum-descriptions": [
                                          "OWNER",
                                          "CO_OWNER",
                                          "EDITOR",
                                          "COMMENTER",
                                          "VIEWER"
                                        ]
                                      }
                                    },
                                    "additionalProperties": false
                                  }
                                ]
                              },
                              "additionalProperties": false
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
                    "value": "{\n  \"event\": \"whiteboard.shared\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operator_collaborator_id\": \"Cvftg6789087654443lkhk\",\n    \"object\": {\n      \"whiteboard_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"meeting_id\": \"1234567890\",\n      \"meeting_ uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n      \"collaborators\": [\n        {\n          \"collaborator_id\": \"x4g6hy789k0o878k\",\n          \"collaborator_name\": \"Jack\",\n          \"email\": \"user@example.com\",\n          \"user_id\": \"er54t67y8ui9o0p0\",\n          \"collaborator_type\": 0,\n          \"role\": 3\n        },\n        {\n          \"collaborator_id\": \"x4g6hy789k0o878k\",\n          \"collaborator_name\": \"ext***@exa***.com\",\n          \"collaborator_type\": 1,\n          \"role\": 3\n        },\n        {\n          \"collaborator_id\": \"df4g6hy789k0o878k\",\n          \"channel_id\": \"example@conference.xmpp.zoom.us\",\n          \"collaborator_name\": \"Channel A\",\n          \"collaborator_type\": 2,\n          \"role\": 3\n        }\n      ]\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "whiteboard.sharing_updated": {
      "post": {
        "tags": [
          "whiteboard"
        ],
        "operationId": "whiteboard.sharing_updated",
        "requestBody": {
          "description": "# whiteboard.sharing_updated\n\nThe **Whiteboard collaborator's sharing settings were updated** event is triggered every time the of a whiteboard collaborator is updated or removed by one of your whiteboard owners.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Whiteboard collaborator's sharing settings were updated** subscription enabled under the **Whiteboard** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read`,`whiteboard:read:admin`,`whiteboard:read`,`whiteboard:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:list_collaborators`,`whiteboard:read:list_collaborators:admin`\n\n**Event type**: `whiteboard.sharing_updated`\n",
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
                    "description": "A timestamp at which the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "operator_collaborator_id",
                      "operation_type",
                      "object"
                    ],
                    "properties": {
                      "operator_id": {
                        "type": "string",
                        "description": "The Zoom user ID of the operator who changed the permission of collaborators of the whiteboard. It should be whiteboard owner user ID."
                      },
                      "operator_collaborator_id": {
                        "type": "string",
                        "description": "not nullable The whiteboard collaborator's universally unique identifier. Each collaborator will have a ID in a whiteboard."
                      },
                      "operation_type": {
                        "type": "integer",
                        "description": "Operation type. \nEnum: \n*0 - REMOVE \n*1 - UPDATE \n",
                        "enum": [
                          0,
                          1
                        ],
                        "x-enum-descriptions": [
                          "REMOVE",
                          "UPDATE"
                        ]
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the whiteboard collaborators.",
                        "required": [
                          "whiteboard_id",
                          "collaborators"
                        ],
                        "properties": {
                          "whiteboard_id": {
                            "type": "string",
                            "description": "The whiteboard ID."
                          },
                          "collaborators": {
                            "type": "array",
                            "description": "Information about the effected whiteboard collaborators.",
                            "items": {
                              "anyOf": [
                                {
                                  "type": "object",
                                  "required": [
                                    "collaborator_id",
                                    "collaborator_type",
                                    "role"
                                  ],
                                  "properties": {
                                    "collaborator_id": {
                                      "type": "string",
                                      "description": "The collaborator's ID. \n* If collaborator_type set to 0, this field refers to a user ID. If collaborator_type set to 2, this field refers to a team chat channel ID. If collaborator_type set to 1, this returns an empty value."
                                    },
                                    "collaborator_name": {
                                      "type": "string",
                                      "description": "The collaborator's display name."
                                    },
                                    "user_id": {
                                      "type": "string",
                                      "description": "The collaborator's user ID. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. "
                                    },
                                    "email": {
                                      "type": "string",
                                      "description": "User email. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. See [Email address display rules](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#email-address) for details."
                                    },
                                    "channel_id": {
                                      "type": "string",
                                      "description": "The team chat channel ID you want to share with. This response only returns if the collaborator_type is 2. Otherwise it should be null."
                                    },
                                    "collaborator_type": {
                                      "type": "integer",
                                      "description": "Collaborator type. \nEnum: \n*0 - SAME_ACCOUNT_USER \n*1 - EXTERNAL_USER \n*2 - CHANNEL \n",
                                      "enum": [
                                        0,
                                        1,
                                        2
                                      ],
                                      "x-enum-descriptions": [
                                        "SAME_ACCOUNT_USER",
                                        "EXTERNAL_USER",
                                        "CHANNEL"
                                      ]
                                    },
                                    "role": {
                                      "type": "integer",
                                      "description": "Collaborator permission. \nEnum: \n*0 - OWNER \n*1 - CO_OWNER \n*2 - EDITOR \n*3 - COMMENTER \n*4 - VIEWER \n.",
                                      "enum": [
                                        0,
                                        1,
                                        2,
                                        3,
                                        4
                                      ],
                                      "x-enum-descriptions": [
                                        "OWNER",
                                        "CO_OWNER",
                                        "EDITOR",
                                        "COMMENTER",
                                        "VIEWER"
                                      ]
                                    }
                                  },
                                  "additionalProperties": false
                                }
                              ]
                            },
                            "additionalProperties": false
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
                  "value": "{\n  \"event\": \"whiteboard.sharing_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operation_type\": 1,\n    \"operator_collaborator_id\": \"Cvftg6789087654443lkhk\",\n    \"object\": {\n      \"whiteboard_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"collaborators\": [\n        {\n          \"collaborator_id\": \"as35hjnh89899fmkl0ou\",\n          \"collaborator_name\": \"Jack\",\n          \"user_id\": \"er54t67y8ui9o0p0\",\n          \"email\": \"user@example.com\",\n          \"collaborator_type\": 0,\n          \"role\": 2\n        },\n        {\n          \"collaborator_id\": \"x4g6hy789k0o878k\",\n          \"collaborator_name\": \"ext***@exa***.com\",\n          \"collaborator_type\": 1,\n          \"role\": 3\n        },\n        {\n          \"collaborator_id\": \"34g6hy789k0o878k\",\n          \"channel_id\": \"example@conference.xmpp.zoom.us\",\n          \"collaborator_name\": \"Channel A\",\n          \"collaborator_type\": 2,\n          \"role\": 2\n        }\n      ]\n    }\n  }\n}"
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
                      "description": "A timestamp at which the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "operator_collaborator_id",
                        "operation_type",
                        "object"
                      ],
                      "properties": {
                        "operator_id": {
                          "type": "string",
                          "description": "The Zoom user ID of the operator who changed the permission of collaborators of the whiteboard. It should be whiteboard owner user ID."
                        },
                        "operator_collaborator_id": {
                          "type": "string",
                          "description": "not nullable The whiteboard collaborator's universally unique identifier. Each collaborator will have a ID in a whiteboard."
                        },
                        "operation_type": {
                          "type": "integer",
                          "description": "Operation type. \nEnum: \n*0 - REMOVE \n*1 - UPDATE \n",
                          "enum": [
                            0,
                            1
                          ],
                          "x-enum-descriptions": [
                            "REMOVE",
                            "UPDATE"
                          ]
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the whiteboard collaborators.",
                          "required": [
                            "whiteboard_id",
                            "collaborators"
                          ],
                          "properties": {
                            "whiteboard_id": {
                              "type": "string",
                              "description": "The whiteboard ID."
                            },
                            "collaborators": {
                              "type": "array",
                              "description": "Information about the effected whiteboard collaborators.",
                              "items": {
                                "anyOf": [
                                  {
                                    "type": "object",
                                    "required": [
                                      "collaborator_id",
                                      "collaborator_type",
                                      "role"
                                    ],
                                    "properties": {
                                      "collaborator_id": {
                                        "type": "string",
                                        "description": "The collaborator's ID. \n* If collaborator_type set to 0, this field refers to a user ID. If collaborator_type set to 2, this field refers to a team chat channel ID. If collaborator_type set to 1, this returns an empty value."
                                      },
                                      "collaborator_name": {
                                        "type": "string",
                                        "description": "The collaborator's display name."
                                      },
                                      "user_id": {
                                        "type": "string",
                                        "description": "The collaborator's user ID. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. "
                                      },
                                      "email": {
                                        "type": "string",
                                        "description": "User email. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. See [Email address display rules](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#email-address) for details."
                                      },
                                      "channel_id": {
                                        "type": "string",
                                        "description": "The team chat channel ID you want to share with. This response only returns if the collaborator_type is 2. Otherwise it should be null."
                                      },
                                      "collaborator_type": {
                                        "type": "integer",
                                        "description": "Collaborator type. \nEnum: \n*0 - SAME_ACCOUNT_USER \n*1 - EXTERNAL_USER \n*2 - CHANNEL \n",
                                        "enum": [
                                          0,
                                          1,
                                          2
                                        ],
                                        "x-enum-descriptions": [
                                          "SAME_ACCOUNT_USER",
                                          "EXTERNAL_USER",
                                          "CHANNEL"
                                        ]
                                      },
                                      "role": {
                                        "type": "integer",
                                        "description": "Collaborator permission. \nEnum: \n*0 - OWNER \n*1 - CO_OWNER \n*2 - EDITOR \n*3 - COMMENTER \n*4 - VIEWER \n.",
                                        "enum": [
                                          0,
                                          1,
                                          2,
                                          3,
                                          4
                                        ],
                                        "x-enum-descriptions": [
                                          "OWNER",
                                          "CO_OWNER",
                                          "EDITOR",
                                          "COMMENTER",
                                          "VIEWER"
                                        ]
                                      }
                                    },
                                    "additionalProperties": false
                                  }
                                ]
                              },
                              "additionalProperties": false
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
                    "value": "{\n  \"event\": \"whiteboard.sharing_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operation_type\": 1,\n    \"operator_collaborator_id\": \"Cvftg6789087654443lkhk\",\n    \"object\": {\n      \"whiteboard_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"collaborators\": [\n        {\n          \"collaborator_id\": \"as35hjnh89899fmkl0ou\",\n          \"collaborator_name\": \"Jack\",\n          \"user_id\": \"er54t67y8ui9o0p0\",\n          \"email\": \"user@example.com\",\n          \"collaborator_type\": 0,\n          \"role\": 2\n        },\n        {\n          \"collaborator_id\": \"x4g6hy789k0o878k\",\n          \"collaborator_name\": \"ext***@exa***.com\",\n          \"collaborator_type\": 1,\n          \"role\": 3\n        },\n        {\n          \"collaborator_id\": \"34g6hy789k0o878k\",\n          \"channel_id\": \"example@conference.xmpp.zoom.us\",\n          \"collaborator_name\": \"Channel A\",\n          \"collaborator_type\": 2,\n          \"role\": 2\n        }\n      ]\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "whiteboard.share_settings_updated": {
      "post": {
        "tags": [
          "whiteboard"
        ],
        "operationId": "whiteboard.share_settings_updated",
        "requestBody": {
          "description": "# whiteboard.share_settings_updated\n\nWhiteboard share settings can be updated by a whiteboard owner or co-owners.\n\nThe **Whiteboard's sharing setting were updated** event is triggered every time a whiteboard share setting is updated by whiteboard owner or co-owners.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Whiteboard's sharing setting were updated** subscription enabled under the **Whiteboard** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read`,`whiteboard:read:admin`,`whiteboard:read`,`whiteboard:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:share_setting`,`whiteboard:read:share_setting:admin`\n\n**Event type**: `whiteboard.share_settings_updated`\n",
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
                    "description": "A timestamp at which the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who updated the whiteboard share settings."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user who updated the whiteboard share settings."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user who updated the whiteboard share settings."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the whiteboard.",
                        "required": [
                          "whiteboard_id",
                          "settings"
                        ],
                        "properties": {
                          "whiteboard_id": {
                            "type": "string",
                            "description": "The whiteboard ID."
                          },
                          "settings": {
                            "type": "object",
                            "description": "Information about the whiteboard share settings.",
                            "properties": {
                              "share_link_setting": {
                                "type": "object",
                                "description": "Information about the whiteboard share link settings.",
                                "properties": {
                                  "share_scope": {
                                    "type": "integer",
                                    "description": "Who can access the share link",
                                    "enum": [
                                      0,
                                      1,
                                      2
                                    ],
                                    "x-enum-descriptions": [
                                      "Only invited members can access this board",
                                      "Anyone at the current organization",
                                      "Anyone with the link can view this board"
                                    ]
                                  },
                                  "share_role": {
                                    "type": "integer",
                                    "description": "Permission of the shared user.",
                                    "enum": [
                                      1,
                                      2,
                                      3,
                                      4
                                    ],
                                    "x-enum-descriptions": [
                                      "CO_OWNER",
                                      "EDITOR",
                                      "COMMENTER",
                                      "VIEWER"
                                    ]
                                  }
                                }
                              },
                              "advanced_share_setting": {
                                "type": "object",
                                "description": "Information about the whiteboard advanced share settings",
                                "properties": {
                                  "can_share": {
                                    "type": "boolean",
                                    "description": "Indicate whether allow to share whiteboard or not."
                                  },
                                  "share_to_external": {
                                    "type": "boolean",
                                    "description": "Indicate whether allowed to share with user who is in the external organization or not."
                                  },
                                  "external_can_share": {
                                    "type": "boolean",
                                    "description": "Indicate whether the external user is allowed to share whiteboard or not."
                                  }
                                }
                              }
                            }
                          }
                        },
                        "additionalProperties": true
                      }
                    },
                    "additionalProperties": true
                  }
                },
                "additionalProperties": true
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n    \"event\":\"whiteboard.share_setting_updated\",\n    \"event_ts\":1626230691572,\n    \"payload\":{\n        \"account_id\":\"AAAAAABBBB\",\n        \"operator\":\"admin@example.com\",\n        \"operator_id\":\"z8yCxjabcdEFGHfp8uQ\",\n        \"object\":{\n            \"whiteboard_id\":\"BK0R3Lg-Toi92viKHhzUVQ\",\n            \"settings\":{\n                \"share_link_setting\":{\n                    \"share_scope\":0,\n                    \"share_role\":4\n                },\n                \"advanced_share_setting\":{\n                    \"can_share\":true,\n                    \"share_to_external\":true,\n                    \"external_can_share\":false\n                }\n            }\n        }\n    }\n}"
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
                      "description": "A timestamp at which the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who updated the whiteboard share settings."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user who updated the whiteboard share settings."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user who updated the whiteboard share settings."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the whiteboard.",
                          "required": [
                            "whiteboard_id",
                            "settings"
                          ],
                          "properties": {
                            "whiteboard_id": {
                              "type": "string",
                              "description": "The whiteboard ID."
                            },
                            "settings": {
                              "type": "object",
                              "description": "Information about the whiteboard share settings.",
                              "properties": {
                                "share_link_setting": {
                                  "type": "object",
                                  "description": "Information about the whiteboard share link settings.",
                                  "properties": {
                                    "share_scope": {
                                      "type": "integer",
                                      "description": "Who can access the share link",
                                      "enum": [
                                        0,
                                        1,
                                        2
                                      ],
                                      "x-enum-descriptions": [
                                        "Only invited members can access this board",
                                        "Anyone at the current organization",
                                        "Anyone with the link can view this board"
                                      ]
                                    },
                                    "share_role": {
                                      "type": "integer",
                                      "description": "Permission of the shared user.",
                                      "enum": [
                                        1,
                                        2,
                                        3,
                                        4
                                      ],
                                      "x-enum-descriptions": [
                                        "CO_OWNER",
                                        "EDITOR",
                                        "COMMENTER",
                                        "VIEWER"
                                      ]
                                    }
                                  }
                                },
                                "advanced_share_setting": {
                                  "type": "object",
                                  "description": "Information about the whiteboard advanced share settings",
                                  "properties": {
                                    "can_share": {
                                      "type": "boolean",
                                      "description": "Indicate whether allow to share whiteboard or not."
                                    },
                                    "share_to_external": {
                                      "type": "boolean",
                                      "description": "Indicate whether allowed to share with user who is in the external organization or not."
                                    },
                                    "external_can_share": {
                                      "type": "boolean",
                                      "description": "Indicate whether the external user is allowed to share whiteboard or not."
                                    }
                                  }
                                }
                              }
                            }
                          },
                          "additionalProperties": true
                        }
                      },
                      "additionalProperties": true
                    }
                  },
                  "additionalProperties": true
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n    \"event\":\"whiteboard.share_setting_updated\",\n    \"event_ts\":1626230691572,\n    \"payload\":{\n        \"account_id\":\"AAAAAABBBB\",\n        \"operator\":\"admin@example.com\",\n        \"operator_id\":\"z8yCxjabcdEFGHfp8uQ\",\n        \"object\":{\n            \"whiteboard_id\":\"BK0R3Lg-Toi92viKHhzUVQ\",\n            \"settings\":{\n                \"share_link_setting\":{\n                    \"share_scope\":0,\n                    \"share_role\":4\n                },\n                \"advanced_share_setting\":{\n                    \"can_share\":true,\n                    \"share_to_external\":true,\n                    \"external_can_share\":false\n                }\n            }\n        }\n    }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "whiteboard.archive_available": {
      "post": {
        "tags": [
          "whiteboard"
        ],
        "operationId": "whiteboard.archive_available",
        "requestBody": {
          "description": "# whiteboard.archive_available\nThe **Whiteboard archive was available** event is emitted whenever a whiteboard archived sessions is successfully generated. This event can be used to notify subscribers when an archive is ready to be accessed.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Whiteboard archive was available** subscription enabled under the **Whiteboard** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_content:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:session:admin`\n\n**Event type**: `whiteboard.archive_available`\n",
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
                    "description": "A timestamp at which the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who owns the session.",
                        "example": "AAAAAABBBB"
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the whiteboard.",
                        "required": [
                          "session_id",
                          "start_time",
                          "end_time",
                          "whiteboard_id",
                          "whiteboard_name",
                          "expire_time"
                        ],
                        "properties": {
                          "session_id": {
                            "type": "string",
                            "description": "The unique ID of the session. This ID can be used to retrieve all activities under the session via the [List whiteboard sessions activities](https://developers.zoom.us/docs/api/rest/reference/whiteboard/methods/#operation/Listwhiteboardsessionsarchivedfiles) API.",
                            "example": "a2f19f96-9294-4f51-8134-6f0eea108eb2"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The session's start time in UTC/GMT, formatted as yyyy-MM-dd'T'HH:mm:ssZ.",
                            "format": "date-time",
                            "example": "2021-03-11T05:41:36Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The session's end time in UTC/GMT, formatted as yyyy-MM-dd'T'HH:mm:ssZ.",
                            "format": "date-time",
                            "example": "2021-03-11T05:41:36Z"
                          },
                          "whiteboard_id": {
                            "type": "string",
                            "description": "The ID of whiteboard.",
                            "example": "HY8-AC_HSou4SXY1S-OmOg"
                          },
                          "whiteboard_name": {
                            "type": "string",
                            "description": "The name of whiteboard.",
                            "example": "example_whiteboard_name"
                          },
                          "expire_time": {
                            "type": "string",
                            "description": "The expiration time of the archived content in UTC/GMT. After this time, the archived content will be permanently deleted. The value is formatted as yyyy-MM-dd'T'HH:mm:ssZ.",
                            "format": "date-time",
                            "example": "2021-03-26T06:41:36Z"
                          }
                        }
                      }
                    },
                    "additionalProperties": true
                  }
                },
                "additionalProperties": true
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"whiteboard.archive_available\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"session_id\": \"a2f19f96-9294-4f51-8134-6f0eea108eb2\",\n      \"start_time\": \"2021-03-11T05:41:36Z\",\n      \"end_time\": \"2021-03-11T05:41:36Z\",\n      \"whiteboard_id\": \"1234567890\",\n      \"whiteboard_name\": \"Q3 Plan\",\n      \"expire_time\": \"2021-03-26T06:41:36Z\"\n    }\n  }\n}"
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
                      "description": "A timestamp at which the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who owns the session.",
                          "example": "AAAAAABBBB"
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the whiteboard.",
                          "required": [
                            "session_id",
                            "start_time",
                            "end_time",
                            "whiteboard_id",
                            "whiteboard_name",
                            "expire_time"
                          ],
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "description": "The unique ID of the session. This ID can be used to retrieve all activities under the session via the [List whiteboard sessions activities](https://developers.zoom.us/docs/api/rest/reference/whiteboard/methods/#operation/Listwhiteboardsessionsarchivedfiles) API.",
                              "example": "a2f19f96-9294-4f51-8134-6f0eea108eb2"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The session's start time in UTC/GMT, formatted as yyyy-MM-dd'T'HH:mm:ssZ.",
                              "format": "date-time",
                              "example": "2021-03-11T05:41:36Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The session's end time in UTC/GMT, formatted as yyyy-MM-dd'T'HH:mm:ssZ.",
                              "format": "date-time",
                              "example": "2021-03-11T05:41:36Z"
                            },
                            "whiteboard_id": {
                              "type": "string",
                              "description": "The ID of whiteboard.",
                              "example": "HY8-AC_HSou4SXY1S-OmOg"
                            },
                            "whiteboard_name": {
                              "type": "string",
                              "description": "The name of whiteboard.",
                              "example": "example_whiteboard_name"
                            },
                            "expire_time": {
                              "type": "string",
                              "description": "The expiration time of the archived content in UTC/GMT. After this time, the archived content will be permanently deleted. The value is formatted as yyyy-MM-dd'T'HH:mm:ssZ.",
                              "format": "date-time",
                              "example": "2021-03-26T06:41:36Z"
                            }
                          }
                        }
                      },
                      "additionalProperties": true
                    }
                  },
                  "additionalProperties": true
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"whiteboard.archive_available\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"session_id\": \"a2f19f96-9294-4f51-8134-6f0eea108eb2\",\n      \"start_time\": \"2021-03-11T05:41:36Z\",\n      \"end_time\": \"2021-03-11T05:41:36Z\",\n      \"whiteboard_id\": \"1234567890\",\n      \"whiteboard_name\": \"Q3 Plan\",\n      \"expire_time\": \"2021-03-26T06:41:36Z\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "whiteboard.opened": {
      "post": {
        "tags": [
          "whiteboard"
        ],
        "operationId": "whiteboard.opened",
        "requestBody": {
          "description": "# whiteboard.opened\n\nThe **Whiteboard was opened** event is triggered every time a whiteboard is opened.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Whiteboard was opened** subscription enabled under the **Whiteboard** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:admin`,`whiteboard:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:whiteboard`,`whiteboard:read:whiteboard:admin`\n\n**Event type**: `whiteboard.opened`\n",
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
                    "description": "A timestamp at which the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "operator_collaborator_id",
                      "object"
                    ],
                    "properties": {
                      "operator_id": {
                        "type": "string",
                        "description": "nullable The Zoom user ID of the operator who opened the whiteboard. It should be null for external user(the user is external to the whiteboard's organization)."
                      },
                      "operator_collaborator_id": {
                        "type": "string",
                        "description": "not nullable The whiteboard collaborator's universally unique identifier of the operator who opened the whiteboard. Each collaborator will have a ID in a whiteboard."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the whiteboard.",
                        "required": [
                          "whiteboard_id",
                          "whiteboard_name"
                        ],
                        "properties": {
                          "whiteboard_id": {
                            "type": "string",
                            "description": "The whiteboard ID."
                          },
                          "whiteboard_name": {
                            "type": "string",
                            "description": "The whiteboard name."
                          },
                          "meeting_id": {
                            "type": "string",
                            "description": "The meeting ID. This response only returns the whiteboard opened in a meeting."
                          },
                          "meeting_uuid": {
                            "type": "string",
                            "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID. This response only returns the whiteboard opened in a meeting."
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
                  "value": "{\n  \"event\": \"whiteboard.opened\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operator_collaborator_id\": \"Cvftg6789087654443lkhk\",\n    \"object\": {\n      \"whiteboard_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"whiteboard_name\": \"Q3 Plan\",\n      \"meeting_id\": \"1234567890\",\n      \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\"\n    }\n  }\n}"
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
                      "description": "A timestamp at which the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "operator_collaborator_id",
                        "object"
                      ],
                      "properties": {
                        "operator_id": {
                          "type": "string",
                          "description": "nullable The Zoom user ID of the operator who opened the whiteboard. It should be null for external user(the user is external to the whiteboard's organization)."
                        },
                        "operator_collaborator_id": {
                          "type": "string",
                          "description": "not nullable The whiteboard collaborator's universally unique identifier of the operator who opened the whiteboard. Each collaborator will have a ID in a whiteboard."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the whiteboard.",
                          "required": [
                            "whiteboard_id",
                            "whiteboard_name"
                          ],
                          "properties": {
                            "whiteboard_id": {
                              "type": "string",
                              "description": "The whiteboard ID."
                            },
                            "whiteboard_name": {
                              "type": "string",
                              "description": "The whiteboard name."
                            },
                            "meeting_id": {
                              "type": "string",
                              "description": "The meeting ID. This response only returns the whiteboard opened in a meeting."
                            },
                            "meeting_uuid": {
                              "type": "string",
                              "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID. This response only returns the whiteboard opened in a meeting."
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
                    "value": "{\n  \"event\": \"whiteboard.opened\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operator_collaborator_id\": \"Cvftg6789087654443lkhk\",\n    \"object\": {\n      \"whiteboard_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"whiteboard_name\": \"Q3 Plan\",\n      \"meeting_id\": \"1234567890\",\n      \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "whiteboard.file_added": {
      "post": {
        "tags": [
          "whiteboard"
        ],
        "operationId": "whiteboard.file_added",
        "requestBody": {
          "description": "# whiteboard.file_added\nThe **Whiteboard file was added**  event is triggered when a file is successfully added to a whiteboard.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Whiteboard file was added** subscription enabled under the **Whiteboard** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_file:read`,`whiteboard_file:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:file:admin`,`whiteboard:read:file`\n\n**Event type**: `whiteboard.file_added`\n",
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
                    "description": "The event name."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "The timestamp when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "operator_collaborator_id",
                      "operator_name",
                      "object"
                    ],
                    "properties": {
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the uploader. Null if the user is external to the whiteboard owner's organization."
                      },
                      "operator_collaborator_id": {
                        "type": "string",
                        "description": "not nullable The whiteboard collaborator's universally unique identifier of the operator who uploaded the file.. Each collaborator will have a ID in a whiteboard."
                      },
                      "operator_name": {
                        "type": "string",
                        "description": "The name of the user who uploaded the file."
                      },
                      "object": {
                        "type": "object",
                        "description": "Details of the whiteboard where the file was uploaded.",
                        "required": [
                          "whiteboard_id",
                          "whiteboard_name",
                          "whiteboard_owner_name",
                          "whiteboard_file"
                        ],
                        "properties": {
                          "whiteboard_id": {
                            "type": "string",
                            "description": "The unique identifier of the whiteboard."
                          },
                          "whiteboard_name": {
                            "type": "string",
                            "description": "The name of the whiteboard."
                          },
                          "whiteboard_owner_name": {
                            "type": "string",
                            "description": "The name of the whiteboard owner."
                          },
                          "whiteboard_file": {
                            "type": "object",
                            "description": "Metadata of the file uploaded to the whiteboard.",
                            "required": [
                              "file_id",
                              "file_name",
                              "file_size",
                              "file_type",
                              "uploaded_ts"
                            ],
                            "properties": {
                              "file_id": {
                                "type": "string",
                                "description": "The unique ID of the file in the whiteboard."
                              },
                              "file_name": {
                                "type": "string",
                                "description": "The name of the uploaded file."
                              },
                              "file_size": {
                                "type": "integer",
                                "description": "The size of the uploaded file, in bytes."
                              },
                              "file_type": {
                                "type": "string",
                                "description": "The MIME type or format of the uploaded file."
                              },
                              "uploaded_ts": {
                                "type": "integer",
                                "format": "int64",
                                "description": "The timestamp when the file was uploaded."
                              }
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
                  "value": "{\n  \"event\": \"whiteboard.file_added\",\n  \"event_ts\": 1722238455000,\n  \"payload\": {\n    \"operator_id\": \"user_789xyz\",\n    \"operator_collaborator_id\": \"collaborator_789xyz\",\n    \"operator_name\": \"Jane Doe\",\n    \"object\": {\n      \"whiteboard_id\": \"wb_123456789\",\n      \"whiteboard_name\": \"Project Kickoff Board\",\n      \"whiteboard_owner_name\": \"Alice Smith\",\n      \"whiteboard_file\": {\n        \"file_id\": \"file_abc123\",\n        \"file_name\": \"requirements.pdf\",\n        \"file_size\": 256000,\n        \"file_type\": \"application/pdf\",\n        \"uploaded_ts\": 1722238455000\n      }\n    }\n  }\n}"
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
                      "description": "The event name."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "The timestamp when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "operator_collaborator_id",
                        "operator_name",
                        "object"
                      ],
                      "properties": {
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the uploader. Null if the user is external to the whiteboard owner's organization."
                        },
                        "operator_collaborator_id": {
                          "type": "string",
                          "description": "not nullable The whiteboard collaborator's universally unique identifier of the operator who uploaded the file.. Each collaborator will have a ID in a whiteboard."
                        },
                        "operator_name": {
                          "type": "string",
                          "description": "The name of the user who uploaded the file."
                        },
                        "object": {
                          "type": "object",
                          "description": "Details of the whiteboard where the file was uploaded.",
                          "required": [
                            "whiteboard_id",
                            "whiteboard_name",
                            "whiteboard_owner_name",
                            "whiteboard_file"
                          ],
                          "properties": {
                            "whiteboard_id": {
                              "type": "string",
                              "description": "The unique identifier of the whiteboard."
                            },
                            "whiteboard_name": {
                              "type": "string",
                              "description": "The name of the whiteboard."
                            },
                            "whiteboard_owner_name": {
                              "type": "string",
                              "description": "The name of the whiteboard owner."
                            },
                            "whiteboard_file": {
                              "type": "object",
                              "description": "Metadata of the file uploaded to the whiteboard.",
                              "required": [
                                "file_id",
                                "file_name",
                                "file_size",
                                "file_type",
                                "uploaded_ts"
                              ],
                              "properties": {
                                "file_id": {
                                  "type": "string",
                                  "description": "The unique ID of the file in the whiteboard."
                                },
                                "file_name": {
                                  "type": "string",
                                  "description": "The name of the uploaded file."
                                },
                                "file_size": {
                                  "type": "integer",
                                  "description": "The size of the uploaded file, in bytes."
                                },
                                "file_type": {
                                  "type": "string",
                                  "description": "The MIME type or format of the uploaded file."
                                },
                                "uploaded_ts": {
                                  "type": "integer",
                                  "format": "int64",
                                  "description": "The timestamp when the file was uploaded."
                                }
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
                    "value": "{\n  \"event\": \"whiteboard.file_added\",\n  \"event_ts\": 1722238455000,\n  \"payload\": {\n    \"operator_id\": \"user_789xyz\",\n    \"operator_collaborator_id\": \"collaborator_789xyz\",\n    \"operator_name\": \"Jane Doe\",\n    \"object\": {\n      \"whiteboard_id\": \"wb_123456789\",\n      \"whiteboard_name\": \"Project Kickoff Board\",\n      \"whiteboard_owner_name\": \"Alice Smith\",\n      \"whiteboard_file\": {\n        \"file_id\": \"file_abc123\",\n        \"file_name\": \"requirements.pdf\",\n        \"file_size\": 256000,\n        \"file_type\": \"application/pdf\",\n        \"uploaded_ts\": 1722238455000\n      }\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "whiteboard.created": {
      "post": {
        "tags": [
          "whiteboard"
        ],
        "operationId": "whiteboard.created",
        "requestBody": {
          "description": "# whiteboard.created\n\nThe **Whiteboard was created** event is triggered every time a whiteboard is created by one of your app users or account users.\n\nZoom offers multiple methods for users to create a new whiteboard. Users can manually create whiteboards via the Zoom client or the [Zoom web portal](https://zoom.us/). Users can also programmatically create whiteboards via the [**Create a new whiteboard**] API.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Whiteboard was created** subscription enabled under the **Whiteboard** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:admin`,`whiteboard:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:whiteboard`,`whiteboard:read:whiteboard:admin`\n\n**Event type**: `whiteboard.created`\n",
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
                    "description": "A timestamp at which the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who created the whiteboard."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user who created the whiteboard."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user who created the whiteboard."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the whiteboard.",
                        "required": [
                          "whiteboard_id",
                          "whiteboard_name",
                          "whiteboard_type",
                          "non_cloud"
                        ],
                        "properties": {
                          "whiteboard_id": {
                            "type": "string",
                            "description": "The whiteboard ID."
                          },
                          "whiteboard_name": {
                            "type": "string",
                            "description": "The whiteboard name."
                          },
                          "whiteboard_type": {
                            "type": "integer",
                            "description": "whiteboard type.",
                            "enum": [
                              0
                            ]
                          },
                          "non_cloud": {
                            "type": "boolean",
                            "description": "Whether the whiteboard allows to be saved to cloud or not."
                          },
                          "meeting_id": {
                            "type": "string",
                            "description": "The meeting ID. This response only returns when the whiteboard created in a meeting."
                          },
                          "meeting_uuid": {
                            "type": "string",
                            "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID. This response only returns when the whiteboard created in a meeting."
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
                  "value": "{\n  \"event\": \"whiteboard.created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"whiteboard_id\": \"1234567890\",\n      \"whiteboard_name\": \"Q3 Plan\",\n      \"whiteboard_type\": 0,\n      \"non_cloud\": false,\n      \"meeting_id\": \"1234567890\",\n      \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\"\n    }\n  }\n}"
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
                      "description": "A timestamp at which the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who created the whiteboard."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user who created the whiteboard."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user who created the whiteboard."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the whiteboard.",
                          "required": [
                            "whiteboard_id",
                            "whiteboard_name",
                            "whiteboard_type",
                            "non_cloud"
                          ],
                          "properties": {
                            "whiteboard_id": {
                              "type": "string",
                              "description": "The whiteboard ID."
                            },
                            "whiteboard_name": {
                              "type": "string",
                              "description": "The whiteboard name."
                            },
                            "whiteboard_type": {
                              "type": "integer",
                              "description": "whiteboard type.",
                              "enum": [
                                0
                              ]
                            },
                            "non_cloud": {
                              "type": "boolean",
                              "description": "Whether the whiteboard allows to be saved to cloud or not."
                            },
                            "meeting_id": {
                              "type": "string",
                              "description": "The meeting ID. This response only returns when the whiteboard created in a meeting."
                            },
                            "meeting_uuid": {
                              "type": "string",
                              "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID. This response only returns when the whiteboard created in a meeting."
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
                    "value": "{\n  \"event\": \"whiteboard.created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"whiteboard_id\": \"1234567890\",\n      \"whiteboard_name\": \"Q3 Plan\",\n      \"whiteboard_type\": 0,\n      \"non_cloud\": false,\n      \"meeting_id\": \"1234567890\",\n      \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "whiteboard.deleted": {
      "post": {
        "tags": [
          "whiteboard"
        ],
        "operationId": "whiteboard.deleted",
        "requestBody": {
          "description": "# whiteboard.deleted\n\nThe **Whiteboard deleted** event is triggered every time a whiteboard is delete by one of your app users or account users.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Whiteboard deleted** subscription enabled under the **Whiteboard** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read`,`whiteboard:read:admin`,`whiteboard:read`,`whiteboard:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:whiteboard`,`whiteboard:read:whiteboard:admin`\n\n**Event type**: `whiteboard.deleted`\n",
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
                    "description": "A timestamp at which the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who deleted the whiteboard."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user who deleted the whiteboard."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user who deleted the whiteboard."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the whiteboard.",
                        "required": [
                          "whiteboard_id",
                          "whiteboard_name",
                          "whiteboard_type",
                          "non_cloud"
                        ],
                        "properties": {
                          "whiteboard_id": {
                            "type": "string",
                            "description": "The whiteboard ID."
                          },
                          "whiteboard_name": {
                            "type": "string",
                            "description": "The whiteboard name."
                          },
                          "whiteboard_type": {
                            "type": "integer",
                            "description": "whiteboard type.",
                            "enum": [
                              0
                            ]
                          },
                          "non_cloud": {
                            "type": "boolean",
                            "description": "Whether the whiteboard allows to be saved to cloud or not."
                          }
                        },
                        "additionalProperties": true
                      }
                    },
                    "additionalProperties": true
                  }
                },
                "additionalProperties": true
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"whiteboard.deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"whiteboard_id\": \"1234567890\",\n      \"whiteboard_name\": \"Q3 Plan\",\n      \"whiteboard_type\": 0,\n      \"non_cloud\":false\n    }\n  }\n}"
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
                      "description": "A timestamp at which the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who deleted the whiteboard."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user who deleted the whiteboard."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user who deleted the whiteboard."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the whiteboard.",
                          "required": [
                            "whiteboard_id",
                            "whiteboard_name",
                            "whiteboard_type",
                            "non_cloud"
                          ],
                          "properties": {
                            "whiteboard_id": {
                              "type": "string",
                              "description": "The whiteboard ID."
                            },
                            "whiteboard_name": {
                              "type": "string",
                              "description": "The whiteboard name."
                            },
                            "whiteboard_type": {
                              "type": "integer",
                              "description": "whiteboard type.",
                              "enum": [
                                0
                              ]
                            },
                            "non_cloud": {
                              "type": "boolean",
                              "description": "Whether the whiteboard allows to be saved to cloud or not."
                            }
                          },
                          "additionalProperties": true
                        }
                      },
                      "additionalProperties": true
                    }
                  },
                  "additionalProperties": true
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"whiteboard.deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"whiteboard_id\": \"1234567890\",\n      \"whiteboard_name\": \"Q3 Plan\",\n      \"whiteboard_type\": 0,\n      \"non_cloud\":false\n    }\n  }\n}"
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
    "title": "Whiteboard",
    "description": "The Zoom Whiteboard webhooks allow developers to receive events for [Zoom Whiteboard](https://developers.zoom.us/docs/zoom-whiteboard/) features.",
    "version": "1.0.0"
  },
  "webhooks": {
    "whiteboard.shared": {
      "post": {
        "tags": [
          "whiteboard"
        ],
        "operationId": "whiteboard.shared",
        "requestBody": {
          "description": "# whiteboard.shared\n\nThe **Whiteboard was shared** event is triggered every time a whiteboard is shared with a Zoom user or a team chat channel by one of your whiteboard collaborators.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Whiteboard was shared** subscription enabled under the **Whiteboard** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read`,`whiteboard:read:admin`,`whiteboard:read`,`whiteboard:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:list_collaborators`,`whiteboard:read:list_collaborators:admin`\n\n**Event type**: `whiteboard.shared`\n",
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
                    "description": "A timestamp at which the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "operator_collaborator_id",
                      "object"
                    ],
                    "properties": {
                      "operator_id": {
                        "type": "string",
                        "description": "nullable The Zoom user ID of the operator who added users or team chat channels to the whiteboard. It should be null for external user(the user is external to the whiteboard's organization)."
                      },
                      "operator_collaborator_id": {
                        "type": "string",
                        "description": "not nullable The unique ID of the collaborator who added users or team chat channels to the whiteboard. Each collaborator will have a ID in a whiteboard."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the whiteboard.",
                        "required": [
                          "whiteboard_id",
                          "collaborators"
                        ],
                        "properties": {
                          "whiteboard_id": {
                            "type": "string",
                            "description": "The whiteboard ID."
                          },
                          "meeting_id": {
                            "type": "string",
                            "description": "The meeting ID. This response only returns the whiteboard is shared in a meeting."
                          },
                          "meeting_uuid": {
                            "type": "string",
                            "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID. This response only returns the whiteboard is shared in a meeting."
                          },
                          "collaborators": {
                            "type": "array",
                            "description": "Information about the whiteboard collaborators.",
                            "items": {
                              "anyOf": [
                                {
                                  "type": "object",
                                  "required": [
                                    "collaborator_id",
                                    "collaborator_type",
                                    "role"
                                  ],
                                  "properties": {
                                    "collaborator_id": {
                                      "type": "string",
                                      "description": "The collaborator's unique ID."
                                    },
                                    "collaborator_name": {
                                      "type": "string",
                                      "description": "The collaborator's display name."
                                    },
                                    "user_id": {
                                      "type": "string",
                                      "description": "The collaborator's user ID. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. "
                                    },
                                    "email": {
                                      "type": "string",
                                      "format": "email",
                                      "description": "User email. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. See [Email address display rules](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#email-address) for details."
                                    },
                                    "channel_id": {
                                      "type": "string",
                                      "description": "The team chat channel ID you want to share with. This response only returns if the collaborator_type is 2. Otherwise it should be null."
                                    },
                                    "collaborator_type": {
                                      "type": "integer",
                                      "description": "Collaborator type. \nEnum: \n*0 - SAME_ACCOUNT_USER \n*1 - EXTERNAL_USER \n*2 - CHANNEL \n",
                                      "enum": [
                                        0,
                                        1,
                                        2
                                      ],
                                      "x-enum-descriptions": [
                                        "SAME_ACCOUNT_USER",
                                        "EXTERNAL_USER",
                                        "CHANNEL"
                                      ]
                                    },
                                    "role": {
                                      "type": "integer",
                                      "description": "Collaborator permission. \nEnum: \n*0 - OWNER \n*1 - CO_OWNER \n*2 - EDITOR \n*3 - COMMENTER \n*4 - VIEWER \n.",
                                      "enum": [
                                        0,
                                        1,
                                        2,
                                        3,
                                        4
                                      ],
                                      "x-enum-descriptions": [
                                        "OWNER",
                                        "CO_OWNER",
                                        "EDITOR",
                                        "COMMENTER",
                                        "VIEWER"
                                      ]
                                    }
                                  },
                                  "additionalProperties": false
                                }
                              ]
                            },
                            "additionalProperties": false
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
                  "value": "{\n  \"event\": \"whiteboard.shared\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operator_collaborator_id\": \"Cvftg6789087654443lkhk\",\n    \"object\": {\n      \"whiteboard_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"meeting_id\": \"1234567890\",\n      \"meeting_ uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n      \"collaborators\": [\n        {\n          \"collaborator_id\": \"x4g6hy789k0o878k\",\n          \"collaborator_name\": \"Jack\",\n          \"email\": \"user@example.com\",\n          \"user_id\": \"er54t67y8ui9o0p0\",\n          \"collaborator_type\": 0,\n          \"role\": 3\n        },\n        {\n          \"collaborator_id\": \"x4g6hy789k0o878k\",\n          \"collaborator_name\": \"ext***@exa***.com\",\n          \"collaborator_type\": 1,\n          \"role\": 3\n        },\n        {\n          \"collaborator_id\": \"df4g6hy789k0o878k\",\n          \"channel_id\": \"example@conference.xmpp.zoom.us\",\n          \"collaborator_name\": \"Channel A\",\n          \"collaborator_type\": 2,\n          \"role\": 3\n        }\n      ]\n    }\n  }\n}"
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
                      "description": "A timestamp at which the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "operator_collaborator_id",
                        "object"
                      ],
                      "properties": {
                        "operator_id": {
                          "type": "string",
                          "description": "nullable The Zoom user ID of the operator who added users or team chat channels to the whiteboard. It should be null for external user(the user is external to the whiteboard's organization)."
                        },
                        "operator_collaborator_id": {
                          "type": "string",
                          "description": "not nullable The unique ID of the collaborator who added users or team chat channels to the whiteboard. Each collaborator will have a ID in a whiteboard."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the whiteboard.",
                          "required": [
                            "whiteboard_id",
                            "collaborators"
                          ],
                          "properties": {
                            "whiteboard_id": {
                              "type": "string",
                              "description": "The whiteboard ID."
                            },
                            "meeting_id": {
                              "type": "string",
                              "description": "The meeting ID. This response only returns the whiteboard is shared in a meeting."
                            },
                            "meeting_uuid": {
                              "type": "string",
                              "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID. This response only returns the whiteboard is shared in a meeting."
                            },
                            "collaborators": {
                              "type": "array",
                              "description": "Information about the whiteboard collaborators.",
                              "items": {
                                "anyOf": [
                                  {
                                    "type": "object",
                                    "required": [
                                      "collaborator_id",
                                      "collaborator_type",
                                      "role"
                                    ],
                                    "properties": {
                                      "collaborator_id": {
                                        "type": "string",
                                        "description": "The collaborator's unique ID."
                                      },
                                      "collaborator_name": {
                                        "type": "string",
                                        "description": "The collaborator's display name."
                                      },
                                      "user_id": {
                                        "type": "string",
                                        "description": "The collaborator's user ID. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. "
                                      },
                                      "email": {
                                        "type": "string",
                                        "format": "email",
                                        "description": "User email. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. See [Email address display rules](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#email-address) for details."
                                      },
                                      "channel_id": {
                                        "type": "string",
                                        "description": "The team chat channel ID you want to share with. This response only returns if the collaborator_type is 2. Otherwise it should be null."
                                      },
                                      "collaborator_type": {
                                        "type": "integer",
                                        "description": "Collaborator type. \nEnum: \n*0 - SAME_ACCOUNT_USER \n*1 - EXTERNAL_USER \n*2 - CHANNEL \n",
                                        "enum": [
                                          0,
                                          1,
                                          2
                                        ],
                                        "x-enum-descriptions": [
                                          "SAME_ACCOUNT_USER",
                                          "EXTERNAL_USER",
                                          "CHANNEL"
                                        ]
                                      },
                                      "role": {
                                        "type": "integer",
                                        "description": "Collaborator permission. \nEnum: \n*0 - OWNER \n*1 - CO_OWNER \n*2 - EDITOR \n*3 - COMMENTER \n*4 - VIEWER \n.",
                                        "enum": [
                                          0,
                                          1,
                                          2,
                                          3,
                                          4
                                        ],
                                        "x-enum-descriptions": [
                                          "OWNER",
                                          "CO_OWNER",
                                          "EDITOR",
                                          "COMMENTER",
                                          "VIEWER"
                                        ]
                                      }
                                    },
                                    "additionalProperties": false
                                  }
                                ]
                              },
                              "additionalProperties": false
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
                    "value": "{\n  \"event\": \"whiteboard.shared\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operator_collaborator_id\": \"Cvftg6789087654443lkhk\",\n    \"object\": {\n      \"whiteboard_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"meeting_id\": \"1234567890\",\n      \"meeting_ uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n      \"collaborators\": [\n        {\n          \"collaborator_id\": \"x4g6hy789k0o878k\",\n          \"collaborator_name\": \"Jack\",\n          \"email\": \"user@example.com\",\n          \"user_id\": \"er54t67y8ui9o0p0\",\n          \"collaborator_type\": 0,\n          \"role\": 3\n        },\n        {\n          \"collaborator_id\": \"x4g6hy789k0o878k\",\n          \"collaborator_name\": \"ext***@exa***.com\",\n          \"collaborator_type\": 1,\n          \"role\": 3\n        },\n        {\n          \"collaborator_id\": \"df4g6hy789k0o878k\",\n          \"channel_id\": \"example@conference.xmpp.zoom.us\",\n          \"collaborator_name\": \"Channel A\",\n          \"collaborator_type\": 2,\n          \"role\": 3\n        }\n      ]\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "whiteboard.sharing_updated": {
      "post": {
        "tags": [
          "whiteboard"
        ],
        "operationId": "whiteboard.sharing_updated",
        "requestBody": {
          "description": "# whiteboard.sharing_updated\n\nThe **Whiteboard collaborator's sharing settings were updated** event is triggered every time the of a whiteboard collaborator is updated or removed by one of your whiteboard owners.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Whiteboard collaborator's sharing settings were updated** subscription enabled under the **Whiteboard** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read`,`whiteboard:read:admin`,`whiteboard:read`,`whiteboard:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:list_collaborators`,`whiteboard:read:list_collaborators:admin`\n\n**Event type**: `whiteboard.sharing_updated`\n",
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
                    "description": "A timestamp at which the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "operator_collaborator_id",
                      "operation_type",
                      "object"
                    ],
                    "properties": {
                      "operator_id": {
                        "type": "string",
                        "description": "The Zoom user ID of the operator who changed the permission of collaborators of the whiteboard. It should be whiteboard owner user ID."
                      },
                      "operator_collaborator_id": {
                        "type": "string",
                        "description": "not nullable The whiteboard collaborator's universally unique identifier. Each collaborator will have a ID in a whiteboard."
                      },
                      "operation_type": {
                        "type": "integer",
                        "description": "Operation type. \nEnum: \n*0 - REMOVE \n*1 - UPDATE \n",
                        "enum": [
                          0,
                          1
                        ],
                        "x-enum-descriptions": [
                          "REMOVE",
                          "UPDATE"
                        ]
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the whiteboard collaborators.",
                        "required": [
                          "whiteboard_id",
                          "collaborators"
                        ],
                        "properties": {
                          "whiteboard_id": {
                            "type": "string",
                            "description": "The whiteboard ID."
                          },
                          "collaborators": {
                            "type": "array",
                            "description": "Information about the effected whiteboard collaborators.",
                            "items": {
                              "anyOf": [
                                {
                                  "type": "object",
                                  "required": [
                                    "collaborator_id",
                                    "collaborator_type",
                                    "role"
                                  ],
                                  "properties": {
                                    "collaborator_id": {
                                      "type": "string",
                                      "description": "The collaborator's ID. \n* If collaborator_type set to 0, this field refers to a user ID. If collaborator_type set to 2, this field refers to a team chat channel ID. If collaborator_type set to 1, this returns an empty value."
                                    },
                                    "collaborator_name": {
                                      "type": "string",
                                      "description": "The collaborator's display name."
                                    },
                                    "user_id": {
                                      "type": "string",
                                      "description": "The collaborator's user ID. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. "
                                    },
                                    "email": {
                                      "type": "string",
                                      "description": "User email. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. See [Email address display rules](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#email-address) for details."
                                    },
                                    "channel_id": {
                                      "type": "string",
                                      "description": "The team chat channel ID you want to share with. This response only returns if the collaborator_type is 2. Otherwise it should be null."
                                    },
                                    "collaborator_type": {
                                      "type": "integer",
                                      "description": "Collaborator type. \nEnum: \n*0 - SAME_ACCOUNT_USER \n*1 - EXTERNAL_USER \n*2 - CHANNEL \n",
                                      "enum": [
                                        0,
                                        1,
                                        2
                                      ],
                                      "x-enum-descriptions": [
                                        "SAME_ACCOUNT_USER",
                                        "EXTERNAL_USER",
                                        "CHANNEL"
                                      ]
                                    },
                                    "role": {
                                      "type": "integer",
                                      "description": "Collaborator permission. \nEnum: \n*0 - OWNER \n*1 - CO_OWNER \n*2 - EDITOR \n*3 - COMMENTER \n*4 - VIEWER \n.",
                                      "enum": [
                                        0,
                                        1,
                                        2,
                                        3,
                                        4
                                      ],
                                      "x-enum-descriptions": [
                                        "OWNER",
                                        "CO_OWNER",
                                        "EDITOR",
                                        "COMMENTER",
                                        "VIEWER"
                                      ]
                                    }
                                  },
                                  "additionalProperties": false
                                }
                              ]
                            },
                            "additionalProperties": false
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
                  "value": "{\n  \"event\": \"whiteboard.sharing_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operation_type\": 1,\n    \"operator_collaborator_id\": \"Cvftg6789087654443lkhk\",\n    \"object\": {\n      \"whiteboard_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"collaborators\": [\n        {\n          \"collaborator_id\": \"as35hjnh89899fmkl0ou\",\n          \"collaborator_name\": \"Jack\",\n          \"user_id\": \"er54t67y8ui9o0p0\",\n          \"email\": \"user@example.com\",\n          \"collaborator_type\": 0,\n          \"role\": 2\n        },\n        {\n          \"collaborator_id\": \"x4g6hy789k0o878k\",\n          \"collaborator_name\": \"ext***@exa***.com\",\n          \"collaborator_type\": 1,\n          \"role\": 3\n        },\n        {\n          \"collaborator_id\": \"34g6hy789k0o878k\",\n          \"channel_id\": \"example@conference.xmpp.zoom.us\",\n          \"collaborator_name\": \"Channel A\",\n          \"collaborator_type\": 2,\n          \"role\": 2\n        }\n      ]\n    }\n  }\n}"
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
                      "description": "A timestamp at which the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "operator_collaborator_id",
                        "operation_type",
                        "object"
                      ],
                      "properties": {
                        "operator_id": {
                          "type": "string",
                          "description": "The Zoom user ID of the operator who changed the permission of collaborators of the whiteboard. It should be whiteboard owner user ID."
                        },
                        "operator_collaborator_id": {
                          "type": "string",
                          "description": "not nullable The whiteboard collaborator's universally unique identifier. Each collaborator will have a ID in a whiteboard."
                        },
                        "operation_type": {
                          "type": "integer",
                          "description": "Operation type. \nEnum: \n*0 - REMOVE \n*1 - UPDATE \n",
                          "enum": [
                            0,
                            1
                          ],
                          "x-enum-descriptions": [
                            "REMOVE",
                            "UPDATE"
                          ]
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the whiteboard collaborators.",
                          "required": [
                            "whiteboard_id",
                            "collaborators"
                          ],
                          "properties": {
                            "whiteboard_id": {
                              "type": "string",
                              "description": "The whiteboard ID."
                            },
                            "collaborators": {
                              "type": "array",
                              "description": "Information about the effected whiteboard collaborators.",
                              "items": {
                                "anyOf": [
                                  {
                                    "type": "object",
                                    "required": [
                                      "collaborator_id",
                                      "collaborator_type",
                                      "role"
                                    ],
                                    "properties": {
                                      "collaborator_id": {
                                        "type": "string",
                                        "description": "The collaborator's ID. \n* If collaborator_type set to 0, this field refers to a user ID. If collaborator_type set to 2, this field refers to a team chat channel ID. If collaborator_type set to 1, this returns an empty value."
                                      },
                                      "collaborator_name": {
                                        "type": "string",
                                        "description": "The collaborator's display name."
                                      },
                                      "user_id": {
                                        "type": "string",
                                        "description": "The collaborator's user ID. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. "
                                      },
                                      "email": {
                                        "type": "string",
                                        "description": "User email. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. See [Email address display rules](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#email-address) for details."
                                      },
                                      "channel_id": {
                                        "type": "string",
                                        "description": "The team chat channel ID you want to share with. This response only returns if the collaborator_type is 2. Otherwise it should be null."
                                      },
                                      "collaborator_type": {
                                        "type": "integer",
                                        "description": "Collaborator type. \nEnum: \n*0 - SAME_ACCOUNT_USER \n*1 - EXTERNAL_USER \n*2 - CHANNEL \n",
                                        "enum": [
                                          0,
                                          1,
                                          2
                                        ],
                                        "x-enum-descriptions": [
                                          "SAME_ACCOUNT_USER",
                                          "EXTERNAL_USER",
                                          "CHANNEL"
                                        ]
                                      },
                                      "role": {
                                        "type": "integer",
                                        "description": "Collaborator permission. \nEnum: \n*0 - OWNER \n*1 - CO_OWNER \n*2 - EDITOR \n*3 - COMMENTER \n*4 - VIEWER \n.",
                                        "enum": [
                                          0,
                                          1,
                                          2,
                                          3,
                                          4
                                        ],
                                        "x-enum-descriptions": [
                                          "OWNER",
                                          "CO_OWNER",
                                          "EDITOR",
                                          "COMMENTER",
                                          "VIEWER"
                                        ]
                                      }
                                    },
                                    "additionalProperties": false
                                  }
                                ]
                              },
                              "additionalProperties": false
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
                    "value": "{\n  \"event\": \"whiteboard.sharing_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operation_type\": 1,\n    \"operator_collaborator_id\": \"Cvftg6789087654443lkhk\",\n    \"object\": {\n      \"whiteboard_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"collaborators\": [\n        {\n          \"collaborator_id\": \"as35hjnh89899fmkl0ou\",\n          \"collaborator_name\": \"Jack\",\n          \"user_id\": \"er54t67y8ui9o0p0\",\n          \"email\": \"user@example.com\",\n          \"collaborator_type\": 0,\n          \"role\": 2\n        },\n        {\n          \"collaborator_id\": \"x4g6hy789k0o878k\",\n          \"collaborator_name\": \"ext***@exa***.com\",\n          \"collaborator_type\": 1,\n          \"role\": 3\n        },\n        {\n          \"collaborator_id\": \"34g6hy789k0o878k\",\n          \"channel_id\": \"example@conference.xmpp.zoom.us\",\n          \"collaborator_name\": \"Channel A\",\n          \"collaborator_type\": 2,\n          \"role\": 2\n        }\n      ]\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "whiteboard.share_settings_updated": {
      "post": {
        "tags": [
          "whiteboard"
        ],
        "operationId": "whiteboard.share_settings_updated",
        "requestBody": {
          "description": "# whiteboard.share_settings_updated\n\nWhiteboard share settings can be updated by a whiteboard owner or co-owners.\n\nThe **Whiteboard's sharing setting were updated** event is triggered every time a whiteboard share setting is updated by whiteboard owner or co-owners.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Whiteboard's sharing setting were updated** subscription enabled under the **Whiteboard** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read`,`whiteboard:read:admin`,`whiteboard:read`,`whiteboard:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:share_setting`,`whiteboard:read:share_setting:admin`\n\n**Event type**: `whiteboard.share_settings_updated`\n",
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
                    "description": "A timestamp at which the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who updated the whiteboard share settings."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user who updated the whiteboard share settings."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user who updated the whiteboard share settings."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the whiteboard.",
                        "required": [
                          "whiteboard_id",
                          "settings"
                        ],
                        "properties": {
                          "whiteboard_id": {
                            "type": "string",
                            "description": "The whiteboard ID."
                          },
                          "settings": {
                            "type": "object",
                            "description": "Information about the whiteboard share settings.",
                            "properties": {
                              "share_link_setting": {
                                "type": "object",
                                "description": "Information about the whiteboard share link settings.",
                                "properties": {
                                  "share_scope": {
                                    "type": "integer",
                                    "description": "Who can access the share link",
                                    "enum": [
                                      0,
                                      1,
                                      2
                                    ],
                                    "x-enum-descriptions": [
                                      "Only invited members can access this board",
                                      "Anyone at the current organization",
                                      "Anyone with the link can view this board"
                                    ]
                                  },
                                  "share_role": {
                                    "type": "integer",
                                    "description": "Permission of the shared user.",
                                    "enum": [
                                      1,
                                      2,
                                      3,
                                      4
                                    ],
                                    "x-enum-descriptions": [
                                      "CO_OWNER",
                                      "EDITOR",
                                      "COMMENTER",
                                      "VIEWER"
                                    ]
                                  }
                                }
                              },
                              "advanced_share_setting": {
                                "type": "object",
                                "description": "Information about the whiteboard advanced share settings",
                                "properties": {
                                  "can_share": {
                                    "type": "boolean",
                                    "description": "Indicate whether allow to share whiteboard or not."
                                  },
                                  "share_to_external": {
                                    "type": "boolean",
                                    "description": "Indicate whether allowed to share with user who is in the external organization or not."
                                  },
                                  "external_can_share": {
                                    "type": "boolean",
                                    "description": "Indicate whether the external user is allowed to share whiteboard or not."
                                  }
                                }
                              }
                            }
                          }
                        },
                        "additionalProperties": true
                      }
                    },
                    "additionalProperties": true
                  }
                },
                "additionalProperties": true
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n    \"event\":\"whiteboard.share_setting_updated\",\n    \"event_ts\":1626230691572,\n    \"payload\":{\n        \"account_id\":\"AAAAAABBBB\",\n        \"operator\":\"admin@example.com\",\n        \"operator_id\":\"z8yCxjabcdEFGHfp8uQ\",\n        \"object\":{\n            \"whiteboard_id\":\"BK0R3Lg-Toi92viKHhzUVQ\",\n            \"settings\":{\n                \"share_link_setting\":{\n                    \"share_scope\":0,\n                    \"share_role\":4\n                },\n                \"advanced_share_setting\":{\n                    \"can_share\":true,\n                    \"share_to_external\":true,\n                    \"external_can_share\":false\n                }\n            }\n        }\n    }\n}"
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
                      "description": "A timestamp at which the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who updated the whiteboard share settings."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user who updated the whiteboard share settings."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user who updated the whiteboard share settings."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the whiteboard.",
                          "required": [
                            "whiteboard_id",
                            "settings"
                          ],
                          "properties": {
                            "whiteboard_id": {
                              "type": "string",
                              "description": "The whiteboard ID."
                            },
                            "settings": {
                              "type": "object",
                              "description": "Information about the whiteboard share settings.",
                              "properties": {
                                "share_link_setting": {
                                  "type": "object",
                                  "description": "Information about the whiteboard share link settings.",
                                  "properties": {
                                    "share_scope": {
                                      "type": "integer",
                                      "description": "Who can access the share link",
                                      "enum": [
                                        0,
                                        1,
                                        2
                                      ],
                                      "x-enum-descriptions": [
                                        "Only invited members can access this board",
                                        "Anyone at the current organization",
                                        "Anyone with the link can view this board"
                                      ]
                                    },
                                    "share_role": {
                                      "type": "integer",
                                      "description": "Permission of the shared user.",
                                      "enum": [
                                        1,
                                        2,
                                        3,
                                        4
                                      ],
                                      "x-enum-descriptions": [
                                        "CO_OWNER",
                                        "EDITOR",
                                        "COMMENTER",
                                        "VIEWER"
                                      ]
                                    }
                                  }
                                },
                                "advanced_share_setting": {
                                  "type": "object",
                                  "description": "Information about the whiteboard advanced share settings",
                                  "properties": {
                                    "can_share": {
                                      "type": "boolean",
                                      "description": "Indicate whether allow to share whiteboard or not."
                                    },
                                    "share_to_external": {
                                      "type": "boolean",
                                      "description": "Indicate whether allowed to share with user who is in the external organization or not."
                                    },
                                    "external_can_share": {
                                      "type": "boolean",
                                      "description": "Indicate whether the external user is allowed to share whiteboard or not."
                                    }
                                  }
                                }
                              }
                            }
                          },
                          "additionalProperties": true
                        }
                      },
                      "additionalProperties": true
                    }
                  },
                  "additionalProperties": true
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n    \"event\":\"whiteboard.share_setting_updated\",\n    \"event_ts\":1626230691572,\n    \"payload\":{\n        \"account_id\":\"AAAAAABBBB\",\n        \"operator\":\"admin@example.com\",\n        \"operator_id\":\"z8yCxjabcdEFGHfp8uQ\",\n        \"object\":{\n            \"whiteboard_id\":\"BK0R3Lg-Toi92viKHhzUVQ\",\n            \"settings\":{\n                \"share_link_setting\":{\n                    \"share_scope\":0,\n                    \"share_role\":4\n                },\n                \"advanced_share_setting\":{\n                    \"can_share\":true,\n                    \"share_to_external\":true,\n                    \"external_can_share\":false\n                }\n            }\n        }\n    }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "whiteboard.archive_available": {
      "post": {
        "tags": [
          "whiteboard"
        ],
        "operationId": "whiteboard.archive_available",
        "requestBody": {
          "description": "# whiteboard.archive_available\nThe **Whiteboard archive was available** event is emitted whenever a whiteboard archived sessions is successfully generated. This event can be used to notify subscribers when an archive is ready to be accessed.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Whiteboard archive was available** subscription enabled under the **Whiteboard** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_content:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:session:admin`\n\n**Event type**: `whiteboard.archive_available`\n",
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
                    "description": "A timestamp at which the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who owns the session.",
                        "example": "AAAAAABBBB"
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the whiteboard.",
                        "required": [
                          "session_id",
                          "start_time",
                          "end_time",
                          "whiteboard_id",
                          "whiteboard_name",
                          "expire_time"
                        ],
                        "properties": {
                          "session_id": {
                            "type": "string",
                            "description": "The unique ID of the session. This ID can be used to retrieve all activities under the session via the [List whiteboard sessions activities](https://developers.zoom.us/docs/api/rest/reference/whiteboard/methods/#operation/Listwhiteboardsessionsarchivedfiles) API.",
                            "example": "a2f19f96-9294-4f51-8134-6f0eea108eb2"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The session's start time in UTC/GMT, formatted as yyyy-MM-dd'T'HH:mm:ssZ.",
                            "format": "date-time",
                            "example": "2021-03-11T05:41:36Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The session's end time in UTC/GMT, formatted as yyyy-MM-dd'T'HH:mm:ssZ.",
                            "format": "date-time",
                            "example": "2021-03-11T05:41:36Z"
                          },
                          "whiteboard_id": {
                            "type": "string",
                            "description": "The ID of whiteboard.",
                            "example": "HY8-AC_HSou4SXY1S-OmOg"
                          },
                          "whiteboard_name": {
                            "type": "string",
                            "description": "The name of whiteboard.",
                            "example": "example_whiteboard_name"
                          },
                          "expire_time": {
                            "type": "string",
                            "description": "The expiration time of the archived content in UTC/GMT. After this time, the archived content will be permanently deleted. The value is formatted as yyyy-MM-dd'T'HH:mm:ssZ.",
                            "format": "date-time",
                            "example": "2021-03-26T06:41:36Z"
                          }
                        }
                      }
                    },
                    "additionalProperties": true
                  }
                },
                "additionalProperties": true
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"whiteboard.archive_available\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"session_id\": \"a2f19f96-9294-4f51-8134-6f0eea108eb2\",\n      \"start_time\": \"2021-03-11T05:41:36Z\",\n      \"end_time\": \"2021-03-11T05:41:36Z\",\n      \"whiteboard_id\": \"1234567890\",\n      \"whiteboard_name\": \"Q3 Plan\",\n      \"expire_time\": \"2021-03-26T06:41:36Z\"\n    }\n  }\n}"
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
                      "description": "A timestamp at which the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who owns the session.",
                          "example": "AAAAAABBBB"
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the whiteboard.",
                          "required": [
                            "session_id",
                            "start_time",
                            "end_time",
                            "whiteboard_id",
                            "whiteboard_name",
                            "expire_time"
                          ],
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "description": "The unique ID of the session. This ID can be used to retrieve all activities under the session via the [List whiteboard sessions activities](https://developers.zoom.us/docs/api/rest/reference/whiteboard/methods/#operation/Listwhiteboardsessionsarchivedfiles) API.",
                              "example": "a2f19f96-9294-4f51-8134-6f0eea108eb2"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The session's start time in UTC/GMT, formatted as yyyy-MM-dd'T'HH:mm:ssZ.",
                              "format": "date-time",
                              "example": "2021-03-11T05:41:36Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The session's end time in UTC/GMT, formatted as yyyy-MM-dd'T'HH:mm:ssZ.",
                              "format": "date-time",
                              "example": "2021-03-11T05:41:36Z"
                            },
                            "whiteboard_id": {
                              "type": "string",
                              "description": "The ID of whiteboard.",
                              "example": "HY8-AC_HSou4SXY1S-OmOg"
                            },
                            "whiteboard_name": {
                              "type": "string",
                              "description": "The name of whiteboard.",
                              "example": "example_whiteboard_name"
                            },
                            "expire_time": {
                              "type": "string",
                              "description": "The expiration time of the archived content in UTC/GMT. After this time, the archived content will be permanently deleted. The value is formatted as yyyy-MM-dd'T'HH:mm:ssZ.",
                              "format": "date-time",
                              "example": "2021-03-26T06:41:36Z"
                            }
                          }
                        }
                      },
                      "additionalProperties": true
                    }
                  },
                  "additionalProperties": true
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"whiteboard.archive_available\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"session_id\": \"a2f19f96-9294-4f51-8134-6f0eea108eb2\",\n      \"start_time\": \"2021-03-11T05:41:36Z\",\n      \"end_time\": \"2021-03-11T05:41:36Z\",\n      \"whiteboard_id\": \"1234567890\",\n      \"whiteboard_name\": \"Q3 Plan\",\n      \"expire_time\": \"2021-03-26T06:41:36Z\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "whiteboard.opened": {
      "post": {
        "tags": [
          "whiteboard"
        ],
        "operationId": "whiteboard.opened",
        "requestBody": {
          "description": "# whiteboard.opened\n\nThe **Whiteboard was opened** event is triggered every time a whiteboard is opened.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Whiteboard was opened** subscription enabled under the **Whiteboard** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:admin`,`whiteboard:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:whiteboard`,`whiteboard:read:whiteboard:admin`\n\n**Event type**: `whiteboard.opened`\n",
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
                    "description": "A timestamp at which the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "operator_collaborator_id",
                      "object"
                    ],
                    "properties": {
                      "operator_id": {
                        "type": "string",
                        "description": "nullable The Zoom user ID of the operator who opened the whiteboard. It should be null for external user(the user is external to the whiteboard's organization)."
                      },
                      "operator_collaborator_id": {
                        "type": "string",
                        "description": "not nullable The whiteboard collaborator's universally unique identifier of the operator who opened the whiteboard. Each collaborator will have a ID in a whiteboard."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the whiteboard.",
                        "required": [
                          "whiteboard_id",
                          "whiteboard_name"
                        ],
                        "properties": {
                          "whiteboard_id": {
                            "type": "string",
                            "description": "The whiteboard ID."
                          },
                          "whiteboard_name": {
                            "type": "string",
                            "description": "The whiteboard name."
                          },
                          "meeting_id": {
                            "type": "string",
                            "description": "The meeting ID. This response only returns the whiteboard opened in a meeting."
                          },
                          "meeting_uuid": {
                            "type": "string",
                            "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID. This response only returns the whiteboard opened in a meeting."
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
                  "value": "{\n  \"event\": \"whiteboard.opened\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operator_collaborator_id\": \"Cvftg6789087654443lkhk\",\n    \"object\": {\n      \"whiteboard_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"whiteboard_name\": \"Q3 Plan\",\n      \"meeting_id\": \"1234567890\",\n      \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\"\n    }\n  }\n}"
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
                      "description": "A timestamp at which the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "operator_collaborator_id",
                        "object"
                      ],
                      "properties": {
                        "operator_id": {
                          "type": "string",
                          "description": "nullable The Zoom user ID of the operator who opened the whiteboard. It should be null for external user(the user is external to the whiteboard's organization)."
                        },
                        "operator_collaborator_id": {
                          "type": "string",
                          "description": "not nullable The whiteboard collaborator's universally unique identifier of the operator who opened the whiteboard. Each collaborator will have a ID in a whiteboard."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the whiteboard.",
                          "required": [
                            "whiteboard_id",
                            "whiteboard_name"
                          ],
                          "properties": {
                            "whiteboard_id": {
                              "type": "string",
                              "description": "The whiteboard ID."
                            },
                            "whiteboard_name": {
                              "type": "string",
                              "description": "The whiteboard name."
                            },
                            "meeting_id": {
                              "type": "string",
                              "description": "The meeting ID. This response only returns the whiteboard opened in a meeting."
                            },
                            "meeting_uuid": {
                              "type": "string",
                              "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID. This response only returns the whiteboard opened in a meeting."
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
                    "value": "{\n  \"event\": \"whiteboard.opened\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operator_collaborator_id\": \"Cvftg6789087654443lkhk\",\n    \"object\": {\n      \"whiteboard_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"whiteboard_name\": \"Q3 Plan\",\n      \"meeting_id\": \"1234567890\",\n      \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "whiteboard.file_added": {
      "post": {
        "tags": [
          "whiteboard"
        ],
        "operationId": "whiteboard.file_added",
        "requestBody": {
          "description": "# whiteboard.file_added\nThe **Whiteboard file was added**  event is triggered when a file is successfully added to a whiteboard.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Whiteboard file was added** subscription enabled under the **Whiteboard** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_file:read`,`whiteboard_file:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:file:admin`,`whiteboard:read:file`\n\n**Event type**: `whiteboard.file_added`\n",
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
                    "description": "The event name."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "The timestamp when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "operator_collaborator_id",
                      "operator_name",
                      "object"
                    ],
                    "properties": {
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the uploader. Null if the user is external to the whiteboard owner's organization."
                      },
                      "operator_collaborator_id": {
                        "type": "string",
                        "description": "not nullable The whiteboard collaborator's universally unique identifier of the operator who uploaded the file.. Each collaborator will have a ID in a whiteboard."
                      },
                      "operator_name": {
                        "type": "string",
                        "description": "The name of the user who uploaded the file."
                      },
                      "object": {
                        "type": "object",
                        "description": "Details of the whiteboard where the file was uploaded.",
                        "required": [
                          "whiteboard_id",
                          "whiteboard_name",
                          "whiteboard_owner_name",
                          "whiteboard_file"
                        ],
                        "properties": {
                          "whiteboard_id": {
                            "type": "string",
                            "description": "The unique identifier of the whiteboard."
                          },
                          "whiteboard_name": {
                            "type": "string",
                            "description": "The name of the whiteboard."
                          },
                          "whiteboard_owner_name": {
                            "type": "string",
                            "description": "The name of the whiteboard owner."
                          },
                          "whiteboard_file": {
                            "type": "object",
                            "description": "Metadata of the file uploaded to the whiteboard.",
                            "required": [
                              "file_id",
                              "file_name",
                              "file_size",
                              "file_type",
                              "uploaded_ts"
                            ],
                            "properties": {
                              "file_id": {
                                "type": "string",
                                "description": "The unique ID of the file in the whiteboard."
                              },
                              "file_name": {
                                "type": "string",
                                "description": "The name of the uploaded file."
                              },
                              "file_size": {
                                "type": "integer",
                                "description": "The size of the uploaded file, in bytes."
                              },
                              "file_type": {
                                "type": "string",
                                "description": "The MIME type or format of the uploaded file."
                              },
                              "uploaded_ts": {
                                "type": "integer",
                                "format": "int64",
                                "description": "The timestamp when the file was uploaded."
                              }
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
                  "value": "{\n  \"event\": \"whiteboard.file_added\",\n  \"event_ts\": 1722238455000,\n  \"payload\": {\n    \"operator_id\": \"user_789xyz\",\n    \"operator_collaborator_id\": \"collaborator_789xyz\",\n    \"operator_name\": \"Jane Doe\",\n    \"object\": {\n      \"whiteboard_id\": \"wb_123456789\",\n      \"whiteboard_name\": \"Project Kickoff Board\",\n      \"whiteboard_owner_name\": \"Alice Smith\",\n      \"whiteboard_file\": {\n        \"file_id\": \"file_abc123\",\n        \"file_name\": \"requirements.pdf\",\n        \"file_size\": 256000,\n        \"file_type\": \"application/pdf\",\n        \"uploaded_ts\": 1722238455000\n      }\n    }\n  }\n}"
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
                      "description": "The event name."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "The timestamp when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "operator_collaborator_id",
                        "operator_name",
                        "object"
                      ],
                      "properties": {
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the uploader. Null if the user is external to the whiteboard owner's organization."
                        },
                        "operator_collaborator_id": {
                          "type": "string",
                          "description": "not nullable The whiteboard collaborator's universally unique identifier of the operator who uploaded the file.. Each collaborator will have a ID in a whiteboard."
                        },
                        "operator_name": {
                          "type": "string",
                          "description": "The name of the user who uploaded the file."
                        },
                        "object": {
                          "type": "object",
                          "description": "Details of the whiteboard where the file was uploaded.",
                          "required": [
                            "whiteboard_id",
                            "whiteboard_name",
                            "whiteboard_owner_name",
                            "whiteboard_file"
                          ],
                          "properties": {
                            "whiteboard_id": {
                              "type": "string",
                              "description": "The unique identifier of the whiteboard."
                            },
                            "whiteboard_name": {
                              "type": "string",
                              "description": "The name of the whiteboard."
                            },
                            "whiteboard_owner_name": {
                              "type": "string",
                              "description": "The name of the whiteboard owner."
                            },
                            "whiteboard_file": {
                              "type": "object",
                              "description": "Metadata of the file uploaded to the whiteboard.",
                              "required": [
                                "file_id",
                                "file_name",
                                "file_size",
                                "file_type",
                                "uploaded_ts"
                              ],
                              "properties": {
                                "file_id": {
                                  "type": "string",
                                  "description": "The unique ID of the file in the whiteboard."
                                },
                                "file_name": {
                                  "type": "string",
                                  "description": "The name of the uploaded file."
                                },
                                "file_size": {
                                  "type": "integer",
                                  "description": "The size of the uploaded file, in bytes."
                                },
                                "file_type": {
                                  "type": "string",
                                  "description": "The MIME type or format of the uploaded file."
                                },
                                "uploaded_ts": {
                                  "type": "integer",
                                  "format": "int64",
                                  "description": "The timestamp when the file was uploaded."
                                }
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
                    "value": "{\n  \"event\": \"whiteboard.file_added\",\n  \"event_ts\": 1722238455000,\n  \"payload\": {\n    \"operator_id\": \"user_789xyz\",\n    \"operator_collaborator_id\": \"collaborator_789xyz\",\n    \"operator_name\": \"Jane Doe\",\n    \"object\": {\n      \"whiteboard_id\": \"wb_123456789\",\n      \"whiteboard_name\": \"Project Kickoff Board\",\n      \"whiteboard_owner_name\": \"Alice Smith\",\n      \"whiteboard_file\": {\n        \"file_id\": \"file_abc123\",\n        \"file_name\": \"requirements.pdf\",\n        \"file_size\": 256000,\n        \"file_type\": \"application/pdf\",\n        \"uploaded_ts\": 1722238455000\n      }\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "whiteboard.created": {
      "post": {
        "tags": [
          "whiteboard"
        ],
        "operationId": "whiteboard.created",
        "requestBody": {
          "description": "# whiteboard.created\n\nThe **Whiteboard was created** event is triggered every time a whiteboard is created by one of your app users or account users.\n\nZoom offers multiple methods for users to create a new whiteboard. Users can manually create whiteboards via the Zoom client or the [Zoom web portal](https://zoom.us/). Users can also programmatically create whiteboards via the [**Create a new whiteboard**] API.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Whiteboard was created** subscription enabled under the **Whiteboard** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:admin`,`whiteboard:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:whiteboard`,`whiteboard:read:whiteboard:admin`\n\n**Event type**: `whiteboard.created`\n",
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
                    "description": "A timestamp at which the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who created the whiteboard."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user who created the whiteboard."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user who created the whiteboard."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the whiteboard.",
                        "required": [
                          "whiteboard_id",
                          "whiteboard_name",
                          "whiteboard_type",
                          "non_cloud"
                        ],
                        "properties": {
                          "whiteboard_id": {
                            "type": "string",
                            "description": "The whiteboard ID."
                          },
                          "whiteboard_name": {
                            "type": "string",
                            "description": "The whiteboard name."
                          },
                          "whiteboard_type": {
                            "type": "integer",
                            "description": "whiteboard type.",
                            "enum": [
                              0
                            ]
                          },
                          "non_cloud": {
                            "type": "boolean",
                            "description": "Whether the whiteboard allows to be saved to cloud or not."
                          },
                          "meeting_id": {
                            "type": "string",
                            "description": "The meeting ID. This response only returns when the whiteboard created in a meeting."
                          },
                          "meeting_uuid": {
                            "type": "string",
                            "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID. This response only returns when the whiteboard created in a meeting."
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
                  "value": "{\n  \"event\": \"whiteboard.created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"whiteboard_id\": \"1234567890\",\n      \"whiteboard_name\": \"Q3 Plan\",\n      \"whiteboard_type\": 0,\n      \"non_cloud\": false,\n      \"meeting_id\": \"1234567890\",\n      \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\"\n    }\n  }\n}"
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
                      "description": "A timestamp at which the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who created the whiteboard."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user who created the whiteboard."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user who created the whiteboard."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the whiteboard.",
                          "required": [
                            "whiteboard_id",
                            "whiteboard_name",
                            "whiteboard_type",
                            "non_cloud"
                          ],
                          "properties": {
                            "whiteboard_id": {
                              "type": "string",
                              "description": "The whiteboard ID."
                            },
                            "whiteboard_name": {
                              "type": "string",
                              "description": "The whiteboard name."
                            },
                            "whiteboard_type": {
                              "type": "integer",
                              "description": "whiteboard type.",
                              "enum": [
                                0
                              ]
                            },
                            "non_cloud": {
                              "type": "boolean",
                              "description": "Whether the whiteboard allows to be saved to cloud or not."
                            },
                            "meeting_id": {
                              "type": "string",
                              "description": "The meeting ID. This response only returns when the whiteboard created in a meeting."
                            },
                            "meeting_uuid": {
                              "type": "string",
                              "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID. This response only returns when the whiteboard created in a meeting."
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
                    "value": "{\n  \"event\": \"whiteboard.created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"whiteboard_id\": \"1234567890\",\n      \"whiteboard_name\": \"Q3 Plan\",\n      \"whiteboard_type\": 0,\n      \"non_cloud\": false,\n      \"meeting_id\": \"1234567890\",\n      \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "whiteboard.deleted": {
      "post": {
        "tags": [
          "whiteboard"
        ],
        "operationId": "whiteboard.deleted",
        "requestBody": {
          "description": "# whiteboard.deleted\n\nThe **Whiteboard deleted** event is triggered every time a whiteboard is delete by one of your app users or account users.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Whiteboard deleted** subscription enabled under the **Whiteboard** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read`,`whiteboard:read:admin`,`whiteboard:read`,`whiteboard:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:whiteboard`,`whiteboard:read:whiteboard:admin`\n\n**Event type**: `whiteboard.deleted`\n",
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
                    "description": "A timestamp at which the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who deleted the whiteboard."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user who deleted the whiteboard."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user who deleted the whiteboard."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the whiteboard.",
                        "required": [
                          "whiteboard_id",
                          "whiteboard_name",
                          "whiteboard_type",
                          "non_cloud"
                        ],
                        "properties": {
                          "whiteboard_id": {
                            "type": "string",
                            "description": "The whiteboard ID."
                          },
                          "whiteboard_name": {
                            "type": "string",
                            "description": "The whiteboard name."
                          },
                          "whiteboard_type": {
                            "type": "integer",
                            "description": "whiteboard type.",
                            "enum": [
                              0
                            ]
                          },
                          "non_cloud": {
                            "type": "boolean",
                            "description": "Whether the whiteboard allows to be saved to cloud or not."
                          }
                        },
                        "additionalProperties": true
                      }
                    },
                    "additionalProperties": true
                  }
                },
                "additionalProperties": true
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"whiteboard.deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"whiteboard_id\": \"1234567890\",\n      \"whiteboard_name\": \"Q3 Plan\",\n      \"whiteboard_type\": 0,\n      \"non_cloud\":false\n    }\n  }\n}"
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
                      "description": "A timestamp at which the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who deleted the whiteboard."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user who deleted the whiteboard."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user who deleted the whiteboard."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the whiteboard.",
                          "required": [
                            "whiteboard_id",
                            "whiteboard_name",
                            "whiteboard_type",
                            "non_cloud"
                          ],
                          "properties": {
                            "whiteboard_id": {
                              "type": "string",
                              "description": "The whiteboard ID."
                            },
                            "whiteboard_name": {
                              "type": "string",
                              "description": "The whiteboard name."
                            },
                            "whiteboard_type": {
                              "type": "integer",
                              "description": "whiteboard type.",
                              "enum": [
                                0
                              ]
                            },
                            "non_cloud": {
                              "type": "boolean",
                              "description": "Whether the whiteboard allows to be saved to cloud or not."
                            }
                          },
                          "additionalProperties": true
                        }
                      },
                      "additionalProperties": true
                    }
                  },
                  "additionalProperties": true
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"whiteboard.deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"whiteboard_id\": \"1234567890\",\n      \"whiteboard_name\": \"Q3 Plan\",\n      \"whiteboard_type\": 0,\n      \"non_cloud\":false\n    }\n  }\n}"
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
      "whiteboard",
      "events"
    ]
  },
  "downloadPath": "/api-hub/whiteboard/events/webhooks.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": false
  },
  "productMeta": {
    "specType": "adoc",
    "slug": "/docs/api/rest/reference/whiteboard/events",
    "productName": "Whiteboard",
    "title": "Zoom API Events - Whiteboard",
    "keywords": "zoom, marketplace, webhook, event, create, update, delete, real-time, data, summary, qos, quality, service",
    "zoapEndpoint": "webhookByProduct",
    "params": {
      "productName": "Whiteboard"
    },
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "productDisplayTitle": "Whiteboard",
    "scalarSlug": "/docs/api/whiteboard/events",
    "svgname": "_whiteboard"
  },
  "frontmatter": {
    "title": "Whiteboard Webhooks",
    "keywords": "zoom, marketplace, webhook, event, create, update, delete, real-time, data, summary, qos, quality, service",
    "description": "The Zoom Whiteboard webhooks allow developers to receive events for [Zoom Whiteboard](https://developers.zoom.us/docs/zoom-whiteboard/) features.",
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
        "whiteboard",
        "events"
      ]
    },
    "downloadPath": "/api-hub/whiteboard/events/webhooks.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": false
    },
    "productMeta": {
      "specType": "adoc",
      "slug": "/docs/api/rest/reference/whiteboard/events",
      "productName": "Whiteboard",
      "title": "Zoom API Events - Whiteboard",
      "keywords": "zoom, marketplace, webhook, event, create, update, delete, real-time, data, summary, qos, quality, service",
      "zoapEndpoint": "webhookByProduct",
      "params": {
        "productName": "Whiteboard"
      },
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "productDisplayTitle": "Whiteboard",
      "scalarSlug": "/docs/api/whiteboard/events",
      "svgname": "_whiteboard"
    },
    "spec": "{\"openapi\":\"3.1.0\",\"info\":{\"title\":\"Whiteboard\",\"description\":\"The Zoom Whiteboard webhooks allow developers to receive events for [Zoom Whiteboard](https://developers.zoom.us/docs/zoom-whiteboard/) features.\",\"version\":\"1.0.0\"},\"paths\":{\"whiteboard.shared\":{\"post\":{\"tags\":[\"whiteboard\"],\"operationId\":\"whiteboard.shared\",\"requestBody\":{\"description\":\"# whiteboard.shared\\n\\nThe **Whiteboard was shared** event is triggered every time a whiteboard is shared with a Zoom user or a team chat channel by one of your whiteboard collaborators.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Whiteboard was shared** subscription enabled under the **Whiteboard** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read`,`whiteboard:read:admin`,`whiteboard:read`,`whiteboard:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:list_collaborators`,`whiteboard:read:list_collaborators:admin`\\n\\n**Event type**: `whiteboard.shared`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"operator_collaborator_id\",\"object\"],\"properties\":{\"operator_id\":{\"type\":\"string\",\"description\":\"nullable The Zoom user ID of the operator who added users or team chat channels to the whiteboard. It should be null for external user(the user is external to the whiteboard's organization).\"},\"operator_collaborator_id\":{\"type\":\"string\",\"description\":\"not nullable The unique ID of the collaborator who added users or team chat channels to the whiteboard. Each collaborator will have a ID in a whiteboard.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the whiteboard.\",\"required\":[\"whiteboard_id\",\"collaborators\"],\"properties\":{\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\"},\"meeting_id\":{\"type\":\"string\",\"description\":\"The meeting ID. This response only returns the whiteboard is shared in a meeting.\"},\"meeting_uuid\":{\"type\":\"string\",\"description\":\"The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID. This response only returns the whiteboard is shared in a meeting.\"},\"collaborators\":{\"type\":\"array\",\"description\":\"Information about the whiteboard collaborators.\",\"items\":{\"anyOf\":[{\"type\":\"object\",\"required\":[\"collaborator_id\",\"collaborator_type\",\"role\"],\"properties\":{\"collaborator_id\":{\"type\":\"string\",\"description\":\"The collaborator's unique ID.\"},\"collaborator_name\":{\"type\":\"string\",\"description\":\"The collaborator's display name.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The collaborator's user ID. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. \"},\"email\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"User email. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. See [Email address display rules](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#email-address) for details.\"},\"channel_id\":{\"type\":\"string\",\"description\":\"The team chat channel ID you want to share with. This response only returns if the collaborator_type is 2. Otherwise it should be null.\"},\"collaborator_type\":{\"type\":\"integer\",\"description\":\"Collaborator type. \\nEnum: \\n*0 - SAME_ACCOUNT_USER \\n*1 - EXTERNAL_USER \\n*2 - CHANNEL \\n\",\"enum\":[0,1,2],\"x-enum-descriptions\":[\"SAME_ACCOUNT_USER\",\"EXTERNAL_USER\",\"CHANNEL\"]},\"role\":{\"type\":\"integer\",\"description\":\"Collaborator permission. \\nEnum: \\n*0 - OWNER \\n*1 - CO_OWNER \\n*2 - EDITOR \\n*3 - COMMENTER \\n*4 - VIEWER \\n.\",\"enum\":[0,1,2,3,4],\"x-enum-descriptions\":[\"OWNER\",\"CO_OWNER\",\"EDITOR\",\"COMMENTER\",\"VIEWER\"]}},\"additionalProperties\":false}]},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"whiteboard.shared\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"operator_collaborator_id\\\": \\\"Cvftg6789087654443lkhk\\\",\\n    \\\"object\\\": {\\n      \\\"whiteboard_id\\\": \\\"X4Hy02w3QUOdskKofgb9Jg\\\",\\n      \\\"meeting_id\\\": \\\"1234567890\\\",\\n      \\\"meeting_ uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\",\\n      \\\"collaborators\\\": [\\n        {\\n          \\\"collaborator_id\\\": \\\"x4g6hy789k0o878k\\\",\\n          \\\"collaborator_name\\\": \\\"Jack\\\",\\n          \\\"email\\\": \\\"user@example.com\\\",\\n          \\\"user_id\\\": \\\"er54t67y8ui9o0p0\\\",\\n          \\\"collaborator_type\\\": 0,\\n          \\\"role\\\": 3\\n        },\\n        {\\n          \\\"collaborator_id\\\": \\\"x4g6hy789k0o878k\\\",\\n          \\\"collaborator_name\\\": \\\"ext***@exa***.com\\\",\\n          \\\"collaborator_type\\\": 1,\\n          \\\"role\\\": 3\\n        },\\n        {\\n          \\\"collaborator_id\\\": \\\"df4g6hy789k0o878k\\\",\\n          \\\"channel_id\\\": \\\"example@conference.xmpp.zoom.us\\\",\\n          \\\"collaborator_name\\\": \\\"Channel A\\\",\\n          \\\"collaborator_type\\\": 2,\\n          \\\"role\\\": 3\\n        }\\n      ]\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"operator_collaborator_id\",\"object\"],\"properties\":{\"operator_id\":{\"type\":\"string\",\"description\":\"nullable The Zoom user ID of the operator who added users or team chat channels to the whiteboard. It should be null for external user(the user is external to the whiteboard's organization).\"},\"operator_collaborator_id\":{\"type\":\"string\",\"description\":\"not nullable The unique ID of the collaborator who added users or team chat channels to the whiteboard. Each collaborator will have a ID in a whiteboard.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the whiteboard.\",\"required\":[\"whiteboard_id\",\"collaborators\"],\"properties\":{\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\"},\"meeting_id\":{\"type\":\"string\",\"description\":\"The meeting ID. This response only returns the whiteboard is shared in a meeting.\"},\"meeting_uuid\":{\"type\":\"string\",\"description\":\"The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID. This response only returns the whiteboard is shared in a meeting.\"},\"collaborators\":{\"type\":\"array\",\"description\":\"Information about the whiteboard collaborators.\",\"items\":{\"anyOf\":[{\"type\":\"object\",\"required\":[\"collaborator_id\",\"collaborator_type\",\"role\"],\"properties\":{\"collaborator_id\":{\"type\":\"string\",\"description\":\"The collaborator's unique ID.\"},\"collaborator_name\":{\"type\":\"string\",\"description\":\"The collaborator's display name.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The collaborator's user ID. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. \"},\"email\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"User email. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. See [Email address display rules](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#email-address) for details.\"},\"channel_id\":{\"type\":\"string\",\"description\":\"The team chat channel ID you want to share with. This response only returns if the collaborator_type is 2. Otherwise it should be null.\"},\"collaborator_type\":{\"type\":\"integer\",\"description\":\"Collaborator type. \\nEnum: \\n*0 - SAME_ACCOUNT_USER \\n*1 - EXTERNAL_USER \\n*2 - CHANNEL \\n\",\"enum\":[0,1,2],\"x-enum-descriptions\":[\"SAME_ACCOUNT_USER\",\"EXTERNAL_USER\",\"CHANNEL\"]},\"role\":{\"type\":\"integer\",\"description\":\"Collaborator permission. \\nEnum: \\n*0 - OWNER \\n*1 - CO_OWNER \\n*2 - EDITOR \\n*3 - COMMENTER \\n*4 - VIEWER \\n.\",\"enum\":[0,1,2,3,4],\"x-enum-descriptions\":[\"OWNER\",\"CO_OWNER\",\"EDITOR\",\"COMMENTER\",\"VIEWER\"]}},\"additionalProperties\":false}]},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"whiteboard.shared\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"operator_collaborator_id\\\": \\\"Cvftg6789087654443lkhk\\\",\\n    \\\"object\\\": {\\n      \\\"whiteboard_id\\\": \\\"X4Hy02w3QUOdskKofgb9Jg\\\",\\n      \\\"meeting_id\\\": \\\"1234567890\\\",\\n      \\\"meeting_ uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\",\\n      \\\"collaborators\\\": [\\n        {\\n          \\\"collaborator_id\\\": \\\"x4g6hy789k0o878k\\\",\\n          \\\"collaborator_name\\\": \\\"Jack\\\",\\n          \\\"email\\\": \\\"user@example.com\\\",\\n          \\\"user_id\\\": \\\"er54t67y8ui9o0p0\\\",\\n          \\\"collaborator_type\\\": 0,\\n          \\\"role\\\": 3\\n        },\\n        {\\n          \\\"collaborator_id\\\": \\\"x4g6hy789k0o878k\\\",\\n          \\\"collaborator_name\\\": \\\"ext***@exa***.com\\\",\\n          \\\"collaborator_type\\\": 1,\\n          \\\"role\\\": 3\\n        },\\n        {\\n          \\\"collaborator_id\\\": \\\"df4g6hy789k0o878k\\\",\\n          \\\"channel_id\\\": \\\"example@conference.xmpp.zoom.us\\\",\\n          \\\"collaborator_name\\\": \\\"Channel A\\\",\\n          \\\"collaborator_type\\\": 2,\\n          \\\"role\\\": 3\\n        }\\n      ]\\n    }\\n  }\\n}\"}}}}}}}},\"whiteboard.sharing_updated\":{\"post\":{\"tags\":[\"whiteboard\"],\"operationId\":\"whiteboard.sharing_updated\",\"requestBody\":{\"description\":\"# whiteboard.sharing_updated\\n\\nThe **Whiteboard collaborator's sharing settings were updated** event is triggered every time the of a whiteboard collaborator is updated or removed by one of your whiteboard owners.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Whiteboard collaborator's sharing settings were updated** subscription enabled under the **Whiteboard** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read`,`whiteboard:read:admin`,`whiteboard:read`,`whiteboard:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:list_collaborators`,`whiteboard:read:list_collaborators:admin`\\n\\n**Event type**: `whiteboard.sharing_updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"operator_collaborator_id\",\"operation_type\",\"object\"],\"properties\":{\"operator_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the operator who changed the permission of collaborators of the whiteboard. It should be whiteboard owner user ID.\"},\"operator_collaborator_id\":{\"type\":\"string\",\"description\":\"not nullable The whiteboard collaborator's universally unique identifier. Each collaborator will have a ID in a whiteboard.\"},\"operation_type\":{\"type\":\"integer\",\"description\":\"Operation type. \\nEnum: \\n*0 - REMOVE \\n*1 - UPDATE \\n\",\"enum\":[0,1],\"x-enum-descriptions\":[\"REMOVE\",\"UPDATE\"]},\"object\":{\"type\":\"object\",\"description\":\"Information about the whiteboard collaborators.\",\"required\":[\"whiteboard_id\",\"collaborators\"],\"properties\":{\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\"},\"collaborators\":{\"type\":\"array\",\"description\":\"Information about the effected whiteboard collaborators.\",\"items\":{\"anyOf\":[{\"type\":\"object\",\"required\":[\"collaborator_id\",\"collaborator_type\",\"role\"],\"properties\":{\"collaborator_id\":{\"type\":\"string\",\"description\":\"The collaborator's ID. \\n* If collaborator_type set to 0, this field refers to a user ID. If collaborator_type set to 2, this field refers to a team chat channel ID. If collaborator_type set to 1, this returns an empty value.\"},\"collaborator_name\":{\"type\":\"string\",\"description\":\"The collaborator's display name.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The collaborator's user ID. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. \"},\"email\":{\"type\":\"string\",\"description\":\"User email. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. See [Email address display rules](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#email-address) for details.\"},\"channel_id\":{\"type\":\"string\",\"description\":\"The team chat channel ID you want to share with. This response only returns if the collaborator_type is 2. Otherwise it should be null.\"},\"collaborator_type\":{\"type\":\"integer\",\"description\":\"Collaborator type. \\nEnum: \\n*0 - SAME_ACCOUNT_USER \\n*1 - EXTERNAL_USER \\n*2 - CHANNEL \\n\",\"enum\":[0,1,2],\"x-enum-descriptions\":[\"SAME_ACCOUNT_USER\",\"EXTERNAL_USER\",\"CHANNEL\"]},\"role\":{\"type\":\"integer\",\"description\":\"Collaborator permission. \\nEnum: \\n*0 - OWNER \\n*1 - CO_OWNER \\n*2 - EDITOR \\n*3 - COMMENTER \\n*4 - VIEWER \\n.\",\"enum\":[0,1,2,3,4],\"x-enum-descriptions\":[\"OWNER\",\"CO_OWNER\",\"EDITOR\",\"COMMENTER\",\"VIEWER\"]}},\"additionalProperties\":false}]},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"whiteboard.sharing_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"operation_type\\\": 1,\\n    \\\"operator_collaborator_id\\\": \\\"Cvftg6789087654443lkhk\\\",\\n    \\\"object\\\": {\\n      \\\"whiteboard_id\\\": \\\"X4Hy02w3QUOdskKofgb9Jg\\\",\\n      \\\"collaborators\\\": [\\n        {\\n          \\\"collaborator_id\\\": \\\"as35hjnh89899fmkl0ou\\\",\\n          \\\"collaborator_name\\\": \\\"Jack\\\",\\n          \\\"user_id\\\": \\\"er54t67y8ui9o0p0\\\",\\n          \\\"email\\\": \\\"user@example.com\\\",\\n          \\\"collaborator_type\\\": 0,\\n          \\\"role\\\": 2\\n        },\\n        {\\n          \\\"collaborator_id\\\": \\\"x4g6hy789k0o878k\\\",\\n          \\\"collaborator_name\\\": \\\"ext***@exa***.com\\\",\\n          \\\"collaborator_type\\\": 1,\\n          \\\"role\\\": 3\\n        },\\n        {\\n          \\\"collaborator_id\\\": \\\"34g6hy789k0o878k\\\",\\n          \\\"channel_id\\\": \\\"example@conference.xmpp.zoom.us\\\",\\n          \\\"collaborator_name\\\": \\\"Channel A\\\",\\n          \\\"collaborator_type\\\": 2,\\n          \\\"role\\\": 2\\n        }\\n      ]\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"operator_collaborator_id\",\"operation_type\",\"object\"],\"properties\":{\"operator_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the operator who changed the permission of collaborators of the whiteboard. It should be whiteboard owner user ID.\"},\"operator_collaborator_id\":{\"type\":\"string\",\"description\":\"not nullable The whiteboard collaborator's universally unique identifier. Each collaborator will have a ID in a whiteboard.\"},\"operation_type\":{\"type\":\"integer\",\"description\":\"Operation type. \\nEnum: \\n*0 - REMOVE \\n*1 - UPDATE \\n\",\"enum\":[0,1],\"x-enum-descriptions\":[\"REMOVE\",\"UPDATE\"]},\"object\":{\"type\":\"object\",\"description\":\"Information about the whiteboard collaborators.\",\"required\":[\"whiteboard_id\",\"collaborators\"],\"properties\":{\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\"},\"collaborators\":{\"type\":\"array\",\"description\":\"Information about the effected whiteboard collaborators.\",\"items\":{\"anyOf\":[{\"type\":\"object\",\"required\":[\"collaborator_id\",\"collaborator_type\",\"role\"],\"properties\":{\"collaborator_id\":{\"type\":\"string\",\"description\":\"The collaborator's ID. \\n* If collaborator_type set to 0, this field refers to a user ID. If collaborator_type set to 2, this field refers to a team chat channel ID. If collaborator_type set to 1, this returns an empty value.\"},\"collaborator_name\":{\"type\":\"string\",\"description\":\"The collaborator's display name.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The collaborator's user ID. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. \"},\"email\":{\"type\":\"string\",\"description\":\"User email. This response only returns if the collaborator_type is 0. If the collaborator_type is **not** 0, this returns null value. See [Email address display rules](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#email-address) for details.\"},\"channel_id\":{\"type\":\"string\",\"description\":\"The team chat channel ID you want to share with. This response only returns if the collaborator_type is 2. Otherwise it should be null.\"},\"collaborator_type\":{\"type\":\"integer\",\"description\":\"Collaborator type. \\nEnum: \\n*0 - SAME_ACCOUNT_USER \\n*1 - EXTERNAL_USER \\n*2 - CHANNEL \\n\",\"enum\":[0,1,2],\"x-enum-descriptions\":[\"SAME_ACCOUNT_USER\",\"EXTERNAL_USER\",\"CHANNEL\"]},\"role\":{\"type\":\"integer\",\"description\":\"Collaborator permission. \\nEnum: \\n*0 - OWNER \\n*1 - CO_OWNER \\n*2 - EDITOR \\n*3 - COMMENTER \\n*4 - VIEWER \\n.\",\"enum\":[0,1,2,3,4],\"x-enum-descriptions\":[\"OWNER\",\"CO_OWNER\",\"EDITOR\",\"COMMENTER\",\"VIEWER\"]}},\"additionalProperties\":false}]},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"whiteboard.sharing_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"operation_type\\\": 1,\\n    \\\"operator_collaborator_id\\\": \\\"Cvftg6789087654443lkhk\\\",\\n    \\\"object\\\": {\\n      \\\"whiteboard_id\\\": \\\"X4Hy02w3QUOdskKofgb9Jg\\\",\\n      \\\"collaborators\\\": [\\n        {\\n          \\\"collaborator_id\\\": \\\"as35hjnh89899fmkl0ou\\\",\\n          \\\"collaborator_name\\\": \\\"Jack\\\",\\n          \\\"user_id\\\": \\\"er54t67y8ui9o0p0\\\",\\n          \\\"email\\\": \\\"user@example.com\\\",\\n          \\\"collaborator_type\\\": 0,\\n          \\\"role\\\": 2\\n        },\\n        {\\n          \\\"collaborator_id\\\": \\\"x4g6hy789k0o878k\\\",\\n          \\\"collaborator_name\\\": \\\"ext***@exa***.com\\\",\\n          \\\"collaborator_type\\\": 1,\\n          \\\"role\\\": 3\\n        },\\n        {\\n          \\\"collaborator_id\\\": \\\"34g6hy789k0o878k\\\",\\n          \\\"channel_id\\\": \\\"example@conference.xmpp.zoom.us\\\",\\n          \\\"collaborator_name\\\": \\\"Channel A\\\",\\n          \\\"collaborator_type\\\": 2,\\n          \\\"role\\\": 2\\n        }\\n      ]\\n    }\\n  }\\n}\"}}}}}}}},\"whiteboard.share_settings_updated\":{\"post\":{\"tags\":[\"whiteboard\"],\"operationId\":\"whiteboard.share_settings_updated\",\"requestBody\":{\"description\":\"# whiteboard.share_settings_updated\\n\\nWhiteboard share settings can be updated by a whiteboard owner or co-owners.\\n\\nThe **Whiteboard's sharing setting were updated** event is triggered every time a whiteboard share setting is updated by whiteboard owner or co-owners.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Whiteboard's sharing setting were updated** subscription enabled under the **Whiteboard** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read`,`whiteboard:read:admin`,`whiteboard:read`,`whiteboard:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:share_setting`,`whiteboard:read:share_setting:admin`\\n\\n**Event type**: `whiteboard.share_settings_updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who updated the whiteboard share settings.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user who updated the whiteboard share settings.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user who updated the whiteboard share settings.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the whiteboard.\",\"required\":[\"whiteboard_id\",\"settings\"],\"properties\":{\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\"},\"settings\":{\"type\":\"object\",\"description\":\"Information about the whiteboard share settings.\",\"properties\":{\"share_link_setting\":{\"type\":\"object\",\"description\":\"Information about the whiteboard share link settings.\",\"properties\":{\"share_scope\":{\"type\":\"integer\",\"description\":\"Who can access the share link\",\"enum\":[0,1,2],\"x-enum-descriptions\":[\"Only invited members can access this board\",\"Anyone at the current organization\",\"Anyone with the link can view this board\"]},\"share_role\":{\"type\":\"integer\",\"description\":\"Permission of the shared user.\",\"enum\":[1,2,3,4],\"x-enum-descriptions\":[\"CO_OWNER\",\"EDITOR\",\"COMMENTER\",\"VIEWER\"]}}},\"advanced_share_setting\":{\"type\":\"object\",\"description\":\"Information about the whiteboard advanced share settings\",\"properties\":{\"can_share\":{\"type\":\"boolean\",\"description\":\"Indicate whether allow to share whiteboard or not.\"},\"share_to_external\":{\"type\":\"boolean\",\"description\":\"Indicate whether allowed to share with user who is in the external organization or not.\"},\"external_can_share\":{\"type\":\"boolean\",\"description\":\"Indicate whether the external user is allowed to share whiteboard or not.\"}}}}}},\"additionalProperties\":true}},\"additionalProperties\":true}},\"additionalProperties\":true},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n    \\\"event\\\":\\\"whiteboard.share_setting_updated\\\",\\n    \\\"event_ts\\\":1626230691572,\\n    \\\"payload\\\":{\\n        \\\"account_id\\\":\\\"AAAAAABBBB\\\",\\n        \\\"operator\\\":\\\"admin@example.com\\\",\\n        \\\"operator_id\\\":\\\"z8yCxjabcdEFGHfp8uQ\\\",\\n        \\\"object\\\":{\\n            \\\"whiteboard_id\\\":\\\"BK0R3Lg-Toi92viKHhzUVQ\\\",\\n            \\\"settings\\\":{\\n                \\\"share_link_setting\\\":{\\n                    \\\"share_scope\\\":0,\\n                    \\\"share_role\\\":4\\n                },\\n                \\\"advanced_share_setting\\\":{\\n                    \\\"can_share\\\":true,\\n                    \\\"share_to_external\\\":true,\\n                    \\\"external_can_share\\\":false\\n                }\\n            }\\n        }\\n    }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who updated the whiteboard share settings.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user who updated the whiteboard share settings.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user who updated the whiteboard share settings.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the whiteboard.\",\"required\":[\"whiteboard_id\",\"settings\"],\"properties\":{\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\"},\"settings\":{\"type\":\"object\",\"description\":\"Information about the whiteboard share settings.\",\"properties\":{\"share_link_setting\":{\"type\":\"object\",\"description\":\"Information about the whiteboard share link settings.\",\"properties\":{\"share_scope\":{\"type\":\"integer\",\"description\":\"Who can access the share link\",\"enum\":[0,1,2],\"x-enum-descriptions\":[\"Only invited members can access this board\",\"Anyone at the current organization\",\"Anyone with the link can view this board\"]},\"share_role\":{\"type\":\"integer\",\"description\":\"Permission of the shared user.\",\"enum\":[1,2,3,4],\"x-enum-descriptions\":[\"CO_OWNER\",\"EDITOR\",\"COMMENTER\",\"VIEWER\"]}}},\"advanced_share_setting\":{\"type\":\"object\",\"description\":\"Information about the whiteboard advanced share settings\",\"properties\":{\"can_share\":{\"type\":\"boolean\",\"description\":\"Indicate whether allow to share whiteboard or not.\"},\"share_to_external\":{\"type\":\"boolean\",\"description\":\"Indicate whether allowed to share with user who is in the external organization or not.\"},\"external_can_share\":{\"type\":\"boolean\",\"description\":\"Indicate whether the external user is allowed to share whiteboard or not.\"}}}}}},\"additionalProperties\":true}},\"additionalProperties\":true}},\"additionalProperties\":true},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n    \\\"event\\\":\\\"whiteboard.share_setting_updated\\\",\\n    \\\"event_ts\\\":1626230691572,\\n    \\\"payload\\\":{\\n        \\\"account_id\\\":\\\"AAAAAABBBB\\\",\\n        \\\"operator\\\":\\\"admin@example.com\\\",\\n        \\\"operator_id\\\":\\\"z8yCxjabcdEFGHfp8uQ\\\",\\n        \\\"object\\\":{\\n            \\\"whiteboard_id\\\":\\\"BK0R3Lg-Toi92viKHhzUVQ\\\",\\n            \\\"settings\\\":{\\n                \\\"share_link_setting\\\":{\\n                    \\\"share_scope\\\":0,\\n                    \\\"share_role\\\":4\\n                },\\n                \\\"advanced_share_setting\\\":{\\n                    \\\"can_share\\\":true,\\n                    \\\"share_to_external\\\":true,\\n                    \\\"external_can_share\\\":false\\n                }\\n            }\\n        }\\n    }\\n}\"}}}}}}}},\"whiteboard.archive_available\":{\"post\":{\"tags\":[\"whiteboard\"],\"operationId\":\"whiteboard.archive_available\",\"requestBody\":{\"description\":\"# whiteboard.archive_available\\nThe **Whiteboard archive was available** event is emitted whenever a whiteboard archived sessions is successfully generated. This event can be used to notify subscribers when an archive is ready to be accessed.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Whiteboard archive was available** subscription enabled under the **Whiteboard** event.\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_content:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:session:admin`\\n\\n**Event type**: `whiteboard.archive_available`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who owns the session.\",\"example\":\"AAAAAABBBB\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the whiteboard.\",\"required\":[\"session_id\",\"start_time\",\"end_time\",\"whiteboard_id\",\"whiteboard_name\",\"expire_time\"],\"properties\":{\"session_id\":{\"type\":\"string\",\"description\":\"The unique ID of the session. This ID can be used to retrieve all activities under the session via the [List whiteboard sessions activities](https://developers.zoom.us/docs/api/rest/reference/whiteboard/methods/#operation/Listwhiteboardsessionsarchivedfiles) API.\",\"example\":\"a2f19f96-9294-4f51-8134-6f0eea108eb2\"},\"start_time\":{\"type\":\"string\",\"description\":\"The session's start time in UTC/GMT, formatted as yyyy-MM-dd'T'HH:mm:ssZ.\",\"format\":\"date-time\",\"example\":\"2021-03-11T05:41:36Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The session's end time in UTC/GMT, formatted as yyyy-MM-dd'T'HH:mm:ssZ.\",\"format\":\"date-time\",\"example\":\"2021-03-11T05:41:36Z\"},\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The ID of whiteboard.\",\"example\":\"HY8-AC_HSou4SXY1S-OmOg\"},\"whiteboard_name\":{\"type\":\"string\",\"description\":\"The name of whiteboard.\",\"example\":\"example_whiteboard_name\"},\"expire_time\":{\"type\":\"string\",\"description\":\"The expiration time of the archived content in UTC/GMT. After this time, the archived content will be permanently deleted. The value is formatted as yyyy-MM-dd'T'HH:mm:ssZ.\",\"format\":\"date-time\",\"example\":\"2021-03-26T06:41:36Z\"}}}},\"additionalProperties\":true}},\"additionalProperties\":true},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"whiteboard.archive_available\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"session_id\\\": \\\"a2f19f96-9294-4f51-8134-6f0eea108eb2\\\",\\n      \\\"start_time\\\": \\\"2021-03-11T05:41:36Z\\\",\\n      \\\"end_time\\\": \\\"2021-03-11T05:41:36Z\\\",\\n      \\\"whiteboard_id\\\": \\\"1234567890\\\",\\n      \\\"whiteboard_name\\\": \\\"Q3 Plan\\\",\\n      \\\"expire_time\\\": \\\"2021-03-26T06:41:36Z\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who owns the session.\",\"example\":\"AAAAAABBBB\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the whiteboard.\",\"required\":[\"session_id\",\"start_time\",\"end_time\",\"whiteboard_id\",\"whiteboard_name\",\"expire_time\"],\"properties\":{\"session_id\":{\"type\":\"string\",\"description\":\"The unique ID of the session. This ID can be used to retrieve all activities under the session via the [List whiteboard sessions activities](https://developers.zoom.us/docs/api/rest/reference/whiteboard/methods/#operation/Listwhiteboardsessionsarchivedfiles) API.\",\"example\":\"a2f19f96-9294-4f51-8134-6f0eea108eb2\"},\"start_time\":{\"type\":\"string\",\"description\":\"The session's start time in UTC/GMT, formatted as yyyy-MM-dd'T'HH:mm:ssZ.\",\"format\":\"date-time\",\"example\":\"2021-03-11T05:41:36Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The session's end time in UTC/GMT, formatted as yyyy-MM-dd'T'HH:mm:ssZ.\",\"format\":\"date-time\",\"example\":\"2021-03-11T05:41:36Z\"},\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The ID of whiteboard.\",\"example\":\"HY8-AC_HSou4SXY1S-OmOg\"},\"whiteboard_name\":{\"type\":\"string\",\"description\":\"The name of whiteboard.\",\"example\":\"example_whiteboard_name\"},\"expire_time\":{\"type\":\"string\",\"description\":\"The expiration time of the archived content in UTC/GMT. After this time, the archived content will be permanently deleted. The value is formatted as yyyy-MM-dd'T'HH:mm:ssZ.\",\"format\":\"date-time\",\"example\":\"2021-03-26T06:41:36Z\"}}}},\"additionalProperties\":true}},\"additionalProperties\":true},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"whiteboard.archive_available\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"session_id\\\": \\\"a2f19f96-9294-4f51-8134-6f0eea108eb2\\\",\\n      \\\"start_time\\\": \\\"2021-03-11T05:41:36Z\\\",\\n      \\\"end_time\\\": \\\"2021-03-11T05:41:36Z\\\",\\n      \\\"whiteboard_id\\\": \\\"1234567890\\\",\\n      \\\"whiteboard_name\\\": \\\"Q3 Plan\\\",\\n      \\\"expire_time\\\": \\\"2021-03-26T06:41:36Z\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"whiteboard.opened\":{\"post\":{\"tags\":[\"whiteboard\"],\"operationId\":\"whiteboard.opened\",\"requestBody\":{\"description\":\"# whiteboard.opened\\n\\nThe **Whiteboard was opened** event is triggered every time a whiteboard is opened.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Whiteboard was opened** subscription enabled under the **Whiteboard** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:admin`,`whiteboard:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:whiteboard`,`whiteboard:read:whiteboard:admin`\\n\\n**Event type**: `whiteboard.opened`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"operator_collaborator_id\",\"object\"],\"properties\":{\"operator_id\":{\"type\":\"string\",\"description\":\"nullable The Zoom user ID of the operator who opened the whiteboard. It should be null for external user(the user is external to the whiteboard's organization).\"},\"operator_collaborator_id\":{\"type\":\"string\",\"description\":\"not nullable The whiteboard collaborator's universally unique identifier of the operator who opened the whiteboard. Each collaborator will have a ID in a whiteboard.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the whiteboard.\",\"required\":[\"whiteboard_id\",\"whiteboard_name\"],\"properties\":{\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\"},\"whiteboard_name\":{\"type\":\"string\",\"description\":\"The whiteboard name.\"},\"meeting_id\":{\"type\":\"string\",\"description\":\"The meeting ID. This response only returns the whiteboard opened in a meeting.\"},\"meeting_uuid\":{\"type\":\"string\",\"description\":\"The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID. This response only returns the whiteboard opened in a meeting.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"whiteboard.opened\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"operator_collaborator_id\\\": \\\"Cvftg6789087654443lkhk\\\",\\n    \\\"object\\\": {\\n      \\\"whiteboard_id\\\": \\\"X4Hy02w3QUOdskKofgb9Jg\\\",\\n      \\\"whiteboard_name\\\": \\\"Q3 Plan\\\",\\n      \\\"meeting_id\\\": \\\"1234567890\\\",\\n      \\\"meeting_uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"operator_collaborator_id\",\"object\"],\"properties\":{\"operator_id\":{\"type\":\"string\",\"description\":\"nullable The Zoom user ID of the operator who opened the whiteboard. It should be null for external user(the user is external to the whiteboard's organization).\"},\"operator_collaborator_id\":{\"type\":\"string\",\"description\":\"not nullable The whiteboard collaborator's universally unique identifier of the operator who opened the whiteboard. Each collaborator will have a ID in a whiteboard.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the whiteboard.\",\"required\":[\"whiteboard_id\",\"whiteboard_name\"],\"properties\":{\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\"},\"whiteboard_name\":{\"type\":\"string\",\"description\":\"The whiteboard name.\"},\"meeting_id\":{\"type\":\"string\",\"description\":\"The meeting ID. This response only returns the whiteboard opened in a meeting.\"},\"meeting_uuid\":{\"type\":\"string\",\"description\":\"The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID. This response only returns the whiteboard opened in a meeting.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"whiteboard.opened\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"operator_collaborator_id\\\": \\\"Cvftg6789087654443lkhk\\\",\\n    \\\"object\\\": {\\n      \\\"whiteboard_id\\\": \\\"X4Hy02w3QUOdskKofgb9Jg\\\",\\n      \\\"whiteboard_name\\\": \\\"Q3 Plan\\\",\\n      \\\"meeting_id\\\": \\\"1234567890\\\",\\n      \\\"meeting_uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"whiteboard.file_added\":{\"post\":{\"tags\":[\"whiteboard\"],\"operationId\":\"whiteboard.file_added\",\"requestBody\":{\"description\":\"# whiteboard.file_added\\nThe **Whiteboard file was added**  event is triggered when a file is successfully added to a whiteboard.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Whiteboard file was added** subscription enabled under the **Whiteboard** event.\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_file:read`,`whiteboard_file:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:file:admin`,`whiteboard:read:file`\\n\\n**Event type**: `whiteboard.file_added`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The event name.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"operator_collaborator_id\",\"operator_name\",\"object\"],\"properties\":{\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the uploader. Null if the user is external to the whiteboard owner's organization.\"},\"operator_collaborator_id\":{\"type\":\"string\",\"description\":\"not nullable The whiteboard collaborator's universally unique identifier of the operator who uploaded the file.. Each collaborator will have a ID in a whiteboard.\"},\"operator_name\":{\"type\":\"string\",\"description\":\"The name of the user who uploaded the file.\"},\"object\":{\"type\":\"object\",\"description\":\"Details of the whiteboard where the file was uploaded.\",\"required\":[\"whiteboard_id\",\"whiteboard_name\",\"whiteboard_owner_name\",\"whiteboard_file\"],\"properties\":{\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the whiteboard.\"},\"whiteboard_name\":{\"type\":\"string\",\"description\":\"The name of the whiteboard.\"},\"whiteboard_owner_name\":{\"type\":\"string\",\"description\":\"The name of the whiteboard owner.\"},\"whiteboard_file\":{\"type\":\"object\",\"description\":\"Metadata of the file uploaded to the whiteboard.\",\"required\":[\"file_id\",\"file_name\",\"file_size\",\"file_type\",\"uploaded_ts\"],\"properties\":{\"file_id\":{\"type\":\"string\",\"description\":\"The unique ID of the file in the whiteboard.\"},\"file_name\":{\"type\":\"string\",\"description\":\"The name of the uploaded file.\"},\"file_size\":{\"type\":\"integer\",\"description\":\"The size of the uploaded file, in bytes.\"},\"file_type\":{\"type\":\"string\",\"description\":\"The MIME type or format of the uploaded file.\"},\"uploaded_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the file was uploaded.\"}}}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"whiteboard.file_added\\\",\\n  \\\"event_ts\\\": 1722238455000,\\n  \\\"payload\\\": {\\n    \\\"operator_id\\\": \\\"user_789xyz\\\",\\n    \\\"operator_collaborator_id\\\": \\\"collaborator_789xyz\\\",\\n    \\\"operator_name\\\": \\\"Jane Doe\\\",\\n    \\\"object\\\": {\\n      \\\"whiteboard_id\\\": \\\"wb_123456789\\\",\\n      \\\"whiteboard_name\\\": \\\"Project Kickoff Board\\\",\\n      \\\"whiteboard_owner_name\\\": \\\"Alice Smith\\\",\\n      \\\"whiteboard_file\\\": {\\n        \\\"file_id\\\": \\\"file_abc123\\\",\\n        \\\"file_name\\\": \\\"requirements.pdf\\\",\\n        \\\"file_size\\\": 256000,\\n        \\\"file_type\\\": \\\"application/pdf\\\",\\n        \\\"uploaded_ts\\\": 1722238455000\\n      }\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The event name.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"operator_collaborator_id\",\"operator_name\",\"object\"],\"properties\":{\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the uploader. Null if the user is external to the whiteboard owner's organization.\"},\"operator_collaborator_id\":{\"type\":\"string\",\"description\":\"not nullable The whiteboard collaborator's universally unique identifier of the operator who uploaded the file.. Each collaborator will have a ID in a whiteboard.\"},\"operator_name\":{\"type\":\"string\",\"description\":\"The name of the user who uploaded the file.\"},\"object\":{\"type\":\"object\",\"description\":\"Details of the whiteboard where the file was uploaded.\",\"required\":[\"whiteboard_id\",\"whiteboard_name\",\"whiteboard_owner_name\",\"whiteboard_file\"],\"properties\":{\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the whiteboard.\"},\"whiteboard_name\":{\"type\":\"string\",\"description\":\"The name of the whiteboard.\"},\"whiteboard_owner_name\":{\"type\":\"string\",\"description\":\"The name of the whiteboard owner.\"},\"whiteboard_file\":{\"type\":\"object\",\"description\":\"Metadata of the file uploaded to the whiteboard.\",\"required\":[\"file_id\",\"file_name\",\"file_size\",\"file_type\",\"uploaded_ts\"],\"properties\":{\"file_id\":{\"type\":\"string\",\"description\":\"The unique ID of the file in the whiteboard.\"},\"file_name\":{\"type\":\"string\",\"description\":\"The name of the uploaded file.\"},\"file_size\":{\"type\":\"integer\",\"description\":\"The size of the uploaded file, in bytes.\"},\"file_type\":{\"type\":\"string\",\"description\":\"The MIME type or format of the uploaded file.\"},\"uploaded_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the file was uploaded.\"}}}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"whiteboard.file_added\\\",\\n  \\\"event_ts\\\": 1722238455000,\\n  \\\"payload\\\": {\\n    \\\"operator_id\\\": \\\"user_789xyz\\\",\\n    \\\"operator_collaborator_id\\\": \\\"collaborator_789xyz\\\",\\n    \\\"operator_name\\\": \\\"Jane Doe\\\",\\n    \\\"object\\\": {\\n      \\\"whiteboard_id\\\": \\\"wb_123456789\\\",\\n      \\\"whiteboard_name\\\": \\\"Project Kickoff Board\\\",\\n      \\\"whiteboard_owner_name\\\": \\\"Alice Smith\\\",\\n      \\\"whiteboard_file\\\": {\\n        \\\"file_id\\\": \\\"file_abc123\\\",\\n        \\\"file_name\\\": \\\"requirements.pdf\\\",\\n        \\\"file_size\\\": 256000,\\n        \\\"file_type\\\": \\\"application/pdf\\\",\\n        \\\"uploaded_ts\\\": 1722238455000\\n      }\\n    }\\n  }\\n}\"}}}}}}}},\"whiteboard.created\":{\"post\":{\"tags\":[\"whiteboard\"],\"operationId\":\"whiteboard.created\",\"requestBody\":{\"description\":\"# whiteboard.created\\n\\nThe **Whiteboard was created** event is triggered every time a whiteboard is created by one of your app users or account users.\\n\\nZoom offers multiple methods for users to create a new whiteboard. Users can manually create whiteboards via the Zoom client or the [Zoom web portal](https://zoom.us/). Users can also programmatically create whiteboards via the [**Create a new whiteboard**] API.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Whiteboard was created** subscription enabled under the **Whiteboard** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:admin`,`whiteboard:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:whiteboard`,`whiteboard:read:whiteboard:admin`\\n\\n**Event type**: `whiteboard.created`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who created the whiteboard.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user who created the whiteboard.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user who created the whiteboard.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the whiteboard.\",\"required\":[\"whiteboard_id\",\"whiteboard_name\",\"whiteboard_type\",\"non_cloud\"],\"properties\":{\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\"},\"whiteboard_name\":{\"type\":\"string\",\"description\":\"The whiteboard name.\"},\"whiteboard_type\":{\"type\":\"integer\",\"description\":\"whiteboard type.\",\"enum\":[0]},\"non_cloud\":{\"type\":\"boolean\",\"description\":\"Whether the whiteboard allows to be saved to cloud or not.\"},\"meeting_id\":{\"type\":\"string\",\"description\":\"The meeting ID. This response only returns when the whiteboard created in a meeting.\"},\"meeting_uuid\":{\"type\":\"string\",\"description\":\"The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID. This response only returns when the whiteboard created in a meeting.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"whiteboard.created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"object\\\": {\\n      \\\"whiteboard_id\\\": \\\"1234567890\\\",\\n      \\\"whiteboard_name\\\": \\\"Q3 Plan\\\",\\n      \\\"whiteboard_type\\\": 0,\\n      \\\"non_cloud\\\": false,\\n      \\\"meeting_id\\\": \\\"1234567890\\\",\\n      \\\"meeting_uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who created the whiteboard.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user who created the whiteboard.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user who created the whiteboard.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the whiteboard.\",\"required\":[\"whiteboard_id\",\"whiteboard_name\",\"whiteboard_type\",\"non_cloud\"],\"properties\":{\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\"},\"whiteboard_name\":{\"type\":\"string\",\"description\":\"The whiteboard name.\"},\"whiteboard_type\":{\"type\":\"integer\",\"description\":\"whiteboard type.\",\"enum\":[0]},\"non_cloud\":{\"type\":\"boolean\",\"description\":\"Whether the whiteboard allows to be saved to cloud or not.\"},\"meeting_id\":{\"type\":\"string\",\"description\":\"The meeting ID. This response only returns when the whiteboard created in a meeting.\"},\"meeting_uuid\":{\"type\":\"string\",\"description\":\"The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID. This response only returns when the whiteboard created in a meeting.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"whiteboard.created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"object\\\": {\\n      \\\"whiteboard_id\\\": \\\"1234567890\\\",\\n      \\\"whiteboard_name\\\": \\\"Q3 Plan\\\",\\n      \\\"whiteboard_type\\\": 0,\\n      \\\"non_cloud\\\": false,\\n      \\\"meeting_id\\\": \\\"1234567890\\\",\\n      \\\"meeting_uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"whiteboard.deleted\":{\"post\":{\"tags\":[\"whiteboard\"],\"operationId\":\"whiteboard.deleted\",\"requestBody\":{\"description\":\"# whiteboard.deleted\\n\\nThe **Whiteboard deleted** event is triggered every time a whiteboard is delete by one of your app users or account users.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Whiteboard deleted** subscription enabled under the **Whiteboard** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read`,`whiteboard:read:admin`,`whiteboard:read`,`whiteboard:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:whiteboard`,`whiteboard:read:whiteboard:admin`\\n\\n**Event type**: `whiteboard.deleted`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who deleted the whiteboard.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user who deleted the whiteboard.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user who deleted the whiteboard.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the whiteboard.\",\"required\":[\"whiteboard_id\",\"whiteboard_name\",\"whiteboard_type\",\"non_cloud\"],\"properties\":{\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\"},\"whiteboard_name\":{\"type\":\"string\",\"description\":\"The whiteboard name.\"},\"whiteboard_type\":{\"type\":\"integer\",\"description\":\"whiteboard type.\",\"enum\":[0]},\"non_cloud\":{\"type\":\"boolean\",\"description\":\"Whether the whiteboard allows to be saved to cloud or not.\"}},\"additionalProperties\":true}},\"additionalProperties\":true}},\"additionalProperties\":true},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"whiteboard.deleted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"object\\\": {\\n      \\\"whiteboard_id\\\": \\\"1234567890\\\",\\n      \\\"whiteboard_name\\\": \\\"Q3 Plan\\\",\\n      \\\"whiteboard_type\\\": 0,\\n      \\\"non_cloud\\\":false\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who deleted the whiteboard.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user who deleted the whiteboard.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user who deleted the whiteboard.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the whiteboard.\",\"required\":[\"whiteboard_id\",\"whiteboard_name\",\"whiteboard_type\",\"non_cloud\"],\"properties\":{\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\"},\"whiteboard_name\":{\"type\":\"string\",\"description\":\"The whiteboard name.\"},\"whiteboard_type\":{\"type\":\"integer\",\"description\":\"whiteboard type.\",\"enum\":[0]},\"non_cloud\":{\"type\":\"boolean\",\"description\":\"Whether the whiteboard allows to be saved to cloud or not.\"}},\"additionalProperties\":true}},\"additionalProperties\":true}},\"additionalProperties\":true},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"whiteboard.deleted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"object\\\": {\\n      \\\"whiteboard_id\\\": \\\"1234567890\\\",\\n      \\\"whiteboard_name\\\": \\\"Q3 Plan\\\",\\n      \\\"whiteboard_type\\\": 0,\\n      \\\"non_cloud\\\":false\\n    }\\n  }\\n}\"}}}}}}}}},\"servers\":[{\"url\":\"https://{your-endpoint-url}\"}]}",
    "frontmatter": {
      "title": "Whiteboard Webhooks",
      "keywords": "zoom, marketplace, webhook, event, create, update, delete, real-time, data, summary, qos, quality, service",
      "description": "The Zoom Whiteboard webhooks allow developers to receive events for [Zoom Whiteboard](https://developers.zoom.us/docs/zoom-whiteboard/) features.",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
