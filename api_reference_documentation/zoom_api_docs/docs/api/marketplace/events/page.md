# Marketplace Webhooks

- Source URL: https://developers.zoom.us/docs/api/marketplace/events/
- Snapshot path: docs/api/marketplace/events
- Generated (UTC): 2026-02-07T22:40:10.844972+00:00

## Frontmatter

```json
{
  "title": "Marketplace Webhooks",
  "keywords": "zoom, marketplace, rest, api, app, webhook, event, deauthorized",
  "description": "Marketplace events",
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
    "title": "Marketplace",
    "description": "",
    "version": "1.0.0"
  },
  "paths": {
    "app_deauthorized": {
      "post": {
        "tags": [
          "app_deauthorized"
        ],
        "operationId": "app_deauthorized",
        "requestBody": {
          "description": "# app_deauthorized\n\nA user or an account admin who installed your app can uninstall it at any time. The **App Deauthorization** event is triggered every time a user uninstalls or deauthorizes your app.\n\n## Prerequisites\n\n* You have configured a valid deauthorization notification URL for your app. The field is located at the bottom of the **Information** tab of the [app configuration page](https://marketplace.zoom.us/develop/create). For more information, see [Deauthorization URLs](https://marketplace.zoom.us/docs/guides/auth/deauthorization/)\n\nSee <a href=\"https://developers.zoom.us/docs/api/rest/webhook-reference/#verify-webhook-events\">Webhook verification</a> for securing your webhook endpoint.\n\n\n\n**Event type**: `app_deauthorized`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "event": {
                    "type": "string",
                    "description": "The name of the event."
                  },
                  "payload": {
                    "type": "object",
                    "properties": {
                      "user_id": {
                        "type": "string",
                        "description": "The user's ID."
                      },
                      "account_id": {
                        "type": "string",
                        "description": "The user's account ID."
                      },
                      "client_id": {
                        "type": "string",
                        "description": "The app's client ID."
                      },
                      "deauthorization_time": {
                        "type": "string",
                        "format": "date-time",
                        "description": "The date and time at which the user uninstalled the app."
                      },
                      "signature": {
                        "type": "string",
                        "description": "A unique ID for each instance of app deauthorization and uninstallation by the user."
                      },
                      "event_ts": {
                        "type": "number",
                        "description": "The timestamp of the webhook event."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"app_deauthorized\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"user_id\": \"abcD3ojfdbjfg\",\n    \"signature\": \"827edc3452044f0bc86bdd5684afb7d1e6becfa1a767f24df1b287853cf73000\",\n    \"deauthorization_time\": \"2020-06-17T13:52:28.632Z\",\n    \"client_id\": \"ABCDEFGHIJKLMNOPQRST!V\",\n    \"event_ts\": \"2024-09-08 15:58:40\"\n  }\n}"
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
                  "properties": {
                    "event": {
                      "type": "string",
                      "description": "The name of the event."
                    },
                    "payload": {
                      "type": "object",
                      "properties": {
                        "user_id": {
                          "type": "string",
                          "description": "The user's ID."
                        },
                        "account_id": {
                          "type": "string",
                          "description": "The user's account ID."
                        },
                        "client_id": {
                          "type": "string",
                          "description": "The app's client ID."
                        },
                        "deauthorization_time": {
                          "type": "string",
                          "format": "date-time",
                          "description": "The date and time at which the user uninstalled the app."
                        },
                        "signature": {
                          "type": "string",
                          "description": "A unique ID for each instance of app deauthorization and uninstallation by the user."
                        },
                        "event_ts": {
                          "type": "number",
                          "description": "The timestamp of the webhook event."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"app_deauthorized\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"user_id\": \"abcD3ojfdbjfg\",\n    \"signature\": \"827edc3452044f0bc86bdd5684afb7d1e6becfa1a767f24df1b287853cf73000\",\n    \"deauthorization_time\": \"2020-06-17T13:52:28.632Z\",\n    \"client_id\": \"ABCDEFGHIJKLMNOPQRST!V\",\n    \"event_ts\": \"2024-09-08 15:58:40\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "app.authorization_request_created": {
      "post": {
        "tags": [
          "app"
        ],
        "operationId": "app.authorization_request_created",
        "requestBody": {
          "description": "# app.authorization_request_created\n\nThe **App Request Approval** event is triggered any time a user requests to install an app.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n  * A valid **Event Notification Endpoint URL**.\n  * The **App Request Approval** subscription enabled under the **Marketplace** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `marketplace_app:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `marketplace:read:app_request:admin`\n\n**Event type**: `app.authorization_request_created`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
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
                    "properties": {
                      "app_name": {
                        "type": "string",
                        "description": "The app's name."
                      },
                      "app_type": {
                        "type": "string",
                        "description": "The app's type."
                      },
                      "app_status": {
                        "type": "string",
                        "description": "The app's status: \n* published \n* development",
                        "enum": [
                          "published",
                          "development"
                        ]
                      },
                      "app_description": {
                        "type": "string",
                        "description": "The app's description."
                      },
                      "app_link": {
                        "type": "object",
                        "description": "Information about the app's developer documentation URLs.",
                        "properties": {
                          "developer_documentation": {
                            "type": "string",
                            "format": "url",
                            "description": "The app's documentation URL."
                          },
                          "developer_privacy_policy": {
                            "type": "string",
                            "format": "url",
                            "description": "The app's privacy policy URL."
                          },
                          "developer_support": {
                            "type": "string",
                            "format": "url",
                            "description": "The app's support URL."
                          },
                          "developer_terms_of_use": {
                            "type": "string",
                            "format": "url",
                            "description": "The app's Terms of Use URL."
                          }
                        },
                        "app_permissions": {
                          "type": "object",
                          "description": "Information about the app's permissions.",
                          "properties": {
                            "View": {
                              "type": "array",
                              "description": "A list of the app's permissions.",
                              "items": {
                                "type": "string",
                                "description": "The permission's name."
                              }
                            },
                            "Manage": {
                              "type": "array",
                              "description": "Information about the app's management permissions.",
                              "items": {
                                "type": "string",
                                "description": "The management permission's name."
                              }
                            }
                          }
                        },
                        "install_requirements": {
                          "type": "object",
                          "description": "Information about the app's installation requirements.",
                          "properties": {
                            "min_client": {
                              "type": "string",
                              "description": "The app's minimum supported Zoom client version."
                            },
                            "user_role": {
                              "type": "string",
                              "description": "The user role that has permission to install the app: \n* Account Admin \u2014 Only users with an account admin role can install the app. \n* Any \u2014 Any user can install the app.",
                              "enum": [
                                "Account Admin",
                                "Any User"
                              ]
                            },
                            "account_eligibility": {
                              "type": "object",
                              "description": "Information about additional account requirements required to install the app.",
                              "properties": {
                                "premium_events": {
                                  "type": "array",
                                  "description": "Information about the app's required premium events.",
                                  "items": {
                                    "type": "string",
                                    "description": "The premium event's name."
                                  }
                                },
                                "account_types": {
                                  "type": "array",
                                  "description": "Information about the app's required account types.",
                                  "items": {
                                    "type": "string",
                                    "description": "The account type's name."
                                  }
                                }
                              }
                            }
                          }
                        },
                        "app_scopes": {
                          "type": "array",
                          "description": "Information about the app's scopes.",
                          "items": {
                            "type": "string",
                            "description": "The scope's name."
                          }
                        },
                        "requester_name": {
                          "type": "string",
                          "description": "The name of the user who requested to install the app."
                        },
                        "requester_department": {
                          "type": "string",
                          "description": "The requester's department."
                        },
                        "requester_user_groups": {
                          "type": "array",
                          "description": "The requester's assigned user group information.",
                          "items": {
                            "type": "string",
                            "description": "The group's name."
                          }
                        },
                        "request_reason": {
                          "type": "string",
                          "description": "The reason that the user requested to install the app."
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"app.authorization_request_created\",\n  \"event_ts\": \"1641884378841\",\n  \"payload\": {\n    \"app_name\": \"DevSlackBot\",\n    \"app_type\": \"SDK OAuth App\",\n    \"app_status\": \"published\",\n    \"app_description\": \"This is DevSlackBot.\",\n    \"app_link\": {\n      \"developer_documentation\": \"https://devslackbot.example.com/docs\",\n      \"developer_privacy_policy\": \"https://devslackbot.example.com/policy\",\n      \"developer_support\": \"https://devslackbot.example.com/support\",\n      \"developer_terms_of_use\": \"https://devslackbot.example.com/toc\"\n    },\n    \"app_permissions\": {\n      \"View\": [\n        \"Product Usage\"\n      ],\n      \"Manage\": [\n        \"Registration & Scheduling\",\n        \"Content\",\n        \"Participants\"\n      ]\n    },\n    \"install_requirements\": {\n      \"min_client\": \"5.7.3\",\n      \"user_role\": \"Any User\",\n      \"account_eligibility\": {\n        \"premium_events\": [\n          \"Meeting Participant QoS\",\n          \"Webinar Participant QoS\",\n          \"Meeting Participant Advanced Data\",\n          \"Webinar Participant Advanced Data\",\n          \"Meeting Participant QoS Summary\",\n          \"Webinar Participant QoS Summary\"\n        ],\n        \"account_types\": [\n          \"On Prem\",\n          \"Education\",\n          \"Pro\",\n          \"VIP\"\n        ]\n      }\n    },\n    \"app_scopes\": [\n      \"meeting:write\"\n    ],\n    \"requester_name\": \"Jill Chill\",\n    \"requester_department\": \"Development\",\n    \"requester_user_groups\": [\n      \"preappoval\"\n    ],\n    \"request_reason\": \"I need to install this app.\"\n  }\n}"
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
                      "properties": {
                        "app_name": {
                          "type": "string",
                          "description": "The app's name."
                        },
                        "app_type": {
                          "type": "string",
                          "description": "The app's type."
                        },
                        "app_status": {
                          "type": "string",
                          "description": "The app's status: \n* published \n* development",
                          "enum": [
                            "published",
                            "development"
                          ]
                        },
                        "app_description": {
                          "type": "string",
                          "description": "The app's description."
                        },
                        "app_link": {
                          "type": "object",
                          "description": "Information about the app's developer documentation URLs.",
                          "properties": {
                            "developer_documentation": {
                              "type": "string",
                              "format": "url",
                              "description": "The app's documentation URL."
                            },
                            "developer_privacy_policy": {
                              "type": "string",
                              "format": "url",
                              "description": "The app's privacy policy URL."
                            },
                            "developer_support": {
                              "type": "string",
                              "format": "url",
                              "description": "The app's support URL."
                            },
                            "developer_terms_of_use": {
                              "type": "string",
                              "format": "url",
                              "description": "The app's Terms of Use URL."
                            }
                          },
                          "app_permissions": {
                            "type": "object",
                            "description": "Information about the app's permissions.",
                            "properties": {
                              "View": {
                                "type": "array",
                                "description": "A list of the app's permissions.",
                                "items": {
                                  "type": "string",
                                  "description": "The permission's name."
                                }
                              },
                              "Manage": {
                                "type": "array",
                                "description": "Information about the app's management permissions.",
                                "items": {
                                  "type": "string",
                                  "description": "The management permission's name."
                                }
                              }
                            }
                          },
                          "install_requirements": {
                            "type": "object",
                            "description": "Information about the app's installation requirements.",
                            "properties": {
                              "min_client": {
                                "type": "string",
                                "description": "The app's minimum supported Zoom client version."
                              },
                              "user_role": {
                                "type": "string",
                                "description": "The user role that has permission to install the app: \n* Account Admin \u2014 Only users with an account admin role can install the app. \n* Any \u2014 Any user can install the app.",
                                "enum": [
                                  "Account Admin",
                                  "Any User"
                                ]
                              },
                              "account_eligibility": {
                                "type": "object",
                                "description": "Information about additional account requirements required to install the app.",
                                "properties": {
                                  "premium_events": {
                                    "type": "array",
                                    "description": "Information about the app's required premium events.",
                                    "items": {
                                      "type": "string",
                                      "description": "The premium event's name."
                                    }
                                  },
                                  "account_types": {
                                    "type": "array",
                                    "description": "Information about the app's required account types.",
                                    "items": {
                                      "type": "string",
                                      "description": "The account type's name."
                                    }
                                  }
                                }
                              }
                            }
                          },
                          "app_scopes": {
                            "type": "array",
                            "description": "Information about the app's scopes.",
                            "items": {
                              "type": "string",
                              "description": "The scope's name."
                            }
                          },
                          "requester_name": {
                            "type": "string",
                            "description": "The name of the user who requested to install the app."
                          },
                          "requester_department": {
                            "type": "string",
                            "description": "The requester's department."
                          },
                          "requester_user_groups": {
                            "type": "array",
                            "description": "The requester's assigned user group information.",
                            "items": {
                              "type": "string",
                              "description": "The group's name."
                            }
                          },
                          "request_reason": {
                            "type": "string",
                            "description": "The reason that the user requested to install the app."
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"app.authorization_request_created\",\n  \"event_ts\": \"1641884378841\",\n  \"payload\": {\n    \"app_name\": \"DevSlackBot\",\n    \"app_type\": \"SDK OAuth App\",\n    \"app_status\": \"published\",\n    \"app_description\": \"This is DevSlackBot.\",\n    \"app_link\": {\n      \"developer_documentation\": \"https://devslackbot.example.com/docs\",\n      \"developer_privacy_policy\": \"https://devslackbot.example.com/policy\",\n      \"developer_support\": \"https://devslackbot.example.com/support\",\n      \"developer_terms_of_use\": \"https://devslackbot.example.com/toc\"\n    },\n    \"app_permissions\": {\n      \"View\": [\n        \"Product Usage\"\n      ],\n      \"Manage\": [\n        \"Registration & Scheduling\",\n        \"Content\",\n        \"Participants\"\n      ]\n    },\n    \"install_requirements\": {\n      \"min_client\": \"5.7.3\",\n      \"user_role\": \"Any User\",\n      \"account_eligibility\": {\n        \"premium_events\": [\n          \"Meeting Participant QoS\",\n          \"Webinar Participant QoS\",\n          \"Meeting Participant Advanced Data\",\n          \"Webinar Participant Advanced Data\",\n          \"Meeting Participant QoS Summary\",\n          \"Webinar Participant QoS Summary\"\n        ],\n        \"account_types\": [\n          \"On Prem\",\n          \"Education\",\n          \"Pro\",\n          \"VIP\"\n        ]\n      }\n    },\n    \"app_scopes\": [\n      \"meeting:write\"\n    ],\n    \"requester_name\": \"Jill Chill\",\n    \"requester_department\": \"Development\",\n    \"requester_user_groups\": [\n      \"preappoval\"\n    ],\n    \"request_reason\": \"I need to install this app.\"\n  }\n}"
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
    "title": "Marketplace",
    "description": "",
    "version": "1.0.0"
  },
  "webhooks": {
    "app_deauthorized": {
      "post": {
        "tags": [
          "app_deauthorized"
        ],
        "operationId": "app_deauthorized",
        "requestBody": {
          "description": "# app_deauthorized\n\nA user or an account admin who installed your app can uninstall it at any time. The **App Deauthorization** event is triggered every time a user uninstalls or deauthorizes your app.\n\n## Prerequisites\n\n* You have configured a valid deauthorization notification URL for your app. The field is located at the bottom of the **Information** tab of the [app configuration page](https://marketplace.zoom.us/develop/create). For more information, see [Deauthorization URLs](https://marketplace.zoom.us/docs/guides/auth/deauthorization/)\n\nSee <a href=\"https://developers.zoom.us/docs/api/rest/webhook-reference/#verify-webhook-events\">Webhook verification</a> for securing your webhook endpoint.\n\n\n\n**Event type**: `app_deauthorized`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "event": {
                    "type": "string",
                    "description": "The name of the event."
                  },
                  "payload": {
                    "type": "object",
                    "properties": {
                      "user_id": {
                        "type": "string",
                        "description": "The user's ID."
                      },
                      "account_id": {
                        "type": "string",
                        "description": "The user's account ID."
                      },
                      "client_id": {
                        "type": "string",
                        "description": "The app's client ID."
                      },
                      "deauthorization_time": {
                        "type": "string",
                        "format": "date-time",
                        "description": "The date and time at which the user uninstalled the app."
                      },
                      "signature": {
                        "type": "string",
                        "description": "A unique ID for each instance of app deauthorization and uninstallation by the user."
                      },
                      "event_ts": {
                        "type": "number",
                        "description": "The timestamp of the webhook event."
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"app_deauthorized\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"user_id\": \"abcD3ojfdbjfg\",\n    \"signature\": \"827edc3452044f0bc86bdd5684afb7d1e6becfa1a767f24df1b287853cf73000\",\n    \"deauthorization_time\": \"2020-06-17T13:52:28.632Z\",\n    \"client_id\": \"ABCDEFGHIJKLMNOPQRST!V\",\n    \"event_ts\": \"2024-09-08 15:58:40\"\n  }\n}"
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
                  "properties": {
                    "event": {
                      "type": "string",
                      "description": "The name of the event."
                    },
                    "payload": {
                      "type": "object",
                      "properties": {
                        "user_id": {
                          "type": "string",
                          "description": "The user's ID."
                        },
                        "account_id": {
                          "type": "string",
                          "description": "The user's account ID."
                        },
                        "client_id": {
                          "type": "string",
                          "description": "The app's client ID."
                        },
                        "deauthorization_time": {
                          "type": "string",
                          "format": "date-time",
                          "description": "The date and time at which the user uninstalled the app."
                        },
                        "signature": {
                          "type": "string",
                          "description": "A unique ID for each instance of app deauthorization and uninstallation by the user."
                        },
                        "event_ts": {
                          "type": "number",
                          "description": "The timestamp of the webhook event."
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"app_deauthorized\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"user_id\": \"abcD3ojfdbjfg\",\n    \"signature\": \"827edc3452044f0bc86bdd5684afb7d1e6becfa1a767f24df1b287853cf73000\",\n    \"deauthorization_time\": \"2020-06-17T13:52:28.632Z\",\n    \"client_id\": \"ABCDEFGHIJKLMNOPQRST!V\",\n    \"event_ts\": \"2024-09-08 15:58:40\"\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "app.authorization_request_created": {
      "post": {
        "tags": [
          "app"
        ],
        "operationId": "app.authorization_request_created",
        "requestBody": {
          "description": "# app.authorization_request_created\n\nThe **App Request Approval** event is triggered any time a user requests to install an app.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n  * A valid **Event Notification Endpoint URL**.\n  * The **App Request Approval** subscription enabled under the **Marketplace** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `marketplace_app:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `marketplace:read:app_request:admin`\n\n**Event type**: `app.authorization_request_created`\n",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
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
                    "properties": {
                      "app_name": {
                        "type": "string",
                        "description": "The app's name."
                      },
                      "app_type": {
                        "type": "string",
                        "description": "The app's type."
                      },
                      "app_status": {
                        "type": "string",
                        "description": "The app's status: \n* published \n* development",
                        "enum": [
                          "published",
                          "development"
                        ]
                      },
                      "app_description": {
                        "type": "string",
                        "description": "The app's description."
                      },
                      "app_link": {
                        "type": "object",
                        "description": "Information about the app's developer documentation URLs.",
                        "properties": {
                          "developer_documentation": {
                            "type": "string",
                            "format": "url",
                            "description": "The app's documentation URL."
                          },
                          "developer_privacy_policy": {
                            "type": "string",
                            "format": "url",
                            "description": "The app's privacy policy URL."
                          },
                          "developer_support": {
                            "type": "string",
                            "format": "url",
                            "description": "The app's support URL."
                          },
                          "developer_terms_of_use": {
                            "type": "string",
                            "format": "url",
                            "description": "The app's Terms of Use URL."
                          }
                        },
                        "app_permissions": {
                          "type": "object",
                          "description": "Information about the app's permissions.",
                          "properties": {
                            "View": {
                              "type": "array",
                              "description": "A list of the app's permissions.",
                              "items": {
                                "type": "string",
                                "description": "The permission's name."
                              }
                            },
                            "Manage": {
                              "type": "array",
                              "description": "Information about the app's management permissions.",
                              "items": {
                                "type": "string",
                                "description": "The management permission's name."
                              }
                            }
                          }
                        },
                        "install_requirements": {
                          "type": "object",
                          "description": "Information about the app's installation requirements.",
                          "properties": {
                            "min_client": {
                              "type": "string",
                              "description": "The app's minimum supported Zoom client version."
                            },
                            "user_role": {
                              "type": "string",
                              "description": "The user role that has permission to install the app: \n* Account Admin \u2014 Only users with an account admin role can install the app. \n* Any \u2014 Any user can install the app.",
                              "enum": [
                                "Account Admin",
                                "Any User"
                              ]
                            },
                            "account_eligibility": {
                              "type": "object",
                              "description": "Information about additional account requirements required to install the app.",
                              "properties": {
                                "premium_events": {
                                  "type": "array",
                                  "description": "Information about the app's required premium events.",
                                  "items": {
                                    "type": "string",
                                    "description": "The premium event's name."
                                  }
                                },
                                "account_types": {
                                  "type": "array",
                                  "description": "Information about the app's required account types.",
                                  "items": {
                                    "type": "string",
                                    "description": "The account type's name."
                                  }
                                }
                              }
                            }
                          }
                        },
                        "app_scopes": {
                          "type": "array",
                          "description": "Information about the app's scopes.",
                          "items": {
                            "type": "string",
                            "description": "The scope's name."
                          }
                        },
                        "requester_name": {
                          "type": "string",
                          "description": "The name of the user who requested to install the app."
                        },
                        "requester_department": {
                          "type": "string",
                          "description": "The requester's department."
                        },
                        "requester_user_groups": {
                          "type": "array",
                          "description": "The requester's assigned user group information.",
                          "items": {
                            "type": "string",
                            "description": "The group's name."
                          }
                        },
                        "request_reason": {
                          "type": "string",
                          "description": "The reason that the user requested to install the app."
                        }
                      }
                    }
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"app.authorization_request_created\",\n  \"event_ts\": \"1641884378841\",\n  \"payload\": {\n    \"app_name\": \"DevSlackBot\",\n    \"app_type\": \"SDK OAuth App\",\n    \"app_status\": \"published\",\n    \"app_description\": \"This is DevSlackBot.\",\n    \"app_link\": {\n      \"developer_documentation\": \"https://devslackbot.example.com/docs\",\n      \"developer_privacy_policy\": \"https://devslackbot.example.com/policy\",\n      \"developer_support\": \"https://devslackbot.example.com/support\",\n      \"developer_terms_of_use\": \"https://devslackbot.example.com/toc\"\n    },\n    \"app_permissions\": {\n      \"View\": [\n        \"Product Usage\"\n      ],\n      \"Manage\": [\n        \"Registration & Scheduling\",\n        \"Content\",\n        \"Participants\"\n      ]\n    },\n    \"install_requirements\": {\n      \"min_client\": \"5.7.3\",\n      \"user_role\": \"Any User\",\n      \"account_eligibility\": {\n        \"premium_events\": [\n          \"Meeting Participant QoS\",\n          \"Webinar Participant QoS\",\n          \"Meeting Participant Advanced Data\",\n          \"Webinar Participant Advanced Data\",\n          \"Meeting Participant QoS Summary\",\n          \"Webinar Participant QoS Summary\"\n        ],\n        \"account_types\": [\n          \"On Prem\",\n          \"Education\",\n          \"Pro\",\n          \"VIP\"\n        ]\n      }\n    },\n    \"app_scopes\": [\n      \"meeting:write\"\n    ],\n    \"requester_name\": \"Jill Chill\",\n    \"requester_department\": \"Development\",\n    \"requester_user_groups\": [\n      \"preappoval\"\n    ],\n    \"request_reason\": \"I need to install this app.\"\n  }\n}"
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
                      "properties": {
                        "app_name": {
                          "type": "string",
                          "description": "The app's name."
                        },
                        "app_type": {
                          "type": "string",
                          "description": "The app's type."
                        },
                        "app_status": {
                          "type": "string",
                          "description": "The app's status: \n* published \n* development",
                          "enum": [
                            "published",
                            "development"
                          ]
                        },
                        "app_description": {
                          "type": "string",
                          "description": "The app's description."
                        },
                        "app_link": {
                          "type": "object",
                          "description": "Information about the app's developer documentation URLs.",
                          "properties": {
                            "developer_documentation": {
                              "type": "string",
                              "format": "url",
                              "description": "The app's documentation URL."
                            },
                            "developer_privacy_policy": {
                              "type": "string",
                              "format": "url",
                              "description": "The app's privacy policy URL."
                            },
                            "developer_support": {
                              "type": "string",
                              "format": "url",
                              "description": "The app's support URL."
                            },
                            "developer_terms_of_use": {
                              "type": "string",
                              "format": "url",
                              "description": "The app's Terms of Use URL."
                            }
                          },
                          "app_permissions": {
                            "type": "object",
                            "description": "Information about the app's permissions.",
                            "properties": {
                              "View": {
                                "type": "array",
                                "description": "A list of the app's permissions.",
                                "items": {
                                  "type": "string",
                                  "description": "The permission's name."
                                }
                              },
                              "Manage": {
                                "type": "array",
                                "description": "Information about the app's management permissions.",
                                "items": {
                                  "type": "string",
                                  "description": "The management permission's name."
                                }
                              }
                            }
                          },
                          "install_requirements": {
                            "type": "object",
                            "description": "Information about the app's installation requirements.",
                            "properties": {
                              "min_client": {
                                "type": "string",
                                "description": "The app's minimum supported Zoom client version."
                              },
                              "user_role": {
                                "type": "string",
                                "description": "The user role that has permission to install the app: \n* Account Admin \u2014 Only users with an account admin role can install the app. \n* Any \u2014 Any user can install the app.",
                                "enum": [
                                  "Account Admin",
                                  "Any User"
                                ]
                              },
                              "account_eligibility": {
                                "type": "object",
                                "description": "Information about additional account requirements required to install the app.",
                                "properties": {
                                  "premium_events": {
                                    "type": "array",
                                    "description": "Information about the app's required premium events.",
                                    "items": {
                                      "type": "string",
                                      "description": "The premium event's name."
                                    }
                                  },
                                  "account_types": {
                                    "type": "array",
                                    "description": "Information about the app's required account types.",
                                    "items": {
                                      "type": "string",
                                      "description": "The account type's name."
                                    }
                                  }
                                }
                              }
                            }
                          },
                          "app_scopes": {
                            "type": "array",
                            "description": "Information about the app's scopes.",
                            "items": {
                              "type": "string",
                              "description": "The scope's name."
                            }
                          },
                          "requester_name": {
                            "type": "string",
                            "description": "The name of the user who requested to install the app."
                          },
                          "requester_department": {
                            "type": "string",
                            "description": "The requester's department."
                          },
                          "requester_user_groups": {
                            "type": "array",
                            "description": "The requester's assigned user group information.",
                            "items": {
                              "type": "string",
                              "description": "The group's name."
                            }
                          },
                          "request_reason": {
                            "type": "string",
                            "description": "The reason that the user requested to install the app."
                          }
                        }
                      }
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"app.authorization_request_created\",\n  \"event_ts\": \"1641884378841\",\n  \"payload\": {\n    \"app_name\": \"DevSlackBot\",\n    \"app_type\": \"SDK OAuth App\",\n    \"app_status\": \"published\",\n    \"app_description\": \"This is DevSlackBot.\",\n    \"app_link\": {\n      \"developer_documentation\": \"https://devslackbot.example.com/docs\",\n      \"developer_privacy_policy\": \"https://devslackbot.example.com/policy\",\n      \"developer_support\": \"https://devslackbot.example.com/support\",\n      \"developer_terms_of_use\": \"https://devslackbot.example.com/toc\"\n    },\n    \"app_permissions\": {\n      \"View\": [\n        \"Product Usage\"\n      ],\n      \"Manage\": [\n        \"Registration & Scheduling\",\n        \"Content\",\n        \"Participants\"\n      ]\n    },\n    \"install_requirements\": {\n      \"min_client\": \"5.7.3\",\n      \"user_role\": \"Any User\",\n      \"account_eligibility\": {\n        \"premium_events\": [\n          \"Meeting Participant QoS\",\n          \"Webinar Participant QoS\",\n          \"Meeting Participant Advanced Data\",\n          \"Webinar Participant Advanced Data\",\n          \"Meeting Participant QoS Summary\",\n          \"Webinar Participant QoS Summary\"\n        ],\n        \"account_types\": [\n          \"On Prem\",\n          \"Education\",\n          \"Pro\",\n          \"VIP\"\n        ]\n      }\n    },\n    \"app_scopes\": [\n      \"meeting:write\"\n    ],\n    \"requester_name\": \"Jill Chill\",\n    \"requester_department\": \"Development\",\n    \"requester_user_groups\": [\n      \"preappoval\"\n    ],\n    \"request_reason\": \"I need to install this app.\"\n  }\n}"
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
      "marketplace",
      "events"
    ]
  },
  "downloadPath": "/api-hub/marketplace/events/webhooks.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": true
  },
  "productMeta": {
    "specType": "adoc",
    "slug": "/docs/api/rest/reference/app/events",
    "title": "Zoom API Events - Zoom App",
    "description": "",
    "keywords": "zoom, marketplace, rest, api, app, webhook, event, deauthorized",
    "zoapEndpoint": "webhookByProduct",
    "params": {
      "productName": "Zoom App"
    },
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "productDisplayTitle": "Marketplace",
    "scalarSlug": "/docs/api/marketplace/events",
    "svgname": "marketplace"
  },
  "frontmatter": {
    "title": "Marketplace Webhooks",
    "keywords": "zoom, marketplace, rest, api, app, webhook, event, deauthorized",
    "description": "Marketplace events",
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
        "marketplace",
        "events"
      ]
    },
    "downloadPath": "/api-hub/marketplace/events/webhooks.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": true
    },
    "productMeta": {
      "specType": "adoc",
      "slug": "/docs/api/rest/reference/app/events",
      "title": "Zoom API Events - Zoom App",
      "description": "",
      "keywords": "zoom, marketplace, rest, api, app, webhook, event, deauthorized",
      "zoapEndpoint": "webhookByProduct",
      "params": {
        "productName": "Zoom App"
      },
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "productDisplayTitle": "Marketplace",
      "scalarSlug": "/docs/api/marketplace/events",
      "svgname": "marketplace"
    },
    "spec": "{\"openapi\":\"3.1.0\",\"info\":{\"title\":\"Marketplace\",\"description\":\"\",\"version\":\"1.0.0\"},\"paths\":{\"app_deauthorized\":{\"post\":{\"tags\":[\"app_deauthorized\"],\"operationId\":\"app_deauthorized\",\"requestBody\":{\"description\":\"# app_deauthorized\\n\\nA user or an account admin who installed your app can uninstall it at any time. The **App Deauthorization** event is triggered every time a user uninstalls or deauthorizes your app.\\n\\n## Prerequisites\\n\\n* You have configured a valid deauthorization notification URL for your app. The field is located at the bottom of the **Information** tab of the [app configuration page](https://marketplace.zoom.us/develop/create). For more information, see [Deauthorization URLs](https://marketplace.zoom.us/docs/guides/auth/deauthorization/)\\n\\nSee <a href=\\\"https://developers.zoom.us/docs/api/rest/webhook-reference/#verify-webhook-events\\\">Webhook verification</a> for securing your webhook endpoint.\\n\\n\\n\\n**Event type**: `app_deauthorized`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The user's ID.\"},\"account_id\":{\"type\":\"string\",\"description\":\"The user's account ID.\"},\"client_id\":{\"type\":\"string\",\"description\":\"The app's client ID.\"},\"deauthorization_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The date and time at which the user uninstalled the app.\"},\"signature\":{\"type\":\"string\",\"description\":\"A unique ID for each instance of app deauthorization and uninstallation by the user.\"},\"event_ts\":{\"type\":\"number\",\"description\":\"The timestamp of the webhook event.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"app_deauthorized\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"user_id\\\": \\\"abcD3ojfdbjfg\\\",\\n    \\\"signature\\\": \\\"827edc3452044f0bc86bdd5684afb7d1e6becfa1a767f24df1b287853cf73000\\\",\\n    \\\"deauthorization_time\\\": \\\"2020-06-17T13:52:28.632Z\\\",\\n    \\\"client_id\\\": \\\"ABCDEFGHIJKLMNOPQRST!V\\\",\\n    \\\"event_ts\\\": \\\"2024-09-08 15:58:40\\\"\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The user's ID.\"},\"account_id\":{\"type\":\"string\",\"description\":\"The user's account ID.\"},\"client_id\":{\"type\":\"string\",\"description\":\"The app's client ID.\"},\"deauthorization_time\":{\"type\":\"string\",\"format\":\"date-time\",\"description\":\"The date and time at which the user uninstalled the app.\"},\"signature\":{\"type\":\"string\",\"description\":\"A unique ID for each instance of app deauthorization and uninstallation by the user.\"},\"event_ts\":{\"type\":\"number\",\"description\":\"The timestamp of the webhook event.\"}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"app_deauthorized\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"user_id\\\": \\\"abcD3ojfdbjfg\\\",\\n    \\\"signature\\\": \\\"827edc3452044f0bc86bdd5684afb7d1e6becfa1a767f24df1b287853cf73000\\\",\\n    \\\"deauthorization_time\\\": \\\"2020-06-17T13:52:28.632Z\\\",\\n    \\\"client_id\\\": \\\"ABCDEFGHIJKLMNOPQRST!V\\\",\\n    \\\"event_ts\\\": \\\"2024-09-08 15:58:40\\\"\\n  }\\n}\"}}}}}}}},\"app.authorization_request_created\":{\"post\":{\"tags\":[\"app\"],\"operationId\":\"app.authorization_request_created\",\"requestBody\":{\"description\":\"# app.authorization_request_created\\n\\nThe **App Request Approval** event is triggered any time a user requests to install an app.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n  * A valid **Event Notification Endpoint URL**.\\n  * The **App Request Approval** subscription enabled under the **Marketplace** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `marketplace_app:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `marketplace:read:app_request:admin`\\n\\n**Event type**: `app.authorization_request_created`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"app_name\":{\"type\":\"string\",\"description\":\"The app's name.\"},\"app_type\":{\"type\":\"string\",\"description\":\"The app's type.\"},\"app_status\":{\"type\":\"string\",\"description\":\"The app's status: \\n* published \\n* development\",\"enum\":[\"published\",\"development\"]},\"app_description\":{\"type\":\"string\",\"description\":\"The app's description.\"},\"app_link\":{\"type\":\"object\",\"description\":\"Information about the app's developer documentation URLs.\",\"properties\":{\"developer_documentation\":{\"type\":\"string\",\"format\":\"url\",\"description\":\"The app's documentation URL.\"},\"developer_privacy_policy\":{\"type\":\"string\",\"format\":\"url\",\"description\":\"The app's privacy policy URL.\"},\"developer_support\":{\"type\":\"string\",\"format\":\"url\",\"description\":\"The app's support URL.\"},\"developer_terms_of_use\":{\"type\":\"string\",\"format\":\"url\",\"description\":\"The app's Terms of Use URL.\"}},\"app_permissions\":{\"type\":\"object\",\"description\":\"Information about the app's permissions.\",\"properties\":{\"View\":{\"type\":\"array\",\"description\":\"A list of the app's permissions.\",\"items\":{\"type\":\"string\",\"description\":\"The permission's name.\"}},\"Manage\":{\"type\":\"array\",\"description\":\"Information about the app's management permissions.\",\"items\":{\"type\":\"string\",\"description\":\"The management permission's name.\"}}}},\"install_requirements\":{\"type\":\"object\",\"description\":\"Information about the app's installation requirements.\",\"properties\":{\"min_client\":{\"type\":\"string\",\"description\":\"The app's minimum supported Zoom client version.\"},\"user_role\":{\"type\":\"string\",\"description\":\"The user role that has permission to install the app: \\n* Account Admin \u2014 Only users with an account admin role can install the app. \\n* Any \u2014 Any user can install the app.\",\"enum\":[\"Account Admin\",\"Any User\"]},\"account_eligibility\":{\"type\":\"object\",\"description\":\"Information about additional account requirements required to install the app.\",\"properties\":{\"premium_events\":{\"type\":\"array\",\"description\":\"Information about the app's required premium events.\",\"items\":{\"type\":\"string\",\"description\":\"The premium event's name.\"}},\"account_types\":{\"type\":\"array\",\"description\":\"Information about the app's required account types.\",\"items\":{\"type\":\"string\",\"description\":\"The account type's name.\"}}}}}},\"app_scopes\":{\"type\":\"array\",\"description\":\"Information about the app's scopes.\",\"items\":{\"type\":\"string\",\"description\":\"The scope's name.\"}},\"requester_name\":{\"type\":\"string\",\"description\":\"The name of the user who requested to install the app.\"},\"requester_department\":{\"type\":\"string\",\"description\":\"The requester's department.\"},\"requester_user_groups\":{\"type\":\"array\",\"description\":\"The requester's assigned user group information.\",\"items\":{\"type\":\"string\",\"description\":\"The group's name.\"}},\"request_reason\":{\"type\":\"string\",\"description\":\"The reason that the user requested to install the app.\"}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"app.authorization_request_created\\\",\\n  \\\"event_ts\\\": \\\"1641884378841\\\",\\n  \\\"payload\\\": {\\n    \\\"app_name\\\": \\\"DevSlackBot\\\",\\n    \\\"app_type\\\": \\\"SDK OAuth App\\\",\\n    \\\"app_status\\\": \\\"published\\\",\\n    \\\"app_description\\\": \\\"This is DevSlackBot.\\\",\\n    \\\"app_link\\\": {\\n      \\\"developer_documentation\\\": \\\"https://devslackbot.example.com/docs\\\",\\n      \\\"developer_privacy_policy\\\": \\\"https://devslackbot.example.com/policy\\\",\\n      \\\"developer_support\\\": \\\"https://devslackbot.example.com/support\\\",\\n      \\\"developer_terms_of_use\\\": \\\"https://devslackbot.example.com/toc\\\"\\n    },\\n    \\\"app_permissions\\\": {\\n      \\\"View\\\": [\\n        \\\"Product Usage\\\"\\n      ],\\n      \\\"Manage\\\": [\\n        \\\"Registration & Scheduling\\\",\\n        \\\"Content\\\",\\n        \\\"Participants\\\"\\n      ]\\n    },\\n    \\\"install_requirements\\\": {\\n      \\\"min_client\\\": \\\"5.7.3\\\",\\n      \\\"user_role\\\": \\\"Any User\\\",\\n      \\\"account_eligibility\\\": {\\n        \\\"premium_events\\\": [\\n          \\\"Meeting Participant QoS\\\",\\n          \\\"Webinar Participant QoS\\\",\\n          \\\"Meeting Participant Advanced Data\\\",\\n          \\\"Webinar Participant Advanced Data\\\",\\n          \\\"Meeting Participant QoS Summary\\\",\\n          \\\"Webinar Participant QoS Summary\\\"\\n        ],\\n        \\\"account_types\\\": [\\n          \\\"On Prem\\\",\\n          \\\"Education\\\",\\n          \\\"Pro\\\",\\n          \\\"VIP\\\"\\n        ]\\n      }\\n    },\\n    \\\"app_scopes\\\": [\\n      \\\"meeting:write\\\"\\n    ],\\n    \\\"requester_name\\\": \\\"Jill Chill\\\",\\n    \\\"requester_department\\\": \\\"Development\\\",\\n    \\\"requester_user_groups\\\": [\\n      \\\"preappoval\\\"\\n    ],\\n    \\\"request_reason\\\": \\\"I need to install this app.\\\"\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"app_name\":{\"type\":\"string\",\"description\":\"The app's name.\"},\"app_type\":{\"type\":\"string\",\"description\":\"The app's type.\"},\"app_status\":{\"type\":\"string\",\"description\":\"The app's status: \\n* published \\n* development\",\"enum\":[\"published\",\"development\"]},\"app_description\":{\"type\":\"string\",\"description\":\"The app's description.\"},\"app_link\":{\"type\":\"object\",\"description\":\"Information about the app's developer documentation URLs.\",\"properties\":{\"developer_documentation\":{\"type\":\"string\",\"format\":\"url\",\"description\":\"The app's documentation URL.\"},\"developer_privacy_policy\":{\"type\":\"string\",\"format\":\"url\",\"description\":\"The app's privacy policy URL.\"},\"developer_support\":{\"type\":\"string\",\"format\":\"url\",\"description\":\"The app's support URL.\"},\"developer_terms_of_use\":{\"type\":\"string\",\"format\":\"url\",\"description\":\"The app's Terms of Use URL.\"}},\"app_permissions\":{\"type\":\"object\",\"description\":\"Information about the app's permissions.\",\"properties\":{\"View\":{\"type\":\"array\",\"description\":\"A list of the app's permissions.\",\"items\":{\"type\":\"string\",\"description\":\"The permission's name.\"}},\"Manage\":{\"type\":\"array\",\"description\":\"Information about the app's management permissions.\",\"items\":{\"type\":\"string\",\"description\":\"The management permission's name.\"}}}},\"install_requirements\":{\"type\":\"object\",\"description\":\"Information about the app's installation requirements.\",\"properties\":{\"min_client\":{\"type\":\"string\",\"description\":\"The app's minimum supported Zoom client version.\"},\"user_role\":{\"type\":\"string\",\"description\":\"The user role that has permission to install the app: \\n* Account Admin \u2014 Only users with an account admin role can install the app. \\n* Any \u2014 Any user can install the app.\",\"enum\":[\"Account Admin\",\"Any User\"]},\"account_eligibility\":{\"type\":\"object\",\"description\":\"Information about additional account requirements required to install the app.\",\"properties\":{\"premium_events\":{\"type\":\"array\",\"description\":\"Information about the app's required premium events.\",\"items\":{\"type\":\"string\",\"description\":\"The premium event's name.\"}},\"account_types\":{\"type\":\"array\",\"description\":\"Information about the app's required account types.\",\"items\":{\"type\":\"string\",\"description\":\"The account type's name.\"}}}}}},\"app_scopes\":{\"type\":\"array\",\"description\":\"Information about the app's scopes.\",\"items\":{\"type\":\"string\",\"description\":\"The scope's name.\"}},\"requester_name\":{\"type\":\"string\",\"description\":\"The name of the user who requested to install the app.\"},\"requester_department\":{\"type\":\"string\",\"description\":\"The requester's department.\"},\"requester_user_groups\":{\"type\":\"array\",\"description\":\"The requester's assigned user group information.\",\"items\":{\"type\":\"string\",\"description\":\"The group's name.\"}},\"request_reason\":{\"type\":\"string\",\"description\":\"The reason that the user requested to install the app.\"}}}}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"app.authorization_request_created\\\",\\n  \\\"event_ts\\\": \\\"1641884378841\\\",\\n  \\\"payload\\\": {\\n    \\\"app_name\\\": \\\"DevSlackBot\\\",\\n    \\\"app_type\\\": \\\"SDK OAuth App\\\",\\n    \\\"app_status\\\": \\\"published\\\",\\n    \\\"app_description\\\": \\\"This is DevSlackBot.\\\",\\n    \\\"app_link\\\": {\\n      \\\"developer_documentation\\\": \\\"https://devslackbot.example.com/docs\\\",\\n      \\\"developer_privacy_policy\\\": \\\"https://devslackbot.example.com/policy\\\",\\n      \\\"developer_support\\\": \\\"https://devslackbot.example.com/support\\\",\\n      \\\"developer_terms_of_use\\\": \\\"https://devslackbot.example.com/toc\\\"\\n    },\\n    \\\"app_permissions\\\": {\\n      \\\"View\\\": [\\n        \\\"Product Usage\\\"\\n      ],\\n      \\\"Manage\\\": [\\n        \\\"Registration & Scheduling\\\",\\n        \\\"Content\\\",\\n        \\\"Participants\\\"\\n      ]\\n    },\\n    \\\"install_requirements\\\": {\\n      \\\"min_client\\\": \\\"5.7.3\\\",\\n      \\\"user_role\\\": \\\"Any User\\\",\\n      \\\"account_eligibility\\\": {\\n        \\\"premium_events\\\": [\\n          \\\"Meeting Participant QoS\\\",\\n          \\\"Webinar Participant QoS\\\",\\n          \\\"Meeting Participant Advanced Data\\\",\\n          \\\"Webinar Participant Advanced Data\\\",\\n          \\\"Meeting Participant QoS Summary\\\",\\n          \\\"Webinar Participant QoS Summary\\\"\\n        ],\\n        \\\"account_types\\\": [\\n          \\\"On Prem\\\",\\n          \\\"Education\\\",\\n          \\\"Pro\\\",\\n          \\\"VIP\\\"\\n        ]\\n      }\\n    },\\n    \\\"app_scopes\\\": [\\n      \\\"meeting:write\\\"\\n    ],\\n    \\\"requester_name\\\": \\\"Jill Chill\\\",\\n    \\\"requester_department\\\": \\\"Development\\\",\\n    \\\"requester_user_groups\\\": [\\n      \\\"preappoval\\\"\\n    ],\\n    \\\"request_reason\\\": \\\"I need to install this app.\\\"\\n  }\\n}\"}}}}}}}}},\"servers\":[{\"url\":\"https://{your-endpoint-url}\"}]}",
    "frontmatter": {
      "title": "Marketplace Webhooks",
      "keywords": "zoom, marketplace, rest, api, app, webhook, event, deauthorized",
      "description": "Marketplace events",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
