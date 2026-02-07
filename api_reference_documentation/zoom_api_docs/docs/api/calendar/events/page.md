# Calendar Webhooks

- Source URL: https://developers.zoom.us/docs/api/calendar/events/
- Snapshot path: docs/api/calendar/events
- Generated (UTC): 2026-02-07T22:40:10.496119+00:00

## Frontmatter

```json
{
  "title": "Calendar Webhooks",
  "keywords": "zoom, webhook, event, calendar",
  "description": "The Zoom Calendar webhooks allow developers to receive events for [Zoom Calendar](https://developers.zoom.us/docs/zoom-calendar/) features.",
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
    "title": "Calendar",
    "description": "The Zoom Calendar webhooks allow developers to receive events for [Zoom Calendar](https://developers.zoom.us/docs/zoom-calendar/) features.",
    "version": "1.0.0"
  },
  "paths": {
    "calendar.event_delete": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.event_delete",
        "requestBody": {
          "description": "# calendar.event_delete\nThe **Event delete** event triggers every time you remove an event in your calendar.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **Event delete** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:event`,`calendar:read:event:admin`\n\n**Event type**: `calendar.event_delete`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user's calendar that has the event."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The information of the event.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The event ID.",
                            "example": "y95rj3e0i54dbz8hstc2q5qoa1"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.event_delete\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"event_id\": \"y95rj3e0i54dbz8hstc2q5qoa1\" // the event ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user's calendar that has the event."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The information of the event.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The event ID.",
                              "example": "y95rj3e0i54dbz8hstc2q5qoa1"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.event_delete\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"event_id\": \"y95rj3e0i54dbz8hstc2q5qoa1\" // the event ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.acl_delete": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.acl_delete",
        "requestBody": {
          "description": "# calendar.acl_delete\nThe **ACL delete** event triggers every time you delete an ACL.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **ACL Delete** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:acl`,`calendar:read:acl:admin`\n\n**Event type**: `calendar.acl_delete`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user's calendar that has the ACL."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The information of the ACL.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "acl_id": {
                            "type": "string",
                            "description": "The ACL ID.",
                            "example": "user:def@zoom.com"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.acl_delete\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"acl_id\": \"user:def@zoom.com\" // the acl ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user's calendar that has the ACL."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The information of the ACL.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "acl_id": {
                              "type": "string",
                              "description": "The ACL ID.",
                              "example": "user:def@zoom.com"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.acl_delete\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"acl_id\": \"user:def@zoom.com\" // the acl ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.acl_create": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.acl_create",
        "requestBody": {
          "description": "# calendar.acl_create\nThe **ACL create** event triggers every time you create an ACL.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **ACL Create** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:acl`,`calendar:read:acl:admin`\n\n**Event type**: `calendar.acl_create`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user's calendar that has the ACL."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The Information of the ACL.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "acl_id": {
                            "type": "string",
                            "description": "The ACL ID.",
                            "example": "user:def@zoom.com"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.acl_create\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n       \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n       \"acl_id\": \"user:def@zoom.com\" // the acl ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user's calendar that has the ACL."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The Information of the ACL.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "acl_id": {
                              "type": "string",
                              "description": "The ACL ID.",
                              "example": "user:def@zoom.com"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.acl_create\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n       \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n       \"acl_id\": \"user:def@zoom.com\" // the acl ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.event_create": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.event_create",
        "requestBody": {
          "description": "# calendar.event_create\nThe **Event create** event triggers every time you create an event in your calendar.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **Event Create** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:event`,`calendar:read:event:admin`\n\n**Event type**: `calendar.event_create`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user's calendar that has the event."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The information of the event.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The event ID.",
                            "example": "y95rj3e0i54dbz8hstc2q5qoa1"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.event_create\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"event_id\": \"y95rj3e0i54dbz8hstc2q5qoa1\" // the event ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user's calendar that has the event."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The information of the event.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The event ID.",
                              "example": "y95rj3e0i54dbz8hstc2q5qoa1"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.event_create\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"event_id\": \"y95rj3e0i54dbz8hstc2q5qoa1\" // the event ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.setting_create": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.setting_create",
        "requestBody": {
          "description": "# calendar.setting_create\nThe **Setting create** event triggers every time you create a setting of your calendar.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **Setting create** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:setting`,`calendar:read:setting:admin`\n\n**Event type**: `calendar.setting_create`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user's calendar that has the setting."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The information of the event.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "setting_id": {
                            "type": "string",
                            "description": "The setting ID.",
                            "example": "timezone"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.setting_create\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"setting_id\": \"timezone\" // the setting ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user's calendar that has the setting."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The information of the event.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "setting_id": {
                              "type": "string",
                              "description": "The setting ID.",
                              "example": "timezone"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.setting_create\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"setting_id\": \"timezone\" // the setting ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.setting_update": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.setting_update",
        "requestBody": {
          "description": "# calendar.setting_update\nThe **Setting update** event triggers every time you update a setting of your calendar.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **Setting update** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:setting`,`calendar:read:setting:admin`\n\n**Event type**: `calendar.setting_update`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user's calendar that has the setting."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The information of the event.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "setting_id": {
                            "type": "string",
                            "description": "The setting ID.",
                            "example": "timezone"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.setting_update\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"setting_id\": \"timezone\" // the setting ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user's calendar that has the setting."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The information of the event.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "setting_id": {
                              "type": "string",
                              "description": "The setting ID.",
                              "example": "timezone"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.setting_update\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"setting_id\": \"timezone\" // the setting ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.calendarlist_update": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.calendarlist_update",
        "requestBody": {
          "description": "# calendar.calendarlist_update\nThe **Calendar list update** event triggers every time you update a calendar in your calendar list.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **Calendar list update** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:calendar_list`,`calendar:read:calendar_list:admin`\n\n**Event type**: `calendar.calendarlist_update`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of a user's calendar where the user can add or remove other calendars for event viewing."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The information of the calendar list.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "subscribed_calendar_id": {
                            "type": "string",
                            "description": "The calendar ID that the user subscribes to.",
                            "example": "def@zoom.com"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.calendarlist_update\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"subscribed_calendar_id\": \"def@zoom.com\" // the subscribed calendar ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of a user's calendar where the user can add or remove other calendars for event viewing."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The information of the calendar list.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "subscribed_calendar_id": {
                              "type": "string",
                              "description": "The calendar ID that the user subscribes to.",
                              "example": "def@zoom.com"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.calendarlist_update\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"subscribed_calendar_id\": \"def@zoom.com\" // the subscribed calendar ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.acl_update": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.acl_update",
        "requestBody": {
          "description": "# calendar.acl_update\nThe **ACL update** event triggers every time you update an ACL.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **ACL Update** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:acl`,`calendar:read:acl:admin`\n\n**Event type**: `calendar.acl_update`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user's calendar that has the ACL."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The Information of the ACL.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "acl_id": {
                            "type": "string",
                            "description": "The ACL ID.",
                            "example": "user:def@zoom.com"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.acl_update\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"acl_id\": \"user:def@zoom.com\" // the acl ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user's calendar that has the ACL."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The Information of the ACL.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "acl_id": {
                              "type": "string",
                              "description": "The ACL ID.",
                              "example": "user:def@zoom.com"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.acl_update\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"acl_id\": \"user:def@zoom.com\" // the acl ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.event_update": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.event_update",
        "requestBody": {
          "description": "# calendar.event_update\nThe **Event update** event triggers every time you update an event in your calendar.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **Event update** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:event`,`calendar:read:event:admin`\n\n**Event type**: `calendar.event_update`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "the account ID of the user's calendar that has the event."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The information of the event.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The event ID.",
                            "example": "y95rj3e0i54dbz8hstc2q5qoa1"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.event_update\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"event_id\": \"y95rj3e0i54dbz8hstc2q5qoa1\" // the event ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "the account ID of the user's calendar that has the event."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The information of the event.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The event ID.",
                              "example": "y95rj3e0i54dbz8hstc2q5qoa1"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.event_update\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"event_id\": \"y95rj3e0i54dbz8hstc2q5qoa1\" // the event ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.calendarlist_create": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.calendarlist_create",
        "requestBody": {
          "description": "# calendar.calendarlist_create\nThe **Calendar list create** event triggers every time you add a calendar to your calendarlist.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **Calendar list create** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:calendar_list`,`calendar:read:calendar_list:admin`\n\n**Event type**: `calendar.calendarlist_create `\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of a user's calendar where the user can add or remove other calendars for event viewing."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The information of the calendar list.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "subscribed_calendar_id": {
                            "type": "string",
                            "description": "The calendar ID that the user subscribes to.",
                            "example": "def@zoom.com"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.calendarlist_create\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"subscribed_calendar_id\": \"def@zoom.com\" // the subscribed calendar ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of a user's calendar where the user can add or remove other calendars for event viewing."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The information of the calendar list.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "subscribed_calendar_id": {
                              "type": "string",
                              "description": "The calendar ID that the user subscribes to.",
                              "example": "def@zoom.com"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.calendarlist_create\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"subscribed_calendar_id\": \"def@zoom.com\" // the subscribed calendar ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.calendarlist_delete": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.calendarlist_delete",
        "requestBody": {
          "description": "# calendar.calendarlist_delete\nThe **Calendar list delete** event triggers every time you remove a calendar from your calendar list.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event otification endpoint URL**.\n* The **Calendar list delete** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:calendar_list`,`calendar:read:calendar_list:admin`\n\n**Event type**: `calendar.calendarlist_delete`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of a user's calendar where the user can add or remove other calendars for event viewing"
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The information of the calendar list.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "subscribed_calendar_id": {
                            "type": "string",
                            "description": "The calendar ID of which the user subscribes.",
                            "example": "def@zoom.com"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.calendarlist_delete\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"subscribed_calendar_id\": \"def@zoom.com\" // the subscribed calendar ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of a user's calendar where the user can add or remove other calendars for event viewing"
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The information of the calendar list.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "subscribed_calendar_id": {
                              "type": "string",
                              "description": "The calendar ID of which the user subscribes.",
                              "example": "def@zoom.com"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.calendarlist_delete\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"subscribed_calendar_id\": \"def@zoom.com\" // the subscribed calendar ID.\n    }\n  }\n}"
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
    "title": "Calendar",
    "description": "The Zoom Calendar webhooks allow developers to receive events for [Zoom Calendar](https://developers.zoom.us/docs/zoom-calendar/) features.",
    "version": "1.0.0"
  },
  "webhooks": {
    "calendar.event_delete": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.event_delete",
        "requestBody": {
          "description": "# calendar.event_delete\nThe **Event delete** event triggers every time you remove an event in your calendar.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **Event delete** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:event`,`calendar:read:event:admin`\n\n**Event type**: `calendar.event_delete`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user's calendar that has the event."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The information of the event.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The event ID.",
                            "example": "y95rj3e0i54dbz8hstc2q5qoa1"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.event_delete\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"event_id\": \"y95rj3e0i54dbz8hstc2q5qoa1\" // the event ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user's calendar that has the event."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The information of the event.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The event ID.",
                              "example": "y95rj3e0i54dbz8hstc2q5qoa1"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.event_delete\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"event_id\": \"y95rj3e0i54dbz8hstc2q5qoa1\" // the event ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.acl_delete": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.acl_delete",
        "requestBody": {
          "description": "# calendar.acl_delete\nThe **ACL delete** event triggers every time you delete an ACL.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **ACL Delete** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:acl`,`calendar:read:acl:admin`\n\n**Event type**: `calendar.acl_delete`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user's calendar that has the ACL."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The information of the ACL.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "acl_id": {
                            "type": "string",
                            "description": "The ACL ID.",
                            "example": "user:def@zoom.com"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.acl_delete\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"acl_id\": \"user:def@zoom.com\" // the acl ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user's calendar that has the ACL."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The information of the ACL.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "acl_id": {
                              "type": "string",
                              "description": "The ACL ID.",
                              "example": "user:def@zoom.com"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.acl_delete\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"acl_id\": \"user:def@zoom.com\" // the acl ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.acl_create": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.acl_create",
        "requestBody": {
          "description": "# calendar.acl_create\nThe **ACL create** event triggers every time you create an ACL.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **ACL Create** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:acl`,`calendar:read:acl:admin`\n\n**Event type**: `calendar.acl_create`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user's calendar that has the ACL."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The Information of the ACL.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "acl_id": {
                            "type": "string",
                            "description": "The ACL ID.",
                            "example": "user:def@zoom.com"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.acl_create\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n       \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n       \"acl_id\": \"user:def@zoom.com\" // the acl ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user's calendar that has the ACL."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The Information of the ACL.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "acl_id": {
                              "type": "string",
                              "description": "The ACL ID.",
                              "example": "user:def@zoom.com"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.acl_create\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n       \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n       \"acl_id\": \"user:def@zoom.com\" // the acl ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.event_create": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.event_create",
        "requestBody": {
          "description": "# calendar.event_create\nThe **Event create** event triggers every time you create an event in your calendar.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **Event Create** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:event`,`calendar:read:event:admin`\n\n**Event type**: `calendar.event_create`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user's calendar that has the event."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The information of the event.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The event ID.",
                            "example": "y95rj3e0i54dbz8hstc2q5qoa1"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.event_create\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"event_id\": \"y95rj3e0i54dbz8hstc2q5qoa1\" // the event ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user's calendar that has the event."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The information of the event.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The event ID.",
                              "example": "y95rj3e0i54dbz8hstc2q5qoa1"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.event_create\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"event_id\": \"y95rj3e0i54dbz8hstc2q5qoa1\" // the event ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.setting_create": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.setting_create",
        "requestBody": {
          "description": "# calendar.setting_create\nThe **Setting create** event triggers every time you create a setting of your calendar.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **Setting create** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:setting`,`calendar:read:setting:admin`\n\n**Event type**: `calendar.setting_create`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user's calendar that has the setting."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The information of the event.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "setting_id": {
                            "type": "string",
                            "description": "The setting ID.",
                            "example": "timezone"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.setting_create\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"setting_id\": \"timezone\" // the setting ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user's calendar that has the setting."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The information of the event.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "setting_id": {
                              "type": "string",
                              "description": "The setting ID.",
                              "example": "timezone"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.setting_create\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"setting_id\": \"timezone\" // the setting ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.setting_update": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.setting_update",
        "requestBody": {
          "description": "# calendar.setting_update\nThe **Setting update** event triggers every time you update a setting of your calendar.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **Setting update** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:setting`,`calendar:read:setting:admin`\n\n**Event type**: `calendar.setting_update`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user's calendar that has the setting."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The information of the event.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "setting_id": {
                            "type": "string",
                            "description": "The setting ID.",
                            "example": "timezone"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.setting_update\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"setting_id\": \"timezone\" // the setting ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user's calendar that has the setting."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The information of the event.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "setting_id": {
                              "type": "string",
                              "description": "The setting ID.",
                              "example": "timezone"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.setting_update\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"setting_id\": \"timezone\" // the setting ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.calendarlist_update": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.calendarlist_update",
        "requestBody": {
          "description": "# calendar.calendarlist_update\nThe **Calendar list update** event triggers every time you update a calendar in your calendar list.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **Calendar list update** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:calendar_list`,`calendar:read:calendar_list:admin`\n\n**Event type**: `calendar.calendarlist_update`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of a user's calendar where the user can add or remove other calendars for event viewing."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The information of the calendar list.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "subscribed_calendar_id": {
                            "type": "string",
                            "description": "The calendar ID that the user subscribes to.",
                            "example": "def@zoom.com"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.calendarlist_update\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"subscribed_calendar_id\": \"def@zoom.com\" // the subscribed calendar ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of a user's calendar where the user can add or remove other calendars for event viewing."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The information of the calendar list.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "subscribed_calendar_id": {
                              "type": "string",
                              "description": "The calendar ID that the user subscribes to.",
                              "example": "def@zoom.com"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.calendarlist_update\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"subscribed_calendar_id\": \"def@zoom.com\" // the subscribed calendar ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.acl_update": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.acl_update",
        "requestBody": {
          "description": "# calendar.acl_update\nThe **ACL update** event triggers every time you update an ACL.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **ACL Update** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:acl`,`calendar:read:acl:admin`\n\n**Event type**: `calendar.acl_update`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user's calendar that has the ACL."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The Information of the ACL.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "acl_id": {
                            "type": "string",
                            "description": "The ACL ID.",
                            "example": "user:def@zoom.com"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.acl_update\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"acl_id\": \"user:def@zoom.com\" // the acl ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user's calendar that has the ACL."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The Information of the ACL.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "acl_id": {
                              "type": "string",
                              "description": "The ACL ID.",
                              "example": "user:def@zoom.com"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.acl_update\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"acl_id\": \"user:def@zoom.com\" // the acl ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.event_update": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.event_update",
        "requestBody": {
          "description": "# calendar.event_update\nThe **Event update** event triggers every time you update an event in your calendar.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **Event update** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:event`,`calendar:read:event:admin`\n\n**Event type**: `calendar.event_update`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "the account ID of the user's calendar that has the event."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The information of the event.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "event_id": {
                            "type": "string",
                            "description": "The event ID.",
                            "example": "y95rj3e0i54dbz8hstc2q5qoa1"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.event_update\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"event_id\": \"y95rj3e0i54dbz8hstc2q5qoa1\" // the event ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "the account ID of the user's calendar that has the event."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The information of the event.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "event_id": {
                              "type": "string",
                              "description": "The event ID.",
                              "example": "y95rj3e0i54dbz8hstc2q5qoa1"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.event_update\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"event_id\": \"y95rj3e0i54dbz8hstc2q5qoa1\" // the event ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.calendarlist_create": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.calendarlist_create",
        "requestBody": {
          "description": "# calendar.calendarlist_create\nThe **Calendar list create** event triggers every time you add a calendar to your calendarlist.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event notification endpoint URL**.\n* The **Calendar list create** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:calendar_list`,`calendar:read:calendar_list:admin`\n\n**Event type**: `calendar.calendarlist_create `\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of a user's calendar where the user can add or remove other calendars for event viewing."
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The information of the calendar list.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "subscribed_calendar_id": {
                            "type": "string",
                            "description": "The calendar ID that the user subscribes to.",
                            "example": "def@zoom.com"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.calendarlist_create\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"subscribed_calendar_id\": \"def@zoom.com\" // the subscribed calendar ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of a user's calendar where the user can add or remove other calendars for event viewing."
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The information of the calendar list.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "subscribed_calendar_id": {
                              "type": "string",
                              "description": "The calendar ID that the user subscribes to.",
                              "example": "def@zoom.com"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.calendarlist_create\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"subscribed_calendar_id\": \"def@zoom.com\" // the subscribed calendar ID.\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "calendar.calendarlist_delete": {
      "post": {
        "tags": [
          "calendar"
        ],
        "operationId": "calendar.calendarlist_delete",
        "requestBody": {
          "description": "# calendar.calendarlist_delete\nThe **Calendar list delete** event triggers every time you remove a calendar from your calendar list.\n\n## Prerequisites\n\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\n\n* A valid **Event otification endpoint URL**.\n* The **Calendar list delete** subscription enabled under the **Zoom Calendar** event.\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:calendar_list`,`calendar:read:calendar_list:admin`\n\n**Event type**: `calendar.calendarlist_delete`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of a user's calendar where the user can add or remove other calendars for event viewing"
                      },
                      "operator": {
                        "type": "string",
                        "description": "This field indicates who performed the action.",
                        "example": "abc@zoom.com"
                      },
                      "object": {
                        "type": "object",
                        "description": "The information of the calendar list.",
                        "properties": {
                          "resource_id": {
                            "type": "string",
                            "description": "The resource ID.",
                            "example": "abc@zoom.com"
                          },
                          "subscribed_calendar_id": {
                            "type": "string",
                            "description": "The calendar ID of which the user subscribes.",
                            "example": "def@zoom.com"
                          }
                        }
                      }
                    }
                  }
                },
                "additionalProperties": false
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"calendar.calendarlist_delete\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"subscribed_calendar_id\": \"def@zoom.com\" // the subscribed calendar ID.\n    }\n  }\n}"
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
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of a user's calendar where the user can add or remove other calendars for event viewing"
                        },
                        "operator": {
                          "type": "string",
                          "description": "This field indicates who performed the action.",
                          "example": "abc@zoom.com"
                        },
                        "object": {
                          "type": "object",
                          "description": "The information of the calendar list.",
                          "properties": {
                            "resource_id": {
                              "type": "string",
                              "description": "The resource ID.",
                              "example": "abc@zoom.com"
                            },
                            "subscribed_calendar_id": {
                              "type": "string",
                              "description": "The calendar ID of which the user subscribes.",
                              "example": "def@zoom.com"
                            }
                          }
                        }
                      }
                    }
                  },
                  "additionalProperties": false
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"calendar.calendarlist_delete\",\n  \"event_ts\": \"1626230691572\",\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"abc@zoom.com\", // Who initiates the action.\n    \"object\": {\n      \"resource_id\": \"abc@zoom.com\", // Your resource ID.\n      \"subscribed_calendar_id\": \"def@zoom.com\" // the subscribed calendar ID.\n    }\n  }\n}"
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
      "calendar",
      "events"
    ]
  },
  "downloadPath": "/api-hub/calendar/events/webhooks.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": false
  },
  "productMeta": {
    "specType": "adoc",
    "slug": "/docs/api/rest/reference/zoom-calendar/events",
    "productName": "Zoom Calendar",
    "title": "Zoom API Events - Calendar",
    "keywords": "zoom, webhook, event, calendar",
    "zoapEndpoint": "webhookByProduct",
    "params": {
      "productName": "Zoom Calendar"
    },
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "productDisplayTitle": "Calendar",
    "scalarSlug": "/docs/api/calendar/events",
    "svgname": "_calendar"
  },
  "frontmatter": {
    "title": "Calendar Webhooks",
    "keywords": "zoom, webhook, event, calendar",
    "description": "The Zoom Calendar webhooks allow developers to receive events for [Zoom Calendar](https://developers.zoom.us/docs/zoom-calendar/) features.",
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
        "calendar",
        "events"
      ]
    },
    "downloadPath": "/api-hub/calendar/events/webhooks.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": false
    },
    "productMeta": {
      "specType": "adoc",
      "slug": "/docs/api/rest/reference/zoom-calendar/events",
      "productName": "Zoom Calendar",
      "title": "Zoom API Events - Calendar",
      "keywords": "zoom, webhook, event, calendar",
      "zoapEndpoint": "webhookByProduct",
      "params": {
        "productName": "Zoom Calendar"
      },
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "productDisplayTitle": "Calendar",
      "scalarSlug": "/docs/api/calendar/events",
      "svgname": "_calendar"
    },
    "spec": "{\"openapi\":\"3.1.0\",\"info\":{\"title\":\"Calendar\",\"description\":\"The Zoom Calendar webhooks allow developers to receive events for [Zoom Calendar](https://developers.zoom.us/docs/zoom-calendar/) features.\",\"version\":\"1.0.0\"},\"paths\":{\"calendar.event_delete\":{\"post\":{\"tags\":[\"calendar\"],\"operationId\":\"calendar.event_delete\",\"requestBody\":{\"description\":\"# calendar.event_delete\\nThe **Event delete** event triggers every time you remove an event in your calendar.\\n\\n## Prerequisites\\n\\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\\n\\n* A valid **Event notification endpoint URL**.\\n* The **Event delete** subscription enabled under the **Zoom Calendar** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:event`,`calendar:read:event:admin`\\n\\n**Event type**: `calendar.event_delete`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user's calendar that has the event.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The information of the event.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"event_id\":{\"type\":\"string\",\"description\":\"The event ID.\",\"example\":\"y95rj3e0i54dbz8hstc2q5qoa1\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.event_delete\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"event_id\\\": \\\"y95rj3e0i54dbz8hstc2q5qoa1\\\" // the event ID.\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user's calendar that has the event.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The information of the event.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"event_id\":{\"type\":\"string\",\"description\":\"The event ID.\",\"example\":\"y95rj3e0i54dbz8hstc2q5qoa1\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.event_delete\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"event_id\\\": \\\"y95rj3e0i54dbz8hstc2q5qoa1\\\" // the event ID.\\n    }\\n  }\\n}\"}}}}}}}},\"calendar.acl_delete\":{\"post\":{\"tags\":[\"calendar\"],\"operationId\":\"calendar.acl_delete\",\"requestBody\":{\"description\":\"# calendar.acl_delete\\nThe **ACL delete** event triggers every time you delete an ACL.\\n\\n## Prerequisites\\n\\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\\n\\n* A valid **Event notification endpoint URL**.\\n* The **ACL Delete** subscription enabled under the **Zoom Calendar** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:acl`,`calendar:read:acl:admin`\\n\\n**Event type**: `calendar.acl_delete`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user's calendar that has the ACL.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The information of the ACL.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"acl_id\":{\"type\":\"string\",\"description\":\"The ACL ID.\",\"example\":\"user:def@zoom.com\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.acl_delete\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"acl_id\\\": \\\"user:def@zoom.com\\\" // the acl ID.\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user's calendar that has the ACL.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The information of the ACL.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"acl_id\":{\"type\":\"string\",\"description\":\"The ACL ID.\",\"example\":\"user:def@zoom.com\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.acl_delete\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"acl_id\\\": \\\"user:def@zoom.com\\\" // the acl ID.\\n    }\\n  }\\n}\"}}}}}}}},\"calendar.acl_create\":{\"post\":{\"tags\":[\"calendar\"],\"operationId\":\"calendar.acl_create\",\"requestBody\":{\"description\":\"# calendar.acl_create\\nThe **ACL create** event triggers every time you create an ACL.\\n\\n## Prerequisites\\n\\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\\n\\n* A valid **Event notification endpoint URL**.\\n* The **ACL Create** subscription enabled under the **Zoom Calendar** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:acl`,`calendar:read:acl:admin`\\n\\n**Event type**: `calendar.acl_create`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user's calendar that has the ACL.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The Information of the ACL.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"acl_id\":{\"type\":\"string\",\"description\":\"The ACL ID.\",\"example\":\"user:def@zoom.com\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.acl_create\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n       \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n       \\\"acl_id\\\": \\\"user:def@zoom.com\\\" // the acl ID.\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user's calendar that has the ACL.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The Information of the ACL.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"acl_id\":{\"type\":\"string\",\"description\":\"The ACL ID.\",\"example\":\"user:def@zoom.com\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.acl_create\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n       \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n       \\\"acl_id\\\": \\\"user:def@zoom.com\\\" // the acl ID.\\n    }\\n  }\\n}\"}}}}}}}},\"calendar.event_create\":{\"post\":{\"tags\":[\"calendar\"],\"operationId\":\"calendar.event_create\",\"requestBody\":{\"description\":\"# calendar.event_create\\nThe **Event create** event triggers every time you create an event in your calendar.\\n\\n## Prerequisites\\n\\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\\n\\n* A valid **Event notification endpoint URL**.\\n* The **Event Create** subscription enabled under the **Zoom Calendar** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:event`,`calendar:read:event:admin`\\n\\n**Event type**: `calendar.event_create`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user's calendar that has the event.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The information of the event.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"event_id\":{\"type\":\"string\",\"description\":\"The event ID.\",\"example\":\"y95rj3e0i54dbz8hstc2q5qoa1\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.event_create\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"event_id\\\": \\\"y95rj3e0i54dbz8hstc2q5qoa1\\\" // the event ID.\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user's calendar that has the event.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The information of the event.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"event_id\":{\"type\":\"string\",\"description\":\"The event ID.\",\"example\":\"y95rj3e0i54dbz8hstc2q5qoa1\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.event_create\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"event_id\\\": \\\"y95rj3e0i54dbz8hstc2q5qoa1\\\" // the event ID.\\n    }\\n  }\\n}\"}}}}}}}},\"calendar.setting_create\":{\"post\":{\"tags\":[\"calendar\"],\"operationId\":\"calendar.setting_create\",\"requestBody\":{\"description\":\"# calendar.setting_create\\nThe **Setting create** event triggers every time you create a setting of your calendar.\\n\\n## Prerequisites\\n\\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\\n\\n* A valid **Event notification endpoint URL**.\\n* The **Setting create** subscription enabled under the **Zoom Calendar** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:setting`,`calendar:read:setting:admin`\\n\\n**Event type**: `calendar.setting_create`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user's calendar that has the setting.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The information of the event.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"setting_id\":{\"type\":\"string\",\"description\":\"The setting ID.\",\"example\":\"timezone\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.setting_create\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"setting_id\\\": \\\"timezone\\\" // the setting ID.\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user's calendar that has the setting.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The information of the event.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"setting_id\":{\"type\":\"string\",\"description\":\"The setting ID.\",\"example\":\"timezone\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.setting_create\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"setting_id\\\": \\\"timezone\\\" // the setting ID.\\n    }\\n  }\\n}\"}}}}}}}},\"calendar.setting_update\":{\"post\":{\"tags\":[\"calendar\"],\"operationId\":\"calendar.setting_update\",\"requestBody\":{\"description\":\"# calendar.setting_update\\nThe **Setting update** event triggers every time you update a setting of your calendar.\\n\\n## Prerequisites\\n\\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\\n\\n* A valid **Event notification endpoint URL**.\\n* The **Setting update** subscription enabled under the **Zoom Calendar** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:setting`,`calendar:read:setting:admin`\\n\\n**Event type**: `calendar.setting_update`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user's calendar that has the setting.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The information of the event.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"setting_id\":{\"type\":\"string\",\"description\":\"The setting ID.\",\"example\":\"timezone\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.setting_update\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"setting_id\\\": \\\"timezone\\\" // the setting ID.\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user's calendar that has the setting.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The information of the event.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"setting_id\":{\"type\":\"string\",\"description\":\"The setting ID.\",\"example\":\"timezone\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.setting_update\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"setting_id\\\": \\\"timezone\\\" // the setting ID.\\n    }\\n  }\\n}\"}}}}}}}},\"calendar.calendarlist_update\":{\"post\":{\"tags\":[\"calendar\"],\"operationId\":\"calendar.calendarlist_update\",\"requestBody\":{\"description\":\"# calendar.calendarlist_update\\nThe **Calendar list update** event triggers every time you update a calendar in your calendar list.\\n\\n## Prerequisites\\n\\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\\n\\n* A valid **Event notification endpoint URL**.\\n* The **Calendar list update** subscription enabled under the **Zoom Calendar** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:calendar_list`,`calendar:read:calendar_list:admin`\\n\\n**Event type**: `calendar.calendarlist_update`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of a user's calendar where the user can add or remove other calendars for event viewing.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The information of the calendar list.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"subscribed_calendar_id\":{\"type\":\"string\",\"description\":\"The calendar ID that the user subscribes to.\",\"example\":\"def@zoom.com\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.calendarlist_update\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"subscribed_calendar_id\\\": \\\"def@zoom.com\\\" // the subscribed calendar ID.\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of a user's calendar where the user can add or remove other calendars for event viewing.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The information of the calendar list.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"subscribed_calendar_id\":{\"type\":\"string\",\"description\":\"The calendar ID that the user subscribes to.\",\"example\":\"def@zoom.com\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.calendarlist_update\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"subscribed_calendar_id\\\": \\\"def@zoom.com\\\" // the subscribed calendar ID.\\n    }\\n  }\\n}\"}}}}}}}},\"calendar.acl_update\":{\"post\":{\"tags\":[\"calendar\"],\"operationId\":\"calendar.acl_update\",\"requestBody\":{\"description\":\"# calendar.acl_update\\nThe **ACL update** event triggers every time you update an ACL.\\n\\n## Prerequisites\\n\\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\\n\\n* A valid **Event notification endpoint URL**.\\n* The **ACL Update** subscription enabled under the **Zoom Calendar** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:acl`,`calendar:read:acl:admin`\\n\\n**Event type**: `calendar.acl_update`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user's calendar that has the ACL.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The Information of the ACL.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"acl_id\":{\"type\":\"string\",\"description\":\"The ACL ID.\",\"example\":\"user:def@zoom.com\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.acl_update\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"acl_id\\\": \\\"user:def@zoom.com\\\" // the acl ID.\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user's calendar that has the ACL.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The Information of the ACL.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"acl_id\":{\"type\":\"string\",\"description\":\"The ACL ID.\",\"example\":\"user:def@zoom.com\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.acl_update\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"acl_id\\\": \\\"user:def@zoom.com\\\" // the acl ID.\\n    }\\n  }\\n}\"}}}}}}}},\"calendar.event_update\":{\"post\":{\"tags\":[\"calendar\"],\"operationId\":\"calendar.event_update\",\"requestBody\":{\"description\":\"# calendar.event_update\\nThe **Event update** event triggers every time you update an event in your calendar.\\n\\n## Prerequisites\\n\\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\\n\\n* A valid **Event notification endpoint URL**.\\n* The **Event update** subscription enabled under the **Zoom Calendar** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:event`,`calendar:read:event:admin`\\n\\n**Event type**: `calendar.event_update`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"the account ID of the user's calendar that has the event.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The information of the event.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"event_id\":{\"type\":\"string\",\"description\":\"The event ID.\",\"example\":\"y95rj3e0i54dbz8hstc2q5qoa1\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.event_update\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"event_id\\\": \\\"y95rj3e0i54dbz8hstc2q5qoa1\\\" // the event ID.\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"the account ID of the user's calendar that has the event.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The information of the event.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"event_id\":{\"type\":\"string\",\"description\":\"The event ID.\",\"example\":\"y95rj3e0i54dbz8hstc2q5qoa1\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.event_update\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"event_id\\\": \\\"y95rj3e0i54dbz8hstc2q5qoa1\\\" // the event ID.\\n    }\\n  }\\n}\"}}}}}}}},\"calendar.calendarlist_create\":{\"post\":{\"tags\":[\"calendar\"],\"operationId\":\"calendar.calendarlist_create\",\"requestBody\":{\"description\":\"# calendar.calendarlist_create\\nThe **Calendar list create** event triggers every time you add a calendar to your calendarlist.\\n\\n## Prerequisites\\n\\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\\n\\n* A valid **Event notification endpoint URL**.\\n* The **Calendar list create** subscription enabled under the **Zoom Calendar** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:calendar_list`,`calendar:read:calendar_list:admin`\\n\\n**Event type**: `calendar.calendarlist_create `\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of a user's calendar where the user can add or remove other calendars for event viewing.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The information of the calendar list.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"subscribed_calendar_id\":{\"type\":\"string\",\"description\":\"The calendar ID that the user subscribes to.\",\"example\":\"def@zoom.com\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.calendarlist_create\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"subscribed_calendar_id\\\": \\\"def@zoom.com\\\" // the subscribed calendar ID.\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of a user's calendar where the user can add or remove other calendars for event viewing.\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The information of the calendar list.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"subscribed_calendar_id\":{\"type\":\"string\",\"description\":\"The calendar ID that the user subscribes to.\",\"example\":\"def@zoom.com\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.calendarlist_create\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"subscribed_calendar_id\\\": \\\"def@zoom.com\\\" // the subscribed calendar ID.\\n    }\\n  }\\n}\"}}}}}}}},\"calendar.calendarlist_delete\":{\"post\":{\"tags\":[\"calendar\"],\"operationId\":\"calendar.calendarlist_delete\",\"requestBody\":{\"description\":\"# calendar.calendarlist_delete\\nThe **Calendar list delete** event triggers every time you remove a calendar from your calendar list.\\n\\n## Prerequisites\\n\\nEnable **Event subscriptions** for your [Marketplace app](http) with the following configurations:\\n\\n* A valid **Event otification endpoint URL**.\\n* The **Calendar list delete** subscription enabled under the **Zoom Calendar** event.\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:admin`,`calendar:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `calendar:read:calendar_list`,`calendar:read:calendar_list:admin`\\n\\n**Event type**: `calendar.calendarlist_delete`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of a user's calendar where the user can add or remove other calendars for event viewing\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The information of the calendar list.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"subscribed_calendar_id\":{\"type\":\"string\",\"description\":\"The calendar ID of which the user subscribes.\",\"example\":\"def@zoom.com\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.calendarlist_delete\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"subscribed_calendar_id\\\": \\\"def@zoom.com\\\" // the subscribed calendar ID.\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of a user's calendar where the user can add or remove other calendars for event viewing\"},\"operator\":{\"type\":\"string\",\"description\":\"This field indicates who performed the action.\",\"example\":\"abc@zoom.com\"},\"object\":{\"type\":\"object\",\"description\":\"The information of the calendar list.\",\"properties\":{\"resource_id\":{\"type\":\"string\",\"description\":\"The resource ID.\",\"example\":\"abc@zoom.com\"},\"subscribed_calendar_id\":{\"type\":\"string\",\"description\":\"The calendar ID of which the user subscribes.\",\"example\":\"def@zoom.com\"}}}}}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"calendar.calendarlist_delete\\\",\\n  \\\"event_ts\\\": \\\"1626230691572\\\",\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"abc@zoom.com\\\", // Who initiates the action.\\n    \\\"object\\\": {\\n      \\\"resource_id\\\": \\\"abc@zoom.com\\\", // Your resource ID.\\n      \\\"subscribed_calendar_id\\\": \\\"def@zoom.com\\\" // the subscribed calendar ID.\\n    }\\n  }\\n}\"}}}}}}}}},\"servers\":[{\"url\":\"https://{your-endpoint-url}\"}]}",
    "frontmatter": {
      "title": "Calendar Webhooks",
      "keywords": "zoom, webhook, event, calendar",
      "description": "The Zoom Calendar webhooks allow developers to receive events for [Zoom Calendar](https://developers.zoom.us/docs/zoom-calendar/) features.",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
