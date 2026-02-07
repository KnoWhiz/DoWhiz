# Rooms Webhooks

- Source URL: https://developers.zoom.us/docs/api/rooms/events/
- Snapshot path: docs/api/rooms/events
- Generated (UTC): 2026-02-07T22:40:11.572681+00:00

## Frontmatter

```json
{
  "title": "Rooms Webhooks",
  "keywords": "zoom, marketplace, chatbot, chat, bot, webhook, event, zoom rooms, interactive message, button, message, action",
  "description": "The Zoom Rooms webhooks allow developers to receive events for [Zoom Rooms](https://developers.zoom.us/docs/zoom-rooms/) features.",
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
    "title": "Rooms",
    "description": "The Zoom Rooms webhooks allow developers to receive events for [Zoom Rooms](https://developers.zoom.us/docs/zoom-rooms/) features.",
    "version": "1.0.0"
  },
  "paths": {
    "visitor_management.checked_out": {
      "post": {
        "tags": [
          "visitor_management"
        ],
        "operationId": "visitor_management.checked_out",
        "requestBody": {
          "description": "# visitor_management.checked_out\nThe **Visitor Management Checked Out** event is triggered when a visitor checks out.\n\n## Prerequisites\n\n* The [**Visitor Management**](https://support.zoom.com/hc/en?id=zm_kb&sysparm_article=KB0057564) feature enabled by your organization.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Visitor Management Checked Out** subscription enabled under the **Visitor Management** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:visitor_checked_out:admin`\n\n**Event type**: `visitor_management.checked_out`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The host user account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the host."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the host."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the checked out invitation.",
                        "required": [
                          "visitor_email",
                          "visitor_name",
                          "start_time",
                          "end_time",
                          "invite_location_id",
                          "host_id",
                          "host_name",
                          "host_email"
                        ],
                        "properties": {
                          "visitor_email": {
                            "type": "string",
                            "description": "The email address of the invited visitor."
                          },
                          "visitor_name": {
                            "type": "string",
                            "description": "The name of the invited visitor."
                          },
                          "start_time": {
                            "type": "long",
                            "description": "The epoch timestamp representing the start time of the invitation."
                          },
                          "end_time": {
                            "type": "long",
                            "description": "The epoch timestamp representing end time of the invitation."
                          },
                          "invite_location_id": {
                            "type": "string",
                            "description": "The location ID of the location to which the visitor is invited to."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The user ID of the invitation host."
                          },
                          "host_name": {
                            "type": "string",
                            "description": "The name of the invitation host."
                          },
                          "host_email": {
                            "type": "string",
                            "description": "The email address of the invitation host."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"visitor_management.checked_out\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"start_time\": 1710153000000,\n      \"end_time\": 1710172799999,\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The host user account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the host."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the host."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the checked out invitation.",
                          "required": [
                            "visitor_email",
                            "visitor_name",
                            "start_time",
                            "end_time",
                            "invite_location_id",
                            "host_id",
                            "host_name",
                            "host_email"
                          ],
                          "properties": {
                            "visitor_email": {
                              "type": "string",
                              "description": "The email address of the invited visitor."
                            },
                            "visitor_name": {
                              "type": "string",
                              "description": "The name of the invited visitor."
                            },
                            "start_time": {
                              "type": "long",
                              "description": "The epoch timestamp representing the start time of the invitation."
                            },
                            "end_time": {
                              "type": "long",
                              "description": "The epoch timestamp representing end time of the invitation."
                            },
                            "invite_location_id": {
                              "type": "string",
                              "description": "The location ID of the location to which the visitor is invited to."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The user ID of the invitation host."
                            },
                            "host_name": {
                              "type": "string",
                              "description": "The name of the invitation host."
                            },
                            "host_email": {
                              "type": "string",
                              "description": "The email address of the invitation host."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"visitor_management.checked_out\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"start_time\": 1710153000000,\n      \"end_time\": 1710172799999,\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.user_unassigned": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.user_unassigned",
        "requestBody": {
          "description": "# workspace.user_unassigned\nThe **Workspace User Unassigned** event is triggered every time a user is unassigned from a Workspace in your account. Admins and account owners can unassign a user from Workspace in you account. The Workspace can be unassign users using the [Delete a desk assignment API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/Deleteadeskassignment) or by deleting the Workspace' assignment using the Zoom Web Portal itself.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace User Unassigned** subscription enabled under the **Workspace** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:assignment`,`workspace:read:assignment:admin`\n\n**Event type**: `workspace.user_unassigned`\n",
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
                    "type": "string",
                    "format": "date-time",
                    "description": "A timestamp when the event occurred."
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that updated the assignment."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that updated the assignment."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the user unassigned when unassign a user from a workspace.",
                        "required": [
                          "workspace_id",
                          "workspace_name",
                          "account_id",
                          "user_id",
                          "assign_type"
                        ],
                        "properties": {
                          "workspace_id": {
                            "type": "string",
                            "description": "The workspace's ID."
                          },
                          "workspace_name": {
                            "type": "string",
                            "description": "The workspace's name."
                          },
                          "account_id": {
                            "type": "string",
                            "description": "The belong's account id of assigned user."
                          },
                          "user_id": {
                            "type": "string",
                            "description": "The id of assigned user."
                          },
                          "assign_type": {
                            "type": "string",
                            "description": "The type of assign before unassign. ",
                            "enum": [
                              "permanent"
                            ]
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"workspace.user_unassigned\",\n  \"event_ts\": 1707848572310,\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"operator\": \"vic-devint@zoom.us\",\n    \"object\": {\n      \"workspace_id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"account_id\": \"bhmIXpdYRcCK6BB3VhMq6w\",\n      \"user_id\": \"oyLyxW48QeC3uBBFY2CF3A\",\n      \"assign_type\": \"permanent\"\n    }\n  }\n}"
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
                      "type": "string",
                      "format": "date-time",
                      "description": "A timestamp when the event occurred."
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that updated the assignment."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that updated the assignment."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the user unassigned when unassign a user from a workspace.",
                          "required": [
                            "workspace_id",
                            "workspace_name",
                            "account_id",
                            "user_id",
                            "assign_type"
                          ],
                          "properties": {
                            "workspace_id": {
                              "type": "string",
                              "description": "The workspace's ID."
                            },
                            "workspace_name": {
                              "type": "string",
                              "description": "The workspace's name."
                            },
                            "account_id": {
                              "type": "string",
                              "description": "The belong's account id of assigned user."
                            },
                            "user_id": {
                              "type": "string",
                              "description": "The id of assigned user."
                            },
                            "assign_type": {
                              "type": "string",
                              "description": "The type of assign before unassign. ",
                              "enum": [
                                "permanent"
                              ]
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"workspace.user_unassigned\",\n  \"event_ts\": 1707848572310,\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"operator\": \"vic-devint@zoom.us\",\n    \"object\": {\n      \"workspace_id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"account_id\": \"bhmIXpdYRcCK6BB3VhMq6w\",\n      \"user_id\": \"oyLyxW48QeC3uBBFY2CF3A\",\n      \"assign_type\": \"permanent\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.reservation_questionnaire_response_created": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.reservation_questionnaire_response_created",
        "requestBody": {
          "description": "# workspace.reservation_questionnaire_response_created\nThe **Workspace reservation questionnaire created** event is triggered every time when a questionnaire is submitted as a part of workspace reservation. Admin and Users can create a workspace reservation and submit a questionnaire during the reservation creation process.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace reservation questionnaire created** subscription enabled under the **Workspace** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:workspace_reservation_questionnaire_response`,`workspace:read:workspace_reservation_questionnaire_response:admin`\n\n**Event type**: `workspace.reservation_questionnaire_response_created`\n",
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
                    "type": "string",
                    "format": "date-time",
                    "description": "A timestamp when the event occurred."
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that completed the questionnaire."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that completed the questionnaire."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the questionnaire when creating a reservation for a workspace.",
                        "required": [
                          "start_time",
                          "event_id",
                          "user_name",
                          "desk_id",
                          "workspace_name",
                          "location_id",
                          "questionnaire_response"
                        ],
                        "properties": {
                          "start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The start time of the workspace reservation."
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The event ID."
                          },
                          "user_name": {
                            "type": "string",
                            "description": "The user name."
                          },
                          "desk_id": {
                            "type": "string",
                            "description": "The desk ID."
                          },
                          "workspace_name": {
                            "type": "string",
                            "description": "The name of the workspace."
                          },
                          "location_id": {
                            "type": "string",
                            "description": "The location ID."
                          },
                          "questionnaire_response": {
                            "type": "array",
                            "description": "List of questionnaires.",
                            "items": {
                              "type": "object",
                              "required": [
                                "id",
                                "title",
                                "subject",
                                "list"
                              ],
                              "properties": {
                                "id": {
                                  "type": "string",
                                  "description": "The view ID."
                                },
                                "title": {
                                  "type": "string",
                                  "description": "The title of the questionnaire."
                                },
                                "subject": {
                                  "type": "string",
                                  "description": "The subject of the questionnaire."
                                },
                                "list": {
                                  "type": "array",
                                  "description": "List of questions in the questionnaire.",
                                  "items": {
                                    "type": "object",
                                    "required": [
                                      "question_id",
                                      "title",
                                      "type",
                                      "options",
                                      "reply_id"
                                    ],
                                    "properties": {
                                      "question_id": {
                                        "type": "string",
                                        "description": "The question ID."
                                      },
                                      "title": {
                                        "type": "string",
                                        "description": "The title of the question."
                                      },
                                      "type": {
                                        "type": "string",
                                        "description": "The type of the question (single-choice, multiple-choice, open-text)."
                                      },
                                      "reply_id": {
                                        "type": "string",
                                        "description": "The reply ID."
                                      },
                                      "options": {
                                        "type": "array",
                                        "description": "List of options for multiple-choice questions.",
                                        "items": {
                                          "type": "object",
                                          "required": [
                                            "option_id",
                                            "text"
                                          ],
                                          "properties": {
                                            "option_id": {
                                              "type": "string",
                                              "description": "The option ID."
                                            },
                                            "text": {
                                              "type": "string",
                                              "description": "The text of the option."
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
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"workspace.reservation_questionnaire_response_created\",\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator\": \"yuki-devint@zoom.us\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"object\": {\n      \"start_time\": \"2024-02-14T03:50:00Z\",\n      \"event_id\": \"dxpwbc2fi8tzz7d012rwx9u8c0\",\n      \"user_name\": \"Simash pokh\",\n      \"desk_id\": \"SJjU1CcGSs2L7odn6PanQw\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"questionnaire_response\": [\n        {\n          \"id\": \"Fl75C43YTmS_IpMzYLipJw\",\n          \"title\": \"Simash Reservation Questionnaire\",\n          \"list\": [\n            {\n              \"question_id\": \"0ds1NFeDQ1WfWlgVK-7OMQ\",\n              \"title\": \"do you like AI ?\",\n              \"type\": \"single\",\n              \"options\": [\n                {\n                  \"option_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\",\n                  \"text\": \"Yes\"\n                },\n                {\n                  \"option_id\": \"1S1hZzFGTwW9Llxdr2vzyw\",\n                  \"text\": \"No\"\n                }\n              ],\n              \"reply_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\"\n            },\n            {\n              \"question_id\": \"2XMaJFwNSr-BJpTfo6rRAQ\",\n              \"title\": \"what is your fav food ? \",\n              \"type\": \"multiple\",\n              \"options\": [\n                {\n                  \"option_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ\",\n                  \"text\": \"momo\"\n                },\n                {\n                  \"option_id\": \"dpv4EZ7QQUqRhBmYj4_b7w\",\n                  \"text\": \"sea food\"\n                },\n                {\n                  \"option_id\": \"__DjlrV7TeyCSZVmCWAnng\",\n                  \"text\": \"rice\"\n                }\n              ],\n              \"reply_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ,dpv4EZ7QQUqRhBmYj4_b7w\"\n            }\n          ],\n          \"subject\": \"reservation\"\n        }\n      ]\n    }\n  },\n  \"event_ts\": 1707848572310\n}"
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
                      "type": "string",
                      "format": "date-time",
                      "description": "A timestamp when the event occurred."
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that completed the questionnaire."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that completed the questionnaire."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the questionnaire when creating a reservation for a workspace.",
                          "required": [
                            "start_time",
                            "event_id",
                            "user_name",
                            "desk_id",
                            "workspace_name",
                            "location_id",
                            "questionnaire_response"
                          ],
                          "properties": {
                            "start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The start time of the workspace reservation."
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The event ID."
                            },
                            "user_name": {
                              "type": "string",
                              "description": "The user name."
                            },
                            "desk_id": {
                              "type": "string",
                              "description": "The desk ID."
                            },
                            "workspace_name": {
                              "type": "string",
                              "description": "The name of the workspace."
                            },
                            "location_id": {
                              "type": "string",
                              "description": "The location ID."
                            },
                            "questionnaire_response": {
                              "type": "array",
                              "description": "List of questionnaires.",
                              "items": {
                                "type": "object",
                                "required": [
                                  "id",
                                  "title",
                                  "subject",
                                  "list"
                                ],
                                "properties": {
                                  "id": {
                                    "type": "string",
                                    "description": "The view ID."
                                  },
                                  "title": {
                                    "type": "string",
                                    "description": "The title of the questionnaire."
                                  },
                                  "subject": {
                                    "type": "string",
                                    "description": "The subject of the questionnaire."
                                  },
                                  "list": {
                                    "type": "array",
                                    "description": "List of questions in the questionnaire.",
                                    "items": {
                                      "type": "object",
                                      "required": [
                                        "question_id",
                                        "title",
                                        "type",
                                        "options",
                                        "reply_id"
                                      ],
                                      "properties": {
                                        "question_id": {
                                          "type": "string",
                                          "description": "The question ID."
                                        },
                                        "title": {
                                          "type": "string",
                                          "description": "The title of the question."
                                        },
                                        "type": {
                                          "type": "string",
                                          "description": "The type of the question (single-choice, multiple-choice, open-text)."
                                        },
                                        "reply_id": {
                                          "type": "string",
                                          "description": "The reply ID."
                                        },
                                        "options": {
                                          "type": "array",
                                          "description": "List of options for multiple-choice questions.",
                                          "items": {
                                            "type": "object",
                                            "required": [
                                              "option_id",
                                              "text"
                                            ],
                                            "properties": {
                                              "option_id": {
                                                "type": "string",
                                                "description": "The option ID."
                                              },
                                              "text": {
                                                "type": "string",
                                                "description": "The text of the option."
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
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"workspace.reservation_questionnaire_response_created\",\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator\": \"yuki-devint@zoom.us\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"object\": {\n      \"start_time\": \"2024-02-14T03:50:00Z\",\n      \"event_id\": \"dxpwbc2fi8tzz7d012rwx9u8c0\",\n      \"user_name\": \"Simash pokh\",\n      \"desk_id\": \"SJjU1CcGSs2L7odn6PanQw\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"questionnaire_response\": [\n        {\n          \"id\": \"Fl75C43YTmS_IpMzYLipJw\",\n          \"title\": \"Simash Reservation Questionnaire\",\n          \"list\": [\n            {\n              \"question_id\": \"0ds1NFeDQ1WfWlgVK-7OMQ\",\n              \"title\": \"do you like AI ?\",\n              \"type\": \"single\",\n              \"options\": [\n                {\n                  \"option_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\",\n                  \"text\": \"Yes\"\n                },\n                {\n                  \"option_id\": \"1S1hZzFGTwW9Llxdr2vzyw\",\n                  \"text\": \"No\"\n                }\n              ],\n              \"reply_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\"\n            },\n            {\n              \"question_id\": \"2XMaJFwNSr-BJpTfo6rRAQ\",\n              \"title\": \"what is your fav food ? \",\n              \"type\": \"multiple\",\n              \"options\": [\n                {\n                  \"option_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ\",\n                  \"text\": \"momo\"\n                },\n                {\n                  \"option_id\": \"dpv4EZ7QQUqRhBmYj4_b7w\",\n                  \"text\": \"sea food\"\n                },\n                {\n                  \"option_id\": \"__DjlrV7TeyCSZVmCWAnng\",\n                  \"text\": \"rice\"\n                }\n              ],\n              \"reply_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ,dpv4EZ7QQUqRhBmYj4_b7w\"\n            }\n          ],\n          \"subject\": \"reservation\"\n        }\n      ]\n    }\n  },\n  \"event_ts\": 1707848572310\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.sensor_data": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.sensor_data",
        "requestBody": {
          "description": "# zoomroom.sensor_data\n\nThe **Zoom Room Sensor Data** event triggers every five minutes when a Zoom Room device sensor reports data.\n\n## Prerequisites\n\n* A Business or Enterprise plan.\n* The ability to detect the room air conditions, a feature available with Zoom Rooms version 5.5.0 or later. Not supported for Mac and Windows Zoom Rooms or Windows Controller.\n* Count the number of people in room: available on Zoom Rooms clients versions 5.5.0 (Windows), 5.5.0 (Mac), 5.6.6 (Android), 5.5.0 (iOS Controller), 5.5.0 (Android Controller) or later.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Room Sensor Data** subscription enabled under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:sensor_data:admin`\n\n**Event type**: `zoomroom.sensor_data`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "id",
                          "room_name",
                          "device_id",
                          "sensor_data"
                        ],
                        "description": "Information about the Zoom Room.",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Room ID."
                          },
                          "room_name": {
                            "type": "string",
                            "description": "The Zoom Room name."
                          },
                          "device_id": {
                            "type": "string",
                            "description": "The device ID."
                          },
                          "sensor_data": {
                            "type": "array",
                            "description": "Information about the sensor data.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "date_time": {
                                  "description": "The time when the sensor data was reported.",
                                  "type": "string",
                                  "format": "datetime",
                                  "example": "2022-06-19T00:00:00Z"
                                },
                                "sensor_type": {
                                  "type": "string",
                                  "enum": [
                                    "CO2",
                                    "TEMPERATURE",
                                    "REAL_TIME_PEOPLE_COUNT",
                                    "HUMIDITY",
                                    "VOC",
                                    "AQI",
                                    "PM25",
                                    "PM10",
                                    "AIR_PRESSURE"
                                  ],
                                  "description": "The sensor type."
                                },
                                "sensor_value": {
                                  "type": "string",
                                  "description": "The sensor value."
                                }
                              },
                              "required": [
                                "date_time",
                                "sensor_type",
                                "sensor_value"
                              ]
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
                  "value": "{\n\t\"event\": \"zoomroom.sensor_data\",\n\t\"event_ts\": 1626230691572,\n\t\"payload\": {\n\t\t\"account_id\": \"AAAAAABBBB\",\n\t\t\"object\": {\n\t\t\t\"id\": \"abcD3ojfdbjfg\",\n\t\t\t\"room_name\": \"My Zoom Room\",\n\t\t\t\"device_id\": \"NiVvY1NWpE2nulNrhVjgU4jD0swziVrXBbaFZyC3u+o=\",\n\t\t\t\"sensor_data\": [{\n\t\t\t\t\"date_time\": \"2022-06-19T00:00:00Z\",\n\t\t\t\t\"sensor_type\": \"REAL_TIME_PEOPLE_COUNT\",\n\t\t\t\t\"sensor_value\": \"20\"\n\t\t\t}]\n\t\t}\n\t}\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "id",
                            "room_name",
                            "device_id",
                            "sensor_data"
                          ],
                          "description": "Information about the Zoom Room.",
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Room ID."
                            },
                            "room_name": {
                              "type": "string",
                              "description": "The Zoom Room name."
                            },
                            "device_id": {
                              "type": "string",
                              "description": "The device ID."
                            },
                            "sensor_data": {
                              "type": "array",
                              "description": "Information about the sensor data.",
                              "items": {
                                "type": "object",
                                "properties": {
                                  "date_time": {
                                    "description": "The time when the sensor data was reported.",
                                    "type": "string",
                                    "format": "datetime",
                                    "example": "2022-06-19T00:00:00Z"
                                  },
                                  "sensor_type": {
                                    "type": "string",
                                    "enum": [
                                      "CO2",
                                      "TEMPERATURE",
                                      "REAL_TIME_PEOPLE_COUNT",
                                      "HUMIDITY",
                                      "VOC",
                                      "AQI",
                                      "PM25",
                                      "PM10",
                                      "AIR_PRESSURE"
                                    ],
                                    "description": "The sensor type."
                                  },
                                  "sensor_value": {
                                    "type": "string",
                                    "description": "The sensor value."
                                  }
                                },
                                "required": [
                                  "date_time",
                                  "sensor_type",
                                  "sensor_value"
                                ]
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
                    "value": "{\n\t\"event\": \"zoomroom.sensor_data\",\n\t\"event_ts\": 1626230691572,\n\t\"payload\": {\n\t\t\"account_id\": \"AAAAAABBBB\",\n\t\t\"object\": {\n\t\t\t\"id\": \"abcD3ojfdbjfg\",\n\t\t\t\"room_name\": \"My Zoom Room\",\n\t\t\t\"device_id\": \"NiVvY1NWpE2nulNrhVjgU4jD0swziVrXBbaFZyC3u+o=\",\n\t\t\t\"sensor_data\": [{\n\t\t\t\t\"date_time\": \"2022-06-19T00:00:00Z\",\n\t\t\t\t\"sensor_type\": \"REAL_TIME_PEOPLE_COUNT\",\n\t\t\t\t\"sensor_value\": \"20\"\n\t\t\t}]\n\t\t}\n\t}\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "visitor_management.invitation_deleted": {
      "post": {
        "tags": [
          "visitor_management"
        ],
        "operationId": "visitor_management.invitation_deleted",
        "requestBody": {
          "description": "# visitor_management.invitation_deleted\n\nThe **Visitor Management Invitation Deleted** event is triggered when an [invitation is deleted](https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0073629#mcetoc_1hi1ubsr1fh).\n\n## Prerequisites\n\n* The [**Visitor Management**](https://support.zoom.com/hc/en?id=zm_kb&sysparm_article=KB0057564) feature enabled by your organization.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Visitor Management Invitation Deleted** subscription enabled under the **Visitor Management** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:visitor_invitation:admin`\n\n**Event type**: `visitor_management.invitation_deleted`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The user account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that deleted the invitation."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that deleted the invitation."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the deleted invitation.",
                        "required": [
                          "visitor_email",
                          "visitor_name",
                          "invite_location_id",
                          "host_id",
                          "host_name",
                          "host_email"
                        ],
                        "properties": {
                          "visitor_email": {
                            "type": "string",
                            "description": "The email address of the invited visitor."
                          },
                          "visitor_name": {
                            "type": "string",
                            "description": "The name of the invited visitor."
                          },
                          "invite_location_id": {
                            "type": "string",
                            "description": "The location ID of the location to which the visitor is invited to."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The user ID of the invitation host."
                          },
                          "host_name": {
                            "type": "string",
                            "description": "The name of the invitation host."
                          },
                          "host_email": {
                            "type": "string",
                            "description": "The email address of the invitation host."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"visitor_management.invitation_deleted\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The user account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that deleted the invitation."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that deleted the invitation."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the deleted invitation.",
                          "required": [
                            "visitor_email",
                            "visitor_name",
                            "invite_location_id",
                            "host_id",
                            "host_name",
                            "host_email"
                          ],
                          "properties": {
                            "visitor_email": {
                              "type": "string",
                              "description": "The email address of the invited visitor."
                            },
                            "visitor_name": {
                              "type": "string",
                              "description": "The name of the invited visitor."
                            },
                            "invite_location_id": {
                              "type": "string",
                              "description": "The location ID of the location to which the visitor is invited to."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The user ID of the invitation host."
                            },
                            "host_name": {
                              "type": "string",
                              "description": "The name of the invitation host."
                            },
                            "host_email": {
                              "type": "string",
                              "description": "The email address of the invitation host."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"visitor_management.invitation_deleted\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.alert": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.alert",
        "requestBody": {
          "description": "# zoomroom.alert\n\nThe **Zoom Room Alert** event is triggered when there is an issue with a Zoom Room on a Zoom account that has installed your [webhook-enabled](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks) app. Some of the issues that can trigger this event are:\n\n* High CPU usage.\n* Low battery, charging, and/or connection issues in a Zoom Room device.\n* Room controller, camera, or speaker disconnections or reconnections.\n* Missing camera or microphone.\n\nIt is recommended that you **only** subscribe to either the **Zoom Room Alert** event or the **Zoom Room Delayed Alert** event to avoid receiving duplicate event notifications.\n\n## Prerequisites\n\n* A Business or Enterprise plan.\n* The administrator or account owner must enable the [**Zoom Room Alert**](https://support.zoom.us/hc/en-us/articles/205394099-Zoom-Room-Alerts) feature with the selected alerts in the [Zoom Web Portal](https://zoom.us/location).\n* The Zoom Room version must be 3.5.24524.0821 and higher.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Room Alert** subscription enabled under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:alert:admin`\n\n**Event type**: `zoomroom.alert`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "id",
                          "room_name",
                          "calendar_name",
                          "email",
                          "issue",
                          "alert_type",
                          "component",
                          "alert_time",
                          "alert_kind"
                        ],
                        "description": "Information about the Zoom Room.",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Room's unique ID."
                          },
                          "room_name": {
                            "type": "string",
                            "description": "The Zoom Room's name."
                          },
                          "calendar_name": {
                            "type": "string",
                            "description": "The calendar's name."
                          },
                          "email": {
                            "type": "string",
                            "format": "email",
                            "description": "The Zoom Room's associated email address."
                          },
                          "issue": {
                            "type": "string",
                            "description": "The issues that occurred during the meeting: \n* Room Controller disconnected \n* Room Controller connected \n* Selected camera has disconnected \n* Selected camera is reconnected \n* Selected microphone has disconnected \n* Selected microphone is reconnected \n* Selected speaker has disconnected \n* Selected speaker is reconnected \n* Zoom room is offline \n* Zoom room is online \n* High CPU usage is detected \n* Low bandwidth network is detected \n* {name} battery is low \n* {name} battery is normal \n* {name} disconnected \n* {name} connected \n Incorrect Room Passcode Entry Detected \n\nThe possible values for {name} are: \n* Zoom Rooms Computer \n* Controller \n* Scheduling Display \n*."
                          },
                          "alert_type": {
                            "type": "integer",
                            "description": "The type of alert sent: \n* 1 \u2014 The controller or scheduling display disconnected. \n* 2 \u2014 The selected microphone, speaker, or camera has disconnected \n* 3 \u2014 The Zoom Room is offline. \n* 4 \u2014 High CPU usage detected. \n* 5 \u2014 Low network bandwidth detected. \n* 6 \u2014 The controller or scheduling display battery is low. \n* 7 \u2014 The Zoom Rooms display is disconnected. \n* 8 \u2014 The audio device has quality issues \n* 9 \u2014 Failed SIP registration \n* 10 \u2014 Incorrect room passcode entry detected \n* 0 \u2014 Other alert.",
                            "enum": [
                              1,
                              2,
                              3,
                              4,
                              5,
                              6,
                              7,
                              8,
                              9,
                              10,
                              0
                            ]
                          },
                          "component": {
                            "description": "The affected component: \n* 1 \u2014 Zoom Rooms Computer \n* 2 \u2014 Controller \n* 3 \u2014 Scheduling Display \n* 4 \u2014 Microphone \n* 5 \u2014 Camera \n* 6 \u2014 Speaker \n* 0 - Other.",
                            "type": "integer",
                            "enum": [
                              1,
                              2,
                              3,
                              4,
                              5,
                              6,
                              0
                            ]
                          },
                          "alert_time": {
                            "type": "integer",
                            "format": "int64",
                            "description": "The timestamp when the alert occurred."
                          },
                          "alert_kind": {
                            "type": "integer",
                            "description": "The kind of alert: \n* 1 \u2014 Triggered. There was an encountered problem. When the problem is resolved, a follow-up response will contain this field with the 2 (Cleared) value. \n* 2 \u2014 Cleared. The problem is resolved. \n* 3 \u2014 Observed. There is a potential issue-causing condition. If the issue is cleared (2), there will **not** be a follow-up response. \n* 4 - Other.",
                            "enum": [
                              1,
                              2,
                              3,
                              4
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
                  "value": "{\n  \"event\": \"zoomroom.alert\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"id\": \"abcD3ojfdbjfg\",\n      \"room_name\": \"My Zoom Room\",\n      \"calendar_name\": \"mycalendar@example.com\",\n      \"email\": \"jchill@example.com\",\n      \"issue\": \"Room Controller disconnected\",\n      \"alert_type\": 1,\n      \"component\": 2,\n      \"alert_kind\": 1\n    }\n  }\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "id",
                            "room_name",
                            "calendar_name",
                            "email",
                            "issue",
                            "alert_type",
                            "component",
                            "alert_time",
                            "alert_kind"
                          ],
                          "description": "Information about the Zoom Room.",
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Room's unique ID."
                            },
                            "room_name": {
                              "type": "string",
                              "description": "The Zoom Room's name."
                            },
                            "calendar_name": {
                              "type": "string",
                              "description": "The calendar's name."
                            },
                            "email": {
                              "type": "string",
                              "format": "email",
                              "description": "The Zoom Room's associated email address."
                            },
                            "issue": {
                              "type": "string",
                              "description": "The issues that occurred during the meeting: \n* Room Controller disconnected \n* Room Controller connected \n* Selected camera has disconnected \n* Selected camera is reconnected \n* Selected microphone has disconnected \n* Selected microphone is reconnected \n* Selected speaker has disconnected \n* Selected speaker is reconnected \n* Zoom room is offline \n* Zoom room is online \n* High CPU usage is detected \n* Low bandwidth network is detected \n* {name} battery is low \n* {name} battery is normal \n* {name} disconnected \n* {name} connected \n Incorrect Room Passcode Entry Detected \n\nThe possible values for {name} are: \n* Zoom Rooms Computer \n* Controller \n* Scheduling Display \n*."
                            },
                            "alert_type": {
                              "type": "integer",
                              "description": "The type of alert sent: \n* 1 \u2014 The controller or scheduling display disconnected. \n* 2 \u2014 The selected microphone, speaker, or camera has disconnected \n* 3 \u2014 The Zoom Room is offline. \n* 4 \u2014 High CPU usage detected. \n* 5 \u2014 Low network bandwidth detected. \n* 6 \u2014 The controller or scheduling display battery is low. \n* 7 \u2014 The Zoom Rooms display is disconnected. \n* 8 \u2014 The audio device has quality issues \n* 9 \u2014 Failed SIP registration \n* 10 \u2014 Incorrect room passcode entry detected \n* 0 \u2014 Other alert.",
                              "enum": [
                                1,
                                2,
                                3,
                                4,
                                5,
                                6,
                                7,
                                8,
                                9,
                                10,
                                0
                              ]
                            },
                            "component": {
                              "description": "The affected component: \n* 1 \u2014 Zoom Rooms Computer \n* 2 \u2014 Controller \n* 3 \u2014 Scheduling Display \n* 4 \u2014 Microphone \n* 5 \u2014 Camera \n* 6 \u2014 Speaker \n* 0 - Other.",
                              "type": "integer",
                              "enum": [
                                1,
                                2,
                                3,
                                4,
                                5,
                                6,
                                0
                              ]
                            },
                            "alert_time": {
                              "type": "integer",
                              "format": "int64",
                              "description": "The timestamp when the alert occurred."
                            },
                            "alert_kind": {
                              "type": "integer",
                              "description": "The kind of alert: \n* 1 \u2014 Triggered. There was an encountered problem. When the problem is resolved, a follow-up response will contain this field with the 2 (Cleared) value. \n* 2 \u2014 Cleared. The problem is resolved. \n* 3 \u2014 Observed. There is a potential issue-causing condition. If the issue is cleared (2), there will **not** be a follow-up response. \n* 4 - Other.",
                              "enum": [
                                1,
                                2,
                                3,
                                4
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
                    "value": "{\n  \"event\": \"zoomroom.alert\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"id\": \"abcD3ojfdbjfg\",\n      \"room_name\": \"My Zoom Room\",\n      \"calendar_name\": \"mycalendar@example.com\",\n      \"email\": \"jchill@example.com\",\n      \"issue\": \"Room Controller disconnected\",\n      \"alert_type\": 1,\n      \"component\": 2,\n      \"alert_kind\": 1\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.reservation_questionnaire_response_updated": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.reservation_questionnaire_response_updated",
        "requestBody": {
          "description": "# workspace.reservation_questionnaire_response_updated\nThe **Workspace reservation questionnaire updated** event is triggered every time a questionnaire is updated. Users or Admins can update the questionnaire by updating the existing reservation.\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace reservation questionnaire updated** subscription enabled under the **Workspace** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:workspace_reservation_questionnaire_response`,`workspace:read:workspace_reservation_questionnaire_response:admin`\n\n**Event type**: `workspace.reservation_questionnaire_response_updated`\n",
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
                    "type": "string",
                    "format": "date-time",
                    "description": "A timestamp when the event occurred."
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that completed the questionnaire."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that completed the questionnaire."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the questionnaire when creating a reservation for a workspace.",
                        "required": [
                          "start_time",
                          "event_id",
                          "user_name",
                          "desk_id",
                          "workspace_name",
                          "location_id",
                          "questionnaire_response"
                        ],
                        "properties": {
                          "start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The start time of the workspace reservation."
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The event ID."
                          },
                          "user_name": {
                            "type": "string",
                            "description": "The user name."
                          },
                          "desk_id": {
                            "type": "string",
                            "description": "The desk ID."
                          },
                          "workspace_name": {
                            "type": "string",
                            "description": "The name of the workspace."
                          },
                          "location_id": {
                            "type": "string",
                            "description": "The location ID."
                          },
                          "questionnaire_response": {
                            "type": "array",
                            "description": "List of questionnaires.",
                            "items": {
                              "type": "object",
                              "required": [
                                "id",
                                "title",
                                "subject",
                                "list"
                              ],
                              "properties": {
                                "id": {
                                  "type": "string",
                                  "description": "The view ID."
                                },
                                "title": {
                                  "type": "string",
                                  "description": "The title of the questionnaire."
                                },
                                "subject": {
                                  "type": "string",
                                  "description": "The subject of the questionnaire."
                                },
                                "list": {
                                  "type": "array",
                                  "description": "List of questions in the questionnaire.",
                                  "items": {
                                    "type": "object",
                                    "required": [
                                      "question_id",
                                      "title",
                                      "type",
                                      "options",
                                      "reply_id"
                                    ],
                                    "properties": {
                                      "question_id": {
                                        "type": "string",
                                        "description": "The question ID."
                                      },
                                      "title": {
                                        "type": "string",
                                        "description": "The title of the question."
                                      },
                                      "type": {
                                        "type": "string",
                                        "description": "The type of the question (single-choice, multiple-choice, open-text)."
                                      },
                                      "reply_id": {
                                        "type": "string",
                                        "description": "The reply ID."
                                      },
                                      "options": {
                                        "type": "array",
                                        "description": "List of options for multiple-choice questions.",
                                        "items": {
                                          "type": "object",
                                          "required": [
                                            "option_id",
                                            "text"
                                          ],
                                          "properties": {
                                            "option_id": {
                                              "type": "string",
                                              "description": "The option ID."
                                            },
                                            "text": {
                                              "type": "string",
                                              "description": "The text of the option."
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
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"workspace.reservation_questionnaire_response_updated\",\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator\": \"yuki-devint@zoom.us\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"object\": {\n      \"start_time\": \"2024-02-14T03:50:00Z\",\n      \"event_id\": \"dxpwbc2fi8tzz7d012rwx9u8c0\",\n      \"user_name\": \"Simash pokh\",\n      \"desk_id\": \"SJjU1CcGSs2L7odn6PanQw\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"questionnaire_response\": [\n        {\n          \"id\": \"Fl75C43YTmS_IpMzYLipJw\",\n          \"title\": \"Simash Reservation Questionnaire\",\n          \"list\": [\n            {\n              \"question_id\": \"0ds1NFeDQ1WfWlgVK-7OMQ\",\n              \"title\": \"do you like AI ?\",\n              \"type\": \"single\",\n              \"options\": [\n                {\n                  \"option_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\",\n                  \"text\": \"Yes\"\n                },\n                {\n                  \"option_id\": \"1S1hZzFGTwW9Llxdr2vzyw\",\n                  \"text\": \"No\"\n                }\n              ],\n              \"reply_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\"\n            },\n            {\n              \"question_id\": \"2XMaJFwNSr-BJpTfo6rRAQ\",\n              \"title\": \"what is your fav food ? \",\n              \"type\": \"multiple\",\n              \"options\": [\n                {\n                  \"option_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ\",\n                  \"text\": \"momo\"\n                },\n                {\n                  \"option_id\": \"dpv4EZ7QQUqRhBmYj4_b7w\",\n                  \"text\": \"sea food\"\n                },\n                {\n                  \"option_id\": \"__DjlrV7TeyCSZVmCWAnng\",\n                  \"text\": \"rice\"\n                }\n              ],\n              \"reply_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ,dpv4EZ7QQUqRhBmYj4_b7w\"\n            }\n          ],\n          \"subject\": \"reservation\"\n        }\n      ]\n    }\n  },\n  \"event_ts\": 1707848572310\n}"
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
                      "type": "string",
                      "format": "date-time",
                      "description": "A timestamp when the event occurred."
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that completed the questionnaire."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that completed the questionnaire."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the questionnaire when creating a reservation for a workspace.",
                          "required": [
                            "start_time",
                            "event_id",
                            "user_name",
                            "desk_id",
                            "workspace_name",
                            "location_id",
                            "questionnaire_response"
                          ],
                          "properties": {
                            "start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The start time of the workspace reservation."
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The event ID."
                            },
                            "user_name": {
                              "type": "string",
                              "description": "The user name."
                            },
                            "desk_id": {
                              "type": "string",
                              "description": "The desk ID."
                            },
                            "workspace_name": {
                              "type": "string",
                              "description": "The name of the workspace."
                            },
                            "location_id": {
                              "type": "string",
                              "description": "The location ID."
                            },
                            "questionnaire_response": {
                              "type": "array",
                              "description": "List of questionnaires.",
                              "items": {
                                "type": "object",
                                "required": [
                                  "id",
                                  "title",
                                  "subject",
                                  "list"
                                ],
                                "properties": {
                                  "id": {
                                    "type": "string",
                                    "description": "The view ID."
                                  },
                                  "title": {
                                    "type": "string",
                                    "description": "The title of the questionnaire."
                                  },
                                  "subject": {
                                    "type": "string",
                                    "description": "The subject of the questionnaire."
                                  },
                                  "list": {
                                    "type": "array",
                                    "description": "List of questions in the questionnaire.",
                                    "items": {
                                      "type": "object",
                                      "required": [
                                        "question_id",
                                        "title",
                                        "type",
                                        "options",
                                        "reply_id"
                                      ],
                                      "properties": {
                                        "question_id": {
                                          "type": "string",
                                          "description": "The question ID."
                                        },
                                        "title": {
                                          "type": "string",
                                          "description": "The title of the question."
                                        },
                                        "type": {
                                          "type": "string",
                                          "description": "The type of the question (single-choice, multiple-choice, open-text)."
                                        },
                                        "reply_id": {
                                          "type": "string",
                                          "description": "The reply ID."
                                        },
                                        "options": {
                                          "type": "array",
                                          "description": "List of options for multiple-choice questions.",
                                          "items": {
                                            "type": "object",
                                            "required": [
                                              "option_id",
                                              "text"
                                            ],
                                            "properties": {
                                              "option_id": {
                                                "type": "string",
                                                "description": "The option ID."
                                              },
                                              "text": {
                                                "type": "string",
                                                "description": "The text of the option."
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
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"workspace.reservation_questionnaire_response_updated\",\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator\": \"yuki-devint@zoom.us\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"object\": {\n      \"start_time\": \"2024-02-14T03:50:00Z\",\n      \"event_id\": \"dxpwbc2fi8tzz7d012rwx9u8c0\",\n      \"user_name\": \"Simash pokh\",\n      \"desk_id\": \"SJjU1CcGSs2L7odn6PanQw\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"questionnaire_response\": [\n        {\n          \"id\": \"Fl75C43YTmS_IpMzYLipJw\",\n          \"title\": \"Simash Reservation Questionnaire\",\n          \"list\": [\n            {\n              \"question_id\": \"0ds1NFeDQ1WfWlgVK-7OMQ\",\n              \"title\": \"do you like AI ?\",\n              \"type\": \"single\",\n              \"options\": [\n                {\n                  \"option_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\",\n                  \"text\": \"Yes\"\n                },\n                {\n                  \"option_id\": \"1S1hZzFGTwW9Llxdr2vzyw\",\n                  \"text\": \"No\"\n                }\n              ],\n              \"reply_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\"\n            },\n            {\n              \"question_id\": \"2XMaJFwNSr-BJpTfo6rRAQ\",\n              \"title\": \"what is your fav food ? \",\n              \"type\": \"multiple\",\n              \"options\": [\n                {\n                  \"option_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ\",\n                  \"text\": \"momo\"\n                },\n                {\n                  \"option_id\": \"dpv4EZ7QQUqRhBmYj4_b7w\",\n                  \"text\": \"sea food\"\n                },\n                {\n                  \"option_id\": \"__DjlrV7TeyCSZVmCWAnng\",\n                  \"text\": \"rice\"\n                }\n              ],\n              \"reply_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ,dpv4EZ7QQUqRhBmYj4_b7w\"\n            }\n          ],\n          \"subject\": \"reservation\"\n        }\n      ]\n    }\n  },\n  \"event_ts\": 1707848572310\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.reservation_update": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.reservation_update",
        "requestBody": {
          "description": "# workspace.reservation_update\n\nThe **Workspace Reservation Updated** event is triggered when a user updates their [Workspace reservation](https://support.zoom.us/hc/en-us/articles/4417951085965-Using-Workspace-Reservation-Beta-for-Zoom-Rooms).\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\n* The Zoom mobile app. This is required for QR code scanning:\n  * Android version 5.9.0.3502 or higher\n  * iOS version 5.9.0 or higher\n* [Zoom Rooms for Touch devices](https://support.zoom.us/hc/en-us/articles/115004773103), version 5.9.0 or higher (*Optional*).\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace Reservation Updated** subscription enabled under the **Workspace Reservation** event.\n\n\n\n**Event type**: `workspace.reservation_update`\n",
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that updated the reservation."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that updated the reservation."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the workspace reservation.",
                        "required": [
                          "id",
                          "workspace_id",
                          "end_time",
                          "start_time",
                          "topic"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The reservation ID."
                          },
                          "workspace_id": {
                            "type": "string",
                            "description": "The workspace ID."
                          },
                          "end_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's end time."
                          },
                          "start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's start time."
                          },
                          "topic": {
                            "type": "string",
                            "description": "The reservation's description."
                          },
                          "reserve_for": {
                            "type": "string",
                            "description": "The ID of the user for whom to create a workspace reservation.",
                            "example": "KDcuGIm1QgePTO8WbOqwIQ"
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
                  "value": "{\n\t\"event\": \"workspace.reservation_updated\",\n\t\"event_ts\": \"290112312321\",\n\t\"payload\": {\n\t\t\"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n\t\t\"operator\" : \"admin@example.com\",\n\t    \"operator_id\" : \"I7zp-pYbTl22Z29H796Ixg\",\n\t\t\"object\": {\n\t\t\t\"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n\t\t\t\"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n\t\t\t\"topic\": \"Reserve Zoom Room-01\",\n\t\t\t\"start_time\": \"2022-01-19T02:10:00Z\",\n\t\t\t\"end_time\": \"2022-01-19T03:55:00Z\",\n\t\t\t\"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\"\n\t\t}\n\t}\n}"
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that updated the reservation."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that updated the reservation."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the workspace reservation.",
                          "required": [
                            "id",
                            "workspace_id",
                            "end_time",
                            "start_time",
                            "topic"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The reservation ID."
                            },
                            "workspace_id": {
                              "type": "string",
                              "description": "The workspace ID."
                            },
                            "end_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's end time."
                            },
                            "start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's start time."
                            },
                            "topic": {
                              "type": "string",
                              "description": "The reservation's description."
                            },
                            "reserve_for": {
                              "type": "string",
                              "description": "The ID of the user for whom to create a workspace reservation.",
                              "example": "KDcuGIm1QgePTO8WbOqwIQ"
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
                    "value": "{\n\t\"event\": \"workspace.reservation_updated\",\n\t\"event_ts\": \"290112312321\",\n\t\"payload\": {\n\t\t\"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n\t\t\"operator\" : \"admin@example.com\",\n\t    \"operator_id\" : \"I7zp-pYbTl22Z29H796Ixg\",\n\t\t\"object\": {\n\t\t\t\"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n\t\t\t\"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n\t\t\t\"topic\": \"Reserve Zoom Room-01\",\n\t\t\t\"start_time\": \"2022-01-19T02:10:00Z\",\n\t\t\t\"end_time\": \"2022-01-19T03:55:00Z\",\n\t\t\t\"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\"\n\t\t}\n\t}\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.user_assigned": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.user_assigned",
        "requestBody": {
          "description": "# workspace.user_assigned\nThe **Workspace User Assigned** event is triggered every time a user is assigned to a Workspace in your account. Admins and account owners can assign a user to Workspace in you account. The Workspace can be assign users using the [Set a desk assignment API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/setADeskAssignment) or by set the Workspace' assignment using the Zoom Web Portal itself.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace User Assigned** subscription enabled under the **Workspace** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:assignment`,`workspace:read:assignment:admin`\n\n**Event type**: `workspace.user_assigned`\n",
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
                    "type": "string",
                    "format": "date-time",
                    "description": "A timestamp when the event occurred."
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that updated the assignment."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that updated the assignment."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the assignment when updating assignment of a workspace.",
                        "required": [
                          "workspace_id",
                          "workspace_name",
                          "assign_type",
                          "account_id",
                          "user_id",
                          "user_email"
                        ],
                        "properties": {
                          "workspace_id": {
                            "type": "string",
                            "description": "The workspace's ID."
                          },
                          "workspace_name": {
                            "type": "string",
                            "description": "The workspace's name."
                          },
                          "assign_type": {
                            "type": "string",
                            "description": "The type of assignment. ",
                            "enum": [
                              "permanent"
                            ]
                          },
                          "account_id": {
                            "type": "string",
                            "description": "The belong's account id of assigned user."
                          },
                          "user_id": {
                            "type": "string",
                            "description": "The id of assigned user."
                          },
                          "user_email": {
                            "type": "string",
                            "description": "The email of assigned user."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"workspace.user_assigned\",\n  \"event_ts\": 1707848572310,\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"operator\": \"vic-devint@zoom.us\",\n    \"object\": {\n      \"workspace_id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"assign_type\": \"permanent\",\n      \"account_id\": \"bhmIXpdYRcCK6BB3VhMq6w\",\n      \"user_id\": \"oyLyxW48QeC3uBBFY2CF3A\",\n      \"user_email\": \"vic-01-devint@zoom.us\"\n    }\n  }\n}"
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
                      "type": "string",
                      "format": "date-time",
                      "description": "A timestamp when the event occurred."
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that updated the assignment."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that updated the assignment."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the assignment when updating assignment of a workspace.",
                          "required": [
                            "workspace_id",
                            "workspace_name",
                            "assign_type",
                            "account_id",
                            "user_id",
                            "user_email"
                          ],
                          "properties": {
                            "workspace_id": {
                              "type": "string",
                              "description": "The workspace's ID."
                            },
                            "workspace_name": {
                              "type": "string",
                              "description": "The workspace's name."
                            },
                            "assign_type": {
                              "type": "string",
                              "description": "The type of assignment. ",
                              "enum": [
                                "permanent"
                              ]
                            },
                            "account_id": {
                              "type": "string",
                              "description": "The belong's account id of assigned user."
                            },
                            "user_id": {
                              "type": "string",
                              "description": "The id of assigned user."
                            },
                            "user_email": {
                              "type": "string",
                              "description": "The email of assigned user."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"workspace.user_assigned\",\n  \"event_ts\": 1707848572310,\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"operator\": \"vic-devint@zoom.us\",\n    \"object\": {\n      \"workspace_id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"assign_type\": \"permanent\",\n      \"account_id\": \"bhmIXpdYRcCK6BB3VhMq6w\",\n      \"user_id\": \"oyLyxW48QeC3uBBFY2CF3A\",\n      \"user_email\": \"vic-01-devint@zoom.us\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.digital_signage_app_ended": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.digital_signage_app_ended",
        "requestBody": {
          "description": "# zoomroom.digital_signage_app_ended\n\nThe **[Zoom Room digital signage](https://support.zoom.us/hc/en-us/articles/360000030683-Zoom-Rooms-digital-signage) play ended** event triggers when the digital signage finishes playing or showing.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n* A valid **Event Notification Endpoint URL**.\n* The **Zoom Room digital signage app ended** subscription enabled under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:digital_signage_app:admin`\n\n**Event type**: `zoomroom.digital_signage_app_ended`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "room_id",
                          "app_name",
                          "app_id",
                          "start_time",
                          "end_time"
                        ],
                        "description": "The information on the digital signage played in a Zoom Room.",
                        "properties": {
                          "room_id": {
                            "type": "string",
                            "description": "The Zoom Room ID."
                          },
                          "app_name": {
                            "type": "string",
                            "description": "The app name."
                          },
                          "app_id": {
                            "type": "string",
                            "description": "The app ID."
                          },
                          "start_time": {
                            "description": "The playback start time of the digital signage.",
                            "type": "string",
                            "format": "datetime",
                            "example": "2022-06-19T00:00:00Z"
                          },
                          "end_time": {
                            "description": "The playback end time of the digital signage.",
                            "type": "string",
                            "format": "datetime",
                            "example": "2022-06-19T00:00:00Z"
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
                  "value": "{\n\t\"event\": \"zoomroom.digital_signage_app_ended\",\n\t\"event_ts\": 1626230691572,\n\t\"payload\": {\n\t\t\"account_id\": \"AAAAAABBBB\",\n\t\t\"object\": {\n\t\t\t\"room_id\": \"abcD3ojfdbjfg\",\n\t\t\t\"app_name\": \"My Zoom Room\",\n\t\t\t\"app_id\": \"NiVvY1NWpE2nulNrhVjgU4jD0swziVrXBbaFZyC3u+o=\",\n\t\t\t\"start_time\": \"2022-06-19T00:00:00Z\",\n            \"end_time\": \"2022-06-19T01:01:00Z\"\n\t\t}\n\t}\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "room_id",
                            "app_name",
                            "app_id",
                            "start_time",
                            "end_time"
                          ],
                          "description": "The information on the digital signage played in a Zoom Room.",
                          "properties": {
                            "room_id": {
                              "type": "string",
                              "description": "The Zoom Room ID."
                            },
                            "app_name": {
                              "type": "string",
                              "description": "The app name."
                            },
                            "app_id": {
                              "type": "string",
                              "description": "The app ID."
                            },
                            "start_time": {
                              "description": "The playback start time of the digital signage.",
                              "type": "string",
                              "format": "datetime",
                              "example": "2022-06-19T00:00:00Z"
                            },
                            "end_time": {
                              "description": "The playback end time of the digital signage.",
                              "type": "string",
                              "format": "datetime",
                              "example": "2022-06-19T00:00:00Z"
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
                    "value": "{\n\t\"event\": \"zoomroom.digital_signage_app_ended\",\n\t\"event_ts\": 1626230691572,\n\t\"payload\": {\n\t\t\"account_id\": \"AAAAAABBBB\",\n\t\t\"object\": {\n\t\t\t\"room_id\": \"abcD3ojfdbjfg\",\n\t\t\t\"app_name\": \"My Zoom Room\",\n\t\t\t\"app_id\": \"NiVvY1NWpE2nulNrhVjgU4jD0swziVrXBbaFZyC3u+o=\",\n\t\t\t\"start_time\": \"2022-06-19T00:00:00Z\",\n            \"end_time\": \"2022-06-19T01:01:00Z\"\n\t\t}\n\t}\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.checked_in": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.checked_in",
        "requestBody": {
          "description": "# zoomroom.checked_in\n\nThe **Zoom Room Checked In** event is triggered when a user [checks in](https://support.zoom.us/hc/en-us/articles/360001147163-Using-the-Zoom-Rooms-check-in-feature) to confirm that they will be using the Zoom Room for a certain period of time.\n\nUsers can check in manually via the Zoom Room controller or the scheduling display, or they can check in programmatically using the [**Check-in or check-out of a Zoom Room**](/api-reference/zoom-api/methods#operation/checkInRooms) API. Users can also check in to the Zoom Room by starting a scheduled meeting, using the whiteboard feature or making a phone call from the Zoom Room.\n\n**Note** Only scheduled meetings will trigger a webhook.\n\n\n## Prerequisites\n\n* The administrator or account owner must enable the [**Room Check-In and Check-Out**](https://support.zoom.us/hc/en-us/articles/360001147163-Using-the-Zoom-Rooms-check-in-feature) feature.\n* The Zoom Room version must be 4.1.27694.0702 or higher.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Room Checked In** subscription enabled under the **Zoom Room** event.\n\n\n\n\n\n\n\n\n\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\n\n**Event type**: `zoomroom.checked_in`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "id",
                          "room_name",
                          "calendar_name",
                          "email",
                          "event_id",
                          "change_key",
                          "resource_email",
                          "calendar_id",
                          "calendar_type",
                          "api_type"
                        ],
                        "description": "Information about the Zoom Room.",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Room's unique ID."
                          },
                          "room_name": {
                            "type": "string",
                            "description": "The Zoom Room's name."
                          },
                          "calendar_name": {
                            "type": "string",
                            "description": "The calendar's name."
                          },
                          "email": {
                            "type": "string",
                            "format": "email",
                            "description": "The Zoom Room's associated email address."
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The unique ID of the Zoom Room's associated calendar event:\n\n* For Microsoft Exchange calendar integration, use Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). The event_id value is the Id value in the ItemId element of the FindItem operation's response. \n* For Google Calendar integration, use Google's [Events: list API](https://developers.google.com/calendar/api/v3/reference/events/list). The event_id value is the id field in the [events resource](https://developers.google.com/calendar/api/v3/reference/events#resource)."
                          },
                          "change_key": {
                            "type": "string",
                            "description": "The specific version of the calendar item. You can get this value using Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). Refer to the ChangeKey field of the ItemId element in the operation's response.\n\nThis field is only returned for Microsoft Exchange or Office 365 calendars."
                          },
                          "resource_email": {
                            "description": "The calendar's associated [resource mailbox](https://support.microsoft.com/en-us/help/10063/creating-and-managing-resource-mailboxes-in-office-365).\n\nThis field is only returned for Microsoft Exchange or Office 365 calendars.",
                            "type": "string"
                          },
                          "calendar_id": {
                            "type": "string",
                            "description": "The calendar's unique ID."
                          },
                          "calendar_type": {
                            "type": "string",
                            "description": "The calendar's type: \n* 0 \u2014 Google Calendar. \n* 1 \u2014 Office 365 Calendar. \n* 2 \u2014 Microsoft Exchange Calendar.",
                            "enum": [
                              "0",
                              "1",
                              "2"
                            ]
                          },
                          "api_type": {
                            "type": "string",
                            "description": "The API type: \n* 0 \u2014 [Exchange Web Services (EWS) API](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-managed-api-reference-for-exchange) \n* 1 \u2014 [Microsoft Graph API](https://docs.microsoft.com/en-us/azure/active-directory/develop/microsoft-graph-intro) \n* An empty string \u2014 The API returns this value when the calendar_type value is 0 (Google Calendar).",
                            "enum": [
                              "0",
                              "1",
                              ""
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
                  "value": "{\n  \"event\": \"zoomroom.checked_in\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n        \"id\": \"abcD3ojfdbjfg\",\n        \"room_name\": \"My Zoom Room\",\n        \"calendar_name\": \"mycalendar@example.com\",\n        \"email\": \"jchill@example.com\",\n        \"event_id\": \"AbbbbbGYxLTc3OTVkMzFmZDc0MwBGAAAAAAD48FI58voYSqDgJePOSZblBwBQ/N0JvB/FRqv5UT2rFfkVAAAAAAENAABQ/N0JvB/FRqv5UT2rFfkVAAE2YC8DAAA=\",\n        \"change_key\": \"DwAAABYAAABQ/N0JvB/FRqv5UT2rFfkVAAE2XqVw\",\n        \"resource_email\": \"it@example.com\",\n        \"calendar_id\": \"mycalendar@example.com\",\n        \"calendar_type\": \"2\",\n        \"api_type\": \"0\"\n    }\n  }\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "id",
                            "room_name",
                            "calendar_name",
                            "email",
                            "event_id",
                            "change_key",
                            "resource_email",
                            "calendar_id",
                            "calendar_type",
                            "api_type"
                          ],
                          "description": "Information about the Zoom Room.",
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Room's unique ID."
                            },
                            "room_name": {
                              "type": "string",
                              "description": "The Zoom Room's name."
                            },
                            "calendar_name": {
                              "type": "string",
                              "description": "The calendar's name."
                            },
                            "email": {
                              "type": "string",
                              "format": "email",
                              "description": "The Zoom Room's associated email address."
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The unique ID of the Zoom Room's associated calendar event:\n\n* For Microsoft Exchange calendar integration, use Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). The event_id value is the Id value in the ItemId element of the FindItem operation's response. \n* For Google Calendar integration, use Google's [Events: list API](https://developers.google.com/calendar/api/v3/reference/events/list). The event_id value is the id field in the [events resource](https://developers.google.com/calendar/api/v3/reference/events#resource)."
                            },
                            "change_key": {
                              "type": "string",
                              "description": "The specific version of the calendar item. You can get this value using Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). Refer to the ChangeKey field of the ItemId element in the operation's response.\n\nThis field is only returned for Microsoft Exchange or Office 365 calendars."
                            },
                            "resource_email": {
                              "description": "The calendar's associated [resource mailbox](https://support.microsoft.com/en-us/help/10063/creating-and-managing-resource-mailboxes-in-office-365).\n\nThis field is only returned for Microsoft Exchange or Office 365 calendars.",
                              "type": "string"
                            },
                            "calendar_id": {
                              "type": "string",
                              "description": "The calendar's unique ID."
                            },
                            "calendar_type": {
                              "type": "string",
                              "description": "The calendar's type: \n* 0 \u2014 Google Calendar. \n* 1 \u2014 Office 365 Calendar. \n* 2 \u2014 Microsoft Exchange Calendar.",
                              "enum": [
                                "0",
                                "1",
                                "2"
                              ]
                            },
                            "api_type": {
                              "type": "string",
                              "description": "The API type: \n* 0 \u2014 [Exchange Web Services (EWS) API](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-managed-api-reference-for-exchange) \n* 1 \u2014 [Microsoft Graph API](https://docs.microsoft.com/en-us/azure/active-directory/develop/microsoft-graph-intro) \n* An empty string \u2014 The API returns this value when the calendar_type value is 0 (Google Calendar).",
                              "enum": [
                                "0",
                                "1",
                                ""
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
                    "value": "{\n  \"event\": \"zoomroom.checked_in\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n        \"id\": \"abcD3ojfdbjfg\",\n        \"room_name\": \"My Zoom Room\",\n        \"calendar_name\": \"mycalendar@example.com\",\n        \"email\": \"jchill@example.com\",\n        \"event_id\": \"AbbbbbGYxLTc3OTVkMzFmZDc0MwBGAAAAAAD48FI58voYSqDgJePOSZblBwBQ/N0JvB/FRqv5UT2rFfkVAAAAAAENAABQ/N0JvB/FRqv5UT2rFfkVAAE2YC8DAAA=\",\n        \"change_key\": \"DwAAABYAAABQ/N0JvB/FRqv5UT2rFfkVAAE2XqVw\",\n        \"resource_email\": \"it@example.com\",\n        \"calendar_id\": \"mycalendar@example.com\",\n        \"calendar_type\": \"2\",\n        \"api_type\": \"0\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.reservation_checked_in": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.reservation_checked_in",
        "requestBody": {
          "description": "# workspace.reservation_checked_in\n\nThe **Workspace Reservation Checked In** event is triggered when a user checks in their [Workspace reservation](https://support.zoom.us/hc/en-us/articles/4417951085965-Using-Workspace-Reservation-Beta-for-Zoom-Rooms) to confirm that they will be using a Zoom Room or Zoom device-enabled desk.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\n* The Zoom mobile app. This is required for QR code scanning:\n  * Android version 5.9.0.3502 or higher\n  * iOS version 5.9.0 or higher\n* [Zoom Rooms for Touch devices](https://support.zoom.us/hc/en-us/articles/115004773103), version 5.9.0 or higher (*Optional*).\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace Reservation Checked In** subscription enabled under the **Workspace Reservation** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:reservation`,`workspace:read:reservation:admin`\n\n**Event type**: `workspace.reservation_checked_in`\n",
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that checked in the reservation."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that checked in the reservation."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the workspace reservation.",
                        "required": [
                          "id",
                          "workspace_id",
                          "end_time",
                          "start_time",
                          "topic"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The reservation ID."
                          },
                          "workspace_id": {
                            "type": "string",
                            "description": "The workspace ID."
                          },
                          "end_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's end time."
                          },
                          "start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's start time."
                          },
                          "topic": {
                            "type": "string",
                            "description": "The reservation's description."
                          },
                          "reserve_for": {
                            "type": "string",
                            "description": "The ID of the user for whom to create a workspace reservation.",
                            "example": "KDcuGIm1QgePTO8WbOqwIQ"
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
                  "value": "{\n\t\"event\": \"workspace.reservation_checked_in\",\n\t\"event_ts\": \"290112312321\",\n\t\"payload\": {\n\t\t\"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n\t\t\"operator\" : \"admin@example.com\",\n\t    \"operator_id\" : \"I7zp-pYbTl22Z29H796Ixg\",\n\t\t\"object\": {\n\t\t\t\"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n\t\t\t\"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n\t\t\t\"topic\": \"Reserve Zoom Room-01\",\n\t\t\t\"start_time\": \"2022-01-19T02:10:00Z\",\n\t\t\t\"end_time\": \"2022-01-19T03:55:00Z\",\n\t\t\t\"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\"\n\t\t}\n\t}\n}"
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that checked in the reservation."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that checked in the reservation."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the workspace reservation.",
                          "required": [
                            "id",
                            "workspace_id",
                            "end_time",
                            "start_time",
                            "topic"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The reservation ID."
                            },
                            "workspace_id": {
                              "type": "string",
                              "description": "The workspace ID."
                            },
                            "end_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's end time."
                            },
                            "start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's start time."
                            },
                            "topic": {
                              "type": "string",
                              "description": "The reservation's description."
                            },
                            "reserve_for": {
                              "type": "string",
                              "description": "The ID of the user for whom to create a workspace reservation.",
                              "example": "KDcuGIm1QgePTO8WbOqwIQ"
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
                    "value": "{\n\t\"event\": \"workspace.reservation_checked_in\",\n\t\"event_ts\": \"290112312321\",\n\t\"payload\": {\n\t\t\"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n\t\t\"operator\" : \"admin@example.com\",\n\t    \"operator_id\" : \"I7zp-pYbTl22Z29H796Ixg\",\n\t\t\"object\": {\n\t\t\t\"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n\t\t\t\"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n\t\t\t\"topic\": \"Reserve Zoom Room-01\",\n\t\t\t\"start_time\": \"2022-01-19T02:10:00Z\",\n\t\t\t\"end_time\": \"2022-01-19T03:55:00Z\",\n\t\t\t\"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\"\n\t\t}\n\t}\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.updated": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.updated",
        "requestBody": {
          "description": "# zoomroom.updated\n\nThe **Zoom Room Updated** event is triggered when a user updates a [**Zoom Room**](https://support.zoom.us/hc/en-us/categories/200108436)\n\n## Prerequisites\n\n* The **Zoom Room** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Room Updated** subscription enabled under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\n\n**Event type**: `zoomroom.updated`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "id",
                          "room_name"
                        ],
                        "description": "Information about the Zoom Room.",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Room's unique ID."
                          },
                          "room_name": {
                            "type": "string",
                            "description": "The Zoom Room's name."
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
                  "value": "{\n  \"event\": \"zoomroom.updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n    \"object\": {\n      \"id\": \"2muAb65FTZCGjj6ginHIbQ\",\n      \"room_name\": \"My Zoom Room\"\n    }\n  }\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "id",
                            "room_name"
                          ],
                          "description": "Information about the Zoom Room.",
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Room's unique ID."
                            },
                            "room_name": {
                              "type": "string",
                              "description": "The Zoom Room's name."
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
                    "value": "{\n  \"event\": \"zoomroom.updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n    \"object\": {\n      \"id\": \"2muAb65FTZCGjj6ginHIbQ\",\n      \"room_name\": \"My Zoom Room\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.location_updated": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.location_updated",
        "requestBody": {
          "description": "# zoomroom.location_updated\nThe **Zoom Rooms Location has been updated** event is triggered every time you update a Zoom Room Location in your account. Admins and account owners can update a Zoom Room Location to an account. Update the location using the [Update a Zoom Rooms Location API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/updateZRLocationProfile) or by updating the Zoom Room Location using the Zoom Web Portal itself.\n\n## Prerequisites\n\n* Enable The **Zoom Rooms** feature by your organization through a Zoom Rooms license or Workspace Reservation feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or **Workspace Reservation** license.\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Rooms Location Updated** subscription enabled under the **Zoom Rooms** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\n\n**Event type**: `zoomroom.location_updated`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who updated the Zoom Rooms Location."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that updated the Zoom Rooms Location."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that updated the Zoom Rooms Location."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the updated Location.",
                        "required": [
                          "id",
                          "name",
                          "parent_location_id",
                          "type"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Rooms location's ID."
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the updated Zoom Rooms location."
                          },
                          "parent_location_id": {
                            "type": "string",
                            "description": "This location's parent Zoom Rooms location ID."
                          },
                          "type": {
                            "type": "enum, country\u2503states\u2503city\u2503campus\u2503building\u2503floor",
                            "description": "The type of location."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"zoomroom.location_updated\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"name\": \"My new Zoom Room Location\",\n      \"parent_location_id\": \"aldjflajsd-fasj\",\n      \"type\": \"floor\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who updated the Zoom Rooms Location."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that updated the Zoom Rooms Location."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that updated the Zoom Rooms Location."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the updated Location.",
                          "required": [
                            "id",
                            "name",
                            "parent_location_id",
                            "type"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Rooms location's ID."
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the updated Zoom Rooms location."
                            },
                            "parent_location_id": {
                              "type": "string",
                              "description": "This location's parent Zoom Rooms location ID."
                            },
                            "type": {
                              "type": "enum, country\u2503states\u2503city\u2503campus\u2503building\u2503floor",
                              "description": "The type of location."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"zoomroom.location_updated\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"name\": \"My new Zoom Room Location\",\n      \"parent_location_id\": \"aldjflajsd-fasj\",\n      \"type\": \"floor\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.pre_check_in_questionnaire_response_created": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.pre_check_in_questionnaire_response_created",
        "requestBody": {
          "description": "# workspace.pre_check_in_questionnaire_response_created\nThe **Workspace pre-check-in questionnaire created** event is triggered every time a pre-check-in questionnaire is submitted. Users can submit the questionnaire by clicking the workspace questionnarie link in the email.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Pre-check-in questionnaire created** subscription enabled under the **Workspace** event.\n\n**Event type**: `workspace.pre_check_in_questionnaire_response_created `\n",
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
                    "type": "string",
                    "format": "date-time",
                    "description": "A timestamp when the event occurred."
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that completed the questionnaire."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that completed the questionnaire."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the questionnaire when creating a reservation for a workspace.",
                        "required": [
                          "start_time",
                          "event_id",
                          "user_name",
                          "desk_id",
                          "workspace_name",
                          "location_id",
                          "questionnaire_response"
                        ],
                        "properties": {
                          "start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The start time of the workspace reservation."
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The event ID."
                          },
                          "user_name": {
                            "type": "string",
                            "description": "The user name."
                          },
                          "desk_id": {
                            "type": "string",
                            "description": "The desk ID."
                          },
                          "workspace_name": {
                            "type": "string",
                            "description": "The name of the workspace."
                          },
                          "location_id": {
                            "type": "string",
                            "description": "The location ID."
                          },
                          "questionnaire_response": {
                            "type": "array",
                            "description": "List of questionnaires.",
                            "items": {
                              "type": "object",
                              "required": [
                                "id",
                                "title",
                                "subject",
                                "list"
                              ],
                              "properties": {
                                "id": {
                                  "type": "string",
                                  "description": "The view ID."
                                },
                                "title": {
                                  "type": "string",
                                  "description": "The title of the questionnaire."
                                },
                                "subject": {
                                  "type": "string",
                                  "description": "The subject of the questionnaire."
                                },
                                "list": {
                                  "type": "array",
                                  "description": "List of questions in the questionnaire.",
                                  "items": {
                                    "type": "object",
                                    "required": [
                                      "question_id",
                                      "title",
                                      "type",
                                      "options",
                                      "reply_id"
                                    ],
                                    "properties": {
                                      "question_id": {
                                        "type": "string",
                                        "description": "The question ID."
                                      },
                                      "title": {
                                        "type": "string",
                                        "description": "The title of the question."
                                      },
                                      "type": {
                                        "type": "string",
                                        "description": "The type of the question (single-choice, multiple-choice, open-text)."
                                      },
                                      "reply_id": {
                                        "type": "string",
                                        "description": "The reply ID."
                                      },
                                      "options": {
                                        "type": "array",
                                        "description": "List of options for multiple-choice questions.",
                                        "items": {
                                          "type": "object",
                                          "required": [
                                            "option_id",
                                            "text"
                                          ],
                                          "properties": {
                                            "option_id": {
                                              "type": "string",
                                              "description": "The option ID."
                                            },
                                            "text": {
                                              "type": "string",
                                              "description": "The text of the option."
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
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"workspace.pre_check_in_questionnaire_response_created\",\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator\": \"yuki-devint@zoom.us\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"object\": {\n      \"start_time\": \"2024-02-14T03:50:00Z\",\n      \"event_id\": \"dxpwbc2fi8tzz7d012rwx9u8c0\",\n      \"user_name\": \"Simash pokh\",\n      \"desk_id\": \"SJjU1CcGSs2L7odn6PanQw\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"questionnaire_respnse\": [\n        {\n          \"id\": \"Fl75C43YTmS_IpMzYLipJw\",\n          \"title\": \"Simash Reservation Questionnaire\",\n          \"list\": [\n            {\n              \"question_id\": \"0ds1NFeDQ1WfWlgVK-7OMQ\",\n              \"title\": \"do you like AI ?\",\n              \"type\": \"single\",\n              \"options\": [\n                {\n                  \"option_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\",\n                  \"text\": \"Yes\"\n                },\n                {\n                  \"option_id\": \"1S1hZzFGTwW9Llxdr2vzyw\",\n                  \"text\": \"No\"\n                }\n              ],\n              \"reply_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\"\n            },\n            {\n              \"question_id\": \"2XMaJFwNSr-BJpTfo6rRAQ\",\n              \"title\": \"what is your fav food ? \",\n              \"type\": \"multiple\",\n              \"options\": [\n                {\n                  \"option_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ\",\n                  \"text\": \"momo\"\n                },\n                {\n                  \"option_id\": \"dpv4EZ7QQUqRhBmYj4_b7w\",\n                  \"text\": \"sea food\"\n                },\n                {\n                  \"option_id\": \"__DjlrV7TeyCSZVmCWAnng\",\n                  \"text\": \"rice\"\n                }\n              ],\n              \"reply_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ,dpv4EZ7QQUqRhBmYj4_b7w\"\n            }\n          ],\n          \"subject\": \"reservation\"\n        }\n      ]\n    }\n  },\n  \"event_ts\": 1707848572310\n}"
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
                      "type": "string",
                      "format": "date-time",
                      "description": "A timestamp when the event occurred."
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that completed the questionnaire."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that completed the questionnaire."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the questionnaire when creating a reservation for a workspace.",
                          "required": [
                            "start_time",
                            "event_id",
                            "user_name",
                            "desk_id",
                            "workspace_name",
                            "location_id",
                            "questionnaire_response"
                          ],
                          "properties": {
                            "start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The start time of the workspace reservation."
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The event ID."
                            },
                            "user_name": {
                              "type": "string",
                              "description": "The user name."
                            },
                            "desk_id": {
                              "type": "string",
                              "description": "The desk ID."
                            },
                            "workspace_name": {
                              "type": "string",
                              "description": "The name of the workspace."
                            },
                            "location_id": {
                              "type": "string",
                              "description": "The location ID."
                            },
                            "questionnaire_response": {
                              "type": "array",
                              "description": "List of questionnaires.",
                              "items": {
                                "type": "object",
                                "required": [
                                  "id",
                                  "title",
                                  "subject",
                                  "list"
                                ],
                                "properties": {
                                  "id": {
                                    "type": "string",
                                    "description": "The view ID."
                                  },
                                  "title": {
                                    "type": "string",
                                    "description": "The title of the questionnaire."
                                  },
                                  "subject": {
                                    "type": "string",
                                    "description": "The subject of the questionnaire."
                                  },
                                  "list": {
                                    "type": "array",
                                    "description": "List of questions in the questionnaire.",
                                    "items": {
                                      "type": "object",
                                      "required": [
                                        "question_id",
                                        "title",
                                        "type",
                                        "options",
                                        "reply_id"
                                      ],
                                      "properties": {
                                        "question_id": {
                                          "type": "string",
                                          "description": "The question ID."
                                        },
                                        "title": {
                                          "type": "string",
                                          "description": "The title of the question."
                                        },
                                        "type": {
                                          "type": "string",
                                          "description": "The type of the question (single-choice, multiple-choice, open-text)."
                                        },
                                        "reply_id": {
                                          "type": "string",
                                          "description": "The reply ID."
                                        },
                                        "options": {
                                          "type": "array",
                                          "description": "List of options for multiple-choice questions.",
                                          "items": {
                                            "type": "object",
                                            "required": [
                                              "option_id",
                                              "text"
                                            ],
                                            "properties": {
                                              "option_id": {
                                                "type": "string",
                                                "description": "The option ID."
                                              },
                                              "text": {
                                                "type": "string",
                                                "description": "The text of the option."
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
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"workspace.pre_check_in_questionnaire_response_created\",\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator\": \"yuki-devint@zoom.us\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"object\": {\n      \"start_time\": \"2024-02-14T03:50:00Z\",\n      \"event_id\": \"dxpwbc2fi8tzz7d012rwx9u8c0\",\n      \"user_name\": \"Simash pokh\",\n      \"desk_id\": \"SJjU1CcGSs2L7odn6PanQw\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"questionnaire_respnse\": [\n        {\n          \"id\": \"Fl75C43YTmS_IpMzYLipJw\",\n          \"title\": \"Simash Reservation Questionnaire\",\n          \"list\": [\n            {\n              \"question_id\": \"0ds1NFeDQ1WfWlgVK-7OMQ\",\n              \"title\": \"do you like AI ?\",\n              \"type\": \"single\",\n              \"options\": [\n                {\n                  \"option_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\",\n                  \"text\": \"Yes\"\n                },\n                {\n                  \"option_id\": \"1S1hZzFGTwW9Llxdr2vzyw\",\n                  \"text\": \"No\"\n                }\n              ],\n              \"reply_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\"\n            },\n            {\n              \"question_id\": \"2XMaJFwNSr-BJpTfo6rRAQ\",\n              \"title\": \"what is your fav food ? \",\n              \"type\": \"multiple\",\n              \"options\": [\n                {\n                  \"option_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ\",\n                  \"text\": \"momo\"\n                },\n                {\n                  \"option_id\": \"dpv4EZ7QQUqRhBmYj4_b7w\",\n                  \"text\": \"sea food\"\n                },\n                {\n                  \"option_id\": \"__DjlrV7TeyCSZVmCWAnng\",\n                  \"text\": \"rice\"\n                }\n              ],\n              \"reply_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ,dpv4EZ7QQUqRhBmYj4_b7w\"\n            }\n          ],\n          \"subject\": \"reservation\"\n        }\n      ]\n    }\n  },\n  \"event_ts\": 1707848572310\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.reservation_checked_out": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.reservation_checked_out",
        "requestBody": {
          "description": "# workspace.reservation_checked_out\n\nThe **Workspace Reservation Checked Out** event is triggered when a user checks out of their [Workspace reservation](https://support.zoom.us/hc/en-us/articles/4417951085965-Using-Workspace-Reservation-Beta-for-Zoom-Rooms) to confirm that they are releasing the Zoom Room or Zoom device-enabled desk.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\n* The Zoom mobile app. This is required for QR code scanning:\n  * Android version 5.9.0.3502 or higher\n  * iOS version 5.9.0 or higher\n* [Zoom Rooms for Touch devices](https://support.zoom.us/hc/en-us/articles/115004773103), version 5.9.0 or higher (*Optional*).\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace Reservation Checked Out** subscription enabled under the **Workspace Reservation** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:reservation`,`workspace:read:reservation:admin`\n\n**Event type**: `workspace.reservation_checked_out`\n",
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that checked out the reservation."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that checked out the reservation."
                      },
                      "object": {
                        "description": "Information about the workspace reservation.",
                        "type": "object",
                        "required": [
                          "id",
                          "workspace_id",
                          "end_time",
                          "start_time",
                          "topic"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The reservation ID."
                          },
                          "workspace_id": {
                            "type": "string",
                            "description": "The workspace ID."
                          },
                          "end_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's end time."
                          },
                          "start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's start time."
                          },
                          "topic": {
                            "type": "string",
                            "description": "The reservation's description."
                          },
                          "reserve_for": {
                            "type": "string",
                            "description": "The ID of the user for whom to create a workspace reservation.",
                            "example": "KDcuGIm1QgePTO8WbOqwIQ"
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
                  "value": "{\n\t\"event\": \"workspace.reservation_checked_out\",\n\t\"event_ts\": \"290112312321\",\n\t\"payload\": {\n\t\t\"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n\t\t\"operator\" : \"admin@example.com\",\n\t    \"operator_id\" : \"I7zp-pYbTl22Z29H796Ixg\",\n\t\t\"object\": {\n\t\t\t\"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n\t\t\t\"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n\t\t\t\"topic\": \"Reserve Zoom Room-01\",\n\t\t\t\"start_time\": \"2022-01-19T02:10:00Z\",\n\t\t\t\"end_time\": \"2022-01-19T03:55:00Z\",\n\t\t\t\"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\"\n\t\t}\n\t}\n}"
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that checked out the reservation."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that checked out the reservation."
                        },
                        "object": {
                          "description": "Information about the workspace reservation.",
                          "type": "object",
                          "required": [
                            "id",
                            "workspace_id",
                            "end_time",
                            "start_time",
                            "topic"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The reservation ID."
                            },
                            "workspace_id": {
                              "type": "string",
                              "description": "The workspace ID."
                            },
                            "end_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's end time."
                            },
                            "start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's start time."
                            },
                            "topic": {
                              "type": "string",
                              "description": "The reservation's description."
                            },
                            "reserve_for": {
                              "type": "string",
                              "description": "The ID of the user for whom to create a workspace reservation.",
                              "example": "KDcuGIm1QgePTO8WbOqwIQ"
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
                    "value": "{\n\t\"event\": \"workspace.reservation_checked_out\",\n\t\"event_ts\": \"290112312321\",\n\t\"payload\": {\n\t\t\"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n\t\t\"operator\" : \"admin@example.com\",\n\t    \"operator_id\" : \"I7zp-pYbTl22Z29H796Ixg\",\n\t\t\"object\": {\n\t\t\t\"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n\t\t\t\"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n\t\t\t\"topic\": \"Reserve Zoom Room-01\",\n\t\t\t\"start_time\": \"2022-01-19T02:10:00Z\",\n\t\t\t\"end_time\": \"2022-01-19T03:55:00Z\",\n\t\t\t\"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\"\n\t\t}\n\t}\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "visitor_management.checked_in": {
      "post": {
        "tags": [
          "visitor_management"
        ],
        "operationId": "visitor_management.checked_in",
        "requestBody": {
          "description": "# visitor_management.checked_in\nThe **Visitor Management Checked In** event is triggered when a [visitor checks in](https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0073629#h_63082000168761703023137137).\n\n## Prerequisites\n\n* The [**Visitor Management**](https://support.zoom.com/hc/en?id=zm_kb&sysparm_article=KB0057564) feature enabled by your organization.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Visitor Management Checked In** subscription enabled under the **Visitor Management** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:visitor_checked_in:admin`\n\n**Event type**: `visitor_management.checked_in`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The host user account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the host."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the host."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the checked in invitation.",
                        "required": [
                          "visitor_email",
                          "visitor_name",
                          "start_time",
                          "end_time",
                          "invite_location_id",
                          "host_id",
                          "host_name",
                          "host_email"
                        ],
                        "properties": {
                          "visitor_email": {
                            "type": "string",
                            "description": "The email address of the invited visitor."
                          },
                          "visitor_name": {
                            "type": "string",
                            "description": "The name of the invited visitor."
                          },
                          "start_time": {
                            "type": "long",
                            "description": "The epoch timestamp representing the start time of the invitation."
                          },
                          "end_time": {
                            "type": "long",
                            "description": "The epoch timestamp representing end time of the invitation."
                          },
                          "invite_location_id": {
                            "type": "string",
                            "description": "The location ID of the location to which the visitor is invited to."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The user ID of the invitation host."
                          },
                          "host_name": {
                            "type": "string",
                            "description": "The name of the invitation host."
                          },
                          "host_email": {
                            "type": "string",
                            "description": "The email address of the invitation host."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"visitor_management.checked_in\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"start_time\": 1710153000000,\n      \"end_time\": 1710172799999,\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The host user account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the host."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the host."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the checked in invitation.",
                          "required": [
                            "visitor_email",
                            "visitor_name",
                            "start_time",
                            "end_time",
                            "invite_location_id",
                            "host_id",
                            "host_name",
                            "host_email"
                          ],
                          "properties": {
                            "visitor_email": {
                              "type": "string",
                              "description": "The email address of the invited visitor."
                            },
                            "visitor_name": {
                              "type": "string",
                              "description": "The name of the invited visitor."
                            },
                            "start_time": {
                              "type": "long",
                              "description": "The epoch timestamp representing the start time of the invitation."
                            },
                            "end_time": {
                              "type": "long",
                              "description": "The epoch timestamp representing end time of the invitation."
                            },
                            "invite_location_id": {
                              "type": "string",
                              "description": "The location ID of the location to which the visitor is invited to."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The user ID of the invitation host."
                            },
                            "host_name": {
                              "type": "string",
                              "description": "The name of the invitation host."
                            },
                            "host_email": {
                              "type": "string",
                              "description": "The email address of the invitation host."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"visitor_management.checked_in\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"start_time\": 1710153000000,\n      \"end_time\": 1710172799999,\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.reservation_deleted": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.reservation_deleted",
        "requestBody": {
          "description": "# workspace.reservation_deleted\n\nThe **Workspace Reservation Deleted** event is triggered when a user deletes their [Workspace reservation](https://support.zoom.us/hc/en-us/articles/4417951085965-Using-Workspace-Reservation-Beta-for-Zoom-Rooms).\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\n* The Zoom mobile app. This is required for QR code scanning:\n  * Android version 5.9.0.3502 or higher\n  * iOS version 5.9.0 or higher\n* [Zoom Rooms for Touch devices](https://support.zoom.us/hc/en-us/articles/115004773103), version 5.9.0 or higher (*Optional*).\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace Reservation Deleted** subscription enabled under the **Workspace Reservation** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:reservation`,`workspace:read:reservation:admin`\n\n**Event type**: `workspace.reservation_deleted`\n",
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that deleted the reservation."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that deleted the reservation."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the workspace reservation.",
                        "required": [
                          "id",
                          "workspace_id",
                          "end_time",
                          "start_time",
                          "topic"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The reservation ID."
                          },
                          "workspace_id": {
                            "type": "string",
                            "description": "The workspace ID."
                          },
                          "end_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's end time."
                          },
                          "start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's start time."
                          },
                          "topic": {
                            "type": "string",
                            "description": "The reservation's description."
                          },
                          "reserve_for": {
                            "type": "string",
                            "description": "The ID of the user for whom to create a workspace reservation.",
                            "example": "KDcuGIm1QgePTO8WbOqwIQ"
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
                  "value": "{\n\t\"event\": \"workspace.reservation_deleted\",\n\t\"event_ts\": \"290112312321\",\n\t\"payload\": {\n\t\t\"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n\t\t\"operator\" : \"admin@example.com\",\n\t    \"operator_id\" : \"I7zp-pYbTl22Z29H796Ixg\",\n\t\t\"object\": {\n\t\t\t\"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n\t\t\t\"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n\t\t\t\"topic\": \"Reserve Zoom Room-01\",\n\t\t\t\"start_time\": \"2022-01-19T02:10:00Z\",\n\t\t\t\"end_time\": \"2022-01-19T03:55:00Z\",\n\t\t\t\"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\"\n\t\t}\n\t}\n}"
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that deleted the reservation."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that deleted the reservation."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the workspace reservation.",
                          "required": [
                            "id",
                            "workspace_id",
                            "end_time",
                            "start_time",
                            "topic"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The reservation ID."
                            },
                            "workspace_id": {
                              "type": "string",
                              "description": "The workspace ID."
                            },
                            "end_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's end time."
                            },
                            "start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's start time."
                            },
                            "topic": {
                              "type": "string",
                              "description": "The reservation's description."
                            },
                            "reserve_for": {
                              "type": "string",
                              "description": "The ID of the user for whom to create a workspace reservation.",
                              "example": "KDcuGIm1QgePTO8WbOqwIQ"
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
                    "value": "{\n\t\"event\": \"workspace.reservation_deleted\",\n\t\"event_ts\": \"290112312321\",\n\t\"payload\": {\n\t\t\"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n\t\t\"operator\" : \"admin@example.com\",\n\t    \"operator_id\" : \"I7zp-pYbTl22Z29H796Ixg\",\n\t\t\"object\": {\n\t\t\t\"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n\t\t\t\"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n\t\t\t\"topic\": \"Reserve Zoom Room-01\",\n\t\t\t\"start_time\": \"2022-01-19T02:10:00Z\",\n\t\t\t\"end_time\": \"2022-01-19T03:55:00Z\",\n\t\t\t\"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\"\n\t\t}\n\t}\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "visitor_management.invitation_created": {
      "post": {
        "tags": [
          "visitor_management"
        ],
        "operationId": "visitor_management.invitation_created",
        "requestBody": {
          "description": "# visitor_management.invitation_created\n\nThe **Visitor Management Invitation Created** event is triggered when an [invitation is created](https://support.zoom.com/hc/en?id=zm_kb&sysparm_article=KB0073629#mcetoc_1hi1ubsr1fb).\n\n## Prerequisites\n\n* The [**Visitor Management**](https://support.zoom.com/hc/en?id=zm_kb&sysparm_article=KB0057564) feature enabled by your organization.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Visitor Management Invitation Created** subscription enabled under the **Visitor Management** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:visitor_invitation:admin`\n\n**Event type**: `visitor_management.invitation_created`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The user account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that created the invitation."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that created the invitation."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the created invitation.",
                        "required": [
                          "invitation_id",
                          "visitor_email",
                          "visitor_name",
                          "start_time",
                          "end_time",
                          "invite_location_id",
                          "host_id",
                          "host_name",
                          "host_email"
                        ],
                        "properties": {
                          "invitation_id": {
                            "type": "string",
                            "description": "The invitation ID."
                          },
                          "visitor_email": {
                            "type": "string",
                            "description": "The email address of the invited visitor."
                          },
                          "visitor_name": {
                            "type": "string",
                            "description": "The name of the invited visitor."
                          },
                          "start_time": {
                            "type": "long",
                            "description": "The epoch timestamp representing the start time of the invitation."
                          },
                          "end_time": {
                            "type": "long",
                            "description": "The epoch timestamp representing end time of the invitation."
                          },
                          "invite_location_id": {
                            "type": "string",
                            "description": "The location ID of the location to which the visitor is invited to."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The user ID of the invitation host."
                          },
                          "host_name": {
                            "type": "string",
                            "description": "The name of the invitation host."
                          },
                          "host_email": {
                            "type": "string",
                            "description": "The email address of the invitation host."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"visitor_management.invitation_created\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"invitation_id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"start_time\": 1710153000000,\n      \"end_time\": 1710172799999,\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The user account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that created the invitation."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that created the invitation."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the created invitation.",
                          "required": [
                            "invitation_id",
                            "visitor_email",
                            "visitor_name",
                            "start_time",
                            "end_time",
                            "invite_location_id",
                            "host_id",
                            "host_name",
                            "host_email"
                          ],
                          "properties": {
                            "invitation_id": {
                              "type": "string",
                              "description": "The invitation ID."
                            },
                            "visitor_email": {
                              "type": "string",
                              "description": "The email address of the invited visitor."
                            },
                            "visitor_name": {
                              "type": "string",
                              "description": "The name of the invited visitor."
                            },
                            "start_time": {
                              "type": "long",
                              "description": "The epoch timestamp representing the start time of the invitation."
                            },
                            "end_time": {
                              "type": "long",
                              "description": "The epoch timestamp representing end time of the invitation."
                            },
                            "invite_location_id": {
                              "type": "string",
                              "description": "The location ID of the location to which the visitor is invited to."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The user ID of the invitation host."
                            },
                            "host_name": {
                              "type": "string",
                              "description": "The name of the invitation host."
                            },
                            "host_email": {
                              "type": "string",
                              "description": "The email address of the invitation host."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"visitor_management.invitation_created\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"invitation_id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"start_time\": 1710153000000,\n      \"end_time\": 1710172799999,\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.location_deleted": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.location_deleted",
        "requestBody": {
          "description": "# zoomroom.location_deleted\nThe **Zoom Rooms location has been deleted** event is triggered every time a Zoom Room location is deleted in your account. Admins and account owners can delete Zoom Rooms locations in a Zoom account.  Deleting Zoom Rooms locations permanently removes them and their data from the Zoom system.  To delete a Zoom Rooms location, delete the Zoom Rooms location from the Zoom Web Portal itself.\n\n## Prerequisites\n\n* Have your organization enable the **Zoom Rooms** feature through a Zoom Rooms license, or Workspace Reservation feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or **Workspace Reservation** license.\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Rooms Location Deleted** subscription enabled under the **Zoom Rooms** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\n\n**Event type**: `zoomroom.location_deleted`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who deleted the Zoom Rooms location."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that deleted the Zoom Rooms location."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that deleted the Zoom Rooms location."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the deleted location.",
                        "required": [
                          "id",
                          "name",
                          "parent_location_id",
                          "type"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Rooms location's ID."
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the deleted Zoom Rooms location."
                          },
                          "parent_location_id": {
                            "type": "string",
                            "description": "This location's parent Zoom Rooms location ID."
                          },
                          "type": {
                            "type": "enum, country\u2503states\u2503city\u2503campus\u2503building\u2503floor",
                            "description": "The location type."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"zoomroom.location_deleted\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"name\": \"My Deleted Zoom Room Location\",\n      \"parent_location_id\": \"aldjflajsd-fasj\",\n      \"type\": \"floor\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who deleted the Zoom Rooms location."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that deleted the Zoom Rooms location."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that deleted the Zoom Rooms location."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the deleted location.",
                          "required": [
                            "id",
                            "name",
                            "parent_location_id",
                            "type"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Rooms location's ID."
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the deleted Zoom Rooms location."
                            },
                            "parent_location_id": {
                              "type": "string",
                              "description": "This location's parent Zoom Rooms location ID."
                            },
                            "type": {
                              "type": "enum, country\u2503states\u2503city\u2503campus\u2503building\u2503floor",
                              "description": "The location type."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"zoomroom.location_deleted\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"name\": \"My Deleted Zoom Room Location\",\n      \"parent_location_id\": \"aldjflajsd-fasj\",\n      \"type\": \"floor\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.reservation_created": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.reservation_created",
        "requestBody": {
          "description": "# workspace.reservation_created\n\nThe **Workspace Reservation Created** event is triggered when a user [reserves](https://support.zoom.us/hc/en-us/articles/4417951085965-Using-Workspace-Reservation-Beta-for-Zoom-Rooms) a Zoom Room or Zoom device-enabled desk.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\n* The Zoom mobile app. This is required for QR code scanning:\n  * Android version 5.9.0.3502 or higher\n  * iOS version 5.9.0 or higher\n* [Zoom Rooms for Touch devices](https://support.zoom.us/hc/en-us/articles/115004773103), version 5.9.0 or higher (*Optional*).\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace Reservation Created** subscription enabled under the **Workspace Reservation** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:reservation`,`workspace:read:reservation:admin`\n\n**Event type**: `workspace.reservation_created`\n",
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that created the reservation."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that created the reservation."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the workspace reservation.",
                        "required": [
                          "id",
                          "workspace_id",
                          "end_time",
                          "start_time",
                          "topic"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The reservation ID."
                          },
                          "workspace_id": {
                            "type": "string",
                            "description": "The workspace ID."
                          },
                          "end_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's end time."
                          },
                          "start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's start time."
                          },
                          "topic": {
                            "type": "string",
                            "description": "The reservation's description."
                          },
                          "reserve_for": {
                            "type": "string",
                            "description": "The ID of the user for whom to create a workspace reservation."
                          },
                          "assignment_checkin": {
                            "type": "boolean",
                            "description": "Whether the assigned workspace is checked in."
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
                  "value": "{\n  \"event\": \"workspace.reservation_created\",\n  \"event_ts\": 290112312321,\n  \"payload\": {\n    \"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n      \"topic\": \"Reserve Zoom Room-01\",\n      \"start_time\": \"2022-01-19T02:10:00Z\",\n      \"end_time\": \"2022-01-19T03:55:00Z\",\n      \"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"assignment_checkin\": true\n    }\n  }\n}"
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that created the reservation."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that created the reservation."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the workspace reservation.",
                          "required": [
                            "id",
                            "workspace_id",
                            "end_time",
                            "start_time",
                            "topic"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The reservation ID."
                            },
                            "workspace_id": {
                              "type": "string",
                              "description": "The workspace ID."
                            },
                            "end_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's end time."
                            },
                            "start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's start time."
                            },
                            "topic": {
                              "type": "string",
                              "description": "The reservation's description."
                            },
                            "reserve_for": {
                              "type": "string",
                              "description": "The ID of the user for whom to create a workspace reservation."
                            },
                            "assignment_checkin": {
                              "type": "boolean",
                              "description": "Whether the assigned workspace is checked in."
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
                    "value": "{\n  \"event\": \"workspace.reservation_created\",\n  \"event_ts\": 290112312321,\n  \"payload\": {\n    \"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n      \"topic\": \"Reserve Zoom Room-01\",\n      \"start_time\": \"2022-01-19T02:10:00Z\",\n      \"end_time\": \"2022-01-19T03:55:00Z\",\n      \"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"assignment_checkin\": true\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.checked_out": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.checked_out",
        "requestBody": {
          "description": "# zoomroom.checked_out\n\nThe **Zoom Room Checked Out** event is triggered when a user checks out of the Zoom Room to confirm that they are releasing the room for other users to use.\n\nUsers can check out manually via the Zoom Room controller or the scheduling display, or they can check out programmatically using the [**Check-in or check-out of a Zoom Room**](/api-reference/zoom-api/methods#operation/checkInRooms) API.\n\n## Prerequisites\n\n* The administrator or account owner must enable the [**Room Check-In and Check-Out**](https://support.zoom.us/hc/en-us/articles/360001147163-Using-the-Zoom-Rooms-check-in-feature) feature.\n* The Zoom Room version must be 4.7.0 or higher.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Room Checked Out** subscription enabled under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\n\n**Event type**: `zoomroom.checked_out`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "id",
                          "room_name",
                          "calendar_name",
                          "email",
                          "event_id",
                          "change_key",
                          "resource_email",
                          "calendar_id",
                          "calendar_type",
                          "api_type"
                        ],
                        "description": "Information about the Zoom Room.",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Room's unique ID."
                          },
                          "room_name": {
                            "type": "string",
                            "description": "The Zoom Room's name."
                          },
                          "calendar_name": {
                            "type": "string",
                            "description": "The calendar's name."
                          },
                          "email": {
                            "type": "string",
                            "format": "email",
                            "description": "The Zoom Room's associated email address."
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The unique ID of the Zoom Room's associated calendar event:\n\n* For Microsoft Exchange calendar integration, use Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). The event_id value is the Id value in the ItemId element of the FindItem operation's response. \n* For Google Calendar integration, use Google's [Events: list API](https://developers.google.com/calendar/api/v3/reference/events/list). The event_id value is the id field in the [events resource](https://developers.google.com/calendar/api/v3/reference/events#resource)."
                          },
                          "change_key": {
                            "type": "string",
                            "description": "The specific version of the calendar item. You can get this value using Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). Refer to the ChangeKey field of the ItemId element in the operation's response.\n\nThis field is only returned for Microsoft Exchange or Office 365 calendars."
                          },
                          "resource_email": {
                            "description": "The calendar's associated [resource mailbox](https://support.microsoft.com/en-us/help/10063/creating-and-managing-resource-mailboxes-in-office-365).\n\nThis field is only returned for Microsoft Exchange or Office 365 calendars.",
                            "type": "string"
                          },
                          "calendar_id": {
                            "type": "string",
                            "description": "The calendar's unique ID."
                          },
                          "calendar_type": {
                            "type": "string",
                            "description": "The calendar's type: \n* 0 \u2014 Google Calendar. \n* 1 \u2014 Office 365 Calendar. \n* 2 \u2014 Microsoft Exchange Calendar.",
                            "enum": [
                              "0",
                              "1",
                              "2"
                            ]
                          },
                          "api_type": {
                            "type": "string",
                            "description": "The API type: \n* 0 \u2014 [Exchange Web Services (EWS) API](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-managed-api-reference-for-exchange) \n* 1 \u2014 [Microsoft Graph API](https://docs.microsoft.com/en-us/azure/active-directory/develop/microsoft-graph-intro) \n* An empty string \u2014 The API returns this value when the calendar_type value is 0 (Google Calendar).",
                            "enum": [
                              "0",
                              "1",
                              ""
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
                  "value": "{\n  \"event\": \"zoomroom.checked_out\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"id\": \"abcD3ojfdbjfg\",\n      \"room_name\": \"My Zoom Room\",\n      \"calendar_name\": \"mycalendar@example.com\",\n      \"email\": \"jchill@example.com\",\n      \"event_id\": \"AbbbbbGYxLTc3OTVkMzFmZDc0MwBGAAAAAAD48FI58voYSqDgJePOSZblBwBQ/N0JvB/FRqv5UT2rFfkVAAAAAAENAABQ/N0JvB/FRqv5UT2rFfkVAAE2YC8DAAA=\",\n      \"change_key\": \"DwAAABYAAABQ/N0JvB/FRqv5UT2rFfkVAAE2XqVw\",\n      \"resource_email\": \"zroom1@example.com\",\n      \"calendar_id\": \"mycalendar@example.com\",\n      \"calendar_type\": \"2\",\n      \"api_type\": \"0\"\n    }\n  }\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "id",
                            "room_name",
                            "calendar_name",
                            "email",
                            "event_id",
                            "change_key",
                            "resource_email",
                            "calendar_id",
                            "calendar_type",
                            "api_type"
                          ],
                          "description": "Information about the Zoom Room.",
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Room's unique ID."
                            },
                            "room_name": {
                              "type": "string",
                              "description": "The Zoom Room's name."
                            },
                            "calendar_name": {
                              "type": "string",
                              "description": "The calendar's name."
                            },
                            "email": {
                              "type": "string",
                              "format": "email",
                              "description": "The Zoom Room's associated email address."
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The unique ID of the Zoom Room's associated calendar event:\n\n* For Microsoft Exchange calendar integration, use Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). The event_id value is the Id value in the ItemId element of the FindItem operation's response. \n* For Google Calendar integration, use Google's [Events: list API](https://developers.google.com/calendar/api/v3/reference/events/list). The event_id value is the id field in the [events resource](https://developers.google.com/calendar/api/v3/reference/events#resource)."
                            },
                            "change_key": {
                              "type": "string",
                              "description": "The specific version of the calendar item. You can get this value using Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). Refer to the ChangeKey field of the ItemId element in the operation's response.\n\nThis field is only returned for Microsoft Exchange or Office 365 calendars."
                            },
                            "resource_email": {
                              "description": "The calendar's associated [resource mailbox](https://support.microsoft.com/en-us/help/10063/creating-and-managing-resource-mailboxes-in-office-365).\n\nThis field is only returned for Microsoft Exchange or Office 365 calendars.",
                              "type": "string"
                            },
                            "calendar_id": {
                              "type": "string",
                              "description": "The calendar's unique ID."
                            },
                            "calendar_type": {
                              "type": "string",
                              "description": "The calendar's type: \n* 0 \u2014 Google Calendar. \n* 1 \u2014 Office 365 Calendar. \n* 2 \u2014 Microsoft Exchange Calendar.",
                              "enum": [
                                "0",
                                "1",
                                "2"
                              ]
                            },
                            "api_type": {
                              "type": "string",
                              "description": "The API type: \n* 0 \u2014 [Exchange Web Services (EWS) API](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-managed-api-reference-for-exchange) \n* 1 \u2014 [Microsoft Graph API](https://docs.microsoft.com/en-us/azure/active-directory/develop/microsoft-graph-intro) \n* An empty string \u2014 The API returns this value when the calendar_type value is 0 (Google Calendar).",
                              "enum": [
                                "0",
                                "1",
                                ""
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
                    "value": "{\n  \"event\": \"zoomroom.checked_out\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"id\": \"abcD3ojfdbjfg\",\n      \"room_name\": \"My Zoom Room\",\n      \"calendar_name\": \"mycalendar@example.com\",\n      \"email\": \"jchill@example.com\",\n      \"event_id\": \"AbbbbbGYxLTc3OTVkMzFmZDc0MwBGAAAAAAD48FI58voYSqDgJePOSZblBwBQ/N0JvB/FRqv5UT2rFfkVAAAAAAENAABQ/N0JvB/FRqv5UT2rFfkVAAE2YC8DAAA=\",\n      \"change_key\": \"DwAAABYAAABQ/N0JvB/FRqv5UT2rFfkVAAE2XqVw\",\n      \"resource_email\": \"zroom1@example.com\",\n      \"calendar_id\": \"mycalendar@example.com\",\n      \"calendar_type\": \"2\",\n      \"api_type\": \"0\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.delayed_alert": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.delayed_alert",
        "requestBody": {
          "description": "# zoomroom.delayed_alert\n\nThe **Zoom Room Delayed Alert** event is triggered when there is an issue with a Zoom Room on a Zoom account that has installed your [webhook-enabled](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks) app. Some of the issues that can trigger this event are:\n\n* High CPU usage.\n* Low battery, charging, and/or connection issues in a Zoom Room device.\n* Room controller, camera, or speaker disconnections or reconnections.\n* Missing camera or microphone.\n\nThis event notification is the same as the **Zoom Room Alert** event notification. However, instead of receiving the event in near real-time, you will receive this event notification **five minutes after an issue occurs in the Zoom Room**.\n\nIt is recommended that you **only** subscribe to either the **Zoom Room Alert** event or the **Zoom Room Delayed Alert** event to avoid receiving duplicate event notifications.\n\n## Prerequisites\n\n* A Business or Enterprise plan.\n* The administrator or account owner must enable the [**Zoom Room Alert**](https://support.zoom.us/hc/en-us/articles/205394099-Zoom-Room-Alerts) feature with the selected alerts in the [Zoom Web Portal](https://zoom.us/location).\n* The Zoom Room version must be 3.5.24524.0821 and higher.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Room Delayed Alert** subscription enabled under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:alert:admin`\n\n**Event type**: `zoomroom.delayed_alert`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "id",
                          "room_name",
                          "calendar_name",
                          "email",
                          "issue",
                          "alert_type",
                          "component",
                          "alert_time",
                          "alert_kind"
                        ],
                        "description": "Information about the Zoom Room.",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Room's unique ID."
                          },
                          "room_name": {
                            "type": "string",
                            "description": "The Zoom Room's name."
                          },
                          "calendar_name": {
                            "type": "string",
                            "description": "The calendar's name."
                          },
                          "email": {
                            "type": "string",
                            "format": "email",
                            "description": "The Zoom Room's associated email address."
                          },
                          "issue": {
                            "type": "string",
                            "description": "The issues that occurred during the meeting: \n* Room controller is disconnected. \n* Room controller connected. \n* Selected camera is disconnected. \n* Selected camera is reconnected. \n* Selected microphone is disconnected. \n* Selected microphone is reconnected. \n* Selected speaker is disconnected. \n* Selected speaker is reconnected. \n* Zoom room is offline. \n* Zoom room is online. \n* High CPU usage detected. \n* Low bandwidth network is detected. \n* {name} battery is low. \n* {name} battery is normal. \n* {name} disconnected. \n* {name} is connected.\n* Incorrect Room Passcode Entry Detected. \n* The possible values for {name} are: \n* Zoom Rooms Computer. \n* Controller. \n* Scheduling Display. \n*"
                          },
                          "alert_type": {
                            "type": "integer",
                            "description": "The type of alert sent: \n* 1 \u2014 The controller or scheduling display disconnected. \n* 2 \u2014 The selected microphone, speaker, or camera has disconnected. \n* 3 \u2014 The Zoom Room is offline. \n* 4 \u2014 High CPU usage detected. \n* 5 \u2014 Low network bandwidth detected. \n* 6 \u2014 The controller or scheduling display battery is low. \n* 7 \u2014 The Zoom Rooms display is disconnected. \n* 8 \u2014 The audio device has quality issues. \n* 9 \u2014 Failed SIP registration.\n* 10 \u2014 Incorrect room passcode entry detected.\n* 0 \u2014 Other alert.",
                            "enum": [
                              1,
                              2,
                              3,
                              4,
                              5,
                              6,
                              7,
                              8,
                              9,
                              10,
                              0
                            ]
                          },
                          "component": {
                            "type": "integer",
                            "description": "The affected component: \n* 1 \u2014 Zoom Rooms computer \n* 2 \u2014 Controller \n* 3 \u2014 Scheduling display \n* 4 \u2014 Microphone \n* 5 \u2014 Camera \n* 6 \u2014 Speaker \n* 0 \u2014 Other",
                            "enum": [
                              1,
                              2,
                              3,
                              4,
                              5,
                              6,
                              0
                            ]
                          },
                          "alert_time": {
                            "type": "integer",
                            "format": "int64",
                            "description": "The timestamp when the alert occurred."
                          },
                          "alert_kind": {
                            "type": "integer",
                            "description": "The kind of alert: \n* 1 \u2014 Triggered. There was an encountered problem. When the problem is resolved, a follow-up response will contain this field with the 2 (Cleared) value. \n* 2 \u2014 Cleared. The problem is resolved. \n* 3 \u2014 Observed. There is a potential issue-causing condition. If the issue is cleared (2), there will **not** be a follow-up response. \n* 4 \u2014 Other.",
                            "enum": [
                              1,
                              2,
                              3,
                              4
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
                  "value": "{\n  \"event\": \"zoomroom.delayed_alert\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"id\": \"abcD3ojfdbjfg\",\n      \"room_name\": \"My Zoom Room\",\n      \"calendar_name\": \"mycalendar@example.com\",\n      \"email\": \"jchill@example.com\",\n      \"issue\": \"Room Controller disconnected\",\n      \"alert_type\": 1,\n      \"component\": 2,\n      \"alert_kind\": 1\n    }\n  }\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "id",
                            "room_name",
                            "calendar_name",
                            "email",
                            "issue",
                            "alert_type",
                            "component",
                            "alert_time",
                            "alert_kind"
                          ],
                          "description": "Information about the Zoom Room.",
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Room's unique ID."
                            },
                            "room_name": {
                              "type": "string",
                              "description": "The Zoom Room's name."
                            },
                            "calendar_name": {
                              "type": "string",
                              "description": "The calendar's name."
                            },
                            "email": {
                              "type": "string",
                              "format": "email",
                              "description": "The Zoom Room's associated email address."
                            },
                            "issue": {
                              "type": "string",
                              "description": "The issues that occurred during the meeting: \n* Room controller is disconnected. \n* Room controller connected. \n* Selected camera is disconnected. \n* Selected camera is reconnected. \n* Selected microphone is disconnected. \n* Selected microphone is reconnected. \n* Selected speaker is disconnected. \n* Selected speaker is reconnected. \n* Zoom room is offline. \n* Zoom room is online. \n* High CPU usage detected. \n* Low bandwidth network is detected. \n* {name} battery is low. \n* {name} battery is normal. \n* {name} disconnected. \n* {name} is connected.\n* Incorrect Room Passcode Entry Detected. \n* The possible values for {name} are: \n* Zoom Rooms Computer. \n* Controller. \n* Scheduling Display. \n*"
                            },
                            "alert_type": {
                              "type": "integer",
                              "description": "The type of alert sent: \n* 1 \u2014 The controller or scheduling display disconnected. \n* 2 \u2014 The selected microphone, speaker, or camera has disconnected. \n* 3 \u2014 The Zoom Room is offline. \n* 4 \u2014 High CPU usage detected. \n* 5 \u2014 Low network bandwidth detected. \n* 6 \u2014 The controller or scheduling display battery is low. \n* 7 \u2014 The Zoom Rooms display is disconnected. \n* 8 \u2014 The audio device has quality issues. \n* 9 \u2014 Failed SIP registration.\n* 10 \u2014 Incorrect room passcode entry detected.\n* 0 \u2014 Other alert.",
                              "enum": [
                                1,
                                2,
                                3,
                                4,
                                5,
                                6,
                                7,
                                8,
                                9,
                                10,
                                0
                              ]
                            },
                            "component": {
                              "type": "integer",
                              "description": "The affected component: \n* 1 \u2014 Zoom Rooms computer \n* 2 \u2014 Controller \n* 3 \u2014 Scheduling display \n* 4 \u2014 Microphone \n* 5 \u2014 Camera \n* 6 \u2014 Speaker \n* 0 \u2014 Other",
                              "enum": [
                                1,
                                2,
                                3,
                                4,
                                5,
                                6,
                                0
                              ]
                            },
                            "alert_time": {
                              "type": "integer",
                              "format": "int64",
                              "description": "The timestamp when the alert occurred."
                            },
                            "alert_kind": {
                              "type": "integer",
                              "description": "The kind of alert: \n* 1 \u2014 Triggered. There was an encountered problem. When the problem is resolved, a follow-up response will contain this field with the 2 (Cleared) value. \n* 2 \u2014 Cleared. The problem is resolved. \n* 3 \u2014 Observed. There is a potential issue-causing condition. If the issue is cleared (2), there will **not** be a follow-up response. \n* 4 \u2014 Other.",
                              "enum": [
                                1,
                                2,
                                3,
                                4
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
                    "value": "{\n  \"event\": \"zoomroom.delayed_alert\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"id\": \"abcD3ojfdbjfg\",\n      \"room_name\": \"My Zoom Room\",\n      \"calendar_name\": \"mycalendar@example.com\",\n      \"email\": \"jchill@example.com\",\n      \"issue\": \"Room Controller disconnected\",\n      \"alert_type\": 1,\n      \"component\": 2,\n      \"alert_kind\": 1\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.deleted": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.deleted",
        "requestBody": {
          "description": "# workspace.deleted\nThe **Workspace Deleted** event is triggered every time a Workspace is deleted in your account. Admins and account owners can delete a Workspaces to an account. The Workspace can be deleted using the [Delete a Workspace API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/deleteWorkspace) or by deleting the Workspace using the Zoom Web Portal itself.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace Deleted** subscription enabled under the **Workspace** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:workspace`,`workspace:read:workspace:admin`\n\n**Event type**: `workspace.deleted`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who deleted the Workspace."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user who deleted the Workspace."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that deleted the Workspace."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the deleted Workspace.",
                        "required": [
                          "id",
                          "workspace_name"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Workspace ID."
                          },
                          "workspace_name": {
                            "type": "string",
                            "description": "The name of the Workspace."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"workspace.deleted\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\" : \"Workspace Name\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who deleted the Workspace."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user who deleted the Workspace."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that deleted the Workspace."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the deleted Workspace.",
                          "required": [
                            "id",
                            "workspace_name"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Workspace ID."
                            },
                            "workspace_name": {
                              "type": "string",
                              "description": "The name of the Workspace."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"workspace.deleted\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\" : \"Workspace Name\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.deleted": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.deleted",
        "requestBody": {
          "description": "# zoomroom.deleted\n\nThe **Zoom Room Deleted** event is triggered when a user deletes a [**Zoom Room**](https://support.zoom.us/hc/en-us/categories/200108436).\n\n## Prerequisites\n\n* The **Zoom Room** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Room Deleted** subscription enabled under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\n\n**Event type**: `zoomroom.deleted`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "id",
                          "room_name"
                        ],
                        "description": "Information about the Zoom Room.",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Room's unique ID."
                          },
                          "room_name": {
                            "type": "string",
                            "description": "The Zoom Room's name."
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
                  "value": "{\n  \"event\": \"zoomroom.deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n    \"object\": {\n      \"id\": \"2muAb65FTZCGjj6ginHIbQ\",\n      \"room_name\": \"My Zoom Room\"\n    }\n  }\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "id",
                            "room_name"
                          ],
                          "description": "Information about the Zoom Room.",
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Room's unique ID."
                            },
                            "room_name": {
                              "type": "string",
                              "description": "The Zoom Room's name."
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
                    "value": "{\n  \"event\": \"zoomroom.deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n    \"object\": {\n      \"id\": \"2muAb65FTZCGjj6ginHIbQ\",\n      \"room_name\": \"My Zoom Room\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "visitor_management.invitation_updated": {
      "post": {
        "tags": [
          "visitor_management"
        ],
        "operationId": "visitor_management.invitation_updated",
        "requestBody": {
          "description": "# visitor_management.invitation_updated\n\nThe **Visitor Management Invitation Updated** event is triggered when the [invitation is updated](https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0073629#mcetoc_1hi1ubsr1ff).\n\n## Prerequisites\n\n* The [**Visitor Management**](https://support.zoom.com/hc/en?id=zm_kb&sysparm_article=KB0057564) feature enabled by your organization.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Visitor Management Invitation Updated** subscription enabled under the **Visitor Management** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:visitor_invitation:admin`\n\n**Event type**: `visitor_management.invitation_updated`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The user account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that updated the invitation."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that updated the invitation."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the updated invitation.",
                        "required": [
                          "invitation_id",
                          "visitor_email",
                          "visitor_name",
                          "start_time",
                          "end_time",
                          "invite_location_id",
                          "host_id",
                          "host_name",
                          "host_email"
                        ],
                        "properties": {
                          "invitation_id": {
                            "type": "string",
                            "description": "The invitation ID."
                          },
                          "visitor_email": {
                            "type": "string",
                            "description": "The email address of the invited visitor."
                          },
                          "visitor_name": {
                            "type": "string",
                            "description": "The name of the invited visitor."
                          },
                          "start_time": {
                            "type": "long",
                            "description": "The epoch timestamp representing the start time of the invitation."
                          },
                          "end_time": {
                            "type": "long",
                            "description": "The epoch timestamp representing end time of the invitation."
                          },
                          "invite_location_id": {
                            "type": "string",
                            "description": "The location ID of the location to which the visitor is invited to."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The user ID of the invitation host."
                          },
                          "host_name": {
                            "type": "string",
                            "description": "The name of the invitation host."
                          },
                          "host_email": {
                            "type": "string",
                            "description": "The email address of the invitation host."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"visitor_management.invitation_updated\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"invitation_id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"start_time\": 1710153000000,\n      \"end_time\": 1710172799999,\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The user account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that updated the invitation."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that updated the invitation."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the updated invitation.",
                          "required": [
                            "invitation_id",
                            "visitor_email",
                            "visitor_name",
                            "start_time",
                            "end_time",
                            "invite_location_id",
                            "host_id",
                            "host_name",
                            "host_email"
                          ],
                          "properties": {
                            "invitation_id": {
                              "type": "string",
                              "description": "The invitation ID."
                            },
                            "visitor_email": {
                              "type": "string",
                              "description": "The email address of the invited visitor."
                            },
                            "visitor_name": {
                              "type": "string",
                              "description": "The name of the invited visitor."
                            },
                            "start_time": {
                              "type": "long",
                              "description": "The epoch timestamp representing the start time of the invitation."
                            },
                            "end_time": {
                              "type": "long",
                              "description": "The epoch timestamp representing end time of the invitation."
                            },
                            "invite_location_id": {
                              "type": "string",
                              "description": "The location ID of the location to which the visitor is invited to."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The user ID of the invitation host."
                            },
                            "host_name": {
                              "type": "string",
                              "description": "The name of the invitation host."
                            },
                            "host_email": {
                              "type": "string",
                              "description": "The email address of the invitation host."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"visitor_management.invitation_updated\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"invitation_id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"start_time\": 1710153000000,\n      \"end_time\": 1710172799999,\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.location_created": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.location_created",
        "requestBody": {
          "description": "# zoomroom.location_created\nThe **Zoom Rooms location has been created** event is triggered every time a new Zoom Room location is created in your account. Admins and account owners can add new Zoom Room Location to an account. The Location can be added using the [Add a Zoom Rooms Location API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/addAZRLocation) or by adding the Zoom Room Location using the Zoom Web Portal itself.\n\n## Prerequisites\n\n* Have your organization enable the **Zoom Rooms** feature through a Zoom Rooms license, or Workspace Reservation feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or **Workspace Reservation** license.\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Rooms Location Created** subscription enabled under the **Zoom Rooms** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\n\n**Event type**: `zoomroom.location_created`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who created the new Zoom Rooms location."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that created the new Zoom Rooms location."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that created the new Zoom Rooms location."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the created location.",
                        "required": [
                          "id",
                          "name",
                          "parent_location_id",
                          "type"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Rooms location's ID."
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the new Zoom Rooms location."
                          },
                          "parent_location_id": {
                            "type": "string",
                            "description": "This location's parent Zoom Rooms location ID."
                          },
                          "type": {
                            "type": "enum, country\u2503states\u2503city\u2503campus\u2503building\u2503floor",
                            "description": "The location's type."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"zoomroom.location_created\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"name\": \"My new Zoom Room Location\",\n      \"parent_location_id\": \"aldjflajsd-fasj\",\n      \"type\": \"floor\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who created the new Zoom Rooms location."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that created the new Zoom Rooms location."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that created the new Zoom Rooms location."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the created location.",
                          "required": [
                            "id",
                            "name",
                            "parent_location_id",
                            "type"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Rooms location's ID."
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the new Zoom Rooms location."
                            },
                            "parent_location_id": {
                              "type": "string",
                              "description": "This location's parent Zoom Rooms location ID."
                            },
                            "type": {
                              "type": "enum, country\u2503states\u2503city\u2503campus\u2503building\u2503floor",
                              "description": "The location's type."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"zoomroom.location_created\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"name\": \"My new Zoom Room Location\",\n      \"parent_location_id\": \"aldjflajsd-fasj\",\n      \"type\": \"floor\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.created": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.created",
        "requestBody": {
          "description": "# zoomroom.created\n\nThe **Zoom Room Created** event is triggered when a user creates a [**Zoom Room**](https://support.zoom.us/hc/en-us/categories/200108436)\n\n## Prerequisites\n\n* The **Zoom Room** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Room Created** subscription enabled under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\n\n**Event type**: `zoomroom.created`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "id",
                          "room_name"
                        ],
                        "description": "Information about the Zoom Room.",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Room's unique ID."
                          },
                          "room_name": {
                            "type": "string",
                            "description": "The Zoom Room's name."
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
                  "value": "{\n  \"event\": \"zoomroom.created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n    \"object\": {\n      \"id\": \"2muAb65FTZCGjj6ginHIbQ\",\n      \"room_name\": \"My Zoom Room\"\n    }\n  }\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "id",
                            "room_name"
                          ],
                          "description": "Information about the Zoom Room.",
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Room's unique ID."
                            },
                            "room_name": {
                              "type": "string",
                              "description": "The Zoom Room's name."
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
                    "value": "{\n  \"event\": \"zoomroom.created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n    \"object\": {\n      \"id\": \"2muAb65FTZCGjj6ginHIbQ\",\n      \"room_name\": \"My Zoom Room\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.created": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.created",
        "requestBody": {
          "description": "# workspace.created\nThe **Workspace created** event is triggered every time a Workspace is created in your account. Admins and account owners can add new Workspaces to an account. The Workspace can be added using the [Create a Workspace API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/createWorkspace) or by adding the Workspace using the Zoom Web Portal itself.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace Created** subscription enabled under the **Workspace** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:workspace`,`workspace:read:workspace:admin`\n\n**Event type**: `workspace.created`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that created the workspace."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that created the workspace."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the created workspace.",
                        "required": [
                          "id",
                          "workspace_name"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Workspace ID."
                          },
                          "workspace_name": {
                            "type": "string",
                            "description": "The name of the workspace"
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"workspace.created\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\": \"My new workspace\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that created the workspace."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that created the workspace."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the created workspace.",
                          "required": [
                            "id",
                            "workspace_name"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Workspace ID."
                            },
                            "workspace_name": {
                              "type": "string",
                              "description": "The name of the workspace"
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"workspace.created\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\": \"My new workspace\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.updated": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.updated",
        "requestBody": {
          "description": "# workspace.updated\nThe **Workspace Updated** event is triggered every time a Workspace is updated in your account. Admins and account owners can update a Workspaces to an account. The Workspace can be updated using the [Update a Workspace API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/updateWorkspace) or by updating the Workspace using the Zoom Web Portal itself.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace Updated** subscription enabled under the **Workspace** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:workspace`,`workspace:read:workspace:admin`\n\n**Event type**: `workspace.updated`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The Workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that updated the Workspace."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that updated the Workspace."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the updated Workspace.",
                        "required": [
                          "id",
                          "workspace_name"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Workspace ID."
                          },
                          "workspace_name": {
                            "type": "string",
                            "description": "The name of the Workspace"
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"workspace.updated\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\": \"My updated workspace\"\n      }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The Workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that updated the Workspace."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that updated the Workspace."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the updated Workspace.",
                          "required": [
                            "id",
                            "workspace_name"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Workspace ID."
                            },
                            "workspace_name": {
                              "type": "string",
                              "description": "The name of the Workspace"
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"workspace.updated\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\": \"My updated workspace\"\n      }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.thirdparty_meeting_attempted": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.thirdparty_meeting_attempted",
        "requestBody": {
          "description": "# zoomroom.thirdparty_meeting_attempted\n\nThe **Zoom Room third party meeting attempted** event triggers when a Zoom Room is invited or actively joins a third party meeting.\n\n## Prerequisites\n\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n* A valid **Event Notification Endpoint URL**.\n* Enable the **Zoom Room third party meeting attempted** subscription under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room_control:admin`\n\n**Event type**: `zoomroom.thirdparty_meeting_attempted`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "room_id"
                        ],
                        "description": "A Zoom Room is about to join a meeting.",
                        "properties": {
                          "room_id": {
                            "type": "string",
                            "description": "The Zoom Room ID."
                          },
                          "join_url": {
                            "type": "string",
                            "description": "The URL for joining a meeting."
                          },
                          "meeting_source_type": {
                            "type": "string",
                            "description": "The meeting type.",
                            "enum": [
                              "MS_TEAMS",
                              "GOOGLE_MEET",
                              "WEBEX",
                              "BLUEJEANS",
                              "FUZE",
                              "UNKNOWN"
                            ]
                          },
                          "join_time": {
                            "description": "The time to execute the command to join a meeting.",
                            "type": "string",
                            "format": "datetime",
                            "example": "2022-06-19T00:00:00Z"
                          },
                          "sip_address": {
                            "type": "string",
                            "description": "The full SIP address to join the third-party meeting."
                          },
                          "third_party_meeting_id": {
                            "type": "string",
                            "description": "The of the third-party meeting's meeting ID."
                          },
                          "passcode": {
                            "type": "string",
                            "description": "The passcode for joining a third-party meeting."
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
                  "value": "{\n\t\"event\": \"zoomroom.thirdparty_meeting_attempted\",\n\t\"event_ts\": 1626230691572,\n\t\"payload\": {\n\t\t\"account_id\": \"AAAAAABBBB\",\n\t\t\"object\": {\n\t\t\t\"room_id\": \"abcD3ojfdbjfg\",\n            \"join_time\": \"2022-07-27T16:56:34Z\",\n            \"meeting_source_type\": \"ZOOM\",\n            \"join_url\": \"url\",\n            \"sip_address\": \"sip address\",\n            \"third_party_meeting_id\": \"12121313123\",\n            \"passcode\": \"123\"\n\t\t}\n\t}\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "room_id"
                          ],
                          "description": "A Zoom Room is about to join a meeting.",
                          "properties": {
                            "room_id": {
                              "type": "string",
                              "description": "The Zoom Room ID."
                            },
                            "join_url": {
                              "type": "string",
                              "description": "The URL for joining a meeting."
                            },
                            "meeting_source_type": {
                              "type": "string",
                              "description": "The meeting type.",
                              "enum": [
                                "MS_TEAMS",
                                "GOOGLE_MEET",
                                "WEBEX",
                                "BLUEJEANS",
                                "FUZE",
                                "UNKNOWN"
                              ]
                            },
                            "join_time": {
                              "description": "The time to execute the command to join a meeting.",
                              "type": "string",
                              "format": "datetime",
                              "example": "2022-06-19T00:00:00Z"
                            },
                            "sip_address": {
                              "type": "string",
                              "description": "The full SIP address to join the third-party meeting."
                            },
                            "third_party_meeting_id": {
                              "type": "string",
                              "description": "The of the third-party meeting's meeting ID."
                            },
                            "passcode": {
                              "type": "string",
                              "description": "The passcode for joining a third-party meeting."
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
                    "value": "{\n\t\"event\": \"zoomroom.thirdparty_meeting_attempted\",\n\t\"event_ts\": 1626230691572,\n\t\"payload\": {\n\t\t\"account_id\": \"AAAAAABBBB\",\n\t\t\"object\": {\n\t\t\t\"room_id\": \"abcD3ojfdbjfg\",\n            \"join_time\": \"2022-07-27T16:56:34Z\",\n            \"meeting_source_type\": \"ZOOM\",\n            \"join_url\": \"url\",\n            \"sip_address\": \"sip address\",\n            \"third_party_meeting_id\": \"12121313123\",\n            \"passcode\": \"123\"\n\t\t}\n\t}\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.digital_signage_app_started": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.digital_signage_app_started",
        "requestBody": {
          "description": "# zoomroom.digital_signage_app_started\n\nThe **[Zoom Room digital signage](https://support.zoom.us/hc/en-us/articles/360000030683-Zoom-Rooms-digital-signage) play started** event triggers when the digital signage starts playing or showing.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n* A valid **Event Notification Endpoint URL**.\n* The **Zoom Room digital signage app started** subscription enabled under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:digital_signage_app:admin`\n\n**Event type**: `zoomroom.digital_signage_app_started`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "room_id",
                          "app_name",
                          "app_id",
                          "start_time"
                        ],
                        "description": "The information on the digital signage played in a Zoom Room.",
                        "properties": {
                          "room_id": {
                            "type": "string",
                            "description": "The Zoom Room ID."
                          },
                          "app_name": {
                            "type": "string",
                            "description": "The app name."
                          },
                          "app_id": {
                            "type": "string",
                            "description": "The app ID."
                          },
                          "start_time": {
                            "description": "The playback start time of the digital signage.",
                            "type": "string",
                            "format": "datetime",
                            "example": "2022-06-19T00:00:00Z"
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
                  "value": "{\n\t\"event\": \"zoomroom.digital_signage_play_started\",\n\t\"event_ts\": 1626230691572,\n\t\"payload\": {\n\t\t\"account_id\": \"AAAAAABBBB\",\n\t\t\"object\": {\n\t\t\t\"room_id\": \"abcD3ojfdbjfg\",\n\t\t\t\"app_name\": \"My Zoom Room\",\n\t\t\t\"app_id\": \"NiVvY1NWpE2nulNrhVjgU4jD0swziVrXBbaFZyC3u+o=\",\n\t\t\t\"start_time\": \"2022-06-19T00:00:00Z\",\n            \"end_time\": \"2022-06-19T01:01:00Z\"\n\t\t}\n\t}\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "room_id",
                            "app_name",
                            "app_id",
                            "start_time"
                          ],
                          "description": "The information on the digital signage played in a Zoom Room.",
                          "properties": {
                            "room_id": {
                              "type": "string",
                              "description": "The Zoom Room ID."
                            },
                            "app_name": {
                              "type": "string",
                              "description": "The app name."
                            },
                            "app_id": {
                              "type": "string",
                              "description": "The app ID."
                            },
                            "start_time": {
                              "description": "The playback start time of the digital signage.",
                              "type": "string",
                              "format": "datetime",
                              "example": "2022-06-19T00:00:00Z"
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
                    "value": "{\n\t\"event\": \"zoomroom.digital_signage_play_started\",\n\t\"event_ts\": 1626230691572,\n\t\"payload\": {\n\t\t\"account_id\": \"AAAAAABBBB\",\n\t\t\"object\": {\n\t\t\t\"room_id\": \"abcD3ojfdbjfg\",\n\t\t\t\"app_name\": \"My Zoom Room\",\n\t\t\t\"app_id\": \"NiVvY1NWpE2nulNrhVjgU4jD0swziVrXBbaFZyC3u+o=\",\n\t\t\t\"start_time\": \"2022-06-19T00:00:00Z\",\n            \"end_time\": \"2022-06-19T01:01:00Z\"\n\t\t}\n\t}\n}"
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
    "title": "Rooms",
    "description": "The Zoom Rooms webhooks allow developers to receive events for [Zoom Rooms](https://developers.zoom.us/docs/zoom-rooms/) features.",
    "version": "1.0.0"
  },
  "webhooks": {
    "visitor_management.checked_out": {
      "post": {
        "tags": [
          "visitor_management"
        ],
        "operationId": "visitor_management.checked_out",
        "requestBody": {
          "description": "# visitor_management.checked_out\nThe **Visitor Management Checked Out** event is triggered when a visitor checks out.\n\n## Prerequisites\n\n* The [**Visitor Management**](https://support.zoom.com/hc/en?id=zm_kb&sysparm_article=KB0057564) feature enabled by your organization.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Visitor Management Checked Out** subscription enabled under the **Visitor Management** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:visitor_checked_out:admin`\n\n**Event type**: `visitor_management.checked_out`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The host user account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the host."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the host."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the checked out invitation.",
                        "required": [
                          "visitor_email",
                          "visitor_name",
                          "start_time",
                          "end_time",
                          "invite_location_id",
                          "host_id",
                          "host_name",
                          "host_email"
                        ],
                        "properties": {
                          "visitor_email": {
                            "type": "string",
                            "description": "The email address of the invited visitor."
                          },
                          "visitor_name": {
                            "type": "string",
                            "description": "The name of the invited visitor."
                          },
                          "start_time": {
                            "type": "long",
                            "description": "The epoch timestamp representing the start time of the invitation."
                          },
                          "end_time": {
                            "type": "long",
                            "description": "The epoch timestamp representing end time of the invitation."
                          },
                          "invite_location_id": {
                            "type": "string",
                            "description": "The location ID of the location to which the visitor is invited to."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The user ID of the invitation host."
                          },
                          "host_name": {
                            "type": "string",
                            "description": "The name of the invitation host."
                          },
                          "host_email": {
                            "type": "string",
                            "description": "The email address of the invitation host."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"visitor_management.checked_out\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"start_time\": 1710153000000,\n      \"end_time\": 1710172799999,\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The host user account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the host."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the host."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the checked out invitation.",
                          "required": [
                            "visitor_email",
                            "visitor_name",
                            "start_time",
                            "end_time",
                            "invite_location_id",
                            "host_id",
                            "host_name",
                            "host_email"
                          ],
                          "properties": {
                            "visitor_email": {
                              "type": "string",
                              "description": "The email address of the invited visitor."
                            },
                            "visitor_name": {
                              "type": "string",
                              "description": "The name of the invited visitor."
                            },
                            "start_time": {
                              "type": "long",
                              "description": "The epoch timestamp representing the start time of the invitation."
                            },
                            "end_time": {
                              "type": "long",
                              "description": "The epoch timestamp representing end time of the invitation."
                            },
                            "invite_location_id": {
                              "type": "string",
                              "description": "The location ID of the location to which the visitor is invited to."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The user ID of the invitation host."
                            },
                            "host_name": {
                              "type": "string",
                              "description": "The name of the invitation host."
                            },
                            "host_email": {
                              "type": "string",
                              "description": "The email address of the invitation host."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"visitor_management.checked_out\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"start_time\": 1710153000000,\n      \"end_time\": 1710172799999,\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.user_unassigned": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.user_unassigned",
        "requestBody": {
          "description": "# workspace.user_unassigned\nThe **Workspace User Unassigned** event is triggered every time a user is unassigned from a Workspace in your account. Admins and account owners can unassign a user from Workspace in you account. The Workspace can be unassign users using the [Delete a desk assignment API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/Deleteadeskassignment) or by deleting the Workspace' assignment using the Zoom Web Portal itself.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace User Unassigned** subscription enabled under the **Workspace** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:assignment`,`workspace:read:assignment:admin`\n\n**Event type**: `workspace.user_unassigned`\n",
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
                    "type": "string",
                    "format": "date-time",
                    "description": "A timestamp when the event occurred."
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that updated the assignment."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that updated the assignment."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the user unassigned when unassign a user from a workspace.",
                        "required": [
                          "workspace_id",
                          "workspace_name",
                          "account_id",
                          "user_id",
                          "assign_type"
                        ],
                        "properties": {
                          "workspace_id": {
                            "type": "string",
                            "description": "The workspace's ID."
                          },
                          "workspace_name": {
                            "type": "string",
                            "description": "The workspace's name."
                          },
                          "account_id": {
                            "type": "string",
                            "description": "The belong's account id of assigned user."
                          },
                          "user_id": {
                            "type": "string",
                            "description": "The id of assigned user."
                          },
                          "assign_type": {
                            "type": "string",
                            "description": "The type of assign before unassign. ",
                            "enum": [
                              "permanent"
                            ]
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"workspace.user_unassigned\",\n  \"event_ts\": 1707848572310,\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"operator\": \"vic-devint@zoom.us\",\n    \"object\": {\n      \"workspace_id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"account_id\": \"bhmIXpdYRcCK6BB3VhMq6w\",\n      \"user_id\": \"oyLyxW48QeC3uBBFY2CF3A\",\n      \"assign_type\": \"permanent\"\n    }\n  }\n}"
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
                      "type": "string",
                      "format": "date-time",
                      "description": "A timestamp when the event occurred."
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that updated the assignment."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that updated the assignment."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the user unassigned when unassign a user from a workspace.",
                          "required": [
                            "workspace_id",
                            "workspace_name",
                            "account_id",
                            "user_id",
                            "assign_type"
                          ],
                          "properties": {
                            "workspace_id": {
                              "type": "string",
                              "description": "The workspace's ID."
                            },
                            "workspace_name": {
                              "type": "string",
                              "description": "The workspace's name."
                            },
                            "account_id": {
                              "type": "string",
                              "description": "The belong's account id of assigned user."
                            },
                            "user_id": {
                              "type": "string",
                              "description": "The id of assigned user."
                            },
                            "assign_type": {
                              "type": "string",
                              "description": "The type of assign before unassign. ",
                              "enum": [
                                "permanent"
                              ]
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"workspace.user_unassigned\",\n  \"event_ts\": 1707848572310,\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"operator\": \"vic-devint@zoom.us\",\n    \"object\": {\n      \"workspace_id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"account_id\": \"bhmIXpdYRcCK6BB3VhMq6w\",\n      \"user_id\": \"oyLyxW48QeC3uBBFY2CF3A\",\n      \"assign_type\": \"permanent\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.reservation_questionnaire_response_created": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.reservation_questionnaire_response_created",
        "requestBody": {
          "description": "# workspace.reservation_questionnaire_response_created\nThe **Workspace reservation questionnaire created** event is triggered every time when a questionnaire is submitted as a part of workspace reservation. Admin and Users can create a workspace reservation and submit a questionnaire during the reservation creation process.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace reservation questionnaire created** subscription enabled under the **Workspace** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:workspace_reservation_questionnaire_response`,`workspace:read:workspace_reservation_questionnaire_response:admin`\n\n**Event type**: `workspace.reservation_questionnaire_response_created`\n",
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
                    "type": "string",
                    "format": "date-time",
                    "description": "A timestamp when the event occurred."
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that completed the questionnaire."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that completed the questionnaire."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the questionnaire when creating a reservation for a workspace.",
                        "required": [
                          "start_time",
                          "event_id",
                          "user_name",
                          "desk_id",
                          "workspace_name",
                          "location_id",
                          "questionnaire_response"
                        ],
                        "properties": {
                          "start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The start time of the workspace reservation."
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The event ID."
                          },
                          "user_name": {
                            "type": "string",
                            "description": "The user name."
                          },
                          "desk_id": {
                            "type": "string",
                            "description": "The desk ID."
                          },
                          "workspace_name": {
                            "type": "string",
                            "description": "The name of the workspace."
                          },
                          "location_id": {
                            "type": "string",
                            "description": "The location ID."
                          },
                          "questionnaire_response": {
                            "type": "array",
                            "description": "List of questionnaires.",
                            "items": {
                              "type": "object",
                              "required": [
                                "id",
                                "title",
                                "subject",
                                "list"
                              ],
                              "properties": {
                                "id": {
                                  "type": "string",
                                  "description": "The view ID."
                                },
                                "title": {
                                  "type": "string",
                                  "description": "The title of the questionnaire."
                                },
                                "subject": {
                                  "type": "string",
                                  "description": "The subject of the questionnaire."
                                },
                                "list": {
                                  "type": "array",
                                  "description": "List of questions in the questionnaire.",
                                  "items": {
                                    "type": "object",
                                    "required": [
                                      "question_id",
                                      "title",
                                      "type",
                                      "options",
                                      "reply_id"
                                    ],
                                    "properties": {
                                      "question_id": {
                                        "type": "string",
                                        "description": "The question ID."
                                      },
                                      "title": {
                                        "type": "string",
                                        "description": "The title of the question."
                                      },
                                      "type": {
                                        "type": "string",
                                        "description": "The type of the question (single-choice, multiple-choice, open-text)."
                                      },
                                      "reply_id": {
                                        "type": "string",
                                        "description": "The reply ID."
                                      },
                                      "options": {
                                        "type": "array",
                                        "description": "List of options for multiple-choice questions.",
                                        "items": {
                                          "type": "object",
                                          "required": [
                                            "option_id",
                                            "text"
                                          ],
                                          "properties": {
                                            "option_id": {
                                              "type": "string",
                                              "description": "The option ID."
                                            },
                                            "text": {
                                              "type": "string",
                                              "description": "The text of the option."
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
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"workspace.reservation_questionnaire_response_created\",\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator\": \"yuki-devint@zoom.us\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"object\": {\n      \"start_time\": \"2024-02-14T03:50:00Z\",\n      \"event_id\": \"dxpwbc2fi8tzz7d012rwx9u8c0\",\n      \"user_name\": \"Simash pokh\",\n      \"desk_id\": \"SJjU1CcGSs2L7odn6PanQw\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"questionnaire_response\": [\n        {\n          \"id\": \"Fl75C43YTmS_IpMzYLipJw\",\n          \"title\": \"Simash Reservation Questionnaire\",\n          \"list\": [\n            {\n              \"question_id\": \"0ds1NFeDQ1WfWlgVK-7OMQ\",\n              \"title\": \"do you like AI ?\",\n              \"type\": \"single\",\n              \"options\": [\n                {\n                  \"option_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\",\n                  \"text\": \"Yes\"\n                },\n                {\n                  \"option_id\": \"1S1hZzFGTwW9Llxdr2vzyw\",\n                  \"text\": \"No\"\n                }\n              ],\n              \"reply_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\"\n            },\n            {\n              \"question_id\": \"2XMaJFwNSr-BJpTfo6rRAQ\",\n              \"title\": \"what is your fav food ? \",\n              \"type\": \"multiple\",\n              \"options\": [\n                {\n                  \"option_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ\",\n                  \"text\": \"momo\"\n                },\n                {\n                  \"option_id\": \"dpv4EZ7QQUqRhBmYj4_b7w\",\n                  \"text\": \"sea food\"\n                },\n                {\n                  \"option_id\": \"__DjlrV7TeyCSZVmCWAnng\",\n                  \"text\": \"rice\"\n                }\n              ],\n              \"reply_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ,dpv4EZ7QQUqRhBmYj4_b7w\"\n            }\n          ],\n          \"subject\": \"reservation\"\n        }\n      ]\n    }\n  },\n  \"event_ts\": 1707848572310\n}"
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
                      "type": "string",
                      "format": "date-time",
                      "description": "A timestamp when the event occurred."
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that completed the questionnaire."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that completed the questionnaire."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the questionnaire when creating a reservation for a workspace.",
                          "required": [
                            "start_time",
                            "event_id",
                            "user_name",
                            "desk_id",
                            "workspace_name",
                            "location_id",
                            "questionnaire_response"
                          ],
                          "properties": {
                            "start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The start time of the workspace reservation."
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The event ID."
                            },
                            "user_name": {
                              "type": "string",
                              "description": "The user name."
                            },
                            "desk_id": {
                              "type": "string",
                              "description": "The desk ID."
                            },
                            "workspace_name": {
                              "type": "string",
                              "description": "The name of the workspace."
                            },
                            "location_id": {
                              "type": "string",
                              "description": "The location ID."
                            },
                            "questionnaire_response": {
                              "type": "array",
                              "description": "List of questionnaires.",
                              "items": {
                                "type": "object",
                                "required": [
                                  "id",
                                  "title",
                                  "subject",
                                  "list"
                                ],
                                "properties": {
                                  "id": {
                                    "type": "string",
                                    "description": "The view ID."
                                  },
                                  "title": {
                                    "type": "string",
                                    "description": "The title of the questionnaire."
                                  },
                                  "subject": {
                                    "type": "string",
                                    "description": "The subject of the questionnaire."
                                  },
                                  "list": {
                                    "type": "array",
                                    "description": "List of questions in the questionnaire.",
                                    "items": {
                                      "type": "object",
                                      "required": [
                                        "question_id",
                                        "title",
                                        "type",
                                        "options",
                                        "reply_id"
                                      ],
                                      "properties": {
                                        "question_id": {
                                          "type": "string",
                                          "description": "The question ID."
                                        },
                                        "title": {
                                          "type": "string",
                                          "description": "The title of the question."
                                        },
                                        "type": {
                                          "type": "string",
                                          "description": "The type of the question (single-choice, multiple-choice, open-text)."
                                        },
                                        "reply_id": {
                                          "type": "string",
                                          "description": "The reply ID."
                                        },
                                        "options": {
                                          "type": "array",
                                          "description": "List of options for multiple-choice questions.",
                                          "items": {
                                            "type": "object",
                                            "required": [
                                              "option_id",
                                              "text"
                                            ],
                                            "properties": {
                                              "option_id": {
                                                "type": "string",
                                                "description": "The option ID."
                                              },
                                              "text": {
                                                "type": "string",
                                                "description": "The text of the option."
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
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"workspace.reservation_questionnaire_response_created\",\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator\": \"yuki-devint@zoom.us\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"object\": {\n      \"start_time\": \"2024-02-14T03:50:00Z\",\n      \"event_id\": \"dxpwbc2fi8tzz7d012rwx9u8c0\",\n      \"user_name\": \"Simash pokh\",\n      \"desk_id\": \"SJjU1CcGSs2L7odn6PanQw\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"questionnaire_response\": [\n        {\n          \"id\": \"Fl75C43YTmS_IpMzYLipJw\",\n          \"title\": \"Simash Reservation Questionnaire\",\n          \"list\": [\n            {\n              \"question_id\": \"0ds1NFeDQ1WfWlgVK-7OMQ\",\n              \"title\": \"do you like AI ?\",\n              \"type\": \"single\",\n              \"options\": [\n                {\n                  \"option_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\",\n                  \"text\": \"Yes\"\n                },\n                {\n                  \"option_id\": \"1S1hZzFGTwW9Llxdr2vzyw\",\n                  \"text\": \"No\"\n                }\n              ],\n              \"reply_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\"\n            },\n            {\n              \"question_id\": \"2XMaJFwNSr-BJpTfo6rRAQ\",\n              \"title\": \"what is your fav food ? \",\n              \"type\": \"multiple\",\n              \"options\": [\n                {\n                  \"option_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ\",\n                  \"text\": \"momo\"\n                },\n                {\n                  \"option_id\": \"dpv4EZ7QQUqRhBmYj4_b7w\",\n                  \"text\": \"sea food\"\n                },\n                {\n                  \"option_id\": \"__DjlrV7TeyCSZVmCWAnng\",\n                  \"text\": \"rice\"\n                }\n              ],\n              \"reply_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ,dpv4EZ7QQUqRhBmYj4_b7w\"\n            }\n          ],\n          \"subject\": \"reservation\"\n        }\n      ]\n    }\n  },\n  \"event_ts\": 1707848572310\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.sensor_data": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.sensor_data",
        "requestBody": {
          "description": "# zoomroom.sensor_data\n\nThe **Zoom Room Sensor Data** event triggers every five minutes when a Zoom Room device sensor reports data.\n\n## Prerequisites\n\n* A Business or Enterprise plan.\n* The ability to detect the room air conditions, a feature available with Zoom Rooms version 5.5.0 or later. Not supported for Mac and Windows Zoom Rooms or Windows Controller.\n* Count the number of people in room: available on Zoom Rooms clients versions 5.5.0 (Windows), 5.5.0 (Mac), 5.6.6 (Android), 5.5.0 (iOS Controller), 5.5.0 (Android Controller) or later.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Room Sensor Data** subscription enabled under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:sensor_data:admin`\n\n**Event type**: `zoomroom.sensor_data`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "id",
                          "room_name",
                          "device_id",
                          "sensor_data"
                        ],
                        "description": "Information about the Zoom Room.",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Room ID."
                          },
                          "room_name": {
                            "type": "string",
                            "description": "The Zoom Room name."
                          },
                          "device_id": {
                            "type": "string",
                            "description": "The device ID."
                          },
                          "sensor_data": {
                            "type": "array",
                            "description": "Information about the sensor data.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "date_time": {
                                  "description": "The time when the sensor data was reported.",
                                  "type": "string",
                                  "format": "datetime",
                                  "example": "2022-06-19T00:00:00Z"
                                },
                                "sensor_type": {
                                  "type": "string",
                                  "enum": [
                                    "CO2",
                                    "TEMPERATURE",
                                    "REAL_TIME_PEOPLE_COUNT",
                                    "HUMIDITY",
                                    "VOC",
                                    "AQI",
                                    "PM25",
                                    "PM10",
                                    "AIR_PRESSURE"
                                  ],
                                  "description": "The sensor type."
                                },
                                "sensor_value": {
                                  "type": "string",
                                  "description": "The sensor value."
                                }
                              },
                              "required": [
                                "date_time",
                                "sensor_type",
                                "sensor_value"
                              ]
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
                  "value": "{\n\t\"event\": \"zoomroom.sensor_data\",\n\t\"event_ts\": 1626230691572,\n\t\"payload\": {\n\t\t\"account_id\": \"AAAAAABBBB\",\n\t\t\"object\": {\n\t\t\t\"id\": \"abcD3ojfdbjfg\",\n\t\t\t\"room_name\": \"My Zoom Room\",\n\t\t\t\"device_id\": \"NiVvY1NWpE2nulNrhVjgU4jD0swziVrXBbaFZyC3u+o=\",\n\t\t\t\"sensor_data\": [{\n\t\t\t\t\"date_time\": \"2022-06-19T00:00:00Z\",\n\t\t\t\t\"sensor_type\": \"REAL_TIME_PEOPLE_COUNT\",\n\t\t\t\t\"sensor_value\": \"20\"\n\t\t\t}]\n\t\t}\n\t}\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "id",
                            "room_name",
                            "device_id",
                            "sensor_data"
                          ],
                          "description": "Information about the Zoom Room.",
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Room ID."
                            },
                            "room_name": {
                              "type": "string",
                              "description": "The Zoom Room name."
                            },
                            "device_id": {
                              "type": "string",
                              "description": "The device ID."
                            },
                            "sensor_data": {
                              "type": "array",
                              "description": "Information about the sensor data.",
                              "items": {
                                "type": "object",
                                "properties": {
                                  "date_time": {
                                    "description": "The time when the sensor data was reported.",
                                    "type": "string",
                                    "format": "datetime",
                                    "example": "2022-06-19T00:00:00Z"
                                  },
                                  "sensor_type": {
                                    "type": "string",
                                    "enum": [
                                      "CO2",
                                      "TEMPERATURE",
                                      "REAL_TIME_PEOPLE_COUNT",
                                      "HUMIDITY",
                                      "VOC",
                                      "AQI",
                                      "PM25",
                                      "PM10",
                                      "AIR_PRESSURE"
                                    ],
                                    "description": "The sensor type."
                                  },
                                  "sensor_value": {
                                    "type": "string",
                                    "description": "The sensor value."
                                  }
                                },
                                "required": [
                                  "date_time",
                                  "sensor_type",
                                  "sensor_value"
                                ]
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
                    "value": "{\n\t\"event\": \"zoomroom.sensor_data\",\n\t\"event_ts\": 1626230691572,\n\t\"payload\": {\n\t\t\"account_id\": \"AAAAAABBBB\",\n\t\t\"object\": {\n\t\t\t\"id\": \"abcD3ojfdbjfg\",\n\t\t\t\"room_name\": \"My Zoom Room\",\n\t\t\t\"device_id\": \"NiVvY1NWpE2nulNrhVjgU4jD0swziVrXBbaFZyC3u+o=\",\n\t\t\t\"sensor_data\": [{\n\t\t\t\t\"date_time\": \"2022-06-19T00:00:00Z\",\n\t\t\t\t\"sensor_type\": \"REAL_TIME_PEOPLE_COUNT\",\n\t\t\t\t\"sensor_value\": \"20\"\n\t\t\t}]\n\t\t}\n\t}\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "visitor_management.invitation_deleted": {
      "post": {
        "tags": [
          "visitor_management"
        ],
        "operationId": "visitor_management.invitation_deleted",
        "requestBody": {
          "description": "# visitor_management.invitation_deleted\n\nThe **Visitor Management Invitation Deleted** event is triggered when an [invitation is deleted](https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0073629#mcetoc_1hi1ubsr1fh).\n\n## Prerequisites\n\n* The [**Visitor Management**](https://support.zoom.com/hc/en?id=zm_kb&sysparm_article=KB0057564) feature enabled by your organization.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Visitor Management Invitation Deleted** subscription enabled under the **Visitor Management** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:visitor_invitation:admin`\n\n**Event type**: `visitor_management.invitation_deleted`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The user account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that deleted the invitation."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that deleted the invitation."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the deleted invitation.",
                        "required": [
                          "visitor_email",
                          "visitor_name",
                          "invite_location_id",
                          "host_id",
                          "host_name",
                          "host_email"
                        ],
                        "properties": {
                          "visitor_email": {
                            "type": "string",
                            "description": "The email address of the invited visitor."
                          },
                          "visitor_name": {
                            "type": "string",
                            "description": "The name of the invited visitor."
                          },
                          "invite_location_id": {
                            "type": "string",
                            "description": "The location ID of the location to which the visitor is invited to."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The user ID of the invitation host."
                          },
                          "host_name": {
                            "type": "string",
                            "description": "The name of the invitation host."
                          },
                          "host_email": {
                            "type": "string",
                            "description": "The email address of the invitation host."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"visitor_management.invitation_deleted\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The user account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that deleted the invitation."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that deleted the invitation."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the deleted invitation.",
                          "required": [
                            "visitor_email",
                            "visitor_name",
                            "invite_location_id",
                            "host_id",
                            "host_name",
                            "host_email"
                          ],
                          "properties": {
                            "visitor_email": {
                              "type": "string",
                              "description": "The email address of the invited visitor."
                            },
                            "visitor_name": {
                              "type": "string",
                              "description": "The name of the invited visitor."
                            },
                            "invite_location_id": {
                              "type": "string",
                              "description": "The location ID of the location to which the visitor is invited to."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The user ID of the invitation host."
                            },
                            "host_name": {
                              "type": "string",
                              "description": "The name of the invitation host."
                            },
                            "host_email": {
                              "type": "string",
                              "description": "The email address of the invitation host."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"visitor_management.invitation_deleted\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.alert": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.alert",
        "requestBody": {
          "description": "# zoomroom.alert\n\nThe **Zoom Room Alert** event is triggered when there is an issue with a Zoom Room on a Zoom account that has installed your [webhook-enabled](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks) app. Some of the issues that can trigger this event are:\n\n* High CPU usage.\n* Low battery, charging, and/or connection issues in a Zoom Room device.\n* Room controller, camera, or speaker disconnections or reconnections.\n* Missing camera or microphone.\n\nIt is recommended that you **only** subscribe to either the **Zoom Room Alert** event or the **Zoom Room Delayed Alert** event to avoid receiving duplicate event notifications.\n\n## Prerequisites\n\n* A Business or Enterprise plan.\n* The administrator or account owner must enable the [**Zoom Room Alert**](https://support.zoom.us/hc/en-us/articles/205394099-Zoom-Room-Alerts) feature with the selected alerts in the [Zoom Web Portal](https://zoom.us/location).\n* The Zoom Room version must be 3.5.24524.0821 and higher.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Room Alert** subscription enabled under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:alert:admin`\n\n**Event type**: `zoomroom.alert`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "id",
                          "room_name",
                          "calendar_name",
                          "email",
                          "issue",
                          "alert_type",
                          "component",
                          "alert_time",
                          "alert_kind"
                        ],
                        "description": "Information about the Zoom Room.",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Room's unique ID."
                          },
                          "room_name": {
                            "type": "string",
                            "description": "The Zoom Room's name."
                          },
                          "calendar_name": {
                            "type": "string",
                            "description": "The calendar's name."
                          },
                          "email": {
                            "type": "string",
                            "format": "email",
                            "description": "The Zoom Room's associated email address."
                          },
                          "issue": {
                            "type": "string",
                            "description": "The issues that occurred during the meeting: \n* Room Controller disconnected \n* Room Controller connected \n* Selected camera has disconnected \n* Selected camera is reconnected \n* Selected microphone has disconnected \n* Selected microphone is reconnected \n* Selected speaker has disconnected \n* Selected speaker is reconnected \n* Zoom room is offline \n* Zoom room is online \n* High CPU usage is detected \n* Low bandwidth network is detected \n* {name} battery is low \n* {name} battery is normal \n* {name} disconnected \n* {name} connected \n Incorrect Room Passcode Entry Detected \n\nThe possible values for {name} are: \n* Zoom Rooms Computer \n* Controller \n* Scheduling Display \n*."
                          },
                          "alert_type": {
                            "type": "integer",
                            "description": "The type of alert sent: \n* 1 \u2014 The controller or scheduling display disconnected. \n* 2 \u2014 The selected microphone, speaker, or camera has disconnected \n* 3 \u2014 The Zoom Room is offline. \n* 4 \u2014 High CPU usage detected. \n* 5 \u2014 Low network bandwidth detected. \n* 6 \u2014 The controller or scheduling display battery is low. \n* 7 \u2014 The Zoom Rooms display is disconnected. \n* 8 \u2014 The audio device has quality issues \n* 9 \u2014 Failed SIP registration \n* 10 \u2014 Incorrect room passcode entry detected \n* 0 \u2014 Other alert.",
                            "enum": [
                              1,
                              2,
                              3,
                              4,
                              5,
                              6,
                              7,
                              8,
                              9,
                              10,
                              0
                            ]
                          },
                          "component": {
                            "description": "The affected component: \n* 1 \u2014 Zoom Rooms Computer \n* 2 \u2014 Controller \n* 3 \u2014 Scheduling Display \n* 4 \u2014 Microphone \n* 5 \u2014 Camera \n* 6 \u2014 Speaker \n* 0 - Other.",
                            "type": "integer",
                            "enum": [
                              1,
                              2,
                              3,
                              4,
                              5,
                              6,
                              0
                            ]
                          },
                          "alert_time": {
                            "type": "integer",
                            "format": "int64",
                            "description": "The timestamp when the alert occurred."
                          },
                          "alert_kind": {
                            "type": "integer",
                            "description": "The kind of alert: \n* 1 \u2014 Triggered. There was an encountered problem. When the problem is resolved, a follow-up response will contain this field with the 2 (Cleared) value. \n* 2 \u2014 Cleared. The problem is resolved. \n* 3 \u2014 Observed. There is a potential issue-causing condition. If the issue is cleared (2), there will **not** be a follow-up response. \n* 4 - Other.",
                            "enum": [
                              1,
                              2,
                              3,
                              4
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
                  "value": "{\n  \"event\": \"zoomroom.alert\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"id\": \"abcD3ojfdbjfg\",\n      \"room_name\": \"My Zoom Room\",\n      \"calendar_name\": \"mycalendar@example.com\",\n      \"email\": \"jchill@example.com\",\n      \"issue\": \"Room Controller disconnected\",\n      \"alert_type\": 1,\n      \"component\": 2,\n      \"alert_kind\": 1\n    }\n  }\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "id",
                            "room_name",
                            "calendar_name",
                            "email",
                            "issue",
                            "alert_type",
                            "component",
                            "alert_time",
                            "alert_kind"
                          ],
                          "description": "Information about the Zoom Room.",
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Room's unique ID."
                            },
                            "room_name": {
                              "type": "string",
                              "description": "The Zoom Room's name."
                            },
                            "calendar_name": {
                              "type": "string",
                              "description": "The calendar's name."
                            },
                            "email": {
                              "type": "string",
                              "format": "email",
                              "description": "The Zoom Room's associated email address."
                            },
                            "issue": {
                              "type": "string",
                              "description": "The issues that occurred during the meeting: \n* Room Controller disconnected \n* Room Controller connected \n* Selected camera has disconnected \n* Selected camera is reconnected \n* Selected microphone has disconnected \n* Selected microphone is reconnected \n* Selected speaker has disconnected \n* Selected speaker is reconnected \n* Zoom room is offline \n* Zoom room is online \n* High CPU usage is detected \n* Low bandwidth network is detected \n* {name} battery is low \n* {name} battery is normal \n* {name} disconnected \n* {name} connected \n Incorrect Room Passcode Entry Detected \n\nThe possible values for {name} are: \n* Zoom Rooms Computer \n* Controller \n* Scheduling Display \n*."
                            },
                            "alert_type": {
                              "type": "integer",
                              "description": "The type of alert sent: \n* 1 \u2014 The controller or scheduling display disconnected. \n* 2 \u2014 The selected microphone, speaker, or camera has disconnected \n* 3 \u2014 The Zoom Room is offline. \n* 4 \u2014 High CPU usage detected. \n* 5 \u2014 Low network bandwidth detected. \n* 6 \u2014 The controller or scheduling display battery is low. \n* 7 \u2014 The Zoom Rooms display is disconnected. \n* 8 \u2014 The audio device has quality issues \n* 9 \u2014 Failed SIP registration \n* 10 \u2014 Incorrect room passcode entry detected \n* 0 \u2014 Other alert.",
                              "enum": [
                                1,
                                2,
                                3,
                                4,
                                5,
                                6,
                                7,
                                8,
                                9,
                                10,
                                0
                              ]
                            },
                            "component": {
                              "description": "The affected component: \n* 1 \u2014 Zoom Rooms Computer \n* 2 \u2014 Controller \n* 3 \u2014 Scheduling Display \n* 4 \u2014 Microphone \n* 5 \u2014 Camera \n* 6 \u2014 Speaker \n* 0 - Other.",
                              "type": "integer",
                              "enum": [
                                1,
                                2,
                                3,
                                4,
                                5,
                                6,
                                0
                              ]
                            },
                            "alert_time": {
                              "type": "integer",
                              "format": "int64",
                              "description": "The timestamp when the alert occurred."
                            },
                            "alert_kind": {
                              "type": "integer",
                              "description": "The kind of alert: \n* 1 \u2014 Triggered. There was an encountered problem. When the problem is resolved, a follow-up response will contain this field with the 2 (Cleared) value. \n* 2 \u2014 Cleared. The problem is resolved. \n* 3 \u2014 Observed. There is a potential issue-causing condition. If the issue is cleared (2), there will **not** be a follow-up response. \n* 4 - Other.",
                              "enum": [
                                1,
                                2,
                                3,
                                4
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
                    "value": "{\n  \"event\": \"zoomroom.alert\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"id\": \"abcD3ojfdbjfg\",\n      \"room_name\": \"My Zoom Room\",\n      \"calendar_name\": \"mycalendar@example.com\",\n      \"email\": \"jchill@example.com\",\n      \"issue\": \"Room Controller disconnected\",\n      \"alert_type\": 1,\n      \"component\": 2,\n      \"alert_kind\": 1\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.reservation_questionnaire_response_updated": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.reservation_questionnaire_response_updated",
        "requestBody": {
          "description": "# workspace.reservation_questionnaire_response_updated\nThe **Workspace reservation questionnaire updated** event is triggered every time a questionnaire is updated. Users or Admins can update the questionnaire by updating the existing reservation.\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace reservation questionnaire updated** subscription enabled under the **Workspace** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:workspace_reservation_questionnaire_response`,`workspace:read:workspace_reservation_questionnaire_response:admin`\n\n**Event type**: `workspace.reservation_questionnaire_response_updated`\n",
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
                    "type": "string",
                    "format": "date-time",
                    "description": "A timestamp when the event occurred."
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that completed the questionnaire."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that completed the questionnaire."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the questionnaire when creating a reservation for a workspace.",
                        "required": [
                          "start_time",
                          "event_id",
                          "user_name",
                          "desk_id",
                          "workspace_name",
                          "location_id",
                          "questionnaire_response"
                        ],
                        "properties": {
                          "start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The start time of the workspace reservation."
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The event ID."
                          },
                          "user_name": {
                            "type": "string",
                            "description": "The user name."
                          },
                          "desk_id": {
                            "type": "string",
                            "description": "The desk ID."
                          },
                          "workspace_name": {
                            "type": "string",
                            "description": "The name of the workspace."
                          },
                          "location_id": {
                            "type": "string",
                            "description": "The location ID."
                          },
                          "questionnaire_response": {
                            "type": "array",
                            "description": "List of questionnaires.",
                            "items": {
                              "type": "object",
                              "required": [
                                "id",
                                "title",
                                "subject",
                                "list"
                              ],
                              "properties": {
                                "id": {
                                  "type": "string",
                                  "description": "The view ID."
                                },
                                "title": {
                                  "type": "string",
                                  "description": "The title of the questionnaire."
                                },
                                "subject": {
                                  "type": "string",
                                  "description": "The subject of the questionnaire."
                                },
                                "list": {
                                  "type": "array",
                                  "description": "List of questions in the questionnaire.",
                                  "items": {
                                    "type": "object",
                                    "required": [
                                      "question_id",
                                      "title",
                                      "type",
                                      "options",
                                      "reply_id"
                                    ],
                                    "properties": {
                                      "question_id": {
                                        "type": "string",
                                        "description": "The question ID."
                                      },
                                      "title": {
                                        "type": "string",
                                        "description": "The title of the question."
                                      },
                                      "type": {
                                        "type": "string",
                                        "description": "The type of the question (single-choice, multiple-choice, open-text)."
                                      },
                                      "reply_id": {
                                        "type": "string",
                                        "description": "The reply ID."
                                      },
                                      "options": {
                                        "type": "array",
                                        "description": "List of options for multiple-choice questions.",
                                        "items": {
                                          "type": "object",
                                          "required": [
                                            "option_id",
                                            "text"
                                          ],
                                          "properties": {
                                            "option_id": {
                                              "type": "string",
                                              "description": "The option ID."
                                            },
                                            "text": {
                                              "type": "string",
                                              "description": "The text of the option."
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
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"workspace.reservation_questionnaire_response_updated\",\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator\": \"yuki-devint@zoom.us\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"object\": {\n      \"start_time\": \"2024-02-14T03:50:00Z\",\n      \"event_id\": \"dxpwbc2fi8tzz7d012rwx9u8c0\",\n      \"user_name\": \"Simash pokh\",\n      \"desk_id\": \"SJjU1CcGSs2L7odn6PanQw\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"questionnaire_response\": [\n        {\n          \"id\": \"Fl75C43YTmS_IpMzYLipJw\",\n          \"title\": \"Simash Reservation Questionnaire\",\n          \"list\": [\n            {\n              \"question_id\": \"0ds1NFeDQ1WfWlgVK-7OMQ\",\n              \"title\": \"do you like AI ?\",\n              \"type\": \"single\",\n              \"options\": [\n                {\n                  \"option_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\",\n                  \"text\": \"Yes\"\n                },\n                {\n                  \"option_id\": \"1S1hZzFGTwW9Llxdr2vzyw\",\n                  \"text\": \"No\"\n                }\n              ],\n              \"reply_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\"\n            },\n            {\n              \"question_id\": \"2XMaJFwNSr-BJpTfo6rRAQ\",\n              \"title\": \"what is your fav food ? \",\n              \"type\": \"multiple\",\n              \"options\": [\n                {\n                  \"option_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ\",\n                  \"text\": \"momo\"\n                },\n                {\n                  \"option_id\": \"dpv4EZ7QQUqRhBmYj4_b7w\",\n                  \"text\": \"sea food\"\n                },\n                {\n                  \"option_id\": \"__DjlrV7TeyCSZVmCWAnng\",\n                  \"text\": \"rice\"\n                }\n              ],\n              \"reply_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ,dpv4EZ7QQUqRhBmYj4_b7w\"\n            }\n          ],\n          \"subject\": \"reservation\"\n        }\n      ]\n    }\n  },\n  \"event_ts\": 1707848572310\n}"
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
                      "type": "string",
                      "format": "date-time",
                      "description": "A timestamp when the event occurred."
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that completed the questionnaire."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that completed the questionnaire."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the questionnaire when creating a reservation for a workspace.",
                          "required": [
                            "start_time",
                            "event_id",
                            "user_name",
                            "desk_id",
                            "workspace_name",
                            "location_id",
                            "questionnaire_response"
                          ],
                          "properties": {
                            "start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The start time of the workspace reservation."
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The event ID."
                            },
                            "user_name": {
                              "type": "string",
                              "description": "The user name."
                            },
                            "desk_id": {
                              "type": "string",
                              "description": "The desk ID."
                            },
                            "workspace_name": {
                              "type": "string",
                              "description": "The name of the workspace."
                            },
                            "location_id": {
                              "type": "string",
                              "description": "The location ID."
                            },
                            "questionnaire_response": {
                              "type": "array",
                              "description": "List of questionnaires.",
                              "items": {
                                "type": "object",
                                "required": [
                                  "id",
                                  "title",
                                  "subject",
                                  "list"
                                ],
                                "properties": {
                                  "id": {
                                    "type": "string",
                                    "description": "The view ID."
                                  },
                                  "title": {
                                    "type": "string",
                                    "description": "The title of the questionnaire."
                                  },
                                  "subject": {
                                    "type": "string",
                                    "description": "The subject of the questionnaire."
                                  },
                                  "list": {
                                    "type": "array",
                                    "description": "List of questions in the questionnaire.",
                                    "items": {
                                      "type": "object",
                                      "required": [
                                        "question_id",
                                        "title",
                                        "type",
                                        "options",
                                        "reply_id"
                                      ],
                                      "properties": {
                                        "question_id": {
                                          "type": "string",
                                          "description": "The question ID."
                                        },
                                        "title": {
                                          "type": "string",
                                          "description": "The title of the question."
                                        },
                                        "type": {
                                          "type": "string",
                                          "description": "The type of the question (single-choice, multiple-choice, open-text)."
                                        },
                                        "reply_id": {
                                          "type": "string",
                                          "description": "The reply ID."
                                        },
                                        "options": {
                                          "type": "array",
                                          "description": "List of options for multiple-choice questions.",
                                          "items": {
                                            "type": "object",
                                            "required": [
                                              "option_id",
                                              "text"
                                            ],
                                            "properties": {
                                              "option_id": {
                                                "type": "string",
                                                "description": "The option ID."
                                              },
                                              "text": {
                                                "type": "string",
                                                "description": "The text of the option."
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
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"workspace.reservation_questionnaire_response_updated\",\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator\": \"yuki-devint@zoom.us\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"object\": {\n      \"start_time\": \"2024-02-14T03:50:00Z\",\n      \"event_id\": \"dxpwbc2fi8tzz7d012rwx9u8c0\",\n      \"user_name\": \"Simash pokh\",\n      \"desk_id\": \"SJjU1CcGSs2L7odn6PanQw\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"questionnaire_response\": [\n        {\n          \"id\": \"Fl75C43YTmS_IpMzYLipJw\",\n          \"title\": \"Simash Reservation Questionnaire\",\n          \"list\": [\n            {\n              \"question_id\": \"0ds1NFeDQ1WfWlgVK-7OMQ\",\n              \"title\": \"do you like AI ?\",\n              \"type\": \"single\",\n              \"options\": [\n                {\n                  \"option_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\",\n                  \"text\": \"Yes\"\n                },\n                {\n                  \"option_id\": \"1S1hZzFGTwW9Llxdr2vzyw\",\n                  \"text\": \"No\"\n                }\n              ],\n              \"reply_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\"\n            },\n            {\n              \"question_id\": \"2XMaJFwNSr-BJpTfo6rRAQ\",\n              \"title\": \"what is your fav food ? \",\n              \"type\": \"multiple\",\n              \"options\": [\n                {\n                  \"option_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ\",\n                  \"text\": \"momo\"\n                },\n                {\n                  \"option_id\": \"dpv4EZ7QQUqRhBmYj4_b7w\",\n                  \"text\": \"sea food\"\n                },\n                {\n                  \"option_id\": \"__DjlrV7TeyCSZVmCWAnng\",\n                  \"text\": \"rice\"\n                }\n              ],\n              \"reply_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ,dpv4EZ7QQUqRhBmYj4_b7w\"\n            }\n          ],\n          \"subject\": \"reservation\"\n        }\n      ]\n    }\n  },\n  \"event_ts\": 1707848572310\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.reservation_update": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.reservation_update",
        "requestBody": {
          "description": "# workspace.reservation_update\n\nThe **Workspace Reservation Updated** event is triggered when a user updates their [Workspace reservation](https://support.zoom.us/hc/en-us/articles/4417951085965-Using-Workspace-Reservation-Beta-for-Zoom-Rooms).\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\n* The Zoom mobile app. This is required for QR code scanning:\n  * Android version 5.9.0.3502 or higher\n  * iOS version 5.9.0 or higher\n* [Zoom Rooms for Touch devices](https://support.zoom.us/hc/en-us/articles/115004773103), version 5.9.0 or higher (*Optional*).\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace Reservation Updated** subscription enabled under the **Workspace Reservation** event.\n\n\n\n**Event type**: `workspace.reservation_update`\n",
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that updated the reservation."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that updated the reservation."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the workspace reservation.",
                        "required": [
                          "id",
                          "workspace_id",
                          "end_time",
                          "start_time",
                          "topic"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The reservation ID."
                          },
                          "workspace_id": {
                            "type": "string",
                            "description": "The workspace ID."
                          },
                          "end_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's end time."
                          },
                          "start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's start time."
                          },
                          "topic": {
                            "type": "string",
                            "description": "The reservation's description."
                          },
                          "reserve_for": {
                            "type": "string",
                            "description": "The ID of the user for whom to create a workspace reservation.",
                            "example": "KDcuGIm1QgePTO8WbOqwIQ"
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
                  "value": "{\n\t\"event\": \"workspace.reservation_updated\",\n\t\"event_ts\": \"290112312321\",\n\t\"payload\": {\n\t\t\"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n\t\t\"operator\" : \"admin@example.com\",\n\t    \"operator_id\" : \"I7zp-pYbTl22Z29H796Ixg\",\n\t\t\"object\": {\n\t\t\t\"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n\t\t\t\"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n\t\t\t\"topic\": \"Reserve Zoom Room-01\",\n\t\t\t\"start_time\": \"2022-01-19T02:10:00Z\",\n\t\t\t\"end_time\": \"2022-01-19T03:55:00Z\",\n\t\t\t\"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\"\n\t\t}\n\t}\n}"
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that updated the reservation."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that updated the reservation."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the workspace reservation.",
                          "required": [
                            "id",
                            "workspace_id",
                            "end_time",
                            "start_time",
                            "topic"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The reservation ID."
                            },
                            "workspace_id": {
                              "type": "string",
                              "description": "The workspace ID."
                            },
                            "end_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's end time."
                            },
                            "start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's start time."
                            },
                            "topic": {
                              "type": "string",
                              "description": "The reservation's description."
                            },
                            "reserve_for": {
                              "type": "string",
                              "description": "The ID of the user for whom to create a workspace reservation.",
                              "example": "KDcuGIm1QgePTO8WbOqwIQ"
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
                    "value": "{\n\t\"event\": \"workspace.reservation_updated\",\n\t\"event_ts\": \"290112312321\",\n\t\"payload\": {\n\t\t\"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n\t\t\"operator\" : \"admin@example.com\",\n\t    \"operator_id\" : \"I7zp-pYbTl22Z29H796Ixg\",\n\t\t\"object\": {\n\t\t\t\"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n\t\t\t\"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n\t\t\t\"topic\": \"Reserve Zoom Room-01\",\n\t\t\t\"start_time\": \"2022-01-19T02:10:00Z\",\n\t\t\t\"end_time\": \"2022-01-19T03:55:00Z\",\n\t\t\t\"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\"\n\t\t}\n\t}\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.user_assigned": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.user_assigned",
        "requestBody": {
          "description": "# workspace.user_assigned\nThe **Workspace User Assigned** event is triggered every time a user is assigned to a Workspace in your account. Admins and account owners can assign a user to Workspace in you account. The Workspace can be assign users using the [Set a desk assignment API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/setADeskAssignment) or by set the Workspace' assignment using the Zoom Web Portal itself.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace User Assigned** subscription enabled under the **Workspace** event.\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:assignment`,`workspace:read:assignment:admin`\n\n**Event type**: `workspace.user_assigned`\n",
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
                    "type": "string",
                    "format": "date-time",
                    "description": "A timestamp when the event occurred."
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that updated the assignment."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that updated the assignment."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the assignment when updating assignment of a workspace.",
                        "required": [
                          "workspace_id",
                          "workspace_name",
                          "assign_type",
                          "account_id",
                          "user_id",
                          "user_email"
                        ],
                        "properties": {
                          "workspace_id": {
                            "type": "string",
                            "description": "The workspace's ID."
                          },
                          "workspace_name": {
                            "type": "string",
                            "description": "The workspace's name."
                          },
                          "assign_type": {
                            "type": "string",
                            "description": "The type of assignment. ",
                            "enum": [
                              "permanent"
                            ]
                          },
                          "account_id": {
                            "type": "string",
                            "description": "The belong's account id of assigned user."
                          },
                          "user_id": {
                            "type": "string",
                            "description": "The id of assigned user."
                          },
                          "user_email": {
                            "type": "string",
                            "description": "The email of assigned user."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"workspace.user_assigned\",\n  \"event_ts\": 1707848572310,\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"operator\": \"vic-devint@zoom.us\",\n    \"object\": {\n      \"workspace_id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"assign_type\": \"permanent\",\n      \"account_id\": \"bhmIXpdYRcCK6BB3VhMq6w\",\n      \"user_id\": \"oyLyxW48QeC3uBBFY2CF3A\",\n      \"user_email\": \"vic-01-devint@zoom.us\"\n    }\n  }\n}"
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
                      "type": "string",
                      "format": "date-time",
                      "description": "A timestamp when the event occurred."
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that updated the assignment."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that updated the assignment."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the assignment when updating assignment of a workspace.",
                          "required": [
                            "workspace_id",
                            "workspace_name",
                            "assign_type",
                            "account_id",
                            "user_id",
                            "user_email"
                          ],
                          "properties": {
                            "workspace_id": {
                              "type": "string",
                              "description": "The workspace's ID."
                            },
                            "workspace_name": {
                              "type": "string",
                              "description": "The workspace's name."
                            },
                            "assign_type": {
                              "type": "string",
                              "description": "The type of assignment. ",
                              "enum": [
                                "permanent"
                              ]
                            },
                            "account_id": {
                              "type": "string",
                              "description": "The belong's account id of assigned user."
                            },
                            "user_id": {
                              "type": "string",
                              "description": "The id of assigned user."
                            },
                            "user_email": {
                              "type": "string",
                              "description": "The email of assigned user."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"workspace.user_assigned\",\n  \"event_ts\": 1707848572310,\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"operator\": \"vic-devint@zoom.us\",\n    \"object\": {\n      \"workspace_id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"assign_type\": \"permanent\",\n      \"account_id\": \"bhmIXpdYRcCK6BB3VhMq6w\",\n      \"user_id\": \"oyLyxW48QeC3uBBFY2CF3A\",\n      \"user_email\": \"vic-01-devint@zoom.us\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.digital_signage_app_ended": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.digital_signage_app_ended",
        "requestBody": {
          "description": "# zoomroom.digital_signage_app_ended\n\nThe **[Zoom Room digital signage](https://support.zoom.us/hc/en-us/articles/360000030683-Zoom-Rooms-digital-signage) play ended** event triggers when the digital signage finishes playing or showing.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n* A valid **Event Notification Endpoint URL**.\n* The **Zoom Room digital signage app ended** subscription enabled under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:digital_signage_app:admin`\n\n**Event type**: `zoomroom.digital_signage_app_ended`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "room_id",
                          "app_name",
                          "app_id",
                          "start_time",
                          "end_time"
                        ],
                        "description": "The information on the digital signage played in a Zoom Room.",
                        "properties": {
                          "room_id": {
                            "type": "string",
                            "description": "The Zoom Room ID."
                          },
                          "app_name": {
                            "type": "string",
                            "description": "The app name."
                          },
                          "app_id": {
                            "type": "string",
                            "description": "The app ID."
                          },
                          "start_time": {
                            "description": "The playback start time of the digital signage.",
                            "type": "string",
                            "format": "datetime",
                            "example": "2022-06-19T00:00:00Z"
                          },
                          "end_time": {
                            "description": "The playback end time of the digital signage.",
                            "type": "string",
                            "format": "datetime",
                            "example": "2022-06-19T00:00:00Z"
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
                  "value": "{\n\t\"event\": \"zoomroom.digital_signage_app_ended\",\n\t\"event_ts\": 1626230691572,\n\t\"payload\": {\n\t\t\"account_id\": \"AAAAAABBBB\",\n\t\t\"object\": {\n\t\t\t\"room_id\": \"abcD3ojfdbjfg\",\n\t\t\t\"app_name\": \"My Zoom Room\",\n\t\t\t\"app_id\": \"NiVvY1NWpE2nulNrhVjgU4jD0swziVrXBbaFZyC3u+o=\",\n\t\t\t\"start_time\": \"2022-06-19T00:00:00Z\",\n            \"end_time\": \"2022-06-19T01:01:00Z\"\n\t\t}\n\t}\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "room_id",
                            "app_name",
                            "app_id",
                            "start_time",
                            "end_time"
                          ],
                          "description": "The information on the digital signage played in a Zoom Room.",
                          "properties": {
                            "room_id": {
                              "type": "string",
                              "description": "The Zoom Room ID."
                            },
                            "app_name": {
                              "type": "string",
                              "description": "The app name."
                            },
                            "app_id": {
                              "type": "string",
                              "description": "The app ID."
                            },
                            "start_time": {
                              "description": "The playback start time of the digital signage.",
                              "type": "string",
                              "format": "datetime",
                              "example": "2022-06-19T00:00:00Z"
                            },
                            "end_time": {
                              "description": "The playback end time of the digital signage.",
                              "type": "string",
                              "format": "datetime",
                              "example": "2022-06-19T00:00:00Z"
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
                    "value": "{\n\t\"event\": \"zoomroom.digital_signage_app_ended\",\n\t\"event_ts\": 1626230691572,\n\t\"payload\": {\n\t\t\"account_id\": \"AAAAAABBBB\",\n\t\t\"object\": {\n\t\t\t\"room_id\": \"abcD3ojfdbjfg\",\n\t\t\t\"app_name\": \"My Zoom Room\",\n\t\t\t\"app_id\": \"NiVvY1NWpE2nulNrhVjgU4jD0swziVrXBbaFZyC3u+o=\",\n\t\t\t\"start_time\": \"2022-06-19T00:00:00Z\",\n            \"end_time\": \"2022-06-19T01:01:00Z\"\n\t\t}\n\t}\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.checked_in": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.checked_in",
        "requestBody": {
          "description": "# zoomroom.checked_in\n\nThe **Zoom Room Checked In** event is triggered when a user [checks in](https://support.zoom.us/hc/en-us/articles/360001147163-Using-the-Zoom-Rooms-check-in-feature) to confirm that they will be using the Zoom Room for a certain period of time.\n\nUsers can check in manually via the Zoom Room controller or the scheduling display, or they can check in programmatically using the [**Check-in or check-out of a Zoom Room**](/api-reference/zoom-api/methods#operation/checkInRooms) API. Users can also check in to the Zoom Room by starting a scheduled meeting, using the whiteboard feature or making a phone call from the Zoom Room.\n\n**Note** Only scheduled meetings will trigger a webhook.\n\n\n## Prerequisites\n\n* The administrator or account owner must enable the [**Room Check-In and Check-Out**](https://support.zoom.us/hc/en-us/articles/360001147163-Using-the-Zoom-Rooms-check-in-feature) feature.\n* The Zoom Room version must be 4.1.27694.0702 or higher.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Room Checked In** subscription enabled under the **Zoom Room** event.\n\n\n\n\n\n\n\n\n\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\n\n**Event type**: `zoomroom.checked_in`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "id",
                          "room_name",
                          "calendar_name",
                          "email",
                          "event_id",
                          "change_key",
                          "resource_email",
                          "calendar_id",
                          "calendar_type",
                          "api_type"
                        ],
                        "description": "Information about the Zoom Room.",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Room's unique ID."
                          },
                          "room_name": {
                            "type": "string",
                            "description": "The Zoom Room's name."
                          },
                          "calendar_name": {
                            "type": "string",
                            "description": "The calendar's name."
                          },
                          "email": {
                            "type": "string",
                            "format": "email",
                            "description": "The Zoom Room's associated email address."
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The unique ID of the Zoom Room's associated calendar event:\n\n* For Microsoft Exchange calendar integration, use Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). The event_id value is the Id value in the ItemId element of the FindItem operation's response. \n* For Google Calendar integration, use Google's [Events: list API](https://developers.google.com/calendar/api/v3/reference/events/list). The event_id value is the id field in the [events resource](https://developers.google.com/calendar/api/v3/reference/events#resource)."
                          },
                          "change_key": {
                            "type": "string",
                            "description": "The specific version of the calendar item. You can get this value using Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). Refer to the ChangeKey field of the ItemId element in the operation's response.\n\nThis field is only returned for Microsoft Exchange or Office 365 calendars."
                          },
                          "resource_email": {
                            "description": "The calendar's associated [resource mailbox](https://support.microsoft.com/en-us/help/10063/creating-and-managing-resource-mailboxes-in-office-365).\n\nThis field is only returned for Microsoft Exchange or Office 365 calendars.",
                            "type": "string"
                          },
                          "calendar_id": {
                            "type": "string",
                            "description": "The calendar's unique ID."
                          },
                          "calendar_type": {
                            "type": "string",
                            "description": "The calendar's type: \n* 0 \u2014 Google Calendar. \n* 1 \u2014 Office 365 Calendar. \n* 2 \u2014 Microsoft Exchange Calendar.",
                            "enum": [
                              "0",
                              "1",
                              "2"
                            ]
                          },
                          "api_type": {
                            "type": "string",
                            "description": "The API type: \n* 0 \u2014 [Exchange Web Services (EWS) API](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-managed-api-reference-for-exchange) \n* 1 \u2014 [Microsoft Graph API](https://docs.microsoft.com/en-us/azure/active-directory/develop/microsoft-graph-intro) \n* An empty string \u2014 The API returns this value when the calendar_type value is 0 (Google Calendar).",
                            "enum": [
                              "0",
                              "1",
                              ""
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
                  "value": "{\n  \"event\": \"zoomroom.checked_in\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n        \"id\": \"abcD3ojfdbjfg\",\n        \"room_name\": \"My Zoom Room\",\n        \"calendar_name\": \"mycalendar@example.com\",\n        \"email\": \"jchill@example.com\",\n        \"event_id\": \"AbbbbbGYxLTc3OTVkMzFmZDc0MwBGAAAAAAD48FI58voYSqDgJePOSZblBwBQ/N0JvB/FRqv5UT2rFfkVAAAAAAENAABQ/N0JvB/FRqv5UT2rFfkVAAE2YC8DAAA=\",\n        \"change_key\": \"DwAAABYAAABQ/N0JvB/FRqv5UT2rFfkVAAE2XqVw\",\n        \"resource_email\": \"it@example.com\",\n        \"calendar_id\": \"mycalendar@example.com\",\n        \"calendar_type\": \"2\",\n        \"api_type\": \"0\"\n    }\n  }\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "id",
                            "room_name",
                            "calendar_name",
                            "email",
                            "event_id",
                            "change_key",
                            "resource_email",
                            "calendar_id",
                            "calendar_type",
                            "api_type"
                          ],
                          "description": "Information about the Zoom Room.",
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Room's unique ID."
                            },
                            "room_name": {
                              "type": "string",
                              "description": "The Zoom Room's name."
                            },
                            "calendar_name": {
                              "type": "string",
                              "description": "The calendar's name."
                            },
                            "email": {
                              "type": "string",
                              "format": "email",
                              "description": "The Zoom Room's associated email address."
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The unique ID of the Zoom Room's associated calendar event:\n\n* For Microsoft Exchange calendar integration, use Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). The event_id value is the Id value in the ItemId element of the FindItem operation's response. \n* For Google Calendar integration, use Google's [Events: list API](https://developers.google.com/calendar/api/v3/reference/events/list). The event_id value is the id field in the [events resource](https://developers.google.com/calendar/api/v3/reference/events#resource)."
                            },
                            "change_key": {
                              "type": "string",
                              "description": "The specific version of the calendar item. You can get this value using Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). Refer to the ChangeKey field of the ItemId element in the operation's response.\n\nThis field is only returned for Microsoft Exchange or Office 365 calendars."
                            },
                            "resource_email": {
                              "description": "The calendar's associated [resource mailbox](https://support.microsoft.com/en-us/help/10063/creating-and-managing-resource-mailboxes-in-office-365).\n\nThis field is only returned for Microsoft Exchange or Office 365 calendars.",
                              "type": "string"
                            },
                            "calendar_id": {
                              "type": "string",
                              "description": "The calendar's unique ID."
                            },
                            "calendar_type": {
                              "type": "string",
                              "description": "The calendar's type: \n* 0 \u2014 Google Calendar. \n* 1 \u2014 Office 365 Calendar. \n* 2 \u2014 Microsoft Exchange Calendar.",
                              "enum": [
                                "0",
                                "1",
                                "2"
                              ]
                            },
                            "api_type": {
                              "type": "string",
                              "description": "The API type: \n* 0 \u2014 [Exchange Web Services (EWS) API](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-managed-api-reference-for-exchange) \n* 1 \u2014 [Microsoft Graph API](https://docs.microsoft.com/en-us/azure/active-directory/develop/microsoft-graph-intro) \n* An empty string \u2014 The API returns this value when the calendar_type value is 0 (Google Calendar).",
                              "enum": [
                                "0",
                                "1",
                                ""
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
                    "value": "{\n  \"event\": \"zoomroom.checked_in\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n        \"id\": \"abcD3ojfdbjfg\",\n        \"room_name\": \"My Zoom Room\",\n        \"calendar_name\": \"mycalendar@example.com\",\n        \"email\": \"jchill@example.com\",\n        \"event_id\": \"AbbbbbGYxLTc3OTVkMzFmZDc0MwBGAAAAAAD48FI58voYSqDgJePOSZblBwBQ/N0JvB/FRqv5UT2rFfkVAAAAAAENAABQ/N0JvB/FRqv5UT2rFfkVAAE2YC8DAAA=\",\n        \"change_key\": \"DwAAABYAAABQ/N0JvB/FRqv5UT2rFfkVAAE2XqVw\",\n        \"resource_email\": \"it@example.com\",\n        \"calendar_id\": \"mycalendar@example.com\",\n        \"calendar_type\": \"2\",\n        \"api_type\": \"0\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.reservation_checked_in": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.reservation_checked_in",
        "requestBody": {
          "description": "# workspace.reservation_checked_in\n\nThe **Workspace Reservation Checked In** event is triggered when a user checks in their [Workspace reservation](https://support.zoom.us/hc/en-us/articles/4417951085965-Using-Workspace-Reservation-Beta-for-Zoom-Rooms) to confirm that they will be using a Zoom Room or Zoom device-enabled desk.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\n* The Zoom mobile app. This is required for QR code scanning:\n  * Android version 5.9.0.3502 or higher\n  * iOS version 5.9.0 or higher\n* [Zoom Rooms for Touch devices](https://support.zoom.us/hc/en-us/articles/115004773103), version 5.9.0 or higher (*Optional*).\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace Reservation Checked In** subscription enabled under the **Workspace Reservation** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:reservation`,`workspace:read:reservation:admin`\n\n**Event type**: `workspace.reservation_checked_in`\n",
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that checked in the reservation."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that checked in the reservation."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the workspace reservation.",
                        "required": [
                          "id",
                          "workspace_id",
                          "end_time",
                          "start_time",
                          "topic"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The reservation ID."
                          },
                          "workspace_id": {
                            "type": "string",
                            "description": "The workspace ID."
                          },
                          "end_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's end time."
                          },
                          "start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's start time."
                          },
                          "topic": {
                            "type": "string",
                            "description": "The reservation's description."
                          },
                          "reserve_for": {
                            "type": "string",
                            "description": "The ID of the user for whom to create a workspace reservation.",
                            "example": "KDcuGIm1QgePTO8WbOqwIQ"
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
                  "value": "{\n\t\"event\": \"workspace.reservation_checked_in\",\n\t\"event_ts\": \"290112312321\",\n\t\"payload\": {\n\t\t\"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n\t\t\"operator\" : \"admin@example.com\",\n\t    \"operator_id\" : \"I7zp-pYbTl22Z29H796Ixg\",\n\t\t\"object\": {\n\t\t\t\"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n\t\t\t\"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n\t\t\t\"topic\": \"Reserve Zoom Room-01\",\n\t\t\t\"start_time\": \"2022-01-19T02:10:00Z\",\n\t\t\t\"end_time\": \"2022-01-19T03:55:00Z\",\n\t\t\t\"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\"\n\t\t}\n\t}\n}"
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that checked in the reservation."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that checked in the reservation."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the workspace reservation.",
                          "required": [
                            "id",
                            "workspace_id",
                            "end_time",
                            "start_time",
                            "topic"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The reservation ID."
                            },
                            "workspace_id": {
                              "type": "string",
                              "description": "The workspace ID."
                            },
                            "end_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's end time."
                            },
                            "start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's start time."
                            },
                            "topic": {
                              "type": "string",
                              "description": "The reservation's description."
                            },
                            "reserve_for": {
                              "type": "string",
                              "description": "The ID of the user for whom to create a workspace reservation.",
                              "example": "KDcuGIm1QgePTO8WbOqwIQ"
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
                    "value": "{\n\t\"event\": \"workspace.reservation_checked_in\",\n\t\"event_ts\": \"290112312321\",\n\t\"payload\": {\n\t\t\"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n\t\t\"operator\" : \"admin@example.com\",\n\t    \"operator_id\" : \"I7zp-pYbTl22Z29H796Ixg\",\n\t\t\"object\": {\n\t\t\t\"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n\t\t\t\"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n\t\t\t\"topic\": \"Reserve Zoom Room-01\",\n\t\t\t\"start_time\": \"2022-01-19T02:10:00Z\",\n\t\t\t\"end_time\": \"2022-01-19T03:55:00Z\",\n\t\t\t\"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\"\n\t\t}\n\t}\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.updated": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.updated",
        "requestBody": {
          "description": "# zoomroom.updated\n\nThe **Zoom Room Updated** event is triggered when a user updates a [**Zoom Room**](https://support.zoom.us/hc/en-us/categories/200108436)\n\n## Prerequisites\n\n* The **Zoom Room** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Room Updated** subscription enabled under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\n\n**Event type**: `zoomroom.updated`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "id",
                          "room_name"
                        ],
                        "description": "Information about the Zoom Room.",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Room's unique ID."
                          },
                          "room_name": {
                            "type": "string",
                            "description": "The Zoom Room's name."
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
                  "value": "{\n  \"event\": \"zoomroom.updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n    \"object\": {\n      \"id\": \"2muAb65FTZCGjj6ginHIbQ\",\n      \"room_name\": \"My Zoom Room\"\n    }\n  }\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "id",
                            "room_name"
                          ],
                          "description": "Information about the Zoom Room.",
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Room's unique ID."
                            },
                            "room_name": {
                              "type": "string",
                              "description": "The Zoom Room's name."
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
                    "value": "{\n  \"event\": \"zoomroom.updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n    \"object\": {\n      \"id\": \"2muAb65FTZCGjj6ginHIbQ\",\n      \"room_name\": \"My Zoom Room\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.location_updated": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.location_updated",
        "requestBody": {
          "description": "# zoomroom.location_updated\nThe **Zoom Rooms Location has been updated** event is triggered every time you update a Zoom Room Location in your account. Admins and account owners can update a Zoom Room Location to an account. Update the location using the [Update a Zoom Rooms Location API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/updateZRLocationProfile) or by updating the Zoom Room Location using the Zoom Web Portal itself.\n\n## Prerequisites\n\n* Enable The **Zoom Rooms** feature by your organization through a Zoom Rooms license or Workspace Reservation feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or **Workspace Reservation** license.\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Rooms Location Updated** subscription enabled under the **Zoom Rooms** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\n\n**Event type**: `zoomroom.location_updated`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who updated the Zoom Rooms Location."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that updated the Zoom Rooms Location."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that updated the Zoom Rooms Location."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the updated Location.",
                        "required": [
                          "id",
                          "name",
                          "parent_location_id",
                          "type"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Rooms location's ID."
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the updated Zoom Rooms location."
                          },
                          "parent_location_id": {
                            "type": "string",
                            "description": "This location's parent Zoom Rooms location ID."
                          },
                          "type": {
                            "type": "enum, country\u2503states\u2503city\u2503campus\u2503building\u2503floor",
                            "description": "The type of location."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"zoomroom.location_updated\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"name\": \"My new Zoom Room Location\",\n      \"parent_location_id\": \"aldjflajsd-fasj\",\n      \"type\": \"floor\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who updated the Zoom Rooms Location."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that updated the Zoom Rooms Location."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that updated the Zoom Rooms Location."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the updated Location.",
                          "required": [
                            "id",
                            "name",
                            "parent_location_id",
                            "type"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Rooms location's ID."
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the updated Zoom Rooms location."
                            },
                            "parent_location_id": {
                              "type": "string",
                              "description": "This location's parent Zoom Rooms location ID."
                            },
                            "type": {
                              "type": "enum, country\u2503states\u2503city\u2503campus\u2503building\u2503floor",
                              "description": "The type of location."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"zoomroom.location_updated\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"name\": \"My new Zoom Room Location\",\n      \"parent_location_id\": \"aldjflajsd-fasj\",\n      \"type\": \"floor\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.pre_check_in_questionnaire_response_created": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.pre_check_in_questionnaire_response_created",
        "requestBody": {
          "description": "# workspace.pre_check_in_questionnaire_response_created\nThe **Workspace pre-check-in questionnaire created** event is triggered every time a pre-check-in questionnaire is submitted. Users can submit the questionnaire by clicking the workspace questionnarie link in the email.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Pre-check-in questionnaire created** subscription enabled under the **Workspace** event.\n\n**Event type**: `workspace.pre_check_in_questionnaire_response_created `\n",
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
                    "type": "string",
                    "format": "date-time",
                    "description": "A timestamp when the event occurred."
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that completed the questionnaire."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that completed the questionnaire."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the questionnaire when creating a reservation for a workspace.",
                        "required": [
                          "start_time",
                          "event_id",
                          "user_name",
                          "desk_id",
                          "workspace_name",
                          "location_id",
                          "questionnaire_response"
                        ],
                        "properties": {
                          "start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The start time of the workspace reservation."
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The event ID."
                          },
                          "user_name": {
                            "type": "string",
                            "description": "The user name."
                          },
                          "desk_id": {
                            "type": "string",
                            "description": "The desk ID."
                          },
                          "workspace_name": {
                            "type": "string",
                            "description": "The name of the workspace."
                          },
                          "location_id": {
                            "type": "string",
                            "description": "The location ID."
                          },
                          "questionnaire_response": {
                            "type": "array",
                            "description": "List of questionnaires.",
                            "items": {
                              "type": "object",
                              "required": [
                                "id",
                                "title",
                                "subject",
                                "list"
                              ],
                              "properties": {
                                "id": {
                                  "type": "string",
                                  "description": "The view ID."
                                },
                                "title": {
                                  "type": "string",
                                  "description": "The title of the questionnaire."
                                },
                                "subject": {
                                  "type": "string",
                                  "description": "The subject of the questionnaire."
                                },
                                "list": {
                                  "type": "array",
                                  "description": "List of questions in the questionnaire.",
                                  "items": {
                                    "type": "object",
                                    "required": [
                                      "question_id",
                                      "title",
                                      "type",
                                      "options",
                                      "reply_id"
                                    ],
                                    "properties": {
                                      "question_id": {
                                        "type": "string",
                                        "description": "The question ID."
                                      },
                                      "title": {
                                        "type": "string",
                                        "description": "The title of the question."
                                      },
                                      "type": {
                                        "type": "string",
                                        "description": "The type of the question (single-choice, multiple-choice, open-text)."
                                      },
                                      "reply_id": {
                                        "type": "string",
                                        "description": "The reply ID."
                                      },
                                      "options": {
                                        "type": "array",
                                        "description": "List of options for multiple-choice questions.",
                                        "items": {
                                          "type": "object",
                                          "required": [
                                            "option_id",
                                            "text"
                                          ],
                                          "properties": {
                                            "option_id": {
                                              "type": "string",
                                              "description": "The option ID."
                                            },
                                            "text": {
                                              "type": "string",
                                              "description": "The text of the option."
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
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"workspace.pre_check_in_questionnaire_response_created\",\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator\": \"yuki-devint@zoom.us\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"object\": {\n      \"start_time\": \"2024-02-14T03:50:00Z\",\n      \"event_id\": \"dxpwbc2fi8tzz7d012rwx9u8c0\",\n      \"user_name\": \"Simash pokh\",\n      \"desk_id\": \"SJjU1CcGSs2L7odn6PanQw\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"questionnaire_respnse\": [\n        {\n          \"id\": \"Fl75C43YTmS_IpMzYLipJw\",\n          \"title\": \"Simash Reservation Questionnaire\",\n          \"list\": [\n            {\n              \"question_id\": \"0ds1NFeDQ1WfWlgVK-7OMQ\",\n              \"title\": \"do you like AI ?\",\n              \"type\": \"single\",\n              \"options\": [\n                {\n                  \"option_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\",\n                  \"text\": \"Yes\"\n                },\n                {\n                  \"option_id\": \"1S1hZzFGTwW9Llxdr2vzyw\",\n                  \"text\": \"No\"\n                }\n              ],\n              \"reply_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\"\n            },\n            {\n              \"question_id\": \"2XMaJFwNSr-BJpTfo6rRAQ\",\n              \"title\": \"what is your fav food ? \",\n              \"type\": \"multiple\",\n              \"options\": [\n                {\n                  \"option_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ\",\n                  \"text\": \"momo\"\n                },\n                {\n                  \"option_id\": \"dpv4EZ7QQUqRhBmYj4_b7w\",\n                  \"text\": \"sea food\"\n                },\n                {\n                  \"option_id\": \"__DjlrV7TeyCSZVmCWAnng\",\n                  \"text\": \"rice\"\n                }\n              ],\n              \"reply_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ,dpv4EZ7QQUqRhBmYj4_b7w\"\n            }\n          ],\n          \"subject\": \"reservation\"\n        }\n      ]\n    }\n  },\n  \"event_ts\": 1707848572310\n}"
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
                      "type": "string",
                      "format": "date-time",
                      "description": "A timestamp when the event occurred."
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that completed the questionnaire."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that completed the questionnaire."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the questionnaire when creating a reservation for a workspace.",
                          "required": [
                            "start_time",
                            "event_id",
                            "user_name",
                            "desk_id",
                            "workspace_name",
                            "location_id",
                            "questionnaire_response"
                          ],
                          "properties": {
                            "start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The start time of the workspace reservation."
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The event ID."
                            },
                            "user_name": {
                              "type": "string",
                              "description": "The user name."
                            },
                            "desk_id": {
                              "type": "string",
                              "description": "The desk ID."
                            },
                            "workspace_name": {
                              "type": "string",
                              "description": "The name of the workspace."
                            },
                            "location_id": {
                              "type": "string",
                              "description": "The location ID."
                            },
                            "questionnaire_response": {
                              "type": "array",
                              "description": "List of questionnaires.",
                              "items": {
                                "type": "object",
                                "required": [
                                  "id",
                                  "title",
                                  "subject",
                                  "list"
                                ],
                                "properties": {
                                  "id": {
                                    "type": "string",
                                    "description": "The view ID."
                                  },
                                  "title": {
                                    "type": "string",
                                    "description": "The title of the questionnaire."
                                  },
                                  "subject": {
                                    "type": "string",
                                    "description": "The subject of the questionnaire."
                                  },
                                  "list": {
                                    "type": "array",
                                    "description": "List of questions in the questionnaire.",
                                    "items": {
                                      "type": "object",
                                      "required": [
                                        "question_id",
                                        "title",
                                        "type",
                                        "options",
                                        "reply_id"
                                      ],
                                      "properties": {
                                        "question_id": {
                                          "type": "string",
                                          "description": "The question ID."
                                        },
                                        "title": {
                                          "type": "string",
                                          "description": "The title of the question."
                                        },
                                        "type": {
                                          "type": "string",
                                          "description": "The type of the question (single-choice, multiple-choice, open-text)."
                                        },
                                        "reply_id": {
                                          "type": "string",
                                          "description": "The reply ID."
                                        },
                                        "options": {
                                          "type": "array",
                                          "description": "List of options for multiple-choice questions.",
                                          "items": {
                                            "type": "object",
                                            "required": [
                                              "option_id",
                                              "text"
                                            ],
                                            "properties": {
                                              "option_id": {
                                                "type": "string",
                                                "description": "The option ID."
                                              },
                                              "text": {
                                                "type": "string",
                                                "description": "The text of the option."
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
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"workspace.pre_check_in_questionnaire_response_created\",\n  \"payload\": {\n    \"account_id\": \"VQYZ9ytmTbaXK9Zr2LvIrg\",\n    \"operator\": \"yuki-devint@zoom.us\",\n    \"operator_id\": \"rZXNXVlVSTaSZDTJvLcBew\",\n    \"object\": {\n      \"start_time\": \"2024-02-14T03:50:00Z\",\n      \"event_id\": \"dxpwbc2fi8tzz7d012rwx9u8c0\",\n      \"user_name\": \"Simash pokh\",\n      \"desk_id\": \"SJjU1CcGSs2L7odn6PanQw\",\n      \"workspace_name\": \"test change date - Desk1\",\n      \"location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"questionnaire_respnse\": [\n        {\n          \"id\": \"Fl75C43YTmS_IpMzYLipJw\",\n          \"title\": \"Simash Reservation Questionnaire\",\n          \"list\": [\n            {\n              \"question_id\": \"0ds1NFeDQ1WfWlgVK-7OMQ\",\n              \"title\": \"do you like AI ?\",\n              \"type\": \"single\",\n              \"options\": [\n                {\n                  \"option_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\",\n                  \"text\": \"Yes\"\n                },\n                {\n                  \"option_id\": \"1S1hZzFGTwW9Llxdr2vzyw\",\n                  \"text\": \"No\"\n                }\n              ],\n              \"reply_id\": \"UClQqM1qT_q-bZQ0Xti7Vw\"\n            },\n            {\n              \"question_id\": \"2XMaJFwNSr-BJpTfo6rRAQ\",\n              \"title\": \"what is your fav food ? \",\n              \"type\": \"multiple\",\n              \"options\": [\n                {\n                  \"option_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ\",\n                  \"text\": \"momo\"\n                },\n                {\n                  \"option_id\": \"dpv4EZ7QQUqRhBmYj4_b7w\",\n                  \"text\": \"sea food\"\n                },\n                {\n                  \"option_id\": \"__DjlrV7TeyCSZVmCWAnng\",\n                  \"text\": \"rice\"\n                }\n              ],\n              \"reply_id\": \"iPkq2UDOSLOLHNq_Ac1WsQ,dpv4EZ7QQUqRhBmYj4_b7w\"\n            }\n          ],\n          \"subject\": \"reservation\"\n        }\n      ]\n    }\n  },\n  \"event_ts\": 1707848572310\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.reservation_checked_out": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.reservation_checked_out",
        "requestBody": {
          "description": "# workspace.reservation_checked_out\n\nThe **Workspace Reservation Checked Out** event is triggered when a user checks out of their [Workspace reservation](https://support.zoom.us/hc/en-us/articles/4417951085965-Using-Workspace-Reservation-Beta-for-Zoom-Rooms) to confirm that they are releasing the Zoom Room or Zoom device-enabled desk.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\n* The Zoom mobile app. This is required for QR code scanning:\n  * Android version 5.9.0.3502 or higher\n  * iOS version 5.9.0 or higher\n* [Zoom Rooms for Touch devices](https://support.zoom.us/hc/en-us/articles/115004773103), version 5.9.0 or higher (*Optional*).\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace Reservation Checked Out** subscription enabled under the **Workspace Reservation** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:reservation`,`workspace:read:reservation:admin`\n\n**Event type**: `workspace.reservation_checked_out`\n",
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that checked out the reservation."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that checked out the reservation."
                      },
                      "object": {
                        "description": "Information about the workspace reservation.",
                        "type": "object",
                        "required": [
                          "id",
                          "workspace_id",
                          "end_time",
                          "start_time",
                          "topic"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The reservation ID."
                          },
                          "workspace_id": {
                            "type": "string",
                            "description": "The workspace ID."
                          },
                          "end_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's end time."
                          },
                          "start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's start time."
                          },
                          "topic": {
                            "type": "string",
                            "description": "The reservation's description."
                          },
                          "reserve_for": {
                            "type": "string",
                            "description": "The ID of the user for whom to create a workspace reservation.",
                            "example": "KDcuGIm1QgePTO8WbOqwIQ"
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
                  "value": "{\n\t\"event\": \"workspace.reservation_checked_out\",\n\t\"event_ts\": \"290112312321\",\n\t\"payload\": {\n\t\t\"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n\t\t\"operator\" : \"admin@example.com\",\n\t    \"operator_id\" : \"I7zp-pYbTl22Z29H796Ixg\",\n\t\t\"object\": {\n\t\t\t\"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n\t\t\t\"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n\t\t\t\"topic\": \"Reserve Zoom Room-01\",\n\t\t\t\"start_time\": \"2022-01-19T02:10:00Z\",\n\t\t\t\"end_time\": \"2022-01-19T03:55:00Z\",\n\t\t\t\"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\"\n\t\t}\n\t}\n}"
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that checked out the reservation."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that checked out the reservation."
                        },
                        "object": {
                          "description": "Information about the workspace reservation.",
                          "type": "object",
                          "required": [
                            "id",
                            "workspace_id",
                            "end_time",
                            "start_time",
                            "topic"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The reservation ID."
                            },
                            "workspace_id": {
                              "type": "string",
                              "description": "The workspace ID."
                            },
                            "end_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's end time."
                            },
                            "start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's start time."
                            },
                            "topic": {
                              "type": "string",
                              "description": "The reservation's description."
                            },
                            "reserve_for": {
                              "type": "string",
                              "description": "The ID of the user for whom to create a workspace reservation.",
                              "example": "KDcuGIm1QgePTO8WbOqwIQ"
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
                    "value": "{\n\t\"event\": \"workspace.reservation_checked_out\",\n\t\"event_ts\": \"290112312321\",\n\t\"payload\": {\n\t\t\"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n\t\t\"operator\" : \"admin@example.com\",\n\t    \"operator_id\" : \"I7zp-pYbTl22Z29H796Ixg\",\n\t\t\"object\": {\n\t\t\t\"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n\t\t\t\"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n\t\t\t\"topic\": \"Reserve Zoom Room-01\",\n\t\t\t\"start_time\": \"2022-01-19T02:10:00Z\",\n\t\t\t\"end_time\": \"2022-01-19T03:55:00Z\",\n\t\t\t\"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\"\n\t\t}\n\t}\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "visitor_management.checked_in": {
      "post": {
        "tags": [
          "visitor_management"
        ],
        "operationId": "visitor_management.checked_in",
        "requestBody": {
          "description": "# visitor_management.checked_in\nThe **Visitor Management Checked In** event is triggered when a [visitor checks in](https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0073629#h_63082000168761703023137137).\n\n## Prerequisites\n\n* The [**Visitor Management**](https://support.zoom.com/hc/en?id=zm_kb&sysparm_article=KB0057564) feature enabled by your organization.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Visitor Management Checked In** subscription enabled under the **Visitor Management** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:visitor_checked_in:admin`\n\n**Event type**: `visitor_management.checked_in`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The host user account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the host."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the host."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the checked in invitation.",
                        "required": [
                          "visitor_email",
                          "visitor_name",
                          "start_time",
                          "end_time",
                          "invite_location_id",
                          "host_id",
                          "host_name",
                          "host_email"
                        ],
                        "properties": {
                          "visitor_email": {
                            "type": "string",
                            "description": "The email address of the invited visitor."
                          },
                          "visitor_name": {
                            "type": "string",
                            "description": "The name of the invited visitor."
                          },
                          "start_time": {
                            "type": "long",
                            "description": "The epoch timestamp representing the start time of the invitation."
                          },
                          "end_time": {
                            "type": "long",
                            "description": "The epoch timestamp representing end time of the invitation."
                          },
                          "invite_location_id": {
                            "type": "string",
                            "description": "The location ID of the location to which the visitor is invited to."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The user ID of the invitation host."
                          },
                          "host_name": {
                            "type": "string",
                            "description": "The name of the invitation host."
                          },
                          "host_email": {
                            "type": "string",
                            "description": "The email address of the invitation host."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"visitor_management.checked_in\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"start_time\": 1710153000000,\n      \"end_time\": 1710172799999,\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The host user account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the host."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the host."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the checked in invitation.",
                          "required": [
                            "visitor_email",
                            "visitor_name",
                            "start_time",
                            "end_time",
                            "invite_location_id",
                            "host_id",
                            "host_name",
                            "host_email"
                          ],
                          "properties": {
                            "visitor_email": {
                              "type": "string",
                              "description": "The email address of the invited visitor."
                            },
                            "visitor_name": {
                              "type": "string",
                              "description": "The name of the invited visitor."
                            },
                            "start_time": {
                              "type": "long",
                              "description": "The epoch timestamp representing the start time of the invitation."
                            },
                            "end_time": {
                              "type": "long",
                              "description": "The epoch timestamp representing end time of the invitation."
                            },
                            "invite_location_id": {
                              "type": "string",
                              "description": "The location ID of the location to which the visitor is invited to."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The user ID of the invitation host."
                            },
                            "host_name": {
                              "type": "string",
                              "description": "The name of the invitation host."
                            },
                            "host_email": {
                              "type": "string",
                              "description": "The email address of the invitation host."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"visitor_management.checked_in\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"start_time\": 1710153000000,\n      \"end_time\": 1710172799999,\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.reservation_deleted": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.reservation_deleted",
        "requestBody": {
          "description": "# workspace.reservation_deleted\n\nThe **Workspace Reservation Deleted** event is triggered when a user deletes their [Workspace reservation](https://support.zoom.us/hc/en-us/articles/4417951085965-Using-Workspace-Reservation-Beta-for-Zoom-Rooms).\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\n* The Zoom mobile app. This is required for QR code scanning:\n  * Android version 5.9.0.3502 or higher\n  * iOS version 5.9.0 or higher\n* [Zoom Rooms for Touch devices](https://support.zoom.us/hc/en-us/articles/115004773103), version 5.9.0 or higher (*Optional*).\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace Reservation Deleted** subscription enabled under the **Workspace Reservation** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:reservation`,`workspace:read:reservation:admin`\n\n**Event type**: `workspace.reservation_deleted`\n",
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that deleted the reservation."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that deleted the reservation."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the workspace reservation.",
                        "required": [
                          "id",
                          "workspace_id",
                          "end_time",
                          "start_time",
                          "topic"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The reservation ID."
                          },
                          "workspace_id": {
                            "type": "string",
                            "description": "The workspace ID."
                          },
                          "end_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's end time."
                          },
                          "start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's start time."
                          },
                          "topic": {
                            "type": "string",
                            "description": "The reservation's description."
                          },
                          "reserve_for": {
                            "type": "string",
                            "description": "The ID of the user for whom to create a workspace reservation.",
                            "example": "KDcuGIm1QgePTO8WbOqwIQ"
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
                  "value": "{\n\t\"event\": \"workspace.reservation_deleted\",\n\t\"event_ts\": \"290112312321\",\n\t\"payload\": {\n\t\t\"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n\t\t\"operator\" : \"admin@example.com\",\n\t    \"operator_id\" : \"I7zp-pYbTl22Z29H796Ixg\",\n\t\t\"object\": {\n\t\t\t\"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n\t\t\t\"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n\t\t\t\"topic\": \"Reserve Zoom Room-01\",\n\t\t\t\"start_time\": \"2022-01-19T02:10:00Z\",\n\t\t\t\"end_time\": \"2022-01-19T03:55:00Z\",\n\t\t\t\"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\"\n\t\t}\n\t}\n}"
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that deleted the reservation."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that deleted the reservation."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the workspace reservation.",
                          "required": [
                            "id",
                            "workspace_id",
                            "end_time",
                            "start_time",
                            "topic"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The reservation ID."
                            },
                            "workspace_id": {
                              "type": "string",
                              "description": "The workspace ID."
                            },
                            "end_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's end time."
                            },
                            "start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's start time."
                            },
                            "topic": {
                              "type": "string",
                              "description": "The reservation's description."
                            },
                            "reserve_for": {
                              "type": "string",
                              "description": "The ID of the user for whom to create a workspace reservation.",
                              "example": "KDcuGIm1QgePTO8WbOqwIQ"
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
                    "value": "{\n\t\"event\": \"workspace.reservation_deleted\",\n\t\"event_ts\": \"290112312321\",\n\t\"payload\": {\n\t\t\"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n\t\t\"operator\" : \"admin@example.com\",\n\t    \"operator_id\" : \"I7zp-pYbTl22Z29H796Ixg\",\n\t\t\"object\": {\n\t\t\t\"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n\t\t\t\"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n\t\t\t\"topic\": \"Reserve Zoom Room-01\",\n\t\t\t\"start_time\": \"2022-01-19T02:10:00Z\",\n\t\t\t\"end_time\": \"2022-01-19T03:55:00Z\",\n\t\t\t\"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\"\n\t\t}\n\t}\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "visitor_management.invitation_created": {
      "post": {
        "tags": [
          "visitor_management"
        ],
        "operationId": "visitor_management.invitation_created",
        "requestBody": {
          "description": "# visitor_management.invitation_created\n\nThe **Visitor Management Invitation Created** event is triggered when an [invitation is created](https://support.zoom.com/hc/en?id=zm_kb&sysparm_article=KB0073629#mcetoc_1hi1ubsr1fb).\n\n## Prerequisites\n\n* The [**Visitor Management**](https://support.zoom.com/hc/en?id=zm_kb&sysparm_article=KB0057564) feature enabled by your organization.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Visitor Management Invitation Created** subscription enabled under the **Visitor Management** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:visitor_invitation:admin`\n\n**Event type**: `visitor_management.invitation_created`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The user account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that created the invitation."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that created the invitation."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the created invitation.",
                        "required": [
                          "invitation_id",
                          "visitor_email",
                          "visitor_name",
                          "start_time",
                          "end_time",
                          "invite_location_id",
                          "host_id",
                          "host_name",
                          "host_email"
                        ],
                        "properties": {
                          "invitation_id": {
                            "type": "string",
                            "description": "The invitation ID."
                          },
                          "visitor_email": {
                            "type": "string",
                            "description": "The email address of the invited visitor."
                          },
                          "visitor_name": {
                            "type": "string",
                            "description": "The name of the invited visitor."
                          },
                          "start_time": {
                            "type": "long",
                            "description": "The epoch timestamp representing the start time of the invitation."
                          },
                          "end_time": {
                            "type": "long",
                            "description": "The epoch timestamp representing end time of the invitation."
                          },
                          "invite_location_id": {
                            "type": "string",
                            "description": "The location ID of the location to which the visitor is invited to."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The user ID of the invitation host."
                          },
                          "host_name": {
                            "type": "string",
                            "description": "The name of the invitation host."
                          },
                          "host_email": {
                            "type": "string",
                            "description": "The email address of the invitation host."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"visitor_management.invitation_created\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"invitation_id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"start_time\": 1710153000000,\n      \"end_time\": 1710172799999,\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The user account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that created the invitation."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that created the invitation."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the created invitation.",
                          "required": [
                            "invitation_id",
                            "visitor_email",
                            "visitor_name",
                            "start_time",
                            "end_time",
                            "invite_location_id",
                            "host_id",
                            "host_name",
                            "host_email"
                          ],
                          "properties": {
                            "invitation_id": {
                              "type": "string",
                              "description": "The invitation ID."
                            },
                            "visitor_email": {
                              "type": "string",
                              "description": "The email address of the invited visitor."
                            },
                            "visitor_name": {
                              "type": "string",
                              "description": "The name of the invited visitor."
                            },
                            "start_time": {
                              "type": "long",
                              "description": "The epoch timestamp representing the start time of the invitation."
                            },
                            "end_time": {
                              "type": "long",
                              "description": "The epoch timestamp representing end time of the invitation."
                            },
                            "invite_location_id": {
                              "type": "string",
                              "description": "The location ID of the location to which the visitor is invited to."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The user ID of the invitation host."
                            },
                            "host_name": {
                              "type": "string",
                              "description": "The name of the invitation host."
                            },
                            "host_email": {
                              "type": "string",
                              "description": "The email address of the invitation host."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"visitor_management.invitation_created\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"invitation_id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"start_time\": 1710153000000,\n      \"end_time\": 1710172799999,\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.location_deleted": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.location_deleted",
        "requestBody": {
          "description": "# zoomroom.location_deleted\nThe **Zoom Rooms location has been deleted** event is triggered every time a Zoom Room location is deleted in your account. Admins and account owners can delete Zoom Rooms locations in a Zoom account.  Deleting Zoom Rooms locations permanently removes them and their data from the Zoom system.  To delete a Zoom Rooms location, delete the Zoom Rooms location from the Zoom Web Portal itself.\n\n## Prerequisites\n\n* Have your organization enable the **Zoom Rooms** feature through a Zoom Rooms license, or Workspace Reservation feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or **Workspace Reservation** license.\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Rooms Location Deleted** subscription enabled under the **Zoom Rooms** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\n\n**Event type**: `zoomroom.location_deleted`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who deleted the Zoom Rooms location."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that deleted the Zoom Rooms location."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that deleted the Zoom Rooms location."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the deleted location.",
                        "required": [
                          "id",
                          "name",
                          "parent_location_id",
                          "type"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Rooms location's ID."
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the deleted Zoom Rooms location."
                          },
                          "parent_location_id": {
                            "type": "string",
                            "description": "This location's parent Zoom Rooms location ID."
                          },
                          "type": {
                            "type": "enum, country\u2503states\u2503city\u2503campus\u2503building\u2503floor",
                            "description": "The location type."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"zoomroom.location_deleted\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"name\": \"My Deleted Zoom Room Location\",\n      \"parent_location_id\": \"aldjflajsd-fasj\",\n      \"type\": \"floor\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who deleted the Zoom Rooms location."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that deleted the Zoom Rooms location."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that deleted the Zoom Rooms location."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the deleted location.",
                          "required": [
                            "id",
                            "name",
                            "parent_location_id",
                            "type"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Rooms location's ID."
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the deleted Zoom Rooms location."
                            },
                            "parent_location_id": {
                              "type": "string",
                              "description": "This location's parent Zoom Rooms location ID."
                            },
                            "type": {
                              "type": "enum, country\u2503states\u2503city\u2503campus\u2503building\u2503floor",
                              "description": "The location type."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"zoomroom.location_deleted\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"name\": \"My Deleted Zoom Room Location\",\n      \"parent_location_id\": \"aldjflajsd-fasj\",\n      \"type\": \"floor\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.reservation_created": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.reservation_created",
        "requestBody": {
          "description": "# workspace.reservation_created\n\nThe **Workspace Reservation Created** event is triggered when a user [reserves](https://support.zoom.us/hc/en-us/articles/4417951085965-Using-Workspace-Reservation-Beta-for-Zoom-Rooms) a Zoom Room or Zoom device-enabled desk.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\n* The Zoom mobile app. This is required for QR code scanning:\n  * Android version 5.9.0.3502 or higher\n  * iOS version 5.9.0 or higher\n* [Zoom Rooms for Touch devices](https://support.zoom.us/hc/en-us/articles/115004773103), version 5.9.0 or higher (*Optional*).\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace Reservation Created** subscription enabled under the **Workspace Reservation** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:reservation`,`workspace:read:reservation:admin`\n\n**Event type**: `workspace.reservation_created`\n",
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
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that created the reservation."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that created the reservation."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the workspace reservation.",
                        "required": [
                          "id",
                          "workspace_id",
                          "end_time",
                          "start_time",
                          "topic"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The reservation ID."
                          },
                          "workspace_id": {
                            "type": "string",
                            "description": "The workspace ID."
                          },
                          "end_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's end time."
                          },
                          "start_time": {
                            "type": "string",
                            "format": "date-time",
                            "description": "The reservation's start time."
                          },
                          "topic": {
                            "type": "string",
                            "description": "The reservation's description."
                          },
                          "reserve_for": {
                            "type": "string",
                            "description": "The ID of the user for whom to create a workspace reservation."
                          },
                          "assignment_checkin": {
                            "type": "boolean",
                            "description": "Whether the assigned workspace is checked in."
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
                  "value": "{\n  \"event\": \"workspace.reservation_created\",\n  \"event_ts\": 290112312321,\n  \"payload\": {\n    \"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n      \"topic\": \"Reserve Zoom Room-01\",\n      \"start_time\": \"2022-01-19T02:10:00Z\",\n      \"end_time\": \"2022-01-19T03:55:00Z\",\n      \"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"assignment_checkin\": true\n    }\n  }\n}"
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
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that created the reservation."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that created the reservation."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the workspace reservation.",
                          "required": [
                            "id",
                            "workspace_id",
                            "end_time",
                            "start_time",
                            "topic"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The reservation ID."
                            },
                            "workspace_id": {
                              "type": "string",
                              "description": "The workspace ID."
                            },
                            "end_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's end time."
                            },
                            "start_time": {
                              "type": "string",
                              "format": "date-time",
                              "description": "The reservation's start time."
                            },
                            "topic": {
                              "type": "string",
                              "description": "The reservation's description."
                            },
                            "reserve_for": {
                              "type": "string",
                              "description": "The ID of the user for whom to create a workspace reservation."
                            },
                            "assignment_checkin": {
                              "type": "boolean",
                              "description": "Whether the assigned workspace is checked in."
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
                    "value": "{\n  \"event\": \"workspace.reservation_created\",\n  \"event_ts\": 290112312321,\n  \"payload\": {\n    \"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_id\": \"qwNQb8u1Qbi3Pc-qBPgshy\",\n      \"topic\": \"Reserve Zoom Room-01\",\n      \"start_time\": \"2022-01-19T02:10:00Z\",\n      \"end_time\": \"2022-01-19T03:55:00Z\",\n      \"reserve_for\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"assignment_checkin\": true\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.checked_out": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.checked_out",
        "requestBody": {
          "description": "# zoomroom.checked_out\n\nThe **Zoom Room Checked Out** event is triggered when a user checks out of the Zoom Room to confirm that they are releasing the room for other users to use.\n\nUsers can check out manually via the Zoom Room controller or the scheduling display, or they can check out programmatically using the [**Check-in or check-out of a Zoom Room**](/api-reference/zoom-api/methods#operation/checkInRooms) API.\n\n## Prerequisites\n\n* The administrator or account owner must enable the [**Room Check-In and Check-Out**](https://support.zoom.us/hc/en-us/articles/360001147163-Using-the-Zoom-Rooms-check-in-feature) feature.\n* The Zoom Room version must be 4.7.0 or higher.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Room Checked Out** subscription enabled under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\n\n**Event type**: `zoomroom.checked_out`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "id",
                          "room_name",
                          "calendar_name",
                          "email",
                          "event_id",
                          "change_key",
                          "resource_email",
                          "calendar_id",
                          "calendar_type",
                          "api_type"
                        ],
                        "description": "Information about the Zoom Room.",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Room's unique ID."
                          },
                          "room_name": {
                            "type": "string",
                            "description": "The Zoom Room's name."
                          },
                          "calendar_name": {
                            "type": "string",
                            "description": "The calendar's name."
                          },
                          "email": {
                            "type": "string",
                            "format": "email",
                            "description": "The Zoom Room's associated email address."
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The unique ID of the Zoom Room's associated calendar event:\n\n* For Microsoft Exchange calendar integration, use Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). The event_id value is the Id value in the ItemId element of the FindItem operation's response. \n* For Google Calendar integration, use Google's [Events: list API](https://developers.google.com/calendar/api/v3/reference/events/list). The event_id value is the id field in the [events resource](https://developers.google.com/calendar/api/v3/reference/events#resource)."
                          },
                          "change_key": {
                            "type": "string",
                            "description": "The specific version of the calendar item. You can get this value using Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). Refer to the ChangeKey field of the ItemId element in the operation's response.\n\nThis field is only returned for Microsoft Exchange or Office 365 calendars."
                          },
                          "resource_email": {
                            "description": "The calendar's associated [resource mailbox](https://support.microsoft.com/en-us/help/10063/creating-and-managing-resource-mailboxes-in-office-365).\n\nThis field is only returned for Microsoft Exchange or Office 365 calendars.",
                            "type": "string"
                          },
                          "calendar_id": {
                            "type": "string",
                            "description": "The calendar's unique ID."
                          },
                          "calendar_type": {
                            "type": "string",
                            "description": "The calendar's type: \n* 0 \u2014 Google Calendar. \n* 1 \u2014 Office 365 Calendar. \n* 2 \u2014 Microsoft Exchange Calendar.",
                            "enum": [
                              "0",
                              "1",
                              "2"
                            ]
                          },
                          "api_type": {
                            "type": "string",
                            "description": "The API type: \n* 0 \u2014 [Exchange Web Services (EWS) API](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-managed-api-reference-for-exchange) \n* 1 \u2014 [Microsoft Graph API](https://docs.microsoft.com/en-us/azure/active-directory/develop/microsoft-graph-intro) \n* An empty string \u2014 The API returns this value when the calendar_type value is 0 (Google Calendar).",
                            "enum": [
                              "0",
                              "1",
                              ""
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
                  "value": "{\n  \"event\": \"zoomroom.checked_out\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"id\": \"abcD3ojfdbjfg\",\n      \"room_name\": \"My Zoom Room\",\n      \"calendar_name\": \"mycalendar@example.com\",\n      \"email\": \"jchill@example.com\",\n      \"event_id\": \"AbbbbbGYxLTc3OTVkMzFmZDc0MwBGAAAAAAD48FI58voYSqDgJePOSZblBwBQ/N0JvB/FRqv5UT2rFfkVAAAAAAENAABQ/N0JvB/FRqv5UT2rFfkVAAE2YC8DAAA=\",\n      \"change_key\": \"DwAAABYAAABQ/N0JvB/FRqv5UT2rFfkVAAE2XqVw\",\n      \"resource_email\": \"zroom1@example.com\",\n      \"calendar_id\": \"mycalendar@example.com\",\n      \"calendar_type\": \"2\",\n      \"api_type\": \"0\"\n    }\n  }\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "id",
                            "room_name",
                            "calendar_name",
                            "email",
                            "event_id",
                            "change_key",
                            "resource_email",
                            "calendar_id",
                            "calendar_type",
                            "api_type"
                          ],
                          "description": "Information about the Zoom Room.",
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Room's unique ID."
                            },
                            "room_name": {
                              "type": "string",
                              "description": "The Zoom Room's name."
                            },
                            "calendar_name": {
                              "type": "string",
                              "description": "The calendar's name."
                            },
                            "email": {
                              "type": "string",
                              "format": "email",
                              "description": "The Zoom Room's associated email address."
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The unique ID of the Zoom Room's associated calendar event:\n\n* For Microsoft Exchange calendar integration, use Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). The event_id value is the Id value in the ItemId element of the FindItem operation's response. \n* For Google Calendar integration, use Google's [Events: list API](https://developers.google.com/calendar/api/v3/reference/events/list). The event_id value is the id field in the [events resource](https://developers.google.com/calendar/api/v3/reference/events#resource)."
                            },
                            "change_key": {
                              "type": "string",
                              "description": "The specific version of the calendar item. You can get this value using Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). Refer to the ChangeKey field of the ItemId element in the operation's response.\n\nThis field is only returned for Microsoft Exchange or Office 365 calendars."
                            },
                            "resource_email": {
                              "description": "The calendar's associated [resource mailbox](https://support.microsoft.com/en-us/help/10063/creating-and-managing-resource-mailboxes-in-office-365).\n\nThis field is only returned for Microsoft Exchange or Office 365 calendars.",
                              "type": "string"
                            },
                            "calendar_id": {
                              "type": "string",
                              "description": "The calendar's unique ID."
                            },
                            "calendar_type": {
                              "type": "string",
                              "description": "The calendar's type: \n* 0 \u2014 Google Calendar. \n* 1 \u2014 Office 365 Calendar. \n* 2 \u2014 Microsoft Exchange Calendar.",
                              "enum": [
                                "0",
                                "1",
                                "2"
                              ]
                            },
                            "api_type": {
                              "type": "string",
                              "description": "The API type: \n* 0 \u2014 [Exchange Web Services (EWS) API](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-managed-api-reference-for-exchange) \n* 1 \u2014 [Microsoft Graph API](https://docs.microsoft.com/en-us/azure/active-directory/develop/microsoft-graph-intro) \n* An empty string \u2014 The API returns this value when the calendar_type value is 0 (Google Calendar).",
                              "enum": [
                                "0",
                                "1",
                                ""
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
                    "value": "{\n  \"event\": \"zoomroom.checked_out\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"id\": \"abcD3ojfdbjfg\",\n      \"room_name\": \"My Zoom Room\",\n      \"calendar_name\": \"mycalendar@example.com\",\n      \"email\": \"jchill@example.com\",\n      \"event_id\": \"AbbbbbGYxLTc3OTVkMzFmZDc0MwBGAAAAAAD48FI58voYSqDgJePOSZblBwBQ/N0JvB/FRqv5UT2rFfkVAAAAAAENAABQ/N0JvB/FRqv5UT2rFfkVAAE2YC8DAAA=\",\n      \"change_key\": \"DwAAABYAAABQ/N0JvB/FRqv5UT2rFfkVAAE2XqVw\",\n      \"resource_email\": \"zroom1@example.com\",\n      \"calendar_id\": \"mycalendar@example.com\",\n      \"calendar_type\": \"2\",\n      \"api_type\": \"0\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.delayed_alert": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.delayed_alert",
        "requestBody": {
          "description": "# zoomroom.delayed_alert\n\nThe **Zoom Room Delayed Alert** event is triggered when there is an issue with a Zoom Room on a Zoom account that has installed your [webhook-enabled](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks) app. Some of the issues that can trigger this event are:\n\n* High CPU usage.\n* Low battery, charging, and/or connection issues in a Zoom Room device.\n* Room controller, camera, or speaker disconnections or reconnections.\n* Missing camera or microphone.\n\nThis event notification is the same as the **Zoom Room Alert** event notification. However, instead of receiving the event in near real-time, you will receive this event notification **five minutes after an issue occurs in the Zoom Room**.\n\nIt is recommended that you **only** subscribe to either the **Zoom Room Alert** event or the **Zoom Room Delayed Alert** event to avoid receiving duplicate event notifications.\n\n## Prerequisites\n\n* A Business or Enterprise plan.\n* The administrator or account owner must enable the [**Zoom Room Alert**](https://support.zoom.us/hc/en-us/articles/205394099-Zoom-Room-Alerts) feature with the selected alerts in the [Zoom Web Portal](https://zoom.us/location).\n* The Zoom Room version must be 3.5.24524.0821 and higher.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Room Delayed Alert** subscription enabled under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:alert:admin`\n\n**Event type**: `zoomroom.delayed_alert`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "id",
                          "room_name",
                          "calendar_name",
                          "email",
                          "issue",
                          "alert_type",
                          "component",
                          "alert_time",
                          "alert_kind"
                        ],
                        "description": "Information about the Zoom Room.",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Room's unique ID."
                          },
                          "room_name": {
                            "type": "string",
                            "description": "The Zoom Room's name."
                          },
                          "calendar_name": {
                            "type": "string",
                            "description": "The calendar's name."
                          },
                          "email": {
                            "type": "string",
                            "format": "email",
                            "description": "The Zoom Room's associated email address."
                          },
                          "issue": {
                            "type": "string",
                            "description": "The issues that occurred during the meeting: \n* Room controller is disconnected. \n* Room controller connected. \n* Selected camera is disconnected. \n* Selected camera is reconnected. \n* Selected microphone is disconnected. \n* Selected microphone is reconnected. \n* Selected speaker is disconnected. \n* Selected speaker is reconnected. \n* Zoom room is offline. \n* Zoom room is online. \n* High CPU usage detected. \n* Low bandwidth network is detected. \n* {name} battery is low. \n* {name} battery is normal. \n* {name} disconnected. \n* {name} is connected.\n* Incorrect Room Passcode Entry Detected. \n* The possible values for {name} are: \n* Zoom Rooms Computer. \n* Controller. \n* Scheduling Display. \n*"
                          },
                          "alert_type": {
                            "type": "integer",
                            "description": "The type of alert sent: \n* 1 \u2014 The controller or scheduling display disconnected. \n* 2 \u2014 The selected microphone, speaker, or camera has disconnected. \n* 3 \u2014 The Zoom Room is offline. \n* 4 \u2014 High CPU usage detected. \n* 5 \u2014 Low network bandwidth detected. \n* 6 \u2014 The controller or scheduling display battery is low. \n* 7 \u2014 The Zoom Rooms display is disconnected. \n* 8 \u2014 The audio device has quality issues. \n* 9 \u2014 Failed SIP registration.\n* 10 \u2014 Incorrect room passcode entry detected.\n* 0 \u2014 Other alert.",
                            "enum": [
                              1,
                              2,
                              3,
                              4,
                              5,
                              6,
                              7,
                              8,
                              9,
                              10,
                              0
                            ]
                          },
                          "component": {
                            "type": "integer",
                            "description": "The affected component: \n* 1 \u2014 Zoom Rooms computer \n* 2 \u2014 Controller \n* 3 \u2014 Scheduling display \n* 4 \u2014 Microphone \n* 5 \u2014 Camera \n* 6 \u2014 Speaker \n* 0 \u2014 Other",
                            "enum": [
                              1,
                              2,
                              3,
                              4,
                              5,
                              6,
                              0
                            ]
                          },
                          "alert_time": {
                            "type": "integer",
                            "format": "int64",
                            "description": "The timestamp when the alert occurred."
                          },
                          "alert_kind": {
                            "type": "integer",
                            "description": "The kind of alert: \n* 1 \u2014 Triggered. There was an encountered problem. When the problem is resolved, a follow-up response will contain this field with the 2 (Cleared) value. \n* 2 \u2014 Cleared. The problem is resolved. \n* 3 \u2014 Observed. There is a potential issue-causing condition. If the issue is cleared (2), there will **not** be a follow-up response. \n* 4 \u2014 Other.",
                            "enum": [
                              1,
                              2,
                              3,
                              4
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
                  "value": "{\n  \"event\": \"zoomroom.delayed_alert\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"id\": \"abcD3ojfdbjfg\",\n      \"room_name\": \"My Zoom Room\",\n      \"calendar_name\": \"mycalendar@example.com\",\n      \"email\": \"jchill@example.com\",\n      \"issue\": \"Room Controller disconnected\",\n      \"alert_type\": 1,\n      \"component\": 2,\n      \"alert_kind\": 1\n    }\n  }\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "id",
                            "room_name",
                            "calendar_name",
                            "email",
                            "issue",
                            "alert_type",
                            "component",
                            "alert_time",
                            "alert_kind"
                          ],
                          "description": "Information about the Zoom Room.",
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Room's unique ID."
                            },
                            "room_name": {
                              "type": "string",
                              "description": "The Zoom Room's name."
                            },
                            "calendar_name": {
                              "type": "string",
                              "description": "The calendar's name."
                            },
                            "email": {
                              "type": "string",
                              "format": "email",
                              "description": "The Zoom Room's associated email address."
                            },
                            "issue": {
                              "type": "string",
                              "description": "The issues that occurred during the meeting: \n* Room controller is disconnected. \n* Room controller connected. \n* Selected camera is disconnected. \n* Selected camera is reconnected. \n* Selected microphone is disconnected. \n* Selected microphone is reconnected. \n* Selected speaker is disconnected. \n* Selected speaker is reconnected. \n* Zoom room is offline. \n* Zoom room is online. \n* High CPU usage detected. \n* Low bandwidth network is detected. \n* {name} battery is low. \n* {name} battery is normal. \n* {name} disconnected. \n* {name} is connected.\n* Incorrect Room Passcode Entry Detected. \n* The possible values for {name} are: \n* Zoom Rooms Computer. \n* Controller. \n* Scheduling Display. \n*"
                            },
                            "alert_type": {
                              "type": "integer",
                              "description": "The type of alert sent: \n* 1 \u2014 The controller or scheduling display disconnected. \n* 2 \u2014 The selected microphone, speaker, or camera has disconnected. \n* 3 \u2014 The Zoom Room is offline. \n* 4 \u2014 High CPU usage detected. \n* 5 \u2014 Low network bandwidth detected. \n* 6 \u2014 The controller or scheduling display battery is low. \n* 7 \u2014 The Zoom Rooms display is disconnected. \n* 8 \u2014 The audio device has quality issues. \n* 9 \u2014 Failed SIP registration.\n* 10 \u2014 Incorrect room passcode entry detected.\n* 0 \u2014 Other alert.",
                              "enum": [
                                1,
                                2,
                                3,
                                4,
                                5,
                                6,
                                7,
                                8,
                                9,
                                10,
                                0
                              ]
                            },
                            "component": {
                              "type": "integer",
                              "description": "The affected component: \n* 1 \u2014 Zoom Rooms computer \n* 2 \u2014 Controller \n* 3 \u2014 Scheduling display \n* 4 \u2014 Microphone \n* 5 \u2014 Camera \n* 6 \u2014 Speaker \n* 0 \u2014 Other",
                              "enum": [
                                1,
                                2,
                                3,
                                4,
                                5,
                                6,
                                0
                              ]
                            },
                            "alert_time": {
                              "type": "integer",
                              "format": "int64",
                              "description": "The timestamp when the alert occurred."
                            },
                            "alert_kind": {
                              "type": "integer",
                              "description": "The kind of alert: \n* 1 \u2014 Triggered. There was an encountered problem. When the problem is resolved, a follow-up response will contain this field with the 2 (Cleared) value. \n* 2 \u2014 Cleared. The problem is resolved. \n* 3 \u2014 Observed. There is a potential issue-causing condition. If the issue is cleared (2), there will **not** be a follow-up response. \n* 4 \u2014 Other.",
                              "enum": [
                                1,
                                2,
                                3,
                                4
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
                    "value": "{\n  \"event\": \"zoomroom.delayed_alert\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"id\": \"abcD3ojfdbjfg\",\n      \"room_name\": \"My Zoom Room\",\n      \"calendar_name\": \"mycalendar@example.com\",\n      \"email\": \"jchill@example.com\",\n      \"issue\": \"Room Controller disconnected\",\n      \"alert_type\": 1,\n      \"component\": 2,\n      \"alert_kind\": 1\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.deleted": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.deleted",
        "requestBody": {
          "description": "# workspace.deleted\nThe **Workspace Deleted** event is triggered every time a Workspace is deleted in your account. Admins and account owners can delete a Workspaces to an account. The Workspace can be deleted using the [Delete a Workspace API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/deleteWorkspace) or by deleting the Workspace using the Zoom Web Portal itself.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace Deleted** subscription enabled under the **Workspace** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:workspace`,`workspace:read:workspace:admin`\n\n**Event type**: `workspace.deleted`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who deleted the Workspace."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user who deleted the Workspace."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that deleted the Workspace."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the deleted Workspace.",
                        "required": [
                          "id",
                          "workspace_name"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Workspace ID."
                          },
                          "workspace_name": {
                            "type": "string",
                            "description": "The name of the Workspace."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"workspace.deleted\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\" : \"Workspace Name\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who deleted the Workspace."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user who deleted the Workspace."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that deleted the Workspace."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the deleted Workspace.",
                          "required": [
                            "id",
                            "workspace_name"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Workspace ID."
                            },
                            "workspace_name": {
                              "type": "string",
                              "description": "The name of the Workspace."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"workspace.deleted\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\" : \"Workspace Name\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.deleted": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.deleted",
        "requestBody": {
          "description": "# zoomroom.deleted\n\nThe **Zoom Room Deleted** event is triggered when a user deletes a [**Zoom Room**](https://support.zoom.us/hc/en-us/categories/200108436).\n\n## Prerequisites\n\n* The **Zoom Room** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Room Deleted** subscription enabled under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\n\n**Event type**: `zoomroom.deleted`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "id",
                          "room_name"
                        ],
                        "description": "Information about the Zoom Room.",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Room's unique ID."
                          },
                          "room_name": {
                            "type": "string",
                            "description": "The Zoom Room's name."
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
                  "value": "{\n  \"event\": \"zoomroom.deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n    \"object\": {\n      \"id\": \"2muAb65FTZCGjj6ginHIbQ\",\n      \"room_name\": \"My Zoom Room\"\n    }\n  }\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "id",
                            "room_name"
                          ],
                          "description": "Information about the Zoom Room.",
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Room's unique ID."
                            },
                            "room_name": {
                              "type": "string",
                              "description": "The Zoom Room's name."
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
                    "value": "{\n  \"event\": \"zoomroom.deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n    \"object\": {\n      \"id\": \"2muAb65FTZCGjj6ginHIbQ\",\n      \"room_name\": \"My Zoom Room\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "visitor_management.invitation_updated": {
      "post": {
        "tags": [
          "visitor_management"
        ],
        "operationId": "visitor_management.invitation_updated",
        "requestBody": {
          "description": "# visitor_management.invitation_updated\n\nThe **Visitor Management Invitation Updated** event is triggered when the [invitation is updated](https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0073629#mcetoc_1hi1ubsr1ff).\n\n## Prerequisites\n\n* The [**Visitor Management**](https://support.zoom.com/hc/en?id=zm_kb&sysparm_article=KB0057564) feature enabled by your organization.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Visitor Management Invitation Updated** subscription enabled under the **Visitor Management** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:visitor_invitation:admin`\n\n**Event type**: `visitor_management.invitation_updated`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The user account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that updated the invitation."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that updated the invitation."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the updated invitation.",
                        "required": [
                          "invitation_id",
                          "visitor_email",
                          "visitor_name",
                          "start_time",
                          "end_time",
                          "invite_location_id",
                          "host_id",
                          "host_name",
                          "host_email"
                        ],
                        "properties": {
                          "invitation_id": {
                            "type": "string",
                            "description": "The invitation ID."
                          },
                          "visitor_email": {
                            "type": "string",
                            "description": "The email address of the invited visitor."
                          },
                          "visitor_name": {
                            "type": "string",
                            "description": "The name of the invited visitor."
                          },
                          "start_time": {
                            "type": "long",
                            "description": "The epoch timestamp representing the start time of the invitation."
                          },
                          "end_time": {
                            "type": "long",
                            "description": "The epoch timestamp representing end time of the invitation."
                          },
                          "invite_location_id": {
                            "type": "string",
                            "description": "The location ID of the location to which the visitor is invited to."
                          },
                          "host_id": {
                            "type": "string",
                            "description": "The user ID of the invitation host."
                          },
                          "host_name": {
                            "type": "string",
                            "description": "The name of the invitation host."
                          },
                          "host_email": {
                            "type": "string",
                            "description": "The email address of the invitation host."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"visitor_management.invitation_updated\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"invitation_id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"start_time\": 1710153000000,\n      \"end_time\": 1710172799999,\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The user account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that updated the invitation."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that updated the invitation."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the updated invitation.",
                          "required": [
                            "invitation_id",
                            "visitor_email",
                            "visitor_name",
                            "start_time",
                            "end_time",
                            "invite_location_id",
                            "host_id",
                            "host_name",
                            "host_email"
                          ],
                          "properties": {
                            "invitation_id": {
                              "type": "string",
                              "description": "The invitation ID."
                            },
                            "visitor_email": {
                              "type": "string",
                              "description": "The email address of the invited visitor."
                            },
                            "visitor_name": {
                              "type": "string",
                              "description": "The name of the invited visitor."
                            },
                            "start_time": {
                              "type": "long",
                              "description": "The epoch timestamp representing the start time of the invitation."
                            },
                            "end_time": {
                              "type": "long",
                              "description": "The epoch timestamp representing end time of the invitation."
                            },
                            "invite_location_id": {
                              "type": "string",
                              "description": "The location ID of the location to which the visitor is invited to."
                            },
                            "host_id": {
                              "type": "string",
                              "description": "The user ID of the invitation host."
                            },
                            "host_name": {
                              "type": "string",
                              "description": "The name of the invitation host."
                            },
                            "host_email": {
                              "type": "string",
                              "description": "The email address of the invitation host."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"visitor_management.invitation_updated\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"invitation_id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"visitor_email\": \"visitor@example.com\",\n      \"visitor_name\": \"visitor\",\n      \"start_time\": 1710153000000,\n      \"end_time\": 1710172799999,\n      \"invite_location_id\": \"wnFFoWRwTLeww81UuAeW4g\",\n      \"host_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n      \"host_name\": \"admin\",\n      \"host_email\": \"admin@example.com\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.location_created": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.location_created",
        "requestBody": {
          "description": "# zoomroom.location_created\nThe **Zoom Rooms location has been created** event is triggered every time a new Zoom Room location is created in your account. Admins and account owners can add new Zoom Room Location to an account. The Location can be added using the [Add a Zoom Rooms Location API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/addAZRLocation) or by adding the Zoom Room Location using the Zoom Web Portal itself.\n\n## Prerequisites\n\n* Have your organization enable the **Zoom Rooms** feature through a Zoom Rooms license, or Workspace Reservation feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or **Workspace Reservation** license.\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Rooms Location Created** subscription enabled under the **Zoom Rooms** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\n\n**Event type**: `zoomroom.location_created`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who created the new Zoom Rooms location."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that created the new Zoom Rooms location."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that created the new Zoom Rooms location."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the created location.",
                        "required": [
                          "id",
                          "name",
                          "parent_location_id",
                          "type"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Rooms location's ID."
                          },
                          "name": {
                            "type": "string",
                            "description": "The name of the new Zoom Rooms location."
                          },
                          "parent_location_id": {
                            "type": "string",
                            "description": "This location's parent Zoom Rooms location ID."
                          },
                          "type": {
                            "type": "enum, country\u2503states\u2503city\u2503campus\u2503building\u2503floor",
                            "description": "The location's type."
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"zoomroom.location_created\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"name\": \"My new Zoom Room Location\",\n      \"parent_location_id\": \"aldjflajsd-fasj\",\n      \"type\": \"floor\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who created the new Zoom Rooms location."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that created the new Zoom Rooms location."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that created the new Zoom Rooms location."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the created location.",
                          "required": [
                            "id",
                            "name",
                            "parent_location_id",
                            "type"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Rooms location's ID."
                            },
                            "name": {
                              "type": "string",
                              "description": "The name of the new Zoom Rooms location."
                            },
                            "parent_location_id": {
                              "type": "string",
                              "description": "This location's parent Zoom Rooms location ID."
                            },
                            "type": {
                              "type": "enum, country\u2503states\u2503city\u2503campus\u2503building\u2503floor",
                              "description": "The location's type."
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"zoomroom.location_created\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"name\": \"My new Zoom Room Location\",\n      \"parent_location_id\": \"aldjflajsd-fasj\",\n      \"type\": \"floor\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.created": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.created",
        "requestBody": {
          "description": "# zoomroom.created\n\nThe **Zoom Room Created** event is triggered when a user creates a [**Zoom Room**](https://support.zoom.us/hc/en-us/categories/200108436)\n\n## Prerequisites\n\n* The **Zoom Room** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Zoom Room Created** subscription enabled under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\n\n**Event type**: `zoomroom.created`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "id",
                          "room_name"
                        ],
                        "description": "Information about the Zoom Room.",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Zoom Room's unique ID."
                          },
                          "room_name": {
                            "type": "string",
                            "description": "The Zoom Room's name."
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
                  "value": "{\n  \"event\": \"zoomroom.created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n    \"object\": {\n      \"id\": \"2muAb65FTZCGjj6ginHIbQ\",\n      \"room_name\": \"My Zoom Room\"\n    }\n  }\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "id",
                            "room_name"
                          ],
                          "description": "Information about the Zoom Room.",
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Zoom Room's unique ID."
                            },
                            "room_name": {
                              "type": "string",
                              "description": "The Zoom Room's name."
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
                    "value": "{\n  \"event\": \"zoomroom.created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"G3NQs8u1Qni4Pc-qBPgsxg\",\n    \"object\": {\n      \"id\": \"2muAb65FTZCGjj6ginHIbQ\",\n      \"room_name\": \"My Zoom Room\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.created": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.created",
        "requestBody": {
          "description": "# workspace.created\nThe **Workspace created** event is triggered every time a Workspace is created in your account. Admins and account owners can add new Workspaces to an account. The Workspace can be added using the [Create a Workspace API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/createWorkspace) or by adding the Workspace using the Zoom Web Portal itself.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace Created** subscription enabled under the **Workspace** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:workspace`,`workspace:read:workspace:admin`\n\n**Event type**: `workspace.created`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that created the workspace."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that created the workspace."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the created workspace.",
                        "required": [
                          "id",
                          "workspace_name"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Workspace ID."
                          },
                          "workspace_name": {
                            "type": "string",
                            "description": "The name of the workspace"
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"workspace.created\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\": \"My new workspace\"\n    }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that created the workspace."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that created the workspace."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the created workspace.",
                          "required": [
                            "id",
                            "workspace_name"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Workspace ID."
                            },
                            "workspace_name": {
                              "type": "string",
                              "description": "The name of the workspace"
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"workspace.created\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\": \"My new workspace\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "workspace.updated": {
      "post": {
        "tags": [
          "workspace"
        ],
        "operationId": "workspace.updated",
        "requestBody": {
          "description": "# workspace.updated\nThe **Workspace Updated** event is triggered every time a Workspace is updated in your account. Admins and account owners can update a Workspaces to an account. The Workspace can be updated using the [Update a Workspace API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/updateWorkspace) or by updating the Workspace using the Zoom Web Portal itself.\n\n## Prerequisites\n\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Workspace Updated** subscription enabled under the **Workspace** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:workspace`,`workspace:read:workspace:admin`\n\n**Event type**: `workspace.updated`\n",
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
                      "operator",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The Workspace account ID."
                      },
                      "operator": {
                        "type": "string",
                        "format": "email",
                        "description": "The email address of the user that updated the Workspace."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user that updated the Workspace."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the updated Workspace.",
                        "required": [
                          "id",
                          "workspace_name"
                        ],
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The Workspace ID."
                          },
                          "workspace_name": {
                            "type": "string",
                            "description": "The name of the Workspace"
                          }
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"workspace.updated\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\": \"My updated workspace\"\n      }\n  }\n}"
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
                        "operator",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The Workspace account ID."
                        },
                        "operator": {
                          "type": "string",
                          "format": "email",
                          "description": "The email address of the user that updated the Workspace."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user that updated the Workspace."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the updated Workspace.",
                          "required": [
                            "id",
                            "workspace_name"
                          ],
                          "properties": {
                            "id": {
                              "type": "string",
                              "description": "The Workspace ID."
                            },
                            "workspace_name": {
                              "type": "string",
                              "description": "The name of the Workspace"
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"workspace.updated\",\n  \"event_ts\": \"290112312321\",\n  \"payload\": {\n    \"account_id\": \"ajdkd-qBPgsxg\",\n    \"operator\": \"admin@example.com\",\n    \"operator_id\": \"I7zp-pYbTl22Z29H796Ixg\",\n    \"object\": {\n      \"id\": \"fqtckjhjhee89ruidvc2h6lc4g\",\n      \"workspace_name\": \"My updated workspace\"\n      }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.thirdparty_meeting_attempted": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.thirdparty_meeting_attempted",
        "requestBody": {
          "description": "# zoomroom.thirdparty_meeting_attempted\n\nThe **Zoom Room third party meeting attempted** event triggers when a Zoom Room is invited or actively joins a third party meeting.\n\n## Prerequisites\n\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n* A valid **Event Notification Endpoint URL**.\n* Enable the **Zoom Room third party meeting attempted** subscription under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room_control:admin`\n\n**Event type**: `zoomroom.thirdparty_meeting_attempted`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "room_id"
                        ],
                        "description": "A Zoom Room is about to join a meeting.",
                        "properties": {
                          "room_id": {
                            "type": "string",
                            "description": "The Zoom Room ID."
                          },
                          "join_url": {
                            "type": "string",
                            "description": "The URL for joining a meeting."
                          },
                          "meeting_source_type": {
                            "type": "string",
                            "description": "The meeting type.",
                            "enum": [
                              "MS_TEAMS",
                              "GOOGLE_MEET",
                              "WEBEX",
                              "BLUEJEANS",
                              "FUZE",
                              "UNKNOWN"
                            ]
                          },
                          "join_time": {
                            "description": "The time to execute the command to join a meeting.",
                            "type": "string",
                            "format": "datetime",
                            "example": "2022-06-19T00:00:00Z"
                          },
                          "sip_address": {
                            "type": "string",
                            "description": "The full SIP address to join the third-party meeting."
                          },
                          "third_party_meeting_id": {
                            "type": "string",
                            "description": "The of the third-party meeting's meeting ID."
                          },
                          "passcode": {
                            "type": "string",
                            "description": "The passcode for joining a third-party meeting."
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
                  "value": "{\n\t\"event\": \"zoomroom.thirdparty_meeting_attempted\",\n\t\"event_ts\": 1626230691572,\n\t\"payload\": {\n\t\t\"account_id\": \"AAAAAABBBB\",\n\t\t\"object\": {\n\t\t\t\"room_id\": \"abcD3ojfdbjfg\",\n            \"join_time\": \"2022-07-27T16:56:34Z\",\n            \"meeting_source_type\": \"ZOOM\",\n            \"join_url\": \"url\",\n            \"sip_address\": \"sip address\",\n            \"third_party_meeting_id\": \"12121313123\",\n            \"passcode\": \"123\"\n\t\t}\n\t}\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "room_id"
                          ],
                          "description": "A Zoom Room is about to join a meeting.",
                          "properties": {
                            "room_id": {
                              "type": "string",
                              "description": "The Zoom Room ID."
                            },
                            "join_url": {
                              "type": "string",
                              "description": "The URL for joining a meeting."
                            },
                            "meeting_source_type": {
                              "type": "string",
                              "description": "The meeting type.",
                              "enum": [
                                "MS_TEAMS",
                                "GOOGLE_MEET",
                                "WEBEX",
                                "BLUEJEANS",
                                "FUZE",
                                "UNKNOWN"
                              ]
                            },
                            "join_time": {
                              "description": "The time to execute the command to join a meeting.",
                              "type": "string",
                              "format": "datetime",
                              "example": "2022-06-19T00:00:00Z"
                            },
                            "sip_address": {
                              "type": "string",
                              "description": "The full SIP address to join the third-party meeting."
                            },
                            "third_party_meeting_id": {
                              "type": "string",
                              "description": "The of the third-party meeting's meeting ID."
                            },
                            "passcode": {
                              "type": "string",
                              "description": "The passcode for joining a third-party meeting."
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
                    "value": "{\n\t\"event\": \"zoomroom.thirdparty_meeting_attempted\",\n\t\"event_ts\": 1626230691572,\n\t\"payload\": {\n\t\t\"account_id\": \"AAAAAABBBB\",\n\t\t\"object\": {\n\t\t\t\"room_id\": \"abcD3ojfdbjfg\",\n            \"join_time\": \"2022-07-27T16:56:34Z\",\n            \"meeting_source_type\": \"ZOOM\",\n            \"join_url\": \"url\",\n            \"sip_address\": \"sip address\",\n            \"third_party_meeting_id\": \"12121313123\",\n            \"passcode\": \"123\"\n\t\t}\n\t}\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "zoomroom.digital_signage_app_started": {
      "post": {
        "tags": [
          "zoomroom"
        ],
        "operationId": "zoomroom.digital_signage_app_started",
        "requestBody": {
          "description": "# zoomroom.digital_signage_app_started\n\nThe **[Zoom Room digital signage](https://support.zoom.us/hc/en-us/articles/360000030683-Zoom-Rooms-digital-signage) play started** event triggers when the digital signage starts playing or showing.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n* A valid **Event Notification Endpoint URL**.\n* The **Zoom Room digital signage app started** subscription enabled under the **Zoom Room** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:digital_signage_app:admin`\n\n**Event type**: `zoomroom.digital_signage_app_started`\n",
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
                        "description": "The Zoom Room account ID."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "room_id",
                          "app_name",
                          "app_id",
                          "start_time"
                        ],
                        "description": "The information on the digital signage played in a Zoom Room.",
                        "properties": {
                          "room_id": {
                            "type": "string",
                            "description": "The Zoom Room ID."
                          },
                          "app_name": {
                            "type": "string",
                            "description": "The app name."
                          },
                          "app_id": {
                            "type": "string",
                            "description": "The app ID."
                          },
                          "start_time": {
                            "description": "The playback start time of the digital signage.",
                            "type": "string",
                            "format": "datetime",
                            "example": "2022-06-19T00:00:00Z"
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
                  "value": "{\n\t\"event\": \"zoomroom.digital_signage_play_started\",\n\t\"event_ts\": 1626230691572,\n\t\"payload\": {\n\t\t\"account_id\": \"AAAAAABBBB\",\n\t\t\"object\": {\n\t\t\t\"room_id\": \"abcD3ojfdbjfg\",\n\t\t\t\"app_name\": \"My Zoom Room\",\n\t\t\t\"app_id\": \"NiVvY1NWpE2nulNrhVjgU4jD0swziVrXBbaFZyC3u+o=\",\n\t\t\t\"start_time\": \"2022-06-19T00:00:00Z\",\n            \"end_time\": \"2022-06-19T01:01:00Z\"\n\t\t}\n\t}\n}"
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
                          "description": "The Zoom Room account ID."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "room_id",
                            "app_name",
                            "app_id",
                            "start_time"
                          ],
                          "description": "The information on the digital signage played in a Zoom Room.",
                          "properties": {
                            "room_id": {
                              "type": "string",
                              "description": "The Zoom Room ID."
                            },
                            "app_name": {
                              "type": "string",
                              "description": "The app name."
                            },
                            "app_id": {
                              "type": "string",
                              "description": "The app ID."
                            },
                            "start_time": {
                              "description": "The playback start time of the digital signage.",
                              "type": "string",
                              "format": "datetime",
                              "example": "2022-06-19T00:00:00Z"
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
                    "value": "{\n\t\"event\": \"zoomroom.digital_signage_play_started\",\n\t\"event_ts\": 1626230691572,\n\t\"payload\": {\n\t\t\"account_id\": \"AAAAAABBBB\",\n\t\t\"object\": {\n\t\t\t\"room_id\": \"abcD3ojfdbjfg\",\n\t\t\t\"app_name\": \"My Zoom Room\",\n\t\t\t\"app_id\": \"NiVvY1NWpE2nulNrhVjgU4jD0swziVrXBbaFZyC3u+o=\",\n\t\t\t\"start_time\": \"2022-06-19T00:00:00Z\",\n            \"end_time\": \"2022-06-19T01:01:00Z\"\n\t\t}\n\t}\n}"
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
      "rooms",
      "events"
    ]
  },
  "downloadPath": "/api-hub/rooms/events/webhooks.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": true
  },
  "productMeta": {
    "specType": "adoc",
    "slug": "/docs/api/rest/reference/zoom-rooms/events",
    "title": "Zoom API Events - Zoom Rooms",
    "description": "",
    "keywords": "zoom, marketplace, chatbot, chat, bot, webhook, event, zoom rooms, interactive message, button, message, action",
    "zoapEndpoint": "webhookByProduct",
    "params": {
      "productName": "Zoom Rooms"
    },
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "productDisplayTitle": "Rooms",
    "scalarSlug": "/docs/api/rooms/events",
    "svgname": "_zoomrooms"
  },
  "frontmatter": {
    "title": "Rooms Webhooks",
    "keywords": "zoom, marketplace, chatbot, chat, bot, webhook, event, zoom rooms, interactive message, button, message, action",
    "description": "The Zoom Rooms webhooks allow developers to receive events for [Zoom Rooms](https://developers.zoom.us/docs/zoom-rooms/) features.",
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
        "rooms",
        "events"
      ]
    },
    "downloadPath": "/api-hub/rooms/events/webhooks.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": true
    },
    "productMeta": {
      "specType": "adoc",
      "slug": "/docs/api/rest/reference/zoom-rooms/events",
      "title": "Zoom API Events - Zoom Rooms",
      "description": "",
      "keywords": "zoom, marketplace, chatbot, chat, bot, webhook, event, zoom rooms, interactive message, button, message, action",
      "zoapEndpoint": "webhookByProduct",
      "params": {
        "productName": "Zoom Rooms"
      },
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "productDisplayTitle": "Rooms",
      "scalarSlug": "/docs/api/rooms/events",
      "svgname": "_zoomrooms"
    },
    "spec": "{\"openapi\":\"3.1.0\",\"info\":{\"title\":\"Rooms\",\"description\":\"The Zoom Rooms webhooks allow developers to receive events for [Zoom Rooms](https://developers.zoom.us/docs/zoom-rooms/) features.\",\"version\":\"1.0.0\"},\"paths\":{\"visitor_management.checked_out\":{\"post\":{\"tags\":[\"visitor_management\"],\"operationId\":\"visitor_management.checked_out\",\"requestBody\":{\"description\":\"# visitor_management.checked_out\\nThe **Visitor Management Checked Out** event is triggered when a visitor checks out.\\n\\n## Prerequisites\\n\\n* The [**Visitor Management**](https://support.zoom.com/hc/en?id=zm_kb&sysparm_article=KB0057564) feature enabled by your organization.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Visitor Management Checked Out** subscription enabled under the **Visitor Management** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:visitor_checked_out:admin`\\n\\n**Event type**: `visitor_management.checked_out`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The host user account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the host.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the host.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the checked out invitation.\",\"required\":[\"visitor_email\",\"visitor_name\",\"start_time\",\"end_time\",\"invite_location_id\",\"host_id\",\"host_name\",\"host_email\"],\"properties\":{\"visitor_email\":{\"type\":\"string\",\"description\":\"The email address of the invited visitor.\"},\"visitor_name\":{\"type\":\"string\",\"description\":\"The name of the invited visitor.\"},\"start_time\":{\"type\":\"long\",\"description\":\"The epoch timestamp representing the start time of the invitation.\"},\"end_time\":{\"type\":\"long\",\"description\":\"The epoch timestamp representing end time of the invitation.\"},\"invite_location_id\":{\"type\":\"string\",\"description\":\"The location ID of the location to which the visitor is invited to.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The user ID of the invitation host.\"},\"host_name\":{\"type\":\"string\",\"description\":\"The name of the invitation host.\"},\"host_email\":{\"type\":\"string\",\"description\":\"The email address of the invitation host.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"visitor_management.checked_out\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"visitor_email\\\": \\\"visitor@example.com\\\",\\n      \\\"visitor_name\\\": \\\"visitor\\\",\\n      \\\"start_time\\\": 1710153000000,\\n      \\\"end_time\\\": 1710172799999,\\n      \\\"invite_location_id\\\": \\\"wnFFoWRwTLeww81UuAeW4g\\\",\\n      \\\"host_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n      \\\"host_name\\\": \\\"admin\\\",\\n      \\\"host_email\\\": \\\"admin@example.com\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The host user account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the host.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the host.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the checked out invitation.\",\"required\":[\"visitor_email\",\"visitor_name\",\"start_time\",\"end_time\",\"invite_location_id\",\"host_id\",\"host_name\",\"host_email\"],\"properties\":{\"visitor_email\":{\"type\":\"string\",\"description\":\"The email address of the invited visitor.\"},\"visitor_name\":{\"type\":\"string\",\"description\":\"The name of the invited visitor.\"},\"start_time\":{\"type\":\"long\",\"description\":\"The epoch timestamp representing the start time of the invitation.\"},\"end_time\":{\"type\":\"long\",\"description\":\"The epoch timestamp representing end time of the invitation.\"},\"invite_location_id\":{\"type\":\"string\",\"description\":\"The location ID of the location to which the visitor is invited to.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The user ID of the invitation host.\"},\"host_name\":{\"type\":\"string\",\"description\":\"The name of the invitation host.\"},\"host_email\":{\"type\":\"string\",\"description\":\"The email address of the invitation host.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"visitor_management.checked_out\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"visitor_email\\\": \\\"visitor@example.com\\\",\\n      \\\"visitor_name\\\": \\\"visitor\\\",\\n      \\\"start_time\\\": 1710153000000,\\n      \\\"end_time\\\": 1710172799999,\\n      \\\"invite_location_id\\\": \\\"wnFFoWRwTLeww81UuAeW4g\\\",\\n      \\\"host_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n      \\\"host_name\\\": \\\"admin\\\",\\n      \\\"host_email\\\": \\\"admin@example.com\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"workspace.user_unassigned\":{\"post\":{\"tags\":[\"workspace\"],\"operationId\":\"workspace.user_unassigned\",\"requestBody\":{\"description\":\"# workspace.user_unassigned\\nThe **Workspace User Unassigned** event is triggered every time a user is unassigned from a Workspace in your account. Admins and account owners can unassign a user from Workspace in you account. The Workspace can be unassign users using the [Delete a desk assignment API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/Deleteadeskassignment) or by deleting the Workspace' assignment using the Zoom Web Portal itself.\\n\\n## Prerequisites\\n\\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Workspace User Unassigned** subscription enabled under the **Workspace** event.\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:assignment`,`workspace:read:assignment:admin`\\n\\n**Event type**: `workspace.user_unassigned`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that updated the assignment.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that updated the assignment.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the user unassigned when unassign a user from a workspace.\",\"required\":[\"workspace_id\",\"workspace_name\",\"account_id\",\"user_id\",\"assign_type\"],\"properties\":{\"workspace_id\":{\"type\":\"string\",\"description\":\"The workspace's ID.\"},\"workspace_name\":{\"type\":\"string\",\"description\":\"The workspace's name.\"},\"account_id\":{\"type\":\"string\",\"description\":\"The belong's account id of assigned user.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The id of assigned user.\"},\"assign_type\":{\"type\":\"string\",\"description\":\"The type of assign before unassign. \",\"enum\":[\"permanent\"]}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"workspace.user_unassigned\\\",\\n  \\\"event_ts\\\": 1707848572310,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"VQYZ9ytmTbaXK9Zr2LvIrg\\\",\\n    \\\"operator_id\\\": \\\"rZXNXVlVSTaSZDTJvLcBew\\\",\\n    \\\"operator\\\": \\\"vic-devint@zoom.us\\\",\\n    \\\"object\\\": {\\n      \\\"workspace_id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"workspace_name\\\": \\\"test change date - Desk1\\\",\\n      \\\"account_id\\\": \\\"bhmIXpdYRcCK6BB3VhMq6w\\\",\\n      \\\"user_id\\\": \\\"oyLyxW48QeC3uBBFY2CF3A\\\",\\n      \\\"assign_type\\\": \\\"permanent\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that updated the assignment.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that updated the assignment.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the user unassigned when unassign a user from a workspace.\",\"required\":[\"workspace_id\",\"workspace_name\",\"account_id\",\"user_id\",\"assign_type\"],\"properties\":{\"workspace_id\":{\"type\":\"string\",\"description\":\"The workspace's ID.\"},\"workspace_name\":{\"type\":\"string\",\"description\":\"The workspace's name.\"},\"account_id\":{\"type\":\"string\",\"description\":\"The belong's account id of assigned user.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The id of assigned user.\"},\"assign_type\":{\"type\":\"string\",\"description\":\"The type of assign before unassign. \",\"enum\":[\"permanent\"]}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"workspace.user_unassigned\\\",\\n  \\\"event_ts\\\": 1707848572310,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"VQYZ9ytmTbaXK9Zr2LvIrg\\\",\\n    \\\"operator_id\\\": \\\"rZXNXVlVSTaSZDTJvLcBew\\\",\\n    \\\"operator\\\": \\\"vic-devint@zoom.us\\\",\\n    \\\"object\\\": {\\n      \\\"workspace_id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"workspace_name\\\": \\\"test change date - Desk1\\\",\\n      \\\"account_id\\\": \\\"bhmIXpdYRcCK6BB3VhMq6w\\\",\\n      \\\"user_id\\\": \\\"oyLyxW48QeC3uBBFY2CF3A\\\",\\n      \\\"assign_type\\\": \\\"permanent\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"workspace.reservation_questionnaire_response_created\":{\"post\":{\"tags\":[\"workspace\"],\"operationId\":\"workspace.reservation_questionnaire_response_created\",\"requestBody\":{\"description\":\"# workspace.reservation_questionnaire_response_created\\nThe **Workspace reservation questionnaire created** event is triggered every time when a questionnaire is submitted as a part of workspace reservation. Admin and Users can create a workspace reservation and submit a questionnaire during the reservation creation process.\\n\\n## Prerequisites\\n\\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Workspace reservation questionnaire created** subscription enabled under the **Workspace** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:workspace_reservation_questionnaire_response`,`workspace:read:workspace_reservation_questionnaire_response:admin`\\n\\n**Event type**: `workspace.reservation_questionnaire_response_created`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that completed the questionnaire.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that completed the questionnaire.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the questionnaire when creating a reservation for a workspace.\",\"required\":[\"start_time\",\"event_id\",\"user_name\",\"desk_id\",\"workspace_name\",\"location_id\",\"questionnaire_response\"],\"properties\":{\"start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The start time of the workspace reservation.\"},\"event_id\":{\"type\":\"string\",\"description\":\"The event ID.\"},\"user_name\":{\"type\":\"string\",\"description\":\"The user name.\"},\"desk_id\":{\"type\":\"string\",\"description\":\"The desk ID.\"},\"workspace_name\":{\"type\":\"string\",\"description\":\"The name of the workspace.\"},\"location_id\":{\"type\":\"string\",\"description\":\"The location ID.\"},\"questionnaire_response\":{\"type\":\"array\",\"description\":\"List of questionnaires.\",\"items\":{\"type\":\"object\",\"required\":[\"id\",\"title\",\"subject\",\"list\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The view ID.\"},\"title\":{\"type\":\"string\",\"description\":\"The title of the questionnaire.\"},\"subject\":{\"type\":\"string\",\"description\":\"The subject of the questionnaire.\"},\"list\":{\"type\":\"array\",\"description\":\"List of questions in the questionnaire.\",\"items\":{\"type\":\"object\",\"required\":[\"question_id\",\"title\",\"type\",\"options\",\"reply_id\"],\"properties\":{\"question_id\":{\"type\":\"string\",\"description\":\"The question ID.\"},\"title\":{\"type\":\"string\",\"description\":\"The title of the question.\"},\"type\":{\"type\":\"string\",\"description\":\"The type of the question (single-choice, multiple-choice, open-text).\"},\"reply_id\":{\"type\":\"string\",\"description\":\"The reply ID.\"},\"options\":{\"type\":\"array\",\"description\":\"List of options for multiple-choice questions.\",\"items\":{\"type\":\"object\",\"required\":[\"option_id\",\"text\"],\"properties\":{\"option_id\":{\"type\":\"string\",\"description\":\"The option ID.\"},\"text\":{\"type\":\"string\",\"description\":\"The text of the option.\"}}}}}}}}}}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"workspace.reservation_questionnaire_response_created\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"VQYZ9ytmTbaXK9Zr2LvIrg\\\",\\n    \\\"operator\\\": \\\"yuki-devint@zoom.us\\\",\\n    \\\"operator_id\\\": \\\"rZXNXVlVSTaSZDTJvLcBew\\\",\\n    \\\"object\\\": {\\n      \\\"start_time\\\": \\\"2024-02-14T03:50:00Z\\\",\\n      \\\"event_id\\\": \\\"dxpwbc2fi8tzz7d012rwx9u8c0\\\",\\n      \\\"user_name\\\": \\\"Simash pokh\\\",\\n      \\\"desk_id\\\": \\\"SJjU1CcGSs2L7odn6PanQw\\\",\\n      \\\"workspace_name\\\": \\\"test change date - Desk1\\\",\\n      \\\"location_id\\\": \\\"wnFFoWRwTLeww81UuAeW4g\\\",\\n      \\\"questionnaire_response\\\": [\\n        {\\n          \\\"id\\\": \\\"Fl75C43YTmS_IpMzYLipJw\\\",\\n          \\\"title\\\": \\\"Simash Reservation Questionnaire\\\",\\n          \\\"list\\\": [\\n            {\\n              \\\"question_id\\\": \\\"0ds1NFeDQ1WfWlgVK-7OMQ\\\",\\n              \\\"title\\\": \\\"do you like AI ?\\\",\\n              \\\"type\\\": \\\"single\\\",\\n              \\\"options\\\": [\\n                {\\n                  \\\"option_id\\\": \\\"UClQqM1qT_q-bZQ0Xti7Vw\\\",\\n                  \\\"text\\\": \\\"Yes\\\"\\n                },\\n                {\\n                  \\\"option_id\\\": \\\"1S1hZzFGTwW9Llxdr2vzyw\\\",\\n                  \\\"text\\\": \\\"No\\\"\\n                }\\n              ],\\n              \\\"reply_id\\\": \\\"UClQqM1qT_q-bZQ0Xti7Vw\\\"\\n            },\\n            {\\n              \\\"question_id\\\": \\\"2XMaJFwNSr-BJpTfo6rRAQ\\\",\\n              \\\"title\\\": \\\"what is your fav food ? \\\",\\n              \\\"type\\\": \\\"multiple\\\",\\n              \\\"options\\\": [\\n                {\\n                  \\\"option_id\\\": \\\"iPkq2UDOSLOLHNq_Ac1WsQ\\\",\\n                  \\\"text\\\": \\\"momo\\\"\\n                },\\n                {\\n                  \\\"option_id\\\": \\\"dpv4EZ7QQUqRhBmYj4_b7w\\\",\\n                  \\\"text\\\": \\\"sea food\\\"\\n                },\\n                {\\n                  \\\"option_id\\\": \\\"__DjlrV7TeyCSZVmCWAnng\\\",\\n                  \\\"text\\\": \\\"rice\\\"\\n                }\\n              ],\\n              \\\"reply_id\\\": \\\"iPkq2UDOSLOLHNq_Ac1WsQ,dpv4EZ7QQUqRhBmYj4_b7w\\\"\\n            }\\n          ],\\n          \\\"subject\\\": \\\"reservation\\\"\\n        }\\n      ]\\n    }\\n  },\\n  \\\"event_ts\\\": 1707848572310\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that completed the questionnaire.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that completed the questionnaire.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the questionnaire when creating a reservation for a workspace.\",\"required\":[\"start_time\",\"event_id\",\"user_name\",\"desk_id\",\"workspace_name\",\"location_id\",\"questionnaire_response\"],\"properties\":{\"start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The start time of the workspace reservation.\"},\"event_id\":{\"type\":\"string\",\"description\":\"The event ID.\"},\"user_name\":{\"type\":\"string\",\"description\":\"The user name.\"},\"desk_id\":{\"type\":\"string\",\"description\":\"The desk ID.\"},\"workspace_name\":{\"type\":\"string\",\"description\":\"The name of the workspace.\"},\"location_id\":{\"type\":\"string\",\"description\":\"The location ID.\"},\"questionnaire_response\":{\"type\":\"array\",\"description\":\"List of questionnaires.\",\"items\":{\"type\":\"object\",\"required\":[\"id\",\"title\",\"subject\",\"list\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The view ID.\"},\"title\":{\"type\":\"string\",\"description\":\"The title of the questionnaire.\"},\"subject\":{\"type\":\"string\",\"description\":\"The subject of the questionnaire.\"},\"list\":{\"type\":\"array\",\"description\":\"List of questions in the questionnaire.\",\"items\":{\"type\":\"object\",\"required\":[\"question_id\",\"title\",\"type\",\"options\",\"reply_id\"],\"properties\":{\"question_id\":{\"type\":\"string\",\"description\":\"The question ID.\"},\"title\":{\"type\":\"string\",\"description\":\"The title of the question.\"},\"type\":{\"type\":\"string\",\"description\":\"The type of the question (single-choice, multiple-choice, open-text).\"},\"reply_id\":{\"type\":\"string\",\"description\":\"The reply ID.\"},\"options\":{\"type\":\"array\",\"description\":\"List of options for multiple-choice questions.\",\"items\":{\"type\":\"object\",\"required\":[\"option_id\",\"text\"],\"properties\":{\"option_id\":{\"type\":\"string\",\"description\":\"The option ID.\"},\"text\":{\"type\":\"string\",\"description\":\"The text of the option.\"}}}}}}}}}}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"workspace.reservation_questionnaire_response_created\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"VQYZ9ytmTbaXK9Zr2LvIrg\\\",\\n    \\\"operator\\\": \\\"yuki-devint@zoom.us\\\",\\n    \\\"operator_id\\\": \\\"rZXNXVlVSTaSZDTJvLcBew\\\",\\n    \\\"object\\\": {\\n      \\\"start_time\\\": \\\"2024-02-14T03:50:00Z\\\",\\n      \\\"event_id\\\": \\\"dxpwbc2fi8tzz7d012rwx9u8c0\\\",\\n      \\\"user_name\\\": \\\"Simash pokh\\\",\\n      \\\"desk_id\\\": \\\"SJjU1CcGSs2L7odn6PanQw\\\",\\n      \\\"workspace_name\\\": \\\"test change date - Desk1\\\",\\n      \\\"location_id\\\": \\\"wnFFoWRwTLeww81UuAeW4g\\\",\\n      \\\"questionnaire_response\\\": [\\n        {\\n          \\\"id\\\": \\\"Fl75C43YTmS_IpMzYLipJw\\\",\\n          \\\"title\\\": \\\"Simash Reservation Questionnaire\\\",\\n          \\\"list\\\": [\\n            {\\n              \\\"question_id\\\": \\\"0ds1NFeDQ1WfWlgVK-7OMQ\\\",\\n              \\\"title\\\": \\\"do you like AI ?\\\",\\n              \\\"type\\\": \\\"single\\\",\\n              \\\"options\\\": [\\n                {\\n                  \\\"option_id\\\": \\\"UClQqM1qT_q-bZQ0Xti7Vw\\\",\\n                  \\\"text\\\": \\\"Yes\\\"\\n                },\\n                {\\n                  \\\"option_id\\\": \\\"1S1hZzFGTwW9Llxdr2vzyw\\\",\\n                  \\\"text\\\": \\\"No\\\"\\n                }\\n              ],\\n              \\\"reply_id\\\": \\\"UClQqM1qT_q-bZQ0Xti7Vw\\\"\\n            },\\n            {\\n              \\\"question_id\\\": \\\"2XMaJFwNSr-BJpTfo6rRAQ\\\",\\n              \\\"title\\\": \\\"what is your fav food ? \\\",\\n              \\\"type\\\": \\\"multiple\\\",\\n              \\\"options\\\": [\\n                {\\n                  \\\"option_id\\\": \\\"iPkq2UDOSLOLHNq_Ac1WsQ\\\",\\n                  \\\"text\\\": \\\"momo\\\"\\n                },\\n                {\\n                  \\\"option_id\\\": \\\"dpv4EZ7QQUqRhBmYj4_b7w\\\",\\n                  \\\"text\\\": \\\"sea food\\\"\\n                },\\n                {\\n                  \\\"option_id\\\": \\\"__DjlrV7TeyCSZVmCWAnng\\\",\\n                  \\\"text\\\": \\\"rice\\\"\\n                }\\n              ],\\n              \\\"reply_id\\\": \\\"iPkq2UDOSLOLHNq_Ac1WsQ,dpv4EZ7QQUqRhBmYj4_b7w\\\"\\n            }\\n          ],\\n          \\\"subject\\\": \\\"reservation\\\"\\n        }\\n      ]\\n    }\\n  },\\n  \\\"event_ts\\\": 1707848572310\\n}\"}}}}}}}},\"zoomroom.sensor_data\":{\"post\":{\"tags\":[\"zoomroom\"],\"operationId\":\"zoomroom.sensor_data\",\"requestBody\":{\"description\":\"# zoomroom.sensor_data\\n\\nThe **Zoom Room Sensor Data** event triggers every five minutes when a Zoom Room device sensor reports data.\\n\\n## Prerequisites\\n\\n* A Business or Enterprise plan.\\n* The ability to detect the room air conditions, a feature available with Zoom Rooms version 5.5.0 or later. Not supported for Mac and Windows Zoom Rooms or Windows Controller.\\n* Count the number of people in room: available on Zoom Rooms clients versions 5.5.0 (Windows), 5.5.0 (Mac), 5.6.6 (Android), 5.5.0 (iOS Controller), 5.5.0 (Android Controller) or later.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Zoom Room Sensor Data** subscription enabled under the **Zoom Room** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:sensor_data:admin`\\n\\n**Event type**: `zoomroom.sensor_data`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"id\",\"room_name\",\"device_id\",\"sensor_data\"],\"description\":\"Information about the Zoom Room.\",\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Room ID.\"},\"room_name\":{\"type\":\"string\",\"description\":\"The Zoom Room name.\"},\"device_id\":{\"type\":\"string\",\"description\":\"The device ID.\"},\"sensor_data\":{\"type\":\"array\",\"description\":\"Information about the sensor data.\",\"items\":{\"type\":\"object\",\"properties\":{\"date_time\":{\"description\":\"The time when the sensor data was reported.\",\"type\":\"string\",\"format\":\"datetime\",\"example\":\"2022-06-19T00:00:00Z\"},\"sensor_type\":{\"type\":\"string\",\"enum\":[\"CO2\",\"TEMPERATURE\",\"REAL_TIME_PEOPLE_COUNT\",\"HUMIDITY\",\"VOC\",\"AQI\",\"PM25\",\"PM10\",\"AIR_PRESSURE\"],\"description\":\"The sensor type.\"},\"sensor_value\":{\"type\":\"string\",\"description\":\"The sensor value.\"}},\"required\":[\"date_time\",\"sensor_type\",\"sensor_value\"]}}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n\\t\\\"event\\\": \\\"zoomroom.sensor_data\\\",\\n\\t\\\"event_ts\\\": 1626230691572,\\n\\t\\\"payload\\\": {\\n\\t\\t\\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n\\t\\t\\\"object\\\": {\\n\\t\\t\\t\\\"id\\\": \\\"abcD3ojfdbjfg\\\",\\n\\t\\t\\t\\\"room_name\\\": \\\"My Zoom Room\\\",\\n\\t\\t\\t\\\"device_id\\\": \\\"NiVvY1NWpE2nulNrhVjgU4jD0swziVrXBbaFZyC3u+o=\\\",\\n\\t\\t\\t\\\"sensor_data\\\": [{\\n\\t\\t\\t\\t\\\"date_time\\\": \\\"2022-06-19T00:00:00Z\\\",\\n\\t\\t\\t\\t\\\"sensor_type\\\": \\\"REAL_TIME_PEOPLE_COUNT\\\",\\n\\t\\t\\t\\t\\\"sensor_value\\\": \\\"20\\\"\\n\\t\\t\\t}]\\n\\t\\t}\\n\\t}\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"id\",\"room_name\",\"device_id\",\"sensor_data\"],\"description\":\"Information about the Zoom Room.\",\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Room ID.\"},\"room_name\":{\"type\":\"string\",\"description\":\"The Zoom Room name.\"},\"device_id\":{\"type\":\"string\",\"description\":\"The device ID.\"},\"sensor_data\":{\"type\":\"array\",\"description\":\"Information about the sensor data.\",\"items\":{\"type\":\"object\",\"properties\":{\"date_time\":{\"description\":\"The time when the sensor data was reported.\",\"type\":\"string\",\"format\":\"datetime\",\"example\":\"2022-06-19T00:00:00Z\"},\"sensor_type\":{\"type\":\"string\",\"enum\":[\"CO2\",\"TEMPERATURE\",\"REAL_TIME_PEOPLE_COUNT\",\"HUMIDITY\",\"VOC\",\"AQI\",\"PM25\",\"PM10\",\"AIR_PRESSURE\"],\"description\":\"The sensor type.\"},\"sensor_value\":{\"type\":\"string\",\"description\":\"The sensor value.\"}},\"required\":[\"date_time\",\"sensor_type\",\"sensor_value\"]}}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n\\t\\\"event\\\": \\\"zoomroom.sensor_data\\\",\\n\\t\\\"event_ts\\\": 1626230691572,\\n\\t\\\"payload\\\": {\\n\\t\\t\\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n\\t\\t\\\"object\\\": {\\n\\t\\t\\t\\\"id\\\": \\\"abcD3ojfdbjfg\\\",\\n\\t\\t\\t\\\"room_name\\\": \\\"My Zoom Room\\\",\\n\\t\\t\\t\\\"device_id\\\": \\\"NiVvY1NWpE2nulNrhVjgU4jD0swziVrXBbaFZyC3u+o=\\\",\\n\\t\\t\\t\\\"sensor_data\\\": [{\\n\\t\\t\\t\\t\\\"date_time\\\": \\\"2022-06-19T00:00:00Z\\\",\\n\\t\\t\\t\\t\\\"sensor_type\\\": \\\"REAL_TIME_PEOPLE_COUNT\\\",\\n\\t\\t\\t\\t\\\"sensor_value\\\": \\\"20\\\"\\n\\t\\t\\t}]\\n\\t\\t}\\n\\t}\\n}\"}}}}}}}},\"visitor_management.invitation_deleted\":{\"post\":{\"tags\":[\"visitor_management\"],\"operationId\":\"visitor_management.invitation_deleted\",\"requestBody\":{\"description\":\"# visitor_management.invitation_deleted\\n\\nThe **Visitor Management Invitation Deleted** event is triggered when an [invitation is deleted](https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0073629#mcetoc_1hi1ubsr1fh).\\n\\n## Prerequisites\\n\\n* The [**Visitor Management**](https://support.zoom.com/hc/en?id=zm_kb&sysparm_article=KB0057564) feature enabled by your organization.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Visitor Management Invitation Deleted** subscription enabled under the **Visitor Management** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:visitor_invitation:admin`\\n\\n**Event type**: `visitor_management.invitation_deleted`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The user account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that deleted the invitation.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that deleted the invitation.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the deleted invitation.\",\"required\":[\"visitor_email\",\"visitor_name\",\"invite_location_id\",\"host_id\",\"host_name\",\"host_email\"],\"properties\":{\"visitor_email\":{\"type\":\"string\",\"description\":\"The email address of the invited visitor.\"},\"visitor_name\":{\"type\":\"string\",\"description\":\"The name of the invited visitor.\"},\"invite_location_id\":{\"type\":\"string\",\"description\":\"The location ID of the location to which the visitor is invited to.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The user ID of the invitation host.\"},\"host_name\":{\"type\":\"string\",\"description\":\"The name of the invitation host.\"},\"host_email\":{\"type\":\"string\",\"description\":\"The email address of the invitation host.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"visitor_management.invitation_deleted\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"visitor_email\\\": \\\"visitor@example.com\\\",\\n      \\\"visitor_name\\\": \\\"visitor\\\",\\n      \\\"invite_location_id\\\": \\\"wnFFoWRwTLeww81UuAeW4g\\\",\\n      \\\"host_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n      \\\"host_name\\\": \\\"admin\\\",\\n      \\\"host_email\\\": \\\"admin@example.com\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The user account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that deleted the invitation.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that deleted the invitation.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the deleted invitation.\",\"required\":[\"visitor_email\",\"visitor_name\",\"invite_location_id\",\"host_id\",\"host_name\",\"host_email\"],\"properties\":{\"visitor_email\":{\"type\":\"string\",\"description\":\"The email address of the invited visitor.\"},\"visitor_name\":{\"type\":\"string\",\"description\":\"The name of the invited visitor.\"},\"invite_location_id\":{\"type\":\"string\",\"description\":\"The location ID of the location to which the visitor is invited to.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The user ID of the invitation host.\"},\"host_name\":{\"type\":\"string\",\"description\":\"The name of the invitation host.\"},\"host_email\":{\"type\":\"string\",\"description\":\"The email address of the invitation host.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"visitor_management.invitation_deleted\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"visitor_email\\\": \\\"visitor@example.com\\\",\\n      \\\"visitor_name\\\": \\\"visitor\\\",\\n      \\\"invite_location_id\\\": \\\"wnFFoWRwTLeww81UuAeW4g\\\",\\n      \\\"host_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n      \\\"host_name\\\": \\\"admin\\\",\\n      \\\"host_email\\\": \\\"admin@example.com\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoomroom.alert\":{\"post\":{\"tags\":[\"zoomroom\"],\"operationId\":\"zoomroom.alert\",\"requestBody\":{\"description\":\"# zoomroom.alert\\n\\nThe **Zoom Room Alert** event is triggered when there is an issue with a Zoom Room on a Zoom account that has installed your [webhook-enabled](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks) app. Some of the issues that can trigger this event are:\\n\\n* High CPU usage.\\n* Low battery, charging, and/or connection issues in a Zoom Room device.\\n* Room controller, camera, or speaker disconnections or reconnections.\\n* Missing camera or microphone.\\n\\nIt is recommended that you **only** subscribe to either the **Zoom Room Alert** event or the **Zoom Room Delayed Alert** event to avoid receiving duplicate event notifications.\\n\\n## Prerequisites\\n\\n* A Business or Enterprise plan.\\n* The administrator or account owner must enable the [**Zoom Room Alert**](https://support.zoom.us/hc/en-us/articles/205394099-Zoom-Room-Alerts) feature with the selected alerts in the [Zoom Web Portal](https://zoom.us/location).\\n* The Zoom Room version must be 3.5.24524.0821 and higher.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Zoom Room Alert** subscription enabled under the **Zoom Room** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:alert:admin`\\n\\n**Event type**: `zoomroom.alert`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"id\",\"room_name\",\"calendar_name\",\"email\",\"issue\",\"alert_type\",\"component\",\"alert_time\",\"alert_kind\"],\"description\":\"Information about the Zoom Room.\",\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Room's unique ID.\"},\"room_name\":{\"type\":\"string\",\"description\":\"The Zoom Room's name.\"},\"calendar_name\":{\"type\":\"string\",\"description\":\"The calendar's name.\"},\"email\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The Zoom Room's associated email address.\"},\"issue\":{\"type\":\"string\",\"description\":\"The issues that occurred during the meeting: \\n* Room Controller disconnected \\n* Room Controller connected \\n* Selected camera has disconnected \\n* Selected camera is reconnected \\n* Selected microphone has disconnected \\n* Selected microphone is reconnected \\n* Selected speaker has disconnected \\n* Selected speaker is reconnected \\n* Zoom room is offline \\n* Zoom room is online \\n* High CPU usage is detected \\n* Low bandwidth network is detected \\n* {name} battery is low \\n* {name} battery is normal \\n* {name} disconnected \\n* {name} connected \\n Incorrect Room Passcode Entry Detected \\n\\nThe possible values for {name} are: \\n* Zoom Rooms Computer \\n* Controller \\n* Scheduling Display \\n*.\"},\"alert_type\":{\"type\":\"integer\",\"description\":\"The type of alert sent: \\n* 1 \u2014 The controller or scheduling display disconnected. \\n* 2 \u2014 The selected microphone, speaker, or camera has disconnected \\n* 3 \u2014 The Zoom Room is offline. \\n* 4 \u2014 High CPU usage detected. \\n* 5 \u2014 Low network bandwidth detected. \\n* 6 \u2014 The controller or scheduling display battery is low. \\n* 7 \u2014 The Zoom Rooms display is disconnected. \\n* 8 \u2014 The audio device has quality issues \\n* 9 \u2014 Failed SIP registration \\n* 10 \u2014 Incorrect room passcode entry detected \\n* 0 \u2014 Other alert.\",\"enum\":[1,2,3,4,5,6,7,8,9,10,0]},\"component\":{\"description\":\"The affected component: \\n* 1 \u2014 Zoom Rooms Computer \\n* 2 \u2014 Controller \\n* 3 \u2014 Scheduling Display \\n* 4 \u2014 Microphone \\n* 5 \u2014 Camera \\n* 6 \u2014 Speaker \\n* 0 - Other.\",\"type\":\"integer\",\"enum\":[1,2,3,4,5,6,0]},\"alert_time\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the alert occurred.\"},\"alert_kind\":{\"type\":\"integer\",\"description\":\"The kind of alert: \\n* 1 \u2014 Triggered. There was an encountered problem. When the problem is resolved, a follow-up response will contain this field with the 2 (Cleared) value. \\n* 2 \u2014 Cleared. The problem is resolved. \\n* 3 \u2014 Observed. There is a potential issue-causing condition. If the issue is cleared (2), there will **not** be a follow-up response. \\n* 4 - Other.\",\"enum\":[1,2,3,4]}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.alert\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"abcD3ojfdbjfg\\\",\\n      \\\"room_name\\\": \\\"My Zoom Room\\\",\\n      \\\"calendar_name\\\": \\\"mycalendar@example.com\\\",\\n      \\\"email\\\": \\\"jchill@example.com\\\",\\n      \\\"issue\\\": \\\"Room Controller disconnected\\\",\\n      \\\"alert_type\\\": 1,\\n      \\\"component\\\": 2,\\n      \\\"alert_kind\\\": 1\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"id\",\"room_name\",\"calendar_name\",\"email\",\"issue\",\"alert_type\",\"component\",\"alert_time\",\"alert_kind\"],\"description\":\"Information about the Zoom Room.\",\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Room's unique ID.\"},\"room_name\":{\"type\":\"string\",\"description\":\"The Zoom Room's name.\"},\"calendar_name\":{\"type\":\"string\",\"description\":\"The calendar's name.\"},\"email\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The Zoom Room's associated email address.\"},\"issue\":{\"type\":\"string\",\"description\":\"The issues that occurred during the meeting: \\n* Room Controller disconnected \\n* Room Controller connected \\n* Selected camera has disconnected \\n* Selected camera is reconnected \\n* Selected microphone has disconnected \\n* Selected microphone is reconnected \\n* Selected speaker has disconnected \\n* Selected speaker is reconnected \\n* Zoom room is offline \\n* Zoom room is online \\n* High CPU usage is detected \\n* Low bandwidth network is detected \\n* {name} battery is low \\n* {name} battery is normal \\n* {name} disconnected \\n* {name} connected \\n Incorrect Room Passcode Entry Detected \\n\\nThe possible values for {name} are: \\n* Zoom Rooms Computer \\n* Controller \\n* Scheduling Display \\n*.\"},\"alert_type\":{\"type\":\"integer\",\"description\":\"The type of alert sent: \\n* 1 \u2014 The controller or scheduling display disconnected. \\n* 2 \u2014 The selected microphone, speaker, or camera has disconnected \\n* 3 \u2014 The Zoom Room is offline. \\n* 4 \u2014 High CPU usage detected. \\n* 5 \u2014 Low network bandwidth detected. \\n* 6 \u2014 The controller or scheduling display battery is low. \\n* 7 \u2014 The Zoom Rooms display is disconnected. \\n* 8 \u2014 The audio device has quality issues \\n* 9 \u2014 Failed SIP registration \\n* 10 \u2014 Incorrect room passcode entry detected \\n* 0 \u2014 Other alert.\",\"enum\":[1,2,3,4,5,6,7,8,9,10,0]},\"component\":{\"description\":\"The affected component: \\n* 1 \u2014 Zoom Rooms Computer \\n* 2 \u2014 Controller \\n* 3 \u2014 Scheduling Display \\n* 4 \u2014 Microphone \\n* 5 \u2014 Camera \\n* 6 \u2014 Speaker \\n* 0 - Other.\",\"type\":\"integer\",\"enum\":[1,2,3,4,5,6,0]},\"alert_time\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the alert occurred.\"},\"alert_kind\":{\"type\":\"integer\",\"description\":\"The kind of alert: \\n* 1 \u2014 Triggered. There was an encountered problem. When the problem is resolved, a follow-up response will contain this field with the 2 (Cleared) value. \\n* 2 \u2014 Cleared. The problem is resolved. \\n* 3 \u2014 Observed. There is a potential issue-causing condition. If the issue is cleared (2), there will **not** be a follow-up response. \\n* 4 - Other.\",\"enum\":[1,2,3,4]}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.alert\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"abcD3ojfdbjfg\\\",\\n      \\\"room_name\\\": \\\"My Zoom Room\\\",\\n      \\\"calendar_name\\\": \\\"mycalendar@example.com\\\",\\n      \\\"email\\\": \\\"jchill@example.com\\\",\\n      \\\"issue\\\": \\\"Room Controller disconnected\\\",\\n      \\\"alert_type\\\": 1,\\n      \\\"component\\\": 2,\\n      \\\"alert_kind\\\": 1\\n    }\\n  }\\n}\"}}}}}}}},\"workspace.reservation_questionnaire_response_updated\":{\"post\":{\"tags\":[\"workspace\"],\"operationId\":\"workspace.reservation_questionnaire_response_updated\",\"requestBody\":{\"description\":\"# workspace.reservation_questionnaire_response_updated\\nThe **Workspace reservation questionnaire updated** event is triggered every time a questionnaire is updated. Users or Admins can update the questionnaire by updating the existing reservation.\\n## Prerequisites\\n\\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Workspace reservation questionnaire updated** subscription enabled under the **Workspace** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:workspace_reservation_questionnaire_response`,`workspace:read:workspace_reservation_questionnaire_response:admin`\\n\\n**Event type**: `workspace.reservation_questionnaire_response_updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that completed the questionnaire.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that completed the questionnaire.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the questionnaire when creating a reservation for a workspace.\",\"required\":[\"start_time\",\"event_id\",\"user_name\",\"desk_id\",\"workspace_name\",\"location_id\",\"questionnaire_response\"],\"properties\":{\"start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The start time of the workspace reservation.\"},\"event_id\":{\"type\":\"string\",\"description\":\"The event ID.\"},\"user_name\":{\"type\":\"string\",\"description\":\"The user name.\"},\"desk_id\":{\"type\":\"string\",\"description\":\"The desk ID.\"},\"workspace_name\":{\"type\":\"string\",\"description\":\"The name of the workspace.\"},\"location_id\":{\"type\":\"string\",\"description\":\"The location ID.\"},\"questionnaire_response\":{\"type\":\"array\",\"description\":\"List of questionnaires.\",\"items\":{\"type\":\"object\",\"required\":[\"id\",\"title\",\"subject\",\"list\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The view ID.\"},\"title\":{\"type\":\"string\",\"description\":\"The title of the questionnaire.\"},\"subject\":{\"type\":\"string\",\"description\":\"The subject of the questionnaire.\"},\"list\":{\"type\":\"array\",\"description\":\"List of questions in the questionnaire.\",\"items\":{\"type\":\"object\",\"required\":[\"question_id\",\"title\",\"type\",\"options\",\"reply_id\"],\"properties\":{\"question_id\":{\"type\":\"string\",\"description\":\"The question ID.\"},\"title\":{\"type\":\"string\",\"description\":\"The title of the question.\"},\"type\":{\"type\":\"string\",\"description\":\"The type of the question (single-choice, multiple-choice, open-text).\"},\"reply_id\":{\"type\":\"string\",\"description\":\"The reply ID.\"},\"options\":{\"type\":\"array\",\"description\":\"List of options for multiple-choice questions.\",\"items\":{\"type\":\"object\",\"required\":[\"option_id\",\"text\"],\"properties\":{\"option_id\":{\"type\":\"string\",\"description\":\"The option ID.\"},\"text\":{\"type\":\"string\",\"description\":\"The text of the option.\"}}}}}}}}}}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"workspace.reservation_questionnaire_response_updated\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"VQYZ9ytmTbaXK9Zr2LvIrg\\\",\\n    \\\"operator\\\": \\\"yuki-devint@zoom.us\\\",\\n    \\\"operator_id\\\": \\\"rZXNXVlVSTaSZDTJvLcBew\\\",\\n    \\\"object\\\": {\\n      \\\"start_time\\\": \\\"2024-02-14T03:50:00Z\\\",\\n      \\\"event_id\\\": \\\"dxpwbc2fi8tzz7d012rwx9u8c0\\\",\\n      \\\"user_name\\\": \\\"Simash pokh\\\",\\n      \\\"desk_id\\\": \\\"SJjU1CcGSs2L7odn6PanQw\\\",\\n      \\\"workspace_name\\\": \\\"test change date - Desk1\\\",\\n      \\\"location_id\\\": \\\"wnFFoWRwTLeww81UuAeW4g\\\",\\n      \\\"questionnaire_response\\\": [\\n        {\\n          \\\"id\\\": \\\"Fl75C43YTmS_IpMzYLipJw\\\",\\n          \\\"title\\\": \\\"Simash Reservation Questionnaire\\\",\\n          \\\"list\\\": [\\n            {\\n              \\\"question_id\\\": \\\"0ds1NFeDQ1WfWlgVK-7OMQ\\\",\\n              \\\"title\\\": \\\"do you like AI ?\\\",\\n              \\\"type\\\": \\\"single\\\",\\n              \\\"options\\\": [\\n                {\\n                  \\\"option_id\\\": \\\"UClQqM1qT_q-bZQ0Xti7Vw\\\",\\n                  \\\"text\\\": \\\"Yes\\\"\\n                },\\n                {\\n                  \\\"option_id\\\": \\\"1S1hZzFGTwW9Llxdr2vzyw\\\",\\n                  \\\"text\\\": \\\"No\\\"\\n                }\\n              ],\\n              \\\"reply_id\\\": \\\"UClQqM1qT_q-bZQ0Xti7Vw\\\"\\n            },\\n            {\\n              \\\"question_id\\\": \\\"2XMaJFwNSr-BJpTfo6rRAQ\\\",\\n              \\\"title\\\": \\\"what is your fav food ? \\\",\\n              \\\"type\\\": \\\"multiple\\\",\\n              \\\"options\\\": [\\n                {\\n                  \\\"option_id\\\": \\\"iPkq2UDOSLOLHNq_Ac1WsQ\\\",\\n                  \\\"text\\\": \\\"momo\\\"\\n                },\\n                {\\n                  \\\"option_id\\\": \\\"dpv4EZ7QQUqRhBmYj4_b7w\\\",\\n                  \\\"text\\\": \\\"sea food\\\"\\n                },\\n                {\\n                  \\\"option_id\\\": \\\"__DjlrV7TeyCSZVmCWAnng\\\",\\n                  \\\"text\\\": \\\"rice\\\"\\n                }\\n              ],\\n              \\\"reply_id\\\": \\\"iPkq2UDOSLOLHNq_Ac1WsQ,dpv4EZ7QQUqRhBmYj4_b7w\\\"\\n            }\\n          ],\\n          \\\"subject\\\": \\\"reservation\\\"\\n        }\\n      ]\\n    }\\n  },\\n  \\\"event_ts\\\": 1707848572310\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that completed the questionnaire.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that completed the questionnaire.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the questionnaire when creating a reservation for a workspace.\",\"required\":[\"start_time\",\"event_id\",\"user_name\",\"desk_id\",\"workspace_name\",\"location_id\",\"questionnaire_response\"],\"properties\":{\"start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The start time of the workspace reservation.\"},\"event_id\":{\"type\":\"string\",\"description\":\"The event ID.\"},\"user_name\":{\"type\":\"string\",\"description\":\"The user name.\"},\"desk_id\":{\"type\":\"string\",\"description\":\"The desk ID.\"},\"workspace_name\":{\"type\":\"string\",\"description\":\"The name of the workspace.\"},\"location_id\":{\"type\":\"string\",\"description\":\"The location ID.\"},\"questionnaire_response\":{\"type\":\"array\",\"description\":\"List of questionnaires.\",\"items\":{\"type\":\"object\",\"required\":[\"id\",\"title\",\"subject\",\"list\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The view ID.\"},\"title\":{\"type\":\"string\",\"description\":\"The title of the questionnaire.\"},\"subject\":{\"type\":\"string\",\"description\":\"The subject of the questionnaire.\"},\"list\":{\"type\":\"array\",\"description\":\"List of questions in the questionnaire.\",\"items\":{\"type\":\"object\",\"required\":[\"question_id\",\"title\",\"type\",\"options\",\"reply_id\"],\"properties\":{\"question_id\":{\"type\":\"string\",\"description\":\"The question ID.\"},\"title\":{\"type\":\"string\",\"description\":\"The title of the question.\"},\"type\":{\"type\":\"string\",\"description\":\"The type of the question (single-choice, multiple-choice, open-text).\"},\"reply_id\":{\"type\":\"string\",\"description\":\"The reply ID.\"},\"options\":{\"type\":\"array\",\"description\":\"List of options for multiple-choice questions.\",\"items\":{\"type\":\"object\",\"required\":[\"option_id\",\"text\"],\"properties\":{\"option_id\":{\"type\":\"string\",\"description\":\"The option ID.\"},\"text\":{\"type\":\"string\",\"description\":\"The text of the option.\"}}}}}}}}}}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"workspace.reservation_questionnaire_response_updated\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"VQYZ9ytmTbaXK9Zr2LvIrg\\\",\\n    \\\"operator\\\": \\\"yuki-devint@zoom.us\\\",\\n    \\\"operator_id\\\": \\\"rZXNXVlVSTaSZDTJvLcBew\\\",\\n    \\\"object\\\": {\\n      \\\"start_time\\\": \\\"2024-02-14T03:50:00Z\\\",\\n      \\\"event_id\\\": \\\"dxpwbc2fi8tzz7d012rwx9u8c0\\\",\\n      \\\"user_name\\\": \\\"Simash pokh\\\",\\n      \\\"desk_id\\\": \\\"SJjU1CcGSs2L7odn6PanQw\\\",\\n      \\\"workspace_name\\\": \\\"test change date - Desk1\\\",\\n      \\\"location_id\\\": \\\"wnFFoWRwTLeww81UuAeW4g\\\",\\n      \\\"questionnaire_response\\\": [\\n        {\\n          \\\"id\\\": \\\"Fl75C43YTmS_IpMzYLipJw\\\",\\n          \\\"title\\\": \\\"Simash Reservation Questionnaire\\\",\\n          \\\"list\\\": [\\n            {\\n              \\\"question_id\\\": \\\"0ds1NFeDQ1WfWlgVK-7OMQ\\\",\\n              \\\"title\\\": \\\"do you like AI ?\\\",\\n              \\\"type\\\": \\\"single\\\",\\n              \\\"options\\\": [\\n                {\\n                  \\\"option_id\\\": \\\"UClQqM1qT_q-bZQ0Xti7Vw\\\",\\n                  \\\"text\\\": \\\"Yes\\\"\\n                },\\n                {\\n                  \\\"option_id\\\": \\\"1S1hZzFGTwW9Llxdr2vzyw\\\",\\n                  \\\"text\\\": \\\"No\\\"\\n                }\\n              ],\\n              \\\"reply_id\\\": \\\"UClQqM1qT_q-bZQ0Xti7Vw\\\"\\n            },\\n            {\\n              \\\"question_id\\\": \\\"2XMaJFwNSr-BJpTfo6rRAQ\\\",\\n              \\\"title\\\": \\\"what is your fav food ? \\\",\\n              \\\"type\\\": \\\"multiple\\\",\\n              \\\"options\\\": [\\n                {\\n                  \\\"option_id\\\": \\\"iPkq2UDOSLOLHNq_Ac1WsQ\\\",\\n                  \\\"text\\\": \\\"momo\\\"\\n                },\\n                {\\n                  \\\"option_id\\\": \\\"dpv4EZ7QQUqRhBmYj4_b7w\\\",\\n                  \\\"text\\\": \\\"sea food\\\"\\n                },\\n                {\\n                  \\\"option_id\\\": \\\"__DjlrV7TeyCSZVmCWAnng\\\",\\n                  \\\"text\\\": \\\"rice\\\"\\n                }\\n              ],\\n              \\\"reply_id\\\": \\\"iPkq2UDOSLOLHNq_Ac1WsQ,dpv4EZ7QQUqRhBmYj4_b7w\\\"\\n            }\\n          ],\\n          \\\"subject\\\": \\\"reservation\\\"\\n        }\\n      ]\\n    }\\n  },\\n  \\\"event_ts\\\": 1707848572310\\n}\"}}}}}}}},\"workspace.reservation_update\":{\"post\":{\"tags\":[\"workspace\"],\"operationId\":\"workspace.reservation_update\",\"requestBody\":{\"description\":\"# workspace.reservation_update\\n\\nThe **Workspace Reservation Updated** event is triggered when a user updates their [Workspace reservation](https://support.zoom.us/hc/en-us/articles/4417951085965-Using-Workspace-Reservation-Beta-for-Zoom-Rooms).\\n\\n## Prerequisites\\n\\n* The **Workspace Reservation** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\\n* The Zoom mobile app. This is required for QR code scanning:\\n  * Android version 5.9.0.3502 or higher\\n  * iOS version 5.9.0 or higher\\n* [Zoom Rooms for Touch devices](https://support.zoom.us/hc/en-us/articles/115004773103), version 5.9.0 or higher (*Optional*).\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Workspace Reservation Updated** subscription enabled under the **Workspace Reservation** event.\\n\\n\\n\\n**Event type**: `workspace.reservation_update`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that updated the reservation.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that updated the reservation.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the workspace reservation.\",\"required\":[\"id\",\"workspace_id\",\"end_time\",\"start_time\",\"topic\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The reservation ID.\"},\"workspace_id\":{\"type\":\"string\",\"description\":\"The workspace ID.\"},\"end_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's end time.\"},\"start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's start time.\"},\"topic\":{\"type\":\"string\",\"description\":\"The reservation's description.\"},\"reserve_for\":{\"type\":\"string\",\"description\":\"The ID of the user for whom to create a workspace reservation.\",\"example\":\"KDcuGIm1QgePTO8WbOqwIQ\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n\\t\\\"event\\\": \\\"workspace.reservation_updated\\\",\\n\\t\\\"event_ts\\\": \\\"290112312321\\\",\\n\\t\\\"payload\\\": {\\n\\t\\t\\\"account_id\\\": \\\"G3NQs8u1Qni4Pc-qBPgsxg\\\",\\n\\t\\t\\\"operator\\\" : \\\"admin@example.com\\\",\\n\\t    \\\"operator_id\\\" : \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n\\t\\t\\\"object\\\": {\\n\\t\\t\\t\\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n\\t\\t\\t\\\"workspace_id\\\": \\\"qwNQb8u1Qbi3Pc-qBPgshy\\\",\\n\\t\\t\\t\\\"topic\\\": \\\"Reserve Zoom Room-01\\\",\\n\\t\\t\\t\\\"start_time\\\": \\\"2022-01-19T02:10:00Z\\\",\\n\\t\\t\\t\\\"end_time\\\": \\\"2022-01-19T03:55:00Z\\\",\\n\\t\\t\\t\\\"reserve_for\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\"\\n\\t\\t}\\n\\t}\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that updated the reservation.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that updated the reservation.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the workspace reservation.\",\"required\":[\"id\",\"workspace_id\",\"end_time\",\"start_time\",\"topic\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The reservation ID.\"},\"workspace_id\":{\"type\":\"string\",\"description\":\"The workspace ID.\"},\"end_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's end time.\"},\"start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's start time.\"},\"topic\":{\"type\":\"string\",\"description\":\"The reservation's description.\"},\"reserve_for\":{\"type\":\"string\",\"description\":\"The ID of the user for whom to create a workspace reservation.\",\"example\":\"KDcuGIm1QgePTO8WbOqwIQ\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n\\t\\\"event\\\": \\\"workspace.reservation_updated\\\",\\n\\t\\\"event_ts\\\": \\\"290112312321\\\",\\n\\t\\\"payload\\\": {\\n\\t\\t\\\"account_id\\\": \\\"G3NQs8u1Qni4Pc-qBPgsxg\\\",\\n\\t\\t\\\"operator\\\" : \\\"admin@example.com\\\",\\n\\t    \\\"operator_id\\\" : \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n\\t\\t\\\"object\\\": {\\n\\t\\t\\t\\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n\\t\\t\\t\\\"workspace_id\\\": \\\"qwNQb8u1Qbi3Pc-qBPgshy\\\",\\n\\t\\t\\t\\\"topic\\\": \\\"Reserve Zoom Room-01\\\",\\n\\t\\t\\t\\\"start_time\\\": \\\"2022-01-19T02:10:00Z\\\",\\n\\t\\t\\t\\\"end_time\\\": \\\"2022-01-19T03:55:00Z\\\",\\n\\t\\t\\t\\\"reserve_for\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\"\\n\\t\\t}\\n\\t}\\n}\"}}}}}}}},\"workspace.user_assigned\":{\"post\":{\"tags\":[\"workspace\"],\"operationId\":\"workspace.user_assigned\",\"requestBody\":{\"description\":\"# workspace.user_assigned\\nThe **Workspace User Assigned** event is triggered every time a user is assigned to a Workspace in your account. Admins and account owners can assign a user to Workspace in you account. The Workspace can be assign users using the [Set a desk assignment API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/setADeskAssignment) or by set the Workspace' assignment using the Zoom Web Portal itself.\\n\\n## Prerequisites\\n\\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Workspace User Assigned** subscription enabled under the **Workspace** event.\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:assignment`,`workspace:read:assignment:admin`\\n\\n**Event type**: `workspace.user_assigned`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that updated the assignment.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that updated the assignment.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the assignment when updating assignment of a workspace.\",\"required\":[\"workspace_id\",\"workspace_name\",\"assign_type\",\"account_id\",\"user_id\",\"user_email\"],\"properties\":{\"workspace_id\":{\"type\":\"string\",\"description\":\"The workspace's ID.\"},\"workspace_name\":{\"type\":\"string\",\"description\":\"The workspace's name.\"},\"assign_type\":{\"type\":\"string\",\"description\":\"The type of assignment. \",\"enum\":[\"permanent\"]},\"account_id\":{\"type\":\"string\",\"description\":\"The belong's account id of assigned user.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The id of assigned user.\"},\"user_email\":{\"type\":\"string\",\"description\":\"The email of assigned user.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"workspace.user_assigned\\\",\\n  \\\"event_ts\\\": 1707848572310,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"VQYZ9ytmTbaXK9Zr2LvIrg\\\",\\n    \\\"operator_id\\\": \\\"rZXNXVlVSTaSZDTJvLcBew\\\",\\n    \\\"operator\\\": \\\"vic-devint@zoom.us\\\",\\n    \\\"object\\\": {\\n      \\\"workspace_id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"workspace_name\\\": \\\"test change date - Desk1\\\",\\n      \\\"assign_type\\\": \\\"permanent\\\",\\n      \\\"account_id\\\": \\\"bhmIXpdYRcCK6BB3VhMq6w\\\",\\n      \\\"user_id\\\": \\\"oyLyxW48QeC3uBBFY2CF3A\\\",\\n      \\\"user_email\\\": \\\"vic-01-devint@zoom.us\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that updated the assignment.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that updated the assignment.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the assignment when updating assignment of a workspace.\",\"required\":[\"workspace_id\",\"workspace_name\",\"assign_type\",\"account_id\",\"user_id\",\"user_email\"],\"properties\":{\"workspace_id\":{\"type\":\"string\",\"description\":\"The workspace's ID.\"},\"workspace_name\":{\"type\":\"string\",\"description\":\"The workspace's name.\"},\"assign_type\":{\"type\":\"string\",\"description\":\"The type of assignment. \",\"enum\":[\"permanent\"]},\"account_id\":{\"type\":\"string\",\"description\":\"The belong's account id of assigned user.\"},\"user_id\":{\"type\":\"string\",\"description\":\"The id of assigned user.\"},\"user_email\":{\"type\":\"string\",\"description\":\"The email of assigned user.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"workspace.user_assigned\\\",\\n  \\\"event_ts\\\": 1707848572310,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"VQYZ9ytmTbaXK9Zr2LvIrg\\\",\\n    \\\"operator_id\\\": \\\"rZXNXVlVSTaSZDTJvLcBew\\\",\\n    \\\"operator\\\": \\\"vic-devint@zoom.us\\\",\\n    \\\"object\\\": {\\n      \\\"workspace_id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"workspace_name\\\": \\\"test change date - Desk1\\\",\\n      \\\"assign_type\\\": \\\"permanent\\\",\\n      \\\"account_id\\\": \\\"bhmIXpdYRcCK6BB3VhMq6w\\\",\\n      \\\"user_id\\\": \\\"oyLyxW48QeC3uBBFY2CF3A\\\",\\n      \\\"user_email\\\": \\\"vic-01-devint@zoom.us\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoomroom.digital_signage_app_ended\":{\"post\":{\"tags\":[\"zoomroom\"],\"operationId\":\"zoomroom.digital_signage_app_ended\",\"requestBody\":{\"description\":\"# zoomroom.digital_signage_app_ended\\n\\nThe **[Zoom Room digital signage](https://support.zoom.us/hc/en-us/articles/360000030683-Zoom-Rooms-digital-signage) play ended** event triggers when the digital signage finishes playing or showing.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n* A valid **Event Notification Endpoint URL**.\\n* The **Zoom Room digital signage app ended** subscription enabled under the **Zoom Room** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:digital_signage_app:admin`\\n\\n**Event type**: `zoomroom.digital_signage_app_ended`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"room_id\",\"app_name\",\"app_id\",\"start_time\",\"end_time\"],\"description\":\"The information on the digital signage played in a Zoom Room.\",\"properties\":{\"room_id\":{\"type\":\"string\",\"description\":\"The Zoom Room ID.\"},\"app_name\":{\"type\":\"string\",\"description\":\"The app name.\"},\"app_id\":{\"type\":\"string\",\"description\":\"The app ID.\"},\"start_time\":{\"description\":\"The playback start time of the digital signage.\",\"type\":\"string\",\"format\":\"datetime\",\"example\":\"2022-06-19T00:00:00Z\"},\"end_time\":{\"description\":\"The playback end time of the digital signage.\",\"type\":\"string\",\"format\":\"datetime\",\"example\":\"2022-06-19T00:00:00Z\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n\\t\\\"event\\\": \\\"zoomroom.digital_signage_app_ended\\\",\\n\\t\\\"event_ts\\\": 1626230691572,\\n\\t\\\"payload\\\": {\\n\\t\\t\\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n\\t\\t\\\"object\\\": {\\n\\t\\t\\t\\\"room_id\\\": \\\"abcD3ojfdbjfg\\\",\\n\\t\\t\\t\\\"app_name\\\": \\\"My Zoom Room\\\",\\n\\t\\t\\t\\\"app_id\\\": \\\"NiVvY1NWpE2nulNrhVjgU4jD0swziVrXBbaFZyC3u+o=\\\",\\n\\t\\t\\t\\\"start_time\\\": \\\"2022-06-19T00:00:00Z\\\",\\n            \\\"end_time\\\": \\\"2022-06-19T01:01:00Z\\\"\\n\\t\\t}\\n\\t}\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"room_id\",\"app_name\",\"app_id\",\"start_time\",\"end_time\"],\"description\":\"The information on the digital signage played in a Zoom Room.\",\"properties\":{\"room_id\":{\"type\":\"string\",\"description\":\"The Zoom Room ID.\"},\"app_name\":{\"type\":\"string\",\"description\":\"The app name.\"},\"app_id\":{\"type\":\"string\",\"description\":\"The app ID.\"},\"start_time\":{\"description\":\"The playback start time of the digital signage.\",\"type\":\"string\",\"format\":\"datetime\",\"example\":\"2022-06-19T00:00:00Z\"},\"end_time\":{\"description\":\"The playback end time of the digital signage.\",\"type\":\"string\",\"format\":\"datetime\",\"example\":\"2022-06-19T00:00:00Z\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n\\t\\\"event\\\": \\\"zoomroom.digital_signage_app_ended\\\",\\n\\t\\\"event_ts\\\": 1626230691572,\\n\\t\\\"payload\\\": {\\n\\t\\t\\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n\\t\\t\\\"object\\\": {\\n\\t\\t\\t\\\"room_id\\\": \\\"abcD3ojfdbjfg\\\",\\n\\t\\t\\t\\\"app_name\\\": \\\"My Zoom Room\\\",\\n\\t\\t\\t\\\"app_id\\\": \\\"NiVvY1NWpE2nulNrhVjgU4jD0swziVrXBbaFZyC3u+o=\\\",\\n\\t\\t\\t\\\"start_time\\\": \\\"2022-06-19T00:00:00Z\\\",\\n            \\\"end_time\\\": \\\"2022-06-19T01:01:00Z\\\"\\n\\t\\t}\\n\\t}\\n}\"}}}}}}}},\"zoomroom.checked_in\":{\"post\":{\"tags\":[\"zoomroom\"],\"operationId\":\"zoomroom.checked_in\",\"requestBody\":{\"description\":\"# zoomroom.checked_in\\n\\nThe **Zoom Room Checked In** event is triggered when a user [checks in](https://support.zoom.us/hc/en-us/articles/360001147163-Using-the-Zoom-Rooms-check-in-feature) to confirm that they will be using the Zoom Room for a certain period of time.\\n\\nUsers can check in manually via the Zoom Room controller or the scheduling display, or they can check in programmatically using the [**Check-in or check-out of a Zoom Room**](/api-reference/zoom-api/methods#operation/checkInRooms) API. Users can also check in to the Zoom Room by starting a scheduled meeting, using the whiteboard feature or making a phone call from the Zoom Room.\\n\\n**Note** Only scheduled meetings will trigger a webhook.\\n\\n\\n## Prerequisites\\n\\n* The administrator or account owner must enable the [**Room Check-In and Check-Out**](https://support.zoom.us/hc/en-us/articles/360001147163-Using-the-Zoom-Rooms-check-in-feature) feature.\\n* The Zoom Room version must be 4.1.27694.0702 or higher.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Zoom Room Checked In** subscription enabled under the **Zoom Room** event.\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\\n\\n**Event type**: `zoomroom.checked_in`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"id\",\"room_name\",\"calendar_name\",\"email\",\"event_id\",\"change_key\",\"resource_email\",\"calendar_id\",\"calendar_type\",\"api_type\"],\"description\":\"Information about the Zoom Room.\",\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Room's unique ID.\"},\"room_name\":{\"type\":\"string\",\"description\":\"The Zoom Room's name.\"},\"calendar_name\":{\"type\":\"string\",\"description\":\"The calendar's name.\"},\"email\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The Zoom Room's associated email address.\"},\"event_id\":{\"type\":\"string\",\"description\":\"The unique ID of the Zoom Room's associated calendar event:\\n\\n* For Microsoft Exchange calendar integration, use Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). The event_id value is the Id value in the ItemId element of the FindItem operation's response. \\n* For Google Calendar integration, use Google's [Events: list API](https://developers.google.com/calendar/api/v3/reference/events/list). The event_id value is the id field in the [events resource](https://developers.google.com/calendar/api/v3/reference/events#resource).\"},\"change_key\":{\"type\":\"string\",\"description\":\"The specific version of the calendar item. You can get this value using Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). Refer to the ChangeKey field of the ItemId element in the operation's response.\\n\\nThis field is only returned for Microsoft Exchange or Office 365 calendars.\"},\"resource_email\":{\"description\":\"The calendar's associated [resource mailbox](https://support.microsoft.com/en-us/help/10063/creating-and-managing-resource-mailboxes-in-office-365).\\n\\nThis field is only returned for Microsoft Exchange or Office 365 calendars.\",\"type\":\"string\"},\"calendar_id\":{\"type\":\"string\",\"description\":\"The calendar's unique ID.\"},\"calendar_type\":{\"type\":\"string\",\"description\":\"The calendar's type: \\n* 0 \u2014 Google Calendar. \\n* 1 \u2014 Office 365 Calendar. \\n* 2 \u2014 Microsoft Exchange Calendar.\",\"enum\":[\"0\",\"1\",\"2\"]},\"api_type\":{\"type\":\"string\",\"description\":\"The API type: \\n* 0 \u2014 [Exchange Web Services (EWS) API](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-managed-api-reference-for-exchange) \\n* 1 \u2014 [Microsoft Graph API](https://docs.microsoft.com/en-us/azure/active-directory/develop/microsoft-graph-intro) \\n* An empty string \u2014 The API returns this value when the calendar_type value is 0 (Google Calendar).\",\"enum\":[\"0\",\"1\",\"\"]}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.checked_in\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n        \\\"id\\\": \\\"abcD3ojfdbjfg\\\",\\n        \\\"room_name\\\": \\\"My Zoom Room\\\",\\n        \\\"calendar_name\\\": \\\"mycalendar@example.com\\\",\\n        \\\"email\\\": \\\"jchill@example.com\\\",\\n        \\\"event_id\\\": \\\"AbbbbbGYxLTc3OTVkMzFmZDc0MwBGAAAAAAD48FI58voYSqDgJePOSZblBwBQ/N0JvB/FRqv5UT2rFfkVAAAAAAENAABQ/N0JvB/FRqv5UT2rFfkVAAE2YC8DAAA=\\\",\\n        \\\"change_key\\\": \\\"DwAAABYAAABQ/N0JvB/FRqv5UT2rFfkVAAE2XqVw\\\",\\n        \\\"resource_email\\\": \\\"it@example.com\\\",\\n        \\\"calendar_id\\\": \\\"mycalendar@example.com\\\",\\n        \\\"calendar_type\\\": \\\"2\\\",\\n        \\\"api_type\\\": \\\"0\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"id\",\"room_name\",\"calendar_name\",\"email\",\"event_id\",\"change_key\",\"resource_email\",\"calendar_id\",\"calendar_type\",\"api_type\"],\"description\":\"Information about the Zoom Room.\",\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Room's unique ID.\"},\"room_name\":{\"type\":\"string\",\"description\":\"The Zoom Room's name.\"},\"calendar_name\":{\"type\":\"string\",\"description\":\"The calendar's name.\"},\"email\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The Zoom Room's associated email address.\"},\"event_id\":{\"type\":\"string\",\"description\":\"The unique ID of the Zoom Room's associated calendar event:\\n\\n* For Microsoft Exchange calendar integration, use Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). The event_id value is the Id value in the ItemId element of the FindItem operation's response. \\n* For Google Calendar integration, use Google's [Events: list API](https://developers.google.com/calendar/api/v3/reference/events/list). The event_id value is the id field in the [events resource](https://developers.google.com/calendar/api/v3/reference/events#resource).\"},\"change_key\":{\"type\":\"string\",\"description\":\"The specific version of the calendar item. You can get this value using Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). Refer to the ChangeKey field of the ItemId element in the operation's response.\\n\\nThis field is only returned for Microsoft Exchange or Office 365 calendars.\"},\"resource_email\":{\"description\":\"The calendar's associated [resource mailbox](https://support.microsoft.com/en-us/help/10063/creating-and-managing-resource-mailboxes-in-office-365).\\n\\nThis field is only returned for Microsoft Exchange or Office 365 calendars.\",\"type\":\"string\"},\"calendar_id\":{\"type\":\"string\",\"description\":\"The calendar's unique ID.\"},\"calendar_type\":{\"type\":\"string\",\"description\":\"The calendar's type: \\n* 0 \u2014 Google Calendar. \\n* 1 \u2014 Office 365 Calendar. \\n* 2 \u2014 Microsoft Exchange Calendar.\",\"enum\":[\"0\",\"1\",\"2\"]},\"api_type\":{\"type\":\"string\",\"description\":\"The API type: \\n* 0 \u2014 [Exchange Web Services (EWS) API](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-managed-api-reference-for-exchange) \\n* 1 \u2014 [Microsoft Graph API](https://docs.microsoft.com/en-us/azure/active-directory/develop/microsoft-graph-intro) \\n* An empty string \u2014 The API returns this value when the calendar_type value is 0 (Google Calendar).\",\"enum\":[\"0\",\"1\",\"\"]}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.checked_in\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n        \\\"id\\\": \\\"abcD3ojfdbjfg\\\",\\n        \\\"room_name\\\": \\\"My Zoom Room\\\",\\n        \\\"calendar_name\\\": \\\"mycalendar@example.com\\\",\\n        \\\"email\\\": \\\"jchill@example.com\\\",\\n        \\\"event_id\\\": \\\"AbbbbbGYxLTc3OTVkMzFmZDc0MwBGAAAAAAD48FI58voYSqDgJePOSZblBwBQ/N0JvB/FRqv5UT2rFfkVAAAAAAENAABQ/N0JvB/FRqv5UT2rFfkVAAE2YC8DAAA=\\\",\\n        \\\"change_key\\\": \\\"DwAAABYAAABQ/N0JvB/FRqv5UT2rFfkVAAE2XqVw\\\",\\n        \\\"resource_email\\\": \\\"it@example.com\\\",\\n        \\\"calendar_id\\\": \\\"mycalendar@example.com\\\",\\n        \\\"calendar_type\\\": \\\"2\\\",\\n        \\\"api_type\\\": \\\"0\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"workspace.reservation_checked_in\":{\"post\":{\"tags\":[\"workspace\"],\"operationId\":\"workspace.reservation_checked_in\",\"requestBody\":{\"description\":\"# workspace.reservation_checked_in\\n\\nThe **Workspace Reservation Checked In** event is triggered when a user checks in their [Workspace reservation](https://support.zoom.us/hc/en-us/articles/4417951085965-Using-Workspace-Reservation-Beta-for-Zoom-Rooms) to confirm that they will be using a Zoom Room or Zoom device-enabled desk.\\n\\n## Prerequisites\\n\\n* The **Workspace Reservation** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\\n* The Zoom mobile app. This is required for QR code scanning:\\n  * Android version 5.9.0.3502 or higher\\n  * iOS version 5.9.0 or higher\\n* [Zoom Rooms for Touch devices](https://support.zoom.us/hc/en-us/articles/115004773103), version 5.9.0 or higher (*Optional*).\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Workspace Reservation Checked In** subscription enabled under the **Workspace Reservation** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:reservation`,`workspace:read:reservation:admin`\\n\\n**Event type**: `workspace.reservation_checked_in`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that checked in the reservation.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that checked in the reservation.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the workspace reservation.\",\"required\":[\"id\",\"workspace_id\",\"end_time\",\"start_time\",\"topic\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The reservation ID.\"},\"workspace_id\":{\"type\":\"string\",\"description\":\"The workspace ID.\"},\"end_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's end time.\"},\"start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's start time.\"},\"topic\":{\"type\":\"string\",\"description\":\"The reservation's description.\"},\"reserve_for\":{\"type\":\"string\",\"description\":\"The ID of the user for whom to create a workspace reservation.\",\"example\":\"KDcuGIm1QgePTO8WbOqwIQ\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n\\t\\\"event\\\": \\\"workspace.reservation_checked_in\\\",\\n\\t\\\"event_ts\\\": \\\"290112312321\\\",\\n\\t\\\"payload\\\": {\\n\\t\\t\\\"account_id\\\": \\\"G3NQs8u1Qni4Pc-qBPgsxg\\\",\\n\\t\\t\\\"operator\\\" : \\\"admin@example.com\\\",\\n\\t    \\\"operator_id\\\" : \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n\\t\\t\\\"object\\\": {\\n\\t\\t\\t\\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n\\t\\t\\t\\\"workspace_id\\\": \\\"qwNQb8u1Qbi3Pc-qBPgshy\\\",\\n\\t\\t\\t\\\"topic\\\": \\\"Reserve Zoom Room-01\\\",\\n\\t\\t\\t\\\"start_time\\\": \\\"2022-01-19T02:10:00Z\\\",\\n\\t\\t\\t\\\"end_time\\\": \\\"2022-01-19T03:55:00Z\\\",\\n\\t\\t\\t\\\"reserve_for\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\"\\n\\t\\t}\\n\\t}\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that checked in the reservation.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that checked in the reservation.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the workspace reservation.\",\"required\":[\"id\",\"workspace_id\",\"end_time\",\"start_time\",\"topic\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The reservation ID.\"},\"workspace_id\":{\"type\":\"string\",\"description\":\"The workspace ID.\"},\"end_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's end time.\"},\"start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's start time.\"},\"topic\":{\"type\":\"string\",\"description\":\"The reservation's description.\"},\"reserve_for\":{\"type\":\"string\",\"description\":\"The ID of the user for whom to create a workspace reservation.\",\"example\":\"KDcuGIm1QgePTO8WbOqwIQ\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n\\t\\\"event\\\": \\\"workspace.reservation_checked_in\\\",\\n\\t\\\"event_ts\\\": \\\"290112312321\\\",\\n\\t\\\"payload\\\": {\\n\\t\\t\\\"account_id\\\": \\\"G3NQs8u1Qni4Pc-qBPgsxg\\\",\\n\\t\\t\\\"operator\\\" : \\\"admin@example.com\\\",\\n\\t    \\\"operator_id\\\" : \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n\\t\\t\\\"object\\\": {\\n\\t\\t\\t\\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n\\t\\t\\t\\\"workspace_id\\\": \\\"qwNQb8u1Qbi3Pc-qBPgshy\\\",\\n\\t\\t\\t\\\"topic\\\": \\\"Reserve Zoom Room-01\\\",\\n\\t\\t\\t\\\"start_time\\\": \\\"2022-01-19T02:10:00Z\\\",\\n\\t\\t\\t\\\"end_time\\\": \\\"2022-01-19T03:55:00Z\\\",\\n\\t\\t\\t\\\"reserve_for\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\"\\n\\t\\t}\\n\\t}\\n}\"}}}}}}}},\"zoomroom.updated\":{\"post\":{\"tags\":[\"zoomroom\"],\"operationId\":\"zoomroom.updated\",\"requestBody\":{\"description\":\"# zoomroom.updated\\n\\nThe **Zoom Room Updated** event is triggered when a user updates a [**Zoom Room**](https://support.zoom.us/hc/en-us/categories/200108436)\\n\\n## Prerequisites\\n\\n* The **Zoom Room** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Zoom Room Updated** subscription enabled under the **Zoom Room** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\\n\\n**Event type**: `zoomroom.updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"id\",\"room_name\"],\"description\":\"Information about the Zoom Room.\",\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Room's unique ID.\"},\"room_name\":{\"type\":\"string\",\"description\":\"The Zoom Room's name.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"G3NQs8u1Qni4Pc-qBPgsxg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"2muAb65FTZCGjj6ginHIbQ\\\",\\n      \\\"room_name\\\": \\\"My Zoom Room\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"id\",\"room_name\"],\"description\":\"Information about the Zoom Room.\",\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Room's unique ID.\"},\"room_name\":{\"type\":\"string\",\"description\":\"The Zoom Room's name.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"G3NQs8u1Qni4Pc-qBPgsxg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"2muAb65FTZCGjj6ginHIbQ\\\",\\n      \\\"room_name\\\": \\\"My Zoom Room\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoomroom.location_updated\":{\"post\":{\"tags\":[\"zoomroom\"],\"operationId\":\"zoomroom.location_updated\",\"requestBody\":{\"description\":\"# zoomroom.location_updated\\nThe **Zoom Rooms Location has been updated** event is triggered every time you update a Zoom Room Location in your account. Admins and account owners can update a Zoom Room Location to an account. Update the location using the [Update a Zoom Rooms Location API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/updateZRLocationProfile) or by updating the Zoom Room Location using the Zoom Web Portal itself.\\n\\n## Prerequisites\\n\\n* Enable The **Zoom Rooms** feature by your organization through a Zoom Rooms license or Workspace Reservation feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or **Workspace Reservation** license.\\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Zoom Rooms Location Updated** subscription enabled under the **Zoom Rooms** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\\n\\n**Event type**: `zoomroom.location_updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who updated the Zoom Rooms Location.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that updated the Zoom Rooms Location.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that updated the Zoom Rooms Location.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the updated Location.\",\"required\":[\"id\",\"name\",\"parent_location_id\",\"type\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Rooms location's ID.\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the updated Zoom Rooms location.\"},\"parent_location_id\":{\"type\":\"string\",\"description\":\"This location's parent Zoom Rooms location ID.\"},\"type\":{\"type\":\"enum, country\u2503states\u2503city\u2503campus\u2503building\u2503floor\",\"description\":\"The type of location.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.location_updated\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"name\\\": \\\"My new Zoom Room Location\\\",\\n      \\\"parent_location_id\\\": \\\"aldjflajsd-fasj\\\",\\n      \\\"type\\\": \\\"floor\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who updated the Zoom Rooms Location.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that updated the Zoom Rooms Location.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that updated the Zoom Rooms Location.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the updated Location.\",\"required\":[\"id\",\"name\",\"parent_location_id\",\"type\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Rooms location's ID.\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the updated Zoom Rooms location.\"},\"parent_location_id\":{\"type\":\"string\",\"description\":\"This location's parent Zoom Rooms location ID.\"},\"type\":{\"type\":\"enum, country\u2503states\u2503city\u2503campus\u2503building\u2503floor\",\"description\":\"The type of location.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.location_updated\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"name\\\": \\\"My new Zoom Room Location\\\",\\n      \\\"parent_location_id\\\": \\\"aldjflajsd-fasj\\\",\\n      \\\"type\\\": \\\"floor\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"workspace.pre_check_in_questionnaire_response_created\":{\"post\":{\"tags\":[\"workspace\"],\"operationId\":\"workspace.pre_check_in_questionnaire_response_created\",\"requestBody\":{\"description\":\"# workspace.pre_check_in_questionnaire_response_created\\nThe **Workspace pre-check-in questionnaire created** event is triggered every time a pre-check-in questionnaire is submitted. Users can submit the questionnaire by clicking the workspace questionnarie link in the email.\\n\\n## Prerequisites\\n\\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Pre-check-in questionnaire created** subscription enabled under the **Workspace** event.\\n\\n**Event type**: `workspace.pre_check_in_questionnaire_response_created `\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that completed the questionnaire.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that completed the questionnaire.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the questionnaire when creating a reservation for a workspace.\",\"required\":[\"start_time\",\"event_id\",\"user_name\",\"desk_id\",\"workspace_name\",\"location_id\",\"questionnaire_response\"],\"properties\":{\"start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The start time of the workspace reservation.\"},\"event_id\":{\"type\":\"string\",\"description\":\"The event ID.\"},\"user_name\":{\"type\":\"string\",\"description\":\"The user name.\"},\"desk_id\":{\"type\":\"string\",\"description\":\"The desk ID.\"},\"workspace_name\":{\"type\":\"string\",\"description\":\"The name of the workspace.\"},\"location_id\":{\"type\":\"string\",\"description\":\"The location ID.\"},\"questionnaire_response\":{\"type\":\"array\",\"description\":\"List of questionnaires.\",\"items\":{\"type\":\"object\",\"required\":[\"id\",\"title\",\"subject\",\"list\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The view ID.\"},\"title\":{\"type\":\"string\",\"description\":\"The title of the questionnaire.\"},\"subject\":{\"type\":\"string\",\"description\":\"The subject of the questionnaire.\"},\"list\":{\"type\":\"array\",\"description\":\"List of questions in the questionnaire.\",\"items\":{\"type\":\"object\",\"required\":[\"question_id\",\"title\",\"type\",\"options\",\"reply_id\"],\"properties\":{\"question_id\":{\"type\":\"string\",\"description\":\"The question ID.\"},\"title\":{\"type\":\"string\",\"description\":\"The title of the question.\"},\"type\":{\"type\":\"string\",\"description\":\"The type of the question (single-choice, multiple-choice, open-text).\"},\"reply_id\":{\"type\":\"string\",\"description\":\"The reply ID.\"},\"options\":{\"type\":\"array\",\"description\":\"List of options for multiple-choice questions.\",\"items\":{\"type\":\"object\",\"required\":[\"option_id\",\"text\"],\"properties\":{\"option_id\":{\"type\":\"string\",\"description\":\"The option ID.\"},\"text\":{\"type\":\"string\",\"description\":\"The text of the option.\"}}}}}}}}}}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"workspace.pre_check_in_questionnaire_response_created\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"VQYZ9ytmTbaXK9Zr2LvIrg\\\",\\n    \\\"operator\\\": \\\"yuki-devint@zoom.us\\\",\\n    \\\"operator_id\\\": \\\"rZXNXVlVSTaSZDTJvLcBew\\\",\\n    \\\"object\\\": {\\n      \\\"start_time\\\": \\\"2024-02-14T03:50:00Z\\\",\\n      \\\"event_id\\\": \\\"dxpwbc2fi8tzz7d012rwx9u8c0\\\",\\n      \\\"user_name\\\": \\\"Simash pokh\\\",\\n      \\\"desk_id\\\": \\\"SJjU1CcGSs2L7odn6PanQw\\\",\\n      \\\"workspace_name\\\": \\\"test change date - Desk1\\\",\\n      \\\"location_id\\\": \\\"wnFFoWRwTLeww81UuAeW4g\\\",\\n      \\\"questionnaire_respnse\\\": [\\n        {\\n          \\\"id\\\": \\\"Fl75C43YTmS_IpMzYLipJw\\\",\\n          \\\"title\\\": \\\"Simash Reservation Questionnaire\\\",\\n          \\\"list\\\": [\\n            {\\n              \\\"question_id\\\": \\\"0ds1NFeDQ1WfWlgVK-7OMQ\\\",\\n              \\\"title\\\": \\\"do you like AI ?\\\",\\n              \\\"type\\\": \\\"single\\\",\\n              \\\"options\\\": [\\n                {\\n                  \\\"option_id\\\": \\\"UClQqM1qT_q-bZQ0Xti7Vw\\\",\\n                  \\\"text\\\": \\\"Yes\\\"\\n                },\\n                {\\n                  \\\"option_id\\\": \\\"1S1hZzFGTwW9Llxdr2vzyw\\\",\\n                  \\\"text\\\": \\\"No\\\"\\n                }\\n              ],\\n              \\\"reply_id\\\": \\\"UClQqM1qT_q-bZQ0Xti7Vw\\\"\\n            },\\n            {\\n              \\\"question_id\\\": \\\"2XMaJFwNSr-BJpTfo6rRAQ\\\",\\n              \\\"title\\\": \\\"what is your fav food ? \\\",\\n              \\\"type\\\": \\\"multiple\\\",\\n              \\\"options\\\": [\\n                {\\n                  \\\"option_id\\\": \\\"iPkq2UDOSLOLHNq_Ac1WsQ\\\",\\n                  \\\"text\\\": \\\"momo\\\"\\n                },\\n                {\\n                  \\\"option_id\\\": \\\"dpv4EZ7QQUqRhBmYj4_b7w\\\",\\n                  \\\"text\\\": \\\"sea food\\\"\\n                },\\n                {\\n                  \\\"option_id\\\": \\\"__DjlrV7TeyCSZVmCWAnng\\\",\\n                  \\\"text\\\": \\\"rice\\\"\\n                }\\n              ],\\n              \\\"reply_id\\\": \\\"iPkq2UDOSLOLHNq_Ac1WsQ,dpv4EZ7QQUqRhBmYj4_b7w\\\"\\n            }\\n          ],\\n          \\\"subject\\\": \\\"reservation\\\"\\n        }\\n      ]\\n    }\\n  },\\n  \\\"event_ts\\\": 1707848572310\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that completed the questionnaire.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that completed the questionnaire.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the questionnaire when creating a reservation for a workspace.\",\"required\":[\"start_time\",\"event_id\",\"user_name\",\"desk_id\",\"workspace_name\",\"location_id\",\"questionnaire_response\"],\"properties\":{\"start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The start time of the workspace reservation.\"},\"event_id\":{\"type\":\"string\",\"description\":\"The event ID.\"},\"user_name\":{\"type\":\"string\",\"description\":\"The user name.\"},\"desk_id\":{\"type\":\"string\",\"description\":\"The desk ID.\"},\"workspace_name\":{\"type\":\"string\",\"description\":\"The name of the workspace.\"},\"location_id\":{\"type\":\"string\",\"description\":\"The location ID.\"},\"questionnaire_response\":{\"type\":\"array\",\"description\":\"List of questionnaires.\",\"items\":{\"type\":\"object\",\"required\":[\"id\",\"title\",\"subject\",\"list\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The view ID.\"},\"title\":{\"type\":\"string\",\"description\":\"The title of the questionnaire.\"},\"subject\":{\"type\":\"string\",\"description\":\"The subject of the questionnaire.\"},\"list\":{\"type\":\"array\",\"description\":\"List of questions in the questionnaire.\",\"items\":{\"type\":\"object\",\"required\":[\"question_id\",\"title\",\"type\",\"options\",\"reply_id\"],\"properties\":{\"question_id\":{\"type\":\"string\",\"description\":\"The question ID.\"},\"title\":{\"type\":\"string\",\"description\":\"The title of the question.\"},\"type\":{\"type\":\"string\",\"description\":\"The type of the question (single-choice, multiple-choice, open-text).\"},\"reply_id\":{\"type\":\"string\",\"description\":\"The reply ID.\"},\"options\":{\"type\":\"array\",\"description\":\"List of options for multiple-choice questions.\",\"items\":{\"type\":\"object\",\"required\":[\"option_id\",\"text\"],\"properties\":{\"option_id\":{\"type\":\"string\",\"description\":\"The option ID.\"},\"text\":{\"type\":\"string\",\"description\":\"The text of the option.\"}}}}}}}}}}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"workspace.pre_check_in_questionnaire_response_created\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"VQYZ9ytmTbaXK9Zr2LvIrg\\\",\\n    \\\"operator\\\": \\\"yuki-devint@zoom.us\\\",\\n    \\\"operator_id\\\": \\\"rZXNXVlVSTaSZDTJvLcBew\\\",\\n    \\\"object\\\": {\\n      \\\"start_time\\\": \\\"2024-02-14T03:50:00Z\\\",\\n      \\\"event_id\\\": \\\"dxpwbc2fi8tzz7d012rwx9u8c0\\\",\\n      \\\"user_name\\\": \\\"Simash pokh\\\",\\n      \\\"desk_id\\\": \\\"SJjU1CcGSs2L7odn6PanQw\\\",\\n      \\\"workspace_name\\\": \\\"test change date - Desk1\\\",\\n      \\\"location_id\\\": \\\"wnFFoWRwTLeww81UuAeW4g\\\",\\n      \\\"questionnaire_respnse\\\": [\\n        {\\n          \\\"id\\\": \\\"Fl75C43YTmS_IpMzYLipJw\\\",\\n          \\\"title\\\": \\\"Simash Reservation Questionnaire\\\",\\n          \\\"list\\\": [\\n            {\\n              \\\"question_id\\\": \\\"0ds1NFeDQ1WfWlgVK-7OMQ\\\",\\n              \\\"title\\\": \\\"do you like AI ?\\\",\\n              \\\"type\\\": \\\"single\\\",\\n              \\\"options\\\": [\\n                {\\n                  \\\"option_id\\\": \\\"UClQqM1qT_q-bZQ0Xti7Vw\\\",\\n                  \\\"text\\\": \\\"Yes\\\"\\n                },\\n                {\\n                  \\\"option_id\\\": \\\"1S1hZzFGTwW9Llxdr2vzyw\\\",\\n                  \\\"text\\\": \\\"No\\\"\\n                }\\n              ],\\n              \\\"reply_id\\\": \\\"UClQqM1qT_q-bZQ0Xti7Vw\\\"\\n            },\\n            {\\n              \\\"question_id\\\": \\\"2XMaJFwNSr-BJpTfo6rRAQ\\\",\\n              \\\"title\\\": \\\"what is your fav food ? \\\",\\n              \\\"type\\\": \\\"multiple\\\",\\n              \\\"options\\\": [\\n                {\\n                  \\\"option_id\\\": \\\"iPkq2UDOSLOLHNq_Ac1WsQ\\\",\\n                  \\\"text\\\": \\\"momo\\\"\\n                },\\n                {\\n                  \\\"option_id\\\": \\\"dpv4EZ7QQUqRhBmYj4_b7w\\\",\\n                  \\\"text\\\": \\\"sea food\\\"\\n                },\\n                {\\n                  \\\"option_id\\\": \\\"__DjlrV7TeyCSZVmCWAnng\\\",\\n                  \\\"text\\\": \\\"rice\\\"\\n                }\\n              ],\\n              \\\"reply_id\\\": \\\"iPkq2UDOSLOLHNq_Ac1WsQ,dpv4EZ7QQUqRhBmYj4_b7w\\\"\\n            }\\n          ],\\n          \\\"subject\\\": \\\"reservation\\\"\\n        }\\n      ]\\n    }\\n  },\\n  \\\"event_ts\\\": 1707848572310\\n}\"}}}}}}}},\"workspace.reservation_checked_out\":{\"post\":{\"tags\":[\"workspace\"],\"operationId\":\"workspace.reservation_checked_out\",\"requestBody\":{\"description\":\"# workspace.reservation_checked_out\\n\\nThe **Workspace Reservation Checked Out** event is triggered when a user checks out of their [Workspace reservation](https://support.zoom.us/hc/en-us/articles/4417951085965-Using-Workspace-Reservation-Beta-for-Zoom-Rooms) to confirm that they are releasing the Zoom Room or Zoom device-enabled desk.\\n\\n## Prerequisites\\n\\n* The **Workspace Reservation** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\\n* The Zoom mobile app. This is required for QR code scanning:\\n  * Android version 5.9.0.3502 or higher\\n  * iOS version 5.9.0 or higher\\n* [Zoom Rooms for Touch devices](https://support.zoom.us/hc/en-us/articles/115004773103), version 5.9.0 or higher (*Optional*).\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Workspace Reservation Checked Out** subscription enabled under the **Workspace Reservation** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:reservation`,`workspace:read:reservation:admin`\\n\\n**Event type**: `workspace.reservation_checked_out`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that checked out the reservation.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that checked out the reservation.\"},\"object\":{\"description\":\"Information about the workspace reservation.\",\"type\":\"object\",\"required\":[\"id\",\"workspace_id\",\"end_time\",\"start_time\",\"topic\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The reservation ID.\"},\"workspace_id\":{\"type\":\"string\",\"description\":\"The workspace ID.\"},\"end_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's end time.\"},\"start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's start time.\"},\"topic\":{\"type\":\"string\",\"description\":\"The reservation's description.\"},\"reserve_for\":{\"type\":\"string\",\"description\":\"The ID of the user for whom to create a workspace reservation.\",\"example\":\"KDcuGIm1QgePTO8WbOqwIQ\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n\\t\\\"event\\\": \\\"workspace.reservation_checked_out\\\",\\n\\t\\\"event_ts\\\": \\\"290112312321\\\",\\n\\t\\\"payload\\\": {\\n\\t\\t\\\"account_id\\\": \\\"G3NQs8u1Qni4Pc-qBPgsxg\\\",\\n\\t\\t\\\"operator\\\" : \\\"admin@example.com\\\",\\n\\t    \\\"operator_id\\\" : \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n\\t\\t\\\"object\\\": {\\n\\t\\t\\t\\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n\\t\\t\\t\\\"workspace_id\\\": \\\"qwNQb8u1Qbi3Pc-qBPgshy\\\",\\n\\t\\t\\t\\\"topic\\\": \\\"Reserve Zoom Room-01\\\",\\n\\t\\t\\t\\\"start_time\\\": \\\"2022-01-19T02:10:00Z\\\",\\n\\t\\t\\t\\\"end_time\\\": \\\"2022-01-19T03:55:00Z\\\",\\n\\t\\t\\t\\\"reserve_for\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\"\\n\\t\\t}\\n\\t}\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that checked out the reservation.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that checked out the reservation.\"},\"object\":{\"description\":\"Information about the workspace reservation.\",\"type\":\"object\",\"required\":[\"id\",\"workspace_id\",\"end_time\",\"start_time\",\"topic\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The reservation ID.\"},\"workspace_id\":{\"type\":\"string\",\"description\":\"The workspace ID.\"},\"end_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's end time.\"},\"start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's start time.\"},\"topic\":{\"type\":\"string\",\"description\":\"The reservation's description.\"},\"reserve_for\":{\"type\":\"string\",\"description\":\"The ID of the user for whom to create a workspace reservation.\",\"example\":\"KDcuGIm1QgePTO8WbOqwIQ\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n\\t\\\"event\\\": \\\"workspace.reservation_checked_out\\\",\\n\\t\\\"event_ts\\\": \\\"290112312321\\\",\\n\\t\\\"payload\\\": {\\n\\t\\t\\\"account_id\\\": \\\"G3NQs8u1Qni4Pc-qBPgsxg\\\",\\n\\t\\t\\\"operator\\\" : \\\"admin@example.com\\\",\\n\\t    \\\"operator_id\\\" : \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n\\t\\t\\\"object\\\": {\\n\\t\\t\\t\\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n\\t\\t\\t\\\"workspace_id\\\": \\\"qwNQb8u1Qbi3Pc-qBPgshy\\\",\\n\\t\\t\\t\\\"topic\\\": \\\"Reserve Zoom Room-01\\\",\\n\\t\\t\\t\\\"start_time\\\": \\\"2022-01-19T02:10:00Z\\\",\\n\\t\\t\\t\\\"end_time\\\": \\\"2022-01-19T03:55:00Z\\\",\\n\\t\\t\\t\\\"reserve_for\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\"\\n\\t\\t}\\n\\t}\\n}\"}}}}}}}},\"visitor_management.checked_in\":{\"post\":{\"tags\":[\"visitor_management\"],\"operationId\":\"visitor_management.checked_in\",\"requestBody\":{\"description\":\"# visitor_management.checked_in\\nThe **Visitor Management Checked In** event is triggered when a [visitor checks in](https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0073629#h_63082000168761703023137137).\\n\\n## Prerequisites\\n\\n* The [**Visitor Management**](https://support.zoom.com/hc/en?id=zm_kb&sysparm_article=KB0057564) feature enabled by your organization.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Visitor Management Checked In** subscription enabled under the **Visitor Management** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:visitor_checked_in:admin`\\n\\n**Event type**: `visitor_management.checked_in`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The host user account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the host.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the host.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the checked in invitation.\",\"required\":[\"visitor_email\",\"visitor_name\",\"start_time\",\"end_time\",\"invite_location_id\",\"host_id\",\"host_name\",\"host_email\"],\"properties\":{\"visitor_email\":{\"type\":\"string\",\"description\":\"The email address of the invited visitor.\"},\"visitor_name\":{\"type\":\"string\",\"description\":\"The name of the invited visitor.\"},\"start_time\":{\"type\":\"long\",\"description\":\"The epoch timestamp representing the start time of the invitation.\"},\"end_time\":{\"type\":\"long\",\"description\":\"The epoch timestamp representing end time of the invitation.\"},\"invite_location_id\":{\"type\":\"string\",\"description\":\"The location ID of the location to which the visitor is invited to.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The user ID of the invitation host.\"},\"host_name\":{\"type\":\"string\",\"description\":\"The name of the invitation host.\"},\"host_email\":{\"type\":\"string\",\"description\":\"The email address of the invitation host.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"visitor_management.checked_in\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"visitor_email\\\": \\\"visitor@example.com\\\",\\n      \\\"visitor_name\\\": \\\"visitor\\\",\\n      \\\"start_time\\\": 1710153000000,\\n      \\\"end_time\\\": 1710172799999,\\n      \\\"invite_location_id\\\": \\\"wnFFoWRwTLeww81UuAeW4g\\\",\\n      \\\"host_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n      \\\"host_name\\\": \\\"admin\\\",\\n      \\\"host_email\\\": \\\"admin@example.com\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The host user account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the host.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the host.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the checked in invitation.\",\"required\":[\"visitor_email\",\"visitor_name\",\"start_time\",\"end_time\",\"invite_location_id\",\"host_id\",\"host_name\",\"host_email\"],\"properties\":{\"visitor_email\":{\"type\":\"string\",\"description\":\"The email address of the invited visitor.\"},\"visitor_name\":{\"type\":\"string\",\"description\":\"The name of the invited visitor.\"},\"start_time\":{\"type\":\"long\",\"description\":\"The epoch timestamp representing the start time of the invitation.\"},\"end_time\":{\"type\":\"long\",\"description\":\"The epoch timestamp representing end time of the invitation.\"},\"invite_location_id\":{\"type\":\"string\",\"description\":\"The location ID of the location to which the visitor is invited to.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The user ID of the invitation host.\"},\"host_name\":{\"type\":\"string\",\"description\":\"The name of the invitation host.\"},\"host_email\":{\"type\":\"string\",\"description\":\"The email address of the invitation host.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"visitor_management.checked_in\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"visitor_email\\\": \\\"visitor@example.com\\\",\\n      \\\"visitor_name\\\": \\\"visitor\\\",\\n      \\\"start_time\\\": 1710153000000,\\n      \\\"end_time\\\": 1710172799999,\\n      \\\"invite_location_id\\\": \\\"wnFFoWRwTLeww81UuAeW4g\\\",\\n      \\\"host_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n      \\\"host_name\\\": \\\"admin\\\",\\n      \\\"host_email\\\": \\\"admin@example.com\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"workspace.reservation_deleted\":{\"post\":{\"tags\":[\"workspace\"],\"operationId\":\"workspace.reservation_deleted\",\"requestBody\":{\"description\":\"# workspace.reservation_deleted\\n\\nThe **Workspace Reservation Deleted** event is triggered when a user deletes their [Workspace reservation](https://support.zoom.us/hc/en-us/articles/4417951085965-Using-Workspace-Reservation-Beta-for-Zoom-Rooms).\\n\\n## Prerequisites\\n\\n* The **Workspace Reservation** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\\n* The Zoom mobile app. This is required for QR code scanning:\\n  * Android version 5.9.0.3502 or higher\\n  * iOS version 5.9.0 or higher\\n* [Zoom Rooms for Touch devices](https://support.zoom.us/hc/en-us/articles/115004773103), version 5.9.0 or higher (*Optional*).\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Workspace Reservation Deleted** subscription enabled under the **Workspace Reservation** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:reservation`,`workspace:read:reservation:admin`\\n\\n**Event type**: `workspace.reservation_deleted`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that deleted the reservation.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that deleted the reservation.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the workspace reservation.\",\"required\":[\"id\",\"workspace_id\",\"end_time\",\"start_time\",\"topic\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The reservation ID.\"},\"workspace_id\":{\"type\":\"string\",\"description\":\"The workspace ID.\"},\"end_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's end time.\"},\"start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's start time.\"},\"topic\":{\"type\":\"string\",\"description\":\"The reservation's description.\"},\"reserve_for\":{\"type\":\"string\",\"description\":\"The ID of the user for whom to create a workspace reservation.\",\"example\":\"KDcuGIm1QgePTO8WbOqwIQ\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n\\t\\\"event\\\": \\\"workspace.reservation_deleted\\\",\\n\\t\\\"event_ts\\\": \\\"290112312321\\\",\\n\\t\\\"payload\\\": {\\n\\t\\t\\\"account_id\\\": \\\"G3NQs8u1Qni4Pc-qBPgsxg\\\",\\n\\t\\t\\\"operator\\\" : \\\"admin@example.com\\\",\\n\\t    \\\"operator_id\\\" : \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n\\t\\t\\\"object\\\": {\\n\\t\\t\\t\\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n\\t\\t\\t\\\"workspace_id\\\": \\\"qwNQb8u1Qbi3Pc-qBPgshy\\\",\\n\\t\\t\\t\\\"topic\\\": \\\"Reserve Zoom Room-01\\\",\\n\\t\\t\\t\\\"start_time\\\": \\\"2022-01-19T02:10:00Z\\\",\\n\\t\\t\\t\\\"end_time\\\": \\\"2022-01-19T03:55:00Z\\\",\\n\\t\\t\\t\\\"reserve_for\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\"\\n\\t\\t}\\n\\t}\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that deleted the reservation.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that deleted the reservation.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the workspace reservation.\",\"required\":[\"id\",\"workspace_id\",\"end_time\",\"start_time\",\"topic\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The reservation ID.\"},\"workspace_id\":{\"type\":\"string\",\"description\":\"The workspace ID.\"},\"end_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's end time.\"},\"start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's start time.\"},\"topic\":{\"type\":\"string\",\"description\":\"The reservation's description.\"},\"reserve_for\":{\"type\":\"string\",\"description\":\"The ID of the user for whom to create a workspace reservation.\",\"example\":\"KDcuGIm1QgePTO8WbOqwIQ\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n\\t\\\"event\\\": \\\"workspace.reservation_deleted\\\",\\n\\t\\\"event_ts\\\": \\\"290112312321\\\",\\n\\t\\\"payload\\\": {\\n\\t\\t\\\"account_id\\\": \\\"G3NQs8u1Qni4Pc-qBPgsxg\\\",\\n\\t\\t\\\"operator\\\" : \\\"admin@example.com\\\",\\n\\t    \\\"operator_id\\\" : \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n\\t\\t\\\"object\\\": {\\n\\t\\t\\t\\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n\\t\\t\\t\\\"workspace_id\\\": \\\"qwNQb8u1Qbi3Pc-qBPgshy\\\",\\n\\t\\t\\t\\\"topic\\\": \\\"Reserve Zoom Room-01\\\",\\n\\t\\t\\t\\\"start_time\\\": \\\"2022-01-19T02:10:00Z\\\",\\n\\t\\t\\t\\\"end_time\\\": \\\"2022-01-19T03:55:00Z\\\",\\n\\t\\t\\t\\\"reserve_for\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\"\\n\\t\\t}\\n\\t}\\n}\"}}}}}}}},\"visitor_management.invitation_created\":{\"post\":{\"tags\":[\"visitor_management\"],\"operationId\":\"visitor_management.invitation_created\",\"requestBody\":{\"description\":\"# visitor_management.invitation_created\\n\\nThe **Visitor Management Invitation Created** event is triggered when an [invitation is created](https://support.zoom.com/hc/en?id=zm_kb&sysparm_article=KB0073629#mcetoc_1hi1ubsr1fb).\\n\\n## Prerequisites\\n\\n* The [**Visitor Management**](https://support.zoom.com/hc/en?id=zm_kb&sysparm_article=KB0057564) feature enabled by your organization.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Visitor Management Invitation Created** subscription enabled under the **Visitor Management** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:visitor_invitation:admin`\\n\\n**Event type**: `visitor_management.invitation_created`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The user account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that created the invitation.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that created the invitation.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the created invitation.\",\"required\":[\"invitation_id\",\"visitor_email\",\"visitor_name\",\"start_time\",\"end_time\",\"invite_location_id\",\"host_id\",\"host_name\",\"host_email\"],\"properties\":{\"invitation_id\":{\"type\":\"string\",\"description\":\"The invitation ID.\"},\"visitor_email\":{\"type\":\"string\",\"description\":\"The email address of the invited visitor.\"},\"visitor_name\":{\"type\":\"string\",\"description\":\"The name of the invited visitor.\"},\"start_time\":{\"type\":\"long\",\"description\":\"The epoch timestamp representing the start time of the invitation.\"},\"end_time\":{\"type\":\"long\",\"description\":\"The epoch timestamp representing end time of the invitation.\"},\"invite_location_id\":{\"type\":\"string\",\"description\":\"The location ID of the location to which the visitor is invited to.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The user ID of the invitation host.\"},\"host_name\":{\"type\":\"string\",\"description\":\"The name of the invitation host.\"},\"host_email\":{\"type\":\"string\",\"description\":\"The email address of the invitation host.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"visitor_management.invitation_created\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"invitation_id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"visitor_email\\\": \\\"visitor@example.com\\\",\\n      \\\"visitor_name\\\": \\\"visitor\\\",\\n      \\\"start_time\\\": 1710153000000,\\n      \\\"end_time\\\": 1710172799999,\\n      \\\"invite_location_id\\\": \\\"wnFFoWRwTLeww81UuAeW4g\\\",\\n      \\\"host_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n      \\\"host_name\\\": \\\"admin\\\",\\n      \\\"host_email\\\": \\\"admin@example.com\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The user account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that created the invitation.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that created the invitation.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the created invitation.\",\"required\":[\"invitation_id\",\"visitor_email\",\"visitor_name\",\"start_time\",\"end_time\",\"invite_location_id\",\"host_id\",\"host_name\",\"host_email\"],\"properties\":{\"invitation_id\":{\"type\":\"string\",\"description\":\"The invitation ID.\"},\"visitor_email\":{\"type\":\"string\",\"description\":\"The email address of the invited visitor.\"},\"visitor_name\":{\"type\":\"string\",\"description\":\"The name of the invited visitor.\"},\"start_time\":{\"type\":\"long\",\"description\":\"The epoch timestamp representing the start time of the invitation.\"},\"end_time\":{\"type\":\"long\",\"description\":\"The epoch timestamp representing end time of the invitation.\"},\"invite_location_id\":{\"type\":\"string\",\"description\":\"The location ID of the location to which the visitor is invited to.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The user ID of the invitation host.\"},\"host_name\":{\"type\":\"string\",\"description\":\"The name of the invitation host.\"},\"host_email\":{\"type\":\"string\",\"description\":\"The email address of the invitation host.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"visitor_management.invitation_created\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"invitation_id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"visitor_email\\\": \\\"visitor@example.com\\\",\\n      \\\"visitor_name\\\": \\\"visitor\\\",\\n      \\\"start_time\\\": 1710153000000,\\n      \\\"end_time\\\": 1710172799999,\\n      \\\"invite_location_id\\\": \\\"wnFFoWRwTLeww81UuAeW4g\\\",\\n      \\\"host_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n      \\\"host_name\\\": \\\"admin\\\",\\n      \\\"host_email\\\": \\\"admin@example.com\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoomroom.location_deleted\":{\"post\":{\"tags\":[\"zoomroom\"],\"operationId\":\"zoomroom.location_deleted\",\"requestBody\":{\"description\":\"# zoomroom.location_deleted\\nThe **Zoom Rooms location has been deleted** event is triggered every time a Zoom Room location is deleted in your account. Admins and account owners can delete Zoom Rooms locations in a Zoom account.  Deleting Zoom Rooms locations permanently removes them and their data from the Zoom system.  To delete a Zoom Rooms location, delete the Zoom Rooms location from the Zoom Web Portal itself.\\n\\n## Prerequisites\\n\\n* Have your organization enable the **Zoom Rooms** feature through a Zoom Rooms license, or Workspace Reservation feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or **Workspace Reservation** license.\\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Zoom Rooms Location Deleted** subscription enabled under the **Zoom Rooms** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\\n\\n**Event type**: `zoomroom.location_deleted`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who deleted the Zoom Rooms location.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that deleted the Zoom Rooms location.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that deleted the Zoom Rooms location.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the deleted location.\",\"required\":[\"id\",\"name\",\"parent_location_id\",\"type\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Rooms location's ID.\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the deleted Zoom Rooms location.\"},\"parent_location_id\":{\"type\":\"string\",\"description\":\"This location's parent Zoom Rooms location ID.\"},\"type\":{\"type\":\"enum, country\u2503states\u2503city\u2503campus\u2503building\u2503floor\",\"description\":\"The location type.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.location_deleted\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"name\\\": \\\"My Deleted Zoom Room Location\\\",\\n      \\\"parent_location_id\\\": \\\"aldjflajsd-fasj\\\",\\n      \\\"type\\\": \\\"floor\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who deleted the Zoom Rooms location.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that deleted the Zoom Rooms location.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that deleted the Zoom Rooms location.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the deleted location.\",\"required\":[\"id\",\"name\",\"parent_location_id\",\"type\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Rooms location's ID.\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the deleted Zoom Rooms location.\"},\"parent_location_id\":{\"type\":\"string\",\"description\":\"This location's parent Zoom Rooms location ID.\"},\"type\":{\"type\":\"enum, country\u2503states\u2503city\u2503campus\u2503building\u2503floor\",\"description\":\"The location type.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.location_deleted\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"name\\\": \\\"My Deleted Zoom Room Location\\\",\\n      \\\"parent_location_id\\\": \\\"aldjflajsd-fasj\\\",\\n      \\\"type\\\": \\\"floor\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"workspace.reservation_created\":{\"post\":{\"tags\":[\"workspace\"],\"operationId\":\"workspace.reservation_created\",\"requestBody\":{\"description\":\"# workspace.reservation_created\\n\\nThe **Workspace Reservation Created** event is triggered when a user [reserves](https://support.zoom.us/hc/en-us/articles/4417951085965-Using-Workspace-Reservation-Beta-for-Zoom-Rooms) a Zoom Room or Zoom device-enabled desk.\\n\\n## Prerequisites\\n\\n* The **Workspace Reservation** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\\n* The Zoom mobile app. This is required for QR code scanning:\\n  * Android version 5.9.0.3502 or higher\\n  * iOS version 5.9.0 or higher\\n* [Zoom Rooms for Touch devices](https://support.zoom.us/hc/en-us/articles/115004773103), version 5.9.0 or higher (*Optional*).\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Workspace Reservation Created** subscription enabled under the **Workspace Reservation** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:reservation`,`workspace:read:reservation:admin`\\n\\n**Event type**: `workspace.reservation_created`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that created the reservation.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that created the reservation.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the workspace reservation.\",\"required\":[\"id\",\"workspace_id\",\"end_time\",\"start_time\",\"topic\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The reservation ID.\"},\"workspace_id\":{\"type\":\"string\",\"description\":\"The workspace ID.\"},\"end_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's end time.\"},\"start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's start time.\"},\"topic\":{\"type\":\"string\",\"description\":\"The reservation's description.\"},\"reserve_for\":{\"type\":\"string\",\"description\":\"The ID of the user for whom to create a workspace reservation.\"},\"assignment_checkin\":{\"type\":\"boolean\",\"description\":\"Whether the assigned workspace is checked in.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"workspace.reservation_created\\\",\\n  \\\"event_ts\\\": 290112312321,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"G3NQs8u1Qni4Pc-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"workspace_id\\\": \\\"qwNQb8u1Qbi3Pc-qBPgshy\\\",\\n      \\\"topic\\\": \\\"Reserve Zoom Room-01\\\",\\n      \\\"start_time\\\": \\\"2022-01-19T02:10:00Z\\\",\\n      \\\"end_time\\\": \\\"2022-01-19T03:55:00Z\\\",\\n      \\\"reserve_for\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n      \\\"assignment_checkin\\\": true\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that created the reservation.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that created the reservation.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the workspace reservation.\",\"required\":[\"id\",\"workspace_id\",\"end_time\",\"start_time\",\"topic\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The reservation ID.\"},\"workspace_id\":{\"type\":\"string\",\"description\":\"The workspace ID.\"},\"end_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's end time.\"},\"start_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The reservation's start time.\"},\"topic\":{\"type\":\"string\",\"description\":\"The reservation's description.\"},\"reserve_for\":{\"type\":\"string\",\"description\":\"The ID of the user for whom to create a workspace reservation.\"},\"assignment_checkin\":{\"type\":\"boolean\",\"description\":\"Whether the assigned workspace is checked in.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"workspace.reservation_created\\\",\\n  \\\"event_ts\\\": 290112312321,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"G3NQs8u1Qni4Pc-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"workspace_id\\\": \\\"qwNQb8u1Qbi3Pc-qBPgshy\\\",\\n      \\\"topic\\\": \\\"Reserve Zoom Room-01\\\",\\n      \\\"start_time\\\": \\\"2022-01-19T02:10:00Z\\\",\\n      \\\"end_time\\\": \\\"2022-01-19T03:55:00Z\\\",\\n      \\\"reserve_for\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n      \\\"assignment_checkin\\\": true\\n    }\\n  }\\n}\"}}}}}}}},\"zoomroom.checked_out\":{\"post\":{\"tags\":[\"zoomroom\"],\"operationId\":\"zoomroom.checked_out\",\"requestBody\":{\"description\":\"# zoomroom.checked_out\\n\\nThe **Zoom Room Checked Out** event is triggered when a user checks out of the Zoom Room to confirm that they are releasing the room for other users to use.\\n\\nUsers can check out manually via the Zoom Room controller or the scheduling display, or they can check out programmatically using the [**Check-in or check-out of a Zoom Room**](/api-reference/zoom-api/methods#operation/checkInRooms) API.\\n\\n## Prerequisites\\n\\n* The administrator or account owner must enable the [**Room Check-In and Check-Out**](https://support.zoom.us/hc/en-us/articles/360001147163-Using-the-Zoom-Rooms-check-in-feature) feature.\\n* The Zoom Room version must be 4.7.0 or higher.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Zoom Room Checked Out** subscription enabled under the **Zoom Room** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\\n\\n**Event type**: `zoomroom.checked_out`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"id\",\"room_name\",\"calendar_name\",\"email\",\"event_id\",\"change_key\",\"resource_email\",\"calendar_id\",\"calendar_type\",\"api_type\"],\"description\":\"Information about the Zoom Room.\",\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Room's unique ID.\"},\"room_name\":{\"type\":\"string\",\"description\":\"The Zoom Room's name.\"},\"calendar_name\":{\"type\":\"string\",\"description\":\"The calendar's name.\"},\"email\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The Zoom Room's associated email address.\"},\"event_id\":{\"type\":\"string\",\"description\":\"The unique ID of the Zoom Room's associated calendar event:\\n\\n* For Microsoft Exchange calendar integration, use Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). The event_id value is the Id value in the ItemId element of the FindItem operation's response. \\n* For Google Calendar integration, use Google's [Events: list API](https://developers.google.com/calendar/api/v3/reference/events/list). The event_id value is the id field in the [events resource](https://developers.google.com/calendar/api/v3/reference/events#resource).\"},\"change_key\":{\"type\":\"string\",\"description\":\"The specific version of the calendar item. You can get this value using Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). Refer to the ChangeKey field of the ItemId element in the operation's response.\\n\\nThis field is only returned for Microsoft Exchange or Office 365 calendars.\"},\"resource_email\":{\"description\":\"The calendar's associated [resource mailbox](https://support.microsoft.com/en-us/help/10063/creating-and-managing-resource-mailboxes-in-office-365).\\n\\nThis field is only returned for Microsoft Exchange or Office 365 calendars.\",\"type\":\"string\"},\"calendar_id\":{\"type\":\"string\",\"description\":\"The calendar's unique ID.\"},\"calendar_type\":{\"type\":\"string\",\"description\":\"The calendar's type: \\n* 0 \u2014 Google Calendar. \\n* 1 \u2014 Office 365 Calendar. \\n* 2 \u2014 Microsoft Exchange Calendar.\",\"enum\":[\"0\",\"1\",\"2\"]},\"api_type\":{\"type\":\"string\",\"description\":\"The API type: \\n* 0 \u2014 [Exchange Web Services (EWS) API](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-managed-api-reference-for-exchange) \\n* 1 \u2014 [Microsoft Graph API](https://docs.microsoft.com/en-us/azure/active-directory/develop/microsoft-graph-intro) \\n* An empty string \u2014 The API returns this value when the calendar_type value is 0 (Google Calendar).\",\"enum\":[\"0\",\"1\",\"\"]}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.checked_out\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"abcD3ojfdbjfg\\\",\\n      \\\"room_name\\\": \\\"My Zoom Room\\\",\\n      \\\"calendar_name\\\": \\\"mycalendar@example.com\\\",\\n      \\\"email\\\": \\\"jchill@example.com\\\",\\n      \\\"event_id\\\": \\\"AbbbbbGYxLTc3OTVkMzFmZDc0MwBGAAAAAAD48FI58voYSqDgJePOSZblBwBQ/N0JvB/FRqv5UT2rFfkVAAAAAAENAABQ/N0JvB/FRqv5UT2rFfkVAAE2YC8DAAA=\\\",\\n      \\\"change_key\\\": \\\"DwAAABYAAABQ/N0JvB/FRqv5UT2rFfkVAAE2XqVw\\\",\\n      \\\"resource_email\\\": \\\"zroom1@example.com\\\",\\n      \\\"calendar_id\\\": \\\"mycalendar@example.com\\\",\\n      \\\"calendar_type\\\": \\\"2\\\",\\n      \\\"api_type\\\": \\\"0\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"id\",\"room_name\",\"calendar_name\",\"email\",\"event_id\",\"change_key\",\"resource_email\",\"calendar_id\",\"calendar_type\",\"api_type\"],\"description\":\"Information about the Zoom Room.\",\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Room's unique ID.\"},\"room_name\":{\"type\":\"string\",\"description\":\"The Zoom Room's name.\"},\"calendar_name\":{\"type\":\"string\",\"description\":\"The calendar's name.\"},\"email\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The Zoom Room's associated email address.\"},\"event_id\":{\"type\":\"string\",\"description\":\"The unique ID of the Zoom Room's associated calendar event:\\n\\n* For Microsoft Exchange calendar integration, use Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). The event_id value is the Id value in the ItemId element of the FindItem operation's response. \\n* For Google Calendar integration, use Google's [Events: list API](https://developers.google.com/calendar/api/v3/reference/events/list). The event_id value is the id field in the [events resource](https://developers.google.com/calendar/api/v3/reference/events#resource).\"},\"change_key\":{\"type\":\"string\",\"description\":\"The specific version of the calendar item. You can get this value using Microsoft's [FindItem operation](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/finditem-operation). Refer to the ChangeKey field of the ItemId element in the operation's response.\\n\\nThis field is only returned for Microsoft Exchange or Office 365 calendars.\"},\"resource_email\":{\"description\":\"The calendar's associated [resource mailbox](https://support.microsoft.com/en-us/help/10063/creating-and-managing-resource-mailboxes-in-office-365).\\n\\nThis field is only returned for Microsoft Exchange or Office 365 calendars.\",\"type\":\"string\"},\"calendar_id\":{\"type\":\"string\",\"description\":\"The calendar's unique ID.\"},\"calendar_type\":{\"type\":\"string\",\"description\":\"The calendar's type: \\n* 0 \u2014 Google Calendar. \\n* 1 \u2014 Office 365 Calendar. \\n* 2 \u2014 Microsoft Exchange Calendar.\",\"enum\":[\"0\",\"1\",\"2\"]},\"api_type\":{\"type\":\"string\",\"description\":\"The API type: \\n* 0 \u2014 [Exchange Web Services (EWS) API](https://docs.microsoft.com/en-us/exchange/client-developer/web-service-reference/ews-managed-api-reference-for-exchange) \\n* 1 \u2014 [Microsoft Graph API](https://docs.microsoft.com/en-us/azure/active-directory/develop/microsoft-graph-intro) \\n* An empty string \u2014 The API returns this value when the calendar_type value is 0 (Google Calendar).\",\"enum\":[\"0\",\"1\",\"\"]}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.checked_out\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"abcD3ojfdbjfg\\\",\\n      \\\"room_name\\\": \\\"My Zoom Room\\\",\\n      \\\"calendar_name\\\": \\\"mycalendar@example.com\\\",\\n      \\\"email\\\": \\\"jchill@example.com\\\",\\n      \\\"event_id\\\": \\\"AbbbbbGYxLTc3OTVkMzFmZDc0MwBGAAAAAAD48FI58voYSqDgJePOSZblBwBQ/N0JvB/FRqv5UT2rFfkVAAAAAAENAABQ/N0JvB/FRqv5UT2rFfkVAAE2YC8DAAA=\\\",\\n      \\\"change_key\\\": \\\"DwAAABYAAABQ/N0JvB/FRqv5UT2rFfkVAAE2XqVw\\\",\\n      \\\"resource_email\\\": \\\"zroom1@example.com\\\",\\n      \\\"calendar_id\\\": \\\"mycalendar@example.com\\\",\\n      \\\"calendar_type\\\": \\\"2\\\",\\n      \\\"api_type\\\": \\\"0\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoomroom.delayed_alert\":{\"post\":{\"tags\":[\"zoomroom\"],\"operationId\":\"zoomroom.delayed_alert\",\"requestBody\":{\"description\":\"# zoomroom.delayed_alert\\n\\nThe **Zoom Room Delayed Alert** event is triggered when there is an issue with a Zoom Room on a Zoom account that has installed your [webhook-enabled](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks) app. Some of the issues that can trigger this event are:\\n\\n* High CPU usage.\\n* Low battery, charging, and/or connection issues in a Zoom Room device.\\n* Room controller, camera, or speaker disconnections or reconnections.\\n* Missing camera or microphone.\\n\\nThis event notification is the same as the **Zoom Room Alert** event notification. However, instead of receiving the event in near real-time, you will receive this event notification **five minutes after an issue occurs in the Zoom Room**.\\n\\nIt is recommended that you **only** subscribe to either the **Zoom Room Alert** event or the **Zoom Room Delayed Alert** event to avoid receiving duplicate event notifications.\\n\\n## Prerequisites\\n\\n* A Business or Enterprise plan.\\n* The administrator or account owner must enable the [**Zoom Room Alert**](https://support.zoom.us/hc/en-us/articles/205394099-Zoom-Room-Alerts) feature with the selected alerts in the [Zoom Web Portal](https://zoom.us/location).\\n* The Zoom Room version must be 3.5.24524.0821 and higher.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Zoom Room Delayed Alert** subscription enabled under the **Zoom Room** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:alert:admin`\\n\\n**Event type**: `zoomroom.delayed_alert`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"id\",\"room_name\",\"calendar_name\",\"email\",\"issue\",\"alert_type\",\"component\",\"alert_time\",\"alert_kind\"],\"description\":\"Information about the Zoom Room.\",\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Room's unique ID.\"},\"room_name\":{\"type\":\"string\",\"description\":\"The Zoom Room's name.\"},\"calendar_name\":{\"type\":\"string\",\"description\":\"The calendar's name.\"},\"email\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The Zoom Room's associated email address.\"},\"issue\":{\"type\":\"string\",\"description\":\"The issues that occurred during the meeting: \\n* Room controller is disconnected. \\n* Room controller connected. \\n* Selected camera is disconnected. \\n* Selected camera is reconnected. \\n* Selected microphone is disconnected. \\n* Selected microphone is reconnected. \\n* Selected speaker is disconnected. \\n* Selected speaker is reconnected. \\n* Zoom room is offline. \\n* Zoom room is online. \\n* High CPU usage detected. \\n* Low bandwidth network is detected. \\n* {name} battery is low. \\n* {name} battery is normal. \\n* {name} disconnected. \\n* {name} is connected.\\n* Incorrect Room Passcode Entry Detected. \\n* The possible values for {name} are: \\n* Zoom Rooms Computer. \\n* Controller. \\n* Scheduling Display. \\n*\"},\"alert_type\":{\"type\":\"integer\",\"description\":\"The type of alert sent: \\n* 1 \u2014 The controller or scheduling display disconnected. \\n* 2 \u2014 The selected microphone, speaker, or camera has disconnected. \\n* 3 \u2014 The Zoom Room is offline. \\n* 4 \u2014 High CPU usage detected. \\n* 5 \u2014 Low network bandwidth detected. \\n* 6 \u2014 The controller or scheduling display battery is low. \\n* 7 \u2014 The Zoom Rooms display is disconnected. \\n* 8 \u2014 The audio device has quality issues. \\n* 9 \u2014 Failed SIP registration.\\n* 10 \u2014 Incorrect room passcode entry detected.\\n* 0 \u2014 Other alert.\",\"enum\":[1,2,3,4,5,6,7,8,9,10,0]},\"component\":{\"type\":\"integer\",\"description\":\"The affected component: \\n* 1 \u2014 Zoom Rooms computer \\n* 2 \u2014 Controller \\n* 3 \u2014 Scheduling display \\n* 4 \u2014 Microphone \\n* 5 \u2014 Camera \\n* 6 \u2014 Speaker \\n* 0 \u2014 Other\",\"enum\":[1,2,3,4,5,6,0]},\"alert_time\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the alert occurred.\"},\"alert_kind\":{\"type\":\"integer\",\"description\":\"The kind of alert: \\n* 1 \u2014 Triggered. There was an encountered problem. When the problem is resolved, a follow-up response will contain this field with the 2 (Cleared) value. \\n* 2 \u2014 Cleared. The problem is resolved. \\n* 3 \u2014 Observed. There is a potential issue-causing condition. If the issue is cleared (2), there will **not** be a follow-up response. \\n* 4 \u2014 Other.\",\"enum\":[1,2,3,4]}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.delayed_alert\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"abcD3ojfdbjfg\\\",\\n      \\\"room_name\\\": \\\"My Zoom Room\\\",\\n      \\\"calendar_name\\\": \\\"mycalendar@example.com\\\",\\n      \\\"email\\\": \\\"jchill@example.com\\\",\\n      \\\"issue\\\": \\\"Room Controller disconnected\\\",\\n      \\\"alert_type\\\": 1,\\n      \\\"component\\\": 2,\\n      \\\"alert_kind\\\": 1\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"id\",\"room_name\",\"calendar_name\",\"email\",\"issue\",\"alert_type\",\"component\",\"alert_time\",\"alert_kind\"],\"description\":\"Information about the Zoom Room.\",\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Room's unique ID.\"},\"room_name\":{\"type\":\"string\",\"description\":\"The Zoom Room's name.\"},\"calendar_name\":{\"type\":\"string\",\"description\":\"The calendar's name.\"},\"email\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The Zoom Room's associated email address.\"},\"issue\":{\"type\":\"string\",\"description\":\"The issues that occurred during the meeting: \\n* Room controller is disconnected. \\n* Room controller connected. \\n* Selected camera is disconnected. \\n* Selected camera is reconnected. \\n* Selected microphone is disconnected. \\n* Selected microphone is reconnected. \\n* Selected speaker is disconnected. \\n* Selected speaker is reconnected. \\n* Zoom room is offline. \\n* Zoom room is online. \\n* High CPU usage detected. \\n* Low bandwidth network is detected. \\n* {name} battery is low. \\n* {name} battery is normal. \\n* {name} disconnected. \\n* {name} is connected.\\n* Incorrect Room Passcode Entry Detected. \\n* The possible values for {name} are: \\n* Zoom Rooms Computer. \\n* Controller. \\n* Scheduling Display. \\n*\"},\"alert_type\":{\"type\":\"integer\",\"description\":\"The type of alert sent: \\n* 1 \u2014 The controller or scheduling display disconnected. \\n* 2 \u2014 The selected microphone, speaker, or camera has disconnected. \\n* 3 \u2014 The Zoom Room is offline. \\n* 4 \u2014 High CPU usage detected. \\n* 5 \u2014 Low network bandwidth detected. \\n* 6 \u2014 The controller or scheduling display battery is low. \\n* 7 \u2014 The Zoom Rooms display is disconnected. \\n* 8 \u2014 The audio device has quality issues. \\n* 9 \u2014 Failed SIP registration.\\n* 10 \u2014 Incorrect room passcode entry detected.\\n* 0 \u2014 Other alert.\",\"enum\":[1,2,3,4,5,6,7,8,9,10,0]},\"component\":{\"type\":\"integer\",\"description\":\"The affected component: \\n* 1 \u2014 Zoom Rooms computer \\n* 2 \u2014 Controller \\n* 3 \u2014 Scheduling display \\n* 4 \u2014 Microphone \\n* 5 \u2014 Camera \\n* 6 \u2014 Speaker \\n* 0 \u2014 Other\",\"enum\":[1,2,3,4,5,6,0]},\"alert_time\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the alert occurred.\"},\"alert_kind\":{\"type\":\"integer\",\"description\":\"The kind of alert: \\n* 1 \u2014 Triggered. There was an encountered problem. When the problem is resolved, a follow-up response will contain this field with the 2 (Cleared) value. \\n* 2 \u2014 Cleared. The problem is resolved. \\n* 3 \u2014 Observed. There is a potential issue-causing condition. If the issue is cleared (2), there will **not** be a follow-up response. \\n* 4 \u2014 Other.\",\"enum\":[1,2,3,4]}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.delayed_alert\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"abcD3ojfdbjfg\\\",\\n      \\\"room_name\\\": \\\"My Zoom Room\\\",\\n      \\\"calendar_name\\\": \\\"mycalendar@example.com\\\",\\n      \\\"email\\\": \\\"jchill@example.com\\\",\\n      \\\"issue\\\": \\\"Room Controller disconnected\\\",\\n      \\\"alert_type\\\": 1,\\n      \\\"component\\\": 2,\\n      \\\"alert_kind\\\": 1\\n    }\\n  }\\n}\"}}}}}}}},\"workspace.deleted\":{\"post\":{\"tags\":[\"workspace\"],\"operationId\":\"workspace.deleted\",\"requestBody\":{\"description\":\"# workspace.deleted\\nThe **Workspace Deleted** event is triggered every time a Workspace is deleted in your account. Admins and account owners can delete a Workspaces to an account. The Workspace can be deleted using the [Delete a Workspace API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/deleteWorkspace) or by deleting the Workspace using the Zoom Web Portal itself.\\n\\n## Prerequisites\\n\\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Workspace Deleted** subscription enabled under the **Workspace** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:workspace`,`workspace:read:workspace:admin`\\n\\n**Event type**: `workspace.deleted`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who deleted the Workspace.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user who deleted the Workspace.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that deleted the Workspace.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the deleted Workspace.\",\"required\":[\"id\",\"workspace_name\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Workspace ID.\"},\"workspace_name\":{\"type\":\"string\",\"description\":\"The name of the Workspace.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"workspace.deleted\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"workspace_name\\\" : \\\"Workspace Name\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who deleted the Workspace.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user who deleted the Workspace.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that deleted the Workspace.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the deleted Workspace.\",\"required\":[\"id\",\"workspace_name\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Workspace ID.\"},\"workspace_name\":{\"type\":\"string\",\"description\":\"The name of the Workspace.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"workspace.deleted\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"workspace_name\\\" : \\\"Workspace Name\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoomroom.deleted\":{\"post\":{\"tags\":[\"zoomroom\"],\"operationId\":\"zoomroom.deleted\",\"requestBody\":{\"description\":\"# zoomroom.deleted\\n\\nThe **Zoom Room Deleted** event is triggered when a user deletes a [**Zoom Room**](https://support.zoom.us/hc/en-us/categories/200108436).\\n\\n## Prerequisites\\n\\n* The **Zoom Room** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Zoom Room Deleted** subscription enabled under the **Zoom Room** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\\n\\n**Event type**: `zoomroom.deleted`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"id\",\"room_name\"],\"description\":\"Information about the Zoom Room.\",\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Room's unique ID.\"},\"room_name\":{\"type\":\"string\",\"description\":\"The Zoom Room's name.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.deleted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"G3NQs8u1Qni4Pc-qBPgsxg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"2muAb65FTZCGjj6ginHIbQ\\\",\\n      \\\"room_name\\\": \\\"My Zoom Room\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"id\",\"room_name\"],\"description\":\"Information about the Zoom Room.\",\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Room's unique ID.\"},\"room_name\":{\"type\":\"string\",\"description\":\"The Zoom Room's name.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.deleted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"G3NQs8u1Qni4Pc-qBPgsxg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"2muAb65FTZCGjj6ginHIbQ\\\",\\n      \\\"room_name\\\": \\\"My Zoom Room\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"visitor_management.invitation_updated\":{\"post\":{\"tags\":[\"visitor_management\"],\"operationId\":\"visitor_management.invitation_updated\",\"requestBody\":{\"description\":\"# visitor_management.invitation_updated\\n\\nThe **Visitor Management Invitation Updated** event is triggered when the [invitation is updated](https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0073629#mcetoc_1hi1ubsr1ff).\\n\\n## Prerequisites\\n\\n* The [**Visitor Management**](https://support.zoom.com/hc/en?id=zm_kb&sysparm_article=KB0057564) feature enabled by your organization.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Visitor Management Invitation Updated** subscription enabled under the **Visitor Management** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `visitor_management:read:visitor_invitation:admin`\\n\\n**Event type**: `visitor_management.invitation_updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The user account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that updated the invitation.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that updated the invitation.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the updated invitation.\",\"required\":[\"invitation_id\",\"visitor_email\",\"visitor_name\",\"start_time\",\"end_time\",\"invite_location_id\",\"host_id\",\"host_name\",\"host_email\"],\"properties\":{\"invitation_id\":{\"type\":\"string\",\"description\":\"The invitation ID.\"},\"visitor_email\":{\"type\":\"string\",\"description\":\"The email address of the invited visitor.\"},\"visitor_name\":{\"type\":\"string\",\"description\":\"The name of the invited visitor.\"},\"start_time\":{\"type\":\"long\",\"description\":\"The epoch timestamp representing the start time of the invitation.\"},\"end_time\":{\"type\":\"long\",\"description\":\"The epoch timestamp representing end time of the invitation.\"},\"invite_location_id\":{\"type\":\"string\",\"description\":\"The location ID of the location to which the visitor is invited to.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The user ID of the invitation host.\"},\"host_name\":{\"type\":\"string\",\"description\":\"The name of the invitation host.\"},\"host_email\":{\"type\":\"string\",\"description\":\"The email address of the invitation host.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"visitor_management.invitation_updated\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"invitation_id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"visitor_email\\\": \\\"visitor@example.com\\\",\\n      \\\"visitor_name\\\": \\\"visitor\\\",\\n      \\\"start_time\\\": 1710153000000,\\n      \\\"end_time\\\": 1710172799999,\\n      \\\"invite_location_id\\\": \\\"wnFFoWRwTLeww81UuAeW4g\\\",\\n      \\\"host_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n      \\\"host_name\\\": \\\"admin\\\",\\n      \\\"host_email\\\": \\\"admin@example.com\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The user account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that updated the invitation.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that updated the invitation.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the updated invitation.\",\"required\":[\"invitation_id\",\"visitor_email\",\"visitor_name\",\"start_time\",\"end_time\",\"invite_location_id\",\"host_id\",\"host_name\",\"host_email\"],\"properties\":{\"invitation_id\":{\"type\":\"string\",\"description\":\"The invitation ID.\"},\"visitor_email\":{\"type\":\"string\",\"description\":\"The email address of the invited visitor.\"},\"visitor_name\":{\"type\":\"string\",\"description\":\"The name of the invited visitor.\"},\"start_time\":{\"type\":\"long\",\"description\":\"The epoch timestamp representing the start time of the invitation.\"},\"end_time\":{\"type\":\"long\",\"description\":\"The epoch timestamp representing end time of the invitation.\"},\"invite_location_id\":{\"type\":\"string\",\"description\":\"The location ID of the location to which the visitor is invited to.\"},\"host_id\":{\"type\":\"string\",\"description\":\"The user ID of the invitation host.\"},\"host_name\":{\"type\":\"string\",\"description\":\"The name of the invitation host.\"},\"host_email\":{\"type\":\"string\",\"description\":\"The email address of the invitation host.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"visitor_management.invitation_updated\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"invitation_id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"visitor_email\\\": \\\"visitor@example.com\\\",\\n      \\\"visitor_name\\\": \\\"visitor\\\",\\n      \\\"start_time\\\": 1710153000000,\\n      \\\"end_time\\\": 1710172799999,\\n      \\\"invite_location_id\\\": \\\"wnFFoWRwTLeww81UuAeW4g\\\",\\n      \\\"host_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n      \\\"host_name\\\": \\\"admin\\\",\\n      \\\"host_email\\\": \\\"admin@example.com\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoomroom.location_created\":{\"post\":{\"tags\":[\"zoomroom\"],\"operationId\":\"zoomroom.location_created\",\"requestBody\":{\"description\":\"# zoomroom.location_created\\nThe **Zoom Rooms location has been created** event is triggered every time a new Zoom Room location is created in your account. Admins and account owners can add new Zoom Room Location to an account. The Location can be added using the [Add a Zoom Rooms Location API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/addAZRLocation) or by adding the Zoom Room Location using the Zoom Web Portal itself.\\n\\n## Prerequisites\\n\\n* Have your organization enable the **Zoom Rooms** feature through a Zoom Rooms license, or Workspace Reservation feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or **Workspace Reservation** license.\\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Zoom Rooms Location Created** subscription enabled under the **Zoom Rooms** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\\n\\n**Event type**: `zoomroom.location_created`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who created the new Zoom Rooms location.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that created the new Zoom Rooms location.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that created the new Zoom Rooms location.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the created location.\",\"required\":[\"id\",\"name\",\"parent_location_id\",\"type\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Rooms location's ID.\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the new Zoom Rooms location.\"},\"parent_location_id\":{\"type\":\"string\",\"description\":\"This location's parent Zoom Rooms location ID.\"},\"type\":{\"type\":\"enum, country\u2503states\u2503city\u2503campus\u2503building\u2503floor\",\"description\":\"The location's type.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.location_created\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"name\\\": \\\"My new Zoom Room Location\\\",\\n      \\\"parent_location_id\\\": \\\"aldjflajsd-fasj\\\",\\n      \\\"type\\\": \\\"floor\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who created the new Zoom Rooms location.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that created the new Zoom Rooms location.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that created the new Zoom Rooms location.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the created location.\",\"required\":[\"id\",\"name\",\"parent_location_id\",\"type\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Rooms location's ID.\"},\"name\":{\"type\":\"string\",\"description\":\"The name of the new Zoom Rooms location.\"},\"parent_location_id\":{\"type\":\"string\",\"description\":\"This location's parent Zoom Rooms location ID.\"},\"type\":{\"type\":\"enum, country\u2503states\u2503city\u2503campus\u2503building\u2503floor\",\"description\":\"The location's type.\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.location_created\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"name\\\": \\\"My new Zoom Room Location\\\",\\n      \\\"parent_location_id\\\": \\\"aldjflajsd-fasj\\\",\\n      \\\"type\\\": \\\"floor\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"zoomroom.created\":{\"post\":{\"tags\":[\"zoomroom\"],\"operationId\":\"zoomroom.created\",\"requestBody\":{\"description\":\"# zoomroom.created\\n\\nThe **Zoom Room Created** event is triggered when a user creates a [**Zoom Room**](https://support.zoom.us/hc/en-us/categories/200108436)\\n\\n## Prerequisites\\n\\n* The **Zoom Room** feature enabled by your organization through a [Zoom Rooms license](https://support.zoom.us/hc/en-us/articles/207483343#h_60837516-ccdd-448c-8874-5e82a4f8c906).\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Zoom Room Created** subscription enabled under the **Zoom Room** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room:admin`\\n\\n**Event type**: `zoomroom.created`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"id\",\"room_name\"],\"description\":\"Information about the Zoom Room.\",\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Room's unique ID.\"},\"room_name\":{\"type\":\"string\",\"description\":\"The Zoom Room's name.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"G3NQs8u1Qni4Pc-qBPgsxg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"2muAb65FTZCGjj6ginHIbQ\\\",\\n      \\\"room_name\\\": \\\"My Zoom Room\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"id\",\"room_name\"],\"description\":\"Information about the Zoom Room.\",\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Zoom Room's unique ID.\"},\"room_name\":{\"type\":\"string\",\"description\":\"The Zoom Room's name.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"zoomroom.created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"G3NQs8u1Qni4Pc-qBPgsxg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"2muAb65FTZCGjj6ginHIbQ\\\",\\n      \\\"room_name\\\": \\\"My Zoom Room\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"workspace.created\":{\"post\":{\"tags\":[\"workspace\"],\"operationId\":\"workspace.created\",\"requestBody\":{\"description\":\"# workspace.created\\nThe **Workspace created** event is triggered every time a Workspace is created in your account. Admins and account owners can add new Workspaces to an account. The Workspace can be added using the [Create a Workspace API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/createWorkspace) or by adding the Workspace using the Zoom Web Portal itself.\\n\\n## Prerequisites\\n\\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Workspace Created** subscription enabled under the **Workspace** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:workspace`,`workspace:read:workspace:admin`\\n\\n**Event type**: `workspace.created`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that created the workspace.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that created the workspace.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the created workspace.\",\"required\":[\"id\",\"workspace_name\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Workspace ID.\"},\"workspace_name\":{\"type\":\"string\",\"description\":\"The name of the workspace\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"workspace.created\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"workspace_name\\\": \\\"My new workspace\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that created the workspace.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that created the workspace.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the created workspace.\",\"required\":[\"id\",\"workspace_name\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Workspace ID.\"},\"workspace_name\":{\"type\":\"string\",\"description\":\"The name of the workspace\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"workspace.created\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"workspace_name\\\": \\\"My new workspace\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"workspace.updated\":{\"post\":{\"tags\":[\"workspace\"],\"operationId\":\"workspace.updated\",\"requestBody\":{\"description\":\"# workspace.updated\\nThe **Workspace Updated** event is triggered every time a Workspace is updated in your account. Admins and account owners can update a Workspaces to an account. The Workspace can be updated using the [Update a Workspace API](https://developers.zoom.us/docs/api/rest/reference/zoom-rooms/methods/#operation/updateWorkspace) or by updating the Workspace using the Zoom Web Portal itself.\\n\\n## Prerequisites\\n\\n* The **Workspace Reservation** feature enabled by your organization through a Zoom Rooms license, Zoom Phone Common Area Phone license or Workspace Reservation license.\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Workspace Updated** subscription enabled under the **Workspace** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:admin`,`workspace:write:admin`,`workspace:read`,`workspace:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workspace:read:workspace`,`workspace:read:workspace:admin`\\n\\n**Event type**: `workspace.updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that updated the Workspace.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that updated the Workspace.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the updated Workspace.\",\"required\":[\"id\",\"workspace_name\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Workspace ID.\"},\"workspace_name\":{\"type\":\"string\",\"description\":\"The name of the Workspace\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"workspace.updated\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"workspace_name\\\": \\\"My updated workspace\\\"\\n      }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Workspace account ID.\"},\"operator\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the user that updated the Workspace.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user that updated the Workspace.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the updated Workspace.\",\"required\":[\"id\",\"workspace_name\"],\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The Workspace ID.\"},\"workspace_name\":{\"type\":\"string\",\"description\":\"The name of the Workspace\"}}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"workspace.updated\\\",\\n  \\\"event_ts\\\": \\\"290112312321\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"ajdkd-qBPgsxg\\\",\\n    \\\"operator\\\": \\\"admin@example.com\\\",\\n    \\\"operator_id\\\": \\\"I7zp-pYbTl22Z29H796Ixg\\\",\\n    \\\"object\\\": {\\n      \\\"id\\\": \\\"fqtckjhjhee89ruidvc2h6lc4g\\\",\\n      \\\"workspace_name\\\": \\\"My updated workspace\\\"\\n      }\\n  }\\n}\"}}}}}}}},\"zoomroom.thirdparty_meeting_attempted\":{\"post\":{\"tags\":[\"zoomroom\"],\"operationId\":\"zoomroom.thirdparty_meeting_attempted\",\"requestBody\":{\"description\":\"# zoomroom.thirdparty_meeting_attempted\\n\\nThe **Zoom Room third party meeting attempted** event triggers when a Zoom Room is invited or actively joins a third party meeting.\\n\\n## Prerequisites\\n\\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n* A valid **Event Notification Endpoint URL**.\\n* Enable the **Zoom Room third party meeting attempted** subscription under the **Zoom Room** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:room_control:admin`\\n\\n**Event type**: `zoomroom.thirdparty_meeting_attempted`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"room_id\"],\"description\":\"A Zoom Room is about to join a meeting.\",\"properties\":{\"room_id\":{\"type\":\"string\",\"description\":\"The Zoom Room ID.\"},\"join_url\":{\"type\":\"string\",\"description\":\"The URL for joining a meeting.\"},\"meeting_source_type\":{\"type\":\"string\",\"description\":\"The meeting type.\",\"enum\":[\"MS_TEAMS\",\"GOOGLE_MEET\",\"WEBEX\",\"BLUEJEANS\",\"FUZE\",\"UNKNOWN\"]},\"join_time\":{\"description\":\"The time to execute the command to join a meeting.\",\"type\":\"string\",\"format\":\"datetime\",\"example\":\"2022-06-19T00:00:00Z\"},\"sip_address\":{\"type\":\"string\",\"description\":\"The full SIP address to join the third-party meeting.\"},\"third_party_meeting_id\":{\"type\":\"string\",\"description\":\"The of the third-party meeting's meeting ID.\"},\"passcode\":{\"type\":\"string\",\"description\":\"The passcode for joining a third-party meeting.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n\\t\\\"event\\\": \\\"zoomroom.thirdparty_meeting_attempted\\\",\\n\\t\\\"event_ts\\\": 1626230691572,\\n\\t\\\"payload\\\": {\\n\\t\\t\\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n\\t\\t\\\"object\\\": {\\n\\t\\t\\t\\\"room_id\\\": \\\"abcD3ojfdbjfg\\\",\\n            \\\"join_time\\\": \\\"2022-07-27T16:56:34Z\\\",\\n            \\\"meeting_source_type\\\": \\\"ZOOM\\\",\\n            \\\"join_url\\\": \\\"url\\\",\\n            \\\"sip_address\\\": \\\"sip address\\\",\\n            \\\"third_party_meeting_id\\\": \\\"12121313123\\\",\\n            \\\"passcode\\\": \\\"123\\\"\\n\\t\\t}\\n\\t}\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"room_id\"],\"description\":\"A Zoom Room is about to join a meeting.\",\"properties\":{\"room_id\":{\"type\":\"string\",\"description\":\"The Zoom Room ID.\"},\"join_url\":{\"type\":\"string\",\"description\":\"The URL for joining a meeting.\"},\"meeting_source_type\":{\"type\":\"string\",\"description\":\"The meeting type.\",\"enum\":[\"MS_TEAMS\",\"GOOGLE_MEET\",\"WEBEX\",\"BLUEJEANS\",\"FUZE\",\"UNKNOWN\"]},\"join_time\":{\"description\":\"The time to execute the command to join a meeting.\",\"type\":\"string\",\"format\":\"datetime\",\"example\":\"2022-06-19T00:00:00Z\"},\"sip_address\":{\"type\":\"string\",\"description\":\"The full SIP address to join the third-party meeting.\"},\"third_party_meeting_id\":{\"type\":\"string\",\"description\":\"The of the third-party meeting's meeting ID.\"},\"passcode\":{\"type\":\"string\",\"description\":\"The passcode for joining a third-party meeting.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n\\t\\\"event\\\": \\\"zoomroom.thirdparty_meeting_attempted\\\",\\n\\t\\\"event_ts\\\": 1626230691572,\\n\\t\\\"payload\\\": {\\n\\t\\t\\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n\\t\\t\\\"object\\\": {\\n\\t\\t\\t\\\"room_id\\\": \\\"abcD3ojfdbjfg\\\",\\n            \\\"join_time\\\": \\\"2022-07-27T16:56:34Z\\\",\\n            \\\"meeting_source_type\\\": \\\"ZOOM\\\",\\n            \\\"join_url\\\": \\\"url\\\",\\n            \\\"sip_address\\\": \\\"sip address\\\",\\n            \\\"third_party_meeting_id\\\": \\\"12121313123\\\",\\n            \\\"passcode\\\": \\\"123\\\"\\n\\t\\t}\\n\\t}\\n}\"}}}}}}}},\"zoomroom.digital_signage_app_started\":{\"post\":{\"tags\":[\"zoomroom\"],\"operationId\":\"zoomroom.digital_signage_app_started\",\"requestBody\":{\"description\":\"# zoomroom.digital_signage_app_started\\n\\nThe **[Zoom Room digital signage](https://support.zoom.us/hc/en-us/articles/360000030683-Zoom-Rooms-digital-signage) play started** event triggers when the digital signage starts playing or showing.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n* A valid **Event Notification Endpoint URL**.\\n* The **Zoom Room digital signage app started** subscription enabled under the **Zoom Room** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `room:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_rooms:read:digital_signage_app:admin`\\n\\n**Event type**: `zoomroom.digital_signage_app_started`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"room_id\",\"app_name\",\"app_id\",\"start_time\"],\"description\":\"The information on the digital signage played in a Zoom Room.\",\"properties\":{\"room_id\":{\"type\":\"string\",\"description\":\"The Zoom Room ID.\"},\"app_name\":{\"type\":\"string\",\"description\":\"The app name.\"},\"app_id\":{\"type\":\"string\",\"description\":\"The app ID.\"},\"start_time\":{\"description\":\"The playback start time of the digital signage.\",\"type\":\"string\",\"format\":\"datetime\",\"example\":\"2022-06-19T00:00:00Z\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n\\t\\\"event\\\": \\\"zoomroom.digital_signage_play_started\\\",\\n\\t\\\"event_ts\\\": 1626230691572,\\n\\t\\\"payload\\\": {\\n\\t\\t\\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n\\t\\t\\\"object\\\": {\\n\\t\\t\\t\\\"room_id\\\": \\\"abcD3ojfdbjfg\\\",\\n\\t\\t\\t\\\"app_name\\\": \\\"My Zoom Room\\\",\\n\\t\\t\\t\\\"app_id\\\": \\\"NiVvY1NWpE2nulNrhVjgU4jD0swziVrXBbaFZyC3u+o=\\\",\\n\\t\\t\\t\\\"start_time\\\": \\\"2022-06-19T00:00:00Z\\\",\\n            \\\"end_time\\\": \\\"2022-06-19T01:01:00Z\\\"\\n\\t\\t}\\n\\t}\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The Zoom Room account ID.\"},\"object\":{\"type\":\"object\",\"required\":[\"room_id\",\"app_name\",\"app_id\",\"start_time\"],\"description\":\"The information on the digital signage played in a Zoom Room.\",\"properties\":{\"room_id\":{\"type\":\"string\",\"description\":\"The Zoom Room ID.\"},\"app_name\":{\"type\":\"string\",\"description\":\"The app name.\"},\"app_id\":{\"type\":\"string\",\"description\":\"The app ID.\"},\"start_time\":{\"description\":\"The playback start time of the digital signage.\",\"type\":\"string\",\"format\":\"datetime\",\"example\":\"2022-06-19T00:00:00Z\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n\\t\\\"event\\\": \\\"zoomroom.digital_signage_play_started\\\",\\n\\t\\\"event_ts\\\": 1626230691572,\\n\\t\\\"payload\\\": {\\n\\t\\t\\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n\\t\\t\\\"object\\\": {\\n\\t\\t\\t\\\"room_id\\\": \\\"abcD3ojfdbjfg\\\",\\n\\t\\t\\t\\\"app_name\\\": \\\"My Zoom Room\\\",\\n\\t\\t\\t\\\"app_id\\\": \\\"NiVvY1NWpE2nulNrhVjgU4jD0swziVrXBbaFZyC3u+o=\\\",\\n\\t\\t\\t\\\"start_time\\\": \\\"2022-06-19T00:00:00Z\\\",\\n            \\\"end_time\\\": \\\"2022-06-19T01:01:00Z\\\"\\n\\t\\t}\\n\\t}\\n}\"}}}}}}}}},\"servers\":[{\"url\":\"https://{your-endpoint-url}\"}]}",
    "frontmatter": {
      "title": "Rooms Webhooks",
      "keywords": "zoom, marketplace, chatbot, chat, bot, webhook, event, zoom rooms, interactive message, button, message, action",
      "description": "The Zoom Rooms webhooks allow developers to receive events for [Zoom Rooms](https://developers.zoom.us/docs/zoom-rooms/) features.",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
