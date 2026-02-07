# Events Webhooks

- Source URL: https://developers.zoom.us/docs/api/events/events/
- Snapshot path: docs/api/events/events
- Generated (UTC): 2026-02-07T22:40:10.786912+00:00

## Frontmatter

```json
{
  "title": "Events Webhooks",
  "keywords": "zoom, marketplace, webhook, event, create, update, delete, real-time, data, summary, qos, quality, service",
  "description": "The Webinars Plus & Events webhooks allow developers to receive events for [Webinars Plus & Events](https://developers.zoom.us/docs/zoom-events/) features.",
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
    "title": "Events",
    "description": "The Webinars Plus & Events webhooks allow developers to receive events for [Webinars Plus & Events](https://developers.zoom.us/docs/zoom-events/) features.",
    "version": "1.0.0"
  },
  "paths": {
    "zoom_events.session_deleted": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.session_deleted",
        "requestBody": {
          "description": "# zoom_events.session_deleted\nThe **Session Deleted** event is triggered every time a user in your account or your app, deletes a session in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host or a co-editor can delete a session.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Session Deleted** subscription enabled under the **Zoom Events** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_sessions:write:admin`,`zoom_events_sessions:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:session`,`zoom_events:read:session:admin`\n\n**Event type**: `zoom_events.session_deleted`\n",
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
                    "description": "The name of the event. zoom_events.event.session_deleted."
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
                        "description": "The account ID of the host who deleted the session post event publishing."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the session.",
                        "required": [
                          "event_id",
                          "host_id",
                          "session_id",
                          "name",
                          "type",
                          "start_time",
                          "end_time",
                          "timezone"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The Zoom Events ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "session_id": {
                            "type": "string",
                            "description": "The session ID.",
                            "example": "4uzfv3JwTeyR5QpC3PXwMg"
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the session.",
                            "example": "Session 1"
                          },
                          "type": {
                            "type": "integer",
                            "description": "The session type. \n * 0 \u2014 Meeting \n * 2 \u2014 Webinar",
                            "enum": [
                              0,
                              2
                            ]
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-05-31T13:00:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-05-31T13:00:00Z"
                          },
                          "timezone": {
                            "type": "string",
                            "description": "The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones)."
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
                  "value": "{\n  \"event\": \"zoom_events.session_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"session_id\": \"fhhfkdhkhdgkhg\",\n      \"name\": \"my session\",\n      \"attendance_type\": \"virtual\",\n      \"type\": 2,\n      \"start_time\": \"2022-05-31T13:00:00Z\",\n      \"end_time\": \"2022-05-31T13:00:00Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
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
                      "description": "The name of the event. zoom_events.event.session_deleted."
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
                          "description": "The account ID of the host who deleted the session post event publishing."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the session.",
                          "required": [
                            "event_id",
                            "host_id",
                            "session_id",
                            "name",
                            "type",
                            "start_time",
                            "end_time",
                            "timezone"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The Zoom Events ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "session_id": {
                              "type": "string",
                              "description": "The session ID.",
                              "example": "4uzfv3JwTeyR5QpC3PXwMg"
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the session.",
                              "example": "Session 1"
                            },
                            "type": {
                              "type": "integer",
                              "description": "The session type. \n * 0 \u2014 Meeting \n * 2 \u2014 Webinar",
                              "enum": [
                                0,
                                2
                              ]
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-05-31T13:00:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-05-31T13:00:00Z"
                            },
                            "timezone": {
                              "type": "string",
                              "description": "The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones)."
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
                    "value": "{\n  \"event\": \"zoom_events.session_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"session_id\": \"fhhfkdhkhdgkhg\",\n      \"name\": \"my session\",\n      \"attendance_type\": \"virtual\",\n      \"type\": 2,\n      \"start_time\": \"2022-05-31T13:00:00Z\",\n      \"end_time\": \"2022-05-31T13:00:00Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.ticket_updated": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.ticket_updated",
        "requestBody": {
          "description": "# zoom_events.ticket_updated\nThe **Ticket Updated**  event  is triggered every time a user in your account or your app updates the attendee registration information in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can update a pre-registration for an event by updating registration question answers of a ticket.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Ticket Updated** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_tickets:write:admin`,`zoom_events_tickets:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:ticket`,`zoom_events:read:ticket:admin`\n\n**Event type**: `zoom_events.ticket_updated`\n",
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
                    "description": "The name of the event, zoom_events.ticket_updated"
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
                        "description": "The account ID of the host who created the event."
                      },
                      "object": {
                        "type": "object",
                        "description": " The ticket Description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "ticket_id",
                          "email"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The Zoom Events ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "ticket_id": {
                            "type": "string",
                            "description": "The ticket ID generated from the registration."
                          },
                          "ticket_type_id": {
                            "type": "string",
                            "description": "The ticket type ID for which the user registers to."
                          },
                          "external_ticket_id": {
                            "type": "string",
                            "description": "A set of unique alphanumeric characters that references the external ticket ID."
                          },
                          "email": {
                            "type": "string",
                            "description": "The registrant's email."
                          },
                          "ticket_role_type": {
                            "type": "string",
                            "description": "The ticket role type: \n * normal The normal ticket \n * alternative_host The alternative host ticket \n * panelist The panelist ticket \n * bind_meeting The bind meeting ticket \n * speaker The speaker ticket \n * interpreter The interpreter ticket \n * sponsor The sponsor ticket \n * host The host ticket \n * moderator The moderator ticket \n * expo_booth_owner The expo booth owner ticket \n * attendee_without_order The attendee without order ticket \n * company_zoom_room The company zoom room ticket \n * moderator_allhost The moderator all host ticket \n * hub_host The hub host ticket \n * hub_owner The hub owner ticket &quot;,\n * subscription The subscription ticket &quot;",
                            "example": "normal",
                            "enum": [
                              "normal",
                              "alternative_host",
                              "panelist",
                              "bind_meeting",
                              "speaker",
                              "subscription",
                              "sponsor",
                              "interpreter",
                              "host",
                              "moderator",
                              "expo_booth_owner",
                              "attendee_without_order",
                              "company_zoom_room",
                              "moderator_allhost",
                              "hub_host",
                              "hub_owner"
                            ]
                          },
                          "authentication_method": {
                            "type": "string",
                            "description": "The event authentication method for the ticket: \n * zoom_account Zoom account holder  * zoom_account_otp Zoom account holder with OTP  * corporate_idp Corporate IDP  * no_auth No authentication",
                            "enum": [
                              "zoom_account",
                              "zoom_account_otp",
                              "corporate_idp",
                              "no_auth"
                            ],
                            "example": "no_auth"
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
                  "value": "{\n  \"event\": \"zoom_events.ticket_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_id\": \"iso9Dllddp39dagjLj9j\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"external_ticket_id\": \"abc-123-efgh\",\n      \"ticket_role_type\": \"normal\",\n      \"email\": \"email@zoom.us\",\n      \"authentication_method\": \"no_auth\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.ticket_updated"
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
                          "description": "The account ID of the host who created the event."
                        },
                        "object": {
                          "type": "object",
                          "description": " The ticket Description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "ticket_id",
                            "email"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The Zoom Events ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "ticket_id": {
                              "type": "string",
                              "description": "The ticket ID generated from the registration."
                            },
                            "ticket_type_id": {
                              "type": "string",
                              "description": "The ticket type ID for which the user registers to."
                            },
                            "external_ticket_id": {
                              "type": "string",
                              "description": "A set of unique alphanumeric characters that references the external ticket ID."
                            },
                            "email": {
                              "type": "string",
                              "description": "The registrant's email."
                            },
                            "ticket_role_type": {
                              "type": "string",
                              "description": "The ticket role type: \n * normal The normal ticket \n * alternative_host The alternative host ticket \n * panelist The panelist ticket \n * bind_meeting The bind meeting ticket \n * speaker The speaker ticket \n * interpreter The interpreter ticket \n * sponsor The sponsor ticket \n * host The host ticket \n * moderator The moderator ticket \n * expo_booth_owner The expo booth owner ticket \n * attendee_without_order The attendee without order ticket \n * company_zoom_room The company zoom room ticket \n * moderator_allhost The moderator all host ticket \n * hub_host The hub host ticket \n * hub_owner The hub owner ticket &quot;,\n * subscription The subscription ticket &quot;",
                              "example": "normal",
                              "enum": [
                                "normal",
                                "alternative_host",
                                "panelist",
                                "bind_meeting",
                                "speaker",
                                "subscription",
                                "sponsor",
                                "interpreter",
                                "host",
                                "moderator",
                                "expo_booth_owner",
                                "attendee_without_order",
                                "company_zoom_room",
                                "moderator_allhost",
                                "hub_host",
                                "hub_owner"
                              ]
                            },
                            "authentication_method": {
                              "type": "string",
                              "description": "The event authentication method for the ticket: \n * zoom_account Zoom account holder  * zoom_account_otp Zoom account holder with OTP  * corporate_idp Corporate IDP  * no_auth No authentication",
                              "enum": [
                                "zoom_account",
                                "zoom_account_otp",
                                "corporate_idp",
                                "no_auth"
                              ],
                              "example": "no_auth"
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
                    "value": "{\n  \"event\": \"zoom_events.ticket_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_id\": \"iso9Dllddp39dagjLj9j\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"external_ticket_id\": \"abc-123-efgh\",\n      \"ticket_role_type\": \"normal\",\n      \"email\": \"email@zoom.us\",\n      \"authentication_method\": \"no_auth\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.event_published": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.event_published",
        "requestBody": {
          "description": "# zoom_events.event_published\nThe **Event Published** event is triggered every time a user in your account or your app, publishes an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host or a co-editor can publish an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Event published** subscription enabled under the **Zoom Events** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_basic:write:admin`,`zoom_events_basic:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:event`,`zoom_events:read:event:admin`\n\n**Event type**: `zoom_events.event_published`\n",
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
                    "description": "The name of the event. zoom_events.event_published."
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
                        "description": "The account ID of the host who published the event."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom Event description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "name",
                          "event_type",
                          "start_time",
                          "end_time",
                          "timezone"
                        ],
                        "properties": {
                          "hub_id": {
                            "type": "string",
                            "description": "The ID of the event hub.",
                            "example": "234kj2h34kljgh23lkhj3"
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The ID of the event.",
                            "example": "234kj2h34kljgh23lkhj3"
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the event.",
                            "example": "OpenAPI Conference Name"
                          },
                          "description": {
                            "type": "string",
                            "description": "The description of the event.",
                            "example": "This event was created with the OpenAPI"
                          },
                          "event_type": {
                            "type": "string",
                            "description": "Event Types: \n * CONFERENCE - Multi session event \n * SIMPLE_EVENT - Single session event \n * RECURRING - Recurring session event",
                            "enum": [
                              "CONFERENCE",
                              "SIMPLE_EVENT",
                              "RECURRING"
                            ],
                            "example": "CONFERENCE"
                          },
                          "status": {
                            "type": "string",
                            "description": "The event status: \n * PUBLISHED - Published event \n * DRAFT - Draft event \n * CANCELLED - Cancelled event",
                            "enum": [
                              "PUBLISHED",
                              "DRAFT",
                              "CANCELLED"
                            ],
                            "example": "PUBLISHED"
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "timezone": {
                            "type": "string",
                            "description": "The timezone of the event.",
                            "example": "America/Los_Angeles"
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
                  "value": "{\n  \"event\": \"zoom_events.event_published\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"hub_id\": \"1234567890abcd\",\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"name\": \"My Event-1\",\n      \"description\": \"This is a test event.\",\n      \"event_type\": \"CONFERENCE\",\n      \"attendance_type\": \"virtual\",\n      \"status\": \"DRAFT\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
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
                      "description": "The name of the event. zoom_events.event_published."
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
                          "description": "The account ID of the host who published the event."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom Event description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "name",
                            "event_type",
                            "start_time",
                            "end_time",
                            "timezone"
                          ],
                          "properties": {
                            "hub_id": {
                              "type": "string",
                              "description": "The ID of the event hub.",
                              "example": "234kj2h34kljgh23lkhj3"
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The ID of the event.",
                              "example": "234kj2h34kljgh23lkhj3"
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the event.",
                              "example": "OpenAPI Conference Name"
                            },
                            "description": {
                              "type": "string",
                              "description": "The description of the event.",
                              "example": "This event was created with the OpenAPI"
                            },
                            "event_type": {
                              "type": "string",
                              "description": "Event Types: \n * CONFERENCE - Multi session event \n * SIMPLE_EVENT - Single session event \n * RECURRING - Recurring session event",
                              "enum": [
                                "CONFERENCE",
                                "SIMPLE_EVENT",
                                "RECURRING"
                              ],
                              "example": "CONFERENCE"
                            },
                            "status": {
                              "type": "string",
                              "description": "The event status: \n * PUBLISHED - Published event \n * DRAFT - Draft event \n * CANCELLED - Cancelled event",
                              "enum": [
                                "PUBLISHED",
                                "DRAFT",
                                "CANCELLED"
                              ],
                              "example": "PUBLISHED"
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "timezone": {
                              "type": "string",
                              "description": "The timezone of the event.",
                              "example": "America/Los_Angeles"
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
                    "value": "{\n  \"event\": \"zoom_events.event_published\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"hub_id\": \"1234567890abcd\",\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"name\": \"My Event-1\",\n      \"description\": \"This is a test event.\",\n      \"event_type\": \"CONFERENCE\",\n      \"attendance_type\": \"virtual\",\n      \"status\": \"DRAFT\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.ticket_type_updated": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.ticket_type_updated",
        "requestBody": {
          "description": "# zoom_events.ticket_type_updated\nThe **TicketType Updated**  event  is triggered every time a user in your account or your app updates ticketType for an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can update a ticketType for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **TicketType Updated** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_ticket_types:write:admin`,`zoom_events_ticket_types:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:list_ticket_types`,`zoom_events:read:list_ticket_types:admin`\n\n**Event type**: `zoom_events.ticket_type_updated`\n",
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
                    "description": "The name of the event, zoom_events.ticket_type_updated"
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
                        "description": "The account ID of the host who updated the ticketType."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the ticketType.",
                        "required": [
                          "event_id",
                          "host_id",
                          "ticket_type_id",
                          "name",
                          "start_time",
                          "end_time"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The zoom-events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "ticket_type_id": {
                            "type": "string",
                            "description": "The ticketType ID for which the user registers to."
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the ticketType.",
                            "example": "General Admission Ticket"
                          },
                          "description": {
                            "type": "string",
                            "description": "The description of the ticketType.",
                            "example": "This is a General Admission ticket"
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
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
                  "value": "{\n  \"event\": \"zoom_events.ticket_type_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"name\": \"General Admission Ticket\",\n      \"description\": \"This is for General Admission\",\n      \"attendance_type\": \"virtual\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.ticket_type_updated"
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
                          "description": "The account ID of the host who updated the ticketType."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the ticketType.",
                          "required": [
                            "event_id",
                            "host_id",
                            "ticket_type_id",
                            "name",
                            "start_time",
                            "end_time"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The zoom-events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "ticket_type_id": {
                              "type": "string",
                              "description": "The ticketType ID for which the user registers to."
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the ticketType.",
                              "example": "General Admission Ticket"
                            },
                            "description": {
                              "type": "string",
                              "description": "The description of the ticketType.",
                              "example": "This is a General Admission ticket"
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
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
                    "value": "{\n  \"event\": \"zoom_events.ticket_type_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"name\": \"General Admission Ticket\",\n      \"description\": \"This is for General Admission\",\n      \"attendance_type\": \"virtual\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.exhibitor_created": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.exhibitor_created",
        "requestBody": {
          "description": "# zoom_events.exhibitor_created\nThe **Exhibitor Created**  event  is triggered every time a user in your account or your app creates exhibitor for an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can create an exhibitor for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Exhibitor Created** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_exhibitors:write:admin`,`zoom_events_exhibitors:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:exhibitor`,`zoom_events:read:exhibitor:admin`\n\n**Event type**: `zoom_events.exhibitor_created`\n",
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
                    "description": "The name of the event, zoom_events.exhibitor_created"
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
                        "description": "The account ID of the host who created the event exhibitor."
                      },
                      "object": {
                        "type": "object",
                        "description": "Exhibitor Description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "exhibitor_id",
                          "name",
                          "contact_name",
                          "contact_email",
                          "is_sponsor"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The zoom-events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "exhibitor_id": {
                            "type": "string",
                            "description": "The exhibitors's ID."
                          },
                          "name": {
                            "type": "string",
                            "description": "The exhibitors's name."
                          },
                          "description": {
                            "type": "string",
                            "description": "The exhibitor's description."
                          },
                          "is_sponsor": {
                            "type": "boolean",
                            "description": "Whether the exhibitor is a sponsor."
                          },
                          "tier_id": {
                            "type": "string",
                            "description": "The sponsor tier ID for a particular event. This field only applies to a sponsor."
                          },
                          "contact_name": {
                            "type": "string",
                            "description": "The contact's full name."
                          },
                          "contact_email": {
                            "type": "string",
                            "description": "The contact's email address."
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
                  "value": "{\n  \"event\": \"zoom_events.exhibitor_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"exhibitor_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"exhibitor1\",\n      \"description\": \"Description of the exhibitor\",\n      \"is_sponsor\": true,\n      \"tier_id\": \"4fgRIOXjQGmoH6raAzJf8g\",\n      \"contact_name\": \"John Doe\",\n      \"contact_email\": \"exhibitor1-email@zoom.us\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.exhibitor_created"
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
                          "description": "The account ID of the host who created the event exhibitor."
                        },
                        "object": {
                          "type": "object",
                          "description": "Exhibitor Description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "exhibitor_id",
                            "name",
                            "contact_name",
                            "contact_email",
                            "is_sponsor"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The zoom-events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "exhibitor_id": {
                              "type": "string",
                              "description": "The exhibitors's ID."
                            },
                            "name": {
                              "type": "string",
                              "description": "The exhibitors's name."
                            },
                            "description": {
                              "type": "string",
                              "description": "The exhibitor's description."
                            },
                            "is_sponsor": {
                              "type": "boolean",
                              "description": "Whether the exhibitor is a sponsor."
                            },
                            "tier_id": {
                              "type": "string",
                              "description": "The sponsor tier ID for a particular event. This field only applies to a sponsor."
                            },
                            "contact_name": {
                              "type": "string",
                              "description": "The contact's full name."
                            },
                            "contact_email": {
                              "type": "string",
                              "description": "The contact's email address."
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
                    "value": "{\n  \"event\": \"zoom_events.exhibitor_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"exhibitor_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"exhibitor1\",\n      \"description\": \"Description of the exhibitor\",\n      \"is_sponsor\": true,\n      \"tier_id\": \"4fgRIOXjQGmoH6raAzJf8g\",\n      \"contact_name\": \"John Doe\",\n      \"contact_email\": \"exhibitor1-email@zoom.us\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.exhibitor_updated": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.exhibitor_updated",
        "requestBody": {
          "description": "# zoom_events.exhibitor_updated\nThe **Exhibitor Updated**  event  is triggered every time a user in your account or your app updates exhibitor for an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can update an exhibitor for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Exhibitor Updated** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_exhibitors:write:admin`,`zoom_events_exhibitors:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:exhibitor`,`zoom_events:read:exhibitor:admin`\n\n**Event type**: `zoom_events.exhibitor_updated`\n",
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
                    "description": "The name of the event, zoom_events.exhibitor_updated"
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
                        "description": "The account ID of the host who updated the event exhibitor."
                      },
                      "object": {
                        "type": "object",
                        "description": "Exhibitor Description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "exhibitor_id",
                          "name",
                          "contact_name",
                          "contact_email",
                          "is_sponsor"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The zoom-events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "exhibitor_id": {
                            "type": "string",
                            "description": "The exhibitors's ID."
                          },
                          "name": {
                            "type": "string",
                            "description": "The exhibitors's name."
                          },
                          "description": {
                            "type": "string",
                            "description": "The exhibitor's description."
                          },
                          "is_sponsor": {
                            "type": "boolean",
                            "description": "Whether the exhibitor is a sponsor."
                          },
                          "tier_id": {
                            "type": "string",
                            "description": "The sponsor tier ID for a particular event. This field only applies to a sponsor."
                          },
                          "contact_name": {
                            "type": "string",
                            "description": "The contact's full name."
                          },
                          "contact_email": {
                            "type": "string",
                            "description": "The contact's email address."
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
                  "value": "{\n  \"event\": \"zoom_events.exhibitor_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"exhibitor_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"exhibitor1\",\n      \"description\": \"Description of the exhibitor\",\n      \"is_sponsor\": true,\n      \"tier_id\": \"4fgRIOXjQGmoH6raAzJf8g\",\n      \"contact_name\": \"John Doe\",\n      \"contact_email\": \"exhibitor1-email@zoom.us\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.exhibitor_updated"
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
                          "description": "The account ID of the host who updated the event exhibitor."
                        },
                        "object": {
                          "type": "object",
                          "description": "Exhibitor Description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "exhibitor_id",
                            "name",
                            "contact_name",
                            "contact_email",
                            "is_sponsor"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The zoom-events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "exhibitor_id": {
                              "type": "string",
                              "description": "The exhibitors's ID."
                            },
                            "name": {
                              "type": "string",
                              "description": "The exhibitors's name."
                            },
                            "description": {
                              "type": "string",
                              "description": "The exhibitor's description."
                            },
                            "is_sponsor": {
                              "type": "boolean",
                              "description": "Whether the exhibitor is a sponsor."
                            },
                            "tier_id": {
                              "type": "string",
                              "description": "The sponsor tier ID for a particular event. This field only applies to a sponsor."
                            },
                            "contact_name": {
                              "type": "string",
                              "description": "The contact's full name."
                            },
                            "contact_email": {
                              "type": "string",
                              "description": "The contact's email address."
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
                    "value": "{\n  \"event\": \"zoom_events.exhibitor_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"exhibitor_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"exhibitor1\",\n      \"description\": \"Description of the exhibitor\",\n      \"is_sponsor\": true,\n      \"tier_id\": \"4fgRIOXjQGmoH6raAzJf8g\",\n      \"contact_name\": \"John Doe\",\n      \"contact_email\": \"exhibitor1-email@zoom.us\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.event_cancelled": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.event_cancelled",
        "requestBody": {
          "description": "# zoom_events.event_cancelled\nThe **Event Cancelled** event is triggered every time a user in your account or your app, cancels the event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host can cancel the event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Event Cancelled** subscription enabled under the **Zoom Events** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_basic:write:admin`,`zoom_events_basic:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:event`,`zoom_events:read:event:admin`\n\n**Event type**: `zoom_events.event_cancelled`\n",
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
                    "description": "The name of the event. zoom_events.event_cancelled."
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
                        "description": "The account ID of the host who cancelled the event."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom Event description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "name",
                          "event_type",
                          "start_time",
                          "end_time",
                          "timezone"
                        ],
                        "properties": {
                          "hub_id": {
                            "type": "string",
                            "description": "The ID of the event hub.",
                            "example": "234kj2h34kljgh23lkhj3"
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The ID of the event.",
                            "example": "234kj2h34kljgh23lkhj3"
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the event.",
                            "example": "OpenAPI Conference Name"
                          },
                          "description": {
                            "type": "string",
                            "description": "The description of the event.",
                            "example": "This event was created with the OpenAPI"
                          },
                          "event_type": {
                            "type": "string",
                            "description": "Event Types: \n * CONFERENCE - Multi session event \n * SIMPLE_EVENT - Single session event \n * RECURRING - Recurring session event",
                            "enum": [
                              "CONFERENCE",
                              "SIMPLE_EVENT",
                              "RECURRING"
                            ],
                            "example": "CONFERENCE"
                          },
                          "status": {
                            "type": "string",
                            "description": "The event status: \n * PUBLISHED - Published event \n * DRAFT - Draft event \n * CANCELLED - Cancelled event",
                            "enum": [
                              "PUBLISHED",
                              "DRAFT",
                              "CANCELLED"
                            ],
                            "example": "PUBLISHED"
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "timezone": {
                            "type": "string",
                            "description": "The timezone of the event.",
                            "example": "America/Los_Angeles"
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
                  "value": "{\n  \"event\": \"zoom_events.event_cancelled\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"hub_id\": \"1234567890abcd\",\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"name\": \"My Event-1\",\n      \"description\": \"This is a test event.\",\n      \"attendance_type\": \"virtual\",\n      \"event_type\": \"CONFERENCE\",\n      \"status\": \"DRAFT\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
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
                      "description": "The name of the event. zoom_events.event_cancelled."
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
                          "description": "The account ID of the host who cancelled the event."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom Event description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "name",
                            "event_type",
                            "start_time",
                            "end_time",
                            "timezone"
                          ],
                          "properties": {
                            "hub_id": {
                              "type": "string",
                              "description": "The ID of the event hub.",
                              "example": "234kj2h34kljgh23lkhj3"
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The ID of the event.",
                              "example": "234kj2h34kljgh23lkhj3"
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the event.",
                              "example": "OpenAPI Conference Name"
                            },
                            "description": {
                              "type": "string",
                              "description": "The description of the event.",
                              "example": "This event was created with the OpenAPI"
                            },
                            "event_type": {
                              "type": "string",
                              "description": "Event Types: \n * CONFERENCE - Multi session event \n * SIMPLE_EVENT - Single session event \n * RECURRING - Recurring session event",
                              "enum": [
                                "CONFERENCE",
                                "SIMPLE_EVENT",
                                "RECURRING"
                              ],
                              "example": "CONFERENCE"
                            },
                            "status": {
                              "type": "string",
                              "description": "The event status: \n * PUBLISHED - Published event \n * DRAFT - Draft event \n * CANCELLED - Cancelled event",
                              "enum": [
                                "PUBLISHED",
                                "DRAFT",
                                "CANCELLED"
                              ],
                              "example": "PUBLISHED"
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "timezone": {
                              "type": "string",
                              "description": "The timezone of the event.",
                              "example": "America/Los_Angeles"
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
                    "value": "{\n  \"event\": \"zoom_events.event_cancelled\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"hub_id\": \"1234567890abcd\",\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"name\": \"My Event-1\",\n      \"description\": \"This is a test event.\",\n      \"attendance_type\": \"virtual\",\n      \"event_type\": \"CONFERENCE\",\n      \"status\": \"DRAFT\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.event_duplicated": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.event_duplicated",
        "requestBody": {
          "description": "# zoom_events.event_duplicated\nThe **Event Duplicated** is triggered every time a user in your account or your app, duplicates an existing event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host can duplicate an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Event Duplicated** subscription enabled under the **Zoom Events** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_basic:write:admin`,`zoom_events_basic:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:event`,`zoom_events:read:event:admin`\n\n**Event type**: `zoom_events.event_duplicated`\n",
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
                    "description": "The name of the event. zoom_events.event_duplicated."
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
                        "description": "The account ID of the host who duplicated the event."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom Event description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "name",
                          "event_type",
                          "start_time",
                          "end_time",
                          "timezone"
                        ],
                        "properties": {
                          "hub_id": {
                            "type": "string",
                            "description": "The ID of the event hub.",
                            "example": "h134kj2h34kljgh23lkhj3"
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The ID of the event.",
                            "example": "234kj2h34kljgh23lkhj3"
                          },
                          "source_event_id": {
                            "type": "string",
                            "description": "The ID of the event from which this event was duplicated from.",
                            "example": "434kj2h34kljgh23lkhj3"
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the event.",
                            "example": "OpenAPI Conference Name"
                          },
                          "description": {
                            "type": "string",
                            "description": "The description of the event.",
                            "example": "This event was created with the OpenAPI"
                          },
                          "event_type": {
                            "type": "string",
                            "description": "Event Types: \n * CONFERENCE - Multi session event \n * SIMPLE_EVENT - Single session event \n * RECURRING - Recurring session event",
                            "enum": [
                              "CONFERENCE",
                              "SIMPLE_EVENT",
                              "RECURRING"
                            ],
                            "example": "CONFERENCE"
                          },
                          "status": {
                            "type": "string",
                            "description": "The event status: \n * PUBLISHED - Published event \n * DRAFT - Draft event \n * CANCELLED - Cancelled event",
                            "enum": [
                              "PUBLISHED",
                              "DRAFT",
                              "CANCELLED"
                            ],
                            "example": "DRAFT"
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "timezone": {
                            "type": "string",
                            "description": "The timezone of the event.",
                            "example": "America/Los_Angeles"
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
                  "value": "{\n  \"event\": \"zoom_events.event_duplicated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"hub_id\": \"1234567890abcd\",\n      \"event_id\": \"1234567890abcd\",\n      \"source_event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"name\": \"My Event-1\",\n      \"description\": \"This is a test event.\",\n      \"attendance_type\": \"virtual\",\n      \"event_type\": \"CONFERENCE\",\n      \"status\": \"DRAFT\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
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
                      "description": "The name of the event. zoom_events.event_duplicated."
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
                          "description": "The account ID of the host who duplicated the event."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom Event description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "name",
                            "event_type",
                            "start_time",
                            "end_time",
                            "timezone"
                          ],
                          "properties": {
                            "hub_id": {
                              "type": "string",
                              "description": "The ID of the event hub.",
                              "example": "h134kj2h34kljgh23lkhj3"
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The ID of the event.",
                              "example": "234kj2h34kljgh23lkhj3"
                            },
                            "source_event_id": {
                              "type": "string",
                              "description": "The ID of the event from which this event was duplicated from.",
                              "example": "434kj2h34kljgh23lkhj3"
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the event.",
                              "example": "OpenAPI Conference Name"
                            },
                            "description": {
                              "type": "string",
                              "description": "The description of the event.",
                              "example": "This event was created with the OpenAPI"
                            },
                            "event_type": {
                              "type": "string",
                              "description": "Event Types: \n * CONFERENCE - Multi session event \n * SIMPLE_EVENT - Single session event \n * RECURRING - Recurring session event",
                              "enum": [
                                "CONFERENCE",
                                "SIMPLE_EVENT",
                                "RECURRING"
                              ],
                              "example": "CONFERENCE"
                            },
                            "status": {
                              "type": "string",
                              "description": "The event status: \n * PUBLISHED - Published event \n * DRAFT - Draft event \n * CANCELLED - Cancelled event",
                              "enum": [
                                "PUBLISHED",
                                "DRAFT",
                                "CANCELLED"
                              ],
                              "example": "DRAFT"
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "timezone": {
                              "type": "string",
                              "description": "The timezone of the event.",
                              "example": "America/Los_Angeles"
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
                    "value": "{\n  \"event\": \"zoom_events.event_duplicated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"hub_id\": \"1234567890abcd\",\n      \"event_id\": \"1234567890abcd\",\n      \"source_event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"name\": \"My Event-1\",\n      \"description\": \"This is a test event.\",\n      \"attendance_type\": \"virtual\",\n      \"event_type\": \"CONFERENCE\",\n      \"status\": \"DRAFT\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.session_ended": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.session_ended",
        "requestBody": {
          "description": "# zoom_events.session_ended\nThe **End Session** event is triggered every time a user in your account or your app ends a zoom-events session.\n\nZoom users with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing zoom-events event sessions. A Zoom Events host can start and end a Zoom Events session.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **End Session** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_sessions:read:admin`,`zoom_events_sessions:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:session`,`zoom_events:read:session:admin`\n\n**Event type**: `zoom_events.session_ended`\n",
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
                        "description": "The account ID of the user who ended the session."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the session.",
                        "required": [
                          "event_id",
                          "host_id",
                          "session_id",
                          "name",
                          "type",
                          "start_time",
                          "end_time",
                          "timezone"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The Zoom Events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "session_id": {
                            "type": "string",
                            "description": "The session ID."
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the session."
                          },
                          "type": {
                            "type": "integer",
                            "description": "The session type. \n* 0 \u2014 Meeting.\n* 2 \u2014 Webinar",
                            "enum": [
                              0,
                              2
                            ]
                          },
                          "start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The session's start time in UTC."
                          },
                          "end_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The date and time when the session ended in UTC."
                          },
                          "timezone": {
                            "type": "string",
                            "description": "The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones)."
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
                  "value": "{\n    \"event\": \"zoom_events.session_ended\",\n    \"event_ts\": 1626230691572,\n    \"payload\": {\n        \"account_id\": \"AAAAAABBBB\",\n        \"object\": {\n            \"event_id\": \"1234567890abcd\",\n            \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n            \"session_id\": \"1234567890abcd\",\n            \"name\": \"My Session-1\",\n            \"type\": 2,\n            \"start_time\": \"2023-07-13T21:44:51Z\",\n            \"timezone\": \"America/Los_Angeles\",\n            \"end_time\": \"2023-07-13T23:00:51Z\"\n        }\n    }\n}"
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
                          "description": "The account ID of the user who ended the session."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the session.",
                          "required": [
                            "event_id",
                            "host_id",
                            "session_id",
                            "name",
                            "type",
                            "start_time",
                            "end_time",
                            "timezone"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The Zoom Events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "session_id": {
                              "type": "string",
                              "description": "The session ID."
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the session."
                            },
                            "type": {
                              "type": "integer",
                              "description": "The session type. \n* 0 \u2014 Meeting.\n* 2 \u2014 Webinar",
                              "enum": [
                                0,
                                2
                              ]
                            },
                            "start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The session's start time in UTC."
                            },
                            "end_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The date and time when the session ended in UTC."
                            },
                            "timezone": {
                              "type": "string",
                              "description": "The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones)."
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
                    "value": "{\n    \"event\": \"zoom_events.session_ended\",\n    \"event_ts\": 1626230691572,\n    \"payload\": {\n        \"account_id\": \"AAAAAABBBB\",\n        \"object\": {\n            \"event_id\": \"1234567890abcd\",\n            \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n            \"session_id\": \"1234567890abcd\",\n            \"name\": \"My Session-1\",\n            \"type\": 2,\n            \"start_time\": \"2023-07-13T21:44:51Z\",\n            \"timezone\": \"America/Los_Angeles\",\n            \"end_time\": \"2023-07-13T23:00:51Z\"\n        }\n    }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.vod_registration_created": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.vod_registration_created",
        "requestBody": {
          "description": "# zoom_events.vod_registration_created\nThe **VOD Registration Created**  event  is triggered when someone registers for a video on-demand(VOD) channel  created by one of your app users or account users.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events.  The host can create an on-demand video channel, set up how their attendees will access their channels with registration.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**\n    * The **VOD Registration Created** subscription enabled under the **Zoom Events** event\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_vod_registration:read`,`zoom_events_vod_registration:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:vod_registrations`,`zoom_events:read:vod_registrations:admin`\n\n**Event type**: `zoom_events.vod_registration_created`\n",
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
                    "description": "The name of the event, zoom_events.vod_registration_created."
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
                        "description": "The account ID of the host who created the event."
                      },
                      "object": {
                        "type": "object",
                        "description": "The VOD channel registration description.",
                        "required": [
                          "channel_id",
                          "host_id",
                          "hub_id",
                          "registration_id",
                          "email"
                        ],
                        "properties": {
                          "channel_id": {
                            "type": "string",
                            "description": "The VOD channel ID."
                          },
                          "hub_id": {
                            "type": "string",
                            "description": "The Zoom Events hub ID",
                            "example": "234kj2h34kljgh23lkhj3"
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The channel host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "registration_id": {
                            "type": "string",
                            "description": "The registration ID generated from the registration."
                          },
                          "email": {
                            "type": "string",
                            "description": "The registrant's email."
                          },
                          "registration_date": {
                            "type": "string",
                            "description": "The registrantion date in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ.",
                            "format": "date-time",
                            "example": "2025-05-25T22:30:00Z"
                          },
                          "authentication_method": {
                            "type": "string",
                            "description": "The VOD channel authentication method \n * zoom_account Zoom account holder   * no_auth No authentication* email_opt registration using email OTP verification* ",
                            "example": "no_auth",
                            "enum": [
                              "zoom_account",
                              "email_opt",
                              "no_auth"
                            ]
                          },
                          "join_link": {
                            "type": "string",
                            "description": "The unique join link for the vod registrant.",
                            "example": "https://events.zoom.us/ejl/xyz"
                          },
                          "first_name": {
                            "type": "string",
                            "description": "The registrant's first name.",
                            "example": "Jill"
                          },
                          "last_name": {
                            "type": "string",
                            "description": "The registrant's last name.",
                            "example": "Chill"
                          },
                          "address": {
                            "type": "string",
                            "description": "The registrant's address.",
                            "example": "1800 Amphibious Blvd"
                          },
                          "city": {
                            "type": "string",
                            "description": "The registrant's city.",
                            "example": "Mountain View"
                          },
                          "state": {
                            "type": "string",
                            "description": "The registrant's state or province.",
                            "example": "CA"
                          },
                          "zip": {
                            "type": "string",
                            "description": "The registrant's ZIP or postal code.",
                            "example": "94045"
                          },
                          "country": {
                            "type": "string",
                            "description": "The registrant's two-letter [country code](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#countries).",
                            "example": "US"
                          },
                          "phone": {
                            "type": "string",
                            "description": "The registrant's phone number.",
                            "example": "5550100"
                          },
                          "industry": {
                            "type": "string",
                            "description": "The registrant's industry.",
                            "example": "Food"
                          },
                          "job_title": {
                            "type": "string",
                            "description": "The registrant's job title.",
                            "example": "Chef"
                          },
                          "organization": {
                            "type": "string",
                            "description": "The registrant's organization.",
                            "example": "ABC Ltd."
                          },
                          "organization_size": {
                            "type": "string",
                            "description": "The registrant's number of employees.",
                            "example": "150"
                          },
                          "comments": {
                            "type": "string",
                            "description": "The registrant's questions and comments.",
                            "example": "Looking forward to the discussion."
                          },
                          "custom_questions": {
                            "maxItems": 100,
                            "type": "array",
                            "description": "The information about custom questions.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "title": {
                                  "type": "string",
                                  "description": "The title of the custom question.",
                                  "example": "What do you hope to learn from this?"
                                },
                                "answer": {
                                  "type": "string",
                                  "description": "The custom question's response value.",
                                  "example": "Look forward to learning how you come up with new recipes and what other services you offer."
                                }
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
                  "value": "{\n  \"event\": \"zoom_events.vod_registration_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"djsjgkjhfhk\",\n      \"hub_id\": \"huCbxjabcdEFGHfp8uQ\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"registration_id\": \"iso9Dllddp39dagjLj9j\",\n      \"email\": \"email@zoom.us\",\n      \"registration_date\": \"2025-05-24T22:30:00Z\",\n      \"authentication_method\": \"no_auth\",\n      \"join_link\": \"https://events.zoom.us/ejl/xyz\",\n      \"first_name\": \"Jill\",\n      \"last_name\": \"Chill\",\n      \"address\": \"1800 Amphibious Blvd\",\n      \"city\": \"Mountain View\",\n      \"state\": \"CA\",\n      \"zip\": \"94045\",\n      \"country\": \"US\",\n      \"phone\": \"5550100\",\n      \"industry\": \"Food\",\n      \"job_title\": \"Chef\",\n      \"organization\": \"ABC Ltd.\",\n      \"organization_size\": \"150\",\n      \"comments\": \"Looking forward to the discussion.\",\n      \"custom_questions\": [\n        {\n          \"title\": \"What do you hope to learn from this?\",\n          \"answer\": \"Look forward to learning how you come up with new recipes and what other services you offer.\"\n        }\n      ]\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.vod_registration_created."
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
                          "description": "The account ID of the host who created the event."
                        },
                        "object": {
                          "type": "object",
                          "description": "The VOD channel registration description.",
                          "required": [
                            "channel_id",
                            "host_id",
                            "hub_id",
                            "registration_id",
                            "email"
                          ],
                          "properties": {
                            "channel_id": {
                              "type": "string",
                              "description": "The VOD channel ID."
                            },
                            "hub_id": {
                              "type": "string",
                              "description": "The Zoom Events hub ID",
                              "example": "234kj2h34kljgh23lkhj3"
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The channel host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "registration_id": {
                              "type": "string",
                              "description": "The registration ID generated from the registration."
                            },
                            "email": {
                              "type": "string",
                              "description": "The registrant's email."
                            },
                            "registration_date": {
                              "type": "string",
                              "description": "The registrantion date in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ.",
                              "format": "date-time",
                              "example": "2025-05-25T22:30:00Z"
                            },
                            "authentication_method": {
                              "type": "string",
                              "description": "The VOD channel authentication method \n * zoom_account Zoom account holder   * no_auth No authentication* email_opt registration using email OTP verification* ",
                              "example": "no_auth",
                              "enum": [
                                "zoom_account",
                                "email_opt",
                                "no_auth"
                              ]
                            },
                            "join_link": {
                              "type": "string",
                              "description": "The unique join link for the vod registrant.",
                              "example": "https://events.zoom.us/ejl/xyz"
                            },
                            "first_name": {
                              "type": "string",
                              "description": "The registrant's first name.",
                              "example": "Jill"
                            },
                            "last_name": {
                              "type": "string",
                              "description": "The registrant's last name.",
                              "example": "Chill"
                            },
                            "address": {
                              "type": "string",
                              "description": "The registrant's address.",
                              "example": "1800 Amphibious Blvd"
                            },
                            "city": {
                              "type": "string",
                              "description": "The registrant's city.",
                              "example": "Mountain View"
                            },
                            "state": {
                              "type": "string",
                              "description": "The registrant's state or province.",
                              "example": "CA"
                            },
                            "zip": {
                              "type": "string",
                              "description": "The registrant's ZIP or postal code.",
                              "example": "94045"
                            },
                            "country": {
                              "type": "string",
                              "description": "The registrant's two-letter [country code](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#countries).",
                              "example": "US"
                            },
                            "phone": {
                              "type": "string",
                              "description": "The registrant's phone number.",
                              "example": "5550100"
                            },
                            "industry": {
                              "type": "string",
                              "description": "The registrant's industry.",
                              "example": "Food"
                            },
                            "job_title": {
                              "type": "string",
                              "description": "The registrant's job title.",
                              "example": "Chef"
                            },
                            "organization": {
                              "type": "string",
                              "description": "The registrant's organization.",
                              "example": "ABC Ltd."
                            },
                            "organization_size": {
                              "type": "string",
                              "description": "The registrant's number of employees.",
                              "example": "150"
                            },
                            "comments": {
                              "type": "string",
                              "description": "The registrant's questions and comments.",
                              "example": "Looking forward to the discussion."
                            },
                            "custom_questions": {
                              "maxItems": 100,
                              "type": "array",
                              "description": "The information about custom questions.",
                              "items": {
                                "type": "object",
                                "properties": {
                                  "title": {
                                    "type": "string",
                                    "description": "The title of the custom question.",
                                    "example": "What do you hope to learn from this?"
                                  },
                                  "answer": {
                                    "type": "string",
                                    "description": "The custom question's response value.",
                                    "example": "Look forward to learning how you come up with new recipes and what other services you offer."
                                  }
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
                    "value": "{\n  \"event\": \"zoom_events.vod_registration_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"djsjgkjhfhk\",\n      \"hub_id\": \"huCbxjabcdEFGHfp8uQ\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"registration_id\": \"iso9Dllddp39dagjLj9j\",\n      \"email\": \"email@zoom.us\",\n      \"registration_date\": \"2025-05-24T22:30:00Z\",\n      \"authentication_method\": \"no_auth\",\n      \"join_link\": \"https://events.zoom.us/ejl/xyz\",\n      \"first_name\": \"Jill\",\n      \"last_name\": \"Chill\",\n      \"address\": \"1800 Amphibious Blvd\",\n      \"city\": \"Mountain View\",\n      \"state\": \"CA\",\n      \"zip\": \"94045\",\n      \"country\": \"US\",\n      \"phone\": \"5550100\",\n      \"industry\": \"Food\",\n      \"job_title\": \"Chef\",\n      \"organization\": \"ABC Ltd.\",\n      \"organization_size\": \"150\",\n      \"comments\": \"Looking forward to the discussion.\",\n      \"custom_questions\": [\n        {\n          \"title\": \"What do you hope to learn from this?\",\n          \"answer\": \"Look forward to learning how you come up with new recipes and what other services you offer.\"\n        }\n      ]\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.speaker_updated": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.speaker_updated",
        "requestBody": {
          "description": "# zoom_events.speaker_updated\nThe **Speaker Updated**  event  is triggered every time a user in your account or your app updates speaker for an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can update a speaker for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Speaker Updated** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_speakers:write:admin`,`zoom_events_speakers:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:speaker`,`zoom_events:read:speaker:admin`\n\n**Event type**: `zoom_events.speaker_updated`\n",
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
                    "description": "The name of the event, zoom_events.speaker_updated"
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
                        "description": "The account ID of the host who updated the event speaker."
                      },
                      "object": {
                        "type": "object",
                        "description": "Speaker Description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "speaker_id",
                          "name"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The zoom-events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "speaker_id": {
                            "type": "string",
                            "description": "The speaker id."
                          },
                          "name": {
                            "type": "string",
                            "description": "The speaker's name."
                          },
                          "email": {
                            "type": "string",
                            "description": "The speaker's email."
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
                  "value": "{\n  \"event\": \"zoom_events.speaker_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"speaker_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"speaker1\",\n      \"email\": \"speaker1-email@zoom.us\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.speaker_updated"
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
                          "description": "The account ID of the host who updated the event speaker."
                        },
                        "object": {
                          "type": "object",
                          "description": "Speaker Description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "speaker_id",
                            "name"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The zoom-events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "speaker_id": {
                              "type": "string",
                              "description": "The speaker id."
                            },
                            "name": {
                              "type": "string",
                              "description": "The speaker's name."
                            },
                            "email": {
                              "type": "string",
                              "description": "The speaker's email."
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
                    "value": "{\n  \"event\": \"zoom_events.speaker_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"speaker_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"speaker1\",\n      \"email\": \"speaker1-email@zoom.us\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.ticket_deleted": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.ticket_deleted",
        "requestBody": {
          "description": "# zoom_events.ticket_deleted\nThe  **Ticket Deleted**  event is triggered when one of your app users or account users cancels the event registration.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events.  The host can delete a pre-registration for an event or a registrant can cancel their ticket.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Ticket Deleted** subscription enabled under the **Zoom Events** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_tickets:write:admin`,`zoom_events_tickets:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:ticket`,`zoom_events:read:ticket:admin`\n\n**Event type**: `zoom_events.ticket_deleted`\n",
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
                    "description": "The name of the event. zoom_events.ticket_deleted."
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
                        "description": "The account ID of the host who created the event."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom Event description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "ticket_id",
                          "email"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The Zoom Events ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "ticket_id": {
                            "type": "string",
                            "description": "The ticket ID generated from the registration."
                          },
                          "ticket_type_id": {
                            "type": "string",
                            "description": "The ticket type ID for which the user registers to."
                          },
                          "external_ticket_id": {
                            "type": "string",
                            "description": "A set of unique alphanumeric characters that references the external ticket ID."
                          },
                          "email": {
                            "type": "string",
                            "description": "The registrant's email."
                          },
                          "ticket_role_type": {
                            "type": "string",
                            "description": "The ticket role type: \n * normal The normal ticket \n * alternative_host The alternative host ticket \n * panelist The panelist ticket \n * bind_meeting The bind meeting ticket \n * speaker The speaker ticket \n * interpreter The interpreter ticket \n * sponsor The sponsor ticket \n * host The host ticket \n * moderator The moderator ticket \n * expo_booth_owner The expo booth owner ticket \n * attendee_without_order The attendee without order ticket \n * pre_registration \u2014 The pre-registration ticket \n * company_zoom_room The company zoom room ticket \n * moderator_allhost The moderator all host ticket \n * hub_host The hub host ticket \n * hub_owner The hub owner ticket \n * subscription The subscription ticket",
                            "example": "normal",
                            "enum": [
                              "normal",
                              "alternative_host",
                              "panelist",
                              "bind_meeting",
                              "speaker",
                              "subscription",
                              "sponsor",
                              "interpreter",
                              "host",
                              "moderator",
                              "expo_booth_owner",
                              "attendee_without_order",
                              "pre_registration",
                              "company_zoom_room",
                              "moderator_allhost",
                              "hub_host",
                              "hub_owner"
                            ]
                          },
                          "authentication_method": {
                            "type": "string",
                            "description": "The event authentication method for the ticket: \n * zoom_account Zoom account holder  * zoom_account_otp Zoom account holder with OTP  * corporate_idp Corporate IDP  * no_auth No authentication",
                            "enum": [
                              "zoom_account",
                              "zoom_account_otp",
                              "corporate_idp",
                              "no_auth"
                            ],
                            "example": "no_auth"
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
                  "value": "{\n  \"event\": \"zoom_events.ticket_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_id\": \"iso9Dllddp39dagjLj9j\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"external_ticket_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"email\": \"email@zoom.us\",\n      \"ticket_role_type\": \"normal\",\n      \"authentication_method\": \"no_auth\"\n    }\n  }\n}"
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
                      "description": "The name of the event. zoom_events.ticket_deleted."
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
                          "description": "The account ID of the host who created the event."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom Event description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "ticket_id",
                            "email"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The Zoom Events ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "ticket_id": {
                              "type": "string",
                              "description": "The ticket ID generated from the registration."
                            },
                            "ticket_type_id": {
                              "type": "string",
                              "description": "The ticket type ID for which the user registers to."
                            },
                            "external_ticket_id": {
                              "type": "string",
                              "description": "A set of unique alphanumeric characters that references the external ticket ID."
                            },
                            "email": {
                              "type": "string",
                              "description": "The registrant's email."
                            },
                            "ticket_role_type": {
                              "type": "string",
                              "description": "The ticket role type: \n * normal The normal ticket \n * alternative_host The alternative host ticket \n * panelist The panelist ticket \n * bind_meeting The bind meeting ticket \n * speaker The speaker ticket \n * interpreter The interpreter ticket \n * sponsor The sponsor ticket \n * host The host ticket \n * moderator The moderator ticket \n * expo_booth_owner The expo booth owner ticket \n * attendee_without_order The attendee without order ticket \n * pre_registration \u2014 The pre-registration ticket \n * company_zoom_room The company zoom room ticket \n * moderator_allhost The moderator all host ticket \n * hub_host The hub host ticket \n * hub_owner The hub owner ticket \n * subscription The subscription ticket",
                              "example": "normal",
                              "enum": [
                                "normal",
                                "alternative_host",
                                "panelist",
                                "bind_meeting",
                                "speaker",
                                "subscription",
                                "sponsor",
                                "interpreter",
                                "host",
                                "moderator",
                                "expo_booth_owner",
                                "attendee_without_order",
                                "pre_registration",
                                "company_zoom_room",
                                "moderator_allhost",
                                "hub_host",
                                "hub_owner"
                              ]
                            },
                            "authentication_method": {
                              "type": "string",
                              "description": "The event authentication method for the ticket: \n * zoom_account Zoom account holder  * zoom_account_otp Zoom account holder with OTP  * corporate_idp Corporate IDP  * no_auth No authentication",
                              "enum": [
                                "zoom_account",
                                "zoom_account_otp",
                                "corporate_idp",
                                "no_auth"
                              ],
                              "example": "no_auth"
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
                    "value": "{\n  \"event\": \"zoom_events.ticket_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_id\": \"iso9Dllddp39dagjLj9j\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"external_ticket_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"email\": \"email@zoom.us\",\n      \"ticket_role_type\": \"normal\",\n      \"authentication_method\": \"no_auth\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.speaker_deleted": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.speaker_deleted",
        "requestBody": {
          "description": "# zoom_events.speaker_deleted\nThe **Speaker Deleted**  event  is triggered every time a user in your account or your app deletes speaker for an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can delete a speaker for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Speaker Deleted** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_speakers:write:admin`,`zoom_events_speakers:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:speaker`,`zoom_events:read:speaker:admin`\n\n**Event type**: `zoom_events.speaker_deleted`\n",
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
                    "description": "The name of the event, zoom_events.speaker_deleted"
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
                        "description": "The account ID of the host who deleted the event speaker."
                      },
                      "object": {
                        "type": "object",
                        "description": "Speaker Description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "speaker_id",
                          "name"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The zoom-events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "speaker_id": {
                            "type": "string",
                            "description": "The speaker ID."
                          },
                          "name": {
                            "type": "string",
                            "description": "The speaker's name."
                          },
                          "email": {
                            "type": "string",
                            "description": "The speaker's email."
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
                  "value": "{\n  \"event\": \"zoom_events.speaker_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"speaker_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"speaker1\",\n      \"email\": \"speaker1-email@zoom.us\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.speaker_deleted"
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
                          "description": "The account ID of the host who deleted the event speaker."
                        },
                        "object": {
                          "type": "object",
                          "description": "Speaker Description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "speaker_id",
                            "name"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The zoom-events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "speaker_id": {
                              "type": "string",
                              "description": "The speaker ID."
                            },
                            "name": {
                              "type": "string",
                              "description": "The speaker's name."
                            },
                            "email": {
                              "type": "string",
                              "description": "The speaker's email."
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
                    "value": "{\n  \"event\": \"zoom_events.speaker_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"speaker_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"speaker1\",\n      \"email\": \"speaker1-email@zoom.us\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.ticket_type_deleted": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.ticket_type_deleted",
        "requestBody": {
          "description": "# zoom_events.ticket_type_deleted\nThe **TicketType Deleted**  event  is triggered every time a user in your account or your app deletes ticketType for an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can delete a ticketType for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **TicketType Deleted** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_ticket_types:write:admin`,`zoom_events_ticket_types:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:list_ticket_types`,`zoom_events:read:list_ticket_types:admin`\n\n**Event type**: `zoom_events.ticket_type_deleted`\n",
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
                    "description": "The name of the event, zoom_events.ticket_type_deleted"
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
                        "description": "The account ID of the host who deleted the ticketType."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the TicketType.",
                        "required": [
                          "event_id",
                          "host_id",
                          "ticket_type_id",
                          "name",
                          "start_time",
                          "end_time"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The zoom-events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "ticket_type_id": {
                            "type": "string",
                            "description": "The ticketType ID for which the user registers to."
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the ticketType.",
                            "example": "General Admission Ticket"
                          },
                          "description": {
                            "type": "string",
                            "description": "The description of the ticketType.",
                            "example": "This is a General Admission ticket"
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
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
                  "value": "{\n  \"event\": \"zoom_events.ticket_type_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"name\": \"General Admission Ticket\",\n      \"description\": \"This is for General Admission\",\n      \"attendance_type\": \"virtual\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.ticket_type_deleted"
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
                          "description": "The account ID of the host who deleted the ticketType."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the TicketType.",
                          "required": [
                            "event_id",
                            "host_id",
                            "ticket_type_id",
                            "name",
                            "start_time",
                            "end_time"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The zoom-events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "ticket_type_id": {
                              "type": "string",
                              "description": "The ticketType ID for which the user registers to."
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the ticketType.",
                              "example": "General Admission Ticket"
                            },
                            "description": {
                              "type": "string",
                              "description": "The description of the ticketType.",
                              "example": "This is a General Admission ticket"
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
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
                    "value": "{\n  \"event\": \"zoom_events.ticket_type_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"name\": \"General Admission Ticket\",\n      \"description\": \"This is for General Admission\",\n      \"attendance_type\": \"virtual\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.session_updated": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.session_updated",
        "requestBody": {
          "description": "# zoom_events.session_updated\n\nThe **Session Updated** event is triggered every time a user in your account or your app, updates a session in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host or a co-editor can update the session.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Session Updated** subscription enabled under the **Zoom Events** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_sessions:write:admin`,`zoom_events_sessions:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:session`,`zoom_events:read:session:admin`\n\n**Event type**: `zoom_events.session_updated`\n",
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
                    "description": "The name of the event. zoom_events.event.session_updated."
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
                        "description": "The account ID of the event host who updated the session post event publishing."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the session.",
                        "required": [
                          "event_id",
                          "host_id",
                          "session_id",
                          "name",
                          "type",
                          "start_time",
                          "end_time",
                          "timezone"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The Zoom Events ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "session_id": {
                            "type": "string",
                            "description": "The session ID.",
                            "example": "4uzfv3JwTeyR5QpC3PXwMg"
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the session.",
                            "example": "Session 1"
                          },
                          "type": {
                            "type": "integer",
                            "description": "The session type. \n * 0 \u2014 Meeting \n * 2 \u2014 Webinar",
                            "enum": [
                              0,
                              2
                            ],
                            "example": "2"
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-05-31T13:00:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-05-31T13:00:00Z"
                          },
                          "timezone": {
                            "type": "string",
                            "description": "The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones).",
                            "example": "America/Los_Angeles"
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
                  "value": "{\n  \"event\": \"zoom_events.session_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"session_id\": \"fhhfkdhkhdgkhg\",\n      \"name\": \"my session\",\n      \"type\": 2,\n      \"attendance_type\": \"virtual\",\n      \"start_time\": \"2022-05-31T13:00:00Z\",\n      \"end_time\": \"2022-05-31T13:00:00Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
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
                      "description": "The name of the event. zoom_events.event.session_updated."
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
                          "description": "The account ID of the event host who updated the session post event publishing."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the session.",
                          "required": [
                            "event_id",
                            "host_id",
                            "session_id",
                            "name",
                            "type",
                            "start_time",
                            "end_time",
                            "timezone"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The Zoom Events ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "session_id": {
                              "type": "string",
                              "description": "The session ID.",
                              "example": "4uzfv3JwTeyR5QpC3PXwMg"
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the session.",
                              "example": "Session 1"
                            },
                            "type": {
                              "type": "integer",
                              "description": "The session type. \n * 0 \u2014 Meeting \n * 2 \u2014 Webinar",
                              "enum": [
                                0,
                                2
                              ],
                              "example": "2"
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-05-31T13:00:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-05-31T13:00:00Z"
                            },
                            "timezone": {
                              "type": "string",
                              "description": "The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones).",
                              "example": "America/Los_Angeles"
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
                    "value": "{\n  \"event\": \"zoom_events.session_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"session_id\": \"fhhfkdhkhdgkhg\",\n      \"name\": \"my session\",\n      \"type\": 2,\n      \"attendance_type\": \"virtual\",\n      \"start_time\": \"2022-05-31T13:00:00Z\",\n      \"end_time\": \"2022-05-31T13:00:00Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.session_created": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.session_created",
        "requestBody": {
          "description": "# zoom_events.session_created\nThe **Session Created** event is triggered every time a user in your account or your app, creates a session in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host or a co-editor can add a session.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Session Created** subscription enabled under the **Zoom Events** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_sessions:write:admin`,`zoom_events_sessions:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:session`,`zoom_events:read:session:admin`\n\n**Event type**: `zoom_events.session_created`\n",
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
                    "description": "The name of the event. zoom_events.event.session_created."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "A timestamp at when the event occurred."
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
                        "description": "The account ID of the host who created the session post event publishing."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the session.",
                        "required": [
                          "event_id",
                          "host_id",
                          "session_id",
                          "name",
                          "type",
                          "start_time",
                          "end_time",
                          "timezone"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The Zoom Events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "session_id": {
                            "type": "string",
                            "description": "The session ID.",
                            "example": "4uzfv3JwTeyR5QpC3PXwMg"
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the session.",
                            "example": "Session 1"
                          },
                          "type": {
                            "type": "integer",
                            "description": "The session type. \n * 0 \u2014 Meeting \n * 2 \u2014 Webinar",
                            "enum": [
                              0,
                              2
                            ]
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-05-31T13:00:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-05-31T13:00:00Z"
                          },
                          "timezone": {
                            "type": "string",
                            "description": "The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones)."
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
                  "value": "{\n  \"event\": \"zoom_events.session_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"session_id\": \"fhhfkdhkhdgkhg\",\n      \"name\": \"my session\",\n      \"attendance_type\": \"virtual\",\n      \"type\": 2,\n      \"start_time\": \"2022-05-31T13:00:00Z\",\n      \"end_time\": \"2022-05-31T13:00:00Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
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
                      "description": "The name of the event. zoom_events.event.session_created."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "A timestamp at when the event occurred."
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
                          "description": "The account ID of the host who created the session post event publishing."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the session.",
                          "required": [
                            "event_id",
                            "host_id",
                            "session_id",
                            "name",
                            "type",
                            "start_time",
                            "end_time",
                            "timezone"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The Zoom Events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "session_id": {
                              "type": "string",
                              "description": "The session ID.",
                              "example": "4uzfv3JwTeyR5QpC3PXwMg"
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the session.",
                              "example": "Session 1"
                            },
                            "type": {
                              "type": "integer",
                              "description": "The session type. \n * 0 \u2014 Meeting \n * 2 \u2014 Webinar",
                              "enum": [
                                0,
                                2
                              ]
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-05-31T13:00:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-05-31T13:00:00Z"
                            },
                            "timezone": {
                              "type": "string",
                              "description": "The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones)."
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
                    "value": "{\n  \"event\": \"zoom_events.session_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"session_id\": \"fhhfkdhkhdgkhg\",\n      \"name\": \"my session\",\n      \"attendance_type\": \"virtual\",\n      \"type\": 2,\n      \"start_time\": \"2022-05-31T13:00:00Z\",\n      \"end_time\": \"2022-05-31T13:00:00Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.ticket_type_created": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.ticket_type_created",
        "requestBody": {
          "description": "# zoom_events.ticket_type_created\nThe **TicketType Created**  event  is triggered every time a user in your account or your app creates ticketType for an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can create a ticketType for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **TicketType Created** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_ticket_types:write:admin`,`zoom_events_ticket_types:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:list_ticket_types`,`zoom_events:read:list_ticket_types:admin`\n\n**Event type**: `zoom_events.ticket_type_created`\n",
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
                    "description": "The name of the event, zoom_events.ticket_type_created"
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
                        "description": "The account ID of the host who created the ticketType."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the ticketType.",
                        "required": [
                          "event_id",
                          "host_id",
                          "ticket_type_id",
                          "name",
                          "start_time",
                          "end_time"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The zoom-events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "ticket_type_id": {
                            "type": "string",
                            "description": "The ticketType ID for which the user registers to."
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the ticketType.",
                            "example": "General Admission Ticket"
                          },
                          "description": {
                            "type": "string",
                            "description": "The description of the ticketType.",
                            "example": "This is a General Admission ticket"
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
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
                  "value": "{\n  \"event\": \"zoom_events.ticket_type_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"name\": \"General Admission Ticket\",\n      \"description\": \"This is for General Admission\",\n      \"attendance_type\": \"virtual\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.ticket_type_created"
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
                          "description": "The account ID of the host who created the ticketType."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the ticketType.",
                          "required": [
                            "event_id",
                            "host_id",
                            "ticket_type_id",
                            "name",
                            "start_time",
                            "end_time"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The zoom-events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "ticket_type_id": {
                              "type": "string",
                              "description": "The ticketType ID for which the user registers to."
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the ticketType.",
                              "example": "General Admission Ticket"
                            },
                            "description": {
                              "type": "string",
                              "description": "The description of the ticketType.",
                              "example": "This is a General Admission ticket"
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
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
                    "value": "{\n  \"event\": \"zoom_events.ticket_type_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"name\": \"General Admission Ticket\",\n      \"description\": \"This is for General Admission\",\n      \"attendance_type\": \"virtual\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.event_updated": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.event_updated",
        "requestBody": {
          "description": "# zoom_events.event_updated\nThe **Event Updated** is triggered every time a user in your account or your app, makes changes to the event data in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host or a co-editor can update an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Event Updated** subscription enabled under the **Zoom Events** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_basic:write:admin`,`zoom_events_basic:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:event`,`zoom_events:read:event:admin`\n\n**Event type**: `zoom_events.event_updated`\n",
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
                    "description": "The name of the event. zoom_events.event_updated."
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
                        "description": "The account ID of the host who updated the event."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom Event description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "name",
                          "event_type",
                          "start_time",
                          "end_time",
                          "timezone"
                        ],
                        "properties": {
                          "hub_id": {
                            "type": "string",
                            "description": "The ID of the event hub.",
                            "example": "234kj2h34kljgh23lkhj3"
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The ID of the event.",
                            "example": "234kj2h34kljgh23lkhj3"
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the event.",
                            "example": "OpenAPI Conference Name"
                          },
                          "description": {
                            "type": "string",
                            "description": "The description of the event.",
                            "example": "This event was created with the OpenAPI"
                          },
                          "event_type": {
                            "type": "string",
                            "description": "Event Types: \n * CONFERENCE - Multi session event \n * SIMPLE_EVENT - Single session event \n * RECURRING - Recurring session event",
                            "enum": [
                              "CONFERENCE",
                              "SIMPLE_EVENT",
                              "RECURRING"
                            ],
                            "example": "CONFERENCE"
                          },
                          "status": {
                            "type": "string",
                            "description": "The event status: \n * PUBLISHED - Published event \n * DRAFT - Draft event \n * CANCELLED - Cancelled event",
                            "enum": [
                              "PUBLISHED",
                              "DRAFT",
                              "CANCELLED"
                            ],
                            "example": "PUBLISHED"
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "timezone": {
                            "type": "string",
                            "description": "The timezone of the event.",
                            "example": "America/Los_Angeles"
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
                  "value": "{\n  \"event\": \"zoom_events.event_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"hub_id\": \"1234567890abcd\",\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"name\": \"My Event-1\",\n      \"description\": \"This is a test event.\",\n      \"attendance_type\": \"virtual\",\n      \"event_type\": \"CONFERENCE\",\n      \"status\": \"DRAFT\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"timezone\": \"America/Los_Angeles\",\n      \"end_time\": \"2023-07-13T23:00:51Z\"\n    }\n  }\n}"
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
                      "description": "The name of the event. zoom_events.event_updated."
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
                          "description": "The account ID of the host who updated the event."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom Event description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "name",
                            "event_type",
                            "start_time",
                            "end_time",
                            "timezone"
                          ],
                          "properties": {
                            "hub_id": {
                              "type": "string",
                              "description": "The ID of the event hub.",
                              "example": "234kj2h34kljgh23lkhj3"
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The ID of the event.",
                              "example": "234kj2h34kljgh23lkhj3"
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the event.",
                              "example": "OpenAPI Conference Name"
                            },
                            "description": {
                              "type": "string",
                              "description": "The description of the event.",
                              "example": "This event was created with the OpenAPI"
                            },
                            "event_type": {
                              "type": "string",
                              "description": "Event Types: \n * CONFERENCE - Multi session event \n * SIMPLE_EVENT - Single session event \n * RECURRING - Recurring session event",
                              "enum": [
                                "CONFERENCE",
                                "SIMPLE_EVENT",
                                "RECURRING"
                              ],
                              "example": "CONFERENCE"
                            },
                            "status": {
                              "type": "string",
                              "description": "The event status: \n * PUBLISHED - Published event \n * DRAFT - Draft event \n * CANCELLED - Cancelled event",
                              "enum": [
                                "PUBLISHED",
                                "DRAFT",
                                "CANCELLED"
                              ],
                              "example": "PUBLISHED"
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "timezone": {
                              "type": "string",
                              "description": "The timezone of the event.",
                              "example": "America/Los_Angeles"
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
                    "value": "{\n  \"event\": \"zoom_events.event_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"hub_id\": \"1234567890abcd\",\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"name\": \"My Event-1\",\n      \"description\": \"This is a test event.\",\n      \"attendance_type\": \"virtual\",\n      \"event_type\": \"CONFERENCE\",\n      \"status\": \"DRAFT\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"timezone\": \"America/Los_Angeles\",\n      \"end_time\": \"2023-07-13T23:00:51Z\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.speaker_created": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.speaker_created",
        "requestBody": {
          "description": "# zoom_events.speaker_created\nThe **Speaker Created**  event  is triggered every time a user in your account or your app creates speaker for an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can create a speaker for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Speaker Created** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_speakers:write:admin`,`zoom_events_speakers:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:speaker`,`zoom_events:read:speaker:admin`\n\n**Event type**: `zoom_events.speaker_created`\n",
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
                    "description": "The name of the event, zoom_events.speaker_created"
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
                        "description": "The account ID of the host who created the event speaker."
                      },
                      "object": {
                        "type": "object",
                        "description": "Speaker Description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "speaker_id",
                          "name"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The Zoom Events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "speaker_id": {
                            "type": "string",
                            "description": "The speaker ID."
                          },
                          "name": {
                            "type": "string",
                            "description": "The speaker's name."
                          },
                          "email": {
                            "type": "string",
                            "description": "The speaker's email."
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
                  "value": "{\n  \"event\": \"zoom_events.speaker_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"speaker_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"speaker1\",\n      \"email\": \"speaker1-email@zoom.us\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.speaker_created"
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
                          "description": "The account ID of the host who created the event speaker."
                        },
                        "object": {
                          "type": "object",
                          "description": "Speaker Description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "speaker_id",
                            "name"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The Zoom Events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "speaker_id": {
                              "type": "string",
                              "description": "The speaker ID."
                            },
                            "name": {
                              "type": "string",
                              "description": "The speaker's name."
                            },
                            "email": {
                              "type": "string",
                              "description": "The speaker's email."
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
                    "value": "{\n  \"event\": \"zoom_events.speaker_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"speaker_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"speaker1\",\n      \"email\": \"speaker1-email@zoom.us\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.ticket_created": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.ticket_created",
        "requestBody": {
          "description": "# zoom_events.ticket_created\n   The **Ticket Created**  event  is triggered when someone registers for an event created by one of your app users or account users.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events.  The host can create an event, set up a registration form, pre-register users or send out a registration email invitation. Users can also register themselves for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Ticket Created** subscription enabled under the **Zoom Events** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_tickets:write:admin`,`zoom_events_tickets:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:ticket`,`zoom_events:read:ticket:admin`\n\n**Event type**: `zoom_events.ticket_created`\n",
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
                    "description": "The name of the event. zoom_events.ticket_created."
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
                        "description": "The account ID of the host who created the event."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom Event description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "ticket_id",
                          "email"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The Zoom Events ID."
                          },
                          "ticket_id": {
                            "type": "string",
                            "description": "The ticket ID generated from the registration."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "ticket_type_id": {
                            "type": "string",
                            "description": "The ticket type ID for which the user registers to."
                          },
                          "external_ticket_id": {
                            "type": "string",
                            "description": "A set of unique alphanumeric characters that references the external ticket ID."
                          },
                          "email": {
                            "type": "string",
                            "description": "The registrant's email."
                          },
                          "ticket_role_type": {
                            "type": "string",
                            "description": "The ticket role type: \n * normal The normal ticket \n * alternative_host The alternative host ticket \n * panelist The panelist ticket \n * bind_meeting The bind meeting ticket \n * speaker The speaker ticket \n * interpreter The interpreter ticket \n * sponsor The sponsor ticket \n * host The host ticket \n * moderator The moderator ticket \n * expo_booth_owner The expo booth owner ticket \n * attendee_without_order The attendee without order ticket \n * pre_registration \u2014 The pre-registration ticket \n * company_zoom_room The company zoom room ticket \n * moderator_allhost The moderator all host ticket \n * hub_host The hub host ticket \n * hub_owner The hub owner ticket \n * subscription The subscription ticket",
                            "example": "normal",
                            "enum": [
                              "normal",
                              "alternative_host",
                              "panelist",
                              "bind_meeting",
                              "speaker",
                              "subscription",
                              "sponsor",
                              "interpreter",
                              "host",
                              "moderator",
                              "expo_booth_owner",
                              "attendee_without_order",
                              "pre_registration",
                              "company_zoom_room",
                              "moderator_allhost",
                              "hub_host",
                              "hub_owner"
                            ]
                          },
                          "authentication_method": {
                            "type": "string",
                            "description": "The event authentication method for the ticket: \n * zoom_account Zoom account holder  * zoom_account_otp Zoom account holder with OTP  * corporate_idp Corporate IDP  * no_auth No authentication",
                            "enum": [
                              "zoom_account",
                              "zoom_account_otp",
                              "corporate_idp",
                              "no_auth"
                            ],
                            "example": "no_auth"
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
                  "value": "{\n  \"event\": \"zoom_events.ticket_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"djsjgkjhfhk\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_id\": \"iso9Dllddp39dagjLj9j\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"external_ticket_id\": \"abc1231245-adf\",\n      \"ticket_role_type\": \"normal\",\n      \"email\": \"email@zoom.us\",\n      \"authentication_method\": \"no_auth\"\n    }\n  }\n}"
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
                      "description": "The name of the event. zoom_events.ticket_created."
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
                          "description": "The account ID of the host who created the event."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom Event description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "ticket_id",
                            "email"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The Zoom Events ID."
                            },
                            "ticket_id": {
                              "type": "string",
                              "description": "The ticket ID generated from the registration."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "ticket_type_id": {
                              "type": "string",
                              "description": "The ticket type ID for which the user registers to."
                            },
                            "external_ticket_id": {
                              "type": "string",
                              "description": "A set of unique alphanumeric characters that references the external ticket ID."
                            },
                            "email": {
                              "type": "string",
                              "description": "The registrant's email."
                            },
                            "ticket_role_type": {
                              "type": "string",
                              "description": "The ticket role type: \n * normal The normal ticket \n * alternative_host The alternative host ticket \n * panelist The panelist ticket \n * bind_meeting The bind meeting ticket \n * speaker The speaker ticket \n * interpreter The interpreter ticket \n * sponsor The sponsor ticket \n * host The host ticket \n * moderator The moderator ticket \n * expo_booth_owner The expo booth owner ticket \n * attendee_without_order The attendee without order ticket \n * pre_registration \u2014 The pre-registration ticket \n * company_zoom_room The company zoom room ticket \n * moderator_allhost The moderator all host ticket \n * hub_host The hub host ticket \n * hub_owner The hub owner ticket \n * subscription The subscription ticket",
                              "example": "normal",
                              "enum": [
                                "normal",
                                "alternative_host",
                                "panelist",
                                "bind_meeting",
                                "speaker",
                                "subscription",
                                "sponsor",
                                "interpreter",
                                "host",
                                "moderator",
                                "expo_booth_owner",
                                "attendee_without_order",
                                "pre_registration",
                                "company_zoom_room",
                                "moderator_allhost",
                                "hub_host",
                                "hub_owner"
                              ]
                            },
                            "authentication_method": {
                              "type": "string",
                              "description": "The event authentication method for the ticket: \n * zoom_account Zoom account holder  * zoom_account_otp Zoom account holder with OTP  * corporate_idp Corporate IDP  * no_auth No authentication",
                              "enum": [
                                "zoom_account",
                                "zoom_account_otp",
                                "corporate_idp",
                                "no_auth"
                              ],
                              "example": "no_auth"
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
                    "value": "{\n  \"event\": \"zoom_events.ticket_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"djsjgkjhfhk\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_id\": \"iso9Dllddp39dagjLj9j\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"external_ticket_id\": \"abc1231245-adf\",\n      \"ticket_role_type\": \"normal\",\n      \"email\": \"email@zoom.us\",\n      \"authentication_method\": \"no_auth\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.exhibitor_deleted": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.exhibitor_deleted",
        "requestBody": {
          "description": "# zoom_events.exhibitor_deleted\nThe **Exhibitor Deleted**  event  is triggered every time a user in your account or your app deletes exhibitor for an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can delete an exhibitor for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Exhibitor Deleted** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_exhibitors:write:admin`,`zoom_events_exhibitors:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:exhibitor`,`zoom_events:read:exhibitor:admin`\n\n**Event type**: `zoom_events.exhibitor_deleted`\n",
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
                    "description": "The name of the event, zoom_events.exhibitor_deleted"
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
                        "description": "The account ID of the host who deleted the event exhibitor."
                      },
                      "object": {
                        "type": "object",
                        "description": "Exhibitor Description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "exhibitor_id",
                          "name",
                          "contact_name",
                          "contact_email",
                          "is_sponsor"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The zoom-events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "exhibitor_id": {
                            "type": "string",
                            "description": "The exhibitors's ID."
                          },
                          "name": {
                            "type": "string",
                            "description": "The exhibitors's name."
                          },
                          "description": {
                            "type": "string",
                            "description": "The exhibitor's description."
                          },
                          "is_sponsor": {
                            "type": "boolean",
                            "description": "Whether the exhibitor is a sponsor."
                          },
                          "tier_id": {
                            "type": "string",
                            "description": "The sponsor tier ID for a particular event. This field only applies to a sponsor."
                          },
                          "contact_name": {
                            "type": "string",
                            "description": "The contact's full name."
                          },
                          "contact_email": {
                            "type": "string",
                            "description": "The contact's email address."
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
                  "value": "{\n  \"event\": \"zoom_events.exhibitor_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"exhibitor_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"exhibitor1\",\n      \"description\": \"Description of the exhibitor\",\n      \"is_sponsor\": true,\n      \"tier_id\": \"4fgRIOXjQGmoH6raAzJf8g\",\n      \"contact_name\": \"John Doe\",\n      \"contact_email\": \"exhibitor1-email@zoom.us\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.exhibitor_deleted"
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
                          "description": "The account ID of the host who deleted the event exhibitor."
                        },
                        "object": {
                          "type": "object",
                          "description": "Exhibitor Description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "exhibitor_id",
                            "name",
                            "contact_name",
                            "contact_email",
                            "is_sponsor"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The zoom-events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "exhibitor_id": {
                              "type": "string",
                              "description": "The exhibitors's ID."
                            },
                            "name": {
                              "type": "string",
                              "description": "The exhibitors's name."
                            },
                            "description": {
                              "type": "string",
                              "description": "The exhibitor's description."
                            },
                            "is_sponsor": {
                              "type": "boolean",
                              "description": "Whether the exhibitor is a sponsor."
                            },
                            "tier_id": {
                              "type": "string",
                              "description": "The sponsor tier ID for a particular event. This field only applies to a sponsor."
                            },
                            "contact_name": {
                              "type": "string",
                              "description": "The contact's full name."
                            },
                            "contact_email": {
                              "type": "string",
                              "description": "The contact's email address."
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
                    "value": "{\n  \"event\": \"zoom_events.exhibitor_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"exhibitor_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"exhibitor1\",\n      \"description\": \"Description of the exhibitor\",\n      \"is_sponsor\": true,\n      \"tier_id\": \"4fgRIOXjQGmoH6raAzJf8g\",\n      \"contact_name\": \"John Doe\",\n      \"contact_email\": \"exhibitor1-email@zoom.us\"\n    }\n  }\n}"
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
    "title": "Events",
    "description": "The Webinars Plus & Events webhooks allow developers to receive events for [Webinars Plus & Events](https://developers.zoom.us/docs/zoom-events/) features.",
    "version": "1.0.0"
  },
  "webhooks": {
    "zoom_events.session_deleted": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.session_deleted",
        "requestBody": {
          "description": "# zoom_events.session_deleted\nThe **Session Deleted** event is triggered every time a user in your account or your app, deletes a session in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host or a co-editor can delete a session.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Session Deleted** subscription enabled under the **Zoom Events** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_sessions:write:admin`,`zoom_events_sessions:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:session`,`zoom_events:read:session:admin`\n\n**Event type**: `zoom_events.session_deleted`\n",
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
                    "description": "The name of the event. zoom_events.event.session_deleted."
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
                        "description": "The account ID of the host who deleted the session post event publishing."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the session.",
                        "required": [
                          "event_id",
                          "host_id",
                          "session_id",
                          "name",
                          "type",
                          "start_time",
                          "end_time",
                          "timezone"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The Zoom Events ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "session_id": {
                            "type": "string",
                            "description": "The session ID.",
                            "example": "4uzfv3JwTeyR5QpC3PXwMg"
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the session.",
                            "example": "Session 1"
                          },
                          "type": {
                            "type": "integer",
                            "description": "The session type. \n * 0 \u2014 Meeting \n * 2 \u2014 Webinar",
                            "enum": [
                              0,
                              2
                            ]
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-05-31T13:00:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-05-31T13:00:00Z"
                          },
                          "timezone": {
                            "type": "string",
                            "description": "The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones)."
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
                  "value": "{\n  \"event\": \"zoom_events.session_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"session_id\": \"fhhfkdhkhdgkhg\",\n      \"name\": \"my session\",\n      \"attendance_type\": \"virtual\",\n      \"type\": 2,\n      \"start_time\": \"2022-05-31T13:00:00Z\",\n      \"end_time\": \"2022-05-31T13:00:00Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
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
                      "description": "The name of the event. zoom_events.event.session_deleted."
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
                          "description": "The account ID of the host who deleted the session post event publishing."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the session.",
                          "required": [
                            "event_id",
                            "host_id",
                            "session_id",
                            "name",
                            "type",
                            "start_time",
                            "end_time",
                            "timezone"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The Zoom Events ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "session_id": {
                              "type": "string",
                              "description": "The session ID.",
                              "example": "4uzfv3JwTeyR5QpC3PXwMg"
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the session.",
                              "example": "Session 1"
                            },
                            "type": {
                              "type": "integer",
                              "description": "The session type. \n * 0 \u2014 Meeting \n * 2 \u2014 Webinar",
                              "enum": [
                                0,
                                2
                              ]
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-05-31T13:00:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-05-31T13:00:00Z"
                            },
                            "timezone": {
                              "type": "string",
                              "description": "The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones)."
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
                    "value": "{\n  \"event\": \"zoom_events.session_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"session_id\": \"fhhfkdhkhdgkhg\",\n      \"name\": \"my session\",\n      \"attendance_type\": \"virtual\",\n      \"type\": 2,\n      \"start_time\": \"2022-05-31T13:00:00Z\",\n      \"end_time\": \"2022-05-31T13:00:00Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.ticket_updated": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.ticket_updated",
        "requestBody": {
          "description": "# zoom_events.ticket_updated\nThe **Ticket Updated**  event  is triggered every time a user in your account or your app updates the attendee registration information in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can update a pre-registration for an event by updating registration question answers of a ticket.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Ticket Updated** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_tickets:write:admin`,`zoom_events_tickets:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:ticket`,`zoom_events:read:ticket:admin`\n\n**Event type**: `zoom_events.ticket_updated`\n",
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
                    "description": "The name of the event, zoom_events.ticket_updated"
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
                        "description": "The account ID of the host who created the event."
                      },
                      "object": {
                        "type": "object",
                        "description": " The ticket Description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "ticket_id",
                          "email"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The Zoom Events ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "ticket_id": {
                            "type": "string",
                            "description": "The ticket ID generated from the registration."
                          },
                          "ticket_type_id": {
                            "type": "string",
                            "description": "The ticket type ID for which the user registers to."
                          },
                          "external_ticket_id": {
                            "type": "string",
                            "description": "A set of unique alphanumeric characters that references the external ticket ID."
                          },
                          "email": {
                            "type": "string",
                            "description": "The registrant's email."
                          },
                          "ticket_role_type": {
                            "type": "string",
                            "description": "The ticket role type: \n * normal The normal ticket \n * alternative_host The alternative host ticket \n * panelist The panelist ticket \n * bind_meeting The bind meeting ticket \n * speaker The speaker ticket \n * interpreter The interpreter ticket \n * sponsor The sponsor ticket \n * host The host ticket \n * moderator The moderator ticket \n * expo_booth_owner The expo booth owner ticket \n * attendee_without_order The attendee without order ticket \n * company_zoom_room The company zoom room ticket \n * moderator_allhost The moderator all host ticket \n * hub_host The hub host ticket \n * hub_owner The hub owner ticket &quot;,\n * subscription The subscription ticket &quot;",
                            "example": "normal",
                            "enum": [
                              "normal",
                              "alternative_host",
                              "panelist",
                              "bind_meeting",
                              "speaker",
                              "subscription",
                              "sponsor",
                              "interpreter",
                              "host",
                              "moderator",
                              "expo_booth_owner",
                              "attendee_without_order",
                              "company_zoom_room",
                              "moderator_allhost",
                              "hub_host",
                              "hub_owner"
                            ]
                          },
                          "authentication_method": {
                            "type": "string",
                            "description": "The event authentication method for the ticket: \n * zoom_account Zoom account holder  * zoom_account_otp Zoom account holder with OTP  * corporate_idp Corporate IDP  * no_auth No authentication",
                            "enum": [
                              "zoom_account",
                              "zoom_account_otp",
                              "corporate_idp",
                              "no_auth"
                            ],
                            "example": "no_auth"
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
                  "value": "{\n  \"event\": \"zoom_events.ticket_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_id\": \"iso9Dllddp39dagjLj9j\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"external_ticket_id\": \"abc-123-efgh\",\n      \"ticket_role_type\": \"normal\",\n      \"email\": \"email@zoom.us\",\n      \"authentication_method\": \"no_auth\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.ticket_updated"
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
                          "description": "The account ID of the host who created the event."
                        },
                        "object": {
                          "type": "object",
                          "description": " The ticket Description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "ticket_id",
                            "email"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The Zoom Events ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "ticket_id": {
                              "type": "string",
                              "description": "The ticket ID generated from the registration."
                            },
                            "ticket_type_id": {
                              "type": "string",
                              "description": "The ticket type ID for which the user registers to."
                            },
                            "external_ticket_id": {
                              "type": "string",
                              "description": "A set of unique alphanumeric characters that references the external ticket ID."
                            },
                            "email": {
                              "type": "string",
                              "description": "The registrant's email."
                            },
                            "ticket_role_type": {
                              "type": "string",
                              "description": "The ticket role type: \n * normal The normal ticket \n * alternative_host The alternative host ticket \n * panelist The panelist ticket \n * bind_meeting The bind meeting ticket \n * speaker The speaker ticket \n * interpreter The interpreter ticket \n * sponsor The sponsor ticket \n * host The host ticket \n * moderator The moderator ticket \n * expo_booth_owner The expo booth owner ticket \n * attendee_without_order The attendee without order ticket \n * company_zoom_room The company zoom room ticket \n * moderator_allhost The moderator all host ticket \n * hub_host The hub host ticket \n * hub_owner The hub owner ticket &quot;,\n * subscription The subscription ticket &quot;",
                              "example": "normal",
                              "enum": [
                                "normal",
                                "alternative_host",
                                "panelist",
                                "bind_meeting",
                                "speaker",
                                "subscription",
                                "sponsor",
                                "interpreter",
                                "host",
                                "moderator",
                                "expo_booth_owner",
                                "attendee_without_order",
                                "company_zoom_room",
                                "moderator_allhost",
                                "hub_host",
                                "hub_owner"
                              ]
                            },
                            "authentication_method": {
                              "type": "string",
                              "description": "The event authentication method for the ticket: \n * zoom_account Zoom account holder  * zoom_account_otp Zoom account holder with OTP  * corporate_idp Corporate IDP  * no_auth No authentication",
                              "enum": [
                                "zoom_account",
                                "zoom_account_otp",
                                "corporate_idp",
                                "no_auth"
                              ],
                              "example": "no_auth"
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
                    "value": "{\n  \"event\": \"zoom_events.ticket_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_id\": \"iso9Dllddp39dagjLj9j\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"external_ticket_id\": \"abc-123-efgh\",\n      \"ticket_role_type\": \"normal\",\n      \"email\": \"email@zoom.us\",\n      \"authentication_method\": \"no_auth\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.event_published": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.event_published",
        "requestBody": {
          "description": "# zoom_events.event_published\nThe **Event Published** event is triggered every time a user in your account or your app, publishes an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host or a co-editor can publish an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Event published** subscription enabled under the **Zoom Events** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_basic:write:admin`,`zoom_events_basic:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:event`,`zoom_events:read:event:admin`\n\n**Event type**: `zoom_events.event_published`\n",
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
                    "description": "The name of the event. zoom_events.event_published."
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
                        "description": "The account ID of the host who published the event."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom Event description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "name",
                          "event_type",
                          "start_time",
                          "end_time",
                          "timezone"
                        ],
                        "properties": {
                          "hub_id": {
                            "type": "string",
                            "description": "The ID of the event hub.",
                            "example": "234kj2h34kljgh23lkhj3"
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The ID of the event.",
                            "example": "234kj2h34kljgh23lkhj3"
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the event.",
                            "example": "OpenAPI Conference Name"
                          },
                          "description": {
                            "type": "string",
                            "description": "The description of the event.",
                            "example": "This event was created with the OpenAPI"
                          },
                          "event_type": {
                            "type": "string",
                            "description": "Event Types: \n * CONFERENCE - Multi session event \n * SIMPLE_EVENT - Single session event \n * RECURRING - Recurring session event",
                            "enum": [
                              "CONFERENCE",
                              "SIMPLE_EVENT",
                              "RECURRING"
                            ],
                            "example": "CONFERENCE"
                          },
                          "status": {
                            "type": "string",
                            "description": "The event status: \n * PUBLISHED - Published event \n * DRAFT - Draft event \n * CANCELLED - Cancelled event",
                            "enum": [
                              "PUBLISHED",
                              "DRAFT",
                              "CANCELLED"
                            ],
                            "example": "PUBLISHED"
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "timezone": {
                            "type": "string",
                            "description": "The timezone of the event.",
                            "example": "America/Los_Angeles"
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
                  "value": "{\n  \"event\": \"zoom_events.event_published\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"hub_id\": \"1234567890abcd\",\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"name\": \"My Event-1\",\n      \"description\": \"This is a test event.\",\n      \"event_type\": \"CONFERENCE\",\n      \"attendance_type\": \"virtual\",\n      \"status\": \"DRAFT\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
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
                      "description": "The name of the event. zoom_events.event_published."
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
                          "description": "The account ID of the host who published the event."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom Event description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "name",
                            "event_type",
                            "start_time",
                            "end_time",
                            "timezone"
                          ],
                          "properties": {
                            "hub_id": {
                              "type": "string",
                              "description": "The ID of the event hub.",
                              "example": "234kj2h34kljgh23lkhj3"
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The ID of the event.",
                              "example": "234kj2h34kljgh23lkhj3"
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the event.",
                              "example": "OpenAPI Conference Name"
                            },
                            "description": {
                              "type": "string",
                              "description": "The description of the event.",
                              "example": "This event was created with the OpenAPI"
                            },
                            "event_type": {
                              "type": "string",
                              "description": "Event Types: \n * CONFERENCE - Multi session event \n * SIMPLE_EVENT - Single session event \n * RECURRING - Recurring session event",
                              "enum": [
                                "CONFERENCE",
                                "SIMPLE_EVENT",
                                "RECURRING"
                              ],
                              "example": "CONFERENCE"
                            },
                            "status": {
                              "type": "string",
                              "description": "The event status: \n * PUBLISHED - Published event \n * DRAFT - Draft event \n * CANCELLED - Cancelled event",
                              "enum": [
                                "PUBLISHED",
                                "DRAFT",
                                "CANCELLED"
                              ],
                              "example": "PUBLISHED"
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "timezone": {
                              "type": "string",
                              "description": "The timezone of the event.",
                              "example": "America/Los_Angeles"
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
                    "value": "{\n  \"event\": \"zoom_events.event_published\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"hub_id\": \"1234567890abcd\",\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"name\": \"My Event-1\",\n      \"description\": \"This is a test event.\",\n      \"event_type\": \"CONFERENCE\",\n      \"attendance_type\": \"virtual\",\n      \"status\": \"DRAFT\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.ticket_type_updated": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.ticket_type_updated",
        "requestBody": {
          "description": "# zoom_events.ticket_type_updated\nThe **TicketType Updated**  event  is triggered every time a user in your account or your app updates ticketType for an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can update a ticketType for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **TicketType Updated** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_ticket_types:write:admin`,`zoom_events_ticket_types:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:list_ticket_types`,`zoom_events:read:list_ticket_types:admin`\n\n**Event type**: `zoom_events.ticket_type_updated`\n",
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
                    "description": "The name of the event, zoom_events.ticket_type_updated"
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
                        "description": "The account ID of the host who updated the ticketType."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the ticketType.",
                        "required": [
                          "event_id",
                          "host_id",
                          "ticket_type_id",
                          "name",
                          "start_time",
                          "end_time"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The zoom-events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "ticket_type_id": {
                            "type": "string",
                            "description": "The ticketType ID for which the user registers to."
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the ticketType.",
                            "example": "General Admission Ticket"
                          },
                          "description": {
                            "type": "string",
                            "description": "The description of the ticketType.",
                            "example": "This is a General Admission ticket"
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
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
                  "value": "{\n  \"event\": \"zoom_events.ticket_type_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"name\": \"General Admission Ticket\",\n      \"description\": \"This is for General Admission\",\n      \"attendance_type\": \"virtual\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.ticket_type_updated"
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
                          "description": "The account ID of the host who updated the ticketType."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the ticketType.",
                          "required": [
                            "event_id",
                            "host_id",
                            "ticket_type_id",
                            "name",
                            "start_time",
                            "end_time"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The zoom-events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "ticket_type_id": {
                              "type": "string",
                              "description": "The ticketType ID for which the user registers to."
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the ticketType.",
                              "example": "General Admission Ticket"
                            },
                            "description": {
                              "type": "string",
                              "description": "The description of the ticketType.",
                              "example": "This is a General Admission ticket"
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
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
                    "value": "{\n  \"event\": \"zoom_events.ticket_type_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"name\": \"General Admission Ticket\",\n      \"description\": \"This is for General Admission\",\n      \"attendance_type\": \"virtual\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.exhibitor_created": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.exhibitor_created",
        "requestBody": {
          "description": "# zoom_events.exhibitor_created\nThe **Exhibitor Created**  event  is triggered every time a user in your account or your app creates exhibitor for an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can create an exhibitor for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Exhibitor Created** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_exhibitors:write:admin`,`zoom_events_exhibitors:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:exhibitor`,`zoom_events:read:exhibitor:admin`\n\n**Event type**: `zoom_events.exhibitor_created`\n",
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
                    "description": "The name of the event, zoom_events.exhibitor_created"
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
                        "description": "The account ID of the host who created the event exhibitor."
                      },
                      "object": {
                        "type": "object",
                        "description": "Exhibitor Description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "exhibitor_id",
                          "name",
                          "contact_name",
                          "contact_email",
                          "is_sponsor"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The zoom-events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "exhibitor_id": {
                            "type": "string",
                            "description": "The exhibitors's ID."
                          },
                          "name": {
                            "type": "string",
                            "description": "The exhibitors's name."
                          },
                          "description": {
                            "type": "string",
                            "description": "The exhibitor's description."
                          },
                          "is_sponsor": {
                            "type": "boolean",
                            "description": "Whether the exhibitor is a sponsor."
                          },
                          "tier_id": {
                            "type": "string",
                            "description": "The sponsor tier ID for a particular event. This field only applies to a sponsor."
                          },
                          "contact_name": {
                            "type": "string",
                            "description": "The contact's full name."
                          },
                          "contact_email": {
                            "type": "string",
                            "description": "The contact's email address."
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
                  "value": "{\n  \"event\": \"zoom_events.exhibitor_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"exhibitor_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"exhibitor1\",\n      \"description\": \"Description of the exhibitor\",\n      \"is_sponsor\": true,\n      \"tier_id\": \"4fgRIOXjQGmoH6raAzJf8g\",\n      \"contact_name\": \"John Doe\",\n      \"contact_email\": \"exhibitor1-email@zoom.us\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.exhibitor_created"
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
                          "description": "The account ID of the host who created the event exhibitor."
                        },
                        "object": {
                          "type": "object",
                          "description": "Exhibitor Description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "exhibitor_id",
                            "name",
                            "contact_name",
                            "contact_email",
                            "is_sponsor"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The zoom-events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "exhibitor_id": {
                              "type": "string",
                              "description": "The exhibitors's ID."
                            },
                            "name": {
                              "type": "string",
                              "description": "The exhibitors's name."
                            },
                            "description": {
                              "type": "string",
                              "description": "The exhibitor's description."
                            },
                            "is_sponsor": {
                              "type": "boolean",
                              "description": "Whether the exhibitor is a sponsor."
                            },
                            "tier_id": {
                              "type": "string",
                              "description": "The sponsor tier ID for a particular event. This field only applies to a sponsor."
                            },
                            "contact_name": {
                              "type": "string",
                              "description": "The contact's full name."
                            },
                            "contact_email": {
                              "type": "string",
                              "description": "The contact's email address."
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
                    "value": "{\n  \"event\": \"zoom_events.exhibitor_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"exhibitor_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"exhibitor1\",\n      \"description\": \"Description of the exhibitor\",\n      \"is_sponsor\": true,\n      \"tier_id\": \"4fgRIOXjQGmoH6raAzJf8g\",\n      \"contact_name\": \"John Doe\",\n      \"contact_email\": \"exhibitor1-email@zoom.us\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.exhibitor_updated": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.exhibitor_updated",
        "requestBody": {
          "description": "# zoom_events.exhibitor_updated\nThe **Exhibitor Updated**  event  is triggered every time a user in your account or your app updates exhibitor for an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can update an exhibitor for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Exhibitor Updated** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_exhibitors:write:admin`,`zoom_events_exhibitors:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:exhibitor`,`zoom_events:read:exhibitor:admin`\n\n**Event type**: `zoom_events.exhibitor_updated`\n",
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
                    "description": "The name of the event, zoom_events.exhibitor_updated"
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
                        "description": "The account ID of the host who updated the event exhibitor."
                      },
                      "object": {
                        "type": "object",
                        "description": "Exhibitor Description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "exhibitor_id",
                          "name",
                          "contact_name",
                          "contact_email",
                          "is_sponsor"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The zoom-events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "exhibitor_id": {
                            "type": "string",
                            "description": "The exhibitors's ID."
                          },
                          "name": {
                            "type": "string",
                            "description": "The exhibitors's name."
                          },
                          "description": {
                            "type": "string",
                            "description": "The exhibitor's description."
                          },
                          "is_sponsor": {
                            "type": "boolean",
                            "description": "Whether the exhibitor is a sponsor."
                          },
                          "tier_id": {
                            "type": "string",
                            "description": "The sponsor tier ID for a particular event. This field only applies to a sponsor."
                          },
                          "contact_name": {
                            "type": "string",
                            "description": "The contact's full name."
                          },
                          "contact_email": {
                            "type": "string",
                            "description": "The contact's email address."
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
                  "value": "{\n  \"event\": \"zoom_events.exhibitor_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"exhibitor_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"exhibitor1\",\n      \"description\": \"Description of the exhibitor\",\n      \"is_sponsor\": true,\n      \"tier_id\": \"4fgRIOXjQGmoH6raAzJf8g\",\n      \"contact_name\": \"John Doe\",\n      \"contact_email\": \"exhibitor1-email@zoom.us\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.exhibitor_updated"
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
                          "description": "The account ID of the host who updated the event exhibitor."
                        },
                        "object": {
                          "type": "object",
                          "description": "Exhibitor Description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "exhibitor_id",
                            "name",
                            "contact_name",
                            "contact_email",
                            "is_sponsor"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The zoom-events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "exhibitor_id": {
                              "type": "string",
                              "description": "The exhibitors's ID."
                            },
                            "name": {
                              "type": "string",
                              "description": "The exhibitors's name."
                            },
                            "description": {
                              "type": "string",
                              "description": "The exhibitor's description."
                            },
                            "is_sponsor": {
                              "type": "boolean",
                              "description": "Whether the exhibitor is a sponsor."
                            },
                            "tier_id": {
                              "type": "string",
                              "description": "The sponsor tier ID for a particular event. This field only applies to a sponsor."
                            },
                            "contact_name": {
                              "type": "string",
                              "description": "The contact's full name."
                            },
                            "contact_email": {
                              "type": "string",
                              "description": "The contact's email address."
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
                    "value": "{\n  \"event\": \"zoom_events.exhibitor_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"exhibitor_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"exhibitor1\",\n      \"description\": \"Description of the exhibitor\",\n      \"is_sponsor\": true,\n      \"tier_id\": \"4fgRIOXjQGmoH6raAzJf8g\",\n      \"contact_name\": \"John Doe\",\n      \"contact_email\": \"exhibitor1-email@zoom.us\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.event_cancelled": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.event_cancelled",
        "requestBody": {
          "description": "# zoom_events.event_cancelled\nThe **Event Cancelled** event is triggered every time a user in your account or your app, cancels the event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host can cancel the event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Event Cancelled** subscription enabled under the **Zoom Events** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_basic:write:admin`,`zoom_events_basic:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:event`,`zoom_events:read:event:admin`\n\n**Event type**: `zoom_events.event_cancelled`\n",
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
                    "description": "The name of the event. zoom_events.event_cancelled."
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
                        "description": "The account ID of the host who cancelled the event."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom Event description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "name",
                          "event_type",
                          "start_time",
                          "end_time",
                          "timezone"
                        ],
                        "properties": {
                          "hub_id": {
                            "type": "string",
                            "description": "The ID of the event hub.",
                            "example": "234kj2h34kljgh23lkhj3"
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The ID of the event.",
                            "example": "234kj2h34kljgh23lkhj3"
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the event.",
                            "example": "OpenAPI Conference Name"
                          },
                          "description": {
                            "type": "string",
                            "description": "The description of the event.",
                            "example": "This event was created with the OpenAPI"
                          },
                          "event_type": {
                            "type": "string",
                            "description": "Event Types: \n * CONFERENCE - Multi session event \n * SIMPLE_EVENT - Single session event \n * RECURRING - Recurring session event",
                            "enum": [
                              "CONFERENCE",
                              "SIMPLE_EVENT",
                              "RECURRING"
                            ],
                            "example": "CONFERENCE"
                          },
                          "status": {
                            "type": "string",
                            "description": "The event status: \n * PUBLISHED - Published event \n * DRAFT - Draft event \n * CANCELLED - Cancelled event",
                            "enum": [
                              "PUBLISHED",
                              "DRAFT",
                              "CANCELLED"
                            ],
                            "example": "PUBLISHED"
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "timezone": {
                            "type": "string",
                            "description": "The timezone of the event.",
                            "example": "America/Los_Angeles"
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
                  "value": "{\n  \"event\": \"zoom_events.event_cancelled\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"hub_id\": \"1234567890abcd\",\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"name\": \"My Event-1\",\n      \"description\": \"This is a test event.\",\n      \"attendance_type\": \"virtual\",\n      \"event_type\": \"CONFERENCE\",\n      \"status\": \"DRAFT\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
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
                      "description": "The name of the event. zoom_events.event_cancelled."
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
                          "description": "The account ID of the host who cancelled the event."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom Event description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "name",
                            "event_type",
                            "start_time",
                            "end_time",
                            "timezone"
                          ],
                          "properties": {
                            "hub_id": {
                              "type": "string",
                              "description": "The ID of the event hub.",
                              "example": "234kj2h34kljgh23lkhj3"
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The ID of the event.",
                              "example": "234kj2h34kljgh23lkhj3"
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the event.",
                              "example": "OpenAPI Conference Name"
                            },
                            "description": {
                              "type": "string",
                              "description": "The description of the event.",
                              "example": "This event was created with the OpenAPI"
                            },
                            "event_type": {
                              "type": "string",
                              "description": "Event Types: \n * CONFERENCE - Multi session event \n * SIMPLE_EVENT - Single session event \n * RECURRING - Recurring session event",
                              "enum": [
                                "CONFERENCE",
                                "SIMPLE_EVENT",
                                "RECURRING"
                              ],
                              "example": "CONFERENCE"
                            },
                            "status": {
                              "type": "string",
                              "description": "The event status: \n * PUBLISHED - Published event \n * DRAFT - Draft event \n * CANCELLED - Cancelled event",
                              "enum": [
                                "PUBLISHED",
                                "DRAFT",
                                "CANCELLED"
                              ],
                              "example": "PUBLISHED"
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "timezone": {
                              "type": "string",
                              "description": "The timezone of the event.",
                              "example": "America/Los_Angeles"
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
                    "value": "{\n  \"event\": \"zoom_events.event_cancelled\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"hub_id\": \"1234567890abcd\",\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"name\": \"My Event-1\",\n      \"description\": \"This is a test event.\",\n      \"attendance_type\": \"virtual\",\n      \"event_type\": \"CONFERENCE\",\n      \"status\": \"DRAFT\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.event_duplicated": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.event_duplicated",
        "requestBody": {
          "description": "# zoom_events.event_duplicated\nThe **Event Duplicated** is triggered every time a user in your account or your app, duplicates an existing event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host can duplicate an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Event Duplicated** subscription enabled under the **Zoom Events** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_basic:write:admin`,`zoom_events_basic:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:event`,`zoom_events:read:event:admin`\n\n**Event type**: `zoom_events.event_duplicated`\n",
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
                    "description": "The name of the event. zoom_events.event_duplicated."
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
                        "description": "The account ID of the host who duplicated the event."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom Event description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "name",
                          "event_type",
                          "start_time",
                          "end_time",
                          "timezone"
                        ],
                        "properties": {
                          "hub_id": {
                            "type": "string",
                            "description": "The ID of the event hub.",
                            "example": "h134kj2h34kljgh23lkhj3"
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The ID of the event.",
                            "example": "234kj2h34kljgh23lkhj3"
                          },
                          "source_event_id": {
                            "type": "string",
                            "description": "The ID of the event from which this event was duplicated from.",
                            "example": "434kj2h34kljgh23lkhj3"
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the event.",
                            "example": "OpenAPI Conference Name"
                          },
                          "description": {
                            "type": "string",
                            "description": "The description of the event.",
                            "example": "This event was created with the OpenAPI"
                          },
                          "event_type": {
                            "type": "string",
                            "description": "Event Types: \n * CONFERENCE - Multi session event \n * SIMPLE_EVENT - Single session event \n * RECURRING - Recurring session event",
                            "enum": [
                              "CONFERENCE",
                              "SIMPLE_EVENT",
                              "RECURRING"
                            ],
                            "example": "CONFERENCE"
                          },
                          "status": {
                            "type": "string",
                            "description": "The event status: \n * PUBLISHED - Published event \n * DRAFT - Draft event \n * CANCELLED - Cancelled event",
                            "enum": [
                              "PUBLISHED",
                              "DRAFT",
                              "CANCELLED"
                            ],
                            "example": "DRAFT"
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "timezone": {
                            "type": "string",
                            "description": "The timezone of the event.",
                            "example": "America/Los_Angeles"
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
                  "value": "{\n  \"event\": \"zoom_events.event_duplicated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"hub_id\": \"1234567890abcd\",\n      \"event_id\": \"1234567890abcd\",\n      \"source_event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"name\": \"My Event-1\",\n      \"description\": \"This is a test event.\",\n      \"attendance_type\": \"virtual\",\n      \"event_type\": \"CONFERENCE\",\n      \"status\": \"DRAFT\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
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
                      "description": "The name of the event. zoom_events.event_duplicated."
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
                          "description": "The account ID of the host who duplicated the event."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom Event description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "name",
                            "event_type",
                            "start_time",
                            "end_time",
                            "timezone"
                          ],
                          "properties": {
                            "hub_id": {
                              "type": "string",
                              "description": "The ID of the event hub.",
                              "example": "h134kj2h34kljgh23lkhj3"
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The ID of the event.",
                              "example": "234kj2h34kljgh23lkhj3"
                            },
                            "source_event_id": {
                              "type": "string",
                              "description": "The ID of the event from which this event was duplicated from.",
                              "example": "434kj2h34kljgh23lkhj3"
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the event.",
                              "example": "OpenAPI Conference Name"
                            },
                            "description": {
                              "type": "string",
                              "description": "The description of the event.",
                              "example": "This event was created with the OpenAPI"
                            },
                            "event_type": {
                              "type": "string",
                              "description": "Event Types: \n * CONFERENCE - Multi session event \n * SIMPLE_EVENT - Single session event \n * RECURRING - Recurring session event",
                              "enum": [
                                "CONFERENCE",
                                "SIMPLE_EVENT",
                                "RECURRING"
                              ],
                              "example": "CONFERENCE"
                            },
                            "status": {
                              "type": "string",
                              "description": "The event status: \n * PUBLISHED - Published event \n * DRAFT - Draft event \n * CANCELLED - Cancelled event",
                              "enum": [
                                "PUBLISHED",
                                "DRAFT",
                                "CANCELLED"
                              ],
                              "example": "DRAFT"
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "timezone": {
                              "type": "string",
                              "description": "The timezone of the event.",
                              "example": "America/Los_Angeles"
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
                    "value": "{\n  \"event\": \"zoom_events.event_duplicated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"hub_id\": \"1234567890abcd\",\n      \"event_id\": \"1234567890abcd\",\n      \"source_event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"name\": \"My Event-1\",\n      \"description\": \"This is a test event.\",\n      \"attendance_type\": \"virtual\",\n      \"event_type\": \"CONFERENCE\",\n      \"status\": \"DRAFT\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.session_ended": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.session_ended",
        "requestBody": {
          "description": "# zoom_events.session_ended\nThe **End Session** event is triggered every time a user in your account or your app ends a zoom-events session.\n\nZoom users with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing zoom-events event sessions. A Zoom Events host can start and end a Zoom Events session.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **End Session** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_sessions:read:admin`,`zoom_events_sessions:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:session`,`zoom_events:read:session:admin`\n\n**Event type**: `zoom_events.session_ended`\n",
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
                        "description": "The account ID of the user who ended the session."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the session.",
                        "required": [
                          "event_id",
                          "host_id",
                          "session_id",
                          "name",
                          "type",
                          "start_time",
                          "end_time",
                          "timezone"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The Zoom Events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "session_id": {
                            "type": "string",
                            "description": "The session ID."
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the session."
                          },
                          "type": {
                            "type": "integer",
                            "description": "The session type. \n* 0 \u2014 Meeting.\n* 2 \u2014 Webinar",
                            "enum": [
                              0,
                              2
                            ]
                          },
                          "start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The session's start time in UTC."
                          },
                          "end_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The date and time when the session ended in UTC."
                          },
                          "timezone": {
                            "type": "string",
                            "description": "The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones)."
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
                  "value": "{\n    \"event\": \"zoom_events.session_ended\",\n    \"event_ts\": 1626230691572,\n    \"payload\": {\n        \"account_id\": \"AAAAAABBBB\",\n        \"object\": {\n            \"event_id\": \"1234567890abcd\",\n            \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n            \"session_id\": \"1234567890abcd\",\n            \"name\": \"My Session-1\",\n            \"type\": 2,\n            \"start_time\": \"2023-07-13T21:44:51Z\",\n            \"timezone\": \"America/Los_Angeles\",\n            \"end_time\": \"2023-07-13T23:00:51Z\"\n        }\n    }\n}"
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
                          "description": "The account ID of the user who ended the session."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the session.",
                          "required": [
                            "event_id",
                            "host_id",
                            "session_id",
                            "name",
                            "type",
                            "start_time",
                            "end_time",
                            "timezone"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The Zoom Events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "session_id": {
                              "type": "string",
                              "description": "The session ID."
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the session."
                            },
                            "type": {
                              "type": "integer",
                              "description": "The session type. \n* 0 \u2014 Meeting.\n* 2 \u2014 Webinar",
                              "enum": [
                                0,
                                2
                              ]
                            },
                            "start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The session's start time in UTC."
                            },
                            "end_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The date and time when the session ended in UTC."
                            },
                            "timezone": {
                              "type": "string",
                              "description": "The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones)."
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
                    "value": "{\n    \"event\": \"zoom_events.session_ended\",\n    \"event_ts\": 1626230691572,\n    \"payload\": {\n        \"account_id\": \"AAAAAABBBB\",\n        \"object\": {\n            \"event_id\": \"1234567890abcd\",\n            \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n            \"session_id\": \"1234567890abcd\",\n            \"name\": \"My Session-1\",\n            \"type\": 2,\n            \"start_time\": \"2023-07-13T21:44:51Z\",\n            \"timezone\": \"America/Los_Angeles\",\n            \"end_time\": \"2023-07-13T23:00:51Z\"\n        }\n    }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.vod_registration_created": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.vod_registration_created",
        "requestBody": {
          "description": "# zoom_events.vod_registration_created\nThe **VOD Registration Created**  event  is triggered when someone registers for a video on-demand(VOD) channel  created by one of your app users or account users.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events.  The host can create an on-demand video channel, set up how their attendees will access their channels with registration.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**\n    * The **VOD Registration Created** subscription enabled under the **Zoom Events** event\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_vod_registration:read`,`zoom_events_vod_registration:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:vod_registrations`,`zoom_events:read:vod_registrations:admin`\n\n**Event type**: `zoom_events.vod_registration_created`\n",
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
                    "description": "The name of the event, zoom_events.vod_registration_created."
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
                        "description": "The account ID of the host who created the event."
                      },
                      "object": {
                        "type": "object",
                        "description": "The VOD channel registration description.",
                        "required": [
                          "channel_id",
                          "host_id",
                          "hub_id",
                          "registration_id",
                          "email"
                        ],
                        "properties": {
                          "channel_id": {
                            "type": "string",
                            "description": "The VOD channel ID."
                          },
                          "hub_id": {
                            "type": "string",
                            "description": "The Zoom Events hub ID",
                            "example": "234kj2h34kljgh23lkhj3"
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The channel host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "registration_id": {
                            "type": "string",
                            "description": "The registration ID generated from the registration."
                          },
                          "email": {
                            "type": "string",
                            "description": "The registrant's email."
                          },
                          "registration_date": {
                            "type": "string",
                            "description": "The registrantion date in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ.",
                            "format": "date-time",
                            "example": "2025-05-25T22:30:00Z"
                          },
                          "authentication_method": {
                            "type": "string",
                            "description": "The VOD channel authentication method \n * zoom_account Zoom account holder   * no_auth No authentication* email_opt registration using email OTP verification* ",
                            "example": "no_auth",
                            "enum": [
                              "zoom_account",
                              "email_opt",
                              "no_auth"
                            ]
                          },
                          "join_link": {
                            "type": "string",
                            "description": "The unique join link for the vod registrant.",
                            "example": "https://events.zoom.us/ejl/xyz"
                          },
                          "first_name": {
                            "type": "string",
                            "description": "The registrant's first name.",
                            "example": "Jill"
                          },
                          "last_name": {
                            "type": "string",
                            "description": "The registrant's last name.",
                            "example": "Chill"
                          },
                          "address": {
                            "type": "string",
                            "description": "The registrant's address.",
                            "example": "1800 Amphibious Blvd"
                          },
                          "city": {
                            "type": "string",
                            "description": "The registrant's city.",
                            "example": "Mountain View"
                          },
                          "state": {
                            "type": "string",
                            "description": "The registrant's state or province.",
                            "example": "CA"
                          },
                          "zip": {
                            "type": "string",
                            "description": "The registrant's ZIP or postal code.",
                            "example": "94045"
                          },
                          "country": {
                            "type": "string",
                            "description": "The registrant's two-letter [country code](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#countries).",
                            "example": "US"
                          },
                          "phone": {
                            "type": "string",
                            "description": "The registrant's phone number.",
                            "example": "5550100"
                          },
                          "industry": {
                            "type": "string",
                            "description": "The registrant's industry.",
                            "example": "Food"
                          },
                          "job_title": {
                            "type": "string",
                            "description": "The registrant's job title.",
                            "example": "Chef"
                          },
                          "organization": {
                            "type": "string",
                            "description": "The registrant's organization.",
                            "example": "ABC Ltd."
                          },
                          "organization_size": {
                            "type": "string",
                            "description": "The registrant's number of employees.",
                            "example": "150"
                          },
                          "comments": {
                            "type": "string",
                            "description": "The registrant's questions and comments.",
                            "example": "Looking forward to the discussion."
                          },
                          "custom_questions": {
                            "maxItems": 100,
                            "type": "array",
                            "description": "The information about custom questions.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "title": {
                                  "type": "string",
                                  "description": "The title of the custom question.",
                                  "example": "What do you hope to learn from this?"
                                },
                                "answer": {
                                  "type": "string",
                                  "description": "The custom question's response value.",
                                  "example": "Look forward to learning how you come up with new recipes and what other services you offer."
                                }
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
                  "value": "{\n  \"event\": \"zoom_events.vod_registration_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"djsjgkjhfhk\",\n      \"hub_id\": \"huCbxjabcdEFGHfp8uQ\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"registration_id\": \"iso9Dllddp39dagjLj9j\",\n      \"email\": \"email@zoom.us\",\n      \"registration_date\": \"2025-05-24T22:30:00Z\",\n      \"authentication_method\": \"no_auth\",\n      \"join_link\": \"https://events.zoom.us/ejl/xyz\",\n      \"first_name\": \"Jill\",\n      \"last_name\": \"Chill\",\n      \"address\": \"1800 Amphibious Blvd\",\n      \"city\": \"Mountain View\",\n      \"state\": \"CA\",\n      \"zip\": \"94045\",\n      \"country\": \"US\",\n      \"phone\": \"5550100\",\n      \"industry\": \"Food\",\n      \"job_title\": \"Chef\",\n      \"organization\": \"ABC Ltd.\",\n      \"organization_size\": \"150\",\n      \"comments\": \"Looking forward to the discussion.\",\n      \"custom_questions\": [\n        {\n          \"title\": \"What do you hope to learn from this?\",\n          \"answer\": \"Look forward to learning how you come up with new recipes and what other services you offer.\"\n        }\n      ]\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.vod_registration_created."
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
                          "description": "The account ID of the host who created the event."
                        },
                        "object": {
                          "type": "object",
                          "description": "The VOD channel registration description.",
                          "required": [
                            "channel_id",
                            "host_id",
                            "hub_id",
                            "registration_id",
                            "email"
                          ],
                          "properties": {
                            "channel_id": {
                              "type": "string",
                              "description": "The VOD channel ID."
                            },
                            "hub_id": {
                              "type": "string",
                              "description": "The Zoom Events hub ID",
                              "example": "234kj2h34kljgh23lkhj3"
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The channel host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "registration_id": {
                              "type": "string",
                              "description": "The registration ID generated from the registration."
                            },
                            "email": {
                              "type": "string",
                              "description": "The registrant's email."
                            },
                            "registration_date": {
                              "type": "string",
                              "description": "The registrantion date in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ.",
                              "format": "date-time",
                              "example": "2025-05-25T22:30:00Z"
                            },
                            "authentication_method": {
                              "type": "string",
                              "description": "The VOD channel authentication method \n * zoom_account Zoom account holder   * no_auth No authentication* email_opt registration using email OTP verification* ",
                              "example": "no_auth",
                              "enum": [
                                "zoom_account",
                                "email_opt",
                                "no_auth"
                              ]
                            },
                            "join_link": {
                              "type": "string",
                              "description": "The unique join link for the vod registrant.",
                              "example": "https://events.zoom.us/ejl/xyz"
                            },
                            "first_name": {
                              "type": "string",
                              "description": "The registrant's first name.",
                              "example": "Jill"
                            },
                            "last_name": {
                              "type": "string",
                              "description": "The registrant's last name.",
                              "example": "Chill"
                            },
                            "address": {
                              "type": "string",
                              "description": "The registrant's address.",
                              "example": "1800 Amphibious Blvd"
                            },
                            "city": {
                              "type": "string",
                              "description": "The registrant's city.",
                              "example": "Mountain View"
                            },
                            "state": {
                              "type": "string",
                              "description": "The registrant's state or province.",
                              "example": "CA"
                            },
                            "zip": {
                              "type": "string",
                              "description": "The registrant's ZIP or postal code.",
                              "example": "94045"
                            },
                            "country": {
                              "type": "string",
                              "description": "The registrant's two-letter [country code](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#countries).",
                              "example": "US"
                            },
                            "phone": {
                              "type": "string",
                              "description": "The registrant's phone number.",
                              "example": "5550100"
                            },
                            "industry": {
                              "type": "string",
                              "description": "The registrant's industry.",
                              "example": "Food"
                            },
                            "job_title": {
                              "type": "string",
                              "description": "The registrant's job title.",
                              "example": "Chef"
                            },
                            "organization": {
                              "type": "string",
                              "description": "The registrant's organization.",
                              "example": "ABC Ltd."
                            },
                            "organization_size": {
                              "type": "string",
                              "description": "The registrant's number of employees.",
                              "example": "150"
                            },
                            "comments": {
                              "type": "string",
                              "description": "The registrant's questions and comments.",
                              "example": "Looking forward to the discussion."
                            },
                            "custom_questions": {
                              "maxItems": 100,
                              "type": "array",
                              "description": "The information about custom questions.",
                              "items": {
                                "type": "object",
                                "properties": {
                                  "title": {
                                    "type": "string",
                                    "description": "The title of the custom question.",
                                    "example": "What do you hope to learn from this?"
                                  },
                                  "answer": {
                                    "type": "string",
                                    "description": "The custom question's response value.",
                                    "example": "Look forward to learning how you come up with new recipes and what other services you offer."
                                  }
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
                    "value": "{\n  \"event\": \"zoom_events.vod_registration_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"channel_id\": \"djsjgkjhfhk\",\n      \"hub_id\": \"huCbxjabcdEFGHfp8uQ\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"registration_id\": \"iso9Dllddp39dagjLj9j\",\n      \"email\": \"email@zoom.us\",\n      \"registration_date\": \"2025-05-24T22:30:00Z\",\n      \"authentication_method\": \"no_auth\",\n      \"join_link\": \"https://events.zoom.us/ejl/xyz\",\n      \"first_name\": \"Jill\",\n      \"last_name\": \"Chill\",\n      \"address\": \"1800 Amphibious Blvd\",\n      \"city\": \"Mountain View\",\n      \"state\": \"CA\",\n      \"zip\": \"94045\",\n      \"country\": \"US\",\n      \"phone\": \"5550100\",\n      \"industry\": \"Food\",\n      \"job_title\": \"Chef\",\n      \"organization\": \"ABC Ltd.\",\n      \"organization_size\": \"150\",\n      \"comments\": \"Looking forward to the discussion.\",\n      \"custom_questions\": [\n        {\n          \"title\": \"What do you hope to learn from this?\",\n          \"answer\": \"Look forward to learning how you come up with new recipes and what other services you offer.\"\n        }\n      ]\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.speaker_updated": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.speaker_updated",
        "requestBody": {
          "description": "# zoom_events.speaker_updated\nThe **Speaker Updated**  event  is triggered every time a user in your account or your app updates speaker for an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can update a speaker for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Speaker Updated** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_speakers:write:admin`,`zoom_events_speakers:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:speaker`,`zoom_events:read:speaker:admin`\n\n**Event type**: `zoom_events.speaker_updated`\n",
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
                    "description": "The name of the event, zoom_events.speaker_updated"
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
                        "description": "The account ID of the host who updated the event speaker."
                      },
                      "object": {
                        "type": "object",
                        "description": "Speaker Description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "speaker_id",
                          "name"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The zoom-events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "speaker_id": {
                            "type": "string",
                            "description": "The speaker id."
                          },
                          "name": {
                            "type": "string",
                            "description": "The speaker's name."
                          },
                          "email": {
                            "type": "string",
                            "description": "The speaker's email."
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
                  "value": "{\n  \"event\": \"zoom_events.speaker_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"speaker_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"speaker1\",\n      \"email\": \"speaker1-email@zoom.us\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.speaker_updated"
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
                          "description": "The account ID of the host who updated the event speaker."
                        },
                        "object": {
                          "type": "object",
                          "description": "Speaker Description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "speaker_id",
                            "name"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The zoom-events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "speaker_id": {
                              "type": "string",
                              "description": "The speaker id."
                            },
                            "name": {
                              "type": "string",
                              "description": "The speaker's name."
                            },
                            "email": {
                              "type": "string",
                              "description": "The speaker's email."
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
                    "value": "{\n  \"event\": \"zoom_events.speaker_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"speaker_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"speaker1\",\n      \"email\": \"speaker1-email@zoom.us\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.ticket_deleted": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.ticket_deleted",
        "requestBody": {
          "description": "# zoom_events.ticket_deleted\nThe  **Ticket Deleted**  event is triggered when one of your app users or account users cancels the event registration.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events.  The host can delete a pre-registration for an event or a registrant can cancel their ticket.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Ticket Deleted** subscription enabled under the **Zoom Events** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_tickets:write:admin`,`zoom_events_tickets:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:ticket`,`zoom_events:read:ticket:admin`\n\n**Event type**: `zoom_events.ticket_deleted`\n",
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
                    "description": "The name of the event. zoom_events.ticket_deleted."
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
                        "description": "The account ID of the host who created the event."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom Event description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "ticket_id",
                          "email"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The Zoom Events ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "ticket_id": {
                            "type": "string",
                            "description": "The ticket ID generated from the registration."
                          },
                          "ticket_type_id": {
                            "type": "string",
                            "description": "The ticket type ID for which the user registers to."
                          },
                          "external_ticket_id": {
                            "type": "string",
                            "description": "A set of unique alphanumeric characters that references the external ticket ID."
                          },
                          "email": {
                            "type": "string",
                            "description": "The registrant's email."
                          },
                          "ticket_role_type": {
                            "type": "string",
                            "description": "The ticket role type: \n * normal The normal ticket \n * alternative_host The alternative host ticket \n * panelist The panelist ticket \n * bind_meeting The bind meeting ticket \n * speaker The speaker ticket \n * interpreter The interpreter ticket \n * sponsor The sponsor ticket \n * host The host ticket \n * moderator The moderator ticket \n * expo_booth_owner The expo booth owner ticket \n * attendee_without_order The attendee without order ticket \n * pre_registration \u2014 The pre-registration ticket \n * company_zoom_room The company zoom room ticket \n * moderator_allhost The moderator all host ticket \n * hub_host The hub host ticket \n * hub_owner The hub owner ticket \n * subscription The subscription ticket",
                            "example": "normal",
                            "enum": [
                              "normal",
                              "alternative_host",
                              "panelist",
                              "bind_meeting",
                              "speaker",
                              "subscription",
                              "sponsor",
                              "interpreter",
                              "host",
                              "moderator",
                              "expo_booth_owner",
                              "attendee_without_order",
                              "pre_registration",
                              "company_zoom_room",
                              "moderator_allhost",
                              "hub_host",
                              "hub_owner"
                            ]
                          },
                          "authentication_method": {
                            "type": "string",
                            "description": "The event authentication method for the ticket: \n * zoom_account Zoom account holder  * zoom_account_otp Zoom account holder with OTP  * corporate_idp Corporate IDP  * no_auth No authentication",
                            "enum": [
                              "zoom_account",
                              "zoom_account_otp",
                              "corporate_idp",
                              "no_auth"
                            ],
                            "example": "no_auth"
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
                  "value": "{\n  \"event\": \"zoom_events.ticket_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_id\": \"iso9Dllddp39dagjLj9j\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"external_ticket_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"email\": \"email@zoom.us\",\n      \"ticket_role_type\": \"normal\",\n      \"authentication_method\": \"no_auth\"\n    }\n  }\n}"
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
                      "description": "The name of the event. zoom_events.ticket_deleted."
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
                          "description": "The account ID of the host who created the event."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom Event description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "ticket_id",
                            "email"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The Zoom Events ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "ticket_id": {
                              "type": "string",
                              "description": "The ticket ID generated from the registration."
                            },
                            "ticket_type_id": {
                              "type": "string",
                              "description": "The ticket type ID for which the user registers to."
                            },
                            "external_ticket_id": {
                              "type": "string",
                              "description": "A set of unique alphanumeric characters that references the external ticket ID."
                            },
                            "email": {
                              "type": "string",
                              "description": "The registrant's email."
                            },
                            "ticket_role_type": {
                              "type": "string",
                              "description": "The ticket role type: \n * normal The normal ticket \n * alternative_host The alternative host ticket \n * panelist The panelist ticket \n * bind_meeting The bind meeting ticket \n * speaker The speaker ticket \n * interpreter The interpreter ticket \n * sponsor The sponsor ticket \n * host The host ticket \n * moderator The moderator ticket \n * expo_booth_owner The expo booth owner ticket \n * attendee_without_order The attendee without order ticket \n * pre_registration \u2014 The pre-registration ticket \n * company_zoom_room The company zoom room ticket \n * moderator_allhost The moderator all host ticket \n * hub_host The hub host ticket \n * hub_owner The hub owner ticket \n * subscription The subscription ticket",
                              "example": "normal",
                              "enum": [
                                "normal",
                                "alternative_host",
                                "panelist",
                                "bind_meeting",
                                "speaker",
                                "subscription",
                                "sponsor",
                                "interpreter",
                                "host",
                                "moderator",
                                "expo_booth_owner",
                                "attendee_without_order",
                                "pre_registration",
                                "company_zoom_room",
                                "moderator_allhost",
                                "hub_host",
                                "hub_owner"
                              ]
                            },
                            "authentication_method": {
                              "type": "string",
                              "description": "The event authentication method for the ticket: \n * zoom_account Zoom account holder  * zoom_account_otp Zoom account holder with OTP  * corporate_idp Corporate IDP  * no_auth No authentication",
                              "enum": [
                                "zoom_account",
                                "zoom_account_otp",
                                "corporate_idp",
                                "no_auth"
                              ],
                              "example": "no_auth"
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
                    "value": "{\n  \"event\": \"zoom_events.ticket_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_id\": \"iso9Dllddp39dagjLj9j\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"external_ticket_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"email\": \"email@zoom.us\",\n      \"ticket_role_type\": \"normal\",\n      \"authentication_method\": \"no_auth\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.speaker_deleted": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.speaker_deleted",
        "requestBody": {
          "description": "# zoom_events.speaker_deleted\nThe **Speaker Deleted**  event  is triggered every time a user in your account or your app deletes speaker for an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can delete a speaker for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Speaker Deleted** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_speakers:write:admin`,`zoom_events_speakers:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:speaker`,`zoom_events:read:speaker:admin`\n\n**Event type**: `zoom_events.speaker_deleted`\n",
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
                    "description": "The name of the event, zoom_events.speaker_deleted"
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
                        "description": "The account ID of the host who deleted the event speaker."
                      },
                      "object": {
                        "type": "object",
                        "description": "Speaker Description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "speaker_id",
                          "name"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The zoom-events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "speaker_id": {
                            "type": "string",
                            "description": "The speaker ID."
                          },
                          "name": {
                            "type": "string",
                            "description": "The speaker's name."
                          },
                          "email": {
                            "type": "string",
                            "description": "The speaker's email."
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
                  "value": "{\n  \"event\": \"zoom_events.speaker_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"speaker_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"speaker1\",\n      \"email\": \"speaker1-email@zoom.us\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.speaker_deleted"
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
                          "description": "The account ID of the host who deleted the event speaker."
                        },
                        "object": {
                          "type": "object",
                          "description": "Speaker Description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "speaker_id",
                            "name"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The zoom-events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "speaker_id": {
                              "type": "string",
                              "description": "The speaker ID."
                            },
                            "name": {
                              "type": "string",
                              "description": "The speaker's name."
                            },
                            "email": {
                              "type": "string",
                              "description": "The speaker's email."
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
                    "value": "{\n  \"event\": \"zoom_events.speaker_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"speaker_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"speaker1\",\n      \"email\": \"speaker1-email@zoom.us\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.ticket_type_deleted": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.ticket_type_deleted",
        "requestBody": {
          "description": "# zoom_events.ticket_type_deleted\nThe **TicketType Deleted**  event  is triggered every time a user in your account or your app deletes ticketType for an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can delete a ticketType for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **TicketType Deleted** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_ticket_types:write:admin`,`zoom_events_ticket_types:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:list_ticket_types`,`zoom_events:read:list_ticket_types:admin`\n\n**Event type**: `zoom_events.ticket_type_deleted`\n",
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
                    "description": "The name of the event, zoom_events.ticket_type_deleted"
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
                        "description": "The account ID of the host who deleted the ticketType."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the TicketType.",
                        "required": [
                          "event_id",
                          "host_id",
                          "ticket_type_id",
                          "name",
                          "start_time",
                          "end_time"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The zoom-events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "ticket_type_id": {
                            "type": "string",
                            "description": "The ticketType ID for which the user registers to."
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the ticketType.",
                            "example": "General Admission Ticket"
                          },
                          "description": {
                            "type": "string",
                            "description": "The description of the ticketType.",
                            "example": "This is a General Admission ticket"
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
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
                  "value": "{\n  \"event\": \"zoom_events.ticket_type_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"name\": \"General Admission Ticket\",\n      \"description\": \"This is for General Admission\",\n      \"attendance_type\": \"virtual\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.ticket_type_deleted"
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
                          "description": "The account ID of the host who deleted the ticketType."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the TicketType.",
                          "required": [
                            "event_id",
                            "host_id",
                            "ticket_type_id",
                            "name",
                            "start_time",
                            "end_time"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The zoom-events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "ticket_type_id": {
                              "type": "string",
                              "description": "The ticketType ID for which the user registers to."
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the ticketType.",
                              "example": "General Admission Ticket"
                            },
                            "description": {
                              "type": "string",
                              "description": "The description of the ticketType.",
                              "example": "This is a General Admission ticket"
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
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
                    "value": "{\n  \"event\": \"zoom_events.ticket_type_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"name\": \"General Admission Ticket\",\n      \"description\": \"This is for General Admission\",\n      \"attendance_type\": \"virtual\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.session_updated": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.session_updated",
        "requestBody": {
          "description": "# zoom_events.session_updated\n\nThe **Session Updated** event is triggered every time a user in your account or your app, updates a session in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host or a co-editor can update the session.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Session Updated** subscription enabled under the **Zoom Events** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_sessions:write:admin`,`zoom_events_sessions:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:session`,`zoom_events:read:session:admin`\n\n**Event type**: `zoom_events.session_updated`\n",
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
                    "description": "The name of the event. zoom_events.event.session_updated."
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
                        "description": "The account ID of the event host who updated the session post event publishing."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the session.",
                        "required": [
                          "event_id",
                          "host_id",
                          "session_id",
                          "name",
                          "type",
                          "start_time",
                          "end_time",
                          "timezone"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The Zoom Events ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "session_id": {
                            "type": "string",
                            "description": "The session ID.",
                            "example": "4uzfv3JwTeyR5QpC3PXwMg"
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the session.",
                            "example": "Session 1"
                          },
                          "type": {
                            "type": "integer",
                            "description": "The session type. \n * 0 \u2014 Meeting \n * 2 \u2014 Webinar",
                            "enum": [
                              0,
                              2
                            ],
                            "example": "2"
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-05-31T13:00:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-05-31T13:00:00Z"
                          },
                          "timezone": {
                            "type": "string",
                            "description": "The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones).",
                            "example": "America/Los_Angeles"
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
                  "value": "{\n  \"event\": \"zoom_events.session_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"session_id\": \"fhhfkdhkhdgkhg\",\n      \"name\": \"my session\",\n      \"type\": 2,\n      \"attendance_type\": \"virtual\",\n      \"start_time\": \"2022-05-31T13:00:00Z\",\n      \"end_time\": \"2022-05-31T13:00:00Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
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
                      "description": "The name of the event. zoom_events.event.session_updated."
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
                          "description": "The account ID of the event host who updated the session post event publishing."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the session.",
                          "required": [
                            "event_id",
                            "host_id",
                            "session_id",
                            "name",
                            "type",
                            "start_time",
                            "end_time",
                            "timezone"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The Zoom Events ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "session_id": {
                              "type": "string",
                              "description": "The session ID.",
                              "example": "4uzfv3JwTeyR5QpC3PXwMg"
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the session.",
                              "example": "Session 1"
                            },
                            "type": {
                              "type": "integer",
                              "description": "The session type. \n * 0 \u2014 Meeting \n * 2 \u2014 Webinar",
                              "enum": [
                                0,
                                2
                              ],
                              "example": "2"
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-05-31T13:00:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-05-31T13:00:00Z"
                            },
                            "timezone": {
                              "type": "string",
                              "description": "The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones).",
                              "example": "America/Los_Angeles"
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
                    "value": "{\n  \"event\": \"zoom_events.session_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"session_id\": \"fhhfkdhkhdgkhg\",\n      \"name\": \"my session\",\n      \"type\": 2,\n      \"attendance_type\": \"virtual\",\n      \"start_time\": \"2022-05-31T13:00:00Z\",\n      \"end_time\": \"2022-05-31T13:00:00Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.session_created": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.session_created",
        "requestBody": {
          "description": "# zoom_events.session_created\nThe **Session Created** event is triggered every time a user in your account or your app, creates a session in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host or a co-editor can add a session.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Session Created** subscription enabled under the **Zoom Events** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_sessions:write:admin`,`zoom_events_sessions:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:session`,`zoom_events:read:session:admin`\n\n**Event type**: `zoom_events.session_created`\n",
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
                    "description": "The name of the event. zoom_events.event.session_created."
                  },
                  "event_ts": {
                    "type": "integer",
                    "format": "int64",
                    "description": "A timestamp at when the event occurred."
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
                        "description": "The account ID of the host who created the session post event publishing."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the session.",
                        "required": [
                          "event_id",
                          "host_id",
                          "session_id",
                          "name",
                          "type",
                          "start_time",
                          "end_time",
                          "timezone"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The Zoom Events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "session_id": {
                            "type": "string",
                            "description": "The session ID.",
                            "example": "4uzfv3JwTeyR5QpC3PXwMg"
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the session.",
                            "example": "Session 1"
                          },
                          "type": {
                            "type": "integer",
                            "description": "The session type. \n * 0 \u2014 Meeting \n * 2 \u2014 Webinar",
                            "enum": [
                              0,
                              2
                            ]
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-05-31T13:00:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-05-31T13:00:00Z"
                          },
                          "timezone": {
                            "type": "string",
                            "description": "The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones)."
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
                  "value": "{\n  \"event\": \"zoom_events.session_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"session_id\": \"fhhfkdhkhdgkhg\",\n      \"name\": \"my session\",\n      \"attendance_type\": \"virtual\",\n      \"type\": 2,\n      \"start_time\": \"2022-05-31T13:00:00Z\",\n      \"end_time\": \"2022-05-31T13:00:00Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
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
                      "description": "The name of the event. zoom_events.event.session_created."
                    },
                    "event_ts": {
                      "type": "integer",
                      "format": "int64",
                      "description": "A timestamp at when the event occurred."
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
                          "description": "The account ID of the host who created the session post event publishing."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the session.",
                          "required": [
                            "event_id",
                            "host_id",
                            "session_id",
                            "name",
                            "type",
                            "start_time",
                            "end_time",
                            "timezone"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The Zoom Events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "session_id": {
                              "type": "string",
                              "description": "The session ID.",
                              "example": "4uzfv3JwTeyR5QpC3PXwMg"
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the session.",
                              "example": "Session 1"
                            },
                            "type": {
                              "type": "integer",
                              "description": "The session type. \n * 0 \u2014 Meeting \n * 2 \u2014 Webinar",
                              "enum": [
                                0,
                                2
                              ]
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-05-31T13:00:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-05-31T13:00:00Z"
                            },
                            "timezone": {
                              "type": "string",
                              "description": "The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones)."
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
                    "value": "{\n  \"event\": \"zoom_events.session_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"session_id\": \"fhhfkdhkhdgkhg\",\n      \"name\": \"my session\",\n      \"attendance_type\": \"virtual\",\n      \"type\": 2,\n      \"start_time\": \"2022-05-31T13:00:00Z\",\n      \"end_time\": \"2022-05-31T13:00:00Z\",\n      \"timezone\": \"America/Los_Angeles\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.ticket_type_created": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.ticket_type_created",
        "requestBody": {
          "description": "# zoom_events.ticket_type_created\nThe **TicketType Created**  event  is triggered every time a user in your account or your app creates ticketType for an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can create a ticketType for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **TicketType Created** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_ticket_types:write:admin`,`zoom_events_ticket_types:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:list_ticket_types`,`zoom_events:read:list_ticket_types:admin`\n\n**Event type**: `zoom_events.ticket_type_created`\n",
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
                    "description": "The name of the event, zoom_events.ticket_type_created"
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
                        "description": "The account ID of the host who created the ticketType."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the ticketType.",
                        "required": [
                          "event_id",
                          "host_id",
                          "ticket_type_id",
                          "name",
                          "start_time",
                          "end_time"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The zoom-events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "ticket_type_id": {
                            "type": "string",
                            "description": "The ticketType ID for which the user registers to."
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the ticketType.",
                            "example": "General Admission Ticket"
                          },
                          "description": {
                            "type": "string",
                            "description": "The description of the ticketType.",
                            "example": "This is a General Admission ticket"
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
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
                  "value": "{\n  \"event\": \"zoom_events.ticket_type_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"name\": \"General Admission Ticket\",\n      \"description\": \"This is for General Admission\",\n      \"attendance_type\": \"virtual\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.ticket_type_created"
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
                          "description": "The account ID of the host who created the ticketType."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the ticketType.",
                          "required": [
                            "event_id",
                            "host_id",
                            "ticket_type_id",
                            "name",
                            "start_time",
                            "end_time"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The zoom-events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "ticket_type_id": {
                              "type": "string",
                              "description": "The ticketType ID for which the user registers to."
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the ticketType.",
                              "example": "General Admission Ticket"
                            },
                            "description": {
                              "type": "string",
                              "description": "The description of the ticketType.",
                              "example": "This is a General Admission ticket"
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
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
                    "value": "{\n  \"event\": \"zoom_events.ticket_type_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"name\": \"General Admission Ticket\",\n      \"description\": \"This is for General Admission\",\n      \"attendance_type\": \"virtual\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"end_time\": \"2023-07-13T23:00:51Z\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.event_updated": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.event_updated",
        "requestBody": {
          "description": "# zoom_events.event_updated\nThe **Event Updated** is triggered every time a user in your account or your app, makes changes to the event data in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host or a co-editor can update an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Event Updated** subscription enabled under the **Zoom Events** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_basic:write:admin`,`zoom_events_basic:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:event`,`zoom_events:read:event:admin`\n\n**Event type**: `zoom_events.event_updated`\n",
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
                    "description": "The name of the event. zoom_events.event_updated."
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
                        "description": "The account ID of the host who updated the event."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom Event description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "name",
                          "event_type",
                          "start_time",
                          "end_time",
                          "timezone"
                        ],
                        "properties": {
                          "hub_id": {
                            "type": "string",
                            "description": "The ID of the event hub.",
                            "example": "234kj2h34kljgh23lkhj3"
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The ID of the event.",
                            "example": "234kj2h34kljgh23lkhj3"
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the event.",
                            "example": "OpenAPI Conference Name"
                          },
                          "description": {
                            "type": "string",
                            "description": "The description of the event.",
                            "example": "This event was created with the OpenAPI"
                          },
                          "event_type": {
                            "type": "string",
                            "description": "Event Types: \n * CONFERENCE - Multi session event \n * SIMPLE_EVENT - Single session event \n * RECURRING - Recurring session event",
                            "enum": [
                              "CONFERENCE",
                              "SIMPLE_EVENT",
                              "RECURRING"
                            ],
                            "example": "CONFERENCE"
                          },
                          "status": {
                            "type": "string",
                            "description": "The event status: \n * PUBLISHED - Published event \n * DRAFT - Draft event \n * CANCELLED - Cancelled event",
                            "enum": [
                              "PUBLISHED",
                              "DRAFT",
                              "CANCELLED"
                            ],
                            "example": "PUBLISHED"
                          },
                          "attendance_type": {
                            "type": "string",
                            "description": "The type of attendee experience for the event.",
                            "enum": [
                              "virtual",
                              "in-person",
                              "hybrid"
                            ],
                            "example": "virtual"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                            "format": "date-time",
                            "example": "2022-06-03T20:51:00Z"
                          },
                          "timezone": {
                            "type": "string",
                            "description": "The timezone of the event.",
                            "example": "America/Los_Angeles"
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
                  "value": "{\n  \"event\": \"zoom_events.event_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"hub_id\": \"1234567890abcd\",\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"name\": \"My Event-1\",\n      \"description\": \"This is a test event.\",\n      \"attendance_type\": \"virtual\",\n      \"event_type\": \"CONFERENCE\",\n      \"status\": \"DRAFT\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"timezone\": \"America/Los_Angeles\",\n      \"end_time\": \"2023-07-13T23:00:51Z\"\n    }\n  }\n}"
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
                      "description": "The name of the event. zoom_events.event_updated."
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
                          "description": "The account ID of the host who updated the event."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom Event description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "name",
                            "event_type",
                            "start_time",
                            "end_time",
                            "timezone"
                          ],
                          "properties": {
                            "hub_id": {
                              "type": "string",
                              "description": "The ID of the event hub.",
                              "example": "234kj2h34kljgh23lkhj3"
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The ID of the event.",
                              "example": "234kj2h34kljgh23lkhj3"
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the event.",
                              "example": "OpenAPI Conference Name"
                            },
                            "description": {
                              "type": "string",
                              "description": "The description of the event.",
                              "example": "This event was created with the OpenAPI"
                            },
                            "event_type": {
                              "type": "string",
                              "description": "Event Types: \n * CONFERENCE - Multi session event \n * SIMPLE_EVENT - Single session event \n * RECURRING - Recurring session event",
                              "enum": [
                                "CONFERENCE",
                                "SIMPLE_EVENT",
                                "RECURRING"
                              ],
                              "example": "CONFERENCE"
                            },
                            "status": {
                              "type": "string",
                              "description": "The event status: \n * PUBLISHED - Published event \n * DRAFT - Draft event \n * CANCELLED - Cancelled event",
                              "enum": [
                                "PUBLISHED",
                                "DRAFT",
                                "CANCELLED"
                              ],
                              "example": "PUBLISHED"
                            },
                            "attendance_type": {
                              "type": "string",
                              "description": "The type of attendee experience for the event.",
                              "enum": [
                                "virtual",
                                "in-person",
                                "hybrid"
                              ],
                              "example": "virtual"
                            },
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "end_time": {
                              "type": "string",
                              "description": "The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ",
                              "format": "date-time",
                              "example": "2022-06-03T20:51:00Z"
                            },
                            "timezone": {
                              "type": "string",
                              "description": "The timezone of the event.",
                              "example": "America/Los_Angeles"
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
                    "value": "{\n  \"event\": \"zoom_events.event_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"hub_id\": \"1234567890abcd\",\n      \"event_id\": \"1234567890abcd\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"name\": \"My Event-1\",\n      \"description\": \"This is a test event.\",\n      \"attendance_type\": \"virtual\",\n      \"event_type\": \"CONFERENCE\",\n      \"status\": \"DRAFT\",\n      \"start_time\": \"2023-07-13T21:44:51Z\",\n      \"timezone\": \"America/Los_Angeles\",\n      \"end_time\": \"2023-07-13T23:00:51Z\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.speaker_created": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.speaker_created",
        "requestBody": {
          "description": "# zoom_events.speaker_created\nThe **Speaker Created**  event  is triggered every time a user in your account or your app creates speaker for an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can create a speaker for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Speaker Created** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_speakers:write:admin`,`zoom_events_speakers:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:speaker`,`zoom_events:read:speaker:admin`\n\n**Event type**: `zoom_events.speaker_created`\n",
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
                    "description": "The name of the event, zoom_events.speaker_created"
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
                        "description": "The account ID of the host who created the event speaker."
                      },
                      "object": {
                        "type": "object",
                        "description": "Speaker Description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "speaker_id",
                          "name"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The Zoom Events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "speaker_id": {
                            "type": "string",
                            "description": "The speaker ID."
                          },
                          "name": {
                            "type": "string",
                            "description": "The speaker's name."
                          },
                          "email": {
                            "type": "string",
                            "description": "The speaker's email."
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
                  "value": "{\n  \"event\": \"zoom_events.speaker_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"speaker_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"speaker1\",\n      \"email\": \"speaker1-email@zoom.us\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.speaker_created"
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
                          "description": "The account ID of the host who created the event speaker."
                        },
                        "object": {
                          "type": "object",
                          "description": "Speaker Description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "speaker_id",
                            "name"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The Zoom Events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "speaker_id": {
                              "type": "string",
                              "description": "The speaker ID."
                            },
                            "name": {
                              "type": "string",
                              "description": "The speaker's name."
                            },
                            "email": {
                              "type": "string",
                              "description": "The speaker's email."
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
                    "value": "{\n  \"event\": \"zoom_events.speaker_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"speaker_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"speaker1\",\n      \"email\": \"speaker1-email@zoom.us\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.ticket_created": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.ticket_created",
        "requestBody": {
          "description": "# zoom_events.ticket_created\n   The **Ticket Created**  event  is triggered when someone registers for an event created by one of your app users or account users.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events.  The host can create an event, set up a registration form, pre-register users or send out a registration email invitation. Users can also register themselves for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Ticket Created** subscription enabled under the **Zoom Events** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_tickets:write:admin`,`zoom_events_tickets:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:ticket`,`zoom_events:read:ticket:admin`\n\n**Event type**: `zoom_events.ticket_created`\n",
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
                    "description": "The name of the event. zoom_events.ticket_created."
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
                        "description": "The account ID of the host who created the event."
                      },
                      "object": {
                        "type": "object",
                        "description": "The Zoom Event description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "ticket_id",
                          "email"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The Zoom Events ID."
                          },
                          "ticket_id": {
                            "type": "string",
                            "description": "The ticket ID generated from the registration."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "ticket_type_id": {
                            "type": "string",
                            "description": "The ticket type ID for which the user registers to."
                          },
                          "external_ticket_id": {
                            "type": "string",
                            "description": "A set of unique alphanumeric characters that references the external ticket ID."
                          },
                          "email": {
                            "type": "string",
                            "description": "The registrant's email."
                          },
                          "ticket_role_type": {
                            "type": "string",
                            "description": "The ticket role type: \n * normal The normal ticket \n * alternative_host The alternative host ticket \n * panelist The panelist ticket \n * bind_meeting The bind meeting ticket \n * speaker The speaker ticket \n * interpreter The interpreter ticket \n * sponsor The sponsor ticket \n * host The host ticket \n * moderator The moderator ticket \n * expo_booth_owner The expo booth owner ticket \n * attendee_without_order The attendee without order ticket \n * pre_registration \u2014 The pre-registration ticket \n * company_zoom_room The company zoom room ticket \n * moderator_allhost The moderator all host ticket \n * hub_host The hub host ticket \n * hub_owner The hub owner ticket \n * subscription The subscription ticket",
                            "example": "normal",
                            "enum": [
                              "normal",
                              "alternative_host",
                              "panelist",
                              "bind_meeting",
                              "speaker",
                              "subscription",
                              "sponsor",
                              "interpreter",
                              "host",
                              "moderator",
                              "expo_booth_owner",
                              "attendee_without_order",
                              "pre_registration",
                              "company_zoom_room",
                              "moderator_allhost",
                              "hub_host",
                              "hub_owner"
                            ]
                          },
                          "authentication_method": {
                            "type": "string",
                            "description": "The event authentication method for the ticket: \n * zoom_account Zoom account holder  * zoom_account_otp Zoom account holder with OTP  * corporate_idp Corporate IDP  * no_auth No authentication",
                            "enum": [
                              "zoom_account",
                              "zoom_account_otp",
                              "corporate_idp",
                              "no_auth"
                            ],
                            "example": "no_auth"
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
                  "value": "{\n  \"event\": \"zoom_events.ticket_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"djsjgkjhfhk\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_id\": \"iso9Dllddp39dagjLj9j\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"external_ticket_id\": \"abc1231245-adf\",\n      \"ticket_role_type\": \"normal\",\n      \"email\": \"email@zoom.us\",\n      \"authentication_method\": \"no_auth\"\n    }\n  }\n}"
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
                      "description": "The name of the event. zoom_events.ticket_created."
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
                          "description": "The account ID of the host who created the event."
                        },
                        "object": {
                          "type": "object",
                          "description": "The Zoom Event description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "ticket_id",
                            "email"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The Zoom Events ID."
                            },
                            "ticket_id": {
                              "type": "string",
                              "description": "The ticket ID generated from the registration."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "ticket_type_id": {
                              "type": "string",
                              "description": "The ticket type ID for which the user registers to."
                            },
                            "external_ticket_id": {
                              "type": "string",
                              "description": "A set of unique alphanumeric characters that references the external ticket ID."
                            },
                            "email": {
                              "type": "string",
                              "description": "The registrant's email."
                            },
                            "ticket_role_type": {
                              "type": "string",
                              "description": "The ticket role type: \n * normal The normal ticket \n * alternative_host The alternative host ticket \n * panelist The panelist ticket \n * bind_meeting The bind meeting ticket \n * speaker The speaker ticket \n * interpreter The interpreter ticket \n * sponsor The sponsor ticket \n * host The host ticket \n * moderator The moderator ticket \n * expo_booth_owner The expo booth owner ticket \n * attendee_without_order The attendee without order ticket \n * pre_registration \u2014 The pre-registration ticket \n * company_zoom_room The company zoom room ticket \n * moderator_allhost The moderator all host ticket \n * hub_host The hub host ticket \n * hub_owner The hub owner ticket \n * subscription The subscription ticket",
                              "example": "normal",
                              "enum": [
                                "normal",
                                "alternative_host",
                                "panelist",
                                "bind_meeting",
                                "speaker",
                                "subscription",
                                "sponsor",
                                "interpreter",
                                "host",
                                "moderator",
                                "expo_booth_owner",
                                "attendee_without_order",
                                "pre_registration",
                                "company_zoom_room",
                                "moderator_allhost",
                                "hub_host",
                                "hub_owner"
                              ]
                            },
                            "authentication_method": {
                              "type": "string",
                              "description": "The event authentication method for the ticket: \n * zoom_account Zoom account holder  * zoom_account_otp Zoom account holder with OTP  * corporate_idp Corporate IDP  * no_auth No authentication",
                              "enum": [
                                "zoom_account",
                                "zoom_account_otp",
                                "corporate_idp",
                                "no_auth"
                              ],
                              "example": "no_auth"
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
                    "value": "{\n  \"event\": \"zoom_events.ticket_created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"djsjgkjhfhk\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"ticket_id\": \"iso9Dllddp39dagjLj9j\",\n      \"ticket_type_id\": \"pfalaof9s83lsLJD9u2d\",\n      \"external_ticket_id\": \"abc1231245-adf\",\n      \"ticket_role_type\": \"normal\",\n      \"email\": \"email@zoom.us\",\n      \"authentication_method\": \"no_auth\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoom_events.exhibitor_deleted": {
      "post": {
        "tags": [
          "zoom_events"
        ],
        "operationId": "zoom_events.exhibitor_deleted",
        "requestBody": {
          "description": "# zoom_events.exhibitor_deleted\nThe **Exhibitor Deleted**  event  is triggered every time a user in your account or your app deletes exhibitor for an event in Zoom Events.\n\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can delete an exhibitor for an event.\n\n## Prerequisites\n\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n    * A valid **Event Notification Endpoint URL**.\n    * The **Exhibitor Deleted** subscription enabled under the **Zoom Events** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_exhibitors:write:admin`,`zoom_events_exhibitors:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:exhibitor`,`zoom_events:read:exhibitor:admin`\n\n**Event type**: `zoom_events.exhibitor_deleted`\n",
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
                    "description": "The name of the event, zoom_events.exhibitor_deleted"
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
                        "description": "The account ID of the host who deleted the event exhibitor."
                      },
                      "object": {
                        "type": "object",
                        "description": "Exhibitor Description.",
                        "required": [
                          "event_id",
                          "host_id",
                          "exhibitor_id",
                          "name",
                          "contact_name",
                          "contact_email",
                          "is_sponsor"
                        ],
                        "properties": {
                          "event_id": {
                            "type": "string",
                            "description": "The zoom-events event ID."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The event host(creator)'s user ID.",
                            "example": "x1yCzABCDEfg23HiJKl4mN"
                          },
                          "exhibitor_id": {
                            "type": "string",
                            "description": "The exhibitors's ID."
                          },
                          "name": {
                            "type": "string",
                            "description": "The exhibitors's name."
                          },
                          "description": {
                            "type": "string",
                            "description": "The exhibitor's description."
                          },
                          "is_sponsor": {
                            "type": "boolean",
                            "description": "Whether the exhibitor is a sponsor."
                          },
                          "tier_id": {
                            "type": "string",
                            "description": "The sponsor tier ID for a particular event. This field only applies to a sponsor."
                          },
                          "contact_name": {
                            "type": "string",
                            "description": "The contact's full name."
                          },
                          "contact_email": {
                            "type": "string",
                            "description": "The contact's email address."
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
                  "value": "{\n  \"event\": \"zoom_events.exhibitor_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"exhibitor_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"exhibitor1\",\n      \"description\": \"Description of the exhibitor\",\n      \"is_sponsor\": true,\n      \"tier_id\": \"4fgRIOXjQGmoH6raAzJf8g\",\n      \"contact_name\": \"John Doe\",\n      \"contact_email\": \"exhibitor1-email@zoom.us\"\n    }\n  }\n}"
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
                      "description": "The name of the event, zoom_events.exhibitor_deleted"
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
                          "description": "The account ID of the host who deleted the event exhibitor."
                        },
                        "object": {
                          "type": "object",
                          "description": "Exhibitor Description.",
                          "required": [
                            "event_id",
                            "host_id",
                            "exhibitor_id",
                            "name",
                            "contact_name",
                            "contact_email",
                            "is_sponsor"
                          ],
                          "properties": {
                            "event_id": {
                              "type": "string",
                              "description": "The zoom-events event ID."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The event host(creator)'s user ID.",
                              "example": "x1yCzABCDEfg23HiJKl4mN"
                            },
                            "exhibitor_id": {
                              "type": "string",
                              "description": "The exhibitors's ID."
                            },
                            "name": {
                              "type": "string",
                              "description": "The exhibitors's name."
                            },
                            "description": {
                              "type": "string",
                              "description": "The exhibitor's description."
                            },
                            "is_sponsor": {
                              "type": "boolean",
                              "description": "Whether the exhibitor is a sponsor."
                            },
                            "tier_id": {
                              "type": "string",
                              "description": "The sponsor tier ID for a particular event. This field only applies to a sponsor."
                            },
                            "contact_name": {
                              "type": "string",
                              "description": "The contact's full name."
                            },
                            "contact_email": {
                              "type": "string",
                              "description": "The contact's email address."
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
                    "value": "{\n  \"event\": \"zoom_events.exhibitor_deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"event_id\": \"qwerq1243124\",\n      \"host_id\": \"z8yCxjabcdEFGHfp8uQ\",\n      \"exhibitor_id\": \"iso9Dllddp39dagjLj9j\",\n      \"name\": \"exhibitor1\",\n      \"description\": \"Description of the exhibitor\",\n      \"is_sponsor\": true,\n      \"tier_id\": \"4fgRIOXjQGmoH6raAzJf8g\",\n      \"contact_name\": \"John Doe\",\n      \"contact_email\": \"exhibitor1-email@zoom.us\"\n    }\n  }\n}"
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
      "events",
      "events"
    ]
  },
  "downloadPath": "/api-hub/events/events/webhooks.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": false
  },
  "productMeta": {
    "specType": "adoc",
    "slug": "/docs/api/rest/reference/event/events",
    "productName": "Zoom Events",
    "title": "Zoom API Events - Zoom Events",
    "keywords": "zoom, marketplace, webhook, event, create, update, delete, real-time, data, summary, qos, quality, service",
    "zoapEndpoint": "webhookByProduct",
    "params": {
      "productName": "Zoom Events"
    },
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "productDisplayTitle": "Events",
    "scalarSlug": "/docs/api/events/events",
    "svgname": "_zoomevents"
  },
  "frontmatter": {
    "title": "Events Webhooks",
    "keywords": "zoom, marketplace, webhook, event, create, update, delete, real-time, data, summary, qos, quality, service",
    "description": "The Webinars Plus & Events webhooks allow developers to receive events for [Webinars Plus & Events](https://developers.zoom.us/docs/zoom-events/) features.",
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
        "events",
        "events"
      ]
    },
    "downloadPath": "/api-hub/events/events/webhooks.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": false
    },
    "productMeta": {
      "specType": "adoc",
      "slug": "/docs/api/rest/reference/event/events",
      "productName": "Zoom Events",
      "title": "Zoom API Events - Zoom Events",
      "keywords": "zoom, marketplace, webhook, event, create, update, delete, real-time, data, summary, qos, quality, service",
      "zoapEndpoint": "webhookByProduct",
      "params": {
        "productName": "Zoom Events"
      },
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "productDisplayTitle": "Events",
      "scalarSlug": "/docs/api/events/events",
      "svgname": "_zoomevents"
    },
    "spec": "{\"openapi\":\"3.1.0\",\"info\":{\"title\":\"Events\",\"description\":\"The Webinars Plus & Events webhooks allow developers to receive events for [Webinars Plus & Events](https://developers.zoom.us/docs/zoom-events/) features.\",\"version\":\"1.0.0\"},\"paths\":{\"zoom_events.session_deleted\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.session_deleted\",\"requestBody\":{\"description\":\"# zoom_events.session_deleted\\nThe **Session Deleted** event is triggered every time a user in your account or your app, deletes a session in Zoom Events.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host or a co-editor can delete a session.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **Session Deleted** subscription enabled under the **Zoom Events** event.\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_sessions:write:admin`,`zoom_events_sessions:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:session`,`zoom_events:read:session:admin`\\n\\n**Event type**: `zoom_events.session_deleted`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. zoom_events.event.session_deleted.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who deleted the session post event publishing.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the session.\",\"required\":[\"event_id\",\"host_id\",\"session_id\",\"name\",\"type\",\"start_time\",\"end_time\",\"timezone\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The Zoom Events ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"session_id\":{\"type\":\"string\",\"description\":\"The session ID.\",\"example\":\"4uzfv3JwTeyR5QpC3PXwMg\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the session.\",\"example\":\"Session 1\"},\"type\":{\"type\":\"integer\",\"description\":\"The session type. \\n * 0 \u2014 Meeting \\n * 2 \u2014 Webinar\",\"enum\":[0,2]},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-05-31T13:00:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-05-31T13:00:00Z\"},\"timezone\":{\"type\":\"string\",\"description\":\"The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones).\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.session_deleted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"1234567890abcd\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"session_id\\\": \\\"fhhfkdhkhdgkhg\\\",\\n      \\\"name\\\": \\\"my session\\\",\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"type\\\": 2,\\n      \\\"start_time\\\": \\\"2022-05-31T13:00:00Z\\\",\\n      \\\"end_time\\\": \\\"2022-05-31T13:00:00Z\\\",\\n      \\\"timezone\\\": \\\"America/Los_Angeles\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. zoom_events.event.session_deleted.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who deleted the session post event publishing.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the session.\",\"required\":[\"event_id\",\"host_id\",\"session_id\",\"name\",\"type\",\"start_time\",\"end_time\",\"timezone\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The Zoom Events ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"session_id\":{\"type\":\"string\",\"description\":\"The session ID.\",\"example\":\"4uzfv3JwTeyR5QpC3PXwMg\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the session.\",\"example\":\"Session 1\"},\"type\":{\"type\":\"integer\",\"description\":\"The session type. \\n * 0 \u2014 Meeting \\n * 2 \u2014 Webinar\",\"enum\":[0,2]},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-05-31T13:00:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-05-31T13:00:00Z\"},\"timezone\":{\"type\":\"string\",\"description\":\"The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones).\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.session_deleted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"1234567890abcd\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"session_id\\\": \\\"fhhfkdhkhdgkhg\\\",\\n      \\\"name\\\": \\\"my session\\\",\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"type\\\": 2,\\n      \\\"start_time\\\": \\\"2022-05-31T13:00:00Z\\\",\\n      \\\"end_time\\\": \\\"2022-05-31T13:00:00Z\\\",\\n      \\\"timezone\\\": \\\"America/Los_Angeles\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.ticket_updated\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.ticket_updated\",\"requestBody\":{\"description\":\"# zoom_events.ticket_updated\\nThe **Ticket Updated**  event  is triggered every time a user in your account or your app updates the attendee registration information in Zoom Events.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can update a pre-registration for an event by updating registration question answers of a ticket.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **Ticket Updated** subscription enabled under the **Zoom Events** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_tickets:write:admin`,`zoom_events_tickets:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:ticket`,`zoom_events:read:ticket:admin`\\n\\n**Event type**: `zoom_events.ticket_updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.ticket_updated\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who created the event.\"},\"object\":{\"type\":\"object\",\"description\":\" The ticket Description.\",\"required\":[\"event_id\",\"host_id\",\"ticket_id\",\"email\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The Zoom Events ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"ticket_id\":{\"type\":\"string\",\"description\":\"The ticket ID generated from the registration.\"},\"ticket_type_id\":{\"type\":\"string\",\"description\":\"The ticket type ID for which the user registers to.\"},\"external_ticket_id\":{\"type\":\"string\",\"description\":\"A set of unique alphanumeric characters that references the external ticket ID.\"},\"email\":{\"type\":\"string\",\"description\":\"The registrant's email.\"},\"ticket_role_type\":{\"type\":\"string\",\"description\":\"The ticket role type: \\n * normal The normal ticket \\n * alternative_host The alternative host ticket \\n * panelist The panelist ticket \\n * bind_meeting The bind meeting ticket \\n * speaker The speaker ticket \\n * interpreter The interpreter ticket \\n * sponsor The sponsor ticket \\n * host The host ticket \\n * moderator The moderator ticket \\n * expo_booth_owner The expo booth owner ticket \\n * attendee_without_order The attendee without order ticket \\n * company_zoom_room The company zoom room ticket \\n * moderator_allhost The moderator all host ticket \\n * hub_host The hub host ticket \\n * hub_owner The hub owner ticket &quot;,\\n * subscription The subscription ticket &quot;\",\"example\":\"normal\",\"enum\":[\"normal\",\"alternative_host\",\"panelist\",\"bind_meeting\",\"speaker\",\"subscription\",\"sponsor\",\"interpreter\",\"host\",\"moderator\",\"expo_booth_owner\",\"attendee_without_order\",\"company_zoom_room\",\"moderator_allhost\",\"hub_host\",\"hub_owner\"]},\"authentication_method\":{\"type\":\"string\",\"description\":\"The event authentication method for the ticket: \\n * zoom_account Zoom account holder  * zoom_account_otp Zoom account holder with OTP  * corporate_idp Corporate IDP  * no_auth No authentication\",\"enum\":[\"zoom_account\",\"zoom_account_otp\",\"corporate_idp\",\"no_auth\"],\"example\":\"no_auth\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.ticket_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"ticket_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"ticket_type_id\\\": \\\"pfalaof9s83lsLJD9u2d\\\",\\n      \\\"external_ticket_id\\\": \\\"abc-123-efgh\\\",\\n      \\\"ticket_role_type\\\": \\\"normal\\\",\\n      \\\"email\\\": \\\"email@zoom.us\\\",\\n      \\\"authentication_method\\\": \\\"no_auth\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.ticket_updated\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who created the event.\"},\"object\":{\"type\":\"object\",\"description\":\" The ticket Description.\",\"required\":[\"event_id\",\"host_id\",\"ticket_id\",\"email\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The Zoom Events ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"ticket_id\":{\"type\":\"string\",\"description\":\"The ticket ID generated from the registration.\"},\"ticket_type_id\":{\"type\":\"string\",\"description\":\"The ticket type ID for which the user registers to.\"},\"external_ticket_id\":{\"type\":\"string\",\"description\":\"A set of unique alphanumeric characters that references the external ticket ID.\"},\"email\":{\"type\":\"string\",\"description\":\"The registrant's email.\"},\"ticket_role_type\":{\"type\":\"string\",\"description\":\"The ticket role type: \\n * normal The normal ticket \\n * alternative_host The alternative host ticket \\n * panelist The panelist ticket \\n * bind_meeting The bind meeting ticket \\n * speaker The speaker ticket \\n * interpreter The interpreter ticket \\n * sponsor The sponsor ticket \\n * host The host ticket \\n * moderator The moderator ticket \\n * expo_booth_owner The expo booth owner ticket \\n * attendee_without_order The attendee without order ticket \\n * company_zoom_room The company zoom room ticket \\n * moderator_allhost The moderator all host ticket \\n * hub_host The hub host ticket \\n * hub_owner The hub owner ticket &quot;,\\n * subscription The subscription ticket &quot;\",\"example\":\"normal\",\"enum\":[\"normal\",\"alternative_host\",\"panelist\",\"bind_meeting\",\"speaker\",\"subscription\",\"sponsor\",\"interpreter\",\"host\",\"moderator\",\"expo_booth_owner\",\"attendee_without_order\",\"company_zoom_room\",\"moderator_allhost\",\"hub_host\",\"hub_owner\"]},\"authentication_method\":{\"type\":\"string\",\"description\":\"The event authentication method for the ticket: \\n * zoom_account Zoom account holder  * zoom_account_otp Zoom account holder with OTP  * corporate_idp Corporate IDP  * no_auth No authentication\",\"enum\":[\"zoom_account\",\"zoom_account_otp\",\"corporate_idp\",\"no_auth\"],\"example\":\"no_auth\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.ticket_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"ticket_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"ticket_type_id\\\": \\\"pfalaof9s83lsLJD9u2d\\\",\\n      \\\"external_ticket_id\\\": \\\"abc-123-efgh\\\",\\n      \\\"ticket_role_type\\\": \\\"normal\\\",\\n      \\\"email\\\": \\\"email@zoom.us\\\",\\n      \\\"authentication_method\\\": \\\"no_auth\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.event_published\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.event_published\",\"requestBody\":{\"description\":\"# zoom_events.event_published\\nThe **Event Published** event is triggered every time a user in your account or your app, publishes an event in Zoom Events.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host or a co-editor can publish an event.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **Event published** subscription enabled under the **Zoom Events** event.\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_basic:write:admin`,`zoom_events_basic:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:event`,`zoom_events:read:event:admin`\\n\\n**Event type**: `zoom_events.event_published`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. zoom_events.event_published.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who published the event.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom Event description.\",\"required\":[\"event_id\",\"host_id\",\"name\",\"event_type\",\"start_time\",\"end_time\",\"timezone\"],\"properties\":{\"hub_id\":{\"type\":\"string\",\"description\":\"The ID of the event hub.\",\"example\":\"234kj2h34kljgh23lkhj3\"},\"event_id\":{\"type\":\"string\",\"description\":\"The ID of the event.\",\"example\":\"234kj2h34kljgh23lkhj3\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the event.\",\"example\":\"OpenAPI Conference Name\"},\"description\":{\"type\":\"string\",\"description\":\"The description of the event.\",\"example\":\"This event was created with the OpenAPI\"},\"event_type\":{\"type\":\"string\",\"description\":\"Event Types: \\n * CONFERENCE - Multi session event \\n * SIMPLE_EVENT - Single session event \\n * RECURRING - Recurring session event\",\"enum\":[\"CONFERENCE\",\"SIMPLE_EVENT\",\"RECURRING\"],\"example\":\"CONFERENCE\"},\"status\":{\"type\":\"string\",\"description\":\"The event status: \\n * PUBLISHED - Published event \\n * DRAFT - Draft event \\n * CANCELLED - Cancelled event\",\"enum\":[\"PUBLISHED\",\"DRAFT\",\"CANCELLED\"],\"example\":\"PUBLISHED\"},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"timezone\":{\"type\":\"string\",\"description\":\"The timezone of the event.\",\"example\":\"America/Los_Angeles\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.event_published\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"hub_id\\\": \\\"1234567890abcd\\\",\\n      \\\"event_id\\\": \\\"1234567890abcd\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"name\\\": \\\"My Event-1\\\",\\n      \\\"description\\\": \\\"This is a test event.\\\",\\n      \\\"event_type\\\": \\\"CONFERENCE\\\",\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"status\\\": \\\"DRAFT\\\",\\n      \\\"start_time\\\": \\\"2023-07-13T21:44:51Z\\\",\\n      \\\"end_time\\\": \\\"2023-07-13T23:00:51Z\\\",\\n      \\\"timezone\\\": \\\"America/Los_Angeles\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. zoom_events.event_published.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who published the event.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom Event description.\",\"required\":[\"event_id\",\"host_id\",\"name\",\"event_type\",\"start_time\",\"end_time\",\"timezone\"],\"properties\":{\"hub_id\":{\"type\":\"string\",\"description\":\"The ID of the event hub.\",\"example\":\"234kj2h34kljgh23lkhj3\"},\"event_id\":{\"type\":\"string\",\"description\":\"The ID of the event.\",\"example\":\"234kj2h34kljgh23lkhj3\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the event.\",\"example\":\"OpenAPI Conference Name\"},\"description\":{\"type\":\"string\",\"description\":\"The description of the event.\",\"example\":\"This event was created with the OpenAPI\"},\"event_type\":{\"type\":\"string\",\"description\":\"Event Types: \\n * CONFERENCE - Multi session event \\n * SIMPLE_EVENT - Single session event \\n * RECURRING - Recurring session event\",\"enum\":[\"CONFERENCE\",\"SIMPLE_EVENT\",\"RECURRING\"],\"example\":\"CONFERENCE\"},\"status\":{\"type\":\"string\",\"description\":\"The event status: \\n * PUBLISHED - Published event \\n * DRAFT - Draft event \\n * CANCELLED - Cancelled event\",\"enum\":[\"PUBLISHED\",\"DRAFT\",\"CANCELLED\"],\"example\":\"PUBLISHED\"},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"timezone\":{\"type\":\"string\",\"description\":\"The timezone of the event.\",\"example\":\"America/Los_Angeles\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.event_published\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"hub_id\\\": \\\"1234567890abcd\\\",\\n      \\\"event_id\\\": \\\"1234567890abcd\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"name\\\": \\\"My Event-1\\\",\\n      \\\"description\\\": \\\"This is a test event.\\\",\\n      \\\"event_type\\\": \\\"CONFERENCE\\\",\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"status\\\": \\\"DRAFT\\\",\\n      \\\"start_time\\\": \\\"2023-07-13T21:44:51Z\\\",\\n      \\\"end_time\\\": \\\"2023-07-13T23:00:51Z\\\",\\n      \\\"timezone\\\": \\\"America/Los_Angeles\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.ticket_type_updated\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.ticket_type_updated\",\"requestBody\":{\"description\":\"# zoom_events.ticket_type_updated\\nThe **TicketType Updated**  event  is triggered every time a user in your account or your app updates ticketType for an event in Zoom Events.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can update a ticketType for an event.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **TicketType Updated** subscription enabled under the **Zoom Events** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_ticket_types:write:admin`,`zoom_events_ticket_types:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:list_ticket_types`,`zoom_events:read:list_ticket_types:admin`\\n\\n**Event type**: `zoom_events.ticket_type_updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.ticket_type_updated\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who updated the ticketType.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the ticketType.\",\"required\":[\"event_id\",\"host_id\",\"ticket_type_id\",\"name\",\"start_time\",\"end_time\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The zoom-events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"ticket_type_id\":{\"type\":\"string\",\"description\":\"The ticketType ID for which the user registers to.\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the ticketType.\",\"example\":\"General Admission Ticket\"},\"description\":{\"type\":\"string\",\"description\":\"The description of the ticketType.\",\"example\":\"This is a General Admission ticket\"},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.ticket_type_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"ticket_type_id\\\": \\\"pfalaof9s83lsLJD9u2d\\\",\\n      \\\"name\\\": \\\"General Admission Ticket\\\",\\n      \\\"description\\\": \\\"This is for General Admission\\\",\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"start_time\\\": \\\"2023-07-13T21:44:51Z\\\",\\n      \\\"end_time\\\": \\\"2023-07-13T23:00:51Z\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.ticket_type_updated\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who updated the ticketType.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the ticketType.\",\"required\":[\"event_id\",\"host_id\",\"ticket_type_id\",\"name\",\"start_time\",\"end_time\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The zoom-events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"ticket_type_id\":{\"type\":\"string\",\"description\":\"The ticketType ID for which the user registers to.\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the ticketType.\",\"example\":\"General Admission Ticket\"},\"description\":{\"type\":\"string\",\"description\":\"The description of the ticketType.\",\"example\":\"This is a General Admission ticket\"},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.ticket_type_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"ticket_type_id\\\": \\\"pfalaof9s83lsLJD9u2d\\\",\\n      \\\"name\\\": \\\"General Admission Ticket\\\",\\n      \\\"description\\\": \\\"This is for General Admission\\\",\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"start_time\\\": \\\"2023-07-13T21:44:51Z\\\",\\n      \\\"end_time\\\": \\\"2023-07-13T23:00:51Z\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.exhibitor_created\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.exhibitor_created\",\"requestBody\":{\"description\":\"# zoom_events.exhibitor_created\\nThe **Exhibitor Created**  event  is triggered every time a user in your account or your app creates exhibitor for an event in Zoom Events.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can create an exhibitor for an event.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **Exhibitor Created** subscription enabled under the **Zoom Events** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_exhibitors:write:admin`,`zoom_events_exhibitors:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:exhibitor`,`zoom_events:read:exhibitor:admin`\\n\\n**Event type**: `zoom_events.exhibitor_created`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.exhibitor_created\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who created the event exhibitor.\"},\"object\":{\"type\":\"object\",\"description\":\"Exhibitor Description.\",\"required\":[\"event_id\",\"host_id\",\"exhibitor_id\",\"name\",\"contact_name\",\"contact_email\",\"is_sponsor\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The zoom-events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"exhibitor_id\":{\"type\":\"string\",\"description\":\"The exhibitors's ID.\"},\"name\":{\"type\":\"string\",\"description\":\"The exhibitors's name.\"},\"description\":{\"type\":\"string\",\"description\":\"The exhibitor's description.\"},\"is_sponsor\":{\"type\":\"boolean\",\"description\":\"Whether the exhibitor is a sponsor.\"},\"tier_id\":{\"type\":\"string\",\"description\":\"The sponsor tier ID for a particular event. This field only applies to a sponsor.\"},\"contact_name\":{\"type\":\"string\",\"description\":\"The contact's full name.\"},\"contact_email\":{\"type\":\"string\",\"description\":\"The contact's email address.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.exhibitor_created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"exhibitor_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"name\\\": \\\"exhibitor1\\\",\\n      \\\"description\\\": \\\"Description of the exhibitor\\\",\\n      \\\"is_sponsor\\\": true,\\n      \\\"tier_id\\\": \\\"4fgRIOXjQGmoH6raAzJf8g\\\",\\n      \\\"contact_name\\\": \\\"John Doe\\\",\\n      \\\"contact_email\\\": \\\"exhibitor1-email@zoom.us\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.exhibitor_created\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who created the event exhibitor.\"},\"object\":{\"type\":\"object\",\"description\":\"Exhibitor Description.\",\"required\":[\"event_id\",\"host_id\",\"exhibitor_id\",\"name\",\"contact_name\",\"contact_email\",\"is_sponsor\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The zoom-events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"exhibitor_id\":{\"type\":\"string\",\"description\":\"The exhibitors's ID.\"},\"name\":{\"type\":\"string\",\"description\":\"The exhibitors's name.\"},\"description\":{\"type\":\"string\",\"description\":\"The exhibitor's description.\"},\"is_sponsor\":{\"type\":\"boolean\",\"description\":\"Whether the exhibitor is a sponsor.\"},\"tier_id\":{\"type\":\"string\",\"description\":\"The sponsor tier ID for a particular event. This field only applies to a sponsor.\"},\"contact_name\":{\"type\":\"string\",\"description\":\"The contact's full name.\"},\"contact_email\":{\"type\":\"string\",\"description\":\"The contact's email address.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.exhibitor_created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"exhibitor_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"name\\\": \\\"exhibitor1\\\",\\n      \\\"description\\\": \\\"Description of the exhibitor\\\",\\n      \\\"is_sponsor\\\": true,\\n      \\\"tier_id\\\": \\\"4fgRIOXjQGmoH6raAzJf8g\\\",\\n      \\\"contact_name\\\": \\\"John Doe\\\",\\n      \\\"contact_email\\\": \\\"exhibitor1-email@zoom.us\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.exhibitor_updated\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.exhibitor_updated\",\"requestBody\":{\"description\":\"# zoom_events.exhibitor_updated\\nThe **Exhibitor Updated**  event  is triggered every time a user in your account or your app updates exhibitor for an event in Zoom Events.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can update an exhibitor for an event.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **Exhibitor Updated** subscription enabled under the **Zoom Events** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_exhibitors:write:admin`,`zoom_events_exhibitors:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:exhibitor`,`zoom_events:read:exhibitor:admin`\\n\\n**Event type**: `zoom_events.exhibitor_updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.exhibitor_updated\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who updated the event exhibitor.\"},\"object\":{\"type\":\"object\",\"description\":\"Exhibitor Description.\",\"required\":[\"event_id\",\"host_id\",\"exhibitor_id\",\"name\",\"contact_name\",\"contact_email\",\"is_sponsor\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The zoom-events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"exhibitor_id\":{\"type\":\"string\",\"description\":\"The exhibitors's ID.\"},\"name\":{\"type\":\"string\",\"description\":\"The exhibitors's name.\"},\"description\":{\"type\":\"string\",\"description\":\"The exhibitor's description.\"},\"is_sponsor\":{\"type\":\"boolean\",\"description\":\"Whether the exhibitor is a sponsor.\"},\"tier_id\":{\"type\":\"string\",\"description\":\"The sponsor tier ID for a particular event. This field only applies to a sponsor.\"},\"contact_name\":{\"type\":\"string\",\"description\":\"The contact's full name.\"},\"contact_email\":{\"type\":\"string\",\"description\":\"The contact's email address.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.exhibitor_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"exhibitor_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"name\\\": \\\"exhibitor1\\\",\\n      \\\"description\\\": \\\"Description of the exhibitor\\\",\\n      \\\"is_sponsor\\\": true,\\n      \\\"tier_id\\\": \\\"4fgRIOXjQGmoH6raAzJf8g\\\",\\n      \\\"contact_name\\\": \\\"John Doe\\\",\\n      \\\"contact_email\\\": \\\"exhibitor1-email@zoom.us\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.exhibitor_updated\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who updated the event exhibitor.\"},\"object\":{\"type\":\"object\",\"description\":\"Exhibitor Description.\",\"required\":[\"event_id\",\"host_id\",\"exhibitor_id\",\"name\",\"contact_name\",\"contact_email\",\"is_sponsor\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The zoom-events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"exhibitor_id\":{\"type\":\"string\",\"description\":\"The exhibitors's ID.\"},\"name\":{\"type\":\"string\",\"description\":\"The exhibitors's name.\"},\"description\":{\"type\":\"string\",\"description\":\"The exhibitor's description.\"},\"is_sponsor\":{\"type\":\"boolean\",\"description\":\"Whether the exhibitor is a sponsor.\"},\"tier_id\":{\"type\":\"string\",\"description\":\"The sponsor tier ID for a particular event. This field only applies to a sponsor.\"},\"contact_name\":{\"type\":\"string\",\"description\":\"The contact's full name.\"},\"contact_email\":{\"type\":\"string\",\"description\":\"The contact's email address.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.exhibitor_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"exhibitor_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"name\\\": \\\"exhibitor1\\\",\\n      \\\"description\\\": \\\"Description of the exhibitor\\\",\\n      \\\"is_sponsor\\\": true,\\n      \\\"tier_id\\\": \\\"4fgRIOXjQGmoH6raAzJf8g\\\",\\n      \\\"contact_name\\\": \\\"John Doe\\\",\\n      \\\"contact_email\\\": \\\"exhibitor1-email@zoom.us\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.event_cancelled\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.event_cancelled\",\"requestBody\":{\"description\":\"# zoom_events.event_cancelled\\nThe **Event Cancelled** event is triggered every time a user in your account or your app, cancels the event in Zoom Events.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host can cancel the event.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **Event Cancelled** subscription enabled under the **Zoom Events** event.\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_basic:write:admin`,`zoom_events_basic:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:event`,`zoom_events:read:event:admin`\\n\\n**Event type**: `zoom_events.event_cancelled`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. zoom_events.event_cancelled.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who cancelled the event.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom Event description.\",\"required\":[\"event_id\",\"host_id\",\"name\",\"event_type\",\"start_time\",\"end_time\",\"timezone\"],\"properties\":{\"hub_id\":{\"type\":\"string\",\"description\":\"The ID of the event hub.\",\"example\":\"234kj2h34kljgh23lkhj3\"},\"event_id\":{\"type\":\"string\",\"description\":\"The ID of the event.\",\"example\":\"234kj2h34kljgh23lkhj3\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the event.\",\"example\":\"OpenAPI Conference Name\"},\"description\":{\"type\":\"string\",\"description\":\"The description of the event.\",\"example\":\"This event was created with the OpenAPI\"},\"event_type\":{\"type\":\"string\",\"description\":\"Event Types: \\n * CONFERENCE - Multi session event \\n * SIMPLE_EVENT - Single session event \\n * RECURRING - Recurring session event\",\"enum\":[\"CONFERENCE\",\"SIMPLE_EVENT\",\"RECURRING\"],\"example\":\"CONFERENCE\"},\"status\":{\"type\":\"string\",\"description\":\"The event status: \\n * PUBLISHED - Published event \\n * DRAFT - Draft event \\n * CANCELLED - Cancelled event\",\"enum\":[\"PUBLISHED\",\"DRAFT\",\"CANCELLED\"],\"example\":\"PUBLISHED\"},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"timezone\":{\"type\":\"string\",\"description\":\"The timezone of the event.\",\"example\":\"America/Los_Angeles\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.event_cancelled\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"hub_id\\\": \\\"1234567890abcd\\\",\\n      \\\"event_id\\\": \\\"1234567890abcd\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"name\\\": \\\"My Event-1\\\",\\n      \\\"description\\\": \\\"This is a test event.\\\",\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"event_type\\\": \\\"CONFERENCE\\\",\\n      \\\"status\\\": \\\"DRAFT\\\",\\n      \\\"start_time\\\": \\\"2023-07-13T21:44:51Z\\\",\\n      \\\"end_time\\\": \\\"2023-07-13T23:00:51Z\\\",\\n      \\\"timezone\\\": \\\"America/Los_Angeles\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. zoom_events.event_cancelled.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who cancelled the event.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom Event description.\",\"required\":[\"event_id\",\"host_id\",\"name\",\"event_type\",\"start_time\",\"end_time\",\"timezone\"],\"properties\":{\"hub_id\":{\"type\":\"string\",\"description\":\"The ID of the event hub.\",\"example\":\"234kj2h34kljgh23lkhj3\"},\"event_id\":{\"type\":\"string\",\"description\":\"The ID of the event.\",\"example\":\"234kj2h34kljgh23lkhj3\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the event.\",\"example\":\"OpenAPI Conference Name\"},\"description\":{\"type\":\"string\",\"description\":\"The description of the event.\",\"example\":\"This event was created with the OpenAPI\"},\"event_type\":{\"type\":\"string\",\"description\":\"Event Types: \\n * CONFERENCE - Multi session event \\n * SIMPLE_EVENT - Single session event \\n * RECURRING - Recurring session event\",\"enum\":[\"CONFERENCE\",\"SIMPLE_EVENT\",\"RECURRING\"],\"example\":\"CONFERENCE\"},\"status\":{\"type\":\"string\",\"description\":\"The event status: \\n * PUBLISHED - Published event \\n * DRAFT - Draft event \\n * CANCELLED - Cancelled event\",\"enum\":[\"PUBLISHED\",\"DRAFT\",\"CANCELLED\"],\"example\":\"PUBLISHED\"},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"timezone\":{\"type\":\"string\",\"description\":\"The timezone of the event.\",\"example\":\"America/Los_Angeles\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.event_cancelled\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"hub_id\\\": \\\"1234567890abcd\\\",\\n      \\\"event_id\\\": \\\"1234567890abcd\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"name\\\": \\\"My Event-1\\\",\\n      \\\"description\\\": \\\"This is a test event.\\\",\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"event_type\\\": \\\"CONFERENCE\\\",\\n      \\\"status\\\": \\\"DRAFT\\\",\\n      \\\"start_time\\\": \\\"2023-07-13T21:44:51Z\\\",\\n      \\\"end_time\\\": \\\"2023-07-13T23:00:51Z\\\",\\n      \\\"timezone\\\": \\\"America/Los_Angeles\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.event_duplicated\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.event_duplicated\",\"requestBody\":{\"description\":\"# zoom_events.event_duplicated\\nThe **Event Duplicated** is triggered every time a user in your account or your app, duplicates an existing event in Zoom Events.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host can duplicate an event.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **Event Duplicated** subscription enabled under the **Zoom Events** event.\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_basic:write:admin`,`zoom_events_basic:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:event`,`zoom_events:read:event:admin`\\n\\n**Event type**: `zoom_events.event_duplicated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. zoom_events.event_duplicated.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who duplicated the event.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom Event description.\",\"required\":[\"event_id\",\"host_id\",\"name\",\"event_type\",\"start_time\",\"end_time\",\"timezone\"],\"properties\":{\"hub_id\":{\"type\":\"string\",\"description\":\"The ID of the event hub.\",\"example\":\"h134kj2h34kljgh23lkhj3\"},\"event_id\":{\"type\":\"string\",\"description\":\"The ID of the event.\",\"example\":\"234kj2h34kljgh23lkhj3\"},\"source_event_id\":{\"type\":\"string\",\"description\":\"The ID of the event from which this event was duplicated from.\",\"example\":\"434kj2h34kljgh23lkhj3\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the event.\",\"example\":\"OpenAPI Conference Name\"},\"description\":{\"type\":\"string\",\"description\":\"The description of the event.\",\"example\":\"This event was created with the OpenAPI\"},\"event_type\":{\"type\":\"string\",\"description\":\"Event Types: \\n * CONFERENCE - Multi session event \\n * SIMPLE_EVENT - Single session event \\n * RECURRING - Recurring session event\",\"enum\":[\"CONFERENCE\",\"SIMPLE_EVENT\",\"RECURRING\"],\"example\":\"CONFERENCE\"},\"status\":{\"type\":\"string\",\"description\":\"The event status: \\n * PUBLISHED - Published event \\n * DRAFT - Draft event \\n * CANCELLED - Cancelled event\",\"enum\":[\"PUBLISHED\",\"DRAFT\",\"CANCELLED\"],\"example\":\"DRAFT\"},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"timezone\":{\"type\":\"string\",\"description\":\"The timezone of the event.\",\"example\":\"America/Los_Angeles\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.event_duplicated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"hub_id\\\": \\\"1234567890abcd\\\",\\n      \\\"event_id\\\": \\\"1234567890abcd\\\",\\n      \\\"source_event_id\\\": \\\"1234567890abcd\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"name\\\": \\\"My Event-1\\\",\\n      \\\"description\\\": \\\"This is a test event.\\\",\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"event_type\\\": \\\"CONFERENCE\\\",\\n      \\\"status\\\": \\\"DRAFT\\\",\\n      \\\"start_time\\\": \\\"2023-07-13T21:44:51Z\\\",\\n      \\\"end_time\\\": \\\"2023-07-13T23:00:51Z\\\",\\n      \\\"timezone\\\": \\\"America/Los_Angeles\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. zoom_events.event_duplicated.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who duplicated the event.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom Event description.\",\"required\":[\"event_id\",\"host_id\",\"name\",\"event_type\",\"start_time\",\"end_time\",\"timezone\"],\"properties\":{\"hub_id\":{\"type\":\"string\",\"description\":\"The ID of the event hub.\",\"example\":\"h134kj2h34kljgh23lkhj3\"},\"event_id\":{\"type\":\"string\",\"description\":\"The ID of the event.\",\"example\":\"234kj2h34kljgh23lkhj3\"},\"source_event_id\":{\"type\":\"string\",\"description\":\"The ID of the event from which this event was duplicated from.\",\"example\":\"434kj2h34kljgh23lkhj3\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the event.\",\"example\":\"OpenAPI Conference Name\"},\"description\":{\"type\":\"string\",\"description\":\"The description of the event.\",\"example\":\"This event was created with the OpenAPI\"},\"event_type\":{\"type\":\"string\",\"description\":\"Event Types: \\n * CONFERENCE - Multi session event \\n * SIMPLE_EVENT - Single session event \\n * RECURRING - Recurring session event\",\"enum\":[\"CONFERENCE\",\"SIMPLE_EVENT\",\"RECURRING\"],\"example\":\"CONFERENCE\"},\"status\":{\"type\":\"string\",\"description\":\"The event status: \\n * PUBLISHED - Published event \\n * DRAFT - Draft event \\n * CANCELLED - Cancelled event\",\"enum\":[\"PUBLISHED\",\"DRAFT\",\"CANCELLED\"],\"example\":\"DRAFT\"},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"timezone\":{\"type\":\"string\",\"description\":\"The timezone of the event.\",\"example\":\"America/Los_Angeles\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.event_duplicated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"hub_id\\\": \\\"1234567890abcd\\\",\\n      \\\"event_id\\\": \\\"1234567890abcd\\\",\\n      \\\"source_event_id\\\": \\\"1234567890abcd\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"name\\\": \\\"My Event-1\\\",\\n      \\\"description\\\": \\\"This is a test event.\\\",\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"event_type\\\": \\\"CONFERENCE\\\",\\n      \\\"status\\\": \\\"DRAFT\\\",\\n      \\\"start_time\\\": \\\"2023-07-13T21:44:51Z\\\",\\n      \\\"end_time\\\": \\\"2023-07-13T23:00:51Z\\\",\\n      \\\"timezone\\\": \\\"America/Los_Angeles\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.session_ended\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.session_ended\",\"requestBody\":{\"description\":\"# zoom_events.session_ended\\nThe **End Session** event is triggered every time a user in your account or your app ends a zoom-events session.\\n\\nZoom users with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing zoom-events event sessions. A Zoom Events host can start and end a Zoom Events session.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **End Session** subscription enabled under the **Zoom Events** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_sessions:read:admin`,`zoom_events_sessions:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:session`,`zoom_events:read:session:admin`\\n\\n**Event type**: `zoom_events.session_ended`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who ended the session.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the session.\",\"required\":[\"event_id\",\"host_id\",\"session_id\",\"name\",\"type\",\"start_time\",\"end_time\",\"timezone\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The Zoom Events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"session_id\":{\"type\":\"string\",\"description\":\"The session ID.\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the session.\"},\"type\":{\"type\":\"integer\",\"description\":\"The session type. \\n* 0 \u2014 Meeting.\\n* 2 \u2014 Webinar\",\"enum\":[0,2]},\"start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The session's start time in UTC.\"},\"end_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The date and time when the session ended in UTC.\"},\"timezone\":{\"type\":\"string\",\"description\":\"The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones).\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n    \\\"event\\\": \\\"zoom_events.session_ended\\\",\\n    \\\"event_ts\\\": 1626230691572,\\n    \\\"payload\\\": {\\n        \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n        \\\"object\\\": {\\n            \\\"event_id\\\": \\\"1234567890abcd\\\",\\n            \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n            \\\"session_id\\\": \\\"1234567890abcd\\\",\\n            \\\"name\\\": \\\"My Session-1\\\",\\n            \\\"type\\\": 2,\\n            \\\"start_time\\\": \\\"2023-07-13T21:44:51Z\\\",\\n            \\\"timezone\\\": \\\"America/Los_Angeles\\\",\\n            \\\"end_time\\\": \\\"2023-07-13T23:00:51Z\\\"\\n        }\\n    }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who ended the session.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the session.\",\"required\":[\"event_id\",\"host_id\",\"session_id\",\"name\",\"type\",\"start_time\",\"end_time\",\"timezone\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The Zoom Events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"session_id\":{\"type\":\"string\",\"description\":\"The session ID.\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the session.\"},\"type\":{\"type\":\"integer\",\"description\":\"The session type. \\n* 0 \u2014 Meeting.\\n* 2 \u2014 Webinar\",\"enum\":[0,2]},\"start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The session's start time in UTC.\"},\"end_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The date and time when the session ended in UTC.\"},\"timezone\":{\"type\":\"string\",\"description\":\"The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones).\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n    \\\"event\\\": \\\"zoom_events.session_ended\\\",\\n    \\\"event_ts\\\": 1626230691572,\\n    \\\"payload\\\": {\\n        \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n        \\\"object\\\": {\\n            \\\"event_id\\\": \\\"1234567890abcd\\\",\\n            \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n            \\\"session_id\\\": \\\"1234567890abcd\\\",\\n            \\\"name\\\": \\\"My Session-1\\\",\\n            \\\"type\\\": 2,\\n            \\\"start_time\\\": \\\"2023-07-13T21:44:51Z\\\",\\n            \\\"timezone\\\": \\\"America/Los_Angeles\\\",\\n            \\\"end_time\\\": \\\"2023-07-13T23:00:51Z\\\"\\n        }\\n    }\\n}\"}}}}}}}},\"zoom_events.vod_registration_created\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.vod_registration_created\",\"requestBody\":{\"description\":\"# zoom_events.vod_registration_created\\nThe **VOD Registration Created**  event  is triggered when someone registers for a video on-demand(VOD) channel  created by one of your app users or account users.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events.  The host can create an on-demand video channel, set up how their attendees will access their channels with registration.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**\\n    * The **VOD Registration Created** subscription enabled under the **Zoom Events** event\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_vod_registration:read`,`zoom_events_vod_registration:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:vod_registrations`,`zoom_events:read:vod_registrations:admin`\\n\\n**Event type**: `zoom_events.vod_registration_created`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.vod_registration_created.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who created the event.\"},\"object\":{\"type\":\"object\",\"description\":\"The VOD channel registration description.\",\"required\":[\"channel_id\",\"host_id\",\"hub_id\",\"registration_id\",\"email\"],\"properties\":{\"channel_id\":{\"type\":\"string\",\"description\":\"The VOD channel ID.\"},\"hub_id\":{\"type\":\"string\",\"description\":\"The Zoom Events hub ID\",\"example\":\"234kj2h34kljgh23lkhj3\"},\"host_id\":{\"type\":\"string\",\"description\":\"The channel host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"registration_id\":{\"type\":\"string\",\"description\":\"The registration ID generated from the registration.\"},\"email\":{\"type\":\"string\",\"description\":\"The registrant's email.\"},\"registration_date\":{\"type\":\"string\",\"description\":\"The registrantion date in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ.\",\"format\":\"date-time\",\"example\":\"2025-05-25T22:30:00Z\"},\"authentication_method\":{\"type\":\"string\",\"description\":\"The VOD channel authentication method \\n * zoom_account Zoom account holder   * no_auth No authentication* email_opt registration using email OTP verification* \",\"example\":\"no_auth\",\"enum\":[\"zoom_account\",\"email_opt\",\"no_auth\"]},\"join_link\":{\"type\":\"string\",\"description\":\"The unique join link for the vod registrant.\",\"example\":\"https://events.zoom.us/ejl/xyz\"},\"first_name\":{\"type\":\"string\",\"description\":\"The registrant's first name.\",\"example\":\"Jill\"},\"last_name\":{\"type\":\"string\",\"description\":\"The registrant's last name.\",\"example\":\"Chill\"},\"address\":{\"type\":\"string\",\"description\":\"The registrant's address.\",\"example\":\"1800 Amphibious Blvd\"},\"city\":{\"type\":\"string\",\"description\":\"The registrant's city.\",\"example\":\"Mountain View\"},\"state\":{\"type\":\"string\",\"description\":\"The registrant's state or province.\",\"example\":\"CA\"},\"zip\":{\"type\":\"string\",\"description\":\"The registrant's ZIP or postal code.\",\"example\":\"94045\"},\"country\":{\"type\":\"string\",\"description\":\"The registrant's two-letter [country code](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#countries).\",\"example\":\"US\"},\"phone\":{\"type\":\"string\",\"description\":\"The registrant's phone number.\",\"example\":\"5550100\"},\"industry\":{\"type\":\"string\",\"description\":\"The registrant's industry.\",\"example\":\"Food\"},\"job_title\":{\"type\":\"string\",\"description\":\"The registrant's job title.\",\"example\":\"Chef\"},\"organization\":{\"type\":\"string\",\"description\":\"The registrant's organization.\",\"example\":\"ABC Ltd.\"},\"organization_size\":{\"type\":\"string\",\"description\":\"The registrant's number of employees.\",\"example\":\"150\"},\"comments\":{\"type\":\"string\",\"description\":\"The registrant's questions and comments.\",\"example\":\"Looking forward to the discussion.\"},\"custom_questions\":{\"maxItems\":100,\"type\":\"array\",\"description\":\"The information about custom questions.\",\"items\":{\"type\":\"object\",\"properties\":{\"title\":{\"type\":\"string\",\"description\":\"The title of the custom question.\",\"example\":\"What do you hope to learn from this?\"},\"answer\":{\"type\":\"string\",\"description\":\"The custom question's response value.\",\"example\":\"Look forward to learning how you come up with new recipes and what other services you offer.\"}}}}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.vod_registration_created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"channel_id\\\": \\\"djsjgkjhfhk\\\",\\n      \\\"hub_id\\\": \\\"huCbxjabcdEFGHfp8uQ\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"registration_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"email\\\": \\\"email@zoom.us\\\",\\n      \\\"registration_date\\\": \\\"2025-05-24T22:30:00Z\\\",\\n      \\\"authentication_method\\\": \\\"no_auth\\\",\\n      \\\"join_link\\\": \\\"https://events.zoom.us/ejl/xyz\\\",\\n      \\\"first_name\\\": \\\"Jill\\\",\\n      \\\"last_name\\\": \\\"Chill\\\",\\n      \\\"address\\\": \\\"1800 Amphibious Blvd\\\",\\n      \\\"city\\\": \\\"Mountain View\\\",\\n      \\\"state\\\": \\\"CA\\\",\\n      \\\"zip\\\": \\\"94045\\\",\\n      \\\"country\\\": \\\"US\\\",\\n      \\\"phone\\\": \\\"5550100\\\",\\n      \\\"industry\\\": \\\"Food\\\",\\n      \\\"job_title\\\": \\\"Chef\\\",\\n      \\\"organization\\\": \\\"ABC Ltd.\\\",\\n      \\\"organization_size\\\": \\\"150\\\",\\n      \\\"comments\\\": \\\"Looking forward to the discussion.\\\",\\n      \\\"custom_questions\\\": [\\n        {\\n          \\\"title\\\": \\\"What do you hope to learn from this?\\\",\\n          \\\"answer\\\": \\\"Look forward to learning how you come up with new recipes and what other services you offer.\\\"\\n        }\\n      ]\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.vod_registration_created.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who created the event.\"},\"object\":{\"type\":\"object\",\"description\":\"The VOD channel registration description.\",\"required\":[\"channel_id\",\"host_id\",\"hub_id\",\"registration_id\",\"email\"],\"properties\":{\"channel_id\":{\"type\":\"string\",\"description\":\"The VOD channel ID.\"},\"hub_id\":{\"type\":\"string\",\"description\":\"The Zoom Events hub ID\",\"example\":\"234kj2h34kljgh23lkhj3\"},\"host_id\":{\"type\":\"string\",\"description\":\"The channel host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"registration_id\":{\"type\":\"string\",\"description\":\"The registration ID generated from the registration.\"},\"email\":{\"type\":\"string\",\"description\":\"The registrant's email.\"},\"registration_date\":{\"type\":\"string\",\"description\":\"The registrantion date in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ.\",\"format\":\"date-time\",\"example\":\"2025-05-25T22:30:00Z\"},\"authentication_method\":{\"type\":\"string\",\"description\":\"The VOD channel authentication method \\n * zoom_account Zoom account holder   * no_auth No authentication* email_opt registration using email OTP verification* \",\"example\":\"no_auth\",\"enum\":[\"zoom_account\",\"email_opt\",\"no_auth\"]},\"join_link\":{\"type\":\"string\",\"description\":\"The unique join link for the vod registrant.\",\"example\":\"https://events.zoom.us/ejl/xyz\"},\"first_name\":{\"type\":\"string\",\"description\":\"The registrant's first name.\",\"example\":\"Jill\"},\"last_name\":{\"type\":\"string\",\"description\":\"The registrant's last name.\",\"example\":\"Chill\"},\"address\":{\"type\":\"string\",\"description\":\"The registrant's address.\",\"example\":\"1800 Amphibious Blvd\"},\"city\":{\"type\":\"string\",\"description\":\"The registrant's city.\",\"example\":\"Mountain View\"},\"state\":{\"type\":\"string\",\"description\":\"The registrant's state or province.\",\"example\":\"CA\"},\"zip\":{\"type\":\"string\",\"description\":\"The registrant's ZIP or postal code.\",\"example\":\"94045\"},\"country\":{\"type\":\"string\",\"description\":\"The registrant's two-letter [country code](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#countries).\",\"example\":\"US\"},\"phone\":{\"type\":\"string\",\"description\":\"The registrant's phone number.\",\"example\":\"5550100\"},\"industry\":{\"type\":\"string\",\"description\":\"The registrant's industry.\",\"example\":\"Food\"},\"job_title\":{\"type\":\"string\",\"description\":\"The registrant's job title.\",\"example\":\"Chef\"},\"organization\":{\"type\":\"string\",\"description\":\"The registrant's organization.\",\"example\":\"ABC Ltd.\"},\"organization_size\":{\"type\":\"string\",\"description\":\"The registrant's number of employees.\",\"example\":\"150\"},\"comments\":{\"type\":\"string\",\"description\":\"The registrant's questions and comments.\",\"example\":\"Looking forward to the discussion.\"},\"custom_questions\":{\"maxItems\":100,\"type\":\"array\",\"description\":\"The information about custom questions.\",\"items\":{\"type\":\"object\",\"properties\":{\"title\":{\"type\":\"string\",\"description\":\"The title of the custom question.\",\"example\":\"What do you hope to learn from this?\"},\"answer\":{\"type\":\"string\",\"description\":\"The custom question's response value.\",\"example\":\"Look forward to learning how you come up with new recipes and what other services you offer.\"}}}}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.vod_registration_created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"channel_id\\\": \\\"djsjgkjhfhk\\\",\\n      \\\"hub_id\\\": \\\"huCbxjabcdEFGHfp8uQ\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"registration_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"email\\\": \\\"email@zoom.us\\\",\\n      \\\"registration_date\\\": \\\"2025-05-24T22:30:00Z\\\",\\n      \\\"authentication_method\\\": \\\"no_auth\\\",\\n      \\\"join_link\\\": \\\"https://events.zoom.us/ejl/xyz\\\",\\n      \\\"first_name\\\": \\\"Jill\\\",\\n      \\\"last_name\\\": \\\"Chill\\\",\\n      \\\"address\\\": \\\"1800 Amphibious Blvd\\\",\\n      \\\"city\\\": \\\"Mountain View\\\",\\n      \\\"state\\\": \\\"CA\\\",\\n      \\\"zip\\\": \\\"94045\\\",\\n      \\\"country\\\": \\\"US\\\",\\n      \\\"phone\\\": \\\"5550100\\\",\\n      \\\"industry\\\": \\\"Food\\\",\\n      \\\"job_title\\\": \\\"Chef\\\",\\n      \\\"organization\\\": \\\"ABC Ltd.\\\",\\n      \\\"organization_size\\\": \\\"150\\\",\\n      \\\"comments\\\": \\\"Looking forward to the discussion.\\\",\\n      \\\"custom_questions\\\": [\\n        {\\n          \\\"title\\\": \\\"What do you hope to learn from this?\\\",\\n          \\\"answer\\\": \\\"Look forward to learning how you come up with new recipes and what other services you offer.\\\"\\n        }\\n      ]\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.speaker_updated\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.speaker_updated\",\"requestBody\":{\"description\":\"# zoom_events.speaker_updated\\nThe **Speaker Updated**  event  is triggered every time a user in your account or your app updates speaker for an event in Zoom Events.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can update a speaker for an event.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **Speaker Updated** subscription enabled under the **Zoom Events** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_speakers:write:admin`,`zoom_events_speakers:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:speaker`,`zoom_events:read:speaker:admin`\\n\\n**Event type**: `zoom_events.speaker_updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.speaker_updated\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who updated the event speaker.\"},\"object\":{\"type\":\"object\",\"description\":\"Speaker Description.\",\"required\":[\"event_id\",\"host_id\",\"speaker_id\",\"name\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The zoom-events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"speaker_id\":{\"type\":\"string\",\"description\":\"The speaker id.\"},\"name\":{\"type\":\"string\",\"description\":\"The speaker's name.\"},\"email\":{\"type\":\"string\",\"description\":\"The speaker's email.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.speaker_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"speaker_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"name\\\": \\\"speaker1\\\",\\n      \\\"email\\\": \\\"speaker1-email@zoom.us\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.speaker_updated\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who updated the event speaker.\"},\"object\":{\"type\":\"object\",\"description\":\"Speaker Description.\",\"required\":[\"event_id\",\"host_id\",\"speaker_id\",\"name\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The zoom-events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"speaker_id\":{\"type\":\"string\",\"description\":\"The speaker id.\"},\"name\":{\"type\":\"string\",\"description\":\"The speaker's name.\"},\"email\":{\"type\":\"string\",\"description\":\"The speaker's email.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.speaker_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"speaker_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"name\\\": \\\"speaker1\\\",\\n      \\\"email\\\": \\\"speaker1-email@zoom.us\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.ticket_deleted\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.ticket_deleted\",\"requestBody\":{\"description\":\"# zoom_events.ticket_deleted\\nThe  **Ticket Deleted**  event is triggered when one of your app users or account users cancels the event registration.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events.  The host can delete a pre-registration for an event or a registrant can cancel their ticket.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **Ticket Deleted** subscription enabled under the **Zoom Events** event.\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_tickets:write:admin`,`zoom_events_tickets:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:ticket`,`zoom_events:read:ticket:admin`\\n\\n**Event type**: `zoom_events.ticket_deleted`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. zoom_events.ticket_deleted.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who created the event.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom Event description.\",\"required\":[\"event_id\",\"host_id\",\"ticket_id\",\"email\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The Zoom Events ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"ticket_id\":{\"type\":\"string\",\"description\":\"The ticket ID generated from the registration.\"},\"ticket_type_id\":{\"type\":\"string\",\"description\":\"The ticket type ID for which the user registers to.\"},\"external_ticket_id\":{\"type\":\"string\",\"description\":\"A set of unique alphanumeric characters that references the external ticket ID.\"},\"email\":{\"type\":\"string\",\"description\":\"The registrant's email.\"},\"ticket_role_type\":{\"type\":\"string\",\"description\":\"The ticket role type: \\n * normal The normal ticket \\n * alternative_host The alternative host ticket \\n * panelist The panelist ticket \\n * bind_meeting The bind meeting ticket \\n * speaker The speaker ticket \\n * interpreter The interpreter ticket \\n * sponsor The sponsor ticket \\n * host The host ticket \\n * moderator The moderator ticket \\n * expo_booth_owner The expo booth owner ticket \\n * attendee_without_order The attendee without order ticket \\n * pre_registration \u2014 The pre-registration ticket \\n * company_zoom_room The company zoom room ticket \\n * moderator_allhost The moderator all host ticket \\n * hub_host The hub host ticket \\n * hub_owner The hub owner ticket \\n * subscription The subscription ticket\",\"example\":\"normal\",\"enum\":[\"normal\",\"alternative_host\",\"panelist\",\"bind_meeting\",\"speaker\",\"subscription\",\"sponsor\",\"interpreter\",\"host\",\"moderator\",\"expo_booth_owner\",\"attendee_without_order\",\"pre_registration\",\"company_zoom_room\",\"moderator_allhost\",\"hub_host\",\"hub_owner\"]},\"authentication_method\":{\"type\":\"string\",\"description\":\"The event authentication method for the ticket: \\n * zoom_account Zoom account holder  * zoom_account_otp Zoom account holder with OTP  * corporate_idp Corporate IDP  * no_auth No authentication\",\"enum\":[\"zoom_account\",\"zoom_account_otp\",\"corporate_idp\",\"no_auth\"],\"example\":\"no_auth\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.ticket_deleted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"ticket_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"ticket_type_id\\\": \\\"pfalaof9s83lsLJD9u2d\\\",\\n      \\\"external_ticket_id\\\": \\\"pfalaof9s83lsLJD9u2d\\\",\\n      \\\"email\\\": \\\"email@zoom.us\\\",\\n      \\\"ticket_role_type\\\": \\\"normal\\\",\\n      \\\"authentication_method\\\": \\\"no_auth\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. zoom_events.ticket_deleted.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who created the event.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom Event description.\",\"required\":[\"event_id\",\"host_id\",\"ticket_id\",\"email\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The Zoom Events ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"ticket_id\":{\"type\":\"string\",\"description\":\"The ticket ID generated from the registration.\"},\"ticket_type_id\":{\"type\":\"string\",\"description\":\"The ticket type ID for which the user registers to.\"},\"external_ticket_id\":{\"type\":\"string\",\"description\":\"A set of unique alphanumeric characters that references the external ticket ID.\"},\"email\":{\"type\":\"string\",\"description\":\"The registrant's email.\"},\"ticket_role_type\":{\"type\":\"string\",\"description\":\"The ticket role type: \\n * normal The normal ticket \\n * alternative_host The alternative host ticket \\n * panelist The panelist ticket \\n * bind_meeting The bind meeting ticket \\n * speaker The speaker ticket \\n * interpreter The interpreter ticket \\n * sponsor The sponsor ticket \\n * host The host ticket \\n * moderator The moderator ticket \\n * expo_booth_owner The expo booth owner ticket \\n * attendee_without_order The attendee without order ticket \\n * pre_registration \u2014 The pre-registration ticket \\n * company_zoom_room The company zoom room ticket \\n * moderator_allhost The moderator all host ticket \\n * hub_host The hub host ticket \\n * hub_owner The hub owner ticket \\n * subscription The subscription ticket\",\"example\":\"normal\",\"enum\":[\"normal\",\"alternative_host\",\"panelist\",\"bind_meeting\",\"speaker\",\"subscription\",\"sponsor\",\"interpreter\",\"host\",\"moderator\",\"expo_booth_owner\",\"attendee_without_order\",\"pre_registration\",\"company_zoom_room\",\"moderator_allhost\",\"hub_host\",\"hub_owner\"]},\"authentication_method\":{\"type\":\"string\",\"description\":\"The event authentication method for the ticket: \\n * zoom_account Zoom account holder  * zoom_account_otp Zoom account holder with OTP  * corporate_idp Corporate IDP  * no_auth No authentication\",\"enum\":[\"zoom_account\",\"zoom_account_otp\",\"corporate_idp\",\"no_auth\"],\"example\":\"no_auth\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.ticket_deleted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"ticket_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"ticket_type_id\\\": \\\"pfalaof9s83lsLJD9u2d\\\",\\n      \\\"external_ticket_id\\\": \\\"pfalaof9s83lsLJD9u2d\\\",\\n      \\\"email\\\": \\\"email@zoom.us\\\",\\n      \\\"ticket_role_type\\\": \\\"normal\\\",\\n      \\\"authentication_method\\\": \\\"no_auth\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.speaker_deleted\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.speaker_deleted\",\"requestBody\":{\"description\":\"# zoom_events.speaker_deleted\\nThe **Speaker Deleted**  event  is triggered every time a user in your account or your app deletes speaker for an event in Zoom Events.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can delete a speaker for an event.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **Speaker Deleted** subscription enabled under the **Zoom Events** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_speakers:write:admin`,`zoom_events_speakers:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:speaker`,`zoom_events:read:speaker:admin`\\n\\n**Event type**: `zoom_events.speaker_deleted`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.speaker_deleted\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who deleted the event speaker.\"},\"object\":{\"type\":\"object\",\"description\":\"Speaker Description.\",\"required\":[\"event_id\",\"host_id\",\"speaker_id\",\"name\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The zoom-events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"speaker_id\":{\"type\":\"string\",\"description\":\"The speaker ID.\"},\"name\":{\"type\":\"string\",\"description\":\"The speaker's name.\"},\"email\":{\"type\":\"string\",\"description\":\"The speaker's email.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.speaker_deleted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"speaker_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"name\\\": \\\"speaker1\\\",\\n      \\\"email\\\": \\\"speaker1-email@zoom.us\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.speaker_deleted\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who deleted the event speaker.\"},\"object\":{\"type\":\"object\",\"description\":\"Speaker Description.\",\"required\":[\"event_id\",\"host_id\",\"speaker_id\",\"name\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The zoom-events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"speaker_id\":{\"type\":\"string\",\"description\":\"The speaker ID.\"},\"name\":{\"type\":\"string\",\"description\":\"The speaker's name.\"},\"email\":{\"type\":\"string\",\"description\":\"The speaker's email.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.speaker_deleted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"speaker_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"name\\\": \\\"speaker1\\\",\\n      \\\"email\\\": \\\"speaker1-email@zoom.us\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.ticket_type_deleted\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.ticket_type_deleted\",\"requestBody\":{\"description\":\"# zoom_events.ticket_type_deleted\\nThe **TicketType Deleted**  event  is triggered every time a user in your account or your app deletes ticketType for an event in Zoom Events.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can delete a ticketType for an event.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **TicketType Deleted** subscription enabled under the **Zoom Events** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_ticket_types:write:admin`,`zoom_events_ticket_types:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:list_ticket_types`,`zoom_events:read:list_ticket_types:admin`\\n\\n**Event type**: `zoom_events.ticket_type_deleted`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.ticket_type_deleted\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who deleted the ticketType.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the TicketType.\",\"required\":[\"event_id\",\"host_id\",\"ticket_type_id\",\"name\",\"start_time\",\"end_time\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The zoom-events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"ticket_type_id\":{\"type\":\"string\",\"description\":\"The ticketType ID for which the user registers to.\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the ticketType.\",\"example\":\"General Admission Ticket\"},\"description\":{\"type\":\"string\",\"description\":\"The description of the ticketType.\",\"example\":\"This is a General Admission ticket\"},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.ticket_type_deleted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"ticket_type_id\\\": \\\"pfalaof9s83lsLJD9u2d\\\",\\n      \\\"name\\\": \\\"General Admission Ticket\\\",\\n      \\\"description\\\": \\\"This is for General Admission\\\",\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"start_time\\\": \\\"2023-07-13T21:44:51Z\\\",\\n      \\\"end_time\\\": \\\"2023-07-13T23:00:51Z\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.ticket_type_deleted\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who deleted the ticketType.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the TicketType.\",\"required\":[\"event_id\",\"host_id\",\"ticket_type_id\",\"name\",\"start_time\",\"end_time\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The zoom-events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"ticket_type_id\":{\"type\":\"string\",\"description\":\"The ticketType ID for which the user registers to.\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the ticketType.\",\"example\":\"General Admission Ticket\"},\"description\":{\"type\":\"string\",\"description\":\"The description of the ticketType.\",\"example\":\"This is a General Admission ticket\"},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.ticket_type_deleted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"ticket_type_id\\\": \\\"pfalaof9s83lsLJD9u2d\\\",\\n      \\\"name\\\": \\\"General Admission Ticket\\\",\\n      \\\"description\\\": \\\"This is for General Admission\\\",\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"start_time\\\": \\\"2023-07-13T21:44:51Z\\\",\\n      \\\"end_time\\\": \\\"2023-07-13T23:00:51Z\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.session_updated\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.session_updated\",\"requestBody\":{\"description\":\"# zoom_events.session_updated\\n\\nThe **Session Updated** event is triggered every time a user in your account or your app, updates a session in Zoom Events.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host or a co-editor can update the session.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **Session Updated** subscription enabled under the **Zoom Events** event.\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_sessions:write:admin`,`zoom_events_sessions:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:session`,`zoom_events:read:session:admin`\\n\\n**Event type**: `zoom_events.session_updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. zoom_events.event.session_updated.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the event host who updated the session post event publishing.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the session.\",\"required\":[\"event_id\",\"host_id\",\"session_id\",\"name\",\"type\",\"start_time\",\"end_time\",\"timezone\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The Zoom Events ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"session_id\":{\"type\":\"string\",\"description\":\"The session ID.\",\"example\":\"4uzfv3JwTeyR5QpC3PXwMg\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the session.\",\"example\":\"Session 1\"},\"type\":{\"type\":\"integer\",\"description\":\"The session type. \\n * 0 \u2014 Meeting \\n * 2 \u2014 Webinar\",\"enum\":[0,2],\"example\":\"2\"},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-05-31T13:00:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-05-31T13:00:00Z\"},\"timezone\":{\"type\":\"string\",\"description\":\"The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones).\",\"example\":\"America/Los_Angeles\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.session_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"1234567890abcd\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"session_id\\\": \\\"fhhfkdhkhdgkhg\\\",\\n      \\\"name\\\": \\\"my session\\\",\\n      \\\"type\\\": 2,\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"start_time\\\": \\\"2022-05-31T13:00:00Z\\\",\\n      \\\"end_time\\\": \\\"2022-05-31T13:00:00Z\\\",\\n      \\\"timezone\\\": \\\"America/Los_Angeles\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. zoom_events.event.session_updated.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the event host who updated the session post event publishing.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the session.\",\"required\":[\"event_id\",\"host_id\",\"session_id\",\"name\",\"type\",\"start_time\",\"end_time\",\"timezone\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The Zoom Events ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"session_id\":{\"type\":\"string\",\"description\":\"The session ID.\",\"example\":\"4uzfv3JwTeyR5QpC3PXwMg\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the session.\",\"example\":\"Session 1\"},\"type\":{\"type\":\"integer\",\"description\":\"The session type. \\n * 0 \u2014 Meeting \\n * 2 \u2014 Webinar\",\"enum\":[0,2],\"example\":\"2\"},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-05-31T13:00:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-05-31T13:00:00Z\"},\"timezone\":{\"type\":\"string\",\"description\":\"The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones).\",\"example\":\"America/Los_Angeles\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.session_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"1234567890abcd\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"session_id\\\": \\\"fhhfkdhkhdgkhg\\\",\\n      \\\"name\\\": \\\"my session\\\",\\n      \\\"type\\\": 2,\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"start_time\\\": \\\"2022-05-31T13:00:00Z\\\",\\n      \\\"end_time\\\": \\\"2022-05-31T13:00:00Z\\\",\\n      \\\"timezone\\\": \\\"America/Los_Angeles\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.session_created\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.session_created\",\"requestBody\":{\"description\":\"# zoom_events.session_created\\nThe **Session Created** event is triggered every time a user in your account or your app, creates a session in Zoom Events.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host or a co-editor can add a session.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **Session Created** subscription enabled under the **Zoom Events** event.\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_sessions:write:admin`,`zoom_events_sessions:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:session`,`zoom_events:read:session:admin`\\n\\n**Event type**: `zoom_events.session_created`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. zoom_events.event.session_created.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who created the session post event publishing.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the session.\",\"required\":[\"event_id\",\"host_id\",\"session_id\",\"name\",\"type\",\"start_time\",\"end_time\",\"timezone\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The Zoom Events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"session_id\":{\"type\":\"string\",\"description\":\"The session ID.\",\"example\":\"4uzfv3JwTeyR5QpC3PXwMg\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the session.\",\"example\":\"Session 1\"},\"type\":{\"type\":\"integer\",\"description\":\"The session type. \\n * 0 \u2014 Meeting \\n * 2 \u2014 Webinar\",\"enum\":[0,2]},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-05-31T13:00:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-05-31T13:00:00Z\"},\"timezone\":{\"type\":\"string\",\"description\":\"The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones).\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.session_created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"1234567890abcd\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"session_id\\\": \\\"fhhfkdhkhdgkhg\\\",\\n      \\\"name\\\": \\\"my session\\\",\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"type\\\": 2,\\n      \\\"start_time\\\": \\\"2022-05-31T13:00:00Z\\\",\\n      \\\"end_time\\\": \\\"2022-05-31T13:00:00Z\\\",\\n      \\\"timezone\\\": \\\"America/Los_Angeles\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. zoom_events.event.session_created.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who created the session post event publishing.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the session.\",\"required\":[\"event_id\",\"host_id\",\"session_id\",\"name\",\"type\",\"start_time\",\"end_time\",\"timezone\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The Zoom Events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"session_id\":{\"type\":\"string\",\"description\":\"The session ID.\",\"example\":\"4uzfv3JwTeyR5QpC3PXwMg\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the session.\",\"example\":\"Session 1\"},\"type\":{\"type\":\"integer\",\"description\":\"The session type. \\n * 0 \u2014 Meeting \\n * 2 \u2014 Webinar\",\"enum\":[0,2]},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-05-31T13:00:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the session in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-05-31T13:00:00Z\"},\"timezone\":{\"type\":\"string\",\"description\":\"The session's [timezone](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#timezones).\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.session_created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"1234567890abcd\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"session_id\\\": \\\"fhhfkdhkhdgkhg\\\",\\n      \\\"name\\\": \\\"my session\\\",\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"type\\\": 2,\\n      \\\"start_time\\\": \\\"2022-05-31T13:00:00Z\\\",\\n      \\\"end_time\\\": \\\"2022-05-31T13:00:00Z\\\",\\n      \\\"timezone\\\": \\\"America/Los_Angeles\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.ticket_type_created\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.ticket_type_created\",\"requestBody\":{\"description\":\"# zoom_events.ticket_type_created\\nThe **TicketType Created**  event  is triggered every time a user in your account or your app creates ticketType for an event in Zoom Events.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can create a ticketType for an event.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **TicketType Created** subscription enabled under the **Zoom Events** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_ticket_types:write:admin`,`zoom_events_ticket_types:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:list_ticket_types`,`zoom_events:read:list_ticket_types:admin`\\n\\n**Event type**: `zoom_events.ticket_type_created`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.ticket_type_created\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who created the ticketType.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the ticketType.\",\"required\":[\"event_id\",\"host_id\",\"ticket_type_id\",\"name\",\"start_time\",\"end_time\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The zoom-events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"ticket_type_id\":{\"type\":\"string\",\"description\":\"The ticketType ID for which the user registers to.\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the ticketType.\",\"example\":\"General Admission Ticket\"},\"description\":{\"type\":\"string\",\"description\":\"The description of the ticketType.\",\"example\":\"This is a General Admission ticket\"},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.ticket_type_created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"ticket_type_id\\\": \\\"pfalaof9s83lsLJD9u2d\\\",\\n      \\\"name\\\": \\\"General Admission Ticket\\\",\\n      \\\"description\\\": \\\"This is for General Admission\\\",\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"start_time\\\": \\\"2023-07-13T21:44:51Z\\\",\\n      \\\"end_time\\\": \\\"2023-07-13T23:00:51Z\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.ticket_type_created\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who created the ticketType.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the ticketType.\",\"required\":[\"event_id\",\"host_id\",\"ticket_type_id\",\"name\",\"start_time\",\"end_time\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The zoom-events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"ticket_type_id\":{\"type\":\"string\",\"description\":\"The ticketType ID for which the user registers to.\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the ticketType.\",\"example\":\"General Admission Ticket\"},\"description\":{\"type\":\"string\",\"description\":\"The description of the ticketType.\",\"example\":\"This is a General Admission ticket\"},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the ticket sales in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.ticket_type_created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"ticket_type_id\\\": \\\"pfalaof9s83lsLJD9u2d\\\",\\n      \\\"name\\\": \\\"General Admission Ticket\\\",\\n      \\\"description\\\": \\\"This is for General Admission\\\",\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"start_time\\\": \\\"2023-07-13T21:44:51Z\\\",\\n      \\\"end_time\\\": \\\"2023-07-13T23:00:51Z\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.event_updated\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.event_updated\",\"requestBody\":{\"description\":\"# zoom_events.event_updated\\nThe **Event Updated** is triggered every time a user in your account or your app, makes changes to the event data in Zoom Events.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. A Zoom Events host or a co-editor can update an event.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **Event Updated** subscription enabled under the **Zoom Events** event.\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_basic:write:admin`,`zoom_events_basic:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:event`,`zoom_events:read:event:admin`\\n\\n**Event type**: `zoom_events.event_updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. zoom_events.event_updated.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who updated the event.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom Event description.\",\"required\":[\"event_id\",\"host_id\",\"name\",\"event_type\",\"start_time\",\"end_time\",\"timezone\"],\"properties\":{\"hub_id\":{\"type\":\"string\",\"description\":\"The ID of the event hub.\",\"example\":\"234kj2h34kljgh23lkhj3\"},\"event_id\":{\"type\":\"string\",\"description\":\"The ID of the event.\",\"example\":\"234kj2h34kljgh23lkhj3\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the event.\",\"example\":\"OpenAPI Conference Name\"},\"description\":{\"type\":\"string\",\"description\":\"The description of the event.\",\"example\":\"This event was created with the OpenAPI\"},\"event_type\":{\"type\":\"string\",\"description\":\"Event Types: \\n * CONFERENCE - Multi session event \\n * SIMPLE_EVENT - Single session event \\n * RECURRING - Recurring session event\",\"enum\":[\"CONFERENCE\",\"SIMPLE_EVENT\",\"RECURRING\"],\"example\":\"CONFERENCE\"},\"status\":{\"type\":\"string\",\"description\":\"The event status: \\n * PUBLISHED - Published event \\n * DRAFT - Draft event \\n * CANCELLED - Cancelled event\",\"enum\":[\"PUBLISHED\",\"DRAFT\",\"CANCELLED\"],\"example\":\"PUBLISHED\"},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"timezone\":{\"type\":\"string\",\"description\":\"The timezone of the event.\",\"example\":\"America/Los_Angeles\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.event_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"hub_id\\\": \\\"1234567890abcd\\\",\\n      \\\"event_id\\\": \\\"1234567890abcd\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"name\\\": \\\"My Event-1\\\",\\n      \\\"description\\\": \\\"This is a test event.\\\",\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"event_type\\\": \\\"CONFERENCE\\\",\\n      \\\"status\\\": \\\"DRAFT\\\",\\n      \\\"start_time\\\": \\\"2023-07-13T21:44:51Z\\\",\\n      \\\"timezone\\\": \\\"America/Los_Angeles\\\",\\n      \\\"end_time\\\": \\\"2023-07-13T23:00:51Z\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. zoom_events.event_updated.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who updated the event.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom Event description.\",\"required\":[\"event_id\",\"host_id\",\"name\",\"event_type\",\"start_time\",\"end_time\",\"timezone\"],\"properties\":{\"hub_id\":{\"type\":\"string\",\"description\":\"The ID of the event hub.\",\"example\":\"234kj2h34kljgh23lkhj3\"},\"event_id\":{\"type\":\"string\",\"description\":\"The ID of the event.\",\"example\":\"234kj2h34kljgh23lkhj3\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the event.\",\"example\":\"OpenAPI Conference Name\"},\"description\":{\"type\":\"string\",\"description\":\"The description of the event.\",\"example\":\"This event was created with the OpenAPI\"},\"event_type\":{\"type\":\"string\",\"description\":\"Event Types: \\n * CONFERENCE - Multi session event \\n * SIMPLE_EVENT - Single session event \\n * RECURRING - Recurring session event\",\"enum\":[\"CONFERENCE\",\"SIMPLE_EVENT\",\"RECURRING\"],\"example\":\"CONFERENCE\"},\"status\":{\"type\":\"string\",\"description\":\"The event status: \\n * PUBLISHED - Published event \\n * DRAFT - Draft event \\n * CANCELLED - Cancelled event\",\"enum\":[\"PUBLISHED\",\"DRAFT\",\"CANCELLED\"],\"example\":\"PUBLISHED\"},\"attendance_type\":{\"type\":\"string\",\"description\":\"The type of attendee experience for the event.\",\"enum\":[\"virtual\",\"in-person\",\"hybrid\"],\"example\":\"virtual\"},\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The end time of the event in UTC. The format should be yyyy-MM-ddTHH:mm:ssZ\",\"format\":\"date-time\",\"example\":\"2022-06-03T20:51:00Z\"},\"timezone\":{\"type\":\"string\",\"description\":\"The timezone of the event.\",\"example\":\"America/Los_Angeles\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.event_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"hub_id\\\": \\\"1234567890abcd\\\",\\n      \\\"event_id\\\": \\\"1234567890abcd\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"name\\\": \\\"My Event-1\\\",\\n      \\\"description\\\": \\\"This is a test event.\\\",\\n      \\\"attendance_type\\\": \\\"virtual\\\",\\n      \\\"event_type\\\": \\\"CONFERENCE\\\",\\n      \\\"status\\\": \\\"DRAFT\\\",\\n      \\\"start_time\\\": \\\"2023-07-13T21:44:51Z\\\",\\n      \\\"timezone\\\": \\\"America/Los_Angeles\\\",\\n      \\\"end_time\\\": \\\"2023-07-13T23:00:51Z\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.speaker_created\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.speaker_created\",\"requestBody\":{\"description\":\"# zoom_events.speaker_created\\nThe **Speaker Created**  event  is triggered every time a user in your account or your app creates speaker for an event in Zoom Events.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can create a speaker for an event.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **Speaker Created** subscription enabled under the **Zoom Events** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_speakers:write:admin`,`zoom_events_speakers:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:speaker`,`zoom_events:read:speaker:admin`\\n\\n**Event type**: `zoom_events.speaker_created`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.speaker_created\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who created the event speaker.\"},\"object\":{\"type\":\"object\",\"description\":\"Speaker Description.\",\"required\":[\"event_id\",\"host_id\",\"speaker_id\",\"name\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The Zoom Events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"speaker_id\":{\"type\":\"string\",\"description\":\"The speaker ID.\"},\"name\":{\"type\":\"string\",\"description\":\"The speaker's name.\"},\"email\":{\"type\":\"string\",\"description\":\"The speaker's email.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.speaker_created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"speaker_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"name\\\": \\\"speaker1\\\",\\n      \\\"email\\\": \\\"speaker1-email@zoom.us\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.speaker_created\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who created the event speaker.\"},\"object\":{\"type\":\"object\",\"description\":\"Speaker Description.\",\"required\":[\"event_id\",\"host_id\",\"speaker_id\",\"name\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The Zoom Events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"speaker_id\":{\"type\":\"string\",\"description\":\"The speaker ID.\"},\"name\":{\"type\":\"string\",\"description\":\"The speaker's name.\"},\"email\":{\"type\":\"string\",\"description\":\"The speaker's email.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.speaker_created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"speaker_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"name\\\": \\\"speaker1\\\",\\n      \\\"email\\\": \\\"speaker1-email@zoom.us\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.ticket_created\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.ticket_created\",\"requestBody\":{\"description\":\"# zoom_events.ticket_created\\n   The **Ticket Created**  event  is triggered when someone registers for an event created by one of your app users or account users.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events.  The host can create an event, set up a registration form, pre-register users or send out a registration email invitation. Users can also register themselves for an event.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **Ticket Created** subscription enabled under the **Zoom Events** event.\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_tickets:write:admin`,`zoom_events_tickets:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:ticket`,`zoom_events:read:ticket:admin`\\n\\n**Event type**: `zoom_events.ticket_created`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. zoom_events.ticket_created.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who created the event.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom Event description.\",\"required\":[\"event_id\",\"host_id\",\"ticket_id\",\"email\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The Zoom Events ID.\"},\"ticket_id\":{\"type\":\"string\",\"description\":\"The ticket ID generated from the registration.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"ticket_type_id\":{\"type\":\"string\",\"description\":\"The ticket type ID for which the user registers to.\"},\"external_ticket_id\":{\"type\":\"string\",\"description\":\"A set of unique alphanumeric characters that references the external ticket ID.\"},\"email\":{\"type\":\"string\",\"description\":\"The registrant's email.\"},\"ticket_role_type\":{\"type\":\"string\",\"description\":\"The ticket role type: \\n * normal The normal ticket \\n * alternative_host The alternative host ticket \\n * panelist The panelist ticket \\n * bind_meeting The bind meeting ticket \\n * speaker The speaker ticket \\n * interpreter The interpreter ticket \\n * sponsor The sponsor ticket \\n * host The host ticket \\n * moderator The moderator ticket \\n * expo_booth_owner The expo booth owner ticket \\n * attendee_without_order The attendee without order ticket \\n * pre_registration \u2014 The pre-registration ticket \\n * company_zoom_room The company zoom room ticket \\n * moderator_allhost The moderator all host ticket \\n * hub_host The hub host ticket \\n * hub_owner The hub owner ticket \\n * subscription The subscription ticket\",\"example\":\"normal\",\"enum\":[\"normal\",\"alternative_host\",\"panelist\",\"bind_meeting\",\"speaker\",\"subscription\",\"sponsor\",\"interpreter\",\"host\",\"moderator\",\"expo_booth_owner\",\"attendee_without_order\",\"pre_registration\",\"company_zoom_room\",\"moderator_allhost\",\"hub_host\",\"hub_owner\"]},\"authentication_method\":{\"type\":\"string\",\"description\":\"The event authentication method for the ticket: \\n * zoom_account Zoom account holder  * zoom_account_otp Zoom account holder with OTP  * corporate_idp Corporate IDP  * no_auth No authentication\",\"enum\":[\"zoom_account\",\"zoom_account_otp\",\"corporate_idp\",\"no_auth\"],\"example\":\"no_auth\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.ticket_created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"djsjgkjhfhk\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"ticket_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"ticket_type_id\\\": \\\"pfalaof9s83lsLJD9u2d\\\",\\n      \\\"external_ticket_id\\\": \\\"abc1231245-adf\\\",\\n      \\\"ticket_role_type\\\": \\\"normal\\\",\\n      \\\"email\\\": \\\"email@zoom.us\\\",\\n      \\\"authentication_method\\\": \\\"no_auth\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event. zoom_events.ticket_created.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who created the event.\"},\"object\":{\"type\":\"object\",\"description\":\"The Zoom Event description.\",\"required\":[\"event_id\",\"host_id\",\"ticket_id\",\"email\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The Zoom Events ID.\"},\"ticket_id\":{\"type\":\"string\",\"description\":\"The ticket ID generated from the registration.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"ticket_type_id\":{\"type\":\"string\",\"description\":\"The ticket type ID for which the user registers to.\"},\"external_ticket_id\":{\"type\":\"string\",\"description\":\"A set of unique alphanumeric characters that references the external ticket ID.\"},\"email\":{\"type\":\"string\",\"description\":\"The registrant's email.\"},\"ticket_role_type\":{\"type\":\"string\",\"description\":\"The ticket role type: \\n * normal The normal ticket \\n * alternative_host The alternative host ticket \\n * panelist The panelist ticket \\n * bind_meeting The bind meeting ticket \\n * speaker The speaker ticket \\n * interpreter The interpreter ticket \\n * sponsor The sponsor ticket \\n * host The host ticket \\n * moderator The moderator ticket \\n * expo_booth_owner The expo booth owner ticket \\n * attendee_without_order The attendee without order ticket \\n * pre_registration \u2014 The pre-registration ticket \\n * company_zoom_room The company zoom room ticket \\n * moderator_allhost The moderator all host ticket \\n * hub_host The hub host ticket \\n * hub_owner The hub owner ticket \\n * subscription The subscription ticket\",\"example\":\"normal\",\"enum\":[\"normal\",\"alternative_host\",\"panelist\",\"bind_meeting\",\"speaker\",\"subscription\",\"sponsor\",\"interpreter\",\"host\",\"moderator\",\"expo_booth_owner\",\"attendee_without_order\",\"pre_registration\",\"company_zoom_room\",\"moderator_allhost\",\"hub_host\",\"hub_owner\"]},\"authentication_method\":{\"type\":\"string\",\"description\":\"The event authentication method for the ticket: \\n * zoom_account Zoom account holder  * zoom_account_otp Zoom account holder with OTP  * corporate_idp Corporate IDP  * no_auth No authentication\",\"enum\":[\"zoom_account\",\"zoom_account_otp\",\"corporate_idp\",\"no_auth\"],\"example\":\"no_auth\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.ticket_created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"djsjgkjhfhk\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"ticket_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"ticket_type_id\\\": \\\"pfalaof9s83lsLJD9u2d\\\",\\n      \\\"external_ticket_id\\\": \\\"abc1231245-adf\\\",\\n      \\\"ticket_role_type\\\": \\\"normal\\\",\\n      \\\"email\\\": \\\"email@zoom.us\\\",\\n      \\\"authentication_method\\\": \\\"no_auth\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoom_events.exhibitor_deleted\":{\"post\":{\"tags\":[\"zoom_events\"],\"operationId\":\"zoom_events.exhibitor_deleted\",\"requestBody\":{\"description\":\"# zoom_events.exhibitor_deleted\\nThe **Exhibitor Deleted**  event  is triggered every time a user in your account or your app deletes exhibitor for an event in Zoom Events.\\n\\nZoom user with a [Zoom Webinars Plus or Events license](https://explore.zoom.us/en/products/event-platform/) have permissions for creating and managing Zoom Events. The host can delete an exhibitor for an event.\\n\\n## Prerequisites\\n\\n* Pro or a higher plan with Zoom Webinars Plus or Events license.\\n* Enable **Event Subscription** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n    * A valid **Event Notification Endpoint URL**.\\n    * The **Exhibitor Deleted** subscription enabled under the **Zoom Events** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events_exhibitors:write:admin`,`zoom_events_exhibitors:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_events:read:exhibitor`,`zoom_events:read:exhibitor:admin`\\n\\n**Event type**: `zoom_events.exhibitor_deleted`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.exhibitor_deleted\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who deleted the event exhibitor.\"},\"object\":{\"type\":\"object\",\"description\":\"Exhibitor Description.\",\"required\":[\"event_id\",\"host_id\",\"exhibitor_id\",\"name\",\"contact_name\",\"contact_email\",\"is_sponsor\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The zoom-events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"exhibitor_id\":{\"type\":\"string\",\"description\":\"The exhibitors's ID.\"},\"name\":{\"type\":\"string\",\"description\":\"The exhibitors's name.\"},\"description\":{\"type\":\"string\",\"description\":\"The exhibitor's description.\"},\"is_sponsor\":{\"type\":\"boolean\",\"description\":\"Whether the exhibitor is a sponsor.\"},\"tier_id\":{\"type\":\"string\",\"description\":\"The sponsor tier ID for a particular event. This field only applies to a sponsor.\"},\"contact_name\":{\"type\":\"string\",\"description\":\"The contact's full name.\"},\"contact_email\":{\"type\":\"string\",\"description\":\"The contact's email address.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.exhibitor_deleted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"exhibitor_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"name\\\": \\\"exhibitor1\\\",\\n      \\\"description\\\": \\\"Description of the exhibitor\\\",\\n      \\\"is_sponsor\\\": true,\\n      \\\"tier_id\\\": \\\"4fgRIOXjQGmoH6raAzJf8g\\\",\\n      \\\"contact_name\\\": \\\"John Doe\\\",\\n      \\\"contact_email\\\": \\\"exhibitor1-email@zoom.us\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event, zoom_events.exhibitor_deleted\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the host who deleted the event exhibitor.\"},\"object\":{\"type\":\"object\",\"description\":\"Exhibitor Description.\",\"required\":[\"event_id\",\"host_id\",\"exhibitor_id\",\"name\",\"contact_name\",\"contact_email\",\"is_sponsor\"],\"properties\":{\"event_id\":{\"type\":\"string\",\"description\":\"The zoom-events event ID.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The event host(creator)'s user ID.\",\"example\":\"x1yCzABCDEfg23HiJKl4mN\"},\"exhibitor_id\":{\"type\":\"string\",\"description\":\"The exhibitors's ID.\"},\"name\":{\"type\":\"string\",\"description\":\"The exhibitors's name.\"},\"description\":{\"type\":\"string\",\"description\":\"The exhibitor's description.\"},\"is_sponsor\":{\"type\":\"boolean\",\"description\":\"Whether the exhibitor is a sponsor.\"},\"tier_id\":{\"type\":\"string\",\"description\":\"The sponsor tier ID for a particular event. This field only applies to a sponsor.\"},\"contact_name\":{\"type\":\"string\",\"description\":\"The contact's full name.\"},\"contact_email\":{\"type\":\"string\",\"description\":\"The contact's email address.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoom_events.exhibitor_deleted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"event_id\\\": \\\"qwerq1243124\\\",\\n      \\\"host_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n      \\\"exhibitor_id\\\": \\\"iso9Dllddp39dagjLj9j\\\",\\n      \\\"name\\\": \\\"exhibitor1\\\",\\n      \\\"description\\\": \\\"Description of the exhibitor\\\",\\n      \\\"is_sponsor\\\": true,\\n      \\\"tier_id\\\": \\\"4fgRIOXjQGmoH6raAzJf8g\\\",\\n      \\\"contact_name\\\": \\\"John Doe\\\",\\n      \\\"contact_email\\\": \\\"exhibitor1-email@zoom.us\\\"\\n    }\\n  }\\n}\"}}}}}}}}},\"servers\":[{\"url\":\"https://{your-endpoint-url}\"}]}",
    "frontmatter": {
      "title": "Events Webhooks",
      "keywords": "zoom, marketplace, webhook, event, create, update, delete, real-time, data, summary, qos, quality, service",
      "description": "The Webinars Plus & Events webhooks allow developers to receive events for [Webinars Plus & Events](https://developers.zoom.us/docs/zoom-events/) features.",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
