# Number Management Webhooks

- Source URL: https://developers.zoom.us/docs/api/number-management/events/
- Snapshot path: docs/api/number-management/events
- Generated (UTC): 2026-02-07T22:40:11.100835+00:00

## Frontmatter

```json
{
  "title": "Number Management Webhooks",
  "keywords": "zoom, webhook, event, number management",
  "description": "The Number Management webhooks allow developers to receive events for Number Management features.",
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
    "title": "Number Management",
    "description": "The Number Management webhooks allow developers to receive events for Number Management features.",
    "version": "1.0.0"
  },
  "paths": {
    "number_management.peering_number_emergency_address_updated": {
      "post": {
        "tags": [
          "number_management"
        ],
        "operationId": "number_management.peering_number_emergency_address_updated",
        "requestBody": {
          "description": "# number_management.peering_number_emergency_address_updated\n---\n\nThe **Emergency address has changed** event triggers when the emergency addresses associated with PSTN phone numbers have been updated.\n\n## Prerequisites\n* [Enable Number Management](https://zoom.us/cpw/page/phoneNumbers#/number-list)\n* **Event subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event notification endpoint URL**.\n\t* Your app must either be a user or account level OAuth App or a Webhook Only app. For more information, see [Enable webhooks](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks).\n\t* **Emergency address has changed** subscription must be enabled under the **Number Management** event.\n\n> This event is for partners who have completed the MoU to peer with Zoom. To become a peering provider or carrier, submit your [request](https://docs.google.com/forms/d/e/1FAIpQLSewkY6ixVyKVNkWC-vgmejC16gigxsJWXji3dWzE3XlWtjsgg/viewform).\n\n\n\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `number_management_numbers:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `number_management:read:peering_numbers_emergency_address:admin`\n\n**Event type**: `number_management.peering_number_emergency_address_updated`\n",
          "content": {
            "application/json": {
              "schema": {
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
                        "description": "The account ID of the customer using provider exchange."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "carrier_code",
                          "phone_numbers"
                        ],
                        "properties": {
                          "emergency_address": {
                            "type": "object",
                            "description": "The emergency address info.",
                            "required": [
                              "address_line1"
                            ],
                            "properties": {
                              "country": {
                                "type": "string",
                                "description": "The abbreviated [country ID](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#countries) of country."
                              },
                              "address_line1": {
                                "type": "string",
                                "description": "The emergency address(house number and street name)."
                              },
                              "address_line2": {
                                "type": "string",
                                "description": "The additional emergency address information (building, floor or unit number). "
                              },
                              "city": {
                                "type": "string",
                                "description": "The city."
                              },
                              "zip": {
                                "type": "string",
                                "description": "The zip code."
                              },
                              "state_code": {
                                "type": "string",
                                "description": "The abbreviated [state code](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#states-1) of the state."
                              }
                            },
                            "additionalProperties": false
                          },
                          "carrier_code": {
                            "type": "integer",
                            "format": "int32",
                            "description": "The carrier code."
                          },
                          "phone_numbers": {
                            "type": "array",
                            "description": "The phone numbers ([E.164](https://en.wikipedia.org/wiki/E.164) format) with emergency address updates."
                          }
                        },
                        "additionalProperties": false
                      }
                    },
                    "additionalProperties": false
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"number_management.peering_number_emergency_address_updated\",\n  \"event_ts\": 1619155436400,\n  \"payload\": {\n    \"account_id\": \"Mt6sv4u0TAyPkwgsL2lxiA\",\n    \"object\": {\n      \"emergency_address\": {\n        \"address_line1\": \"55 ALMADEN BLVD\",\n        \"address_line2\": \"13F\",\n        \"city\": \"SAN JOSE\",\n        \"country\": \"US\",\n        \"state_code\": \"CA\",\n        \"zip\": \"95113\"\n      },\n      \"carrier_code\": 3457,\n      \"phone_numbers\": [\n        \"+18008001000\"\n      ]\n    }\n  }\n}"
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
                          "description": "The account ID of the customer using provider exchange."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "carrier_code",
                            "phone_numbers"
                          ],
                          "properties": {
                            "emergency_address": {
                              "type": "object",
                              "description": "The emergency address info.",
                              "required": [
                                "address_line1"
                              ],
                              "properties": {
                                "country": {
                                  "type": "string",
                                  "description": "The abbreviated [country ID](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#countries) of country."
                                },
                                "address_line1": {
                                  "type": "string",
                                  "description": "The emergency address(house number and street name)."
                                },
                                "address_line2": {
                                  "type": "string",
                                  "description": "The additional emergency address information (building, floor or unit number). "
                                },
                                "city": {
                                  "type": "string",
                                  "description": "The city."
                                },
                                "zip": {
                                  "type": "string",
                                  "description": "The zip code."
                                },
                                "state_code": {
                                  "type": "string",
                                  "description": "The abbreviated [state code](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#states-1) of the state."
                                }
                              },
                              "additionalProperties": false
                            },
                            "carrier_code": {
                              "type": "integer",
                              "format": "int32",
                              "description": "The carrier code."
                            },
                            "phone_numbers": {
                              "type": "array",
                              "description": "The phone numbers ([E.164](https://en.wikipedia.org/wiki/E.164) format) with emergency address updates."
                            }
                          },
                          "additionalProperties": false
                        }
                      },
                      "additionalProperties": false
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"number_management.peering_number_emergency_address_updated\",\n  \"event_ts\": 1619155436400,\n  \"payload\": {\n    \"account_id\": \"Mt6sv4u0TAyPkwgsL2lxiA\",\n    \"object\": {\n      \"emergency_address\": {\n        \"address_line1\": \"55 ALMADEN BLVD\",\n        \"address_line2\": \"13F\",\n        \"city\": \"SAN JOSE\",\n        \"country\": \"US\",\n        \"state_code\": \"CA\",\n        \"zip\": \"95113\"\n      },\n      \"carrier_code\": 3457,\n      \"phone_numbers\": [\n        \"+18008001000\"\n      ]\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "number_management.peering_number_cnam_updated": {
      "post": {
        "tags": [
          "number_management"
        ],
        "operationId": "number_management.peering_number_cnam_updated",
        "requestBody": {
          "description": "# number_management.peering_number_cnam_updated\n---\n\nThe **CNAM has changed** event triggers when the Caller ID name (CNAM) has been updated for PSTN phone numbers.\n\n## Prerequisites\n* [Enable Number Management](https://zoom.us/cpw/page/phoneNumbers#/number-list)\n* **Event subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event notification endpoint URL**.\n\t* Your app must either be a user or account level OAuth App or a Webhook Only app. For more information, see [Enable webhooks](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks).\n\t* **CNAM has changed** subscription must be enabled under the **Number Management** event.\n\n> This event is for partners who have completed the MoU to peer with Zoom. To become a peering provider or carrier, submit your [request](https://docs.google.com/forms/d/e/1FAIpQLSewkY6ixVyKVNkWC-vgmejC16gigxsJWXji3dWzE3XlWtjsgg/viewform).\n\n\n\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `number_management_numbers:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `number_management:read:peering_numbers_cnam:admin`\n\n**Event type**: `number_management.peering_number_cnam_updated`\n",
          "content": {
            "application/json": {
              "schema": {
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
                        "description": "The account ID of the customer using provider exchange."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "carrier_code",
                          "phone_numbers"
                        ],
                        "properties": {
                          "cnam": {
                            "type": "string",
                            "description": "The caller ID name."
                          },
                          "carrier_code": {
                            "type": "integer",
                            "format": "int32",
                            "description": "The carrier code."
                          },
                          "phone_numbers": {
                            "type": "array",
                            "description": "The phone numbers ([E.164](https://en.wikipedia.org/wiki/E.164) format) with CNAM updates."
                          }
                        },
                        "additionalProperties": false
                      }
                    },
                    "additionalProperties": false
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"number_management.peering_number_cnam_updated\",\n  \"event_ts\": 1619155436400,\n  \"payload\": {\n    \"account_id\": \"Mt6sv4u0TAyPkwgsL2lxiA\",\n    \"object\": {\n      \"cnam\": \"name\",\n      \"carrier_code\": 3457,\n      \"phone_numbers\": [\n        \"+18008001000\"\n      ]\n    }\n  }\n}"
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
                          "description": "The account ID of the customer using provider exchange."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "carrier_code",
                            "phone_numbers"
                          ],
                          "properties": {
                            "cnam": {
                              "type": "string",
                              "description": "The caller ID name."
                            },
                            "carrier_code": {
                              "type": "integer",
                              "format": "int32",
                              "description": "The carrier code."
                            },
                            "phone_numbers": {
                              "type": "array",
                              "description": "The phone numbers ([E.164](https://en.wikipedia.org/wiki/E.164) format) with CNAM updates."
                            }
                          },
                          "additionalProperties": false
                        }
                      },
                      "additionalProperties": false
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"number_management.peering_number_cnam_updated\",\n  \"event_ts\": 1619155436400,\n  \"payload\": {\n    \"account_id\": \"Mt6sv4u0TAyPkwgsL2lxiA\",\n    \"object\": {\n      \"cnam\": \"name\",\n      \"carrier_code\": 3457,\n      \"phone_numbers\": [\n        \"+18008001000\"\n      ]\n    }\n  }\n}"
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
    "title": "Number Management",
    "description": "The Number Management webhooks allow developers to receive events for Number Management features.",
    "version": "1.0.0"
  },
  "webhooks": {
    "number_management.peering_number_emergency_address_updated": {
      "post": {
        "tags": [
          "number_management"
        ],
        "operationId": "number_management.peering_number_emergency_address_updated",
        "requestBody": {
          "description": "# number_management.peering_number_emergency_address_updated\n---\n\nThe **Emergency address has changed** event triggers when the emergency addresses associated with PSTN phone numbers have been updated.\n\n## Prerequisites\n* [Enable Number Management](https://zoom.us/cpw/page/phoneNumbers#/number-list)\n* **Event subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event notification endpoint URL**.\n\t* Your app must either be a user or account level OAuth App or a Webhook Only app. For more information, see [Enable webhooks](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks).\n\t* **Emergency address has changed** subscription must be enabled under the **Number Management** event.\n\n> This event is for partners who have completed the MoU to peer with Zoom. To become a peering provider or carrier, submit your [request](https://docs.google.com/forms/d/e/1FAIpQLSewkY6ixVyKVNkWC-vgmejC16gigxsJWXji3dWzE3XlWtjsgg/viewform).\n\n\n\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `number_management_numbers:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `number_management:read:peering_numbers_emergency_address:admin`\n\n**Event type**: `number_management.peering_number_emergency_address_updated`\n",
          "content": {
            "application/json": {
              "schema": {
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
                        "description": "The account ID of the customer using provider exchange."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "carrier_code",
                          "phone_numbers"
                        ],
                        "properties": {
                          "emergency_address": {
                            "type": "object",
                            "description": "The emergency address info.",
                            "required": [
                              "address_line1"
                            ],
                            "properties": {
                              "country": {
                                "type": "string",
                                "description": "The abbreviated [country ID](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#countries) of country."
                              },
                              "address_line1": {
                                "type": "string",
                                "description": "The emergency address(house number and street name)."
                              },
                              "address_line2": {
                                "type": "string",
                                "description": "The additional emergency address information (building, floor or unit number). "
                              },
                              "city": {
                                "type": "string",
                                "description": "The city."
                              },
                              "zip": {
                                "type": "string",
                                "description": "The zip code."
                              },
                              "state_code": {
                                "type": "string",
                                "description": "The abbreviated [state code](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#states-1) of the state."
                              }
                            },
                            "additionalProperties": false
                          },
                          "carrier_code": {
                            "type": "integer",
                            "format": "int32",
                            "description": "The carrier code."
                          },
                          "phone_numbers": {
                            "type": "array",
                            "description": "The phone numbers ([E.164](https://en.wikipedia.org/wiki/E.164) format) with emergency address updates."
                          }
                        },
                        "additionalProperties": false
                      }
                    },
                    "additionalProperties": false
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"number_management.peering_number_emergency_address_updated\",\n  \"event_ts\": 1619155436400,\n  \"payload\": {\n    \"account_id\": \"Mt6sv4u0TAyPkwgsL2lxiA\",\n    \"object\": {\n      \"emergency_address\": {\n        \"address_line1\": \"55 ALMADEN BLVD\",\n        \"address_line2\": \"13F\",\n        \"city\": \"SAN JOSE\",\n        \"country\": \"US\",\n        \"state_code\": \"CA\",\n        \"zip\": \"95113\"\n      },\n      \"carrier_code\": 3457,\n      \"phone_numbers\": [\n        \"+18008001000\"\n      ]\n    }\n  }\n}"
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
                          "description": "The account ID of the customer using provider exchange."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "carrier_code",
                            "phone_numbers"
                          ],
                          "properties": {
                            "emergency_address": {
                              "type": "object",
                              "description": "The emergency address info.",
                              "required": [
                                "address_line1"
                              ],
                              "properties": {
                                "country": {
                                  "type": "string",
                                  "description": "The abbreviated [country ID](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#countries) of country."
                                },
                                "address_line1": {
                                  "type": "string",
                                  "description": "The emergency address(house number and street name)."
                                },
                                "address_line2": {
                                  "type": "string",
                                  "description": "The additional emergency address information (building, floor or unit number). "
                                },
                                "city": {
                                  "type": "string",
                                  "description": "The city."
                                },
                                "zip": {
                                  "type": "string",
                                  "description": "The zip code."
                                },
                                "state_code": {
                                  "type": "string",
                                  "description": "The abbreviated [state code](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#states-1) of the state."
                                }
                              },
                              "additionalProperties": false
                            },
                            "carrier_code": {
                              "type": "integer",
                              "format": "int32",
                              "description": "The carrier code."
                            },
                            "phone_numbers": {
                              "type": "array",
                              "description": "The phone numbers ([E.164](https://en.wikipedia.org/wiki/E.164) format) with emergency address updates."
                            }
                          },
                          "additionalProperties": false
                        }
                      },
                      "additionalProperties": false
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"number_management.peering_number_emergency_address_updated\",\n  \"event_ts\": 1619155436400,\n  \"payload\": {\n    \"account_id\": \"Mt6sv4u0TAyPkwgsL2lxiA\",\n    \"object\": {\n      \"emergency_address\": {\n        \"address_line1\": \"55 ALMADEN BLVD\",\n        \"address_line2\": \"13F\",\n        \"city\": \"SAN JOSE\",\n        \"country\": \"US\",\n        \"state_code\": \"CA\",\n        \"zip\": \"95113\"\n      },\n      \"carrier_code\": 3457,\n      \"phone_numbers\": [\n        \"+18008001000\"\n      ]\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "number_management.peering_number_cnam_updated": {
      "post": {
        "tags": [
          "number_management"
        ],
        "operationId": "number_management.peering_number_cnam_updated",
        "requestBody": {
          "description": "# number_management.peering_number_cnam_updated\n---\n\nThe **CNAM has changed** event triggers when the Caller ID name (CNAM) has been updated for PSTN phone numbers.\n\n## Prerequisites\n* [Enable Number Management](https://zoom.us/cpw/page/phoneNumbers#/number-list)\n* **Event subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event notification endpoint URL**.\n\t* Your app must either be a user or account level OAuth App or a Webhook Only app. For more information, see [Enable webhooks](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks).\n\t* **CNAM has changed** subscription must be enabled under the **Number Management** event.\n\n> This event is for partners who have completed the MoU to peer with Zoom. To become a peering provider or carrier, submit your [request](https://docs.google.com/forms/d/e/1FAIpQLSewkY6ixVyKVNkWC-vgmejC16gigxsJWXji3dWzE3XlWtjsgg/viewform).\n\n\n\n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `number_management_numbers:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `number_management:read:peering_numbers_cnam:admin`\n\n**Event type**: `number_management.peering_number_cnam_updated`\n",
          "content": {
            "application/json": {
              "schema": {
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
                        "description": "The account ID of the customer using provider exchange."
                      },
                      "object": {
                        "type": "object",
                        "required": [
                          "carrier_code",
                          "phone_numbers"
                        ],
                        "properties": {
                          "cnam": {
                            "type": "string",
                            "description": "The caller ID name."
                          },
                          "carrier_code": {
                            "type": "integer",
                            "format": "int32",
                            "description": "The carrier code."
                          },
                          "phone_numbers": {
                            "type": "array",
                            "description": "The phone numbers ([E.164](https://en.wikipedia.org/wiki/E.164) format) with CNAM updates."
                          }
                        },
                        "additionalProperties": false
                      }
                    },
                    "additionalProperties": false
                  }
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"number_management.peering_number_cnam_updated\",\n  \"event_ts\": 1619155436400,\n  \"payload\": {\n    \"account_id\": \"Mt6sv4u0TAyPkwgsL2lxiA\",\n    \"object\": {\n      \"cnam\": \"name\",\n      \"carrier_code\": 3457,\n      \"phone_numbers\": [\n        \"+18008001000\"\n      ]\n    }\n  }\n}"
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
                          "description": "The account ID of the customer using provider exchange."
                        },
                        "object": {
                          "type": "object",
                          "required": [
                            "carrier_code",
                            "phone_numbers"
                          ],
                          "properties": {
                            "cnam": {
                              "type": "string",
                              "description": "The caller ID name."
                            },
                            "carrier_code": {
                              "type": "integer",
                              "format": "int32",
                              "description": "The carrier code."
                            },
                            "phone_numbers": {
                              "type": "array",
                              "description": "The phone numbers ([E.164](https://en.wikipedia.org/wiki/E.164) format) with CNAM updates."
                            }
                          },
                          "additionalProperties": false
                        }
                      },
                      "additionalProperties": false
                    }
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"number_management.peering_number_cnam_updated\",\n  \"event_ts\": 1619155436400,\n  \"payload\": {\n    \"account_id\": \"Mt6sv4u0TAyPkwgsL2lxiA\",\n    \"object\": {\n      \"cnam\": \"name\",\n      \"carrier_code\": 3457,\n      \"phone_numbers\": [\n        \"+18008001000\"\n      ]\n    }\n  }\n}"
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
      "number-management",
      "events"
    ]
  },
  "downloadPath": "/api-hub/number-management/events/webhooks.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": true
  },
  "productMeta": {
    "specType": "adoc",
    "slug": "/docs/api/rest/reference/number-management/events",
    "productName": "Number Management",
    "title": "Zoom API Events - Number Management",
    "keywords": "zoom, webhook, event, number management",
    "zoapEndpoint": "webhookByProduct",
    "params": {
      "productName": "Number Management"
    },
    "productDisplayTitle": "Number Management",
    "scalarSlug": "/docs/api/number-management/events",
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "svgname": "number-management"
  },
  "frontmatter": {
    "title": "Number Management Webhooks",
    "keywords": "zoom, webhook, event, number management",
    "description": "The Number Management webhooks allow developers to receive events for Number Management features.",
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
        "number-management",
        "events"
      ]
    },
    "downloadPath": "/api-hub/number-management/events/webhooks.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": true
    },
    "productMeta": {
      "specType": "adoc",
      "slug": "/docs/api/rest/reference/number-management/events",
      "productName": "Number Management",
      "title": "Zoom API Events - Number Management",
      "keywords": "zoom, webhook, event, number management",
      "zoapEndpoint": "webhookByProduct",
      "params": {
        "productName": "Number Management"
      },
      "productDisplayTitle": "Number Management",
      "scalarSlug": "/docs/api/number-management/events",
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "svgname": "number-management"
    },
    "spec": "{\"openapi\":\"3.1.0\",\"info\":{\"title\":\"Number Management\",\"description\":\"The Number Management webhooks allow developers to receive events for Number Management features.\",\"version\":\"1.0.0\"},\"paths\":{\"number_management.peering_number_emergency_address_updated\":{\"post\":{\"tags\":[\"number_management\"],\"operationId\":\"number_management.peering_number_emergency_address_updated\",\"requestBody\":{\"description\":\"# number_management.peering_number_emergency_address_updated\\n---\\n\\nThe **Emergency address has changed** event triggers when the emergency addresses associated with PSTN phone numbers have been updated.\\n\\n## Prerequisites\\n* [Enable Number Management](https://zoom.us/cpw/page/phoneNumbers#/number-list)\\n* **Event subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event notification endpoint URL**.\\n\\t* Your app must either be a user or account level OAuth App or a Webhook Only app. For more information, see [Enable webhooks](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks).\\n\\t* **Emergency address has changed** subscription must be enabled under the **Number Management** event.\\n\\n> This event is for partners who have completed the MoU to peer with Zoom. To become a peering provider or carrier, submit your [request](https://docs.google.com/forms/d/e/1FAIpQLSewkY6ixVyKVNkWC-vgmejC16gigxsJWXji3dWzE3XlWtjsgg/viewform).\\n\\n\\n\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `number_management_numbers:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `number_management:read:peering_numbers_emergency_address:admin`\\n\\n**Event type**: `number_management.peering_number_emergency_address_updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp (in milliseconds since epoch) at which the event occurred. The value of this field is returned in long(int64) format.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the customer using provider exchange.\"},\"object\":{\"type\":\"object\",\"required\":[\"carrier_code\",\"phone_numbers\"],\"properties\":{\"emergency_address\":{\"type\":\"object\",\"description\":\"The emergency address info.\",\"required\":[\"address_line1\"],\"properties\":{\"country\":{\"type\":\"string\",\"description\":\"The abbreviated [country ID](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#countries) of country.\"},\"address_line1\":{\"type\":\"string\",\"description\":\"The emergency address(house number and street name).\"},\"address_line2\":{\"type\":\"string\",\"description\":\"The additional emergency address information (building, floor or unit number). \"},\"city\":{\"type\":\"string\",\"description\":\"The city.\"},\"zip\":{\"type\":\"string\",\"description\":\"The zip code.\"},\"state_code\":{\"type\":\"string\",\"description\":\"The abbreviated [state code](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#states-1) of the state.\"}},\"additionalProperties\":false},\"carrier_code\":{\"type\":\"integer\",\"format\":\"int32\",\"description\":\"The carrier code.\"},\"phone_numbers\":{\"type\":\"array\",\"description\":\"The phone numbers ([E.164](https://en.wikipedia.org/wiki/E.164) format) with emergency address updates.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"number_management.peering_number_emergency_address_updated\\\",\\n  \\\"event_ts\\\": 1619155436400,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"Mt6sv4u0TAyPkwgsL2lxiA\\\",\\n    \\\"object\\\": {\\n      \\\"emergency_address\\\": {\\n        \\\"address_line1\\\": \\\"55 ALMADEN BLVD\\\",\\n        \\\"address_line2\\\": \\\"13F\\\",\\n        \\\"city\\\": \\\"SAN JOSE\\\",\\n        \\\"country\\\": \\\"US\\\",\\n        \\\"state_code\\\": \\\"CA\\\",\\n        \\\"zip\\\": \\\"95113\\\"\\n      },\\n      \\\"carrier_code\\\": 3457,\\n      \\\"phone_numbers\\\": [\\n        \\\"+18008001000\\\"\\n      ]\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp (in milliseconds since epoch) at which the event occurred. The value of this field is returned in long(int64) format.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the customer using provider exchange.\"},\"object\":{\"type\":\"object\",\"required\":[\"carrier_code\",\"phone_numbers\"],\"properties\":{\"emergency_address\":{\"type\":\"object\",\"description\":\"The emergency address info.\",\"required\":[\"address_line1\"],\"properties\":{\"country\":{\"type\":\"string\",\"description\":\"The abbreviated [country ID](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#countries) of country.\"},\"address_line1\":{\"type\":\"string\",\"description\":\"The emergency address(house number and street name).\"},\"address_line2\":{\"type\":\"string\",\"description\":\"The additional emergency address information (building, floor or unit number). \"},\"city\":{\"type\":\"string\",\"description\":\"The city.\"},\"zip\":{\"type\":\"string\",\"description\":\"The zip code.\"},\"state_code\":{\"type\":\"string\",\"description\":\"The abbreviated [state code](https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists#states-1) of the state.\"}},\"additionalProperties\":false},\"carrier_code\":{\"type\":\"integer\",\"format\":\"int32\",\"description\":\"The carrier code.\"},\"phone_numbers\":{\"type\":\"array\",\"description\":\"The phone numbers ([E.164](https://en.wikipedia.org/wiki/E.164) format) with emergency address updates.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"number_management.peering_number_emergency_address_updated\\\",\\n  \\\"event_ts\\\": 1619155436400,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"Mt6sv4u0TAyPkwgsL2lxiA\\\",\\n    \\\"object\\\": {\\n      \\\"emergency_address\\\": {\\n        \\\"address_line1\\\": \\\"55 ALMADEN BLVD\\\",\\n        \\\"address_line2\\\": \\\"13F\\\",\\n        \\\"city\\\": \\\"SAN JOSE\\\",\\n        \\\"country\\\": \\\"US\\\",\\n        \\\"state_code\\\": \\\"CA\\\",\\n        \\\"zip\\\": \\\"95113\\\"\\n      },\\n      \\\"carrier_code\\\": 3457,\\n      \\\"phone_numbers\\\": [\\n        \\\"+18008001000\\\"\\n      ]\\n    }\\n  }\\n}\"}}}}}}}},\"number_management.peering_number_cnam_updated\":{\"post\":{\"tags\":[\"number_management\"],\"operationId\":\"number_management.peering_number_cnam_updated\",\"requestBody\":{\"description\":\"# number_management.peering_number_cnam_updated\\n---\\n\\nThe **CNAM has changed** event triggers when the Caller ID name (CNAM) has been updated for PSTN phone numbers.\\n\\n## Prerequisites\\n* [Enable Number Management](https://zoom.us/cpw/page/phoneNumbers#/number-list)\\n* **Event subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event notification endpoint URL**.\\n\\t* Your app must either be a user or account level OAuth App or a Webhook Only app. For more information, see [Enable webhooks](https://marketplace.zoom.us/docs/api-reference/webhook-reference#enable-webhooks).\\n\\t* **CNAM has changed** subscription must be enabled under the **Number Management** event.\\n\\n> This event is for partners who have completed the MoU to peer with Zoom. To become a peering provider or carrier, submit your [request](https://docs.google.com/forms/d/e/1FAIpQLSewkY6ixVyKVNkWC-vgmejC16gigxsJWXji3dWzE3XlWtjsgg/viewform).\\n\\n\\n\\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `number_management_numbers:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `number_management:read:peering_numbers_cnam:admin`\\n\\n**Event type**: `number_management.peering_number_cnam_updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp (in milliseconds since epoch) at which the event occurred. The value of this field is returned in long(int64) format.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the customer using provider exchange.\"},\"object\":{\"type\":\"object\",\"required\":[\"carrier_code\",\"phone_numbers\"],\"properties\":{\"cnam\":{\"type\":\"string\",\"description\":\"The caller ID name.\"},\"carrier_code\":{\"type\":\"integer\",\"format\":\"int32\",\"description\":\"The carrier code.\"},\"phone_numbers\":{\"type\":\"array\",\"description\":\"The phone numbers ([E.164](https://en.wikipedia.org/wiki/E.164) format) with CNAM updates.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"number_management.peering_number_cnam_updated\\\",\\n  \\\"event_ts\\\": 1619155436400,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"Mt6sv4u0TAyPkwgsL2lxiA\\\",\\n    \\\"object\\\": {\\n      \\\"cnam\\\": \\\"name\\\",\\n      \\\"carrier_code\\\": 3457,\\n      \\\"phone_numbers\\\": [\\n        \\\"+18008001000\\\"\\n      ]\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp (in milliseconds since epoch) at which the event occurred. The value of this field is returned in long(int64) format.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the customer using provider exchange.\"},\"object\":{\"type\":\"object\",\"required\":[\"carrier_code\",\"phone_numbers\"],\"properties\":{\"cnam\":{\"type\":\"string\",\"description\":\"The caller ID name.\"},\"carrier_code\":{\"type\":\"integer\",\"format\":\"int32\",\"description\":\"The carrier code.\"},\"phone_numbers\":{\"type\":\"array\",\"description\":\"The phone numbers ([E.164](https://en.wikipedia.org/wiki/E.164) format) with CNAM updates.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"number_management.peering_number_cnam_updated\\\",\\n  \\\"event_ts\\\": 1619155436400,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"Mt6sv4u0TAyPkwgsL2lxiA\\\",\\n    \\\"object\\\": {\\n      \\\"cnam\\\": \\\"name\\\",\\n      \\\"carrier_code\\\": 3457,\\n      \\\"phone_numbers\\\": [\\n        \\\"+18008001000\\\"\\n      ]\\n    }\\n  }\\n}\"}}}}}}}}},\"servers\":[{\"url\":\"https://{your-endpoint-url}\"}]}",
    "frontmatter": {
      "title": "Number Management Webhooks",
      "keywords": "zoom, webhook, event, number management",
      "description": "The Number Management webhooks allow developers to receive events for Number Management features.",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
