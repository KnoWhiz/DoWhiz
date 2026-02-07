# Contact Center Master APIs

- Source URL: https://developers.zoom.us/docs/api/contact-center/ma/
- Snapshot path: docs/api/contact-center/ma
- Generated (UTC): 2026-02-07T22:40:10.736204+00:00

## Frontmatter

```json
{
  "title": "Contact Center Master APIs",
  "keywords": "zoom, marketplace, rest, api, create, user, get, delete, contact center, custom, programmatic, task, queue, agent, supervisor, disposition, asset, skill, address, book, report, note",
  "description": "The Contact Center APIs allow developers to interface with [Contact Center](https://developers.zoom.us/docs/contact-center/) features programmatically.",
  "skipQualtrics": true
}
```

## Content

No MDX content found for this page.

## OpenAPI

```json
{
  "openapi": "3.0.0",
  "info": {
    "title": "Contact Center",
    "description": "The Contact Center APIs allow developers to interface with [Contact Center](https://developers.zoom.us/docs/contact-center/) features programmatically.",
    "termsOfService": "https://zoom.us/docs/en-us/zoom_api_license_and_tou.html",
    "contact": {
      "name": "Zoom Developers",
      "url": "https://developer.zoom.us/"
    },
    "version": "2"
  },
  "externalDocs": {
    "description": "Find out more about Swagger",
    "url": "https://swagger.io"
  },
  "servers": [
    {
      "url": "https://api.zoom.us/v2"
    }
  ],
  "paths": {
    "/accounts/{accountId}/contact_center/sms": {
      "get": {
        "tags": [
          "Logs"
        ],
        "summary": "List SMS logs",
        "description": "Returns a list of SMS engagement logs. Engagement data are available via API once they are completed.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `contact_center_sms:master`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `contact_center:read:sms_log:master`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `HEAVY`",
        "operationId": "listSMS",
        "parameters": [
          {
            "name": "next_page_token",
            "in": "query",
            "description": "Use the next page token to paginate through large result sets. A next page token is returned whenever the set of available results exceeds the current page size. This token's expiration period is 15 minutes.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42"
            }
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of items returned per page.",
            "required": false,
            "schema": {
              "maximum": 100,
              "type": "integer",
              "format": "int32",
              "example": 10,
              "default": 10
            }
          },
          {
            "name": "from",
            "in": "query",
            "description": "The start time and date in `yyyy-mm-dd` or ISO 8601 format, which is either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`. The defined date range should be a month, as the response only includes one month's worth of data. If no start date is specified, return data from the past 24 hours.",
            "required": false,
            "schema": {
              "type": "string",
              "format": "date",
              "example": "2021-12-10"
            }
          },
          {
            "name": "to",
            "in": "query",
            "description": "**Required** only when the `from` parameter is specified. The end time and date in `yyyy-mm-dd` or ISO 8601 format, which is either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`.",
            "required": false,
            "schema": {
              "type": "string",
              "format": "date",
              "example": "2021-12-10"
            }
          },
          {
            "name": "country_iso_codes",
            "in": "query",
            "description": "Two-letter ISO country [code](/docs/api/references/abbreviations/#countries).",
            "required": false,
            "schema": {
              "type": "array",
              "example": [
                "US"
              ],
              "items": {
                "type": "string",
                "example": "CN"
              }
            }
          },
          {
            "name": "direction",
            "in": "query",
            "description": "The engagement's direction.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "inbound",
              "enum": [
                "inbound",
                "outbound"
              ]
            }
          },
          {
            "name": "user_id",
            "in": "query",
            "description": "The agent's ID.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "ukAAkZKfROKMSw1bj_RDFQ"
            }
          },
          {
            "name": "engagement_id",
            "in": "query",
            "description": "The engagement's ID.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "3XilEfOvQEKRIWMWhX1jDg"
            }
          },
          {
            "name": "consumer_number",
            "in": "query",
            "description": "The consumer's phone number.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "+12059300920"
            }
          },
          {
            "name": "accountId",
            "in": "path",
            "description": "Unique identifier of the account.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "TQTvjT52Tmi_wrhASNNOEw"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "**HTTP Status Code:** `200`   \n \nVariables list returned",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "next_page_token": {
                      "type": "string",
                      "description": "Use the next page token to paginate through large result sets. A next page token returns when the set of available results exceeds the current page size. This token's expiration period is 15 minutes.",
                      "example": "R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42"
                    },
                    "page_size": {
                      "maximum": 100,
                      "type": "integer",
                      "description": "The number of records returned in a single API call.",
                      "example": 30,
                      "default": 10
                    },
                    "total_records": {
                      "type": "integer",
                      "description": "The total number of all the records available across pages.",
                      "example": 1
                    },
                    "from": {
                      "type": "string",
                      "description": "The start time and date in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'`, or `yyyy-MM-dd'T'HH:mm:ss'TZD'`. The defined date range should be a month, as the response only includes one month's worth of data. If no start date is specified, return data from the past 24 hours.",
                      "format": "date-time",
                      "example": "2023-01-01T08:00:00Z"
                    },
                    "to": {
                      "type": "string",
                      "description": "**Required** only when the `from` parameter is specified. The end time and date in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`, the same format as the `from` parameter.",
                      "format": "date-time",
                      "example": "2023-01-01T09:00:00Z"
                    },
                    "sms": {
                      "type": "array",
                      "description": "Information about the SMS.",
                      "items": {
                        "type": "object",
                        "properties": {
                          "direction": {
                            "type": "string",
                            "description": "The engagement's direction. `inbound` | `outbound`.",
                            "example": "inbound",
                            "enum": [
                              "inbound",
                              "outbound"
                            ]
                          },
                          "engagement_id": {
                            "type": "string",
                            "description": "The engagement's ID",
                            "example": "3XilEfOvQEKRIWMWhX1jDg"
                          },
                          "contact_center_number": {
                            "type": "string",
                            "description": "The agent's number. An agent used a flow or queue number and did not really have its own number.",
                            "example": "+18108001001"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The date and time when the engagement started in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`.",
                            "format": "date-time",
                            "example": "2023-01-01T08:00:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The date and time when the engagement ended in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`.",
                            "format": "date-time",
                            "example": "2023-01-01T09:00:00Z"
                          },
                          "consumer_number": {
                            "type": "string",
                            "description": "The consumer's phone number.",
                            "example": "+12059300920"
                          },
                          "consumer_display_name": {
                            "type": "string",
                            "description": "The consumer's name.",
                            "example": "Tester"
                          },
                          "queues": {
                            "type": "array",
                            "description": "Information about the engagement's queues.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "queue_id": {
                                  "type": "string",
                                  "description": "The queue's ID.",
                                  "example": "d95avl1eRJ-H162PZUJ-qg",
                                  "deprecated": true
                                },
                                "cc_queue_id": {
                                  "type": "string",
                                  "description": "The Contact Center queue's ID.",
                                  "example": "d95avl1eRJ-H162PZUJ-qg"
                                },
                                "queue_name": {
                                  "type": "string",
                                  "description": "The queue's name.",
                                  "example": "agentQueue"
                                }
                              },
                              "description": "Information about the queue."
                            }
                          },
                          "agents": {
                            "type": "array",
                            "description": "Information about the engagement's agents.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "user_id": {
                                  "type": "string",
                                  "description": "The agent's ID.",
                                  "example": "ukAAkZKfROKMSw1bj_RDFQ"
                                },
                                "display_name": {
                                  "type": "string",
                                  "description": "The agent's name.",
                                  "example": "Jilly"
                                }
                              },
                              "description": "Information about the queue."
                            }
                          },
                          "country_name": {
                            "type": "string",
                            "description": "The country name.",
                            "example": "China"
                          },
                          "sms_types": {
                            "type": "array",
                            "description": "The SMS's type.",
                            "items": {
                              "type": "string",
                              "example": "sms",
                              "enum": [
                                "sms",
                                "mms"
                              ]
                            }
                          },
                          "total_sent_messages": {
                            "type": "integer",
                            "description": "The total number of sent messages.",
                            "format": "int64",
                            "example": 0
                          },
                          "total_received_messages": {
                            "type": "integer",
                            "description": "The total number of received messages.",
                            "format": "int64",
                            "example": 0
                          },
                          "total_sent_files": {
                            "type": "integer",
                            "description": "The total number of sent files.",
                            "format": "int64",
                            "example": 0
                          },
                          "total_received_files": {
                            "type": "integer",
                            "description": "The total number of received files.",
                            "format": "int64",
                            "example": 0
                          }
                        },
                        "description": "Information about the SMS."
                      }
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `2901` <br>\n Engagement does not exist: $engagementId. <br>\n**Error Code:** `1001` <br>\n User does not exist: $userId. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [],
          "x-macro-scopes": [
            "contact_center_sms:master"
          ],
          "x-granular-scopes": [
            "contact_center:read:sms_log:master"
          ]
        }
      }
    },
    "/accounts/{accountId}/contact_center/voice_calls": {
      "get": {
        "tags": [
          "Logs"
        ],
        "summary": "List voice call logs",
        "description": "Return a list of voice call engagement logs. Engagement data is available via API once it is completed. **Note** These fields - `agents.user_ip_address`, `agents.user_device_mac_address`, `agents.user_registered_sip_zone`, `agents.sip_zone_ip_address`, `sbcInfos.session_border_controller_name`, `sbcInfos.session_border_controller_ip_address`, `sbcInfos.consumer_zoom_trunk_name` and `sbcInfos.consumer_sbc_ip_address` - are only available to accounts which have enabled an internal flag to return these additional fields. To enable the feature flag, contact Zoom Support.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `contact_center_voice_call:master`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `contact_center:read:voice_call_log:master`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `HEAVY`",
        "operationId": "listVoiceCall",
        "parameters": [
          {
            "name": "from",
            "in": "query",
            "description": "The start time and date in either `yyyy-MM-dd`, `yyyy-MM-dd'T'HH:mm:ss'Z'`, or `yyyy-MM-dd'T'HH:mm:ss\u00b1HH:mm` format. The defined date range should be a month, as the response only includes one month's worth of data. If no start date is specified, return data from the past 24 hours.\n\n**Note** When using `yyyy-MM-dd'T'HH:mm:ss'Z'` format, the `'Z'` is treated as a literal character. The timezone offset from `yyyy-MM-dd'T'HH:mm:ss\u00b1HH:mm` format is used for response datetime formatting.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "2021-12-10"
            }
          },
          {
            "name": "to",
            "in": "query",
            "description": "**Required** only when the `from` parameter is specified. The end time and date in `yyyy-MM-dd`, `yyyy-MM-dd'T'HH:mm:ss'Z'`, or `yyyy-MM-dd'T'HH:mm:ss\u00b1HH:mm` format, matching the format of the `from` parameter.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "2021-12-10"
            }
          },
          {
            "name": "types",
            "in": "query",
            "description": "The call's type.",
            "required": false,
            "schema": {
              "type": "array",
              "items": {
                "type": "string",
                "example": "external_call",
                "enum": [
                  "external_call",
                  "external_conference_call",
                  "external_callback_call",
                  "internal_call",
                  "internal_conference_call",
                  "internal_callback_call"
                ]
              }
            }
          },
          {
            "name": "distribution",
            "in": "query",
            "description": "The engagement's distribution.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "acd",
              "enum": [
                "acd",
                "non_acd"
              ]
            }
          },
          {
            "name": "consumer_number",
            "in": "query",
            "description": "The consumer's phone number.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "+12059300920"
            }
          },
          {
            "name": "direction",
            "in": "query",
            "description": "The engagement's direction.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "inbound",
              "enum": [
                "inbound",
                "outbound"
              ]
            }
          },
          {
            "name": "user_ids",
            "in": "query",
            "description": "A comma-separated list of agents' user IDs to query, up to 20 agent IDs.",
            "required": false,
            "schema": {
              "type": "array",
              "items": {
                "type": "string",
                "example": "t9PXwOsSQ2qXp7YSPDrALQ"
              }
            }
          },
          {
            "name": "engagement_id",
            "in": "query",
            "description": "The engagement's ID.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "3XilEfOvQEKRIWMWhX1jDg"
            }
          },
          {
            "name": "queue_ids",
            "in": "query",
            "description": "A comma-separated list of queue IDs to query, up to 20 queue IDs. If user provide `none`, filter by **No queue used** in the web UI.",
            "required": false,
            "deprecated": true,
            "schema": {
              "type": "array",
              "items": {
                "type": "string",
                "example": "Ql3qxUWXSHes4PmcOmzyOg"
              }
            }
          },
          {
            "name": "cc_queue_ids",
            "in": "query",
            "description": "A comma-separated list of Contact Center queue IDs to query, up to 20 queue IDs. If a user provides `none`, filter by **No queue used** in the web UI.",
            "required": false,
            "schema": {
              "type": "array",
              "items": {
                "type": "string",
                "example": "Ql3qxUWXSHes4PmcOmzyOg"
              }
            }
          },
          {
            "name": "country_iso_codes",
            "in": "query",
            "description": "Two-letter ISO country [code](/docs/api/references/abbreviations/#countries).",
            "required": false,
            "schema": {
              "type": "array",
              "items": {
                "type": "string",
                "example": "CN"
              }
            }
          },
          {
            "name": "number_types",
            "in": "query",
            "description": "The number's type.",
            "required": false,
            "schema": {
              "type": "array",
              "items": {
                "type": "string",
                "example": "toll_free_number",
                "enum": [
                  "toll_free_number",
                  "virtual_service_number",
                  "byoc_number"
                ]
              }
            }
          },
          {
            "name": "region_ids",
            "in": "query",
            "description": "A comma-separated list of region IDs to query, up to 20 region IDs.",
            "required": false,
            "schema": {
              "maxItems": 20,
              "uniqueItems": true,
              "type": "array",
              "example": [
                "3XilEfOvQEKRIWMWhX1jDg"
              ],
              "items": {
                "type": "string"
              }
            }
          },
          {
            "name": "region_names",
            "in": "query",
            "description": "A comma-separated list of region names to query, up to 20 region Names.",
            "required": false,
            "schema": {
              "maxItems": 20,
              "uniqueItems": true,
              "type": "array",
              "example": [
                "Main Region"
              ],
              "items": {
                "type": "string"
              }
            }
          },
          {
            "name": "team_ids",
            "in": "query",
            "description": "A comma-separated list of team IDs to query, up to 20 team IDs.",
            "required": false,
            "schema": {
              "maxItems": 20,
              "uniqueItems": true,
              "type": "array",
              "example": [
                "3XilEfOvQEKRIWMWhX1jDg"
              ],
              "items": {
                "type": "string"
              }
            }
          },
          {
            "name": "team_names",
            "in": "query",
            "description": "A comma-separated list of team names to query, up to 20 team names.",
            "required": false,
            "schema": {
              "maxItems": 20,
              "uniqueItems": true,
              "type": "array",
              "example": [
                "My Team"
              ],
              "items": {
                "type": "string"
              }
            }
          },
          {
            "name": "sub_types",
            "in": "query",
            "description": "A comma-separated list of detailed call leg types.\n**Allowed subtypes**\n* `external_warm_conference_call`\n* `external_direct_conference_call`\n* `external_warm_transfer_call`\n* `external_direct_transfer_call`\n* `internal_warm_conference_call`\n* `internal_direct_conference_call`\n* `internal_warm_transfer_call`\n* `internal_direct_transfer_call`\n* `flow_initiated_outgoing_call` - flow outgoing",
            "required": false,
            "schema": {
              "type": "array",
              "example": [
                "external_direct_transfer_call"
              ],
              "items": {
                "type": "string",
                "example": "external_direct_transfer_call",
                "enum": [
                  "external_warm_conference_call",
                  "external_direct_conference_call",
                  "external_warm_transfer_call",
                  "external_direct_transfer_call",
                  "internal_warm_conference_call",
                  "internal_direct_conference_call",
                  "internal_warm_transfer_call",
                  "internal_direct_transfer_call",
                  "flow_initiated_outgoing_call"
                ]
              }
            }
          },
          {
            "name": "view_type",
            "in": "query",
            "description": "By default, this API returns the summary in List voice logs.\nIf `detail` is selected, it returns all the call logs' additional details. ",
            "required": false,
            "schema": {
              "type": "string",
              "example": "summary",
              "default": "summary",
              "enum": [
                "summary",
                "detail"
              ]
            }
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "Use the next page token to paginate through large result sets. A next page token is returned whenever the set of available results exceeds the current page size. This token's expiration period is 15 minutes.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42"
            }
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of items returned per page.",
            "required": false,
            "schema": {
              "maximum": 100,
              "type": "integer",
              "format": "int32",
              "example": 10,
              "default": 10
            }
          },
          {
            "name": "accountId",
            "in": "path",
            "description": "Unique identifier of the account.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "TQTvjT52Tmi_wrhASNNOEw"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "**HTTP Status Code:** `200`   Variables list returned.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "next_page_token": {
                      "maxLength": 100,
                      "type": "string",
                      "description": "Use the next page token to paginate through large result sets. A next page token is returned whenever the set of available results exceeds the current page size. This token's expiration period is 15 minutes.",
                      "example": "R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42"
                    },
                    "page_size": {
                      "maximum": 100,
                      "minimum": 1,
                      "type": "integer",
                      "description": "The number of records returned within a single API call.",
                      "example": 10,
                      "default": 10
                    },
                    "total_records": {
                      "minimum": 0,
                      "type": "integer",
                      "description": "The total number of all the records available across pages.",
                      "example": 10
                    },
                    "from": {
                      "type": "string",
                      "description": "The start time and date in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`. The defined date range should be a month, as the response only includes one month's worth of data. If no start date is specified, return data from the past 24 hours.",
                      "format": "date-time",
                      "example": "2023-01-01T08:00:00Z"
                    },
                    "to": {
                      "type": "string",
                      "description": "**Required** only when the `from` parameter is specified. The end time and date in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`, the same format as the `from` parameter.",
                      "format": "date-time",
                      "example": "2023-01-01T09:00:00Z"
                    },
                    "voice_calls": {
                      "type": "array",
                      "description": "Information about the voice call.",
                      "items": {
                        "type": "object",
                        "properties": {
                          "engagement_id": {
                            "type": "string",
                            "description": "The engagement's ID",
                            "example": "3XilEfOvQEKRIWMWhX1jDg"
                          },
                          "direction": {
                            "type": "string",
                            "description": "The engagement's direction.",
                            "example": "inbound",
                            "enum": [
                              "inbound",
                              "outbound"
                            ]
                          },
                          "call_legs": {
                            "type": "array",
                            "description": "The direction of the engagement call leg. `inbound` | `outbound`.",
                            "items": {
                              "type": "string",
                              "example": "inbound",
                              "enum": [
                                "inbound",
                                "outbound"
                              ]
                            }
                          },
                          "distributions": {
                            "type": "array",
                            "description": "The engagement's distribution.",
                            "items": {
                              "type": "string",
                              "example": "acd",
                              "enum": [
                                "acd",
                                "non_acd"
                              ]
                            }
                          },
                          "types": {
                            "type": "array",
                            "description": "The call's type.",
                            "items": {
                              "type": "string",
                              "example": "external_call",
                              "enum": [
                                "external_call",
                                "external_conference_call",
                                "external_callback_call",
                                "internal_call",
                                "internal_conference_call",
                                "internal_callback_call"
                              ]
                            }
                          },
                          "sub_types": {
                            "type": "array",
                            "description": "Specifies the detailed call leg type, such as a transfer or conference call.",
                            "items": {
                              "type": "string",
                              "example": "external_direct_transfer_call",
                              "enum": [
                                "external_warm_conference_call",
                                "external_direct_conference_call",
                                "external_warm_transfer_call",
                                "external_direct_transfer_call",
                                "internal_warm_conference_call",
                                "internal_direct_conference_call",
                                "internal_warm_transfer_call",
                                "internal_direct_transfer_call",
                                "flow_initiated_outgoing_call"
                              ]
                            }
                          },
                          "caller_number": {
                            "type": "string",
                            "description": "The caller's phone number.",
                            "example": "+18108001001"
                          },
                          "caller_number_type": {
                            "type": "string",
                            "description": "The caller's number type.",
                            "example": "toll_free_number",
                            "enum": [
                              "toll_free_number",
                              "virtual_service_number",
                              "byoc_number"
                            ]
                          },
                          "callee_number": {
                            "type": "string",
                            "description": "The callee's phone number.",
                            "example": "+12055437350"
                          },
                          "callee_number_type": {
                            "type": "string",
                            "description": "The callee's number type.",
                            "example": "toll_free_number",
                            "enum": [
                              "toll_free_number",
                              "virtual_service_number",
                              "byoc_number"
                            ]
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The date and time when the engagement started in `yyyy-MM-dd'T'HH:mm:ss\u00b1HH:mm` format. The timezone offset matches the offset provided in the `from` input parameter, or defaults to UTC (+00:00) if no offset was provided.",
                            "example": "2023-01-01T00:00:00-08:00"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The date and time when the engagement ended, in `yyyy-MM-dd'T'HH:mm:ss\u00b1HH:mm` format. The timezone offset is derived from the `from` input parameter if it includes a timezone offset (e.g., `-08:00`), otherwise it defaults to `+00:00` (UTC).",
                            "example": "2023-01-01T01:00:00-08:00"
                          },
                          "total_duration": {
                            "type": "integer",
                            "description": "The engagement's total duration, in seconds.",
                            "format": "int64",
                            "example": 10
                          },
                          "consumer_number": {
                            "type": "string",
                            "description": "The consumer's phone number.",
                            "example": "+12059300920"
                          },
                          "consumer_display_name": {
                            "type": "string",
                            "description": "The consumer's name.",
                            "example": "Tester"
                          },
                          "queues": {
                            "type": "array",
                            "description": "Information about the engagement's queues.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "queue_id": {
                                  "type": "string",
                                  "description": "The queue's ID.",
                                  "example": "d95avl1eRJ-H162PZUJ-qg",
                                  "deprecated": true
                                },
                                "cc_queue_id": {
                                  "type": "string",
                                  "description": "The Contact Center queue's ID.",
                                  "example": "d95avl1eRJ-H162PZUJ-qg"
                                },
                                "queue_name": {
                                  "type": "string",
                                  "description": "The queue's name.",
                                  "example": "agentQueue"
                                }
                              },
                              "description": "Information about the queue."
                            }
                          },
                          "agents": {
                            "type": "array",
                            "description": "Information about the engagement's agents.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "user_id": {
                                  "type": "string",
                                  "description": "The agent's ID.",
                                  "example": "ukAAkZKfROKMSw1bj_RDFQ"
                                },
                                "display_name": {
                                  "type": "string",
                                  "description": "The agent's name.",
                                  "example": "Jilly"
                                },
                                "user_email": {
                                  "type": "string",
                                  "description": "The agent's email.",
                                  "example": "jakie@zoom.us"
                                },
                                "user_ip_address": {
                                  "type": "string",
                                  "description": "IP address of the agent's device.",
                                  "format": "ipv4",
                                  "example": "38.145.73.4"
                                },
                                "user_device_mac_address": {
                                  "pattern": "^([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2})$",
                                  "type": "string",
                                  "description": "MAC address of the agent's device.",
                                  "example": "05:FA:15:25:EE:FF"
                                },
                                "user_registered_sip_zone": {
                                  "type": "string",
                                  "description": "The SIP zone where the agent registered.",
                                  "example": "ams1sbc"
                                },
                                "sip_zone_ip_address": {
                                  "type": "string",
                                  "description": "The SIP zome's IP address.",
                                  "format": "ipv4",
                                  "example": "38.145.73.4"
                                },
                                "user_region": {
                                  "type": "string",
                                  "description": "The agent's region name",
                                  "example": "Main Region"
                                },
                                "team_ids": {
                                  "type": "array",
                                  "description": "The agent's team IDs",
                                  "items": {
                                    "type": "string",
                                    "description": "agent team ID list",
                                    "example": "[\"3XilEfOvQEKRIWMWhX1jDg\"]"
                                  }
                                },
                                "team_names": {
                                  "type": "array",
                                  "description": "The agent's team names.",
                                  "items": {
                                    "type": "string",
                                    "description": "agent team name list.",
                                    "example": "[\"My team\"]"
                                  }
                                }
                              },
                              "description": "Information about the queue."
                            }
                          },
                          "flows": {
                            "type": "array",
                            "description": "Information about the engagement's flows.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "flow_id": {
                                  "type": "string",
                                  "description": "The flow's ID.",
                                  "example": "zeYjXoDOS_eV1QmTpj63PQ"
                                },
                                "flow_name": {
                                  "type": "string",
                                  "description": "The flow's name.",
                                  "example": "Demo"
                                }
                              },
                              "description": "Information about the flow."
                            }
                          },
                          "country_name": {
                            "type": "string",
                            "description": "The country name.",
                            "example": "China"
                          },
                          "recorded": {
                            "type": "boolean",
                            "description": "Flag to determine whether to recorded.",
                            "example": false
                          },
                          "monitored": {
                            "type": "boolean",
                            "description": "Flag to determine whether to monitored.",
                            "example": false
                          },
                          "inbox": {
                            "type": "boolean",
                            "description": "Flag to determine whether a message is in the box.",
                            "example": false
                          },
                          "result": {
                            "type": "string",
                            "description": "The engagement's results.",
                            "example": "completed",
                            "enum": [
                              "completed",
                              "short_abandoned",
                              "long_abandoned",
                              "hold_abandoned",
                              "long_calls",
                              "short_calls",
                              "hang_up_calls",
                              "overflowed_to_disconnect",
                              "overflowed_to_inbox",
                              "overflowed",
                              "abandon_quit",
                              "auto_closed",
                              "contained",
                              "missed",
                              "declined",
                              "callback_abandoned_by_consumer"
                            ]
                          },
                          "caller_id": {
                            "type": "string",
                            "description": "The caller's number.",
                            "example": "+12058945728"
                          },
                          "calling_party": {
                            "type": "string",
                            "description": "The calling's party.",
                            "example": "consumer",
                            "enum": [
                              "consumer",
                              "agent"
                            ]
                          },
                          "charge": {
                            "type": "string",
                            "description": "The bill's charge.",
                            "example": "$0.019"
                          },
                          "rates": {
                            "type": "array",
                            "description": "The price for each sent message.",
                            "items": {
                              "type": "string",
                              "example": "$0.019"
                            }
                          },
                          "charge_type": {
                            "type": "string",
                            "description": "The charge type.",
                            "example": "per_minute",
                            "enum": [
                              "per_minute"
                            ]
                          },
                          "session_border_controller_list": {
                            "type": "array",
                            "description": "Information about the session border controllers (SBCs) involved in the call.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "session_border_controller_name": {
                                  "type": "string",
                                  "description": "The session border controller's name.",
                                  "example": "ams1sbc"
                                },
                                "session_border_controller_ip_address": {
                                  "type": "string",
                                  "description": "The SBC's IP address.",
                                  "example": "38.145.73.20"
                                },
                                "consumer_zoom_trunk_name": {
                                  "type": "string",
                                  "description": "The name of the trunk between the customer SBC and Zoom SBC.",
                                  "example": "C10010142296A06"
                                },
                                "consumer_sbc_ip_address": {
                                  "type": "string",
                                  "description": "The customer's SBC IP address.",
                                  "example": "39.15.60.145"
                                }
                              },
                              "description": "Information about the session border controller."
                            }
                          },
                          "voice_call_details": {
                            "type": "array",
                            "description": "All related call logs.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "engagement_id": {
                                  "type": "string",
                                  "description": "The engagement's ID.",
                                  "example": "3XilEfOvQEKRIWMWhX1jDg"
                                },
                                "call_leg": {
                                  "type": "string",
                                  "description": "The direction of the engagement call log.",
                                  "example": "inbound",
                                  "enum": [
                                    "inbound",
                                    "outbound"
                                  ]
                                },
                                "distribution": {
                                  "type": "string",
                                  "description": "The call log's distribution.",
                                  "example": "acd",
                                  "enum": [
                                    "acd",
                                    "non_acd"
                                  ]
                                },
                                "type": {
                                  "type": "string",
                                  "description": "The call's type.",
                                  "example": "external_call",
                                  "enum": [
                                    "external_call",
                                    "external_conference_call",
                                    "external_callback_call",
                                    "internal_call",
                                    "internal_conference_call",
                                    "internal_callback_call"
                                  ]
                                },
                                "caller_number": {
                                  "type": "string",
                                  "description": "The caller's phone number.",
                                  "example": "+18108001001"
                                },
                                "caller_number_type": {
                                  "type": "string",
                                  "description": "The caller's number type.",
                                  "example": "toll_free_number",
                                  "enum": [
                                    "toll_free_number",
                                    "virtual_service_number",
                                    "byoc_number"
                                  ]
                                },
                                "callee_number": {
                                  "type": "string",
                                  "description": "The callee's phone number.",
                                  "example": "+12055437350"
                                },
                                "callee_number_type": {
                                  "type": "string",
                                  "description": "The callee's number type.",
                                  "example": "toll_free_number",
                                  "enum": [
                                    "toll_free_number",
                                    "virtual_service_number",
                                    "byoc_number"
                                  ]
                                },
                                "start_time": {
                                  "type": "string",
                                  "description": "The date and time when the call started, in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`.",
                                  "example": "2023-01-01T00:00:00-08:00"
                                },
                                "end_time": {
                                  "type": "string",
                                  "description": "The date and time when the call ended, in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`.",
                                  "example": "2023-01-01T01:00:00-08:00"
                                },
                                "total_duration": {
                                  "type": "integer",
                                  "description": "The call log's total duration, in seconds.",
                                  "format": "int64",
                                  "example": 10
                                },
                                "consumer_number": {
                                  "type": "string",
                                  "description": "The consumer's phone number.",
                                  "example": "+12059300920"
                                },
                                "consumer_display_name": {
                                  "type": "string",
                                  "description": "The consumer's name.",
                                  "example": "Tester"
                                },
                                "queue": {
                                  "type": "object",
                                  "properties": {
                                    "cc_queue_id": {
                                      "type": "string",
                                      "description": "The Contact Center queue's ID.",
                                      "example": "d95avl1eRJ-H162PZUJ-qg"
                                    },
                                    "queue_name": {
                                      "type": "string",
                                      "description": "The queue's name.",
                                      "example": "agentQueue"
                                    }
                                  },
                                  "description": "Information about the queue."
                                },
                                "agents": {
                                  "type": "array",
                                  "description": "Information about the engagement's agents.",
                                  "items": {
                                    "type": "object",
                                    "properties": {
                                      "user_id": {
                                        "type": "string",
                                        "description": "The agent's ID.",
                                        "example": "ukAAkZKfROKMSw1bj_RDFQ"
                                      },
                                      "display_name": {
                                        "type": "string",
                                        "description": "The agent's name.",
                                        "example": "Jilly"
                                      },
                                      "user_email": {
                                        "type": "string",
                                        "description": "The agent's email.",
                                        "example": "jakie@zoom.us"
                                      },
                                      "user_ip_address": {
                                        "type": "string",
                                        "description": "The agent's device's IP address.",
                                        "format": "ipv4",
                                        "example": "38.145.73.4"
                                      },
                                      "user_device_mac_address": {
                                        "pattern": "^([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2})$",
                                        "type": "string",
                                        "description": "The agent's device's MAC address.",
                                        "example": "05:FA:15:25:EE:FF"
                                      },
                                      "user_registered_sip_zone": {
                                        "type": "string",
                                        "description": "The SIP zone where the agent registered.",
                                        "example": "ams1sbc"
                                      },
                                      "sip_zone_ip_address": {
                                        "type": "string",
                                        "description": "The SIP zone's IP address.",
                                        "format": "ipv4",
                                        "example": "38.145.73.4"
                                      },
                                      "user_region": {
                                        "type": "string",
                                        "description": "The agent's region name.",
                                        "example": "Main Region"
                                      },
                                      "team_ids": {
                                        "type": "array",
                                        "description": "The agent's team IDs",
                                        "items": {
                                          "type": "string",
                                          "description": "Agent team ID list.",
                                          "example": "3XilEfOvQEKRIWMWhX1jDg"
                                        }
                                      },
                                      "team_names": {
                                        "type": "array",
                                        "description": "The agent's team names.",
                                        "items": {
                                          "type": "string",
                                          "description": "Agent team name list.",
                                          "example": "My team"
                                        }
                                      }
                                    },
                                    "description": "Information about the queue."
                                  }
                                },
                                "flow": {
                                  "type": "object",
                                  "properties": {
                                    "flow_id": {
                                      "type": "string",
                                      "description": "The flow's ID.",
                                      "example": "zeYjXoDOS_eV1QmTpj63PQ"
                                    },
                                    "flow_name": {
                                      "type": "string",
                                      "description": "The flow's name.",
                                      "example": "Demo"
                                    }
                                  },
                                  "description": "Information about the flow."
                                },
                                "country_name": {
                                  "type": "string",
                                  "description": "The country name.",
                                  "example": "China"
                                },
                                "recorded": {
                                  "type": "boolean",
                                  "description": "Whether to record.",
                                  "example": false
                                },
                                "monitored": {
                                  "type": "boolean",
                                  "description": "Whether to monitor.",
                                  "example": false
                                },
                                "inbox": {
                                  "type": "boolean",
                                  "description": "Whether a message is in the box.",
                                  "example": false
                                },
                                "result": {
                                  "type": "string",
                                  "description": "The engagement's results.",
                                  "example": "completed",
                                  "enum": [
                                    "completed",
                                    "short_abandoned",
                                    "long_abandoned",
                                    "hold_abandoned",
                                    "long_calls",
                                    "short_calls",
                                    "hang_up_calls",
                                    "overflowed_to_disconnect",
                                    "overflowed_to_inbox",
                                    "overflowed",
                                    "abandon_quit",
                                    "auto_closed",
                                    "contained",
                                    "missed",
                                    "declined",
                                    "callback_abandoned_by_consumer"
                                  ]
                                },
                                "caller_id": {
                                  "type": "string",
                                  "description": "The caller's number.",
                                  "example": "+12058945728"
                                },
                                "calling_party": {
                                  "type": "string",
                                  "description": "The calling's party.",
                                  "example": "consumer",
                                  "enum": [
                                    "consumer",
                                    "agent"
                                  ]
                                },
                                "charge": {
                                  "type": "string",
                                  "description": "The bill's charge.",
                                  "example": "$0.019"
                                },
                                "rate": {
                                  "type": "string",
                                  "description": "The price for sent message.",
                                  "example": "$0.019"
                                },
                                "charge_type": {
                                  "type": "string",
                                  "description": "The charge type.",
                                  "example": "per_minute",
                                  "enum": [
                                    "per_minute"
                                  ]
                                }
                              },
                              "description": "Related call leg details"
                            }
                          }
                        },
                        "description": "Information about the voice call."
                      }
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `300` <br>\n The time and date must be in `yyyy-mm-dd`` or ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`. <br>\n**Error Code:** `300` <br>\n The requested report exceeds the 1-month limit. <br>\n**Error Code:** `300` <br>\n Invalid field. <br>\n"
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `1001` <br>\n User does not exist: {userId}. <br>\n**Error Code:** `1201` <br>\n Queue does not exist: {queueId}. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [],
          "x-macro-scopes": [
            "contact_center_voice_call:master"
          ],
          "x-granular-scopes": [
            "contact_center:read:voice_call_log:master"
          ]
        }
      }
    }
  },
  "components": {
    "securitySchemes": {
      "openapi_authorization": {
        "type": "http",
        "scheme": "bearer"
      }
    }
  }
}
```

## Downloads

### master.json
```json
{
  "openapi": "3.0.0",
  "info": {
    "title": "Contact Center",
    "description": "The Contact Center APIs allow developers to interface with [Contact Center](https://developers.zoom.us/docs/contact-center/) features programmatically.",
    "termsOfService": "https://zoom.us/docs/en-us/zoom_api_license_and_tou.html",
    "contact": {
      "name": "Zoom Developers",
      "url": "https://developer.zoom.us/"
    },
    "version": "2"
  },
  "externalDocs": {
    "description": "Find out more about Swagger",
    "url": "https://swagger.io"
  },
  "servers": [
    {
      "url": "https://api.zoom.us/v2"
    }
  ],
  "paths": {
    "/accounts/{accountId}/contact_center/sms": {
      "get": {
        "tags": [
          "Logs"
        ],
        "summary": "List SMS logs",
        "description": "Returns a list of SMS engagement logs. Engagement data are available via API once they are completed.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `contact_center_sms:master`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `contact_center:read:sms_log:master`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `HEAVY`",
        "operationId": "listSMS",
        "parameters": [
          {
            "name": "next_page_token",
            "in": "query",
            "description": "Use the next page token to paginate through large result sets. A next page token is returned whenever the set of available results exceeds the current page size. This token's expiration period is 15 minutes.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42"
            }
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of items returned per page.",
            "required": false,
            "schema": {
              "maximum": 100,
              "type": "integer",
              "format": "int32",
              "example": 10,
              "default": 10
            }
          },
          {
            "name": "from",
            "in": "query",
            "description": "The start time and date in `yyyy-mm-dd` or ISO 8601 format, which is either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`. The defined date range should be a month, as the response only includes one month's worth of data. If no start date is specified, return data from the past 24 hours.",
            "required": false,
            "schema": {
              "type": "string",
              "format": "date",
              "example": "2021-12-10"
            }
          },
          {
            "name": "to",
            "in": "query",
            "description": "**Required** only when the `from` parameter is specified. The end time and date in `yyyy-mm-dd` or ISO 8601 format, which is either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`.",
            "required": false,
            "schema": {
              "type": "string",
              "format": "date",
              "example": "2021-12-10"
            }
          },
          {
            "name": "country_iso_codes",
            "in": "query",
            "description": "Two-letter ISO country [code](/docs/api/references/abbreviations/#countries).",
            "required": false,
            "schema": {
              "type": "array",
              "example": [
                "US"
              ],
              "items": {
                "type": "string",
                "example": "CN"
              }
            }
          },
          {
            "name": "direction",
            "in": "query",
            "description": "The engagement's direction.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "inbound",
              "enum": [
                "inbound",
                "outbound"
              ]
            }
          },
          {
            "name": "user_id",
            "in": "query",
            "description": "The agent's ID.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "ukAAkZKfROKMSw1bj_RDFQ"
            }
          },
          {
            "name": "engagement_id",
            "in": "query",
            "description": "The engagement's ID.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "3XilEfOvQEKRIWMWhX1jDg"
            }
          },
          {
            "name": "consumer_number",
            "in": "query",
            "description": "The consumer's phone number.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "+12059300920"
            }
          },
          {
            "name": "accountId",
            "in": "path",
            "description": "Unique identifier of the account.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "TQTvjT52Tmi_wrhASNNOEw"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "**HTTP Status Code:** `200`   \n \nVariables list returned",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "next_page_token": {
                      "type": "string",
                      "description": "Use the next page token to paginate through large result sets. A next page token returns when the set of available results exceeds the current page size. This token's expiration period is 15 minutes.",
                      "example": "R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42"
                    },
                    "page_size": {
                      "maximum": 100,
                      "type": "integer",
                      "description": "The number of records returned in a single API call.",
                      "example": 30,
                      "default": 10
                    },
                    "total_records": {
                      "type": "integer",
                      "description": "The total number of all the records available across pages.",
                      "example": 1
                    },
                    "from": {
                      "type": "string",
                      "description": "The start time and date in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'`, or `yyyy-MM-dd'T'HH:mm:ss'TZD'`. The defined date range should be a month, as the response only includes one month's worth of data. If no start date is specified, return data from the past 24 hours.",
                      "format": "date-time",
                      "example": "2023-01-01T08:00:00Z"
                    },
                    "to": {
                      "type": "string",
                      "description": "**Required** only when the `from` parameter is specified. The end time and date in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`, the same format as the `from` parameter.",
                      "format": "date-time",
                      "example": "2023-01-01T09:00:00Z"
                    },
                    "sms": {
                      "type": "array",
                      "description": "Information about the SMS.",
                      "items": {
                        "type": "object",
                        "properties": {
                          "direction": {
                            "type": "string",
                            "description": "The engagement's direction. `inbound` | `outbound`.",
                            "example": "inbound",
                            "enum": [
                              "inbound",
                              "outbound"
                            ]
                          },
                          "engagement_id": {
                            "type": "string",
                            "description": "The engagement's ID",
                            "example": "3XilEfOvQEKRIWMWhX1jDg"
                          },
                          "contact_center_number": {
                            "type": "string",
                            "description": "The agent's number. An agent used a flow or queue number and did not really have its own number.",
                            "example": "+18108001001"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The date and time when the engagement started in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`.",
                            "format": "date-time",
                            "example": "2023-01-01T08:00:00Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The date and time when the engagement ended in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`.",
                            "format": "date-time",
                            "example": "2023-01-01T09:00:00Z"
                          },
                          "consumer_number": {
                            "type": "string",
                            "description": "The consumer's phone number.",
                            "example": "+12059300920"
                          },
                          "consumer_display_name": {
                            "type": "string",
                            "description": "The consumer's name.",
                            "example": "Tester"
                          },
                          "queues": {
                            "type": "array",
                            "description": "Information about the engagement's queues.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "queue_id": {
                                  "type": "string",
                                  "description": "The queue's ID.",
                                  "example": "d95avl1eRJ-H162PZUJ-qg",
                                  "deprecated": true
                                },
                                "cc_queue_id": {
                                  "type": "string",
                                  "description": "The Contact Center queue's ID.",
                                  "example": "d95avl1eRJ-H162PZUJ-qg"
                                },
                                "queue_name": {
                                  "type": "string",
                                  "description": "The queue's name.",
                                  "example": "agentQueue"
                                }
                              },
                              "description": "Information about the queue."
                            }
                          },
                          "agents": {
                            "type": "array",
                            "description": "Information about the engagement's agents.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "user_id": {
                                  "type": "string",
                                  "description": "The agent's ID.",
                                  "example": "ukAAkZKfROKMSw1bj_RDFQ"
                                },
                                "display_name": {
                                  "type": "string",
                                  "description": "The agent's name.",
                                  "example": "Jilly"
                                }
                              },
                              "description": "Information about the queue."
                            }
                          },
                          "country_name": {
                            "type": "string",
                            "description": "The country name.",
                            "example": "China"
                          },
                          "sms_types": {
                            "type": "array",
                            "description": "The SMS's type.",
                            "items": {
                              "type": "string",
                              "example": "sms",
                              "enum": [
                                "sms",
                                "mms"
                              ]
                            }
                          },
                          "total_sent_messages": {
                            "type": "integer",
                            "description": "The total number of sent messages.",
                            "format": "int64",
                            "example": 0
                          },
                          "total_received_messages": {
                            "type": "integer",
                            "description": "The total number of received messages.",
                            "format": "int64",
                            "example": 0
                          },
                          "total_sent_files": {
                            "type": "integer",
                            "description": "The total number of sent files.",
                            "format": "int64",
                            "example": 0
                          },
                          "total_received_files": {
                            "type": "integer",
                            "description": "The total number of received files.",
                            "format": "int64",
                            "example": 0
                          }
                        },
                        "description": "Information about the SMS."
                      }
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `2901` <br>\n Engagement does not exist: $engagementId. <br>\n**Error Code:** `1001` <br>\n User does not exist: $userId. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "contact_center_sms:master",
              "contact_center:read:sms_log:master"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [],
          "x-macro-scopes": [
            "contact_center_sms:master"
          ],
          "x-granular-scopes": [
            "contact_center:read:sms_log:master"
          ]
        }
      }
    },
    "/accounts/{accountId}/contact_center/voice_calls": {
      "get": {
        "tags": [
          "Logs"
        ],
        "summary": "List voice call logs",
        "description": "Return a list of voice call engagement logs. Engagement data is available via API once it is completed. **Note** These fields - `agents.user_ip_address`, `agents.user_device_mac_address`, `agents.user_registered_sip_zone`, `agents.sip_zone_ip_address`, `sbcInfos.session_border_controller_name`, `sbcInfos.session_border_controller_ip_address`, `sbcInfos.consumer_zoom_trunk_name` and `sbcInfos.consumer_sbc_ip_address` - are only available to accounts which have enabled an internal flag to return these additional fields. To enable the feature flag, contact Zoom Support.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `contact_center_voice_call:master`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `contact_center:read:voice_call_log:master`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `HEAVY`",
        "operationId": "listVoiceCall",
        "parameters": [
          {
            "name": "from",
            "in": "query",
            "description": "The start time and date in either `yyyy-MM-dd`, `yyyy-MM-dd'T'HH:mm:ss'Z'`, or `yyyy-MM-dd'T'HH:mm:ss\u00b1HH:mm` format. The defined date range should be a month, as the response only includes one month's worth of data. If no start date is specified, return data from the past 24 hours.\n\n**Note** When using `yyyy-MM-dd'T'HH:mm:ss'Z'` format, the `'Z'` is treated as a literal character. The timezone offset from `yyyy-MM-dd'T'HH:mm:ss\u00b1HH:mm` format is used for response datetime formatting.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "2021-12-10"
            }
          },
          {
            "name": "to",
            "in": "query",
            "description": "**Required** only when the `from` parameter is specified. The end time and date in `yyyy-MM-dd`, `yyyy-MM-dd'T'HH:mm:ss'Z'`, or `yyyy-MM-dd'T'HH:mm:ss\u00b1HH:mm` format, matching the format of the `from` parameter.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "2021-12-10"
            }
          },
          {
            "name": "types",
            "in": "query",
            "description": "The call's type.",
            "required": false,
            "schema": {
              "type": "array",
              "items": {
                "type": "string",
                "example": "external_call",
                "enum": [
                  "external_call",
                  "external_conference_call",
                  "external_callback_call",
                  "internal_call",
                  "internal_conference_call",
                  "internal_callback_call"
                ]
              }
            }
          },
          {
            "name": "distribution",
            "in": "query",
            "description": "The engagement's distribution.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "acd",
              "enum": [
                "acd",
                "non_acd"
              ]
            }
          },
          {
            "name": "consumer_number",
            "in": "query",
            "description": "The consumer's phone number.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "+12059300920"
            }
          },
          {
            "name": "direction",
            "in": "query",
            "description": "The engagement's direction.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "inbound",
              "enum": [
                "inbound",
                "outbound"
              ]
            }
          },
          {
            "name": "user_ids",
            "in": "query",
            "description": "A comma-separated list of agents' user IDs to query, up to 20 agent IDs.",
            "required": false,
            "schema": {
              "type": "array",
              "items": {
                "type": "string",
                "example": "t9PXwOsSQ2qXp7YSPDrALQ"
              }
            }
          },
          {
            "name": "engagement_id",
            "in": "query",
            "description": "The engagement's ID.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "3XilEfOvQEKRIWMWhX1jDg"
            }
          },
          {
            "name": "queue_ids",
            "in": "query",
            "description": "A comma-separated list of queue IDs to query, up to 20 queue IDs. If user provide `none`, filter by **No queue used** in the web UI.",
            "required": false,
            "deprecated": true,
            "schema": {
              "type": "array",
              "items": {
                "type": "string",
                "example": "Ql3qxUWXSHes4PmcOmzyOg"
              }
            }
          },
          {
            "name": "cc_queue_ids",
            "in": "query",
            "description": "A comma-separated list of Contact Center queue IDs to query, up to 20 queue IDs. If a user provides `none`, filter by **No queue used** in the web UI.",
            "required": false,
            "schema": {
              "type": "array",
              "items": {
                "type": "string",
                "example": "Ql3qxUWXSHes4PmcOmzyOg"
              }
            }
          },
          {
            "name": "country_iso_codes",
            "in": "query",
            "description": "Two-letter ISO country [code](/docs/api/references/abbreviations/#countries).",
            "required": false,
            "schema": {
              "type": "array",
              "items": {
                "type": "string",
                "example": "CN"
              }
            }
          },
          {
            "name": "number_types",
            "in": "query",
            "description": "The number's type.",
            "required": false,
            "schema": {
              "type": "array",
              "items": {
                "type": "string",
                "example": "toll_free_number",
                "enum": [
                  "toll_free_number",
                  "virtual_service_number",
                  "byoc_number"
                ]
              }
            }
          },
          {
            "name": "region_ids",
            "in": "query",
            "description": "A comma-separated list of region IDs to query, up to 20 region IDs.",
            "required": false,
            "schema": {
              "maxItems": 20,
              "uniqueItems": true,
              "type": "array",
              "example": [
                "3XilEfOvQEKRIWMWhX1jDg"
              ],
              "items": {
                "type": "string"
              }
            }
          },
          {
            "name": "region_names",
            "in": "query",
            "description": "A comma-separated list of region names to query, up to 20 region Names.",
            "required": false,
            "schema": {
              "maxItems": 20,
              "uniqueItems": true,
              "type": "array",
              "example": [
                "Main Region"
              ],
              "items": {
                "type": "string"
              }
            }
          },
          {
            "name": "team_ids",
            "in": "query",
            "description": "A comma-separated list of team IDs to query, up to 20 team IDs.",
            "required": false,
            "schema": {
              "maxItems": 20,
              "uniqueItems": true,
              "type": "array",
              "example": [
                "3XilEfOvQEKRIWMWhX1jDg"
              ],
              "items": {
                "type": "string"
              }
            }
          },
          {
            "name": "team_names",
            "in": "query",
            "description": "A comma-separated list of team names to query, up to 20 team names.",
            "required": false,
            "schema": {
              "maxItems": 20,
              "uniqueItems": true,
              "type": "array",
              "example": [
                "My Team"
              ],
              "items": {
                "type": "string"
              }
            }
          },
          {
            "name": "sub_types",
            "in": "query",
            "description": "A comma-separated list of detailed call leg types.\n**Allowed subtypes**\n* `external_warm_conference_call`\n* `external_direct_conference_call`\n* `external_warm_transfer_call`\n* `external_direct_transfer_call`\n* `internal_warm_conference_call`\n* `internal_direct_conference_call`\n* `internal_warm_transfer_call`\n* `internal_direct_transfer_call`\n* `flow_initiated_outgoing_call` - flow outgoing",
            "required": false,
            "schema": {
              "type": "array",
              "example": [
                "external_direct_transfer_call"
              ],
              "items": {
                "type": "string",
                "example": "external_direct_transfer_call",
                "enum": [
                  "external_warm_conference_call",
                  "external_direct_conference_call",
                  "external_warm_transfer_call",
                  "external_direct_transfer_call",
                  "internal_warm_conference_call",
                  "internal_direct_conference_call",
                  "internal_warm_transfer_call",
                  "internal_direct_transfer_call",
                  "flow_initiated_outgoing_call"
                ]
              }
            }
          },
          {
            "name": "view_type",
            "in": "query",
            "description": "By default, this API returns the summary in List voice logs.\nIf `detail` is selected, it returns all the call logs' additional details. ",
            "required": false,
            "schema": {
              "type": "string",
              "example": "summary",
              "default": "summary",
              "enum": [
                "summary",
                "detail"
              ]
            }
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "Use the next page token to paginate through large result sets. A next page token is returned whenever the set of available results exceeds the current page size. This token's expiration period is 15 minutes.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42"
            }
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of items returned per page.",
            "required": false,
            "schema": {
              "maximum": 100,
              "type": "integer",
              "format": "int32",
              "example": 10,
              "default": 10
            }
          },
          {
            "name": "accountId",
            "in": "path",
            "description": "Unique identifier of the account.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "TQTvjT52Tmi_wrhASNNOEw"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "**HTTP Status Code:** `200`   Variables list returned.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "next_page_token": {
                      "maxLength": 100,
                      "type": "string",
                      "description": "Use the next page token to paginate through large result sets. A next page token is returned whenever the set of available results exceeds the current page size. This token's expiration period is 15 minutes.",
                      "example": "R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42"
                    },
                    "page_size": {
                      "maximum": 100,
                      "minimum": 1,
                      "type": "integer",
                      "description": "The number of records returned within a single API call.",
                      "example": 10,
                      "default": 10
                    },
                    "total_records": {
                      "minimum": 0,
                      "type": "integer",
                      "description": "The total number of all the records available across pages.",
                      "example": 10
                    },
                    "from": {
                      "type": "string",
                      "description": "The start time and date in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`. The defined date range should be a month, as the response only includes one month's worth of data. If no start date is specified, return data from the past 24 hours.",
                      "format": "date-time",
                      "example": "2023-01-01T08:00:00Z"
                    },
                    "to": {
                      "type": "string",
                      "description": "**Required** only when the `from` parameter is specified. The end time and date in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`, the same format as the `from` parameter.",
                      "format": "date-time",
                      "example": "2023-01-01T09:00:00Z"
                    },
                    "voice_calls": {
                      "type": "array",
                      "description": "Information about the voice call.",
                      "items": {
                        "type": "object",
                        "properties": {
                          "engagement_id": {
                            "type": "string",
                            "description": "The engagement's ID",
                            "example": "3XilEfOvQEKRIWMWhX1jDg"
                          },
                          "direction": {
                            "type": "string",
                            "description": "The engagement's direction.",
                            "example": "inbound",
                            "enum": [
                              "inbound",
                              "outbound"
                            ]
                          },
                          "call_legs": {
                            "type": "array",
                            "description": "The direction of the engagement call leg. `inbound` | `outbound`.",
                            "items": {
                              "type": "string",
                              "example": "inbound",
                              "enum": [
                                "inbound",
                                "outbound"
                              ]
                            }
                          },
                          "distributions": {
                            "type": "array",
                            "description": "The engagement's distribution.",
                            "items": {
                              "type": "string",
                              "example": "acd",
                              "enum": [
                                "acd",
                                "non_acd"
                              ]
                            }
                          },
                          "types": {
                            "type": "array",
                            "description": "The call's type.",
                            "items": {
                              "type": "string",
                              "example": "external_call",
                              "enum": [
                                "external_call",
                                "external_conference_call",
                                "external_callback_call",
                                "internal_call",
                                "internal_conference_call",
                                "internal_callback_call"
                              ]
                            }
                          },
                          "sub_types": {
                            "type": "array",
                            "description": "Specifies the detailed call leg type, such as a transfer or conference call.",
                            "items": {
                              "type": "string",
                              "example": "external_direct_transfer_call",
                              "enum": [
                                "external_warm_conference_call",
                                "external_direct_conference_call",
                                "external_warm_transfer_call",
                                "external_direct_transfer_call",
                                "internal_warm_conference_call",
                                "internal_direct_conference_call",
                                "internal_warm_transfer_call",
                                "internal_direct_transfer_call",
                                "flow_initiated_outgoing_call"
                              ]
                            }
                          },
                          "caller_number": {
                            "type": "string",
                            "description": "The caller's phone number.",
                            "example": "+18108001001"
                          },
                          "caller_number_type": {
                            "type": "string",
                            "description": "The caller's number type.",
                            "example": "toll_free_number",
                            "enum": [
                              "toll_free_number",
                              "virtual_service_number",
                              "byoc_number"
                            ]
                          },
                          "callee_number": {
                            "type": "string",
                            "description": "The callee's phone number.",
                            "example": "+12055437350"
                          },
                          "callee_number_type": {
                            "type": "string",
                            "description": "The callee's number type.",
                            "example": "toll_free_number",
                            "enum": [
                              "toll_free_number",
                              "virtual_service_number",
                              "byoc_number"
                            ]
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The date and time when the engagement started in `yyyy-MM-dd'T'HH:mm:ss\u00b1HH:mm` format. The timezone offset matches the offset provided in the `from` input parameter, or defaults to UTC (+00:00) if no offset was provided.",
                            "example": "2023-01-01T00:00:00-08:00"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The date and time when the engagement ended, in `yyyy-MM-dd'T'HH:mm:ss\u00b1HH:mm` format. The timezone offset is derived from the `from` input parameter if it includes a timezone offset (e.g., `-08:00`), otherwise it defaults to `+00:00` (UTC).",
                            "example": "2023-01-01T01:00:00-08:00"
                          },
                          "total_duration": {
                            "type": "integer",
                            "description": "The engagement's total duration, in seconds.",
                            "format": "int64",
                            "example": 10
                          },
                          "consumer_number": {
                            "type": "string",
                            "description": "The consumer's phone number.",
                            "example": "+12059300920"
                          },
                          "consumer_display_name": {
                            "type": "string",
                            "description": "The consumer's name.",
                            "example": "Tester"
                          },
                          "queues": {
                            "type": "array",
                            "description": "Information about the engagement's queues.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "queue_id": {
                                  "type": "string",
                                  "description": "The queue's ID.",
                                  "example": "d95avl1eRJ-H162PZUJ-qg",
                                  "deprecated": true
                                },
                                "cc_queue_id": {
                                  "type": "string",
                                  "description": "The Contact Center queue's ID.",
                                  "example": "d95avl1eRJ-H162PZUJ-qg"
                                },
                                "queue_name": {
                                  "type": "string",
                                  "description": "The queue's name.",
                                  "example": "agentQueue"
                                }
                              },
                              "description": "Information about the queue."
                            }
                          },
                          "agents": {
                            "type": "array",
                            "description": "Information about the engagement's agents.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "user_id": {
                                  "type": "string",
                                  "description": "The agent's ID.",
                                  "example": "ukAAkZKfROKMSw1bj_RDFQ"
                                },
                                "display_name": {
                                  "type": "string",
                                  "description": "The agent's name.",
                                  "example": "Jilly"
                                },
                                "user_email": {
                                  "type": "string",
                                  "description": "The agent's email.",
                                  "example": "jakie@zoom.us"
                                },
                                "user_ip_address": {
                                  "type": "string",
                                  "description": "IP address of the agent's device.",
                                  "format": "ipv4",
                                  "example": "38.145.73.4"
                                },
                                "user_device_mac_address": {
                                  "pattern": "^([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2})$",
                                  "type": "string",
                                  "description": "MAC address of the agent's device.",
                                  "example": "05:FA:15:25:EE:FF"
                                },
                                "user_registered_sip_zone": {
                                  "type": "string",
                                  "description": "The SIP zone where the agent registered.",
                                  "example": "ams1sbc"
                                },
                                "sip_zone_ip_address": {
                                  "type": "string",
                                  "description": "The SIP zome's IP address.",
                                  "format": "ipv4",
                                  "example": "38.145.73.4"
                                },
                                "user_region": {
                                  "type": "string",
                                  "description": "The agent's region name",
                                  "example": "Main Region"
                                },
                                "team_ids": {
                                  "type": "array",
                                  "description": "The agent's team IDs",
                                  "items": {
                                    "type": "string",
                                    "description": "agent team ID list",
                                    "example": "[\"3XilEfOvQEKRIWMWhX1jDg\"]"
                                  }
                                },
                                "team_names": {
                                  "type": "array",
                                  "description": "The agent's team names.",
                                  "items": {
                                    "type": "string",
                                    "description": "agent team name list.",
                                    "example": "[\"My team\"]"
                                  }
                                }
                              },
                              "description": "Information about the queue."
                            }
                          },
                          "flows": {
                            "type": "array",
                            "description": "Information about the engagement's flows.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "flow_id": {
                                  "type": "string",
                                  "description": "The flow's ID.",
                                  "example": "zeYjXoDOS_eV1QmTpj63PQ"
                                },
                                "flow_name": {
                                  "type": "string",
                                  "description": "The flow's name.",
                                  "example": "Demo"
                                }
                              },
                              "description": "Information about the flow."
                            }
                          },
                          "country_name": {
                            "type": "string",
                            "description": "The country name.",
                            "example": "China"
                          },
                          "recorded": {
                            "type": "boolean",
                            "description": "Flag to determine whether to recorded.",
                            "example": false
                          },
                          "monitored": {
                            "type": "boolean",
                            "description": "Flag to determine whether to monitored.",
                            "example": false
                          },
                          "inbox": {
                            "type": "boolean",
                            "description": "Flag to determine whether a message is in the box.",
                            "example": false
                          },
                          "result": {
                            "type": "string",
                            "description": "The engagement's results.",
                            "example": "completed",
                            "enum": [
                              "completed",
                              "short_abandoned",
                              "long_abandoned",
                              "hold_abandoned",
                              "long_calls",
                              "short_calls",
                              "hang_up_calls",
                              "overflowed_to_disconnect",
                              "overflowed_to_inbox",
                              "overflowed",
                              "abandon_quit",
                              "auto_closed",
                              "contained",
                              "missed",
                              "declined",
                              "callback_abandoned_by_consumer"
                            ]
                          },
                          "caller_id": {
                            "type": "string",
                            "description": "The caller's number.",
                            "example": "+12058945728"
                          },
                          "calling_party": {
                            "type": "string",
                            "description": "The calling's party.",
                            "example": "consumer",
                            "enum": [
                              "consumer",
                              "agent"
                            ]
                          },
                          "charge": {
                            "type": "string",
                            "description": "The bill's charge.",
                            "example": "$0.019"
                          },
                          "rates": {
                            "type": "array",
                            "description": "The price for each sent message.",
                            "items": {
                              "type": "string",
                              "example": "$0.019"
                            }
                          },
                          "charge_type": {
                            "type": "string",
                            "description": "The charge type.",
                            "example": "per_minute",
                            "enum": [
                              "per_minute"
                            ]
                          },
                          "session_border_controller_list": {
                            "type": "array",
                            "description": "Information about the session border controllers (SBCs) involved in the call.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "session_border_controller_name": {
                                  "type": "string",
                                  "description": "The session border controller's name.",
                                  "example": "ams1sbc"
                                },
                                "session_border_controller_ip_address": {
                                  "type": "string",
                                  "description": "The SBC's IP address.",
                                  "example": "38.145.73.20"
                                },
                                "consumer_zoom_trunk_name": {
                                  "type": "string",
                                  "description": "The name of the trunk between the customer SBC and Zoom SBC.",
                                  "example": "C10010142296A06"
                                },
                                "consumer_sbc_ip_address": {
                                  "type": "string",
                                  "description": "The customer's SBC IP address.",
                                  "example": "39.15.60.145"
                                }
                              },
                              "description": "Information about the session border controller."
                            }
                          },
                          "voice_call_details": {
                            "type": "array",
                            "description": "All related call logs.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "engagement_id": {
                                  "type": "string",
                                  "description": "The engagement's ID.",
                                  "example": "3XilEfOvQEKRIWMWhX1jDg"
                                },
                                "call_leg": {
                                  "type": "string",
                                  "description": "The direction of the engagement call log.",
                                  "example": "inbound",
                                  "enum": [
                                    "inbound",
                                    "outbound"
                                  ]
                                },
                                "distribution": {
                                  "type": "string",
                                  "description": "The call log's distribution.",
                                  "example": "acd",
                                  "enum": [
                                    "acd",
                                    "non_acd"
                                  ]
                                },
                                "type": {
                                  "type": "string",
                                  "description": "The call's type.",
                                  "example": "external_call",
                                  "enum": [
                                    "external_call",
                                    "external_conference_call",
                                    "external_callback_call",
                                    "internal_call",
                                    "internal_conference_call",
                                    "internal_callback_call"
                                  ]
                                },
                                "caller_number": {
                                  "type": "string",
                                  "description": "The caller's phone number.",
                                  "example": "+18108001001"
                                },
                                "caller_number_type": {
                                  "type": "string",
                                  "description": "The caller's number type.",
                                  "example": "toll_free_number",
                                  "enum": [
                                    "toll_free_number",
                                    "virtual_service_number",
                                    "byoc_number"
                                  ]
                                },
                                "callee_number": {
                                  "type": "string",
                                  "description": "The callee's phone number.",
                                  "example": "+12055437350"
                                },
                                "callee_number_type": {
                                  "type": "string",
                                  "description": "The callee's number type.",
                                  "example": "toll_free_number",
                                  "enum": [
                                    "toll_free_number",
                                    "virtual_service_number",
                                    "byoc_number"
                                  ]
                                },
                                "start_time": {
                                  "type": "string",
                                  "description": "The date and time when the call started, in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`.",
                                  "example": "2023-01-01T00:00:00-08:00"
                                },
                                "end_time": {
                                  "type": "string",
                                  "description": "The date and time when the call ended, in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`.",
                                  "example": "2023-01-01T01:00:00-08:00"
                                },
                                "total_duration": {
                                  "type": "integer",
                                  "description": "The call log's total duration, in seconds.",
                                  "format": "int64",
                                  "example": 10
                                },
                                "consumer_number": {
                                  "type": "string",
                                  "description": "The consumer's phone number.",
                                  "example": "+12059300920"
                                },
                                "consumer_display_name": {
                                  "type": "string",
                                  "description": "The consumer's name.",
                                  "example": "Tester"
                                },
                                "queue": {
                                  "type": "object",
                                  "properties": {
                                    "cc_queue_id": {
                                      "type": "string",
                                      "description": "The Contact Center queue's ID.",
                                      "example": "d95avl1eRJ-H162PZUJ-qg"
                                    },
                                    "queue_name": {
                                      "type": "string",
                                      "description": "The queue's name.",
                                      "example": "agentQueue"
                                    }
                                  },
                                  "description": "Information about the queue."
                                },
                                "agents": {
                                  "type": "array",
                                  "description": "Information about the engagement's agents.",
                                  "items": {
                                    "type": "object",
                                    "properties": {
                                      "user_id": {
                                        "type": "string",
                                        "description": "The agent's ID.",
                                        "example": "ukAAkZKfROKMSw1bj_RDFQ"
                                      },
                                      "display_name": {
                                        "type": "string",
                                        "description": "The agent's name.",
                                        "example": "Jilly"
                                      },
                                      "user_email": {
                                        "type": "string",
                                        "description": "The agent's email.",
                                        "example": "jakie@zoom.us"
                                      },
                                      "user_ip_address": {
                                        "type": "string",
                                        "description": "The agent's device's IP address.",
                                        "format": "ipv4",
                                        "example": "38.145.73.4"
                                      },
                                      "user_device_mac_address": {
                                        "pattern": "^([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2})$",
                                        "type": "string",
                                        "description": "The agent's device's MAC address.",
                                        "example": "05:FA:15:25:EE:FF"
                                      },
                                      "user_registered_sip_zone": {
                                        "type": "string",
                                        "description": "The SIP zone where the agent registered.",
                                        "example": "ams1sbc"
                                      },
                                      "sip_zone_ip_address": {
                                        "type": "string",
                                        "description": "The SIP zone's IP address.",
                                        "format": "ipv4",
                                        "example": "38.145.73.4"
                                      },
                                      "user_region": {
                                        "type": "string",
                                        "description": "The agent's region name.",
                                        "example": "Main Region"
                                      },
                                      "team_ids": {
                                        "type": "array",
                                        "description": "The agent's team IDs",
                                        "items": {
                                          "type": "string",
                                          "description": "Agent team ID list.",
                                          "example": "3XilEfOvQEKRIWMWhX1jDg"
                                        }
                                      },
                                      "team_names": {
                                        "type": "array",
                                        "description": "The agent's team names.",
                                        "items": {
                                          "type": "string",
                                          "description": "Agent team name list.",
                                          "example": "My team"
                                        }
                                      }
                                    },
                                    "description": "Information about the queue."
                                  }
                                },
                                "flow": {
                                  "type": "object",
                                  "properties": {
                                    "flow_id": {
                                      "type": "string",
                                      "description": "The flow's ID.",
                                      "example": "zeYjXoDOS_eV1QmTpj63PQ"
                                    },
                                    "flow_name": {
                                      "type": "string",
                                      "description": "The flow's name.",
                                      "example": "Demo"
                                    }
                                  },
                                  "description": "Information about the flow."
                                },
                                "country_name": {
                                  "type": "string",
                                  "description": "The country name.",
                                  "example": "China"
                                },
                                "recorded": {
                                  "type": "boolean",
                                  "description": "Whether to record.",
                                  "example": false
                                },
                                "monitored": {
                                  "type": "boolean",
                                  "description": "Whether to monitor.",
                                  "example": false
                                },
                                "inbox": {
                                  "type": "boolean",
                                  "description": "Whether a message is in the box.",
                                  "example": false
                                },
                                "result": {
                                  "type": "string",
                                  "description": "The engagement's results.",
                                  "example": "completed",
                                  "enum": [
                                    "completed",
                                    "short_abandoned",
                                    "long_abandoned",
                                    "hold_abandoned",
                                    "long_calls",
                                    "short_calls",
                                    "hang_up_calls",
                                    "overflowed_to_disconnect",
                                    "overflowed_to_inbox",
                                    "overflowed",
                                    "abandon_quit",
                                    "auto_closed",
                                    "contained",
                                    "missed",
                                    "declined",
                                    "callback_abandoned_by_consumer"
                                  ]
                                },
                                "caller_id": {
                                  "type": "string",
                                  "description": "The caller's number.",
                                  "example": "+12058945728"
                                },
                                "calling_party": {
                                  "type": "string",
                                  "description": "The calling's party.",
                                  "example": "consumer",
                                  "enum": [
                                    "consumer",
                                    "agent"
                                  ]
                                },
                                "charge": {
                                  "type": "string",
                                  "description": "The bill's charge.",
                                  "example": "$0.019"
                                },
                                "rate": {
                                  "type": "string",
                                  "description": "The price for sent message.",
                                  "example": "$0.019"
                                },
                                "charge_type": {
                                  "type": "string",
                                  "description": "The charge type.",
                                  "example": "per_minute",
                                  "enum": [
                                    "per_minute"
                                  ]
                                }
                              },
                              "description": "Related call leg details"
                            }
                          }
                        },
                        "description": "Information about the voice call."
                      }
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `300` <br>\n The time and date must be in `yyyy-mm-dd`` or ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`. <br>\n**Error Code:** `300` <br>\n The requested report exceeds the 1-month limit. <br>\n**Error Code:** `300` <br>\n Invalid field. <br>\n"
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `1001` <br>\n User does not exist: {userId}. <br>\n**Error Code:** `1201` <br>\n Queue does not exist: {queueId}. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "contact_center_voice_call:master",
              "contact_center:read:voice_call_log:master"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [],
          "x-macro-scopes": [
            "contact_center_voice_call:master"
          ],
          "x-granular-scopes": [
            "contact_center:read:voice_call_log:master"
          ]
        }
      }
    }
  },
  "components": {
    "securitySchemes": {
      "openapi_authorization": {
        "type": "apiKey",
        "name": "Authorization",
        "in": "header"
      },
      "openapi_oauth": {
        "type": "oauth2",
        "flows": {
          "authorizationCode": {
            "authorizationUrl": "/",
            "tokenUrl": "",
            "refreshUrl": "",
            "scopes": {
              "contact_center_sms:master": "contact_center_sms:master",
              "contact_center:read:sms_log:master": "contact_center:read:sms_log:master",
              "contact_center_voice_call:master": "contact_center_voice_call:master",
              "contact_center:read:voice_call_log:master": "contact_center:read:voice_call_log:master"
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
      "contact-center",
      "ma"
    ]
  },
  "downloadPath": "/api-hub/contact-center/ma/master.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": true
  },
  "productMeta": {
    "specType": "oas",
    "slug": "/docs/api/rest/reference/contact-center/ma",
    "keywords": "zoom, marketplace, rest, api, create, user, get, delete, contact center, custom, programmatic, task, queue, agent, supervisor, disposition, asset, skill, address, book, report, note",
    "title": "Zoom Contact Center Master API",
    "description": "",
    "params": {
      "productName": "Contact Center",
      "maFlag": true
    },
    "zoapEndpoint": "docJsonByFixVersion",
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "productDisplayTitle": "Contact Center",
    "scalarSlug": "/docs/api/contact-center/ma",
    "svgname": "contactCenter"
  },
  "frontmatter": {
    "title": "Contact Center Master APIs",
    "keywords": "zoom, marketplace, rest, api, create, user, get, delete, contact center, custom, programmatic, task, queue, agent, supervisor, disposition, asset, skill, address, book, report, note",
    "description": "The Contact Center APIs allow developers to interface with [Contact Center](https://developers.zoom.us/docs/contact-center/) features programmatically.",
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
        "contact-center",
        "ma"
      ]
    },
    "downloadPath": "/api-hub/contact-center/ma/master.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": true
    },
    "productMeta": {
      "specType": "oas",
      "slug": "/docs/api/rest/reference/contact-center/ma",
      "keywords": "zoom, marketplace, rest, api, create, user, get, delete, contact center, custom, programmatic, task, queue, agent, supervisor, disposition, asset, skill, address, book, report, note",
      "title": "Zoom Contact Center Master API",
      "description": "",
      "params": {
        "productName": "Contact Center",
        "maFlag": true
      },
      "zoapEndpoint": "docJsonByFixVersion",
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "productDisplayTitle": "Contact Center",
      "scalarSlug": "/docs/api/contact-center/ma",
      "svgname": "contactCenter"
    },
    "spec": "{\"openapi\":\"3.0.0\",\"info\":{\"title\":\"Contact Center\",\"description\":\"The Contact Center APIs allow developers to interface with [Contact Center](https://developers.zoom.us/docs/contact-center/) features programmatically.\",\"termsOfService\":\"https://zoom.us/docs/en-us/zoom_api_license_and_tou.html\",\"contact\":{\"name\":\"Zoom Developers\",\"url\":\"https://developer.zoom.us/\"},\"version\":\"2\"},\"externalDocs\":{\"description\":\"Find out more about Swagger\",\"url\":\"https://swagger.io\"},\"servers\":[{\"url\":\"https://api.zoom.us/v2\"}],\"paths\":{\"/accounts/{accountId}/contact_center/sms\":{\"get\":{\"tags\":[\"Logs\"],\"summary\":\"List SMS logs\",\"description\":\"Returns a list of SMS engagement logs. Engagement data are available via API once they are completed.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `contact_center_sms:master`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `contact_center:read:sms_log:master`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `HEAVY`\",\"operationId\":\"listSMS\",\"parameters\":[{\"name\":\"next_page_token\",\"in\":\"query\",\"description\":\"Use the next page token to paginate through large result sets. A next page token is returned whenever the set of available results exceeds the current page size. This token's expiration period is 15 minutes.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42\"}},{\"name\":\"page_size\",\"in\":\"query\",\"description\":\"The number of items returned per page.\",\"required\":false,\"schema\":{\"maximum\":100,\"type\":\"integer\",\"format\":\"int32\",\"example\":10,\"default\":10}},{\"name\":\"from\",\"in\":\"query\",\"description\":\"The start time and date in `yyyy-mm-dd` or ISO 8601 format, which is either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`. The defined date range should be a month, as the response only includes one month's worth of data. If no start date is specified, return data from the past 24 hours.\",\"required\":false,\"schema\":{\"type\":\"string\",\"format\":\"date\",\"example\":\"2021-12-10\"}},{\"name\":\"to\",\"in\":\"query\",\"description\":\"**Required** only when the `from` parameter is specified. The end time and date in `yyyy-mm-dd` or ISO 8601 format, which is either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`.\",\"required\":false,\"schema\":{\"type\":\"string\",\"format\":\"date\",\"example\":\"2021-12-10\"}},{\"name\":\"country_iso_codes\",\"in\":\"query\",\"description\":\"Two-letter ISO country [code](/docs/api/references/abbreviations/#countries).\",\"required\":false,\"schema\":{\"type\":\"array\",\"example\":[\"US\"],\"items\":{\"type\":\"string\",\"example\":\"CN\"}}},{\"name\":\"direction\",\"in\":\"query\",\"description\":\"The engagement's direction.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"inbound\",\"enum\":[\"inbound\",\"outbound\"]}},{\"name\":\"user_id\",\"in\":\"query\",\"description\":\"The agent's ID.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"ukAAkZKfROKMSw1bj_RDFQ\"}},{\"name\":\"engagement_id\",\"in\":\"query\",\"description\":\"The engagement's ID.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"3XilEfOvQEKRIWMWhX1jDg\"}},{\"name\":\"consumer_number\",\"in\":\"query\",\"description\":\"The consumer's phone number.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"+12059300920\"}},{\"name\":\"accountId\",\"in\":\"path\",\"description\":\"Unique identifier of the account.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"TQTvjT52Tmi_wrhASNNOEw\"}}],\"responses\":{\"200\":{\"description\":\"**HTTP Status Code:** `200`   \\n \\nVariables list returned\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"next_page_token\":{\"type\":\"string\",\"description\":\"Use the next page token to paginate through large result sets. A next page token returns when the set of available results exceeds the current page size. This token's expiration period is 15 minutes.\",\"example\":\"R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42\"},\"page_size\":{\"maximum\":100,\"type\":\"integer\",\"description\":\"The number of records returned in a single API call.\",\"example\":30,\"default\":10},\"total_records\":{\"type\":\"integer\",\"description\":\"The total number of all the records available across pages.\",\"example\":1},\"from\":{\"type\":\"string\",\"description\":\"The start time and date in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'`, or `yyyy-MM-dd'T'HH:mm:ss'TZD'`. The defined date range should be a month, as the response only includes one month's worth of data. If no start date is specified, return data from the past 24 hours.\",\"format\":\"date-time\",\"example\":\"2023-01-01T08:00:00Z\"},\"to\":{\"type\":\"string\",\"description\":\"**Required** only when the `from` parameter is specified. The end time and date in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`, the same format as the `from` parameter.\",\"format\":\"date-time\",\"example\":\"2023-01-01T09:00:00Z\"},\"sms\":{\"type\":\"array\",\"description\":\"Information about the SMS.\",\"items\":{\"type\":\"object\",\"properties\":{\"direction\":{\"type\":\"string\",\"description\":\"The engagement's direction. `inbound` | `outbound`.\",\"example\":\"inbound\",\"enum\":[\"inbound\",\"outbound\"]},\"engagement_id\":{\"type\":\"string\",\"description\":\"The engagement's ID\",\"example\":\"3XilEfOvQEKRIWMWhX1jDg\"},\"contact_center_number\":{\"type\":\"string\",\"description\":\"The agent's number. An agent used a flow or queue number and did not really have its own number.\",\"example\":\"+18108001001\"},\"start_time\":{\"type\":\"string\",\"description\":\"The date and time when the engagement started in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`.\",\"format\":\"date-time\",\"example\":\"2023-01-01T08:00:00Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The date and time when the engagement ended in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`.\",\"format\":\"date-time\",\"example\":\"2023-01-01T09:00:00Z\"},\"consumer_number\":{\"type\":\"string\",\"description\":\"The consumer's phone number.\",\"example\":\"+12059300920\"},\"consumer_display_name\":{\"type\":\"string\",\"description\":\"The consumer's name.\",\"example\":\"Tester\"},\"queues\":{\"type\":\"array\",\"description\":\"Information about the engagement's queues.\",\"items\":{\"type\":\"object\",\"properties\":{\"queue_id\":{\"type\":\"string\",\"description\":\"The queue's ID.\",\"example\":\"d95avl1eRJ-H162PZUJ-qg\",\"deprecated\":true},\"cc_queue_id\":{\"type\":\"string\",\"description\":\"The Contact Center queue's ID.\",\"example\":\"d95avl1eRJ-H162PZUJ-qg\"},\"queue_name\":{\"type\":\"string\",\"description\":\"The queue's name.\",\"example\":\"agentQueue\"}},\"description\":\"Information about the queue.\"}},\"agents\":{\"type\":\"array\",\"description\":\"Information about the engagement's agents.\",\"items\":{\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The agent's ID.\",\"example\":\"ukAAkZKfROKMSw1bj_RDFQ\"},\"display_name\":{\"type\":\"string\",\"description\":\"The agent's name.\",\"example\":\"Jilly\"}},\"description\":\"Information about the queue.\"}},\"country_name\":{\"type\":\"string\",\"description\":\"The country name.\",\"example\":\"China\"},\"sms_types\":{\"type\":\"array\",\"description\":\"The SMS's type.\",\"items\":{\"type\":\"string\",\"example\":\"sms\",\"enum\":[\"sms\",\"mms\"]}},\"total_sent_messages\":{\"type\":\"integer\",\"description\":\"The total number of sent messages.\",\"format\":\"int64\",\"example\":0},\"total_received_messages\":{\"type\":\"integer\",\"description\":\"The total number of received messages.\",\"format\":\"int64\",\"example\":0},\"total_sent_files\":{\"type\":\"integer\",\"description\":\"The total number of sent files.\",\"format\":\"int64\",\"example\":0},\"total_received_files\":{\"type\":\"integer\",\"description\":\"The total number of received files.\",\"format\":\"int64\",\"example\":0}},\"description\":\"Information about the SMS.\"}}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `2901` <br>\\n Engagement does not exist: $engagementId. <br>\\n**Error Code:** `1001` <br>\\n User does not exist: $userId. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[],\"x-macro-scopes\":[\"contact_center_sms:master\"],\"x-granular-scopes\":[\"contact_center:read:sms_log:master\"]}}},\"/accounts/{accountId}/contact_center/voice_calls\":{\"get\":{\"tags\":[\"Logs\"],\"summary\":\"List voice call logs\",\"description\":\"Return a list of voice call engagement logs. Engagement data is available via API once it is completed. **Note** These fields - `agents.user_ip_address`, `agents.user_device_mac_address`, `agents.user_registered_sip_zone`, `agents.sip_zone_ip_address`, `sbcInfos.session_border_controller_name`, `sbcInfos.session_border_controller_ip_address`, `sbcInfos.consumer_zoom_trunk_name` and `sbcInfos.consumer_sbc_ip_address` - are only available to accounts which have enabled an internal flag to return these additional fields. To enable the feature flag, contact Zoom Support.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `contact_center_voice_call:master`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `contact_center:read:voice_call_log:master`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `HEAVY`\",\"operationId\":\"listVoiceCall\",\"parameters\":[{\"name\":\"from\",\"in\":\"query\",\"description\":\"The start time and date in either `yyyy-MM-dd`, `yyyy-MM-dd'T'HH:mm:ss'Z'`, or `yyyy-MM-dd'T'HH:mm:ss\u00b1HH:mm` format. The defined date range should be a month, as the response only includes one month's worth of data. If no start date is specified, return data from the past 24 hours.\\n\\n**Note** When using `yyyy-MM-dd'T'HH:mm:ss'Z'` format, the `'Z'` is treated as a literal character. The timezone offset from `yyyy-MM-dd'T'HH:mm:ss\u00b1HH:mm` format is used for response datetime formatting.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"2021-12-10\"}},{\"name\":\"to\",\"in\":\"query\",\"description\":\"**Required** only when the `from` parameter is specified. The end time and date in `yyyy-MM-dd`, `yyyy-MM-dd'T'HH:mm:ss'Z'`, or `yyyy-MM-dd'T'HH:mm:ss\u00b1HH:mm` format, matching the format of the `from` parameter.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"2021-12-10\"}},{\"name\":\"types\",\"in\":\"query\",\"description\":\"The call's type.\",\"required\":false,\"schema\":{\"type\":\"array\",\"items\":{\"type\":\"string\",\"example\":\"external_call\",\"enum\":[\"external_call\",\"external_conference_call\",\"external_callback_call\",\"internal_call\",\"internal_conference_call\",\"internal_callback_call\"]}}},{\"name\":\"distribution\",\"in\":\"query\",\"description\":\"The engagement's distribution.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"acd\",\"enum\":[\"acd\",\"non_acd\"]}},{\"name\":\"consumer_number\",\"in\":\"query\",\"description\":\"The consumer's phone number.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"+12059300920\"}},{\"name\":\"direction\",\"in\":\"query\",\"description\":\"The engagement's direction.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"inbound\",\"enum\":[\"inbound\",\"outbound\"]}},{\"name\":\"user_ids\",\"in\":\"query\",\"description\":\"A comma-separated list of agents' user IDs to query, up to 20 agent IDs.\",\"required\":false,\"schema\":{\"type\":\"array\",\"items\":{\"type\":\"string\",\"example\":\"t9PXwOsSQ2qXp7YSPDrALQ\"}}},{\"name\":\"engagement_id\",\"in\":\"query\",\"description\":\"The engagement's ID.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"3XilEfOvQEKRIWMWhX1jDg\"}},{\"name\":\"queue_ids\",\"in\":\"query\",\"description\":\"A comma-separated list of queue IDs to query, up to 20 queue IDs. If user provide `none`, filter by **No queue used** in the web UI.\",\"required\":false,\"deprecated\":true,\"schema\":{\"type\":\"array\",\"items\":{\"type\":\"string\",\"example\":\"Ql3qxUWXSHes4PmcOmzyOg\"}}},{\"name\":\"cc_queue_ids\",\"in\":\"query\",\"description\":\"A comma-separated list of Contact Center queue IDs to query, up to 20 queue IDs. If a user provides `none`, filter by **No queue used** in the web UI.\",\"required\":false,\"schema\":{\"type\":\"array\",\"items\":{\"type\":\"string\",\"example\":\"Ql3qxUWXSHes4PmcOmzyOg\"}}},{\"name\":\"country_iso_codes\",\"in\":\"query\",\"description\":\"Two-letter ISO country [code](/docs/api/references/abbreviations/#countries).\",\"required\":false,\"schema\":{\"type\":\"array\",\"items\":{\"type\":\"string\",\"example\":\"CN\"}}},{\"name\":\"number_types\",\"in\":\"query\",\"description\":\"The number's type.\",\"required\":false,\"schema\":{\"type\":\"array\",\"items\":{\"type\":\"string\",\"example\":\"toll_free_number\",\"enum\":[\"toll_free_number\",\"virtual_service_number\",\"byoc_number\"]}}},{\"name\":\"region_ids\",\"in\":\"query\",\"description\":\"A comma-separated list of region IDs to query, up to 20 region IDs.\",\"required\":false,\"schema\":{\"maxItems\":20,\"uniqueItems\":true,\"type\":\"array\",\"example\":[\"3XilEfOvQEKRIWMWhX1jDg\"],\"items\":{\"type\":\"string\"}}},{\"name\":\"region_names\",\"in\":\"query\",\"description\":\"A comma-separated list of region names to query, up to 20 region Names.\",\"required\":false,\"schema\":{\"maxItems\":20,\"uniqueItems\":true,\"type\":\"array\",\"example\":[\"Main Region\"],\"items\":{\"type\":\"string\"}}},{\"name\":\"team_ids\",\"in\":\"query\",\"description\":\"A comma-separated list of team IDs to query, up to 20 team IDs.\",\"required\":false,\"schema\":{\"maxItems\":20,\"uniqueItems\":true,\"type\":\"array\",\"example\":[\"3XilEfOvQEKRIWMWhX1jDg\"],\"items\":{\"type\":\"string\"}}},{\"name\":\"team_names\",\"in\":\"query\",\"description\":\"A comma-separated list of team names to query, up to 20 team names.\",\"required\":false,\"schema\":{\"maxItems\":20,\"uniqueItems\":true,\"type\":\"array\",\"example\":[\"My Team\"],\"items\":{\"type\":\"string\"}}},{\"name\":\"sub_types\",\"in\":\"query\",\"description\":\"A comma-separated list of detailed call leg types.\\n**Allowed subtypes**\\n* `external_warm_conference_call`\\n* `external_direct_conference_call`\\n* `external_warm_transfer_call`\\n* `external_direct_transfer_call`\\n* `internal_warm_conference_call`\\n* `internal_direct_conference_call`\\n* `internal_warm_transfer_call`\\n* `internal_direct_transfer_call`\\n* `flow_initiated_outgoing_call` - flow outgoing\",\"required\":false,\"schema\":{\"type\":\"array\",\"example\":[\"external_direct_transfer_call\"],\"items\":{\"type\":\"string\",\"example\":\"external_direct_transfer_call\",\"enum\":[\"external_warm_conference_call\",\"external_direct_conference_call\",\"external_warm_transfer_call\",\"external_direct_transfer_call\",\"internal_warm_conference_call\",\"internal_direct_conference_call\",\"internal_warm_transfer_call\",\"internal_direct_transfer_call\",\"flow_initiated_outgoing_call\"]}}},{\"name\":\"view_type\",\"in\":\"query\",\"description\":\"By default, this API returns the summary in List voice logs.\\nIf `detail` is selected, it returns all the call logs' additional details. \",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"summary\",\"default\":\"summary\",\"enum\":[\"summary\",\"detail\"]}},{\"name\":\"next_page_token\",\"in\":\"query\",\"description\":\"Use the next page token to paginate through large result sets. A next page token is returned whenever the set of available results exceeds the current page size. This token's expiration period is 15 minutes.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42\"}},{\"name\":\"page_size\",\"in\":\"query\",\"description\":\"The number of items returned per page.\",\"required\":false,\"schema\":{\"maximum\":100,\"type\":\"integer\",\"format\":\"int32\",\"example\":10,\"default\":10}},{\"name\":\"accountId\",\"in\":\"path\",\"description\":\"Unique identifier of the account.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"TQTvjT52Tmi_wrhASNNOEw\"}}],\"responses\":{\"200\":{\"description\":\"**HTTP Status Code:** `200`   Variables list returned.\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"next_page_token\":{\"maxLength\":100,\"type\":\"string\",\"description\":\"Use the next page token to paginate through large result sets. A next page token is returned whenever the set of available results exceeds the current page size. This token's expiration period is 15 minutes.\",\"example\":\"R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42\"},\"page_size\":{\"maximum\":100,\"minimum\":1,\"type\":\"integer\",\"description\":\"The number of records returned within a single API call.\",\"example\":10,\"default\":10},\"total_records\":{\"minimum\":0,\"type\":\"integer\",\"description\":\"The total number of all the records available across pages.\",\"example\":10},\"from\":{\"type\":\"string\",\"description\":\"The start time and date in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`. The defined date range should be a month, as the response only includes one month's worth of data. If no start date is specified, return data from the past 24 hours.\",\"format\":\"date-time\",\"example\":\"2023-01-01T08:00:00Z\"},\"to\":{\"type\":\"string\",\"description\":\"**Required** only when the `from` parameter is specified. The end time and date in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`, the same format as the `from` parameter.\",\"format\":\"date-time\",\"example\":\"2023-01-01T09:00:00Z\"},\"voice_calls\":{\"type\":\"array\",\"description\":\"Information about the voice call.\",\"items\":{\"type\":\"object\",\"properties\":{\"engagement_id\":{\"type\":\"string\",\"description\":\"The engagement's ID\",\"example\":\"3XilEfOvQEKRIWMWhX1jDg\"},\"direction\":{\"type\":\"string\",\"description\":\"The engagement's direction.\",\"example\":\"inbound\",\"enum\":[\"inbound\",\"outbound\"]},\"call_legs\":{\"type\":\"array\",\"description\":\"The direction of the engagement call leg. `inbound` | `outbound`.\",\"items\":{\"type\":\"string\",\"example\":\"inbound\",\"enum\":[\"inbound\",\"outbound\"]}},\"distributions\":{\"type\":\"array\",\"description\":\"The engagement's distribution.\",\"items\":{\"type\":\"string\",\"example\":\"acd\",\"enum\":[\"acd\",\"non_acd\"]}},\"types\":{\"type\":\"array\",\"description\":\"The call's type.\",\"items\":{\"type\":\"string\",\"example\":\"external_call\",\"enum\":[\"external_call\",\"external_conference_call\",\"external_callback_call\",\"internal_call\",\"internal_conference_call\",\"internal_callback_call\"]}},\"sub_types\":{\"type\":\"array\",\"description\":\"Specifies the detailed call leg type, such as a transfer or conference call.\",\"items\":{\"type\":\"string\",\"example\":\"external_direct_transfer_call\",\"enum\":[\"external_warm_conference_call\",\"external_direct_conference_call\",\"external_warm_transfer_call\",\"external_direct_transfer_call\",\"internal_warm_conference_call\",\"internal_direct_conference_call\",\"internal_warm_transfer_call\",\"internal_direct_transfer_call\",\"flow_initiated_outgoing_call\"]}},\"caller_number\":{\"type\":\"string\",\"description\":\"The caller's phone number.\",\"example\":\"+18108001001\"},\"caller_number_type\":{\"type\":\"string\",\"description\":\"The caller's number type.\",\"example\":\"toll_free_number\",\"enum\":[\"toll_free_number\",\"virtual_service_number\",\"byoc_number\"]},\"callee_number\":{\"type\":\"string\",\"description\":\"The callee's phone number.\",\"example\":\"+12055437350\"},\"callee_number_type\":{\"type\":\"string\",\"description\":\"The callee's number type.\",\"example\":\"toll_free_number\",\"enum\":[\"toll_free_number\",\"virtual_service_number\",\"byoc_number\"]},\"start_time\":{\"type\":\"string\",\"description\":\"The date and time when the engagement started in `yyyy-MM-dd'T'HH:mm:ss\u00b1HH:mm` format. The timezone offset matches the offset provided in the `from` input parameter, or defaults to UTC (+00:00) if no offset was provided.\",\"example\":\"2023-01-01T00:00:00-08:00\"},\"end_time\":{\"type\":\"string\",\"description\":\"The date and time when the engagement ended, in `yyyy-MM-dd'T'HH:mm:ss\u00b1HH:mm` format. The timezone offset is derived from the `from` input parameter if it includes a timezone offset (e.g., `-08:00`), otherwise it defaults to `+00:00` (UTC).\",\"example\":\"2023-01-01T01:00:00-08:00\"},\"total_duration\":{\"type\":\"integer\",\"description\":\"The engagement's total duration, in seconds.\",\"format\":\"int64\",\"example\":10},\"consumer_number\":{\"type\":\"string\",\"description\":\"The consumer's phone number.\",\"example\":\"+12059300920\"},\"consumer_display_name\":{\"type\":\"string\",\"description\":\"The consumer's name.\",\"example\":\"Tester\"},\"queues\":{\"type\":\"array\",\"description\":\"Information about the engagement's queues.\",\"items\":{\"type\":\"object\",\"properties\":{\"queue_id\":{\"type\":\"string\",\"description\":\"The queue's ID.\",\"example\":\"d95avl1eRJ-H162PZUJ-qg\",\"deprecated\":true},\"cc_queue_id\":{\"type\":\"string\",\"description\":\"The Contact Center queue's ID.\",\"example\":\"d95avl1eRJ-H162PZUJ-qg\"},\"queue_name\":{\"type\":\"string\",\"description\":\"The queue's name.\",\"example\":\"agentQueue\"}},\"description\":\"Information about the queue.\"}},\"agents\":{\"type\":\"array\",\"description\":\"Information about the engagement's agents.\",\"items\":{\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The agent's ID.\",\"example\":\"ukAAkZKfROKMSw1bj_RDFQ\"},\"display_name\":{\"type\":\"string\",\"description\":\"The agent's name.\",\"example\":\"Jilly\"},\"user_email\":{\"type\":\"string\",\"description\":\"The agent's email.\",\"example\":\"jakie@zoom.us\"},\"user_ip_address\":{\"type\":\"string\",\"description\":\"IP address of the agent's device.\",\"format\":\"ipv4\",\"example\":\"38.145.73.4\"},\"user_device_mac_address\":{\"pattern\":\"^([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2})$\",\"type\":\"string\",\"description\":\"MAC address of the agent's device.\",\"example\":\"05:FA:15:25:EE:FF\"},\"user_registered_sip_zone\":{\"type\":\"string\",\"description\":\"The SIP zone where the agent registered.\",\"example\":\"ams1sbc\"},\"sip_zone_ip_address\":{\"type\":\"string\",\"description\":\"The SIP zome's IP address.\",\"format\":\"ipv4\",\"example\":\"38.145.73.4\"},\"user_region\":{\"type\":\"string\",\"description\":\"The agent's region name\",\"example\":\"Main Region\"},\"team_ids\":{\"type\":\"array\",\"description\":\"The agent's team IDs\",\"items\":{\"type\":\"string\",\"description\":\"agent team ID list\",\"example\":\"[\\\"3XilEfOvQEKRIWMWhX1jDg\\\"]\"}},\"team_names\":{\"type\":\"array\",\"description\":\"The agent's team names.\",\"items\":{\"type\":\"string\",\"description\":\"agent team name list.\",\"example\":\"[\\\"My team\\\"]\"}}},\"description\":\"Information about the queue.\"}},\"flows\":{\"type\":\"array\",\"description\":\"Information about the engagement's flows.\",\"items\":{\"type\":\"object\",\"properties\":{\"flow_id\":{\"type\":\"string\",\"description\":\"The flow's ID.\",\"example\":\"zeYjXoDOS_eV1QmTpj63PQ\"},\"flow_name\":{\"type\":\"string\",\"description\":\"The flow's name.\",\"example\":\"Demo\"}},\"description\":\"Information about the flow.\"}},\"country_name\":{\"type\":\"string\",\"description\":\"The country name.\",\"example\":\"China\"},\"recorded\":{\"type\":\"boolean\",\"description\":\"Flag to determine whether to recorded.\",\"example\":false},\"monitored\":{\"type\":\"boolean\",\"description\":\"Flag to determine whether to monitored.\",\"example\":false},\"inbox\":{\"type\":\"boolean\",\"description\":\"Flag to determine whether a message is in the box.\",\"example\":false},\"result\":{\"type\":\"string\",\"description\":\"The engagement's results.\",\"example\":\"completed\",\"enum\":[\"completed\",\"short_abandoned\",\"long_abandoned\",\"hold_abandoned\",\"long_calls\",\"short_calls\",\"hang_up_calls\",\"overflowed_to_disconnect\",\"overflowed_to_inbox\",\"overflowed\",\"abandon_quit\",\"auto_closed\",\"contained\",\"missed\",\"declined\",\"callback_abandoned_by_consumer\"]},\"caller_id\":{\"type\":\"string\",\"description\":\"The caller's number.\",\"example\":\"+12058945728\"},\"calling_party\":{\"type\":\"string\",\"description\":\"The calling's party.\",\"example\":\"consumer\",\"enum\":[\"consumer\",\"agent\"]},\"charge\":{\"type\":\"string\",\"description\":\"The bill's charge.\",\"example\":\"$0.019\"},\"rates\":{\"type\":\"array\",\"description\":\"The price for each sent message.\",\"items\":{\"type\":\"string\",\"example\":\"$0.019\"}},\"charge_type\":{\"type\":\"string\",\"description\":\"The charge type.\",\"example\":\"per_minute\",\"enum\":[\"per_minute\"]},\"session_border_controller_list\":{\"type\":\"array\",\"description\":\"Information about the session border controllers (SBCs) involved in the call.\",\"items\":{\"type\":\"object\",\"properties\":{\"session_border_controller_name\":{\"type\":\"string\",\"description\":\"The session border controller's name.\",\"example\":\"ams1sbc\"},\"session_border_controller_ip_address\":{\"type\":\"string\",\"description\":\"The SBC's IP address.\",\"example\":\"38.145.73.20\"},\"consumer_zoom_trunk_name\":{\"type\":\"string\",\"description\":\"The name of the trunk between the customer SBC and Zoom SBC.\",\"example\":\"C10010142296A06\"},\"consumer_sbc_ip_address\":{\"type\":\"string\",\"description\":\"The customer's SBC IP address.\",\"example\":\"39.15.60.145\"}},\"description\":\"Information about the session border controller.\"}},\"voice_call_details\":{\"type\":\"array\",\"description\":\"All related call logs.\",\"items\":{\"type\":\"object\",\"properties\":{\"engagement_id\":{\"type\":\"string\",\"description\":\"The engagement's ID.\",\"example\":\"3XilEfOvQEKRIWMWhX1jDg\"},\"call_leg\":{\"type\":\"string\",\"description\":\"The direction of the engagement call log.\",\"example\":\"inbound\",\"enum\":[\"inbound\",\"outbound\"]},\"distribution\":{\"type\":\"string\",\"description\":\"The call log's distribution.\",\"example\":\"acd\",\"enum\":[\"acd\",\"non_acd\"]},\"type\":{\"type\":\"string\",\"description\":\"The call's type.\",\"example\":\"external_call\",\"enum\":[\"external_call\",\"external_conference_call\",\"external_callback_call\",\"internal_call\",\"internal_conference_call\",\"internal_callback_call\"]},\"caller_number\":{\"type\":\"string\",\"description\":\"The caller's phone number.\",\"example\":\"+18108001001\"},\"caller_number_type\":{\"type\":\"string\",\"description\":\"The caller's number type.\",\"example\":\"toll_free_number\",\"enum\":[\"toll_free_number\",\"virtual_service_number\",\"byoc_number\"]},\"callee_number\":{\"type\":\"string\",\"description\":\"The callee's phone number.\",\"example\":\"+12055437350\"},\"callee_number_type\":{\"type\":\"string\",\"description\":\"The callee's number type.\",\"example\":\"toll_free_number\",\"enum\":[\"toll_free_number\",\"virtual_service_number\",\"byoc_number\"]},\"start_time\":{\"type\":\"string\",\"description\":\"The date and time when the call started, in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`.\",\"example\":\"2023-01-01T00:00:00-08:00\"},\"end_time\":{\"type\":\"string\",\"description\":\"The date and time when the call ended, in ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`.\",\"example\":\"2023-01-01T01:00:00-08:00\"},\"total_duration\":{\"type\":\"integer\",\"description\":\"The call log's total duration, in seconds.\",\"format\":\"int64\",\"example\":10},\"consumer_number\":{\"type\":\"string\",\"description\":\"The consumer's phone number.\",\"example\":\"+12059300920\"},\"consumer_display_name\":{\"type\":\"string\",\"description\":\"The consumer's name.\",\"example\":\"Tester\"},\"queue\":{\"type\":\"object\",\"properties\":{\"cc_queue_id\":{\"type\":\"string\",\"description\":\"The Contact Center queue's ID.\",\"example\":\"d95avl1eRJ-H162PZUJ-qg\"},\"queue_name\":{\"type\":\"string\",\"description\":\"The queue's name.\",\"example\":\"agentQueue\"}},\"description\":\"Information about the queue.\"},\"agents\":{\"type\":\"array\",\"description\":\"Information about the engagement's agents.\",\"items\":{\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The agent's ID.\",\"example\":\"ukAAkZKfROKMSw1bj_RDFQ\"},\"display_name\":{\"type\":\"string\",\"description\":\"The agent's name.\",\"example\":\"Jilly\"},\"user_email\":{\"type\":\"string\",\"description\":\"The agent's email.\",\"example\":\"jakie@zoom.us\"},\"user_ip_address\":{\"type\":\"string\",\"description\":\"The agent's device's IP address.\",\"format\":\"ipv4\",\"example\":\"38.145.73.4\"},\"user_device_mac_address\":{\"pattern\":\"^([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2})$\",\"type\":\"string\",\"description\":\"The agent's device's MAC address.\",\"example\":\"05:FA:15:25:EE:FF\"},\"user_registered_sip_zone\":{\"type\":\"string\",\"description\":\"The SIP zone where the agent registered.\",\"example\":\"ams1sbc\"},\"sip_zone_ip_address\":{\"type\":\"string\",\"description\":\"The SIP zone's IP address.\",\"format\":\"ipv4\",\"example\":\"38.145.73.4\"},\"user_region\":{\"type\":\"string\",\"description\":\"The agent's region name.\",\"example\":\"Main Region\"},\"team_ids\":{\"type\":\"array\",\"description\":\"The agent's team IDs\",\"items\":{\"type\":\"string\",\"description\":\"Agent team ID list.\",\"example\":\"3XilEfOvQEKRIWMWhX1jDg\"}},\"team_names\":{\"type\":\"array\",\"description\":\"The agent's team names.\",\"items\":{\"type\":\"string\",\"description\":\"Agent team name list.\",\"example\":\"My team\"}}},\"description\":\"Information about the queue.\"}},\"flow\":{\"type\":\"object\",\"properties\":{\"flow_id\":{\"type\":\"string\",\"description\":\"The flow's ID.\",\"example\":\"zeYjXoDOS_eV1QmTpj63PQ\"},\"flow_name\":{\"type\":\"string\",\"description\":\"The flow's name.\",\"example\":\"Demo\"}},\"description\":\"Information about the flow.\"},\"country_name\":{\"type\":\"string\",\"description\":\"The country name.\",\"example\":\"China\"},\"recorded\":{\"type\":\"boolean\",\"description\":\"Whether to record.\",\"example\":false},\"monitored\":{\"type\":\"boolean\",\"description\":\"Whether to monitor.\",\"example\":false},\"inbox\":{\"type\":\"boolean\",\"description\":\"Whether a message is in the box.\",\"example\":false},\"result\":{\"type\":\"string\",\"description\":\"The engagement's results.\",\"example\":\"completed\",\"enum\":[\"completed\",\"short_abandoned\",\"long_abandoned\",\"hold_abandoned\",\"long_calls\",\"short_calls\",\"hang_up_calls\",\"overflowed_to_disconnect\",\"overflowed_to_inbox\",\"overflowed\",\"abandon_quit\",\"auto_closed\",\"contained\",\"missed\",\"declined\",\"callback_abandoned_by_consumer\"]},\"caller_id\":{\"type\":\"string\",\"description\":\"The caller's number.\",\"example\":\"+12058945728\"},\"calling_party\":{\"type\":\"string\",\"description\":\"The calling's party.\",\"example\":\"consumer\",\"enum\":[\"consumer\",\"agent\"]},\"charge\":{\"type\":\"string\",\"description\":\"The bill's charge.\",\"example\":\"$0.019\"},\"rate\":{\"type\":\"string\",\"description\":\"The price for sent message.\",\"example\":\"$0.019\"},\"charge_type\":{\"type\":\"string\",\"description\":\"The charge type.\",\"example\":\"per_minute\",\"enum\":[\"per_minute\"]}},\"description\":\"Related call leg details\"}}},\"description\":\"Information about the voice call.\"}}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `300` <br>\\n The time and date must be in `yyyy-mm-dd`` or ISO 8601 format, either `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`. <br>\\n**Error Code:** `300` <br>\\n The requested report exceeds the 1-month limit. <br>\\n**Error Code:** `300` <br>\\n Invalid field. <br>\\n\"},\"401\":{\"description\":\"**HTTP Status Code:** `401` <br>\\n Unauthorized  \\n\\n \"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n \"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `1001` <br>\\n User does not exist: {userId}. <br>\\n**Error Code:** `1201` <br>\\n Queue does not exist: {queueId}. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[],\"x-macro-scopes\":[\"contact_center_voice_call:master\"],\"x-granular-scopes\":[\"contact_center:read:voice_call_log:master\"]}}}},\"components\":{\"securitySchemes\":{\"openapi_authorization\":{\"type\":\"http\",\"scheme\":\"bearer\"}}}}",
    "frontmatter": {
      "title": "Contact Center Master APIs",
      "keywords": "zoom, marketplace, rest, api, create, user, get, delete, contact center, custom, programmatic, task, queue, agent, supervisor, disposition, asset, skill, address, book, report, note",
      "description": "The Contact Center APIs allow developers to interface with [Contact Center](https://developers.zoom.us/docs/contact-center/) features programmatically.",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
