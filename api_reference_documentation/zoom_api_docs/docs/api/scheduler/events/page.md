# Scheduler Webhooks

- Source URL: https://developers.zoom.us/docs/api/scheduler/events/
- Snapshot path: docs/api/scheduler/events
- Generated (UTC): 2026-02-07T22:40:11.629953+00:00

## Frontmatter

```json
{
  "title": "Scheduler Webhooks",
  "keywords": "zoom, api, webhooks, scheduler",
  "description": "Scheduler webhooks allow developers to receive events for [Scheduler](https://developers.zoom.us/docs/zoom-scheduler/). ",
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
    "title": "Scheduler",
    "description": "Scheduler webhooks allow developers to receive events for [Scheduler](https://developers.zoom.us/docs/zoom-scheduler/). ",
    "version": "1.0.0"
  },
  "paths": {
    "scheduler.scheduled_event_created": {
      "post": {
        "tags": [
          "scheduler"
        ],
        "operationId": "scheduler.scheduled_event_created",
        "requestBody": {
          "description": "# scheduler.scheduled_event_created\nThe **Scheduled event created** event triggers each time one user schedules an event with the scheduler host through the public booking page.\n\n## Prerequisites\nEnable **Event subscriptions** for your Marketplace app with:\n\n* A valid **Event notification endpoint URL**.\n* The **Scheduled event created** subscription is enabled under the **Scheduler** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `scheduler:read:admin`,`scheduler:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `scheduler:read:scheduled_event`,`scheduler:read:scheduled_event:admin`\n\n**Event type**: `scheduler.scheduled_event_created`\n",
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
                    "description": "The timestamp when the event was created."
                  },
                  "payload": {
                    "type": "object",
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the scheduled event host."
                      },
                      "user_id": {
                        "type": "string",
                        "description": "The user ID of the scheduled event host."
                      },
                      "object": {
                        "type": "object",
                        "description": "The event information.",
                        "properties": {
                          "invitee_email": {
                            "type": "string",
                            "description": "The email address of the user who scheduled the event.",
                            "example": "example@example.com"
                          },
                          "invitee_first_name": {
                            "type": "string",
                            "description": "The first name of the user who scheduled the event.",
                            "example": "alax"
                          },
                          "invitee_last_name": {
                            "type": "string",
                            "description": "The last name of the user who scheduled the event.",
                            "example": "bbbb"
                          },
                          "manage_url": {
                            "type": "string",
                            "description": "The URL for the user to reschedule or cancel the scheduled event.",
                            "example": "https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx"
                          },
                          "time_zone": {
                            "type": "string",
                            "description": "The user's time zone.",
                            "example": "Asia/Shanghai"
                          },
                          "questions_and_answers": {
                            "type": "array",
                            "description": "The collection of questions and answers from the booking form.",
                            "items": {
                              "type": "object",
                              "required": [
                                "question",
                                "position",
                                "answer"
                              ],
                              "properties": {
                                "question": {
                                  "type": "string",
                                  "description": "The question from the booking page.",
                                  "example": "Please share anything that will help prepare for our meeting."
                                },
                                "position": {
                                  "type": "integer",
                                  "description": "The index of the question."
                                },
                                "answer": {
                                  "description": "The answers to the question.",
                                  "type": "array",
                                  "items": {
                                    "type": "string",
                                    "description": "The answer to the question."
                                  }
                                }
                              }
                            }
                          },
                          "scheduled_event": {
                            "type": "object",
                            "description": "The scheduled event information.",
                            "properties": {
                              "event_id": {
                                "type": "string",
                                "description": "The unique identifier of the event.",
                                "example": "woft7torlatbw8ek24bmit5k60"
                              },
                              "attendees": {
                                "type": "array",
                                "description": "The attendees of the event.",
                                "items": {
                                  "type": "object",
                                  "properties": {
                                    "display_name": {
                                      "type": "string",
                                      "description": "The attendee's name.",
                                      "example": "bob"
                                    },
                                    "email": {
                                      "type": "string",
                                      "description": "The attendee's email address.",
                                      "example": "abc@zoom.us"
                                    }
                                  }
                                }
                              },
                              "invitee_counters": {
                                "type": "object",
                                "description": "The event's guest counters.",
                                "properties": {
                                  "total": {
                                    "type": "integer",
                                    "format": "int64",
                                    "description": "The total count of invitees in the event."
                                  },
                                  "active": {
                                    "type": "integer",
                                    "format": "int64",
                                    "description": "The count of active invitees in the event."
                                  },
                                  "limit": {
                                    "type": "integer",
                                    "format": "int64",
                                    "description": "The maximum number of invitees allowed for the event."
                                  }
                                }
                              },
                              "end_date_time": {
                                "type": "string",
                                "description": "The scheduled event's end date and time.",
                                "example": "2023-12-21T16:30:00Z"
                              },
                              "guests": {
                                "type": "array",
                                "description": "The collection of guests.",
                                "items": {
                                  "type": "string",
                                  "description": "The guest's email address.",
                                  "example": "tom@zoom.us"
                                }
                              },
                              "location_kind": {
                                "type": "string",
                                "description": "Unified location type for both location and external_location objects. If location type is online meeting, location info is in external_location object. If not, location info is in location object.",
                                "example": "zoom",
                                "enum": [
                                  "custom",
                                  "in-person_meeting",
                                  "outbound_call",
                                  "inbound_call",
                                  "invitee_specified",
                                  "zoom",
                                  "google_meet",
                                  "microsoft_teams"
                                ]
                              },
                              "location": {
                                "oneOf": [
                                  {
                                    "deprecated": true,
                                    "type": "object",
                                    "title": "zoom",
                                    "description": "All the information about the Zoom meeting.",
                                    "properties": {
                                      "zoom_join_url": {
                                        "maxLength": 1024,
                                        "type": "string",
                                        "description": "The URL to join a meeting.",
                                        "example": "https://success.zoom.us/j/xxxxxxxx?pwd=xxxxxxxx"
                                      },
                                      "location_type": {
                                        "type": "string",
                                        "description": "Whether it's a Zoom meeting.",
                                        "enum": [
                                          "zoom"
                                        ]
                                      },
                                      "zoom_meeting_data": {
                                        "type": "object",
                                        "description": "The extra info for the meeting.",
                                        "properties": {
                                          "zoom_meeting_id": {
                                            "maxLength": 1024,
                                            "type": "string",
                                            "description": "The unique identifier of the meeting in string format. This field is also known as the meeting number.",
                                            "example": "1234567890"
                                          },
                                          "zoom_meeting_passcode": {
                                            "maxLength": 1024,
                                            "type": "string",
                                            "description": "The passcode of the meeting.",
                                            "example": "dnjchd"
                                          },
                                          "zoom_personal_meeting_id": {
                                            "maxLength": 1024,
                                            "type": "string",
                                            "description": "The personal meeting ID in string format.",
                                            "example": "98656786887"
                                          }
                                        }
                                      },
                                      "zoom_meeting_status": {
                                        "type": "string",
                                        "description": "The status of meeting.",
                                        "enum": [
                                          "scheduled",
                                          "failed"
                                        ]
                                      }
                                    }
                                  },
                                  {
                                    "type": "object",
                                    "title": "Non-online meeting location",
                                    "description": "All the information about the Non-online meeting location.",
                                    "properties": {
                                      "custom_location": {
                                        "maxLength": 1024,
                                        "type": "string",
                                        "description": "The information for a location. May be a phone number or a physical location.",
                                        "example": "AAA office"
                                      },
                                      "location_type": {
                                        "type": "string",
                                        "description": "The location type.",
                                        "enum": [
                                          "custom",
                                          "in-person_meeting",
                                          "outbound_call",
                                          "inbound_call",
                                          "invitee_specified"
                                        ]
                                      },
                                      "additional_info": {
                                        "type": "string",
                                        "description": "Additional info for location.",
                                        "example": "extra info"
                                      }
                                    }
                                  }
                                ]
                              },
                              "external_location": {
                                "description": "The meeting details in the scheduled event.",
                                "readOnly": true,
                                "oneOf": [
                                  {
                                    "type": "object",
                                    "properties": {
                                      "kind": {
                                        "type": "string",
                                        "description": "The type of meeting specified in the schedule.",
                                        "example": "zoom",
                                        "enum": [
                                          "zoom",
                                          "google_meet",
                                          "microsoft_teams"
                                        ]
                                      },
                                      "meeting_id": {
                                        "type": "string",
                                        "description": "The unique identifier of the meeting in string format.",
                                        "example": "94777886776"
                                      },
                                      "personal_meeting_id": {
                                        "type": "string",
                                        "description": "The personal meeting ID in string format.",
                                        "example": "98656786887"
                                      },
                                      "meeting_passcode": {
                                        "type": "string",
                                        "description": "The meeting passcode.",
                                        "example": "958000"
                                      },
                                      "meeting_description": {
                                        "type": "string",
                                        "description": "The Zoom meeting description.",
                                        "example": "Invite you to meeting"
                                      },
                                      "meeting_join_url": {
                                        "type": "string",
                                        "description": "The URL to join the meeting.",
                                        "example": "https://zoom.us/j/7427752805"
                                      }
                                    }
                                  }
                                ]
                              },
                              "start_date_time": {
                                "type": "string",
                                "description": "The scheduled event's start date and time.",
                                "example": "2023-12-21T16:00:00Z"
                              },
                              "status": {
                                "type": "string",
                                "description": "The event status: confirmed or cancelled.",
                                "example": "confirmed",
                                "enum": [
                                  "confirmed",
                                  "cancelled"
                                ]
                              },
                              "summary": {
                                "maxLength": 256,
                                "type": "string",
                                "description": "The schedule name.",
                                "example": "daily meeting"
                              },
                              "updated": {
                                "type": "string",
                                "description": "The moment the scheduled event was updated.",
                                "readOnly": true,
                                "example": "2023-12-21T06:18:32.087Z"
                              },
                              "created": {
                                "type": "string",
                                "description": "The moment the scheduled event was created.",
                                "readOnly": true,
                                "example": "2023-12-21T06:18:32.087Z"
                              },
                              "meeting_notes": {
                                "maxLength": 1024,
                                "type": "string",
                                "description": "The scheduled event's meeting notes.",
                                "example": "meeting notes"
                              }
                            },
                            "additionalProperties": false
                          },
                          "tracking": {
                            "type": "object",
                            "description": "The information to track the invitee source. This field is only present when UTM parameters are added.",
                            "properties": {
                              "utm_source": {
                                "type": "string",
                                "description": "This field identifies which site sent the traffic and is a required parameter.",
                                "example": "zoom"
                              },
                              "utm_medium": {
                                "type": "string",
                                "description": "This field identifies what type of link was used, such as pay-per-click or email.",
                                "example": "pay-per-clik"
                              },
                              "utm_campaign": {
                                "type": "string",
                                "description": "This field identifies a specific product promotion or strategic campaign.",
                                "example": "spring_sale"
                              },
                              "utm_content": {
                                "type": "string",
                                "description": "This field identifies what specifically was clicked to bring the user to the site, such as a banner ad or a text link. It is often used for A/B testing and content-targeted ads.",
                                "example": "schedule link"
                              },
                              "utm_term": {
                                "type": "string",
                                "description": "This field identifies the search terms.",
                                "example": "learning"
                              },
                              "salesforce_ids": {
                                "type": "array",
                                "description": "The tracking of Salesforce IDs.",
                                "items": {
                                  "type": "object",
                                  "property": {
                                    "type": "string",
                                    "description": "The Salesforce object ID that represents an entity."
                                  }
                                }
                              }
                            }
                          },
                          "rescheduled": {
                            "type": "boolean",
                            "description": "Indicates whether this is from a reschedule request."
                          },
                          "created": {
                            "type": "string",
                            "description": "The moment the scheduled request was created.",
                            "readOnly": true,
                            "example": "2023-12-21T06:18:32.087Z"
                          },
                          "text_phone_number": {
                            "type": "string",
                            "description": "The phone number of the invitee to receive text reminders.",
                            "example": "+1 83798773"
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
                  "value": "{\n  \"event\": \"scheduler.scheduled_event_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"user_id\": \"dndhfdjdihd\",\n    \"object\": {\n      \"invitee_email\": \"john.doe@example.com\",\n      \"invitee_first_name\": \"John\",\n      \"invitee_last_name\": \"Doe\",\n      \"manage_url\": \"https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx\",\n      \"time_zone\": \"Asia/Shanghai\",\n      \"questions_and_answers\": [\n        {\n          \"question\": \"Please share anything that will help prepare for our meeting.\",\n          \"position\": 0,\n          \"answer\": [\n            \"First line\",\n            \"Second line\"\n          ]\n        }\n      ],\n      \"scheduled_event\": {\n        \"event_id\": \"woft7torlatbw8ek24bmit5k60\",\n        \"attendees\": [\n          {\n            \"display_name\": \"Jane Doe\",\n            \"email\": \"jane@example.com\"\n          },\n          {\n            \"display_name\": \"David Smith\",\n            \"email\": \"david@example.com\"\n          }\n        ],\n        \"end_date_time\": \"2023-12-21T16:30:00Z\",\n        \"guests\": [\n          \"guest1@example.com\",\n          \"guest2@example.com\"\n        ],\n        \"location_kind\": \"zoom\",\n        \"location\": {\n          \"custom_location\": \"123 Main St\",\n          \"location_type\": \"in-person_meeting\",\n          \"additional_info\": \"additional information\"\n        },\n        \"external_location\": {\n          \"kind\": \"zoom\",\n          \"meeting_id\": \"94777886776\",\n          \"personal_meeting_id\": \"98656786887\",\n          \"meeting_passcode\": \"958000\",\n          \"meeting_description\": \"Invite you to meeting\",\n          \"meeting_join_url\": \"https://zoom.us/j/7427752805\"\n        },\n        \"start_date_time\": \"2023-12-21T16:00:00Z\",\n        \"status\": \"confirmed\",\n        \"summary\": \"30 minute meeting\",\n        \"updated\": \"2023-12-21T06:19:28.309Z\",\n        \"created\": \"2023-12-21T06:18:28.309Z\",\n        \"meeting_notes\": \"meeting notes\",\n        \"invitee_counters\": {\n          \"total\": 1,\n          \"active\": 1,\n          \"limit\": 3\n        }\n      },\n      \"tracking\": {\n        \"utm_source\": \"email\",\n        \"utm_medium\": \"newsletter\",\n        \"utm_campaign\": \"spring_sale\",\n        \"utm_content\": \"booking_link\",\n        \"utm_term\": \"shopping\",\n        \"salesforce_ids\": [\n          {\n            \"sf_person\": \"0035g00000AbCdEAAZ\"\n          },\n          {\n            \"sf_relates\": \"0015g00000AbCdEAAZ\"\n          }\n        ]\n      },\n      \"rescheduled\": false,\n      \"created\": \"2023-12-21T06:18:28.309Z\",\n      \"text_phone_number\": \"+1 83798773\"\n    }\n  }\n}"
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
                      "description": "The timestamp when the event was created."
                    },
                    "payload": {
                      "type": "object",
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the scheduled event host."
                        },
                        "user_id": {
                          "type": "string",
                          "description": "The user ID of the scheduled event host."
                        },
                        "object": {
                          "type": "object",
                          "description": "The event information.",
                          "properties": {
                            "invitee_email": {
                              "type": "string",
                              "description": "The email address of the user who scheduled the event.",
                              "example": "example@example.com"
                            },
                            "invitee_first_name": {
                              "type": "string",
                              "description": "The first name of the user who scheduled the event.",
                              "example": "alax"
                            },
                            "invitee_last_name": {
                              "type": "string",
                              "description": "The last name of the user who scheduled the event.",
                              "example": "bbbb"
                            },
                            "manage_url": {
                              "type": "string",
                              "description": "The URL for the user to reschedule or cancel the scheduled event.",
                              "example": "https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx"
                            },
                            "time_zone": {
                              "type": "string",
                              "description": "The user's time zone.",
                              "example": "Asia/Shanghai"
                            },
                            "questions_and_answers": {
                              "type": "array",
                              "description": "The collection of questions and answers from the booking form.",
                              "items": {
                                "type": "object",
                                "required": [
                                  "question",
                                  "position",
                                  "answer"
                                ],
                                "properties": {
                                  "question": {
                                    "type": "string",
                                    "description": "The question from the booking page.",
                                    "example": "Please share anything that will help prepare for our meeting."
                                  },
                                  "position": {
                                    "type": "integer",
                                    "description": "The index of the question."
                                  },
                                  "answer": {
                                    "description": "The answers to the question.",
                                    "type": "array",
                                    "items": {
                                      "type": "string",
                                      "description": "The answer to the question."
                                    }
                                  }
                                }
                              }
                            },
                            "scheduled_event": {
                              "type": "object",
                              "description": "The scheduled event information.",
                              "properties": {
                                "event_id": {
                                  "type": "string",
                                  "description": "The unique identifier of the event.",
                                  "example": "woft7torlatbw8ek24bmit5k60"
                                },
                                "attendees": {
                                  "type": "array",
                                  "description": "The attendees of the event.",
                                  "items": {
                                    "type": "object",
                                    "properties": {
                                      "display_name": {
                                        "type": "string",
                                        "description": "The attendee's name.",
                                        "example": "bob"
                                      },
                                      "email": {
                                        "type": "string",
                                        "description": "The attendee's email address.",
                                        "example": "abc@zoom.us"
                                      }
                                    }
                                  }
                                },
                                "invitee_counters": {
                                  "type": "object",
                                  "description": "The event's guest counters.",
                                  "properties": {
                                    "total": {
                                      "type": "integer",
                                      "format": "int64",
                                      "description": "The total count of invitees in the event."
                                    },
                                    "active": {
                                      "type": "integer",
                                      "format": "int64",
                                      "description": "The count of active invitees in the event."
                                    },
                                    "limit": {
                                      "type": "integer",
                                      "format": "int64",
                                      "description": "The maximum number of invitees allowed for the event."
                                    }
                                  }
                                },
                                "end_date_time": {
                                  "type": "string",
                                  "description": "The scheduled event's end date and time.",
                                  "example": "2023-12-21T16:30:00Z"
                                },
                                "guests": {
                                  "type": "array",
                                  "description": "The collection of guests.",
                                  "items": {
                                    "type": "string",
                                    "description": "The guest's email address.",
                                    "example": "tom@zoom.us"
                                  }
                                },
                                "location_kind": {
                                  "type": "string",
                                  "description": "Unified location type for both location and external_location objects. If location type is online meeting, location info is in external_location object. If not, location info is in location object.",
                                  "example": "zoom",
                                  "enum": [
                                    "custom",
                                    "in-person_meeting",
                                    "outbound_call",
                                    "inbound_call",
                                    "invitee_specified",
                                    "zoom",
                                    "google_meet",
                                    "microsoft_teams"
                                  ]
                                },
                                "location": {
                                  "oneOf": [
                                    {
                                      "deprecated": true,
                                      "type": "object",
                                      "title": "zoom",
                                      "description": "All the information about the Zoom meeting.",
                                      "properties": {
                                        "zoom_join_url": {
                                          "maxLength": 1024,
                                          "type": "string",
                                          "description": "The URL to join a meeting.",
                                          "example": "https://success.zoom.us/j/xxxxxxxx?pwd=xxxxxxxx"
                                        },
                                        "location_type": {
                                          "type": "string",
                                          "description": "Whether it's a Zoom meeting.",
                                          "enum": [
                                            "zoom"
                                          ]
                                        },
                                        "zoom_meeting_data": {
                                          "type": "object",
                                          "description": "The extra info for the meeting.",
                                          "properties": {
                                            "zoom_meeting_id": {
                                              "maxLength": 1024,
                                              "type": "string",
                                              "description": "The unique identifier of the meeting in string format. This field is also known as the meeting number.",
                                              "example": "1234567890"
                                            },
                                            "zoom_meeting_passcode": {
                                              "maxLength": 1024,
                                              "type": "string",
                                              "description": "The passcode of the meeting.",
                                              "example": "dnjchd"
                                            },
                                            "zoom_personal_meeting_id": {
                                              "maxLength": 1024,
                                              "type": "string",
                                              "description": "The personal meeting ID in string format.",
                                              "example": "98656786887"
                                            }
                                          }
                                        },
                                        "zoom_meeting_status": {
                                          "type": "string",
                                          "description": "The status of meeting.",
                                          "enum": [
                                            "scheduled",
                                            "failed"
                                          ]
                                        }
                                      }
                                    },
                                    {
                                      "type": "object",
                                      "title": "Non-online meeting location",
                                      "description": "All the information about the Non-online meeting location.",
                                      "properties": {
                                        "custom_location": {
                                          "maxLength": 1024,
                                          "type": "string",
                                          "description": "The information for a location. May be a phone number or a physical location.",
                                          "example": "AAA office"
                                        },
                                        "location_type": {
                                          "type": "string",
                                          "description": "The location type.",
                                          "enum": [
                                            "custom",
                                            "in-person_meeting",
                                            "outbound_call",
                                            "inbound_call",
                                            "invitee_specified"
                                          ]
                                        },
                                        "additional_info": {
                                          "type": "string",
                                          "description": "Additional info for location.",
                                          "example": "extra info"
                                        }
                                      }
                                    }
                                  ]
                                },
                                "external_location": {
                                  "description": "The meeting details in the scheduled event.",
                                  "readOnly": true,
                                  "oneOf": [
                                    {
                                      "type": "object",
                                      "properties": {
                                        "kind": {
                                          "type": "string",
                                          "description": "The type of meeting specified in the schedule.",
                                          "example": "zoom",
                                          "enum": [
                                            "zoom",
                                            "google_meet",
                                            "microsoft_teams"
                                          ]
                                        },
                                        "meeting_id": {
                                          "type": "string",
                                          "description": "The unique identifier of the meeting in string format.",
                                          "example": "94777886776"
                                        },
                                        "personal_meeting_id": {
                                          "type": "string",
                                          "description": "The personal meeting ID in string format.",
                                          "example": "98656786887"
                                        },
                                        "meeting_passcode": {
                                          "type": "string",
                                          "description": "The meeting passcode.",
                                          "example": "958000"
                                        },
                                        "meeting_description": {
                                          "type": "string",
                                          "description": "The Zoom meeting description.",
                                          "example": "Invite you to meeting"
                                        },
                                        "meeting_join_url": {
                                          "type": "string",
                                          "description": "The URL to join the meeting.",
                                          "example": "https://zoom.us/j/7427752805"
                                        }
                                      }
                                    }
                                  ]
                                },
                                "start_date_time": {
                                  "type": "string",
                                  "description": "The scheduled event's start date and time.",
                                  "example": "2023-12-21T16:00:00Z"
                                },
                                "status": {
                                  "type": "string",
                                  "description": "The event status: confirmed or cancelled.",
                                  "example": "confirmed",
                                  "enum": [
                                    "confirmed",
                                    "cancelled"
                                  ]
                                },
                                "summary": {
                                  "maxLength": 256,
                                  "type": "string",
                                  "description": "The schedule name.",
                                  "example": "daily meeting"
                                },
                                "updated": {
                                  "type": "string",
                                  "description": "The moment the scheduled event was updated.",
                                  "readOnly": true,
                                  "example": "2023-12-21T06:18:32.087Z"
                                },
                                "created": {
                                  "type": "string",
                                  "description": "The moment the scheduled event was created.",
                                  "readOnly": true,
                                  "example": "2023-12-21T06:18:32.087Z"
                                },
                                "meeting_notes": {
                                  "maxLength": 1024,
                                  "type": "string",
                                  "description": "The scheduled event's meeting notes.",
                                  "example": "meeting notes"
                                }
                              },
                              "additionalProperties": false
                            },
                            "tracking": {
                              "type": "object",
                              "description": "The information to track the invitee source. This field is only present when UTM parameters are added.",
                              "properties": {
                                "utm_source": {
                                  "type": "string",
                                  "description": "This field identifies which site sent the traffic and is a required parameter.",
                                  "example": "zoom"
                                },
                                "utm_medium": {
                                  "type": "string",
                                  "description": "This field identifies what type of link was used, such as pay-per-click or email.",
                                  "example": "pay-per-clik"
                                },
                                "utm_campaign": {
                                  "type": "string",
                                  "description": "This field identifies a specific product promotion or strategic campaign.",
                                  "example": "spring_sale"
                                },
                                "utm_content": {
                                  "type": "string",
                                  "description": "This field identifies what specifically was clicked to bring the user to the site, such as a banner ad or a text link. It is often used for A/B testing and content-targeted ads.",
                                  "example": "schedule link"
                                },
                                "utm_term": {
                                  "type": "string",
                                  "description": "This field identifies the search terms.",
                                  "example": "learning"
                                },
                                "salesforce_ids": {
                                  "type": "array",
                                  "description": "The tracking of Salesforce IDs.",
                                  "items": {
                                    "type": "object",
                                    "property": {
                                      "type": "string",
                                      "description": "The Salesforce object ID that represents an entity."
                                    }
                                  }
                                }
                              }
                            },
                            "rescheduled": {
                              "type": "boolean",
                              "description": "Indicates whether this is from a reschedule request."
                            },
                            "created": {
                              "type": "string",
                              "description": "The moment the scheduled request was created.",
                              "readOnly": true,
                              "example": "2023-12-21T06:18:32.087Z"
                            },
                            "text_phone_number": {
                              "type": "string",
                              "description": "The phone number of the invitee to receive text reminders.",
                              "example": "+1 83798773"
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
                    "value": "{\n  \"event\": \"scheduler.scheduled_event_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"user_id\": \"dndhfdjdihd\",\n    \"object\": {\n      \"invitee_email\": \"john.doe@example.com\",\n      \"invitee_first_name\": \"John\",\n      \"invitee_last_name\": \"Doe\",\n      \"manage_url\": \"https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx\",\n      \"time_zone\": \"Asia/Shanghai\",\n      \"questions_and_answers\": [\n        {\n          \"question\": \"Please share anything that will help prepare for our meeting.\",\n          \"position\": 0,\n          \"answer\": [\n            \"First line\",\n            \"Second line\"\n          ]\n        }\n      ],\n      \"scheduled_event\": {\n        \"event_id\": \"woft7torlatbw8ek24bmit5k60\",\n        \"attendees\": [\n          {\n            \"display_name\": \"Jane Doe\",\n            \"email\": \"jane@example.com\"\n          },\n          {\n            \"display_name\": \"David Smith\",\n            \"email\": \"david@example.com\"\n          }\n        ],\n        \"end_date_time\": \"2023-12-21T16:30:00Z\",\n        \"guests\": [\n          \"guest1@example.com\",\n          \"guest2@example.com\"\n        ],\n        \"location_kind\": \"zoom\",\n        \"location\": {\n          \"custom_location\": \"123 Main St\",\n          \"location_type\": \"in-person_meeting\",\n          \"additional_info\": \"additional information\"\n        },\n        \"external_location\": {\n          \"kind\": \"zoom\",\n          \"meeting_id\": \"94777886776\",\n          \"personal_meeting_id\": \"98656786887\",\n          \"meeting_passcode\": \"958000\",\n          \"meeting_description\": \"Invite you to meeting\",\n          \"meeting_join_url\": \"https://zoom.us/j/7427752805\"\n        },\n        \"start_date_time\": \"2023-12-21T16:00:00Z\",\n        \"status\": \"confirmed\",\n        \"summary\": \"30 minute meeting\",\n        \"updated\": \"2023-12-21T06:19:28.309Z\",\n        \"created\": \"2023-12-21T06:18:28.309Z\",\n        \"meeting_notes\": \"meeting notes\",\n        \"invitee_counters\": {\n          \"total\": 1,\n          \"active\": 1,\n          \"limit\": 3\n        }\n      },\n      \"tracking\": {\n        \"utm_source\": \"email\",\n        \"utm_medium\": \"newsletter\",\n        \"utm_campaign\": \"spring_sale\",\n        \"utm_content\": \"booking_link\",\n        \"utm_term\": \"shopping\",\n        \"salesforce_ids\": [\n          {\n            \"sf_person\": \"0035g00000AbCdEAAZ\"\n          },\n          {\n            \"sf_relates\": \"0015g00000AbCdEAAZ\"\n          }\n        ]\n      },\n      \"rescheduled\": false,\n      \"created\": \"2023-12-21T06:18:28.309Z\",\n      \"text_phone_number\": \"+1 83798773\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "scheduler.scheduled_event_canceled": {
      "post": {
        "tags": [
          "scheduler"
        ],
        "operationId": "scheduler.scheduled_event_canceled",
        "requestBody": {
          "description": "# scheduler.scheduled_event_canceled\nThe **Scheduled event canceled** event triggers each time the host or booker cancels the scheduled event.\n\n## Prerequisites\nEnable **Event subscriptions** for your Marketplace app with:\n\n* A valid **Event notification endpoint URL**.\n* Enable the **Scheduled event canceled** subscription under the **Scheduler** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `scheduler:read:admin`,`scheduler:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `scheduler:read:scheduled_event`,`scheduler:read:scheduled_event:admin`\n\n**Event type**: `scheduler.scheduled_event_canceled`\n",
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
                    "description": "The timestamp when the event was created."
                  },
                  "payload": {
                    "type": "object",
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the scheduled event host."
                      },
                      "user_id": {
                        "type": "string",
                        "description": "The user ID of the scheduled event host."
                      },
                      "object": {
                        "type": "object",
                        "description": "The information about the event.",
                        "properties": {
                          "invitee_email": {
                            "type": "string",
                            "description": "The email address of the user who scheduled the event.",
                            "example": "example@example.com"
                          },
                          "invitee_first_name": {
                            "type": "string",
                            "description": "The first name of the user who scheduled the event.",
                            "example": "alax"
                          },
                          "invitee_last_name": {
                            "type": "string",
                            "description": "The last name of the user who scheduled the event.",
                            "example": "bbbb"
                          },
                          "manage_url": {
                            "type": "string",
                            "description": "The URL for the user to reschedule or cancel the scheduled event.",
                            "example": "https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx"
                          },
                          "cancellation": {
                            "type": "object",
                            "description": "The scheduled event cancellation information.",
                            "properties": {
                              "canceled_by": {
                                "type": "string",
                                "description": "The name or email address of the canceler.",
                                "example": "aa@bb.c"
                              },
                              "canceler_type": {
                                "type": "string",
                                "description": "The user type of the person who canceled.",
                                "example": "host",
                                "enum": [
                                  "host",
                                  "invitee"
                                ]
                              },
                              "cancel_at": {
                                "type": "string",
                                "description": "The date and time when this scheduled event was canceled.",
                                "example": "2023-12-21T16:30:00+08:00"
                              },
                              "reason": {
                                "type": "string",
                                "description": "The reason for canceling this scheduled event.",
                                "example": "Time conflict"
                              }
                            }
                          },
                          "time_zone": {
                            "type": "string",
                            "description": "The user's time zone.",
                            "example": "Asia/Shanghai"
                          },
                          "questions_and_answers": {
                            "type": "array",
                            "description": "All questions and answers.",
                            "items": {
                              "type": "object",
                              "required": [
                                "question",
                                "position",
                                "answer"
                              ],
                              "properties": {
                                "question": {
                                  "type": "string",
                                  "description": "The question on the booking page.",
                                  "example": "Please share anything that will help prepare for our meeting."
                                },
                                "position": {
                                  "type": "integer",
                                  "description": "The index of the question."
                                },
                                "answer": {
                                  "description": "The answers to the question.",
                                  "type": "array",
                                  "items": {
                                    "type": "string",
                                    "description": "The answer to the question."
                                  }
                                }
                              }
                            }
                          },
                          "scheduled_event": {
                            "type": "object",
                            "description": "The information about the scheduled event.",
                            "properties": {
                              "event_id": {
                                "type": "string",
                                "description": "The unique identifier of the event.",
                                "example": "woft7torlatbw8ek24bmit5k60"
                              },
                              "attendees": {
                                "type": "array",
                                "description": "The hosts of the scheduled event.",
                                "items": {
                                  "type": "object",
                                  "properties": {
                                    "display_name": {
                                      "type": "string",
                                      "description": "The attendee's name.",
                                      "example": "bob"
                                    },
                                    "email": {
                                      "type": "string",
                                      "description": "The attendee's email address.",
                                      "example": "abc@zoom.us"
                                    }
                                  }
                                }
                              },
                              "invitee_counters": {
                                "type": "object",
                                "description": "The event's guest counters.",
                                "properties": {
                                  "total": {
                                    "type": "integer",
                                    "format": "int64",
                                    "description": "The total count of invitees in the event."
                                  },
                                  "active": {
                                    "type": "integer",
                                    "format": "int64",
                                    "description": "The count of active invitees in the event."
                                  },
                                  "limit": {
                                    "type": "integer",
                                    "format": "int64",
                                    "description": "The maximum limit of invitees in the event."
                                  }
                                }
                              },
                              "end_date_time": {
                                "type": "string",
                                "description": "The scheduled event's end date and time.",
                                "example": "2023-12-21T16:30:00Z"
                              },
                              "guests": {
                                "type": "array",
                                "description": "The collection of guests.",
                                "items": {
                                  "type": "string",
                                  "description": "The guest's email address.",
                                  "example": "tom@zoom.us"
                                }
                              },
                              "location_kind": {
                                "type": "string",
                                "description": "Unified location type for both location and external_location objects. If location type is online meeting, location info is in external_location object. If not, location info is in location object.",
                                "example": "zoom",
                                "enum": [
                                  "custom",
                                  "in-person_meeting",
                                  "outbound_call",
                                  "inbound_call",
                                  "invitee_specified",
                                  "zoom",
                                  "google_meet",
                                  "microsoft_teams"
                                ]
                              },
                              "location": {
                                "oneOf": [
                                  {
                                    "deprecated": true,
                                    "type": "object",
                                    "title": "zoom",
                                    "description": "All the information about the Zoom meeting.",
                                    "properties": {
                                      "zoom_join_url": {
                                        "maxLength": 1024,
                                        "type": "string",
                                        "description": "The URL to join a meeting.",
                                        "example": "https://success.zoom.us/j/xxxxxxxx?pwd=xxxxxxxx"
                                      },
                                      "location_type": {
                                        "type": "string",
                                        "description": "Whether it's a Zoom meeting.",
                                        "enum": [
                                          "zoom"
                                        ]
                                      },
                                      "zoom_meeting_data": {
                                        "type": "object",
                                        "description": "The additional information for the meeting.",
                                        "properties": {
                                          "zoom_meeting_id": {
                                            "type": "string",
                                            "maxLength": 1024,
                                            "description": "The unique identifier of the meeting in string format. This field is also known as the meeting number.",
                                            "example": "1234567890"
                                          },
                                          "zoom_meeting_passcode": {
                                            "maxLength": 1024,
                                            "type": "string",
                                            "description": "The passcode for the meeting.",
                                            "example": "dnjchd"
                                          },
                                          "zoom_personal_meeting_id": {
                                            "maxLength": 1024,
                                            "type": "string",
                                            "description": "The personal meeting ID in string format.",
                                            "example": "98656786887"
                                          }
                                        }
                                      },
                                      "zoom_meeting_status": {
                                        "type": "string",
                                        "description": "The status of the meeting.",
                                        "enum": [
                                          "scheduled",
                                          "failed"
                                        ]
                                      }
                                    }
                                  },
                                  {
                                    "type": "object",
                                    "title": "Non-online meeting location",
                                    "description": "All the information about the non-online meeting location.",
                                    "properties": {
                                      "custom_location": {
                                        "maxLength": 1024,
                                        "type": "string",
                                        "description": "The information for a location. May be a phone number or a physical location.",
                                        "example": "AAA office"
                                      },
                                      "location_type": {
                                        "type": "string",
                                        "description": "The location type.",
                                        "enum": [
                                          "custom",
                                          "in-person_meeting",
                                          "outbound_call",
                                          "inbound_call",
                                          "invitee_specified"
                                        ]
                                      },
                                      "additional_info": {
                                        "type": "string",
                                        "description": "Additional information for the location.",
                                        "example": "extra info"
                                      }
                                    }
                                  }
                                ]
                              },
                              "external_location": {
                                "description": "The meeting details in the scheduled event.",
                                "readOnly": true,
                                "oneOf": [
                                  {
                                    "type": "object",
                                    "properties": {
                                      "kind": {
                                        "type": "string",
                                        "description": "The type of meeting specified in the schedule.",
                                        "example": "zoom",
                                        "enum": [
                                          "zoom",
                                          "google_meet",
                                          "microsoft_teams"
                                        ]
                                      },
                                      "meeting_id": {
                                        "type": "string",
                                        "description": "The unique identifier of the meeting in string format.",
                                        "example": "94777886776"
                                      },
                                      "personal_meeting_id": {
                                        "type": "string",
                                        "description": "The personal meeting ID in string format.",
                                        "example": "98656786887"
                                      },
                                      "meeting_passcode": {
                                        "type": "string",
                                        "description": "The passcode for the meeting.",
                                        "example": "958000"
                                      },
                                      "meeting_description": {
                                        "type": "string",
                                        "description": "The description of the Zoom meeting.",
                                        "example": "Invite you to meeting"
                                      },
                                      "meeting_join_url": {
                                        "type": "string",
                                        "description": "The URL to join the meeting.",
                                        "example": "https://zoom.us/j/7427752805"
                                      }
                                    }
                                  }
                                ]
                              },
                              "start_date_time": {
                                "type": "string",
                                "description": "The scheduled event's start date and time.",
                                "example": "2023-12-21T16:00:00Z"
                              },
                              "status": {
                                "type": "string",
                                "description": "The status of the event: confirmed or cancelled.",
                                "example": "confirmed",
                                "enum": [
                                  "confirmed",
                                  "cancelled"
                                ]
                              },
                              "summary": {
                                "maxLength": 256,
                                "type": "string",
                                "description": "The name of the schedule.",
                                "example": "daily meeting"
                              },
                              "updated": {
                                "type": "string",
                                "description": "The moment the scheduled event was updated.",
                                "readOnly": true,
                                "example": "2023-12-21T06:18:32.087Z"
                              },
                              "created": {
                                "type": "string",
                                "description": "The moment the scheduled event was created.",
                                "readOnly": true,
                                "example": "2023-12-21T06:18:32.087Z"
                              },
                              "meeting_notes": {
                                "maxLength": 1024,
                                "type": "string",
                                "description": "The meeting notes for the scheduled event.",
                                "example": "meeting notes"
                              }
                            },
                            "additionalProperties": false
                          },
                          "tracking": {
                            "type": "object",
                            "description": "The information to track the source of the invitee.",
                            "properties": {
                              "utm_source": {
                                "type": "string",
                                "description": "This field identifies which site sent the traffic, and is a required parameter.",
                                "example": "zoom"
                              },
                              "utm_medium": {
                                "type": "string",
                                "description": "This field identifies what type of link was used, such as pay-per-click or email.",
                                "example": "pay-per-clik"
                              },
                              "utm_campaign": {
                                "type": "string",
                                "description": "This field identifies a specific product promotion or strategic campaign.",
                                "example": "spring_sale"
                              },
                              "utm_content": {
                                "type": "string",
                                "description": "This field identifies what specifically was clicked to bring the user to the site, such as a banner ad or a text link. It is often used for A/B testing and content-targeted ads.",
                                "example": "schedule link"
                              },
                              "utm_term": {
                                "type": "string",
                                "description": "This field identifies search terms.",
                                "example": "learning"
                              },
                              "salesforce_ids": {
                                "type": "array",
                                "description": "The tracking of Salesforce IDs.",
                                "items": {
                                  "type": "object",
                                  "property": {
                                    "type": "string",
                                    "description": "The Salesforce object ID that represents an entity."
                                  }
                                }
                              }
                            }
                          },
                          "rescheduled": {
                            "type": "boolean",
                            "description": "Whether it's from a reschedule request."
                          },
                          "created": {
                            "type": "string",
                            "description": "The moment the scheduled request was created.",
                            "readOnly": true,
                            "example": "2023-12-21T06:18:32.087Z"
                          },
                          "text_phone_number": {
                            "type": "string",
                            "description": "The phone number of the invitee to receive text reminders.",
                            "example": "+1 83798773"
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
                  "value": "{\n  \"event\": \"scheduler.scheduled_event_canceled\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"user_id\": \"dndhfdjdihd\",\n    \"object\": {\n      \"invitee_email\": \"john.doe@example.com\",\n      \"invitee_first_name\": \"John\",\n      \"invitee_last_name\": \"Doe\",\n      \"manage_url\": \"https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx\",\n      \"cancellation\": {\n        \"canceled_by\": \"aa@bb.c\",\n        \"canceler_type\": \"host\",\n        \"cancel_at\": \"2023-12-21T16:30:00+08:00\",\n        \"reason\": \"Time conflict\"\n      },\n      \"time_zone\": \"Asia/Shanghai\",\n      \"questions_and_answers\": [\n        {\n          \"question\": \"Please share anything that will help prepare for our meeting.\",\n          \"position\": 0,\n          \"answer\": [\n            \"First line\",\n            \"Second line\"\n          ]\n        }\n      ],\n      \"scheduled_event\": {\n        \"event_id\": \"woft7torlatbw8ek24bmit5k60\",\n        \"attendees\": [\n          {\n            \"display_name\": \"Jane Doe\",\n            \"email\": \"jane@example.com\"\n          },\n          {\n            \"display_name\": \"David Smith\",\n            \"email\": \"david@example.com\"\n          }\n        ],\n        \"end_date_time\": \"2023-12-21T16:30:00Z\",\n        \"guests\": [\n          \"guest1@example.com\",\n          \"guest2@example.com\"\n        ],\n        \"location_kind\": \"custom\",\n        \"location\": {\n          \"custom_location\": \"123 Main St\",\n          \"location_type\": \"in-person_meeting\",\n          \"additional_info\": \"additional information\"\n        },\n        \"external_location\": {\n          \"kind\": \"zoom\",\n          \"meeting_id\": \"94777886776\",\n          \"personal_meeting_id\": \"98656786887\",\n          \"meeting_passcode\": \"958000\",\n          \"meeting_description\": \"Invite you to meeting\",\n          \"meeting_join_url\": \"https://zoom.us/j/7427752805\"\n        },\n        \"start_date_time\": \"2023-12-21T16:00:00Z\",\n        \"status\": \"confirmed\",\n        \"summary\": \"30 minute meeting\",\n        \"updated\": \"2023-12-21T06:19:28.309Z\",\n        \"created\": \"2023-12-21T06:18:28.309Z\",\n        \"meeting_notes\": \"meeting notes\",\n        \"invitee_counters\": {\n          \"total\": 1,\n          \"active\": 1,\n          \"limit\": 3\n        }\n      },\n      \"tracking\": {\n        \"utm_source\": \"email\",\n        \"utm_medium\": \"newsletter\",\n        \"utm_campaign\": \"spring_sale\",\n        \"utm_content\": \"booking_link\",\n        \"utm_term\": \"shopping\",\n        \"salesforce_ids\": [\n          {\n            \"sf_person\": \"0035g00000AbCdEAAZ\"\n          },\n          {\n            \"sf_relates\": \"0015g00000AbCdEAAZ\"\n          }\n        ]\n      },\n      \"rescheduled\": false,\n      \"created\": \"2023-12-21T06:18:28.309Z\",\n      \"text_phone_number\": \"+1 48837847\"\n    }\n  }\n}"
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
                      "description": "The timestamp when the event was created."
                    },
                    "payload": {
                      "type": "object",
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the scheduled event host."
                        },
                        "user_id": {
                          "type": "string",
                          "description": "The user ID of the scheduled event host."
                        },
                        "object": {
                          "type": "object",
                          "description": "The information about the event.",
                          "properties": {
                            "invitee_email": {
                              "type": "string",
                              "description": "The email address of the user who scheduled the event.",
                              "example": "example@example.com"
                            },
                            "invitee_first_name": {
                              "type": "string",
                              "description": "The first name of the user who scheduled the event.",
                              "example": "alax"
                            },
                            "invitee_last_name": {
                              "type": "string",
                              "description": "The last name of the user who scheduled the event.",
                              "example": "bbbb"
                            },
                            "manage_url": {
                              "type": "string",
                              "description": "The URL for the user to reschedule or cancel the scheduled event.",
                              "example": "https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx"
                            },
                            "cancellation": {
                              "type": "object",
                              "description": "The scheduled event cancellation information.",
                              "properties": {
                                "canceled_by": {
                                  "type": "string",
                                  "description": "The name or email address of the canceler.",
                                  "example": "aa@bb.c"
                                },
                                "canceler_type": {
                                  "type": "string",
                                  "description": "The user type of the person who canceled.",
                                  "example": "host",
                                  "enum": [
                                    "host",
                                    "invitee"
                                  ]
                                },
                                "cancel_at": {
                                  "type": "string",
                                  "description": "The date and time when this scheduled event was canceled.",
                                  "example": "2023-12-21T16:30:00+08:00"
                                },
                                "reason": {
                                  "type": "string",
                                  "description": "The reason for canceling this scheduled event.",
                                  "example": "Time conflict"
                                }
                              }
                            },
                            "time_zone": {
                              "type": "string",
                              "description": "The user's time zone.",
                              "example": "Asia/Shanghai"
                            },
                            "questions_and_answers": {
                              "type": "array",
                              "description": "All questions and answers.",
                              "items": {
                                "type": "object",
                                "required": [
                                  "question",
                                  "position",
                                  "answer"
                                ],
                                "properties": {
                                  "question": {
                                    "type": "string",
                                    "description": "The question on the booking page.",
                                    "example": "Please share anything that will help prepare for our meeting."
                                  },
                                  "position": {
                                    "type": "integer",
                                    "description": "The index of the question."
                                  },
                                  "answer": {
                                    "description": "The answers to the question.",
                                    "type": "array",
                                    "items": {
                                      "type": "string",
                                      "description": "The answer to the question."
                                    }
                                  }
                                }
                              }
                            },
                            "scheduled_event": {
                              "type": "object",
                              "description": "The information about the scheduled event.",
                              "properties": {
                                "event_id": {
                                  "type": "string",
                                  "description": "The unique identifier of the event.",
                                  "example": "woft7torlatbw8ek24bmit5k60"
                                },
                                "attendees": {
                                  "type": "array",
                                  "description": "The hosts of the scheduled event.",
                                  "items": {
                                    "type": "object",
                                    "properties": {
                                      "display_name": {
                                        "type": "string",
                                        "description": "The attendee's name.",
                                        "example": "bob"
                                      },
                                      "email": {
                                        "type": "string",
                                        "description": "The attendee's email address.",
                                        "example": "abc@zoom.us"
                                      }
                                    }
                                  }
                                },
                                "invitee_counters": {
                                  "type": "object",
                                  "description": "The event's guest counters.",
                                  "properties": {
                                    "total": {
                                      "type": "integer",
                                      "format": "int64",
                                      "description": "The total count of invitees in the event."
                                    },
                                    "active": {
                                      "type": "integer",
                                      "format": "int64",
                                      "description": "The count of active invitees in the event."
                                    },
                                    "limit": {
                                      "type": "integer",
                                      "format": "int64",
                                      "description": "The maximum limit of invitees in the event."
                                    }
                                  }
                                },
                                "end_date_time": {
                                  "type": "string",
                                  "description": "The scheduled event's end date and time.",
                                  "example": "2023-12-21T16:30:00Z"
                                },
                                "guests": {
                                  "type": "array",
                                  "description": "The collection of guests.",
                                  "items": {
                                    "type": "string",
                                    "description": "The guest's email address.",
                                    "example": "tom@zoom.us"
                                  }
                                },
                                "location_kind": {
                                  "type": "string",
                                  "description": "Unified location type for both location and external_location objects. If location type is online meeting, location info is in external_location object. If not, location info is in location object.",
                                  "example": "zoom",
                                  "enum": [
                                    "custom",
                                    "in-person_meeting",
                                    "outbound_call",
                                    "inbound_call",
                                    "invitee_specified",
                                    "zoom",
                                    "google_meet",
                                    "microsoft_teams"
                                  ]
                                },
                                "location": {
                                  "oneOf": [
                                    {
                                      "deprecated": true,
                                      "type": "object",
                                      "title": "zoom",
                                      "description": "All the information about the Zoom meeting.",
                                      "properties": {
                                        "zoom_join_url": {
                                          "maxLength": 1024,
                                          "type": "string",
                                          "description": "The URL to join a meeting.",
                                          "example": "https://success.zoom.us/j/xxxxxxxx?pwd=xxxxxxxx"
                                        },
                                        "location_type": {
                                          "type": "string",
                                          "description": "Whether it's a Zoom meeting.",
                                          "enum": [
                                            "zoom"
                                          ]
                                        },
                                        "zoom_meeting_data": {
                                          "type": "object",
                                          "description": "The additional information for the meeting.",
                                          "properties": {
                                            "zoom_meeting_id": {
                                              "type": "string",
                                              "maxLength": 1024,
                                              "description": "The unique identifier of the meeting in string format. This field is also known as the meeting number.",
                                              "example": "1234567890"
                                            },
                                            "zoom_meeting_passcode": {
                                              "maxLength": 1024,
                                              "type": "string",
                                              "description": "The passcode for the meeting.",
                                              "example": "dnjchd"
                                            },
                                            "zoom_personal_meeting_id": {
                                              "maxLength": 1024,
                                              "type": "string",
                                              "description": "The personal meeting ID in string format.",
                                              "example": "98656786887"
                                            }
                                          }
                                        },
                                        "zoom_meeting_status": {
                                          "type": "string",
                                          "description": "The status of the meeting.",
                                          "enum": [
                                            "scheduled",
                                            "failed"
                                          ]
                                        }
                                      }
                                    },
                                    {
                                      "type": "object",
                                      "title": "Non-online meeting location",
                                      "description": "All the information about the non-online meeting location.",
                                      "properties": {
                                        "custom_location": {
                                          "maxLength": 1024,
                                          "type": "string",
                                          "description": "The information for a location. May be a phone number or a physical location.",
                                          "example": "AAA office"
                                        },
                                        "location_type": {
                                          "type": "string",
                                          "description": "The location type.",
                                          "enum": [
                                            "custom",
                                            "in-person_meeting",
                                            "outbound_call",
                                            "inbound_call",
                                            "invitee_specified"
                                          ]
                                        },
                                        "additional_info": {
                                          "type": "string",
                                          "description": "Additional information for the location.",
                                          "example": "extra info"
                                        }
                                      }
                                    }
                                  ]
                                },
                                "external_location": {
                                  "description": "The meeting details in the scheduled event.",
                                  "readOnly": true,
                                  "oneOf": [
                                    {
                                      "type": "object",
                                      "properties": {
                                        "kind": {
                                          "type": "string",
                                          "description": "The type of meeting specified in the schedule.",
                                          "example": "zoom",
                                          "enum": [
                                            "zoom",
                                            "google_meet",
                                            "microsoft_teams"
                                          ]
                                        },
                                        "meeting_id": {
                                          "type": "string",
                                          "description": "The unique identifier of the meeting in string format.",
                                          "example": "94777886776"
                                        },
                                        "personal_meeting_id": {
                                          "type": "string",
                                          "description": "The personal meeting ID in string format.",
                                          "example": "98656786887"
                                        },
                                        "meeting_passcode": {
                                          "type": "string",
                                          "description": "The passcode for the meeting.",
                                          "example": "958000"
                                        },
                                        "meeting_description": {
                                          "type": "string",
                                          "description": "The description of the Zoom meeting.",
                                          "example": "Invite you to meeting"
                                        },
                                        "meeting_join_url": {
                                          "type": "string",
                                          "description": "The URL to join the meeting.",
                                          "example": "https://zoom.us/j/7427752805"
                                        }
                                      }
                                    }
                                  ]
                                },
                                "start_date_time": {
                                  "type": "string",
                                  "description": "The scheduled event's start date and time.",
                                  "example": "2023-12-21T16:00:00Z"
                                },
                                "status": {
                                  "type": "string",
                                  "description": "The status of the event: confirmed or cancelled.",
                                  "example": "confirmed",
                                  "enum": [
                                    "confirmed",
                                    "cancelled"
                                  ]
                                },
                                "summary": {
                                  "maxLength": 256,
                                  "type": "string",
                                  "description": "The name of the schedule.",
                                  "example": "daily meeting"
                                },
                                "updated": {
                                  "type": "string",
                                  "description": "The moment the scheduled event was updated.",
                                  "readOnly": true,
                                  "example": "2023-12-21T06:18:32.087Z"
                                },
                                "created": {
                                  "type": "string",
                                  "description": "The moment the scheduled event was created.",
                                  "readOnly": true,
                                  "example": "2023-12-21T06:18:32.087Z"
                                },
                                "meeting_notes": {
                                  "maxLength": 1024,
                                  "type": "string",
                                  "description": "The meeting notes for the scheduled event.",
                                  "example": "meeting notes"
                                }
                              },
                              "additionalProperties": false
                            },
                            "tracking": {
                              "type": "object",
                              "description": "The information to track the source of the invitee.",
                              "properties": {
                                "utm_source": {
                                  "type": "string",
                                  "description": "This field identifies which site sent the traffic, and is a required parameter.",
                                  "example": "zoom"
                                },
                                "utm_medium": {
                                  "type": "string",
                                  "description": "This field identifies what type of link was used, such as pay-per-click or email.",
                                  "example": "pay-per-clik"
                                },
                                "utm_campaign": {
                                  "type": "string",
                                  "description": "This field identifies a specific product promotion or strategic campaign.",
                                  "example": "spring_sale"
                                },
                                "utm_content": {
                                  "type": "string",
                                  "description": "This field identifies what specifically was clicked to bring the user to the site, such as a banner ad or a text link. It is often used for A/B testing and content-targeted ads.",
                                  "example": "schedule link"
                                },
                                "utm_term": {
                                  "type": "string",
                                  "description": "This field identifies search terms.",
                                  "example": "learning"
                                },
                                "salesforce_ids": {
                                  "type": "array",
                                  "description": "The tracking of Salesforce IDs.",
                                  "items": {
                                    "type": "object",
                                    "property": {
                                      "type": "string",
                                      "description": "The Salesforce object ID that represents an entity."
                                    }
                                  }
                                }
                              }
                            },
                            "rescheduled": {
                              "type": "boolean",
                              "description": "Whether it's from a reschedule request."
                            },
                            "created": {
                              "type": "string",
                              "description": "The moment the scheduled request was created.",
                              "readOnly": true,
                              "example": "2023-12-21T06:18:32.087Z"
                            },
                            "text_phone_number": {
                              "type": "string",
                              "description": "The phone number of the invitee to receive text reminders.",
                              "example": "+1 83798773"
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
                    "value": "{\n  \"event\": \"scheduler.scheduled_event_canceled\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"user_id\": \"dndhfdjdihd\",\n    \"object\": {\n      \"invitee_email\": \"john.doe@example.com\",\n      \"invitee_first_name\": \"John\",\n      \"invitee_last_name\": \"Doe\",\n      \"manage_url\": \"https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx\",\n      \"cancellation\": {\n        \"canceled_by\": \"aa@bb.c\",\n        \"canceler_type\": \"host\",\n        \"cancel_at\": \"2023-12-21T16:30:00+08:00\",\n        \"reason\": \"Time conflict\"\n      },\n      \"time_zone\": \"Asia/Shanghai\",\n      \"questions_and_answers\": [\n        {\n          \"question\": \"Please share anything that will help prepare for our meeting.\",\n          \"position\": 0,\n          \"answer\": [\n            \"First line\",\n            \"Second line\"\n          ]\n        }\n      ],\n      \"scheduled_event\": {\n        \"event_id\": \"woft7torlatbw8ek24bmit5k60\",\n        \"attendees\": [\n          {\n            \"display_name\": \"Jane Doe\",\n            \"email\": \"jane@example.com\"\n          },\n          {\n            \"display_name\": \"David Smith\",\n            \"email\": \"david@example.com\"\n          }\n        ],\n        \"end_date_time\": \"2023-12-21T16:30:00Z\",\n        \"guests\": [\n          \"guest1@example.com\",\n          \"guest2@example.com\"\n        ],\n        \"location_kind\": \"custom\",\n        \"location\": {\n          \"custom_location\": \"123 Main St\",\n          \"location_type\": \"in-person_meeting\",\n          \"additional_info\": \"additional information\"\n        },\n        \"external_location\": {\n          \"kind\": \"zoom\",\n          \"meeting_id\": \"94777886776\",\n          \"personal_meeting_id\": \"98656786887\",\n          \"meeting_passcode\": \"958000\",\n          \"meeting_description\": \"Invite you to meeting\",\n          \"meeting_join_url\": \"https://zoom.us/j/7427752805\"\n        },\n        \"start_date_time\": \"2023-12-21T16:00:00Z\",\n        \"status\": \"confirmed\",\n        \"summary\": \"30 minute meeting\",\n        \"updated\": \"2023-12-21T06:19:28.309Z\",\n        \"created\": \"2023-12-21T06:18:28.309Z\",\n        \"meeting_notes\": \"meeting notes\",\n        \"invitee_counters\": {\n          \"total\": 1,\n          \"active\": 1,\n          \"limit\": 3\n        }\n      },\n      \"tracking\": {\n        \"utm_source\": \"email\",\n        \"utm_medium\": \"newsletter\",\n        \"utm_campaign\": \"spring_sale\",\n        \"utm_content\": \"booking_link\",\n        \"utm_term\": \"shopping\",\n        \"salesforce_ids\": [\n          {\n            \"sf_person\": \"0035g00000AbCdEAAZ\"\n          },\n          {\n            \"sf_relates\": \"0015g00000AbCdEAAZ\"\n          }\n        ]\n      },\n      \"rescheduled\": false,\n      \"created\": \"2023-12-21T06:18:28.309Z\",\n      \"text_phone_number\": \"+1 48837847\"\n    }\n  }\n}"
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
    "title": "Scheduler",
    "description": "Scheduler webhooks allow developers to receive events for [Scheduler](https://developers.zoom.us/docs/zoom-scheduler/). ",
    "version": "1.0.0"
  },
  "webhooks": {
    "scheduler.scheduled_event_created": {
      "post": {
        "tags": [
          "scheduler"
        ],
        "operationId": "scheduler.scheduled_event_created",
        "requestBody": {
          "description": "# scheduler.scheduled_event_created\nThe **Scheduled event created** event triggers each time one user schedules an event with the scheduler host through the public booking page.\n\n## Prerequisites\nEnable **Event subscriptions** for your Marketplace app with:\n\n* A valid **Event notification endpoint URL**.\n* The **Scheduled event created** subscription is enabled under the **Scheduler** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `scheduler:read:admin`,`scheduler:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `scheduler:read:scheduled_event`,`scheduler:read:scheduled_event:admin`\n\n**Event type**: `scheduler.scheduled_event_created`\n",
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
                    "description": "The timestamp when the event was created."
                  },
                  "payload": {
                    "type": "object",
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the scheduled event host."
                      },
                      "user_id": {
                        "type": "string",
                        "description": "The user ID of the scheduled event host."
                      },
                      "object": {
                        "type": "object",
                        "description": "The event information.",
                        "properties": {
                          "invitee_email": {
                            "type": "string",
                            "description": "The email address of the user who scheduled the event.",
                            "example": "example@example.com"
                          },
                          "invitee_first_name": {
                            "type": "string",
                            "description": "The first name of the user who scheduled the event.",
                            "example": "alax"
                          },
                          "invitee_last_name": {
                            "type": "string",
                            "description": "The last name of the user who scheduled the event.",
                            "example": "bbbb"
                          },
                          "manage_url": {
                            "type": "string",
                            "description": "The URL for the user to reschedule or cancel the scheduled event.",
                            "example": "https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx"
                          },
                          "time_zone": {
                            "type": "string",
                            "description": "The user's time zone.",
                            "example": "Asia/Shanghai"
                          },
                          "questions_and_answers": {
                            "type": "array",
                            "description": "The collection of questions and answers from the booking form.",
                            "items": {
                              "type": "object",
                              "required": [
                                "question",
                                "position",
                                "answer"
                              ],
                              "properties": {
                                "question": {
                                  "type": "string",
                                  "description": "The question from the booking page.",
                                  "example": "Please share anything that will help prepare for our meeting."
                                },
                                "position": {
                                  "type": "integer",
                                  "description": "The index of the question."
                                },
                                "answer": {
                                  "description": "The answers to the question.",
                                  "type": "array",
                                  "items": {
                                    "type": "string",
                                    "description": "The answer to the question."
                                  }
                                }
                              }
                            }
                          },
                          "scheduled_event": {
                            "type": "object",
                            "description": "The scheduled event information.",
                            "properties": {
                              "event_id": {
                                "type": "string",
                                "description": "The unique identifier of the event.",
                                "example": "woft7torlatbw8ek24bmit5k60"
                              },
                              "attendees": {
                                "type": "array",
                                "description": "The attendees of the event.",
                                "items": {
                                  "type": "object",
                                  "properties": {
                                    "display_name": {
                                      "type": "string",
                                      "description": "The attendee's name.",
                                      "example": "bob"
                                    },
                                    "email": {
                                      "type": "string",
                                      "description": "The attendee's email address.",
                                      "example": "abc@zoom.us"
                                    }
                                  }
                                }
                              },
                              "invitee_counters": {
                                "type": "object",
                                "description": "The event's guest counters.",
                                "properties": {
                                  "total": {
                                    "type": "integer",
                                    "format": "int64",
                                    "description": "The total count of invitees in the event."
                                  },
                                  "active": {
                                    "type": "integer",
                                    "format": "int64",
                                    "description": "The count of active invitees in the event."
                                  },
                                  "limit": {
                                    "type": "integer",
                                    "format": "int64",
                                    "description": "The maximum number of invitees allowed for the event."
                                  }
                                }
                              },
                              "end_date_time": {
                                "type": "string",
                                "description": "The scheduled event's end date and time.",
                                "example": "2023-12-21T16:30:00Z"
                              },
                              "guests": {
                                "type": "array",
                                "description": "The collection of guests.",
                                "items": {
                                  "type": "string",
                                  "description": "The guest's email address.",
                                  "example": "tom@zoom.us"
                                }
                              },
                              "location_kind": {
                                "type": "string",
                                "description": "Unified location type for both location and external_location objects. If location type is online meeting, location info is in external_location object. If not, location info is in location object.",
                                "example": "zoom",
                                "enum": [
                                  "custom",
                                  "in-person_meeting",
                                  "outbound_call",
                                  "inbound_call",
                                  "invitee_specified",
                                  "zoom",
                                  "google_meet",
                                  "microsoft_teams"
                                ]
                              },
                              "location": {
                                "oneOf": [
                                  {
                                    "deprecated": true,
                                    "type": "object",
                                    "title": "zoom",
                                    "description": "All the information about the Zoom meeting.",
                                    "properties": {
                                      "zoom_join_url": {
                                        "maxLength": 1024,
                                        "type": "string",
                                        "description": "The URL to join a meeting.",
                                        "example": "https://success.zoom.us/j/xxxxxxxx?pwd=xxxxxxxx"
                                      },
                                      "location_type": {
                                        "type": "string",
                                        "description": "Whether it's a Zoom meeting.",
                                        "enum": [
                                          "zoom"
                                        ]
                                      },
                                      "zoom_meeting_data": {
                                        "type": "object",
                                        "description": "The extra info for the meeting.",
                                        "properties": {
                                          "zoom_meeting_id": {
                                            "maxLength": 1024,
                                            "type": "string",
                                            "description": "The unique identifier of the meeting in string format. This field is also known as the meeting number.",
                                            "example": "1234567890"
                                          },
                                          "zoom_meeting_passcode": {
                                            "maxLength": 1024,
                                            "type": "string",
                                            "description": "The passcode of the meeting.",
                                            "example": "dnjchd"
                                          },
                                          "zoom_personal_meeting_id": {
                                            "maxLength": 1024,
                                            "type": "string",
                                            "description": "The personal meeting ID in string format.",
                                            "example": "98656786887"
                                          }
                                        }
                                      },
                                      "zoom_meeting_status": {
                                        "type": "string",
                                        "description": "The status of meeting.",
                                        "enum": [
                                          "scheduled",
                                          "failed"
                                        ]
                                      }
                                    }
                                  },
                                  {
                                    "type": "object",
                                    "title": "Non-online meeting location",
                                    "description": "All the information about the Non-online meeting location.",
                                    "properties": {
                                      "custom_location": {
                                        "maxLength": 1024,
                                        "type": "string",
                                        "description": "The information for a location. May be a phone number or a physical location.",
                                        "example": "AAA office"
                                      },
                                      "location_type": {
                                        "type": "string",
                                        "description": "The location type.",
                                        "enum": [
                                          "custom",
                                          "in-person_meeting",
                                          "outbound_call",
                                          "inbound_call",
                                          "invitee_specified"
                                        ]
                                      },
                                      "additional_info": {
                                        "type": "string",
                                        "description": "Additional info for location.",
                                        "example": "extra info"
                                      }
                                    }
                                  }
                                ]
                              },
                              "external_location": {
                                "description": "The meeting details in the scheduled event.",
                                "readOnly": true,
                                "oneOf": [
                                  {
                                    "type": "object",
                                    "properties": {
                                      "kind": {
                                        "type": "string",
                                        "description": "The type of meeting specified in the schedule.",
                                        "example": "zoom",
                                        "enum": [
                                          "zoom",
                                          "google_meet",
                                          "microsoft_teams"
                                        ]
                                      },
                                      "meeting_id": {
                                        "type": "string",
                                        "description": "The unique identifier of the meeting in string format.",
                                        "example": "94777886776"
                                      },
                                      "personal_meeting_id": {
                                        "type": "string",
                                        "description": "The personal meeting ID in string format.",
                                        "example": "98656786887"
                                      },
                                      "meeting_passcode": {
                                        "type": "string",
                                        "description": "The meeting passcode.",
                                        "example": "958000"
                                      },
                                      "meeting_description": {
                                        "type": "string",
                                        "description": "The Zoom meeting description.",
                                        "example": "Invite you to meeting"
                                      },
                                      "meeting_join_url": {
                                        "type": "string",
                                        "description": "The URL to join the meeting.",
                                        "example": "https://zoom.us/j/7427752805"
                                      }
                                    }
                                  }
                                ]
                              },
                              "start_date_time": {
                                "type": "string",
                                "description": "The scheduled event's start date and time.",
                                "example": "2023-12-21T16:00:00Z"
                              },
                              "status": {
                                "type": "string",
                                "description": "The event status: confirmed or cancelled.",
                                "example": "confirmed",
                                "enum": [
                                  "confirmed",
                                  "cancelled"
                                ]
                              },
                              "summary": {
                                "maxLength": 256,
                                "type": "string",
                                "description": "The schedule name.",
                                "example": "daily meeting"
                              },
                              "updated": {
                                "type": "string",
                                "description": "The moment the scheduled event was updated.",
                                "readOnly": true,
                                "example": "2023-12-21T06:18:32.087Z"
                              },
                              "created": {
                                "type": "string",
                                "description": "The moment the scheduled event was created.",
                                "readOnly": true,
                                "example": "2023-12-21T06:18:32.087Z"
                              },
                              "meeting_notes": {
                                "maxLength": 1024,
                                "type": "string",
                                "description": "The scheduled event's meeting notes.",
                                "example": "meeting notes"
                              }
                            },
                            "additionalProperties": false
                          },
                          "tracking": {
                            "type": "object",
                            "description": "The information to track the invitee source. This field is only present when UTM parameters are added.",
                            "properties": {
                              "utm_source": {
                                "type": "string",
                                "description": "This field identifies which site sent the traffic and is a required parameter.",
                                "example": "zoom"
                              },
                              "utm_medium": {
                                "type": "string",
                                "description": "This field identifies what type of link was used, such as pay-per-click or email.",
                                "example": "pay-per-clik"
                              },
                              "utm_campaign": {
                                "type": "string",
                                "description": "This field identifies a specific product promotion or strategic campaign.",
                                "example": "spring_sale"
                              },
                              "utm_content": {
                                "type": "string",
                                "description": "This field identifies what specifically was clicked to bring the user to the site, such as a banner ad or a text link. It is often used for A/B testing and content-targeted ads.",
                                "example": "schedule link"
                              },
                              "utm_term": {
                                "type": "string",
                                "description": "This field identifies the search terms.",
                                "example": "learning"
                              },
                              "salesforce_ids": {
                                "type": "array",
                                "description": "The tracking of Salesforce IDs.",
                                "items": {
                                  "type": "object",
                                  "property": {
                                    "type": "string",
                                    "description": "The Salesforce object ID that represents an entity."
                                  }
                                }
                              }
                            }
                          },
                          "rescheduled": {
                            "type": "boolean",
                            "description": "Indicates whether this is from a reschedule request."
                          },
                          "created": {
                            "type": "string",
                            "description": "The moment the scheduled request was created.",
                            "readOnly": true,
                            "example": "2023-12-21T06:18:32.087Z"
                          },
                          "text_phone_number": {
                            "type": "string",
                            "description": "The phone number of the invitee to receive text reminders.",
                            "example": "+1 83798773"
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
                  "value": "{\n  \"event\": \"scheduler.scheduled_event_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"user_id\": \"dndhfdjdihd\",\n    \"object\": {\n      \"invitee_email\": \"john.doe@example.com\",\n      \"invitee_first_name\": \"John\",\n      \"invitee_last_name\": \"Doe\",\n      \"manage_url\": \"https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx\",\n      \"time_zone\": \"Asia/Shanghai\",\n      \"questions_and_answers\": [\n        {\n          \"question\": \"Please share anything that will help prepare for our meeting.\",\n          \"position\": 0,\n          \"answer\": [\n            \"First line\",\n            \"Second line\"\n          ]\n        }\n      ],\n      \"scheduled_event\": {\n        \"event_id\": \"woft7torlatbw8ek24bmit5k60\",\n        \"attendees\": [\n          {\n            \"display_name\": \"Jane Doe\",\n            \"email\": \"jane@example.com\"\n          },\n          {\n            \"display_name\": \"David Smith\",\n            \"email\": \"david@example.com\"\n          }\n        ],\n        \"end_date_time\": \"2023-12-21T16:30:00Z\",\n        \"guests\": [\n          \"guest1@example.com\",\n          \"guest2@example.com\"\n        ],\n        \"location_kind\": \"zoom\",\n        \"location\": {\n          \"custom_location\": \"123 Main St\",\n          \"location_type\": \"in-person_meeting\",\n          \"additional_info\": \"additional information\"\n        },\n        \"external_location\": {\n          \"kind\": \"zoom\",\n          \"meeting_id\": \"94777886776\",\n          \"personal_meeting_id\": \"98656786887\",\n          \"meeting_passcode\": \"958000\",\n          \"meeting_description\": \"Invite you to meeting\",\n          \"meeting_join_url\": \"https://zoom.us/j/7427752805\"\n        },\n        \"start_date_time\": \"2023-12-21T16:00:00Z\",\n        \"status\": \"confirmed\",\n        \"summary\": \"30 minute meeting\",\n        \"updated\": \"2023-12-21T06:19:28.309Z\",\n        \"created\": \"2023-12-21T06:18:28.309Z\",\n        \"meeting_notes\": \"meeting notes\",\n        \"invitee_counters\": {\n          \"total\": 1,\n          \"active\": 1,\n          \"limit\": 3\n        }\n      },\n      \"tracking\": {\n        \"utm_source\": \"email\",\n        \"utm_medium\": \"newsletter\",\n        \"utm_campaign\": \"spring_sale\",\n        \"utm_content\": \"booking_link\",\n        \"utm_term\": \"shopping\",\n        \"salesforce_ids\": [\n          {\n            \"sf_person\": \"0035g00000AbCdEAAZ\"\n          },\n          {\n            \"sf_relates\": \"0015g00000AbCdEAAZ\"\n          }\n        ]\n      },\n      \"rescheduled\": false,\n      \"created\": \"2023-12-21T06:18:28.309Z\",\n      \"text_phone_number\": \"+1 83798773\"\n    }\n  }\n}"
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
                      "description": "The timestamp when the event was created."
                    },
                    "payload": {
                      "type": "object",
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the scheduled event host."
                        },
                        "user_id": {
                          "type": "string",
                          "description": "The user ID of the scheduled event host."
                        },
                        "object": {
                          "type": "object",
                          "description": "The event information.",
                          "properties": {
                            "invitee_email": {
                              "type": "string",
                              "description": "The email address of the user who scheduled the event.",
                              "example": "example@example.com"
                            },
                            "invitee_first_name": {
                              "type": "string",
                              "description": "The first name of the user who scheduled the event.",
                              "example": "alax"
                            },
                            "invitee_last_name": {
                              "type": "string",
                              "description": "The last name of the user who scheduled the event.",
                              "example": "bbbb"
                            },
                            "manage_url": {
                              "type": "string",
                              "description": "The URL for the user to reschedule or cancel the scheduled event.",
                              "example": "https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx"
                            },
                            "time_zone": {
                              "type": "string",
                              "description": "The user's time zone.",
                              "example": "Asia/Shanghai"
                            },
                            "questions_and_answers": {
                              "type": "array",
                              "description": "The collection of questions and answers from the booking form.",
                              "items": {
                                "type": "object",
                                "required": [
                                  "question",
                                  "position",
                                  "answer"
                                ],
                                "properties": {
                                  "question": {
                                    "type": "string",
                                    "description": "The question from the booking page.",
                                    "example": "Please share anything that will help prepare for our meeting."
                                  },
                                  "position": {
                                    "type": "integer",
                                    "description": "The index of the question."
                                  },
                                  "answer": {
                                    "description": "The answers to the question.",
                                    "type": "array",
                                    "items": {
                                      "type": "string",
                                      "description": "The answer to the question."
                                    }
                                  }
                                }
                              }
                            },
                            "scheduled_event": {
                              "type": "object",
                              "description": "The scheduled event information.",
                              "properties": {
                                "event_id": {
                                  "type": "string",
                                  "description": "The unique identifier of the event.",
                                  "example": "woft7torlatbw8ek24bmit5k60"
                                },
                                "attendees": {
                                  "type": "array",
                                  "description": "The attendees of the event.",
                                  "items": {
                                    "type": "object",
                                    "properties": {
                                      "display_name": {
                                        "type": "string",
                                        "description": "The attendee's name.",
                                        "example": "bob"
                                      },
                                      "email": {
                                        "type": "string",
                                        "description": "The attendee's email address.",
                                        "example": "abc@zoom.us"
                                      }
                                    }
                                  }
                                },
                                "invitee_counters": {
                                  "type": "object",
                                  "description": "The event's guest counters.",
                                  "properties": {
                                    "total": {
                                      "type": "integer",
                                      "format": "int64",
                                      "description": "The total count of invitees in the event."
                                    },
                                    "active": {
                                      "type": "integer",
                                      "format": "int64",
                                      "description": "The count of active invitees in the event."
                                    },
                                    "limit": {
                                      "type": "integer",
                                      "format": "int64",
                                      "description": "The maximum number of invitees allowed for the event."
                                    }
                                  }
                                },
                                "end_date_time": {
                                  "type": "string",
                                  "description": "The scheduled event's end date and time.",
                                  "example": "2023-12-21T16:30:00Z"
                                },
                                "guests": {
                                  "type": "array",
                                  "description": "The collection of guests.",
                                  "items": {
                                    "type": "string",
                                    "description": "The guest's email address.",
                                    "example": "tom@zoom.us"
                                  }
                                },
                                "location_kind": {
                                  "type": "string",
                                  "description": "Unified location type for both location and external_location objects. If location type is online meeting, location info is in external_location object. If not, location info is in location object.",
                                  "example": "zoom",
                                  "enum": [
                                    "custom",
                                    "in-person_meeting",
                                    "outbound_call",
                                    "inbound_call",
                                    "invitee_specified",
                                    "zoom",
                                    "google_meet",
                                    "microsoft_teams"
                                  ]
                                },
                                "location": {
                                  "oneOf": [
                                    {
                                      "deprecated": true,
                                      "type": "object",
                                      "title": "zoom",
                                      "description": "All the information about the Zoom meeting.",
                                      "properties": {
                                        "zoom_join_url": {
                                          "maxLength": 1024,
                                          "type": "string",
                                          "description": "The URL to join a meeting.",
                                          "example": "https://success.zoom.us/j/xxxxxxxx?pwd=xxxxxxxx"
                                        },
                                        "location_type": {
                                          "type": "string",
                                          "description": "Whether it's a Zoom meeting.",
                                          "enum": [
                                            "zoom"
                                          ]
                                        },
                                        "zoom_meeting_data": {
                                          "type": "object",
                                          "description": "The extra info for the meeting.",
                                          "properties": {
                                            "zoom_meeting_id": {
                                              "maxLength": 1024,
                                              "type": "string",
                                              "description": "The unique identifier of the meeting in string format. This field is also known as the meeting number.",
                                              "example": "1234567890"
                                            },
                                            "zoom_meeting_passcode": {
                                              "maxLength": 1024,
                                              "type": "string",
                                              "description": "The passcode of the meeting.",
                                              "example": "dnjchd"
                                            },
                                            "zoom_personal_meeting_id": {
                                              "maxLength": 1024,
                                              "type": "string",
                                              "description": "The personal meeting ID in string format.",
                                              "example": "98656786887"
                                            }
                                          }
                                        },
                                        "zoom_meeting_status": {
                                          "type": "string",
                                          "description": "The status of meeting.",
                                          "enum": [
                                            "scheduled",
                                            "failed"
                                          ]
                                        }
                                      }
                                    },
                                    {
                                      "type": "object",
                                      "title": "Non-online meeting location",
                                      "description": "All the information about the Non-online meeting location.",
                                      "properties": {
                                        "custom_location": {
                                          "maxLength": 1024,
                                          "type": "string",
                                          "description": "The information for a location. May be a phone number or a physical location.",
                                          "example": "AAA office"
                                        },
                                        "location_type": {
                                          "type": "string",
                                          "description": "The location type.",
                                          "enum": [
                                            "custom",
                                            "in-person_meeting",
                                            "outbound_call",
                                            "inbound_call",
                                            "invitee_specified"
                                          ]
                                        },
                                        "additional_info": {
                                          "type": "string",
                                          "description": "Additional info for location.",
                                          "example": "extra info"
                                        }
                                      }
                                    }
                                  ]
                                },
                                "external_location": {
                                  "description": "The meeting details in the scheduled event.",
                                  "readOnly": true,
                                  "oneOf": [
                                    {
                                      "type": "object",
                                      "properties": {
                                        "kind": {
                                          "type": "string",
                                          "description": "The type of meeting specified in the schedule.",
                                          "example": "zoom",
                                          "enum": [
                                            "zoom",
                                            "google_meet",
                                            "microsoft_teams"
                                          ]
                                        },
                                        "meeting_id": {
                                          "type": "string",
                                          "description": "The unique identifier of the meeting in string format.",
                                          "example": "94777886776"
                                        },
                                        "personal_meeting_id": {
                                          "type": "string",
                                          "description": "The personal meeting ID in string format.",
                                          "example": "98656786887"
                                        },
                                        "meeting_passcode": {
                                          "type": "string",
                                          "description": "The meeting passcode.",
                                          "example": "958000"
                                        },
                                        "meeting_description": {
                                          "type": "string",
                                          "description": "The Zoom meeting description.",
                                          "example": "Invite you to meeting"
                                        },
                                        "meeting_join_url": {
                                          "type": "string",
                                          "description": "The URL to join the meeting.",
                                          "example": "https://zoom.us/j/7427752805"
                                        }
                                      }
                                    }
                                  ]
                                },
                                "start_date_time": {
                                  "type": "string",
                                  "description": "The scheduled event's start date and time.",
                                  "example": "2023-12-21T16:00:00Z"
                                },
                                "status": {
                                  "type": "string",
                                  "description": "The event status: confirmed or cancelled.",
                                  "example": "confirmed",
                                  "enum": [
                                    "confirmed",
                                    "cancelled"
                                  ]
                                },
                                "summary": {
                                  "maxLength": 256,
                                  "type": "string",
                                  "description": "The schedule name.",
                                  "example": "daily meeting"
                                },
                                "updated": {
                                  "type": "string",
                                  "description": "The moment the scheduled event was updated.",
                                  "readOnly": true,
                                  "example": "2023-12-21T06:18:32.087Z"
                                },
                                "created": {
                                  "type": "string",
                                  "description": "The moment the scheduled event was created.",
                                  "readOnly": true,
                                  "example": "2023-12-21T06:18:32.087Z"
                                },
                                "meeting_notes": {
                                  "maxLength": 1024,
                                  "type": "string",
                                  "description": "The scheduled event's meeting notes.",
                                  "example": "meeting notes"
                                }
                              },
                              "additionalProperties": false
                            },
                            "tracking": {
                              "type": "object",
                              "description": "The information to track the invitee source. This field is only present when UTM parameters are added.",
                              "properties": {
                                "utm_source": {
                                  "type": "string",
                                  "description": "This field identifies which site sent the traffic and is a required parameter.",
                                  "example": "zoom"
                                },
                                "utm_medium": {
                                  "type": "string",
                                  "description": "This field identifies what type of link was used, such as pay-per-click or email.",
                                  "example": "pay-per-clik"
                                },
                                "utm_campaign": {
                                  "type": "string",
                                  "description": "This field identifies a specific product promotion or strategic campaign.",
                                  "example": "spring_sale"
                                },
                                "utm_content": {
                                  "type": "string",
                                  "description": "This field identifies what specifically was clicked to bring the user to the site, such as a banner ad or a text link. It is often used for A/B testing and content-targeted ads.",
                                  "example": "schedule link"
                                },
                                "utm_term": {
                                  "type": "string",
                                  "description": "This field identifies the search terms.",
                                  "example": "learning"
                                },
                                "salesforce_ids": {
                                  "type": "array",
                                  "description": "The tracking of Salesforce IDs.",
                                  "items": {
                                    "type": "object",
                                    "property": {
                                      "type": "string",
                                      "description": "The Salesforce object ID that represents an entity."
                                    }
                                  }
                                }
                              }
                            },
                            "rescheduled": {
                              "type": "boolean",
                              "description": "Indicates whether this is from a reschedule request."
                            },
                            "created": {
                              "type": "string",
                              "description": "The moment the scheduled request was created.",
                              "readOnly": true,
                              "example": "2023-12-21T06:18:32.087Z"
                            },
                            "text_phone_number": {
                              "type": "string",
                              "description": "The phone number of the invitee to receive text reminders.",
                              "example": "+1 83798773"
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
                    "value": "{\n  \"event\": \"scheduler.scheduled_event_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"user_id\": \"dndhfdjdihd\",\n    \"object\": {\n      \"invitee_email\": \"john.doe@example.com\",\n      \"invitee_first_name\": \"John\",\n      \"invitee_last_name\": \"Doe\",\n      \"manage_url\": \"https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx\",\n      \"time_zone\": \"Asia/Shanghai\",\n      \"questions_and_answers\": [\n        {\n          \"question\": \"Please share anything that will help prepare for our meeting.\",\n          \"position\": 0,\n          \"answer\": [\n            \"First line\",\n            \"Second line\"\n          ]\n        }\n      ],\n      \"scheduled_event\": {\n        \"event_id\": \"woft7torlatbw8ek24bmit5k60\",\n        \"attendees\": [\n          {\n            \"display_name\": \"Jane Doe\",\n            \"email\": \"jane@example.com\"\n          },\n          {\n            \"display_name\": \"David Smith\",\n            \"email\": \"david@example.com\"\n          }\n        ],\n        \"end_date_time\": \"2023-12-21T16:30:00Z\",\n        \"guests\": [\n          \"guest1@example.com\",\n          \"guest2@example.com\"\n        ],\n        \"location_kind\": \"zoom\",\n        \"location\": {\n          \"custom_location\": \"123 Main St\",\n          \"location_type\": \"in-person_meeting\",\n          \"additional_info\": \"additional information\"\n        },\n        \"external_location\": {\n          \"kind\": \"zoom\",\n          \"meeting_id\": \"94777886776\",\n          \"personal_meeting_id\": \"98656786887\",\n          \"meeting_passcode\": \"958000\",\n          \"meeting_description\": \"Invite you to meeting\",\n          \"meeting_join_url\": \"https://zoom.us/j/7427752805\"\n        },\n        \"start_date_time\": \"2023-12-21T16:00:00Z\",\n        \"status\": \"confirmed\",\n        \"summary\": \"30 minute meeting\",\n        \"updated\": \"2023-12-21T06:19:28.309Z\",\n        \"created\": \"2023-12-21T06:18:28.309Z\",\n        \"meeting_notes\": \"meeting notes\",\n        \"invitee_counters\": {\n          \"total\": 1,\n          \"active\": 1,\n          \"limit\": 3\n        }\n      },\n      \"tracking\": {\n        \"utm_source\": \"email\",\n        \"utm_medium\": \"newsletter\",\n        \"utm_campaign\": \"spring_sale\",\n        \"utm_content\": \"booking_link\",\n        \"utm_term\": \"shopping\",\n        \"salesforce_ids\": [\n          {\n            \"sf_person\": \"0035g00000AbCdEAAZ\"\n          },\n          {\n            \"sf_relates\": \"0015g00000AbCdEAAZ\"\n          }\n        ]\n      },\n      \"rescheduled\": false,\n      \"created\": \"2023-12-21T06:18:28.309Z\",\n      \"text_phone_number\": \"+1 83798773\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "scheduler.scheduled_event_canceled": {
      "post": {
        "tags": [
          "scheduler"
        ],
        "operationId": "scheduler.scheduled_event_canceled",
        "requestBody": {
          "description": "# scheduler.scheduled_event_canceled\nThe **Scheduled event canceled** event triggers each time the host or booker cancels the scheduled event.\n\n## Prerequisites\nEnable **Event subscriptions** for your Marketplace app with:\n\n* A valid **Event notification endpoint URL**.\n* Enable the **Scheduled event canceled** subscription under the **Scheduler** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `scheduler:read:admin`,`scheduler:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `scheduler:read:scheduled_event`,`scheduler:read:scheduled_event:admin`\n\n**Event type**: `scheduler.scheduled_event_canceled`\n",
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
                    "description": "The timestamp when the event was created."
                  },
                  "payload": {
                    "type": "object",
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the scheduled event host."
                      },
                      "user_id": {
                        "type": "string",
                        "description": "The user ID of the scheduled event host."
                      },
                      "object": {
                        "type": "object",
                        "description": "The information about the event.",
                        "properties": {
                          "invitee_email": {
                            "type": "string",
                            "description": "The email address of the user who scheduled the event.",
                            "example": "example@example.com"
                          },
                          "invitee_first_name": {
                            "type": "string",
                            "description": "The first name of the user who scheduled the event.",
                            "example": "alax"
                          },
                          "invitee_last_name": {
                            "type": "string",
                            "description": "The last name of the user who scheduled the event.",
                            "example": "bbbb"
                          },
                          "manage_url": {
                            "type": "string",
                            "description": "The URL for the user to reschedule or cancel the scheduled event.",
                            "example": "https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx"
                          },
                          "cancellation": {
                            "type": "object",
                            "description": "The scheduled event cancellation information.",
                            "properties": {
                              "canceled_by": {
                                "type": "string",
                                "description": "The name or email address of the canceler.",
                                "example": "aa@bb.c"
                              },
                              "canceler_type": {
                                "type": "string",
                                "description": "The user type of the person who canceled.",
                                "example": "host",
                                "enum": [
                                  "host",
                                  "invitee"
                                ]
                              },
                              "cancel_at": {
                                "type": "string",
                                "description": "The date and time when this scheduled event was canceled.",
                                "example": "2023-12-21T16:30:00+08:00"
                              },
                              "reason": {
                                "type": "string",
                                "description": "The reason for canceling this scheduled event.",
                                "example": "Time conflict"
                              }
                            }
                          },
                          "time_zone": {
                            "type": "string",
                            "description": "The user's time zone.",
                            "example": "Asia/Shanghai"
                          },
                          "questions_and_answers": {
                            "type": "array",
                            "description": "All questions and answers.",
                            "items": {
                              "type": "object",
                              "required": [
                                "question",
                                "position",
                                "answer"
                              ],
                              "properties": {
                                "question": {
                                  "type": "string",
                                  "description": "The question on the booking page.",
                                  "example": "Please share anything that will help prepare for our meeting."
                                },
                                "position": {
                                  "type": "integer",
                                  "description": "The index of the question."
                                },
                                "answer": {
                                  "description": "The answers to the question.",
                                  "type": "array",
                                  "items": {
                                    "type": "string",
                                    "description": "The answer to the question."
                                  }
                                }
                              }
                            }
                          },
                          "scheduled_event": {
                            "type": "object",
                            "description": "The information about the scheduled event.",
                            "properties": {
                              "event_id": {
                                "type": "string",
                                "description": "The unique identifier of the event.",
                                "example": "woft7torlatbw8ek24bmit5k60"
                              },
                              "attendees": {
                                "type": "array",
                                "description": "The hosts of the scheduled event.",
                                "items": {
                                  "type": "object",
                                  "properties": {
                                    "display_name": {
                                      "type": "string",
                                      "description": "The attendee's name.",
                                      "example": "bob"
                                    },
                                    "email": {
                                      "type": "string",
                                      "description": "The attendee's email address.",
                                      "example": "abc@zoom.us"
                                    }
                                  }
                                }
                              },
                              "invitee_counters": {
                                "type": "object",
                                "description": "The event's guest counters.",
                                "properties": {
                                  "total": {
                                    "type": "integer",
                                    "format": "int64",
                                    "description": "The total count of invitees in the event."
                                  },
                                  "active": {
                                    "type": "integer",
                                    "format": "int64",
                                    "description": "The count of active invitees in the event."
                                  },
                                  "limit": {
                                    "type": "integer",
                                    "format": "int64",
                                    "description": "The maximum limit of invitees in the event."
                                  }
                                }
                              },
                              "end_date_time": {
                                "type": "string",
                                "description": "The scheduled event's end date and time.",
                                "example": "2023-12-21T16:30:00Z"
                              },
                              "guests": {
                                "type": "array",
                                "description": "The collection of guests.",
                                "items": {
                                  "type": "string",
                                  "description": "The guest's email address.",
                                  "example": "tom@zoom.us"
                                }
                              },
                              "location_kind": {
                                "type": "string",
                                "description": "Unified location type for both location and external_location objects. If location type is online meeting, location info is in external_location object. If not, location info is in location object.",
                                "example": "zoom",
                                "enum": [
                                  "custom",
                                  "in-person_meeting",
                                  "outbound_call",
                                  "inbound_call",
                                  "invitee_specified",
                                  "zoom",
                                  "google_meet",
                                  "microsoft_teams"
                                ]
                              },
                              "location": {
                                "oneOf": [
                                  {
                                    "deprecated": true,
                                    "type": "object",
                                    "title": "zoom",
                                    "description": "All the information about the Zoom meeting.",
                                    "properties": {
                                      "zoom_join_url": {
                                        "maxLength": 1024,
                                        "type": "string",
                                        "description": "The URL to join a meeting.",
                                        "example": "https://success.zoom.us/j/xxxxxxxx?pwd=xxxxxxxx"
                                      },
                                      "location_type": {
                                        "type": "string",
                                        "description": "Whether it's a Zoom meeting.",
                                        "enum": [
                                          "zoom"
                                        ]
                                      },
                                      "zoom_meeting_data": {
                                        "type": "object",
                                        "description": "The additional information for the meeting.",
                                        "properties": {
                                          "zoom_meeting_id": {
                                            "type": "string",
                                            "maxLength": 1024,
                                            "description": "The unique identifier of the meeting in string format. This field is also known as the meeting number.",
                                            "example": "1234567890"
                                          },
                                          "zoom_meeting_passcode": {
                                            "maxLength": 1024,
                                            "type": "string",
                                            "description": "The passcode for the meeting.",
                                            "example": "dnjchd"
                                          },
                                          "zoom_personal_meeting_id": {
                                            "maxLength": 1024,
                                            "type": "string",
                                            "description": "The personal meeting ID in string format.",
                                            "example": "98656786887"
                                          }
                                        }
                                      },
                                      "zoom_meeting_status": {
                                        "type": "string",
                                        "description": "The status of the meeting.",
                                        "enum": [
                                          "scheduled",
                                          "failed"
                                        ]
                                      }
                                    }
                                  },
                                  {
                                    "type": "object",
                                    "title": "Non-online meeting location",
                                    "description": "All the information about the non-online meeting location.",
                                    "properties": {
                                      "custom_location": {
                                        "maxLength": 1024,
                                        "type": "string",
                                        "description": "The information for a location. May be a phone number or a physical location.",
                                        "example": "AAA office"
                                      },
                                      "location_type": {
                                        "type": "string",
                                        "description": "The location type.",
                                        "enum": [
                                          "custom",
                                          "in-person_meeting",
                                          "outbound_call",
                                          "inbound_call",
                                          "invitee_specified"
                                        ]
                                      },
                                      "additional_info": {
                                        "type": "string",
                                        "description": "Additional information for the location.",
                                        "example": "extra info"
                                      }
                                    }
                                  }
                                ]
                              },
                              "external_location": {
                                "description": "The meeting details in the scheduled event.",
                                "readOnly": true,
                                "oneOf": [
                                  {
                                    "type": "object",
                                    "properties": {
                                      "kind": {
                                        "type": "string",
                                        "description": "The type of meeting specified in the schedule.",
                                        "example": "zoom",
                                        "enum": [
                                          "zoom",
                                          "google_meet",
                                          "microsoft_teams"
                                        ]
                                      },
                                      "meeting_id": {
                                        "type": "string",
                                        "description": "The unique identifier of the meeting in string format.",
                                        "example": "94777886776"
                                      },
                                      "personal_meeting_id": {
                                        "type": "string",
                                        "description": "The personal meeting ID in string format.",
                                        "example": "98656786887"
                                      },
                                      "meeting_passcode": {
                                        "type": "string",
                                        "description": "The passcode for the meeting.",
                                        "example": "958000"
                                      },
                                      "meeting_description": {
                                        "type": "string",
                                        "description": "The description of the Zoom meeting.",
                                        "example": "Invite you to meeting"
                                      },
                                      "meeting_join_url": {
                                        "type": "string",
                                        "description": "The URL to join the meeting.",
                                        "example": "https://zoom.us/j/7427752805"
                                      }
                                    }
                                  }
                                ]
                              },
                              "start_date_time": {
                                "type": "string",
                                "description": "The scheduled event's start date and time.",
                                "example": "2023-12-21T16:00:00Z"
                              },
                              "status": {
                                "type": "string",
                                "description": "The status of the event: confirmed or cancelled.",
                                "example": "confirmed",
                                "enum": [
                                  "confirmed",
                                  "cancelled"
                                ]
                              },
                              "summary": {
                                "maxLength": 256,
                                "type": "string",
                                "description": "The name of the schedule.",
                                "example": "daily meeting"
                              },
                              "updated": {
                                "type": "string",
                                "description": "The moment the scheduled event was updated.",
                                "readOnly": true,
                                "example": "2023-12-21T06:18:32.087Z"
                              },
                              "created": {
                                "type": "string",
                                "description": "The moment the scheduled event was created.",
                                "readOnly": true,
                                "example": "2023-12-21T06:18:32.087Z"
                              },
                              "meeting_notes": {
                                "maxLength": 1024,
                                "type": "string",
                                "description": "The meeting notes for the scheduled event.",
                                "example": "meeting notes"
                              }
                            },
                            "additionalProperties": false
                          },
                          "tracking": {
                            "type": "object",
                            "description": "The information to track the source of the invitee.",
                            "properties": {
                              "utm_source": {
                                "type": "string",
                                "description": "This field identifies which site sent the traffic, and is a required parameter.",
                                "example": "zoom"
                              },
                              "utm_medium": {
                                "type": "string",
                                "description": "This field identifies what type of link was used, such as pay-per-click or email.",
                                "example": "pay-per-clik"
                              },
                              "utm_campaign": {
                                "type": "string",
                                "description": "This field identifies a specific product promotion or strategic campaign.",
                                "example": "spring_sale"
                              },
                              "utm_content": {
                                "type": "string",
                                "description": "This field identifies what specifically was clicked to bring the user to the site, such as a banner ad or a text link. It is often used for A/B testing and content-targeted ads.",
                                "example": "schedule link"
                              },
                              "utm_term": {
                                "type": "string",
                                "description": "This field identifies search terms.",
                                "example": "learning"
                              },
                              "salesforce_ids": {
                                "type": "array",
                                "description": "The tracking of Salesforce IDs.",
                                "items": {
                                  "type": "object",
                                  "property": {
                                    "type": "string",
                                    "description": "The Salesforce object ID that represents an entity."
                                  }
                                }
                              }
                            }
                          },
                          "rescheduled": {
                            "type": "boolean",
                            "description": "Whether it's from a reschedule request."
                          },
                          "created": {
                            "type": "string",
                            "description": "The moment the scheduled request was created.",
                            "readOnly": true,
                            "example": "2023-12-21T06:18:32.087Z"
                          },
                          "text_phone_number": {
                            "type": "string",
                            "description": "The phone number of the invitee to receive text reminders.",
                            "example": "+1 83798773"
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
                  "value": "{\n  \"event\": \"scheduler.scheduled_event_canceled\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"user_id\": \"dndhfdjdihd\",\n    \"object\": {\n      \"invitee_email\": \"john.doe@example.com\",\n      \"invitee_first_name\": \"John\",\n      \"invitee_last_name\": \"Doe\",\n      \"manage_url\": \"https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx\",\n      \"cancellation\": {\n        \"canceled_by\": \"aa@bb.c\",\n        \"canceler_type\": \"host\",\n        \"cancel_at\": \"2023-12-21T16:30:00+08:00\",\n        \"reason\": \"Time conflict\"\n      },\n      \"time_zone\": \"Asia/Shanghai\",\n      \"questions_and_answers\": [\n        {\n          \"question\": \"Please share anything that will help prepare for our meeting.\",\n          \"position\": 0,\n          \"answer\": [\n            \"First line\",\n            \"Second line\"\n          ]\n        }\n      ],\n      \"scheduled_event\": {\n        \"event_id\": \"woft7torlatbw8ek24bmit5k60\",\n        \"attendees\": [\n          {\n            \"display_name\": \"Jane Doe\",\n            \"email\": \"jane@example.com\"\n          },\n          {\n            \"display_name\": \"David Smith\",\n            \"email\": \"david@example.com\"\n          }\n        ],\n        \"end_date_time\": \"2023-12-21T16:30:00Z\",\n        \"guests\": [\n          \"guest1@example.com\",\n          \"guest2@example.com\"\n        ],\n        \"location_kind\": \"custom\",\n        \"location\": {\n          \"custom_location\": \"123 Main St\",\n          \"location_type\": \"in-person_meeting\",\n          \"additional_info\": \"additional information\"\n        },\n        \"external_location\": {\n          \"kind\": \"zoom\",\n          \"meeting_id\": \"94777886776\",\n          \"personal_meeting_id\": \"98656786887\",\n          \"meeting_passcode\": \"958000\",\n          \"meeting_description\": \"Invite you to meeting\",\n          \"meeting_join_url\": \"https://zoom.us/j/7427752805\"\n        },\n        \"start_date_time\": \"2023-12-21T16:00:00Z\",\n        \"status\": \"confirmed\",\n        \"summary\": \"30 minute meeting\",\n        \"updated\": \"2023-12-21T06:19:28.309Z\",\n        \"created\": \"2023-12-21T06:18:28.309Z\",\n        \"meeting_notes\": \"meeting notes\",\n        \"invitee_counters\": {\n          \"total\": 1,\n          \"active\": 1,\n          \"limit\": 3\n        }\n      },\n      \"tracking\": {\n        \"utm_source\": \"email\",\n        \"utm_medium\": \"newsletter\",\n        \"utm_campaign\": \"spring_sale\",\n        \"utm_content\": \"booking_link\",\n        \"utm_term\": \"shopping\",\n        \"salesforce_ids\": [\n          {\n            \"sf_person\": \"0035g00000AbCdEAAZ\"\n          },\n          {\n            \"sf_relates\": \"0015g00000AbCdEAAZ\"\n          }\n        ]\n      },\n      \"rescheduled\": false,\n      \"created\": \"2023-12-21T06:18:28.309Z\",\n      \"text_phone_number\": \"+1 48837847\"\n    }\n  }\n}"
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
                      "description": "The timestamp when the event was created."
                    },
                    "payload": {
                      "type": "object",
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the scheduled event host."
                        },
                        "user_id": {
                          "type": "string",
                          "description": "The user ID of the scheduled event host."
                        },
                        "object": {
                          "type": "object",
                          "description": "The information about the event.",
                          "properties": {
                            "invitee_email": {
                              "type": "string",
                              "description": "The email address of the user who scheduled the event.",
                              "example": "example@example.com"
                            },
                            "invitee_first_name": {
                              "type": "string",
                              "description": "The first name of the user who scheduled the event.",
                              "example": "alax"
                            },
                            "invitee_last_name": {
                              "type": "string",
                              "description": "The last name of the user who scheduled the event.",
                              "example": "bbbb"
                            },
                            "manage_url": {
                              "type": "string",
                              "description": "The URL for the user to reschedule or cancel the scheduled event.",
                              "example": "https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx"
                            },
                            "cancellation": {
                              "type": "object",
                              "description": "The scheduled event cancellation information.",
                              "properties": {
                                "canceled_by": {
                                  "type": "string",
                                  "description": "The name or email address of the canceler.",
                                  "example": "aa@bb.c"
                                },
                                "canceler_type": {
                                  "type": "string",
                                  "description": "The user type of the person who canceled.",
                                  "example": "host",
                                  "enum": [
                                    "host",
                                    "invitee"
                                  ]
                                },
                                "cancel_at": {
                                  "type": "string",
                                  "description": "The date and time when this scheduled event was canceled.",
                                  "example": "2023-12-21T16:30:00+08:00"
                                },
                                "reason": {
                                  "type": "string",
                                  "description": "The reason for canceling this scheduled event.",
                                  "example": "Time conflict"
                                }
                              }
                            },
                            "time_zone": {
                              "type": "string",
                              "description": "The user's time zone.",
                              "example": "Asia/Shanghai"
                            },
                            "questions_and_answers": {
                              "type": "array",
                              "description": "All questions and answers.",
                              "items": {
                                "type": "object",
                                "required": [
                                  "question",
                                  "position",
                                  "answer"
                                ],
                                "properties": {
                                  "question": {
                                    "type": "string",
                                    "description": "The question on the booking page.",
                                    "example": "Please share anything that will help prepare for our meeting."
                                  },
                                  "position": {
                                    "type": "integer",
                                    "description": "The index of the question."
                                  },
                                  "answer": {
                                    "description": "The answers to the question.",
                                    "type": "array",
                                    "items": {
                                      "type": "string",
                                      "description": "The answer to the question."
                                    }
                                  }
                                }
                              }
                            },
                            "scheduled_event": {
                              "type": "object",
                              "description": "The information about the scheduled event.",
                              "properties": {
                                "event_id": {
                                  "type": "string",
                                  "description": "The unique identifier of the event.",
                                  "example": "woft7torlatbw8ek24bmit5k60"
                                },
                                "attendees": {
                                  "type": "array",
                                  "description": "The hosts of the scheduled event.",
                                  "items": {
                                    "type": "object",
                                    "properties": {
                                      "display_name": {
                                        "type": "string",
                                        "description": "The attendee's name.",
                                        "example": "bob"
                                      },
                                      "email": {
                                        "type": "string",
                                        "description": "The attendee's email address.",
                                        "example": "abc@zoom.us"
                                      }
                                    }
                                  }
                                },
                                "invitee_counters": {
                                  "type": "object",
                                  "description": "The event's guest counters.",
                                  "properties": {
                                    "total": {
                                      "type": "integer",
                                      "format": "int64",
                                      "description": "The total count of invitees in the event."
                                    },
                                    "active": {
                                      "type": "integer",
                                      "format": "int64",
                                      "description": "The count of active invitees in the event."
                                    },
                                    "limit": {
                                      "type": "integer",
                                      "format": "int64",
                                      "description": "The maximum limit of invitees in the event."
                                    }
                                  }
                                },
                                "end_date_time": {
                                  "type": "string",
                                  "description": "The scheduled event's end date and time.",
                                  "example": "2023-12-21T16:30:00Z"
                                },
                                "guests": {
                                  "type": "array",
                                  "description": "The collection of guests.",
                                  "items": {
                                    "type": "string",
                                    "description": "The guest's email address.",
                                    "example": "tom@zoom.us"
                                  }
                                },
                                "location_kind": {
                                  "type": "string",
                                  "description": "Unified location type for both location and external_location objects. If location type is online meeting, location info is in external_location object. If not, location info is in location object.",
                                  "example": "zoom",
                                  "enum": [
                                    "custom",
                                    "in-person_meeting",
                                    "outbound_call",
                                    "inbound_call",
                                    "invitee_specified",
                                    "zoom",
                                    "google_meet",
                                    "microsoft_teams"
                                  ]
                                },
                                "location": {
                                  "oneOf": [
                                    {
                                      "deprecated": true,
                                      "type": "object",
                                      "title": "zoom",
                                      "description": "All the information about the Zoom meeting.",
                                      "properties": {
                                        "zoom_join_url": {
                                          "maxLength": 1024,
                                          "type": "string",
                                          "description": "The URL to join a meeting.",
                                          "example": "https://success.zoom.us/j/xxxxxxxx?pwd=xxxxxxxx"
                                        },
                                        "location_type": {
                                          "type": "string",
                                          "description": "Whether it's a Zoom meeting.",
                                          "enum": [
                                            "zoom"
                                          ]
                                        },
                                        "zoom_meeting_data": {
                                          "type": "object",
                                          "description": "The additional information for the meeting.",
                                          "properties": {
                                            "zoom_meeting_id": {
                                              "type": "string",
                                              "maxLength": 1024,
                                              "description": "The unique identifier of the meeting in string format. This field is also known as the meeting number.",
                                              "example": "1234567890"
                                            },
                                            "zoom_meeting_passcode": {
                                              "maxLength": 1024,
                                              "type": "string",
                                              "description": "The passcode for the meeting.",
                                              "example": "dnjchd"
                                            },
                                            "zoom_personal_meeting_id": {
                                              "maxLength": 1024,
                                              "type": "string",
                                              "description": "The personal meeting ID in string format.",
                                              "example": "98656786887"
                                            }
                                          }
                                        },
                                        "zoom_meeting_status": {
                                          "type": "string",
                                          "description": "The status of the meeting.",
                                          "enum": [
                                            "scheduled",
                                            "failed"
                                          ]
                                        }
                                      }
                                    },
                                    {
                                      "type": "object",
                                      "title": "Non-online meeting location",
                                      "description": "All the information about the non-online meeting location.",
                                      "properties": {
                                        "custom_location": {
                                          "maxLength": 1024,
                                          "type": "string",
                                          "description": "The information for a location. May be a phone number or a physical location.",
                                          "example": "AAA office"
                                        },
                                        "location_type": {
                                          "type": "string",
                                          "description": "The location type.",
                                          "enum": [
                                            "custom",
                                            "in-person_meeting",
                                            "outbound_call",
                                            "inbound_call",
                                            "invitee_specified"
                                          ]
                                        },
                                        "additional_info": {
                                          "type": "string",
                                          "description": "Additional information for the location.",
                                          "example": "extra info"
                                        }
                                      }
                                    }
                                  ]
                                },
                                "external_location": {
                                  "description": "The meeting details in the scheduled event.",
                                  "readOnly": true,
                                  "oneOf": [
                                    {
                                      "type": "object",
                                      "properties": {
                                        "kind": {
                                          "type": "string",
                                          "description": "The type of meeting specified in the schedule.",
                                          "example": "zoom",
                                          "enum": [
                                            "zoom",
                                            "google_meet",
                                            "microsoft_teams"
                                          ]
                                        },
                                        "meeting_id": {
                                          "type": "string",
                                          "description": "The unique identifier of the meeting in string format.",
                                          "example": "94777886776"
                                        },
                                        "personal_meeting_id": {
                                          "type": "string",
                                          "description": "The personal meeting ID in string format.",
                                          "example": "98656786887"
                                        },
                                        "meeting_passcode": {
                                          "type": "string",
                                          "description": "The passcode for the meeting.",
                                          "example": "958000"
                                        },
                                        "meeting_description": {
                                          "type": "string",
                                          "description": "The description of the Zoom meeting.",
                                          "example": "Invite you to meeting"
                                        },
                                        "meeting_join_url": {
                                          "type": "string",
                                          "description": "The URL to join the meeting.",
                                          "example": "https://zoom.us/j/7427752805"
                                        }
                                      }
                                    }
                                  ]
                                },
                                "start_date_time": {
                                  "type": "string",
                                  "description": "The scheduled event's start date and time.",
                                  "example": "2023-12-21T16:00:00Z"
                                },
                                "status": {
                                  "type": "string",
                                  "description": "The status of the event: confirmed or cancelled.",
                                  "example": "confirmed",
                                  "enum": [
                                    "confirmed",
                                    "cancelled"
                                  ]
                                },
                                "summary": {
                                  "maxLength": 256,
                                  "type": "string",
                                  "description": "The name of the schedule.",
                                  "example": "daily meeting"
                                },
                                "updated": {
                                  "type": "string",
                                  "description": "The moment the scheduled event was updated.",
                                  "readOnly": true,
                                  "example": "2023-12-21T06:18:32.087Z"
                                },
                                "created": {
                                  "type": "string",
                                  "description": "The moment the scheduled event was created.",
                                  "readOnly": true,
                                  "example": "2023-12-21T06:18:32.087Z"
                                },
                                "meeting_notes": {
                                  "maxLength": 1024,
                                  "type": "string",
                                  "description": "The meeting notes for the scheduled event.",
                                  "example": "meeting notes"
                                }
                              },
                              "additionalProperties": false
                            },
                            "tracking": {
                              "type": "object",
                              "description": "The information to track the source of the invitee.",
                              "properties": {
                                "utm_source": {
                                  "type": "string",
                                  "description": "This field identifies which site sent the traffic, and is a required parameter.",
                                  "example": "zoom"
                                },
                                "utm_medium": {
                                  "type": "string",
                                  "description": "This field identifies what type of link was used, such as pay-per-click or email.",
                                  "example": "pay-per-clik"
                                },
                                "utm_campaign": {
                                  "type": "string",
                                  "description": "This field identifies a specific product promotion or strategic campaign.",
                                  "example": "spring_sale"
                                },
                                "utm_content": {
                                  "type": "string",
                                  "description": "This field identifies what specifically was clicked to bring the user to the site, such as a banner ad or a text link. It is often used for A/B testing and content-targeted ads.",
                                  "example": "schedule link"
                                },
                                "utm_term": {
                                  "type": "string",
                                  "description": "This field identifies search terms.",
                                  "example": "learning"
                                },
                                "salesforce_ids": {
                                  "type": "array",
                                  "description": "The tracking of Salesforce IDs.",
                                  "items": {
                                    "type": "object",
                                    "property": {
                                      "type": "string",
                                      "description": "The Salesforce object ID that represents an entity."
                                    }
                                  }
                                }
                              }
                            },
                            "rescheduled": {
                              "type": "boolean",
                              "description": "Whether it's from a reschedule request."
                            },
                            "created": {
                              "type": "string",
                              "description": "The moment the scheduled request was created.",
                              "readOnly": true,
                              "example": "2023-12-21T06:18:32.087Z"
                            },
                            "text_phone_number": {
                              "type": "string",
                              "description": "The phone number of the invitee to receive text reminders.",
                              "example": "+1 83798773"
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
                    "value": "{\n  \"event\": \"scheduler.scheduled_event_canceled\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"user_id\": \"dndhfdjdihd\",\n    \"object\": {\n      \"invitee_email\": \"john.doe@example.com\",\n      \"invitee_first_name\": \"John\",\n      \"invitee_last_name\": \"Doe\",\n      \"manage_url\": \"https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx\",\n      \"cancellation\": {\n        \"canceled_by\": \"aa@bb.c\",\n        \"canceler_type\": \"host\",\n        \"cancel_at\": \"2023-12-21T16:30:00+08:00\",\n        \"reason\": \"Time conflict\"\n      },\n      \"time_zone\": \"Asia/Shanghai\",\n      \"questions_and_answers\": [\n        {\n          \"question\": \"Please share anything that will help prepare for our meeting.\",\n          \"position\": 0,\n          \"answer\": [\n            \"First line\",\n            \"Second line\"\n          ]\n        }\n      ],\n      \"scheduled_event\": {\n        \"event_id\": \"woft7torlatbw8ek24bmit5k60\",\n        \"attendees\": [\n          {\n            \"display_name\": \"Jane Doe\",\n            \"email\": \"jane@example.com\"\n          },\n          {\n            \"display_name\": \"David Smith\",\n            \"email\": \"david@example.com\"\n          }\n        ],\n        \"end_date_time\": \"2023-12-21T16:30:00Z\",\n        \"guests\": [\n          \"guest1@example.com\",\n          \"guest2@example.com\"\n        ],\n        \"location_kind\": \"custom\",\n        \"location\": {\n          \"custom_location\": \"123 Main St\",\n          \"location_type\": \"in-person_meeting\",\n          \"additional_info\": \"additional information\"\n        },\n        \"external_location\": {\n          \"kind\": \"zoom\",\n          \"meeting_id\": \"94777886776\",\n          \"personal_meeting_id\": \"98656786887\",\n          \"meeting_passcode\": \"958000\",\n          \"meeting_description\": \"Invite you to meeting\",\n          \"meeting_join_url\": \"https://zoom.us/j/7427752805\"\n        },\n        \"start_date_time\": \"2023-12-21T16:00:00Z\",\n        \"status\": \"confirmed\",\n        \"summary\": \"30 minute meeting\",\n        \"updated\": \"2023-12-21T06:19:28.309Z\",\n        \"created\": \"2023-12-21T06:18:28.309Z\",\n        \"meeting_notes\": \"meeting notes\",\n        \"invitee_counters\": {\n          \"total\": 1,\n          \"active\": 1,\n          \"limit\": 3\n        }\n      },\n      \"tracking\": {\n        \"utm_source\": \"email\",\n        \"utm_medium\": \"newsletter\",\n        \"utm_campaign\": \"spring_sale\",\n        \"utm_content\": \"booking_link\",\n        \"utm_term\": \"shopping\",\n        \"salesforce_ids\": [\n          {\n            \"sf_person\": \"0035g00000AbCdEAAZ\"\n          },\n          {\n            \"sf_relates\": \"0015g00000AbCdEAAZ\"\n          }\n        ]\n      },\n      \"rescheduled\": false,\n      \"created\": \"2023-12-21T06:18:28.309Z\",\n      \"text_phone_number\": \"+1 48837847\"\n    }\n  }\n}"
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
      "scheduler",
      "events"
    ]
  },
  "downloadPath": "/api-hub/scheduler/events/webhooks.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": false
  },
  "productMeta": {
    "specType": "adoc",
    "slug": "/docs/api/rest/reference/zoom-scheduler/events",
    "title": "Scheduler Webhooks",
    "description": "",
    "keywords": "zoom, api, webhooks, scheduler",
    "params": {
      "productName": "Zoom Scheduler"
    },
    "zoapEndpoint": "webhookByProduct",
    "productDisplayTitle": "Scheduler",
    "scalarSlug": "/docs/api/scheduler/events",
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "svgname": "_zoomscheduler"
  },
  "frontmatter": {
    "title": "Scheduler Webhooks",
    "keywords": "zoom, api, webhooks, scheduler",
    "description": "Scheduler webhooks allow developers to receive events for [Scheduler](https://developers.zoom.us/docs/zoom-scheduler/). ",
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
        "scheduler",
        "events"
      ]
    },
    "downloadPath": "/api-hub/scheduler/events/webhooks.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": false
    },
    "productMeta": {
      "specType": "adoc",
      "slug": "/docs/api/rest/reference/zoom-scheduler/events",
      "title": "Scheduler Webhooks",
      "description": "",
      "keywords": "zoom, api, webhooks, scheduler",
      "params": {
        "productName": "Zoom Scheduler"
      },
      "zoapEndpoint": "webhookByProduct",
      "productDisplayTitle": "Scheduler",
      "scalarSlug": "/docs/api/scheduler/events",
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "svgname": "_zoomscheduler"
    },
    "spec": "{\"openapi\":\"3.1.0\",\"info\":{\"title\":\"Scheduler\",\"description\":\"Scheduler webhooks allow developers to receive events for [Scheduler](https://developers.zoom.us/docs/zoom-scheduler/). \",\"version\":\"1.0.0\"},\"paths\":{\"scheduler.scheduled_event_created\":{\"post\":{\"tags\":[\"scheduler\"],\"operationId\":\"scheduler.scheduled_event_created\",\"requestBody\":{\"description\":\"# scheduler.scheduled_event_created\\nThe **Scheduled event created** event triggers each time one user schedules an event with the scheduler host through the public booking page.\\n\\n## Prerequisites\\nEnable **Event subscriptions** for your Marketplace app with:\\n\\n* A valid **Event notification endpoint URL**.\\n* The **Scheduled event created** subscription is enabled under the **Scheduler** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `scheduler:read:admin`,`scheduler:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `scheduler:read:scheduled_event`,`scheduler:read:scheduled_event:admin`\\n\\n**Event type**: `scheduler.scheduled_event_created`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event was created.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the scheduled event host.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The user ID of the scheduled event host.\"},\"object\":{\"type\":\"object\",\"description\":\"The event information.\",\"properties\":{\"invitee_email\":{\"type\":\"string\",\"description\":\"The email address of the user who scheduled the event.\",\"example\":\"example@example.com\"},\"invitee_first_name\":{\"type\":\"string\",\"description\":\"The first name of the user who scheduled the event.\",\"example\":\"alax\"},\"invitee_last_name\":{\"type\":\"string\",\"description\":\"The last name of the user who scheduled the event.\",\"example\":\"bbbb\"},\"manage_url\":{\"type\":\"string\",\"description\":\"The URL for the user to reschedule or cancel the scheduled event.\",\"example\":\"https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx\"},\"time_zone\":{\"type\":\"string\",\"description\":\"The user's time zone.\",\"example\":\"Asia/Shanghai\"},\"questions_and_answers\":{\"type\":\"array\",\"description\":\"The collection of questions and answers from the booking form.\",\"items\":{\"type\":\"object\",\"required\":[\"question\",\"position\",\"answer\"],\"properties\":{\"question\":{\"type\":\"string\",\"description\":\"The question from the booking page.\",\"example\":\"Please share anything that will help prepare for our meeting.\"},\"position\":{\"type\":\"integer\",\"description\":\"The index of the question.\"},\"answer\":{\"description\":\"The answers to the question.\",\"type\":\"array\",\"items\":{\"type\":\"string\",\"description\":\"The answer to the question.\"}}}}},\"scheduled_event\":{\"type\":\"object\",\"description\":\"The scheduled event information.\",\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the event.\",\"example\":\"woft7torlatbw8ek24bmit5k60\"},\"attendees\":{\"type\":\"array\",\"description\":\"The attendees of the event.\",\"items\":{\"type\":\"object\",\"properties\":{\"display_name\":{\"type\":\"string\",\"description\":\"The attendee's name.\",\"example\":\"bob\"},\"email\":{\"type\":\"string\",\"description\":\"The attendee's email address.\",\"example\":\"abc@zoom.us\"}}}},\"invitee_counters\":{\"type\":\"object\",\"description\":\"The event's guest counters.\",\"properties\":{\"total\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The total count of invitees in the event.\"},\"active\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The count of active invitees in the event.\"},\"limit\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The maximum number of invitees allowed for the event.\"}}},\"end_date_time\":{\"type\":\"string\",\"description\":\"The scheduled event's end date and time.\",\"example\":\"2023-12-21T16:30:00Z\"},\"guests\":{\"type\":\"array\",\"description\":\"The collection of guests.\",\"items\":{\"type\":\"string\",\"description\":\"The guest's email address.\",\"example\":\"tom@zoom.us\"}},\"location_kind\":{\"type\":\"string\",\"description\":\"Unified location type for both location and external_location objects. If location type is online meeting, location info is in external_location object. If not, location info is in location object.\",\"example\":\"zoom\",\"enum\":[\"custom\",\"in-person_meeting\",\"outbound_call\",\"inbound_call\",\"invitee_specified\",\"zoom\",\"google_meet\",\"microsoft_teams\"]},\"location\":{\"oneOf\":[{\"deprecated\":true,\"type\":\"object\",\"title\":\"zoom\",\"description\":\"All the information about the Zoom meeting.\",\"properties\":{\"zoom_join_url\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The URL to join a meeting.\",\"example\":\"https://success.zoom.us/j/xxxxxxxx?pwd=xxxxxxxx\"},\"location_type\":{\"type\":\"string\",\"description\":\"Whether it's a Zoom meeting.\",\"enum\":[\"zoom\"]},\"zoom_meeting_data\":{\"type\":\"object\",\"description\":\"The extra info for the meeting.\",\"properties\":{\"zoom_meeting_id\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The unique identifier of the meeting in string format. This field is also known as the meeting number.\",\"example\":\"1234567890\"},\"zoom_meeting_passcode\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The passcode of the meeting.\",\"example\":\"dnjchd\"},\"zoom_personal_meeting_id\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The personal meeting ID in string format.\",\"example\":\"98656786887\"}}},\"zoom_meeting_status\":{\"type\":\"string\",\"description\":\"The status of meeting.\",\"enum\":[\"scheduled\",\"failed\"]}}},{\"type\":\"object\",\"title\":\"Non-online meeting location\",\"description\":\"All the information about the Non-online meeting location.\",\"properties\":{\"custom_location\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The information for a location. May be a phone number or a physical location.\",\"example\":\"AAA office\"},\"location_type\":{\"type\":\"string\",\"description\":\"The location type.\",\"enum\":[\"custom\",\"in-person_meeting\",\"outbound_call\",\"inbound_call\",\"invitee_specified\"]},\"additional_info\":{\"type\":\"string\",\"description\":\"Additional info for location.\",\"example\":\"extra info\"}}}]},\"external_location\":{\"description\":\"The meeting details in the scheduled event.\",\"readOnly\":true,\"oneOf\":[{\"type\":\"object\",\"properties\":{\"kind\":{\"type\":\"string\",\"description\":\"The type of meeting specified in the schedule.\",\"example\":\"zoom\",\"enum\":[\"zoom\",\"google_meet\",\"microsoft_teams\"]},\"meeting_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the meeting in string format.\",\"example\":\"94777886776\"},\"personal_meeting_id\":{\"type\":\"string\",\"description\":\"The personal meeting ID in string format.\",\"example\":\"98656786887\"},\"meeting_passcode\":{\"type\":\"string\",\"description\":\"The meeting passcode.\",\"example\":\"958000\"},\"meeting_description\":{\"type\":\"string\",\"description\":\"The Zoom meeting description.\",\"example\":\"Invite you to meeting\"},\"meeting_join_url\":{\"type\":\"string\",\"description\":\"The URL to join the meeting.\",\"example\":\"https://zoom.us/j/7427752805\"}}}]},\"start_date_time\":{\"type\":\"string\",\"description\":\"The scheduled event's start date and time.\",\"example\":\"2023-12-21T16:00:00Z\"},\"status\":{\"type\":\"string\",\"description\":\"The event status: confirmed or cancelled.\",\"example\":\"confirmed\",\"enum\":[\"confirmed\",\"cancelled\"]},\"summary\":{\"maxLength\":256,\"type\":\"string\",\"description\":\"The schedule name.\",\"example\":\"daily meeting\"},\"updated\":{\"type\":\"string\",\"description\":\"The moment the scheduled event was updated.\",\"readOnly\":true,\"example\":\"2023-12-21T06:18:32.087Z\"},\"created\":{\"type\":\"string\",\"description\":\"The moment the scheduled event was created.\",\"readOnly\":true,\"example\":\"2023-12-21T06:18:32.087Z\"},\"meeting_notes\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The scheduled event's meeting notes.\",\"example\":\"meeting notes\"}},\"additionalProperties\":false},\"tracking\":{\"type\":\"object\",\"description\":\"The information to track the invitee source. This field is only present when UTM parameters are added.\",\"properties\":{\"utm_source\":{\"type\":\"string\",\"description\":\"This field identifies which site sent the traffic and is a required parameter.\",\"example\":\"zoom\"},\"utm_medium\":{\"type\":\"string\",\"description\":\"This field identifies what type of link was used, such as pay-per-click or email.\",\"example\":\"pay-per-clik\"},\"utm_campaign\":{\"type\":\"string\",\"description\":\"This field identifies a specific product promotion or strategic campaign.\",\"example\":\"spring_sale\"},\"utm_content\":{\"type\":\"string\",\"description\":\"This field identifies what specifically was clicked to bring the user to the site, such as a banner ad or a text link. It is often used for A/B testing and content-targeted ads.\",\"example\":\"schedule link\"},\"utm_term\":{\"type\":\"string\",\"description\":\"This field identifies the search terms.\",\"example\":\"learning\"},\"salesforce_ids\":{\"type\":\"array\",\"description\":\"The tracking of Salesforce IDs.\",\"items\":{\"type\":\"object\",\"property\":{\"type\":\"string\",\"description\":\"The Salesforce object ID that represents an entity.\"}}}}},\"rescheduled\":{\"type\":\"boolean\",\"description\":\"Indicates whether this is from a reschedule request.\"},\"created\":{\"type\":\"string\",\"description\":\"The moment the scheduled request was created.\",\"readOnly\":true,\"example\":\"2023-12-21T06:18:32.087Z\"},\"text_phone_number\":{\"type\":\"string\",\"description\":\"The phone number of the invitee to receive text reminders.\",\"example\":\"+1 83798773\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"scheduler.scheduled_event_created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"user_id\\\": \\\"dndhfdjdihd\\\",\\n    \\\"object\\\": {\\n      \\\"invitee_email\\\": \\\"john.doe@example.com\\\",\\n      \\\"invitee_first_name\\\": \\\"John\\\",\\n      \\\"invitee_last_name\\\": \\\"Doe\\\",\\n      \\\"manage_url\\\": \\\"https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx\\\",\\n      \\\"time_zone\\\": \\\"Asia/Shanghai\\\",\\n      \\\"questions_and_answers\\\": [\\n        {\\n          \\\"question\\\": \\\"Please share anything that will help prepare for our meeting.\\\",\\n          \\\"position\\\": 0,\\n          \\\"answer\\\": [\\n            \\\"First line\\\",\\n            \\\"Second line\\\"\\n          ]\\n        }\\n      ],\\n      \\\"scheduled_event\\\": {\\n        \\\"event_id\\\": \\\"woft7torlatbw8ek24bmit5k60\\\",\\n        \\\"attendees\\\": [\\n          {\\n            \\\"display_name\\\": \\\"Jane Doe\\\",\\n            \\\"email\\\": \\\"jane@example.com\\\"\\n          },\\n          {\\n            \\\"display_name\\\": \\\"David Smith\\\",\\n            \\\"email\\\": \\\"david@example.com\\\"\\n          }\\n        ],\\n        \\\"end_date_time\\\": \\\"2023-12-21T16:30:00Z\\\",\\n        \\\"guests\\\": [\\n          \\\"guest1@example.com\\\",\\n          \\\"guest2@example.com\\\"\\n        ],\\n        \\\"location_kind\\\": \\\"zoom\\\",\\n        \\\"location\\\": {\\n          \\\"custom_location\\\": \\\"123 Main St\\\",\\n          \\\"location_type\\\": \\\"in-person_meeting\\\",\\n          \\\"additional_info\\\": \\\"additional information\\\"\\n        },\\n        \\\"external_location\\\": {\\n          \\\"kind\\\": \\\"zoom\\\",\\n          \\\"meeting_id\\\": \\\"94777886776\\\",\\n          \\\"personal_meeting_id\\\": \\\"98656786887\\\",\\n          \\\"meeting_passcode\\\": \\\"958000\\\",\\n          \\\"meeting_description\\\": \\\"Invite you to meeting\\\",\\n          \\\"meeting_join_url\\\": \\\"https://zoom.us/j/7427752805\\\"\\n        },\\n        \\\"start_date_time\\\": \\\"2023-12-21T16:00:00Z\\\",\\n        \\\"status\\\": \\\"confirmed\\\",\\n        \\\"summary\\\": \\\"30 minute meeting\\\",\\n        \\\"updated\\\": \\\"2023-12-21T06:19:28.309Z\\\",\\n        \\\"created\\\": \\\"2023-12-21T06:18:28.309Z\\\",\\n        \\\"meeting_notes\\\": \\\"meeting notes\\\",\\n        \\\"invitee_counters\\\": {\\n          \\\"total\\\": 1,\\n          \\\"active\\\": 1,\\n          \\\"limit\\\": 3\\n        }\\n      },\\n      \\\"tracking\\\": {\\n        \\\"utm_source\\\": \\\"email\\\",\\n        \\\"utm_medium\\\": \\\"newsletter\\\",\\n        \\\"utm_campaign\\\": \\\"spring_sale\\\",\\n        \\\"utm_content\\\": \\\"booking_link\\\",\\n        \\\"utm_term\\\": \\\"shopping\\\",\\n        \\\"salesforce_ids\\\": [\\n          {\\n            \\\"sf_person\\\": \\\"0035g00000AbCdEAAZ\\\"\\n          },\\n          {\\n            \\\"sf_relates\\\": \\\"0015g00000AbCdEAAZ\\\"\\n          }\\n        ]\\n      },\\n      \\\"rescheduled\\\": false,\\n      \\\"created\\\": \\\"2023-12-21T06:18:28.309Z\\\",\\n      \\\"text_phone_number\\\": \\\"+1 83798773\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event was created.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the scheduled event host.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The user ID of the scheduled event host.\"},\"object\":{\"type\":\"object\",\"description\":\"The event information.\",\"properties\":{\"invitee_email\":{\"type\":\"string\",\"description\":\"The email address of the user who scheduled the event.\",\"example\":\"example@example.com\"},\"invitee_first_name\":{\"type\":\"string\",\"description\":\"The first name of the user who scheduled the event.\",\"example\":\"alax\"},\"invitee_last_name\":{\"type\":\"string\",\"description\":\"The last name of the user who scheduled the event.\",\"example\":\"bbbb\"},\"manage_url\":{\"type\":\"string\",\"description\":\"The URL for the user to reschedule or cancel the scheduled event.\",\"example\":\"https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx\"},\"time_zone\":{\"type\":\"string\",\"description\":\"The user's time zone.\",\"example\":\"Asia/Shanghai\"},\"questions_and_answers\":{\"type\":\"array\",\"description\":\"The collection of questions and answers from the booking form.\",\"items\":{\"type\":\"object\",\"required\":[\"question\",\"position\",\"answer\"],\"properties\":{\"question\":{\"type\":\"string\",\"description\":\"The question from the booking page.\",\"example\":\"Please share anything that will help prepare for our meeting.\"},\"position\":{\"type\":\"integer\",\"description\":\"The index of the question.\"},\"answer\":{\"description\":\"The answers to the question.\",\"type\":\"array\",\"items\":{\"type\":\"string\",\"description\":\"The answer to the question.\"}}}}},\"scheduled_event\":{\"type\":\"object\",\"description\":\"The scheduled event information.\",\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the event.\",\"example\":\"woft7torlatbw8ek24bmit5k60\"},\"attendees\":{\"type\":\"array\",\"description\":\"The attendees of the event.\",\"items\":{\"type\":\"object\",\"properties\":{\"display_name\":{\"type\":\"string\",\"description\":\"The attendee's name.\",\"example\":\"bob\"},\"email\":{\"type\":\"string\",\"description\":\"The attendee's email address.\",\"example\":\"abc@zoom.us\"}}}},\"invitee_counters\":{\"type\":\"object\",\"description\":\"The event's guest counters.\",\"properties\":{\"total\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The total count of invitees in the event.\"},\"active\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The count of active invitees in the event.\"},\"limit\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The maximum number of invitees allowed for the event.\"}}},\"end_date_time\":{\"type\":\"string\",\"description\":\"The scheduled event's end date and time.\",\"example\":\"2023-12-21T16:30:00Z\"},\"guests\":{\"type\":\"array\",\"description\":\"The collection of guests.\",\"items\":{\"type\":\"string\",\"description\":\"The guest's email address.\",\"example\":\"tom@zoom.us\"}},\"location_kind\":{\"type\":\"string\",\"description\":\"Unified location type for both location and external_location objects. If location type is online meeting, location info is in external_location object. If not, location info is in location object.\",\"example\":\"zoom\",\"enum\":[\"custom\",\"in-person_meeting\",\"outbound_call\",\"inbound_call\",\"invitee_specified\",\"zoom\",\"google_meet\",\"microsoft_teams\"]},\"location\":{\"oneOf\":[{\"deprecated\":true,\"type\":\"object\",\"title\":\"zoom\",\"description\":\"All the information about the Zoom meeting.\",\"properties\":{\"zoom_join_url\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The URL to join a meeting.\",\"example\":\"https://success.zoom.us/j/xxxxxxxx?pwd=xxxxxxxx\"},\"location_type\":{\"type\":\"string\",\"description\":\"Whether it's a Zoom meeting.\",\"enum\":[\"zoom\"]},\"zoom_meeting_data\":{\"type\":\"object\",\"description\":\"The extra info for the meeting.\",\"properties\":{\"zoom_meeting_id\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The unique identifier of the meeting in string format. This field is also known as the meeting number.\",\"example\":\"1234567890\"},\"zoom_meeting_passcode\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The passcode of the meeting.\",\"example\":\"dnjchd\"},\"zoom_personal_meeting_id\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The personal meeting ID in string format.\",\"example\":\"98656786887\"}}},\"zoom_meeting_status\":{\"type\":\"string\",\"description\":\"The status of meeting.\",\"enum\":[\"scheduled\",\"failed\"]}}},{\"type\":\"object\",\"title\":\"Non-online meeting location\",\"description\":\"All the information about the Non-online meeting location.\",\"properties\":{\"custom_location\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The information for a location. May be a phone number or a physical location.\",\"example\":\"AAA office\"},\"location_type\":{\"type\":\"string\",\"description\":\"The location type.\",\"enum\":[\"custom\",\"in-person_meeting\",\"outbound_call\",\"inbound_call\",\"invitee_specified\"]},\"additional_info\":{\"type\":\"string\",\"description\":\"Additional info for location.\",\"example\":\"extra info\"}}}]},\"external_location\":{\"description\":\"The meeting details in the scheduled event.\",\"readOnly\":true,\"oneOf\":[{\"type\":\"object\",\"properties\":{\"kind\":{\"type\":\"string\",\"description\":\"The type of meeting specified in the schedule.\",\"example\":\"zoom\",\"enum\":[\"zoom\",\"google_meet\",\"microsoft_teams\"]},\"meeting_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the meeting in string format.\",\"example\":\"94777886776\"},\"personal_meeting_id\":{\"type\":\"string\",\"description\":\"The personal meeting ID in string format.\",\"example\":\"98656786887\"},\"meeting_passcode\":{\"type\":\"string\",\"description\":\"The meeting passcode.\",\"example\":\"958000\"},\"meeting_description\":{\"type\":\"string\",\"description\":\"The Zoom meeting description.\",\"example\":\"Invite you to meeting\"},\"meeting_join_url\":{\"type\":\"string\",\"description\":\"The URL to join the meeting.\",\"example\":\"https://zoom.us/j/7427752805\"}}}]},\"start_date_time\":{\"type\":\"string\",\"description\":\"The scheduled event's start date and time.\",\"example\":\"2023-12-21T16:00:00Z\"},\"status\":{\"type\":\"string\",\"description\":\"The event status: confirmed or cancelled.\",\"example\":\"confirmed\",\"enum\":[\"confirmed\",\"cancelled\"]},\"summary\":{\"maxLength\":256,\"type\":\"string\",\"description\":\"The schedule name.\",\"example\":\"daily meeting\"},\"updated\":{\"type\":\"string\",\"description\":\"The moment the scheduled event was updated.\",\"readOnly\":true,\"example\":\"2023-12-21T06:18:32.087Z\"},\"created\":{\"type\":\"string\",\"description\":\"The moment the scheduled event was created.\",\"readOnly\":true,\"example\":\"2023-12-21T06:18:32.087Z\"},\"meeting_notes\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The scheduled event's meeting notes.\",\"example\":\"meeting notes\"}},\"additionalProperties\":false},\"tracking\":{\"type\":\"object\",\"description\":\"The information to track the invitee source. This field is only present when UTM parameters are added.\",\"properties\":{\"utm_source\":{\"type\":\"string\",\"description\":\"This field identifies which site sent the traffic and is a required parameter.\",\"example\":\"zoom\"},\"utm_medium\":{\"type\":\"string\",\"description\":\"This field identifies what type of link was used, such as pay-per-click or email.\",\"example\":\"pay-per-clik\"},\"utm_campaign\":{\"type\":\"string\",\"description\":\"This field identifies a specific product promotion or strategic campaign.\",\"example\":\"spring_sale\"},\"utm_content\":{\"type\":\"string\",\"description\":\"This field identifies what specifically was clicked to bring the user to the site, such as a banner ad or a text link. It is often used for A/B testing and content-targeted ads.\",\"example\":\"schedule link\"},\"utm_term\":{\"type\":\"string\",\"description\":\"This field identifies the search terms.\",\"example\":\"learning\"},\"salesforce_ids\":{\"type\":\"array\",\"description\":\"The tracking of Salesforce IDs.\",\"items\":{\"type\":\"object\",\"property\":{\"type\":\"string\",\"description\":\"The Salesforce object ID that represents an entity.\"}}}}},\"rescheduled\":{\"type\":\"boolean\",\"description\":\"Indicates whether this is from a reschedule request.\"},\"created\":{\"type\":\"string\",\"description\":\"The moment the scheduled request was created.\",\"readOnly\":true,\"example\":\"2023-12-21T06:18:32.087Z\"},\"text_phone_number\":{\"type\":\"string\",\"description\":\"The phone number of the invitee to receive text reminders.\",\"example\":\"+1 83798773\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"scheduler.scheduled_event_created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"user_id\\\": \\\"dndhfdjdihd\\\",\\n    \\\"object\\\": {\\n      \\\"invitee_email\\\": \\\"john.doe@example.com\\\",\\n      \\\"invitee_first_name\\\": \\\"John\\\",\\n      \\\"invitee_last_name\\\": \\\"Doe\\\",\\n      \\\"manage_url\\\": \\\"https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx\\\",\\n      \\\"time_zone\\\": \\\"Asia/Shanghai\\\",\\n      \\\"questions_and_answers\\\": [\\n        {\\n          \\\"question\\\": \\\"Please share anything that will help prepare for our meeting.\\\",\\n          \\\"position\\\": 0,\\n          \\\"answer\\\": [\\n            \\\"First line\\\",\\n            \\\"Second line\\\"\\n          ]\\n        }\\n      ],\\n      \\\"scheduled_event\\\": {\\n        \\\"event_id\\\": \\\"woft7torlatbw8ek24bmit5k60\\\",\\n        \\\"attendees\\\": [\\n          {\\n            \\\"display_name\\\": \\\"Jane Doe\\\",\\n            \\\"email\\\": \\\"jane@example.com\\\"\\n          },\\n          {\\n            \\\"display_name\\\": \\\"David Smith\\\",\\n            \\\"email\\\": \\\"david@example.com\\\"\\n          }\\n        ],\\n        \\\"end_date_time\\\": \\\"2023-12-21T16:30:00Z\\\",\\n        \\\"guests\\\": [\\n          \\\"guest1@example.com\\\",\\n          \\\"guest2@example.com\\\"\\n        ],\\n        \\\"location_kind\\\": \\\"zoom\\\",\\n        \\\"location\\\": {\\n          \\\"custom_location\\\": \\\"123 Main St\\\",\\n          \\\"location_type\\\": \\\"in-person_meeting\\\",\\n          \\\"additional_info\\\": \\\"additional information\\\"\\n        },\\n        \\\"external_location\\\": {\\n          \\\"kind\\\": \\\"zoom\\\",\\n          \\\"meeting_id\\\": \\\"94777886776\\\",\\n          \\\"personal_meeting_id\\\": \\\"98656786887\\\",\\n          \\\"meeting_passcode\\\": \\\"958000\\\",\\n          \\\"meeting_description\\\": \\\"Invite you to meeting\\\",\\n          \\\"meeting_join_url\\\": \\\"https://zoom.us/j/7427752805\\\"\\n        },\\n        \\\"start_date_time\\\": \\\"2023-12-21T16:00:00Z\\\",\\n        \\\"status\\\": \\\"confirmed\\\",\\n        \\\"summary\\\": \\\"30 minute meeting\\\",\\n        \\\"updated\\\": \\\"2023-12-21T06:19:28.309Z\\\",\\n        \\\"created\\\": \\\"2023-12-21T06:18:28.309Z\\\",\\n        \\\"meeting_notes\\\": \\\"meeting notes\\\",\\n        \\\"invitee_counters\\\": {\\n          \\\"total\\\": 1,\\n          \\\"active\\\": 1,\\n          \\\"limit\\\": 3\\n        }\\n      },\\n      \\\"tracking\\\": {\\n        \\\"utm_source\\\": \\\"email\\\",\\n        \\\"utm_medium\\\": \\\"newsletter\\\",\\n        \\\"utm_campaign\\\": \\\"spring_sale\\\",\\n        \\\"utm_content\\\": \\\"booking_link\\\",\\n        \\\"utm_term\\\": \\\"shopping\\\",\\n        \\\"salesforce_ids\\\": [\\n          {\\n            \\\"sf_person\\\": \\\"0035g00000AbCdEAAZ\\\"\\n          },\\n          {\\n            \\\"sf_relates\\\": \\\"0015g00000AbCdEAAZ\\\"\\n          }\\n        ]\\n      },\\n      \\\"rescheduled\\\": false,\\n      \\\"created\\\": \\\"2023-12-21T06:18:28.309Z\\\",\\n      \\\"text_phone_number\\\": \\\"+1 83798773\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"scheduler.scheduled_event_canceled\":{\"post\":{\"tags\":[\"scheduler\"],\"operationId\":\"scheduler.scheduled_event_canceled\",\"requestBody\":{\"description\":\"# scheduler.scheduled_event_canceled\\nThe **Scheduled event canceled** event triggers each time the host or booker cancels the scheduled event.\\n\\n## Prerequisites\\nEnable **Event subscriptions** for your Marketplace app with:\\n\\n* A valid **Event notification endpoint URL**.\\n* Enable the **Scheduled event canceled** subscription under the **Scheduler** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `scheduler:read:admin`,`scheduler:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `scheduler:read:scheduled_event`,`scheduler:read:scheduled_event:admin`\\n\\n**Event type**: `scheduler.scheduled_event_canceled`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event was created.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the scheduled event host.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The user ID of the scheduled event host.\"},\"object\":{\"type\":\"object\",\"description\":\"The information about the event.\",\"properties\":{\"invitee_email\":{\"type\":\"string\",\"description\":\"The email address of the user who scheduled the event.\",\"example\":\"example@example.com\"},\"invitee_first_name\":{\"type\":\"string\",\"description\":\"The first name of the user who scheduled the event.\",\"example\":\"alax\"},\"invitee_last_name\":{\"type\":\"string\",\"description\":\"The last name of the user who scheduled the event.\",\"example\":\"bbbb\"},\"manage_url\":{\"type\":\"string\",\"description\":\"The URL for the user to reschedule or cancel the scheduled event.\",\"example\":\"https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx\"},\"cancellation\":{\"type\":\"object\",\"description\":\"The scheduled event cancellation information.\",\"properties\":{\"canceled_by\":{\"type\":\"string\",\"description\":\"The name or email address of the canceler.\",\"example\":\"aa@bb.c\"},\"canceler_type\":{\"type\":\"string\",\"description\":\"The user type of the person who canceled.\",\"example\":\"host\",\"enum\":[\"host\",\"invitee\"]},\"cancel_at\":{\"type\":\"string\",\"description\":\"The date and time when this scheduled event was canceled.\",\"example\":\"2023-12-21T16:30:00+08:00\"},\"reason\":{\"type\":\"string\",\"description\":\"The reason for canceling this scheduled event.\",\"example\":\"Time conflict\"}}},\"time_zone\":{\"type\":\"string\",\"description\":\"The user's time zone.\",\"example\":\"Asia/Shanghai\"},\"questions_and_answers\":{\"type\":\"array\",\"description\":\"All questions and answers.\",\"items\":{\"type\":\"object\",\"required\":[\"question\",\"position\",\"answer\"],\"properties\":{\"question\":{\"type\":\"string\",\"description\":\"The question on the booking page.\",\"example\":\"Please share anything that will help prepare for our meeting.\"},\"position\":{\"type\":\"integer\",\"description\":\"The index of the question.\"},\"answer\":{\"description\":\"The answers to the question.\",\"type\":\"array\",\"items\":{\"type\":\"string\",\"description\":\"The answer to the question.\"}}}}},\"scheduled_event\":{\"type\":\"object\",\"description\":\"The information about the scheduled event.\",\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the event.\",\"example\":\"woft7torlatbw8ek24bmit5k60\"},\"attendees\":{\"type\":\"array\",\"description\":\"The hosts of the scheduled event.\",\"items\":{\"type\":\"object\",\"properties\":{\"display_name\":{\"type\":\"string\",\"description\":\"The attendee's name.\",\"example\":\"bob\"},\"email\":{\"type\":\"string\",\"description\":\"The attendee's email address.\",\"example\":\"abc@zoom.us\"}}}},\"invitee_counters\":{\"type\":\"object\",\"description\":\"The event's guest counters.\",\"properties\":{\"total\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The total count of invitees in the event.\"},\"active\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The count of active invitees in the event.\"},\"limit\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The maximum limit of invitees in the event.\"}}},\"end_date_time\":{\"type\":\"string\",\"description\":\"The scheduled event's end date and time.\",\"example\":\"2023-12-21T16:30:00Z\"},\"guests\":{\"type\":\"array\",\"description\":\"The collection of guests.\",\"items\":{\"type\":\"string\",\"description\":\"The guest's email address.\",\"example\":\"tom@zoom.us\"}},\"location_kind\":{\"type\":\"string\",\"description\":\"Unified location type for both location and external_location objects. If location type is online meeting, location info is in external_location object. If not, location info is in location object.\",\"example\":\"zoom\",\"enum\":[\"custom\",\"in-person_meeting\",\"outbound_call\",\"inbound_call\",\"invitee_specified\",\"zoom\",\"google_meet\",\"microsoft_teams\"]},\"location\":{\"oneOf\":[{\"deprecated\":true,\"type\":\"object\",\"title\":\"zoom\",\"description\":\"All the information about the Zoom meeting.\",\"properties\":{\"zoom_join_url\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The URL to join a meeting.\",\"example\":\"https://success.zoom.us/j/xxxxxxxx?pwd=xxxxxxxx\"},\"location_type\":{\"type\":\"string\",\"description\":\"Whether it's a Zoom meeting.\",\"enum\":[\"zoom\"]},\"zoom_meeting_data\":{\"type\":\"object\",\"description\":\"The additional information for the meeting.\",\"properties\":{\"zoom_meeting_id\":{\"type\":\"string\",\"maxLength\":1024,\"description\":\"The unique identifier of the meeting in string format. This field is also known as the meeting number.\",\"example\":\"1234567890\"},\"zoom_meeting_passcode\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The passcode for the meeting.\",\"example\":\"dnjchd\"},\"zoom_personal_meeting_id\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The personal meeting ID in string format.\",\"example\":\"98656786887\"}}},\"zoom_meeting_status\":{\"type\":\"string\",\"description\":\"The status of the meeting.\",\"enum\":[\"scheduled\",\"failed\"]}}},{\"type\":\"object\",\"title\":\"Non-online meeting location\",\"description\":\"All the information about the non-online meeting location.\",\"properties\":{\"custom_location\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The information for a location. May be a phone number or a physical location.\",\"example\":\"AAA office\"},\"location_type\":{\"type\":\"string\",\"description\":\"The location type.\",\"enum\":[\"custom\",\"in-person_meeting\",\"outbound_call\",\"inbound_call\",\"invitee_specified\"]},\"additional_info\":{\"type\":\"string\",\"description\":\"Additional information for the location.\",\"example\":\"extra info\"}}}]},\"external_location\":{\"description\":\"The meeting details in the scheduled event.\",\"readOnly\":true,\"oneOf\":[{\"type\":\"object\",\"properties\":{\"kind\":{\"type\":\"string\",\"description\":\"The type of meeting specified in the schedule.\",\"example\":\"zoom\",\"enum\":[\"zoom\",\"google_meet\",\"microsoft_teams\"]},\"meeting_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the meeting in string format.\",\"example\":\"94777886776\"},\"personal_meeting_id\":{\"type\":\"string\",\"description\":\"The personal meeting ID in string format.\",\"example\":\"98656786887\"},\"meeting_passcode\":{\"type\":\"string\",\"description\":\"The passcode for the meeting.\",\"example\":\"958000\"},\"meeting_description\":{\"type\":\"string\",\"description\":\"The description of the Zoom meeting.\",\"example\":\"Invite you to meeting\"},\"meeting_join_url\":{\"type\":\"string\",\"description\":\"The URL to join the meeting.\",\"example\":\"https://zoom.us/j/7427752805\"}}}]},\"start_date_time\":{\"type\":\"string\",\"description\":\"The scheduled event's start date and time.\",\"example\":\"2023-12-21T16:00:00Z\"},\"status\":{\"type\":\"string\",\"description\":\"The status of the event: confirmed or cancelled.\",\"example\":\"confirmed\",\"enum\":[\"confirmed\",\"cancelled\"]},\"summary\":{\"maxLength\":256,\"type\":\"string\",\"description\":\"The name of the schedule.\",\"example\":\"daily meeting\"},\"updated\":{\"type\":\"string\",\"description\":\"The moment the scheduled event was updated.\",\"readOnly\":true,\"example\":\"2023-12-21T06:18:32.087Z\"},\"created\":{\"type\":\"string\",\"description\":\"The moment the scheduled event was created.\",\"readOnly\":true,\"example\":\"2023-12-21T06:18:32.087Z\"},\"meeting_notes\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The meeting notes for the scheduled event.\",\"example\":\"meeting notes\"}},\"additionalProperties\":false},\"tracking\":{\"type\":\"object\",\"description\":\"The information to track the source of the invitee.\",\"properties\":{\"utm_source\":{\"type\":\"string\",\"description\":\"This field identifies which site sent the traffic, and is a required parameter.\",\"example\":\"zoom\"},\"utm_medium\":{\"type\":\"string\",\"description\":\"This field identifies what type of link was used, such as pay-per-click or email.\",\"example\":\"pay-per-clik\"},\"utm_campaign\":{\"type\":\"string\",\"description\":\"This field identifies a specific product promotion or strategic campaign.\",\"example\":\"spring_sale\"},\"utm_content\":{\"type\":\"string\",\"description\":\"This field identifies what specifically was clicked to bring the user to the site, such as a banner ad or a text link. It is often used for A/B testing and content-targeted ads.\",\"example\":\"schedule link\"},\"utm_term\":{\"type\":\"string\",\"description\":\"This field identifies search terms.\",\"example\":\"learning\"},\"salesforce_ids\":{\"type\":\"array\",\"description\":\"The tracking of Salesforce IDs.\",\"items\":{\"type\":\"object\",\"property\":{\"type\":\"string\",\"description\":\"The Salesforce object ID that represents an entity.\"}}}}},\"rescheduled\":{\"type\":\"boolean\",\"description\":\"Whether it's from a reschedule request.\"},\"created\":{\"type\":\"string\",\"description\":\"The moment the scheduled request was created.\",\"readOnly\":true,\"example\":\"2023-12-21T06:18:32.087Z\"},\"text_phone_number\":{\"type\":\"string\",\"description\":\"The phone number of the invitee to receive text reminders.\",\"example\":\"+1 83798773\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"scheduler.scheduled_event_canceled\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"user_id\\\": \\\"dndhfdjdihd\\\",\\n    \\\"object\\\": {\\n      \\\"invitee_email\\\": \\\"john.doe@example.com\\\",\\n      \\\"invitee_first_name\\\": \\\"John\\\",\\n      \\\"invitee_last_name\\\": \\\"Doe\\\",\\n      \\\"manage_url\\\": \\\"https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx\\\",\\n      \\\"cancellation\\\": {\\n        \\\"canceled_by\\\": \\\"aa@bb.c\\\",\\n        \\\"canceler_type\\\": \\\"host\\\",\\n        \\\"cancel_at\\\": \\\"2023-12-21T16:30:00+08:00\\\",\\n        \\\"reason\\\": \\\"Time conflict\\\"\\n      },\\n      \\\"time_zone\\\": \\\"Asia/Shanghai\\\",\\n      \\\"questions_and_answers\\\": [\\n        {\\n          \\\"question\\\": \\\"Please share anything that will help prepare for our meeting.\\\",\\n          \\\"position\\\": 0,\\n          \\\"answer\\\": [\\n            \\\"First line\\\",\\n            \\\"Second line\\\"\\n          ]\\n        }\\n      ],\\n      \\\"scheduled_event\\\": {\\n        \\\"event_id\\\": \\\"woft7torlatbw8ek24bmit5k60\\\",\\n        \\\"attendees\\\": [\\n          {\\n            \\\"display_name\\\": \\\"Jane Doe\\\",\\n            \\\"email\\\": \\\"jane@example.com\\\"\\n          },\\n          {\\n            \\\"display_name\\\": \\\"David Smith\\\",\\n            \\\"email\\\": \\\"david@example.com\\\"\\n          }\\n        ],\\n        \\\"end_date_time\\\": \\\"2023-12-21T16:30:00Z\\\",\\n        \\\"guests\\\": [\\n          \\\"guest1@example.com\\\",\\n          \\\"guest2@example.com\\\"\\n        ],\\n        \\\"location_kind\\\": \\\"custom\\\",\\n        \\\"location\\\": {\\n          \\\"custom_location\\\": \\\"123 Main St\\\",\\n          \\\"location_type\\\": \\\"in-person_meeting\\\",\\n          \\\"additional_info\\\": \\\"additional information\\\"\\n        },\\n        \\\"external_location\\\": {\\n          \\\"kind\\\": \\\"zoom\\\",\\n          \\\"meeting_id\\\": \\\"94777886776\\\",\\n          \\\"personal_meeting_id\\\": \\\"98656786887\\\",\\n          \\\"meeting_passcode\\\": \\\"958000\\\",\\n          \\\"meeting_description\\\": \\\"Invite you to meeting\\\",\\n          \\\"meeting_join_url\\\": \\\"https://zoom.us/j/7427752805\\\"\\n        },\\n        \\\"start_date_time\\\": \\\"2023-12-21T16:00:00Z\\\",\\n        \\\"status\\\": \\\"confirmed\\\",\\n        \\\"summary\\\": \\\"30 minute meeting\\\",\\n        \\\"updated\\\": \\\"2023-12-21T06:19:28.309Z\\\",\\n        \\\"created\\\": \\\"2023-12-21T06:18:28.309Z\\\",\\n        \\\"meeting_notes\\\": \\\"meeting notes\\\",\\n        \\\"invitee_counters\\\": {\\n          \\\"total\\\": 1,\\n          \\\"active\\\": 1,\\n          \\\"limit\\\": 3\\n        }\\n      },\\n      \\\"tracking\\\": {\\n        \\\"utm_source\\\": \\\"email\\\",\\n        \\\"utm_medium\\\": \\\"newsletter\\\",\\n        \\\"utm_campaign\\\": \\\"spring_sale\\\",\\n        \\\"utm_content\\\": \\\"booking_link\\\",\\n        \\\"utm_term\\\": \\\"shopping\\\",\\n        \\\"salesforce_ids\\\": [\\n          {\\n            \\\"sf_person\\\": \\\"0035g00000AbCdEAAZ\\\"\\n          },\\n          {\\n            \\\"sf_relates\\\": \\\"0015g00000AbCdEAAZ\\\"\\n          }\\n        ]\\n      },\\n      \\\"rescheduled\\\": false,\\n      \\\"created\\\": \\\"2023-12-21T06:18:28.309Z\\\",\\n      \\\"text_phone_number\\\": \\\"+1 48837847\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event was created.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the scheduled event host.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The user ID of the scheduled event host.\"},\"object\":{\"type\":\"object\",\"description\":\"The information about the event.\",\"properties\":{\"invitee_email\":{\"type\":\"string\",\"description\":\"The email address of the user who scheduled the event.\",\"example\":\"example@example.com\"},\"invitee_first_name\":{\"type\":\"string\",\"description\":\"The first name of the user who scheduled the event.\",\"example\":\"alax\"},\"invitee_last_name\":{\"type\":\"string\",\"description\":\"The last name of the user who scheduled the event.\",\"example\":\"bbbb\"},\"manage_url\":{\"type\":\"string\",\"description\":\"The URL for the user to reschedule or cancel the scheduled event.\",\"example\":\"https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx\"},\"cancellation\":{\"type\":\"object\",\"description\":\"The scheduled event cancellation information.\",\"properties\":{\"canceled_by\":{\"type\":\"string\",\"description\":\"The name or email address of the canceler.\",\"example\":\"aa@bb.c\"},\"canceler_type\":{\"type\":\"string\",\"description\":\"The user type of the person who canceled.\",\"example\":\"host\",\"enum\":[\"host\",\"invitee\"]},\"cancel_at\":{\"type\":\"string\",\"description\":\"The date and time when this scheduled event was canceled.\",\"example\":\"2023-12-21T16:30:00+08:00\"},\"reason\":{\"type\":\"string\",\"description\":\"The reason for canceling this scheduled event.\",\"example\":\"Time conflict\"}}},\"time_zone\":{\"type\":\"string\",\"description\":\"The user's time zone.\",\"example\":\"Asia/Shanghai\"},\"questions_and_answers\":{\"type\":\"array\",\"description\":\"All questions and answers.\",\"items\":{\"type\":\"object\",\"required\":[\"question\",\"position\",\"answer\"],\"properties\":{\"question\":{\"type\":\"string\",\"description\":\"The question on the booking page.\",\"example\":\"Please share anything that will help prepare for our meeting.\"},\"position\":{\"type\":\"integer\",\"description\":\"The index of the question.\"},\"answer\":{\"description\":\"The answers to the question.\",\"type\":\"array\",\"items\":{\"type\":\"string\",\"description\":\"The answer to the question.\"}}}}},\"scheduled_event\":{\"type\":\"object\",\"description\":\"The information about the scheduled event.\",\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the event.\",\"example\":\"woft7torlatbw8ek24bmit5k60\"},\"attendees\":{\"type\":\"array\",\"description\":\"The hosts of the scheduled event.\",\"items\":{\"type\":\"object\",\"properties\":{\"display_name\":{\"type\":\"string\",\"description\":\"The attendee's name.\",\"example\":\"bob\"},\"email\":{\"type\":\"string\",\"description\":\"The attendee's email address.\",\"example\":\"abc@zoom.us\"}}}},\"invitee_counters\":{\"type\":\"object\",\"description\":\"The event's guest counters.\",\"properties\":{\"total\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The total count of invitees in the event.\"},\"active\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The count of active invitees in the event.\"},\"limit\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The maximum limit of invitees in the event.\"}}},\"end_date_time\":{\"type\":\"string\",\"description\":\"The scheduled event's end date and time.\",\"example\":\"2023-12-21T16:30:00Z\"},\"guests\":{\"type\":\"array\",\"description\":\"The collection of guests.\",\"items\":{\"type\":\"string\",\"description\":\"The guest's email address.\",\"example\":\"tom@zoom.us\"}},\"location_kind\":{\"type\":\"string\",\"description\":\"Unified location type for both location and external_location objects. If location type is online meeting, location info is in external_location object. If not, location info is in location object.\",\"example\":\"zoom\",\"enum\":[\"custom\",\"in-person_meeting\",\"outbound_call\",\"inbound_call\",\"invitee_specified\",\"zoom\",\"google_meet\",\"microsoft_teams\"]},\"location\":{\"oneOf\":[{\"deprecated\":true,\"type\":\"object\",\"title\":\"zoom\",\"description\":\"All the information about the Zoom meeting.\",\"properties\":{\"zoom_join_url\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The URL to join a meeting.\",\"example\":\"https://success.zoom.us/j/xxxxxxxx?pwd=xxxxxxxx\"},\"location_type\":{\"type\":\"string\",\"description\":\"Whether it's a Zoom meeting.\",\"enum\":[\"zoom\"]},\"zoom_meeting_data\":{\"type\":\"object\",\"description\":\"The additional information for the meeting.\",\"properties\":{\"zoom_meeting_id\":{\"type\":\"string\",\"maxLength\":1024,\"description\":\"The unique identifier of the meeting in string format. This field is also known as the meeting number.\",\"example\":\"1234567890\"},\"zoom_meeting_passcode\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The passcode for the meeting.\",\"example\":\"dnjchd\"},\"zoom_personal_meeting_id\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The personal meeting ID in string format.\",\"example\":\"98656786887\"}}},\"zoom_meeting_status\":{\"type\":\"string\",\"description\":\"The status of the meeting.\",\"enum\":[\"scheduled\",\"failed\"]}}},{\"type\":\"object\",\"title\":\"Non-online meeting location\",\"description\":\"All the information about the non-online meeting location.\",\"properties\":{\"custom_location\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The information for a location. May be a phone number or a physical location.\",\"example\":\"AAA office\"},\"location_type\":{\"type\":\"string\",\"description\":\"The location type.\",\"enum\":[\"custom\",\"in-person_meeting\",\"outbound_call\",\"inbound_call\",\"invitee_specified\"]},\"additional_info\":{\"type\":\"string\",\"description\":\"Additional information for the location.\",\"example\":\"extra info\"}}}]},\"external_location\":{\"description\":\"The meeting details in the scheduled event.\",\"readOnly\":true,\"oneOf\":[{\"type\":\"object\",\"properties\":{\"kind\":{\"type\":\"string\",\"description\":\"The type of meeting specified in the schedule.\",\"example\":\"zoom\",\"enum\":[\"zoom\",\"google_meet\",\"microsoft_teams\"]},\"meeting_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the meeting in string format.\",\"example\":\"94777886776\"},\"personal_meeting_id\":{\"type\":\"string\",\"description\":\"The personal meeting ID in string format.\",\"example\":\"98656786887\"},\"meeting_passcode\":{\"type\":\"string\",\"description\":\"The passcode for the meeting.\",\"example\":\"958000\"},\"meeting_description\":{\"type\":\"string\",\"description\":\"The description of the Zoom meeting.\",\"example\":\"Invite you to meeting\"},\"meeting_join_url\":{\"type\":\"string\",\"description\":\"The URL to join the meeting.\",\"example\":\"https://zoom.us/j/7427752805\"}}}]},\"start_date_time\":{\"type\":\"string\",\"description\":\"The scheduled event's start date and time.\",\"example\":\"2023-12-21T16:00:00Z\"},\"status\":{\"type\":\"string\",\"description\":\"The status of the event: confirmed or cancelled.\",\"example\":\"confirmed\",\"enum\":[\"confirmed\",\"cancelled\"]},\"summary\":{\"maxLength\":256,\"type\":\"string\",\"description\":\"The name of the schedule.\",\"example\":\"daily meeting\"},\"updated\":{\"type\":\"string\",\"description\":\"The moment the scheduled event was updated.\",\"readOnly\":true,\"example\":\"2023-12-21T06:18:32.087Z\"},\"created\":{\"type\":\"string\",\"description\":\"The moment the scheduled event was created.\",\"readOnly\":true,\"example\":\"2023-12-21T06:18:32.087Z\"},\"meeting_notes\":{\"maxLength\":1024,\"type\":\"string\",\"description\":\"The meeting notes for the scheduled event.\",\"example\":\"meeting notes\"}},\"additionalProperties\":false},\"tracking\":{\"type\":\"object\",\"description\":\"The information to track the source of the invitee.\",\"properties\":{\"utm_source\":{\"type\":\"string\",\"description\":\"This field identifies which site sent the traffic, and is a required parameter.\",\"example\":\"zoom\"},\"utm_medium\":{\"type\":\"string\",\"description\":\"This field identifies what type of link was used, such as pay-per-click or email.\",\"example\":\"pay-per-clik\"},\"utm_campaign\":{\"type\":\"string\",\"description\":\"This field identifies a specific product promotion or strategic campaign.\",\"example\":\"spring_sale\"},\"utm_content\":{\"type\":\"string\",\"description\":\"This field identifies what specifically was clicked to bring the user to the site, such as a banner ad or a text link. It is often used for A/B testing and content-targeted ads.\",\"example\":\"schedule link\"},\"utm_term\":{\"type\":\"string\",\"description\":\"This field identifies search terms.\",\"example\":\"learning\"},\"salesforce_ids\":{\"type\":\"array\",\"description\":\"The tracking of Salesforce IDs.\",\"items\":{\"type\":\"object\",\"property\":{\"type\":\"string\",\"description\":\"The Salesforce object ID that represents an entity.\"}}}}},\"rescheduled\":{\"type\":\"boolean\",\"description\":\"Whether it's from a reschedule request.\"},\"created\":{\"type\":\"string\",\"description\":\"The moment the scheduled request was created.\",\"readOnly\":true,\"example\":\"2023-12-21T06:18:32.087Z\"},\"text_phone_number\":{\"type\":\"string\",\"description\":\"The phone number of the invitee to receive text reminders.\",\"example\":\"+1 83798773\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"scheduler.scheduled_event_canceled\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"user_id\\\": \\\"dndhfdjdihd\\\",\\n    \\\"object\\\": {\\n      \\\"invitee_email\\\": \\\"john.doe@example.com\\\",\\n      \\\"invitee_first_name\\\": \\\"John\\\",\\n      \\\"invitee_last_name\\\": \\\"Doe\\\",\\n      \\\"manage_url\\\": \\\"https://scheduler.zoom.us/manage/m0o1uf25uq89wllenxxxxxxxxxxxx\\\",\\n      \\\"cancellation\\\": {\\n        \\\"canceled_by\\\": \\\"aa@bb.c\\\",\\n        \\\"canceler_type\\\": \\\"host\\\",\\n        \\\"cancel_at\\\": \\\"2023-12-21T16:30:00+08:00\\\",\\n        \\\"reason\\\": \\\"Time conflict\\\"\\n      },\\n      \\\"time_zone\\\": \\\"Asia/Shanghai\\\",\\n      \\\"questions_and_answers\\\": [\\n        {\\n          \\\"question\\\": \\\"Please share anything that will help prepare for our meeting.\\\",\\n          \\\"position\\\": 0,\\n          \\\"answer\\\": [\\n            \\\"First line\\\",\\n            \\\"Second line\\\"\\n          ]\\n        }\\n      ],\\n      \\\"scheduled_event\\\": {\\n        \\\"event_id\\\": \\\"woft7torlatbw8ek24bmit5k60\\\",\\n        \\\"attendees\\\": [\\n          {\\n            \\\"display_name\\\": \\\"Jane Doe\\\",\\n            \\\"email\\\": \\\"jane@example.com\\\"\\n          },\\n          {\\n            \\\"display_name\\\": \\\"David Smith\\\",\\n            \\\"email\\\": \\\"david@example.com\\\"\\n          }\\n        ],\\n        \\\"end_date_time\\\": \\\"2023-12-21T16:30:00Z\\\",\\n        \\\"guests\\\": [\\n          \\\"guest1@example.com\\\",\\n          \\\"guest2@example.com\\\"\\n        ],\\n        \\\"location_kind\\\": \\\"custom\\\",\\n        \\\"location\\\": {\\n          \\\"custom_location\\\": \\\"123 Main St\\\",\\n          \\\"location_type\\\": \\\"in-person_meeting\\\",\\n          \\\"additional_info\\\": \\\"additional information\\\"\\n        },\\n        \\\"external_location\\\": {\\n          \\\"kind\\\": \\\"zoom\\\",\\n          \\\"meeting_id\\\": \\\"94777886776\\\",\\n          \\\"personal_meeting_id\\\": \\\"98656786887\\\",\\n          \\\"meeting_passcode\\\": \\\"958000\\\",\\n          \\\"meeting_description\\\": \\\"Invite you to meeting\\\",\\n          \\\"meeting_join_url\\\": \\\"https://zoom.us/j/7427752805\\\"\\n        },\\n        \\\"start_date_time\\\": \\\"2023-12-21T16:00:00Z\\\",\\n        \\\"status\\\": \\\"confirmed\\\",\\n        \\\"summary\\\": \\\"30 minute meeting\\\",\\n        \\\"updated\\\": \\\"2023-12-21T06:19:28.309Z\\\",\\n        \\\"created\\\": \\\"2023-12-21T06:18:28.309Z\\\",\\n        \\\"meeting_notes\\\": \\\"meeting notes\\\",\\n        \\\"invitee_counters\\\": {\\n          \\\"total\\\": 1,\\n          \\\"active\\\": 1,\\n          \\\"limit\\\": 3\\n        }\\n      },\\n      \\\"tracking\\\": {\\n        \\\"utm_source\\\": \\\"email\\\",\\n        \\\"utm_medium\\\": \\\"newsletter\\\",\\n        \\\"utm_campaign\\\": \\\"spring_sale\\\",\\n        \\\"utm_content\\\": \\\"booking_link\\\",\\n        \\\"utm_term\\\": \\\"shopping\\\",\\n        \\\"salesforce_ids\\\": [\\n          {\\n            \\\"sf_person\\\": \\\"0035g00000AbCdEAAZ\\\"\\n          },\\n          {\\n            \\\"sf_relates\\\": \\\"0015g00000AbCdEAAZ\\\"\\n          }\\n        ]\\n      },\\n      \\\"rescheduled\\\": false,\\n      \\\"created\\\": \\\"2023-12-21T06:18:28.309Z\\\",\\n      \\\"text_phone_number\\\": \\\"+1 48837847\\\"\\n    }\\n  }\\n}\"}}}}}}}}},\"servers\":[{\"url\":\"https://{your-endpoint-url}\"}]}",
    "frontmatter": {
      "title": "Scheduler Webhooks",
      "keywords": "zoom, api, webhooks, scheduler",
      "description": "Scheduler webhooks allow developers to receive events for [Scheduler](https://developers.zoom.us/docs/zoom-scheduler/). ",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
