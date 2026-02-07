# Revenue Accelerator Webhooks

- Source URL: https://developers.zoom.us/docs/api/iq/events/
- Snapshot path: docs/api/iq/events
- Generated (UTC): 2026-02-07T22:40:10.822191+00:00

## Frontmatter

```json
{
  "title": "Revenue Accelerator Webhooks",
  "keywords": "zoom, api, iq",
  "description": "Revenue Accelerator events",
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
    "title": "Zoom Revenue Accelerator Webhooks",
    "description": "",
    "version": "1.0.0"
  },
  "paths": {
    "revenue_accelerator.conversation_analysis_completed": {
      "post": {
        "tags": [
          "revenue_accelerator"
        ],
        "operationId": "revenue_accelerator.conversation_analysis_completed",
        "requestBody": {
          "description": "# revenue_accelerator.conversation_analysis_completed\nZoom triggers this event after the ZRA users conversation analysis is completed.\n## Prerequisites\n* A Pro or higher plan.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**\n\t* The **Revenue Accelerator conversations analysis completed** subscription must be enabled under the **Revenue Accelerator** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `iq_conversation:read:admin`,`iq_conversation:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zra:read:conversations`,`zra:read:conversations:admin`\n\n**Event type**: `revenue_accelerator.conversation_analysis_completed`\n",
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
                        "description": "The account ID of the user who recorded the conversation."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the conversation.",
                        "required": [
                          "conversation_id"
                        ],
                        "properties": {
                          "conversation_id": {
                            "type": "string",
                            "description": "The conversation's ID."
                          },
                          "source": {
                            "type": "string",
                            "description": "The conversation's source.",
                            "enum": [
                              "ZOOM_MEETING",
                              "ZOOM_PHONE",
                              "ZOOM_CONTACT_CENTER",
                              "MICROSOFT_TEAMS",
                              "GOOGLE_MEET",
                              "OTHERS"
                            ]
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The conversation host's Zoom user ID."
                          },
                          "engagement_id": {
                            "type": "string",
                            "description": "The ID of the engagement in Zoom Contact Center (ZCC) that this conversation is associated with. Present only when the conversation source is ZOOM_CONTACT_CENTER."
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
                  "value": "{\n  \"event\": \"revenue_accelerator.conversation_analysis_completed \",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"conversation_id\": \"abcD3ojfdbjfg\",\n      \"source\": \"ZOOM_MEETING\",\n      \"host_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\"\n    }\n  }\n}"
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
                          "description": "The account ID of the user who recorded the conversation."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the conversation.",
                          "required": [
                            "conversation_id"
                          ],
                          "properties": {
                            "conversation_id": {
                              "type": "string",
                              "description": "The conversation's ID."
                            },
                            "source": {
                              "type": "string",
                              "description": "The conversation's source.",
                              "enum": [
                                "ZOOM_MEETING",
                                "ZOOM_PHONE",
                                "ZOOM_CONTACT_CENTER",
                                "MICROSOFT_TEAMS",
                                "GOOGLE_MEET",
                                "OTHERS"
                              ]
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The conversation host's Zoom user ID."
                            },
                            "engagement_id": {
                              "type": "string",
                              "description": "The ID of the engagement in Zoom Contact Center (ZCC) that this conversation is associated with. Present only when the conversation source is ZOOM_CONTACT_CENTER."
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
                    "value": "{\n  \"event\": \"revenue_accelerator.conversation_analysis_completed \",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"conversation_id\": \"abcD3ojfdbjfg\",\n      \"source\": \"ZOOM_MEETING\",\n      \"host_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "revenue_accelerator.playlist_remove_moment": {
      "post": {
        "tags": [
          "revenue_accelerator"
        ],
        "operationId": "revenue_accelerator.playlist_remove_moment",
        "requestBody": {
          "description": "# revenue_accelerator.playlist_remove_moment\nZoom triggers this event when a ZRA user removes a moment from a playlist.\n## Prerequisites\n* A Pro or higher plan.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**\n\t* The **Remove Moment From Playlist** subscription must be enabled under the **Revenue Accelerator** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `iq_playlist:read:admin`,`iq_playlist:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zra:read:list_conversation_playlists`,`zra:read:list_conversation_playlists:admin`\n\n**Event type**: `revenue_accelerator.playlist_remove_moment`\n",
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
                    "description": "The timestamp when the event occurred."
                  },
                  "payload": {
                    "required": [
                      "account_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who removed the moment."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The ID of the user who added the moment to the playlist."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The identifier of the user who added the moment to the playlist, represented by email."
                      },
                      "object": {
                        "type": "object",
                        "description": "Details about the removed moment.",
                        "required": [
                          "moment_id",
                          "playlist_id"
                        ],
                        "properties": {
                          "playlist_id": {
                            "type": "string",
                            "description": "The ID of the playlist from which the moment was removed."
                          },
                          "playlist_name": {
                            "type": "string",
                            "description": "The name of the playlist from which the moment was removed."
                          },
                          "playlist_detail_id": {
                            "type": "string",
                            "description": "The ID representing the relationship between the playlist and the moment."
                          },
                          "moment_id": {
                            "type": "string",
                            "description": "The ID of the removed moment."
                          },
                          "moment_name": {
                            "type": "string",
                            "description": "The name of the removed moment."
                          },
                          "update_time": {
                            "type": "integer",
                            "description": "The timestamp (in milliseconds) when the moment was removed from the playlist."
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
                  "value": "{\n  \"event\": \"revenue_accelerator.playlist_remove_moment\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test@zoom.us\",\n    \"object\": {\n      \"playlist_id\": \"AAAAAABBBB\",\n      \"playlist_name\": \"playlist name\",\n      \"playlist_detail_id\": \"323Tl2Ra0ASfO5wIzYPYceiA\",\n      \"moment_id\": \"ASfO5wIzYPYceiA\",\n      \"moment_name\": \"moment name\",\n      \"update_time\": 1755591952177\n    }\n  }\n}"
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
                      "description": "The timestamp when the event occurred."
                    },
                    "payload": {
                      "required": [
                        "account_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who removed the moment."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The ID of the user who added the moment to the playlist."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The identifier of the user who added the moment to the playlist, represented by email."
                        },
                        "object": {
                          "type": "object",
                          "description": "Details about the removed moment.",
                          "required": [
                            "moment_id",
                            "playlist_id"
                          ],
                          "properties": {
                            "playlist_id": {
                              "type": "string",
                              "description": "The ID of the playlist from which the moment was removed."
                            },
                            "playlist_name": {
                              "type": "string",
                              "description": "The name of the playlist from which the moment was removed."
                            },
                            "playlist_detail_id": {
                              "type": "string",
                              "description": "The ID representing the relationship between the playlist and the moment."
                            },
                            "moment_id": {
                              "type": "string",
                              "description": "The ID of the removed moment."
                            },
                            "moment_name": {
                              "type": "string",
                              "description": "The name of the removed moment."
                            },
                            "update_time": {
                              "type": "integer",
                              "description": "The timestamp (in milliseconds) when the moment was removed from the playlist."
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
                    "value": "{\n  \"event\": \"revenue_accelerator.playlist_remove_moment\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test@zoom.us\",\n    \"object\": {\n      \"playlist_id\": \"AAAAAABBBB\",\n      \"playlist_name\": \"playlist name\",\n      \"playlist_detail_id\": \"323Tl2Ra0ASfO5wIzYPYceiA\",\n      \"moment_id\": \"ASfO5wIzYPYceiA\",\n      \"moment_name\": \"moment name\",\n      \"update_time\": 1755591952177\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "revenue_accelerator.playlist_add_moment": {
      "post": {
        "tags": [
          "revenue_accelerator"
        ],
        "operationId": "revenue_accelerator.playlist_add_moment",
        "requestBody": {
          "description": "# revenue_accelerator.playlist_add_moment\nZoom triggers this event when a ZRA user adds a moment to a playlist.\n## Prerequisites\n* A Pro or higher plan.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**\n\t* The **Add Moment To Playlist** subscription must be enabled under the **Revenue Accelerator** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `iq_playlist:read:admin`,`iq_playlist:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zra:read:list_conversation_playlists`,`zra:read:list_conversation_playlists:admin`\n\n**Event type**: `revenue_accelerator.playlist_add_moment`\n",
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
                    "description": "The timestamp when the event occurred."
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
                        "description": "The account ID of the user who added the moment to the playlist."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The ID of the user who added the moment to the playlist."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The identifier of the user who added the moment to the playlist, represented by email."
                      },
                      "object": {
                        "type": "object",
                        "description": "Details about adding the moment to the playlist.",
                        "required": [
                          "moment_id",
                          "playlist_id"
                        ],
                        "properties": {
                          "playlist_id": {
                            "type": "string",
                            "description": "The ID of the playlist the moment was added to."
                          },
                          "playlist_name": {
                            "type": "string",
                            "description": "The name of the playlist the moment was added to."
                          },
                          "playlist_detail_id": {
                            "type": "string",
                            "description": "The ID representing the relationship between the playlist and the moment."
                          },
                          "moment_id": {
                            "type": "string",
                            "description": "The ID of the moment that was added to the playlist."
                          },
                          "moment_name": {
                            "type": "string",
                            "description": "The name of the moment that was added to the playlist."
                          },
                          "update_time": {
                            "type": "integer",
                            "description": "The timestamp (in milliseconds) when the moment was added to the playlist."
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
                  "value": "{\n  \"event\": \"revenue_accelerator.playlist_add_moment\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test@zoom.us\",\n    \"object\": {\n      \"playlist_id\": \"AAAAAABBBB\",\n      \"playlist_name\": \"playlist name\",\n      \"playlist_detail_id\": \"323Tl2Ra0ASfO5wIzYPYceiA\",\n      \"moment_id\": \"ASfO5wIzYPYceiA\",\n      \"moment_name\": \"moment name\",\n      \"update_time\": 1755591952177\n    }\n  }\n}"
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
                      "description": "The timestamp when the event occurred."
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
                          "description": "The account ID of the user who added the moment to the playlist."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The ID of the user who added the moment to the playlist."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The identifier of the user who added the moment to the playlist, represented by email."
                        },
                        "object": {
                          "type": "object",
                          "description": "Details about adding the moment to the playlist.",
                          "required": [
                            "moment_id",
                            "playlist_id"
                          ],
                          "properties": {
                            "playlist_id": {
                              "type": "string",
                              "description": "The ID of the playlist the moment was added to."
                            },
                            "playlist_name": {
                              "type": "string",
                              "description": "The name of the playlist the moment was added to."
                            },
                            "playlist_detail_id": {
                              "type": "string",
                              "description": "The ID representing the relationship between the playlist and the moment."
                            },
                            "moment_id": {
                              "type": "string",
                              "description": "The ID of the moment that was added to the playlist."
                            },
                            "moment_name": {
                              "type": "string",
                              "description": "The name of the moment that was added to the playlist."
                            },
                            "update_time": {
                              "type": "integer",
                              "description": "The timestamp (in milliseconds) when the moment was added to the playlist."
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
                    "value": "{\n  \"event\": \"revenue_accelerator.playlist_add_moment\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test@zoom.us\",\n    \"object\": {\n      \"playlist_id\": \"AAAAAABBBB\",\n      \"playlist_name\": \"playlist name\",\n      \"playlist_detail_id\": \"323Tl2Ra0ASfO5wIzYPYceiA\",\n      \"moment_id\": \"ASfO5wIzYPYceiA\",\n      \"moment_name\": \"moment name\",\n      \"update_time\": 1755591952177\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "revenue_accelerator.playlist_create": {
      "post": {
        "tags": [
          "revenue_accelerator"
        ],
        "operationId": "revenue_accelerator.playlist_create",
        "requestBody": {
          "description": "# revenue_accelerator.playlist_create\nZoom triggers this event when a ZRA user creates a playlist.\n## Prerequisites\n* A Pro or higher plan.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**\n\t* The **Playlist Created** subscription must be enabled under the **Revenue Accelerator** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `iq_playlist:read:admin`,`iq_playlist:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zra:read:list_conversation_playlists`,`zra:read:list_conversation_playlists:admin`\n\n**Event type**: `revenue_accelerator.playlist_create`\n",
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
                    "description": "The timestamp when the event occurred."
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
                        "description": "The account ID of the user who created the playlist."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The ID of the user who created the playlist."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The identifier of the user who created the playlist, represented by email."
                      },
                      "object": {
                        "type": "object",
                        "description": "Details about the created playlist.",
                        "required": [
                          "playlist_id",
                          "playlist_type",
                          "playlist_name"
                        ],
                        "properties": {
                          "playlist_id": {
                            "type": "string",
                            "description": "The ID of the created playlist."
                          },
                          "playlist_type": {
                            "type": "string",
                            "description": "The type of playlist created.",
                            "enum": [
                              "BASIC",
                              "SMART"
                            ]
                          },
                          "playlist_name": {
                            "type": "string",
                            "description": "The name of the created playlist."
                          },
                          "create_time": {
                            "type": "integer",
                            "description": "The creation timestamp, in milliseconds."
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
                  "value": "{\n  \"event\": \"revenue_accelerator.playlist_create\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"account_id\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test@zoom.us\",\n    \"object\": {\n      \"playlist_id\": \"AAAAAABBBB\",\n      \"playlist_type\": \"SMART\",\n      \"playlist_name\": \"playlist name\",\n      \"create_time\": 1755591952177\n    }\n  }\n}"
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
                      "description": "The timestamp when the event occurred."
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
                          "description": "The account ID of the user who created the playlist."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The ID of the user who created the playlist."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The identifier of the user who created the playlist, represented by email."
                        },
                        "object": {
                          "type": "object",
                          "description": "Details about the created playlist.",
                          "required": [
                            "playlist_id",
                            "playlist_type",
                            "playlist_name"
                          ],
                          "properties": {
                            "playlist_id": {
                              "type": "string",
                              "description": "The ID of the created playlist."
                            },
                            "playlist_type": {
                              "type": "string",
                              "description": "The type of playlist created.",
                              "enum": [
                                "BASIC",
                                "SMART"
                              ]
                            },
                            "playlist_name": {
                              "type": "string",
                              "description": "The name of the created playlist."
                            },
                            "create_time": {
                              "type": "integer",
                              "description": "The creation timestamp, in milliseconds."
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
                    "value": "{\n  \"event\": \"revenue_accelerator.playlist_create\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"account_id\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test@zoom.us\",\n    \"object\": {\n      \"playlist_id\": \"AAAAAABBBB\",\n      \"playlist_type\": \"SMART\",\n      \"playlist_name\": \"playlist name\",\n      \"create_time\": 1755591952177\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "revenue_accelerator.post_comment_completed": {
      "post": {
        "tags": [
          "revenue_accelerator"
        ],
        "operationId": "revenue_accelerator.post_comment_completed",
        "requestBody": {
          "description": "# revenue_accelerator.post_comment_completed\nZoom triggers this event after a ZRA user comments on a conversation.\n## Prerequisites\n* A Pro or higher plan.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**\n\t* The **Post Comments On Conversation** subscription must be enabled under the **Revenue Accelerator** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `iq_comment:read:admin`,`iq_comment:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zra:read:list_conversation_comments`,`zra:read:list_conversation_comments:admin`\n\n**Event type**: `revenue_accelerator.post_comment_completed`\n",
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
                    "description": "The timestamp when the event occurred."
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
                        "description": "The account ID under which the comment was created."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The unique identifier of the user who posted the comment."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The identifier of the user who posted the comment, represented by email."
                      },
                      "object": {
                        "type": "object",
                        "description": "Detailed information about the comment.",
                        "required": [
                          "comment_id",
                          "conversation_id"
                        ],
                        "properties": {
                          "comment_id": {
                            "type": "string",
                            "description": "The unique identifier of the comment."
                          },
                          "conversation_id": {
                            "type": "string",
                            "description": "The unique identifier of the conversation being commented on."
                          },
                          "text": {
                            "type": "string",
                            "description": "The content of the comment."
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
                  "value": "{\n  \"event\": \"revenue_accelerator.post_comment_completed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"account_id\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test.cheng@zoom.us\",\n    \"object\": {\n      \"comment_id\": \"AAAAAABBBB\",\n      \"conversation_id\": \"abcD3ojfdbjfg\",\n      \"text\": \"test of this comment\"\n    }\n  }\n}"
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
                      "description": "The timestamp when the event occurred."
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
                          "description": "The account ID under which the comment was created."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The unique identifier of the user who posted the comment."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The identifier of the user who posted the comment, represented by email."
                        },
                        "object": {
                          "type": "object",
                          "description": "Detailed information about the comment.",
                          "required": [
                            "comment_id",
                            "conversation_id"
                          ],
                          "properties": {
                            "comment_id": {
                              "type": "string",
                              "description": "The unique identifier of the comment."
                            },
                            "conversation_id": {
                              "type": "string",
                              "description": "The unique identifier of the conversation being commented on."
                            },
                            "text": {
                              "type": "string",
                              "description": "The content of the comment."
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
                    "value": "{\n  \"event\": \"revenue_accelerator.post_comment_completed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"account_id\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test.cheng@zoom.us\",\n    \"object\": {\n      \"comment_id\": \"AAAAAABBBB\",\n      \"conversation_id\": \"abcD3ojfdbjfg\",\n      \"text\": \"test of this comment\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "revenue_accelerator.playlist_delete": {
      "post": {
        "tags": [
          "revenue_accelerator"
        ],
        "operationId": "revenue_accelerator.playlist_delete",
        "requestBody": {
          "description": "# revenue_accelerator.playlist_delete\nZoom triggers this event when a ZRA user deletes a playlist.\n## Prerequisites\n* A Pro or higher plan.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**\n\t* The **Playlist Deleted** subscription must be enabled under the **Revenue Accelerator** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `iq_playlist:read:admin`,`iq_playlist:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zra:read:list_conversation_playlists`,`zra:read:list_conversation_playlists:admin`\n\n**Event type**: `revenue_accelerator.playlist_delete`\n",
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
                    "description": "The timestamp when the event occurred."
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
                        "description": "The account ID of the user who deleted the playlist."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The ID of the user who deleted the playlist."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The identifier of the user who deleted the playlist, represented by email."
                      },
                      "object": {
                        "type": "object",
                        "description": "Details about the deleted playlist.",
                        "required": [
                          "playlist_id"
                        ],
                        "properties": {
                          "playlist_id": {
                            "type": "string",
                            "description": "The ID of the deleted playlist."
                          },
                          "playlist_name": {
                            "type": "string",
                            "description": "The name of the deleted playlist."
                          },
                          "delete_time": {
                            "type": "integer",
                            "description": "The timestamp when the playlist was deleted, in milliseconds."
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
                  "value": "{\n  \"event\": \"revenue_accelerator.playlist_delete\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test@zoom.us\",\n    \"object\": {\n      \"playlist_id\": \"AAAAAABBBB\",\n      \"playlist_name\": \"playlist name\",\n      \"delete_time\": 12132323232\n    }\n  }\n}"
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
                      "description": "The timestamp when the event occurred."
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
                          "description": "The account ID of the user who deleted the playlist."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The ID of the user who deleted the playlist."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The identifier of the user who deleted the playlist, represented by email."
                        },
                        "object": {
                          "type": "object",
                          "description": "Details about the deleted playlist.",
                          "required": [
                            "playlist_id"
                          ],
                          "properties": {
                            "playlist_id": {
                              "type": "string",
                              "description": "The ID of the deleted playlist."
                            },
                            "playlist_name": {
                              "type": "string",
                              "description": "The name of the deleted playlist."
                            },
                            "delete_time": {
                              "type": "integer",
                              "description": "The timestamp when the playlist was deleted, in milliseconds."
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
                    "value": "{\n  \"event\": \"revenue_accelerator.playlist_delete\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test@zoom.us\",\n    \"object\": {\n      \"playlist_id\": \"AAAAAABBBB\",\n      \"playlist_name\": \"playlist name\",\n      \"delete_time\": 12132323232\n    }\n  }\n}"
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
    "title": "Zoom Revenue Accelerator Webhooks",
    "description": "",
    "version": "1.0.0"
  },
  "webhooks": {
    "revenue_accelerator.conversation_analysis_completed": {
      "post": {
        "tags": [
          "revenue_accelerator"
        ],
        "operationId": "revenue_accelerator.conversation_analysis_completed",
        "requestBody": {
          "description": "# revenue_accelerator.conversation_analysis_completed\nZoom triggers this event after the ZRA users conversation analysis is completed.\n## Prerequisites\n* A Pro or higher plan.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**\n\t* The **Revenue Accelerator conversations analysis completed** subscription must be enabled under the **Revenue Accelerator** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `iq_conversation:read:admin`,`iq_conversation:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zra:read:conversations`,`zra:read:conversations:admin`\n\n**Event type**: `revenue_accelerator.conversation_analysis_completed`\n",
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
                        "description": "The account ID of the user who recorded the conversation."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the conversation.",
                        "required": [
                          "conversation_id"
                        ],
                        "properties": {
                          "conversation_id": {
                            "type": "string",
                            "description": "The conversation's ID."
                          },
                          "source": {
                            "type": "string",
                            "description": "The conversation's source.",
                            "enum": [
                              "ZOOM_MEETING",
                              "ZOOM_PHONE",
                              "ZOOM_CONTACT_CENTER",
                              "MICROSOFT_TEAMS",
                              "GOOGLE_MEET",
                              "OTHERS"
                            ]
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The conversation host's Zoom user ID."
                          },
                          "engagement_id": {
                            "type": "string",
                            "description": "The ID of the engagement in Zoom Contact Center (ZCC) that this conversation is associated with. Present only when the conversation source is ZOOM_CONTACT_CENTER."
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
                  "value": "{\n  \"event\": \"revenue_accelerator.conversation_analysis_completed \",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"conversation_id\": \"abcD3ojfdbjfg\",\n      \"source\": \"ZOOM_MEETING\",\n      \"host_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\"\n    }\n  }\n}"
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
                          "description": "The account ID of the user who recorded the conversation."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the conversation.",
                          "required": [
                            "conversation_id"
                          ],
                          "properties": {
                            "conversation_id": {
                              "type": "string",
                              "description": "The conversation's ID."
                            },
                            "source": {
                              "type": "string",
                              "description": "The conversation's source.",
                              "enum": [
                                "ZOOM_MEETING",
                                "ZOOM_PHONE",
                                "ZOOM_CONTACT_CENTER",
                                "MICROSOFT_TEAMS",
                                "GOOGLE_MEET",
                                "OTHERS"
                              ]
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The conversation host's Zoom user ID."
                            },
                            "engagement_id": {
                              "type": "string",
                              "description": "The ID of the engagement in Zoom Contact Center (ZCC) that this conversation is associated with. Present only when the conversation source is ZOOM_CONTACT_CENTER."
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
                    "value": "{\n  \"event\": \"revenue_accelerator.conversation_analysis_completed \",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"conversation_id\": \"abcD3ojfdbjfg\",\n      \"source\": \"ZOOM_MEETING\",\n      \"host_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "revenue_accelerator.playlist_remove_moment": {
      "post": {
        "tags": [
          "revenue_accelerator"
        ],
        "operationId": "revenue_accelerator.playlist_remove_moment",
        "requestBody": {
          "description": "# revenue_accelerator.playlist_remove_moment\nZoom triggers this event when a ZRA user removes a moment from a playlist.\n## Prerequisites\n* A Pro or higher plan.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**\n\t* The **Remove Moment From Playlist** subscription must be enabled under the **Revenue Accelerator** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `iq_playlist:read:admin`,`iq_playlist:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zra:read:list_conversation_playlists`,`zra:read:list_conversation_playlists:admin`\n\n**Event type**: `revenue_accelerator.playlist_remove_moment`\n",
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
                    "description": "The timestamp when the event occurred."
                  },
                  "payload": {
                    "required": [
                      "account_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who removed the moment."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The ID of the user who added the moment to the playlist."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The identifier of the user who added the moment to the playlist, represented by email."
                      },
                      "object": {
                        "type": "object",
                        "description": "Details about the removed moment.",
                        "required": [
                          "moment_id",
                          "playlist_id"
                        ],
                        "properties": {
                          "playlist_id": {
                            "type": "string",
                            "description": "The ID of the playlist from which the moment was removed."
                          },
                          "playlist_name": {
                            "type": "string",
                            "description": "The name of the playlist from which the moment was removed."
                          },
                          "playlist_detail_id": {
                            "type": "string",
                            "description": "The ID representing the relationship between the playlist and the moment."
                          },
                          "moment_id": {
                            "type": "string",
                            "description": "The ID of the removed moment."
                          },
                          "moment_name": {
                            "type": "string",
                            "description": "The name of the removed moment."
                          },
                          "update_time": {
                            "type": "integer",
                            "description": "The timestamp (in milliseconds) when the moment was removed from the playlist."
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
                  "value": "{\n  \"event\": \"revenue_accelerator.playlist_remove_moment\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test@zoom.us\",\n    \"object\": {\n      \"playlist_id\": \"AAAAAABBBB\",\n      \"playlist_name\": \"playlist name\",\n      \"playlist_detail_id\": \"323Tl2Ra0ASfO5wIzYPYceiA\",\n      \"moment_id\": \"ASfO5wIzYPYceiA\",\n      \"moment_name\": \"moment name\",\n      \"update_time\": 1755591952177\n    }\n  }\n}"
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
                      "description": "The timestamp when the event occurred."
                    },
                    "payload": {
                      "required": [
                        "account_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who removed the moment."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The ID of the user who added the moment to the playlist."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The identifier of the user who added the moment to the playlist, represented by email."
                        },
                        "object": {
                          "type": "object",
                          "description": "Details about the removed moment.",
                          "required": [
                            "moment_id",
                            "playlist_id"
                          ],
                          "properties": {
                            "playlist_id": {
                              "type": "string",
                              "description": "The ID of the playlist from which the moment was removed."
                            },
                            "playlist_name": {
                              "type": "string",
                              "description": "The name of the playlist from which the moment was removed."
                            },
                            "playlist_detail_id": {
                              "type": "string",
                              "description": "The ID representing the relationship between the playlist and the moment."
                            },
                            "moment_id": {
                              "type": "string",
                              "description": "The ID of the removed moment."
                            },
                            "moment_name": {
                              "type": "string",
                              "description": "The name of the removed moment."
                            },
                            "update_time": {
                              "type": "integer",
                              "description": "The timestamp (in milliseconds) when the moment was removed from the playlist."
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
                    "value": "{\n  \"event\": \"revenue_accelerator.playlist_remove_moment\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test@zoom.us\",\n    \"object\": {\n      \"playlist_id\": \"AAAAAABBBB\",\n      \"playlist_name\": \"playlist name\",\n      \"playlist_detail_id\": \"323Tl2Ra0ASfO5wIzYPYceiA\",\n      \"moment_id\": \"ASfO5wIzYPYceiA\",\n      \"moment_name\": \"moment name\",\n      \"update_time\": 1755591952177\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "revenue_accelerator.playlist_add_moment": {
      "post": {
        "tags": [
          "revenue_accelerator"
        ],
        "operationId": "revenue_accelerator.playlist_add_moment",
        "requestBody": {
          "description": "# revenue_accelerator.playlist_add_moment\nZoom triggers this event when a ZRA user adds a moment to a playlist.\n## Prerequisites\n* A Pro or higher plan.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**\n\t* The **Add Moment To Playlist** subscription must be enabled under the **Revenue Accelerator** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `iq_playlist:read:admin`,`iq_playlist:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zra:read:list_conversation_playlists`,`zra:read:list_conversation_playlists:admin`\n\n**Event type**: `revenue_accelerator.playlist_add_moment`\n",
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
                    "description": "The timestamp when the event occurred."
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
                        "description": "The account ID of the user who added the moment to the playlist."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The ID of the user who added the moment to the playlist."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The identifier of the user who added the moment to the playlist, represented by email."
                      },
                      "object": {
                        "type": "object",
                        "description": "Details about adding the moment to the playlist.",
                        "required": [
                          "moment_id",
                          "playlist_id"
                        ],
                        "properties": {
                          "playlist_id": {
                            "type": "string",
                            "description": "The ID of the playlist the moment was added to."
                          },
                          "playlist_name": {
                            "type": "string",
                            "description": "The name of the playlist the moment was added to."
                          },
                          "playlist_detail_id": {
                            "type": "string",
                            "description": "The ID representing the relationship between the playlist and the moment."
                          },
                          "moment_id": {
                            "type": "string",
                            "description": "The ID of the moment that was added to the playlist."
                          },
                          "moment_name": {
                            "type": "string",
                            "description": "The name of the moment that was added to the playlist."
                          },
                          "update_time": {
                            "type": "integer",
                            "description": "The timestamp (in milliseconds) when the moment was added to the playlist."
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
                  "value": "{\n  \"event\": \"revenue_accelerator.playlist_add_moment\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test@zoom.us\",\n    \"object\": {\n      \"playlist_id\": \"AAAAAABBBB\",\n      \"playlist_name\": \"playlist name\",\n      \"playlist_detail_id\": \"323Tl2Ra0ASfO5wIzYPYceiA\",\n      \"moment_id\": \"ASfO5wIzYPYceiA\",\n      \"moment_name\": \"moment name\",\n      \"update_time\": 1755591952177\n    }\n  }\n}"
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
                      "description": "The timestamp when the event occurred."
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
                          "description": "The account ID of the user who added the moment to the playlist."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The ID of the user who added the moment to the playlist."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The identifier of the user who added the moment to the playlist, represented by email."
                        },
                        "object": {
                          "type": "object",
                          "description": "Details about adding the moment to the playlist.",
                          "required": [
                            "moment_id",
                            "playlist_id"
                          ],
                          "properties": {
                            "playlist_id": {
                              "type": "string",
                              "description": "The ID of the playlist the moment was added to."
                            },
                            "playlist_name": {
                              "type": "string",
                              "description": "The name of the playlist the moment was added to."
                            },
                            "playlist_detail_id": {
                              "type": "string",
                              "description": "The ID representing the relationship between the playlist and the moment."
                            },
                            "moment_id": {
                              "type": "string",
                              "description": "The ID of the moment that was added to the playlist."
                            },
                            "moment_name": {
                              "type": "string",
                              "description": "The name of the moment that was added to the playlist."
                            },
                            "update_time": {
                              "type": "integer",
                              "description": "The timestamp (in milliseconds) when the moment was added to the playlist."
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
                    "value": "{\n  \"event\": \"revenue_accelerator.playlist_add_moment\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test@zoom.us\",\n    \"object\": {\n      \"playlist_id\": \"AAAAAABBBB\",\n      \"playlist_name\": \"playlist name\",\n      \"playlist_detail_id\": \"323Tl2Ra0ASfO5wIzYPYceiA\",\n      \"moment_id\": \"ASfO5wIzYPYceiA\",\n      \"moment_name\": \"moment name\",\n      \"update_time\": 1755591952177\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "revenue_accelerator.playlist_create": {
      "post": {
        "tags": [
          "revenue_accelerator"
        ],
        "operationId": "revenue_accelerator.playlist_create",
        "requestBody": {
          "description": "# revenue_accelerator.playlist_create\nZoom triggers this event when a ZRA user creates a playlist.\n## Prerequisites\n* A Pro or higher plan.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**\n\t* The **Playlist Created** subscription must be enabled under the **Revenue Accelerator** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `iq_playlist:read:admin`,`iq_playlist:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zra:read:list_conversation_playlists`,`zra:read:list_conversation_playlists:admin`\n\n**Event type**: `revenue_accelerator.playlist_create`\n",
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
                    "description": "The timestamp when the event occurred."
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
                        "description": "The account ID of the user who created the playlist."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The ID of the user who created the playlist."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The identifier of the user who created the playlist, represented by email."
                      },
                      "object": {
                        "type": "object",
                        "description": "Details about the created playlist.",
                        "required": [
                          "playlist_id",
                          "playlist_type",
                          "playlist_name"
                        ],
                        "properties": {
                          "playlist_id": {
                            "type": "string",
                            "description": "The ID of the created playlist."
                          },
                          "playlist_type": {
                            "type": "string",
                            "description": "The type of playlist created.",
                            "enum": [
                              "BASIC",
                              "SMART"
                            ]
                          },
                          "playlist_name": {
                            "type": "string",
                            "description": "The name of the created playlist."
                          },
                          "create_time": {
                            "type": "integer",
                            "description": "The creation timestamp, in milliseconds."
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
                  "value": "{\n  \"event\": \"revenue_accelerator.playlist_create\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"account_id\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test@zoom.us\",\n    \"object\": {\n      \"playlist_id\": \"AAAAAABBBB\",\n      \"playlist_type\": \"SMART\",\n      \"playlist_name\": \"playlist name\",\n      \"create_time\": 1755591952177\n    }\n  }\n}"
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
                      "description": "The timestamp when the event occurred."
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
                          "description": "The account ID of the user who created the playlist."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The ID of the user who created the playlist."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The identifier of the user who created the playlist, represented by email."
                        },
                        "object": {
                          "type": "object",
                          "description": "Details about the created playlist.",
                          "required": [
                            "playlist_id",
                            "playlist_type",
                            "playlist_name"
                          ],
                          "properties": {
                            "playlist_id": {
                              "type": "string",
                              "description": "The ID of the created playlist."
                            },
                            "playlist_type": {
                              "type": "string",
                              "description": "The type of playlist created.",
                              "enum": [
                                "BASIC",
                                "SMART"
                              ]
                            },
                            "playlist_name": {
                              "type": "string",
                              "description": "The name of the created playlist."
                            },
                            "create_time": {
                              "type": "integer",
                              "description": "The creation timestamp, in milliseconds."
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
                    "value": "{\n  \"event\": \"revenue_accelerator.playlist_create\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"account_id\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test@zoom.us\",\n    \"object\": {\n      \"playlist_id\": \"AAAAAABBBB\",\n      \"playlist_type\": \"SMART\",\n      \"playlist_name\": \"playlist name\",\n      \"create_time\": 1755591952177\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "revenue_accelerator.post_comment_completed": {
      "post": {
        "tags": [
          "revenue_accelerator"
        ],
        "operationId": "revenue_accelerator.post_comment_completed",
        "requestBody": {
          "description": "# revenue_accelerator.post_comment_completed\nZoom triggers this event after a ZRA user comments on a conversation.\n## Prerequisites\n* A Pro or higher plan.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**\n\t* The **Post Comments On Conversation** subscription must be enabled under the **Revenue Accelerator** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `iq_comment:read:admin`,`iq_comment:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zra:read:list_conversation_comments`,`zra:read:list_conversation_comments:admin`\n\n**Event type**: `revenue_accelerator.post_comment_completed`\n",
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
                    "description": "The timestamp when the event occurred."
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
                        "description": "The account ID under which the comment was created."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The unique identifier of the user who posted the comment."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The identifier of the user who posted the comment, represented by email."
                      },
                      "object": {
                        "type": "object",
                        "description": "Detailed information about the comment.",
                        "required": [
                          "comment_id",
                          "conversation_id"
                        ],
                        "properties": {
                          "comment_id": {
                            "type": "string",
                            "description": "The unique identifier of the comment."
                          },
                          "conversation_id": {
                            "type": "string",
                            "description": "The unique identifier of the conversation being commented on."
                          },
                          "text": {
                            "type": "string",
                            "description": "The content of the comment."
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
                  "value": "{\n  \"event\": \"revenue_accelerator.post_comment_completed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"account_id\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test.cheng@zoom.us\",\n    \"object\": {\n      \"comment_id\": \"AAAAAABBBB\",\n      \"conversation_id\": \"abcD3ojfdbjfg\",\n      \"text\": \"test of this comment\"\n    }\n  }\n}"
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
                      "description": "The timestamp when the event occurred."
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
                          "description": "The account ID under which the comment was created."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The unique identifier of the user who posted the comment."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The identifier of the user who posted the comment, represented by email."
                        },
                        "object": {
                          "type": "object",
                          "description": "Detailed information about the comment.",
                          "required": [
                            "comment_id",
                            "conversation_id"
                          ],
                          "properties": {
                            "comment_id": {
                              "type": "string",
                              "description": "The unique identifier of the comment."
                            },
                            "conversation_id": {
                              "type": "string",
                              "description": "The unique identifier of the conversation being commented on."
                            },
                            "text": {
                              "type": "string",
                              "description": "The content of the comment."
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
                    "value": "{\n  \"event\": \"revenue_accelerator.post_comment_completed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"account_id\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test.cheng@zoom.us\",\n    \"object\": {\n      \"comment_id\": \"AAAAAABBBB\",\n      \"conversation_id\": \"abcD3ojfdbjfg\",\n      \"text\": \"test of this comment\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "revenue_accelerator.playlist_delete": {
      "post": {
        "tags": [
          "revenue_accelerator"
        ],
        "operationId": "revenue_accelerator.playlist_delete",
        "requestBody": {
          "description": "# revenue_accelerator.playlist_delete\nZoom triggers this event when a ZRA user deletes a playlist.\n## Prerequisites\n* A Pro or higher plan.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**\n\t* The **Playlist Deleted** subscription must be enabled under the **Revenue Accelerator** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `iq_playlist:read:admin`,`iq_playlist:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zra:read:list_conversation_playlists`,`zra:read:list_conversation_playlists:admin`\n\n**Event type**: `revenue_accelerator.playlist_delete`\n",
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
                    "description": "The timestamp when the event occurred."
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
                        "description": "The account ID of the user who deleted the playlist."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The ID of the user who deleted the playlist."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The identifier of the user who deleted the playlist, represented by email."
                      },
                      "object": {
                        "type": "object",
                        "description": "Details about the deleted playlist.",
                        "required": [
                          "playlist_id"
                        ],
                        "properties": {
                          "playlist_id": {
                            "type": "string",
                            "description": "The ID of the deleted playlist."
                          },
                          "playlist_name": {
                            "type": "string",
                            "description": "The name of the deleted playlist."
                          },
                          "delete_time": {
                            "type": "integer",
                            "description": "The timestamp when the playlist was deleted, in milliseconds."
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
                  "value": "{\n  \"event\": \"revenue_accelerator.playlist_delete\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test@zoom.us\",\n    \"object\": {\n      \"playlist_id\": \"AAAAAABBBB\",\n      \"playlist_name\": \"playlist name\",\n      \"delete_time\": 12132323232\n    }\n  }\n}"
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
                      "description": "The timestamp when the event occurred."
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
                          "description": "The account ID of the user who deleted the playlist."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The ID of the user who deleted the playlist."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The identifier of the user who deleted the playlist, represented by email."
                        },
                        "object": {
                          "type": "object",
                          "description": "Details about the deleted playlist.",
                          "required": [
                            "playlist_id"
                          ],
                          "properties": {
                            "playlist_id": {
                              "type": "string",
                              "description": "The ID of the deleted playlist."
                            },
                            "playlist_name": {
                              "type": "string",
                              "description": "The name of the deleted playlist."
                            },
                            "delete_time": {
                              "type": "integer",
                              "description": "The timestamp when the playlist was deleted, in milliseconds."
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
                    "value": "{\n  \"event\": \"revenue_accelerator.playlist_delete\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator_id\": \"Gtl2Ra0ASfO5wIzYPYceiA\",\n    \"operator\": \"test@zoom.us\",\n    \"object\": {\n      \"playlist_id\": \"AAAAAABBBB\",\n      \"playlist_name\": \"playlist name\",\n      \"delete_time\": 12132323232\n    }\n  }\n}"
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
      "iq",
      "events"
    ]
  },
  "downloadPath": "/api-hub/iq/events/webhooks.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": false
  },
  "productMeta": {
    "specType": "adoc",
    "slug": "/docs/api/rest/reference/iq/events",
    "title": "Zoom Revenue Accelerator API",
    "description": "",
    "keywords": "zoom, api, iq",
    "params": {
      "productName": "Zoom Revenue Accelerator"
    },
    "zoapEndpoint": "webhookByProduct",
    "productDisplayTitle": "Revenue Accelerator",
    "scalarSlug": "/docs/api/iq/events",
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "svgname": "zra"
  },
  "frontmatter": {
    "title": "Revenue Accelerator Webhooks",
    "keywords": "zoom, api, iq",
    "description": "Revenue Accelerator events",
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
        "iq",
        "events"
      ]
    },
    "downloadPath": "/api-hub/iq/events/webhooks.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": false
    },
    "productMeta": {
      "specType": "adoc",
      "slug": "/docs/api/rest/reference/iq/events",
      "title": "Zoom Revenue Accelerator API",
      "description": "",
      "keywords": "zoom, api, iq",
      "params": {
        "productName": "Zoom Revenue Accelerator"
      },
      "zoapEndpoint": "webhookByProduct",
      "productDisplayTitle": "Revenue Accelerator",
      "scalarSlug": "/docs/api/iq/events",
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "svgname": "zra"
    },
    "spec": "{\"openapi\":\"3.1.0\",\"info\":{\"title\":\"Zoom Revenue Accelerator Webhooks\",\"description\":\"\",\"version\":\"1.0.0\"},\"paths\":{\"revenue_accelerator.conversation_analysis_completed\":{\"post\":{\"tags\":[\"revenue_accelerator\"],\"operationId\":\"revenue_accelerator.conversation_analysis_completed\",\"requestBody\":{\"description\":\"# revenue_accelerator.conversation_analysis_completed\\nZoom triggers this event after the ZRA users conversation analysis is completed.\\n## Prerequisites\\n* A Pro or higher plan.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**\\n\\t* The **Revenue Accelerator conversations analysis completed** subscription must be enabled under the **Revenue Accelerator** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `iq_conversation:read:admin`,`iq_conversation:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zra:read:conversations`,`zra:read:conversations:admin`\\n\\n**Event type**: `revenue_accelerator.conversation_analysis_completed`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who recorded the conversation.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the conversation.\",\"required\":[\"conversation_id\"],\"properties\":{\"conversation_id\":{\"type\":\"string\",\"description\":\"The conversation's ID.\"},\"source\":{\"type\":\"string\",\"description\":\"The conversation's source.\",\"enum\":[\"ZOOM_MEETING\",\"ZOOM_PHONE\",\"ZOOM_CONTACT_CENTER\",\"MICROSOFT_TEAMS\",\"GOOGLE_MEET\",\"OTHERS\"]},\"host_id\":{\"type\":\"string\",\"description\":\"The conversation host's Zoom user ID.\"},\"engagement_id\":{\"type\":\"string\",\"description\":\"The ID of the engagement in Zoom Contact Center (ZCC) that this conversation is associated with. Present only when the conversation source is ZOOM_CONTACT_CENTER.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"revenue_accelerator.conversation_analysis_completed \\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"conversation_id\\\": \\\"abcD3ojfdbjfg\\\",\\n      \\\"source\\\": \\\"ZOOM_MEETING\\\",\\n      \\\"host_id\\\": \\\"Gtl2Ra0ASfO5wIzYPYceiA\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who recorded the conversation.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the conversation.\",\"required\":[\"conversation_id\"],\"properties\":{\"conversation_id\":{\"type\":\"string\",\"description\":\"The conversation's ID.\"},\"source\":{\"type\":\"string\",\"description\":\"The conversation's source.\",\"enum\":[\"ZOOM_MEETING\",\"ZOOM_PHONE\",\"ZOOM_CONTACT_CENTER\",\"MICROSOFT_TEAMS\",\"GOOGLE_MEET\",\"OTHERS\"]},\"host_id\":{\"type\":\"string\",\"description\":\"The conversation host's Zoom user ID.\"},\"engagement_id\":{\"type\":\"string\",\"description\":\"The ID of the engagement in Zoom Contact Center (ZCC) that this conversation is associated with. Present only when the conversation source is ZOOM_CONTACT_CENTER.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"revenue_accelerator.conversation_analysis_completed \\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"conversation_id\\\": \\\"abcD3ojfdbjfg\\\",\\n      \\\"source\\\": \\\"ZOOM_MEETING\\\",\\n      \\\"host_id\\\": \\\"Gtl2Ra0ASfO5wIzYPYceiA\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"revenue_accelerator.playlist_remove_moment\":{\"post\":{\"tags\":[\"revenue_accelerator\"],\"operationId\":\"revenue_accelerator.playlist_remove_moment\",\"requestBody\":{\"description\":\"# revenue_accelerator.playlist_remove_moment\\nZoom triggers this event when a ZRA user removes a moment from a playlist.\\n## Prerequisites\\n* A Pro or higher plan.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**\\n\\t* The **Remove Moment From Playlist** subscription must be enabled under the **Revenue Accelerator** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `iq_playlist:read:admin`,`iq_playlist:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zra:read:list_conversation_playlists`,`zra:read:list_conversation_playlists:admin`\\n\\n**Event type**: `revenue_accelerator.playlist_remove_moment`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who removed the moment.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The ID of the user who added the moment to the playlist.\"},\"operator\":{\"type\":\"string\",\"description\":\"The identifier of the user who added the moment to the playlist, represented by email.\"},\"object\":{\"type\":\"object\",\"description\":\"Details about the removed moment.\",\"required\":[\"moment_id\",\"playlist_id\"],\"properties\":{\"playlist_id\":{\"type\":\"string\",\"description\":\"The ID of the playlist from which the moment was removed.\"},\"playlist_name\":{\"type\":\"string\",\"description\":\"The name of the playlist from which the moment was removed.\"},\"playlist_detail_id\":{\"type\":\"string\",\"description\":\"The ID representing the relationship between the playlist and the moment.\"},\"moment_id\":{\"type\":\"string\",\"description\":\"The ID of the removed moment.\"},\"moment_name\":{\"type\":\"string\",\"description\":\"The name of the removed moment.\"},\"update_time\":{\"type\":\"integer\",\"description\":\"The timestamp (in milliseconds) when the moment was removed from the playlist.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"revenue_accelerator.playlist_remove_moment\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator_id\\\": \\\"Gtl2Ra0ASfO5wIzYPYceiA\\\",\\n    \\\"operator\\\": \\\"test@zoom.us\\\",\\n    \\\"object\\\": {\\n      \\\"playlist_id\\\": \\\"AAAAAABBBB\\\",\\n      \\\"playlist_name\\\": \\\"playlist name\\\",\\n      \\\"playlist_detail_id\\\": \\\"323Tl2Ra0ASfO5wIzYPYceiA\\\",\\n      \\\"moment_id\\\": \\\"ASfO5wIzYPYceiA\\\",\\n      \\\"moment_name\\\": \\\"moment name\\\",\\n      \\\"update_time\\\": 1755591952177\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who removed the moment.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The ID of the user who added the moment to the playlist.\"},\"operator\":{\"type\":\"string\",\"description\":\"The identifier of the user who added the moment to the playlist, represented by email.\"},\"object\":{\"type\":\"object\",\"description\":\"Details about the removed moment.\",\"required\":[\"moment_id\",\"playlist_id\"],\"properties\":{\"playlist_id\":{\"type\":\"string\",\"description\":\"The ID of the playlist from which the moment was removed.\"},\"playlist_name\":{\"type\":\"string\",\"description\":\"The name of the playlist from which the moment was removed.\"},\"playlist_detail_id\":{\"type\":\"string\",\"description\":\"The ID representing the relationship between the playlist and the moment.\"},\"moment_id\":{\"type\":\"string\",\"description\":\"The ID of the removed moment.\"},\"moment_name\":{\"type\":\"string\",\"description\":\"The name of the removed moment.\"},\"update_time\":{\"type\":\"integer\",\"description\":\"The timestamp (in milliseconds) when the moment was removed from the playlist.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"revenue_accelerator.playlist_remove_moment\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator_id\\\": \\\"Gtl2Ra0ASfO5wIzYPYceiA\\\",\\n    \\\"operator\\\": \\\"test@zoom.us\\\",\\n    \\\"object\\\": {\\n      \\\"playlist_id\\\": \\\"AAAAAABBBB\\\",\\n      \\\"playlist_name\\\": \\\"playlist name\\\",\\n      \\\"playlist_detail_id\\\": \\\"323Tl2Ra0ASfO5wIzYPYceiA\\\",\\n      \\\"moment_id\\\": \\\"ASfO5wIzYPYceiA\\\",\\n      \\\"moment_name\\\": \\\"moment name\\\",\\n      \\\"update_time\\\": 1755591952177\\n    }\\n  }\\n}\"}}}}}}}},\"revenue_accelerator.playlist_add_moment\":{\"post\":{\"tags\":[\"revenue_accelerator\"],\"operationId\":\"revenue_accelerator.playlist_add_moment\",\"requestBody\":{\"description\":\"# revenue_accelerator.playlist_add_moment\\nZoom triggers this event when a ZRA user adds a moment to a playlist.\\n## Prerequisites\\n* A Pro or higher plan.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**\\n\\t* The **Add Moment To Playlist** subscription must be enabled under the **Revenue Accelerator** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `iq_playlist:read:admin`,`iq_playlist:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zra:read:list_conversation_playlists`,`zra:read:list_conversation_playlists:admin`\\n\\n**Event type**: `revenue_accelerator.playlist_add_moment`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who added the moment to the playlist.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The ID of the user who added the moment to the playlist.\"},\"operator\":{\"type\":\"string\",\"description\":\"The identifier of the user who added the moment to the playlist, represented by email.\"},\"object\":{\"type\":\"object\",\"description\":\"Details about adding the moment to the playlist.\",\"required\":[\"moment_id\",\"playlist_id\"],\"properties\":{\"playlist_id\":{\"type\":\"string\",\"description\":\"The ID of the playlist the moment was added to.\"},\"playlist_name\":{\"type\":\"string\",\"description\":\"The name of the playlist the moment was added to.\"},\"playlist_detail_id\":{\"type\":\"string\",\"description\":\"The ID representing the relationship between the playlist and the moment.\"},\"moment_id\":{\"type\":\"string\",\"description\":\"The ID of the moment that was added to the playlist.\"},\"moment_name\":{\"type\":\"string\",\"description\":\"The name of the moment that was added to the playlist.\"},\"update_time\":{\"type\":\"integer\",\"description\":\"The timestamp (in milliseconds) when the moment was added to the playlist.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"revenue_accelerator.playlist_add_moment\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator_id\\\": \\\"Gtl2Ra0ASfO5wIzYPYceiA\\\",\\n    \\\"operator\\\": \\\"test@zoom.us\\\",\\n    \\\"object\\\": {\\n      \\\"playlist_id\\\": \\\"AAAAAABBBB\\\",\\n      \\\"playlist_name\\\": \\\"playlist name\\\",\\n      \\\"playlist_detail_id\\\": \\\"323Tl2Ra0ASfO5wIzYPYceiA\\\",\\n      \\\"moment_id\\\": \\\"ASfO5wIzYPYceiA\\\",\\n      \\\"moment_name\\\": \\\"moment name\\\",\\n      \\\"update_time\\\": 1755591952177\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who added the moment to the playlist.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The ID of the user who added the moment to the playlist.\"},\"operator\":{\"type\":\"string\",\"description\":\"The identifier of the user who added the moment to the playlist, represented by email.\"},\"object\":{\"type\":\"object\",\"description\":\"Details about adding the moment to the playlist.\",\"required\":[\"moment_id\",\"playlist_id\"],\"properties\":{\"playlist_id\":{\"type\":\"string\",\"description\":\"The ID of the playlist the moment was added to.\"},\"playlist_name\":{\"type\":\"string\",\"description\":\"The name of the playlist the moment was added to.\"},\"playlist_detail_id\":{\"type\":\"string\",\"description\":\"The ID representing the relationship between the playlist and the moment.\"},\"moment_id\":{\"type\":\"string\",\"description\":\"The ID of the moment that was added to the playlist.\"},\"moment_name\":{\"type\":\"string\",\"description\":\"The name of the moment that was added to the playlist.\"},\"update_time\":{\"type\":\"integer\",\"description\":\"The timestamp (in milliseconds) when the moment was added to the playlist.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"revenue_accelerator.playlist_add_moment\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator_id\\\": \\\"Gtl2Ra0ASfO5wIzYPYceiA\\\",\\n    \\\"operator\\\": \\\"test@zoom.us\\\",\\n    \\\"object\\\": {\\n      \\\"playlist_id\\\": \\\"AAAAAABBBB\\\",\\n      \\\"playlist_name\\\": \\\"playlist name\\\",\\n      \\\"playlist_detail_id\\\": \\\"323Tl2Ra0ASfO5wIzYPYceiA\\\",\\n      \\\"moment_id\\\": \\\"ASfO5wIzYPYceiA\\\",\\n      \\\"moment_name\\\": \\\"moment name\\\",\\n      \\\"update_time\\\": 1755591952177\\n    }\\n  }\\n}\"}}}}}}}},\"revenue_accelerator.playlist_create\":{\"post\":{\"tags\":[\"revenue_accelerator\"],\"operationId\":\"revenue_accelerator.playlist_create\",\"requestBody\":{\"description\":\"# revenue_accelerator.playlist_create\\nZoom triggers this event when a ZRA user creates a playlist.\\n## Prerequisites\\n* A Pro or higher plan.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**\\n\\t* The **Playlist Created** subscription must be enabled under the **Revenue Accelerator** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `iq_playlist:read:admin`,`iq_playlist:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zra:read:list_conversation_playlists`,`zra:read:list_conversation_playlists:admin`\\n\\n**Event type**: `revenue_accelerator.playlist_create`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who created the playlist.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The ID of the user who created the playlist.\"},\"operator\":{\"type\":\"string\",\"description\":\"The identifier of the user who created the playlist, represented by email.\"},\"object\":{\"type\":\"object\",\"description\":\"Details about the created playlist.\",\"required\":[\"playlist_id\",\"playlist_type\",\"playlist_name\"],\"properties\":{\"playlist_id\":{\"type\":\"string\",\"description\":\"The ID of the created playlist.\"},\"playlist_type\":{\"type\":\"string\",\"description\":\"The type of playlist created.\",\"enum\":[\"BASIC\",\"SMART\"]},\"playlist_name\":{\"type\":\"string\",\"description\":\"The name of the created playlist.\"},\"create_time\":{\"type\":\"integer\",\"description\":\"The creation timestamp, in milliseconds.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"revenue_accelerator.playlist_create\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"account_id\\\",\\n    \\\"operator_id\\\": \\\"Gtl2Ra0ASfO5wIzYPYceiA\\\",\\n    \\\"operator\\\": \\\"test@zoom.us\\\",\\n    \\\"object\\\": {\\n      \\\"playlist_id\\\": \\\"AAAAAABBBB\\\",\\n      \\\"playlist_type\\\": \\\"SMART\\\",\\n      \\\"playlist_name\\\": \\\"playlist name\\\",\\n      \\\"create_time\\\": 1755591952177\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who created the playlist.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The ID of the user who created the playlist.\"},\"operator\":{\"type\":\"string\",\"description\":\"The identifier of the user who created the playlist, represented by email.\"},\"object\":{\"type\":\"object\",\"description\":\"Details about the created playlist.\",\"required\":[\"playlist_id\",\"playlist_type\",\"playlist_name\"],\"properties\":{\"playlist_id\":{\"type\":\"string\",\"description\":\"The ID of the created playlist.\"},\"playlist_type\":{\"type\":\"string\",\"description\":\"The type of playlist created.\",\"enum\":[\"BASIC\",\"SMART\"]},\"playlist_name\":{\"type\":\"string\",\"description\":\"The name of the created playlist.\"},\"create_time\":{\"type\":\"integer\",\"description\":\"The creation timestamp, in milliseconds.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"revenue_accelerator.playlist_create\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"account_id\\\",\\n    \\\"operator_id\\\": \\\"Gtl2Ra0ASfO5wIzYPYceiA\\\",\\n    \\\"operator\\\": \\\"test@zoom.us\\\",\\n    \\\"object\\\": {\\n      \\\"playlist_id\\\": \\\"AAAAAABBBB\\\",\\n      \\\"playlist_type\\\": \\\"SMART\\\",\\n      \\\"playlist_name\\\": \\\"playlist name\\\",\\n      \\\"create_time\\\": 1755591952177\\n    }\\n  }\\n}\"}}}}}}}},\"revenue_accelerator.post_comment_completed\":{\"post\":{\"tags\":[\"revenue_accelerator\"],\"operationId\":\"revenue_accelerator.post_comment_completed\",\"requestBody\":{\"description\":\"# revenue_accelerator.post_comment_completed\\nZoom triggers this event after a ZRA user comments on a conversation.\\n## Prerequisites\\n* A Pro or higher plan.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**\\n\\t* The **Post Comments On Conversation** subscription must be enabled under the **Revenue Accelerator** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `iq_comment:read:admin`,`iq_comment:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zra:read:list_conversation_comments`,`zra:read:list_conversation_comments:admin`\\n\\n**Event type**: `revenue_accelerator.post_comment_completed`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID under which the comment was created.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the user who posted the comment.\"},\"operator\":{\"type\":\"string\",\"description\":\"The identifier of the user who posted the comment, represented by email.\"},\"object\":{\"type\":\"object\",\"description\":\"Detailed information about the comment.\",\"required\":[\"comment_id\",\"conversation_id\"],\"properties\":{\"comment_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the comment.\"},\"conversation_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the conversation being commented on.\"},\"text\":{\"type\":\"string\",\"description\":\"The content of the comment.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"revenue_accelerator.post_comment_completed\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"account_id\\\",\\n    \\\"operator_id\\\": \\\"Gtl2Ra0ASfO5wIzYPYceiA\\\",\\n    \\\"operator\\\": \\\"test.cheng@zoom.us\\\",\\n    \\\"object\\\": {\\n      \\\"comment_id\\\": \\\"AAAAAABBBB\\\",\\n      \\\"conversation_id\\\": \\\"abcD3ojfdbjfg\\\",\\n      \\\"text\\\": \\\"test of this comment\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID under which the comment was created.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the user who posted the comment.\"},\"operator\":{\"type\":\"string\",\"description\":\"The identifier of the user who posted the comment, represented by email.\"},\"object\":{\"type\":\"object\",\"description\":\"Detailed information about the comment.\",\"required\":[\"comment_id\",\"conversation_id\"],\"properties\":{\"comment_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the comment.\"},\"conversation_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the conversation being commented on.\"},\"text\":{\"type\":\"string\",\"description\":\"The content of the comment.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"revenue_accelerator.post_comment_completed\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"account_id\\\",\\n    \\\"operator_id\\\": \\\"Gtl2Ra0ASfO5wIzYPYceiA\\\",\\n    \\\"operator\\\": \\\"test.cheng@zoom.us\\\",\\n    \\\"object\\\": {\\n      \\\"comment_id\\\": \\\"AAAAAABBBB\\\",\\n      \\\"conversation_id\\\": \\\"abcD3ojfdbjfg\\\",\\n      \\\"text\\\": \\\"test of this comment\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"revenue_accelerator.playlist_delete\":{\"post\":{\"tags\":[\"revenue_accelerator\"],\"operationId\":\"revenue_accelerator.playlist_delete\",\"requestBody\":{\"description\":\"# revenue_accelerator.playlist_delete\\nZoom triggers this event when a ZRA user deletes a playlist.\\n## Prerequisites\\n* A Pro or higher plan.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**\\n\\t* The **Playlist Deleted** subscription must be enabled under the **Revenue Accelerator** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `iq_playlist:read:admin`,`iq_playlist:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zra:read:list_conversation_playlists`,`zra:read:list_conversation_playlists:admin`\\n\\n**Event type**: `revenue_accelerator.playlist_delete`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who deleted the playlist.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The ID of the user who deleted the playlist.\"},\"operator\":{\"type\":\"string\",\"description\":\"The identifier of the user who deleted the playlist, represented by email.\"},\"object\":{\"type\":\"object\",\"description\":\"Details about the deleted playlist.\",\"required\":[\"playlist_id\"],\"properties\":{\"playlist_id\":{\"type\":\"string\",\"description\":\"The ID of the deleted playlist.\"},\"playlist_name\":{\"type\":\"string\",\"description\":\"The name of the deleted playlist.\"},\"delete_time\":{\"type\":\"integer\",\"description\":\"The timestamp when the playlist was deleted, in milliseconds.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"revenue_accelerator.playlist_delete\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator_id\\\": \\\"Gtl2Ra0ASfO5wIzYPYceiA\\\",\\n    \\\"operator\\\": \\\"test@zoom.us\\\",\\n    \\\"object\\\": {\\n      \\\"playlist_id\\\": \\\"AAAAAABBBB\\\",\\n      \\\"playlist_name\\\": \\\"playlist name\\\",\\n      \\\"delete_time\\\": 12132323232\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who deleted the playlist.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The ID of the user who deleted the playlist.\"},\"operator\":{\"type\":\"string\",\"description\":\"The identifier of the user who deleted the playlist, represented by email.\"},\"object\":{\"type\":\"object\",\"description\":\"Details about the deleted playlist.\",\"required\":[\"playlist_id\"],\"properties\":{\"playlist_id\":{\"type\":\"string\",\"description\":\"The ID of the deleted playlist.\"},\"playlist_name\":{\"type\":\"string\",\"description\":\"The name of the deleted playlist.\"},\"delete_time\":{\"type\":\"integer\",\"description\":\"The timestamp when the playlist was deleted, in milliseconds.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"revenue_accelerator.playlist_delete\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator_id\\\": \\\"Gtl2Ra0ASfO5wIzYPYceiA\\\",\\n    \\\"operator\\\": \\\"test@zoom.us\\\",\\n    \\\"object\\\": {\\n      \\\"playlist_id\\\": \\\"AAAAAABBBB\\\",\\n      \\\"playlist_name\\\": \\\"playlist name\\\",\\n      \\\"delete_time\\\": 12132323232\\n    }\\n  }\\n}\"}}}}}}}}},\"servers\":[{\"url\":\"https://{your-endpoint-url}\"}]}",
    "frontmatter": {
      "title": "Revenue Accelerator Webhooks",
      "keywords": "zoom, api, iq",
      "description": "Revenue Accelerator events",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
