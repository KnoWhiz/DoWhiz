# Cobrowse SDK Webhooks

- Source URL: https://developers.zoom.us/docs/api/cobrowse-sdk/events/
- Snapshot path: docs/api/cobrowse-sdk/events
- Generated (UTC): 2026-02-07T22:40:10.511097+00:00

## Frontmatter

```json
{
  "title": "Cobrowse SDK Webhooks",
  "keywords": "zoom, rest, api, quality managment",
  "description": "The Zoom Cobrowse SDK enables users to share their web browsing experience with an organization without compromising privacy and security. Collaborators can annotate and see what is being shared. Certain parts of the screen can be masked or redacted from view by the user to protect their privacy and confidentiality.",
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
    "title": "Cobrowse SDK",
    "description": "The Zoom Cobrowse SDK enables users to share their web browsing experience with an organization without compromising privacy and security. Collaborators can annotate and see what is being shared. Certain parts of the screen can be masked or redacted from view by the user to protect their privacy and confidentiality.",
    "version": "1.0.0"
  },
  "paths": {
    "cobrowse.session_started": {
      "post": {
        "tags": [
          "cobrowse"
        ],
        "operationId": "cobrowse.session_started",
        "requestBody": {
          "description": "# cobrowse.session_started\nThe Session Start event is triggered when the first user joins.\n\n## Prerequisites\n\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n  * A valid **Event Notification Endpoint URL**.\n  * The **Start session** subscription enabled under the **Cobrowse SDK** event.\n\n\n**Event type**: `cobrowse.session_started`\n",
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
                    "enum": [
                      "cobrowse.session_started"
                    ],
                    "description": "The name of the event."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format."
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
                        "description": "The account ID."
                      },
                      "object": {
                        "type": "object",
                        "description": "The session's information",
                        "required": [
                          "session_id",
                          "start_time"
                        ],
                        "properties": {
                          "session_id": {
                            "type": "string",
                            "description": "The Cobrowse session's ID."
                          },
                          "start_url": {
                            "type": "string",
                            "description": "The URL the session started from."
                          },
                          "user_id": {
                            "type": "string",
                            "description": "The ID of the user who started the session. This is a unique ID assigned to the user joining a session and is valid for only that session."
                          },
                          "user_name": {
                            "type": "string",
                            "description": "The user's display name."
                          },
                          "role_type": {
                            "type": "string",
                            "description": "The joining user's role type.",
                            "enum": [
                              "customer",
                              "agent"
                            ]
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The session's start time.",
                            "format": "date-time"
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
                  "value": "{\n  \"event\": \"cobrowse.session_started\",\n  \"event_ts\": 1743483350000,\n  \"payload\": {\n    \"account_id\": \"TEODL21kRmehjRceRnyYqw\",\n    \"object\": {\n      \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n      \"start_url\": \"https://sample.zoom.us\",\n      \"user_id\": \"YZ8uRj9zRf2yY5cshmzrTA\",\n      \"user_name\": \"exampleuser\",\n      \"role_type\": \"customer\",\n      \"start_time\": \"2025-04-01T12:22:26Z\"\n    }\n  }\n}"
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
                      "enum": [
                        "cobrowse.session_started"
                      ],
                      "description": "The name of the event."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format."
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
                          "description": "The account ID."
                        },
                        "object": {
                          "type": "object",
                          "description": "The session's information",
                          "required": [
                            "session_id",
                            "start_time"
                          ],
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "description": "The Cobrowse session's ID."
                            },
                            "start_url": {
                              "type": "string",
                              "description": "The URL the session started from."
                            },
                            "user_id": {
                              "type": "string",
                              "description": "The ID of the user who started the session. This is a unique ID assigned to the user joining a session and is valid for only that session."
                            },
                            "user_name": {
                              "type": "string",
                              "description": "The user's display name."
                            },
                            "role_type": {
                              "type": "string",
                              "description": "The joining user's role type.",
                              "enum": [
                                "customer",
                                "agent"
                              ]
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The session's start time.",
                              "format": "date-time"
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
                    "value": "{\n  \"event\": \"cobrowse.session_started\",\n  \"event_ts\": 1743483350000,\n  \"payload\": {\n    \"account_id\": \"TEODL21kRmehjRceRnyYqw\",\n    \"object\": {\n      \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n      \"start_url\": \"https://sample.zoom.us\",\n      \"user_id\": \"YZ8uRj9zRf2yY5cshmzrTA\",\n      \"user_name\": \"exampleuser\",\n      \"role_type\": \"customer\",\n      \"start_time\": \"2025-04-01T12:22:26Z\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "cobrowse.session_user_left": {
      "post": {
        "tags": [
          "cobrowse"
        ],
        "operationId": "cobrowse.session_user_left",
        "requestBody": {
          "description": "# cobrowse.session_user_left\nThe Session User Left event is triggered every time a user leaves a session.\n\n## Prerequisites\n\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n  * A valid **Event Notification Endpoint URL**.\n  * The **User left** subscription enabled under the **Cobrowse SDK** event.\n\n**Event type**: `cobrowse.session_user_left`\n",
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
                    "enum": [
                      "cobrowse.session_user_left"
                    ],
                    "description": "The name of the event."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format."
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
                        "description": "The account ID."
                      },
                      "object": {
                        "type": "object",
                        "description": "The session's information.",
                        "required": [
                          "session_id",
                          "user_connection_id",
                          "user_id",
                          "leave_time"
                        ],
                        "properties": {
                          "session_id": {
                            "type": "string",
                            "description": "The Cobrowse session's ID."
                          },
                          "user_connection_id": {
                            "type": "string",
                            "description": "The connection's ID for each time the user joined the session."
                          },
                          "user_id": {
                            "type": "string",
                            "description": "The user's ID. This is a unique ID assigned to the user joining a session and is valid for only that session."
                          },
                          "user_name": {
                            "type": "string",
                            "description": "The user's display name."
                          },
                          "role_type": {
                            "type": "string",
                            "description": "The joining user's role type.",
                            "enum": [
                              "customer",
                              "agent"
                            ]
                          },
                          "leave_time": {
                            "type": "string",
                            "description": "The time when the user left the session.",
                            "format": "date-time"
                          },
                          "leave_reason": {
                            "type": "string",
                            "description": "The reason why the user left the session.",
                            "default": "Left the session",
                            "enum": [
                              "Left the session",
                              "Got disconnected from the session",
                              "The session end",
                              "Network connection error",
                              "Unknown reason"
                            ]
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
                  "value": "{\n  \"event\": \"cobrowse.session_user_left\",\n  \"event_ts\": 1743483350000,\n  \"payload\": {\n    \"account_id\": \"TEODL21kRmehjRceRnyYqw\",\n    \"object\": {\n      \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n      \"user_connection_id\": \"Nlg8wgA0TFe3jT87CjQZqg\",\n      \"user_id\": \"YZ8uRj9zRf2yY5cshmzrTA\",\n      \"user_name\": \"exampleuser\",\n      \"role_type\": \"customer\",\n      \"leave_time\": \"2025-04-01T12:22:26Z\",\n      \"leave_reason\": \"Left the session\"\n    }\n  }\n}"
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
                      "enum": [
                        "cobrowse.session_user_left"
                      ],
                      "description": "The name of the event."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format."
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
                          "description": "The account ID."
                        },
                        "object": {
                          "type": "object",
                          "description": "The session's information.",
                          "required": [
                            "session_id",
                            "user_connection_id",
                            "user_id",
                            "leave_time"
                          ],
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "description": "The Cobrowse session's ID."
                            },
                            "user_connection_id": {
                              "type": "string",
                              "description": "The connection's ID for each time the user joined the session."
                            },
                            "user_id": {
                              "type": "string",
                              "description": "The user's ID. This is a unique ID assigned to the user joining a session and is valid for only that session."
                            },
                            "user_name": {
                              "type": "string",
                              "description": "The user's display name."
                            },
                            "role_type": {
                              "type": "string",
                              "description": "The joining user's role type.",
                              "enum": [
                                "customer",
                                "agent"
                              ]
                            },
                            "leave_time": {
                              "type": "string",
                              "description": "The time when the user left the session.",
                              "format": "date-time"
                            },
                            "leave_reason": {
                              "type": "string",
                              "description": "The reason why the user left the session.",
                              "default": "Left the session",
                              "enum": [
                                "Left the session",
                                "Got disconnected from the session",
                                "The session end",
                                "Network connection error",
                                "Unknown reason"
                              ]
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
                    "value": "{\n  \"event\": \"cobrowse.session_user_left\",\n  \"event_ts\": 1743483350000,\n  \"payload\": {\n    \"account_id\": \"TEODL21kRmehjRceRnyYqw\",\n    \"object\": {\n      \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n      \"user_connection_id\": \"Nlg8wgA0TFe3jT87CjQZqg\",\n      \"user_id\": \"YZ8uRj9zRf2yY5cshmzrTA\",\n      \"user_name\": \"exampleuser\",\n      \"role_type\": \"customer\",\n      \"leave_time\": \"2025-04-01T12:22:26Z\",\n      \"leave_reason\": \"Left the session\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "cobrowse.session_user_joined": {
      "post": {
        "tags": [
          "cobrowse"
        ],
        "operationId": "cobrowse.session_user_joined",
        "requestBody": {
          "description": "# cobrowse.session_user_joined\nThe Session User Joined event is triggered every time a user joins a session.\n\n## Prerequisites\n\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n  * A valid **Event Notification Endpoint URL**.\n  * The **User joined** subscription enabled under the **Cobrowse SDK** event.\n\n\n**Event type**: `cobrowse.session_user_joined`\n",
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
                    "enum": [
                      "cobrowse.session_user_joined"
                    ],
                    "description": "The name of the event."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format."
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
                        "description": "The account ID."
                      },
                      "object": {
                        "type": "object",
                        "description": "The session's information.",
                        "required": [
                          "session_id",
                          "user_connection_id",
                          "user_id",
                          "join_time"
                        ],
                        "properties": {
                          "session_id": {
                            "type": "string",
                            "description": "The Cobrowse session's ID."
                          },
                          "user_connection_id": {
                            "type": "string",
                            "description": "The connection's ID for each time the user joined the session."
                          },
                          "user_id": {
                            "type": "string",
                            "description": "The user's ID. This is a unique ID assigned to the user joining a session and is valid for only that session."
                          },
                          "user_name": {
                            "type": "string",
                            "description": "The user's display name."
                          },
                          "role_type": {
                            "type": "string",
                            "description": "The joining user's role type.",
                            "enum": [
                              "customer",
                              "agent"
                            ]
                          },
                          "join_time": {
                            "type": "string",
                            "description": "The time when the user joined the session.",
                            "format": "date-time"
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
                  "value": "{\n  \"event\": \"cobrowse.session_user_joined\",\n  \"event_ts\": 1743483350000,\n  \"payload\": {\n    \"account_id\": \"TEODL21kRmehjRceRnyYqw\",\n    \"object\": {\n      \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n      \"user_connection_id\": \"Nlg8wgA0TFe3jT87CjQZqg\",\n      \"user_id\": \"YZ8uRj9zRf2yY5cshmzrTA\",\n      \"user_name\": \"exampleuser\",\n      \"role_type\": \"customer\",\n      \"join_time\": \"2025-04-01T12:22:26Z\"\n    }\n  }\n}"
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
                      "enum": [
                        "cobrowse.session_user_joined"
                      ],
                      "description": "The name of the event."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format."
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
                          "description": "The account ID."
                        },
                        "object": {
                          "type": "object",
                          "description": "The session's information.",
                          "required": [
                            "session_id",
                            "user_connection_id",
                            "user_id",
                            "join_time"
                          ],
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "description": "The Cobrowse session's ID."
                            },
                            "user_connection_id": {
                              "type": "string",
                              "description": "The connection's ID for each time the user joined the session."
                            },
                            "user_id": {
                              "type": "string",
                              "description": "The user's ID. This is a unique ID assigned to the user joining a session and is valid for only that session."
                            },
                            "user_name": {
                              "type": "string",
                              "description": "The user's display name."
                            },
                            "role_type": {
                              "type": "string",
                              "description": "The joining user's role type.",
                              "enum": [
                                "customer",
                                "agent"
                              ]
                            },
                            "join_time": {
                              "type": "string",
                              "description": "The time when the user joined the session.",
                              "format": "date-time"
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
                    "value": "{\n  \"event\": \"cobrowse.session_user_joined\",\n  \"event_ts\": 1743483350000,\n  \"payload\": {\n    \"account_id\": \"TEODL21kRmehjRceRnyYqw\",\n    \"object\": {\n      \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n      \"user_connection_id\": \"Nlg8wgA0TFe3jT87CjQZqg\",\n      \"user_id\": \"YZ8uRj9zRf2yY5cshmzrTA\",\n      \"user_name\": \"exampleuser\",\n      \"role_type\": \"customer\",\n      \"join_time\": \"2025-04-01T12:22:26Z\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "cobrowse.session_ended": {
      "post": {
        "tags": [
          "cobrowse"
        ],
        "operationId": "cobrowse.session_ended",
        "requestBody": {
          "description": "# cobrowse.session_ended\nThe Session End event is triggered when someone ends the session, or when all users have left the session.\n\n## Prerequisites\n\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n  * A valid **Event Notification Endpoint URL**.\n  * The **End session** subscription enabled under the **Cobrowse SDK** event.\n\n**Event type**: `cobrowse.session_ended`\n",
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
                    "enum": [
                      "cobrowse.session_ended"
                    ],
                    "description": "The name of the event."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format."
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
                        "description": "The account ID."
                      },
                      "object": {
                        "type": "object",
                        "description": "The session's information",
                        "required": [
                          "session_id",
                          "end_time"
                        ],
                        "properties": {
                          "session_id": {
                            "type": "string",
                            "description": "The Cobrowse session's ID."
                          },
                          "session_pin": {
                            "type": "string",
                            "description": "The Cobrowse session's pin code that can be used to join the session."
                          },
                          "user_id": {
                            "type": "string",
                            "description": "The ID of the user who ended the session. This is a unique ID assigned to the user joining a session and is valid for only that session."
                          },
                          "user_name": {
                            "type": "string",
                            "description": "The user's display name."
                          },
                          "role_type": {
                            "type": "string",
                            "description": "The joining user's role type.",
                            "enum": [
                              "customer",
                              "agent"
                            ]
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The Cobrowse session's start time.",
                            "format": "date-time"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The Cobrowse session's end time.",
                            "format": "date-time"
                          },
                          "end_reason": {
                            "type": "string",
                            "description": "The reason why the user left the session.",
                            "enum": [
                              "The session ended",
                              "Network connection error",
                              "Unknown reason"
                            ]
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
                  "value": "{\n  \"event\": \"cobrowse.session_ended\",\n  \"event_ts\": 1743483350000,\n  \"payload\": {\n    \"account_id\": \"TEODL21kRmehjRceRnyYqw\",\n    \"object\": {\n      \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n      \"session_pin\": \"987536\",\n      \"user_id\": \"YZ8uRj9zRf2yY5cshmzrTA\",\n      \"user_name\": \"exampleuser\",\n      \"role_type\": \"customer\",\n      \"start_time\": \"2025-04-01T12:22:26Z\",\n      \"end_time\": \"2025-04-01T12:32:26Z\",\n      \"end_reason\": \"The session ended\"\n    }\n  }\n}"
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
                      "enum": [
                        "cobrowse.session_ended"
                      ],
                      "description": "The name of the event."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format."
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
                          "description": "The account ID."
                        },
                        "object": {
                          "type": "object",
                          "description": "The session's information",
                          "required": [
                            "session_id",
                            "end_time"
                          ],
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "description": "The Cobrowse session's ID."
                            },
                            "session_pin": {
                              "type": "string",
                              "description": "The Cobrowse session's pin code that can be used to join the session."
                            },
                            "user_id": {
                              "type": "string",
                              "description": "The ID of the user who ended the session. This is a unique ID assigned to the user joining a session and is valid for only that session."
                            },
                            "user_name": {
                              "type": "string",
                              "description": "The user's display name."
                            },
                            "role_type": {
                              "type": "string",
                              "description": "The joining user's role type.",
                              "enum": [
                                "customer",
                                "agent"
                              ]
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The Cobrowse session's start time.",
                              "format": "date-time"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The Cobrowse session's end time.",
                              "format": "date-time"
                            },
                            "end_reason": {
                              "type": "string",
                              "description": "The reason why the user left the session.",
                              "enum": [
                                "The session ended",
                                "Network connection error",
                                "Unknown reason"
                              ]
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
                    "value": "{\n  \"event\": \"cobrowse.session_ended\",\n  \"event_ts\": 1743483350000,\n  \"payload\": {\n    \"account_id\": \"TEODL21kRmehjRceRnyYqw\",\n    \"object\": {\n      \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n      \"session_pin\": \"987536\",\n      \"user_id\": \"YZ8uRj9zRf2yY5cshmzrTA\",\n      \"user_name\": \"exampleuser\",\n      \"role_type\": \"customer\",\n      \"start_time\": \"2025-04-01T12:22:26Z\",\n      \"end_time\": \"2025-04-01T12:32:26Z\",\n      \"end_reason\": \"The session ended\"\n    }\n  }\n}"
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
    "title": "Cobrowse SDK",
    "description": "The Zoom Cobrowse SDK enables users to share their web browsing experience with an organization without compromising privacy and security. Collaborators can annotate and see what is being shared. Certain parts of the screen can be masked or redacted from view by the user to protect their privacy and confidentiality.",
    "version": "1.0.0"
  },
  "webhooks": {
    "cobrowse.session_started": {
      "post": {
        "tags": [
          "cobrowse"
        ],
        "operationId": "cobrowse.session_started",
        "requestBody": {
          "description": "# cobrowse.session_started\nThe Session Start event is triggered when the first user joins.\n\n## Prerequisites\n\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n  * A valid **Event Notification Endpoint URL**.\n  * The **Start session** subscription enabled under the **Cobrowse SDK** event.\n\n\n**Event type**: `cobrowse.session_started`\n",
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
                    "enum": [
                      "cobrowse.session_started"
                    ],
                    "description": "The name of the event."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format."
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
                        "description": "The account ID."
                      },
                      "object": {
                        "type": "object",
                        "description": "The session's information",
                        "required": [
                          "session_id",
                          "start_time"
                        ],
                        "properties": {
                          "session_id": {
                            "type": "string",
                            "description": "The Cobrowse session's ID."
                          },
                          "start_url": {
                            "type": "string",
                            "description": "The URL the session started from."
                          },
                          "user_id": {
                            "type": "string",
                            "description": "The ID of the user who started the session. This is a unique ID assigned to the user joining a session and is valid for only that session."
                          },
                          "user_name": {
                            "type": "string",
                            "description": "The user's display name."
                          },
                          "role_type": {
                            "type": "string",
                            "description": "The joining user's role type.",
                            "enum": [
                              "customer",
                              "agent"
                            ]
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The session's start time.",
                            "format": "date-time"
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
                  "value": "{\n  \"event\": \"cobrowse.session_started\",\n  \"event_ts\": 1743483350000,\n  \"payload\": {\n    \"account_id\": \"TEODL21kRmehjRceRnyYqw\",\n    \"object\": {\n      \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n      \"start_url\": \"https://sample.zoom.us\",\n      \"user_id\": \"YZ8uRj9zRf2yY5cshmzrTA\",\n      \"user_name\": \"exampleuser\",\n      \"role_type\": \"customer\",\n      \"start_time\": \"2025-04-01T12:22:26Z\"\n    }\n  }\n}"
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
                      "enum": [
                        "cobrowse.session_started"
                      ],
                      "description": "The name of the event."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format."
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
                          "description": "The account ID."
                        },
                        "object": {
                          "type": "object",
                          "description": "The session's information",
                          "required": [
                            "session_id",
                            "start_time"
                          ],
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "description": "The Cobrowse session's ID."
                            },
                            "start_url": {
                              "type": "string",
                              "description": "The URL the session started from."
                            },
                            "user_id": {
                              "type": "string",
                              "description": "The ID of the user who started the session. This is a unique ID assigned to the user joining a session and is valid for only that session."
                            },
                            "user_name": {
                              "type": "string",
                              "description": "The user's display name."
                            },
                            "role_type": {
                              "type": "string",
                              "description": "The joining user's role type.",
                              "enum": [
                                "customer",
                                "agent"
                              ]
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The session's start time.",
                              "format": "date-time"
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
                    "value": "{\n  \"event\": \"cobrowse.session_started\",\n  \"event_ts\": 1743483350000,\n  \"payload\": {\n    \"account_id\": \"TEODL21kRmehjRceRnyYqw\",\n    \"object\": {\n      \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n      \"start_url\": \"https://sample.zoom.us\",\n      \"user_id\": \"YZ8uRj9zRf2yY5cshmzrTA\",\n      \"user_name\": \"exampleuser\",\n      \"role_type\": \"customer\",\n      \"start_time\": \"2025-04-01T12:22:26Z\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "cobrowse.session_user_left": {
      "post": {
        "tags": [
          "cobrowse"
        ],
        "operationId": "cobrowse.session_user_left",
        "requestBody": {
          "description": "# cobrowse.session_user_left\nThe Session User Left event is triggered every time a user leaves a session.\n\n## Prerequisites\n\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n  * A valid **Event Notification Endpoint URL**.\n  * The **User left** subscription enabled under the **Cobrowse SDK** event.\n\n**Event type**: `cobrowse.session_user_left`\n",
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
                    "enum": [
                      "cobrowse.session_user_left"
                    ],
                    "description": "The name of the event."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format."
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
                        "description": "The account ID."
                      },
                      "object": {
                        "type": "object",
                        "description": "The session's information.",
                        "required": [
                          "session_id",
                          "user_connection_id",
                          "user_id",
                          "leave_time"
                        ],
                        "properties": {
                          "session_id": {
                            "type": "string",
                            "description": "The Cobrowse session's ID."
                          },
                          "user_connection_id": {
                            "type": "string",
                            "description": "The connection's ID for each time the user joined the session."
                          },
                          "user_id": {
                            "type": "string",
                            "description": "The user's ID. This is a unique ID assigned to the user joining a session and is valid for only that session."
                          },
                          "user_name": {
                            "type": "string",
                            "description": "The user's display name."
                          },
                          "role_type": {
                            "type": "string",
                            "description": "The joining user's role type.",
                            "enum": [
                              "customer",
                              "agent"
                            ]
                          },
                          "leave_time": {
                            "type": "string",
                            "description": "The time when the user left the session.",
                            "format": "date-time"
                          },
                          "leave_reason": {
                            "type": "string",
                            "description": "The reason why the user left the session.",
                            "default": "Left the session",
                            "enum": [
                              "Left the session",
                              "Got disconnected from the session",
                              "The session end",
                              "Network connection error",
                              "Unknown reason"
                            ]
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
                  "value": "{\n  \"event\": \"cobrowse.session_user_left\",\n  \"event_ts\": 1743483350000,\n  \"payload\": {\n    \"account_id\": \"TEODL21kRmehjRceRnyYqw\",\n    \"object\": {\n      \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n      \"user_connection_id\": \"Nlg8wgA0TFe3jT87CjQZqg\",\n      \"user_id\": \"YZ8uRj9zRf2yY5cshmzrTA\",\n      \"user_name\": \"exampleuser\",\n      \"role_type\": \"customer\",\n      \"leave_time\": \"2025-04-01T12:22:26Z\",\n      \"leave_reason\": \"Left the session\"\n    }\n  }\n}"
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
                      "enum": [
                        "cobrowse.session_user_left"
                      ],
                      "description": "The name of the event."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format."
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
                          "description": "The account ID."
                        },
                        "object": {
                          "type": "object",
                          "description": "The session's information.",
                          "required": [
                            "session_id",
                            "user_connection_id",
                            "user_id",
                            "leave_time"
                          ],
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "description": "The Cobrowse session's ID."
                            },
                            "user_connection_id": {
                              "type": "string",
                              "description": "The connection's ID for each time the user joined the session."
                            },
                            "user_id": {
                              "type": "string",
                              "description": "The user's ID. This is a unique ID assigned to the user joining a session and is valid for only that session."
                            },
                            "user_name": {
                              "type": "string",
                              "description": "The user's display name."
                            },
                            "role_type": {
                              "type": "string",
                              "description": "The joining user's role type.",
                              "enum": [
                                "customer",
                                "agent"
                              ]
                            },
                            "leave_time": {
                              "type": "string",
                              "description": "The time when the user left the session.",
                              "format": "date-time"
                            },
                            "leave_reason": {
                              "type": "string",
                              "description": "The reason why the user left the session.",
                              "default": "Left the session",
                              "enum": [
                                "Left the session",
                                "Got disconnected from the session",
                                "The session end",
                                "Network connection error",
                                "Unknown reason"
                              ]
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
                    "value": "{\n  \"event\": \"cobrowse.session_user_left\",\n  \"event_ts\": 1743483350000,\n  \"payload\": {\n    \"account_id\": \"TEODL21kRmehjRceRnyYqw\",\n    \"object\": {\n      \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n      \"user_connection_id\": \"Nlg8wgA0TFe3jT87CjQZqg\",\n      \"user_id\": \"YZ8uRj9zRf2yY5cshmzrTA\",\n      \"user_name\": \"exampleuser\",\n      \"role_type\": \"customer\",\n      \"leave_time\": \"2025-04-01T12:22:26Z\",\n      \"leave_reason\": \"Left the session\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "cobrowse.session_user_joined": {
      "post": {
        "tags": [
          "cobrowse"
        ],
        "operationId": "cobrowse.session_user_joined",
        "requestBody": {
          "description": "# cobrowse.session_user_joined\nThe Session User Joined event is triggered every time a user joins a session.\n\n## Prerequisites\n\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n  * A valid **Event Notification Endpoint URL**.\n  * The **User joined** subscription enabled under the **Cobrowse SDK** event.\n\n\n**Event type**: `cobrowse.session_user_joined`\n",
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
                    "enum": [
                      "cobrowse.session_user_joined"
                    ],
                    "description": "The name of the event."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format."
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
                        "description": "The account ID."
                      },
                      "object": {
                        "type": "object",
                        "description": "The session's information.",
                        "required": [
                          "session_id",
                          "user_connection_id",
                          "user_id",
                          "join_time"
                        ],
                        "properties": {
                          "session_id": {
                            "type": "string",
                            "description": "The Cobrowse session's ID."
                          },
                          "user_connection_id": {
                            "type": "string",
                            "description": "The connection's ID for each time the user joined the session."
                          },
                          "user_id": {
                            "type": "string",
                            "description": "The user's ID. This is a unique ID assigned to the user joining a session and is valid for only that session."
                          },
                          "user_name": {
                            "type": "string",
                            "description": "The user's display name."
                          },
                          "role_type": {
                            "type": "string",
                            "description": "The joining user's role type.",
                            "enum": [
                              "customer",
                              "agent"
                            ]
                          },
                          "join_time": {
                            "type": "string",
                            "description": "The time when the user joined the session.",
                            "format": "date-time"
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
                  "value": "{\n  \"event\": \"cobrowse.session_user_joined\",\n  \"event_ts\": 1743483350000,\n  \"payload\": {\n    \"account_id\": \"TEODL21kRmehjRceRnyYqw\",\n    \"object\": {\n      \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n      \"user_connection_id\": \"Nlg8wgA0TFe3jT87CjQZqg\",\n      \"user_id\": \"YZ8uRj9zRf2yY5cshmzrTA\",\n      \"user_name\": \"exampleuser\",\n      \"role_type\": \"customer\",\n      \"join_time\": \"2025-04-01T12:22:26Z\"\n    }\n  }\n}"
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
                      "enum": [
                        "cobrowse.session_user_joined"
                      ],
                      "description": "The name of the event."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format."
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
                          "description": "The account ID."
                        },
                        "object": {
                          "type": "object",
                          "description": "The session's information.",
                          "required": [
                            "session_id",
                            "user_connection_id",
                            "user_id",
                            "join_time"
                          ],
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "description": "The Cobrowse session's ID."
                            },
                            "user_connection_id": {
                              "type": "string",
                              "description": "The connection's ID for each time the user joined the session."
                            },
                            "user_id": {
                              "type": "string",
                              "description": "The user's ID. This is a unique ID assigned to the user joining a session and is valid for only that session."
                            },
                            "user_name": {
                              "type": "string",
                              "description": "The user's display name."
                            },
                            "role_type": {
                              "type": "string",
                              "description": "The joining user's role type.",
                              "enum": [
                                "customer",
                                "agent"
                              ]
                            },
                            "join_time": {
                              "type": "string",
                              "description": "The time when the user joined the session.",
                              "format": "date-time"
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
                    "value": "{\n  \"event\": \"cobrowse.session_user_joined\",\n  \"event_ts\": 1743483350000,\n  \"payload\": {\n    \"account_id\": \"TEODL21kRmehjRceRnyYqw\",\n    \"object\": {\n      \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n      \"user_connection_id\": \"Nlg8wgA0TFe3jT87CjQZqg\",\n      \"user_id\": \"YZ8uRj9zRf2yY5cshmzrTA\",\n      \"user_name\": \"exampleuser\",\n      \"role_type\": \"customer\",\n      \"join_time\": \"2025-04-01T12:22:26Z\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "cobrowse.session_ended": {
      "post": {
        "tags": [
          "cobrowse"
        ],
        "operationId": "cobrowse.session_ended",
        "requestBody": {
          "description": "# cobrowse.session_ended\nThe Session End event is triggered when someone ends the session, or when all users have left the session.\n\n## Prerequisites\n\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n  * A valid **Event Notification Endpoint URL**.\n  * The **End session** subscription enabled under the **Cobrowse SDK** event.\n\n**Event type**: `cobrowse.session_ended`\n",
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
                    "enum": [
                      "cobrowse.session_ended"
                    ],
                    "description": "The name of the event."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format."
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
                        "description": "The account ID."
                      },
                      "object": {
                        "type": "object",
                        "description": "The session's information",
                        "required": [
                          "session_id",
                          "end_time"
                        ],
                        "properties": {
                          "session_id": {
                            "type": "string",
                            "description": "The Cobrowse session's ID."
                          },
                          "session_pin": {
                            "type": "string",
                            "description": "The Cobrowse session's pin code that can be used to join the session."
                          },
                          "user_id": {
                            "type": "string",
                            "description": "The ID of the user who ended the session. This is a unique ID assigned to the user joining a session and is valid for only that session."
                          },
                          "user_name": {
                            "type": "string",
                            "description": "The user's display name."
                          },
                          "role_type": {
                            "type": "string",
                            "description": "The joining user's role type.",
                            "enum": [
                              "customer",
                              "agent"
                            ]
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The Cobrowse session's start time.",
                            "format": "date-time"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The Cobrowse session's end time.",
                            "format": "date-time"
                          },
                          "end_reason": {
                            "type": "string",
                            "description": "The reason why the user left the session.",
                            "enum": [
                              "The session ended",
                              "Network connection error",
                              "Unknown reason"
                            ]
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
                  "value": "{\n  \"event\": \"cobrowse.session_ended\",\n  \"event_ts\": 1743483350000,\n  \"payload\": {\n    \"account_id\": \"TEODL21kRmehjRceRnyYqw\",\n    \"object\": {\n      \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n      \"session_pin\": \"987536\",\n      \"user_id\": \"YZ8uRj9zRf2yY5cshmzrTA\",\n      \"user_name\": \"exampleuser\",\n      \"role_type\": \"customer\",\n      \"start_time\": \"2025-04-01T12:22:26Z\",\n      \"end_time\": \"2025-04-01T12:32:26Z\",\n      \"end_reason\": \"The session ended\"\n    }\n  }\n}"
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
                      "enum": [
                        "cobrowse.session_ended"
                      ],
                      "description": "The name of the event."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format."
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
                          "description": "The account ID."
                        },
                        "object": {
                          "type": "object",
                          "description": "The session's information",
                          "required": [
                            "session_id",
                            "end_time"
                          ],
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "description": "The Cobrowse session's ID."
                            },
                            "session_pin": {
                              "type": "string",
                              "description": "The Cobrowse session's pin code that can be used to join the session."
                            },
                            "user_id": {
                              "type": "string",
                              "description": "The ID of the user who ended the session. This is a unique ID assigned to the user joining a session and is valid for only that session."
                            },
                            "user_name": {
                              "type": "string",
                              "description": "The user's display name."
                            },
                            "role_type": {
                              "type": "string",
                              "description": "The joining user's role type.",
                              "enum": [
                                "customer",
                                "agent"
                              ]
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The Cobrowse session's start time.",
                              "format": "date-time"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The Cobrowse session's end time.",
                              "format": "date-time"
                            },
                            "end_reason": {
                              "type": "string",
                              "description": "The reason why the user left the session.",
                              "enum": [
                                "The session ended",
                                "Network connection error",
                                "Unknown reason"
                              ]
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
                    "value": "{\n  \"event\": \"cobrowse.session_ended\",\n  \"event_ts\": 1743483350000,\n  \"payload\": {\n    \"account_id\": \"TEODL21kRmehjRceRnyYqw\",\n    \"object\": {\n      \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n      \"session_pin\": \"987536\",\n      \"user_id\": \"YZ8uRj9zRf2yY5cshmzrTA\",\n      \"user_name\": \"exampleuser\",\n      \"role_type\": \"customer\",\n      \"start_time\": \"2025-04-01T12:22:26Z\",\n      \"end_time\": \"2025-04-01T12:32:26Z\",\n      \"end_reason\": \"The session ended\"\n    }\n  }\n}"
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
      "cobrowse-sdk",
      "events"
    ]
  },
  "downloadPath": "/api-hub/cobrowse-sdk/events/webhooks.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": false
  },
  "productMeta": {
    "specType": "adoc",
    "slug": "/docs/api/rest/reference/cobrowse-sdk/events",
    "title": "Cobrowse SDK",
    "description": "The Zoom Cobrowse SDK enables users to share their web browsing experience with an organization without compromising privacy and security. Collaborators can annotate and see what is being shared. Certain parts of the screen can be masked or redacted from view by the user to protect their privacy and confidentiality.",
    "keywords": "zoom, rest, api, quality managment",
    "params": {
      "productName": "Cobrowse SDK"
    },
    "zoapEndpoint": "webhookByProduct",
    "productDisplayTitle": "Cobrowse SDK",
    "scalarSlug": "/docs/api/cobrowse-sdk/events",
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "svgname": "cobrowse-sdk"
  },
  "frontmatter": {
    "title": "Cobrowse SDK Webhooks",
    "keywords": "zoom, rest, api, quality managment",
    "description": "The Zoom Cobrowse SDK enables users to share their web browsing experience with an organization without compromising privacy and security. Collaborators can annotate and see what is being shared. Certain parts of the screen can be masked or redacted from view by the user to protect their privacy and confidentiality.",
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
        "cobrowse-sdk",
        "events"
      ]
    },
    "downloadPath": "/api-hub/cobrowse-sdk/events/webhooks.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": false
    },
    "productMeta": {
      "specType": "adoc",
      "slug": "/docs/api/rest/reference/cobrowse-sdk/events",
      "title": "Cobrowse SDK",
      "description": "The Zoom Cobrowse SDK enables users to share their web browsing experience with an organization without compromising privacy and security. Collaborators can annotate and see what is being shared. Certain parts of the screen can be masked or redacted from view by the user to protect their privacy and confidentiality.",
      "keywords": "zoom, rest, api, quality managment",
      "params": {
        "productName": "Cobrowse SDK"
      },
      "zoapEndpoint": "webhookByProduct",
      "productDisplayTitle": "Cobrowse SDK",
      "scalarSlug": "/docs/api/cobrowse-sdk/events",
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "svgname": "cobrowse-sdk"
    },
    "spec": "{\"openapi\":\"3.1.0\",\"info\":{\"title\":\"Cobrowse SDK\",\"description\":\"The Zoom Cobrowse SDK enables users to share their web browsing experience with an organization without compromising privacy and security. Collaborators can annotate and see what is being shared. Certain parts of the screen can be masked or redacted from view by the user to protect their privacy and confidentiality.\",\"version\":\"1.0.0\"},\"paths\":{\"cobrowse.session_started\":{\"post\":{\"tags\":[\"cobrowse\"],\"operationId\":\"cobrowse.session_started\",\"requestBody\":{\"description\":\"# cobrowse.session_started\\nThe Session Start event is triggered when the first user joins.\\n\\n## Prerequisites\\n\\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\\n  * A valid **Event Notification Endpoint URL**.\\n  * The **Start session** subscription enabled under the **Cobrowse SDK** event.\\n\\n\\n**Event type**: `cobrowse.session_started`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"enum\":[\"cobrowse.session_started\"],\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID.\"},\"object\":{\"type\":\"object\",\"description\":\"The session's information\",\"required\":[\"session_id\",\"start_time\"],\"properties\":{\"session_id\":{\"type\":\"string\",\"description\":\"The Cobrowse session's ID.\"},\"start_url\":{\"type\":\"string\",\"description\":\"The URL the session started from.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The ID of the user who started the session. This is a unique ID assigned to the user joining a session and is valid for only that session.\"},\"user_name\":{\"type\":\"string\",\"description\":\"The user's display name.\"},\"role_type\":{\"type\":\"string\",\"description\":\"The joining user's role type.\",\"enum\":[\"customer\",\"agent\"]},\"start_time\":{\"type\":\"string\",\"description\":\"The session's start time.\",\"format\":\"date-time\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"cobrowse.session_started\\\",\\n  \\\"event_ts\\\": 1743483350000,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"TEODL21kRmehjRceRnyYqw\\\",\\n    \\\"object\\\": {\\n      \\\"session_id\\\": \\\"4567UVWxYZABCdEfGhiJkl==\\\",\\n      \\\"start_url\\\": \\\"https://sample.zoom.us\\\",\\n      \\\"user_id\\\": \\\"YZ8uRj9zRf2yY5cshmzrTA\\\",\\n      \\\"user_name\\\": \\\"exampleuser\\\",\\n      \\\"role_type\\\": \\\"customer\\\",\\n      \\\"start_time\\\": \\\"2025-04-01T12:22:26Z\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"enum\":[\"cobrowse.session_started\"],\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID.\"},\"object\":{\"type\":\"object\",\"description\":\"The session's information\",\"required\":[\"session_id\",\"start_time\"],\"properties\":{\"session_id\":{\"type\":\"string\",\"description\":\"The Cobrowse session's ID.\"},\"start_url\":{\"type\":\"string\",\"description\":\"The URL the session started from.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The ID of the user who started the session. This is a unique ID assigned to the user joining a session and is valid for only that session.\"},\"user_name\":{\"type\":\"string\",\"description\":\"The user's display name.\"},\"role_type\":{\"type\":\"string\",\"description\":\"The joining user's role type.\",\"enum\":[\"customer\",\"agent\"]},\"start_time\":{\"type\":\"string\",\"description\":\"The session's start time.\",\"format\":\"date-time\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"cobrowse.session_started\\\",\\n  \\\"event_ts\\\": 1743483350000,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"TEODL21kRmehjRceRnyYqw\\\",\\n    \\\"object\\\": {\\n      \\\"session_id\\\": \\\"4567UVWxYZABCdEfGhiJkl==\\\",\\n      \\\"start_url\\\": \\\"https://sample.zoom.us\\\",\\n      \\\"user_id\\\": \\\"YZ8uRj9zRf2yY5cshmzrTA\\\",\\n      \\\"user_name\\\": \\\"exampleuser\\\",\\n      \\\"role_type\\\": \\\"customer\\\",\\n      \\\"start_time\\\": \\\"2025-04-01T12:22:26Z\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"cobrowse.session_user_left\":{\"post\":{\"tags\":[\"cobrowse\"],\"operationId\":\"cobrowse.session_user_left\",\"requestBody\":{\"description\":\"# cobrowse.session_user_left\\nThe Session User Left event is triggered every time a user leaves a session.\\n\\n## Prerequisites\\n\\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\\n  * A valid **Event Notification Endpoint URL**.\\n  * The **User left** subscription enabled under the **Cobrowse SDK** event.\\n\\n**Event type**: `cobrowse.session_user_left`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"enum\":[\"cobrowse.session_user_left\"],\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID.\"},\"object\":{\"type\":\"object\",\"description\":\"The session's information.\",\"required\":[\"session_id\",\"user_connection_id\",\"user_id\",\"leave_time\"],\"properties\":{\"session_id\":{\"type\":\"string\",\"description\":\"The Cobrowse session's ID.\"},\"user_connection_id\":{\"type\":\"string\",\"description\":\"The connection's ID for each time the user joined the session.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The user's ID. This is a unique ID assigned to the user joining a session and is valid for only that session.\"},\"user_name\":{\"type\":\"string\",\"description\":\"The user's display name.\"},\"role_type\":{\"type\":\"string\",\"description\":\"The joining user's role type.\",\"enum\":[\"customer\",\"agent\"]},\"leave_time\":{\"type\":\"string\",\"description\":\"The time when the user left the session.\",\"format\":\"date-time\"},\"leave_reason\":{\"type\":\"string\",\"description\":\"The reason why the user left the session.\",\"default\":\"Left the session\",\"enum\":[\"Left the session\",\"Got disconnected from the session\",\"The session end\",\"Network connection error\",\"Unknown reason\"]}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"cobrowse.session_user_left\\\",\\n  \\\"event_ts\\\": 1743483350000,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"TEODL21kRmehjRceRnyYqw\\\",\\n    \\\"object\\\": {\\n      \\\"session_id\\\": \\\"4567UVWxYZABCdEfGhiJkl==\\\",\\n      \\\"user_connection_id\\\": \\\"Nlg8wgA0TFe3jT87CjQZqg\\\",\\n      \\\"user_id\\\": \\\"YZ8uRj9zRf2yY5cshmzrTA\\\",\\n      \\\"user_name\\\": \\\"exampleuser\\\",\\n      \\\"role_type\\\": \\\"customer\\\",\\n      \\\"leave_time\\\": \\\"2025-04-01T12:22:26Z\\\",\\n      \\\"leave_reason\\\": \\\"Left the session\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"enum\":[\"cobrowse.session_user_left\"],\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID.\"},\"object\":{\"type\":\"object\",\"description\":\"The session's information.\",\"required\":[\"session_id\",\"user_connection_id\",\"user_id\",\"leave_time\"],\"properties\":{\"session_id\":{\"type\":\"string\",\"description\":\"The Cobrowse session's ID.\"},\"user_connection_id\":{\"type\":\"string\",\"description\":\"The connection's ID for each time the user joined the session.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The user's ID. This is a unique ID assigned to the user joining a session and is valid for only that session.\"},\"user_name\":{\"type\":\"string\",\"description\":\"The user's display name.\"},\"role_type\":{\"type\":\"string\",\"description\":\"The joining user's role type.\",\"enum\":[\"customer\",\"agent\"]},\"leave_time\":{\"type\":\"string\",\"description\":\"The time when the user left the session.\",\"format\":\"date-time\"},\"leave_reason\":{\"type\":\"string\",\"description\":\"The reason why the user left the session.\",\"default\":\"Left the session\",\"enum\":[\"Left the session\",\"Got disconnected from the session\",\"The session end\",\"Network connection error\",\"Unknown reason\"]}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"cobrowse.session_user_left\\\",\\n  \\\"event_ts\\\": 1743483350000,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"TEODL21kRmehjRceRnyYqw\\\",\\n    \\\"object\\\": {\\n      \\\"session_id\\\": \\\"4567UVWxYZABCdEfGhiJkl==\\\",\\n      \\\"user_connection_id\\\": \\\"Nlg8wgA0TFe3jT87CjQZqg\\\",\\n      \\\"user_id\\\": \\\"YZ8uRj9zRf2yY5cshmzrTA\\\",\\n      \\\"user_name\\\": \\\"exampleuser\\\",\\n      \\\"role_type\\\": \\\"customer\\\",\\n      \\\"leave_time\\\": \\\"2025-04-01T12:22:26Z\\\",\\n      \\\"leave_reason\\\": \\\"Left the session\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"cobrowse.session_user_joined\":{\"post\":{\"tags\":[\"cobrowse\"],\"operationId\":\"cobrowse.session_user_joined\",\"requestBody\":{\"description\":\"# cobrowse.session_user_joined\\nThe Session User Joined event is triggered every time a user joins a session.\\n\\n## Prerequisites\\n\\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\\n  * A valid **Event Notification Endpoint URL**.\\n  * The **User joined** subscription enabled under the **Cobrowse SDK** event.\\n\\n\\n**Event type**: `cobrowse.session_user_joined`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"enum\":[\"cobrowse.session_user_joined\"],\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID.\"},\"object\":{\"type\":\"object\",\"description\":\"The session's information.\",\"required\":[\"session_id\",\"user_connection_id\",\"user_id\",\"join_time\"],\"properties\":{\"session_id\":{\"type\":\"string\",\"description\":\"The Cobrowse session's ID.\"},\"user_connection_id\":{\"type\":\"string\",\"description\":\"The connection's ID for each time the user joined the session.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The user's ID. This is a unique ID assigned to the user joining a session and is valid for only that session.\"},\"user_name\":{\"type\":\"string\",\"description\":\"The user's display name.\"},\"role_type\":{\"type\":\"string\",\"description\":\"The joining user's role type.\",\"enum\":[\"customer\",\"agent\"]},\"join_time\":{\"type\":\"string\",\"description\":\"The time when the user joined the session.\",\"format\":\"date-time\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"cobrowse.session_user_joined\\\",\\n  \\\"event_ts\\\": 1743483350000,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"TEODL21kRmehjRceRnyYqw\\\",\\n    \\\"object\\\": {\\n      \\\"session_id\\\": \\\"4567UVWxYZABCdEfGhiJkl==\\\",\\n      \\\"user_connection_id\\\": \\\"Nlg8wgA0TFe3jT87CjQZqg\\\",\\n      \\\"user_id\\\": \\\"YZ8uRj9zRf2yY5cshmzrTA\\\",\\n      \\\"user_name\\\": \\\"exampleuser\\\",\\n      \\\"role_type\\\": \\\"customer\\\",\\n      \\\"join_time\\\": \\\"2025-04-01T12:22:26Z\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"enum\":[\"cobrowse.session_user_joined\"],\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID.\"},\"object\":{\"type\":\"object\",\"description\":\"The session's information.\",\"required\":[\"session_id\",\"user_connection_id\",\"user_id\",\"join_time\"],\"properties\":{\"session_id\":{\"type\":\"string\",\"description\":\"The Cobrowse session's ID.\"},\"user_connection_id\":{\"type\":\"string\",\"description\":\"The connection's ID for each time the user joined the session.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The user's ID. This is a unique ID assigned to the user joining a session and is valid for only that session.\"},\"user_name\":{\"type\":\"string\",\"description\":\"The user's display name.\"},\"role_type\":{\"type\":\"string\",\"description\":\"The joining user's role type.\",\"enum\":[\"customer\",\"agent\"]},\"join_time\":{\"type\":\"string\",\"description\":\"The time when the user joined the session.\",\"format\":\"date-time\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"cobrowse.session_user_joined\\\",\\n  \\\"event_ts\\\": 1743483350000,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"TEODL21kRmehjRceRnyYqw\\\",\\n    \\\"object\\\": {\\n      \\\"session_id\\\": \\\"4567UVWxYZABCdEfGhiJkl==\\\",\\n      \\\"user_connection_id\\\": \\\"Nlg8wgA0TFe3jT87CjQZqg\\\",\\n      \\\"user_id\\\": \\\"YZ8uRj9zRf2yY5cshmzrTA\\\",\\n      \\\"user_name\\\": \\\"exampleuser\\\",\\n      \\\"role_type\\\": \\\"customer\\\",\\n      \\\"join_time\\\": \\\"2025-04-01T12:22:26Z\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"cobrowse.session_ended\":{\"post\":{\"tags\":[\"cobrowse\"],\"operationId\":\"cobrowse.session_ended\",\"requestBody\":{\"description\":\"# cobrowse.session_ended\\nThe Session End event is triggered when someone ends the session, or when all users have left the session.\\n\\n## Prerequisites\\n\\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\\n  * A valid **Event Notification Endpoint URL**.\\n  * The **End session** subscription enabled under the **Cobrowse SDK** event.\\n\\n**Event type**: `cobrowse.session_ended`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"enum\":[\"cobrowse.session_ended\"],\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID.\"},\"object\":{\"type\":\"object\",\"description\":\"The session's information\",\"required\":[\"session_id\",\"end_time\"],\"properties\":{\"session_id\":{\"type\":\"string\",\"description\":\"The Cobrowse session's ID.\"},\"session_pin\":{\"type\":\"string\",\"description\":\"The Cobrowse session's pin code that can be used to join the session.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The ID of the user who ended the session. This is a unique ID assigned to the user joining a session and is valid for only that session.\"},\"user_name\":{\"type\":\"string\",\"description\":\"The user's display name.\"},\"role_type\":{\"type\":\"string\",\"description\":\"The joining user's role type.\",\"enum\":[\"customer\",\"agent\"]},\"start_time\":{\"type\":\"string\",\"description\":\"The Cobrowse session's start time.\",\"format\":\"date-time\"},\"end_time\":{\"type\":\"string\",\"description\":\"The Cobrowse session's end time.\",\"format\":\"date-time\"},\"end_reason\":{\"type\":\"string\",\"description\":\"The reason why the user left the session.\",\"enum\":[\"The session ended\",\"Network connection error\",\"Unknown reason\"]}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"cobrowse.session_ended\\\",\\n  \\\"event_ts\\\": 1743483350000,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"TEODL21kRmehjRceRnyYqw\\\",\\n    \\\"object\\\": {\\n      \\\"session_id\\\": \\\"4567UVWxYZABCdEfGhiJkl==\\\",\\n      \\\"session_pin\\\": \\\"987536\\\",\\n      \\\"user_id\\\": \\\"YZ8uRj9zRf2yY5cshmzrTA\\\",\\n      \\\"user_name\\\": \\\"exampleuser\\\",\\n      \\\"role_type\\\": \\\"customer\\\",\\n      \\\"start_time\\\": \\\"2025-04-01T12:22:26Z\\\",\\n      \\\"end_time\\\": \\\"2025-04-01T12:32:26Z\\\",\\n      \\\"end_reason\\\": \\\"The session ended\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"enum\":[\"cobrowse.session_ended\"],\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"Timestamp (in milliseconds since epoch) when the event occurred. The value of this field is returned in long (int64) format.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID.\"},\"object\":{\"type\":\"object\",\"description\":\"The session's information\",\"required\":[\"session_id\",\"end_time\"],\"properties\":{\"session_id\":{\"type\":\"string\",\"description\":\"The Cobrowse session's ID.\"},\"session_pin\":{\"type\":\"string\",\"description\":\"The Cobrowse session's pin code that can be used to join the session.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The ID of the user who ended the session. This is a unique ID assigned to the user joining a session and is valid for only that session.\"},\"user_name\":{\"type\":\"string\",\"description\":\"The user's display name.\"},\"role_type\":{\"type\":\"string\",\"description\":\"The joining user's role type.\",\"enum\":[\"customer\",\"agent\"]},\"start_time\":{\"type\":\"string\",\"description\":\"The Cobrowse session's start time.\",\"format\":\"date-time\"},\"end_time\":{\"type\":\"string\",\"description\":\"The Cobrowse session's end time.\",\"format\":\"date-time\"},\"end_reason\":{\"type\":\"string\",\"description\":\"The reason why the user left the session.\",\"enum\":[\"The session ended\",\"Network connection error\",\"Unknown reason\"]}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"cobrowse.session_ended\\\",\\n  \\\"event_ts\\\": 1743483350000,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"TEODL21kRmehjRceRnyYqw\\\",\\n    \\\"object\\\": {\\n      \\\"session_id\\\": \\\"4567UVWxYZABCdEfGhiJkl==\\\",\\n      \\\"session_pin\\\": \\\"987536\\\",\\n      \\\"user_id\\\": \\\"YZ8uRj9zRf2yY5cshmzrTA\\\",\\n      \\\"user_name\\\": \\\"exampleuser\\\",\\n      \\\"role_type\\\": \\\"customer\\\",\\n      \\\"start_time\\\": \\\"2025-04-01T12:22:26Z\\\",\\n      \\\"end_time\\\": \\\"2025-04-01T12:32:26Z\\\",\\n      \\\"end_reason\\\": \\\"The session ended\\\"\\n    }\\n  }\\n}\"}}}}}}}}},\"servers\":[{\"url\":\"https://{your-endpoint-url}\"}]}",
    "frontmatter": {
      "title": "Cobrowse SDK Webhooks",
      "keywords": "zoom, rest, api, quality managment",
      "description": "The Zoom Cobrowse SDK enables users to share their web browsing experience with an organization without compromising privacy and security. Collaborators can annotate and see what is being shared. Certain parts of the screen can be masked or redacted from view by the user to protect their privacy and confidentiality.",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
