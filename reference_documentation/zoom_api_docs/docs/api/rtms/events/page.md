# RTMS Webhooks

- Source URL: https://developers.zoom.us/docs/api/rtms/events/
- Snapshot path: docs/api/rtms/events
- Generated (UTC): 2026-02-07T22:40:11.609009+00:00

## Frontmatter

```json
{
  "title": "RTMS Webhooks",
  "keywords": "zoom, webhooks, api, rtms",
  "description": "RTMS is a data pipeline that gives your app access to live audio, video, and transcript data from Zoom Meetings.",
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
    "title": "RTMS",
    "description": "RTMS is a data pipeline that gives your app access to live audio, video, and transcript data from Zoom Meetings.",
    "version": "1.0.0"
  },
  "paths": {
    "session.rtms_interrupted": {
      "post": {
        "tags": [
          "session"
        ],
        "operationId": "session.rtms_interrupted",
        "requestBody": {
          "description": "# session.rtms_interrupted\nThe **RTMS Interrupted** event is triggered when an ongoing session stream gets disrupted due to a broken signaling socket connection.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS Interrupted** subscription enabled under the **RTMS** event.\n\n**Event type**: `session.rtms_interrupted`\n",
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
                    "description": "A timestamp of when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "description": "Information about the session rtms event.",
                    "required": [
                      "session_id",
                      "session_key",
                      "rtms_stream_id"
                    ],
                    "properties": {
                      "session_id": {
                        "type": "string",
                        "description": "The session's universally unique identifier (UUID). Each session instance generates a meeting UUID."
                      },
                      "session_key": {
                        "type": "string",
                        "description": "The Video SDK custom session ID."
                      },
                      "rtms_stream_id": {
                        "type": "string",
                        "description": "The unique ID of one of streams of the session.\n **Note**: A session can have multiple streams."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"session.rtms_interrupted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n    \"session_key\": \"ABC36jaBI145\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\"\n  }\n}"
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
                      "description": "A timestamp of when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "description": "Information about the session rtms event.",
                      "required": [
                        "session_id",
                        "session_key",
                        "rtms_stream_id"
                      ],
                      "properties": {
                        "session_id": {
                          "type": "string",
                          "description": "The session's universally unique identifier (UUID). Each session instance generates a meeting UUID."
                        },
                        "session_key": {
                          "type": "string",
                          "description": "The Video SDK custom session ID."
                        },
                        "rtms_stream_id": {
                          "type": "string",
                          "description": "The unique ID of one of streams of the session.\n **Note**: A session can have multiple streams."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"session.rtms_interrupted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n    \"session_key\": \"ABC36jaBI145\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "meeting.rtms_interrupted": {
      "post": {
        "tags": [
          "meeting"
        ],
        "operationId": "meeting.rtms_interrupted",
        "requestBody": {
          "description": "# meeting.rtms_interrupted\nThe **RTMS interrupted in Meeting** event is triggered when an ongoing meeting stream gets disrupted due to a broken signaling socket connection.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS interrupted in Meeting** subscription enabled under the **RTMS** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_interrupted`,`rtms:read:rtms_interrupted:admin`\n\n**Event type**: `meeting.rtms_interrupted`\n",
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
                    "description": "Information about the meeting rtms event.",
                    "required": [
                      "meeting_uuid",
                      "rtms_stream_id"
                    ],
                    "properties": {
                      "meeting_uuid": {
                        "type": "string",
                        "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID."
                      },
                      "rtms_stream_id": {
                        "type": "string",
                        "description": "The unique ID of one of streams of the meeting. Each meeting can has multiple streams."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"meeting.rtms_interrupted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\"\n  }\n}"
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
                      "description": "Information about the meeting rtms event.",
                      "required": [
                        "meeting_uuid",
                        "rtms_stream_id"
                      ],
                      "properties": {
                        "meeting_uuid": {
                          "type": "string",
                          "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID."
                        },
                        "rtms_stream_id": {
                          "type": "string",
                          "description": "The unique ID of one of streams of the meeting. Each meeting can has multiple streams."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"meeting.rtms_interrupted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "meeting.rtms_stopped": {
      "post": {
        "tags": [
          "meeting"
        ],
        "operationId": "meeting.rtms_stopped",
        "requestBody": {
          "description": "# meeting.rtms_stopped\nThe **RTMS stopped in Meeting** event is triggered when the meeting host or the initiator stops a real-time stream.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS stopped in Meeting** subscription enabled under the **RTMS** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_stopped`,`rtms:read:rtms_stopped:admin`\n\n**Event type**: `meeting.rtms_stopped`\n",
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
                    "description": "Information about the meeting rtms event.",
                    "required": [
                      "meeting_uuid",
                      "rtms_stream_id",
                      "stop_reason"
                    ],
                    "properties": {
                      "meeting_uuid": {
                        "type": "string",
                        "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID."
                      },
                      "rtms_stream_id": {
                        "type": "string",
                        "description": "The unique ID of one of streams of the meeting. Each meeting can has multiple streams."
                      },
                      "stop_reason": {
                        "type": "integer",
                        "format": "int",
                        "description": "The reason code of the stream stopped.\n* 1 \u2014 Triggered by the meeting host. \n* 2 \u2014 Triggered by the app user. \n* 3 \u2014 App user left meeting. \n* 4 \u2014 App user ejected by the meeting host. \n* 5 \u2014 The app is disabled by the meeting host. \n* 6 \u2014 The meeting is ended. \n* 7 \u2014 The stream is canceled. \n* 8 \u2014 The stream revoked. \n* 9 \u2014 All apps are disabled. \n* 10 \u2014 Server internal exception. \n* 11 \u2014 The connection timed out.",
                        "enum": [
                          1,
                          2,
                          3,
                          4,
                          7,
                          8,
                          9,
                          10,
                          11
                        ]
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"meeting.rtms_stopped\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"stop_reason\": 1\n  }\n}"
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
                      "description": "Information about the meeting rtms event.",
                      "required": [
                        "meeting_uuid",
                        "rtms_stream_id",
                        "stop_reason"
                      ],
                      "properties": {
                        "meeting_uuid": {
                          "type": "string",
                          "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID."
                        },
                        "rtms_stream_id": {
                          "type": "string",
                          "description": "The unique ID of one of streams of the meeting. Each meeting can has multiple streams."
                        },
                        "stop_reason": {
                          "type": "integer",
                          "format": "int",
                          "description": "The reason code of the stream stopped.\n* 1 \u2014 Triggered by the meeting host. \n* 2 \u2014 Triggered by the app user. \n* 3 \u2014 App user left meeting. \n* 4 \u2014 App user ejected by the meeting host. \n* 5 \u2014 The app is disabled by the meeting host. \n* 6 \u2014 The meeting is ended. \n* 7 \u2014 The stream is canceled. \n* 8 \u2014 The stream revoked. \n* 9 \u2014 All apps are disabled. \n* 10 \u2014 Server internal exception. \n* 11 \u2014 The connection timed out.",
                          "enum": [
                            1,
                            2,
                            3,
                            4,
                            7,
                            8,
                            9,
                            10,
                            11
                          ]
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"meeting.rtms_stopped\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"stop_reason\": 1\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "rtms.concurrency_near_limit": {
      "post": {
        "tags": [
          "rtms"
        ],
        "operationId": "rtms.concurrency_near_limit",
        "requestBody": {
          "description": "# rtms.concurrency_near_limit\nThe **RTMS Concurrency Near Limit** event is triggered when the app is nearing the predetermined number of concurrent streams. The default limit is 2000 concurrent streams.\n\nThis event periodically notifies the app owner when the number of concurrent streams exceeds 80% of the available concurrent streams.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**\n\t* The **RTMS Concurrency Near Limit** subscription enabled under the **RTMS** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_concurrency_near_limit`,`rtms:read:rtms_concurrency_near_limit:admin`\n\n**Event type**: `rtms.concurrency_near_limit`\n",
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
                    "description": "A timestamp of when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "description": "Information about the meeting RTMS event.",
                    "required": [
                      "concurrency_limit",
                      "concurrency_number",
                      "percentage"
                    ],
                    "properties": {
                      "concurrency_limit": {
                        "type": "integer",
                        "description": "The number of the predetermined available concurrent streams."
                      },
                      "concurrency_number": {
                        "type": "integer",
                        "description": "The number of concurrent streams."
                      },
                      "percentage": {
                        "type": "integer",
                        "description": "The number of concurrent streams as a percentage of the available concurrent streams."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"rtms.concurrency_near_limit\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"concurrency_limit\": 2000,\n    \"concurrency_number\": 1601,\n    \"percentage\": 80\n  }\n}"
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
                      "description": "A timestamp of when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "description": "Information about the meeting RTMS event.",
                      "required": [
                        "concurrency_limit",
                        "concurrency_number",
                        "percentage"
                      ],
                      "properties": {
                        "concurrency_limit": {
                          "type": "integer",
                          "description": "The number of the predetermined available concurrent streams."
                        },
                        "concurrency_number": {
                          "type": "integer",
                          "description": "The number of concurrent streams."
                        },
                        "percentage": {
                          "type": "integer",
                          "description": "The number of concurrent streams as a percentage of the available concurrent streams."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"rtms.concurrency_near_limit\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"concurrency_limit\": 2000,\n    \"concurrency_number\": 1601,\n    \"percentage\": 80\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "meeting.rtms_started": {
      "post": {
        "tags": [
          "meeting"
        ],
        "operationId": "meeting.rtms_started",
        "requestBody": {
          "description": "# meeting.rtms_started\nThe **RTMS started in Meeting** event is triggered when a meeting host or a meeting participant starts a real-time stream action.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS started in Meeting** subscription enabled under the **RTMS** event.\n  * The initiator must be a user of your [webhook-enabled](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks) app.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_started`,`rtms:read:rtms_started:admin`\n\n**Event type**: `meeting.rtms_started`\n",
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
                    "description": "Information about the meeting rtms event.",
                    "required": [
                      "meeting_uuid",
                      "operator_id",
                      "rtms_stream_id",
                      "server_urls"
                    ],
                    "properties": {
                      "meeting_uuid": {
                        "type": "string",
                        "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The ID of the user who is triggered this rtms event.",
                        "example": "30R7kT7bTIKSNUFEuH_Qlg"
                      },
                      "rtms_stream_id": {
                        "type": "string",
                        "description": "The unique ID of one of streams of the meeting. Each meeting can has multiple streams."
                      },
                      "server_urls": {
                        "type": "string",
                        "description": "The RTMS server urls for app to connect with. Multiple URLs for different protocols will be separated by commas."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"meeting.rtms_started\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"operator_id\": \"30R7kT7bTIKSNUFEuH_Qlg\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"server_urls\": \"wss://127.0.0.1:443\"\n  }\n}"
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
                      "description": "Information about the meeting rtms event.",
                      "required": [
                        "meeting_uuid",
                        "operator_id",
                        "rtms_stream_id",
                        "server_urls"
                      ],
                      "properties": {
                        "meeting_uuid": {
                          "type": "string",
                          "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The ID of the user who is triggered this rtms event.",
                          "example": "30R7kT7bTIKSNUFEuH_Qlg"
                        },
                        "rtms_stream_id": {
                          "type": "string",
                          "description": "The unique ID of one of streams of the meeting. Each meeting can has multiple streams."
                        },
                        "server_urls": {
                          "type": "string",
                          "description": "The RTMS server urls for app to connect with. Multiple URLs for different protocols will be separated by commas."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"meeting.rtms_started\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"operator_id\": \"30R7kT7bTIKSNUFEuH_Qlg\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"server_urls\": \"wss://127.0.0.1:443\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "webinar.rtms_stopped": {
      "post": {
        "tags": [
          "webinar"
        ],
        "operationId": "webinar.rtms_stopped",
        "requestBody": {
          "description": "# webinar.rtms_stopped\nThe **RTMS stopped in Webinar** event is triggered when the webinar host or the initiator stops a real-time stream.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS stopped in Webinar** subscription enabled under the **RTMS** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_stopped`,`rtms:read:rtms_stopped:admin`\n\n**Event type**: `webinar.rtms_stopped`\n",
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
                    "description": "Information about the webinar RTMS event.",
                    "required": [
                      "webinar_uuid",
                      "rtms_stream_id",
                      "stop_reason"
                    ],
                    "properties": {
                      "webinar_uuid": {
                        "type": "string",
                        "description": "The webinar's universally unique identifier (UUID). Each webinar instance generates a webinar UUID."
                      },
                      "rtms_stream_id": {
                        "type": "string",
                        "description": "The unique ID of one of streams of the webinar. Each webinar can has multiple streams."
                      },
                      "stop_reason": {
                        "type": "integer",
                        "format": "int",
                        "description": "The reason code of why the stream stopped.\n* 1 = Triggered by the webinar host. \n* 2 = Triggered by the app user. \n* 3 = App user left webinar. \n* 4 = App user ejected by the webinar host. \n* 5 = The app is disabled by the webinar host. \n* 6 = The webinar is ended. \n* 7 = The stream is canceled. \n* 8 = The stream revoked. \n* 9 = All apps are disabled. \n* 10 = Server internal exception. \n* 11 = The connection timed out.",
                        "enum": [
                          1,
                          2,
                          3,
                          4,
                          7,
                          8,
                          9,
                          10,
                          11
                        ]
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"webinar.rtms_stopped\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"webinar_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"stop_reason\": 1\n  }\n}"
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
                      "description": "Information about the webinar RTMS event.",
                      "required": [
                        "webinar_uuid",
                        "rtms_stream_id",
                        "stop_reason"
                      ],
                      "properties": {
                        "webinar_uuid": {
                          "type": "string",
                          "description": "The webinar's universally unique identifier (UUID). Each webinar instance generates a webinar UUID."
                        },
                        "rtms_stream_id": {
                          "type": "string",
                          "description": "The unique ID of one of streams of the webinar. Each webinar can has multiple streams."
                        },
                        "stop_reason": {
                          "type": "integer",
                          "format": "int",
                          "description": "The reason code of why the stream stopped.\n* 1 = Triggered by the webinar host. \n* 2 = Triggered by the app user. \n* 3 = App user left webinar. \n* 4 = App user ejected by the webinar host. \n* 5 = The app is disabled by the webinar host. \n* 6 = The webinar is ended. \n* 7 = The stream is canceled. \n* 8 = The stream revoked. \n* 9 = All apps are disabled. \n* 10 = Server internal exception. \n* 11 = The connection timed out.",
                          "enum": [
                            1,
                            2,
                            3,
                            4,
                            7,
                            8,
                            9,
                            10,
                            11
                          ]
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"webinar.rtms_stopped\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"webinar_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"stop_reason\": 1\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "webinar.rtms_started": {
      "post": {
        "tags": [
          "webinar"
        ],
        "operationId": "webinar.rtms_started",
        "requestBody": {
          "description": "# webinar.rtms_started\nThe **RTMS started in Webinar** event is triggered when a webinar host or a webinar panelist starts a real-time stream action.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS started in Webinar** subscription enabled under the **RTMS** event.\n  * The initiator must be a user of your [webhook-enabled](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks) app.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_started`,`rtms:read:rtms_started:admin`\n\n**Event type**: `webinar.rtms_started`\n",
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
                    "description": "Information about the webinar RTMS event.",
                    "required": [
                      "webinar_uuid",
                      "operator_id",
                      "rtms_stream_id",
                      "server_urls"
                    ],
                    "properties": {
                      "webinar_uuid": {
                        "type": "string",
                        "description": "The webinar's universally unique identifier (UUID). Each webinar instance generates a webinar UUID."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The ID of the user who is triggered this rtms event.",
                        "example": "30R7kT7bTIKSNUFEuH_Qlg"
                      },
                      "rtms_stream_id": {
                        "type": "string",
                        "description": "The unique ID of one of streams of the webinar. Each webinar can has multiple streams."
                      },
                      "server_urls": {
                        "type": "string",
                        "description": "The RTMS server urls for the app to connect with. Multiple URLs for different media protocols will be separated by commas."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"webinar.rtms_started\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"webinar_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"operator_id\": \"30R7kT7bTIKSNUFEuH_Qlg\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"server_urls\": \"wss://127.0.0.1:443\"\n  }\n}"
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
                      "description": "Information about the webinar RTMS event.",
                      "required": [
                        "webinar_uuid",
                        "operator_id",
                        "rtms_stream_id",
                        "server_urls"
                      ],
                      "properties": {
                        "webinar_uuid": {
                          "type": "string",
                          "description": "The webinar's universally unique identifier (UUID). Each webinar instance generates a webinar UUID."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The ID of the user who is triggered this rtms event.",
                          "example": "30R7kT7bTIKSNUFEuH_Qlg"
                        },
                        "rtms_stream_id": {
                          "type": "string",
                          "description": "The unique ID of one of streams of the webinar. Each webinar can has multiple streams."
                        },
                        "server_urls": {
                          "type": "string",
                          "description": "The RTMS server urls for the app to connect with. Multiple URLs for different media protocols will be separated by commas."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"webinar.rtms_started\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"webinar_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"operator_id\": \"30R7kT7bTIKSNUFEuH_Qlg\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"server_urls\": \"wss://127.0.0.1:443\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "webinar.rtms_interrupted": {
      "post": {
        "tags": [
          "webinar"
        ],
        "operationId": "webinar.rtms_interrupted",
        "requestBody": {
          "description": "# webinar.rtms_interrupted\nThe **RTMS interrupted in Webinar** event is triggered when an ongoing webinar stream gets disrupted due to a broken signaling socket connection.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS interrupted in Webinar** subscription enabled under the **RTMS** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_interrupted`,`rtms:read:rtms_interrupted:admin`\n\n**Event type**: `webinar.rtms_interrupted`\n",
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
                    "description": "Information about the webinar RTMS event.",
                    "required": [
                      "webinar_uuid",
                      "rtms_stream_id"
                    ],
                    "properties": {
                      "webinar_uuid": {
                        "type": "string",
                        "description": "The webinar's universally unique identifier (UUID). Each webinar instance generates a webinar UUID."
                      },
                      "rtms_stream_id": {
                        "type": "string",
                        "description": "The unique ID of one of streams of the webinar. Each webinar can has multiple streams."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"webinar.rtms_interrupted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"webinar_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\"\n  }\n}"
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
                      "description": "Information about the webinar RTMS event.",
                      "required": [
                        "webinar_uuid",
                        "rtms_stream_id"
                      ],
                      "properties": {
                        "webinar_uuid": {
                          "type": "string",
                          "description": "The webinar's universally unique identifier (UUID). Each webinar instance generates a webinar UUID."
                        },
                        "rtms_stream_id": {
                          "type": "string",
                          "description": "The unique ID of one of streams of the webinar. Each webinar can has multiple streams."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"webinar.rtms_interrupted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"webinar_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "session.rtms_stopped": {
      "post": {
        "tags": [
          "session"
        ],
        "operationId": "session.rtms_stopped",
        "requestBody": {
          "description": "# session.rtms_stopped\nThe **RTMS Stopped** event is triggered when the session host stops a realtime media stream.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS Stopped** subscription enabled under the **RTMS** event.\n\n**Event type**: `session.rtms_stopped`\n",
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
                    "description": "A timestamp of when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "description": "Information about the session rtms event.",
                    "required": [
                      "session_id",
                      "session_key",
                      "rtms_stream_id",
                      "stop_reason"
                    ],
                    "properties": {
                      "session_id": {
                        "type": "string",
                        "description": "The session's universally unique identifier (UUID). Each session instance generates a session UUID."
                      },
                      "session_key": {
                        "type": "string",
                        "description": "The Video SDK custom session ID."
                      },
                      "rtms_stream_id": {
                        "type": "string",
                        "description": "The unique ID of one of streams of the session. \n **Note**: A session can have multiple streams."
                      },
                      "stop_reason": {
                        "type": "integer",
                        "format": "int",
                        "description": "The reason code of the stream stopped.\n* 1 = Triggered by the session host. \n* 2 = Triggered by the app user. \n* 3 = App user left session. \n* 4 = App user ejected by the session host. \n* 5 = The app is disabled by the session host. \n* 6 = The session is ended. \n* 7 = The stream is canceled. \n* 8 = The stream was revoked. \n* 9 = All apps are disabled. \n* 10 = Server internal exception. \n* 11 = The connection timed out.",
                        "enum": [
                          1,
                          2,
                          3,
                          4,
                          7,
                          8,
                          9,
                          10,
                          11
                        ]
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"session.rtms_stopped\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n    \"session_key\": \"ABC36jaBI145\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"stop_reason\": 1\n  }\n}"
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
                      "description": "A timestamp of when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "description": "Information about the session rtms event.",
                      "required": [
                        "session_id",
                        "session_key",
                        "rtms_stream_id",
                        "stop_reason"
                      ],
                      "properties": {
                        "session_id": {
                          "type": "string",
                          "description": "The session's universally unique identifier (UUID). Each session instance generates a session UUID."
                        },
                        "session_key": {
                          "type": "string",
                          "description": "The Video SDK custom session ID."
                        },
                        "rtms_stream_id": {
                          "type": "string",
                          "description": "The unique ID of one of streams of the session. \n **Note**: A session can have multiple streams."
                        },
                        "stop_reason": {
                          "type": "integer",
                          "format": "int",
                          "description": "The reason code of the stream stopped.\n* 1 = Triggered by the session host. \n* 2 = Triggered by the app user. \n* 3 = App user left session. \n* 4 = App user ejected by the session host. \n* 5 = The app is disabled by the session host. \n* 6 = The session is ended. \n* 7 = The stream is canceled. \n* 8 = The stream was revoked. \n* 9 = All apps are disabled. \n* 10 = Server internal exception. \n* 11 = The connection timed out.",
                          "enum": [
                            1,
                            2,
                            3,
                            4,
                            7,
                            8,
                            9,
                            10,
                            11
                          ]
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"session.rtms_stopped\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n    \"session_key\": \"ABC36jaBI145\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"stop_reason\": 1\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "session.rtms_started": {
      "post": {
        "tags": [
          "session"
        ],
        "operationId": "session.rtms_started",
        "requestBody": {
          "description": "# session.rtms_started\nThe **RTMS Started** event is triggered when a session host starts a real-time stream.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS Started** subscription enabled under the **RTMS** event.\n  * The initiator must be a user of your [webhook-enabled](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks) app.\n\n**Event type**: `session.rtms_started`\n",
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
                    "description": "A timestamp of when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "description": "Information about the session rtms event.",
                    "required": [
                      "session_id",
                      "session_key",
                      "rtms_stream_id",
                      "server_urls"
                    ],
                    "properties": {
                      "session_id": {
                        "type": "string",
                        "description": "Unique session identifier. Each instance of the session will have its own session_id."
                      },
                      "session_key": {
                        "type": "string",
                        "description": "The Video SDK custom session ID."
                      },
                      "rtms_stream_id": {
                        "type": "string",
                        "description": "The unique ID of one of streams of the session. \n **Note**: A session can have multiple streams."
                      },
                      "server_urls": {
                        "type": "string",
                        "description": "The RTMS server urls for an app to connect with. Multiple URLs for different protocols will be separated by commas."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"session.rtms_started\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n    \"session_key\": \"ABC36jaBI145\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"server_urls\": \"wss://127.0.0.1:443\"\n  }\n}"
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
                      "description": "A timestamp of when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "description": "Information about the session rtms event.",
                      "required": [
                        "session_id",
                        "session_key",
                        "rtms_stream_id",
                        "server_urls"
                      ],
                      "properties": {
                        "session_id": {
                          "type": "string",
                          "description": "Unique session identifier. Each instance of the session will have its own session_id."
                        },
                        "session_key": {
                          "type": "string",
                          "description": "The Video SDK custom session ID."
                        },
                        "rtms_stream_id": {
                          "type": "string",
                          "description": "The unique ID of one of streams of the session. \n **Note**: A session can have multiple streams."
                        },
                        "server_urls": {
                          "type": "string",
                          "description": "The RTMS server urls for an app to connect with. Multiple URLs for different protocols will be separated by commas."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"session.rtms_started\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n    \"session_key\": \"ABC36jaBI145\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"server_urls\": \"wss://127.0.0.1:443\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "rtms.concurrency_limited": {
      "post": {
        "tags": [
          "rtms"
        ],
        "operationId": "rtms.concurrency_limited",
        "requestBody": {
          "description": "# rtms.concurrency_limited\nThe **RTMS Concurrency Limited** event is triggered when an app reaches the predetermined limit of concurrent streams for the app owner's account. The default is 2000 concurrent streams.\n\n\nThis event notifies the app owner if their usage exceeds the number of available concurrent streams.\n\n\n\nWhen the concurrency limit is reached, no new streams can be initiated.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS Concurrency Limited** subscription enabled under the **RTMS** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_concurrency_limited`,`rtms:read:rtms_concurrency_limited:admin`\n\n**Event type**: `rtms.concurrency_limited`\n",
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
                    "description": "A timestamp of when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "description": "Information about the meeting rtms event.",
                    "required": [
                      "operator_id",
                      "concurrency_number",
                      "id_field"
                    ],
                    "properties": {
                      "operator_id": {
                        "type": "string",
                        "description": "The ID of the user who triggered this RTMS event."
                      },
                      "concurrency_number": {
                        "type": "integer",
                        "description": "The number of concurrent streams."
                      },
                      "id_field": {
                        "oneOf": [
                          {
                            "properties": {
                              "meeting_uuid": {
                                "type": "string",
                                "description": "The meeting's universally unique identifier (UUID)."
                              }
                            },
                            "required": [
                              "meeting_uuid"
                            ]
                          },
                          {
                            "properties": {
                              "webinar_uuid": {
                                "type": "string",
                                "description": "The webinar's universally unique identifier (UUID)."
                              }
                            },
                            "required": [
                              "webinar_uuid"
                            ]
                          },
                          {
                            "properties": {
                              "session_id": {
                                "type": "string",
                                "description": "The session's universally unique identifier (UUID)."
                              }
                            },
                            "required": [
                              "session_id"
                            ]
                          }
                        ]
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"rtms.concurrency_limited\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"operator_id\": \"30R7kT7bTIKSNUFEuH_Qlg\",\n    \"concurrency_number\": 2001,\n    \"id_field\": {\n      \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\"\n    }\n  }\n}"
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
                      "description": "A timestamp of when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "description": "Information about the meeting rtms event.",
                      "required": [
                        "operator_id",
                        "concurrency_number",
                        "id_field"
                      ],
                      "properties": {
                        "operator_id": {
                          "type": "string",
                          "description": "The ID of the user who triggered this RTMS event."
                        },
                        "concurrency_number": {
                          "type": "integer",
                          "description": "The number of concurrent streams."
                        },
                        "id_field": {
                          "oneOf": [
                            {
                              "properties": {
                                "meeting_uuid": {
                                  "type": "string",
                                  "description": "The meeting's universally unique identifier (UUID)."
                                }
                              },
                              "required": [
                                "meeting_uuid"
                              ]
                            },
                            {
                              "properties": {
                                "webinar_uuid": {
                                  "type": "string",
                                  "description": "The webinar's universally unique identifier (UUID)."
                                }
                              },
                              "required": [
                                "webinar_uuid"
                              ]
                            },
                            {
                              "properties": {
                                "session_id": {
                                  "type": "string",
                                  "description": "The session's universally unique identifier (UUID)."
                                }
                              },
                              "required": [
                                "session_id"
                              ]
                            }
                          ]
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"rtms.concurrency_limited\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"operator_id\": \"30R7kT7bTIKSNUFEuH_Qlg\",\n    \"concurrency_number\": 2001,\n    \"id_field\": {\n      \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\"\n    }\n  }\n}"
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
    "title": "RTMS",
    "description": "RTMS is a data pipeline that gives your app access to live audio, video, and transcript data from Zoom Meetings.",
    "version": "1.0.0"
  },
  "webhooks": {
    "session.rtms_interrupted": {
      "post": {
        "tags": [
          "session"
        ],
        "operationId": "session.rtms_interrupted",
        "requestBody": {
          "description": "# session.rtms_interrupted\nThe **RTMS Interrupted** event is triggered when an ongoing session stream gets disrupted due to a broken signaling socket connection.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS Interrupted** subscription enabled under the **RTMS** event.\n\n**Event type**: `session.rtms_interrupted`\n",
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
                    "description": "A timestamp of when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "description": "Information about the session rtms event.",
                    "required": [
                      "session_id",
                      "session_key",
                      "rtms_stream_id"
                    ],
                    "properties": {
                      "session_id": {
                        "type": "string",
                        "description": "The session's universally unique identifier (UUID). Each session instance generates a meeting UUID."
                      },
                      "session_key": {
                        "type": "string",
                        "description": "The Video SDK custom session ID."
                      },
                      "rtms_stream_id": {
                        "type": "string",
                        "description": "The unique ID of one of streams of the session.\n **Note**: A session can have multiple streams."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"session.rtms_interrupted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n    \"session_key\": \"ABC36jaBI145\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\"\n  }\n}"
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
                      "description": "A timestamp of when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "description": "Information about the session rtms event.",
                      "required": [
                        "session_id",
                        "session_key",
                        "rtms_stream_id"
                      ],
                      "properties": {
                        "session_id": {
                          "type": "string",
                          "description": "The session's universally unique identifier (UUID). Each session instance generates a meeting UUID."
                        },
                        "session_key": {
                          "type": "string",
                          "description": "The Video SDK custom session ID."
                        },
                        "rtms_stream_id": {
                          "type": "string",
                          "description": "The unique ID of one of streams of the session.\n **Note**: A session can have multiple streams."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"session.rtms_interrupted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n    \"session_key\": \"ABC36jaBI145\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "meeting.rtms_interrupted": {
      "post": {
        "tags": [
          "meeting"
        ],
        "operationId": "meeting.rtms_interrupted",
        "requestBody": {
          "description": "# meeting.rtms_interrupted\nThe **RTMS interrupted in Meeting** event is triggered when an ongoing meeting stream gets disrupted due to a broken signaling socket connection.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS interrupted in Meeting** subscription enabled under the **RTMS** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_interrupted`,`rtms:read:rtms_interrupted:admin`\n\n**Event type**: `meeting.rtms_interrupted`\n",
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
                    "description": "Information about the meeting rtms event.",
                    "required": [
                      "meeting_uuid",
                      "rtms_stream_id"
                    ],
                    "properties": {
                      "meeting_uuid": {
                        "type": "string",
                        "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID."
                      },
                      "rtms_stream_id": {
                        "type": "string",
                        "description": "The unique ID of one of streams of the meeting. Each meeting can has multiple streams."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"meeting.rtms_interrupted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\"\n  }\n}"
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
                      "description": "Information about the meeting rtms event.",
                      "required": [
                        "meeting_uuid",
                        "rtms_stream_id"
                      ],
                      "properties": {
                        "meeting_uuid": {
                          "type": "string",
                          "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID."
                        },
                        "rtms_stream_id": {
                          "type": "string",
                          "description": "The unique ID of one of streams of the meeting. Each meeting can has multiple streams."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"meeting.rtms_interrupted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "meeting.rtms_stopped": {
      "post": {
        "tags": [
          "meeting"
        ],
        "operationId": "meeting.rtms_stopped",
        "requestBody": {
          "description": "# meeting.rtms_stopped\nThe **RTMS stopped in Meeting** event is triggered when the meeting host or the initiator stops a real-time stream.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS stopped in Meeting** subscription enabled under the **RTMS** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_stopped`,`rtms:read:rtms_stopped:admin`\n\n**Event type**: `meeting.rtms_stopped`\n",
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
                    "description": "Information about the meeting rtms event.",
                    "required": [
                      "meeting_uuid",
                      "rtms_stream_id",
                      "stop_reason"
                    ],
                    "properties": {
                      "meeting_uuid": {
                        "type": "string",
                        "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID."
                      },
                      "rtms_stream_id": {
                        "type": "string",
                        "description": "The unique ID of one of streams of the meeting. Each meeting can has multiple streams."
                      },
                      "stop_reason": {
                        "type": "integer",
                        "format": "int",
                        "description": "The reason code of the stream stopped.\n* 1 \u2014 Triggered by the meeting host. \n* 2 \u2014 Triggered by the app user. \n* 3 \u2014 App user left meeting. \n* 4 \u2014 App user ejected by the meeting host. \n* 5 \u2014 The app is disabled by the meeting host. \n* 6 \u2014 The meeting is ended. \n* 7 \u2014 The stream is canceled. \n* 8 \u2014 The stream revoked. \n* 9 \u2014 All apps are disabled. \n* 10 \u2014 Server internal exception. \n* 11 \u2014 The connection timed out.",
                        "enum": [
                          1,
                          2,
                          3,
                          4,
                          7,
                          8,
                          9,
                          10,
                          11
                        ]
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"meeting.rtms_stopped\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"stop_reason\": 1\n  }\n}"
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
                      "description": "Information about the meeting rtms event.",
                      "required": [
                        "meeting_uuid",
                        "rtms_stream_id",
                        "stop_reason"
                      ],
                      "properties": {
                        "meeting_uuid": {
                          "type": "string",
                          "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID."
                        },
                        "rtms_stream_id": {
                          "type": "string",
                          "description": "The unique ID of one of streams of the meeting. Each meeting can has multiple streams."
                        },
                        "stop_reason": {
                          "type": "integer",
                          "format": "int",
                          "description": "The reason code of the stream stopped.\n* 1 \u2014 Triggered by the meeting host. \n* 2 \u2014 Triggered by the app user. \n* 3 \u2014 App user left meeting. \n* 4 \u2014 App user ejected by the meeting host. \n* 5 \u2014 The app is disabled by the meeting host. \n* 6 \u2014 The meeting is ended. \n* 7 \u2014 The stream is canceled. \n* 8 \u2014 The stream revoked. \n* 9 \u2014 All apps are disabled. \n* 10 \u2014 Server internal exception. \n* 11 \u2014 The connection timed out.",
                          "enum": [
                            1,
                            2,
                            3,
                            4,
                            7,
                            8,
                            9,
                            10,
                            11
                          ]
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"meeting.rtms_stopped\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"stop_reason\": 1\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "rtms.concurrency_near_limit": {
      "post": {
        "tags": [
          "rtms"
        ],
        "operationId": "rtms.concurrency_near_limit",
        "requestBody": {
          "description": "# rtms.concurrency_near_limit\nThe **RTMS Concurrency Near Limit** event is triggered when the app is nearing the predetermined number of concurrent streams. The default limit is 2000 concurrent streams.\n\nThis event periodically notifies the app owner when the number of concurrent streams exceeds 80% of the available concurrent streams.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**\n\t* The **RTMS Concurrency Near Limit** subscription enabled under the **RTMS** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_concurrency_near_limit`,`rtms:read:rtms_concurrency_near_limit:admin`\n\n**Event type**: `rtms.concurrency_near_limit`\n",
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
                    "description": "A timestamp of when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "description": "Information about the meeting RTMS event.",
                    "required": [
                      "concurrency_limit",
                      "concurrency_number",
                      "percentage"
                    ],
                    "properties": {
                      "concurrency_limit": {
                        "type": "integer",
                        "description": "The number of the predetermined available concurrent streams."
                      },
                      "concurrency_number": {
                        "type": "integer",
                        "description": "The number of concurrent streams."
                      },
                      "percentage": {
                        "type": "integer",
                        "description": "The number of concurrent streams as a percentage of the available concurrent streams."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"rtms.concurrency_near_limit\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"concurrency_limit\": 2000,\n    \"concurrency_number\": 1601,\n    \"percentage\": 80\n  }\n}"
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
                      "description": "A timestamp of when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "description": "Information about the meeting RTMS event.",
                      "required": [
                        "concurrency_limit",
                        "concurrency_number",
                        "percentage"
                      ],
                      "properties": {
                        "concurrency_limit": {
                          "type": "integer",
                          "description": "The number of the predetermined available concurrent streams."
                        },
                        "concurrency_number": {
                          "type": "integer",
                          "description": "The number of concurrent streams."
                        },
                        "percentage": {
                          "type": "integer",
                          "description": "The number of concurrent streams as a percentage of the available concurrent streams."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"rtms.concurrency_near_limit\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"concurrency_limit\": 2000,\n    \"concurrency_number\": 1601,\n    \"percentage\": 80\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "meeting.rtms_started": {
      "post": {
        "tags": [
          "meeting"
        ],
        "operationId": "meeting.rtms_started",
        "requestBody": {
          "description": "# meeting.rtms_started\nThe **RTMS started in Meeting** event is triggered when a meeting host or a meeting participant starts a real-time stream action.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS started in Meeting** subscription enabled under the **RTMS** event.\n  * The initiator must be a user of your [webhook-enabled](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks) app.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_started`,`rtms:read:rtms_started:admin`\n\n**Event type**: `meeting.rtms_started`\n",
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
                    "description": "Information about the meeting rtms event.",
                    "required": [
                      "meeting_uuid",
                      "operator_id",
                      "rtms_stream_id",
                      "server_urls"
                    ],
                    "properties": {
                      "meeting_uuid": {
                        "type": "string",
                        "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The ID of the user who is triggered this rtms event.",
                        "example": "30R7kT7bTIKSNUFEuH_Qlg"
                      },
                      "rtms_stream_id": {
                        "type": "string",
                        "description": "The unique ID of one of streams of the meeting. Each meeting can has multiple streams."
                      },
                      "server_urls": {
                        "type": "string",
                        "description": "The RTMS server urls for app to connect with. Multiple URLs for different protocols will be separated by commas."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"meeting.rtms_started\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"operator_id\": \"30R7kT7bTIKSNUFEuH_Qlg\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"server_urls\": \"wss://127.0.0.1:443\"\n  }\n}"
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
                      "description": "Information about the meeting rtms event.",
                      "required": [
                        "meeting_uuid",
                        "operator_id",
                        "rtms_stream_id",
                        "server_urls"
                      ],
                      "properties": {
                        "meeting_uuid": {
                          "type": "string",
                          "description": "The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The ID of the user who is triggered this rtms event.",
                          "example": "30R7kT7bTIKSNUFEuH_Qlg"
                        },
                        "rtms_stream_id": {
                          "type": "string",
                          "description": "The unique ID of one of streams of the meeting. Each meeting can has multiple streams."
                        },
                        "server_urls": {
                          "type": "string",
                          "description": "The RTMS server urls for app to connect with. Multiple URLs for different protocols will be separated by commas."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"meeting.rtms_started\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"operator_id\": \"30R7kT7bTIKSNUFEuH_Qlg\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"server_urls\": \"wss://127.0.0.1:443\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "webinar.rtms_stopped": {
      "post": {
        "tags": [
          "webinar"
        ],
        "operationId": "webinar.rtms_stopped",
        "requestBody": {
          "description": "# webinar.rtms_stopped\nThe **RTMS stopped in Webinar** event is triggered when the webinar host or the initiator stops a real-time stream.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS stopped in Webinar** subscription enabled under the **RTMS** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_stopped`,`rtms:read:rtms_stopped:admin`\n\n**Event type**: `webinar.rtms_stopped`\n",
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
                    "description": "Information about the webinar RTMS event.",
                    "required": [
                      "webinar_uuid",
                      "rtms_stream_id",
                      "stop_reason"
                    ],
                    "properties": {
                      "webinar_uuid": {
                        "type": "string",
                        "description": "The webinar's universally unique identifier (UUID). Each webinar instance generates a webinar UUID."
                      },
                      "rtms_stream_id": {
                        "type": "string",
                        "description": "The unique ID of one of streams of the webinar. Each webinar can has multiple streams."
                      },
                      "stop_reason": {
                        "type": "integer",
                        "format": "int",
                        "description": "The reason code of why the stream stopped.\n* 1 = Triggered by the webinar host. \n* 2 = Triggered by the app user. \n* 3 = App user left webinar. \n* 4 = App user ejected by the webinar host. \n* 5 = The app is disabled by the webinar host. \n* 6 = The webinar is ended. \n* 7 = The stream is canceled. \n* 8 = The stream revoked. \n* 9 = All apps are disabled. \n* 10 = Server internal exception. \n* 11 = The connection timed out.",
                        "enum": [
                          1,
                          2,
                          3,
                          4,
                          7,
                          8,
                          9,
                          10,
                          11
                        ]
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"webinar.rtms_stopped\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"webinar_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"stop_reason\": 1\n  }\n}"
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
                      "description": "Information about the webinar RTMS event.",
                      "required": [
                        "webinar_uuid",
                        "rtms_stream_id",
                        "stop_reason"
                      ],
                      "properties": {
                        "webinar_uuid": {
                          "type": "string",
                          "description": "The webinar's universally unique identifier (UUID). Each webinar instance generates a webinar UUID."
                        },
                        "rtms_stream_id": {
                          "type": "string",
                          "description": "The unique ID of one of streams of the webinar. Each webinar can has multiple streams."
                        },
                        "stop_reason": {
                          "type": "integer",
                          "format": "int",
                          "description": "The reason code of why the stream stopped.\n* 1 = Triggered by the webinar host. \n* 2 = Triggered by the app user. \n* 3 = App user left webinar. \n* 4 = App user ejected by the webinar host. \n* 5 = The app is disabled by the webinar host. \n* 6 = The webinar is ended. \n* 7 = The stream is canceled. \n* 8 = The stream revoked. \n* 9 = All apps are disabled. \n* 10 = Server internal exception. \n* 11 = The connection timed out.",
                          "enum": [
                            1,
                            2,
                            3,
                            4,
                            7,
                            8,
                            9,
                            10,
                            11
                          ]
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"webinar.rtms_stopped\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"webinar_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"stop_reason\": 1\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "webinar.rtms_started": {
      "post": {
        "tags": [
          "webinar"
        ],
        "operationId": "webinar.rtms_started",
        "requestBody": {
          "description": "# webinar.rtms_started\nThe **RTMS started in Webinar** event is triggered when a webinar host or a webinar panelist starts a real-time stream action.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS started in Webinar** subscription enabled under the **RTMS** event.\n  * The initiator must be a user of your [webhook-enabled](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks) app.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_started`,`rtms:read:rtms_started:admin`\n\n**Event type**: `webinar.rtms_started`\n",
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
                    "description": "Information about the webinar RTMS event.",
                    "required": [
                      "webinar_uuid",
                      "operator_id",
                      "rtms_stream_id",
                      "server_urls"
                    ],
                    "properties": {
                      "webinar_uuid": {
                        "type": "string",
                        "description": "The webinar's universally unique identifier (UUID). Each webinar instance generates a webinar UUID."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The ID of the user who is triggered this rtms event.",
                        "example": "30R7kT7bTIKSNUFEuH_Qlg"
                      },
                      "rtms_stream_id": {
                        "type": "string",
                        "description": "The unique ID of one of streams of the webinar. Each webinar can has multiple streams."
                      },
                      "server_urls": {
                        "type": "string",
                        "description": "The RTMS server urls for the app to connect with. Multiple URLs for different media protocols will be separated by commas."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"webinar.rtms_started\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"webinar_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"operator_id\": \"30R7kT7bTIKSNUFEuH_Qlg\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"server_urls\": \"wss://127.0.0.1:443\"\n  }\n}"
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
                      "description": "Information about the webinar RTMS event.",
                      "required": [
                        "webinar_uuid",
                        "operator_id",
                        "rtms_stream_id",
                        "server_urls"
                      ],
                      "properties": {
                        "webinar_uuid": {
                          "type": "string",
                          "description": "The webinar's universally unique identifier (UUID). Each webinar instance generates a webinar UUID."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The ID of the user who is triggered this rtms event.",
                          "example": "30R7kT7bTIKSNUFEuH_Qlg"
                        },
                        "rtms_stream_id": {
                          "type": "string",
                          "description": "The unique ID of one of streams of the webinar. Each webinar can has multiple streams."
                        },
                        "server_urls": {
                          "type": "string",
                          "description": "The RTMS server urls for the app to connect with. Multiple URLs for different media protocols will be separated by commas."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"webinar.rtms_started\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"webinar_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"operator_id\": \"30R7kT7bTIKSNUFEuH_Qlg\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"server_urls\": \"wss://127.0.0.1:443\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "webinar.rtms_interrupted": {
      "post": {
        "tags": [
          "webinar"
        ],
        "operationId": "webinar.rtms_interrupted",
        "requestBody": {
          "description": "# webinar.rtms_interrupted\nThe **RTMS interrupted in Webinar** event is triggered when an ongoing webinar stream gets disrupted due to a broken signaling socket connection.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS interrupted in Webinar** subscription enabled under the **RTMS** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_interrupted`,`rtms:read:rtms_interrupted:admin`\n\n**Event type**: `webinar.rtms_interrupted`\n",
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
                    "description": "Information about the webinar RTMS event.",
                    "required": [
                      "webinar_uuid",
                      "rtms_stream_id"
                    ],
                    "properties": {
                      "webinar_uuid": {
                        "type": "string",
                        "description": "The webinar's universally unique identifier (UUID). Each webinar instance generates a webinar UUID."
                      },
                      "rtms_stream_id": {
                        "type": "string",
                        "description": "The unique ID of one of streams of the webinar. Each webinar can has multiple streams."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"webinar.rtms_interrupted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"webinar_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\"\n  }\n}"
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
                      "description": "Information about the webinar RTMS event.",
                      "required": [
                        "webinar_uuid",
                        "rtms_stream_id"
                      ],
                      "properties": {
                        "webinar_uuid": {
                          "type": "string",
                          "description": "The webinar's universally unique identifier (UUID). Each webinar instance generates a webinar UUID."
                        },
                        "rtms_stream_id": {
                          "type": "string",
                          "description": "The unique ID of one of streams of the webinar. Each webinar can has multiple streams."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"webinar.rtms_interrupted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"webinar_uuid\": \"4444AAAiAAAAAiAiAiiAii==\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "session.rtms_stopped": {
      "post": {
        "tags": [
          "session"
        ],
        "operationId": "session.rtms_stopped",
        "requestBody": {
          "description": "# session.rtms_stopped\nThe **RTMS Stopped** event is triggered when the session host stops a realtime media stream.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS Stopped** subscription enabled under the **RTMS** event.\n\n**Event type**: `session.rtms_stopped`\n",
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
                    "description": "A timestamp of when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "description": "Information about the session rtms event.",
                    "required": [
                      "session_id",
                      "session_key",
                      "rtms_stream_id",
                      "stop_reason"
                    ],
                    "properties": {
                      "session_id": {
                        "type": "string",
                        "description": "The session's universally unique identifier (UUID). Each session instance generates a session UUID."
                      },
                      "session_key": {
                        "type": "string",
                        "description": "The Video SDK custom session ID."
                      },
                      "rtms_stream_id": {
                        "type": "string",
                        "description": "The unique ID of one of streams of the session. \n **Note**: A session can have multiple streams."
                      },
                      "stop_reason": {
                        "type": "integer",
                        "format": "int",
                        "description": "The reason code of the stream stopped.\n* 1 = Triggered by the session host. \n* 2 = Triggered by the app user. \n* 3 = App user left session. \n* 4 = App user ejected by the session host. \n* 5 = The app is disabled by the session host. \n* 6 = The session is ended. \n* 7 = The stream is canceled. \n* 8 = The stream was revoked. \n* 9 = All apps are disabled. \n* 10 = Server internal exception. \n* 11 = The connection timed out.",
                        "enum": [
                          1,
                          2,
                          3,
                          4,
                          7,
                          8,
                          9,
                          10,
                          11
                        ]
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"session.rtms_stopped\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n    \"session_key\": \"ABC36jaBI145\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"stop_reason\": 1\n  }\n}"
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
                      "description": "A timestamp of when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "description": "Information about the session rtms event.",
                      "required": [
                        "session_id",
                        "session_key",
                        "rtms_stream_id",
                        "stop_reason"
                      ],
                      "properties": {
                        "session_id": {
                          "type": "string",
                          "description": "The session's universally unique identifier (UUID). Each session instance generates a session UUID."
                        },
                        "session_key": {
                          "type": "string",
                          "description": "The Video SDK custom session ID."
                        },
                        "rtms_stream_id": {
                          "type": "string",
                          "description": "The unique ID of one of streams of the session. \n **Note**: A session can have multiple streams."
                        },
                        "stop_reason": {
                          "type": "integer",
                          "format": "int",
                          "description": "The reason code of the stream stopped.\n* 1 = Triggered by the session host. \n* 2 = Triggered by the app user. \n* 3 = App user left session. \n* 4 = App user ejected by the session host. \n* 5 = The app is disabled by the session host. \n* 6 = The session is ended. \n* 7 = The stream is canceled. \n* 8 = The stream was revoked. \n* 9 = All apps are disabled. \n* 10 = Server internal exception. \n* 11 = The connection timed out.",
                          "enum": [
                            1,
                            2,
                            3,
                            4,
                            7,
                            8,
                            9,
                            10,
                            11
                          ]
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"session.rtms_stopped\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n    \"session_key\": \"ABC36jaBI145\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"stop_reason\": 1\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "session.rtms_started": {
      "post": {
        "tags": [
          "session"
        ],
        "operationId": "session.rtms_started",
        "requestBody": {
          "description": "# session.rtms_started\nThe **RTMS Started** event is triggered when a session host starts a real-time stream.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS Started** subscription enabled under the **RTMS** event.\n  * The initiator must be a user of your [webhook-enabled](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks) app.\n\n**Event type**: `session.rtms_started`\n",
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
                    "description": "A timestamp of when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "description": "Information about the session rtms event.",
                    "required": [
                      "session_id",
                      "session_key",
                      "rtms_stream_id",
                      "server_urls"
                    ],
                    "properties": {
                      "session_id": {
                        "type": "string",
                        "description": "Unique session identifier. Each instance of the session will have its own session_id."
                      },
                      "session_key": {
                        "type": "string",
                        "description": "The Video SDK custom session ID."
                      },
                      "rtms_stream_id": {
                        "type": "string",
                        "description": "The unique ID of one of streams of the session. \n **Note**: A session can have multiple streams."
                      },
                      "server_urls": {
                        "type": "string",
                        "description": "The RTMS server urls for an app to connect with. Multiple URLs for different protocols will be separated by commas."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"session.rtms_started\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n    \"session_key\": \"ABC36jaBI145\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"server_urls\": \"wss://127.0.0.1:443\"\n  }\n}"
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
                      "description": "A timestamp of when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "description": "Information about the session rtms event.",
                      "required": [
                        "session_id",
                        "session_key",
                        "rtms_stream_id",
                        "server_urls"
                      ],
                      "properties": {
                        "session_id": {
                          "type": "string",
                          "description": "Unique session identifier. Each instance of the session will have its own session_id."
                        },
                        "session_key": {
                          "type": "string",
                          "description": "The Video SDK custom session ID."
                        },
                        "rtms_stream_id": {
                          "type": "string",
                          "description": "The unique ID of one of streams of the session. \n **Note**: A session can have multiple streams."
                        },
                        "server_urls": {
                          "type": "string",
                          "description": "The RTMS server urls for an app to connect with. Multiple URLs for different protocols will be separated by commas."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"session.rtms_started\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"session_id\": \"4567UVWxYZABCdEfGhiJkl==\",\n    \"session_key\": \"ABC36jaBI145\",\n    \"rtms_stream_id\": \"609340fb2a7946909659956c8aa9250c\",\n    \"server_urls\": \"wss://127.0.0.1:443\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "rtms.concurrency_limited": {
      "post": {
        "tags": [
          "rtms"
        ],
        "operationId": "rtms.concurrency_limited",
        "requestBody": {
          "description": "# rtms.concurrency_limited\nThe **RTMS Concurrency Limited** event is triggered when an app reaches the predetermined limit of concurrent streams for the app owner's account. The default is 2000 concurrent streams.\n\n\nThis event notifies the app owner if their usage exceeds the number of available concurrent streams.\n\n\n\nWhen the concurrency limit is reached, no new streams can be initiated.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **RTMS Concurrency Limited** subscription enabled under the **RTMS** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_concurrency_limited`,`rtms:read:rtms_concurrency_limited:admin`\n\n**Event type**: `rtms.concurrency_limited`\n",
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
                    "description": "A timestamp of when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "description": "Information about the meeting rtms event.",
                    "required": [
                      "operator_id",
                      "concurrency_number",
                      "id_field"
                    ],
                    "properties": {
                      "operator_id": {
                        "type": "string",
                        "description": "The ID of the user who triggered this RTMS event."
                      },
                      "concurrency_number": {
                        "type": "integer",
                        "description": "The number of concurrent streams."
                      },
                      "id_field": {
                        "oneOf": [
                          {
                            "properties": {
                              "meeting_uuid": {
                                "type": "string",
                                "description": "The meeting's universally unique identifier (UUID)."
                              }
                            },
                            "required": [
                              "meeting_uuid"
                            ]
                          },
                          {
                            "properties": {
                              "webinar_uuid": {
                                "type": "string",
                                "description": "The webinar's universally unique identifier (UUID)."
                              }
                            },
                            "required": [
                              "webinar_uuid"
                            ]
                          },
                          {
                            "properties": {
                              "session_id": {
                                "type": "string",
                                "description": "The session's universally unique identifier (UUID)."
                              }
                            },
                            "required": [
                              "session_id"
                            ]
                          }
                        ]
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"rtms.concurrency_limited\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"operator_id\": \"30R7kT7bTIKSNUFEuH_Qlg\",\n    \"concurrency_number\": 2001,\n    \"id_field\": {\n      \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\"\n    }\n  }\n}"
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
                      "description": "A timestamp of when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "description": "Information about the meeting rtms event.",
                      "required": [
                        "operator_id",
                        "concurrency_number",
                        "id_field"
                      ],
                      "properties": {
                        "operator_id": {
                          "type": "string",
                          "description": "The ID of the user who triggered this RTMS event."
                        },
                        "concurrency_number": {
                          "type": "integer",
                          "description": "The number of concurrent streams."
                        },
                        "id_field": {
                          "oneOf": [
                            {
                              "properties": {
                                "meeting_uuid": {
                                  "type": "string",
                                  "description": "The meeting's universally unique identifier (UUID)."
                                }
                              },
                              "required": [
                                "meeting_uuid"
                              ]
                            },
                            {
                              "properties": {
                                "webinar_uuid": {
                                  "type": "string",
                                  "description": "The webinar's universally unique identifier (UUID)."
                                }
                              },
                              "required": [
                                "webinar_uuid"
                              ]
                            },
                            {
                              "properties": {
                                "session_id": {
                                  "type": "string",
                                  "description": "The session's universally unique identifier (UUID)."
                                }
                              },
                              "required": [
                                "session_id"
                              ]
                            }
                          ]
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"rtms.concurrency_limited\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"operator_id\": \"30R7kT7bTIKSNUFEuH_Qlg\",\n    \"concurrency_number\": 2001,\n    \"id_field\": {\n      \"meeting_uuid\": \"4444AAAiAAAAAiAiAiiAii==\"\n    }\n  }\n}"
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
      "rtms",
      "events"
    ]
  },
  "downloadPath": "/api-hub/rtms/events/webhooks.json",
  "apiOptions": {
    "events": true,
    "methods": false,
    "ma": false
  },
  "productMeta": {
    "specType": "adoc",
    "slug": "/docs/api/rest/reference/rtms/events",
    "title": "RTMS",
    "keywords": "zoom, webhooks, api, rtms",
    "params": {
      "productName": "rtms"
    },
    "description": "RTMS is a data pipeline that gives your app access to live audio, video, and transcript data from Zoom Meetings.",
    "zoapEndpoint": "webhookByProduct",
    "productDisplayTitle": "RTMS",
    "scalarSlug": "/docs/api/rtms/events",
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "svgname": "_rtms"
  },
  "frontmatter": {
    "title": "RTMS Webhooks",
    "keywords": "zoom, webhooks, api, rtms",
    "description": "RTMS is a data pipeline that gives your app access to live audio, video, and transcript data from Zoom Meetings.",
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
        "rtms",
        "events"
      ]
    },
    "downloadPath": "/api-hub/rtms/events/webhooks.json",
    "apiOptions": {
      "events": true,
      "methods": false,
      "ma": false
    },
    "productMeta": {
      "specType": "adoc",
      "slug": "/docs/api/rest/reference/rtms/events",
      "title": "RTMS",
      "keywords": "zoom, webhooks, api, rtms",
      "params": {
        "productName": "rtms"
      },
      "description": "RTMS is a data pipeline that gives your app access to live audio, video, and transcript data from Zoom Meetings.",
      "zoapEndpoint": "webhookByProduct",
      "productDisplayTitle": "RTMS",
      "scalarSlug": "/docs/api/rtms/events",
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "svgname": "_rtms"
    },
    "spec": "{\"openapi\":\"3.1.0\",\"info\":{\"title\":\"RTMS\",\"description\":\"RTMS is a data pipeline that gives your app access to live audio, video, and transcript data from Zoom Meetings.\",\"version\":\"1.0.0\"},\"paths\":{\"session.rtms_interrupted\":{\"post\":{\"tags\":[\"session\"],\"operationId\":\"session.rtms_interrupted\",\"requestBody\":{\"description\":\"# session.rtms_interrupted\\nThe **RTMS Interrupted** event is triggered when an ongoing session stream gets disrupted due to a broken signaling socket connection.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **RTMS Interrupted** subscription enabled under the **RTMS** event.\\n\\n**Event type**: `session.rtms_interrupted`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the session rtms event.\",\"required\":[\"session_id\",\"session_key\",\"rtms_stream_id\"],\"properties\":{\"session_id\":{\"type\":\"string\",\"description\":\"The session's universally unique identifier (UUID). Each session instance generates a meeting UUID.\"},\"session_key\":{\"type\":\"string\",\"description\":\"The Video SDK custom session ID.\"},\"rtms_stream_id\":{\"type\":\"string\",\"description\":\"The unique ID of one of streams of the session.\\n **Note**: A session can have multiple streams.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"session.rtms_interrupted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"session_id\\\": \\\"4567UVWxYZABCdEfGhiJkl==\\\",\\n    \\\"session_key\\\": \\\"ABC36jaBI145\\\",\\n    \\\"rtms_stream_id\\\": \\\"609340fb2a7946909659956c8aa9250c\\\"\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the session rtms event.\",\"required\":[\"session_id\",\"session_key\",\"rtms_stream_id\"],\"properties\":{\"session_id\":{\"type\":\"string\",\"description\":\"The session's universally unique identifier (UUID). Each session instance generates a meeting UUID.\"},\"session_key\":{\"type\":\"string\",\"description\":\"The Video SDK custom session ID.\"},\"rtms_stream_id\":{\"type\":\"string\",\"description\":\"The unique ID of one of streams of the session.\\n **Note**: A session can have multiple streams.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"session.rtms_interrupted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"session_id\\\": \\\"4567UVWxYZABCdEfGhiJkl==\\\",\\n    \\\"session_key\\\": \\\"ABC36jaBI145\\\",\\n    \\\"rtms_stream_id\\\": \\\"609340fb2a7946909659956c8aa9250c\\\"\\n  }\\n}\"}}}}}}}},\"meeting.rtms_interrupted\":{\"post\":{\"tags\":[\"meeting\"],\"operationId\":\"meeting.rtms_interrupted\",\"requestBody\":{\"description\":\"# meeting.rtms_interrupted\\nThe **RTMS interrupted in Meeting** event is triggered when an ongoing meeting stream gets disrupted due to a broken signaling socket connection.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **RTMS interrupted in Meeting** subscription enabled under the **RTMS** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_interrupted`,`rtms:read:rtms_interrupted:admin`\\n\\n**Event type**: `meeting.rtms_interrupted`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the meeting rtms event.\",\"required\":[\"meeting_uuid\",\"rtms_stream_id\"],\"properties\":{\"meeting_uuid\":{\"type\":\"string\",\"description\":\"The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID.\"},\"rtms_stream_id\":{\"type\":\"string\",\"description\":\"The unique ID of one of streams of the meeting. Each meeting can has multiple streams.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"meeting.rtms_interrupted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"meeting_uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\",\\n    \\\"rtms_stream_id\\\": \\\"609340fb2a7946909659956c8aa9250c\\\"\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the meeting rtms event.\",\"required\":[\"meeting_uuid\",\"rtms_stream_id\"],\"properties\":{\"meeting_uuid\":{\"type\":\"string\",\"description\":\"The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID.\"},\"rtms_stream_id\":{\"type\":\"string\",\"description\":\"The unique ID of one of streams of the meeting. Each meeting can has multiple streams.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"meeting.rtms_interrupted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"meeting_uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\",\\n    \\\"rtms_stream_id\\\": \\\"609340fb2a7946909659956c8aa9250c\\\"\\n  }\\n}\"}}}}}}}},\"meeting.rtms_stopped\":{\"post\":{\"tags\":[\"meeting\"],\"operationId\":\"meeting.rtms_stopped\",\"requestBody\":{\"description\":\"# meeting.rtms_stopped\\nThe **RTMS stopped in Meeting** event is triggered when the meeting host or the initiator stops a real-time stream.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **RTMS stopped in Meeting** subscription enabled under the **RTMS** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_stopped`,`rtms:read:rtms_stopped:admin`\\n\\n**Event type**: `meeting.rtms_stopped`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the meeting rtms event.\",\"required\":[\"meeting_uuid\",\"rtms_stream_id\",\"stop_reason\"],\"properties\":{\"meeting_uuid\":{\"type\":\"string\",\"description\":\"The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID.\"},\"rtms_stream_id\":{\"type\":\"string\",\"description\":\"The unique ID of one of streams of the meeting. Each meeting can has multiple streams.\"},\"stop_reason\":{\"type\":\"integer\",\"format\":\"int\",\"description\":\"The reason code of the stream stopped.\\n* 1 \u2014 Triggered by the meeting host. \\n* 2 \u2014 Triggered by the app user. \\n* 3 \u2014 App user left meeting. \\n* 4 \u2014 App user ejected by the meeting host. \\n* 5 \u2014 The app is disabled by the meeting host. \\n* 6 \u2014 The meeting is ended. \\n* 7 \u2014 The stream is canceled. \\n* 8 \u2014 The stream revoked. \\n* 9 \u2014 All apps are disabled. \\n* 10 \u2014 Server internal exception. \\n* 11 \u2014 The connection timed out.\",\"enum\":[1,2,3,4,7,8,9,10,11]}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"meeting.rtms_stopped\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"meeting_uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\",\\n    \\\"rtms_stream_id\\\": \\\"609340fb2a7946909659956c8aa9250c\\\",\\n    \\\"stop_reason\\\": 1\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the meeting rtms event.\",\"required\":[\"meeting_uuid\",\"rtms_stream_id\",\"stop_reason\"],\"properties\":{\"meeting_uuid\":{\"type\":\"string\",\"description\":\"The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID.\"},\"rtms_stream_id\":{\"type\":\"string\",\"description\":\"The unique ID of one of streams of the meeting. Each meeting can has multiple streams.\"},\"stop_reason\":{\"type\":\"integer\",\"format\":\"int\",\"description\":\"The reason code of the stream stopped.\\n* 1 \u2014 Triggered by the meeting host. \\n* 2 \u2014 Triggered by the app user. \\n* 3 \u2014 App user left meeting. \\n* 4 \u2014 App user ejected by the meeting host. \\n* 5 \u2014 The app is disabled by the meeting host. \\n* 6 \u2014 The meeting is ended. \\n* 7 \u2014 The stream is canceled. \\n* 8 \u2014 The stream revoked. \\n* 9 \u2014 All apps are disabled. \\n* 10 \u2014 Server internal exception. \\n* 11 \u2014 The connection timed out.\",\"enum\":[1,2,3,4,7,8,9,10,11]}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"meeting.rtms_stopped\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"meeting_uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\",\\n    \\\"rtms_stream_id\\\": \\\"609340fb2a7946909659956c8aa9250c\\\",\\n    \\\"stop_reason\\\": 1\\n  }\\n}\"}}}}}}}},\"rtms.concurrency_near_limit\":{\"post\":{\"tags\":[\"rtms\"],\"operationId\":\"rtms.concurrency_near_limit\",\"requestBody\":{\"description\":\"# rtms.concurrency_near_limit\\nThe **RTMS Concurrency Near Limit** event is triggered when the app is nearing the predetermined number of concurrent streams. The default limit is 2000 concurrent streams.\\n\\nThis event periodically notifies the app owner when the number of concurrent streams exceeds 80% of the available concurrent streams.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**\\n\\t* The **RTMS Concurrency Near Limit** subscription enabled under the **RTMS** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_concurrency_near_limit`,`rtms:read:rtms_concurrency_near_limit:admin`\\n\\n**Event type**: `rtms.concurrency_near_limit`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the meeting RTMS event.\",\"required\":[\"concurrency_limit\",\"concurrency_number\",\"percentage\"],\"properties\":{\"concurrency_limit\":{\"type\":\"integer\",\"description\":\"The number of the predetermined available concurrent streams.\"},\"concurrency_number\":{\"type\":\"integer\",\"description\":\"The number of concurrent streams.\"},\"percentage\":{\"type\":\"integer\",\"description\":\"The number of concurrent streams as a percentage of the available concurrent streams.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"rtms.concurrency_near_limit\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"concurrency_limit\\\": 2000,\\n    \\\"concurrency_number\\\": 1601,\\n    \\\"percentage\\\": 80\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the meeting RTMS event.\",\"required\":[\"concurrency_limit\",\"concurrency_number\",\"percentage\"],\"properties\":{\"concurrency_limit\":{\"type\":\"integer\",\"description\":\"The number of the predetermined available concurrent streams.\"},\"concurrency_number\":{\"type\":\"integer\",\"description\":\"The number of concurrent streams.\"},\"percentage\":{\"type\":\"integer\",\"description\":\"The number of concurrent streams as a percentage of the available concurrent streams.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"rtms.concurrency_near_limit\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"concurrency_limit\\\": 2000,\\n    \\\"concurrency_number\\\": 1601,\\n    \\\"percentage\\\": 80\\n  }\\n}\"}}}}}}}},\"meeting.rtms_started\":{\"post\":{\"tags\":[\"meeting\"],\"operationId\":\"meeting.rtms_started\",\"requestBody\":{\"description\":\"# meeting.rtms_started\\nThe **RTMS started in Meeting** event is triggered when a meeting host or a meeting participant starts a real-time stream action.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **RTMS started in Meeting** subscription enabled under the **RTMS** event.\\n  * The initiator must be a user of your [webhook-enabled](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks) app.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_started`,`rtms:read:rtms_started:admin`\\n\\n**Event type**: `meeting.rtms_started`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the meeting rtms event.\",\"required\":[\"meeting_uuid\",\"operator_id\",\"rtms_stream_id\",\"server_urls\"],\"properties\":{\"meeting_uuid\":{\"type\":\"string\",\"description\":\"The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The ID of the user who is triggered this rtms event.\",\"example\":\"30R7kT7bTIKSNUFEuH_Qlg\"},\"rtms_stream_id\":{\"type\":\"string\",\"description\":\"The unique ID of one of streams of the meeting. Each meeting can has multiple streams.\"},\"server_urls\":{\"type\":\"string\",\"description\":\"The RTMS server urls for app to connect with. Multiple URLs for different protocols will be separated by commas.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"meeting.rtms_started\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"meeting_uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\",\\n    \\\"operator_id\\\": \\\"30R7kT7bTIKSNUFEuH_Qlg\\\",\\n    \\\"rtms_stream_id\\\": \\\"609340fb2a7946909659956c8aa9250c\\\",\\n    \\\"server_urls\\\": \\\"wss://127.0.0.1:443\\\"\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the meeting rtms event.\",\"required\":[\"meeting_uuid\",\"operator_id\",\"rtms_stream_id\",\"server_urls\"],\"properties\":{\"meeting_uuid\":{\"type\":\"string\",\"description\":\"The meeting's universally unique identifier (UUID). Each meeting instance generates a meeting UUID.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The ID of the user who is triggered this rtms event.\",\"example\":\"30R7kT7bTIKSNUFEuH_Qlg\"},\"rtms_stream_id\":{\"type\":\"string\",\"description\":\"The unique ID of one of streams of the meeting. Each meeting can has multiple streams.\"},\"server_urls\":{\"type\":\"string\",\"description\":\"The RTMS server urls for app to connect with. Multiple URLs for different protocols will be separated by commas.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"meeting.rtms_started\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"meeting_uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\",\\n    \\\"operator_id\\\": \\\"30R7kT7bTIKSNUFEuH_Qlg\\\",\\n    \\\"rtms_stream_id\\\": \\\"609340fb2a7946909659956c8aa9250c\\\",\\n    \\\"server_urls\\\": \\\"wss://127.0.0.1:443\\\"\\n  }\\n}\"}}}}}}}},\"webinar.rtms_stopped\":{\"post\":{\"tags\":[\"webinar\"],\"operationId\":\"webinar.rtms_stopped\",\"requestBody\":{\"description\":\"# webinar.rtms_stopped\\nThe **RTMS stopped in Webinar** event is triggered when the webinar host or the initiator stops a real-time stream.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **RTMS stopped in Webinar** subscription enabled under the **RTMS** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_stopped`,`rtms:read:rtms_stopped:admin`\\n\\n**Event type**: `webinar.rtms_stopped`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the webinar RTMS event.\",\"required\":[\"webinar_uuid\",\"rtms_stream_id\",\"stop_reason\"],\"properties\":{\"webinar_uuid\":{\"type\":\"string\",\"description\":\"The webinar's universally unique identifier (UUID). Each webinar instance generates a webinar UUID.\"},\"rtms_stream_id\":{\"type\":\"string\",\"description\":\"The unique ID of one of streams of the webinar. Each webinar can has multiple streams.\"},\"stop_reason\":{\"type\":\"integer\",\"format\":\"int\",\"description\":\"The reason code of why the stream stopped.\\n* 1 = Triggered by the webinar host. \\n* 2 = Triggered by the app user. \\n* 3 = App user left webinar. \\n* 4 = App user ejected by the webinar host. \\n* 5 = The app is disabled by the webinar host. \\n* 6 = The webinar is ended. \\n* 7 = The stream is canceled. \\n* 8 = The stream revoked. \\n* 9 = All apps are disabled. \\n* 10 = Server internal exception. \\n* 11 = The connection timed out.\",\"enum\":[1,2,3,4,7,8,9,10,11]}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"webinar.rtms_stopped\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"webinar_uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\",\\n    \\\"rtms_stream_id\\\": \\\"609340fb2a7946909659956c8aa9250c\\\",\\n    \\\"stop_reason\\\": 1\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the webinar RTMS event.\",\"required\":[\"webinar_uuid\",\"rtms_stream_id\",\"stop_reason\"],\"properties\":{\"webinar_uuid\":{\"type\":\"string\",\"description\":\"The webinar's universally unique identifier (UUID). Each webinar instance generates a webinar UUID.\"},\"rtms_stream_id\":{\"type\":\"string\",\"description\":\"The unique ID of one of streams of the webinar. Each webinar can has multiple streams.\"},\"stop_reason\":{\"type\":\"integer\",\"format\":\"int\",\"description\":\"The reason code of why the stream stopped.\\n* 1 = Triggered by the webinar host. \\n* 2 = Triggered by the app user. \\n* 3 = App user left webinar. \\n* 4 = App user ejected by the webinar host. \\n* 5 = The app is disabled by the webinar host. \\n* 6 = The webinar is ended. \\n* 7 = The stream is canceled. \\n* 8 = The stream revoked. \\n* 9 = All apps are disabled. \\n* 10 = Server internal exception. \\n* 11 = The connection timed out.\",\"enum\":[1,2,3,4,7,8,9,10,11]}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"webinar.rtms_stopped\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"webinar_uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\",\\n    \\\"rtms_stream_id\\\": \\\"609340fb2a7946909659956c8aa9250c\\\",\\n    \\\"stop_reason\\\": 1\\n  }\\n}\"}}}}}}}},\"webinar.rtms_started\":{\"post\":{\"tags\":[\"webinar\"],\"operationId\":\"webinar.rtms_started\",\"requestBody\":{\"description\":\"# webinar.rtms_started\\nThe **RTMS started in Webinar** event is triggered when a webinar host or a webinar panelist starts a real-time stream action.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **RTMS started in Webinar** subscription enabled under the **RTMS** event.\\n  * The initiator must be a user of your [webhook-enabled](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks) app.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_started`,`rtms:read:rtms_started:admin`\\n\\n**Event type**: `webinar.rtms_started`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the webinar RTMS event.\",\"required\":[\"webinar_uuid\",\"operator_id\",\"rtms_stream_id\",\"server_urls\"],\"properties\":{\"webinar_uuid\":{\"type\":\"string\",\"description\":\"The webinar's universally unique identifier (UUID). Each webinar instance generates a webinar UUID.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The ID of the user who is triggered this rtms event.\",\"example\":\"30R7kT7bTIKSNUFEuH_Qlg\"},\"rtms_stream_id\":{\"type\":\"string\",\"description\":\"The unique ID of one of streams of the webinar. Each webinar can has multiple streams.\"},\"server_urls\":{\"type\":\"string\",\"description\":\"The RTMS server urls for the app to connect with. Multiple URLs for different media protocols will be separated by commas.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"webinar.rtms_started\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"webinar_uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\",\\n    \\\"operator_id\\\": \\\"30R7kT7bTIKSNUFEuH_Qlg\\\",\\n    \\\"rtms_stream_id\\\": \\\"609340fb2a7946909659956c8aa9250c\\\",\\n    \\\"server_urls\\\": \\\"wss://127.0.0.1:443\\\"\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the webinar RTMS event.\",\"required\":[\"webinar_uuid\",\"operator_id\",\"rtms_stream_id\",\"server_urls\"],\"properties\":{\"webinar_uuid\":{\"type\":\"string\",\"description\":\"The webinar's universally unique identifier (UUID). Each webinar instance generates a webinar UUID.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The ID of the user who is triggered this rtms event.\",\"example\":\"30R7kT7bTIKSNUFEuH_Qlg\"},\"rtms_stream_id\":{\"type\":\"string\",\"description\":\"The unique ID of one of streams of the webinar. Each webinar can has multiple streams.\"},\"server_urls\":{\"type\":\"string\",\"description\":\"The RTMS server urls for the app to connect with. Multiple URLs for different media protocols will be separated by commas.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"webinar.rtms_started\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"webinar_uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\",\\n    \\\"operator_id\\\": \\\"30R7kT7bTIKSNUFEuH_Qlg\\\",\\n    \\\"rtms_stream_id\\\": \\\"609340fb2a7946909659956c8aa9250c\\\",\\n    \\\"server_urls\\\": \\\"wss://127.0.0.1:443\\\"\\n  }\\n}\"}}}}}}}},\"webinar.rtms_interrupted\":{\"post\":{\"tags\":[\"webinar\"],\"operationId\":\"webinar.rtms_interrupted\",\"requestBody\":{\"description\":\"# webinar.rtms_interrupted\\nThe **RTMS interrupted in Webinar** event is triggered when an ongoing webinar stream gets disrupted due to a broken signaling socket connection.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **RTMS interrupted in Webinar** subscription enabled under the **RTMS** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_interrupted`,`rtms:read:rtms_interrupted:admin`\\n\\n**Event type**: `webinar.rtms_interrupted`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the webinar RTMS event.\",\"required\":[\"webinar_uuid\",\"rtms_stream_id\"],\"properties\":{\"webinar_uuid\":{\"type\":\"string\",\"description\":\"The webinar's universally unique identifier (UUID). Each webinar instance generates a webinar UUID.\"},\"rtms_stream_id\":{\"type\":\"string\",\"description\":\"The unique ID of one of streams of the webinar. Each webinar can has multiple streams.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"webinar.rtms_interrupted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"webinar_uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\",\\n    \\\"rtms_stream_id\\\": \\\"609340fb2a7946909659956c8aa9250c\\\"\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the webinar RTMS event.\",\"required\":[\"webinar_uuid\",\"rtms_stream_id\"],\"properties\":{\"webinar_uuid\":{\"type\":\"string\",\"description\":\"The webinar's universally unique identifier (UUID). Each webinar instance generates a webinar UUID.\"},\"rtms_stream_id\":{\"type\":\"string\",\"description\":\"The unique ID of one of streams of the webinar. Each webinar can has multiple streams.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"webinar.rtms_interrupted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"webinar_uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\",\\n    \\\"rtms_stream_id\\\": \\\"609340fb2a7946909659956c8aa9250c\\\"\\n  }\\n}\"}}}}}}}},\"session.rtms_stopped\":{\"post\":{\"tags\":[\"session\"],\"operationId\":\"session.rtms_stopped\",\"requestBody\":{\"description\":\"# session.rtms_stopped\\nThe **RTMS Stopped** event is triggered when the session host stops a realtime media stream.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **RTMS Stopped** subscription enabled under the **RTMS** event.\\n\\n**Event type**: `session.rtms_stopped`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the session rtms event.\",\"required\":[\"session_id\",\"session_key\",\"rtms_stream_id\",\"stop_reason\"],\"properties\":{\"session_id\":{\"type\":\"string\",\"description\":\"The session's universally unique identifier (UUID). Each session instance generates a session UUID.\"},\"session_key\":{\"type\":\"string\",\"description\":\"The Video SDK custom session ID.\"},\"rtms_stream_id\":{\"type\":\"string\",\"description\":\"The unique ID of one of streams of the session. \\n **Note**: A session can have multiple streams.\"},\"stop_reason\":{\"type\":\"integer\",\"format\":\"int\",\"description\":\"The reason code of the stream stopped.\\n* 1 = Triggered by the session host. \\n* 2 = Triggered by the app user. \\n* 3 = App user left session. \\n* 4 = App user ejected by the session host. \\n* 5 = The app is disabled by the session host. \\n* 6 = The session is ended. \\n* 7 = The stream is canceled. \\n* 8 = The stream was revoked. \\n* 9 = All apps are disabled. \\n* 10 = Server internal exception. \\n* 11 = The connection timed out.\",\"enum\":[1,2,3,4,7,8,9,10,11]}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"session.rtms_stopped\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"session_id\\\": \\\"4567UVWxYZABCdEfGhiJkl==\\\",\\n    \\\"session_key\\\": \\\"ABC36jaBI145\\\",\\n    \\\"rtms_stream_id\\\": \\\"609340fb2a7946909659956c8aa9250c\\\",\\n    \\\"stop_reason\\\": 1\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the session rtms event.\",\"required\":[\"session_id\",\"session_key\",\"rtms_stream_id\",\"stop_reason\"],\"properties\":{\"session_id\":{\"type\":\"string\",\"description\":\"The session's universally unique identifier (UUID). Each session instance generates a session UUID.\"},\"session_key\":{\"type\":\"string\",\"description\":\"The Video SDK custom session ID.\"},\"rtms_stream_id\":{\"type\":\"string\",\"description\":\"The unique ID of one of streams of the session. \\n **Note**: A session can have multiple streams.\"},\"stop_reason\":{\"type\":\"integer\",\"format\":\"int\",\"description\":\"The reason code of the stream stopped.\\n* 1 = Triggered by the session host. \\n* 2 = Triggered by the app user. \\n* 3 = App user left session. \\n* 4 = App user ejected by the session host. \\n* 5 = The app is disabled by the session host. \\n* 6 = The session is ended. \\n* 7 = The stream is canceled. \\n* 8 = The stream was revoked. \\n* 9 = All apps are disabled. \\n* 10 = Server internal exception. \\n* 11 = The connection timed out.\",\"enum\":[1,2,3,4,7,8,9,10,11]}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"session.rtms_stopped\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"session_id\\\": \\\"4567UVWxYZABCdEfGhiJkl==\\\",\\n    \\\"session_key\\\": \\\"ABC36jaBI145\\\",\\n    \\\"rtms_stream_id\\\": \\\"609340fb2a7946909659956c8aa9250c\\\",\\n    \\\"stop_reason\\\": 1\\n  }\\n}\"}}}}}}}},\"session.rtms_started\":{\"post\":{\"tags\":[\"session\"],\"operationId\":\"session.rtms_started\",\"requestBody\":{\"description\":\"# session.rtms_started\\nThe **RTMS Started** event is triggered when a session host starts a real-time stream.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **RTMS Started** subscription enabled under the **RTMS** event.\\n  * The initiator must be a user of your [webhook-enabled](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks) app.\\n\\n**Event type**: `session.rtms_started`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the session rtms event.\",\"required\":[\"session_id\",\"session_key\",\"rtms_stream_id\",\"server_urls\"],\"properties\":{\"session_id\":{\"type\":\"string\",\"description\":\"Unique session identifier. Each instance of the session will have its own session_id.\"},\"session_key\":{\"type\":\"string\",\"description\":\"The Video SDK custom session ID.\"},\"rtms_stream_id\":{\"type\":\"string\",\"description\":\"The unique ID of one of streams of the session. \\n **Note**: A session can have multiple streams.\"},\"server_urls\":{\"type\":\"string\",\"description\":\"The RTMS server urls for an app to connect with. Multiple URLs for different protocols will be separated by commas.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"session.rtms_started\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"session_id\\\": \\\"4567UVWxYZABCdEfGhiJkl==\\\",\\n    \\\"session_key\\\": \\\"ABC36jaBI145\\\",\\n    \\\"rtms_stream_id\\\": \\\"609340fb2a7946909659956c8aa9250c\\\",\\n    \\\"server_urls\\\": \\\"wss://127.0.0.1:443\\\"\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the session rtms event.\",\"required\":[\"session_id\",\"session_key\",\"rtms_stream_id\",\"server_urls\"],\"properties\":{\"session_id\":{\"type\":\"string\",\"description\":\"Unique session identifier. Each instance of the session will have its own session_id.\"},\"session_key\":{\"type\":\"string\",\"description\":\"The Video SDK custom session ID.\"},\"rtms_stream_id\":{\"type\":\"string\",\"description\":\"The unique ID of one of streams of the session. \\n **Note**: A session can have multiple streams.\"},\"server_urls\":{\"type\":\"string\",\"description\":\"The RTMS server urls for an app to connect with. Multiple URLs for different protocols will be separated by commas.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"session.rtms_started\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"session_id\\\": \\\"4567UVWxYZABCdEfGhiJkl==\\\",\\n    \\\"session_key\\\": \\\"ABC36jaBI145\\\",\\n    \\\"rtms_stream_id\\\": \\\"609340fb2a7946909659956c8aa9250c\\\",\\n    \\\"server_urls\\\": \\\"wss://127.0.0.1:443\\\"\\n  }\\n}\"}}}}}}}},\"rtms.concurrency_limited\":{\"post\":{\"tags\":[\"rtms\"],\"operationId\":\"rtms.concurrency_limited\",\"requestBody\":{\"description\":\"# rtms.concurrency_limited\\nThe **RTMS Concurrency Limited** event is triggered when an app reaches the predetermined limit of concurrent streams for the app owner's account. The default is 2000 concurrent streams.\\n\\n\\nThis event notifies the app owner if their usage exceeds the number of available concurrent streams.\\n\\n\\n\\nWhen the concurrency limit is reached, no new streams can be initiated.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **RTMS Concurrency Limited** subscription enabled under the **RTMS** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `rtms:read:rtms_concurrency_limited`,`rtms:read:rtms_concurrency_limited:admin`\\n\\n**Event type**: `rtms.concurrency_limited`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the meeting rtms event.\",\"required\":[\"operator_id\",\"concurrency_number\",\"id_field\"],\"properties\":{\"operator_id\":{\"type\":\"string\",\"description\":\"The ID of the user who triggered this RTMS event.\"},\"concurrency_number\":{\"type\":\"integer\",\"description\":\"The number of concurrent streams.\"},\"id_field\":{\"oneOf\":[{\"properties\":{\"meeting_uuid\":{\"type\":\"string\",\"description\":\"The meeting's universally unique identifier (UUID).\"}},\"required\":[\"meeting_uuid\"]},{\"properties\":{\"webinar_uuid\":{\"type\":\"string\",\"description\":\"The webinar's universally unique identifier (UUID).\"}},\"required\":[\"webinar_uuid\"]},{\"properties\":{\"session_id\":{\"type\":\"string\",\"description\":\"The session's universally unique identifier (UUID).\"}},\"required\":[\"session_id\"]}]}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"rtms.concurrency_limited\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"operator_id\\\": \\\"30R7kT7bTIKSNUFEuH_Qlg\\\",\\n    \\\"concurrency_number\\\": 2001,\\n    \\\"id_field\\\": {\\n      \\\"meeting_uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"description\":\"Information about the meeting rtms event.\",\"required\":[\"operator_id\",\"concurrency_number\",\"id_field\"],\"properties\":{\"operator_id\":{\"type\":\"string\",\"description\":\"The ID of the user who triggered this RTMS event.\"},\"concurrency_number\":{\"type\":\"integer\",\"description\":\"The number of concurrent streams.\"},\"id_field\":{\"oneOf\":[{\"properties\":{\"meeting_uuid\":{\"type\":\"string\",\"description\":\"The meeting's universally unique identifier (UUID).\"}},\"required\":[\"meeting_uuid\"]},{\"properties\":{\"webinar_uuid\":{\"type\":\"string\",\"description\":\"The webinar's universally unique identifier (UUID).\"}},\"required\":[\"webinar_uuid\"]},{\"properties\":{\"session_id\":{\"type\":\"string\",\"description\":\"The session's universally unique identifier (UUID).\"}},\"required\":[\"session_id\"]}]}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"rtms.concurrency_limited\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"operator_id\\\": \\\"30R7kT7bTIKSNUFEuH_Qlg\\\",\\n    \\\"concurrency_number\\\": 2001,\\n    \\\"id_field\\\": {\\n      \\\"meeting_uuid\\\": \\\"4444AAAiAAAAAiAiAiiAii==\\\"\\n    }\\n  }\\n}\"}}}}}}}}},\"servers\":[{\"url\":\"https://{your-endpoint-url}\"}]}",
    "frontmatter": {
      "title": "RTMS Webhooks",
      "keywords": "zoom, webhooks, api, rtms",
      "description": "RTMS is a data pipeline that gives your app access to live audio, video, and transcript data from Zoom Meetings.",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
