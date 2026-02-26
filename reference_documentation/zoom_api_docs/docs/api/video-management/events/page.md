# Video Management Webhooks

- Source URL: https://developers.zoom.us/docs/api/video-management/events/
- Snapshot path: docs/api/video-management/events
- Generated (UTC): 2026-02-07T22:40:11.912862+00:00

## Frontmatter

```json
{
  "title": "Video Management Webhooks",
  "keywords": "zoom, rest, api, ai, aic, video-management",
  "description": "Video Management events",
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
    "title": "Video Management",
    "description": "",
    "version": "1.0.0"
  },
  "paths": {
    "video_management.channel_video_added": {
      "post": {
        "tags": [
          "video_management"
        ],
        "operationId": "video_management.channel_video_added",
        "requestBody": {
          "description": "# video_management.channel_video_added\nThe **channel video added** event is triggered every time a user in your account, or your app, adds a video to a channel.\n\n\n## Prerequisites\n\n* A [Pro or higher account plan](https://www.zoom.us/pricing?)\n* A Zoom Video Management license\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**\n    * The **Channel Video Added** subscription enabled under the **Video Management** event\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt_videos:read`,`video_mgmt_videos:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt:read:list_videos`,`video_mgmt:read:list_videos:admin`\n\n**Event type**: `video_management.channel_video_added`\n",
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
                    "description": "The name of the event. video_management.channel_video_added."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "A timestamp of when the event occurred."
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
                        "description": "The account ID of the host who published the video channel."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom Event description.",
                        "required": [
                          "channel_id",
                          "video_id"
                        ],
                        "properties": {
                          "channel_id": {
                            "type": "string",
                            "description": "The channel ID.",
                            "example": "iso9Dllddp39dagjLj9j"
                          },
                          "video_id": {
                            "type": "string",
                            "description": "The video ID.",
                            "example": "csduzfv3JwTeyR5QpC3PXw52"
                          },
                          "meeting_id": {
                            "type": "string",
                            "description": "Zoom meeting ID or UUID. This is available only for recordings.",
                            "example": "rec1zfv3JwTeyR5QpC3PXw52"
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
                  "value": "{\n  \"event\": \"video_management.channel_video_added\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"iso9Dllddp39dagjLj9j\",\n      \"video_id\": \"abc9Dllddp39dagjLj9j\"\n    }\n  }\n}"
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
                      "description": "The name of the event. video_management.channel_video_added."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "A timestamp of when the event occurred."
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
                          "description": "The account ID of the host who published the video channel."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom Event description.",
                          "required": [
                            "channel_id",
                            "video_id"
                          ],
                          "properties": {
                            "channel_id": {
                              "type": "string",
                              "description": "The channel ID.",
                              "example": "iso9Dllddp39dagjLj9j"
                            },
                            "video_id": {
                              "type": "string",
                              "description": "The video ID.",
                              "example": "csduzfv3JwTeyR5QpC3PXw52"
                            },
                            "meeting_id": {
                              "type": "string",
                              "description": "Zoom meeting ID or UUID. This is available only for recordings.",
                              "example": "rec1zfv3JwTeyR5QpC3PXw52"
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
                    "value": "{\n  \"event\": \"video_management.channel_video_added\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"iso9Dllddp39dagjLj9j\",\n      \"video_id\": \"abc9Dllddp39dagjLj9j\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "video_management.channel_published": {
      "post": {
        "tags": [
          "video_management"
        ],
        "operationId": "video_management.channel_published",
        "requestBody": {
          "description": "# video_management.channel_published\nThe **Channel Published** event is triggered every time a user in your account or your app, publishes a video channel.\n\n\n## Prerequisites\n\n* Pro or a higher plan.\n* A Zoom Video Management license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Channel published** subscription enabled under the **Video Management** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt_channels:read`,`video_mgmt_channels:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt:read:channel`,`video_mgmt:read:channel:admin`\n\n**Event type**: `video_management.channel_published`\n",
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
                    "description": "The name of the event. video_management.channel_published."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "A timestamp when the event occurred."
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
                        "description": "The account ID of the host who published the video channel."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom Event description.",
                        "required": [
                          "channel_id",
                          "name",
                          "owner_id"
                        ],
                        "properties": {
                          "channel_id": {
                            "type": "string",
                            "description": "The channel ID.",
                            "example": "iso9Dllddp39dagjLj9j"
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the channel.",
                            "example": "FoodieFlicks"
                          },
                          "description": {
                            "type": "string",
                            "description": "Channel description.",
                            "example": "A cinematic journey through the world of cuisine!."
                          },
                          "owner_id": {
                            "type": "string",
                            "description": "The ID of the channel owner.",
                            "example": "abc9Dllddp39dagjLj9j"
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
                  "value": "{\n  \"event\": \"video_management.channel_published\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"FoodieFlicks\",\n      \"description\": \"A cinematic journey through the world of cuisine!.\",\n      \"owner_id\": \"abc9Dllddp39dagjLj9j\"\n    }\n  }\n}"
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
                      "description": "The name of the event. video_management.channel_published."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "A timestamp when the event occurred."
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
                          "description": "The account ID of the host who published the video channel."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom Event description.",
                          "required": [
                            "channel_id",
                            "name",
                            "owner_id"
                          ],
                          "properties": {
                            "channel_id": {
                              "type": "string",
                              "description": "The channel ID.",
                              "example": "iso9Dllddp39dagjLj9j"
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the channel.",
                              "example": "FoodieFlicks"
                            },
                            "description": {
                              "type": "string",
                              "description": "Channel description.",
                              "example": "A cinematic journey through the world of cuisine!."
                            },
                            "owner_id": {
                              "type": "string",
                              "description": "The ID of the channel owner.",
                              "example": "abc9Dllddp39dagjLj9j"
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
                    "value": "{\n  \"event\": \"video_management.channel_published\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"FoodieFlicks\",\n      \"description\": \"A cinematic journey through the world of cuisine!.\",\n      \"owner_id\": \"abc9Dllddp39dagjLj9j\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "video_management.channel_unpublished": {
      "post": {
        "tags": [
          "video_management"
        ],
        "operationId": "video_management.channel_unpublished",
        "requestBody": {
          "description": "# video_management.channel_unpublished\nThe **Channel unpublished** event is triggered every time a user in your account or your app unpublishes a video channel\n\n\n## Prerequisites\n\n* A [Pro or higher account plan](https://www.zoom.us/pricing?).\n* A Zoom Video Management license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Channel unpublished** subscription enabled under the **Video Management** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt_channels:read`,`video_mgmt_channels:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt:read:channel`,`video_mgmt:read:channel:admin`\n\n**Event type**: `video_management.channel_unpublished`\n",
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
                    "description": "The name of the event. "
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "A timestamp of when the event occurred."
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
                        "description": "The account ID of the host who published the video channel."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom Event description.",
                        "required": [
                          "channel_id",
                          "name",
                          "owner_id"
                        ],
                        "properties": {
                          "channel_id": {
                            "type": "string",
                            "description": "The channel ID.",
                            "example": "iso9Dllddp39dagjLj9j"
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the channel.",
                            "example": "FoodieFlicks"
                          },
                          "description": {
                            "type": "string",
                            "description": "The description of the channel.",
                            "example": "A cinematic journey through the world of cuisine!."
                          },
                          "owner_id": {
                            "type": "string",
                            "description": "The ID of the channel owner.",
                            "example": "abc9Dllddp39dagjLj9j"
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
                  "value": "{\n  \"event\": \"video_management.channel_unpublished\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"FoodieFlicks\",\n      \"description\": \"A cinematic journey through the world of cuisine!.\",\n      \"owner_id\": \"abc9Dllddp39dagjLj9j\"\n    }\n  }\n}"
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
                      "description": "The name of the event. "
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "A timestamp of when the event occurred."
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
                          "description": "The account ID of the host who published the video channel."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom Event description.",
                          "required": [
                            "channel_id",
                            "name",
                            "owner_id"
                          ],
                          "properties": {
                            "channel_id": {
                              "type": "string",
                              "description": "The channel ID.",
                              "example": "iso9Dllddp39dagjLj9j"
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the channel.",
                              "example": "FoodieFlicks"
                            },
                            "description": {
                              "type": "string",
                              "description": "The description of the channel.",
                              "example": "A cinematic journey through the world of cuisine!."
                            },
                            "owner_id": {
                              "type": "string",
                              "description": "The ID of the channel owner.",
                              "example": "abc9Dllddp39dagjLj9j"
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
                    "value": "{\n  \"event\": \"video_management.channel_unpublished\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"FoodieFlicks\",\n      \"description\": \"A cinematic journey through the world of cuisine!.\",\n      \"owner_id\": \"abc9Dllddp39dagjLj9j\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "video_management.channel_video_removed": {
      "post": {
        "tags": [
          "video_management"
        ],
        "operationId": "video_management.channel_video_removed",
        "requestBody": {
          "description": "# video_management.channel_video_removed\nThe **channel video removed** event is triggered every time a user in your account or your app, removed a video from a channel.\n\n\n## Prerequisites\n\n* A [Pro or higher account plan](https://www.zoom.us/pricing?)\n* A Zoom Video Management license\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**\n    * The **Channel Video Removed** subscription enabled under the **Video Management** event\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt_videos:read`,`video_mgmt_videos:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt:read:list_videos`,`video_mgmt:read:list_videos:admin`\n\n**Event type**: `video_management.channel_video_removed`\n",
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
                    "description": "The name of the event. video_management.channel_video_removed."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "The timestamp of when the event occurred."
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
                        "description": "The account ID of the host who published the video channel."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom event description.",
                        "required": [
                          "channel_id",
                          "video_id"
                        ],
                        "properties": {
                          "channel_id": {
                            "type": "string",
                            "description": "The channel ID.",
                            "example": "iso9Dllddp39dagjLj9j"
                          },
                          "video_id": {
                            "type": "string",
                            "description": "The video ID.",
                            "example": "csduzfv3JwTeyR5QpC3PXw52"
                          },
                          "meeting_id": {
                            "type": "string",
                            "description": "Zoom meeting ID or UUID. This is available only for recordings.",
                            "example": "rec1zfv3JwTeyR5QpC3PXw52"
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
                  "value": "{\n  \"event\": \"video_management.channel_video_removed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"iso9Dllddp39dagjLj9j\",\n      \"video_id\": \"abc9Dllddp39dagjLj9j\"\n    }\n  }\n}"
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
                      "description": "The name of the event. video_management.channel_video_removed."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "The timestamp of when the event occurred."
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
                          "description": "The account ID of the host who published the video channel."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom event description.",
                          "required": [
                            "channel_id",
                            "video_id"
                          ],
                          "properties": {
                            "channel_id": {
                              "type": "string",
                              "description": "The channel ID.",
                              "example": "iso9Dllddp39dagjLj9j"
                            },
                            "video_id": {
                              "type": "string",
                              "description": "The video ID.",
                              "example": "csduzfv3JwTeyR5QpC3PXw52"
                            },
                            "meeting_id": {
                              "type": "string",
                              "description": "Zoom meeting ID or UUID. This is available only for recordings.",
                              "example": "rec1zfv3JwTeyR5QpC3PXw52"
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
                    "value": "{\n  \"event\": \"video_management.channel_video_removed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"iso9Dllddp39dagjLj9j\",\n      \"video_id\": \"abc9Dllddp39dagjLj9j\"\n    }\n  }\n}"
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
    "title": "Video Management",
    "description": "",
    "version": "1.0.0"
  },
  "webhooks": {
    "video_management.channel_video_added": {
      "post": {
        "tags": [
          "video_management"
        ],
        "operationId": "video_management.channel_video_added",
        "requestBody": {
          "description": "# video_management.channel_video_added\nThe **channel video added** event is triggered every time a user in your account, or your app, adds a video to a channel.\n\n\n## Prerequisites\n\n* A [Pro or higher account plan](https://www.zoom.us/pricing?)\n* A Zoom Video Management license\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**\n    * The **Channel Video Added** subscription enabled under the **Video Management** event\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt_videos:read`,`video_mgmt_videos:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt:read:list_videos`,`video_mgmt:read:list_videos:admin`\n\n**Event type**: `video_management.channel_video_added`\n",
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
                    "description": "The name of the event. video_management.channel_video_added."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "A timestamp of when the event occurred."
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
                        "description": "The account ID of the host who published the video channel."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom Event description.",
                        "required": [
                          "channel_id",
                          "video_id"
                        ],
                        "properties": {
                          "channel_id": {
                            "type": "string",
                            "description": "The channel ID.",
                            "example": "iso9Dllddp39dagjLj9j"
                          },
                          "video_id": {
                            "type": "string",
                            "description": "The video ID.",
                            "example": "csduzfv3JwTeyR5QpC3PXw52"
                          },
                          "meeting_id": {
                            "type": "string",
                            "description": "Zoom meeting ID or UUID. This is available only for recordings.",
                            "example": "rec1zfv3JwTeyR5QpC3PXw52"
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
                  "value": "{\n  \"event\": \"video_management.channel_video_added\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"iso9Dllddp39dagjLj9j\",\n      \"video_id\": \"abc9Dllddp39dagjLj9j\"\n    }\n  }\n}"
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
                      "description": "The name of the event. video_management.channel_video_added."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "A timestamp of when the event occurred."
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
                          "description": "The account ID of the host who published the video channel."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom Event description.",
                          "required": [
                            "channel_id",
                            "video_id"
                          ],
                          "properties": {
                            "channel_id": {
                              "type": "string",
                              "description": "The channel ID.",
                              "example": "iso9Dllddp39dagjLj9j"
                            },
                            "video_id": {
                              "type": "string",
                              "description": "The video ID.",
                              "example": "csduzfv3JwTeyR5QpC3PXw52"
                            },
                            "meeting_id": {
                              "type": "string",
                              "description": "Zoom meeting ID or UUID. This is available only for recordings.",
                              "example": "rec1zfv3JwTeyR5QpC3PXw52"
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
                    "value": "{\n  \"event\": \"video_management.channel_video_added\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"iso9Dllddp39dagjLj9j\",\n      \"video_id\": \"abc9Dllddp39dagjLj9j\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "video_management.channel_published": {
      "post": {
        "tags": [
          "video_management"
        ],
        "operationId": "video_management.channel_published",
        "requestBody": {
          "description": "# video_management.channel_published\nThe **Channel Published** event is triggered every time a user in your account or your app, publishes a video channel.\n\n\n## Prerequisites\n\n* Pro or a higher plan.\n* A Zoom Video Management license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Channel published** subscription enabled under the **Video Management** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt_channels:read`,`video_mgmt_channels:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt:read:channel`,`video_mgmt:read:channel:admin`\n\n**Event type**: `video_management.channel_published`\n",
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
                    "description": "The name of the event. video_management.channel_published."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "A timestamp when the event occurred."
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
                        "description": "The account ID of the host who published the video channel."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom Event description.",
                        "required": [
                          "channel_id",
                          "name",
                          "owner_id"
                        ],
                        "properties": {
                          "channel_id": {
                            "type": "string",
                            "description": "The channel ID.",
                            "example": "iso9Dllddp39dagjLj9j"
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the channel.",
                            "example": "FoodieFlicks"
                          },
                          "description": {
                            "type": "string",
                            "description": "Channel description.",
                            "example": "A cinematic journey through the world of cuisine!."
                          },
                          "owner_id": {
                            "type": "string",
                            "description": "The ID of the channel owner.",
                            "example": "abc9Dllddp39dagjLj9j"
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
                  "value": "{\n  \"event\": \"video_management.channel_published\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"FoodieFlicks\",\n      \"description\": \"A cinematic journey through the world of cuisine!.\",\n      \"owner_id\": \"abc9Dllddp39dagjLj9j\"\n    }\n  }\n}"
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
                      "description": "The name of the event. video_management.channel_published."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "A timestamp when the event occurred."
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
                          "description": "The account ID of the host who published the video channel."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom Event description.",
                          "required": [
                            "channel_id",
                            "name",
                            "owner_id"
                          ],
                          "properties": {
                            "channel_id": {
                              "type": "string",
                              "description": "The channel ID.",
                              "example": "iso9Dllddp39dagjLj9j"
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the channel.",
                              "example": "FoodieFlicks"
                            },
                            "description": {
                              "type": "string",
                              "description": "Channel description.",
                              "example": "A cinematic journey through the world of cuisine!."
                            },
                            "owner_id": {
                              "type": "string",
                              "description": "The ID of the channel owner.",
                              "example": "abc9Dllddp39dagjLj9j"
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
                    "value": "{\n  \"event\": \"video_management.channel_published\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"FoodieFlicks\",\n      \"description\": \"A cinematic journey through the world of cuisine!.\",\n      \"owner_id\": \"abc9Dllddp39dagjLj9j\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "video_management.channel_unpublished": {
      "post": {
        "tags": [
          "video_management"
        ],
        "operationId": "video_management.channel_unpublished",
        "requestBody": {
          "description": "# video_management.channel_unpublished\nThe **Channel unpublished** event is triggered every time a user in your account or your app unpublishes a video channel\n\n\n## Prerequisites\n\n* A [Pro or higher account plan](https://www.zoom.us/pricing?).\n* A Zoom Video Management license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Channel unpublished** subscription enabled under the **Video Management** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt_channels:read`,`video_mgmt_channels:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt:read:channel`,`video_mgmt:read:channel:admin`\n\n**Event type**: `video_management.channel_unpublished`\n",
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
                    "description": "The name of the event. "
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "A timestamp of when the event occurred."
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
                        "description": "The account ID of the host who published the video channel."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom Event description.",
                        "required": [
                          "channel_id",
                          "name",
                          "owner_id"
                        ],
                        "properties": {
                          "channel_id": {
                            "type": "string",
                            "description": "The channel ID.",
                            "example": "iso9Dllddp39dagjLj9j"
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the channel.",
                            "example": "FoodieFlicks"
                          },
                          "description": {
                            "type": "string",
                            "description": "The description of the channel.",
                            "example": "A cinematic journey through the world of cuisine!."
                          },
                          "owner_id": {
                            "type": "string",
                            "description": "The ID of the channel owner.",
                            "example": "abc9Dllddp39dagjLj9j"
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
                  "value": "{\n  \"event\": \"video_management.channel_unpublished\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"FoodieFlicks\",\n      \"description\": \"A cinematic journey through the world of cuisine!.\",\n      \"owner_id\": \"abc9Dllddp39dagjLj9j\"\n    }\n  }\n}"
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
                      "description": "The name of the event. "
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "A timestamp of when the event occurred."
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
                          "description": "The account ID of the host who published the video channel."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom Event description.",
                          "required": [
                            "channel_id",
                            "name",
                            "owner_id"
                          ],
                          "properties": {
                            "channel_id": {
                              "type": "string",
                              "description": "The channel ID.",
                              "example": "iso9Dllddp39dagjLj9j"
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the channel.",
                              "example": "FoodieFlicks"
                            },
                            "description": {
                              "type": "string",
                              "description": "The description of the channel.",
                              "example": "A cinematic journey through the world of cuisine!."
                            },
                            "owner_id": {
                              "type": "string",
                              "description": "The ID of the channel owner.",
                              "example": "abc9Dllddp39dagjLj9j"
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
                    "value": "{\n  \"event\": \"video_management.channel_unpublished\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"FoodieFlicks\",\n      \"description\": \"A cinematic journey through the world of cuisine!.\",\n      \"owner_id\": \"abc9Dllddp39dagjLj9j\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "video_management.channel_video_removed": {
      "post": {
        "tags": [
          "video_management"
        ],
        "operationId": "video_management.channel_video_removed",
        "requestBody": {
          "description": "# video_management.channel_video_removed\nThe **channel video removed** event is triggered every time a user in your account or your app, removed a video from a channel.\n\n\n## Prerequisites\n\n* A [Pro or higher account plan](https://www.zoom.us/pricing?)\n* A Zoom Video Management license\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**\n    * The **Channel Video Removed** subscription enabled under the **Video Management** event\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt_videos:read`,`video_mgmt_videos:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt:read:list_videos`,`video_mgmt:read:list_videos:admin`\n\n**Event type**: `video_management.channel_video_removed`\n",
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
                    "description": "The name of the event. video_management.channel_video_removed."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "The timestamp of when the event occurred."
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
                        "description": "The account ID of the host who published the video channel."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom event description.",
                        "required": [
                          "channel_id",
                          "video_id"
                        ],
                        "properties": {
                          "channel_id": {
                            "type": "string",
                            "description": "The channel ID.",
                            "example": "iso9Dllddp39dagjLj9j"
                          },
                          "video_id": {
                            "type": "string",
                            "description": "The video ID.",
                            "example": "csduzfv3JwTeyR5QpC3PXw52"
                          },
                          "meeting_id": {
                            "type": "string",
                            "description": "Zoom meeting ID or UUID. This is available only for recordings.",
                            "example": "rec1zfv3JwTeyR5QpC3PXw52"
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
                  "value": "{\n  \"event\": \"video_management.channel_video_removed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"iso9Dllddp39dagjLj9j\",\n      \"video_id\": \"abc9Dllddp39dagjLj9j\"\n    }\n  }\n}"
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
                      "description": "The name of the event. video_management.channel_video_removed."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "The timestamp of when the event occurred."
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
                          "description": "The account ID of the host who published the video channel."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom event description.",
                          "required": [
                            "channel_id",
                            "video_id"
                          ],
                          "properties": {
                            "channel_id": {
                              "type": "string",
                              "description": "The channel ID.",
                              "example": "iso9Dllddp39dagjLj9j"
                            },
                            "video_id": {
                              "type": "string",
                              "description": "The video ID.",
                              "example": "csduzfv3JwTeyR5QpC3PXw52"
                            },
                            "meeting_id": {
                              "type": "string",
                              "description": "Zoom meeting ID or UUID. This is available only for recordings.",
                              "example": "rec1zfv3JwTeyR5QpC3PXw52"
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
                    "value": "{\n  \"event\": \"video_management.channel_video_removed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"iso9Dllddp39dagjLj9j\",\n      \"video_id\": \"abc9Dllddp39dagjLj9j\"\n    }\n  }\n}"
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
      "video-management",
      "events"
    ]
  },
  "downloadPath": "/api-hub/video-management/events/webhooks.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": false
  },
  "productMeta": {
    "specType": "adoc",
    "slug": "/docs/api/rest/reference/video-management/events",
    "title": "Video Management",
    "keywords": "zoom, rest, api, ai, aic, video-management",
    "params": {
      "productName": "Zoom Video Management"
    },
    "zoapEndpoint": "webhookByProduct",
    "productDisplayTitle": "Video Management",
    "scalarSlug": "/docs/api/video-management/events",
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "svgname": "video-management"
  },
  "frontmatter": {
    "title": "Video Management Webhooks",
    "keywords": "zoom, rest, api, ai, aic, video-management",
    "description": "Video Management events",
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
        "video-management",
        "events"
      ]
    },
    "downloadPath": "/api-hub/video-management/events/webhooks.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": false
    },
    "productMeta": {
      "specType": "adoc",
      "slug": "/docs/api/rest/reference/video-management/events",
      "title": "Video Management",
      "keywords": "zoom, rest, api, ai, aic, video-management",
      "params": {
        "productName": "Zoom Video Management"
      },
      "zoapEndpoint": "webhookByProduct",
      "productDisplayTitle": "Video Management",
      "scalarSlug": "/docs/api/video-management/events",
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "svgname": "video-management"
    },
    "spec": "{\"openapi\":\"3.1.0\",\"info\":{\"title\":\"Video Management\",\"description\":\"\",\"version\":\"1.0.0\"},\"paths\":{\"video_management.channel_video_added\":{\"post\":{\"tags\":[\"video_management\"],\"operationId\":\"video_management.channel_video_added\",\"requestBody\":{\"description\":\"# video_management.channel_video_added\\nThe **channel video added** event is triggered every time a user in your account, or your app, adds a video to a channel.\\n\\n\\n## Prerequisites\\n\\n* A [Pro or higher account plan](https://www.zoom.us/pricing?)\\n* A Zoom Video Management license\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**\\n    * The **Channel Video Added** subscription enabled under the **Video Management** event\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt_videos:read`,`video_mgmt_videos:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt:read:list_videos`,`video_mgmt:read:list_videos:admin`\\n\\n**Event type**: `video_management.channel_video_added`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. video_management.channel_video_added.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who published the video channel.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom Event description.\",\"required\":[\"channel_id\",\"video_id\"],\"properties\":{\"channel_id\":{\"type\":\"string\",\"description\":\"The channel ID.\",\"example\":\"iso9Dllddp39dagjLj9j\"},\"video_id\":{\"type\":\"string\",\"description\":\"The video ID.\",\"example\":\"csduzfv3JwTeyR5QpC3PXw52\"},\"meeting_id\":{\"type\":\"string\",\"description\":\"Zoom meeting ID or UUID. This is available only for recordings.\",\"example\":\"rec1zfv3JwTeyR5QpC3PXw52\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"video_management.channel_video_added\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"channel_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"video_id\\\": \\\"abc9Dllddp39dagjLj9j\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. video_management.channel_video_added.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who published the video channel.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom Event description.\",\"required\":[\"channel_id\",\"video_id\"],\"properties\":{\"channel_id\":{\"type\":\"string\",\"description\":\"The channel ID.\",\"example\":\"iso9Dllddp39dagjLj9j\"},\"video_id\":{\"type\":\"string\",\"description\":\"The video ID.\",\"example\":\"csduzfv3JwTeyR5QpC3PXw52\"},\"meeting_id\":{\"type\":\"string\",\"description\":\"Zoom meeting ID or UUID. This is available only for recordings.\",\"example\":\"rec1zfv3JwTeyR5QpC3PXw52\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"video_management.channel_video_added\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"channel_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"video_id\\\": \\\"abc9Dllddp39dagjLj9j\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"video_management.channel_published\":{\"post\":{\"tags\":[\"video_management\"],\"operationId\":\"video_management.channel_published\",\"requestBody\":{\"description\":\"# video_management.channel_published\\nThe **Channel Published** event is triggered every time a user in your account or your app, publishes a video channel.\\n\\n\\n## Prerequisites\\n\\n* Pro or a higher plan.\\n* A Zoom Video Management license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **Channel published** subscription enabled under the **Video Management** event.\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt_channels:read`,`video_mgmt_channels:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt:read:channel`,`video_mgmt:read:channel:admin`\\n\\n**Event type**: `video_management.channel_published`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. video_management.channel_published.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who published the video channel.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom Event description.\",\"required\":[\"channel_id\",\"name\",\"owner_id\"],\"properties\":{\"channel_id\":{\"type\":\"string\",\"description\":\"The channel ID.\",\"example\":\"iso9Dllddp39dagjLj9j\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the channel.\",\"example\":\"FoodieFlicks\"},\"description\":{\"type\":\"string\",\"description\":\"Channel description.\",\"example\":\"A cinematic journey through the world of cuisine!.\"},\"owner_id\":{\"type\":\"string\",\"description\":\"The ID of the channel owner.\",\"example\":\"abc9Dllddp39dagjLj9j\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"video_management.channel_published\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"channel_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"name\\\": \\\"FoodieFlicks\\\",\\n      \\\"description\\\": \\\"A cinematic journey through the world of cuisine!.\\\",\\n      \\\"owner_id\\\": \\\"abc9Dllddp39dagjLj9j\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. video_management.channel_published.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who published the video channel.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom Event description.\",\"required\":[\"channel_id\",\"name\",\"owner_id\"],\"properties\":{\"channel_id\":{\"type\":\"string\",\"description\":\"The channel ID.\",\"example\":\"iso9Dllddp39dagjLj9j\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the channel.\",\"example\":\"FoodieFlicks\"},\"description\":{\"type\":\"string\",\"description\":\"Channel description.\",\"example\":\"A cinematic journey through the world of cuisine!.\"},\"owner_id\":{\"type\":\"string\",\"description\":\"The ID of the channel owner.\",\"example\":\"abc9Dllddp39dagjLj9j\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"video_management.channel_published\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"channel_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"name\\\": \\\"FoodieFlicks\\\",\\n      \\\"description\\\": \\\"A cinematic journey through the world of cuisine!.\\\",\\n      \\\"owner_id\\\": \\\"abc9Dllddp39dagjLj9j\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"video_management.channel_unpublished\":{\"post\":{\"tags\":[\"video_management\"],\"operationId\":\"video_management.channel_unpublished\",\"requestBody\":{\"description\":\"# video_management.channel_unpublished\\nThe **Channel unpublished** event is triggered every time a user in your account or your app unpublishes a video channel\\n\\n\\n## Prerequisites\\n\\n* A [Pro or higher account plan](https://www.zoom.us/pricing?).\\n* A Zoom Video Management license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **Channel unpublished** subscription enabled under the **Video Management** event.\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt_channels:read`,`video_mgmt_channels:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt:read:channel`,`video_mgmt:read:channel:admin`\\n\\n**Event type**: `video_management.channel_unpublished`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. \"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who published the video channel.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom Event description.\",\"required\":[\"channel_id\",\"name\",\"owner_id\"],\"properties\":{\"channel_id\":{\"type\":\"string\",\"description\":\"The channel ID.\",\"example\":\"iso9Dllddp39dagjLj9j\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the channel.\",\"example\":\"FoodieFlicks\"},\"description\":{\"type\":\"string\",\"description\":\"The description of the channel.\",\"example\":\"A cinematic journey through the world of cuisine!.\"},\"owner_id\":{\"type\":\"string\",\"description\":\"The ID of the channel owner.\",\"example\":\"abc9Dllddp39dagjLj9j\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"video_management.channel_unpublished\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"channel_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"name\\\": \\\"FoodieFlicks\\\",\\n      \\\"description\\\": \\\"A cinematic journey through the world of cuisine!.\\\",\\n      \\\"owner_id\\\": \\\"abc9Dllddp39dagjLj9j\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. \"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who published the video channel.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom Event description.\",\"required\":[\"channel_id\",\"name\",\"owner_id\"],\"properties\":{\"channel_id\":{\"type\":\"string\",\"description\":\"The channel ID.\",\"example\":\"iso9Dllddp39dagjLj9j\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the channel.\",\"example\":\"FoodieFlicks\"},\"description\":{\"type\":\"string\",\"description\":\"The description of the channel.\",\"example\":\"A cinematic journey through the world of cuisine!.\"},\"owner_id\":{\"type\":\"string\",\"description\":\"The ID of the channel owner.\",\"example\":\"abc9Dllddp39dagjLj9j\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"video_management.channel_unpublished\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"channel_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"name\\\": \\\"FoodieFlicks\\\",\\n      \\\"description\\\": \\\"A cinematic journey through the world of cuisine!.\\\",\\n      \\\"owner_id\\\": \\\"abc9Dllddp39dagjLj9j\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"video_management.channel_video_removed\":{\"post\":{\"tags\":[\"video_management\"],\"operationId\":\"video_management.channel_video_removed\",\"requestBody\":{\"description\":\"# video_management.channel_video_removed\\nThe **channel video removed** event is triggered every time a user in your account or your app, removed a video from a channel.\\n\\n\\n## Prerequisites\\n\\n* A [Pro or higher account plan](https://www.zoom.us/pricing?)\\n* A Zoom Video Management license\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**\\n    * The **Channel Video Removed** subscription enabled under the **Video Management** event\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt_videos:read`,`video_mgmt_videos:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `video_mgmt:read:list_videos`,`video_mgmt:read:list_videos:admin`\\n\\n**Event type**: `video_management.channel_video_removed`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. video_management.channel_video_removed.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who published the video channel.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom event description.\",\"required\":[\"channel_id\",\"video_id\"],\"properties\":{\"channel_id\":{\"type\":\"string\",\"description\":\"The channel ID.\",\"example\":\"iso9Dllddp39dagjLj9j\"},\"video_id\":{\"type\":\"string\",\"description\":\"The video ID.\",\"example\":\"csduzfv3JwTeyR5QpC3PXw52\"},\"meeting_id\":{\"type\":\"string\",\"description\":\"Zoom meeting ID or UUID. This is available only for recordings.\",\"example\":\"rec1zfv3JwTeyR5QpC3PXw52\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"video_management.channel_video_removed\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"channel_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"video_id\\\": \\\"abc9Dllddp39dagjLj9j\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. video_management.channel_video_removed.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who published the video channel.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom event description.\",\"required\":[\"channel_id\",\"video_id\"],\"properties\":{\"channel_id\":{\"type\":\"string\",\"description\":\"The channel ID.\",\"example\":\"iso9Dllddp39dagjLj9j\"},\"video_id\":{\"type\":\"string\",\"description\":\"The video ID.\",\"example\":\"csduzfv3JwTeyR5QpC3PXw52\"},\"meeting_id\":{\"type\":\"string\",\"description\":\"Zoom meeting ID or UUID. This is available only for recordings.\",\"example\":\"rec1zfv3JwTeyR5QpC3PXw52\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"video_management.channel_video_removed\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"channel_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"video_id\\\": \\\"abc9Dllddp39dagjLj9j\\\"\\n    }\\n  }\\n}\"}}}}}}}}},\"servers\":[{\"url\":\"https://{your-endpoint-url}\"}]}",
    "frontmatter": {
      "title": "Video Management Webhooks",
      "keywords": "zoom, rest, api, ai, aic, video-management",
      "description": "Video Management events",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
