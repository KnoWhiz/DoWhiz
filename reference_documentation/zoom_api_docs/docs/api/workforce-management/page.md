# Workforce Management APIs

- Source URL: https://developers.zoom.us/docs/api/workforce-management/
- Snapshot path: docs/api/workforce-management
- Generated (UTC): 2026-02-07T22:40:11.992261+00:00

## Frontmatter

```json
{
  "title": "Workforce Management APIs",
  "keywords": "",
  "description": "Workforce Management",
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
    "title": "Workforce Management",
    "description": "Workforce Management",
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
    "/workforce-management/forecasts": {
      "get": {
        "tags": [
          "Forecasts"
        ],
        "summary": "List forecasts",
        "description": "List forecasts.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:forecasts:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\n\n**Not supported in Gov cluster** ",
        "operationId": "Listforecasts",
        "parameters": [
          {
            "name": "name",
            "in": "query",
            "description": "The forecast name.",
            "required": false,
            "schema": {
              "maxLength": 36,
              "type": "string",
              "example": "Weekday forecast"
            },
            "example": "Weekday forecast"
          },
          {
            "name": "scheduling_group_name",
            "in": "query",
            "description": "The scheduling group name.",
            "required": false,
            "schema": {
              "maxLength": 36,
              "type": "string",
              "example": "Infra support"
            },
            "example": "Infra support"
          },
          {
            "name": "status",
            "in": "query",
            "description": "The forecast status.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "ready_step_1",
              "enum": [
                "ready_step_1",
                "processing",
                "ready_step_2",
                "completed",
                "failed",
                "applied",
                "generating",
                "generating_failed",
                "reforecasting",
                "reforecasting_failed"
              ]
            },
            "example": "ready_step_1"
          },
          {
            "name": "type",
            "in": "query",
            "description": "The forecast type.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "SHORT_TERM",
              "enum": [
                "SHORT_TERM",
                "LONG_TERM"
              ]
            },
            "example": "SHORT_TERM"
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "The next page token from the previous page, if available. Not required for the first page.",
            "required": false,
            "schema": {
              "maxLength": 36,
              "type": "string",
              "example": "cm2n7z2vl000e9cllp4ckoc45"
            },
            "example": "cm2n7z2vl000e9cllp4ckoc45"
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of items to be returned per page.",
            "required": false,
            "schema": {
              "maximum": 50,
              "minimum": 1,
              "type": "number",
              "example": 50,
              "default": 10
            },
            "example": 50
          },
          {
            "name": "division_id",
            "in": "query",
            "description": "The division ID.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "ya2kZo77RE6LFqaoKwWM0g"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Forecast list response.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "forecasts",
                    "next_page_token",
                    "page_size",
                    "total_records"
                  ],
                  "type": "object",
                  "properties": {
                    "next_page_token": {
                      "description": "Use the next page token to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size.",
                      "anyOf": [
                        {
                          "type": "string",
                          "example": "cm2n7z2vl000e9cllp4ckoc45"
                        },
                        {
                          "type": "null"
                        }
                      ]
                    },
                    "page_size": {
                      "type": "integer",
                      "description": "The number of items returned per page.",
                      "example": 10
                    },
                    "total_records": {
                      "type": "integer",
                      "description": "The total number of items available across all pages.",
                      "example": 100
                    },
                    "forecasts": {
                      "type": "array",
                      "description": "List of forecasts.",
                      "items": {
                        "required": [
                          "create_time",
                          "description",
                          "end",
                          "forecast_id",
                          "forecast_profile_id",
                          "interval_minutes",
                          "modify_time",
                          "name",
                          "scheduling_groups",
                          "start",
                          "status",
                          "type"
                        ],
                        "type": "object",
                        "properties": {
                          "forecast_id": {
                            "maxLength": 30,
                            "type": "string",
                            "description": "The forecast ID.",
                            "example": "cm2n7z2vl000e9cllp4ckoc45"
                          },
                          "name": {
                            "maxLength": 36,
                            "type": "string",
                            "description": "The forecast name.",
                            "example": "Weekday forecast"
                          },
                          "description": {
                            "type": "string",
                            "description": "The forecast description.",
                            "example": "Forecast for business days"
                          },
                          "type": {
                            "type": "string",
                            "description": "The forecast type.",
                            "example": "SHORT_TERM",
                            "enum": [
                              "SHORT_TERM",
                              "LONG_TERM"
                            ]
                          },
                          "status": {
                            "type": "string",
                            "description": "The forecast status.",
                            "example": "ready_step_1",
                            "enum": [
                              "ready_step_1",
                              "processing",
                              "ready_step_2",
                              "completed",
                              "failed",
                              "applied",
                              "generating",
                              "generating_failed",
                              "reforecasting",
                              "reforecasting_failed"
                            ]
                          },
                          "start": {
                            "type": "string",
                            "description": "The forecast start.",
                            "format": "date-time",
                            "example": "2025-01-01T00:00:00Z"
                          },
                          "end": {
                            "type": "string",
                            "description": "The forecast end.",
                            "format": "date-time",
                            "example": "2025-01-06T23:59:59Z"
                          },
                          "create_time": {
                            "type": "string",
                            "description": "The forecast request create time.",
                            "format": "date-time",
                            "example": "2024-12-16T04:21:23.207Z"
                          },
                          "modify_time": {
                            "type": "string",
                            "description": "The forecast request modify time.",
                            "format": "date-time",
                            "example": "2024-12-16T04:21:23.207Z"
                          },
                          "interval_minutes": {
                            "maximum": 60,
                            "minimum": 15,
                            "type": "number",
                            "description": "The forecast granularity length, in minutes.",
                            "example": 15
                          },
                          "forecast_profile_id": {
                            "description": "The forecast profile ID.",
                            "anyOf": [
                              {
                                "type": "string",
                                "example": "cm2n7z2vl000e9cllp4ckoc45"
                              },
                              {
                                "type": "null"
                              }
                            ]
                          },
                          "scheduling_groups": {
                            "type": "array",
                            "description": "List of scheduling groups present in the forecast.",
                            "items": {
                              "required": [
                                "asa",
                                "color_code",
                                "deferrable_sla",
                                "deferrable_sla_target_time",
                                "dynamic_shrinkage",
                                "fixed_shrinkage",
                                "name",
                                "occupancy",
                                "scheduling_group_id",
                                "sla",
                                "sla_target_time"
                              ],
                              "type": "object",
                              "properties": {
                                "scheduling_group_id": {
                                  "maxLength": 30,
                                  "type": "string",
                                  "description": "The scheduling group ID.",
                                  "example": "cm2n7z2vl000e9cllp4ckoc45"
                                },
                                "name": {
                                  "maxLength": 36,
                                  "type": "string",
                                  "description": "The scheduling group name.",
                                  "example": "Infra support"
                                },
                                "color_code": {
                                  "description": "The RGB hexadecimal color code.",
                                  "anyOf": [
                                    {
                                      "type": "string",
                                      "example": "#c0106f"
                                    },
                                    {
                                      "type": "null",
                                      "description": "Null if color code is not present."
                                    }
                                  ]
                                },
                                "sla": {
                                  "maximum": 100,
                                  "minimum": -1,
                                  "type": "number",
                                  "description": "Service level target goal percentage. `-1` if not set.",
                                  "example": 75,
                                  "default": -1
                                },
                                "sla_target_time": {
                                  "minimum": -1,
                                  "type": "integer",
                                  "description": "Service level target time in seconds. `-1` if not set.",
                                  "example": 30,
                                  "default": -1
                                },
                                "asa": {
                                  "minimum": -1,
                                  "type": "integer",
                                  "description": "Average speed of answer in seconds. `-1` if not set.",
                                  "example": 30,
                                  "default": -1
                                },
                                "occupancy": {
                                  "maximum": 100,
                                  "minimum": 0,
                                  "type": "number",
                                  "description": "Occupancy percentage. `0` if not set.",
                                  "example": 85,
                                  "default": 0
                                },
                                "fixed_shrinkage": {
                                  "description": "Fixed shrinkage value to use for all days.",
                                  "anyOf": [
                                    {
                                      "maximum": 99,
                                      "minimum": 0,
                                      "type": "number",
                                      "description": "Shrinkage percentage.",
                                      "example": 20
                                    },
                                    {
                                      "type": "null",
                                      "description": "Value will be `null` if `dynamic_shrinkage` is present."
                                    }
                                  ]
                                },
                                "dynamic_shrinkage": {
                                  "description": "Dynamic shrinkage value to use, based on configured days.",
                                  "anyOf": [
                                    {
                                      "required": [
                                        "default_shrinkage",
                                        "dynamic_config"
                                      ],
                                      "type": "object",
                                      "properties": {
                                        "default_shrinkage": {
                                          "maximum": 99,
                                          "minimum": 0,
                                          "type": "number",
                                          "description": "Shrinkage percentage.",
                                          "example": 20
                                        },
                                        "dynamic_config": {
                                          "type": "array",
                                          "description": "List of shrinkage values for required intervals.",
                                          "items": {
                                            "required": [
                                              "days",
                                              "end_time",
                                              "start_time",
                                              "value"
                                            ],
                                            "type": "object",
                                            "properties": {
                                              "days": {
                                                "type": "array",
                                                "description": "List of days this configuration will apply for.",
                                                "items": {
                                                  "type": "number",
                                                  "description": "`0` - Sunday \n`1` - Monday\n`2` - Tuesday\n`3` - Wednesday\n`4` - Thursday\n`5` - Friday\n`6` - Saturday",
                                                  "example": 1,
                                                  "enum": [
                                                    0,
                                                    1,
                                                    2,
                                                    3,
                                                    4,
                                                    5,
                                                    6
                                                  ]
                                                }
                                              },
                                              "start_time": {
                                                "maximum": 1440,
                                                "minimum": 0,
                                                "type": "integer",
                                                "description": "The starting minute of a day to apply this shrinkage value.",
                                                "example": 600
                                              },
                                              "end_time": {
                                                "maximum": 1440,
                                                "minimum": 0,
                                                "type": "integer",
                                                "description": "The ending minute of a day until when to apply this shrinkage value.",
                                                "example": 660
                                              },
                                              "value": {
                                                "maximum": 99,
                                                "minimum": 0,
                                                "type": "number",
                                                "description": "Shrinkage percentage.",
                                                "example": 20
                                              }
                                            }
                                          }
                                        }
                                      }
                                    },
                                    {
                                      "type": "null",
                                      "description": "Value will be `null` if `fixed_shrinkage` is present.",
                                      "example": ""
                                    }
                                  ]
                                },
                                "deferrable_sla": {
                                  "maximum": 100,
                                  "minimum": -1,
                                  "type": "number",
                                  "description": "Deferrable channels interactions' percentage. `-1` if not set.",
                                  "example": 75,
                                  "default": -1
                                },
                                "deferrable_sla_target_time": {
                                  "minimum": -1,
                                  "type": "integer",
                                  "description": "Deferrable channels interactions' target time in hours. `-1` if not set.",
                                  "example": 30,
                                  "default": -1
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
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](/docs/api/rest/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-macro-scopes": [
            "workforce_management:read:admin"
          ],
          "x-granular-scopes": [
            "workforce_management:read:forecasts:admin"
          ]
        }
      }
    },
    "/workforce-management/forecasts/{forecastId}/scheduling-groups/{schedulingGroupId}": {
      "get": {
        "tags": [
          "Forecasts"
        ],
        "summary": "Get a forecast for a scheduling group",
        "description": "List forecasted values for a scheduling group within a forecast.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:forecasts:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "Getforecastofschedulinggroup",
        "parameters": [
          {
            "name": "forecastId",
            "in": "path",
            "description": "The forecast ID.",
            "required": true,
            "schema": {
              "maxLength": 30,
              "type": "string",
              "example": "cm2n7z2vl000e9cllp4ckoc45"
            },
            "example": "cm2n7z2vl000e9cllp4ckoc45"
          },
          {
            "name": "schedulingGroupId",
            "in": "path",
            "description": "The scheduling group ID.",
            "required": true,
            "schema": {
              "maxLength": 30,
              "type": "string",
              "example": "cm2n7z2vl000e9cllp4ckoc45"
            },
            "example": "cm2n7z2vl000e9cllp4ckoc45"
          },
          {
            "name": "division_id",
            "in": "query",
            "description": "The division ID.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "ya2kZo77RE6LFqaoKwWM0g"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Forecast scheduling group response.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "forecast_id",
                    "forecast_intervals",
                    "scheduling_group_id"
                  ],
                  "type": "object",
                  "properties": {
                    "forecast_id": {
                      "maxLength": 30,
                      "type": "string",
                      "description": "The forecast ID.",
                      "example": "cm2n7z2vl000e9cllp4ckoc45"
                    },
                    "scheduling_group_id": {
                      "maxLength": 30,
                      "type": "string",
                      "description": "The scheduling group ID.",
                      "example": "cm2n7z2vl000e9cllp4ckoc45"
                    },
                    "forecast_intervals": {
                      "maxItems": 1000,
                      "type": "array",
                      "description": "List of forecasted values per interval.",
                      "items": {
                        "required": [
                          "forecast",
                          "timestamp"
                        ],
                        "type": "object",
                        "properties": {
                          "timestamp": {
                            "type": "string",
                            "description": "The interval's starting timestamp.",
                            "format": "date-time",
                            "example": "2025-01-01T00:00:00Z"
                          },
                          "forecast": {
                            "required": [
                              "agents"
                            ],
                            "type": "object",
                            "properties": {
                              "agents": {
                                "type": "number",
                                "description": "The forecasted staffing count.",
                                "example": 5
                              },
                              "voice": {
                                "required": [
                                  "aht",
                                  "e_aht",
                                  "e_vol",
                                  "vol"
                                ],
                                "type": "object",
                                "properties": {
                                  "aht": {
                                    "type": "number",
                                    "description": "The forecasted average handle time, in seconds.",
                                    "example": 40
                                  },
                                  "e_aht": {
                                    "type": "number",
                                    "description": "The edited average handle time, in seconds. `-1` if not edited.",
                                    "example": 38
                                  },
                                  "vol": {
                                    "type": "number",
                                    "description": "The forecasted volume.",
                                    "example": 10
                                  },
                                  "e_vol": {
                                    "type": "number",
                                    "description": "The edited volume. `-1` if not edited.",
                                    "example": 12
                                  }
                                },
                                "description": "Interval forecast for all queues in scheduling group of channel category `voice`."
                              },
                              "messaging": {
                                "required": [
                                  "aht",
                                  "e_aht",
                                  "e_vol",
                                  "vol"
                                ],
                                "type": "object",
                                "properties": {
                                  "aht": {
                                    "type": "number",
                                    "description": "The forecasted average handle time, in seconds.",
                                    "example": 40
                                  },
                                  "e_aht": {
                                    "type": "number",
                                    "description": "The edited average handle time, in seconds. `-1` if not edited.",
                                    "example": 38
                                  },
                                  "vol": {
                                    "type": "number",
                                    "description": "The forecasted volume.",
                                    "example": 10
                                  },
                                  "e_vol": {
                                    "type": "number",
                                    "description": "The edited volume. `-1` if not edited.",
                                    "example": 12
                                  }
                                },
                                "description": "Interval forecast for all queues in scheduling group of channel category `messaging`."
                              },
                              "deferrable": {
                                "required": [
                                  "aht",
                                  "e_aht",
                                  "e_vol",
                                  "vol"
                                ],
                                "type": "object",
                                "properties": {
                                  "aht": {
                                    "type": "number",
                                    "description": "The forecasted average handle time, in seconds.",
                                    "example": 40
                                  },
                                  "e_aht": {
                                    "type": "number",
                                    "description": "The edited average handle time, in seconds. `-1` if not edited.",
                                    "example": 38
                                  },
                                  "vol": {
                                    "type": "number",
                                    "description": "The forecasted volume.",
                                    "example": 10
                                  },
                                  "e_vol": {
                                    "type": "number",
                                    "description": "The edited volume. `-1` if not edited.",
                                    "example": 12
                                  }
                                },
                                "description": "Interval forecast for all queues in scheduling group of channel category `deferrable`."
                              }
                            },
                            "description": "Forecasted values."
                          },
                          "reforecast": {
                            "required": [
                              "agents"
                            ],
                            "type": "object",
                            "properties": {
                              "agents": {
                                "type": "number",
                                "description": "The forecasted staffing count.",
                                "example": 5
                              },
                              "voice": {
                                "required": [
                                  "aht",
                                  "e_aht",
                                  "e_vol",
                                  "vol"
                                ],
                                "type": "object",
                                "properties": {
                                  "aht": {
                                    "type": "number",
                                    "description": "The forecasted average handle time, in seconds.",
                                    "example": 40
                                  },
                                  "e_aht": {
                                    "type": "number",
                                    "description": "The edited average handle time, in seconds. `-1` if not edited.",
                                    "example": 38
                                  },
                                  "vol": {
                                    "type": "number",
                                    "description": "The forecasted volume.",
                                    "example": 10
                                  },
                                  "e_vol": {
                                    "type": "number",
                                    "description": "The edited volume. `-1` if not edited.",
                                    "example": 12
                                  }
                                },
                                "description": "Interval forecast for all queues in scheduling group of channel category `voice`."
                              },
                              "messaging": {
                                "required": [
                                  "aht",
                                  "e_aht",
                                  "e_vol",
                                  "vol"
                                ],
                                "type": "object",
                                "properties": {
                                  "aht": {
                                    "type": "number",
                                    "description": "The forecasted average handle time, in seconds.",
                                    "example": 40
                                  },
                                  "e_aht": {
                                    "type": "number",
                                    "description": "The edited average handle time, in seconds. `-1` if not edited.",
                                    "example": 38
                                  },
                                  "vol": {
                                    "type": "number",
                                    "description": "The forecasted volume.",
                                    "example": 10
                                  },
                                  "e_vol": {
                                    "type": "number",
                                    "description": "The edited volume. `-1` if not edited.",
                                    "example": 12
                                  }
                                },
                                "description": "Interval forecast for all queues in scheduling group of channel category `messaging`."
                              },
                              "deferrable": {
                                "required": [
                                  "aht",
                                  "e_aht",
                                  "e_vol",
                                  "vol"
                                ],
                                "type": "object",
                                "properties": {
                                  "aht": {
                                    "type": "number",
                                    "description": "The forecasted average handle time, in seconds.",
                                    "example": 40
                                  },
                                  "e_aht": {
                                    "type": "number",
                                    "description": "The edited average handle time, in seconds. `-1` if not edited.",
                                    "example": 38
                                  },
                                  "vol": {
                                    "type": "number",
                                    "description": "The forecasted volume.",
                                    "example": 10
                                  },
                                  "e_vol": {
                                    "type": "number",
                                    "description": "The edited volume. `-1` if not edited.",
                                    "example": 12
                                  }
                                },
                                "description": "Interval forecast for all queues in scheduling group of channel category `deferrable`."
                              }
                            },
                            "description": "Re-forecasted values."
                          }
                        }
                      }
                    }
                  },
                  "description": "Forecast scheduling group response."
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
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
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-macro-scopes": [
            "workforce_management:read:admin"
          ],
          "x-granular-scopes": [
            "workforce_management:read:forecasts:admin"
          ]
        }
      }
    },
    "/workforce-management/imports/historical-queue-metrics": {
      "post": {
        "tags": [
          "Imports"
        ],
        "summary": "Upload historical queue metrics",
        "description": "Import historical queue metrics to Workforce Management (WFM).\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:write:queue_metrics:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `HEAVY`\n\n**Not supported in Gov cluster** ",
        "operationId": "Uploadhistoricalqueuemetrics",
        "requestBody": {
          "content": {
            "multipart/form-data": {
              "schema": {
                "type": "object",
                "properties": {
                  "file": {
                    "type": "string",
                    "description": "Attachment file. Only CSV format is supported. For format details, log in to WFM and visit **Forecasting -> Historical data -> Import historical data**.",
                    "format": "binary",
                    "example": "Vm0wd2QyUXlVWGxWV0d4V1YwZDRWMVl3WkRSV01WbDNXa1JTVjAxV2JETlhhMUpUVmpBeFYySkVUbGhoTVVwVVZtcEJlRll5U2tWVWJHaG9UVlZ3VlZadGNFSmxSbGw1VTJ0V1ZXSkhhRzlVVmxaM1ZsWmFkR05GU214U2JHdzFWVEowVjFaWFNraGhSemxWVm14YU0xWnNXbUZrUjA1R1UyMTRVMkpIZHpGV1ZFb3dWakZhV0ZOcmFHaFNlbXhXVm0xNFlVMHhXbk5YYlVaclVqQTFSMVV5TVRSVk1rcElaSHBHVjFaRmIzZFdha1poVjBaT2NtRkhhRk5sYlhoWFZtMHhORmxWTUhoWGJrNVlZbFZhY2xWcVFURlNNVlY1VFZSU1ZrMXJjRWxhU0hCSFZqRmFSbUl6WkZkaGExcG9WakJhVDJOdFJraGhSazVzWWxob1dGWnRNSGhPUm14V1RVaG9XR0pyTlZsWmJGWmhZMnhXY1ZGVVJsTk5WbFkxVkZaU1UxWnJNWEpqUld4aFUwaENTRlpxUm1GU2JVbDZXa1prYUdFeGNHOVdha0poVkRKT2RGSnJhR2hTYXpWeldXeG9iMWRHV25STlNHaFBVbTE0VjFSVmFHOVhSMHB5VGxac1dtSkdXbWhaTW5oWFkxWkdWVkpzVGs1V2JGa3hWa1phVTFVeFduSk5XRXBxVWxkNGFGVXdhRU5UUmxweFVtMUdVMkpWYkRaWGExcHJZVWRGZUdOSE9WZGhhMHBvVmtSS1QyUkdTbkpoUjJoVFlYcFdlbGRYZUc5aU1XUkhWMjVTVGxOSGFGQlZiVEUwVmpGU1ZtRkhPVmhTTUhCNVZHeGFjMWR0U2tkWGJXaGFUVzVvV0ZreFdrZFdWa3B6VkdzMVYySkdhM2hXYTFwaFZURlZlRmR1U2s1WFJYQnhWV3hrTkdGR1ZYZGhSVTVVVW14d2VGVnRNVWRWTWtwV1lrUmFXR0V4Y0hKWlZXUkdaVWRPU0U5V1pHaGhNSEJ2Vm10U1MxUXlVa2RUYmtwb1VqSm9WRmxZY0ZkbGJHUllaVWM1YVUxWFVraFdNalZUVkd4T1NHRkdRbFppVkVVd1ZtcEdVMVp0UmtoUFZtaFRUVWhDTlZaSGVHRmpNV1IwVTJ0a1dHSlhhR0ZVVnpWdlYwWnJlRmRyWkZkV2EzQjZWa2R6TVZZeVNrZGhNMmhYWVRGd2FGWlVSbFpsUm1SMVUyczFXRkpZUW5oV1YzaHJUa2RHUjFaWVpHaFNWVFZWVlcxNGQyVkdWblJOVldSV1RXdHdWMWxyVW1GWFIwVjRZMGhLV2xaWFVrZGFWV1JQVTBVNVYxcEhhR2hOU0VKMlZtMTBVMU14VVhsVmEyUlVZbXR3YjFWcVNtOVdSbXhaWTBaa2JHSkhVbGxhVldNMVlWVXhXRlZyYUZkTmFsWlVWa2Q0VDFOSFJrZFJiRnBwVmtWVmQxWnRjRWRWTVZwMFVtdG9VRlp0YUZSVVZXaERUbFphU0dWSFJtcE5WMUl3VlRKMGExZEhTbGhoUjBaVlZteHdNMWxWV25kU2JIQkhWR3hTVTJFelFqVldSM2hoVkRKR1YxTnVVbEJXUlRWWVZGYzFiMWRHWkZkWGJFcHNVbTFTZWxsVldsTmhWa3AxVVc1b1YxWXphSEpXVkVaelZqRldjMWRzYUdsV1ZuQlFWa1phWVdReVZrZFdXR3hyVWtWS1dGUldXbmRsVm10M1YyNWtXRkl3VmpSWk1GSlBWakpHY2xkcmVGZGhhM0JRVlRGa1MxSXhjRWRhUms1WFYwVktNbFp0TVRCVk1VMTRWVmhzVlZkSGVGWlpWRVozWWpGV2NWUnJUbGRTYlhoNVZtMDFhMVl4V25OalJFSmhWbGROTVZaWGMzaFhSbFoxWTBaa1RsWXlhREpXYWtKclV6RktjazVXWkZaaVJscFlWRlJLYjFOV1draGxSMFpYVFd4S1NWWlhkRzloTVVsNVlVaENWbUpIYUVOYVJFWmhZekZ3UlZWdGNFNVdNVWwzVmxSS01HRXhaRWhUYkdob1VqQmFWbFp1Y0Zka2JGbDNWMjVPVDJKRmNIcFhhMlIzWVZaT1JsTnJiRmROYmxKeVdYcEdWbVZXVG5WVGJGSnBWbFp3V1ZaR1l6RmlNV1JIV2taa1dHSkZjSE5WYlRGVFpXeHNWbGRzVG1oV2EzQXhWVmQ0ZDFZeVNsbFZiR2hoVWtWYVlWcFZaRXRUVms1ellVZG9UazFWY0ZkV2JHTjRUa2RSZVZaclpGZFhSM2h5VldwQ1lXTkdWblJsU0dSc1lrWnNOVnBWYUd0WFJrbDNWbXBTV2sxR1NsQldNakZHWlZaV2NscEhSbGROTW1oSlYxUkplRk14U1hsU2EyUm9VbXhLVkZac2FFTlRNVnAwVFZSQ1ZrMVZNVFJXYkdodlYwWmtTR0ZHYkZwaVdHaG9WbXBHYzJNeGNFaFBWbVJUWWxob1lWZFVRbUZoTWtwSVUydG9WbUpIZUdoV2JHUk9UVlpzVjFaWWFGaFNiRnA1V1ZWYWExUnRSbk5YYkZaWFlUSlJNRlpFUms5U01WcDFWbXhXYVZKdVFuWlhWbEpEWkRBMWMxZHJhR3RTTUZwWVdXeGFZVmRXV2xoa1J6bG9UVlZ3TUZsVldsTldWbHBZWVVWU1ZrMXVhR2haZWtaM1VsWldkR05GTlZkTlZXd3pWbXhTUzAxSFJYaGFSV2hVWWtkb2IxVnFRbUZXYkZwMFpVaGtUazFYZUZkV01qVnJWVEpLU1ZGcmFGZFNNMmhVVmxSS1JtVnNSbkZXYkdSVFRUSm9iMVpyVWt0U01WbDRWRzVXVm1KRlNsaFZiRkpYVjFaYVIxbDZSbWxOVjFKSVdXdG9SMVpIUlhoalNFNVdZbFJHVkZZeWVHdGpiRnBWVW14a1RsWnVRalpYVkVKaFZqRmtSMWR1VGxSaE1taG9WV3RXWVZsV2NGWmFSWFJVVm1zMWVsbFZaRzlVYXpGV1kwWmtWMkpIVGpSVWEyUlNaVlphY2xwR1pHbGlSWEJRVm0xNGExVXhXWGhWYkdoclUwZFNXRlJXWkRSbFZscFlUVlZrV0ZKcmJETldiWEJUVjJzeFNHRkZlRmROYm1ob1ZqQmFWMk5zY0VoU2JHUlhUVlZ3VWxac1VrTldhelZYVjFob2FsSlhhRzlWYWtwdlZERlZkMVpyZEU1aVJuQXdWRlpTUTFack1WWk5WRkpYVm0xb2VsWnRNVVpsVmxaelZteHdhVmRHU1hwWFYzQkhWakpPVjFSdVVsQldiVkpVV1d4b2IxbFdaRlZSYlVab1RXdHdTVlV5ZEc5V2JVcElaVWRvVjJKSFVrOVVWbHB6VmpGYVdXRkdhRk5pUm5BMVYxWldZV0V4VW5SU2JrNVlZa1phV0ZsVVNsSk5SbXhWVW1zNVUwMVdjREZXUjNodllWWktjMk5HYkZoV00xSjJWVlJHVDFZeFpISmFSbHBvVFd4S1dGWkdXbUZrTURGSFZtNVNhMUo2YkZkVmJYaHpUbFpzVm1GRlRsZGlWWEJKV1ZWV1UxWlhTbGxoU0VwWFlrWndTRnBGV2t0a1IwNUdUbFprVGxaWGQzcFdiWGhUVXpBeFNGSllhR0ZTVjJoVldXdGtiMkl4Vm5GUmJVWlhZa1p3TUZwVmFHdFVhekZZWlVaa1YwMXFWbkpXVkVwTFUxWkdjbUZHWkZOTk1taFZWbXhTUzFKdFZuTlNia1pYWWtkU1ZGUlVTbTlpTVZweVZXdDBVMDFXYkRSV1J6VlhWbTFLUmxOc2FGWmlSa3BZVmpGYVlWSXhiRFpTYld4T1ZqRktTVmRYZEc5U01WcElVbGhvYWxORk5WZFpiRkpIVmtaWmVXVkhkR3BpUm5CV1ZXMTRhMVJzV25WUmFscFlWa1ZLYUZacVJtdFNNV1IxVkd4U2FFMXRhRzlXVjNSWFdWWnNWMk5HV21GU1dGSlZWbTF6TVdWc2JGWmFSemxWWVhwR1Yxa3dXbXRXTWtwSVZHcFNWV0V5VWxOYVZscGhZMnh3UjFwSGJHbFNXRUpSVm0weE5HRXhWWGhYV0doV1lrZG9jbFV3WkZOWFJsSlhXa1JTYkZac2NGbFVWbFpyVjBaS2NtSkVUbGRpV0VKVVZqSnplRk5IUmtabFJtUk9ZbTFvYjFacVFtRldNazV6WTBWb1UySkhVbGhVVmxaM1ZXeGFjMVZyVGxkaGVsWllWakZvYjJGc1NsaGhSemxXWVd0d2RsWkVSbFprTVZweVpFVTFhVkp1UVhkV1JscFRVVEZhY2sxV1drNVdSa3BZVm0weGIyVnNXblJOVlZwc1ZteGFlbFp0ZUhkaFZtUkhVMWh3V0ZZelFraFdha3BQVmpGU2RWVnNRbGRpVmtwVlYxZDRiMkl4YkZkYVJsWlNZbFZhYjFSWGRIZFRWbFY1WkVjNVYySlZjRWxhVldSdlZqSktTRlZyT1ZWV2JIQjZWbXBHWVZkWFJrZGhSazVwVW01Qk1WWXhXbGRaVjBWNFZXNVNVMkpyTlZsWmExcGhWMFpzVlZOc1NrNVNiSEJHVlcxek5WWkdTbkpqU0d4WVlURndURmxXV2t0a1ZrWjFXa1prYVZkR1NsbFdiVEI0VlcxV1IxcElWbFppV0VKVVZtcEdTMWRHV2tkWGJVWnJUVVJHU0ZadE5WTmhNVW8yWWtaa1ZtSllhRE5VVlZwelZteGtjMVJzWkZkaVNFSmFWMVpXVjFVeFdsaFRhMlJxVWpKb1YxUlZaRk5YUmxWM1YydDBhMUl3TlVkVWJGcHJWR3hhV0dRemNGZGlXR2hVVlhwQmVGTkdTbGxoUjBaVFZqSm9WbGRYZEd0aU1rbDRWbTVHVW1KVldsaFphMXAzVFZacmQxZHRkR2hOYTNCSVdXdFNUMVl3TVhGV2EzaGFZVEZ3VEZwRldsZGtWMHBIWVVkb1RsZEZTalZXYlRGM1V6RktkRlp1VGxOaWExcFpXV3RrVTJJeFVsaGxSWEJPWWtad1NGWXlNVWRYUjBwWFVtcE9WVlpzY0hKV01HUkxWMGRXU1ZSc2NGZFNWbTk2Vm1wR1lXRXhaRWhXYTJoUVZtdHdUMVp0ZEhkVFZscHpXWHBHVkUxWFVrbFZNblJoWVd4T1JrNVdaRnBpUmtwSVZtdGFkMWRIVmtsVWJHUnBVakZLTmxaclkzaGlNVmw1VWxod1VsZEhhRmhXYlRGU1RVWndSVkpzY0d4V2F6VjZXV3RhWVdGV1NYbGhSemxYVmpOU1dGZFdXbk5XTWs1SFYyMUdVMWRHU2xaV2JURjZUVmRXYzJKR1dtRlNlbXhXV1Zod1IxWXhhM2RYYlhSWFRXdHdTVlpIY0ZOV1YwVjVWV3hPWVZac2NHaFpNbmgzVWpGd1NHSkhiRk5YUlVwU1ZtMHdlRTVIUlhoV1dHaFlWMGRvVjFsclpHOWpiRlYzV2taT1dGSnNTbGhYYTFKVFlrWmFjMk5HYkZWV2JGcHlWakJhUzJOdFNrVlViR1JwVjBWS1ZWWnFTbnBsUms1SVVtdGFiRkp0VWs5WmJURnZZakZhY1ZGdFJsZE5helY2V1RCV2IxVXlTa2hWYkdoVlZteGFNMVpYZUdGak1WWnlWR3hrYUdWc1dsbFdha1p2WWpKR2MxTnNhR2hTZW14WFdWZDBkMlJzV2tWU2JGcHJUVlp3ZVZwRlZURmhWa3AxVVZoa1YxSnNjRlJWVkVaaFkyc3hWMWRyTlZkU2EzQlpWbTB3ZUdJeVZuTlhibEpPVmxad2MxWnRlR0ZsYkZwMFpVaGthRlp0VWtoVk1XaDNWMFpaZWxGcmFGZGhhM0JRVm1wR1UyUldWbk5SYkdScFZtdHdWbFl4WkRSaU1rbDRZa1prWVZKc1dsTlpiRlpoWWpGU1dHUklaRlJXYkZZMVdrVmpOVll5U2xaalJXeGhWbGRTZGxacVNrdFRSbFp5VDFaV1YySklRalpXYlhCSFdWWmtXRkpyWkdoU2F6VndWVzAxUWsxV1dYaFhiR1JhVmpCV05GWlhOVk5WTWtweVRsWnNXbUV4Y0doV01GcHpWbFpPYzFOck5WTmlXR2hYVmxjeGVrNVdWWGROVm1ScVVrVmFXRlZxVG05a2JHeFhWMnQwYWsxck5VaFphMXB2VmpBd2VGTnFTbGRXYkVwSVZrUkdXbVZHY0VsVGJVWlRZa2hDZGxaR1pEUlRNV1JIVjJ0a1dHSlZXbkpVVlZKSFUwWmFkRTVXVG1oTlZXOHlXV3RhYTFadFNsbGhTSEJWWWtad2VsWnRNVWRTYkZKeldrZHNWMWRGU2t0V01WcFhWakZWZUZkc2FGUmlSM2h2VlRCV2QxZEdiSEpYYm1SVVVtNUNSMVl5ZERCaE1VbDNUbFZrVldKR2NISldSM2hoVjBkUmVtTkdaR2xYUjJoVlZsaHdRbVZHVGtkVGJHeG9VakJhVkZacVNtOVdiR1JZWkVkMGFVMXJiRFJXYlRWSFZXMUtWbGRzYUZwaE1YQXpWRlphY21ReFpIUmtSMmhPWVROQ1NWZFhkRk5VTVZsM1RWaEdWMkV6YUdGWlZFWjNWRVp3Umxkc1pHdFdNSEJJV1ZWa2IxUnNaRVpUYWxaWFlsUkdNMVY2Umt0a1JscDFWR3hPYVdFeGNGcFhWM1JyVlRKSmVGVnNhR3hTV0VKUFdXdGFkMU5HV1hsTlZXUm9VbFJHVjFsclVsTldNVW8yVm14Q1YwMUhVa3hXTVZwSFl6RldjMXBIYUdobGJGbDVWbTF3UjFsWFJYaGFSV2hYWVRKb1VWWnRkSGRVTVZwMFpFaGtWRlpzY0hwWFdIQkhWa1V4V0ZWcmJGWk5ibEpvV1ZkNFQxSnJOVmRhUm5CcFVtdHdTVlp0ZEdGa01XUklWbXRzVldKSFVuQlZha1pMVG14YWNsa3phR2xOVmxZMFZqSjBZVmRIUm5OalJtaFhZVEZhZVZwVlduTmpWa3B6WTBkNFUySldTalJXYWtvMFV6SkdTRk5yV21wU2JWSlpWbTE0UzFSR1dsVlNhemxUVFZad01WVnRlRTloVmxwWFkwWndWMkpVUVhoVlZFWlBWakpLUjFkc2FHaGxiWGhaVmtaYVlXUXhUa2RYV0d4c1VqTlNXRlJYZEhkVFZscElZMFpPVjFZd1ZqVldWM2hQV1ZaYWMyTkhhRnBOYm1nelZXcEdkMU5IU2toaVJrNVlVbFZ3VTFadE1IZGxSVFZJVWxob1YxZEhhRmxXTUdSdlZqRnNjbHBIT1dwaVJsWXpWMnRhVDFkR1NuTlRiR2hZWVRGS1ZGbHJXbUZqTWs1SFlrWldWMkpHY0hsV2FrSmhVekpPZEZOclZsVmlSMUp3VmpCV1MxWldXbkZUYm5Cc1VtczFTVlZ0ZEdGaVJrcDBWV3M1V21KVVJuWlpha1poWTFaR2RGSnNaRTVoZWxZMlYxUkNWMkl4VlhsVGEyaFdZa2RvVmxadGVHRk5NVnBZWlVkR2FrMVdXbmxXUjNocllVZFdjMWRzYkZkaGExcDJXV3BLUjJNeFRuTmhSMmhUWlcxNFdGZFdaREJrTWxKelYydFdVMkpHY0hKVVZscDNaVlp3UmxaVVJtaFdhM0F4VlZab2ExWXhTbk5qUmxKV1ZrVmFhRmt5YzNoV01XUnlUbFprVTJFelFscFdiVEIzWlVkSmVWWnVUbGhpYkVwUFZteGFkMk14V25SbFIwWnNZa1p3TUZwVmFHdGhSbHAwVld0b1ZrMVhhRE5XTUZwYVpXeEdjbUZHWkdoaE0wSlJWbTF3UjFVeVVsZFdiazVTWWtkU2NGWnRkSGRXYkZsNFdrUlNhVTFyYkRSWmExcHJXVlpLYzFkc1RscGlSbHBvV1dwR1UxZEhVa2xhUmxKWFlrWlpNVmRyVm1wT1ZsbDVVMnhhYWxKWGVHaFdiR1JUWkd4YWNWTnJkRlJTYkZwNVZERmFWMVl5U2tsUmFscFhZbGhDU0ZkV1dtdFhSa3B5V2tkb1UyRjZWbmRXVnpBeFVXc3hWMWRZYUZoaVIxSmhWbXBDVjA1R1dsaE9WazVXVFd0d2VWUnNXbk5YYlVWNFkwZG9WMDFHY0hwV2JGcFBZekZPY2s1V1RtbFRSVXBJVm14amVHVnJOVmRYYTJSVVltdHdjVlV3WkRSWFZteFZVbTVrVlZKdGVGaFdNblF3WVRGYWNsZHFSbGhoTVhCeVdWWmFhMUl4VG5OaVJtUnBWa1ZKTUZac1kzaFdNVWw0WTBWc1YySkdjSEJXTUZaTFZsWmFkR05GWkZSTlYxSklWakowYTFkSFNsWlhia3BWVmxaS1NGVXdXbUZrUlRWWFZHMW9UbFpYT0hsWFYzUmhZVEZhZEZOc2JHaFNSVFZXVm14YWQyVnNXblJsUjNScVRWWktlbGRyV210aFZrNUdVMjFHVjAxV2NGaFdha1pXWlVaa2MyRkdVbWhOYkVwNFZsZHdTMkl4V1hoVmJGcGhVbXMxV1ZWdGVGZE5NVmw1WkVSQ2FFMVZiRE5VYkZaclZsZEtSMk5JU2xkU00yaG9WakJrVW1WdFRrZGFSMnhZVWpKb1ZsWnNhSGRSYlZaSFZHdGtWR0pIZUc5VmFrSmhWa1phY1ZOdE9WZGlSMUpaV2tWa01HRlZNWEppUkZKWFlsUldTRlpYTVV0V2JHUnpZVVp3YUUxWVFYcFdSbHBoWTIxUmVGcElVbXRTTW1oUFdWUk9RMU5XWkZoa1JrNVZUVlpzTTFSV2FFZFdNa1Y2WVVkR1dsWkZXak5XUlZwM1VteGtjMXBIZEZkTlNFSkpWMVpXYTJJeFVuTmFSVnBVWVd4YVdGbHNVa2ROTVZZMlVtdDBhMUpzY0RGV1IzaFBZVmRGZUdOR2NGaFdNMUp5VmxSS1NtVkdWbk5oUjNoVFYwVktVRmRYZEdGa01VNUhWMWhzVGxaWFVtOVZiRkpIVmpGc2NsVnNUbGROVlc4eVZtMTRRMVl3TVVoVmEzaFhUVlp3U0ZsNlJsZGpNWEJJWVVkb2JHSkdiRFZXYlRGM1VqRnNXRkpZYUdGU1YyaFlXVlJLYjFWV1duUmtTR1JWVFZad01GcEZhR3RXUmxwelkwaHdXRmRJUWtoV2ExVjRWMFpXY21KR1drNWliRXB2VjFaa05GUXhTbkpPVm1SaFVtNUNjRlZ0ZEhkVFZscDBaRWRHYTAxWFVrbFdiWFJ6VmxkS1NGVnVRbFpoYTFwTVZHMTRZV05zYTNwaFIyeE9WbXhaTUZacVNqQlpWbVJJVW01T2FsSnRhRmhaVkVaaFpWWndWbGR1VG1wV2EzQjZXVEJrTkZVeVNsZFRhbEpYWVd0dk1GWkVSbHBsUms1elYyMXdVMlZ0ZUZsV2JYQlBWVEZrUjFwR1pHRlNWR3h4VkZaYWQwMVdWblJsUms1b1VsUkNORlV5ZUdGV01rcFZVbFJDVm1WclJqUlZha3BMVWpGa2MyRkhhR3hpV0dneVZteGtkMUl4YkZoV2JrNVlWMGQ0YzFWc1pGTlhSbEpZWkVaa1QxSnRkRE5YYTJNMVYwWktjMk5FUWxkV00yaFFWMVphWVdNeVRraGhSbkJPWW0xbmVsWlhjRWRrTVVsNVVtdGtWV0Y2Vms5WmJHUnFaVVphZEUxVVVtaE5SRVpJVmpGb2IxWldaRWhoUjJoV1lrZFNWRlpxUm5OamJIQkhWR3hvVjJKWVozZFdSbHBoVkRKR2NrMVdaR3BTUlVwV1ZtMTRTMUpHV2toak0yaFhUV3R3UmxadGVGZGhWa2w0VW1wT1YyRnJTbWhWZWtwUFpFWk9jbFpzVm1sU01VcDNWbXBDVjFNeFdsZFhia1pVWW01Q2MxVnRkSE5PUmxwSVRsVTVWbUpWY0VsV1Z6QTFWMnhhUms1VlVsWk5WbkJ5Vm14YVQyTldWblJoUlRWb1pXeFdNMVp0TUhoTlIwVjRZa1prVkZkSGVHOVZibkJ6Vm14YWNsWnJkRlZTYkhCWldsVmtSMkZyTVZkWGJteFlZVEpTZGxaSGVHRldWa3BWVVd4a1YxSldjREpYVjNCTFVqSk5lRlJ1VG1oU2JIQndWbXBLYjFkR1pGaGxSemxWWWxaYVNGWXhhR3RYUjBwSlVXNUtWVlpzY0ROYVZscDNVbXhhVlZKdGFGZGhNMEY0VmxaYWIyRXhXWGhUYms1cVVteEtWMVpyVm1GaFJtdDVZek5vVjAxWFVubFViRnByVlRKRmVsRnNjRmRpVkVJeldsVmtTbVZXV25WVWJHaHBZWHBXV2xkWGVHOVZNVnBYVm01R1UySlZXbFZWYlhoelRsWndWbUZIZEZkTlJFWlhXVEJhYjFkdFJuSk9WRTVYVFVad1lWcFhlRWRqYlVaSFdrZG9hRTB3U2xKV2JURjNVakZaZVZWc1pGVmlhelZZV1d4a05GWkdVbGRXYm1SWFlrWnNORmRyVWxOaFZURnlZa1JTV21FeGNISlhWbHBMWXpGa2MxUnNjR2xTYkhCdlYxZHdSMVV4V1hoalJXUlZZa1pLVkZsclpETk5WbHBJWlVaYVQxWXdXa2xWTW5SaFlXeEtSbE50YUZkaVIxSlVWVEJhWVdNeFpIUlNiR1JPWVhwV1NWWXlkR0ZXTVZsNVUydHNVbUZzY0ZoWmJHaERVMFprVjFkdGRGaFNhM0I1V1ZWa2QxVXdNWFZoUm14WFlXdEtXRlY2Ums5U01XUjFWVzE0VTAweFNsRldWM0JEVmpBMVIxZHNhRTlXYXpWWFZGZDBkMlZXVW5OWGJrNVlZbFZXTkZrd1pHOVdNREZIWTBod1YySkdjRkJaZWtaUFkyczFWMVJ0YkZoU01taFpWbTE0YW1WSFZuSk9WbVJYVjBkU1QxWnRNVk5qVmxaeVZtMUdXRkpzV25oV1J6QTFZVEpLU0dWR2FGZE5ibEYzV1ZkemVHTnJOVlpoUm5Cb1RWaENNbFp0Y0VKa01sWkhWRzVHVkdKSFVsaFphMVozVTFaa1YxVnJaRmhpVmxwSVdUQldjMVpIU2xoaFJsSlZWa1ZhVEZac1dtRlNNVnAwVW0xMFRsWXhTbGxXTW5SaFlqSktSMU5ZWkZoaVZWcFlXV3hvVG1WR1dYZFhiVVpyVWxSV1dGWkhNVzlVYkZwWVQwaHNXRll6VW1oWlZFWmhaRVpPYzJKSGFGUlRSVXBYVjFkMFlXUXlWbk5YYmxKT1ZsZFNWRmxyV2t0bGJHUnlXa2hPVjAxWFVrZFZNblF3VmpBeFYyTkdhRmRoYTFwWFdsVmFhMk15UmtkVmJXeFRUVEpvTWxac1VrTldNbEY0V2tWa2FWTkZjRmxaYlRGVFZsWldkRTVWVGxOTlZtdzFXa1ZTUTJGRk1WWmlSRTVWWWtaYWNsWnNaRXRTTWs1SlUyeGtVMDB5YUc5V2FrSnJWVzFXZEZSclpHRlNNbmhaVldwS2IxWnNXbk5oU0dSU1lsWmFTRlpIZEd0V1IwcElaVWM1Vm1GclNtaFdhMXBoWTFaT2RFOVdaR2xTTVVwYVYydFdhMDFIUmxaTldFcHBVa1pLV0Zsc1VsZFRSbHBZVFZWMFYySkhVbnBaVlZwM1lVVXhXVkZZY0ZkU2JIQm9XVEl4VW1WR2NFbFZiWFJUVFcxb1VGZFdVazlSTVU1WFdrWm9hMUpyTlZaVVZscHpUVEZTVjJGRlpGZE5hMVkyV1ZWa1IxZHNXa1pYYWs1WFVsWndNMVZ0ZUd0ak1YQklaRVprVGxORlNrcFdNVnBYWVRKSmVGVnVUbUZTVm5CVldWZDRZVmRHYkhOVmEyUk9UVlpaTWxWdGREQmhNVXB6WTBSR1YxSXphRkJaVm1SR1pVZE9SMk5HYUZkTk1VcDVWbXhTUzFOdFZrZFdiazVvVWpOb1ZGWXdWa3RWUm1SWVpFZEdhMDFWTlVoV01XaHpWR3hhV0ZWc2JGWmhhMG96VmpCYVYyUkhUa1pQVm1SWFlraENObGRVUW05VE1WbDNUVlZvVm1FemFGaFVWV1JUVjBaVmVGZHNUbXBOYXpWSVYydGFUMVl5U2xWaGVrcFhZbFJHTTFWcVJuTlhSa3BaWVVkR1UxWXlhRmxYVmxKTFlqRldWMWR1VW10VFIxSldWRlphZDJWc1dsaGtSMFpWWWtWd2Vsa3dhSGRYUjBWNFUyeFNXbFpXVmpSV01HUlhVakpHUjFkck5XbGlWMmcxVmpGYWEwNUdVWGhUYmtwUFZtMVNhRlZ0TlVOalJsWjBaRWhrVkZac2NEQmFSV1JIVjBkS1YxZHNiRmRpV0ZKMldWUkdTbVZzUm5WUmJGcHBWa1phUlZadGNFZFpWbVJHVFZaV1ZtSlhhRTlXYlhSM1RteGFjVk5VUmxkTlZsb3dWVzAxVDJGc1NuUmxSbWhhVmtWYWFGWkZXbk5XYkdSMFVtMXdWMkV6UVhkWGExWmhWREZTYzFkWWFGUmlWVnBaVm10V1MyRkdaRmRhUldSWVVtdHdXbGt3V2t0aFZrbDZZVVpzV0ZZemFIWldWRVpyVWpGd1IxcEhSbE5pVmtwNFZrWmFhMVV3TVZkV2JsSnNVbFJzYjFadGRITk9SbFY1VGxjNVYwMVZjSHBaTUdoTFZqRmFSbU5HYUZwbGEzQk1WV3BHYTJSR1NuTlZiRTVYVW14d1lWWnRkR0ZaVm14WVZWaGtUbFp0VWxoWmEyUnZZekZXYzFwR1RsaFNiRXBaV2tWYWExUXhTblJrUkU1WVlUSk5NVll3V2t0ak1rNUZVV3hrVjAweWFGRldNVnBoV1ZkTmVWUnJhR2hTYmtKUFdXMHhibVZzV2xoalJXUnJUVlZzTlZWdGRHdFdWMFkyVm1zNVdtSkhVbkZhVlZwaFpFVXhWVlZ0YUU1U1JWcEpWbXBHYjJJeFdsZGFSV2hvVWpKb1YxbHJaRk5XUmxweFVtNU9hbUpJUWtoV1IzaHZWakpLV1ZvelpGZFNiSEJvVmtSS1IyTnJNVmRhUjNCVFVsVndWMVp0TVRSVE1VcEhWMnRvYWsweWFGaFVWM1IzVW14YWRHVkdaR2hXYTJ3MFZXMXdUMVl5Um5KVGJXaFdaV3R3VUZwRldrOWpiRnB6Vm0xc1UwMVZjRE5XYlhCRFZqRk5lRmRzWkZoaWExcFZXVzAxUTJNeFZuUmpla1pVVm14YU1GcEZaRWRoVmtweVkwUkNWMVl6VFRGV01qRkxWbFpLZFZkc2NHaGhNWEJ2VjFSR1lWSXlVa2RUYms1aFVsUldiMXBYZUZwTmJGbDRWV3RPVjAxcmJEUldiR2h6VmtkRmVXRkdaRnBoTWxKMlZtMTRZV1JIVmtoU2F6Vk9WbTVDV2xkc1ZtdFNNa1p5VFZac1VtRXpRbGxaVkVaM1ZrWmFTR1ZIUmxkV2EzQldWVmQ0YTFZeFNsZGpSRXBZVjBoQ1NGWnRNVmRXTVU1ellrZHNVMDF0YUZWWGJHTjRUa1prUjFwR2FHdFNNRnBWVkZaa1UxTkdXWGhoUjNSWVVqQndlVlJzYUV0V2JVWnlWMnQ0Vm1GcldqTlZiWGhoVmxaS2RHRkdaR2hOTUVZMFZteGFZV0l5UlhoWFdHaFVZbXMxY1ZWdGVFdFdNVnB5Vm0xR2FGSnRkRFZaZWs1dlZqQXhXRlZ1YkZWaVJuQnlWbFJLUm1Wc1JuTmpSbVJwVmtWR05GZHJVa2RYYlZaSFdraEtZVkp0YUhCWlZFWjNWbXhrVjFWclpHdE5hekUwVmpJMVUySkdTWGRYYXpsWFlsaG9hRlJWV25kV2JIQkdXa2RvVTJKSVFqVlhWM1JUVlRGYWRGTnJhR2hUUlhCWVdWZDBkMVJHV25OWGEzUlhWbXRhZWxrd1pEUmhSVEIzVTJ4R1YwMXVhRmhaVkVaU1pVWk9kVlZzVG1oTmJFcFVWMWQwYTFVeVRrZFhibEpyVTBkU1dGbHJXbk5PUm1SeVZXdE9WMDFFUm5wVmJYQlRWMGRGZVdGR1FsZE5SMUpIV2xaa1IxTldVbk5YYldob1pXeGFSbFp0ZUZkWlZteFlWV3RvVjJFeWVGWlpXSEJ6Vmtac2NscEVUazVXYkhCSldsVmFZVlF4V1hkWGEyeFdUVzVTYUZsWGVFdGtSMVpJVW14a2FWSnVRWHBYYkdRMFdWZE9WMVJ1U2xoaVdHaFVXV3RvUTFsV1pGZFdiWFJUVFZaV00xUlZhSE5oUmtwSFkwWm9XbUpIYUVSVk1GcHJWakZrZEdSR2FGTmhNMEkxVmpKMGEySXhWWGxTYWxwWFltMVNXRlp1Y0VOTk1WSnpWbFJHVTAxWFVscFpWVnBoWVVVeFJWWnRhRmRpV0VKRVZtcEJNVkl4WkhOaFJUbFhWa2Q0V2xaWE1IaFZNVTVYWTBaYVdtVnNXbGhaYkZaWFRrWlplV05GT1ZkTlJFWklXVEJhVTFZeVJuSlhia3BoVW14d00xVXdaRWRTTVZKMFlrWk9hVmRHUlhoV2JURTBZVzFXUmsxVmFGWlhSM2hWV1ZSS1UxWldiSFJqZWtaWFZtMTRXVnBGWkVkWFJrcDBaVWh3V2xaWFRURldiRnBMWkVkV1IyRkdaRTVpYkVZelYxWmtlbVZIVG5KT1ZscG9VbTVDYjFSV2FFTmxWbHB6Vld0MFZFMVZjREJXUjNSaFlWWktkRlZzYkZaTlJscE1WV3RhWVZJeFpIUmtSbFpwVm14d1NWZFVRbGRqTVZwSVUyeG9hRkpzU2xaV2ExWjNaR3hhYzFkdFJsTk5WbkF3VlcweE1GUnRSWGhqUld4WFlXdHJlRlpVU2tkak1VNXpZVVphYVZKc2NGZFdiVEUwV1ZkV2MxVnVTbGhpVlZwWlZXeFNWMVl4VVhoWGJUbFdUVVJHU2xWWGVHOVdNa1p5WW5wQ1dsWkZXbWhhUlZwUFl6SktTR0ZHVGxOV1JscFlWbTB4TkZsWFVYbFNhMlJZVjBkU1dGWnJWbUZYUm14eVYydDBiR0pIVW5sV2JGSkhZVVpLVlZGcVRsWk5ibEYzVm1wQmVGWXlUa2RoUm1Sb1lURndXRmRzVm1GaE1sSlhWRzVLVDFadFVuQldiWFIzVGtaYWMxcElaRlJOYTJ3MFdXdGFhMVp0U2toVmJHeGFZbFJHVkZacVJsZGtSVEZWVVdzNVUySkhkekZYYkZaclRVZEdTRkpxV2xOaVIzaFlWbTV3VjJWc1duTmFSWFJUVFdzMVNsVXllR3RWTURCNVlVYzVWMkZyU25KV1Z6RlhaRVpTY2xwSFJsTk5ibWhhVmxkd1MySXhXbk5YYms1b1UwZFNVMWxzV21GVFJscElaRWQwVjFJd2NFbGFWV00xVm0xR2NsTnRhRmRoTVhCb1dURmFUMlJGT1ZkYVIyeFRUVlZ3WVZZeWRGZGhNREZJVW14YVRsZEZjSEJWYTFwTFYwWnNjMVZzWkZSU2JIQlpWRlpXTUZZeVNraFZibkJhVmxad00xbFZWWGhYVmtaWlkwWmtVMkpJUW05WFZFbDRWVzFXUjJORmJGWmlXRkpVVkZjeGIxVkdaRmRWYXpsU1RWVTFXRmRyV205aE1VcHlUbFphVlZac2NIcFVWRVpUVmpKR1JscEdaRTVoTVZreVYxWldhMUl4WkhOWGExcFlZV3MxV0ZWc1duZE5NVlowWlVkMGFrMVlRa2xhUlZwclZHeE9SbE5yY0ZkTlZuQllXVlJLU21WR1ZuVlViVVpUVm01Q1ZsZFdaRFJaVm1SSFlraE9WbUV5VWs5VVZscGFUV3hXZEdSRVFtaE5WWEI2V1RCU1IxWXhTWHBoUmtKYVZtMVNSMXBWWkU5U01rNUhZVVprVGsxVmNEVldiWGhUVXpGT2RGWnJaRmRpYXpWdlZXcE9iMVpHV1hkV2EzUldVbTFTV1ZwRlpFZGhNREZXVGxWb1YwMVhhSHBXUjNoaFYwWldjMkpHV21oaE1HOTZWbXBDWVZZeVVraFVhMXBRVm0xU1dGbHJhRU5PYkdSVlVXMDVWRTFXYkROVVZsWnZWbTFGZW1GR2FGVldWMUpVVmpCYVZtVkdaSE5VYlhCT1lYcFdTVll5ZEdGVU1WbDVVMnhXYUdWcmNGaFpiRkpIVFRGV2NWSnVUbGhTYTNCYVdWVmtSMVV4V1hsaFJteFlWbXhLVEZscVJsZFdNV1IxVlcxMFZGSXhTbGRXUmxwaFpEQXhSMk5GV21oU00xSllWVzE0ZDJWc2EzZGhTR1JZVW0xU1NsVlhkRzlXTWtWNFZtcFNWMDFHY0ZoWk1qRkxVakZrY2s1WGJHbFNXRUl5Vm1wR1lXRXdOVWRWV0doVVltdHdVRlp0TVZOaFJsWjBUbFZPYWxKc1dqQmFSV2hyVmtaYWMyTkVRbUZTVjFKSVdWUkJlRmRHVm5KaVJtUnBWMFpLTmxadGVHdFRNVTVIVm01V1VtSkdjRzlaVkVaM1ZGWmFTRTFVVWxwV01EVjZXVlJPZDJGR1NYbGxSVGxYWWxSRmVsUldXbUZYUjFaSVQxZDBUbFpzY0RaV01uUlhZakpHUjFOdVVtaFNiV2hoV1d0YVMxTkdXa1ZTYm1SWFlrZFNNVlpITVhkV01rVjZVV3BhV0ZaRlNuSlVhMXBXWlZaT2MxcEdVbWxXVm5CYVZtMTBWMU15VG5OaVNFWlRZbFZhVkZSV1ZURmxiR3h5VjJ0MGFGWnJiRFpaVlZwelYwWlplbUZJV2xkV1JWcHlWV3BHZDFJeGNFaFNiRTVwVm10d05GWnJXbUZoTVZWM1RWWmtWMkpzU25OVmJGSnpZakZhZEdSSVRrOVNiRlkxVkd4ak5WWXdNVlpqU0hCYVRVZG9URlpxUm1GU2JVNUhZVVp3YkdFelFrMVdWM0JIWVRKU1IxTnVUbFppUlRWWVZXMTRkMWxXV25Sa1IwWmFWbTE0V1ZaV2FHdFViRnAwVld4b1dtRXlVblpaZWtaWFl6RndSMVJzYUZOaVJYQmFWMnRXWVdFeFVYaFRibEpyVWtVMVdWbFVTazVOVmxsM1YydDBhazFyTlVaVlYzaHJWakF3ZVdGR1JsZE5WbkJvVjFaa1RtVldVbkpoUjJoVFltdEtVRmRYTUhoaWJWWnpWMnhvYWxKWFVuSlVWbFp6VGxaV2RHUkhkRmRXTUZreVZtMXpOVmR0U2toVmJrWmhWbFp3YUZwRlZYaFdWbFowWVVVMVUxSldjRXBXYlhCSFlqSkplRmRzWkdGU1YyaHZWV3BLYjFZeGJISmFSazVYVW14d2VGVnROVTloYlVwSlVXdG9WMUl6YUhaV2FrcExWMVp2ZWxwR1pGTmlTRUp2Vm10U1MxSXlUbkpOVm1SaFVqTkNWRlZzVm5kV01WcDBUVlJTVkUxVmNFaFdNblJ2WVdzd2VXRkdhRlZXTTFJelZXMTRZV1JIVmtkYVIyaFhZVEozTUZaWE1YZFVNVmw1VW01S1ZHSlhhRmRhVjNSM1ZVWndSbGRyT1dwaVJuQjZWbTB4YzFVd01WWmpSRTVYWWxoQ1JGWnFSazVsUm1SeldrWm9hV0V4Y0ZaWFZ6QXhVVEpOZUZadVVtcGxhMXBWVm0xNFlVMUdjRVphUldSb1ZtdHdlVmt3VWtOV01WbDZWRlJHVjAxcVJreFdiRnBMWkZaa2MyTkhhR2hOV0VKMlZqRlNSMWxXU1hsVmEyUlVZbXMxYUZWc1VsZGlNWEJZWlVad1RtSkdiRFJXVjNSUFZqQXhjbUpFVWxkaVIyaDZWbXRrUm1WWFJrZGFSbkJvVFd4S01sWnRNVFJaVjFKSVZtdG9VMkY2Vms5V2JUVkRWMnhrYzFadGRGTk5hMXA1VkZaYWMxVnRSWHBSYkd4YVZrVmFNMVpyV21GVFIxWklVbXhrVjJFelFsZFdWbVEwWVRGWmVGTnNaR3BTUlhCWlZqQm9RMU5HV25STlZtUlhUVlUxZWxsclpITlZNREYwWVVaV1dHSkhUalJVYTFwclVqRndSMkZIZEZOTlJuQldWa1phYTFVeFRrZFhXR2hvVWpOU1dWVnFRbmRsVmxKellVVjBWMkpWV25sV01uUTBWbTFHY2xkcVRsWmlXR2hvVm14YVMyUkhSa2RhUjJoT1RVVlpNRlp0ZUdGWlYwbDVVbGhvV0ZkSGFGVlpiWE14WTFaV2RHVkZkRmROVm5CNVZtMHhSMkZHU25Sa1JGWmFaV3MxZGxacVFYaFhSbFoxWWtaV2FWSnVRbmxXYkZKTFVtMVdjMUp1VG1wU2JWSndWbXRXU21Wc1pITldiWFJVWWxaYVdGWXlOVmRXVjBwSVlVaENXbGRJUWxoV01uaHJWMGRXU0U5V1drNVdia0paVm0wd01WSXhWWGxUYkZwWVlrWmFWMWxzYUc5bGJGSnpWMjVrVjJKVldrbGFSVnByVkd4S1JsZHNhRmhXUlVwb1dXcEdhMlJHU25WVGF6bFhVbFJXV1ZaR1ZtRmtNa1pIWWtSYVUySllVbkpWYWtKaFUwWmtjbHBIT1doU1ZFSXpWVEo0UzFZeVNsVlNhazVXWVd0YVYxcFdXbGRqTVhCSFdrZHNhR1ZzV2xWV01XUXdZVEZWZVZSclpGaGliRXB5VlRCa1UxZEdiSEpYYms1UFVtMVNlVlpYZEU5V01ERkZVbTV3Vm1KWVFsUldNR1JHWlZkR1IxRnNaRTVTTVVwSVYyeGFhMUl4VGxkVmJrNVlZbGRvYjFwWGVHRlNiRnAwWlVaa1dsWnRlRmxXVjNSdlZqSkZlV1ZHV2xwWFNFSjZWbXRhYzJOc2NFVlVhelZYWWtoQ1NsZHNWbUZaVmxGNFYxaGtXR0V5ZUZkVVZ6VlRZVVpzV0dWRmRHdFNNVnBKVlcxNGEyRldTblZSV0hCWVlrZFJNRmRXWkU5V01WSjFVMnhvYVZkSGFGVlhWM1J2VVRGc1YxcEdaRmRpV0ZKVVZGWmFWMDB4VWxaaFNFNW9VbXR3U0Zrd1dtOVhiVXBJWVVWU1ZrMUdjSEpXYWtaclpFZFNSMkZHVG1oTk1Fa3hWakZhVjJFeVVYaFdXR1JPVjBaYWIxVnRNVFJYUm14WVpFVjBXRkpzV1RKVmJYTTFWVEF4V0ZWcVJsZFdla1V3VmxSQmQyVkdUbk5TYkdSWFRUQktSVlp0Y0VKTlZrbDRXa2hPYUZKVWJGaFdhMlEwVjJ4YVdFMUlhRlpOVlRWWVdUQmFZVmR0Vm5OWGJHaGFZa1phV0ZScldscGxWMDVHVDFaa1RsSkZXa2xYVkVKdlpERlpkMDFWYUZaaVJrcFhXV3hTUjFaR1ZYZGFSVGxVVWpCd1NGWkhNWE5WTURCNFVsaG9WMkpVUlRCWlZ6RlhVbXN4Vmxkck5WZFdNbWhWVjFjeE5GTXhXa2RpUm1oc1UwZFNXVlZ0TVRSbFZtUnlXWHBXV0ZKc2NEQmFWVnBoVm0xS1ZWWnVTbHBXVm5CTVdrVmtWMUl5UmtkYVJUVnBZa1ZaZWxZeFdtdGxiVlpJVkc1S1QxWnNjRzlWYlRWRFlqRlNWMkZGVGs1aVJuQXdXVEJXUzJFd01YTlhiR3hXWWxoU2RsWlZXbUZrUmxaellVZEdWMDB4U2t4V1JsWmhXVmROZUdFemNHaFNiVkpQVm14a00wMUdaRlZSYkdScVRWWnNOVlV5ZEd0aGJFNUdVMnhvVlZaV2NHaFdSVnBoWXpGa2RGSnRkRk5OUkZaSVZqSjBZVlV4VW5OWFdHaFVZVEo0V0ZsWGRIWmtNV3hWVW0xMFZGSnJOWHBXTW5odllWZEZlbEZzWkZkaVdHaG9WWHBHVDFJeFdsbGFSbWhwVmxad2VWWlhlRk5XTVdSSFYydG9UbFo2YkZaWmExcDNWMVpSZDJGSVRsaFNiR3cyVmxjeGIxZHNXa1pYYldoaFVsWndVRmw2U2t0VFIwWkhWR3hPVjFKc2NGbFdiVEI0VGtac1dGVlliRmRpYXpWWldXMTRTMk14Vm5GVWJFNXFWbXhLV0ZZeWREQmlSMHBJVlc1c1YwMXFSWGRaVlZwaFkyczFWbHBHV2s1aGExcEpWbTF3UW1WR1dYbFVhMlJvVW0xU2IxbFVSbmRYUm1SelZtMUdhRTFyTVRWVmJYUnJWbTFLV0dGRk9WcGhNVm96Vkd0YVlXTXhjRWxhUmxKT1lUSjNNRlpxUm05aU1WcDBVMnRrYWxJeWFGZFpiR2hPWlVaWmVGZHNaR3RTTVZwSVYydFZNVlJzV25Sa2VrWllZVEZhVkZWVVNrZFNhekZYVjIxc1UxSlVWbGRYVjNSaFdWZE9jMWR1VG1GU1dGSlVWRmR6TVZOc1ZYbGxSM1JXVFVSQ05WbFZZelZXTWtaeVUyNUtWMVo2UmxoYVJWcFRZMnh3U0ZKc1RrNWliV2hoVm14a2QxTXlTWGhYYms1WVltczFiMVV3WkZOVk1XeHpWMjFHVkZKc2NFbGFSV1F3VmpGWmQyTkZiR0ZXVjJoRVZtMHhTMWRXVWxsaFJuQm9ZVEZ3VlZacVFtdFdNVXAwVld0a1lWSXllRlJVVmxaM1dWWmFjbGR0Um1sTmJFWTBWbGMxVDFkSFNuSk9XRVpXWWxSR2RsZFdXbk5XVms1MFQxWldVMkpXU2xsV2Frb3dUVVpSZVZOc1dtcFNWMmhoVm14YWQwMHhiRlpXV0doWVZtdGFXbGt3V205aFZrbDRVbGhrVjJGcmJEUldha1poWTJzMVYxcEhhRk5OTVVwVlYxZDBZV1F5VW5OYVNFNWhVa1ZLWVZadE1WTlhSbVJ5VjJ4a1ZtSlZjRlpaYTJoSFZsWmFjMk5JY0ZWV1JWcFVWbXBLUjFKc2NFZFhhelZvVFZacmVGWXhXbGRoTVVsNFYxaHNVMkpyY0ZCV01GcDNXVlphYzFkdVpHeFdia0pZVjJ0Vk5WUXlTa1pqU0hCYVZsWndjbFpIZUU5U2JFNXlZMFprVGxadVFsbFhWbEpMVTIxV1IxWnVWbFZpVlZwVVZtMDFRMVpzWkZoa1JtUnJUVmRTU0ZaWGVHdFhSMHAwVlcwNVZtSlVSbFJXTUZwYVpWZE9ObEp0YkZOaVNFSTJWbFJLZDFJeFdYaFhXSEJvVW0xb1lWWnNXbGRPUmxweFUyczVVMDFyTlVoWlZWcHJWR3N4Vm1ORVZsZGlXR2hVVlhwR1VtVkdUbGxoUm1ocFlrWndWVmRYZEZka01XUkhWbTVPVjJKdFVsZFZiWGgzVjFaU1ZtRkhPVmRoZWtaWlZsYzFkMWRzV2taT1dFcFhZV3R3VEZZeFdsZGpNa1pIVm14a1RrMVZiRFpXYlhCRFdWWlplVlZyYUZWaE1YQlJWbTB4VTFkR2JISmhSVTVQWWtad2VGVXhVa2RoTURGWVZXNXdWMDF1YUhKWlYzaExWMWRHU0ZKc1ZsZGxiRnBRVjJ4V1lWZHRVWGhYYkZaWFlrZFNUMWxZY0ZkVGJHUlhWbXhhVGxZd1dsaFdNV2h6WVVaS1dGVnNhRnBpUmtwSVZGUkdWMk5XU25WVWJHaFRZVE5DWVZkV1ZtRmlNV1JIVTFoc2FGTkhhRmhXYWs1dllVWmtWMWRzV214U01IQktWa2N4YjFVeVJqWldibWhYVm5wRk1GcEVRWGhTTWs1R1YyeG9hV0pXU2xkWFYzUlhaREpXYzJFelpHaFNlbXhZV1Zod1IwMVdWbGhsUms1WFRXdGFlVlZ0Y0ZOV01rWnlWMjFvV2sxV2NHaFpla1pyWTJzMVdHSkhiRk5YUlVWNFZtMXdSMWxYUlhkT1ZXUldWMGRvVlZsdGRIZFZSbHAwVFZaT2FGSnNXakJVYkZaUFlXeEtjMWRxUW1GU1YyaHlWbXRhWVdNeVRrVlJiVVpUVmpGS1NWWnRNWHBsUmxsNVUydFdWbUpIYUc5VVZ6RnZWVlphY1ZGdGRGUk5WMUpaVlcxMGEyRkdTbk5YYkdoWFlsaFNNMVpyV210ak1XUjBVbXh3VjJKV1NraFdSbHBoVmpGYWRGTnNhR3hTVkd4WVdXeG9iMWxXVWxkWGJVWllVakZhU1ZReFpEQlViRnB6WWpOa1YxWkZiekJYVmxwclUwWk9jbUZIYUZOaVYyaG9WMWQwWVZNeFRrZFhhMVpUWW0xU1dGbHJaRk5OVmxwSVRWaGtWMDFyVmpaVlYzQmhWMFphUmxOdVNsVmhNWEJYV2xaYVQyTXhjRWRWYld4WFZtNUNZVlp0TVRCV01VMTVWRzVPYVZORldsUlpiRlpoVmtaU1YxWlVSbXhpUjNoNVZqSjBNR0ZHV25KaVJGSldUVzVvTTFacVNrdFdWa3BWVVd4YWJHRXhjRlZXVjNCSFlUSk5lRmR1VGxWaVYyaHpXV3hvYjFac1duSlhiVVphVmpGR00xUldXbXRXTWtwelUyNU9WbUpHU25wWmFrWmhaRWRXU1ZwR2FGTmlXRkY2Vm1wSk1XRXhXWGROV0VwcVVteHdWMVp0TVc5a2JGcDBaVWRHVTJKSFVucFZWM2hyWVVkRmVsRnJNVmRXTTBKSVdWUktUbVZXU25KaFIyaFRUVEpvZDFacVFtdGlNV3hYVjI1U2ExSXdXbUZXYlRFMFYxWmFWMkZIT1ZkV01GWTFWbGQ0YzFZd01VaFZWRUphWld0YVdGa3hXbE5qTVZKellVWk9hVkpZUW1GV01uUlhZakpGZUZkclpGaGlhelZ4Vld4a05GbFdVbFpYYm1SV1VteHdlRlZ0ZUhkaE1ERlhZMGhvVjJKWWFISldha0YzWlZkR1IxSnNaRTVXYmtKdlZqRmFhMVJ0VmxkVmJrcG9VakpvVkZsdGRFdFZSbVJZVFVob2FXSldXbnBXTW5odllXeEtXRlZ1U2xWV2JGcDZWR3hhVjJSSFVraGtSbVJYVFVacmVWWnRNWHBPVm1SMFUydG9WbUpyV21oV2JGcDNZVVp3UmxaWWFGZFdiRm93V1ZWVk1XRldTbGRoTTJSWFlrZFJNRmxxU2twbFZrcHpXa1pTYVdKRmNIWldWekUwWkRGc1YxZHVTbGRpYlZKWVZXMTRjMDB4V1hsT1ZUbG9UVlZzTkZZeWRHdFpWa3BZVldwT1YxSXphR2haZWtaclYxZE9SMXBGTldsaVJYQjJWbTEwVTFNeFNYbFVia3BPVm0xU2IxVnFUbTlXVmxaeFZHMDVXRlpzY0VoWGEyaHJWakF4VjFkcmFGZE5ibEpZVmxSS1MxWnNaSFZUYkdScFYwWktiMWRzWkRSaE1rNXpXa2hXWVZKdFVrOVdiVEV6VFVaYVdFMVlaRlZOVm13MVZXMDFTMVpIU2xoaFJtUmhWak5TTTFaVlduZFNiR1J5VDFkd2FWSnVRalpXYlRCNFl6RlZkMDFXV21wU1JUVllXV3hvUTFKR1duRlNhelZzVW0xU2VsZHJXbTloVjBWNFkwWmFXRlo2UVhoVlZFcE9aVVphZFZWdGRGUlNWRlpSVmxjeE5HUXhUbGRWYms1WFZrWmFXRmxzVm5kV01XdDNWbTVPVjAxV2JEWldSbEpIVmpBeGRXRkhSbUZXTTJoWVdrVmtSMUl4WkhKT1ZrNXBZVEJ2TVZacVJtOWtNVWw1VWxoc1ZtRXlVbFZaYlhNeFlVWldjMVZzWkdsTldFSlpXbFZhZDFSdFNrZGlSRkpYVmpOU1VGWnFRWGhqYlVwRlZXeGthVkl5YURaV2JYUnJVbTFXV0ZKclpHRlNia0paVlRCV1MwNXNXblJOVkZKYVZqRktXVlpIZEdGWlZrbDZWVzVDVm1KWWFHRlVWbHByWXpGYWRGSnRkRTVoTTBKSlZsUkpNVlF4WkVoVGJsSnNVakJhVmxac1pGTlZNVkpXVjIxR1YxWnJOWGxYYTFVMVlWWktXVkZzWkZkV2JWRXdXVlJHV21WR1RuTmFSM0JUVWxoQ1dsWnRjRTlSTVZKSFZtNUdWR0Y2Vm5OVmJYaExUVlphU0UxWE9WWk5SRVl3V1ZWYVlWWXhXWHBoUm1oaFVrVmFjbFZxUm5kU01rWklaVVpPYUdWc1dsWldhMXByVGtaTmVWWnVUbGhYUjNoUVZteG9VMWRXVm5GUmJtUlhUVlpzTlZSc1ZtdFdNVnB6WWtSYVYxWjZSblpXYlRGSFkyeGtkR0ZHV2s1U01VcFpWbGR3UjFVeVRYaFhibEpwVW1zMWNGbFVRbHBOUmxwMFpVZEdXbFl3YkRWVmJHaHZWMFprU0dGR1ZscGlXRTE0V1RGYVYyUkhWa1prUm1ST1ZtNUNOVlp0TUhoU01rWkhVMjVTYTFKR1dtRmFWM014Wkd4YWNWRllhRmhTYkZveFdXdGFWMkZXU25WUmJteFhZa1pLVEZWdE1WZGpNVXAxVkcxd1UySldTbEJXYlhCSFVqQXdlRnBHWkZaaE0xSlZXV3hhWVZkR1duUk9WbVJYVmpCd1NWbFZXbk5XYlVaeVYydDRWMDF1YUhKV01HUlhVMFU1VjFkck5WZGlhMHBhVm0wd2QwMVdTWGhXYmxKVVlrWndXVmxyV21GV01XeHpWV3RhVGxKdVFsaFdNbmgzWWtaYVZWWnNiR0ZTVjFKMlZsUktTMU5XUmxsYVJtUnBVakZHTTFkWWNFZGhNazE0V2toT2FGSnNXbGhWYkZwM1YxWmtWMXBFUW10TlZrcElWakkxVDFkSFJYcFZiR2hYWWxoU2FGcFhlSEprTVhCRlZXMW9VMVpGU1hwV1ZFbzBVekZaZDAxV2FHaFNSVFZYV2xkMFlWVXhjRlpYYXpsclZteGFlbFpITVc5VWJVcEdZMFpXVjAxWFVYZFpWRVpXWlVaV1dXRkdXbWxTYmtKWlYxY3dlRlV4WkVkaVNFcFlZbXMxVUZWdE1WTmxWbHBZWkVVNWFGSnNjRlpWYlhoclZqSktXVkZzUWxkaVIxSk1WV3BHYTJSV1RuTmFSVFZUVFZWd1RsWXhVa3BsUmxGNFZHdGtWR0pyTlZoWmEyUlRWa1paZDFacmRGZGlSbkF3V2xWYWExZHNXblJsUm14WFZqTlNkbFpxU2tabFYxWkhWMnh3V0ZORlNqWldiWFJoWXpKT2MyTkZWbFZoZWxaWVdXdG9RMU5zWkhOV2JVWmFWbXhHTkZZeGFHdFhSMFY1WlVaa1ZWWjZWbFJaVlZwelYwZFdSbVJIZEZOTlZuQktWbGN4TkdFeVJsZFRibFpTVjBoQ1dWbHJaRkpOUm1SWFdrVTVWMDFXU2pGVk1uaHZZVlprU0dGR1ZsaFdNMUoyVlZSR2MxWXhjRWRhUm1ScFYwWktWVlpHV21GV01EQjRWV3hhYUZORk5XOVZiWFIzVTBacmQxZHRPVmhpVlhCSldWVldVMVpyTVVkalNFcFhZV3RhY2xwRlpGTlNNa1pIVkcxb1RrMUZhM2hXYlhCTFRVZEZlVk5ZYUZkWFIzaFVXVmQ0ZDFkR2JIUk5WazVZVW14d01Wa3dWa3RpUmtwelYyeHNXbVZyTlhaV2FrRjRWMFpXYzJOR2NGZFdia0l5VjFaYVlWTnRWa2RVYmxKb1VtMVNjRll3Vmt0U1ZscDBZMFZLVGxacmJEUlhhMmhQWVVaS1ZXSkdhRnBYU0VKWVZqQmFZVkl4Y0VWUmJYUk9Za1p3V1Zac1l6RlVNV1JJVTI1S1QxWnRlR0ZaYTFwaFYwWlNjbGR1U210TldFSkpWREZhYTFSc1NrWlhhMnhZVmtWS2RscEVSbHBsVms1eVdrZHdVMkV3Y0doWGJHUXdXVlpXUjJFemJHcGxhMXBZVkZaYVMwMUdiSEpYYlhSb1VsUkNNMVV5TVc5V01rcFZVV3BTWVZKRldrOWFSRUY0Vm14a2NrNVdaRmROYldoV1ZqRmplRTFIVVhoVldHaFlZbXhLVDFadGN6RlhSbHAwVFZSU1dGWnVRa2RYYTFaclYwWktjbU5JY0ZkTmJrSklWakp6ZUZJeFpIVlRiSEJYVWxoQ1RWWlhjRXRVTVVsNFdraE9XR0Y2Vm5CWmExWmFaV3hhZEdWR1RsSmlWbHBKVmtjMVMxUXhXbk5qUnpsV1lsUldSRmxxUmxOV01rWkdWRzE0VjJKSVFqWldiR1F3VFVaYVJrMVdXbXBTUlVwWVZXcE9VMkZHV2tobFIwWnJVakJhU0ZsVldsZFdNVXBXWTBVeFYxWkZXbWhYVm1SUFl6RmFkVk50UmxOaVYyaDNWbTB4TkdReFRsZGFSbFpTWWtkU2NWUldXbmRUVmxWNVpVYzVhRkpyY0hwVk1uaGhWakpGZUZkcVRtRlNNMmhvV1hwR2EyUkdTbk5VYXpWWFltdEdNMVpzWTNkbFJURkhWVzVTVTFkSGFIRlZiR1EwVjBac1dHUklTbXhTYkhCV1ZXMTRUMkZyTVZkVGJteGFWbGRTZGxaVVNrdFdNVTV6V2taa2FWWkZWWGRXTVZwclZqSk5lRlp1VmxSaVIyaHdWbXhhZDFkc1dsaGxSazVYVFZac05GWldhRzlYUjBwVllrWmtWVlpXU2toVk1uaFhWMGRXU0U5V1pFNWhNWEExVjFjMWQxRXhaSE5YYTFwWVlsZG9hRlZzV25kVlJtdzJVMnQwVkZJd1draFdSM2hQVmpGS2MyTkdiRmRpVkVJMFZHdGFjMWRHVmxsaVJtUnBZWHBXV2xkWE1UUlRNazE0Vm01R1ZXSkZOWEZXYlhoTFYwWlplV1JFUW1sU01IQkpWbGMxYzFZd01YRlNXR1JhVmxad2FGa3ljekZXYkhCSFYyczFhV0pYWkRaV2JURjNVVzFXUjFkWWFGVmlhM0JYV1d0b1ExZEdWbk5YYm1SVlVtMVNlbFp0TVVkaFJURnlUVlJTV2sxR1duSlpWVnBLWlZkV1NFOVhSbGROTW1odlZrZDBZVmxYVFhoalJXeGhVbXhLVDFsVVNqTk5SbVJZWkVkR2FFMXJXbnBWTWpWVFlVWk9SbU5HWkZwV00xSk1XVlZhWVZKV1NuTlViR1JPVmpGSmQxWlhNSGhPUjBaWFUyNVNhMlZyV2xsWmExcGhZMnhTVmxwRlpGTmhla1pZVmpJeGMxVXhTbFZXYkdSWVZqTlNhRmRXWkU5ak1XUjFWRzFHVTFkR1NsVldSbVI2VFZaT2MxWlliR3hTTTFKWFZGVlNRMDVXYkZaWmVsWlhUV3RhZVZVeU5VdFdNVm8yVW14b1lWSldjRlJaTW5oM1UwWktjMVJyTldsaVYyaG9WbTE0YTJReFRYbFRXR2hZWW1zMVdWbHRjekZpTVZWM1drWk9WazFYZUhwV01qRkhWa1pLYzJORmJHRlNWMUYzVm1wR1NtUXlUa1ppUjBaWFZqQXdlRlp0TUhoU01rNXpWRzVTYUZKdFVtOVVWbWhEWWpGa1ZWSnRSbFZOYkVwSFZERmFhMWxXU1hsbFJsSlZWbXhhTTFkV1dscGxWMUpIV2tkb1RsSkZXa3BXVnpFMFdWWlNjMXBGV21wVFJVcFhXVlJHZDFSR1dYZFhiazVxVFZaYWVsZHJXbE5WTWtwSlVXeHdWMUpzY0ZoVVZWcGFaVlpPY21GR1dtbGlhMHBvVm0xNFlXUXdNSGhpUmxwWFYwZG9XVlp0ZEdGWGJHdDNWMjEwVmsxcmNFaFpNR00xVjBkRmVGTnJhRmhXYlZKVVZXcEdUMlJXVG5SaFJrNU9UVlZ3VmxZeFpEQlpWMUY1Vlc1T1lWTkZOV2hWYTFaTFdWWmFkRTFXVGxoU2JWSjVWako0YTFadFNsWmpSbHBhWVRKb1NGWnNXbUZqTWs1SlkwWldWMUpXY0ZWV2JYQkxVakZLYzJORlpGaGhlbFp2VkZaYWNtVkdXWGhYYkdSb1RWVnNORll4YUhOVk1rcHlZMFpDVjAxR2NETlhWbHBoWXpGa2RWcEhlRmRpYTBwWlZtMTRiMkl4V1hsU2FscHBVbTFvV0ZWcVRtOWpiR3hXVmxob1dGSlViRlpXUjNocllWWkpkMDVFUWxkU00xSm9WbGN4VjFJeGNFZGlSMFpUVmtaYVdWZFhkRzlSTVdSelYyNUdVbUpVYkZOVVZtUlRVMFpzY21GRk9WZGlWVlkxVmxkek5WWldXbk5qUjBaVlZqTm9XRnBGWkU5T2JFcDBZa1pPYVZORlNUSldiRkpMWkRGSmVGZFlhRlJYU0VKdlZXMHhiMWxXV25STlZrNVRUVmhDV1ZwVlZtdGhNVnAwVlc1d1dHRXhjSEpaYTJSR1pESkZlbHBHWkdsWFIyaFZWbGR3UjJNeFdYaFViR3hvVW0xU2NGbFljRmRYVm1SWVpFZEdWRTFyTlVoV01qVkxXVlpLUjFOdVNsVldiSEF6VlRCYVdtVlZNVlphUjJoT1VrVmFObFpxU2pSaU1WbDNUVmhXYUZOR2NHRldhMVpoVkVaV2RHVkZPV3BOV0VKSVZtMTRhMVl5UlhwUmEzQlhZbGhDUkZkV1pFNWxWbEoxVkd4a2FXRjZWbGRYVjNSclZURlJlRmR1VGxwbGJFcHpWbTE0ZDAxR1VsZFZiRTVvVmxSR1dGbHJVbGRXYXpGMVlVWm9WMVpGY0VkYVZ6RkhVakZ3UjJOSGJGTk5iV2Q1Vm0xNFYxbFhUWGRPV0U1VlltdHdhRlZ0Y3pGVU1WcHhVbXR3YTAxWFVsbGFSV1JIVmxVeGNsZHJaRlZXVjJoeVZtdGtTMVpzWkhOaFJuQnBVbXR3U1ZaR1VrZFdNazVYVTI1V1ZXSkhVazlaVkU1RFUxWlplV1JIUmxaTlYxSklWVEZvYzJGR1RraFZiR2hYWVdzMVZGbFZXbmRTVmtwMFVteE9UbUV6UWt0V2EyTjRVakZWZUZOdVNtcFNiVkpZVlcxNGQyRkdjRVZUYkdSVFRWWndNVlpYTVRCVk1VcFdZMFpzVjFaNlJUQmFSRUY0VWpGa2MyRkhlRlJTVkZab1ZtMHhNR1F4WkZkWGJHaHJVbnBzV0ZSWGRHRmxWbFowWTBaT1YwMXJXbmxXTWpWTFdWWktkRlZzUWxaaGEzQklWVEJrUjFKck5WZGFSMnhvVFVacmVGWnRNWGRTTVd4WFYxaG9hVkp0VWxaV01HUTBWbXhaZDFwSE9WZGlSM2d3V1ROd1IxZEdXblJsU0d4WVlURndVRmxYZUV0ak1rNUpZa1prYVZaRldrbFdiWEJIV1ZkTmVWUnJWbEppVlZwVVZtcEdTMU5XWkZkVmEwcHNVbTFTU0ZadE5VdGhWa3AwVlc1Q1YwMUdjRXhVYlhoaFpFZFdTR1JHVWs1V1ZGWkpWbTB3TVZZeFZuUlRhMlJxVW5wc1lWWnJWbUZOTVZwSFYyMUdWRkpVUmxaV1YzaDNWakF4UlZacVdsaFdiRXBRV2tSS1IxSXhUbGxpUjNoVFZrZDRXVmRYZUc5aU1XeFhWV3hhV0dKVlduRldiWE14WlVaV1dHVklaRmROYTNCWVZUSjBkMVl4V2paUldHaFhVa1Z3U0ZsNlJrOWtWbFowWVVaT2FWWXlhRzlXYlRFd1ZqRnNXRkpyWkdwU2JWSlpXVzB4VTFkR2JISmFSazVPVm14d1NWUldhR3RXTURGeVZtcGFWbFl6UW1oV01GcGhVbXhPZFZOc1ZtaE5XRUpZVjJ4YVlWUXlUWGhqUldSV1lsZDRXRlp0TlVOWlZscDBUVlJDVjAxVk1UVlZiWFJyV1ZaS2RGVnRhRlppUjFKVVZqQmFVMVl4V2xWU2JHaHBVak5vTlZZeWRHcE9WbHB5VFZoS2FsTkhlRmRVVnpWdlpXeFpkMWRzY0d4aVZWcElWbGQ0WVdGV1pFWk9SRUpYVmpOQ1RGWkVSa3RqTVdSMVZHeFdhVll6YUhaWGJHUTBXVlprYzFkdVNsaGlXRkp6V1Zod1IxZEdXa2hrUjNSWVVtMVNTVnBWYUU5V2JVcDFVVzV3V21GcldtaGFSVlY0VmpGU2MxVnNUbWxTYkd0NFZtdGtOR0l4VVhoWFdHUk9WMFZ3Y1ZWdGVIZFdNV3h5V2tjNWFtSkhVbGhXTW5Rd1ZUSktWazVWYUZwV1ZuQXpXV3RrUm1WSFRrbGlSbWhvVFZoQ2IxWnRjRXRVTWs1eVRsWmtZVkl5YUZSWldIQlhWMVphV0UxVVVtdE5WM2hZVmpJMVUyRXhTWGRYYlVaWFlsaG9lbFJzV21GU01rWklaRVpXVGxZeFNraFdSM2h2VVRGYWRGTnNiR2hTUlRWWVdWZDBZV1ZzYTNsbFNHUlRUVlpLZWxsVldrOWhWa3BaVVd4c1YySllhSEpVVldSR1pVWndSbHBHYUdsaGVsWjRWbGN3ZUdJeGJGZFhiazVoVW10d2NsUldXbk5OTVZsNVpFUkNhR0Y2Um5sVWJGWmhWbXhaZW1GSVNsZGhhMFl6V2xaa1IxSXhTbk5hUlRWWFltdEtkbFpxUm1GaE1WSjBWbXhrVm1KcmNHOVZha3B2Vmtac2RHUkZkRlpTYlZKWlZGVm9iMkZYU2xkWGJuQlhUVzVvZGxsWGVFdFhSbFoxVTJ4V1YySkdWalJXVkVaaFZtMVdXRlZyYUZCV01taFlXVlJPUTA1V1draGxSMFpxVFZad01GVXlkR3RYUjBwMFpVZG9WMkpHY0ROWlZWcDNVbXhhV1dGR1pGZGlSbTkzVjFaV1ZrMVdWWGhUYkdScVVsaENXRmxzYUVOVFJteHhVMnR3YkZKdFVucFhhMlJIVlRGWmVXRkljRmRXZWtVd1YxWmFhMUl4Vm5OWGJVWlRZWHBXZVZkWGRHdE5SVEI0VjFoc2JGSXpVbTlVVmxaM1ZteFdkR1ZIUmxkTlZtOHlWVzF3VDFZeVJuSmpSWGhXWVd0d1VGVnNXbXRqTVhCR1RsZHNhVkpZUWpKV2JURXdWVEZOZUZOWWJGVmlhM0JQVm0weFUyRkdWbkZVYTA1WFVteGFlbFp0Tld0aFZURlhZMFJDWVZaWFVraFdNR1JMVmpGS2NWVnNjR2hOYldoTlZqRmFZVk14WkZkV2JrNVdZa1phV0ZSVVFYaE9SbHBZVFZSU1YwMVhVa2xXYlhScldWWktkRlZyT1ZkaVZGWkVWR3hhYTFaV1JuUlNiWEJPVm0xM01WWlhNREZUTVZwSVUyeG9hRkp0VWxaV2FrNURVMFphVlZKdVpGZE5WMUo2VmtjeGQyRldTWGhUV0dSWVZteGFhRlpFU2tkWFJrcFpXa2R3VTJFeGNGbFhWM1JyVkRBd2VGVnNaRmhoTVhCeVZGVlNSMWRzV2tobFJ6bG9WakJXTkZrd1dtRlhSbG8yVVZob1ZsWkZjRTlhVlZwaFpGWldkR05IZUdoTldFSlhWbTB3ZUU1R2JGWk5WV1JxVWxad2FGVnNaRk5pTVZwMFpFaGtiRlp0ZEROV1YzUlBWakF4VjJKRVdsZFNiRXBJVm1wQmVHTldTbkpsUm1ScFVtNUNUVmRyV21GaE1rMTVWR3RvYUZJeWFFOVdNRlpMVFd4YWNWSnNUbE5OVjNoWVZURm9jMVp0UlhsVmJGWmFZbGhOZUZsVldsZGpNVlp6V2tkc1RsZEZTbHBYYkZaaFpERlNkRk5yWkdsU1JrcFlWbXBPVTFWR1duRlJXR2hxWWxWd1JsWlhlR3RoVmtsNVlVUmFWMkpZVW5KVmJURlhWMFpLY2xwSFJsTmlhMHBhVmxSQ1YxTXdOWE5YYmxKT1ZrWktZVlpxUmt0WFZscElaRWRHV2xacmJEWldWelZEVm0xS1dXRkhSbFZXYkhCeVdYcEdZV1JHU25Sa1JUVm9UVEJLU2xac1VrcE5WbEY0V2tWb1ZHSnNTbk5WYWtKaFZteGFjMXBHVGxaTlZrcFhWakl4UjFZeFduSk9XR3hZWVRGVk1WbFdXa3RYVmtaeldrWmtWMkpXUlhkWFZsSkhWMjFXUjFSdVRtRlNNMEpVVld4b1ExZHNXblJqUlRsU1RWWktTRlp0TlVkV1IwWTJWbTVPVm1KWWFFeFVWbHBoVTBVeFdFOVdXazVoTTBKTFZsWmpNV0V4V1hsVGJHeFNZVE5vWVZsWGRHRmpiRlp4VW01a1UwMXJXa2xhVlZwUFlrZEtXV0ZHYUZkTlYxRXdWMVphYzFZeFVuSmFSbWhZVWpKb2RsWlhjRTlWTVdSWFZtNUdVbUp0VWxWWmExcGhWMVpTYzFacVFsZE5WbkI2V1RCb2QxZHJNVWRUYkVKWVZteHdhRmw2U2s5U1ZsSnpWMjFzV0ZJeWFIWldiVEUwV1ZaTmVWVnJhRk5oTWxKb1ZXcEtiMVF4VWxaV2JIQk9VbXh3TUZwVlpFZFhiRmwzWWtSV1ZtSkhhSFpYVmxwTFYwWldjMkpHY0ZkU1ZtOTZWbTE0WVdNeVRuTmpSV2hRVm01Q1dGbHJhRU5PVmxwMFRVaG9VMDFyV2pCVk1uUnZZVEpXY21OR1pGcGlSa3BIV2tSR1dtUXhaSFJTYlhSWFlURndOVmRXVm1GaU1rWkdUVmhPVkdGcmNGbFdiWGgzWVVaV05sSnRkRmROVlhBeFZrY3hkMVV5U2xkVGJFSlhZbGhDU0ZWcVFYZGxSbkJIV2taYWFFMHhTbFpYVjNoaFdWWmtSMWR1VWs5V2JWSllWbXBDZDFOV2JGWlhiazVYWWxWYWVWWXlkRFJXTURGMVlVaEtWMDFIVWxoVk1GVTFWakZrY2s1V1pHbGhNSEJoVm0wd2QyUXdNVmRUV0doWFlteEtWVmxyV25kWFZteHlWMjFHVjAxWGVIcFpWV00xWWtkS1IxTnVjRmROYWxaeVZtcEtTMVpzWkhKaVIwWlRWakZLTWxac1VrdFRNVTVYVW14c1lWSnRVbkJWYWtaSFRURmtXV05GWkdoTmF6RTBWMnRvVDJGR1NuTlhiR2hXWWtad00xWXhXbXRXTVhCRlVXMTBUbFpyY0VsV2Fra3hWREZrU0ZKWWNGWmlSMmhZV1d4b1UyRkdjRmhsU0VwclRWaENSbFpYTVc5Vk1WcEdVMWh3VjFaRmJ6QlZla1phWkRBeFYyRkhhRk5TVkZaWVYxWmtNR1F4VlhoalJscFlZa1UxY1ZSV2FFTlRSbHAwVFZoT2FGWnJjRnBWVjNCWFZqSktTRlJxVW1GV00wNDBWbXBHVjFkWFJrZFhiV3hUWW10S1dsWXhZM2RsUmxWNFdrVmthVk5GY0ZsWmJYTXhWVVpXZEUxV1RteFdiSEJaV2tWVk5WZEdTWGRqUldoYVlURktWRlp0TVV0WFZrWnpVV3hrYVZJeFNrMVdWM0JIWVRKU1YxUnVUbUZTVkZaVVZGWldkMVJzV25OVmEwNW9UVlZzTkZVeU5VdFZSbHAwVlcxb1ZtRnJjSFpXUkVaaFpFVTFWbFJzVm1sU2JrRjNWMnRXWVZReFZYZE5WbVJwVWtaYVdGWnFUbE5rYkZweFUydGFiRll4V2toV01uaDNWMFpPUmxOcVdsZGlXRkpvV1hwS1QyTXlUa1pXYkZab1pXeGFXVmRYZEc5Uk1VNXpXa2hPVjJKVldtRldiWGhIVGtaWmVHRkhkRmhTTUZZMVdsVmFhMVpXV2paV2JGSmFWbFp3ZWxZd1pGZFNiVkpJWkVaT1RsWnVRa3RXYkdONFRrZEZlVkpzWkZSaWJGcFRWakJvUTFkV2JGVlRiRXBPVW0xU1dGZHJWVFZXTURGWFUycEdWazFxUmtoV1ZFRjNaREZPZFZkc2FHbFNNVVYzVmpGYVlXRXhXWGhhU0ZaVllsVmFWRlZ0ZEhka2JGcEhWMjFHVjAxRVZraFdNV2h6VkRGYWRGVnNaRnBpUmxwb1dsZDRjbVF4WkhSUFZtaHBWbGhDU2xkV1ZtOVpWbVJ6VjJ4a2FsSnRhR0ZVVmxwM1ZFWndSbHBHVGxOTmExcElWa2N4YjFSck1VWmhNMmhYWWxoQ1RGUlZXbk5XTVdSWllrWk9hVkl4U2xaV2FrSnJZakZhVjJKSVVteFRSMUpaVlcxNFlVMUdjRlpXYWtKV1RWVndlbFp0Y0ZkWGJWWnlUbGhhV2xZelRqUldNRnBYWTFaS2MxcEdaR3hoTVZsNlZtMTRVMUZ0VVhsV2EyUlZZbXMxVkZsclpGTlhSbkJZWkVWd1RsSnNjRWxhUldSSFlVVXhWbUpFVWxoaE1taFlWbXhrUzFkSFJrZFdiRnBwVW10d1dWWnFSbUZaVm1SSVZHdGFWV0pIVWs5VmExcGhVMVprVjFWck9XaE5WMUpKVlRJMVMxUXhXblJoUmxwWFlrWndURlV3V2xabFJtUjFWR3hrYVZKdVFqWldhMlEwWVRGU2MxTnJXbGhoTW5oWlZtdFdTMk5zYkhGU2JYUnJVbXh3TVZVeWVHOWhWMFY2VVcwNVYxWjZSVEJhUkVGM1pVWmFkVkpzVG1sU1ZGWjRWMVpvZDFZeFpFZGFTRXBoVW5wc1ZsbHJXbk5PYkZwWFlVaGtWMDFyY0ZaVlYzQlRXVlpLUmxkck9XRlNSVVkwVm0xNGQxSXhaSE5oUms1T1lsZG9hRlpxUm1wbFJUVkhWRmhvV0dKSFVuQlZiWGhoVmtac2RFMVdUbGhTYkhBd1ZGWlNVMkpHV25OaVJGWlZZa2RSZDFacVFYaGpiVTVHWVVad1RsSXlaM3BXYlhCQ1pVZFNWMVp1VmxSaVIxSlVWRlJCTUUxR1duUmpSV1JvVFdzMU1GVnRkR3RYUjBwWVpVWlNXbFpGTlVOVVYzaGhaRWRXU0ZKc2NGZGlSM2N3Vm1wS05GbFdVbk5YYkdSWVlsUnNhRlpxVGtOUk1WRjRVbFJzVVZWVU1Eaz0="
                  },
                  "timezone": {
                    "type": "string",
                    "description": "The timezone of the intervals in the CSV file. The default is `UTC`.",
                    "example": "America/Los_Angeles"
                  },
                  "allow_overwrite": {
                    "type": "boolean",
                    "description": "When set to `false`, the import fails if existing data conflicts with the uploaded data.",
                    "example": true
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "File uploaded successfully.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "import_id": {
                      "type": "string",
                      "description": "The import's unique reference ID.",
                      "example": "cmgoyz7mb00002f6obth9ojea"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-macro-scopes": [
            "workforce_management:write:admin"
          ],
          "x-granular-scopes": [
            "workforce_management:write:queue_metrics:admin"
          ]
        }
      }
    },
    "/workforce-management/imports/{importId}/historical-queue-metrics": {
      "get": {
        "tags": [
          "Imports"
        ],
        "summary": "Get historical queue metrics import metadata",
        "description": "Retrieve a list of historical queue metrics import metadata.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:queue_metrics:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\n\n**Not supported in Gov cluster** ",
        "operationId": "Viewhistoricalqueuemetricimoprtmeta",
        "parameters": [
          {
            "name": "importId",
            "in": "path",
            "description": "The import's unique reference ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "cmgoyz7mb00002f6obth9ojea"
            }
          },
          {
            "name": "division_id",
            "in": "query",
            "description": "The division ID.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "ya2kZo77RE6LFqaoKwWM0g"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Historical queue metrics import metadata response.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "create_time",
                    "file_name",
                    "import_id",
                    "imported_by",
                    "row_processed",
                    "status",
                    "timezone",
                    "total_rows"
                  ],
                  "type": "object",
                  "properties": {
                    "status": {
                      "type": "string",
                      "description": "The import's status.",
                      "example": "SUCCESS",
                      "enum": [
                        "PENDING",
                        "FAILED",
                        "SUCCESS",
                        "DELETING"
                      ]
                    },
                    "create_time": {
                      "type": "string",
                      "description": "The import creation time.",
                      "example": "2025-11-05T04:13:40.800Z"
                    },
                    "row_processed": {
                      "type": "number",
                      "description": "The number of rows processed from the import file.",
                      "example": 1000
                    },
                    "total_rows": {
                      "type": "number",
                      "description": "The number of rows present in the import file.",
                      "example": 1000
                    },
                    "import_id": {
                      "type": "string",
                      "description": "The import's unique reference ID.",
                      "example": "cmgoyz7mb00002f6obth9ojea"
                    },
                    "file_name": {
                      "type": "string",
                      "description": "The import file's name.",
                      "example": "HistoricalData.csv"
                    },
                    "timezone": {
                      "type": "string",
                      "description": "Themetrics data's timezone.",
                      "example": "America/New_York"
                    },
                    "imported_by": {
                      "type": "string",
                      "description": "The unique identifier of the user who imported the metrics.",
                      "example": "3U-_aQo_ROeigseGAQL5_A"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-macro-scopes": [
            "workforce_management:read:admin"
          ],
          "x-granular-scopes": [
            "workforce_management:read:queue_metrics:admin"
          ]
        }
      }
    },
    "/workforce-management/organizational-groups": {
      "get": {
        "tags": [
          "Organizational Groups"
        ],
        "summary": "Get multiple organizational groups",
        "description": "Get multiple organizational groups.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:organizational_groups:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\n\n**Not supported in Gov cluster** ",
        "operationId": "getMultipleOrganizationalGroups",
        "parameters": [
          {
            "name": "name",
            "in": "query",
            "description": "The organizational group's name.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "West Coast Operations"
            }
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of items to be returned per page.",
            "required": false,
            "schema": {
              "maximum": 50,
              "minimum": 1,
              "type": "integer",
              "example": 15,
              "default": 10
            }
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "The next page token from the previous page, if available. Not required for the first page.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "cmgoyz7mb00002f6obth9ojea"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Get multiple organizational groups response.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "organizational_groups",
                    "page_size",
                    "total_records"
                  ],
                  "type": "object",
                  "properties": {
                    "next_page_token": {
                      "type": "string",
                      "description": "Use the next page token to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size.",
                      "example": "cm2n7z2vl000e9cllp4ckoc45"
                    },
                    "page_size": {
                      "type": "integer",
                      "description": "The number of items returned per page.",
                      "example": 15
                    },
                    "total_records": {
                      "type": "string",
                      "description": "The total number of items available across all pages.",
                      "example": "45"
                    },
                    "organizational_groups": {
                      "maxItems": 50,
                      "type": "array",
                      "description": "List of organizational groups.",
                      "items": {
                        "required": [
                          "created_at",
                          "description",
                          "name",
                          "org_group_id",
                          "updated_at"
                        ],
                        "type": "object",
                        "properties": {
                          "org_group_id": {
                            "type": "string",
                            "description": "The organizational group's ID.",
                            "example": "cmgoyz7mb00002f6obth9ojea"
                          },
                          "name": {
                            "type": "string",
                            "description": "The organizational group's name.",
                            "example": "West Coast Operations"
                          },
                          "description": {
                            "type": "string",
                            "description": "The organizational group's description.",
                            "example": "Organizational group for west coast teams"
                          },
                          "created_at": {
                            "type": "string",
                            "description": "The organizational group's creation time.",
                            "example": "2024-01-15T10:30:00Z"
                          },
                          "updated_at": {
                            "type": "string",
                            "description": "The organizational group's update time.",
                            "example": "2024-01-15T10:30:00Z"
                          }
                        }
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
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-macro-scopes": [
            "workforce_management:read:admin"
          ],
          "x-granular-scopes": [
            "workforce_management:read:organizational_groups:admin"
          ]
        }
      },
      "post": {
        "tags": [
          "Organizational Groups"
        ],
        "summary": "Create an organizational group",
        "description": "Create an organizational group.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:write:organizational_groups:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\n\n**Not supported in Gov cluster** ",
        "operationId": "createOrganizationalGroup",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "name"
                ],
                "type": "object",
                "properties": {
                  "name": {
                    "type": "string",
                    "description": "The organizational group's name.",
                    "example": "West Coast Operations"
                  },
                  "description": {
                    "type": "string",
                    "description": "The organizational group's description.",
                    "example": "Organizational group for west coast teams"
                  },
                  "scheduling_group_ids": {
                    "maxItems": 100,
                    "type": "array",
                    "description": "The IDs of the scheduling groups to add to the organizational group.",
                    "example": [
                      "cmgoyz7mb00002f6obth9ojea"
                    ],
                    "items": {
                      "type": "string"
                    }
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "Create organizational group response.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "created_at",
                    "description",
                    "name",
                    "org_group_id",
                    "updated_at"
                  ],
                  "type": "object",
                  "properties": {
                    "org_group_id": {
                      "type": "string",
                      "description": "The organizational group's ID.",
                      "example": "cmgoyz7mb00002f6obth9ojea"
                    },
                    "name": {
                      "type": "string",
                      "description": "The organizational group's name.",
                      "example": "West Coast Operations"
                    },
                    "description": {
                      "type": "string",
                      "description": "The organizational group's description.",
                      "example": "Organizational group for west coast teams"
                    },
                    "created_at": {
                      "type": "string",
                      "description": "The organizational group's creation time.",
                      "example": "2024-01-15T10:30:00Z"
                    },
                    "updated_at": {
                      "type": "string",
                      "description": "The organizational group's update time.",
                      "example": "2024-01-15T10:30:00Z"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-macro-scopes": [
            "workforce_management:write:admin"
          ],
          "x-granular-scopes": [
            "workforce_management:write:organizational_groups:admin"
          ]
        }
      }
    },
    "/workforce-management/organizational-groups/{organizationalGroupId}": {
      "get": {
        "tags": [
          "Organizational Groups"
        ],
        "summary": "Get a single organizational group",
        "description": "Get a single organizational group.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:organizational_groups:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\n\n**Not supported in Gov cluster** ",
        "operationId": "getOrganizationalGroup",
        "parameters": [
          {
            "name": "organizationalGroupId",
            "in": "path",
            "description": "The organizational group's ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "cmgoyz7mb00002f6obth9ojea"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Get a single organizational group's response.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "agent_count",
                    "created_at",
                    "description",
                    "name",
                    "org_group_id",
                    "queue_count",
                    "scheduling_groups",
                    "updated_at"
                  ],
                  "type": "object",
                  "properties": {
                    "org_group_id": {
                      "type": "string",
                      "description": "The organizational group's ID.",
                      "example": "cmgoyz7mb00002f6obth9ojea"
                    },
                    "name": {
                      "type": "string",
                      "description": "The organizational group's name.",
                      "example": "West Coast Operations"
                    },
                    "description": {
                      "type": "string",
                      "description": "The organizational group's description.",
                      "example": "Organizational group for west coast teams"
                    },
                    "created_at": {
                      "type": "string",
                      "description": "The organizational group's creation time.",
                      "example": "2024-01-15T10:30:00Z"
                    },
                    "updated_at": {
                      "type": "string",
                      "description": "The organizational group's update time.",
                      "example": "2024-01-15T10:30:00Z"
                    },
                    "agent_count": {
                      "type": "integer",
                      "description": "The number of agents in the organizational group.",
                      "example": 42
                    },
                    "queue_count": {
                      "type": "integer",
                      "description": "The number of queues in the organizational group.",
                      "example": 8
                    },
                    "scheduling_groups": {
                      "maxItems": 100,
                      "type": "array",
                      "description": "Associated scheduling groups",
                      "items": {
                        "required": [
                          "name",
                          "scheduling_group_id"
                        ],
                        "type": "object",
                        "properties": {
                          "scheduling_group_id": {
                            "type": "string",
                            "description": "The scheduling group's ID.",
                            "example": "cmgoyz7mb00002f6obth9ojea"
                          },
                          "name": {
                            "type": "string",
                            "description": "The scheduling group's name.",
                            "example": "Morning Shift"
                          }
                        }
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
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-macro-scopes": [
            "workforce_management:read:admin"
          ],
          "x-granular-scopes": [
            "workforce_management:read:organizational_groups:admin"
          ]
        }
      },
      "delete": {
        "tags": [
          "Organizational Groups"
        ],
        "summary": "Delete organizational group",
        "description": "Delete an organizational group.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:delete:organizational_groups:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\n\n**Not supported in Gov cluster** ",
        "operationId": "deleteOrganizationalGroup",
        "parameters": [
          {
            "name": "organizationalGroupId",
            "in": "path",
            "description": "The organizational group's ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "cmgoyz7mb00002f6obth9ojea"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "Delete organizational group response."
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-macro-scopes": [
            "workforce_management:write:admin"
          ],
          "x-granular-scopes": [
            "workforce_management:delete:organizational_groups:admin"
          ]
        }
      },
      "patch": {
        "tags": [
          "Organizational Groups"
        ],
        "summary": "Update an organizational group",
        "description": "Update an organizational group.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:update:organizational_groups:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\n\n**Not supported in Gov cluster** ",
        "operationId": "updateOrganizationalGroup",
        "parameters": [
          {
            "name": "organizationalGroupId",
            "in": "path",
            "required": true,
            "schema": {
              "type": "string"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "name": {
                    "type": "string",
                    "description": "The organizational group's name.",
                    "example": "West Coast Operations"
                  },
                  "description": {
                    "type": "string",
                    "description": "The organizational group's description.",
                    "example": "Organizational group for west coast teams"
                  },
                  "scheduling_group_ids": {
                    "maxItems": 100,
                    "type": "array",
                    "description": "The IDs of the scheduling groups to add to the organizational group.",
                    "items": {
                      "type": "string",
                      "example": "cmgoyz7mb00002f6obth9ojea"
                    }
                  }
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "description": "Update organizational group response.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "created_at",
                    "description",
                    "name",
                    "org_group_id",
                    "updated_at"
                  ],
                  "type": "object",
                  "properties": {
                    "org_group_id": {
                      "type": "string",
                      "description": "The organizational group's ID.",
                      "example": "cmgoyz7mb00002f6obth9ojea"
                    },
                    "name": {
                      "type": "string",
                      "description": "The organizational group's name.",
                      "example": "West Coast Operations"
                    },
                    "description": {
                      "type": "string",
                      "description": "The organizational group's description.",
                      "example": "Organizational group for west coast teams"
                    },
                    "created_at": {
                      "type": "string",
                      "description": "The organizational group creation time.",
                      "example": "2024-01-15T10:30:00Z"
                    },
                    "updated_at": {
                      "type": "string",
                      "description": "The organizational group update time.",
                      "example": "2024-01-15T10:30:00Z"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-macro-scopes": [
            "workforce_management:write:admin"
          ],
          "x-granular-scopes": [
            "workforce_management:update:organizational_groups:admin"
          ]
        }
      }
    },
    "/workforce-management/reports/adherence/agents": {
      "get": {
        "tags": [
          "Reports"
        ],
        "summary": "List agents' adherence data",
        "description": "Get agents' adherence data.\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:list_adherence_agents:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "listAdherenceAgents",
        "parameters": [
          {
            "name": "date",
            "in": "query",
            "description": "The date in 'yyyy-MM-dd' format.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "2025-09-01"
            }
          },
          {
            "name": "timezone",
            "in": "query",
            "description": "The adherence timezone. The default is UTC.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "America/Los_Angeles",
              "default": "UTC"
            }
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "Use the next page token to paginate through large result sets. A next page token returns whenever the set of available results exceeds the current page size.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "jA5csQv1W1oXuYZLspNIZzMOXqTD9r9Rje2",
              "default": "null"
            }
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of agents returned per page.",
            "required": false,
            "schema": {
              "maximum": 10,
              "minimum": 1,
              "type": "integer",
              "example": 10,
              "default": 10
            }
          },
          {
            "name": "division_id",
            "in": "query",
            "description": "The division ID.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "ya2kZo77RE6LFqaoKwWM0g"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "List of agents with adherence for the date.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "agents",
                    "next_page_token",
                    "page_size",
                    "timezone",
                    "total_records"
                  ],
                  "type": "object",
                  "properties": {
                    "next_page_token": {
                      "type": "string",
                      "description": "Use the next page token to paginate through large result sets. A next page token returns whenever the set of available results exceeds the current page size.",
                      "example": "jA5csQv1W1oXuYZLspNIZzMOXqTD9r9Rje2"
                    },
                    "page_size": {
                      "type": "integer",
                      "description": "The number of agents returned per page.",
                      "example": 10
                    },
                    "total_records": {
                      "type": "integer",
                      "description": "The total number of agents available across pages.",
                      "example": 100
                    },
                    "timezone": {
                      "type": "string",
                      "description": "The adherence timezone. The default is UTC.",
                      "example": "America/Los_Angeles"
                    },
                    "agents": {
                      "maxItems": 10,
                      "type": "array",
                      "description": "List of agents with adherence.",
                      "items": {
                        "required": [
                          "adherence_time_for_activity",
                          "display_name",
                          "schedule_activities",
                          "shifts",
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The agent's unique identifier.",
                            "example": "0c30da44bd68aae3b5714abb885a59"
                          },
                          "display_name": {
                            "type": "string",
                            "description": "The agent's name.",
                            "example": "Alex"
                          },
                          "shifts": {
                            "maxItems": 10,
                            "type": "array",
                            "description": "Shift details.",
                            "items": {
                              "required": [
                                "end_in_adherence",
                                "end_time",
                                "start_in_adherence",
                                "start_time"
                              ],
                              "type": "object",
                              "properties": {
                                "start_time": {
                                  "type": "string",
                                  "description": "The shift start time, in ISO 8601 format `yyyy-MM-dd'T'HH:mm:ss'Z'` or yyyy-MM-dd'T'HH:mm:ss'TZD'`.",
                                  "example": "2024-11-06T03:30:00.000Z"
                                },
                                "end_time": {
                                  "type": "string",
                                  "description": "The shift end time, in ISO 8601 format `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`.",
                                  "example": "2024-11-06T03:30:00.000Z"
                                },
                                "start_in_adherence": {
                                  "type": "integer",
                                  "description": "The number of seconds after shift start that the user started being in adherence.",
                                  "example": 0
                                },
                                "end_in_adherence": {
                                  "type": "integer",
                                  "description": "The number of seconds before shift end that the user started being in adherence.",
                                  "example": 0
                                }
                              }
                            }
                          },
                          "schedule_activities": {
                            "maxItems": 250,
                            "type": "array",
                            "description": "All activities in the schedule.",
                            "items": {
                              "required": [
                                "activity_id",
                                "activity_name",
                                "end_time",
                                "start_time"
                              ],
                              "type": "object",
                              "properties": {
                                "activity_id": {
                                  "type": "string",
                                  "description": "The activity's unique identifier.",
                                  "example": "0c30da44bd68aae3b5714abb885a59"
                                },
                                "activity_name": {
                                  "type": "string",
                                  "description": "The activity's name.",
                                  "example": "Chat"
                                },
                                "start_time": {
                                  "type": "integer",
                                  "description": "The activity's start time in timestamp.",
                                  "example": 1730896200000
                                },
                                "end_time": {
                                  "type": "integer",
                                  "description": "The activity's end time in timestamp.",
                                  "example": 1730896200000
                                }
                              },
                              "description": "Activity details"
                            }
                          },
                          "adherence_time_for_activity": {
                            "maxItems": 250,
                            "type": "array",
                            "description": "All unique activities in the schedule with adherence.",
                            "items": {
                              "required": [
                                "activity_id",
                                "activity_in_adherence",
                                "activity_name",
                                "activity_out_adherence"
                              ],
                              "type": "object",
                              "properties": {
                                "activity_id": {
                                  "type": "string",
                                  "description": "The activity's unique identifier.",
                                  "example": "0c30da44bd68aae3b5714abb885a59"
                                },
                                "activity_name": {
                                  "type": "string",
                                  "description": "The activity's name.",
                                  "example": "Chat"
                                },
                                "activity_in_adherence": {
                                  "type": "integer",
                                  "description": "Activity in adherence, in seconds.",
                                  "example": 0
                                },
                                "activity_out_adherence": {
                                  "type": "integer",
                                  "description": "Activity out of adherence, in seconds.",
                                  "example": 0
                                }
                              },
                              "description": "The activity's adherence details."
                            }
                          }
                        },
                        "description": "Agent and adherence details."
                      }
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `1902` <br>\n Invalid date parameter. <br>\n**Error Code:** `1903` <br>\n Invalid timezone. <br>\n**Error Code:** `1904` <br>\n Invalid page size. <br>\n"
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](/docs/api/rest/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-granular-scopes": [
            "workforce_management:read:list_adherence_agents:admin"
          ]
        }
      }
    },
    "/workforce-management/reports/schedules/agents": {
      "get": {
        "tags": [
          "Reports"
        ],
        "summary": "List all schedule agents",
        "description": "List all the schedule agents for the particular date and timezone\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:list_schedule_agents:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "listScheduleAgents",
        "parameters": [
          {
            "name": "date",
            "in": "query",
            "description": "The date in 'yyyy-MM-dd' format.",
            "required": true,
            "schema": {
              "type": "string",
              "format": "date",
              "example": "2025-09-01"
            }
          },
          {
            "name": "user_ids",
            "in": "query",
            "description": "Return results only for the specified user IDs. Add multiple `user_ids` parameters to include more users. Example: `?user_ids=123&user_ids=456`.",
            "required": false,
            "schema": {
              "type": "array",
              "format": "string",
              "example": [
                "jA5csQv1W1oXuYZLspNIZzMOXqTD9r9Rje2"
              ]
            }
          },
          {
            "name": "timezone",
            "in": "query",
            "description": "The schedule's timezone. The default is UTC.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "America/Los_Angeles",
              "default": "null"
            }
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "Use the next page token to paginate through large result sets. A next page token returns whenever the set of available results exceeds the current page size. ",
            "required": false,
            "schema": {
              "type": "string",
              "example": "jA5csQv1W1oXuYZLspNIZzMOXqTD9r9Rje2",
              "default": "null"
            }
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of scheduled agents returned per page.",
            "required": false,
            "schema": {
              "maximum": 10,
              "minimum": 1,
              "type": "integer",
              "example": 10,
              "default": 10
            }
          },
          {
            "name": "division_id",
            "in": "query",
            "description": "The division ID.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "ya2kZo77RE6LFqaoKwWM0g"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "List of agents in the schedule for the date",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "agents",
                    "next_page_token",
                    "page_size",
                    "timezone",
                    "total_records"
                  ],
                  "type": "object",
                  "properties": {
                    "next_page_token": {
                      "type": "string",
                      "description": "Use the next page token to paginate through large result sets. A next page token returns whenever the set of available results exceeds the current page size. ",
                      "example": "jA5csQv1W1oXuYZLspNIZzMOXqTD9r9Rje2",
                      "default": "null"
                    },
                    "page_size": {
                      "maxLength": 10,
                      "minLength": 1,
                      "type": "integer",
                      "description": "The number of scheduled agents returned per page.",
                      "example": 10,
                      "default": 10
                    },
                    "total_records": {
                      "type": "integer",
                      "description": " The total number of all scheduled agents available across pages.",
                      "example": 100
                    },
                    "timezone": {
                      "type": "string",
                      "description": "The schedule's timezone. The default is UTC.",
                      "example": "America/Los_Angeles"
                    },
                    "agents": {
                      "maxItems": 250,
                      "type": "array",
                      "description": "List of agents in the schedule",
                      "items": {
                        "required": [
                          "activities",
                          "display_name",
                          "scheduling_group_name",
                          "total_paid_time",
                          "total_scheduled_time",
                          "user_email",
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The agent unique identifier",
                            "example": "0c30da44bd68aae3b5714abb885a59"
                          },
                          "display_name": {
                            "type": "string",
                            "description": "The agent's display name",
                            "example": "Alex"
                          },
                          "user_email": {
                            "type": "string",
                            "description": "The agent's email address",
                            "example": "alex@example.com"
                          },
                          "scheduling_group_name": {
                            "type": "string",
                            "description": "The name of the scheduling group",
                            "example": "Sales",
                            "default": "null"
                          },
                          "total_scheduled_time": {
                            "type": "integer",
                            "description": "The total schedule duration, in minutes.",
                            "example": 480,
                            "default": 0
                          },
                          "total_paid_time": {
                            "type": "integer",
                            "description": "The total duration of paid schedule activity in minutes",
                            "example": 340,
                            "default": 0
                          },
                          "activities": {
                            "maxItems": 250,
                            "type": "array",
                            "description": "List of activities",
                            "items": {
                              "required": [
                                "activity_name",
                                "activity_type",
                                "end_time",
                                "is_paid",
                                "schedule_activity_id",
                                "start_time"
                              ],
                              "type": "object",
                              "properties": {
                                "schedule_activity_id": {
                                  "type": "string",
                                  "description": "The activity unique identifier",
                                  "example": "cll20wknm01cwex0isvewiegl"
                                },
                                "start_time": {
                                  "type": "string",
                                  "description": "The schedule activity start time. In ISO 8601 format (yyyy-MM-dd'T'HH:mm:ss'Z' or yyyy-MM-dd'T'HH:mm:ss\u2019TZD\u2019).",
                                  "example": "2024-07-20T14:00:00-08:00"
                                },
                                "end_time": {
                                  "type": "string",
                                  "description": "The schedule activity end time. in ISO 8601 format (yyyy-MM-dd'T'HH:mm:ss'Z' or yyyy-MM-dd'T'HH:mm:ss\u2019TZD\u2019).",
                                  "example": "2024-07-20T14:00:00-08:00"
                                },
                                "activity_name": {
                                  "type": "string",
                                  "description": "The activity name identifier",
                                  "example": "SMS"
                                },
                                "activity_type": {
                                  "type": "string",
                                  "description": "Type of the activity",
                                  "example": "Productive"
                                },
                                "is_paid": {
                                  "type": "boolean",
                                  "description": "The is_paid indicates if the scheduled activity counts as paid time for calculating the agent's total paid duration.",
                                  "example": true
                                }
                              },
                              "description": "The activity details"
                            }
                          }
                        },
                        "description": "The schedule agent details"
                      }
                    }
                  },
                  "description": "Schedule agents list returned"
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `1902` <br>\n Invalid date parameter <br>\n**Error Code:** `1903` <br>\n Invalid timezone <br>\n**Error Code:** `1904` <br>\n Invalid page size <br>\n"
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  Internal server error \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-granular-scopes": [
            "workforce_management:read:list_schedule_agents:admin"
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

### endpoints.json
```json
{
  "openapi": "3.0.0",
  "info": {
    "title": "Workforce Management",
    "description": "Workforce Management",
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
    "/workforce-management/forecasts": {
      "get": {
        "tags": [
          "Forecasts"
        ],
        "summary": "List forecasts",
        "description": "List forecasts.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:forecasts:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\n\n**Not supported in Gov cluster** ",
        "operationId": "Listforecasts",
        "parameters": [
          {
            "name": "name",
            "in": "query",
            "description": "The forecast name.",
            "required": false,
            "schema": {
              "maxLength": 36,
              "type": "string",
              "example": "Weekday forecast"
            },
            "example": "Weekday forecast"
          },
          {
            "name": "scheduling_group_name",
            "in": "query",
            "description": "The scheduling group name.",
            "required": false,
            "schema": {
              "maxLength": 36,
              "type": "string",
              "example": "Infra support"
            },
            "example": "Infra support"
          },
          {
            "name": "status",
            "in": "query",
            "description": "The forecast status.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "ready_step_1",
              "enum": [
                "ready_step_1",
                "processing",
                "ready_step_2",
                "completed",
                "failed",
                "applied",
                "generating",
                "generating_failed",
                "reforecasting",
                "reforecasting_failed"
              ]
            },
            "example": "ready_step_1"
          },
          {
            "name": "type",
            "in": "query",
            "description": "The forecast type.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "SHORT_TERM",
              "enum": [
                "SHORT_TERM",
                "LONG_TERM"
              ]
            },
            "example": "SHORT_TERM"
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "The next page token from the previous page, if available. Not required for the first page.",
            "required": false,
            "schema": {
              "maxLength": 36,
              "type": "string",
              "example": "cm2n7z2vl000e9cllp4ckoc45"
            },
            "example": "cm2n7z2vl000e9cllp4ckoc45"
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of items to be returned per page.",
            "required": false,
            "schema": {
              "maximum": 50,
              "minimum": 1,
              "type": "number",
              "example": 50,
              "default": 10
            },
            "example": 50
          },
          {
            "name": "division_id",
            "in": "query",
            "description": "The division ID.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "ya2kZo77RE6LFqaoKwWM0g"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Forecast list response.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "forecasts",
                    "next_page_token",
                    "page_size",
                    "total_records"
                  ],
                  "type": "object",
                  "properties": {
                    "next_page_token": {
                      "description": "Use the next page token to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size.",
                      "anyOf": [
                        {
                          "type": "string",
                          "example": "cm2n7z2vl000e9cllp4ckoc45"
                        },
                        {
                          "type": "null"
                        }
                      ]
                    },
                    "page_size": {
                      "type": "integer",
                      "description": "The number of items returned per page.",
                      "example": 10
                    },
                    "total_records": {
                      "type": "integer",
                      "description": "The total number of items available across all pages.",
                      "example": 100
                    },
                    "forecasts": {
                      "type": "array",
                      "description": "List of forecasts.",
                      "items": {
                        "required": [
                          "create_time",
                          "description",
                          "end",
                          "forecast_id",
                          "forecast_profile_id",
                          "interval_minutes",
                          "modify_time",
                          "name",
                          "scheduling_groups",
                          "start",
                          "status",
                          "type"
                        ],
                        "type": "object",
                        "properties": {
                          "forecast_id": {
                            "maxLength": 30,
                            "type": "string",
                            "description": "The forecast ID.",
                            "example": "cm2n7z2vl000e9cllp4ckoc45"
                          },
                          "name": {
                            "maxLength": 36,
                            "type": "string",
                            "description": "The forecast name.",
                            "example": "Weekday forecast"
                          },
                          "description": {
                            "type": "string",
                            "description": "The forecast description.",
                            "example": "Forecast for business days"
                          },
                          "type": {
                            "type": "string",
                            "description": "The forecast type.",
                            "example": "SHORT_TERM",
                            "enum": [
                              "SHORT_TERM",
                              "LONG_TERM"
                            ]
                          },
                          "status": {
                            "type": "string",
                            "description": "The forecast status.",
                            "example": "ready_step_1",
                            "enum": [
                              "ready_step_1",
                              "processing",
                              "ready_step_2",
                              "completed",
                              "failed",
                              "applied",
                              "generating",
                              "generating_failed",
                              "reforecasting",
                              "reforecasting_failed"
                            ]
                          },
                          "start": {
                            "type": "string",
                            "description": "The forecast start.",
                            "format": "date-time",
                            "example": "2025-01-01T00:00:00Z"
                          },
                          "end": {
                            "type": "string",
                            "description": "The forecast end.",
                            "format": "date-time",
                            "example": "2025-01-06T23:59:59Z"
                          },
                          "create_time": {
                            "type": "string",
                            "description": "The forecast request create time.",
                            "format": "date-time",
                            "example": "2024-12-16T04:21:23.207Z"
                          },
                          "modify_time": {
                            "type": "string",
                            "description": "The forecast request modify time.",
                            "format": "date-time",
                            "example": "2024-12-16T04:21:23.207Z"
                          },
                          "interval_minutes": {
                            "maximum": 60,
                            "minimum": 15,
                            "type": "number",
                            "description": "The forecast granularity length, in minutes.",
                            "example": 15
                          },
                          "forecast_profile_id": {
                            "description": "The forecast profile ID.",
                            "anyOf": [
                              {
                                "type": "string",
                                "example": "cm2n7z2vl000e9cllp4ckoc45"
                              },
                              {
                                "type": "null"
                              }
                            ]
                          },
                          "scheduling_groups": {
                            "type": "array",
                            "description": "List of scheduling groups present in the forecast.",
                            "items": {
                              "required": [
                                "asa",
                                "color_code",
                                "deferrable_sla",
                                "deferrable_sla_target_time",
                                "dynamic_shrinkage",
                                "fixed_shrinkage",
                                "name",
                                "occupancy",
                                "scheduling_group_id",
                                "sla",
                                "sla_target_time"
                              ],
                              "type": "object",
                              "properties": {
                                "scheduling_group_id": {
                                  "maxLength": 30,
                                  "type": "string",
                                  "description": "The scheduling group ID.",
                                  "example": "cm2n7z2vl000e9cllp4ckoc45"
                                },
                                "name": {
                                  "maxLength": 36,
                                  "type": "string",
                                  "description": "The scheduling group name.",
                                  "example": "Infra support"
                                },
                                "color_code": {
                                  "description": "The RGB hexadecimal color code.",
                                  "anyOf": [
                                    {
                                      "type": "string",
                                      "example": "#c0106f"
                                    },
                                    {
                                      "type": "null",
                                      "description": "Null if color code is not present."
                                    }
                                  ]
                                },
                                "sla": {
                                  "maximum": 100,
                                  "minimum": -1,
                                  "type": "number",
                                  "description": "Service level target goal percentage. `-1` if not set.",
                                  "example": 75,
                                  "default": -1
                                },
                                "sla_target_time": {
                                  "minimum": -1,
                                  "type": "integer",
                                  "description": "Service level target time in seconds. `-1` if not set.",
                                  "example": 30,
                                  "default": -1
                                },
                                "asa": {
                                  "minimum": -1,
                                  "type": "integer",
                                  "description": "Average speed of answer in seconds. `-1` if not set.",
                                  "example": 30,
                                  "default": -1
                                },
                                "occupancy": {
                                  "maximum": 100,
                                  "minimum": 0,
                                  "type": "number",
                                  "description": "Occupancy percentage. `0` if not set.",
                                  "example": 85,
                                  "default": 0
                                },
                                "fixed_shrinkage": {
                                  "description": "Fixed shrinkage value to use for all days.",
                                  "anyOf": [
                                    {
                                      "maximum": 99,
                                      "minimum": 0,
                                      "type": "number",
                                      "description": "Shrinkage percentage.",
                                      "example": 20
                                    },
                                    {
                                      "type": "null",
                                      "description": "Value will be `null` if `dynamic_shrinkage` is present."
                                    }
                                  ]
                                },
                                "dynamic_shrinkage": {
                                  "description": "Dynamic shrinkage value to use, based on configured days.",
                                  "anyOf": [
                                    {
                                      "required": [
                                        "default_shrinkage",
                                        "dynamic_config"
                                      ],
                                      "type": "object",
                                      "properties": {
                                        "default_shrinkage": {
                                          "maximum": 99,
                                          "minimum": 0,
                                          "type": "number",
                                          "description": "Shrinkage percentage.",
                                          "example": 20
                                        },
                                        "dynamic_config": {
                                          "type": "array",
                                          "description": "List of shrinkage values for required intervals.",
                                          "items": {
                                            "required": [
                                              "days",
                                              "end_time",
                                              "start_time",
                                              "value"
                                            ],
                                            "type": "object",
                                            "properties": {
                                              "days": {
                                                "type": "array",
                                                "description": "List of days this configuration will apply for.",
                                                "items": {
                                                  "type": "number",
                                                  "description": "`0` - Sunday \n`1` - Monday\n`2` - Tuesday\n`3` - Wednesday\n`4` - Thursday\n`5` - Friday\n`6` - Saturday",
                                                  "example": 1,
                                                  "enum": [
                                                    0,
                                                    1,
                                                    2,
                                                    3,
                                                    4,
                                                    5,
                                                    6
                                                  ]
                                                }
                                              },
                                              "start_time": {
                                                "maximum": 1440,
                                                "minimum": 0,
                                                "type": "integer",
                                                "description": "The starting minute of a day to apply this shrinkage value.",
                                                "example": 600
                                              },
                                              "end_time": {
                                                "maximum": 1440,
                                                "minimum": 0,
                                                "type": "integer",
                                                "description": "The ending minute of a day until when to apply this shrinkage value.",
                                                "example": 660
                                              },
                                              "value": {
                                                "maximum": 99,
                                                "minimum": 0,
                                                "type": "number",
                                                "description": "Shrinkage percentage.",
                                                "example": 20
                                              }
                                            }
                                          }
                                        }
                                      }
                                    },
                                    {
                                      "type": "null",
                                      "description": "Value will be `null` if `fixed_shrinkage` is present.",
                                      "example": ""
                                    }
                                  ]
                                },
                                "deferrable_sla": {
                                  "maximum": 100,
                                  "minimum": -1,
                                  "type": "number",
                                  "description": "Deferrable channels interactions' percentage. `-1` if not set.",
                                  "example": 75,
                                  "default": -1
                                },
                                "deferrable_sla_target_time": {
                                  "minimum": -1,
                                  "type": "integer",
                                  "description": "Deferrable channels interactions' target time in hours. `-1` if not set.",
                                  "example": 30,
                                  "default": -1
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
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](/docs/api/rest/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "workforce_management:read:admin",
              "workforce_management:read:forecasts:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-macro-scopes": [
            "workforce_management:read:admin"
          ],
          "x-granular-scopes": [
            "workforce_management:read:forecasts:admin"
          ]
        }
      }
    },
    "/workforce-management/forecasts/{forecastId}/scheduling-groups/{schedulingGroupId}": {
      "get": {
        "tags": [
          "Forecasts"
        ],
        "summary": "Get a forecast for a scheduling group",
        "description": "List forecasted values for a scheduling group within a forecast.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:forecasts:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "Getforecastofschedulinggroup",
        "parameters": [
          {
            "name": "forecastId",
            "in": "path",
            "description": "The forecast ID.",
            "required": true,
            "schema": {
              "maxLength": 30,
              "type": "string",
              "example": "cm2n7z2vl000e9cllp4ckoc45"
            },
            "example": "cm2n7z2vl000e9cllp4ckoc45"
          },
          {
            "name": "schedulingGroupId",
            "in": "path",
            "description": "The scheduling group ID.",
            "required": true,
            "schema": {
              "maxLength": 30,
              "type": "string",
              "example": "cm2n7z2vl000e9cllp4ckoc45"
            },
            "example": "cm2n7z2vl000e9cllp4ckoc45"
          },
          {
            "name": "division_id",
            "in": "query",
            "description": "The division ID.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "ya2kZo77RE6LFqaoKwWM0g"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Forecast scheduling group response.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "forecast_id",
                    "forecast_intervals",
                    "scheduling_group_id"
                  ],
                  "type": "object",
                  "properties": {
                    "forecast_id": {
                      "maxLength": 30,
                      "type": "string",
                      "description": "The forecast ID.",
                      "example": "cm2n7z2vl000e9cllp4ckoc45"
                    },
                    "scheduling_group_id": {
                      "maxLength": 30,
                      "type": "string",
                      "description": "The scheduling group ID.",
                      "example": "cm2n7z2vl000e9cllp4ckoc45"
                    },
                    "forecast_intervals": {
                      "maxItems": 1000,
                      "type": "array",
                      "description": "List of forecasted values per interval.",
                      "items": {
                        "required": [
                          "forecast",
                          "timestamp"
                        ],
                        "type": "object",
                        "properties": {
                          "timestamp": {
                            "type": "string",
                            "description": "The interval's starting timestamp.",
                            "format": "date-time",
                            "example": "2025-01-01T00:00:00Z"
                          },
                          "forecast": {
                            "required": [
                              "agents"
                            ],
                            "type": "object",
                            "properties": {
                              "agents": {
                                "type": "number",
                                "description": "The forecasted staffing count.",
                                "example": 5
                              },
                              "voice": {
                                "required": [
                                  "aht",
                                  "e_aht",
                                  "e_vol",
                                  "vol"
                                ],
                                "type": "object",
                                "properties": {
                                  "aht": {
                                    "type": "number",
                                    "description": "The forecasted average handle time, in seconds.",
                                    "example": 40
                                  },
                                  "e_aht": {
                                    "type": "number",
                                    "description": "The edited average handle time, in seconds. `-1` if not edited.",
                                    "example": 38
                                  },
                                  "vol": {
                                    "type": "number",
                                    "description": "The forecasted volume.",
                                    "example": 10
                                  },
                                  "e_vol": {
                                    "type": "number",
                                    "description": "The edited volume. `-1` if not edited.",
                                    "example": 12
                                  }
                                },
                                "description": "Interval forecast for all queues in scheduling group of channel category `voice`."
                              },
                              "messaging": {
                                "required": [
                                  "aht",
                                  "e_aht",
                                  "e_vol",
                                  "vol"
                                ],
                                "type": "object",
                                "properties": {
                                  "aht": {
                                    "type": "number",
                                    "description": "The forecasted average handle time, in seconds.",
                                    "example": 40
                                  },
                                  "e_aht": {
                                    "type": "number",
                                    "description": "The edited average handle time, in seconds. `-1` if not edited.",
                                    "example": 38
                                  },
                                  "vol": {
                                    "type": "number",
                                    "description": "The forecasted volume.",
                                    "example": 10
                                  },
                                  "e_vol": {
                                    "type": "number",
                                    "description": "The edited volume. `-1` if not edited.",
                                    "example": 12
                                  }
                                },
                                "description": "Interval forecast for all queues in scheduling group of channel category `messaging`."
                              },
                              "deferrable": {
                                "required": [
                                  "aht",
                                  "e_aht",
                                  "e_vol",
                                  "vol"
                                ],
                                "type": "object",
                                "properties": {
                                  "aht": {
                                    "type": "number",
                                    "description": "The forecasted average handle time, in seconds.",
                                    "example": 40
                                  },
                                  "e_aht": {
                                    "type": "number",
                                    "description": "The edited average handle time, in seconds. `-1` if not edited.",
                                    "example": 38
                                  },
                                  "vol": {
                                    "type": "number",
                                    "description": "The forecasted volume.",
                                    "example": 10
                                  },
                                  "e_vol": {
                                    "type": "number",
                                    "description": "The edited volume. `-1` if not edited.",
                                    "example": 12
                                  }
                                },
                                "description": "Interval forecast for all queues in scheduling group of channel category `deferrable`."
                              }
                            },
                            "description": "Forecasted values."
                          },
                          "reforecast": {
                            "required": [
                              "agents"
                            ],
                            "type": "object",
                            "properties": {
                              "agents": {
                                "type": "number",
                                "description": "The forecasted staffing count.",
                                "example": 5
                              },
                              "voice": {
                                "required": [
                                  "aht",
                                  "e_aht",
                                  "e_vol",
                                  "vol"
                                ],
                                "type": "object",
                                "properties": {
                                  "aht": {
                                    "type": "number",
                                    "description": "The forecasted average handle time, in seconds.",
                                    "example": 40
                                  },
                                  "e_aht": {
                                    "type": "number",
                                    "description": "The edited average handle time, in seconds. `-1` if not edited.",
                                    "example": 38
                                  },
                                  "vol": {
                                    "type": "number",
                                    "description": "The forecasted volume.",
                                    "example": 10
                                  },
                                  "e_vol": {
                                    "type": "number",
                                    "description": "The edited volume. `-1` if not edited.",
                                    "example": 12
                                  }
                                },
                                "description": "Interval forecast for all queues in scheduling group of channel category `voice`."
                              },
                              "messaging": {
                                "required": [
                                  "aht",
                                  "e_aht",
                                  "e_vol",
                                  "vol"
                                ],
                                "type": "object",
                                "properties": {
                                  "aht": {
                                    "type": "number",
                                    "description": "The forecasted average handle time, in seconds.",
                                    "example": 40
                                  },
                                  "e_aht": {
                                    "type": "number",
                                    "description": "The edited average handle time, in seconds. `-1` if not edited.",
                                    "example": 38
                                  },
                                  "vol": {
                                    "type": "number",
                                    "description": "The forecasted volume.",
                                    "example": 10
                                  },
                                  "e_vol": {
                                    "type": "number",
                                    "description": "The edited volume. `-1` if not edited.",
                                    "example": 12
                                  }
                                },
                                "description": "Interval forecast for all queues in scheduling group of channel category `messaging`."
                              },
                              "deferrable": {
                                "required": [
                                  "aht",
                                  "e_aht",
                                  "e_vol",
                                  "vol"
                                ],
                                "type": "object",
                                "properties": {
                                  "aht": {
                                    "type": "number",
                                    "description": "The forecasted average handle time, in seconds.",
                                    "example": 40
                                  },
                                  "e_aht": {
                                    "type": "number",
                                    "description": "The edited average handle time, in seconds. `-1` if not edited.",
                                    "example": 38
                                  },
                                  "vol": {
                                    "type": "number",
                                    "description": "The forecasted volume.",
                                    "example": 10
                                  },
                                  "e_vol": {
                                    "type": "number",
                                    "description": "The edited volume. `-1` if not edited.",
                                    "example": 12
                                  }
                                },
                                "description": "Interval forecast for all queues in scheduling group of channel category `deferrable`."
                              }
                            },
                            "description": "Re-forecasted values."
                          }
                        }
                      }
                    }
                  },
                  "description": "Forecast scheduling group response."
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "workforce_management:read:admin",
              "workforce_management:read:forecasts:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-macro-scopes": [
            "workforce_management:read:admin"
          ],
          "x-granular-scopes": [
            "workforce_management:read:forecasts:admin"
          ]
        }
      }
    },
    "/workforce-management/imports/historical-queue-metrics": {
      "post": {
        "tags": [
          "Imports"
        ],
        "summary": "Upload historical queue metrics",
        "description": "Import historical queue metrics to Workforce Management (WFM).\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:write:queue_metrics:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `HEAVY`\n\n**Not supported in Gov cluster** ",
        "operationId": "Uploadhistoricalqueuemetrics",
        "requestBody": {
          "content": {
            "multipart/form-data": {
              "schema": {
                "type": "object",
                "properties": {
                  "file": {
                    "type": "string",
                    "description": "Attachment file. Only CSV format is supported. For format details, log in to WFM and visit **Forecasting -> Historical data -> Import historical data**.",
                    "format": "binary",
                    "example": "Vm0wd2QyUXlVWGxWV0d4V1YwZDRWMVl3WkRSV01WbDNXa1JTVjAxV2JETlhhMUpUVmpBeFYySkVUbGhoTVVwVVZtcEJlRll5U2tWVWJHaG9UVlZ3VlZadGNFSmxSbGw1VTJ0V1ZXSkhhRzlVVmxaM1ZsWmFkR05GU214U2JHdzFWVEowVjFaWFNraGhSemxWVm14YU0xWnNXbUZrUjA1R1UyMTRVMkpIZHpGV1ZFb3dWakZhV0ZOcmFHaFNlbXhXVm0xNFlVMHhXbk5YYlVaclVqQTFSMVV5TVRSVk1rcElaSHBHVjFaRmIzZFdha1poVjBaT2NtRkhhRk5sYlhoWFZtMHhORmxWTUhoWGJrNVlZbFZhY2xWcVFURlNNVlY1VFZSU1ZrMXJjRWxhU0hCSFZqRmFSbUl6WkZkaGExcG9WakJhVDJOdFJraGhSazVzWWxob1dGWnRNSGhPUm14V1RVaG9XR0pyTlZsWmJGWmhZMnhXY1ZGVVJsTk5WbFkxVkZaU1UxWnJNWEpqUld4aFUwaENTRlpxUm1GU2JVbDZXa1prYUdFeGNHOVdha0poVkRKT2RGSnJhR2hTYXpWeldXeG9iMWRHV25STlNHaFBVbTE0VjFSVmFHOVhSMHB5VGxac1dtSkdXbWhaTW5oWFkxWkdWVkpzVGs1V2JGa3hWa1phVTFVeFduSk5XRXBxVWxkNGFGVXdhRU5UUmxweFVtMUdVMkpWYkRaWGExcHJZVWRGZUdOSE9WZGhhMHBvVmtSS1QyUkdTbkpoUjJoVFlYcFdlbGRYZUc5aU1XUkhWMjVTVGxOSGFGQlZiVEUwVmpGU1ZtRkhPVmhTTUhCNVZHeGFjMWR0U2tkWGJXaGFUVzVvV0ZreFdrZFdWa3B6VkdzMVYySkdhM2hXYTFwaFZURlZlRmR1U2s1WFJYQnhWV3hrTkdGR1ZYZGhSVTVVVW14d2VGVnRNVWRWTWtwV1lrUmFXR0V4Y0hKWlZXUkdaVWRPU0U5V1pHaGhNSEJ2Vm10U1MxUXlVa2RUYmtwb1VqSm9WRmxZY0ZkbGJHUllaVWM1YVUxWFVraFdNalZUVkd4T1NHRkdRbFppVkVVd1ZtcEdVMVp0UmtoUFZtaFRUVWhDTlZaSGVHRmpNV1IwVTJ0a1dHSlhhR0ZVVnpWdlYwWnJlRmRyWkZkV2EzQjZWa2R6TVZZeVNrZGhNMmhYWVRGd2FGWlVSbFpsUm1SMVUyczFXRkpZUW5oV1YzaHJUa2RHUjFaWVpHaFNWVFZWVlcxNGQyVkdWblJOVldSV1RXdHdWMWxyVW1GWFIwVjRZMGhLV2xaWFVrZGFWV1JQVTBVNVYxcEhhR2hOU0VKMlZtMTBVMU14VVhsVmEyUlVZbXR3YjFWcVNtOVdSbXhaWTBaa2JHSkhVbGxhVldNMVlWVXhXRlZyYUZkTmFsWlVWa2Q0VDFOSFJrZFJiRnBwVmtWVmQxWnRjRWRWTVZwMFVtdG9VRlp0YUZSVVZXaERUbFphU0dWSFJtcE5WMUl3VlRKMGExZEhTbGhoUjBaVlZteHdNMWxWV25kU2JIQkhWR3hTVTJFelFqVldSM2hoVkRKR1YxTnVVbEJXUlRWWVZGYzFiMWRHWkZkWGJFcHNVbTFTZWxsVldsTmhWa3AxVVc1b1YxWXphSEpXVkVaelZqRldjMWRzYUdsV1ZuQlFWa1phWVdReVZrZFdXR3hyVWtWS1dGUldXbmRsVm10M1YyNWtXRkl3VmpSWk1GSlBWakpHY2xkcmVGZGhhM0JRVlRGa1MxSXhjRWRhUms1WFYwVktNbFp0TVRCVk1VMTRWVmhzVlZkSGVGWlpWRVozWWpGV2NWUnJUbGRTYlhoNVZtMDFhMVl4V25OalJFSmhWbGROTVZaWGMzaFhSbFoxWTBaa1RsWXlhREpXYWtKclV6RktjazVXWkZaaVJscFlWRlJLYjFOV1draGxSMFpYVFd4S1NWWlhkRzloTVVsNVlVaENWbUpIYUVOYVJFWmhZekZ3UlZWdGNFNVdNVWwzVmxSS01HRXhaRWhUYkdob1VqQmFWbFp1Y0Zka2JGbDNWMjVPVDJKRmNIcFhhMlIzWVZaT1JsTnJiRmROYmxKeVdYcEdWbVZXVG5WVGJGSnBWbFp3V1ZaR1l6RmlNV1JIV2taa1dHSkZjSE5WYlRGVFpXeHNWbGRzVG1oV2EzQXhWVmQ0ZDFZeVNsbFZiR2hoVWtWYVlWcFZaRXRUVms1ellVZG9UazFWY0ZkV2JHTjRUa2RSZVZaclpGZFhSM2h5VldwQ1lXTkdWblJsU0dSc1lrWnNOVnBWYUd0WFJrbDNWbXBTV2sxR1NsQldNakZHWlZaV2NscEhSbGROTW1oSlYxUkplRk14U1hsU2EyUm9VbXhLVkZac2FFTlRNVnAwVFZSQ1ZrMVZNVFJXYkdodlYwWmtTR0ZHYkZwaVdHaG9WbXBHYzJNeGNFaFBWbVJUWWxob1lWZFVRbUZoTWtwSVUydG9WbUpIZUdoV2JHUk9UVlpzVjFaWWFGaFNiRnA1V1ZWYWExUnRSbk5YYkZaWFlUSlJNRlpFUms5U01WcDFWbXhXYVZKdVFuWlhWbEpEWkRBMWMxZHJhR3RTTUZwWVdXeGFZVmRXV2xoa1J6bG9UVlZ3TUZsVldsTldWbHBZWVVWU1ZrMXVhR2haZWtaM1VsWldkR05GTlZkTlZXd3pWbXhTUzAxSFJYaGFSV2hVWWtkb2IxVnFRbUZXYkZwMFpVaGtUazFYZUZkV01qVnJWVEpLU1ZGcmFGZFNNMmhVVmxSS1JtVnNSbkZXYkdSVFRUSm9iMVpyVWt0U01WbDRWRzVXVm1KRlNsaFZiRkpYVjFaYVIxbDZSbWxOVjFKSVdXdG9SMVpIUlhoalNFNVdZbFJHVkZZeWVHdGpiRnBWVW14a1RsWnVRalpYVkVKaFZqRmtSMWR1VGxSaE1taG9WV3RXWVZsV2NGWmFSWFJVVm1zMWVsbFZaRzlVYXpGV1kwWmtWMkpIVGpSVWEyUlNaVlphY2xwR1pHbGlSWEJRVm0xNGExVXhXWGhWYkdoclUwZFNXRlJXWkRSbFZscFlUVlZrV0ZKcmJETldiWEJUVjJzeFNHRkZlRmROYm1ob1ZqQmFWMk5zY0VoU2JHUlhUVlZ3VWxac1VrTldhelZYVjFob2FsSlhhRzlWYWtwdlZERlZkMVpyZEU1aVJuQXdWRlpTUTFack1WWk5WRkpYVm0xb2VsWnRNVVpsVmxaelZteHdhVmRHU1hwWFYzQkhWakpPVjFSdVVsQldiVkpVV1d4b2IxbFdaRlZSYlVab1RXdHdTVlV5ZEc5V2JVcElaVWRvVjJKSFVrOVVWbHB6VmpGYVdXRkdhRk5pUm5BMVYxWldZV0V4VW5SU2JrNVlZa1phV0ZsVVNsSk5SbXhWVW1zNVUwMVdjREZXUjNodllWWktjMk5HYkZoV00xSjJWVlJHVDFZeFpISmFSbHBvVFd4S1dGWkdXbUZrTURGSFZtNVNhMUo2YkZkVmJYaHpUbFpzVm1GRlRsZGlWWEJKV1ZWV1UxWlhTbGxoU0VwWFlrWndTRnBGV2t0a1IwNUdUbFprVGxaWGQzcFdiWGhUVXpBeFNGSllhR0ZTVjJoVldXdGtiMkl4Vm5GUmJVWlhZa1p3TUZwVmFHdFVhekZZWlVaa1YwMXFWbkpXVkVwTFUxWkdjbUZHWkZOTk1taFZWbXhTUzFKdFZuTlNia1pYWWtkU1ZGUlVTbTlpTVZweVZXdDBVMDFXYkRSV1J6VlhWbTFLUmxOc2FGWmlSa3BZVmpGYVlWSXhiRFpTYld4T1ZqRktTVmRYZEc5U01WcElVbGhvYWxORk5WZFpiRkpIVmtaWmVXVkhkR3BpUm5CV1ZXMTRhMVJzV25WUmFscFlWa1ZLYUZacVJtdFNNV1IxVkd4U2FFMXRhRzlXVjNSWFdWWnNWMk5HV21GU1dGSlZWbTF6TVdWc2JGWmFSemxWWVhwR1Yxa3dXbXRXTWtwSVZHcFNWV0V5VWxOYVZscGhZMnh3UjFwSGJHbFNXRUpSVm0weE5HRXhWWGhYV0doV1lrZG9jbFV3WkZOWFJsSlhXa1JTYkZac2NGbFVWbFpyVjBaS2NtSkVUbGRpV0VKVVZqSnplRk5IUmtabFJtUk9ZbTFvYjFacVFtRldNazV6WTBWb1UySkhVbGhVVmxaM1ZXeGFjMVZyVGxkaGVsWllWakZvYjJGc1NsaGhSemxXWVd0d2RsWkVSbFprTVZweVpFVTFhVkp1UVhkV1JscFRVVEZhY2sxV1drNVdSa3BZVm0weGIyVnNXblJOVlZwc1ZteGFlbFp0ZUhkaFZtUkhVMWh3V0ZZelFraFdha3BQVmpGU2RWVnNRbGRpVmtwVlYxZDRiMkl4YkZkYVJsWlNZbFZhYjFSWGRIZFRWbFY1WkVjNVYySlZjRWxhVldSdlZqSktTRlZyT1ZWV2JIQjZWbXBHWVZkWFJrZGhSazVwVW01Qk1WWXhXbGRaVjBWNFZXNVNVMkpyTlZsWmExcGhWMFpzVlZOc1NrNVNiSEJHVlcxek5WWkdTbkpqU0d4WVlURndURmxXV2t0a1ZrWjFXa1prYVZkR1NsbFdiVEI0VlcxV1IxcElWbFppV0VKVVZtcEdTMWRHV2tkWGJVWnJUVVJHU0ZadE5WTmhNVW8yWWtaa1ZtSllhRE5VVlZwelZteGtjMVJzWkZkaVNFSmFWMVpXVjFVeFdsaFRhMlJxVWpKb1YxUlZaRk5YUmxWM1YydDBhMUl3TlVkVWJGcHJWR3hhV0dRemNGZGlXR2hVVlhwQmVGTkdTbGxoUjBaVFZqSm9WbGRYZEd0aU1rbDRWbTVHVW1KVldsaFphMXAzVFZacmQxZHRkR2hOYTNCSVdXdFNUMVl3TVhGV2EzaGFZVEZ3VEZwRldsZGtWMHBIWVVkb1RsZEZTalZXYlRGM1V6RktkRlp1VGxOaWExcFpXV3RrVTJJeFVsaGxSWEJPWWtad1NGWXlNVWRYUjBwWFVtcE9WVlpzY0hKV01HUkxWMGRXU1ZSc2NGZFNWbTk2Vm1wR1lXRXhaRWhXYTJoUVZtdHdUMVp0ZEhkVFZscHpXWHBHVkUxWFVrbFZNblJoWVd4T1JrNVdaRnBpUmtwSVZtdGFkMWRIVmtsVWJHUnBVakZLTmxaclkzaGlNVmw1VWxod1VsZEhhRmhXYlRGU1RVWndSVkpzY0d4V2F6VjZXV3RhWVdGV1NYbGhSemxYVmpOU1dGZFdXbk5XTWs1SFYyMUdVMWRHU2xaV2JURjZUVmRXYzJKR1dtRlNlbXhXV1Zod1IxWXhhM2RYYlhSWFRXdHdTVlpIY0ZOV1YwVjVWV3hPWVZac2NHaFpNbmgzVWpGd1NHSkhiRk5YUlVwU1ZtMHdlRTVIUlhoV1dHaFlWMGRvVjFsclpHOWpiRlYzV2taT1dGSnNTbGhYYTFKVFlrWmFjMk5HYkZWV2JGcHlWakJhUzJOdFNrVlViR1JwVjBWS1ZWWnFTbnBsUms1SVVtdGFiRkp0VWs5WmJURnZZakZhY1ZGdFJsZE5helY2V1RCV2IxVXlTa2hWYkdoVlZteGFNMVpYZUdGak1WWnlWR3hrYUdWc1dsbFdha1p2WWpKR2MxTnNhR2hTZW14WFdWZDBkMlJzV2tWU2JGcHJUVlp3ZVZwRlZURmhWa3AxVVZoa1YxSnNjRlJWVkVaaFkyc3hWMWRyTlZkU2EzQlpWbTB3ZUdJeVZuTlhibEpPVmxad2MxWnRlR0ZsYkZwMFpVaGthRlp0VWtoVk1XaDNWMFpaZWxGcmFGZGhhM0JRVm1wR1UyUldWbk5SYkdScFZtdHdWbFl4WkRSaU1rbDRZa1prWVZKc1dsTlpiRlpoWWpGU1dHUklaRlJXYkZZMVdrVmpOVll5U2xaalJXeGhWbGRTZGxacVNrdFRSbFp5VDFaV1YySklRalpXYlhCSFdWWmtXRkpyWkdoU2F6VndWVzAxUWsxV1dYaFhiR1JhVmpCV05GWlhOVk5WTWtweVRsWnNXbUV4Y0doV01GcHpWbFpPYzFOck5WTmlXR2hYVmxjeGVrNVdWWGROVm1ScVVrVmFXRlZxVG05a2JHeFhWMnQwYWsxck5VaFphMXB2VmpBd2VGTnFTbGRXYkVwSVZrUkdXbVZHY0VsVGJVWlRZa2hDZGxaR1pEUlRNV1JIVjJ0a1dHSlZXbkpVVlZKSFUwWmFkRTVXVG1oTlZXOHlXV3RhYTFadFNsbGhTSEJWWWtad2VsWnRNVWRTYkZKeldrZHNWMWRGU2t0V01WcFhWakZWZUZkc2FGUmlSM2h2VlRCV2QxZEdiSEpYYm1SVVVtNUNSMVl5ZERCaE1VbDNUbFZrVldKR2NISldSM2hoVjBkUmVtTkdaR2xYUjJoVlZsaHdRbVZHVGtkVGJHeG9VakJhVkZacVNtOVdiR1JZWkVkMGFVMXJiRFJXYlRWSFZXMUtWbGRzYUZwaE1YQXpWRlphY21ReFpIUmtSMmhPWVROQ1NWZFhkRk5VTVZsM1RWaEdWMkV6YUdGWlZFWjNWRVp3Umxkc1pHdFdNSEJJV1ZWa2IxUnNaRVpUYWxaWFlsUkdNMVY2Umt0a1JscDFWR3hPYVdFeGNGcFhWM1JyVlRKSmVGVnNhR3hTV0VKUFdXdGFkMU5HV1hsTlZXUm9VbFJHVjFsclVsTldNVW8yVm14Q1YwMUhVa3hXTVZwSFl6RldjMXBIYUdobGJGbDVWbTF3UjFsWFJYaGFSV2hYWVRKb1VWWnRkSGRVTVZwMFpFaGtWRlpzY0hwWFdIQkhWa1V4V0ZWcmJGWk5ibEpvV1ZkNFQxSnJOVmRhUm5CcFVtdHdTVlp0ZEdGa01XUklWbXRzVldKSFVuQlZha1pMVG14YWNsa3phR2xOVmxZMFZqSjBZVmRIUm5OalJtaFhZVEZhZVZwVlduTmpWa3B6WTBkNFUySldTalJXYWtvMFV6SkdTRk5yV21wU2JWSlpWbTE0UzFSR1dsVlNhemxUVFZad01WVnRlRTloVmxwWFkwWndWMkpVUVhoVlZFWlBWakpLUjFkc2FHaGxiWGhaVmtaYVlXUXhUa2RYV0d4c1VqTlNXRlJYZEhkVFZscElZMFpPVjFZd1ZqVldWM2hQV1ZaYWMyTkhhRnBOYm1nelZXcEdkMU5IU2toaVJrNVlVbFZ3VTFadE1IZGxSVFZJVWxob1YxZEhhRmxXTUdSdlZqRnNjbHBIT1dwaVJsWXpWMnRhVDFkR1NuTlRiR2hZWVRGS1ZGbHJXbUZqTWs1SFlrWldWMkpHY0hsV2FrSmhVekpPZEZOclZsVmlSMUp3VmpCV1MxWldXbkZUYm5Cc1VtczFTVlZ0ZEdGaVJrcDBWV3M1V21KVVJuWlpha1poWTFaR2RGSnNaRTVoZWxZMlYxUkNWMkl4VlhsVGEyaFdZa2RvVmxadGVHRk5NVnBZWlVkR2FrMVdXbmxXUjNocllVZFdjMWRzYkZkaGExcDJXV3BLUjJNeFRuTmhSMmhUWlcxNFdGZFdaREJrTWxKelYydFdVMkpHY0hKVVZscDNaVlp3UmxaVVJtaFdhM0F4VlZab2ExWXhTbk5qUmxKV1ZrVmFhRmt5YzNoV01XUnlUbFprVTJFelFscFdiVEIzWlVkSmVWWnVUbGhpYkVwUFZteGFkMk14V25SbFIwWnNZa1p3TUZwVmFHdGhSbHAwVld0b1ZrMVhhRE5XTUZwYVpXeEdjbUZHWkdoaE0wSlJWbTF3UjFVeVVsZFdiazVTWWtkU2NGWnRkSGRXYkZsNFdrUlNhVTFyYkRSWmExcHJXVlpLYzFkc1RscGlSbHBvV1dwR1UxZEhVa2xhUmxKWFlrWlpNVmRyVm1wT1ZsbDVVMnhhYWxKWGVHaFdiR1JUWkd4YWNWTnJkRlJTYkZwNVZERmFWMVl5U2tsUmFscFhZbGhDU0ZkV1dtdFhSa3B5V2tkb1UyRjZWbmRXVnpBeFVXc3hWMWRZYUZoaVIxSmhWbXBDVjA1R1dsaE9WazVXVFd0d2VWUnNXbk5YYlVWNFkwZG9WMDFHY0hwV2JGcFBZekZPY2s1V1RtbFRSVXBJVm14amVHVnJOVmRYYTJSVVltdHdjVlV3WkRSWFZteFZVbTVrVlZKdGVGaFdNblF3WVRGYWNsZHFSbGhoTVhCeVdWWmFhMUl4VG5OaVJtUnBWa1ZKTUZac1kzaFdNVWw0WTBWc1YySkdjSEJXTUZaTFZsWmFkR05GWkZSTlYxSklWakowYTFkSFNsWlhia3BWVmxaS1NGVXdXbUZrUlRWWFZHMW9UbFpYT0hsWFYzUmhZVEZhZEZOc2JHaFNSVFZXVm14YWQyVnNXblJsUjNScVRWWktlbGRyV210aFZrNUdVMjFHVjAxV2NGaFdha1pXWlVaa2MyRkdVbWhOYkVwNFZsZHdTMkl4V1hoVmJGcGhVbXMxV1ZWdGVGZE5NVmw1WkVSQ2FFMVZiRE5VYkZaclZsZEtSMk5JU2xkU00yaG9WakJrVW1WdFRrZGFSMnhZVWpKb1ZsWnNhSGRSYlZaSFZHdGtWR0pIZUc5VmFrSmhWa1phY1ZOdE9WZGlSMUpaV2tWa01HRlZNWEppUkZKWFlsUldTRlpYTVV0V2JHUnpZVVp3YUUxWVFYcFdSbHBoWTIxUmVGcElVbXRTTW1oUFdWUk9RMU5XWkZoa1JrNVZUVlpzTTFSV2FFZFdNa1Y2WVVkR1dsWkZXak5XUlZwM1VteGtjMXBIZEZkTlNFSkpWMVpXYTJJeFVuTmFSVnBVWVd4YVdGbHNVa2ROTVZZMlVtdDBhMUpzY0RGV1IzaFBZVmRGZUdOR2NGaFdNMUp5VmxSS1NtVkdWbk5oUjNoVFYwVktVRmRYZEdGa01VNUhWMWhzVGxaWFVtOVZiRkpIVmpGc2NsVnNUbGROVlc4eVZtMTRRMVl3TVVoVmEzaFhUVlp3U0ZsNlJsZGpNWEJJWVVkb2JHSkdiRFZXYlRGM1VqRnNXRkpZYUdGU1YyaFlXVlJLYjFWV1duUmtTR1JWVFZad01GcEZhR3RXUmxwelkwaHdXRmRJUWtoV2ExVjRWMFpXY21KR1drNWliRXB2VjFaa05GUXhTbkpPVm1SaFVtNUNjRlZ0ZEhkVFZscDBaRWRHYTAxWFVrbFdiWFJ6VmxkS1NGVnVRbFpoYTFwTVZHMTRZV05zYTNwaFIyeE9WbXhaTUZacVNqQlpWbVJJVW01T2FsSnRhRmhaVkVaaFpWWndWbGR1VG1wV2EzQjZXVEJrTkZVeVNsZFRhbEpYWVd0dk1GWkVSbHBsUms1elYyMXdVMlZ0ZUZsV2JYQlBWVEZrUjFwR1pHRlNWR3h4VkZaYWQwMVdWblJsUms1b1VsUkNORlV5ZUdGV01rcFZVbFJDVm1WclJqUlZha3BMVWpGa2MyRkhhR3hpV0dneVZteGtkMUl4YkZoV2JrNVlWMGQ0YzFWc1pGTlhSbEpZWkVaa1QxSnRkRE5YYTJNMVYwWktjMk5FUWxkV00yaFFWMVphWVdNeVRraGhSbkJPWW0xbmVsWlhjRWRrTVVsNVVtdGtWV0Y2Vms5WmJHUnFaVVphZEUxVVVtaE5SRVpJVmpGb2IxWldaRWhoUjJoV1lrZFNWRlpxUm5OamJIQkhWR3hvVjJKWVozZFdSbHBoVkRKR2NrMVdaR3BTUlVwV1ZtMTRTMUpHV2toak0yaFhUV3R3UmxadGVGZGhWa2w0VW1wT1YyRnJTbWhWZWtwUFpFWk9jbFpzVm1sU01VcDNWbXBDVjFNeFdsZFhia1pVWW01Q2MxVnRkSE5PUmxwSVRsVTVWbUpWY0VsV1Z6QTFWMnhhUms1VlVsWk5WbkJ5Vm14YVQyTldWblJoUlRWb1pXeFdNMVp0TUhoTlIwVjRZa1prVkZkSGVHOVZibkJ6Vm14YWNsWnJkRlZTYkhCWldsVmtSMkZyTVZkWGJteFlZVEpTZGxaSGVHRldWa3BWVVd4a1YxSldjREpYVjNCTFVqSk5lRlJ1VG1oU2JIQndWbXBLYjFkR1pGaGxSemxWWWxaYVNGWXhhR3RYUjBwSlVXNUtWVlpzY0ROYVZscDNVbXhhVlZKdGFGZGhNMEY0VmxaYWIyRXhXWGhUYms1cVVteEtWMVpyVm1GaFJtdDVZek5vVjAxWFVubFViRnByVlRKRmVsRnNjRmRpVkVJeldsVmtTbVZXV25WVWJHaHBZWHBXV2xkWGVHOVZNVnBYVm01R1UySlZXbFZWYlhoelRsWndWbUZIZEZkTlJFWlhXVEJhYjFkdFJuSk9WRTVYVFVad1lWcFhlRWRqYlVaSFdrZG9hRTB3U2xKV2JURjNVakZaZVZWc1pGVmlhelZZV1d4a05GWkdVbGRXYm1SWFlrWnNORmRyVWxOaFZURnlZa1JTV21FeGNISlhWbHBMWXpGa2MxUnNjR2xTYkhCdlYxZHdSMVV4V1hoalJXUlZZa1pLVkZsclpETk5WbHBJWlVaYVQxWXdXa2xWTW5SaFlXeEtSbE50YUZkaVIxSlVWVEJhWVdNeFpIUlNiR1JPWVhwV1NWWXlkR0ZXTVZsNVUydHNVbUZzY0ZoWmJHaERVMFprVjFkdGRGaFNhM0I1V1ZWa2QxVXdNWFZoUm14WFlXdEtXRlY2Ums5U01XUjFWVzE0VTAweFNsRldWM0JEVmpBMVIxZHNhRTlXYXpWWFZGZDBkMlZXVW5OWGJrNVlZbFZXTkZrd1pHOVdNREZIWTBod1YySkdjRkJaZWtaUFkyczFWMVJ0YkZoU01taFpWbTE0YW1WSFZuSk9WbVJYVjBkU1QxWnRNVk5qVmxaeVZtMUdXRkpzV25oV1J6QTFZVEpLU0dWR2FGZE5ibEYzV1ZkemVHTnJOVlpoUm5Cb1RWaENNbFp0Y0VKa01sWkhWRzVHVkdKSFVsaFphMVozVTFaa1YxVnJaRmhpVmxwSVdUQldjMVpIU2xoaFJsSlZWa1ZhVEZac1dtRlNNVnAwVW0xMFRsWXhTbGxXTW5SaFlqSktSMU5ZWkZoaVZWcFlXV3hvVG1WR1dYZFhiVVpyVWxSV1dGWkhNVzlVYkZwWVQwaHNXRll6VW1oWlZFWmhaRVpPYzJKSGFGUlRSVXBYVjFkMFlXUXlWbk5YYmxKT1ZsZFNWRmxyV2t0bGJHUnlXa2hPVjAxWFVrZFZNblF3VmpBeFYyTkdhRmRoYTFwWFdsVmFhMk15UmtkVmJXeFRUVEpvTWxac1VrTldNbEY0V2tWa2FWTkZjRmxaYlRGVFZsWldkRTVWVGxOTlZtdzFXa1ZTUTJGRk1WWmlSRTVWWWtaYWNsWnNaRXRTTWs1SlUyeGtVMDB5YUc5V2FrSnJWVzFXZEZSclpHRlNNbmhaVldwS2IxWnNXbk5oU0dSU1lsWmFTRlpIZEd0V1IwcElaVWM1Vm1GclNtaFdhMXBoWTFaT2RFOVdaR2xTTVVwYVYydFdhMDFIUmxaTldFcHBVa1pLV0Zsc1VsZFRSbHBZVFZWMFYySkhVbnBaVlZwM1lVVXhXVkZZY0ZkU2JIQm9XVEl4VW1WR2NFbFZiWFJUVFcxb1VGZFdVazlSTVU1WFdrWm9hMUpyTlZaVVZscHpUVEZTVjJGRlpGZE5hMVkyV1ZWa1IxZHNXa1pYYWs1WFVsWndNMVZ0ZUd0ak1YQklaRVprVGxORlNrcFdNVnBYWVRKSmVGVnVUbUZTVm5CVldWZDRZVmRHYkhOVmEyUk9UVlpaTWxWdGREQmhNVXB6WTBSR1YxSXphRkJaVm1SR1pVZE9SMk5HYUZkTk1VcDVWbXhTUzFOdFZrZFdiazVvVWpOb1ZGWXdWa3RWUm1SWVpFZEdhMDFWTlVoV01XaHpWR3hhV0ZWc2JGWmhhMG96VmpCYVYyUkhUa1pQVm1SWFlraENObGRVUW05VE1WbDNUVlZvVm1FemFGaFVWV1JUVjBaVmVGZHNUbXBOYXpWSVYydGFUMVl5U2xWaGVrcFhZbFJHTTFWcVJuTlhSa3BaWVVkR1UxWXlhRmxYVmxKTFlqRldWMWR1VW10VFIxSldWRlphZDJWc1dsaGtSMFpWWWtWd2Vsa3dhSGRYUjBWNFUyeFNXbFpXVmpSV01HUlhVakpHUjFkck5XbGlWMmcxVmpGYWEwNUdVWGhUYmtwUFZtMVNhRlZ0TlVOalJsWjBaRWhrVkZac2NEQmFSV1JIVjBkS1YxZHNiRmRpV0ZKMldWUkdTbVZzUm5WUmJGcHBWa1phUlZadGNFZFpWbVJHVFZaV1ZtSlhhRTlXYlhSM1RteGFjVk5VUmxkTlZsb3dWVzAxVDJGc1NuUmxSbWhhVmtWYWFGWkZXbk5XYkdSMFVtMXdWMkV6UVhkWGExWmhWREZTYzFkWWFGUmlWVnBaVm10V1MyRkdaRmRhUldSWVVtdHdXbGt3V2t0aFZrbDZZVVpzV0ZZemFIWldWRVpyVWpGd1IxcEhSbE5pVmtwNFZrWmFhMVV3TVZkV2JsSnNVbFJzYjFadGRITk9SbFY1VGxjNVYwMVZjSHBaTUdoTFZqRmFSbU5HYUZwbGEzQk1WV3BHYTJSR1NuTlZiRTVYVW14d1lWWnRkR0ZaVm14WVZWaGtUbFp0VWxoWmEyUnZZekZXYzFwR1RsaFNiRXBaV2tWYWExUXhTblJrUkU1WVlUSk5NVll3V2t0ak1rNUZVV3hrVjAweWFGRldNVnBoV1ZkTmVWUnJhR2hTYmtKUFdXMHhibVZzV2xoalJXUnJUVlZzTlZWdGRHdFdWMFkyVm1zNVdtSkhVbkZhVlZwaFpFVXhWVlZ0YUU1U1JWcEpWbXBHYjJJeFdsZGFSV2hvVWpKb1YxbHJaRk5XUmxweFVtNU9hbUpJUWtoV1IzaHZWakpLV1ZvelpGZFNiSEJvVmtSS1IyTnJNVmRhUjNCVFVsVndWMVp0TVRSVE1VcEhWMnRvYWsweWFGaFVWM1IzVW14YWRHVkdaR2hXYTJ3MFZXMXdUMVl5Um5KVGJXaFdaV3R3VUZwRldrOWpiRnB6Vm0xc1UwMVZjRE5XYlhCRFZqRk5lRmRzWkZoaWExcFZXVzAxUTJNeFZuUmpla1pVVm14YU1GcEZaRWRoVmtweVkwUkNWMVl6VFRGV01qRkxWbFpLZFZkc2NHaGhNWEJ2VjFSR1lWSXlVa2RUYms1aFVsUldiMXBYZUZwTmJGbDRWV3RPVjAxcmJEUldiR2h6VmtkRmVXRkdaRnBoTWxKMlZtMTRZV1JIVmtoU2F6Vk9WbTVDV2xkc1ZtdFNNa1p5VFZac1VtRXpRbGxaVkVaM1ZrWmFTR1ZIUmxkV2EzQldWVmQ0YTFZeFNsZGpSRXBZVjBoQ1NGWnRNVmRXTVU1ellrZHNVMDF0YUZWWGJHTjRUa1prUjFwR2FHdFNNRnBWVkZaa1UxTkdXWGhoUjNSWVVqQndlVlJzYUV0V2JVWnlWMnQ0Vm1GcldqTlZiWGhoVmxaS2RHRkdaR2hOTUVZMFZteGFZV0l5UlhoWFdHaFVZbXMxY1ZWdGVFdFdNVnB5Vm0xR2FGSnRkRFZaZWs1dlZqQXhXRlZ1YkZWaVJuQnlWbFJLUm1Wc1JuTmpSbVJwVmtWR05GZHJVa2RYYlZaSFdraEtZVkp0YUhCWlZFWjNWbXhrVjFWclpHdE5hekUwVmpJMVUySkdTWGRYYXpsWFlsaG9hRlJWV25kV2JIQkdXa2RvVTJKSVFqVlhWM1JUVlRGYWRGTnJhR2hUUlhCWVdWZDBkMVJHV25OWGEzUlhWbXRhZWxrd1pEUmhSVEIzVTJ4R1YwMXVhRmhaVkVaU1pVWk9kVlZzVG1oTmJFcFVWMWQwYTFVeVRrZFhibEpyVTBkU1dGbHJXbk5PUm1SeVZXdE9WMDFFUm5wVmJYQlRWMGRGZVdGR1FsZE5SMUpIV2xaa1IxTldVbk5YYldob1pXeGFSbFp0ZUZkWlZteFlWV3RvVjJFeWVGWlpXSEJ6Vmtac2NscEVUazVXYkhCSldsVmFZVlF4V1hkWGEyeFdUVzVTYUZsWGVFdGtSMVpJVW14a2FWSnVRWHBYYkdRMFdWZE9WMVJ1U2xoaVdHaFVXV3RvUTFsV1pGZFdiWFJUVFZaV00xUlZhSE5oUmtwSFkwWm9XbUpIYUVSVk1GcHJWakZrZEdSR2FGTmhNMEkxVmpKMGEySXhWWGxTYWxwWFltMVNXRlp1Y0VOTk1WSnpWbFJHVTAxWFVscFpWVnBoWVVVeFJWWnRhRmRpV0VKRVZtcEJNVkl4WkhOaFJUbFhWa2Q0V2xaWE1IaFZNVTVYWTBaYVdtVnNXbGhaYkZaWFRrWlplV05GT1ZkTlJFWklXVEJhVTFZeVJuSlhia3BoVW14d00xVXdaRWRTTVZKMFlrWk9hVmRHUlhoV2JURTBZVzFXUmsxVmFGWlhSM2hWV1ZSS1UxWldiSFJqZWtaWFZtMTRXVnBGWkVkWFJrcDBaVWh3V2xaWFRURldiRnBMWkVkV1IyRkdaRTVpYkVZelYxWmtlbVZIVG5KT1ZscG9VbTVDYjFSV2FFTmxWbHB6Vld0MFZFMVZjREJXUjNSaFlWWktkRlZzYkZaTlJscE1WV3RhWVZJeFpIUmtSbFpwVm14d1NWZFVRbGRqTVZwSVUyeG9hRkpzU2xaV2ExWjNaR3hhYzFkdFJsTk5WbkF3VlcweE1GUnRSWGhqUld4WFlXdHJlRlpVU2tkak1VNXpZVVphYVZKc2NGZFdiVEUwV1ZkV2MxVnVTbGhpVlZwWlZXeFNWMVl4VVhoWGJUbFdUVVJHU2xWWGVHOVdNa1p5WW5wQ1dsWkZXbWhhUlZwUFl6SktTR0ZHVGxOV1JscFlWbTB4TkZsWFVYbFNhMlJZVjBkU1dGWnJWbUZYUm14eVYydDBiR0pIVW5sV2JGSkhZVVpLVlZGcVRsWk5ibEYzVm1wQmVGWXlUa2RoUm1Sb1lURndXRmRzVm1GaE1sSlhWRzVLVDFadFVuQldiWFIzVGtaYWMxcElaRlJOYTJ3MFdXdGFhMVp0U2toVmJHeGFZbFJHVkZacVJsZGtSVEZWVVdzNVUySkhkekZYYkZaclRVZEdTRkpxV2xOaVIzaFlWbTV3VjJWc1duTmFSWFJUVFdzMVNsVXllR3RWTURCNVlVYzVWMkZyU25KV1Z6RlhaRVpTY2xwSFJsTk5ibWhhVmxkd1MySXhXbk5YYms1b1UwZFNVMWxzV21GVFJscElaRWQwVjFJd2NFbGFWV00xVm0xR2NsTnRhRmRoTVhCb1dURmFUMlJGT1ZkYVIyeFRUVlZ3WVZZeWRGZGhNREZJVW14YVRsZEZjSEJWYTFwTFYwWnNjMVZzWkZSU2JIQlpWRlpXTUZZeVNraFZibkJhVmxad00xbFZWWGhYVmtaWlkwWmtVMkpJUW05WFZFbDRWVzFXUjJORmJGWmlXRkpVVkZjeGIxVkdaRmRWYXpsU1RWVTFXRmRyV205aE1VcHlUbFphVlZac2NIcFVWRVpUVmpKR1JscEdaRTVoTVZreVYxWldhMUl4WkhOWGExcFlZV3MxV0ZWc1duZE5NVlowWlVkMGFrMVlRa2xhUlZwclZHeE9SbE5yY0ZkTlZuQllXVlJLU21WR1ZuVlViVVpUVm01Q1ZsZFdaRFJaVm1SSFlraE9WbUV5VWs5VVZscGFUV3hXZEdSRVFtaE5WWEI2V1RCU1IxWXhTWHBoUmtKYVZtMVNSMXBWWkU5U01rNUhZVVprVGsxVmNEVldiWGhUVXpGT2RGWnJaRmRpYXpWdlZXcE9iMVpHV1hkV2EzUldVbTFTV1ZwRlpFZGhNREZXVGxWb1YwMVhhSHBXUjNoaFYwWldjMkpHV21oaE1HOTZWbXBDWVZZeVVraFVhMXBRVm0xU1dGbHJhRU5PYkdSVlVXMDVWRTFXYkROVVZsWnZWbTFGZW1GR2FGVldWMUpVVmpCYVZtVkdaSE5VYlhCT1lYcFdTVll5ZEdGVU1WbDVVMnhXYUdWcmNGaFpiRkpIVFRGV2NWSnVUbGhTYTNCYVdWVmtSMVV4V1hsaFJteFlWbXhLVEZscVJsZFdNV1IxVlcxMFZGSXhTbGRXUmxwaFpEQXhSMk5GV21oU00xSllWVzE0ZDJWc2EzZGhTR1JZVW0xU1NsVlhkRzlXTWtWNFZtcFNWMDFHY0ZoWk1qRkxVakZrY2s1WGJHbFNXRUl5Vm1wR1lXRXdOVWRWV0doVVltdHdVRlp0TVZOaFJsWjBUbFZPYWxKc1dqQmFSV2hyVmtaYWMyTkVRbUZTVjFKSVdWUkJlRmRHVm5KaVJtUnBWMFpLTmxadGVHdFRNVTVIVm01V1VtSkdjRzlaVkVaM1ZGWmFTRTFVVWxwV01EVjZXVlJPZDJGR1NYbGxSVGxYWWxSRmVsUldXbUZYUjFaSVQxZDBUbFpzY0RaV01uUlhZakpHUjFOdVVtaFNiV2hoV1d0YVMxTkdXa1ZTYm1SWFlrZFNNVlpITVhkV01rVjZVV3BhV0ZaRlNuSlVhMXBXWlZaT2MxcEdVbWxXVm5CYVZtMTBWMU15VG5OaVNFWlRZbFZhVkZSV1ZURmxiR3h5VjJ0MGFGWnJiRFpaVlZwelYwWlplbUZJV2xkV1JWcHlWV3BHZDFJeGNFaFNiRTVwVm10d05GWnJXbUZoTVZWM1RWWmtWMkpzU25OVmJGSnpZakZhZEdSSVRrOVNiRlkxVkd4ak5WWXdNVlpqU0hCYVRVZG9URlpxUm1GU2JVNUhZVVp3YkdFelFrMVdWM0JIWVRKU1IxTnVUbFppUlRWWVZXMTRkMWxXV25Sa1IwWmFWbTE0V1ZaV2FHdFViRnAwVld4b1dtRXlVblpaZWtaWFl6RndSMVJzYUZOaVJYQmFWMnRXWVdFeFVYaFRibEpyVWtVMVdWbFVTazVOVmxsM1YydDBhazFyTlVaVlYzaHJWakF3ZVdGR1JsZE5WbkJvVjFaa1RtVldVbkpoUjJoVFltdEtVRmRYTUhoaWJWWnpWMnhvYWxKWFVuSlVWbFp6VGxaV2RHUkhkRmRXTUZreVZtMXpOVmR0U2toVmJrWmhWbFp3YUZwRlZYaFdWbFowWVVVMVUxSldjRXBXYlhCSFlqSkplRmRzWkdGU1YyaHZWV3BLYjFZeGJISmFSazVYVW14d2VGVnROVTloYlVwSlVXdG9WMUl6YUhaV2FrcExWMVp2ZWxwR1pGTmlTRUp2Vm10U1MxSXlUbkpOVm1SaFVqTkNWRlZzVm5kV01WcDBUVlJTVkUxVmNFaFdNblJ2WVdzd2VXRkdhRlZXTTFJelZXMTRZV1JIVmtkYVIyaFhZVEozTUZaWE1YZFVNVmw1VW01S1ZHSlhhRmRhVjNSM1ZVWndSbGRyT1dwaVJuQjZWbTB4YzFVd01WWmpSRTVYWWxoQ1JGWnFSazVsUm1SeldrWm9hV0V4Y0ZaWFZ6QXhVVEpOZUZadVVtcGxhMXBWVm0xNFlVMUdjRVphUldSb1ZtdHdlVmt3VWtOV01WbDZWRlJHVjAxcVJreFdiRnBMWkZaa2MyTkhhR2hOV0VKMlZqRlNSMWxXU1hsVmEyUlVZbXMxYUZWc1VsZGlNWEJZWlVad1RtSkdiRFJXVjNSUFZqQXhjbUpFVWxkaVIyaDZWbXRrUm1WWFJrZGFSbkJvVFd4S01sWnRNVFJaVjFKSVZtdG9VMkY2Vms5V2JUVkRWMnhrYzFadGRGTk5hMXA1VkZaYWMxVnRSWHBSYkd4YVZrVmFNMVpyV21GVFIxWklVbXhrVjJFelFsZFdWbVEwWVRGWmVGTnNaR3BTUlhCWlZqQm9RMU5HV25STlZtUlhUVlUxZWxsclpITlZNREYwWVVaV1dHSkhUalJVYTFwclVqRndSMkZIZEZOTlJuQldWa1phYTFVeFRrZFhXR2hvVWpOU1dWVnFRbmRsVmxKellVVjBWMkpWV25sV01uUTBWbTFHY2xkcVRsWmlXR2hvVm14YVMyUkhSa2RhUjJoT1RVVlpNRlp0ZUdGWlYwbDVVbGhvV0ZkSGFGVlpiWE14WTFaV2RHVkZkRmROVm5CNVZtMHhSMkZHU25Sa1JGWmFaV3MxZGxacVFYaFhSbFoxWWtaV2FWSnVRbmxXYkZKTFVtMVdjMUp1VG1wU2JWSndWbXRXU21Wc1pITldiWFJVWWxaYVdGWXlOVmRXVjBwSVlVaENXbGRJUWxoV01uaHJWMGRXU0U5V1drNVdia0paVm0wd01WSXhWWGxUYkZwWVlrWmFWMWxzYUc5bGJGSnpWMjVrVjJKVldrbGFSVnByVkd4S1JsZHNhRmhXUlVwb1dXcEdhMlJHU25WVGF6bFhVbFJXV1ZaR1ZtRmtNa1pIWWtSYVUySllVbkpWYWtKaFUwWmtjbHBIT1doU1ZFSXpWVEo0UzFZeVNsVlNhazVXWVd0YVYxcFdXbGRqTVhCSFdrZHNhR1ZzV2xWV01XUXdZVEZWZVZSclpGaGliRXB5VlRCa1UxZEdiSEpYYms1UFVtMVNlVlpYZEU5V01ERkZVbTV3Vm1KWVFsUldNR1JHWlZkR1IxRnNaRTVTTVVwSVYyeGFhMUl4VGxkVmJrNVlZbGRvYjFwWGVHRlNiRnAwWlVaa1dsWnRlRmxXVjNSdlZqSkZlV1ZHV2xwWFNFSjZWbXRhYzJOc2NFVlVhelZYWWtoQ1NsZHNWbUZaVmxGNFYxaGtXR0V5ZUZkVVZ6VlRZVVpzV0dWRmRHdFNNVnBKVlcxNGEyRldTblZSV0hCWVlrZFJNRmRXWkU5V01WSjFVMnhvYVZkSGFGVlhWM1J2VVRGc1YxcEdaRmRpV0ZKVVZGWmFWMDB4VWxaaFNFNW9VbXR3U0Zrd1dtOVhiVXBJWVVWU1ZrMUdjSEpXYWtaclpFZFNSMkZHVG1oTk1Fa3hWakZhVjJFeVVYaFdXR1JPVjBaYWIxVnRNVFJYUm14WVpFVjBXRkpzV1RKVmJYTTFWVEF4V0ZWcVJsZFdla1V3VmxSQmQyVkdUbk5TYkdSWFRUQktSVlp0Y0VKTlZrbDRXa2hPYUZKVWJGaFdhMlEwVjJ4YVdFMUlhRlpOVlRWWVdUQmFZVmR0Vm5OWGJHaGFZa1phV0ZScldscGxWMDVHVDFaa1RsSkZXa2xYVkVKdlpERlpkMDFWYUZaaVJrcFhXV3hTUjFaR1ZYZGFSVGxVVWpCd1NGWkhNWE5WTURCNFVsaG9WMkpVUlRCWlZ6RlhVbXN4Vmxkck5WZFdNbWhWVjFjeE5GTXhXa2RpUm1oc1UwZFNXVlZ0TVRSbFZtUnlXWHBXV0ZKc2NEQmFWVnBoVm0xS1ZWWnVTbHBXVm5CTVdrVmtWMUl5UmtkYVJUVnBZa1ZaZWxZeFdtdGxiVlpJVkc1S1QxWnNjRzlWYlRWRFlqRlNWMkZGVGs1aVJuQXdXVEJXUzJFd01YTlhiR3hXWWxoU2RsWlZXbUZrUmxaellVZEdWMDB4U2t4V1JsWmhXVmROZUdFemNHaFNiVkpQVm14a00wMUdaRlZSYkdScVRWWnNOVlV5ZEd0aGJFNUdVMnhvVlZaV2NHaFdSVnBoWXpGa2RGSnRkRk5OUkZaSVZqSjBZVlV4VW5OWFdHaFVZVEo0V0ZsWGRIWmtNV3hWVW0xMFZGSnJOWHBXTW5odllWZEZlbEZzWkZkaVdHaG9WWHBHVDFJeFdsbGFSbWhwVmxad2VWWlhlRk5XTVdSSFYydG9UbFo2YkZaWmExcDNWMVpSZDJGSVRsaFNiR3cyVmxjeGIxZHNXa1pYYldoaFVsWndVRmw2U2t0VFIwWkhWR3hPVjFKc2NGbFdiVEI0VGtac1dGVlliRmRpYXpWWldXMTRTMk14Vm5GVWJFNXFWbXhLV0ZZeWREQmlSMHBJVlc1c1YwMXFSWGRaVlZwaFkyczFWbHBHV2s1aGExcEpWbTF3UW1WR1dYbFVhMlJvVW0xU2IxbFVSbmRYUm1SelZtMUdhRTFyTVRWVmJYUnJWbTFLV0dGRk9WcGhNVm96Vkd0YVlXTXhjRWxhUmxKT1lUSjNNRlpxUm05aU1WcDBVMnRrYWxJeWFGZFpiR2hPWlVaWmVGZHNaR3RTTVZwSVYydFZNVlJzV25Sa2VrWllZVEZhVkZWVVNrZFNhekZYVjIxc1UxSlVWbGRYVjNSaFdWZE9jMWR1VG1GU1dGSlVWRmR6TVZOc1ZYbGxSM1JXVFVSQ05WbFZZelZXTWtaeVUyNUtWMVo2UmxoYVJWcFRZMnh3U0ZKc1RrNWliV2hoVm14a2QxTXlTWGhYYms1WVltczFiMVV3WkZOVk1XeHpWMjFHVkZKc2NFbGFSV1F3VmpGWmQyTkZiR0ZXVjJoRVZtMHhTMWRXVWxsaFJuQm9ZVEZ3VlZacVFtdFdNVXAwVld0a1lWSXllRlJVVmxaM1dWWmFjbGR0Um1sTmJFWTBWbGMxVDFkSFNuSk9XRVpXWWxSR2RsZFdXbk5XVms1MFQxWldVMkpXU2xsV2Frb3dUVVpSZVZOc1dtcFNWMmhoVm14YWQwMHhiRlpXV0doWVZtdGFXbGt3V205aFZrbDRVbGhrVjJGcmJEUldha1poWTJzMVYxcEhhRk5OTVVwVlYxZDBZV1F5VW5OYVNFNWhVa1ZLWVZadE1WTlhSbVJ5VjJ4a1ZtSlZjRlpaYTJoSFZsWmFjMk5JY0ZWV1JWcFVWbXBLUjFKc2NFZFhhelZvVFZacmVGWXhXbGRoTVVsNFYxaHNVMkpyY0ZCV01GcDNXVlphYzFkdVpHeFdia0pZVjJ0Vk5WUXlTa1pqU0hCYVZsWndjbFpIZUU5U2JFNXlZMFprVGxadVFsbFhWbEpMVTIxV1IxWnVWbFZpVlZwVVZtMDFRMVpzWkZoa1JtUnJUVmRTU0ZaWGVHdFhSMHAwVlcwNVZtSlVSbFJXTUZwYVpWZE9ObEp0YkZOaVNFSTJWbFJLZDFJeFdYaFhXSEJvVW0xb1lWWnNXbGRPUmxweFUyczVVMDFyTlVoWlZWcHJWR3N4Vm1ORVZsZGlXR2hVVlhwR1VtVkdUbGxoUm1ocFlrWndWVmRYZEZka01XUkhWbTVPVjJKdFVsZFZiWGgzVjFaU1ZtRkhPVmRoZWtaWlZsYzFkMWRzV2taT1dFcFhZV3R3VEZZeFdsZGpNa1pIVm14a1RrMVZiRFpXYlhCRFdWWlplVlZyYUZWaE1YQlJWbTB4VTFkR2JISmhSVTVQWWtad2VGVXhVa2RoTURGWVZXNXdWMDF1YUhKWlYzaExWMWRHU0ZKc1ZsZGxiRnBRVjJ4V1lWZHRVWGhYYkZaWFlrZFNUMWxZY0ZkVGJHUlhWbXhhVGxZd1dsaFdNV2h6WVVaS1dGVnNhRnBpUmtwSVZGUkdWMk5XU25WVWJHaFRZVE5DWVZkV1ZtRmlNV1JIVTFoc2FGTkhhRmhXYWs1dllVWmtWMWRzV214U01IQktWa2N4YjFVeVJqWldibWhYVm5wRk1GcEVRWGhTTWs1R1YyeG9hV0pXU2xkWFYzUlhaREpXYzJFelpHaFNlbXhZV1Zod1IwMVdWbGhsUms1WFRXdGFlVlZ0Y0ZOV01rWnlWMjFvV2sxV2NHaFpla1pyWTJzMVdHSkhiRk5YUlVWNFZtMXdSMWxYUlhkT1ZXUldWMGRvVlZsdGRIZFZSbHAwVFZaT2FGSnNXakJVYkZaUFlXeEtjMWRxUW1GU1YyaHlWbXRhWVdNeVRrVlJiVVpUVmpGS1NWWnRNWHBsUmxsNVUydFdWbUpIYUc5VVZ6RnZWVlphY1ZGdGRGUk5WMUpaVlcxMGEyRkdTbk5YYkdoWFlsaFNNMVpyV210ak1XUjBVbXh3VjJKV1NraFdSbHBoVmpGYWRGTnNhR3hTVkd4WVdXeG9iMWxXVWxkWGJVWllVakZhU1ZReFpEQlViRnB6WWpOa1YxWkZiekJYVmxwclUwWk9jbUZIYUZOaVYyaG9WMWQwWVZNeFRrZFhhMVpUWW0xU1dGbHJaRk5OVmxwSVRWaGtWMDFyVmpaVlYzQmhWMFphUmxOdVNsVmhNWEJYV2xaYVQyTXhjRWRWYld4WFZtNUNZVlp0TVRCV01VMTVWRzVPYVZORldsUlpiRlpoVmtaU1YxWlVSbXhpUjNoNVZqSjBNR0ZHV25KaVJGSldUVzVvTTFacVNrdFdWa3BWVVd4YWJHRXhjRlZXVjNCSFlUSk5lRmR1VGxWaVYyaHpXV3hvYjFac1duSlhiVVphVmpGR00xUldXbXRXTWtwelUyNU9WbUpHU25wWmFrWmhaRWRXU1ZwR2FGTmlXRkY2Vm1wSk1XRXhXWGROV0VwcVVteHdWMVp0TVc5a2JGcDBaVWRHVTJKSFVucFZWM2hyWVVkRmVsRnJNVmRXTTBKSVdWUktUbVZXU25KaFIyaFRUVEpvZDFacVFtdGlNV3hYVjI1U2ExSXdXbUZXYlRFMFYxWmFWMkZIT1ZkV01GWTFWbGQ0YzFZd01VaFZWRUphWld0YVdGa3hXbE5qTVZKellVWk9hVkpZUW1GV01uUlhZakpGZUZkclpGaGlhelZ4Vld4a05GbFdVbFpYYm1SV1VteHdlRlZ0ZUhkaE1ERlhZMGhvVjJKWWFISldha0YzWlZkR1IxSnNaRTVXYmtKdlZqRmFhMVJ0VmxkVmJrcG9VakpvVkZsdGRFdFZSbVJZVFVob2FXSldXbnBXTW5odllXeEtXRlZ1U2xWV2JGcDZWR3hhVjJSSFVraGtSbVJYVFVacmVWWnRNWHBPVm1SMFUydG9WbUpyV21oV2JGcDNZVVp3UmxaWWFGZFdiRm93V1ZWVk1XRldTbGRoTTJSWFlrZFJNRmxxU2twbFZrcHpXa1pTYVdKRmNIWldWekUwWkRGc1YxZHVTbGRpYlZKWVZXMTRjMDB4V1hsT1ZUbG9UVlZzTkZZeWRHdFpWa3BZVldwT1YxSXphR2haZWtaclYxZE9SMXBGTldsaVJYQjJWbTEwVTFNeFNYbFVia3BPVm0xU2IxVnFUbTlXVmxaeFZHMDVXRlpzY0VoWGEyaHJWakF4VjFkcmFGZE5ibEpZVmxSS1MxWnNaSFZUYkdScFYwWktiMWRzWkRSaE1rNXpXa2hXWVZKdFVrOVdiVEV6VFVaYVdFMVlaRlZOVm13MVZXMDFTMVpIU2xoaFJtUmhWak5TTTFaVlduZFNiR1J5VDFkd2FWSnVRalpXYlRCNFl6RlZkMDFXV21wU1JUVllXV3hvUTFKR1duRlNhelZzVW0xU2VsZHJXbTloVjBWNFkwWmFXRlo2UVhoVlZFcE9aVVphZFZWdGRGUlNWRlpSVmxjeE5HUXhUbGRWYms1WFZrWmFXRmxzVm5kV01XdDNWbTVPVjAxV2JEWldSbEpIVmpBeGRXRkhSbUZXTTJoWVdrVmtSMUl4WkhKT1ZrNXBZVEJ2TVZacVJtOWtNVWw1VWxoc1ZtRXlVbFZaYlhNeFlVWldjMVZzWkdsTldFSlpXbFZhZDFSdFNrZGlSRkpYVmpOU1VGWnFRWGhqYlVwRlZXeGthVkl5YURaV2JYUnJVbTFXV0ZKclpHRlNia0paVlRCV1MwNXNXblJOVkZKYVZqRktXVlpIZEdGWlZrbDZWVzVDVm1KWWFHRlVWbHByWXpGYWRGSnRkRTVoTTBKSlZsUkpNVlF4WkVoVGJsSnNVakJhVmxac1pGTlZNVkpXVjIxR1YxWnJOWGxYYTFVMVlWWktXVkZzWkZkV2JWRXdXVlJHV21WR1RuTmFSM0JUVWxoQ1dsWnRjRTlSTVZKSFZtNUdWR0Y2Vm5OVmJYaExUVlphU0UxWE9WWk5SRVl3V1ZWYVlWWXhXWHBoUm1oaFVrVmFjbFZxUm5kU01rWklaVVpPYUdWc1dsWldhMXByVGtaTmVWWnVUbGhYUjNoUVZteG9VMWRXVm5GUmJtUlhUVlpzTlZSc1ZtdFdNVnB6WWtSYVYxWjZSblpXYlRGSFkyeGtkR0ZHV2s1U01VcFpWbGR3UjFVeVRYaFhibEpwVW1zMWNGbFVRbHBOUmxwMFpVZEdXbFl3YkRWVmJHaHZWMFprU0dGR1ZscGlXRTE0V1RGYVYyUkhWa1prUm1ST1ZtNUNOVlp0TUhoU01rWkhVMjVTYTFKR1dtRmFWM014Wkd4YWNWRllhRmhTYkZveFdXdGFWMkZXU25WUmJteFhZa1pLVEZWdE1WZGpNVXAxVkcxd1UySldTbEJXYlhCSFVqQXdlRnBHWkZaaE0xSlZXV3hhWVZkR1duUk9WbVJYVmpCd1NWbFZXbk5XYlVaeVYydDRWMDF1YUhKV01HUlhVMFU1VjFkck5WZGlhMHBhVm0wd2QwMVdTWGhXYmxKVVlrWndXVmxyV21GV01XeHpWV3RhVGxKdVFsaFdNbmgzWWtaYVZWWnNiR0ZTVjFKMlZsUktTMU5XUmxsYVJtUnBVakZHTTFkWWNFZGhNazE0V2toT2FGSnNXbGhWYkZwM1YxWmtWMXBFUW10TlZrcElWakkxVDFkSFJYcFZiR2hYWWxoU2FGcFhlSEprTVhCRlZXMW9VMVpGU1hwV1ZFbzBVekZaZDAxV2FHaFNSVFZYV2xkMFlWVXhjRlpYYXpsclZteGFlbFpITVc5VWJVcEdZMFpXVjAxWFVYZFpWRVpXWlVaV1dXRkdXbWxTYmtKWlYxY3dlRlV4WkVkaVNFcFlZbXMxVUZWdE1WTmxWbHBZWkVVNWFGSnNjRlpWYlhoclZqSktXVkZzUWxkaVIxSk1WV3BHYTJSV1RuTmFSVFZUVFZWd1RsWXhVa3BsUmxGNFZHdGtWR0pyTlZoWmEyUlRWa1paZDFacmRGZGlSbkF3V2xWYWExZHNXblJsUm14WFZqTlNkbFpxU2tabFYxWkhWMnh3V0ZORlNqWldiWFJoWXpKT2MyTkZWbFZoZWxaWVdXdG9RMU5zWkhOV2JVWmFWbXhHTkZZeGFHdFhSMFY1WlVaa1ZWWjZWbFJaVlZwelYwZFdSbVJIZEZOTlZuQktWbGN4TkdFeVJsZFRibFpTVjBoQ1dWbHJaRkpOUm1SWFdrVTVWMDFXU2pGVk1uaHZZVlprU0dGR1ZsaFdNMUoyVlZSR2MxWXhjRWRhUm1ScFYwWktWVlpHV21GV01EQjRWV3hhYUZORk5XOVZiWFIzVTBacmQxZHRPVmhpVlhCSldWVldVMVpyTVVkalNFcFhZV3RhY2xwRlpGTlNNa1pIVkcxb1RrMUZhM2hXYlhCTFRVZEZlVk5ZYUZkWFIzaFVXVmQ0ZDFkR2JIUk5WazVZVW14d01Wa3dWa3RpUmtwelYyeHNXbVZyTlhaV2FrRjRWMFpXYzJOR2NGZFdia0l5VjFaYVlWTnRWa2RVYmxKb1VtMVNjRll3Vmt0U1ZscDBZMFZLVGxacmJEUlhhMmhQWVVaS1ZXSkdhRnBYU0VKWVZqQmFZVkl4Y0VWUmJYUk9Za1p3V1Zac1l6RlVNV1JJVTI1S1QxWnRlR0ZaYTFwaFYwWlNjbGR1U210TldFSkpWREZhYTFSc1NrWlhhMnhZVmtWS2RscEVSbHBsVms1eVdrZHdVMkV3Y0doWGJHUXdXVlpXUjJFemJHcGxhMXBZVkZaYVMwMUdiSEpYYlhSb1VsUkNNMVV5TVc5V01rcFZVV3BTWVZKRldrOWFSRUY0Vm14a2NrNVdaRmROYldoV1ZqRmplRTFIVVhoVldHaFlZbXhLVDFadGN6RlhSbHAwVFZSU1dGWnVRa2RYYTFaclYwWktjbU5JY0ZkTmJrSklWakp6ZUZJeFpIVlRiSEJYVWxoQ1RWWlhjRXRVTVVsNFdraE9XR0Y2Vm5CWmExWmFaV3hhZEdWR1RsSmlWbHBKVmtjMVMxUXhXbk5qUnpsV1lsUldSRmxxUmxOV01rWkdWRzE0VjJKSVFqWldiR1F3VFVaYVJrMVdXbXBTUlVwWVZXcE9VMkZHV2tobFIwWnJVakJhU0ZsVldsZFdNVXBXWTBVeFYxWkZXbWhYVm1SUFl6RmFkVk50UmxOaVYyaDNWbTB4TkdReFRsZGFSbFpTWWtkU2NWUldXbmRUVmxWNVpVYzVhRkpyY0hwVk1uaGhWakpGZUZkcVRtRlNNMmhvV1hwR2EyUkdTbk5VYXpWWFltdEdNMVpzWTNkbFJURkhWVzVTVTFkSGFIRlZiR1EwVjBac1dHUklTbXhTYkhCV1ZXMTRUMkZyTVZkVGJteGFWbGRTZGxaVVNrdFdNVTV6V2taa2FWWkZWWGRXTVZwclZqSk5lRlp1VmxSaVIyaHdWbXhhZDFkc1dsaGxSazVYVFZac05GWldhRzlYUjBwVllrWmtWVlpXU2toVk1uaFhWMGRXU0U5V1pFNWhNWEExVjFjMWQxRXhaSE5YYTFwWVlsZG9hRlZzV25kVlJtdzJVMnQwVkZJd1draFdSM2hQVmpGS2MyTkdiRmRpVkVJMFZHdGFjMWRHVmxsaVJtUnBZWHBXV2xkWE1UUlRNazE0Vm01R1ZXSkZOWEZXYlhoTFYwWlplV1JFUW1sU01IQkpWbGMxYzFZd01YRlNXR1JhVmxad2FGa3ljekZXYkhCSFYyczFhV0pYWkRaV2JURjNVVzFXUjFkWWFGVmlhM0JYV1d0b1ExZEdWbk5YYm1SVlVtMVNlbFp0TVVkaFJURnlUVlJTV2sxR1duSlpWVnBLWlZkV1NFOVhSbGROTW1odlZrZDBZVmxYVFhoalJXeGhVbXhLVDFsVVNqTk5SbVJZWkVkR2FFMXJXbnBWTWpWVFlVWk9SbU5HWkZwV00xSk1XVlZhWVZKV1NuTlViR1JPVmpGSmQxWlhNSGhPUjBaWFUyNVNhMlZyV2xsWmExcGhZMnhTVmxwRlpGTmhla1pZVmpJeGMxVXhTbFZXYkdSWVZqTlNhRmRXWkU5ak1XUjFWRzFHVTFkR1NsVldSbVI2VFZaT2MxWlliR3hTTTFKWFZGVlNRMDVXYkZaWmVsWlhUV3RhZVZVeU5VdFdNVm8yVW14b1lWSldjRlJaTW5oM1UwWktjMVJyTldsaVYyaG9WbTE0YTJReFRYbFRXR2hZWW1zMVdWbHRjekZpTVZWM1drWk9WazFYZUhwV01qRkhWa1pLYzJORmJHRlNWMUYzVm1wR1NtUXlUa1ppUjBaWFZqQXdlRlp0TUhoU01rNXpWRzVTYUZKdFVtOVVWbWhEWWpGa1ZWSnRSbFZOYkVwSFZERmFhMWxXU1hsbFJsSlZWbXhhTTFkV1dscGxWMUpIV2tkb1RsSkZXa3BXVnpFMFdWWlNjMXBGV21wVFJVcFhXVlJHZDFSR1dYZFhiazVxVFZaYWVsZHJXbE5WTWtwSlVXeHdWMUpzY0ZoVVZWcGFaVlpPY21GR1dtbGlhMHBvVm0xNFlXUXdNSGhpUmxwWFYwZG9XVlp0ZEdGWGJHdDNWMjEwVmsxcmNFaFpNR00xVjBkRmVGTnJhRmhXYlZKVVZXcEdUMlJXVG5SaFJrNU9UVlZ3VmxZeFpEQlpWMUY1Vlc1T1lWTkZOV2hWYTFaTFdWWmFkRTFXVGxoU2JWSjVWako0YTFadFNsWmpSbHBhWVRKb1NGWnNXbUZqTWs1SlkwWldWMUpXY0ZWV2JYQkxVakZLYzJORlpGaGhlbFp2VkZaYWNtVkdXWGhYYkdSb1RWVnNORll4YUhOVk1rcHlZMFpDVjAxR2NETlhWbHBoWXpGa2RWcEhlRmRpYTBwWlZtMTRiMkl4V1hsU2FscHBVbTFvV0ZWcVRtOWpiR3hXVmxob1dGSlViRlpXUjNocllWWkpkMDVFUWxkU00xSm9WbGN4VjFJeGNFZGlSMFpUVmtaYVdWZFhkRzlSTVdSelYyNUdVbUpVYkZOVVZtUlRVMFpzY21GRk9WZGlWVlkxVmxkek5WWldXbk5qUjBaVlZqTm9XRnBGWkU5T2JFcDBZa1pPYVZORlNUSldiRkpMWkRGSmVGZFlhRlJYU0VKdlZXMHhiMWxXV25STlZrNVRUVmhDV1ZwVlZtdGhNVnAwVlc1d1dHRXhjSEpaYTJSR1pESkZlbHBHWkdsWFIyaFZWbGR3UjJNeFdYaFViR3hvVW0xU2NGbFljRmRYVm1SWVpFZEdWRTFyTlVoV01qVkxXVlpLUjFOdVNsVldiSEF6VlRCYVdtVlZNVlphUjJoT1VrVmFObFpxU2pSaU1WbDNUVmhXYUZOR2NHRldhMVpoVkVaV2RHVkZPV3BOV0VKSVZtMTRhMVl5UlhwUmEzQlhZbGhDUkZkV1pFNWxWbEoxVkd4a2FXRjZWbGRYVjNSclZURlJlRmR1VGxwbGJFcHpWbTE0ZDAxR1VsZFZiRTVvVmxSR1dGbHJVbGRXYXpGMVlVWm9WMVpGY0VkYVZ6RkhVakZ3UjJOSGJGTk5iV2Q1Vm0xNFYxbFhUWGRPV0U1VlltdHdhRlZ0Y3pGVU1WcHhVbXR3YTAxWFVsbGFSV1JIVmxVeGNsZHJaRlZXVjJoeVZtdGtTMVpzWkhOaFJuQnBVbXR3U1ZaR1VrZFdNazVYVTI1V1ZXSkhVazlaVkU1RFUxWlplV1JIUmxaTlYxSklWVEZvYzJGR1RraFZiR2hYWVdzMVZGbFZXbmRTVmtwMFVteE9UbUV6UWt0V2EyTjRVakZWZUZOdVNtcFNiVkpZVlcxNGQyRkdjRVZUYkdSVFRWWndNVlpYTVRCVk1VcFdZMFpzVjFaNlJUQmFSRUY0VWpGa2MyRkhlRlJTVkZab1ZtMHhNR1F4WkZkWGJHaHJVbnBzV0ZSWGRHRmxWbFowWTBaT1YwMXJXbmxXTWpWTFdWWktkRlZzUWxaaGEzQklWVEJrUjFKck5WZGFSMnhvVFVacmVGWnRNWGRTTVd4WFYxaG9hVkp0VWxaV01HUTBWbXhaZDFwSE9WZGlSM2d3V1ROd1IxZEdXblJsU0d4WVlURndVRmxYZUV0ak1rNUpZa1prYVZaRldrbFdiWEJIV1ZkTmVWUnJWbEppVlZwVVZtcEdTMU5XWkZkVmEwcHNVbTFTU0ZadE5VdGhWa3AwVlc1Q1YwMUdjRXhVYlhoaFpFZFdTR1JHVWs1V1ZGWkpWbTB3TVZZeFZuUlRhMlJxVW5wc1lWWnJWbUZOTVZwSFYyMUdWRkpVUmxaV1YzaDNWakF4UlZacVdsaFdiRXBRV2tSS1IxSXhUbGxpUjNoVFZrZDRXVmRYZUc5aU1XeFhWV3hhV0dKVlduRldiWE14WlVaV1dHVklaRmROYTNCWVZUSjBkMVl4V2paUldHaFhVa1Z3U0ZsNlJrOWtWbFowWVVaT2FWWXlhRzlXYlRFd1ZqRnNXRkpyWkdwU2JWSlpXVzB4VTFkR2JISmFSazVPVm14d1NWUldhR3RXTURGeVZtcGFWbFl6UW1oV01GcGhVbXhPZFZOc1ZtaE5XRUpZVjJ4YVlWUXlUWGhqUldSV1lsZDRXRlp0TlVOWlZscDBUVlJDVjAxVk1UVlZiWFJyV1ZaS2RGVnRhRlppUjFKVVZqQmFVMVl4V2xWU2JHaHBVak5vTlZZeWRHcE9WbHB5VFZoS2FsTkhlRmRVVnpWdlpXeFpkMWRzY0d4aVZWcElWbGQ0WVdGV1pFWk9SRUpYVmpOQ1RGWkVSa3RqTVdSMVZHeFdhVll6YUhaWGJHUTBXVlprYzFkdVNsaGlXRkp6V1Zod1IxZEdXa2hrUjNSWVVtMVNTVnBWYUU5V2JVcDFVVzV3V21GcldtaGFSVlY0VmpGU2MxVnNUbWxTYkd0NFZtdGtOR0l4VVhoWFdHUk9WMFZ3Y1ZWdGVIZFdNV3h5V2tjNWFtSkhVbGhXTW5Rd1ZUSktWazVWYUZwV1ZuQXpXV3RrUm1WSFRrbGlSbWhvVFZoQ2IxWnRjRXRVTWs1eVRsWmtZVkl5YUZSWldIQlhWMVphV0UxVVVtdE5WM2hZVmpJMVUyRXhTWGRYYlVaWFlsaG9lbFJzV21GU01rWklaRVpXVGxZeFNraFdSM2h2VVRGYWRGTnNiR2hTUlRWWVdWZDBZV1ZzYTNsbFNHUlRUVlpLZWxsVldrOWhWa3BaVVd4c1YySllhSEpVVldSR1pVWndSbHBHYUdsaGVsWjRWbGN3ZUdJeGJGZFhiazVoVW10d2NsUldXbk5OTVZsNVpFUkNhR0Y2Um5sVWJGWmhWbXhaZW1GSVNsZGhhMFl6V2xaa1IxSXhTbk5hUlRWWFltdEtkbFpxUm1GaE1WSjBWbXhrVm1KcmNHOVZha3B2Vmtac2RHUkZkRlpTYlZKWlZGVm9iMkZYU2xkWGJuQlhUVzVvZGxsWGVFdFhSbFoxVTJ4V1YySkdWalJXVkVaaFZtMVdXRlZyYUZCV01taFlXVlJPUTA1V1draGxSMFpxVFZad01GVXlkR3RYUjBwMFpVZG9WMkpHY0ROWlZWcDNVbXhhV1dGR1pGZGlSbTkzVjFaV1ZrMVdWWGhUYkdScVVsaENXRmxzYUVOVFJteHhVMnR3YkZKdFVucFhhMlJIVlRGWmVXRkljRmRXZWtVd1YxWmFhMUl4Vm5OWGJVWlRZWHBXZVZkWGRHdE5SVEI0VjFoc2JGSXpVbTlVVmxaM1ZteFdkR1ZIUmxkTlZtOHlWVzF3VDFZeVJuSmpSWGhXWVd0d1VGVnNXbXRqTVhCR1RsZHNhVkpZUWpKV2JURXdWVEZOZUZOWWJGVmlhM0JQVm0weFUyRkdWbkZVYTA1WFVteGFlbFp0Tld0aFZURlhZMFJDWVZaWFVraFdNR1JMVmpGS2NWVnNjR2hOYldoTlZqRmFZVk14WkZkV2JrNVdZa1phV0ZSVVFYaE9SbHBZVFZSU1YwMVhVa2xXYlhScldWWktkRlZyT1ZkaVZGWkVWR3hhYTFaV1JuUlNiWEJPVm0xM01WWlhNREZUTVZwSVUyeG9hRkp0VWxaV2FrNURVMFphVlZKdVpGZE5WMUo2VmtjeGQyRldTWGhUV0dSWVZteGFhRlpFU2tkWFJrcFpXa2R3VTJFeGNGbFhWM1JyVkRBd2VGVnNaRmhoTVhCeVZGVlNSMWRzV2tobFJ6bG9WakJXTkZrd1dtRlhSbG8yVVZob1ZsWkZjRTlhVlZwaFpGWldkR05IZUdoTldFSlhWbTB3ZUU1R2JGWk5WV1JxVWxad2FGVnNaRk5pTVZwMFpFaGtiRlp0ZEROV1YzUlBWakF4VjJKRVdsZFNiRXBJVm1wQmVHTldTbkpsUm1ScFVtNUNUVmRyV21GaE1rMTVWR3RvYUZJeWFFOVdNRlpMVFd4YWNWSnNUbE5OVjNoWVZURm9jMVp0UlhsVmJGWmFZbGhOZUZsVldsZGpNVlp6V2tkc1RsZEZTbHBYYkZaaFpERlNkRk5yWkdsU1JrcFlWbXBPVTFWR1duRlJXR2hxWWxWd1JsWlhlR3RoVmtsNVlVUmFWMkpZVW5KVmJURlhWMFpLY2xwSFJsTmlhMHBhVmxSQ1YxTXdOWE5YYmxKT1ZrWktZVlpxUmt0WFZscElaRWRHV2xacmJEWldWelZEVm0xS1dXRkhSbFZXYkhCeVdYcEdZV1JHU25Sa1JUVm9UVEJLU2xac1VrcE5WbEY0V2tWb1ZHSnNTbk5WYWtKaFZteGFjMXBHVGxaTlZrcFhWakl4UjFZeFduSk9XR3hZWVRGVk1WbFdXa3RYVmtaeldrWmtWMkpXUlhkWFZsSkhWMjFXUjFSdVRtRlNNMEpVVld4b1ExZHNXblJqUlRsU1RWWktTRlp0TlVkV1IwWTJWbTVPVm1KWWFFeFVWbHBoVTBVeFdFOVdXazVoTTBKTFZsWmpNV0V4V1hsVGJHeFNZVE5vWVZsWGRHRmpiRlp4VW01a1UwMXJXa2xhVlZwUFlrZEtXV0ZHYUZkTlYxRXdWMVphYzFZeFVuSmFSbWhZVWpKb2RsWlhjRTlWTVdSWFZtNUdVbUp0VWxWWmExcGhWMVpTYzFacVFsZE5WbkI2V1RCb2QxZHJNVWRUYkVKWVZteHdhRmw2U2s5U1ZsSnpWMjFzV0ZJeWFIWldiVEUwV1ZaTmVWVnJhRk5oTWxKb1ZXcEtiMVF4VWxaV2JIQk9VbXh3TUZwVlpFZFhiRmwzWWtSV1ZtSkhhSFpYVmxwTFYwWldjMkpHY0ZkU1ZtOTZWbTE0WVdNeVRuTmpSV2hRVm01Q1dGbHJhRU5PVmxwMFRVaG9VMDFyV2pCVk1uUnZZVEpXY21OR1pGcGlSa3BIV2tSR1dtUXhaSFJTYlhSWFlURndOVmRXVm1GaU1rWkdUVmhPVkdGcmNGbFdiWGgzWVVaV05sSnRkRmROVlhBeFZrY3hkMVV5U2xkVGJFSlhZbGhDU0ZWcVFYZGxSbkJIV2taYWFFMHhTbFpYVjNoaFdWWmtSMWR1VWs5V2JWSllWbXBDZDFOV2JGWlhiazVYWWxWYWVWWXlkRFJXTURGMVlVaEtWMDFIVWxoVk1GVTFWakZrY2s1V1pHbGhNSEJoVm0wd2QyUXdNVmRUV0doWFlteEtWVmxyV25kWFZteHlWMjFHVjAxWGVIcFpWV00xWWtkS1IxTnVjRmROYWxaeVZtcEtTMVpzWkhKaVIwWlRWakZLTWxac1VrdFRNVTVYVW14c1lWSnRVbkJWYWtaSFRURmtXV05GWkdoTmF6RTBWMnRvVDJGR1NuTlhiR2hXWWtad00xWXhXbXRXTVhCRlVXMTBUbFpyY0VsV2Fra3hWREZrU0ZKWWNGWmlSMmhZV1d4b1UyRkdjRmhsU0VwclRWaENSbFpYTVc5Vk1WcEdVMWh3VjFaRmJ6QlZla1phWkRBeFYyRkhhRk5TVkZaWVYxWmtNR1F4VlhoalJscFlZa1UxY1ZSV2FFTlRSbHAwVFZoT2FGWnJjRnBWVjNCWFZqSktTRlJxVW1GV00wNDBWbXBHVjFkWFJrZFhiV3hUWW10S1dsWXhZM2RsUmxWNFdrVmthVk5GY0ZsWmJYTXhWVVpXZEUxV1RteFdiSEJaV2tWVk5WZEdTWGRqUldoYVlURktWRlp0TVV0WFZrWnpVV3hrYVZJeFNrMVdWM0JIWVRKU1YxUnVUbUZTVkZaVVZGWldkMVJzV25OVmEwNW9UVlZzTkZVeU5VdFZSbHAwVlcxb1ZtRnJjSFpXUkVaaFpFVTFWbFJzVm1sU2JrRjNWMnRXWVZReFZYZE5WbVJwVWtaYVdGWnFUbE5rYkZweFUydGFiRll4V2toV01uaDNWMFpPUmxOcVdsZGlXRkpvV1hwS1QyTXlUa1pXYkZab1pXeGFXVmRYZEc5Uk1VNXpXa2hPVjJKVldtRldiWGhIVGtaWmVHRkhkRmhTTUZZMVdsVmFhMVpXV2paV2JGSmFWbFp3ZWxZd1pGZFNiVkpJWkVaT1RsWnVRa3RXYkdONFRrZEZlVkpzWkZSaWJGcFRWakJvUTFkV2JGVlRiRXBPVW0xU1dGZHJWVFZXTURGWFUycEdWazFxUmtoV1ZFRjNaREZPZFZkc2FHbFNNVVYzVmpGYVlXRXhXWGhhU0ZaVllsVmFWRlZ0ZEhka2JGcEhWMjFHVjAxRVZraFdNV2h6VkRGYWRGVnNaRnBpUmxwb1dsZDRjbVF4WkhSUFZtaHBWbGhDU2xkV1ZtOVpWbVJ6VjJ4a2FsSnRhR0ZVVmxwM1ZFWndSbHBHVGxOTmExcElWa2N4YjFSck1VWmhNMmhYWWxoQ1RGUlZXbk5XTVdSWllrWk9hVkl4U2xaV2FrSnJZakZhVjJKSVVteFRSMUpaVlcxNFlVMUdjRlpXYWtKV1RWVndlbFp0Y0ZkWGJWWnlUbGhhV2xZelRqUldNRnBYWTFaS2MxcEdaR3hoTVZsNlZtMTRVMUZ0VVhsV2EyUlZZbXMxVkZsclpGTlhSbkJZWkVWd1RsSnNjRWxhUldSSFlVVXhWbUpFVWxoaE1taFlWbXhrUzFkSFJrZFdiRnBwVW10d1dWWnFSbUZaVm1SSVZHdGFWV0pIVWs5VmExcGhVMVprVjFWck9XaE5WMUpKVlRJMVMxUXhXblJoUmxwWFlrWndURlV3V2xabFJtUjFWR3hrYVZKdVFqWldhMlEwWVRGU2MxTnJXbGhoTW5oWlZtdFdTMk5zYkhGU2JYUnJVbXh3TVZVeWVHOWhWMFY2VVcwNVYxWjZSVEJhUkVGM1pVWmFkVkpzVG1sU1ZGWjRWMVpvZDFZeFpFZGFTRXBoVW5wc1ZsbHJXbk5PYkZwWFlVaGtWMDFyY0ZaVlYzQlRXVlpLUmxkck9XRlNSVVkwVm0xNGQxSXhaSE5oUms1T1lsZG9hRlpxUm1wbFJUVkhWRmhvV0dKSFVuQlZiWGhoVmtac2RFMVdUbGhTYkhBd1ZGWlNVMkpHV25OaVJGWlZZa2RSZDFacVFYaGpiVTVHWVVad1RsSXlaM3BXYlhCQ1pVZFNWMVp1VmxSaVIxSlVWRlJCTUUxR1duUmpSV1JvVFdzMU1GVnRkR3RYUjBwWVpVWlNXbFpGTlVOVVYzaGhaRWRXU0ZKc2NGZGlSM2N3Vm1wS05GbFdVbk5YYkdSWVlsUnNhRlpxVGtOUk1WRjRVbFJzVVZWVU1Eaz0="
                  },
                  "timezone": {
                    "type": "string",
                    "description": "The timezone of the intervals in the CSV file. The default is `UTC`.",
                    "example": "America/Los_Angeles"
                  },
                  "allow_overwrite": {
                    "type": "boolean",
                    "description": "When set to `false`, the import fails if existing data conflicts with the uploaded data.",
                    "example": true
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "File uploaded successfully.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "import_id": {
                      "type": "string",
                      "description": "The import's unique reference ID.",
                      "example": "cmgoyz7mb00002f6obth9ojea"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "workforce_management:write:admin",
              "workforce_management:write:queue_metrics:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-macro-scopes": [
            "workforce_management:write:admin"
          ],
          "x-granular-scopes": [
            "workforce_management:write:queue_metrics:admin"
          ]
        }
      }
    },
    "/workforce-management/imports/{importId}/historical-queue-metrics": {
      "get": {
        "tags": [
          "Imports"
        ],
        "summary": "Get historical queue metrics import metadata",
        "description": "Retrieve a list of historical queue metrics import metadata.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:queue_metrics:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\n\n**Not supported in Gov cluster** ",
        "operationId": "Viewhistoricalqueuemetricimoprtmeta",
        "parameters": [
          {
            "name": "importId",
            "in": "path",
            "description": "The import's unique reference ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "cmgoyz7mb00002f6obth9ojea"
            }
          },
          {
            "name": "division_id",
            "in": "query",
            "description": "The division ID.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "ya2kZo77RE6LFqaoKwWM0g"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Historical queue metrics import metadata response.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "create_time",
                    "file_name",
                    "import_id",
                    "imported_by",
                    "row_processed",
                    "status",
                    "timezone",
                    "total_rows"
                  ],
                  "type": "object",
                  "properties": {
                    "status": {
                      "type": "string",
                      "description": "The import's status.",
                      "example": "SUCCESS",
                      "enum": [
                        "PENDING",
                        "FAILED",
                        "SUCCESS",
                        "DELETING"
                      ]
                    },
                    "create_time": {
                      "type": "string",
                      "description": "The import creation time.",
                      "example": "2025-11-05T04:13:40.800Z"
                    },
                    "row_processed": {
                      "type": "number",
                      "description": "The number of rows processed from the import file.",
                      "example": 1000
                    },
                    "total_rows": {
                      "type": "number",
                      "description": "The number of rows present in the import file.",
                      "example": 1000
                    },
                    "import_id": {
                      "type": "string",
                      "description": "The import's unique reference ID.",
                      "example": "cmgoyz7mb00002f6obth9ojea"
                    },
                    "file_name": {
                      "type": "string",
                      "description": "The import file's name.",
                      "example": "HistoricalData.csv"
                    },
                    "timezone": {
                      "type": "string",
                      "description": "Themetrics data's timezone.",
                      "example": "America/New_York"
                    },
                    "imported_by": {
                      "type": "string",
                      "description": "The unique identifier of the user who imported the metrics.",
                      "example": "3U-_aQo_ROeigseGAQL5_A"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "workforce_management:read:admin",
              "workforce_management:read:queue_metrics:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-macro-scopes": [
            "workforce_management:read:admin"
          ],
          "x-granular-scopes": [
            "workforce_management:read:queue_metrics:admin"
          ]
        }
      }
    },
    "/workforce-management/organizational-groups": {
      "get": {
        "tags": [
          "Organizational Groups"
        ],
        "summary": "Get multiple organizational groups",
        "description": "Get multiple organizational groups.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:organizational_groups:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\n\n**Not supported in Gov cluster** ",
        "operationId": "getMultipleOrganizationalGroups",
        "parameters": [
          {
            "name": "name",
            "in": "query",
            "description": "The organizational group's name.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "West Coast Operations"
            }
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of items to be returned per page.",
            "required": false,
            "schema": {
              "maximum": 50,
              "minimum": 1,
              "type": "integer",
              "example": 15,
              "default": 10
            }
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "The next page token from the previous page, if available. Not required for the first page.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "cmgoyz7mb00002f6obth9ojea"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Get multiple organizational groups response.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "organizational_groups",
                    "page_size",
                    "total_records"
                  ],
                  "type": "object",
                  "properties": {
                    "next_page_token": {
                      "type": "string",
                      "description": "Use the next page token to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size.",
                      "example": "cm2n7z2vl000e9cllp4ckoc45"
                    },
                    "page_size": {
                      "type": "integer",
                      "description": "The number of items returned per page.",
                      "example": 15
                    },
                    "total_records": {
                      "type": "string",
                      "description": "The total number of items available across all pages.",
                      "example": "45"
                    },
                    "organizational_groups": {
                      "maxItems": 50,
                      "type": "array",
                      "description": "List of organizational groups.",
                      "items": {
                        "required": [
                          "created_at",
                          "description",
                          "name",
                          "org_group_id",
                          "updated_at"
                        ],
                        "type": "object",
                        "properties": {
                          "org_group_id": {
                            "type": "string",
                            "description": "The organizational group's ID.",
                            "example": "cmgoyz7mb00002f6obth9ojea"
                          },
                          "name": {
                            "type": "string",
                            "description": "The organizational group's name.",
                            "example": "West Coast Operations"
                          },
                          "description": {
                            "type": "string",
                            "description": "The organizational group's description.",
                            "example": "Organizational group for west coast teams"
                          },
                          "created_at": {
                            "type": "string",
                            "description": "The organizational group's creation time.",
                            "example": "2024-01-15T10:30:00Z"
                          },
                          "updated_at": {
                            "type": "string",
                            "description": "The organizational group's update time.",
                            "example": "2024-01-15T10:30:00Z"
                          }
                        }
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
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "workforce_management:read:admin",
              "workforce_management:read:organizational_groups:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-macro-scopes": [
            "workforce_management:read:admin"
          ],
          "x-granular-scopes": [
            "workforce_management:read:organizational_groups:admin"
          ]
        }
      },
      "post": {
        "tags": [
          "Organizational Groups"
        ],
        "summary": "Create an organizational group",
        "description": "Create an organizational group.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:write:organizational_groups:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\n\n**Not supported in Gov cluster** ",
        "operationId": "createOrganizationalGroup",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "name"
                ],
                "type": "object",
                "properties": {
                  "name": {
                    "type": "string",
                    "description": "The organizational group's name.",
                    "example": "West Coast Operations"
                  },
                  "description": {
                    "type": "string",
                    "description": "The organizational group's description.",
                    "example": "Organizational group for west coast teams"
                  },
                  "scheduling_group_ids": {
                    "maxItems": 100,
                    "type": "array",
                    "description": "The IDs of the scheduling groups to add to the organizational group.",
                    "example": [
                      "cmgoyz7mb00002f6obth9ojea"
                    ],
                    "items": {
                      "type": "string"
                    }
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "Create organizational group response.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "created_at",
                    "description",
                    "name",
                    "org_group_id",
                    "updated_at"
                  ],
                  "type": "object",
                  "properties": {
                    "org_group_id": {
                      "type": "string",
                      "description": "The organizational group's ID.",
                      "example": "cmgoyz7mb00002f6obth9ojea"
                    },
                    "name": {
                      "type": "string",
                      "description": "The organizational group's name.",
                      "example": "West Coast Operations"
                    },
                    "description": {
                      "type": "string",
                      "description": "The organizational group's description.",
                      "example": "Organizational group for west coast teams"
                    },
                    "created_at": {
                      "type": "string",
                      "description": "The organizational group's creation time.",
                      "example": "2024-01-15T10:30:00Z"
                    },
                    "updated_at": {
                      "type": "string",
                      "description": "The organizational group's update time.",
                      "example": "2024-01-15T10:30:00Z"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "workforce_management:write:admin",
              "workforce_management:write:organizational_groups:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-macro-scopes": [
            "workforce_management:write:admin"
          ],
          "x-granular-scopes": [
            "workforce_management:write:organizational_groups:admin"
          ]
        }
      }
    },
    "/workforce-management/organizational-groups/{organizationalGroupId}": {
      "get": {
        "tags": [
          "Organizational Groups"
        ],
        "summary": "Get a single organizational group",
        "description": "Get a single organizational group.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:organizational_groups:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\n\n**Not supported in Gov cluster** ",
        "operationId": "getOrganizationalGroup",
        "parameters": [
          {
            "name": "organizationalGroupId",
            "in": "path",
            "description": "The organizational group's ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "cmgoyz7mb00002f6obth9ojea"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Get a single organizational group's response.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "agent_count",
                    "created_at",
                    "description",
                    "name",
                    "org_group_id",
                    "queue_count",
                    "scheduling_groups",
                    "updated_at"
                  ],
                  "type": "object",
                  "properties": {
                    "org_group_id": {
                      "type": "string",
                      "description": "The organizational group's ID.",
                      "example": "cmgoyz7mb00002f6obth9ojea"
                    },
                    "name": {
                      "type": "string",
                      "description": "The organizational group's name.",
                      "example": "West Coast Operations"
                    },
                    "description": {
                      "type": "string",
                      "description": "The organizational group's description.",
                      "example": "Organizational group for west coast teams"
                    },
                    "created_at": {
                      "type": "string",
                      "description": "The organizational group's creation time.",
                      "example": "2024-01-15T10:30:00Z"
                    },
                    "updated_at": {
                      "type": "string",
                      "description": "The organizational group's update time.",
                      "example": "2024-01-15T10:30:00Z"
                    },
                    "agent_count": {
                      "type": "integer",
                      "description": "The number of agents in the organizational group.",
                      "example": 42
                    },
                    "queue_count": {
                      "type": "integer",
                      "description": "The number of queues in the organizational group.",
                      "example": 8
                    },
                    "scheduling_groups": {
                      "maxItems": 100,
                      "type": "array",
                      "description": "Associated scheduling groups",
                      "items": {
                        "required": [
                          "name",
                          "scheduling_group_id"
                        ],
                        "type": "object",
                        "properties": {
                          "scheduling_group_id": {
                            "type": "string",
                            "description": "The scheduling group's ID.",
                            "example": "cmgoyz7mb00002f6obth9ojea"
                          },
                          "name": {
                            "type": "string",
                            "description": "The scheduling group's name.",
                            "example": "Morning Shift"
                          }
                        }
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
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "workforce_management:read:admin",
              "workforce_management:read:organizational_groups:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-macro-scopes": [
            "workforce_management:read:admin"
          ],
          "x-granular-scopes": [
            "workforce_management:read:organizational_groups:admin"
          ]
        }
      },
      "delete": {
        "tags": [
          "Organizational Groups"
        ],
        "summary": "Delete organizational group",
        "description": "Delete an organizational group.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:delete:organizational_groups:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\n\n**Not supported in Gov cluster** ",
        "operationId": "deleteOrganizationalGroup",
        "parameters": [
          {
            "name": "organizationalGroupId",
            "in": "path",
            "description": "The organizational group's ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "cmgoyz7mb00002f6obth9ojea"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "Delete organizational group response."
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "workforce_management:write:admin",
              "workforce_management:delete:organizational_groups:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-macro-scopes": [
            "workforce_management:write:admin"
          ],
          "x-granular-scopes": [
            "workforce_management:delete:organizational_groups:admin"
          ]
        }
      },
      "patch": {
        "tags": [
          "Organizational Groups"
        ],
        "summary": "Update an organizational group",
        "description": "Update an organizational group.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:update:organizational_groups:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\n\n**Not supported in Gov cluster** ",
        "operationId": "updateOrganizationalGroup",
        "parameters": [
          {
            "name": "organizationalGroupId",
            "in": "path",
            "required": true,
            "schema": {
              "type": "string"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "name": {
                    "type": "string",
                    "description": "The organizational group's name.",
                    "example": "West Coast Operations"
                  },
                  "description": {
                    "type": "string",
                    "description": "The organizational group's description.",
                    "example": "Organizational group for west coast teams"
                  },
                  "scheduling_group_ids": {
                    "maxItems": 100,
                    "type": "array",
                    "description": "The IDs of the scheduling groups to add to the organizational group.",
                    "items": {
                      "type": "string",
                      "example": "cmgoyz7mb00002f6obth9ojea"
                    }
                  }
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "description": "Update organizational group response.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "created_at",
                    "description",
                    "name",
                    "org_group_id",
                    "updated_at"
                  ],
                  "type": "object",
                  "properties": {
                    "org_group_id": {
                      "type": "string",
                      "description": "The organizational group's ID.",
                      "example": "cmgoyz7mb00002f6obth9ojea"
                    },
                    "name": {
                      "type": "string",
                      "description": "The organizational group's name.",
                      "example": "West Coast Operations"
                    },
                    "description": {
                      "type": "string",
                      "description": "The organizational group's description.",
                      "example": "Organizational group for west coast teams"
                    },
                    "created_at": {
                      "type": "string",
                      "description": "The organizational group creation time.",
                      "example": "2024-01-15T10:30:00Z"
                    },
                    "updated_at": {
                      "type": "string",
                      "description": "The organizational group update time.",
                      "example": "2024-01-15T10:30:00Z"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "workforce_management:write:admin",
              "workforce_management:update:organizational_groups:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-macro-scopes": [
            "workforce_management:write:admin"
          ],
          "x-granular-scopes": [
            "workforce_management:update:organizational_groups:admin"
          ]
        }
      }
    },
    "/workforce-management/reports/adherence/agents": {
      "get": {
        "tags": [
          "Reports"
        ],
        "summary": "List agents' adherence data",
        "description": "Get agents' adherence data.\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:list_adherence_agents:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "listAdherenceAgents",
        "parameters": [
          {
            "name": "date",
            "in": "query",
            "description": "The date in 'yyyy-MM-dd' format.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "2025-09-01"
            }
          },
          {
            "name": "timezone",
            "in": "query",
            "description": "The adherence timezone. The default is UTC.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "America/Los_Angeles",
              "default": "UTC"
            }
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "Use the next page token to paginate through large result sets. A next page token returns whenever the set of available results exceeds the current page size.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "jA5csQv1W1oXuYZLspNIZzMOXqTD9r9Rje2",
              "default": "null"
            }
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of agents returned per page.",
            "required": false,
            "schema": {
              "maximum": 10,
              "minimum": 1,
              "type": "integer",
              "example": 10,
              "default": 10
            }
          },
          {
            "name": "division_id",
            "in": "query",
            "description": "The division ID.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "ya2kZo77RE6LFqaoKwWM0g"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "List of agents with adherence for the date.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "agents",
                    "next_page_token",
                    "page_size",
                    "timezone",
                    "total_records"
                  ],
                  "type": "object",
                  "properties": {
                    "next_page_token": {
                      "type": "string",
                      "description": "Use the next page token to paginate through large result sets. A next page token returns whenever the set of available results exceeds the current page size.",
                      "example": "jA5csQv1W1oXuYZLspNIZzMOXqTD9r9Rje2"
                    },
                    "page_size": {
                      "type": "integer",
                      "description": "The number of agents returned per page.",
                      "example": 10
                    },
                    "total_records": {
                      "type": "integer",
                      "description": "The total number of agents available across pages.",
                      "example": 100
                    },
                    "timezone": {
                      "type": "string",
                      "description": "The adherence timezone. The default is UTC.",
                      "example": "America/Los_Angeles"
                    },
                    "agents": {
                      "maxItems": 10,
                      "type": "array",
                      "description": "List of agents with adherence.",
                      "items": {
                        "required": [
                          "adherence_time_for_activity",
                          "display_name",
                          "schedule_activities",
                          "shifts",
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The agent's unique identifier.",
                            "example": "0c30da44bd68aae3b5714abb885a59"
                          },
                          "display_name": {
                            "type": "string",
                            "description": "The agent's name.",
                            "example": "Alex"
                          },
                          "shifts": {
                            "maxItems": 10,
                            "type": "array",
                            "description": "Shift details.",
                            "items": {
                              "required": [
                                "end_in_adherence",
                                "end_time",
                                "start_in_adherence",
                                "start_time"
                              ],
                              "type": "object",
                              "properties": {
                                "start_time": {
                                  "type": "string",
                                  "description": "The shift start time, in ISO 8601 format `yyyy-MM-dd'T'HH:mm:ss'Z'` or yyyy-MM-dd'T'HH:mm:ss'TZD'`.",
                                  "example": "2024-11-06T03:30:00.000Z"
                                },
                                "end_time": {
                                  "type": "string",
                                  "description": "The shift end time, in ISO 8601 format `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`.",
                                  "example": "2024-11-06T03:30:00.000Z"
                                },
                                "start_in_adherence": {
                                  "type": "integer",
                                  "description": "The number of seconds after shift start that the user started being in adherence.",
                                  "example": 0
                                },
                                "end_in_adherence": {
                                  "type": "integer",
                                  "description": "The number of seconds before shift end that the user started being in adherence.",
                                  "example": 0
                                }
                              }
                            }
                          },
                          "schedule_activities": {
                            "maxItems": 250,
                            "type": "array",
                            "description": "All activities in the schedule.",
                            "items": {
                              "required": [
                                "activity_id",
                                "activity_name",
                                "end_time",
                                "start_time"
                              ],
                              "type": "object",
                              "properties": {
                                "activity_id": {
                                  "type": "string",
                                  "description": "The activity's unique identifier.",
                                  "example": "0c30da44bd68aae3b5714abb885a59"
                                },
                                "activity_name": {
                                  "type": "string",
                                  "description": "The activity's name.",
                                  "example": "Chat"
                                },
                                "start_time": {
                                  "type": "integer",
                                  "description": "The activity's start time in timestamp.",
                                  "example": 1730896200000
                                },
                                "end_time": {
                                  "type": "integer",
                                  "description": "The activity's end time in timestamp.",
                                  "example": 1730896200000
                                }
                              },
                              "description": "Activity details"
                            }
                          },
                          "adherence_time_for_activity": {
                            "maxItems": 250,
                            "type": "array",
                            "description": "All unique activities in the schedule with adherence.",
                            "items": {
                              "required": [
                                "activity_id",
                                "activity_in_adherence",
                                "activity_name",
                                "activity_out_adherence"
                              ],
                              "type": "object",
                              "properties": {
                                "activity_id": {
                                  "type": "string",
                                  "description": "The activity's unique identifier.",
                                  "example": "0c30da44bd68aae3b5714abb885a59"
                                },
                                "activity_name": {
                                  "type": "string",
                                  "description": "The activity's name.",
                                  "example": "Chat"
                                },
                                "activity_in_adherence": {
                                  "type": "integer",
                                  "description": "Activity in adherence, in seconds.",
                                  "example": 0
                                },
                                "activity_out_adherence": {
                                  "type": "integer",
                                  "description": "Activity out of adherence, in seconds.",
                                  "example": 0
                                }
                              },
                              "description": "The activity's adherence details."
                            }
                          }
                        },
                        "description": "Agent and adherence details."
                      }
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `1902` <br>\n Invalid date parameter. <br>\n**Error Code:** `1903` <br>\n Invalid timezone. <br>\n**Error Code:** `1904` <br>\n Invalid page size. <br>\n"
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](/docs/api/rest/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "workforce_management:read:list_adherence_agents:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-granular-scopes": [
            "workforce_management:read:list_adherence_agents:admin"
          ]
        }
      }
    },
    "/workforce-management/reports/schedules/agents": {
      "get": {
        "tags": [
          "Reports"
        ],
        "summary": "List all schedule agents",
        "description": "List all the schedule agents for the particular date and timezone\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:list_schedule_agents:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "listScheduleAgents",
        "parameters": [
          {
            "name": "date",
            "in": "query",
            "description": "The date in 'yyyy-MM-dd' format.",
            "required": true,
            "schema": {
              "type": "string",
              "format": "date",
              "example": "2025-09-01"
            }
          },
          {
            "name": "user_ids",
            "in": "query",
            "description": "Return results only for the specified user IDs. Add multiple `user_ids` parameters to include more users. Example: `?user_ids=123&user_ids=456`.",
            "required": false,
            "schema": {
              "type": "array",
              "format": "string",
              "example": [
                "jA5csQv1W1oXuYZLspNIZzMOXqTD9r9Rje2"
              ]
            }
          },
          {
            "name": "timezone",
            "in": "query",
            "description": "The schedule's timezone. The default is UTC.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "America/Los_Angeles",
              "default": "null"
            }
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "Use the next page token to paginate through large result sets. A next page token returns whenever the set of available results exceeds the current page size. ",
            "required": false,
            "schema": {
              "type": "string",
              "example": "jA5csQv1W1oXuYZLspNIZzMOXqTD9r9Rje2",
              "default": "null"
            }
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of scheduled agents returned per page.",
            "required": false,
            "schema": {
              "maximum": 10,
              "minimum": 1,
              "type": "integer",
              "example": 10,
              "default": 10
            }
          },
          {
            "name": "division_id",
            "in": "query",
            "description": "The division ID.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "ya2kZo77RE6LFqaoKwWM0g"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "List of agents in the schedule for the date",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "agents",
                    "next_page_token",
                    "page_size",
                    "timezone",
                    "total_records"
                  ],
                  "type": "object",
                  "properties": {
                    "next_page_token": {
                      "type": "string",
                      "description": "Use the next page token to paginate through large result sets. A next page token returns whenever the set of available results exceeds the current page size. ",
                      "example": "jA5csQv1W1oXuYZLspNIZzMOXqTD9r9Rje2",
                      "default": "null"
                    },
                    "page_size": {
                      "maxLength": 10,
                      "minLength": 1,
                      "type": "integer",
                      "description": "The number of scheduled agents returned per page.",
                      "example": 10,
                      "default": 10
                    },
                    "total_records": {
                      "type": "integer",
                      "description": " The total number of all scheduled agents available across pages.",
                      "example": 100
                    },
                    "timezone": {
                      "type": "string",
                      "description": "The schedule's timezone. The default is UTC.",
                      "example": "America/Los_Angeles"
                    },
                    "agents": {
                      "maxItems": 250,
                      "type": "array",
                      "description": "List of agents in the schedule",
                      "items": {
                        "required": [
                          "activities",
                          "display_name",
                          "scheduling_group_name",
                          "total_paid_time",
                          "total_scheduled_time",
                          "user_email",
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The agent unique identifier",
                            "example": "0c30da44bd68aae3b5714abb885a59"
                          },
                          "display_name": {
                            "type": "string",
                            "description": "The agent's display name",
                            "example": "Alex"
                          },
                          "user_email": {
                            "type": "string",
                            "description": "The agent's email address",
                            "example": "alex@example.com"
                          },
                          "scheduling_group_name": {
                            "type": "string",
                            "description": "The name of the scheduling group",
                            "example": "Sales",
                            "default": "null"
                          },
                          "total_scheduled_time": {
                            "type": "integer",
                            "description": "The total schedule duration, in minutes.",
                            "example": 480,
                            "default": 0
                          },
                          "total_paid_time": {
                            "type": "integer",
                            "description": "The total duration of paid schedule activity in minutes",
                            "example": 340,
                            "default": 0
                          },
                          "activities": {
                            "maxItems": 250,
                            "type": "array",
                            "description": "List of activities",
                            "items": {
                              "required": [
                                "activity_name",
                                "activity_type",
                                "end_time",
                                "is_paid",
                                "schedule_activity_id",
                                "start_time"
                              ],
                              "type": "object",
                              "properties": {
                                "schedule_activity_id": {
                                  "type": "string",
                                  "description": "The activity unique identifier",
                                  "example": "cll20wknm01cwex0isvewiegl"
                                },
                                "start_time": {
                                  "type": "string",
                                  "description": "The schedule activity start time. In ISO 8601 format (yyyy-MM-dd'T'HH:mm:ss'Z' or yyyy-MM-dd'T'HH:mm:ss\u2019TZD\u2019).",
                                  "example": "2024-07-20T14:00:00-08:00"
                                },
                                "end_time": {
                                  "type": "string",
                                  "description": "The schedule activity end time. in ISO 8601 format (yyyy-MM-dd'T'HH:mm:ss'Z' or yyyy-MM-dd'T'HH:mm:ss\u2019TZD\u2019).",
                                  "example": "2024-07-20T14:00:00-08:00"
                                },
                                "activity_name": {
                                  "type": "string",
                                  "description": "The activity name identifier",
                                  "example": "SMS"
                                },
                                "activity_type": {
                                  "type": "string",
                                  "description": "Type of the activity",
                                  "example": "Productive"
                                },
                                "is_paid": {
                                  "type": "boolean",
                                  "description": "The is_paid indicates if the scheduled activity counts as paid time for calculating the agent's total paid duration.",
                                  "example": true
                                }
                              },
                              "description": "The activity details"
                            }
                          }
                        },
                        "description": "The schedule agent details"
                      }
                    }
                  },
                  "description": "Schedule agents list returned"
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `1902` <br>\n Invalid date parameter <br>\n**Error Code:** `1903` <br>\n Invalid timezone <br>\n**Error Code:** `1904` <br>\n Invalid page size <br>\n"
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          },
          "500": {
            "description": "**HTTP Status Code:** `500` <br>\n Internal Server Error  Internal server error \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "workforce_management:read:list_schedule_agents:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WorkforceAdmin:Edit"
          ],
          "x-granular-scopes": [
            "workforce_management:read:list_schedule_agents:admin"
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
              "workforce_management:read:admin": "workforce_management:read:admin",
              "workforce_management:read:forecasts:admin": "workforce_management:read:forecasts:admin",
              "workforce_management:write:admin": "workforce_management:write:admin",
              "workforce_management:write:queue_metrics:admin": "workforce_management:write:queue_metrics:admin",
              "workforce_management:read:queue_metrics:admin": "workforce_management:read:queue_metrics:admin",
              "workforce_management:write:organizational_groups:admin": "workforce_management:write:organizational_groups:admin",
              "workforce_management:read:organizational_groups:admin": "workforce_management:read:organizational_groups:admin",
              "workforce_management:update:organizational_groups:admin": "workforce_management:update:organizational_groups:admin",
              "workforce_management:delete:organizational_groups:admin": "workforce_management:delete:organizational_groups:admin",
              "workforce_management:read:list_adherence_agents:admin": "workforce_management:read:list_adherence_agents:admin",
              "workforce_management:read:list_schedule_agents:admin": "workforce_management:read:list_schedule_agents:admin"
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
      "workforce-management"
    ]
  },
  "downloadPath": "/api-hub/workforce-management/methods/endpoints.json",
  "apiOptions": {
    "events": false,
    "methods": true,
    "ma": false
  },
  "productMeta": {
    "specType": "oas",
    "slug": "/docs/api/rest/reference/workforce-management/methods",
    "title": "Workforce Management",
    "description": "",
    "params": {
      "productName": "Workforce Management"
    },
    "zoapEndpoint": "docJsonByFixVersion",
    "productDisplayTitle": "Workforce Management",
    "scalarSlug": "/docs/api/workforce-management/methods",
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "svgname": "workforceManagement"
  },
  "frontmatter": {
    "title": "Workforce Management APIs",
    "keywords": "",
    "description": "Workforce Management",
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
        "workforce-management"
      ]
    },
    "downloadPath": "/api-hub/workforce-management/methods/endpoints.json",
    "apiOptions": {
      "events": false,
      "methods": true,
      "ma": false
    },
    "productMeta": {
      "specType": "oas",
      "slug": "/docs/api/rest/reference/workforce-management/methods",
      "title": "Workforce Management",
      "description": "",
      "params": {
        "productName": "Workforce Management"
      },
      "zoapEndpoint": "docJsonByFixVersion",
      "productDisplayTitle": "Workforce Management",
      "scalarSlug": "/docs/api/workforce-management/methods",
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "svgname": "workforceManagement"
    },
    "spec": "{\"openapi\":\"3.0.0\",\"info\":{\"title\":\"Workforce Management\",\"description\":\"Workforce Management\",\"termsOfService\":\"https://zoom.us/docs/en-us/zoom_api_license_and_tou.html\",\"contact\":{\"name\":\"Zoom Developers\",\"url\":\"https://developer.zoom.us/\"},\"version\":\"2\"},\"externalDocs\":{\"description\":\"Find out more about Swagger\",\"url\":\"https://swagger.io\"},\"servers\":[{\"url\":\"https://api.zoom.us/v2\"}],\"paths\":{\"/workforce-management/forecasts\":{\"get\":{\"tags\":[\"Forecasts\"],\"summary\":\"List forecasts\",\"description\":\"List forecasts.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:forecasts:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\\n\\n**Not supported in Gov cluster** \",\"operationId\":\"Listforecasts\",\"parameters\":[{\"name\":\"name\",\"in\":\"query\",\"description\":\"The forecast name.\",\"required\":false,\"schema\":{\"maxLength\":36,\"type\":\"string\",\"example\":\"Weekday forecast\"},\"example\":\"Weekday forecast\"},{\"name\":\"scheduling_group_name\",\"in\":\"query\",\"description\":\"The scheduling group name.\",\"required\":false,\"schema\":{\"maxLength\":36,\"type\":\"string\",\"example\":\"Infra support\"},\"example\":\"Infra support\"},{\"name\":\"status\",\"in\":\"query\",\"description\":\"The forecast status.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"ready_step_1\",\"enum\":[\"ready_step_1\",\"processing\",\"ready_step_2\",\"completed\",\"failed\",\"applied\",\"generating\",\"generating_failed\",\"reforecasting\",\"reforecasting_failed\"]},\"example\":\"ready_step_1\"},{\"name\":\"type\",\"in\":\"query\",\"description\":\"The forecast type.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"SHORT_TERM\",\"enum\":[\"SHORT_TERM\",\"LONG_TERM\"]},\"example\":\"SHORT_TERM\"},{\"name\":\"next_page_token\",\"in\":\"query\",\"description\":\"The next page token from the previous page, if available. Not required for the first page.\",\"required\":false,\"schema\":{\"maxLength\":36,\"type\":\"string\",\"example\":\"cm2n7z2vl000e9cllp4ckoc45\"},\"example\":\"cm2n7z2vl000e9cllp4ckoc45\"},{\"name\":\"page_size\",\"in\":\"query\",\"description\":\"The number of items to be returned per page.\",\"required\":false,\"schema\":{\"maximum\":50,\"minimum\":1,\"type\":\"number\",\"example\":50,\"default\":10},\"example\":50},{\"name\":\"division_id\",\"in\":\"query\",\"description\":\"The division ID.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"ya2kZo77RE6LFqaoKwWM0g\"}}],\"responses\":{\"200\":{\"description\":\"Forecast list response.\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"forecasts\",\"next_page_token\",\"page_size\",\"total_records\"],\"type\":\"object\",\"properties\":{\"next_page_token\":{\"description\":\"Use the next page token to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size.\",\"anyOf\":[{\"type\":\"string\",\"example\":\"cm2n7z2vl000e9cllp4ckoc45\"},{\"type\":\"null\"}]},\"page_size\":{\"type\":\"integer\",\"description\":\"The number of items returned per page.\",\"example\":10},\"total_records\":{\"type\":\"integer\",\"description\":\"The total number of items available across all pages.\",\"example\":100},\"forecasts\":{\"type\":\"array\",\"description\":\"List of forecasts.\",\"items\":{\"required\":[\"create_time\",\"description\",\"end\",\"forecast_id\",\"forecast_profile_id\",\"interval_minutes\",\"modify_time\",\"name\",\"scheduling_groups\",\"start\",\"status\",\"type\"],\"type\":\"object\",\"properties\":{\"forecast_id\":{\"maxLength\":30,\"type\":\"string\",\"description\":\"The forecast ID.\",\"example\":\"cm2n7z2vl000e9cllp4ckoc45\"},\"name\":{\"maxLength\":36,\"type\":\"string\",\"description\":\"The forecast name.\",\"example\":\"Weekday forecast\"},\"description\":{\"type\":\"string\",\"description\":\"The forecast description.\",\"example\":\"Forecast for business days\"},\"type\":{\"type\":\"string\",\"description\":\"The forecast type.\",\"example\":\"SHORT_TERM\",\"enum\":[\"SHORT_TERM\",\"LONG_TERM\"]},\"status\":{\"type\":\"string\",\"description\":\"The forecast status.\",\"example\":\"ready_step_1\",\"enum\":[\"ready_step_1\",\"processing\",\"ready_step_2\",\"completed\",\"failed\",\"applied\",\"generating\",\"generating_failed\",\"reforecasting\",\"reforecasting_failed\"]},\"start\":{\"type\":\"string\",\"description\":\"The forecast start.\",\"format\":\"date-time\",\"example\":\"2025-01-01T00:00:00Z\"},\"end\":{\"type\":\"string\",\"description\":\"The forecast end.\",\"format\":\"date-time\",\"example\":\"2025-01-06T23:59:59Z\"},\"create_time\":{\"type\":\"string\",\"description\":\"The forecast request create time.\",\"format\":\"date-time\",\"example\":\"2024-12-16T04:21:23.207Z\"},\"modify_time\":{\"type\":\"string\",\"description\":\"The forecast request modify time.\",\"format\":\"date-time\",\"example\":\"2024-12-16T04:21:23.207Z\"},\"interval_minutes\":{\"maximum\":60,\"minimum\":15,\"type\":\"number\",\"description\":\"The forecast granularity length, in minutes.\",\"example\":15},\"forecast_profile_id\":{\"description\":\"The forecast profile ID.\",\"anyOf\":[{\"type\":\"string\",\"example\":\"cm2n7z2vl000e9cllp4ckoc45\"},{\"type\":\"null\"}]},\"scheduling_groups\":{\"type\":\"array\",\"description\":\"List of scheduling groups present in the forecast.\",\"items\":{\"required\":[\"asa\",\"color_code\",\"deferrable_sla\",\"deferrable_sla_target_time\",\"dynamic_shrinkage\",\"fixed_shrinkage\",\"name\",\"occupancy\",\"scheduling_group_id\",\"sla\",\"sla_target_time\"],\"type\":\"object\",\"properties\":{\"scheduling_group_id\":{\"maxLength\":30,\"type\":\"string\",\"description\":\"The scheduling group ID.\",\"example\":\"cm2n7z2vl000e9cllp4ckoc45\"},\"name\":{\"maxLength\":36,\"type\":\"string\",\"description\":\"The scheduling group name.\",\"example\":\"Infra support\"},\"color_code\":{\"description\":\"The RGB hexadecimal color code.\",\"anyOf\":[{\"type\":\"string\",\"example\":\"#c0106f\"},{\"type\":\"null\",\"description\":\"Null if color code is not present.\"}]},\"sla\":{\"maximum\":100,\"minimum\":-1,\"type\":\"number\",\"description\":\"Service level target goal percentage. `-1` if not set.\",\"example\":75,\"default\":-1},\"sla_target_time\":{\"minimum\":-1,\"type\":\"integer\",\"description\":\"Service level target time in seconds. `-1` if not set.\",\"example\":30,\"default\":-1},\"asa\":{\"minimum\":-1,\"type\":\"integer\",\"description\":\"Average speed of answer in seconds. `-1` if not set.\",\"example\":30,\"default\":-1},\"occupancy\":{\"maximum\":100,\"minimum\":0,\"type\":\"number\",\"description\":\"Occupancy percentage. `0` if not set.\",\"example\":85,\"default\":0},\"fixed_shrinkage\":{\"description\":\"Fixed shrinkage value to use for all days.\",\"anyOf\":[{\"maximum\":99,\"minimum\":0,\"type\":\"number\",\"description\":\"Shrinkage percentage.\",\"example\":20},{\"type\":\"null\",\"description\":\"Value will be `null` if `dynamic_shrinkage` is present.\"}]},\"dynamic_shrinkage\":{\"description\":\"Dynamic shrinkage value to use, based on configured days.\",\"anyOf\":[{\"required\":[\"default_shrinkage\",\"dynamic_config\"],\"type\":\"object\",\"properties\":{\"default_shrinkage\":{\"maximum\":99,\"minimum\":0,\"type\":\"number\",\"description\":\"Shrinkage percentage.\",\"example\":20},\"dynamic_config\":{\"type\":\"array\",\"description\":\"List of shrinkage values for required intervals.\",\"items\":{\"required\":[\"days\",\"end_time\",\"start_time\",\"value\"],\"type\":\"object\",\"properties\":{\"days\":{\"type\":\"array\",\"description\":\"List of days this configuration will apply for.\",\"items\":{\"type\":\"number\",\"description\":\"`0` - Sunday \\n`1` - Monday\\n`2` - Tuesday\\n`3` - Wednesday\\n`4` - Thursday\\n`5` - Friday\\n`6` - Saturday\",\"example\":1,\"enum\":[0,1,2,3,4,5,6]}},\"start_time\":{\"maximum\":1440,\"minimum\":0,\"type\":\"integer\",\"description\":\"The starting minute of a day to apply this shrinkage value.\",\"example\":600},\"end_time\":{\"maximum\":1440,\"minimum\":0,\"type\":\"integer\",\"description\":\"The ending minute of a day until when to apply this shrinkage value.\",\"example\":660},\"value\":{\"maximum\":99,\"minimum\":0,\"type\":\"number\",\"description\":\"Shrinkage percentage.\",\"example\":20}}}}}},{\"type\":\"null\",\"description\":\"Value will be `null` if `fixed_shrinkage` is present.\",\"example\":\"\"}]},\"deferrable_sla\":{\"maximum\":100,\"minimum\":-1,\"type\":\"number\",\"description\":\"Deferrable channels interactions' percentage. `-1` if not set.\",\"example\":75,\"default\":-1},\"deferrable_sla_target_time\":{\"minimum\":-1,\"type\":\"integer\",\"description\":\"Deferrable channels interactions' target time in hours. `-1` if not set.\",\"example\":30,\"default\":-1}}}}}}}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"401\":{\"description\":\"**HTTP Status Code:** `401` <br>\\n Unauthorized  \\n\\n \"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](/docs/api/rest/rate-limits/). \\n\\n \"},\"500\":{\"description\":\"**HTTP Status Code:** `500` <br>\\n Internal Server Error  \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WorkforceAdmin:Edit\"],\"x-macro-scopes\":[\"workforce_management:read:admin\"],\"x-granular-scopes\":[\"workforce_management:read:forecasts:admin\"]}}},\"/workforce-management/forecasts/{forecastId}/scheduling-groups/{schedulingGroupId}\":{\"get\":{\"tags\":[\"Forecasts\"],\"summary\":\"Get a forecast for a scheduling group\",\"description\":\"List forecasted values for a scheduling group within a forecast.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:forecasts:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\",\"operationId\":\"Getforecastofschedulinggroup\",\"parameters\":[{\"name\":\"forecastId\",\"in\":\"path\",\"description\":\"The forecast ID.\",\"required\":true,\"schema\":{\"maxLength\":30,\"type\":\"string\",\"example\":\"cm2n7z2vl000e9cllp4ckoc45\"},\"example\":\"cm2n7z2vl000e9cllp4ckoc45\"},{\"name\":\"schedulingGroupId\",\"in\":\"path\",\"description\":\"The scheduling group ID.\",\"required\":true,\"schema\":{\"maxLength\":30,\"type\":\"string\",\"example\":\"cm2n7z2vl000e9cllp4ckoc45\"},\"example\":\"cm2n7z2vl000e9cllp4ckoc45\"},{\"name\":\"division_id\",\"in\":\"query\",\"description\":\"The division ID.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"ya2kZo77RE6LFqaoKwWM0g\"}}],\"responses\":{\"200\":{\"description\":\"Forecast scheduling group response.\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"forecast_id\",\"forecast_intervals\",\"scheduling_group_id\"],\"type\":\"object\",\"properties\":{\"forecast_id\":{\"maxLength\":30,\"type\":\"string\",\"description\":\"The forecast ID.\",\"example\":\"cm2n7z2vl000e9cllp4ckoc45\"},\"scheduling_group_id\":{\"maxLength\":30,\"type\":\"string\",\"description\":\"The scheduling group ID.\",\"example\":\"cm2n7z2vl000e9cllp4ckoc45\"},\"forecast_intervals\":{\"maxItems\":1000,\"type\":\"array\",\"description\":\"List of forecasted values per interval.\",\"items\":{\"required\":[\"forecast\",\"timestamp\"],\"type\":\"object\",\"properties\":{\"timestamp\":{\"type\":\"string\",\"description\":\"The interval's starting timestamp.\",\"format\":\"date-time\",\"example\":\"2025-01-01T00:00:00Z\"},\"forecast\":{\"required\":[\"agents\"],\"type\":\"object\",\"properties\":{\"agents\":{\"type\":\"number\",\"description\":\"The forecasted staffing count.\",\"example\":5},\"voice\":{\"required\":[\"aht\",\"e_aht\",\"e_vol\",\"vol\"],\"type\":\"object\",\"properties\":{\"aht\":{\"type\":\"number\",\"description\":\"The forecasted average handle time, in seconds.\",\"example\":40},\"e_aht\":{\"type\":\"number\",\"description\":\"The edited average handle time, in seconds. `-1` if not edited.\",\"example\":38},\"vol\":{\"type\":\"number\",\"description\":\"The forecasted volume.\",\"example\":10},\"e_vol\":{\"type\":\"number\",\"description\":\"The edited volume. `-1` if not edited.\",\"example\":12}},\"description\":\"Interval forecast for all queues in scheduling group of channel category `voice`.\"},\"messaging\":{\"required\":[\"aht\",\"e_aht\",\"e_vol\",\"vol\"],\"type\":\"object\",\"properties\":{\"aht\":{\"type\":\"number\",\"description\":\"The forecasted average handle time, in seconds.\",\"example\":40},\"e_aht\":{\"type\":\"number\",\"description\":\"The edited average handle time, in seconds. `-1` if not edited.\",\"example\":38},\"vol\":{\"type\":\"number\",\"description\":\"The forecasted volume.\",\"example\":10},\"e_vol\":{\"type\":\"number\",\"description\":\"The edited volume. `-1` if not edited.\",\"example\":12}},\"description\":\"Interval forecast for all queues in scheduling group of channel category `messaging`.\"},\"deferrable\":{\"required\":[\"aht\",\"e_aht\",\"e_vol\",\"vol\"],\"type\":\"object\",\"properties\":{\"aht\":{\"type\":\"number\",\"description\":\"The forecasted average handle time, in seconds.\",\"example\":40},\"e_aht\":{\"type\":\"number\",\"description\":\"The edited average handle time, in seconds. `-1` if not edited.\",\"example\":38},\"vol\":{\"type\":\"number\",\"description\":\"The forecasted volume.\",\"example\":10},\"e_vol\":{\"type\":\"number\",\"description\":\"The edited volume. `-1` if not edited.\",\"example\":12}},\"description\":\"Interval forecast for all queues in scheduling group of channel category `deferrable`.\"}},\"description\":\"Forecasted values.\"},\"reforecast\":{\"required\":[\"agents\"],\"type\":\"object\",\"properties\":{\"agents\":{\"type\":\"number\",\"description\":\"The forecasted staffing count.\",\"example\":5},\"voice\":{\"required\":[\"aht\",\"e_aht\",\"e_vol\",\"vol\"],\"type\":\"object\",\"properties\":{\"aht\":{\"type\":\"number\",\"description\":\"The forecasted average handle time, in seconds.\",\"example\":40},\"e_aht\":{\"type\":\"number\",\"description\":\"The edited average handle time, in seconds. `-1` if not edited.\",\"example\":38},\"vol\":{\"type\":\"number\",\"description\":\"The forecasted volume.\",\"example\":10},\"e_vol\":{\"type\":\"number\",\"description\":\"The edited volume. `-1` if not edited.\",\"example\":12}},\"description\":\"Interval forecast for all queues in scheduling group of channel category `voice`.\"},\"messaging\":{\"required\":[\"aht\",\"e_aht\",\"e_vol\",\"vol\"],\"type\":\"object\",\"properties\":{\"aht\":{\"type\":\"number\",\"description\":\"The forecasted average handle time, in seconds.\",\"example\":40},\"e_aht\":{\"type\":\"number\",\"description\":\"The edited average handle time, in seconds. `-1` if not edited.\",\"example\":38},\"vol\":{\"type\":\"number\",\"description\":\"The forecasted volume.\",\"example\":10},\"e_vol\":{\"type\":\"number\",\"description\":\"The edited volume. `-1` if not edited.\",\"example\":12}},\"description\":\"Interval forecast for all queues in scheduling group of channel category `messaging`.\"},\"deferrable\":{\"required\":[\"aht\",\"e_aht\",\"e_vol\",\"vol\"],\"type\":\"object\",\"properties\":{\"aht\":{\"type\":\"number\",\"description\":\"The forecasted average handle time, in seconds.\",\"example\":40},\"e_aht\":{\"type\":\"number\",\"description\":\"The edited average handle time, in seconds. `-1` if not edited.\",\"example\":38},\"vol\":{\"type\":\"number\",\"description\":\"The forecasted volume.\",\"example\":10},\"e_vol\":{\"type\":\"number\",\"description\":\"The edited volume. `-1` if not edited.\",\"example\":12}},\"description\":\"Interval forecast for all queues in scheduling group of channel category `deferrable`.\"}},\"description\":\"Re-forecasted values.\"}}}}},\"description\":\"Forecast scheduling group response.\"}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"401\":{\"description\":\"**HTTP Status Code:** `401` <br>\\n Unauthorized  \\n\\n \"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n \"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WorkforceAdmin:Edit\"],\"x-macro-scopes\":[\"workforce_management:read:admin\"],\"x-granular-scopes\":[\"workforce_management:read:forecasts:admin\"]}}},\"/workforce-management/imports/historical-queue-metrics\":{\"post\":{\"tags\":[\"Imports\"],\"summary\":\"Upload historical queue metrics\",\"description\":\"Import historical queue metrics to Workforce Management (WFM).\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:write:queue_metrics:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `HEAVY`\\n\\n**Not supported in Gov cluster** \",\"operationId\":\"Uploadhistoricalqueuemetrics\",\"requestBody\":{\"content\":{\"multipart/form-data\":{\"schema\":{\"type\":\"object\",\"properties\":{\"file\":{\"type\":\"string\",\"description\":\"Attachment file. Only CSV format is supported. For format details, log in to WFM and visit **Forecasting -> Historical data -> Import historical data**.\",\"format\":\"binary\",\"example\":\"Vm0wd2QyUXlVWGxWV0d4V1YwZDRWMVl3WkRSV01WbDNXa1JTVjAxV2JETlhhMUpUVmpBeFYySkVUbGhoTVVwVVZtcEJlRll5U2tWVWJHaG9UVlZ3VlZadGNFSmxSbGw1VTJ0V1ZXSkhhRzlVVmxaM1ZsWmFkR05GU214U2JHdzFWVEowVjFaWFNraGhSemxWVm14YU0xWnNXbUZrUjA1R1UyMTRVMkpIZHpGV1ZFb3dWakZhV0ZOcmFHaFNlbXhXVm0xNFlVMHhXbk5YYlVaclVqQTFSMVV5TVRSVk1rcElaSHBHVjFaRmIzZFdha1poVjBaT2NtRkhhRk5sYlhoWFZtMHhORmxWTUhoWGJrNVlZbFZhY2xWcVFURlNNVlY1VFZSU1ZrMXJjRWxhU0hCSFZqRmFSbUl6WkZkaGExcG9WakJhVDJOdFJraGhSazVzWWxob1dGWnRNSGhPUm14V1RVaG9XR0pyTlZsWmJGWmhZMnhXY1ZGVVJsTk5WbFkxVkZaU1UxWnJNWEpqUld4aFUwaENTRlpxUm1GU2JVbDZXa1prYUdFeGNHOVdha0poVkRKT2RGSnJhR2hTYXpWeldXeG9iMWRHV25STlNHaFBVbTE0VjFSVmFHOVhSMHB5VGxac1dtSkdXbWhaTW5oWFkxWkdWVkpzVGs1V2JGa3hWa1phVTFVeFduSk5XRXBxVWxkNGFGVXdhRU5UUmxweFVtMUdVMkpWYkRaWGExcHJZVWRGZUdOSE9WZGhhMHBvVmtSS1QyUkdTbkpoUjJoVFlYcFdlbGRYZUc5aU1XUkhWMjVTVGxOSGFGQlZiVEUwVmpGU1ZtRkhPVmhTTUhCNVZHeGFjMWR0U2tkWGJXaGFUVzVvV0ZreFdrZFdWa3B6VkdzMVYySkdhM2hXYTFwaFZURlZlRmR1U2s1WFJYQnhWV3hrTkdGR1ZYZGhSVTVVVW14d2VGVnRNVWRWTWtwV1lrUmFXR0V4Y0hKWlZXUkdaVWRPU0U5V1pHaGhNSEJ2Vm10U1MxUXlVa2RUYmtwb1VqSm9WRmxZY0ZkbGJHUllaVWM1YVUxWFVraFdNalZUVkd4T1NHRkdRbFppVkVVd1ZtcEdVMVp0UmtoUFZtaFRUVWhDTlZaSGVHRmpNV1IwVTJ0a1dHSlhhR0ZVVnpWdlYwWnJlRmRyWkZkV2EzQjZWa2R6TVZZeVNrZGhNMmhYWVRGd2FGWlVSbFpsUm1SMVUyczFXRkpZUW5oV1YzaHJUa2RHUjFaWVpHaFNWVFZWVlcxNGQyVkdWblJOVldSV1RXdHdWMWxyVW1GWFIwVjRZMGhLV2xaWFVrZGFWV1JQVTBVNVYxcEhhR2hOU0VKMlZtMTBVMU14VVhsVmEyUlVZbXR3YjFWcVNtOVdSbXhaWTBaa2JHSkhVbGxhVldNMVlWVXhXRlZyYUZkTmFsWlVWa2Q0VDFOSFJrZFJiRnBwVmtWVmQxWnRjRWRWTVZwMFVtdG9VRlp0YUZSVVZXaERUbFphU0dWSFJtcE5WMUl3VlRKMGExZEhTbGhoUjBaVlZteHdNMWxWV25kU2JIQkhWR3hTVTJFelFqVldSM2hoVkRKR1YxTnVVbEJXUlRWWVZGYzFiMWRHWkZkWGJFcHNVbTFTZWxsVldsTmhWa3AxVVc1b1YxWXphSEpXVkVaelZqRldjMWRzYUdsV1ZuQlFWa1phWVdReVZrZFdXR3hyVWtWS1dGUldXbmRsVm10M1YyNWtXRkl3VmpSWk1GSlBWakpHY2xkcmVGZGhhM0JRVlRGa1MxSXhjRWRhUms1WFYwVktNbFp0TVRCVk1VMTRWVmhzVlZkSGVGWlpWRVozWWpGV2NWUnJUbGRTYlhoNVZtMDFhMVl4V25OalJFSmhWbGROTVZaWGMzaFhSbFoxWTBaa1RsWXlhREpXYWtKclV6RktjazVXWkZaaVJscFlWRlJLYjFOV1draGxSMFpYVFd4S1NWWlhkRzloTVVsNVlVaENWbUpIYUVOYVJFWmhZekZ3UlZWdGNFNVdNVWwzVmxSS01HRXhaRWhUYkdob1VqQmFWbFp1Y0Zka2JGbDNWMjVPVDJKRmNIcFhhMlIzWVZaT1JsTnJiRmROYmxKeVdYcEdWbVZXVG5WVGJGSnBWbFp3V1ZaR1l6RmlNV1JIV2taa1dHSkZjSE5WYlRGVFpXeHNWbGRzVG1oV2EzQXhWVmQ0ZDFZeVNsbFZiR2hoVWtWYVlWcFZaRXRUVms1ellVZG9UazFWY0ZkV2JHTjRUa2RSZVZaclpGZFhSM2h5VldwQ1lXTkdWblJsU0dSc1lrWnNOVnBWYUd0WFJrbDNWbXBTV2sxR1NsQldNakZHWlZaV2NscEhSbGROTW1oSlYxUkplRk14U1hsU2EyUm9VbXhLVkZac2FFTlRNVnAwVFZSQ1ZrMVZNVFJXYkdodlYwWmtTR0ZHYkZwaVdHaG9WbXBHYzJNeGNFaFBWbVJUWWxob1lWZFVRbUZoTWtwSVUydG9WbUpIZUdoV2JHUk9UVlpzVjFaWWFGaFNiRnA1V1ZWYWExUnRSbk5YYkZaWFlUSlJNRlpFUms5U01WcDFWbXhXYVZKdVFuWlhWbEpEWkRBMWMxZHJhR3RTTUZwWVdXeGFZVmRXV2xoa1J6bG9UVlZ3TUZsVldsTldWbHBZWVVWU1ZrMXVhR2haZWtaM1VsWldkR05GTlZkTlZXd3pWbXhTUzAxSFJYaGFSV2hVWWtkb2IxVnFRbUZXYkZwMFpVaGtUazFYZUZkV01qVnJWVEpLU1ZGcmFGZFNNMmhVVmxSS1JtVnNSbkZXYkdSVFRUSm9iMVpyVWt0U01WbDRWRzVXVm1KRlNsaFZiRkpYVjFaYVIxbDZSbWxOVjFKSVdXdG9SMVpIUlhoalNFNVdZbFJHVkZZeWVHdGpiRnBWVW14a1RsWnVRalpYVkVKaFZqRmtSMWR1VGxSaE1taG9WV3RXWVZsV2NGWmFSWFJVVm1zMWVsbFZaRzlVYXpGV1kwWmtWMkpIVGpSVWEyUlNaVlphY2xwR1pHbGlSWEJRVm0xNGExVXhXWGhWYkdoclUwZFNXRlJXWkRSbFZscFlUVlZrV0ZKcmJETldiWEJUVjJzeFNHRkZlRmROYm1ob1ZqQmFWMk5zY0VoU2JHUlhUVlZ3VWxac1VrTldhelZYVjFob2FsSlhhRzlWYWtwdlZERlZkMVpyZEU1aVJuQXdWRlpTUTFack1WWk5WRkpYVm0xb2VsWnRNVVpsVmxaelZteHdhVmRHU1hwWFYzQkhWakpPVjFSdVVsQldiVkpVV1d4b2IxbFdaRlZSYlVab1RXdHdTVlV5ZEc5V2JVcElaVWRvVjJKSFVrOVVWbHB6VmpGYVdXRkdhRk5pUm5BMVYxWldZV0V4VW5SU2JrNVlZa1phV0ZsVVNsSk5SbXhWVW1zNVUwMVdjREZXUjNodllWWktjMk5HYkZoV00xSjJWVlJHVDFZeFpISmFSbHBvVFd4S1dGWkdXbUZrTURGSFZtNVNhMUo2YkZkVmJYaHpUbFpzVm1GRlRsZGlWWEJKV1ZWV1UxWlhTbGxoU0VwWFlrWndTRnBGV2t0a1IwNUdUbFprVGxaWGQzcFdiWGhUVXpBeFNGSllhR0ZTVjJoVldXdGtiMkl4Vm5GUmJVWlhZa1p3TUZwVmFHdFVhekZZWlVaa1YwMXFWbkpXVkVwTFUxWkdjbUZHWkZOTk1taFZWbXhTUzFKdFZuTlNia1pYWWtkU1ZGUlVTbTlpTVZweVZXdDBVMDFXYkRSV1J6VlhWbTFLUmxOc2FGWmlSa3BZVmpGYVlWSXhiRFpTYld4T1ZqRktTVmRYZEc5U01WcElVbGhvYWxORk5WZFpiRkpIVmtaWmVXVkhkR3BpUm5CV1ZXMTRhMVJzV25WUmFscFlWa1ZLYUZacVJtdFNNV1IxVkd4U2FFMXRhRzlXVjNSWFdWWnNWMk5HV21GU1dGSlZWbTF6TVdWc2JGWmFSemxWWVhwR1Yxa3dXbXRXTWtwSVZHcFNWV0V5VWxOYVZscGhZMnh3UjFwSGJHbFNXRUpSVm0weE5HRXhWWGhYV0doV1lrZG9jbFV3WkZOWFJsSlhXa1JTYkZac2NGbFVWbFpyVjBaS2NtSkVUbGRpV0VKVVZqSnplRk5IUmtabFJtUk9ZbTFvYjFacVFtRldNazV6WTBWb1UySkhVbGhVVmxaM1ZXeGFjMVZyVGxkaGVsWllWakZvYjJGc1NsaGhSemxXWVd0d2RsWkVSbFprTVZweVpFVTFhVkp1UVhkV1JscFRVVEZhY2sxV1drNVdSa3BZVm0weGIyVnNXblJOVlZwc1ZteGFlbFp0ZUhkaFZtUkhVMWh3V0ZZelFraFdha3BQVmpGU2RWVnNRbGRpVmtwVlYxZDRiMkl4YkZkYVJsWlNZbFZhYjFSWGRIZFRWbFY1WkVjNVYySlZjRWxhVldSdlZqSktTRlZyT1ZWV2JIQjZWbXBHWVZkWFJrZGhSazVwVW01Qk1WWXhXbGRaVjBWNFZXNVNVMkpyTlZsWmExcGhWMFpzVlZOc1NrNVNiSEJHVlcxek5WWkdTbkpqU0d4WVlURndURmxXV2t0a1ZrWjFXa1prYVZkR1NsbFdiVEI0VlcxV1IxcElWbFppV0VKVVZtcEdTMWRHV2tkWGJVWnJUVVJHU0ZadE5WTmhNVW8yWWtaa1ZtSllhRE5VVlZwelZteGtjMVJzWkZkaVNFSmFWMVpXVjFVeFdsaFRhMlJxVWpKb1YxUlZaRk5YUmxWM1YydDBhMUl3TlVkVWJGcHJWR3hhV0dRemNGZGlXR2hVVlhwQmVGTkdTbGxoUjBaVFZqSm9WbGRYZEd0aU1rbDRWbTVHVW1KVldsaFphMXAzVFZacmQxZHRkR2hOYTNCSVdXdFNUMVl3TVhGV2EzaGFZVEZ3VEZwRldsZGtWMHBIWVVkb1RsZEZTalZXYlRGM1V6RktkRlp1VGxOaWExcFpXV3RrVTJJeFVsaGxSWEJPWWtad1NGWXlNVWRYUjBwWFVtcE9WVlpzY0hKV01HUkxWMGRXU1ZSc2NGZFNWbTk2Vm1wR1lXRXhaRWhXYTJoUVZtdHdUMVp0ZEhkVFZscHpXWHBHVkUxWFVrbFZNblJoWVd4T1JrNVdaRnBpUmtwSVZtdGFkMWRIVmtsVWJHUnBVakZLTmxaclkzaGlNVmw1VWxod1VsZEhhRmhXYlRGU1RVWndSVkpzY0d4V2F6VjZXV3RhWVdGV1NYbGhSemxYVmpOU1dGZFdXbk5XTWs1SFYyMUdVMWRHU2xaV2JURjZUVmRXYzJKR1dtRlNlbXhXV1Zod1IxWXhhM2RYYlhSWFRXdHdTVlpIY0ZOV1YwVjVWV3hPWVZac2NHaFpNbmgzVWpGd1NHSkhiRk5YUlVwU1ZtMHdlRTVIUlhoV1dHaFlWMGRvVjFsclpHOWpiRlYzV2taT1dGSnNTbGhYYTFKVFlrWmFjMk5HYkZWV2JGcHlWakJhUzJOdFNrVlViR1JwVjBWS1ZWWnFTbnBsUms1SVVtdGFiRkp0VWs5WmJURnZZakZhY1ZGdFJsZE5helY2V1RCV2IxVXlTa2hWYkdoVlZteGFNMVpYZUdGak1WWnlWR3hrYUdWc1dsbFdha1p2WWpKR2MxTnNhR2hTZW14WFdWZDBkMlJzV2tWU2JGcHJUVlp3ZVZwRlZURmhWa3AxVVZoa1YxSnNjRlJWVkVaaFkyc3hWMWRyTlZkU2EzQlpWbTB3ZUdJeVZuTlhibEpPVmxad2MxWnRlR0ZsYkZwMFpVaGthRlp0VWtoVk1XaDNWMFpaZWxGcmFGZGhhM0JRVm1wR1UyUldWbk5SYkdScFZtdHdWbFl4WkRSaU1rbDRZa1prWVZKc1dsTlpiRlpoWWpGU1dHUklaRlJXYkZZMVdrVmpOVll5U2xaalJXeGhWbGRTZGxacVNrdFRSbFp5VDFaV1YySklRalpXYlhCSFdWWmtXRkpyWkdoU2F6VndWVzAxUWsxV1dYaFhiR1JhVmpCV05GWlhOVk5WTWtweVRsWnNXbUV4Y0doV01GcHpWbFpPYzFOck5WTmlXR2hYVmxjeGVrNVdWWGROVm1ScVVrVmFXRlZxVG05a2JHeFhWMnQwYWsxck5VaFphMXB2VmpBd2VGTnFTbGRXYkVwSVZrUkdXbVZHY0VsVGJVWlRZa2hDZGxaR1pEUlRNV1JIVjJ0a1dHSlZXbkpVVlZKSFUwWmFkRTVXVG1oTlZXOHlXV3RhYTFadFNsbGhTSEJWWWtad2VsWnRNVWRTYkZKeldrZHNWMWRGU2t0V01WcFhWakZWZUZkc2FGUmlSM2h2VlRCV2QxZEdiSEpYYm1SVVVtNUNSMVl5ZERCaE1VbDNUbFZrVldKR2NISldSM2hoVjBkUmVtTkdaR2xYUjJoVlZsaHdRbVZHVGtkVGJHeG9VakJhVkZacVNtOVdiR1JZWkVkMGFVMXJiRFJXYlRWSFZXMUtWbGRzYUZwaE1YQXpWRlphY21ReFpIUmtSMmhPWVROQ1NWZFhkRk5VTVZsM1RWaEdWMkV6YUdGWlZFWjNWRVp3Umxkc1pHdFdNSEJJV1ZWa2IxUnNaRVpUYWxaWFlsUkdNMVY2Umt0a1JscDFWR3hPYVdFeGNGcFhWM1JyVlRKSmVGVnNhR3hTV0VKUFdXdGFkMU5HV1hsTlZXUm9VbFJHVjFsclVsTldNVW8yVm14Q1YwMUhVa3hXTVZwSFl6RldjMXBIYUdobGJGbDVWbTF3UjFsWFJYaGFSV2hYWVRKb1VWWnRkSGRVTVZwMFpFaGtWRlpzY0hwWFdIQkhWa1V4V0ZWcmJGWk5ibEpvV1ZkNFQxSnJOVmRhUm5CcFVtdHdTVlp0ZEdGa01XUklWbXRzVldKSFVuQlZha1pMVG14YWNsa3phR2xOVmxZMFZqSjBZVmRIUm5OalJtaFhZVEZhZVZwVlduTmpWa3B6WTBkNFUySldTalJXYWtvMFV6SkdTRk5yV21wU2JWSlpWbTE0UzFSR1dsVlNhemxUVFZad01WVnRlRTloVmxwWFkwWndWMkpVUVhoVlZFWlBWakpLUjFkc2FHaGxiWGhaVmtaYVlXUXhUa2RYV0d4c1VqTlNXRlJYZEhkVFZscElZMFpPVjFZd1ZqVldWM2hQV1ZaYWMyTkhhRnBOYm1nelZXcEdkMU5IU2toaVJrNVlVbFZ3VTFadE1IZGxSVFZJVWxob1YxZEhhRmxXTUdSdlZqRnNjbHBIT1dwaVJsWXpWMnRhVDFkR1NuTlRiR2hZWVRGS1ZGbHJXbUZqTWs1SFlrWldWMkpHY0hsV2FrSmhVekpPZEZOclZsVmlSMUp3VmpCV1MxWldXbkZUYm5Cc1VtczFTVlZ0ZEdGaVJrcDBWV3M1V21KVVJuWlpha1poWTFaR2RGSnNaRTVoZWxZMlYxUkNWMkl4VlhsVGEyaFdZa2RvVmxadGVHRk5NVnBZWlVkR2FrMVdXbmxXUjNocllVZFdjMWRzYkZkaGExcDJXV3BLUjJNeFRuTmhSMmhUWlcxNFdGZFdaREJrTWxKelYydFdVMkpHY0hKVVZscDNaVlp3UmxaVVJtaFdhM0F4VlZab2ExWXhTbk5qUmxKV1ZrVmFhRmt5YzNoV01XUnlUbFprVTJFelFscFdiVEIzWlVkSmVWWnVUbGhpYkVwUFZteGFkMk14V25SbFIwWnNZa1p3TUZwVmFHdGhSbHAwVld0b1ZrMVhhRE5XTUZwYVpXeEdjbUZHWkdoaE0wSlJWbTF3UjFVeVVsZFdiazVTWWtkU2NGWnRkSGRXYkZsNFdrUlNhVTFyYkRSWmExcHJXVlpLYzFkc1RscGlSbHBvV1dwR1UxZEhVa2xhUmxKWFlrWlpNVmRyVm1wT1ZsbDVVMnhhYWxKWGVHaFdiR1JUWkd4YWNWTnJkRlJTYkZwNVZERmFWMVl5U2tsUmFscFhZbGhDU0ZkV1dtdFhSa3B5V2tkb1UyRjZWbmRXVnpBeFVXc3hWMWRZYUZoaVIxSmhWbXBDVjA1R1dsaE9WazVXVFd0d2VWUnNXbk5YYlVWNFkwZG9WMDFHY0hwV2JGcFBZekZPY2s1V1RtbFRSVXBJVm14amVHVnJOVmRYYTJSVVltdHdjVlV3WkRSWFZteFZVbTVrVlZKdGVGaFdNblF3WVRGYWNsZHFSbGhoTVhCeVdWWmFhMUl4VG5OaVJtUnBWa1ZKTUZac1kzaFdNVWw0WTBWc1YySkdjSEJXTUZaTFZsWmFkR05GWkZSTlYxSklWakowYTFkSFNsWlhia3BWVmxaS1NGVXdXbUZrUlRWWFZHMW9UbFpYT0hsWFYzUmhZVEZhZEZOc2JHaFNSVFZXVm14YWQyVnNXblJsUjNScVRWWktlbGRyV210aFZrNUdVMjFHVjAxV2NGaFdha1pXWlVaa2MyRkdVbWhOYkVwNFZsZHdTMkl4V1hoVmJGcGhVbXMxV1ZWdGVGZE5NVmw1WkVSQ2FFMVZiRE5VYkZaclZsZEtSMk5JU2xkU00yaG9WakJrVW1WdFRrZGFSMnhZVWpKb1ZsWnNhSGRSYlZaSFZHdGtWR0pIZUc5VmFrSmhWa1phY1ZOdE9WZGlSMUpaV2tWa01HRlZNWEppUkZKWFlsUldTRlpYTVV0V2JHUnpZVVp3YUUxWVFYcFdSbHBoWTIxUmVGcElVbXRTTW1oUFdWUk9RMU5XWkZoa1JrNVZUVlpzTTFSV2FFZFdNa1Y2WVVkR1dsWkZXak5XUlZwM1VteGtjMXBIZEZkTlNFSkpWMVpXYTJJeFVuTmFSVnBVWVd4YVdGbHNVa2ROTVZZMlVtdDBhMUpzY0RGV1IzaFBZVmRGZUdOR2NGaFdNMUp5VmxSS1NtVkdWbk5oUjNoVFYwVktVRmRYZEdGa01VNUhWMWhzVGxaWFVtOVZiRkpIVmpGc2NsVnNUbGROVlc4eVZtMTRRMVl3TVVoVmEzaFhUVlp3U0ZsNlJsZGpNWEJJWVVkb2JHSkdiRFZXYlRGM1VqRnNXRkpZYUdGU1YyaFlXVlJLYjFWV1duUmtTR1JWVFZad01GcEZhR3RXUmxwelkwaHdXRmRJUWtoV2ExVjRWMFpXY21KR1drNWliRXB2VjFaa05GUXhTbkpPVm1SaFVtNUNjRlZ0ZEhkVFZscDBaRWRHYTAxWFVrbFdiWFJ6VmxkS1NGVnVRbFpoYTFwTVZHMTRZV05zYTNwaFIyeE9WbXhaTUZacVNqQlpWbVJJVW01T2FsSnRhRmhaVkVaaFpWWndWbGR1VG1wV2EzQjZXVEJrTkZVeVNsZFRhbEpYWVd0dk1GWkVSbHBsUms1elYyMXdVMlZ0ZUZsV2JYQlBWVEZrUjFwR1pHRlNWR3h4VkZaYWQwMVdWblJsUms1b1VsUkNORlV5ZUdGV01rcFZVbFJDVm1WclJqUlZha3BMVWpGa2MyRkhhR3hpV0dneVZteGtkMUl4YkZoV2JrNVlWMGQ0YzFWc1pGTlhSbEpZWkVaa1QxSnRkRE5YYTJNMVYwWktjMk5FUWxkV00yaFFWMVphWVdNeVRraGhSbkJPWW0xbmVsWlhjRWRrTVVsNVVtdGtWV0Y2Vms5WmJHUnFaVVphZEUxVVVtaE5SRVpJVmpGb2IxWldaRWhoUjJoV1lrZFNWRlpxUm5OamJIQkhWR3hvVjJKWVozZFdSbHBoVkRKR2NrMVdaR3BTUlVwV1ZtMTRTMUpHV2toak0yaFhUV3R3UmxadGVGZGhWa2w0VW1wT1YyRnJTbWhWZWtwUFpFWk9jbFpzVm1sU01VcDNWbXBDVjFNeFdsZFhia1pVWW01Q2MxVnRkSE5PUmxwSVRsVTVWbUpWY0VsV1Z6QTFWMnhhUms1VlVsWk5WbkJ5Vm14YVQyTldWblJoUlRWb1pXeFdNMVp0TUhoTlIwVjRZa1prVkZkSGVHOVZibkJ6Vm14YWNsWnJkRlZTYkhCWldsVmtSMkZyTVZkWGJteFlZVEpTZGxaSGVHRldWa3BWVVd4a1YxSldjREpYVjNCTFVqSk5lRlJ1VG1oU2JIQndWbXBLYjFkR1pGaGxSemxWWWxaYVNGWXhhR3RYUjBwSlVXNUtWVlpzY0ROYVZscDNVbXhhVlZKdGFGZGhNMEY0VmxaYWIyRXhXWGhUYms1cVVteEtWMVpyVm1GaFJtdDVZek5vVjAxWFVubFViRnByVlRKRmVsRnNjRmRpVkVJeldsVmtTbVZXV25WVWJHaHBZWHBXV2xkWGVHOVZNVnBYVm01R1UySlZXbFZWYlhoelRsWndWbUZIZEZkTlJFWlhXVEJhYjFkdFJuSk9WRTVYVFVad1lWcFhlRWRqYlVaSFdrZG9hRTB3U2xKV2JURjNVakZaZVZWc1pGVmlhelZZV1d4a05GWkdVbGRXYm1SWFlrWnNORmRyVWxOaFZURnlZa1JTV21FeGNISlhWbHBMWXpGa2MxUnNjR2xTYkhCdlYxZHdSMVV4V1hoalJXUlZZa1pLVkZsclpETk5WbHBJWlVaYVQxWXdXa2xWTW5SaFlXeEtSbE50YUZkaVIxSlVWVEJhWVdNeFpIUlNiR1JPWVhwV1NWWXlkR0ZXTVZsNVUydHNVbUZzY0ZoWmJHaERVMFprVjFkdGRGaFNhM0I1V1ZWa2QxVXdNWFZoUm14WFlXdEtXRlY2Ums5U01XUjFWVzE0VTAweFNsRldWM0JEVmpBMVIxZHNhRTlXYXpWWFZGZDBkMlZXVW5OWGJrNVlZbFZXTkZrd1pHOVdNREZIWTBod1YySkdjRkJaZWtaUFkyczFWMVJ0YkZoU01taFpWbTE0YW1WSFZuSk9WbVJYVjBkU1QxWnRNVk5qVmxaeVZtMUdXRkpzV25oV1J6QTFZVEpLU0dWR2FGZE5ibEYzV1ZkemVHTnJOVlpoUm5Cb1RWaENNbFp0Y0VKa01sWkhWRzVHVkdKSFVsaFphMVozVTFaa1YxVnJaRmhpVmxwSVdUQldjMVpIU2xoaFJsSlZWa1ZhVEZac1dtRlNNVnAwVW0xMFRsWXhTbGxXTW5SaFlqSktSMU5ZWkZoaVZWcFlXV3hvVG1WR1dYZFhiVVpyVWxSV1dGWkhNVzlVYkZwWVQwaHNXRll6VW1oWlZFWmhaRVpPYzJKSGFGUlRSVXBYVjFkMFlXUXlWbk5YYmxKT1ZsZFNWRmxyV2t0bGJHUnlXa2hPVjAxWFVrZFZNblF3VmpBeFYyTkdhRmRoYTFwWFdsVmFhMk15UmtkVmJXeFRUVEpvTWxac1VrTldNbEY0V2tWa2FWTkZjRmxaYlRGVFZsWldkRTVWVGxOTlZtdzFXa1ZTUTJGRk1WWmlSRTVWWWtaYWNsWnNaRXRTTWs1SlUyeGtVMDB5YUc5V2FrSnJWVzFXZEZSclpHRlNNbmhaVldwS2IxWnNXbk5oU0dSU1lsWmFTRlpIZEd0V1IwcElaVWM1Vm1GclNtaFdhMXBoWTFaT2RFOVdaR2xTTVVwYVYydFdhMDFIUmxaTldFcHBVa1pLV0Zsc1VsZFRSbHBZVFZWMFYySkhVbnBaVlZwM1lVVXhXVkZZY0ZkU2JIQm9XVEl4VW1WR2NFbFZiWFJUVFcxb1VGZFdVazlSTVU1WFdrWm9hMUpyTlZaVVZscHpUVEZTVjJGRlpGZE5hMVkyV1ZWa1IxZHNXa1pYYWs1WFVsWndNMVZ0ZUd0ak1YQklaRVprVGxORlNrcFdNVnBYWVRKSmVGVnVUbUZTVm5CVldWZDRZVmRHYkhOVmEyUk9UVlpaTWxWdGREQmhNVXB6WTBSR1YxSXphRkJaVm1SR1pVZE9SMk5HYUZkTk1VcDVWbXhTUzFOdFZrZFdiazVvVWpOb1ZGWXdWa3RWUm1SWVpFZEdhMDFWTlVoV01XaHpWR3hhV0ZWc2JGWmhhMG96VmpCYVYyUkhUa1pQVm1SWFlraENObGRVUW05VE1WbDNUVlZvVm1FemFGaFVWV1JUVjBaVmVGZHNUbXBOYXpWSVYydGFUMVl5U2xWaGVrcFhZbFJHTTFWcVJuTlhSa3BaWVVkR1UxWXlhRmxYVmxKTFlqRldWMWR1VW10VFIxSldWRlphZDJWc1dsaGtSMFpWWWtWd2Vsa3dhSGRYUjBWNFUyeFNXbFpXVmpSV01HUlhVakpHUjFkck5XbGlWMmcxVmpGYWEwNUdVWGhUYmtwUFZtMVNhRlZ0TlVOalJsWjBaRWhrVkZac2NEQmFSV1JIVjBkS1YxZHNiRmRpV0ZKMldWUkdTbVZzUm5WUmJGcHBWa1phUlZadGNFZFpWbVJHVFZaV1ZtSlhhRTlXYlhSM1RteGFjVk5VUmxkTlZsb3dWVzAxVDJGc1NuUmxSbWhhVmtWYWFGWkZXbk5XYkdSMFVtMXdWMkV6UVhkWGExWmhWREZTYzFkWWFGUmlWVnBaVm10V1MyRkdaRmRhUldSWVVtdHdXbGt3V2t0aFZrbDZZVVpzV0ZZemFIWldWRVpyVWpGd1IxcEhSbE5pVmtwNFZrWmFhMVV3TVZkV2JsSnNVbFJzYjFadGRITk9SbFY1VGxjNVYwMVZjSHBaTUdoTFZqRmFSbU5HYUZwbGEzQk1WV3BHYTJSR1NuTlZiRTVYVW14d1lWWnRkR0ZaVm14WVZWaGtUbFp0VWxoWmEyUnZZekZXYzFwR1RsaFNiRXBaV2tWYWExUXhTblJrUkU1WVlUSk5NVll3V2t0ak1rNUZVV3hrVjAweWFGRldNVnBoV1ZkTmVWUnJhR2hTYmtKUFdXMHhibVZzV2xoalJXUnJUVlZzTlZWdGRHdFdWMFkyVm1zNVdtSkhVbkZhVlZwaFpFVXhWVlZ0YUU1U1JWcEpWbXBHYjJJeFdsZGFSV2hvVWpKb1YxbHJaRk5XUmxweFVtNU9hbUpJUWtoV1IzaHZWakpLV1ZvelpGZFNiSEJvVmtSS1IyTnJNVmRhUjNCVFVsVndWMVp0TVRSVE1VcEhWMnRvYWsweWFGaFVWM1IzVW14YWRHVkdaR2hXYTJ3MFZXMXdUMVl5Um5KVGJXaFdaV3R3VUZwRldrOWpiRnB6Vm0xc1UwMVZjRE5XYlhCRFZqRk5lRmRzWkZoaWExcFZXVzAxUTJNeFZuUmpla1pVVm14YU1GcEZaRWRoVmtweVkwUkNWMVl6VFRGV01qRkxWbFpLZFZkc2NHaGhNWEJ2VjFSR1lWSXlVa2RUYms1aFVsUldiMXBYZUZwTmJGbDRWV3RPVjAxcmJEUldiR2h6VmtkRmVXRkdaRnBoTWxKMlZtMTRZV1JIVmtoU2F6Vk9WbTVDV2xkc1ZtdFNNa1p5VFZac1VtRXpRbGxaVkVaM1ZrWmFTR1ZIUmxkV2EzQldWVmQ0YTFZeFNsZGpSRXBZVjBoQ1NGWnRNVmRXTVU1ellrZHNVMDF0YUZWWGJHTjRUa1prUjFwR2FHdFNNRnBWVkZaa1UxTkdXWGhoUjNSWVVqQndlVlJzYUV0V2JVWnlWMnQ0Vm1GcldqTlZiWGhoVmxaS2RHRkdaR2hOTUVZMFZteGFZV0l5UlhoWFdHaFVZbXMxY1ZWdGVFdFdNVnB5Vm0xR2FGSnRkRFZaZWs1dlZqQXhXRlZ1YkZWaVJuQnlWbFJLUm1Wc1JuTmpSbVJwVmtWR05GZHJVa2RYYlZaSFdraEtZVkp0YUhCWlZFWjNWbXhrVjFWclpHdE5hekUwVmpJMVUySkdTWGRYYXpsWFlsaG9hRlJWV25kV2JIQkdXa2RvVTJKSVFqVlhWM1JUVlRGYWRGTnJhR2hUUlhCWVdWZDBkMVJHV25OWGEzUlhWbXRhZWxrd1pEUmhSVEIzVTJ4R1YwMXVhRmhaVkVaU1pVWk9kVlZzVG1oTmJFcFVWMWQwYTFVeVRrZFhibEpyVTBkU1dGbHJXbk5PUm1SeVZXdE9WMDFFUm5wVmJYQlRWMGRGZVdGR1FsZE5SMUpIV2xaa1IxTldVbk5YYldob1pXeGFSbFp0ZUZkWlZteFlWV3RvVjJFeWVGWlpXSEJ6Vmtac2NscEVUazVXYkhCSldsVmFZVlF4V1hkWGEyeFdUVzVTYUZsWGVFdGtSMVpJVW14a2FWSnVRWHBYYkdRMFdWZE9WMVJ1U2xoaVdHaFVXV3RvUTFsV1pGZFdiWFJUVFZaV00xUlZhSE5oUmtwSFkwWm9XbUpIYUVSVk1GcHJWakZrZEdSR2FGTmhNMEkxVmpKMGEySXhWWGxTYWxwWFltMVNXRlp1Y0VOTk1WSnpWbFJHVTAxWFVscFpWVnBoWVVVeFJWWnRhRmRpV0VKRVZtcEJNVkl4WkhOaFJUbFhWa2Q0V2xaWE1IaFZNVTVYWTBaYVdtVnNXbGhaYkZaWFRrWlplV05GT1ZkTlJFWklXVEJhVTFZeVJuSlhia3BoVW14d00xVXdaRWRTTVZKMFlrWk9hVmRHUlhoV2JURTBZVzFXUmsxVmFGWlhSM2hWV1ZSS1UxWldiSFJqZWtaWFZtMTRXVnBGWkVkWFJrcDBaVWh3V2xaWFRURldiRnBMWkVkV1IyRkdaRTVpYkVZelYxWmtlbVZIVG5KT1ZscG9VbTVDYjFSV2FFTmxWbHB6Vld0MFZFMVZjREJXUjNSaFlWWktkRlZzYkZaTlJscE1WV3RhWVZJeFpIUmtSbFpwVm14d1NWZFVRbGRqTVZwSVUyeG9hRkpzU2xaV2ExWjNaR3hhYzFkdFJsTk5WbkF3VlcweE1GUnRSWGhqUld4WFlXdHJlRlpVU2tkak1VNXpZVVphYVZKc2NGZFdiVEUwV1ZkV2MxVnVTbGhpVlZwWlZXeFNWMVl4VVhoWGJUbFdUVVJHU2xWWGVHOVdNa1p5WW5wQ1dsWkZXbWhhUlZwUFl6SktTR0ZHVGxOV1JscFlWbTB4TkZsWFVYbFNhMlJZVjBkU1dGWnJWbUZYUm14eVYydDBiR0pIVW5sV2JGSkhZVVpLVlZGcVRsWk5ibEYzVm1wQmVGWXlUa2RoUm1Sb1lURndXRmRzVm1GaE1sSlhWRzVLVDFadFVuQldiWFIzVGtaYWMxcElaRlJOYTJ3MFdXdGFhMVp0U2toVmJHeGFZbFJHVkZacVJsZGtSVEZWVVdzNVUySkhkekZYYkZaclRVZEdTRkpxV2xOaVIzaFlWbTV3VjJWc1duTmFSWFJUVFdzMVNsVXllR3RWTURCNVlVYzVWMkZyU25KV1Z6RlhaRVpTY2xwSFJsTk5ibWhhVmxkd1MySXhXbk5YYms1b1UwZFNVMWxzV21GVFJscElaRWQwVjFJd2NFbGFWV00xVm0xR2NsTnRhRmRoTVhCb1dURmFUMlJGT1ZkYVIyeFRUVlZ3WVZZeWRGZGhNREZJVW14YVRsZEZjSEJWYTFwTFYwWnNjMVZzWkZSU2JIQlpWRlpXTUZZeVNraFZibkJhVmxad00xbFZWWGhYVmtaWlkwWmtVMkpJUW05WFZFbDRWVzFXUjJORmJGWmlXRkpVVkZjeGIxVkdaRmRWYXpsU1RWVTFXRmRyV205aE1VcHlUbFphVlZac2NIcFVWRVpUVmpKR1JscEdaRTVoTVZreVYxWldhMUl4WkhOWGExcFlZV3MxV0ZWc1duZE5NVlowWlVkMGFrMVlRa2xhUlZwclZHeE9SbE5yY0ZkTlZuQllXVlJLU21WR1ZuVlViVVpUVm01Q1ZsZFdaRFJaVm1SSFlraE9WbUV5VWs5VVZscGFUV3hXZEdSRVFtaE5WWEI2V1RCU1IxWXhTWHBoUmtKYVZtMVNSMXBWWkU5U01rNUhZVVprVGsxVmNEVldiWGhUVXpGT2RGWnJaRmRpYXpWdlZXcE9iMVpHV1hkV2EzUldVbTFTV1ZwRlpFZGhNREZXVGxWb1YwMVhhSHBXUjNoaFYwWldjMkpHV21oaE1HOTZWbXBDWVZZeVVraFVhMXBRVm0xU1dGbHJhRU5PYkdSVlVXMDVWRTFXYkROVVZsWnZWbTFGZW1GR2FGVldWMUpVVmpCYVZtVkdaSE5VYlhCT1lYcFdTVll5ZEdGVU1WbDVVMnhXYUdWcmNGaFpiRkpIVFRGV2NWSnVUbGhTYTNCYVdWVmtSMVV4V1hsaFJteFlWbXhLVEZscVJsZFdNV1IxVlcxMFZGSXhTbGRXUmxwaFpEQXhSMk5GV21oU00xSllWVzE0ZDJWc2EzZGhTR1JZVW0xU1NsVlhkRzlXTWtWNFZtcFNWMDFHY0ZoWk1qRkxVakZrY2s1WGJHbFNXRUl5Vm1wR1lXRXdOVWRWV0doVVltdHdVRlp0TVZOaFJsWjBUbFZPYWxKc1dqQmFSV2hyVmtaYWMyTkVRbUZTVjFKSVdWUkJlRmRHVm5KaVJtUnBWMFpLTmxadGVHdFRNVTVIVm01V1VtSkdjRzlaVkVaM1ZGWmFTRTFVVWxwV01EVjZXVlJPZDJGR1NYbGxSVGxYWWxSRmVsUldXbUZYUjFaSVQxZDBUbFpzY0RaV01uUlhZakpHUjFOdVVtaFNiV2hoV1d0YVMxTkdXa1ZTYm1SWFlrZFNNVlpITVhkV01rVjZVV3BhV0ZaRlNuSlVhMXBXWlZaT2MxcEdVbWxXVm5CYVZtMTBWMU15VG5OaVNFWlRZbFZhVkZSV1ZURmxiR3h5VjJ0MGFGWnJiRFpaVlZwelYwWlplbUZJV2xkV1JWcHlWV3BHZDFJeGNFaFNiRTVwVm10d05GWnJXbUZoTVZWM1RWWmtWMkpzU25OVmJGSnpZakZhZEdSSVRrOVNiRlkxVkd4ak5WWXdNVlpqU0hCYVRVZG9URlpxUm1GU2JVNUhZVVp3YkdFelFrMVdWM0JIWVRKU1IxTnVUbFppUlRWWVZXMTRkMWxXV25Sa1IwWmFWbTE0V1ZaV2FHdFViRnAwVld4b1dtRXlVblpaZWtaWFl6RndSMVJzYUZOaVJYQmFWMnRXWVdFeFVYaFRibEpyVWtVMVdWbFVTazVOVmxsM1YydDBhazFyTlVaVlYzaHJWakF3ZVdGR1JsZE5WbkJvVjFaa1RtVldVbkpoUjJoVFltdEtVRmRYTUhoaWJWWnpWMnhvYWxKWFVuSlVWbFp6VGxaV2RHUkhkRmRXTUZreVZtMXpOVmR0U2toVmJrWmhWbFp3YUZwRlZYaFdWbFowWVVVMVUxSldjRXBXYlhCSFlqSkplRmRzWkdGU1YyaHZWV3BLYjFZeGJISmFSazVYVW14d2VGVnROVTloYlVwSlVXdG9WMUl6YUhaV2FrcExWMVp2ZWxwR1pGTmlTRUp2Vm10U1MxSXlUbkpOVm1SaFVqTkNWRlZzVm5kV01WcDBUVlJTVkUxVmNFaFdNblJ2WVdzd2VXRkdhRlZXTTFJelZXMTRZV1JIVmtkYVIyaFhZVEozTUZaWE1YZFVNVmw1VW01S1ZHSlhhRmRhVjNSM1ZVWndSbGRyT1dwaVJuQjZWbTB4YzFVd01WWmpSRTVYWWxoQ1JGWnFSazVsUm1SeldrWm9hV0V4Y0ZaWFZ6QXhVVEpOZUZadVVtcGxhMXBWVm0xNFlVMUdjRVphUldSb1ZtdHdlVmt3VWtOV01WbDZWRlJHVjAxcVJreFdiRnBMWkZaa2MyTkhhR2hOV0VKMlZqRlNSMWxXU1hsVmEyUlVZbXMxYUZWc1VsZGlNWEJZWlVad1RtSkdiRFJXVjNSUFZqQXhjbUpFVWxkaVIyaDZWbXRrUm1WWFJrZGFSbkJvVFd4S01sWnRNVFJaVjFKSVZtdG9VMkY2Vms5V2JUVkRWMnhrYzFadGRGTk5hMXA1VkZaYWMxVnRSWHBSYkd4YVZrVmFNMVpyV21GVFIxWklVbXhrVjJFelFsZFdWbVEwWVRGWmVGTnNaR3BTUlhCWlZqQm9RMU5HV25STlZtUlhUVlUxZWxsclpITlZNREYwWVVaV1dHSkhUalJVYTFwclVqRndSMkZIZEZOTlJuQldWa1phYTFVeFRrZFhXR2hvVWpOU1dWVnFRbmRsVmxKellVVjBWMkpWV25sV01uUTBWbTFHY2xkcVRsWmlXR2hvVm14YVMyUkhSa2RhUjJoT1RVVlpNRlp0ZUdGWlYwbDVVbGhvV0ZkSGFGVlpiWE14WTFaV2RHVkZkRmROVm5CNVZtMHhSMkZHU25Sa1JGWmFaV3MxZGxacVFYaFhSbFoxWWtaV2FWSnVRbmxXYkZKTFVtMVdjMUp1VG1wU2JWSndWbXRXU21Wc1pITldiWFJVWWxaYVdGWXlOVmRXVjBwSVlVaENXbGRJUWxoV01uaHJWMGRXU0U5V1drNVdia0paVm0wd01WSXhWWGxUYkZwWVlrWmFWMWxzYUc5bGJGSnpWMjVrVjJKVldrbGFSVnByVkd4S1JsZHNhRmhXUlVwb1dXcEdhMlJHU25WVGF6bFhVbFJXV1ZaR1ZtRmtNa1pIWWtSYVUySllVbkpWYWtKaFUwWmtjbHBIT1doU1ZFSXpWVEo0UzFZeVNsVlNhazVXWVd0YVYxcFdXbGRqTVhCSFdrZHNhR1ZzV2xWV01XUXdZVEZWZVZSclpGaGliRXB5VlRCa1UxZEdiSEpYYms1UFVtMVNlVlpYZEU5V01ERkZVbTV3Vm1KWVFsUldNR1JHWlZkR1IxRnNaRTVTTVVwSVYyeGFhMUl4VGxkVmJrNVlZbGRvYjFwWGVHRlNiRnAwWlVaa1dsWnRlRmxXVjNSdlZqSkZlV1ZHV2xwWFNFSjZWbXRhYzJOc2NFVlVhelZYWWtoQ1NsZHNWbUZaVmxGNFYxaGtXR0V5ZUZkVVZ6VlRZVVpzV0dWRmRHdFNNVnBKVlcxNGEyRldTblZSV0hCWVlrZFJNRmRXWkU5V01WSjFVMnhvYVZkSGFGVlhWM1J2VVRGc1YxcEdaRmRpV0ZKVVZGWmFWMDB4VWxaaFNFNW9VbXR3U0Zrd1dtOVhiVXBJWVVWU1ZrMUdjSEpXYWtaclpFZFNSMkZHVG1oTk1Fa3hWakZhVjJFeVVYaFdXR1JPVjBaYWIxVnRNVFJYUm14WVpFVjBXRkpzV1RKVmJYTTFWVEF4V0ZWcVJsZFdla1V3VmxSQmQyVkdUbk5TYkdSWFRUQktSVlp0Y0VKTlZrbDRXa2hPYUZKVWJGaFdhMlEwVjJ4YVdFMUlhRlpOVlRWWVdUQmFZVmR0Vm5OWGJHaGFZa1phV0ZScldscGxWMDVHVDFaa1RsSkZXa2xYVkVKdlpERlpkMDFWYUZaaVJrcFhXV3hTUjFaR1ZYZGFSVGxVVWpCd1NGWkhNWE5WTURCNFVsaG9WMkpVUlRCWlZ6RlhVbXN4Vmxkck5WZFdNbWhWVjFjeE5GTXhXa2RpUm1oc1UwZFNXVlZ0TVRSbFZtUnlXWHBXV0ZKc2NEQmFWVnBoVm0xS1ZWWnVTbHBXVm5CTVdrVmtWMUl5UmtkYVJUVnBZa1ZaZWxZeFdtdGxiVlpJVkc1S1QxWnNjRzlWYlRWRFlqRlNWMkZGVGs1aVJuQXdXVEJXUzJFd01YTlhiR3hXWWxoU2RsWlZXbUZrUmxaellVZEdWMDB4U2t4V1JsWmhXVmROZUdFemNHaFNiVkpQVm14a00wMUdaRlZSYkdScVRWWnNOVlV5ZEd0aGJFNUdVMnhvVlZaV2NHaFdSVnBoWXpGa2RGSnRkRk5OUkZaSVZqSjBZVlV4VW5OWFdHaFVZVEo0V0ZsWGRIWmtNV3hWVW0xMFZGSnJOWHBXTW5odllWZEZlbEZzWkZkaVdHaG9WWHBHVDFJeFdsbGFSbWhwVmxad2VWWlhlRk5XTVdSSFYydG9UbFo2YkZaWmExcDNWMVpSZDJGSVRsaFNiR3cyVmxjeGIxZHNXa1pYYldoaFVsWndVRmw2U2t0VFIwWkhWR3hPVjFKc2NGbFdiVEI0VGtac1dGVlliRmRpYXpWWldXMTRTMk14Vm5GVWJFNXFWbXhLV0ZZeWREQmlSMHBJVlc1c1YwMXFSWGRaVlZwaFkyczFWbHBHV2s1aGExcEpWbTF3UW1WR1dYbFVhMlJvVW0xU2IxbFVSbmRYUm1SelZtMUdhRTFyTVRWVmJYUnJWbTFLV0dGRk9WcGhNVm96Vkd0YVlXTXhjRWxhUmxKT1lUSjNNRlpxUm05aU1WcDBVMnRrYWxJeWFGZFpiR2hPWlVaWmVGZHNaR3RTTVZwSVYydFZNVlJzV25Sa2VrWllZVEZhVkZWVVNrZFNhekZYVjIxc1UxSlVWbGRYVjNSaFdWZE9jMWR1VG1GU1dGSlVWRmR6TVZOc1ZYbGxSM1JXVFVSQ05WbFZZelZXTWtaeVUyNUtWMVo2UmxoYVJWcFRZMnh3U0ZKc1RrNWliV2hoVm14a2QxTXlTWGhYYms1WVltczFiMVV3WkZOVk1XeHpWMjFHVkZKc2NFbGFSV1F3VmpGWmQyTkZiR0ZXVjJoRVZtMHhTMWRXVWxsaFJuQm9ZVEZ3VlZacVFtdFdNVXAwVld0a1lWSXllRlJVVmxaM1dWWmFjbGR0Um1sTmJFWTBWbGMxVDFkSFNuSk9XRVpXWWxSR2RsZFdXbk5XVms1MFQxWldVMkpXU2xsV2Frb3dUVVpSZVZOc1dtcFNWMmhoVm14YWQwMHhiRlpXV0doWVZtdGFXbGt3V205aFZrbDRVbGhrVjJGcmJEUldha1poWTJzMVYxcEhhRk5OTVVwVlYxZDBZV1F5VW5OYVNFNWhVa1ZLWVZadE1WTlhSbVJ5VjJ4a1ZtSlZjRlpaYTJoSFZsWmFjMk5JY0ZWV1JWcFVWbXBLUjFKc2NFZFhhelZvVFZacmVGWXhXbGRoTVVsNFYxaHNVMkpyY0ZCV01GcDNXVlphYzFkdVpHeFdia0pZVjJ0Vk5WUXlTa1pqU0hCYVZsWndjbFpIZUU5U2JFNXlZMFprVGxadVFsbFhWbEpMVTIxV1IxWnVWbFZpVlZwVVZtMDFRMVpzWkZoa1JtUnJUVmRTU0ZaWGVHdFhSMHAwVlcwNVZtSlVSbFJXTUZwYVpWZE9ObEp0YkZOaVNFSTJWbFJLZDFJeFdYaFhXSEJvVW0xb1lWWnNXbGRPUmxweFUyczVVMDFyTlVoWlZWcHJWR3N4Vm1ORVZsZGlXR2hVVlhwR1VtVkdUbGxoUm1ocFlrWndWVmRYZEZka01XUkhWbTVPVjJKdFVsZFZiWGgzVjFaU1ZtRkhPVmRoZWtaWlZsYzFkMWRzV2taT1dFcFhZV3R3VEZZeFdsZGpNa1pIVm14a1RrMVZiRFpXYlhCRFdWWlplVlZyYUZWaE1YQlJWbTB4VTFkR2JISmhSVTVQWWtad2VGVXhVa2RoTURGWVZXNXdWMDF1YUhKWlYzaExWMWRHU0ZKc1ZsZGxiRnBRVjJ4V1lWZHRVWGhYYkZaWFlrZFNUMWxZY0ZkVGJHUlhWbXhhVGxZd1dsaFdNV2h6WVVaS1dGVnNhRnBpUmtwSVZGUkdWMk5XU25WVWJHaFRZVE5DWVZkV1ZtRmlNV1JIVTFoc2FGTkhhRmhXYWs1dllVWmtWMWRzV214U01IQktWa2N4YjFVeVJqWldibWhYVm5wRk1GcEVRWGhTTWs1R1YyeG9hV0pXU2xkWFYzUlhaREpXYzJFelpHaFNlbXhZV1Zod1IwMVdWbGhsUms1WFRXdGFlVlZ0Y0ZOV01rWnlWMjFvV2sxV2NHaFpla1pyWTJzMVdHSkhiRk5YUlVWNFZtMXdSMWxYUlhkT1ZXUldWMGRvVlZsdGRIZFZSbHAwVFZaT2FGSnNXakJVYkZaUFlXeEtjMWRxUW1GU1YyaHlWbXRhWVdNeVRrVlJiVVpUVmpGS1NWWnRNWHBsUmxsNVUydFdWbUpIYUc5VVZ6RnZWVlphY1ZGdGRGUk5WMUpaVlcxMGEyRkdTbk5YYkdoWFlsaFNNMVpyV210ak1XUjBVbXh3VjJKV1NraFdSbHBoVmpGYWRGTnNhR3hTVkd4WVdXeG9iMWxXVWxkWGJVWllVakZhU1ZReFpEQlViRnB6WWpOa1YxWkZiekJYVmxwclUwWk9jbUZIYUZOaVYyaG9WMWQwWVZNeFRrZFhhMVpUWW0xU1dGbHJaRk5OVmxwSVRWaGtWMDFyVmpaVlYzQmhWMFphUmxOdVNsVmhNWEJYV2xaYVQyTXhjRWRWYld4WFZtNUNZVlp0TVRCV01VMTVWRzVPYVZORldsUlpiRlpoVmtaU1YxWlVSbXhpUjNoNVZqSjBNR0ZHV25KaVJGSldUVzVvTTFacVNrdFdWa3BWVVd4YWJHRXhjRlZXVjNCSFlUSk5lRmR1VGxWaVYyaHpXV3hvYjFac1duSlhiVVphVmpGR00xUldXbXRXTWtwelUyNU9WbUpHU25wWmFrWmhaRWRXU1ZwR2FGTmlXRkY2Vm1wSk1XRXhXWGROV0VwcVVteHdWMVp0TVc5a2JGcDBaVWRHVTJKSFVucFZWM2hyWVVkRmVsRnJNVmRXTTBKSVdWUktUbVZXU25KaFIyaFRUVEpvZDFacVFtdGlNV3hYVjI1U2ExSXdXbUZXYlRFMFYxWmFWMkZIT1ZkV01GWTFWbGQ0YzFZd01VaFZWRUphWld0YVdGa3hXbE5qTVZKellVWk9hVkpZUW1GV01uUlhZakpGZUZkclpGaGlhelZ4Vld4a05GbFdVbFpYYm1SV1VteHdlRlZ0ZUhkaE1ERlhZMGhvVjJKWWFISldha0YzWlZkR1IxSnNaRTVXYmtKdlZqRmFhMVJ0VmxkVmJrcG9VakpvVkZsdGRFdFZSbVJZVFVob2FXSldXbnBXTW5odllXeEtXRlZ1U2xWV2JGcDZWR3hhVjJSSFVraGtSbVJYVFVacmVWWnRNWHBPVm1SMFUydG9WbUpyV21oV2JGcDNZVVp3UmxaWWFGZFdiRm93V1ZWVk1XRldTbGRoTTJSWFlrZFJNRmxxU2twbFZrcHpXa1pTYVdKRmNIWldWekUwWkRGc1YxZHVTbGRpYlZKWVZXMTRjMDB4V1hsT1ZUbG9UVlZzTkZZeWRHdFpWa3BZVldwT1YxSXphR2haZWtaclYxZE9SMXBGTldsaVJYQjJWbTEwVTFNeFNYbFVia3BPVm0xU2IxVnFUbTlXVmxaeFZHMDVXRlpzY0VoWGEyaHJWakF4VjFkcmFGZE5ibEpZVmxSS1MxWnNaSFZUYkdScFYwWktiMWRzWkRSaE1rNXpXa2hXWVZKdFVrOVdiVEV6VFVaYVdFMVlaRlZOVm13MVZXMDFTMVpIU2xoaFJtUmhWak5TTTFaVlduZFNiR1J5VDFkd2FWSnVRalpXYlRCNFl6RlZkMDFXV21wU1JUVllXV3hvUTFKR1duRlNhelZzVW0xU2VsZHJXbTloVjBWNFkwWmFXRlo2UVhoVlZFcE9aVVphZFZWdGRGUlNWRlpSVmxjeE5HUXhUbGRWYms1WFZrWmFXRmxzVm5kV01XdDNWbTVPVjAxV2JEWldSbEpIVmpBeGRXRkhSbUZXTTJoWVdrVmtSMUl4WkhKT1ZrNXBZVEJ2TVZacVJtOWtNVWw1VWxoc1ZtRXlVbFZaYlhNeFlVWldjMVZzWkdsTldFSlpXbFZhZDFSdFNrZGlSRkpYVmpOU1VGWnFRWGhqYlVwRlZXeGthVkl5YURaV2JYUnJVbTFXV0ZKclpHRlNia0paVlRCV1MwNXNXblJOVkZKYVZqRktXVlpIZEdGWlZrbDZWVzVDVm1KWWFHRlVWbHByWXpGYWRGSnRkRTVoTTBKSlZsUkpNVlF4WkVoVGJsSnNVakJhVmxac1pGTlZNVkpXVjIxR1YxWnJOWGxYYTFVMVlWWktXVkZzWkZkV2JWRXdXVlJHV21WR1RuTmFSM0JUVWxoQ1dsWnRjRTlSTVZKSFZtNUdWR0Y2Vm5OVmJYaExUVlphU0UxWE9WWk5SRVl3V1ZWYVlWWXhXWHBoUm1oaFVrVmFjbFZxUm5kU01rWklaVVpPYUdWc1dsWldhMXByVGtaTmVWWnVUbGhYUjNoUVZteG9VMWRXVm5GUmJtUlhUVlpzTlZSc1ZtdFdNVnB6WWtSYVYxWjZSblpXYlRGSFkyeGtkR0ZHV2s1U01VcFpWbGR3UjFVeVRYaFhibEpwVW1zMWNGbFVRbHBOUmxwMFpVZEdXbFl3YkRWVmJHaHZWMFprU0dGR1ZscGlXRTE0V1RGYVYyUkhWa1prUm1ST1ZtNUNOVlp0TUhoU01rWkhVMjVTYTFKR1dtRmFWM014Wkd4YWNWRllhRmhTYkZveFdXdGFWMkZXU25WUmJteFhZa1pLVEZWdE1WZGpNVXAxVkcxd1UySldTbEJXYlhCSFVqQXdlRnBHWkZaaE0xSlZXV3hhWVZkR1duUk9WbVJYVmpCd1NWbFZXbk5XYlVaeVYydDRWMDF1YUhKV01HUlhVMFU1VjFkck5WZGlhMHBhVm0wd2QwMVdTWGhXYmxKVVlrWndXVmxyV21GV01XeHpWV3RhVGxKdVFsaFdNbmgzWWtaYVZWWnNiR0ZTVjFKMlZsUktTMU5XUmxsYVJtUnBVakZHTTFkWWNFZGhNazE0V2toT2FGSnNXbGhWYkZwM1YxWmtWMXBFUW10TlZrcElWakkxVDFkSFJYcFZiR2hYWWxoU2FGcFhlSEprTVhCRlZXMW9VMVpGU1hwV1ZFbzBVekZaZDAxV2FHaFNSVFZYV2xkMFlWVXhjRlpYYXpsclZteGFlbFpITVc5VWJVcEdZMFpXVjAxWFVYZFpWRVpXWlVaV1dXRkdXbWxTYmtKWlYxY3dlRlV4WkVkaVNFcFlZbXMxVUZWdE1WTmxWbHBZWkVVNWFGSnNjRlpWYlhoclZqSktXVkZzUWxkaVIxSk1WV3BHYTJSV1RuTmFSVFZUVFZWd1RsWXhVa3BsUmxGNFZHdGtWR0pyTlZoWmEyUlRWa1paZDFacmRGZGlSbkF3V2xWYWExZHNXblJsUm14WFZqTlNkbFpxU2tabFYxWkhWMnh3V0ZORlNqWldiWFJoWXpKT2MyTkZWbFZoZWxaWVdXdG9RMU5zWkhOV2JVWmFWbXhHTkZZeGFHdFhSMFY1WlVaa1ZWWjZWbFJaVlZwelYwZFdSbVJIZEZOTlZuQktWbGN4TkdFeVJsZFRibFpTVjBoQ1dWbHJaRkpOUm1SWFdrVTVWMDFXU2pGVk1uaHZZVlprU0dGR1ZsaFdNMUoyVlZSR2MxWXhjRWRhUm1ScFYwWktWVlpHV21GV01EQjRWV3hhYUZORk5XOVZiWFIzVTBacmQxZHRPVmhpVlhCSldWVldVMVpyTVVkalNFcFhZV3RhY2xwRlpGTlNNa1pIVkcxb1RrMUZhM2hXYlhCTFRVZEZlVk5ZYUZkWFIzaFVXVmQ0ZDFkR2JIUk5WazVZVW14d01Wa3dWa3RpUmtwelYyeHNXbVZyTlhaV2FrRjRWMFpXYzJOR2NGZFdia0l5VjFaYVlWTnRWa2RVYmxKb1VtMVNjRll3Vmt0U1ZscDBZMFZLVGxacmJEUlhhMmhQWVVaS1ZXSkdhRnBYU0VKWVZqQmFZVkl4Y0VWUmJYUk9Za1p3V1Zac1l6RlVNV1JJVTI1S1QxWnRlR0ZaYTFwaFYwWlNjbGR1U210TldFSkpWREZhYTFSc1NrWlhhMnhZVmtWS2RscEVSbHBsVms1eVdrZHdVMkV3Y0doWGJHUXdXVlpXUjJFemJHcGxhMXBZVkZaYVMwMUdiSEpYYlhSb1VsUkNNMVV5TVc5V01rcFZVV3BTWVZKRldrOWFSRUY0Vm14a2NrNVdaRmROYldoV1ZqRmplRTFIVVhoVldHaFlZbXhLVDFadGN6RlhSbHAwVFZSU1dGWnVRa2RYYTFaclYwWktjbU5JY0ZkTmJrSklWakp6ZUZJeFpIVlRiSEJYVWxoQ1RWWlhjRXRVTVVsNFdraE9XR0Y2Vm5CWmExWmFaV3hhZEdWR1RsSmlWbHBKVmtjMVMxUXhXbk5qUnpsV1lsUldSRmxxUmxOV01rWkdWRzE0VjJKSVFqWldiR1F3VFVaYVJrMVdXbXBTUlVwWVZXcE9VMkZHV2tobFIwWnJVakJhU0ZsVldsZFdNVXBXWTBVeFYxWkZXbWhYVm1SUFl6RmFkVk50UmxOaVYyaDNWbTB4TkdReFRsZGFSbFpTWWtkU2NWUldXbmRUVmxWNVpVYzVhRkpyY0hwVk1uaGhWakpGZUZkcVRtRlNNMmhvV1hwR2EyUkdTbk5VYXpWWFltdEdNMVpzWTNkbFJURkhWVzVTVTFkSGFIRlZiR1EwVjBac1dHUklTbXhTYkhCV1ZXMTRUMkZyTVZkVGJteGFWbGRTZGxaVVNrdFdNVTV6V2taa2FWWkZWWGRXTVZwclZqSk5lRlp1VmxSaVIyaHdWbXhhZDFkc1dsaGxSazVYVFZac05GWldhRzlYUjBwVllrWmtWVlpXU2toVk1uaFhWMGRXU0U5V1pFNWhNWEExVjFjMWQxRXhaSE5YYTFwWVlsZG9hRlZzV25kVlJtdzJVMnQwVkZJd1draFdSM2hQVmpGS2MyTkdiRmRpVkVJMFZHdGFjMWRHVmxsaVJtUnBZWHBXV2xkWE1UUlRNazE0Vm01R1ZXSkZOWEZXYlhoTFYwWlplV1JFUW1sU01IQkpWbGMxYzFZd01YRlNXR1JhVmxad2FGa3ljekZXYkhCSFYyczFhV0pYWkRaV2JURjNVVzFXUjFkWWFGVmlhM0JYV1d0b1ExZEdWbk5YYm1SVlVtMVNlbFp0TVVkaFJURnlUVlJTV2sxR1duSlpWVnBLWlZkV1NFOVhSbGROTW1odlZrZDBZVmxYVFhoalJXeGhVbXhLVDFsVVNqTk5SbVJZWkVkR2FFMXJXbnBWTWpWVFlVWk9SbU5HWkZwV00xSk1XVlZhWVZKV1NuTlViR1JPVmpGSmQxWlhNSGhPUjBaWFUyNVNhMlZyV2xsWmExcGhZMnhTVmxwRlpGTmhla1pZVmpJeGMxVXhTbFZXYkdSWVZqTlNhRmRXWkU5ak1XUjFWRzFHVTFkR1NsVldSbVI2VFZaT2MxWlliR3hTTTFKWFZGVlNRMDVXYkZaWmVsWlhUV3RhZVZVeU5VdFdNVm8yVW14b1lWSldjRlJaTW5oM1UwWktjMVJyTldsaVYyaG9WbTE0YTJReFRYbFRXR2hZWW1zMVdWbHRjekZpTVZWM1drWk9WazFYZUhwV01qRkhWa1pLYzJORmJHRlNWMUYzVm1wR1NtUXlUa1ppUjBaWFZqQXdlRlp0TUhoU01rNXpWRzVTYUZKdFVtOVVWbWhEWWpGa1ZWSnRSbFZOYkVwSFZERmFhMWxXU1hsbFJsSlZWbXhhTTFkV1dscGxWMUpIV2tkb1RsSkZXa3BXVnpFMFdWWlNjMXBGV21wVFJVcFhXVlJHZDFSR1dYZFhiazVxVFZaYWVsZHJXbE5WTWtwSlVXeHdWMUpzY0ZoVVZWcGFaVlpPY21GR1dtbGlhMHBvVm0xNFlXUXdNSGhpUmxwWFYwZG9XVlp0ZEdGWGJHdDNWMjEwVmsxcmNFaFpNR00xVjBkRmVGTnJhRmhXYlZKVVZXcEdUMlJXVG5SaFJrNU9UVlZ3VmxZeFpEQlpWMUY1Vlc1T1lWTkZOV2hWYTFaTFdWWmFkRTFXVGxoU2JWSjVWako0YTFadFNsWmpSbHBhWVRKb1NGWnNXbUZqTWs1SlkwWldWMUpXY0ZWV2JYQkxVakZLYzJORlpGaGhlbFp2VkZaYWNtVkdXWGhYYkdSb1RWVnNORll4YUhOVk1rcHlZMFpDVjAxR2NETlhWbHBoWXpGa2RWcEhlRmRpYTBwWlZtMTRiMkl4V1hsU2FscHBVbTFvV0ZWcVRtOWpiR3hXVmxob1dGSlViRlpXUjNocllWWkpkMDVFUWxkU00xSm9WbGN4VjFJeGNFZGlSMFpUVmtaYVdWZFhkRzlSTVdSelYyNUdVbUpVYkZOVVZtUlRVMFpzY21GRk9WZGlWVlkxVmxkek5WWldXbk5qUjBaVlZqTm9XRnBGWkU5T2JFcDBZa1pPYVZORlNUSldiRkpMWkRGSmVGZFlhRlJYU0VKdlZXMHhiMWxXV25STlZrNVRUVmhDV1ZwVlZtdGhNVnAwVlc1d1dHRXhjSEpaYTJSR1pESkZlbHBHWkdsWFIyaFZWbGR3UjJNeFdYaFViR3hvVW0xU2NGbFljRmRYVm1SWVpFZEdWRTFyTlVoV01qVkxXVlpLUjFOdVNsVldiSEF6VlRCYVdtVlZNVlphUjJoT1VrVmFObFpxU2pSaU1WbDNUVmhXYUZOR2NHRldhMVpoVkVaV2RHVkZPV3BOV0VKSVZtMTRhMVl5UlhwUmEzQlhZbGhDUkZkV1pFNWxWbEoxVkd4a2FXRjZWbGRYVjNSclZURlJlRmR1VGxwbGJFcHpWbTE0ZDAxR1VsZFZiRTVvVmxSR1dGbHJVbGRXYXpGMVlVWm9WMVpGY0VkYVZ6RkhVakZ3UjJOSGJGTk5iV2Q1Vm0xNFYxbFhUWGRPV0U1VlltdHdhRlZ0Y3pGVU1WcHhVbXR3YTAxWFVsbGFSV1JIVmxVeGNsZHJaRlZXVjJoeVZtdGtTMVpzWkhOaFJuQnBVbXR3U1ZaR1VrZFdNazVYVTI1V1ZXSkhVazlaVkU1RFUxWlplV1JIUmxaTlYxSklWVEZvYzJGR1RraFZiR2hYWVdzMVZGbFZXbmRTVmtwMFVteE9UbUV6UWt0V2EyTjRVakZWZUZOdVNtcFNiVkpZVlcxNGQyRkdjRVZUYkdSVFRWWndNVlpYTVRCVk1VcFdZMFpzVjFaNlJUQmFSRUY0VWpGa2MyRkhlRlJTVkZab1ZtMHhNR1F4WkZkWGJHaHJVbnBzV0ZSWGRHRmxWbFowWTBaT1YwMXJXbmxXTWpWTFdWWktkRlZzUWxaaGEzQklWVEJrUjFKck5WZGFSMnhvVFVacmVGWnRNWGRTTVd4WFYxaG9hVkp0VWxaV01HUTBWbXhaZDFwSE9WZGlSM2d3V1ROd1IxZEdXblJsU0d4WVlURndVRmxYZUV0ak1rNUpZa1prYVZaRldrbFdiWEJIV1ZkTmVWUnJWbEppVlZwVVZtcEdTMU5XWkZkVmEwcHNVbTFTU0ZadE5VdGhWa3AwVlc1Q1YwMUdjRXhVYlhoaFpFZFdTR1JHVWs1V1ZGWkpWbTB3TVZZeFZuUlRhMlJxVW5wc1lWWnJWbUZOTVZwSFYyMUdWRkpVUmxaV1YzaDNWakF4UlZacVdsaFdiRXBRV2tSS1IxSXhUbGxpUjNoVFZrZDRXVmRYZUc5aU1XeFhWV3hhV0dKVlduRldiWE14WlVaV1dHVklaRmROYTNCWVZUSjBkMVl4V2paUldHaFhVa1Z3U0ZsNlJrOWtWbFowWVVaT2FWWXlhRzlXYlRFd1ZqRnNXRkpyWkdwU2JWSlpXVzB4VTFkR2JISmFSazVPVm14d1NWUldhR3RXTURGeVZtcGFWbFl6UW1oV01GcGhVbXhPZFZOc1ZtaE5XRUpZVjJ4YVlWUXlUWGhqUldSV1lsZDRXRlp0TlVOWlZscDBUVlJDVjAxVk1UVlZiWFJyV1ZaS2RGVnRhRlppUjFKVVZqQmFVMVl4V2xWU2JHaHBVak5vTlZZeWRHcE9WbHB5VFZoS2FsTkhlRmRVVnpWdlpXeFpkMWRzY0d4aVZWcElWbGQ0WVdGV1pFWk9SRUpYVmpOQ1RGWkVSa3RqTVdSMVZHeFdhVll6YUhaWGJHUTBXVlprYzFkdVNsaGlXRkp6V1Zod1IxZEdXa2hrUjNSWVVtMVNTVnBWYUU5V2JVcDFVVzV3V21GcldtaGFSVlY0VmpGU2MxVnNUbWxTYkd0NFZtdGtOR0l4VVhoWFdHUk9WMFZ3Y1ZWdGVIZFdNV3h5V2tjNWFtSkhVbGhXTW5Rd1ZUSktWazVWYUZwV1ZuQXpXV3RrUm1WSFRrbGlSbWhvVFZoQ2IxWnRjRXRVTWs1eVRsWmtZVkl5YUZSWldIQlhWMVphV0UxVVVtdE5WM2hZVmpJMVUyRXhTWGRYYlVaWFlsaG9lbFJzV21GU01rWklaRVpXVGxZeFNraFdSM2h2VVRGYWRGTnNiR2hTUlRWWVdWZDBZV1ZzYTNsbFNHUlRUVlpLZWxsVldrOWhWa3BaVVd4c1YySllhSEpVVldSR1pVWndSbHBHYUdsaGVsWjRWbGN3ZUdJeGJGZFhiazVoVW10d2NsUldXbk5OTVZsNVpFUkNhR0Y2Um5sVWJGWmhWbXhaZW1GSVNsZGhhMFl6V2xaa1IxSXhTbk5hUlRWWFltdEtkbFpxUm1GaE1WSjBWbXhrVm1KcmNHOVZha3B2Vmtac2RHUkZkRlpTYlZKWlZGVm9iMkZYU2xkWGJuQlhUVzVvZGxsWGVFdFhSbFoxVTJ4V1YySkdWalJXVkVaaFZtMVdXRlZyYUZCV01taFlXVlJPUTA1V1draGxSMFpxVFZad01GVXlkR3RYUjBwMFpVZG9WMkpHY0ROWlZWcDNVbXhhV1dGR1pGZGlSbTkzVjFaV1ZrMVdWWGhUYkdScVVsaENXRmxzYUVOVFJteHhVMnR3YkZKdFVucFhhMlJIVlRGWmVXRkljRmRXZWtVd1YxWmFhMUl4Vm5OWGJVWlRZWHBXZVZkWGRHdE5SVEI0VjFoc2JGSXpVbTlVVmxaM1ZteFdkR1ZIUmxkTlZtOHlWVzF3VDFZeVJuSmpSWGhXWVd0d1VGVnNXbXRqTVhCR1RsZHNhVkpZUWpKV2JURXdWVEZOZUZOWWJGVmlhM0JQVm0weFUyRkdWbkZVYTA1WFVteGFlbFp0Tld0aFZURlhZMFJDWVZaWFVraFdNR1JMVmpGS2NWVnNjR2hOYldoTlZqRmFZVk14WkZkV2JrNVdZa1phV0ZSVVFYaE9SbHBZVFZSU1YwMVhVa2xXYlhScldWWktkRlZyT1ZkaVZGWkVWR3hhYTFaV1JuUlNiWEJPVm0xM01WWlhNREZUTVZwSVUyeG9hRkp0VWxaV2FrNURVMFphVlZKdVpGZE5WMUo2VmtjeGQyRldTWGhUV0dSWVZteGFhRlpFU2tkWFJrcFpXa2R3VTJFeGNGbFhWM1JyVkRBd2VGVnNaRmhoTVhCeVZGVlNSMWRzV2tobFJ6bG9WakJXTkZrd1dtRlhSbG8yVVZob1ZsWkZjRTlhVlZwaFpGWldkR05IZUdoTldFSlhWbTB3ZUU1R2JGWk5WV1JxVWxad2FGVnNaRk5pTVZwMFpFaGtiRlp0ZEROV1YzUlBWakF4VjJKRVdsZFNiRXBJVm1wQmVHTldTbkpsUm1ScFVtNUNUVmRyV21GaE1rMTVWR3RvYUZJeWFFOVdNRlpMVFd4YWNWSnNUbE5OVjNoWVZURm9jMVp0UlhsVmJGWmFZbGhOZUZsVldsZGpNVlp6V2tkc1RsZEZTbHBYYkZaaFpERlNkRk5yWkdsU1JrcFlWbXBPVTFWR1duRlJXR2hxWWxWd1JsWlhlR3RoVmtsNVlVUmFWMkpZVW5KVmJURlhWMFpLY2xwSFJsTmlhMHBhVmxSQ1YxTXdOWE5YYmxKT1ZrWktZVlpxUmt0WFZscElaRWRHV2xacmJEWldWelZEVm0xS1dXRkhSbFZXYkhCeVdYcEdZV1JHU25Sa1JUVm9UVEJLU2xac1VrcE5WbEY0V2tWb1ZHSnNTbk5WYWtKaFZteGFjMXBHVGxaTlZrcFhWakl4UjFZeFduSk9XR3hZWVRGVk1WbFdXa3RYVmtaeldrWmtWMkpXUlhkWFZsSkhWMjFXUjFSdVRtRlNNMEpVVld4b1ExZHNXblJqUlRsU1RWWktTRlp0TlVkV1IwWTJWbTVPVm1KWWFFeFVWbHBoVTBVeFdFOVdXazVoTTBKTFZsWmpNV0V4V1hsVGJHeFNZVE5vWVZsWGRHRmpiRlp4VW01a1UwMXJXa2xhVlZwUFlrZEtXV0ZHYUZkTlYxRXdWMVphYzFZeFVuSmFSbWhZVWpKb2RsWlhjRTlWTVdSWFZtNUdVbUp0VWxWWmExcGhWMVpTYzFacVFsZE5WbkI2V1RCb2QxZHJNVWRUYkVKWVZteHdhRmw2U2s5U1ZsSnpWMjFzV0ZJeWFIWldiVEUwV1ZaTmVWVnJhRk5oTWxKb1ZXcEtiMVF4VWxaV2JIQk9VbXh3TUZwVlpFZFhiRmwzWWtSV1ZtSkhhSFpYVmxwTFYwWldjMkpHY0ZkU1ZtOTZWbTE0WVdNeVRuTmpSV2hRVm01Q1dGbHJhRU5PVmxwMFRVaG9VMDFyV2pCVk1uUnZZVEpXY21OR1pGcGlSa3BIV2tSR1dtUXhaSFJTYlhSWFlURndOVmRXVm1GaU1rWkdUVmhPVkdGcmNGbFdiWGgzWVVaV05sSnRkRmROVlhBeFZrY3hkMVV5U2xkVGJFSlhZbGhDU0ZWcVFYZGxSbkJIV2taYWFFMHhTbFpYVjNoaFdWWmtSMWR1VWs5V2JWSllWbXBDZDFOV2JGWlhiazVYWWxWYWVWWXlkRFJXTURGMVlVaEtWMDFIVWxoVk1GVTFWakZrY2s1V1pHbGhNSEJoVm0wd2QyUXdNVmRUV0doWFlteEtWVmxyV25kWFZteHlWMjFHVjAxWGVIcFpWV00xWWtkS1IxTnVjRmROYWxaeVZtcEtTMVpzWkhKaVIwWlRWakZLTWxac1VrdFRNVTVYVW14c1lWSnRVbkJWYWtaSFRURmtXV05GWkdoTmF6RTBWMnRvVDJGR1NuTlhiR2hXWWtad00xWXhXbXRXTVhCRlVXMTBUbFpyY0VsV2Fra3hWREZrU0ZKWWNGWmlSMmhZV1d4b1UyRkdjRmhsU0VwclRWaENSbFpYTVc5Vk1WcEdVMWh3VjFaRmJ6QlZla1phWkRBeFYyRkhhRk5TVkZaWVYxWmtNR1F4VlhoalJscFlZa1UxY1ZSV2FFTlRSbHAwVFZoT2FGWnJjRnBWVjNCWFZqSktTRlJxVW1GV00wNDBWbXBHVjFkWFJrZFhiV3hUWW10S1dsWXhZM2RsUmxWNFdrVmthVk5GY0ZsWmJYTXhWVVpXZEUxV1RteFdiSEJaV2tWVk5WZEdTWGRqUldoYVlURktWRlp0TVV0WFZrWnpVV3hrYVZJeFNrMVdWM0JIWVRKU1YxUnVUbUZTVkZaVVZGWldkMVJzV25OVmEwNW9UVlZzTkZVeU5VdFZSbHAwVlcxb1ZtRnJjSFpXUkVaaFpFVTFWbFJzVm1sU2JrRjNWMnRXWVZReFZYZE5WbVJwVWtaYVdGWnFUbE5rYkZweFUydGFiRll4V2toV01uaDNWMFpPUmxOcVdsZGlXRkpvV1hwS1QyTXlUa1pXYkZab1pXeGFXVmRYZEc5Uk1VNXpXa2hPVjJKVldtRldiWGhIVGtaWmVHRkhkRmhTTUZZMVdsVmFhMVpXV2paV2JGSmFWbFp3ZWxZd1pGZFNiVkpJWkVaT1RsWnVRa3RXYkdONFRrZEZlVkpzWkZSaWJGcFRWakJvUTFkV2JGVlRiRXBPVW0xU1dGZHJWVFZXTURGWFUycEdWazFxUmtoV1ZFRjNaREZPZFZkc2FHbFNNVVYzVmpGYVlXRXhXWGhhU0ZaVllsVmFWRlZ0ZEhka2JGcEhWMjFHVjAxRVZraFdNV2h6VkRGYWRGVnNaRnBpUmxwb1dsZDRjbVF4WkhSUFZtaHBWbGhDU2xkV1ZtOVpWbVJ6VjJ4a2FsSnRhR0ZVVmxwM1ZFWndSbHBHVGxOTmExcElWa2N4YjFSck1VWmhNMmhYWWxoQ1RGUlZXbk5XTVdSWllrWk9hVkl4U2xaV2FrSnJZakZhVjJKSVVteFRSMUpaVlcxNFlVMUdjRlpXYWtKV1RWVndlbFp0Y0ZkWGJWWnlUbGhhV2xZelRqUldNRnBYWTFaS2MxcEdaR3hoTVZsNlZtMTRVMUZ0VVhsV2EyUlZZbXMxVkZsclpGTlhSbkJZWkVWd1RsSnNjRWxhUldSSFlVVXhWbUpFVWxoaE1taFlWbXhrUzFkSFJrZFdiRnBwVW10d1dWWnFSbUZaVm1SSVZHdGFWV0pIVWs5VmExcGhVMVprVjFWck9XaE5WMUpKVlRJMVMxUXhXblJoUmxwWFlrWndURlV3V2xabFJtUjFWR3hrYVZKdVFqWldhMlEwWVRGU2MxTnJXbGhoTW5oWlZtdFdTMk5zYkhGU2JYUnJVbXh3TVZVeWVHOWhWMFY2VVcwNVYxWjZSVEJhUkVGM1pVWmFkVkpzVG1sU1ZGWjRWMVpvZDFZeFpFZGFTRXBoVW5wc1ZsbHJXbk5PYkZwWFlVaGtWMDFyY0ZaVlYzQlRXVlpLUmxkck9XRlNSVVkwVm0xNGQxSXhaSE5oUms1T1lsZG9hRlpxUm1wbFJUVkhWRmhvV0dKSFVuQlZiWGhoVmtac2RFMVdUbGhTYkhBd1ZGWlNVMkpHV25OaVJGWlZZa2RSZDFacVFYaGpiVTVHWVVad1RsSXlaM3BXYlhCQ1pVZFNWMVp1VmxSaVIxSlVWRlJCTUUxR1duUmpSV1JvVFdzMU1GVnRkR3RYUjBwWVpVWlNXbFpGTlVOVVYzaGhaRWRXU0ZKc2NGZGlSM2N3Vm1wS05GbFdVbk5YYkdSWVlsUnNhRlpxVGtOUk1WRjRVbFJzVVZWVU1Eaz0=\"},\"timezone\":{\"type\":\"string\",\"description\":\"The timezone of the intervals in the CSV file. The default is `UTC`.\",\"example\":\"America/Los_Angeles\"},\"allow_overwrite\":{\"type\":\"boolean\",\"description\":\"When set to `false`, the import fails if existing data conflicts with the uploaded data.\",\"example\":true}}}}}},\"responses\":{\"201\":{\"description\":\"File uploaded successfully.\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"import_id\":{\"type\":\"string\",\"description\":\"The import's unique reference ID.\",\"example\":\"cmgoyz7mb00002f6obth9ojea\"}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"401\":{\"description\":\"**HTTP Status Code:** `401` <br>\\n Unauthorized  \\n\\n \"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"},\"500\":{\"description\":\"**HTTP Status Code:** `500` <br>\\n Internal Server Error  \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WorkforceAdmin:Edit\"],\"x-macro-scopes\":[\"workforce_management:write:admin\"],\"x-granular-scopes\":[\"workforce_management:write:queue_metrics:admin\"]}}},\"/workforce-management/imports/{importId}/historical-queue-metrics\":{\"get\":{\"tags\":[\"Imports\"],\"summary\":\"Get historical queue metrics import metadata\",\"description\":\"Retrieve a list of historical queue metrics import metadata.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:queue_metrics:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\\n\\n**Not supported in Gov cluster** \",\"operationId\":\"Viewhistoricalqueuemetricimoprtmeta\",\"parameters\":[{\"name\":\"importId\",\"in\":\"path\",\"description\":\"The import's unique reference ID.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"cmgoyz7mb00002f6obth9ojea\"}},{\"name\":\"division_id\",\"in\":\"query\",\"description\":\"The division ID.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"ya2kZo77RE6LFqaoKwWM0g\"}}],\"responses\":{\"200\":{\"description\":\"Historical queue metrics import metadata response.\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"create_time\",\"file_name\",\"import_id\",\"imported_by\",\"row_processed\",\"status\",\"timezone\",\"total_rows\"],\"type\":\"object\",\"properties\":{\"status\":{\"type\":\"string\",\"description\":\"The import's status.\",\"example\":\"SUCCESS\",\"enum\":[\"PENDING\",\"FAILED\",\"SUCCESS\",\"DELETING\"]},\"create_time\":{\"type\":\"string\",\"description\":\"The import creation time.\",\"example\":\"2025-11-05T04:13:40.800Z\"},\"row_processed\":{\"type\":\"number\",\"description\":\"The number of rows processed from the import file.\",\"example\":1000},\"total_rows\":{\"type\":\"number\",\"description\":\"The number of rows present in the import file.\",\"example\":1000},\"import_id\":{\"type\":\"string\",\"description\":\"The import's unique reference ID.\",\"example\":\"cmgoyz7mb00002f6obth9ojea\"},\"file_name\":{\"type\":\"string\",\"description\":\"The import file's name.\",\"example\":\"HistoricalData.csv\"},\"timezone\":{\"type\":\"string\",\"description\":\"Themetrics data's timezone.\",\"example\":\"America/New_York\"},\"imported_by\":{\"type\":\"string\",\"description\":\"The unique identifier of the user who imported the metrics.\",\"example\":\"3U-_aQo_ROeigseGAQL5_A\"}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"401\":{\"description\":\"**HTTP Status Code:** `401` <br>\\n Unauthorized  \\n\\n \"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n \"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"},\"500\":{\"description\":\"**HTTP Status Code:** `500` <br>\\n Internal Server Error  \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WorkforceAdmin:Edit\"],\"x-macro-scopes\":[\"workforce_management:read:admin\"],\"x-granular-scopes\":[\"workforce_management:read:queue_metrics:admin\"]}}},\"/workforce-management/organizational-groups\":{\"get\":{\"tags\":[\"Organizational Groups\"],\"summary\":\"Get multiple organizational groups\",\"description\":\"Get multiple organizational groups.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:organizational_groups:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\\n\\n**Not supported in Gov cluster** \",\"operationId\":\"getMultipleOrganizationalGroups\",\"parameters\":[{\"name\":\"name\",\"in\":\"query\",\"description\":\"The organizational group's name.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"West Coast Operations\"}},{\"name\":\"page_size\",\"in\":\"query\",\"description\":\"The number of items to be returned per page.\",\"required\":false,\"schema\":{\"maximum\":50,\"minimum\":1,\"type\":\"integer\",\"example\":15,\"default\":10}},{\"name\":\"next_page_token\",\"in\":\"query\",\"description\":\"The next page token from the previous page, if available. Not required for the first page.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"cmgoyz7mb00002f6obth9ojea\"}}],\"responses\":{\"200\":{\"description\":\"Get multiple organizational groups response.\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"organizational_groups\",\"page_size\",\"total_records\"],\"type\":\"object\",\"properties\":{\"next_page_token\":{\"type\":\"string\",\"description\":\"Use the next page token to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size.\",\"example\":\"cm2n7z2vl000e9cllp4ckoc45\"},\"page_size\":{\"type\":\"integer\",\"description\":\"The number of items returned per page.\",\"example\":15},\"total_records\":{\"type\":\"string\",\"description\":\"The total number of items available across all pages.\",\"example\":\"45\"},\"organizational_groups\":{\"maxItems\":50,\"type\":\"array\",\"description\":\"List of organizational groups.\",\"items\":{\"required\":[\"created_at\",\"description\",\"name\",\"org_group_id\",\"updated_at\"],\"type\":\"object\",\"properties\":{\"org_group_id\":{\"type\":\"string\",\"description\":\"The organizational group's ID.\",\"example\":\"cmgoyz7mb00002f6obth9ojea\"},\"name\":{\"type\":\"string\",\"description\":\"The organizational group's name.\",\"example\":\"West Coast Operations\"},\"description\":{\"type\":\"string\",\"description\":\"The organizational group's description.\",\"example\":\"Organizational group for west coast teams\"},\"created_at\":{\"type\":\"string\",\"description\":\"The organizational group's creation time.\",\"example\":\"2024-01-15T10:30:00Z\"},\"updated_at\":{\"type\":\"string\",\"description\":\"The organizational group's update time.\",\"example\":\"2024-01-15T10:30:00Z\"}}}}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"401\":{\"description\":\"**HTTP Status Code:** `401` <br>\\n Unauthorized  \\n\\n \"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"},\"500\":{\"description\":\"**HTTP Status Code:** `500` <br>\\n Internal Server Error  \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WorkforceAdmin:Edit\"],\"x-macro-scopes\":[\"workforce_management:read:admin\"],\"x-granular-scopes\":[\"workforce_management:read:organizational_groups:admin\"]}},\"post\":{\"tags\":[\"Organizational Groups\"],\"summary\":\"Create an organizational group\",\"description\":\"Create an organizational group.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:write:organizational_groups:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\\n\\n**Not supported in Gov cluster** \",\"operationId\":\"createOrganizationalGroup\",\"requestBody\":{\"content\":{\"application/json\":{\"schema\":{\"required\":[\"name\"],\"type\":\"object\",\"properties\":{\"name\":{\"type\":\"string\",\"description\":\"The organizational group's name.\",\"example\":\"West Coast Operations\"},\"description\":{\"type\":\"string\",\"description\":\"The organizational group's description.\",\"example\":\"Organizational group for west coast teams\"},\"scheduling_group_ids\":{\"maxItems\":100,\"type\":\"array\",\"description\":\"The IDs of the scheduling groups to add to the organizational group.\",\"example\":[\"cmgoyz7mb00002f6obth9ojea\"],\"items\":{\"type\":\"string\"}}}}}}},\"responses\":{\"201\":{\"description\":\"Create organizational group response.\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"created_at\",\"description\",\"name\",\"org_group_id\",\"updated_at\"],\"type\":\"object\",\"properties\":{\"org_group_id\":{\"type\":\"string\",\"description\":\"The organizational group's ID.\",\"example\":\"cmgoyz7mb00002f6obth9ojea\"},\"name\":{\"type\":\"string\",\"description\":\"The organizational group's name.\",\"example\":\"West Coast Operations\"},\"description\":{\"type\":\"string\",\"description\":\"The organizational group's description.\",\"example\":\"Organizational group for west coast teams\"},\"created_at\":{\"type\":\"string\",\"description\":\"The organizational group's creation time.\",\"example\":\"2024-01-15T10:30:00Z\"},\"updated_at\":{\"type\":\"string\",\"description\":\"The organizational group's update time.\",\"example\":\"2024-01-15T10:30:00Z\"}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"401\":{\"description\":\"**HTTP Status Code:** `401` <br>\\n Unauthorized  \\n\\n \"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"},\"500\":{\"description\":\"**HTTP Status Code:** `500` <br>\\n Internal Server Error  \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WorkforceAdmin:Edit\"],\"x-macro-scopes\":[\"workforce_management:write:admin\"],\"x-granular-scopes\":[\"workforce_management:write:organizational_groups:admin\"]}}},\"/workforce-management/organizational-groups/{organizationalGroupId}\":{\"get\":{\"tags\":[\"Organizational Groups\"],\"summary\":\"Get a single organizational group\",\"description\":\"Get a single organizational group.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:organizational_groups:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\\n\\n**Not supported in Gov cluster** \",\"operationId\":\"getOrganizationalGroup\",\"parameters\":[{\"name\":\"organizationalGroupId\",\"in\":\"path\",\"description\":\"The organizational group's ID.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"cmgoyz7mb00002f6obth9ojea\"}}],\"responses\":{\"200\":{\"description\":\"Get a single organizational group's response.\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"agent_count\",\"created_at\",\"description\",\"name\",\"org_group_id\",\"queue_count\",\"scheduling_groups\",\"updated_at\"],\"type\":\"object\",\"properties\":{\"org_group_id\":{\"type\":\"string\",\"description\":\"The organizational group's ID.\",\"example\":\"cmgoyz7mb00002f6obth9ojea\"},\"name\":{\"type\":\"string\",\"description\":\"The organizational group's name.\",\"example\":\"West Coast Operations\"},\"description\":{\"type\":\"string\",\"description\":\"The organizational group's description.\",\"example\":\"Organizational group for west coast teams\"},\"created_at\":{\"type\":\"string\",\"description\":\"The organizational group's creation time.\",\"example\":\"2024-01-15T10:30:00Z\"},\"updated_at\":{\"type\":\"string\",\"description\":\"The organizational group's update time.\",\"example\":\"2024-01-15T10:30:00Z\"},\"agent_count\":{\"type\":\"integer\",\"description\":\"The number of agents in the organizational group.\",\"example\":42},\"queue_count\":{\"type\":\"integer\",\"description\":\"The number of queues in the organizational group.\",\"example\":8},\"scheduling_groups\":{\"maxItems\":100,\"type\":\"array\",\"description\":\"Associated scheduling groups\",\"items\":{\"required\":[\"name\",\"scheduling_group_id\"],\"type\":\"object\",\"properties\":{\"scheduling_group_id\":{\"type\":\"string\",\"description\":\"The scheduling group's ID.\",\"example\":\"cmgoyz7mb00002f6obth9ojea\"},\"name\":{\"type\":\"string\",\"description\":\"The scheduling group's name.\",\"example\":\"Morning Shift\"}}}}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"401\":{\"description\":\"**HTTP Status Code:** `401` <br>\\n Unauthorized  \\n\\n \"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n \"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"},\"500\":{\"description\":\"**HTTP Status Code:** `500` <br>\\n Internal Server Error  \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WorkforceAdmin:Edit\"],\"x-macro-scopes\":[\"workforce_management:read:admin\"],\"x-granular-scopes\":[\"workforce_management:read:organizational_groups:admin\"]}},\"delete\":{\"tags\":[\"Organizational Groups\"],\"summary\":\"Delete organizational group\",\"description\":\"Delete an organizational group.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:delete:organizational_groups:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\\n\\n**Not supported in Gov cluster** \",\"operationId\":\"deleteOrganizationalGroup\",\"parameters\":[{\"name\":\"organizationalGroupId\",\"in\":\"path\",\"description\":\"The organizational group's ID.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"cmgoyz7mb00002f6obth9ojea\"}}],\"responses\":{\"204\":{\"description\":\"Delete organizational group response.\"},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"401\":{\"description\":\"**HTTP Status Code:** `401` <br>\\n Unauthorized  \\n\\n \"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n \"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"},\"500\":{\"description\":\"**HTTP Status Code:** `500` <br>\\n Internal Server Error  \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WorkforceAdmin:Edit\"],\"x-macro-scopes\":[\"workforce_management:write:admin\"],\"x-granular-scopes\":[\"workforce_management:delete:organizational_groups:admin\"]}},\"patch\":{\"tags\":[\"Organizational Groups\"],\"summary\":\"Update an organizational group\",\"description\":\"Update an organizational group.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:update:organizational_groups:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\\n\\n**Not supported in Gov cluster** \",\"operationId\":\"updateOrganizationalGroup\",\"parameters\":[{\"name\":\"organizationalGroupId\",\"in\":\"path\",\"required\":true,\"schema\":{\"type\":\"string\"}}],\"requestBody\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"name\":{\"type\":\"string\",\"description\":\"The organizational group's name.\",\"example\":\"West Coast Operations\"},\"description\":{\"type\":\"string\",\"description\":\"The organizational group's description.\",\"example\":\"Organizational group for west coast teams\"},\"scheduling_group_ids\":{\"maxItems\":100,\"type\":\"array\",\"description\":\"The IDs of the scheduling groups to add to the organizational group.\",\"items\":{\"type\":\"string\",\"example\":\"cmgoyz7mb00002f6obth9ojea\"}}}}}}},\"responses\":{\"200\":{\"description\":\"Update organizational group response.\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"created_at\",\"description\",\"name\",\"org_group_id\",\"updated_at\"],\"type\":\"object\",\"properties\":{\"org_group_id\":{\"type\":\"string\",\"description\":\"The organizational group's ID.\",\"example\":\"cmgoyz7mb00002f6obth9ojea\"},\"name\":{\"type\":\"string\",\"description\":\"The organizational group's name.\",\"example\":\"West Coast Operations\"},\"description\":{\"type\":\"string\",\"description\":\"The organizational group's description.\",\"example\":\"Organizational group for west coast teams\"},\"created_at\":{\"type\":\"string\",\"description\":\"The organizational group creation time.\",\"example\":\"2024-01-15T10:30:00Z\"},\"updated_at\":{\"type\":\"string\",\"description\":\"The organizational group update time.\",\"example\":\"2024-01-15T10:30:00Z\"}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"401\":{\"description\":\"**HTTP Status Code:** `401` <br>\\n Unauthorized  \\n\\n \"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n \"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"},\"500\":{\"description\":\"**HTTP Status Code:** `500` <br>\\n Internal Server Error  \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WorkforceAdmin:Edit\"],\"x-macro-scopes\":[\"workforce_management:write:admin\"],\"x-granular-scopes\":[\"workforce_management:update:organizational_groups:admin\"]}}},\"/workforce-management/reports/adherence/agents\":{\"get\":{\"tags\":[\"Reports\"],\"summary\":\"List agents' adherence data\",\"description\":\"Get agents' adherence data.\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:list_adherence_agents:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\",\"operationId\":\"listAdherenceAgents\",\"parameters\":[{\"name\":\"date\",\"in\":\"query\",\"description\":\"The date in 'yyyy-MM-dd' format.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"2025-09-01\"}},{\"name\":\"timezone\",\"in\":\"query\",\"description\":\"The adherence timezone. The default is UTC.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"America/Los_Angeles\",\"default\":\"UTC\"}},{\"name\":\"next_page_token\",\"in\":\"query\",\"description\":\"Use the next page token to paginate through large result sets. A next page token returns whenever the set of available results exceeds the current page size.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"jA5csQv1W1oXuYZLspNIZzMOXqTD9r9Rje2\",\"default\":\"null\"}},{\"name\":\"page_size\",\"in\":\"query\",\"description\":\"The number of agents returned per page.\",\"required\":false,\"schema\":{\"maximum\":10,\"minimum\":1,\"type\":\"integer\",\"example\":10,\"default\":10}},{\"name\":\"division_id\",\"in\":\"query\",\"description\":\"The division ID.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"ya2kZo77RE6LFqaoKwWM0g\"}}],\"responses\":{\"200\":{\"description\":\"List of agents with adherence for the date.\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"agents\",\"next_page_token\",\"page_size\",\"timezone\",\"total_records\"],\"type\":\"object\",\"properties\":{\"next_page_token\":{\"type\":\"string\",\"description\":\"Use the next page token to paginate through large result sets. A next page token returns whenever the set of available results exceeds the current page size.\",\"example\":\"jA5csQv1W1oXuYZLspNIZzMOXqTD9r9Rje2\"},\"page_size\":{\"type\":\"integer\",\"description\":\"The number of agents returned per page.\",\"example\":10},\"total_records\":{\"type\":\"integer\",\"description\":\"The total number of agents available across pages.\",\"example\":100},\"timezone\":{\"type\":\"string\",\"description\":\"The adherence timezone. The default is UTC.\",\"example\":\"America/Los_Angeles\"},\"agents\":{\"maxItems\":10,\"type\":\"array\",\"description\":\"List of agents with adherence.\",\"items\":{\"required\":[\"adherence_time_for_activity\",\"display_name\",\"schedule_activities\",\"shifts\",\"user_id\"],\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The agent's unique identifier.\",\"example\":\"0c30da44bd68aae3b5714abb885a59\"},\"display_name\":{\"type\":\"string\",\"description\":\"The agent's name.\",\"example\":\"Alex\"},\"shifts\":{\"maxItems\":10,\"type\":\"array\",\"description\":\"Shift details.\",\"items\":{\"required\":[\"end_in_adherence\",\"end_time\",\"start_in_adherence\",\"start_time\"],\"type\":\"object\",\"properties\":{\"start_time\":{\"type\":\"string\",\"description\":\"The shift start time, in ISO 8601 format `yyyy-MM-dd'T'HH:mm:ss'Z'` or yyyy-MM-dd'T'HH:mm:ss'TZD'`.\",\"example\":\"2024-11-06T03:30:00.000Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The shift end time, in ISO 8601 format `yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd'T'HH:mm:ss'TZD'`.\",\"example\":\"2024-11-06T03:30:00.000Z\"},\"start_in_adherence\":{\"type\":\"integer\",\"description\":\"The number of seconds after shift start that the user started being in adherence.\",\"example\":0},\"end_in_adherence\":{\"type\":\"integer\",\"description\":\"The number of seconds before shift end that the user started being in adherence.\",\"example\":0}}}},\"schedule_activities\":{\"maxItems\":250,\"type\":\"array\",\"description\":\"All activities in the schedule.\",\"items\":{\"required\":[\"activity_id\",\"activity_name\",\"end_time\",\"start_time\"],\"type\":\"object\",\"properties\":{\"activity_id\":{\"type\":\"string\",\"description\":\"The activity's unique identifier.\",\"example\":\"0c30da44bd68aae3b5714abb885a59\"},\"activity_name\":{\"type\":\"string\",\"description\":\"The activity's name.\",\"example\":\"Chat\"},\"start_time\":{\"type\":\"integer\",\"description\":\"The activity's start time in timestamp.\",\"example\":1730896200000},\"end_time\":{\"type\":\"integer\",\"description\":\"The activity's end time in timestamp.\",\"example\":1730896200000}},\"description\":\"Activity details\"}},\"adherence_time_for_activity\":{\"maxItems\":250,\"type\":\"array\",\"description\":\"All unique activities in the schedule with adherence.\",\"items\":{\"required\":[\"activity_id\",\"activity_in_adherence\",\"activity_name\",\"activity_out_adherence\"],\"type\":\"object\",\"properties\":{\"activity_id\":{\"type\":\"string\",\"description\":\"The activity's unique identifier.\",\"example\":\"0c30da44bd68aae3b5714abb885a59\"},\"activity_name\":{\"type\":\"string\",\"description\":\"The activity's name.\",\"example\":\"Chat\"},\"activity_in_adherence\":{\"type\":\"integer\",\"description\":\"Activity in adherence, in seconds.\",\"example\":0},\"activity_out_adherence\":{\"type\":\"integer\",\"description\":\"Activity out of adherence, in seconds.\",\"example\":0}},\"description\":\"The activity's adherence details.\"}}},\"description\":\"Agent and adherence details.\"}}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `1902` <br>\\n Invalid date parameter. <br>\\n**Error Code:** `1903` <br>\\n Invalid timezone. <br>\\n**Error Code:** `1904` <br>\\n Invalid page size. <br>\\n\"},\"401\":{\"description\":\"**HTTP Status Code:** `401` <br>\\n Unauthorized  \\n\\n \"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n \"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](/docs/api/rest/rate-limits/). \\n\\n \"},\"500\":{\"description\":\"**HTTP Status Code:** `500` <br>\\n Internal Server Error  \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WorkforceAdmin:Edit\"],\"x-granular-scopes\":[\"workforce_management:read:list_adherence_agents:admin\"]}}},\"/workforce-management/reports/schedules/agents\":{\"get\":{\"tags\":[\"Reports\"],\"summary\":\"List all schedule agents\",\"description\":\"List all the schedule agents for the particular date and timezone\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `workforce_management:read:list_schedule_agents:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\",\"operationId\":\"listScheduleAgents\",\"parameters\":[{\"name\":\"date\",\"in\":\"query\",\"description\":\"The date in 'yyyy-MM-dd' format.\",\"required\":true,\"schema\":{\"type\":\"string\",\"format\":\"date\",\"example\":\"2025-09-01\"}},{\"name\":\"user_ids\",\"in\":\"query\",\"description\":\"Return results only for the specified user IDs. Add multiple `user_ids` parameters to include more users. Example: `?user_ids=123&user_ids=456`.\",\"required\":false,\"schema\":{\"type\":\"array\",\"format\":\"string\",\"example\":[\"jA5csQv1W1oXuYZLspNIZzMOXqTD9r9Rje2\"]}},{\"name\":\"timezone\",\"in\":\"query\",\"description\":\"The schedule's timezone. The default is UTC.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"America/Los_Angeles\",\"default\":\"null\"}},{\"name\":\"next_page_token\",\"in\":\"query\",\"description\":\"Use the next page token to paginate through large result sets. A next page token returns whenever the set of available results exceeds the current page size. \",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"jA5csQv1W1oXuYZLspNIZzMOXqTD9r9Rje2\",\"default\":\"null\"}},{\"name\":\"page_size\",\"in\":\"query\",\"description\":\"The number of scheduled agents returned per page.\",\"required\":false,\"schema\":{\"maximum\":10,\"minimum\":1,\"type\":\"integer\",\"example\":10,\"default\":10}},{\"name\":\"division_id\",\"in\":\"query\",\"description\":\"The division ID.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"ya2kZo77RE6LFqaoKwWM0g\"}}],\"responses\":{\"200\":{\"description\":\"List of agents in the schedule for the date\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"agents\",\"next_page_token\",\"page_size\",\"timezone\",\"total_records\"],\"type\":\"object\",\"properties\":{\"next_page_token\":{\"type\":\"string\",\"description\":\"Use the next page token to paginate through large result sets. A next page token returns whenever the set of available results exceeds the current page size. \",\"example\":\"jA5csQv1W1oXuYZLspNIZzMOXqTD9r9Rje2\",\"default\":\"null\"},\"page_size\":{\"maxLength\":10,\"minLength\":1,\"type\":\"integer\",\"description\":\"The number of scheduled agents returned per page.\",\"example\":10,\"default\":10},\"total_records\":{\"type\":\"integer\",\"description\":\" The total number of all scheduled agents available across pages.\",\"example\":100},\"timezone\":{\"type\":\"string\",\"description\":\"The schedule's timezone. The default is UTC.\",\"example\":\"America/Los_Angeles\"},\"agents\":{\"maxItems\":250,\"type\":\"array\",\"description\":\"List of agents in the schedule\",\"items\":{\"required\":[\"activities\",\"display_name\",\"scheduling_group_name\",\"total_paid_time\",\"total_scheduled_time\",\"user_email\",\"user_id\"],\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The agent unique identifier\",\"example\":\"0c30da44bd68aae3b5714abb885a59\"},\"display_name\":{\"type\":\"string\",\"description\":\"The agent's display name\",\"example\":\"Alex\"},\"user_email\":{\"type\":\"string\",\"description\":\"The agent's email address\",\"example\":\"alex@example.com\"},\"scheduling_group_name\":{\"type\":\"string\",\"description\":\"The name of the scheduling group\",\"example\":\"Sales\",\"default\":\"null\"},\"total_scheduled_time\":{\"type\":\"integer\",\"description\":\"The total schedule duration, in minutes.\",\"example\":480,\"default\":0},\"total_paid_time\":{\"type\":\"integer\",\"description\":\"The total duration of paid schedule activity in minutes\",\"example\":340,\"default\":0},\"activities\":{\"maxItems\":250,\"type\":\"array\",\"description\":\"List of activities\",\"items\":{\"required\":[\"activity_name\",\"activity_type\",\"end_time\",\"is_paid\",\"schedule_activity_id\",\"start_time\"],\"type\":\"object\",\"properties\":{\"schedule_activity_id\":{\"type\":\"string\",\"description\":\"The activity unique identifier\",\"example\":\"cll20wknm01cwex0isvewiegl\"},\"start_time\":{\"type\":\"string\",\"description\":\"The schedule activity start time. In ISO 8601 format (yyyy-MM-dd'T'HH:mm:ss'Z' or yyyy-MM-dd'T'HH:mm:ss\u2019TZD\u2019).\",\"example\":\"2024-07-20T14:00:00-08:00\"},\"end_time\":{\"type\":\"string\",\"description\":\"The schedule activity end time. in ISO 8601 format (yyyy-MM-dd'T'HH:mm:ss'Z' or yyyy-MM-dd'T'HH:mm:ss\u2019TZD\u2019).\",\"example\":\"2024-07-20T14:00:00-08:00\"},\"activity_name\":{\"type\":\"string\",\"description\":\"The activity name identifier\",\"example\":\"SMS\"},\"activity_type\":{\"type\":\"string\",\"description\":\"Type of the activity\",\"example\":\"Productive\"},\"is_paid\":{\"type\":\"boolean\",\"description\":\"The is_paid indicates if the scheduled activity counts as paid time for calculating the agent's total paid duration.\",\"example\":true}},\"description\":\"The activity details\"}}},\"description\":\"The schedule agent details\"}}},\"description\":\"Schedule agents list returned\"}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `1902` <br>\\n Invalid date parameter <br>\\n**Error Code:** `1903` <br>\\n Invalid timezone <br>\\n**Error Code:** `1904` <br>\\n Invalid page size <br>\\n\"},\"401\":{\"description\":\"**HTTP Status Code:** `401` <br>\\n Unauthorized  \\n\\n \"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n \"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"},\"500\":{\"description\":\"**HTTP Status Code:** `500` <br>\\n Internal Server Error  Internal server error \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WorkforceAdmin:Edit\"],\"x-granular-scopes\":[\"workforce_management:read:list_schedule_agents:admin\"]}}}},\"components\":{\"securitySchemes\":{\"openapi_authorization\":{\"type\":\"http\",\"scheme\":\"bearer\"}}}}",
    "frontmatter": {
      "title": "Workforce Management APIs",
      "keywords": "",
      "description": "Workforce Management",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
