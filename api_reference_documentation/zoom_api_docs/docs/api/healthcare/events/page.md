# Healthcare Webhooks

- Source URL: https://developers.zoom.us/docs/api/healthcare/events/
- Snapshot path: docs/api/healthcare/events
- Generated (UTC): 2026-02-07T22:40:10.802618+00:00

## Frontmatter

```json
{
  "title": "Healthcare Webhooks",
  "keywords": "zoom, webhooks, api, clinical notes",
  "description": "Healthcare events",
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
    "title": "Healthcare",
    "description": "",
    "version": "1.0.0"
  },
  "paths": {
    "clinical_notes.note_created": {
      "post": {
        "tags": [
          "clinical_notes"
        ],
        "operationId": "clinical_notes.note_created",
        "requestBody": {
          "description": "# clinical_notes.note_created\nThe **Clinical Note Created** event is triggered every time the clinical note of the meeting is made available for one of your account users after the meeting's clinical note streaming session ends.\n\n## Prerequisites\n\n* Host user must be Pro or higher plan.\n* Enable the Clinical Note feature in the host's account.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n  * A valid **Event Notification Endpoint URL**\n  * The **Clinical Note have created** subscription must be enabled under the **Healthcare** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `clinical_note:read:admin`,`clinical_note:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `healthcare:read:list_clinical_notes:admin`,`healthcare:read:list_clinical_notes:master`,`healthcare:read:list_clinical_notes`\n\n**Event type**: `clinical_notes.note_created`\n",
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
                    "description": "The event's name."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "The timestamp (in milliseconds since epoch) at which the event occurred. The value of this field is returned in long(int64) format."
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
                        "description": "The account ID of the user who started the clinical notes session in the meeting and owns the note document."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the meeting clinical note.",
                        "required": [
                          "meeting_id",
                          "meeting_number",
                          "meeting_topic",
                          "meeting_host_id",
                          "note_id",
                          "note_owner_user_id",
                          "note_start_time",
                          "note_end_time",
                          "note_created_time",
                          "note_title",
                          "ehr_context_available"
                        ],
                        "properties": {
                          "meeting_id": {
                            "type": "string",
                            "description": "Unique meeting ID. Each meeting instance generates its own meeting UUID. After a meeting ends, a new UUID is generated for the next instance of the meeting. Retrieve a list of UUIDs from past meeting instances using the [**List past meeting instances**](https://developers.zoom.us/docs/api/rest/reference/zoom-api/methods/#operation/pastMeetings) API. [Double encode](https://developers.zoom.us/docs/api/using-zoom-apis/#double-encoding) your UUID when using it for API calls if the UUID begins with a / or contains // in it.\n"
                          },
                          "meeting_number": {
                            "type": "string",
                            "description": "The meeting's number."
                          },
                          "meeting_topic": {
                            "type": "string",
                            "description": "Meeting topic."
                          },
                          "meeting_host_id": {
                            "type": "string",
                            "description": "The ID of the user who is set as the meeting host."
                          },
                          "note_id": {
                            "type": "string",
                            "description": "Unique identifier of the note.",
                            "example": "ArfRXokSTiGTCSYFVtxNa"
                          },
                          "note_owner_user_id": {
                            "type": "string",
                            "description": "The user ID of the user who started the clinical notes session and owns the note document.",
                            "example": "dqV7XEpnS5G3rGsvqylmjQ"
                          },
                          "note_start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The clinical note's start date and time in yyyy-MM-dd'T'HH:mm:ss'Z' UTC format."
                          },
                          "note_end_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The clinical note's end date and time in yyyy-MM-dd'T'HH:mm:ss'Z' UTC format."
                          },
                          "note_created_time": {
                            "type": "string",
                            "description": "The date and time when the clinical note was created.",
                            "format": "date-time"
                          },
                          "note_title": {
                            "type": "string",
                            "description": "The clinical note title."
                          },
                          "ehr_context_available": {
                            "type": "boolean",
                            "description": "Indicates whether the EHR context data (such as appointment, patient, and provider identifiers) is available for this clinical note."
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
                  "value": "{\n  \"event\": \"clinical_notes.note_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"lm5YQGvrQkufX4Z87vVZyQ\",\n    \"object\": {\n      \"meeting_id\": \"I3pHE/RnRbmm+ex2g07sxw==\",\n      \"meeting_number\": \"94564522297\",\n      \"meeting_topic\": \"Provider John's Zoom Meeting\",\n      \"meeting_host_id\": \"dqV7XEpnS5G3rGsvqylmjQ\",\n      \"note_id\": \"ArfRXokSTiGTCSYFVtxNa\",\n      \"note_owner_user_id\": \"dqV7XEpnS5G3rGsvqylmjQ\",\n      \"note_start_time\": \"2025-05-15T03:01:08.608Z\",\n      \"note_end_time\": \"2025-05-15T03:25:59.608Z\",\n      \"note_created_time\": \"2025-05-15T03:26:08.608Z\",\n      \"note_title\": \"Monica, F, 25\",\n      \"ehr_context_available\": false\n    }\n  }\n}"
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
                      "description": "The event's name."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "The timestamp (in milliseconds since epoch) at which the event occurred. The value of this field is returned in long(int64) format."
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
                          "description": "The account ID of the user who started the clinical notes session in the meeting and owns the note document."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the meeting clinical note.",
                          "required": [
                            "meeting_id",
                            "meeting_number",
                            "meeting_topic",
                            "meeting_host_id",
                            "note_id",
                            "note_owner_user_id",
                            "note_start_time",
                            "note_end_time",
                            "note_created_time",
                            "note_title",
                            "ehr_context_available"
                          ],
                          "properties": {
                            "meeting_id": {
                              "type": "string",
                              "description": "Unique meeting ID. Each meeting instance generates its own meeting UUID. After a meeting ends, a new UUID is generated for the next instance of the meeting. Retrieve a list of UUIDs from past meeting instances using the [**List past meeting instances**](https://developers.zoom.us/docs/api/rest/reference/zoom-api/methods/#operation/pastMeetings) API. [Double encode](https://developers.zoom.us/docs/api/using-zoom-apis/#double-encoding) your UUID when using it for API calls if the UUID begins with a / or contains // in it.\n"
                            },
                            "meeting_number": {
                              "type": "string",
                              "description": "The meeting's number."
                            },
                            "meeting_topic": {
                              "type": "string",
                              "description": "Meeting topic."
                            },
                            "meeting_host_id": {
                              "type": "string",
                              "description": "The ID of the user who is set as the meeting host."
                            },
                            "note_id": {
                              "type": "string",
                              "description": "Unique identifier of the note.",
                              "example": "ArfRXokSTiGTCSYFVtxNa"
                            },
                            "note_owner_user_id": {
                              "type": "string",
                              "description": "The user ID of the user who started the clinical notes session and owns the note document.",
                              "example": "dqV7XEpnS5G3rGsvqylmjQ"
                            },
                            "note_start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The clinical note's start date and time in yyyy-MM-dd'T'HH:mm:ss'Z' UTC format."
                            },
                            "note_end_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The clinical note's end date and time in yyyy-MM-dd'T'HH:mm:ss'Z' UTC format."
                            },
                            "note_created_time": {
                              "type": "string",
                              "description": "The date and time when the clinical note was created.",
                              "format": "date-time"
                            },
                            "note_title": {
                              "type": "string",
                              "description": "The clinical note title."
                            },
                            "ehr_context_available": {
                              "type": "boolean",
                              "description": "Indicates whether the EHR context data (such as appointment, patient, and provider identifiers) is available for this clinical note."
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
                    "value": "{\n  \"event\": \"clinical_notes.note_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"lm5YQGvrQkufX4Z87vVZyQ\",\n    \"object\": {\n      \"meeting_id\": \"I3pHE/RnRbmm+ex2g07sxw==\",\n      \"meeting_number\": \"94564522297\",\n      \"meeting_topic\": \"Provider John's Zoom Meeting\",\n      \"meeting_host_id\": \"dqV7XEpnS5G3rGsvqylmjQ\",\n      \"note_id\": \"ArfRXokSTiGTCSYFVtxNa\",\n      \"note_owner_user_id\": \"dqV7XEpnS5G3rGsvqylmjQ\",\n      \"note_start_time\": \"2025-05-15T03:01:08.608Z\",\n      \"note_end_time\": \"2025-05-15T03:25:59.608Z\",\n      \"note_created_time\": \"2025-05-15T03:26:08.608Z\",\n      \"note_title\": \"Monica, F, 25\",\n      \"ehr_context_available\": false\n    }\n  }\n}"
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
    "title": "Healthcare",
    "description": "",
    "version": "1.0.0"
  },
  "webhooks": {
    "clinical_notes.note_created": {
      "post": {
        "tags": [
          "clinical_notes"
        ],
        "operationId": "clinical_notes.note_created",
        "requestBody": {
          "description": "# clinical_notes.note_created\nThe **Clinical Note Created** event is triggered every time the clinical note of the meeting is made available for one of your account users after the meeting's clinical note streaming session ends.\n\n## Prerequisites\n\n* Host user must be Pro or higher plan.\n* Enable the Clinical Note feature in the host's account.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n  * A valid **Event Notification Endpoint URL**\n  * The **Clinical Note have created** subscription must be enabled under the **Healthcare** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `clinical_note:read:admin`,`clinical_note:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `healthcare:read:list_clinical_notes:admin`,`healthcare:read:list_clinical_notes:master`,`healthcare:read:list_clinical_notes`\n\n**Event type**: `clinical_notes.note_created`\n",
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
                    "description": "The event's name."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "The timestamp (in milliseconds since epoch) at which the event occurred. The value of this field is returned in long(int64) format."
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
                        "description": "The account ID of the user who started the clinical notes session in the meeting and owns the note document."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the meeting clinical note.",
                        "required": [
                          "meeting_id",
                          "meeting_number",
                          "meeting_topic",
                          "meeting_host_id",
                          "note_id",
                          "note_owner_user_id",
                          "note_start_time",
                          "note_end_time",
                          "note_created_time",
                          "note_title",
                          "ehr_context_available"
                        ],
                        "properties": {
                          "meeting_id": {
                            "type": "string",
                            "description": "Unique meeting ID. Each meeting instance generates its own meeting UUID. After a meeting ends, a new UUID is generated for the next instance of the meeting. Retrieve a list of UUIDs from past meeting instances using the [**List past meeting instances**](https://developers.zoom.us/docs/api/rest/reference/zoom-api/methods/#operation/pastMeetings) API. [Double encode](https://developers.zoom.us/docs/api/using-zoom-apis/#double-encoding) your UUID when using it for API calls if the UUID begins with a / or contains // in it.\n"
                          },
                          "meeting_number": {
                            "type": "string",
                            "description": "The meeting's number."
                          },
                          "meeting_topic": {
                            "type": "string",
                            "description": "Meeting topic."
                          },
                          "meeting_host_id": {
                            "type": "string",
                            "description": "The ID of the user who is set as the meeting host."
                          },
                          "note_id": {
                            "type": "string",
                            "description": "Unique identifier of the note.",
                            "example": "ArfRXokSTiGTCSYFVtxNa"
                          },
                          "note_owner_user_id": {
                            "type": "string",
                            "description": "The user ID of the user who started the clinical notes session and owns the note document.",
                            "example": "dqV7XEpnS5G3rGsvqylmjQ"
                          },
                          "note_start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The clinical note's start date and time in yyyy-MM-dd'T'HH:mm:ss'Z' UTC format."
                          },
                          "note_end_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The clinical note's end date and time in yyyy-MM-dd'T'HH:mm:ss'Z' UTC format."
                          },
                          "note_created_time": {
                            "type": "string",
                            "description": "The date and time when the clinical note was created.",
                            "format": "date-time"
                          },
                          "note_title": {
                            "type": "string",
                            "description": "The clinical note title."
                          },
                          "ehr_context_available": {
                            "type": "boolean",
                            "description": "Indicates whether the EHR context data (such as appointment, patient, and provider identifiers) is available for this clinical note."
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
                  "value": "{\n  \"event\": \"clinical_notes.note_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"lm5YQGvrQkufX4Z87vVZyQ\",\n    \"object\": {\n      \"meeting_id\": \"I3pHE/RnRbmm+ex2g07sxw==\",\n      \"meeting_number\": \"94564522297\",\n      \"meeting_topic\": \"Provider John's Zoom Meeting\",\n      \"meeting_host_id\": \"dqV7XEpnS5G3rGsvqylmjQ\",\n      \"note_id\": \"ArfRXokSTiGTCSYFVtxNa\",\n      \"note_owner_user_id\": \"dqV7XEpnS5G3rGsvqylmjQ\",\n      \"note_start_time\": \"2025-05-15T03:01:08.608Z\",\n      \"note_end_time\": \"2025-05-15T03:25:59.608Z\",\n      \"note_created_time\": \"2025-05-15T03:26:08.608Z\",\n      \"note_title\": \"Monica, F, 25\",\n      \"ehr_context_available\": false\n    }\n  }\n}"
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
                      "description": "The event's name."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "The timestamp (in milliseconds since epoch) at which the event occurred. The value of this field is returned in long(int64) format."
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
                          "description": "The account ID of the user who started the clinical notes session in the meeting and owns the note document."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the meeting clinical note.",
                          "required": [
                            "meeting_id",
                            "meeting_number",
                            "meeting_topic",
                            "meeting_host_id",
                            "note_id",
                            "note_owner_user_id",
                            "note_start_time",
                            "note_end_time",
                            "note_created_time",
                            "note_title",
                            "ehr_context_available"
                          ],
                          "properties": {
                            "meeting_id": {
                              "type": "string",
                              "description": "Unique meeting ID. Each meeting instance generates its own meeting UUID. After a meeting ends, a new UUID is generated for the next instance of the meeting. Retrieve a list of UUIDs from past meeting instances using the [**List past meeting instances**](https://developers.zoom.us/docs/api/rest/reference/zoom-api/methods/#operation/pastMeetings) API. [Double encode](https://developers.zoom.us/docs/api/using-zoom-apis/#double-encoding) your UUID when using it for API calls if the UUID begins with a / or contains // in it.\n"
                            },
                            "meeting_number": {
                              "type": "string",
                              "description": "The meeting's number."
                            },
                            "meeting_topic": {
                              "type": "string",
                              "description": "Meeting topic."
                            },
                            "meeting_host_id": {
                              "type": "string",
                              "description": "The ID of the user who is set as the meeting host."
                            },
                            "note_id": {
                              "type": "string",
                              "description": "Unique identifier of the note.",
                              "example": "ArfRXokSTiGTCSYFVtxNa"
                            },
                            "note_owner_user_id": {
                              "type": "string",
                              "description": "The user ID of the user who started the clinical notes session and owns the note document.",
                              "example": "dqV7XEpnS5G3rGsvqylmjQ"
                            },
                            "note_start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The clinical note's start date and time in yyyy-MM-dd'T'HH:mm:ss'Z' UTC format."
                            },
                            "note_end_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The clinical note's end date and time in yyyy-MM-dd'T'HH:mm:ss'Z' UTC format."
                            },
                            "note_created_time": {
                              "type": "string",
                              "description": "The date and time when the clinical note was created.",
                              "format": "date-time"
                            },
                            "note_title": {
                              "type": "string",
                              "description": "The clinical note title."
                            },
                            "ehr_context_available": {
                              "type": "boolean",
                              "description": "Indicates whether the EHR context data (such as appointment, patient, and provider identifiers) is available for this clinical note."
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
                    "value": "{\n  \"event\": \"clinical_notes.note_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"lm5YQGvrQkufX4Z87vVZyQ\",\n    \"object\": {\n      \"meeting_id\": \"I3pHE/RnRbmm+ex2g07sxw==\",\n      \"meeting_number\": \"94564522297\",\n      \"meeting_topic\": \"Provider John's Zoom Meeting\",\n      \"meeting_host_id\": \"dqV7XEpnS5G3rGsvqylmjQ\",\n      \"note_id\": \"ArfRXokSTiGTCSYFVtxNa\",\n      \"note_owner_user_id\": \"dqV7XEpnS5G3rGsvqylmjQ\",\n      \"note_start_time\": \"2025-05-15T03:01:08.608Z\",\n      \"note_end_time\": \"2025-05-15T03:25:59.608Z\",\n      \"note_created_time\": \"2025-05-15T03:26:08.608Z\",\n      \"note_title\": \"Monica, F, 25\",\n      \"ehr_context_available\": false\n    }\n  }\n}"
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
      "healthcare",
      "events"
    ]
  },
  "downloadPath": "/api-hub/healthcare/events/webhooks.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": false
  },
  "productMeta": {
    "specType": "adoc",
    "slug": "/docs/api/rest/reference/healthcare/events",
    "title": "Healthcare",
    "keywords": "zoom, webhooks, api, clinical notes",
    "params": {
      "productName": "Integration Healthcare"
    },
    "zoapEndpoint": "webhookByProduct",
    "productDisplayTitle": "Healthcare",
    "scalarSlug": "/docs/api/healthcare/events",
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "svgname": "clinical-notes"
  },
  "frontmatter": {
    "title": "Healthcare Webhooks",
    "keywords": "zoom, webhooks, api, clinical notes",
    "description": "Healthcare events",
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
        "healthcare",
        "events"
      ]
    },
    "downloadPath": "/api-hub/healthcare/events/webhooks.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": false
    },
    "productMeta": {
      "specType": "adoc",
      "slug": "/docs/api/rest/reference/healthcare/events",
      "title": "Healthcare",
      "keywords": "zoom, webhooks, api, clinical notes",
      "params": {
        "productName": "Integration Healthcare"
      },
      "zoapEndpoint": "webhookByProduct",
      "productDisplayTitle": "Healthcare",
      "scalarSlug": "/docs/api/healthcare/events",
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "svgname": "clinical-notes"
    },
    "spec": "{\"openapi\":\"3.1.0\",\"info\":{\"title\":\"Healthcare\",\"description\":\"\",\"version\":\"1.0.0\"},\"paths\":{\"clinical_notes.note_created\":{\"post\":{\"tags\":[\"clinical_notes\"],\"operationId\":\"clinical_notes.note_created\",\"requestBody\":{\"description\":\"# clinical_notes.note_created\\nThe **Clinical Note Created** event is triggered every time the clinical note of the meeting is made available for one of your account users after the meeting's clinical note streaming session ends.\\n\\n## Prerequisites\\n\\n* Host user must be Pro or higher plan.\\n* Enable the Clinical Note feature in the host's account.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n  * A valid **Event Notification Endpoint URL**\\n  * The **Clinical Note have created** subscription must be enabled under the **Healthcare** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `clinical_note:read:admin`,`clinical_note:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `healthcare:read:list_clinical_notes:admin`,`healthcare:read:list_clinical_notes:master`,`healthcare:read:list_clinical_notes`\\n\\n**Event type**: `clinical_notes.note_created`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The event's name.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp (in milliseconds since epoch) at which the event occurred. The value of this field is returned in long(int64) format.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who started the clinical notes session in the meeting and owns the note document.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the meeting clinical note.\",\"required\":[\"meeting_id\",\"meeting_number\",\"meeting_topic\",\"meeting_host_id\",\"note_id\",\"note_owner_user_id\",\"note_start_time\",\"note_end_time\",\"note_created_time\",\"note_title\",\"ehr_context_available\"],\"properties\":{\"meeting_id\":{\"type\":\"string\",\"description\":\"Unique meeting ID. Each meeting instance generates its own meeting UUID. After a meeting ends, a new UUID is generated for the next instance of the meeting. Retrieve a list of UUIDs from past meeting instances using the [**List past meeting instances**](https://developers.zoom.us/docs/api/rest/reference/zoom-api/methods/#operation/pastMeetings) API. [Double encode](https://developers.zoom.us/docs/api/using-zoom-apis/#double-encoding) your UUID when using it for API calls if the UUID begins with a / or contains // in it.\\n\"},\"meeting_number\":{\"type\":\"string\",\"description\":\"The meeting's number.\"},\"meeting_topic\":{\"type\":\"string\",\"description\":\"Meeting topic.\"},\"meeting_host_id\":{\"type\":\"string\",\"description\":\"The ID of the user who is set as the meeting host.\"},\"note_id\":{\"type\":\"string\",\"description\":\"Unique identifier of the note.\",\"example\":\"ArfRXokSTiGTCSYFVtxNa\"},\"note_owner_user_id\":{\"type\":\"string\",\"description\":\"The user ID of the user who started the clinical notes session and owns the note document.\",\"example\":\"dqV7XEpnS5G3rGsvqylmjQ\"},\"note_start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The clinical note's start date and time in yyyy-MM-dd'T'HH:mm:ss'Z' UTC format.\"},\"note_end_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The clinical note's end date and time in yyyy-MM-dd'T'HH:mm:ss'Z' UTC format.\"},\"note_created_time\":{\"type\":\"string\",\"description\":\"The date and time when the clinical note was created.\",\"format\":\"date-time\"},\"note_title\":{\"type\":\"string\",\"description\":\"The clinical note title.\"},\"ehr_context_available\":{\"type\":\"boolean\",\"description\":\"Indicates whether the EHR context data (such as appointment, patient, and provider identifiers) is available for this clinical note.\"}},\"additionalProperties\":true}},\"additionalProperties\":true}},\"additionalProperties\":true},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"clinical_notes.note_created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"lm5YQGvrQkufX4Z87vVZyQ\\\",\\n    \\\"object\\\": {\\n      \\\"meeting_id\\\": \\\"I3pHE/RnRbmm+ex2g07sxw==\\\",\\n      \\\"meeting_number\\\": \\\"94564522297\\\",\\n      \\\"meeting_topic\\\": \\\"Provider John's Zoom Meeting\\\",\\n      \\\"meeting_host_id\\\": \\\"dqV7XEpnS5G3rGsvqylmjQ\\\",\\n      \\\"note_id\\\": \\\"ArfRXokSTiGTCSYFVtxNa\\\",\\n      \\\"note_owner_user_id\\\": \\\"dqV7XEpnS5G3rGsvqylmjQ\\\",\\n      \\\"note_start_time\\\": \\\"2025-05-15T03:01:08.608Z\\\",\\n      \\\"note_end_time\\\": \\\"2025-05-15T03:25:59.608Z\\\",\\n      \\\"note_created_time\\\": \\\"2025-05-15T03:26:08.608Z\\\",\\n      \\\"note_title\\\": \\\"Monica, F, 25\\\",\\n      \\\"ehr_context_available\\\": false\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The event's name.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp (in milliseconds since epoch) at which the event occurred. The value of this field is returned in long(int64) format.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who started the clinical notes session in the meeting and owns the note document.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the meeting clinical note.\",\"required\":[\"meeting_id\",\"meeting_number\",\"meeting_topic\",\"meeting_host_id\",\"note_id\",\"note_owner_user_id\",\"note_start_time\",\"note_end_time\",\"note_created_time\",\"note_title\",\"ehr_context_available\"],\"properties\":{\"meeting_id\":{\"type\":\"string\",\"description\":\"Unique meeting ID. Each meeting instance generates its own meeting UUID. After a meeting ends, a new UUID is generated for the next instance of the meeting. Retrieve a list of UUIDs from past meeting instances using the [**List past meeting instances**](https://developers.zoom.us/docs/api/rest/reference/zoom-api/methods/#operation/pastMeetings) API. [Double encode](https://developers.zoom.us/docs/api/using-zoom-apis/#double-encoding) your UUID when using it for API calls if the UUID begins with a / or contains // in it.\\n\"},\"meeting_number\":{\"type\":\"string\",\"description\":\"The meeting's number.\"},\"meeting_topic\":{\"type\":\"string\",\"description\":\"Meeting topic.\"},\"meeting_host_id\":{\"type\":\"string\",\"description\":\"The ID of the user who is set as the meeting host.\"},\"note_id\":{\"type\":\"string\",\"description\":\"Unique identifier of the note.\",\"example\":\"ArfRXokSTiGTCSYFVtxNa\"},\"note_owner_user_id\":{\"type\":\"string\",\"description\":\"The user ID of the user who started the clinical notes session and owns the note document.\",\"example\":\"dqV7XEpnS5G3rGsvqylmjQ\"},\"note_start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The clinical note's start date and time in yyyy-MM-dd'T'HH:mm:ss'Z' UTC format.\"},\"note_end_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The clinical note's end date and time in yyyy-MM-dd'T'HH:mm:ss'Z' UTC format.\"},\"note_created_time\":{\"type\":\"string\",\"description\":\"The date and time when the clinical note was created.\",\"format\":\"date-time\"},\"note_title\":{\"type\":\"string\",\"description\":\"The clinical note title.\"},\"ehr_context_available\":{\"type\":\"boolean\",\"description\":\"Indicates whether the EHR context data (such as appointment, patient, and provider identifiers) is available for this clinical note.\"}},\"additionalProperties\":true}},\"additionalProperties\":true}},\"additionalProperties\":true},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"clinical_notes.note_created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"lm5YQGvrQkufX4Z87vVZyQ\\\",\\n    \\\"object\\\": {\\n      \\\"meeting_id\\\": \\\"I3pHE/RnRbmm+ex2g07sxw==\\\",\\n      \\\"meeting_number\\\": \\\"94564522297\\\",\\n      \\\"meeting_topic\\\": \\\"Provider John's Zoom Meeting\\\",\\n      \\\"meeting_host_id\\\": \\\"dqV7XEpnS5G3rGsvqylmjQ\\\",\\n      \\\"note_id\\\": \\\"ArfRXokSTiGTCSYFVtxNa\\\",\\n      \\\"note_owner_user_id\\\": \\\"dqV7XEpnS5G3rGsvqylmjQ\\\",\\n      \\\"note_start_time\\\": \\\"2025-05-15T03:01:08.608Z\\\",\\n      \\\"note_end_time\\\": \\\"2025-05-15T03:25:59.608Z\\\",\\n      \\\"note_created_time\\\": \\\"2025-05-15T03:26:08.608Z\\\",\\n      \\\"note_title\\\": \\\"Monica, F, 25\\\",\\n      \\\"ehr_context_available\\\": false\\n    }\\n  }\\n}\"}}}}}}}}},\"servers\":[{\"url\":\"https://{your-endpoint-url}\"}]}",
    "frontmatter": {
      "title": "Healthcare Webhooks",
      "keywords": "zoom, webhooks, api, clinical notes",
      "description": "Healthcare events",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
