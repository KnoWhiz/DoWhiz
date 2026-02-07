# Quality Management Webhooks

- Source URL: https://developers.zoom.us/docs/api/quality-management/events/
- Snapshot path: docs/api/quality-management/events
- Generated (UTC): 2026-02-07T22:40:11.501443+00:00

## Frontmatter

```json
{
  "title": "Quality Management Webhooks",
  "keywords": "zoom, rest, api, quality managment",
  "description": "Zoom Quality Management is designed to help contact centers track and analyze customer interactions, measure agent performance, provide actionable insights, and proactively identify areas for improvement while maintaining organizational compliance.",
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
    "title": "Quality Management",
    "description": "Zoom Quality Management is designed to help contact centers track and analyze customer interactions, measure agent performance, provide actionable insights, and proactively identify areas for improvement while maintaining organizational compliance.",
    "version": "1.0.0"
  },
  "paths": {
    "quality_management.interaction_analysis_completed": {
      "post": {
        "tags": [
          "quality_management"
        ],
        "operationId": "quality_management.interaction_analysis_completed",
        "requestBody": {
          "description": "# quality_management.interaction_analysis_completed\nZoom triggers this event after the Quality Management users interaction analysis is completed.\n\n## Prerequisites\n\n* A Pro or higher plan.\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n\t* A valid **Event Notification Endpoint URL**\n\t* The **Quality Management interaction analysis completed** subscription must be enabled under the **Quality Management** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `qm_interactions:read:admin`,`qm_interactions:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_quality_management:read:interactions`,`zoom_quality_management:read:interactions:admin`\n\n**Event type**: `quality_management.interaction_analysis_completed`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "description": "The timestamp at which the event occurred."
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
                        "description": "The account ID of the user who recorded the interaction."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the interaction.",
                        "required": [
                          "interaction_info"
                        ],
                        "properties": {
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the interaction."
                          },
                          "duration": {
                            "type": "integer",
                            "description": "The interaction duration in seconds."
                          },
                          "queue_id": {
                            "type": "string",
                            "description": "The ID of the queue."
                          },
                          "queue_name": {
                            "type": "string",
                            "description": "The name of the queue."
                          },
                          "engagement_id": {
                            "type": "string",
                            "description": "The engagement ID."
                          },
                          "direction": {
                            "type": "string",
                            "description": "The direction."
                          },
                          "disposition": {
                            "type": "string",
                            "description": "The disposition."
                          },
                          "silence_event_count": {
                            "type": "integer",
                            "description": "The count of silence events"
                          },
                          "crosstalk_event_count": {
                            "type": "integer",
                            "description": "The count of crosstalk events."
                          },
                          "hold_event_count": {
                            "type": "integer",
                            "description": "The count of hold events"
                          },
                          "primary_language": {
                            "type": "string",
                            "description": "The primary language."
                          },
                          "interaction_info": {
                            "type": "object",
                            "description": "The interaction infomation.",
                            "required": [
                              "interaction_id"
                            ],
                            "properties": {
                              "from": {
                                "type": "string",
                                "description": "The caller's phone number in E.164 format."
                              },
                              "to": {
                                "type": "string",
                                "description": "The recipient's phone number in E.164 format."
                              },
                              "interaction_id": {
                                "type": "string",
                                "description": "The interaction ID."
                              },
                              "agent_name": {
                                "type": "string",
                                "description": "The agent name."
                              },
                              "consumer_name": {
                                "type": "string",
                                "description": "The consumer name."
                              },
                              "channel_type": {
                                "type": "string",
                                "description": "The channel type.",
                                "enum": [
                                  "VIDEO",
                                  "MANUAL",
                                  "SMS",
                                  "AUDIO",
                                  "WEB_CHAT"
                                ]
                              },
                              "associated_ids": {
                                "type": "array",
                                "description": "The associated interaction IDs.",
                                "items": {
                                  "type": "string",
                                  "description": "The associated interaction ID."
                                }
                              }
                            }
                          },
                          "summary": {
                            "type": "object",
                            "description": "The summary information.",
                            "properties": {
                              "evaluation_score": {
                                "type": "number",
                                "description": "The evaluation score."
                              },
                              "sentiment_score": {
                                "type": "number",
                                "description": "The sentiment score."
                              },
                              "engagement_score": {
                                "type": "number",
                                "description": "The engagement score."
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
                  "value": "{\n  \"event\": \"quality_management.interaction_analysis_completed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"start_time\": \"2020-05-01T19:13:02Z\",\n      \"duration\": 5,\n      \"queue_id\": \"Je4zLPZ-SBiLCiebE35ALQ\",\n      \"queue_name\": \"Demo\",\n      \"interaction_info\": {\n        \"from\": \"+16692520210\",\n        \"to\": \"+12364352078\",\n        \"interaction_id\": \"X51n17BZQTqKtxwF4mmTIw\",\n        \"agent_name\": \"Admin3\",\n        \"consumer_name\": \"+16692520210\",\n        \"channel_type\": \"AUDIO\",\n        \"associated_ids\": [\n          \"xxxx\"\n        ]\n      },\n      \"engagement_id\": \"PH6wferlTACAfBzYGAVOYg\",\n      \"direction\": \"inbound\",\n      \"disposition\": \"dev-disposition\",\n      \"silence_event_count\": 2,\n      \"crosstalk_event_count\": 2,\n      \"hold_event_count\": 3,\n      \"summary\": {\n        \"evaluation_score\": 58.63,\n        \"sentiment_score\": 58.63,\n        \"engagement_score\": 58.63\n      },\n      \"primary_language\": \"en\"\n    }\n  }\n}"
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
                      "description": "The timestamp at which the event occurred."
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
                          "description": "The account ID of the user who recorded the interaction."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the interaction.",
                          "required": [
                            "interaction_info"
                          ],
                          "properties": {
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the interaction."
                            },
                            "duration": {
                              "type": "integer",
                              "description": "The interaction duration in seconds."
                            },
                            "queue_id": {
                              "type": "string",
                              "description": "The ID of the queue."
                            },
                            "queue_name": {
                              "type": "string",
                              "description": "The name of the queue."
                            },
                            "engagement_id": {
                              "type": "string",
                              "description": "The engagement ID."
                            },
                            "direction": {
                              "type": "string",
                              "description": "The direction."
                            },
                            "disposition": {
                              "type": "string",
                              "description": "The disposition."
                            },
                            "silence_event_count": {
                              "type": "integer",
                              "description": "The count of silence events"
                            },
                            "crosstalk_event_count": {
                              "type": "integer",
                              "description": "The count of crosstalk events."
                            },
                            "hold_event_count": {
                              "type": "integer",
                              "description": "The count of hold events"
                            },
                            "primary_language": {
                              "type": "string",
                              "description": "The primary language."
                            },
                            "interaction_info": {
                              "type": "object",
                              "description": "The interaction infomation.",
                              "required": [
                                "interaction_id"
                              ],
                              "properties": {
                                "from": {
                                  "type": "string",
                                  "description": "The caller's phone number in E.164 format."
                                },
                                "to": {
                                  "type": "string",
                                  "description": "The recipient's phone number in E.164 format."
                                },
                                "interaction_id": {
                                  "type": "string",
                                  "description": "The interaction ID."
                                },
                                "agent_name": {
                                  "type": "string",
                                  "description": "The agent name."
                                },
                                "consumer_name": {
                                  "type": "string",
                                  "description": "The consumer name."
                                },
                                "channel_type": {
                                  "type": "string",
                                  "description": "The channel type.",
                                  "enum": [
                                    "VIDEO",
                                    "MANUAL",
                                    "SMS",
                                    "AUDIO",
                                    "WEB_CHAT"
                                  ]
                                },
                                "associated_ids": {
                                  "type": "array",
                                  "description": "The associated interaction IDs.",
                                  "items": {
                                    "type": "string",
                                    "description": "The associated interaction ID."
                                  }
                                }
                              }
                            },
                            "summary": {
                              "type": "object",
                              "description": "The summary information.",
                              "properties": {
                                "evaluation_score": {
                                  "type": "number",
                                  "description": "The evaluation score."
                                },
                                "sentiment_score": {
                                  "type": "number",
                                  "description": "The sentiment score."
                                },
                                "engagement_score": {
                                  "type": "number",
                                  "description": "The engagement score."
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
                    "value": "{\n  \"event\": \"quality_management.interaction_analysis_completed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"start_time\": \"2020-05-01T19:13:02Z\",\n      \"duration\": 5,\n      \"queue_id\": \"Je4zLPZ-SBiLCiebE35ALQ\",\n      \"queue_name\": \"Demo\",\n      \"interaction_info\": {\n        \"from\": \"+16692520210\",\n        \"to\": \"+12364352078\",\n        \"interaction_id\": \"X51n17BZQTqKtxwF4mmTIw\",\n        \"agent_name\": \"Admin3\",\n        \"consumer_name\": \"+16692520210\",\n        \"channel_type\": \"AUDIO\",\n        \"associated_ids\": [\n          \"xxxx\"\n        ]\n      },\n      \"engagement_id\": \"PH6wferlTACAfBzYGAVOYg\",\n      \"direction\": \"inbound\",\n      \"disposition\": \"dev-disposition\",\n      \"silence_event_count\": 2,\n      \"crosstalk_event_count\": 2,\n      \"hold_event_count\": 3,\n      \"summary\": {\n        \"evaluation_score\": 58.63,\n        \"sentiment_score\": 58.63,\n        \"engagement_score\": 58.63\n      },\n      \"primary_language\": \"en\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "quality_management.evaluation_completed": {
      "post": {
        "tags": [
          "quality_management"
        ],
        "operationId": "quality_management.evaluation_completed",
        "requestBody": {
          "description": "# quality_management.evaluation_completed\nZoom triggers this event after the Quality Management user evaluation is completed.\n\n## Prerequisites\n\n* A Pro or higher plan.\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n\t* A valid **Event Notification Endpoint URL**\n\t* The **Quality Management evaluation completed** subscription must be enabled under the **Quality Management** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `qm_evaluations:read:admin`,`qm_evaluations:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_quality_management:read:evaluations`,`zoom_quality_management:read:evaluations:admin`\n\n**Event type**: `quality_management.evaluation_completed`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "description": "The timestamp at which the event occurred."
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
                        "description": "The account ID of the user who recorded the interaction."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the interaction.",
                        "required": [
                          "evaluation_id"
                        ],
                        "properties": {
                          "interaction_id": {
                            "type": "string",
                            "description": "The interaction's ID."
                          },
                          "agent_user_id": {
                            "type": "string",
                            "description": "The agent's ID."
                          },
                          "evaluator_user_id": {
                            "type": "string",
                            "description": "The evaluator's ID."
                          },
                          "evaluation_id": {
                            "type": "string",
                            "description": "The evaluation's ID."
                          },
                          "agent_info": {
                            "type": "object",
                            "description": "The agent information.",
                            "properties": {
                              "agent_user_id": {
                                "type": "string",
                                "description": "The agent's user ID."
                              },
                              "display_name": {
                                "type": "string",
                                "description": "The agent's name."
                              }
                            }
                          },
                          "evaluators_info": {
                            "type": "array",
                            "description": "The evaluators' information.",
                            "items": {
                              "type": "object",
                              "description": "The evaluator's information.",
                              "properties": {
                                "evaluator_user_id": {
                                  "type": "string",
                                  "description": "The evaluator's ID."
                                },
                                "display_name": {
                                  "type": "string",
                                  "description": "The evaluator's name."
                                }
                              }
                            }
                          },
                          "scorecard_name": {
                            "type": "string",
                            "description": "The scorecard's name."
                          },
                          "scoring_format": {
                            "type": "string",
                            "description": "The scoring format of the scorecard."
                          },
                          "total_possible_score": {
                            "type": "number",
                            "description": "the maximum possible score for this evaluation"
                          },
                          "evaluation_score": {
                            "type": "number",
                            "description": "The overall score of the evaluation, calculated based on the scorecard and answers."
                          },
                          "evaluation_score_ratio": {
                            "type": "number",
                            "description": "Score received by the agent, expressed as a ratio 0.00-1.00."
                          },
                          "evaluation_type": {
                            "type": "string",
                            "description": "Distinguish between automatic scoring and manual scoring.",
                            "enum": [
                              "MANUAL",
                              "AUTOMATED"
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
                  "value": "{\n  \"event\": \"quality_management.interaction_analysis_completed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"interaction_id\": \"ldl3k52kdfl3hd3\",\n      \"agent_user_id\": \"ZWR1C1FB695C654A829FE56553A709E8EBB\",\n      \"evaluator_user_id\": \"ZWR1C1FB695C654A829FE56553A709E8EBB\",\n      \"evaluation_id\": \"0e73a69f8f104d24a589fa0c615abca8\",\n      \"agent_info\": {\n        \"agent_user_id\": \"httgPnTgTxq0d65-alfYVw\",\n        \"display_name\": \"Ashlee York\"\n      },\n      \"evaluators_info\": [\n        {\n          \"evaluator_user_id\": \"b0_yDuNGSq65EF8i1f0PcQ\",\n          \"display_name\": \"Rick Arps\"\n        }\n      ],\n      \"scorecard_name\": \"Customer Care\",\n      \"scoring_format\": \"points\",\n      \"total_possible_score\": 50,\n      \"evaluation_score\": 10,\n      \"evaluation_score_ratio\": 0.2,\n      \"evaluation_type\": \"AUTOMATED\"\n    }\n  }\n}"
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
                      "description": "The timestamp at which the event occurred."
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
                          "description": "The account ID of the user who recorded the interaction."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the interaction.",
                          "required": [
                            "evaluation_id"
                          ],
                          "properties": {
                            "interaction_id": {
                              "type": "string",
                              "description": "The interaction's ID."
                            },
                            "agent_user_id": {
                              "type": "string",
                              "description": "The agent's ID."
                            },
                            "evaluator_user_id": {
                              "type": "string",
                              "description": "The evaluator's ID."
                            },
                            "evaluation_id": {
                              "type": "string",
                              "description": "The evaluation's ID."
                            },
                            "agent_info": {
                              "type": "object",
                              "description": "The agent information.",
                              "properties": {
                                "agent_user_id": {
                                  "type": "string",
                                  "description": "The agent's user ID."
                                },
                                "display_name": {
                                  "type": "string",
                                  "description": "The agent's name."
                                }
                              }
                            },
                            "evaluators_info": {
                              "type": "array",
                              "description": "The evaluators' information.",
                              "items": {
                                "type": "object",
                                "description": "The evaluator's information.",
                                "properties": {
                                  "evaluator_user_id": {
                                    "type": "string",
                                    "description": "The evaluator's ID."
                                  },
                                  "display_name": {
                                    "type": "string",
                                    "description": "The evaluator's name."
                                  }
                                }
                              }
                            },
                            "scorecard_name": {
                              "type": "string",
                              "description": "The scorecard's name."
                            },
                            "scoring_format": {
                              "type": "string",
                              "description": "The scoring format of the scorecard."
                            },
                            "total_possible_score": {
                              "type": "number",
                              "description": "the maximum possible score for this evaluation"
                            },
                            "evaluation_score": {
                              "type": "number",
                              "description": "The overall score of the evaluation, calculated based on the scorecard and answers."
                            },
                            "evaluation_score_ratio": {
                              "type": "number",
                              "description": "Score received by the agent, expressed as a ratio 0.00-1.00."
                            },
                            "evaluation_type": {
                              "type": "string",
                              "description": "Distinguish between automatic scoring and manual scoring.",
                              "enum": [
                                "MANUAL",
                                "AUTOMATED"
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
                    "value": "{\n  \"event\": \"quality_management.interaction_analysis_completed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"interaction_id\": \"ldl3k52kdfl3hd3\",\n      \"agent_user_id\": \"ZWR1C1FB695C654A829FE56553A709E8EBB\",\n      \"evaluator_user_id\": \"ZWR1C1FB695C654A829FE56553A709E8EBB\",\n      \"evaluation_id\": \"0e73a69f8f104d24a589fa0c615abca8\",\n      \"agent_info\": {\n        \"agent_user_id\": \"httgPnTgTxq0d65-alfYVw\",\n        \"display_name\": \"Ashlee York\"\n      },\n      \"evaluators_info\": [\n        {\n          \"evaluator_user_id\": \"b0_yDuNGSq65EF8i1f0PcQ\",\n          \"display_name\": \"Rick Arps\"\n        }\n      ],\n      \"scorecard_name\": \"Customer Care\",\n      \"scoring_format\": \"points\",\n      \"total_possible_score\": 50,\n      \"evaluation_score\": 10,\n      \"evaluation_score_ratio\": 0.2,\n      \"evaluation_type\": \"AUTOMATED\"\n    }\n  }\n}"
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
    "title": "Quality Management",
    "description": "Zoom Quality Management is designed to help contact centers track and analyze customer interactions, measure agent performance, provide actionable insights, and proactively identify areas for improvement while maintaining organizational compliance.",
    "version": "1.0.0"
  },
  "webhooks": {
    "quality_management.interaction_analysis_completed": {
      "post": {
        "tags": [
          "quality_management"
        ],
        "operationId": "quality_management.interaction_analysis_completed",
        "requestBody": {
          "description": "# quality_management.interaction_analysis_completed\nZoom triggers this event after the Quality Management users interaction analysis is completed.\n\n## Prerequisites\n\n* A Pro or higher plan.\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n\t* A valid **Event Notification Endpoint URL**\n\t* The **Quality Management interaction analysis completed** subscription must be enabled under the **Quality Management** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `qm_interactions:read:admin`,`qm_interactions:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_quality_management:read:interactions`,`zoom_quality_management:read:interactions:admin`\n\n**Event type**: `quality_management.interaction_analysis_completed`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "description": "The timestamp at which the event occurred."
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
                        "description": "The account ID of the user who recorded the interaction."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the interaction.",
                        "required": [
                          "interaction_info"
                        ],
                        "properties": {
                          "start_time": {
                            "type": "string",
                            "description": "The start time of the interaction."
                          },
                          "duration": {
                            "type": "integer",
                            "description": "The interaction duration in seconds."
                          },
                          "queue_id": {
                            "type": "string",
                            "description": "The ID of the queue."
                          },
                          "queue_name": {
                            "type": "string",
                            "description": "The name of the queue."
                          },
                          "engagement_id": {
                            "type": "string",
                            "description": "The engagement ID."
                          },
                          "direction": {
                            "type": "string",
                            "description": "The direction."
                          },
                          "disposition": {
                            "type": "string",
                            "description": "The disposition."
                          },
                          "silence_event_count": {
                            "type": "integer",
                            "description": "The count of silence events"
                          },
                          "crosstalk_event_count": {
                            "type": "integer",
                            "description": "The count of crosstalk events."
                          },
                          "hold_event_count": {
                            "type": "integer",
                            "description": "The count of hold events"
                          },
                          "primary_language": {
                            "type": "string",
                            "description": "The primary language."
                          },
                          "interaction_info": {
                            "type": "object",
                            "description": "The interaction infomation.",
                            "required": [
                              "interaction_id"
                            ],
                            "properties": {
                              "from": {
                                "type": "string",
                                "description": "The caller's phone number in E.164 format."
                              },
                              "to": {
                                "type": "string",
                                "description": "The recipient's phone number in E.164 format."
                              },
                              "interaction_id": {
                                "type": "string",
                                "description": "The interaction ID."
                              },
                              "agent_name": {
                                "type": "string",
                                "description": "The agent name."
                              },
                              "consumer_name": {
                                "type": "string",
                                "description": "The consumer name."
                              },
                              "channel_type": {
                                "type": "string",
                                "description": "The channel type.",
                                "enum": [
                                  "VIDEO",
                                  "MANUAL",
                                  "SMS",
                                  "AUDIO",
                                  "WEB_CHAT"
                                ]
                              },
                              "associated_ids": {
                                "type": "array",
                                "description": "The associated interaction IDs.",
                                "items": {
                                  "type": "string",
                                  "description": "The associated interaction ID."
                                }
                              }
                            }
                          },
                          "summary": {
                            "type": "object",
                            "description": "The summary information.",
                            "properties": {
                              "evaluation_score": {
                                "type": "number",
                                "description": "The evaluation score."
                              },
                              "sentiment_score": {
                                "type": "number",
                                "description": "The sentiment score."
                              },
                              "engagement_score": {
                                "type": "number",
                                "description": "The engagement score."
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
                  "value": "{\n  \"event\": \"quality_management.interaction_analysis_completed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"start_time\": \"2020-05-01T19:13:02Z\",\n      \"duration\": 5,\n      \"queue_id\": \"Je4zLPZ-SBiLCiebE35ALQ\",\n      \"queue_name\": \"Demo\",\n      \"interaction_info\": {\n        \"from\": \"+16692520210\",\n        \"to\": \"+12364352078\",\n        \"interaction_id\": \"X51n17BZQTqKtxwF4mmTIw\",\n        \"agent_name\": \"Admin3\",\n        \"consumer_name\": \"+16692520210\",\n        \"channel_type\": \"AUDIO\",\n        \"associated_ids\": [\n          \"xxxx\"\n        ]\n      },\n      \"engagement_id\": \"PH6wferlTACAfBzYGAVOYg\",\n      \"direction\": \"inbound\",\n      \"disposition\": \"dev-disposition\",\n      \"silence_event_count\": 2,\n      \"crosstalk_event_count\": 2,\n      \"hold_event_count\": 3,\n      \"summary\": {\n        \"evaluation_score\": 58.63,\n        \"sentiment_score\": 58.63,\n        \"engagement_score\": 58.63\n      },\n      \"primary_language\": \"en\"\n    }\n  }\n}"
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
                      "description": "The timestamp at which the event occurred."
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
                          "description": "The account ID of the user who recorded the interaction."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the interaction.",
                          "required": [
                            "interaction_info"
                          ],
                          "properties": {
                            "start_time": {
                              "type": "string",
                              "description": "The start time of the interaction."
                            },
                            "duration": {
                              "type": "integer",
                              "description": "The interaction duration in seconds."
                            },
                            "queue_id": {
                              "type": "string",
                              "description": "The ID of the queue."
                            },
                            "queue_name": {
                              "type": "string",
                              "description": "The name of the queue."
                            },
                            "engagement_id": {
                              "type": "string",
                              "description": "The engagement ID."
                            },
                            "direction": {
                              "type": "string",
                              "description": "The direction."
                            },
                            "disposition": {
                              "type": "string",
                              "description": "The disposition."
                            },
                            "silence_event_count": {
                              "type": "integer",
                              "description": "The count of silence events"
                            },
                            "crosstalk_event_count": {
                              "type": "integer",
                              "description": "The count of crosstalk events."
                            },
                            "hold_event_count": {
                              "type": "integer",
                              "description": "The count of hold events"
                            },
                            "primary_language": {
                              "type": "string",
                              "description": "The primary language."
                            },
                            "interaction_info": {
                              "type": "object",
                              "description": "The interaction infomation.",
                              "required": [
                                "interaction_id"
                              ],
                              "properties": {
                                "from": {
                                  "type": "string",
                                  "description": "The caller's phone number in E.164 format."
                                },
                                "to": {
                                  "type": "string",
                                  "description": "The recipient's phone number in E.164 format."
                                },
                                "interaction_id": {
                                  "type": "string",
                                  "description": "The interaction ID."
                                },
                                "agent_name": {
                                  "type": "string",
                                  "description": "The agent name."
                                },
                                "consumer_name": {
                                  "type": "string",
                                  "description": "The consumer name."
                                },
                                "channel_type": {
                                  "type": "string",
                                  "description": "The channel type.",
                                  "enum": [
                                    "VIDEO",
                                    "MANUAL",
                                    "SMS",
                                    "AUDIO",
                                    "WEB_CHAT"
                                  ]
                                },
                                "associated_ids": {
                                  "type": "array",
                                  "description": "The associated interaction IDs.",
                                  "items": {
                                    "type": "string",
                                    "description": "The associated interaction ID."
                                  }
                                }
                              }
                            },
                            "summary": {
                              "type": "object",
                              "description": "The summary information.",
                              "properties": {
                                "evaluation_score": {
                                  "type": "number",
                                  "description": "The evaluation score."
                                },
                                "sentiment_score": {
                                  "type": "number",
                                  "description": "The sentiment score."
                                },
                                "engagement_score": {
                                  "type": "number",
                                  "description": "The engagement score."
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
                    "value": "{\n  \"event\": \"quality_management.interaction_analysis_completed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"start_time\": \"2020-05-01T19:13:02Z\",\n      \"duration\": 5,\n      \"queue_id\": \"Je4zLPZ-SBiLCiebE35ALQ\",\n      \"queue_name\": \"Demo\",\n      \"interaction_info\": {\n        \"from\": \"+16692520210\",\n        \"to\": \"+12364352078\",\n        \"interaction_id\": \"X51n17BZQTqKtxwF4mmTIw\",\n        \"agent_name\": \"Admin3\",\n        \"consumer_name\": \"+16692520210\",\n        \"channel_type\": \"AUDIO\",\n        \"associated_ids\": [\n          \"xxxx\"\n        ]\n      },\n      \"engagement_id\": \"PH6wferlTACAfBzYGAVOYg\",\n      \"direction\": \"inbound\",\n      \"disposition\": \"dev-disposition\",\n      \"silence_event_count\": 2,\n      \"crosstalk_event_count\": 2,\n      \"hold_event_count\": 3,\n      \"summary\": {\n        \"evaluation_score\": 58.63,\n        \"sentiment_score\": 58.63,\n        \"engagement_score\": 58.63\n      },\n      \"primary_language\": \"en\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "quality_management.evaluation_completed": {
      "post": {
        "tags": [
          "quality_management"
        ],
        "operationId": "quality_management.evaluation_completed",
        "requestBody": {
          "description": "# quality_management.evaluation_completed\nZoom triggers this event after the Quality Management user evaluation is completed.\n\n## Prerequisites\n\n* A Pro or higher plan.\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n\t* A valid **Event Notification Endpoint URL**\n\t* The **Quality Management evaluation completed** subscription must be enabled under the **Quality Management** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `qm_evaluations:read:admin`,`qm_evaluations:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_quality_management:read:evaluations`,`zoom_quality_management:read:evaluations:admin`\n\n**Event type**: `quality_management.evaluation_completed`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "description": "The timestamp at which the event occurred."
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
                        "description": "The account ID of the user who recorded the interaction."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the interaction.",
                        "required": [
                          "evaluation_id"
                        ],
                        "properties": {
                          "interaction_id": {
                            "type": "string",
                            "description": "The interaction's ID."
                          },
                          "agent_user_id": {
                            "type": "string",
                            "description": "The agent's ID."
                          },
                          "evaluator_user_id": {
                            "type": "string",
                            "description": "The evaluator's ID."
                          },
                          "evaluation_id": {
                            "type": "string",
                            "description": "The evaluation's ID."
                          },
                          "agent_info": {
                            "type": "object",
                            "description": "The agent information.",
                            "properties": {
                              "agent_user_id": {
                                "type": "string",
                                "description": "The agent's user ID."
                              },
                              "display_name": {
                                "type": "string",
                                "description": "The agent's name."
                              }
                            }
                          },
                          "evaluators_info": {
                            "type": "array",
                            "description": "The evaluators' information.",
                            "items": {
                              "type": "object",
                              "description": "The evaluator's information.",
                              "properties": {
                                "evaluator_user_id": {
                                  "type": "string",
                                  "description": "The evaluator's ID."
                                },
                                "display_name": {
                                  "type": "string",
                                  "description": "The evaluator's name."
                                }
                              }
                            }
                          },
                          "scorecard_name": {
                            "type": "string",
                            "description": "The scorecard's name."
                          },
                          "scoring_format": {
                            "type": "string",
                            "description": "The scoring format of the scorecard."
                          },
                          "total_possible_score": {
                            "type": "number",
                            "description": "the maximum possible score for this evaluation"
                          },
                          "evaluation_score": {
                            "type": "number",
                            "description": "The overall score of the evaluation, calculated based on the scorecard and answers."
                          },
                          "evaluation_score_ratio": {
                            "type": "number",
                            "description": "Score received by the agent, expressed as a ratio 0.00-1.00."
                          },
                          "evaluation_type": {
                            "type": "string",
                            "description": "Distinguish between automatic scoring and manual scoring.",
                            "enum": [
                              "MANUAL",
                              "AUTOMATED"
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
                  "value": "{\n  \"event\": \"quality_management.interaction_analysis_completed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"interaction_id\": \"ldl3k52kdfl3hd3\",\n      \"agent_user_id\": \"ZWR1C1FB695C654A829FE56553A709E8EBB\",\n      \"evaluator_user_id\": \"ZWR1C1FB695C654A829FE56553A709E8EBB\",\n      \"evaluation_id\": \"0e73a69f8f104d24a589fa0c615abca8\",\n      \"agent_info\": {\n        \"agent_user_id\": \"httgPnTgTxq0d65-alfYVw\",\n        \"display_name\": \"Ashlee York\"\n      },\n      \"evaluators_info\": [\n        {\n          \"evaluator_user_id\": \"b0_yDuNGSq65EF8i1f0PcQ\",\n          \"display_name\": \"Rick Arps\"\n        }\n      ],\n      \"scorecard_name\": \"Customer Care\",\n      \"scoring_format\": \"points\",\n      \"total_possible_score\": 50,\n      \"evaluation_score\": 10,\n      \"evaluation_score_ratio\": 0.2,\n      \"evaluation_type\": \"AUTOMATED\"\n    }\n  }\n}"
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
                      "description": "The timestamp at which the event occurred."
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
                          "description": "The account ID of the user who recorded the interaction."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the interaction.",
                          "required": [
                            "evaluation_id"
                          ],
                          "properties": {
                            "interaction_id": {
                              "type": "string",
                              "description": "The interaction's ID."
                            },
                            "agent_user_id": {
                              "type": "string",
                              "description": "The agent's ID."
                            },
                            "evaluator_user_id": {
                              "type": "string",
                              "description": "The evaluator's ID."
                            },
                            "evaluation_id": {
                              "type": "string",
                              "description": "The evaluation's ID."
                            },
                            "agent_info": {
                              "type": "object",
                              "description": "The agent information.",
                              "properties": {
                                "agent_user_id": {
                                  "type": "string",
                                  "description": "The agent's user ID."
                                },
                                "display_name": {
                                  "type": "string",
                                  "description": "The agent's name."
                                }
                              }
                            },
                            "evaluators_info": {
                              "type": "array",
                              "description": "The evaluators' information.",
                              "items": {
                                "type": "object",
                                "description": "The evaluator's information.",
                                "properties": {
                                  "evaluator_user_id": {
                                    "type": "string",
                                    "description": "The evaluator's ID."
                                  },
                                  "display_name": {
                                    "type": "string",
                                    "description": "The evaluator's name."
                                  }
                                }
                              }
                            },
                            "scorecard_name": {
                              "type": "string",
                              "description": "The scorecard's name."
                            },
                            "scoring_format": {
                              "type": "string",
                              "description": "The scoring format of the scorecard."
                            },
                            "total_possible_score": {
                              "type": "number",
                              "description": "the maximum possible score for this evaluation"
                            },
                            "evaluation_score": {
                              "type": "number",
                              "description": "The overall score of the evaluation, calculated based on the scorecard and answers."
                            },
                            "evaluation_score_ratio": {
                              "type": "number",
                              "description": "Score received by the agent, expressed as a ratio 0.00-1.00."
                            },
                            "evaluation_type": {
                              "type": "string",
                              "description": "Distinguish between automatic scoring and manual scoring.",
                              "enum": [
                                "MANUAL",
                                "AUTOMATED"
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
                    "value": "{\n  \"event\": \"quality_management.interaction_analysis_completed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"object\": {\n      \"interaction_id\": \"ldl3k52kdfl3hd3\",\n      \"agent_user_id\": \"ZWR1C1FB695C654A829FE56553A709E8EBB\",\n      \"evaluator_user_id\": \"ZWR1C1FB695C654A829FE56553A709E8EBB\",\n      \"evaluation_id\": \"0e73a69f8f104d24a589fa0c615abca8\",\n      \"agent_info\": {\n        \"agent_user_id\": \"httgPnTgTxq0d65-alfYVw\",\n        \"display_name\": \"Ashlee York\"\n      },\n      \"evaluators_info\": [\n        {\n          \"evaluator_user_id\": \"b0_yDuNGSq65EF8i1f0PcQ\",\n          \"display_name\": \"Rick Arps\"\n        }\n      ],\n      \"scorecard_name\": \"Customer Care\",\n      \"scoring_format\": \"points\",\n      \"total_possible_score\": 50,\n      \"evaluation_score\": 10,\n      \"evaluation_score_ratio\": 0.2,\n      \"evaluation_type\": \"AUTOMATED\"\n    }\n  }\n}"
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
      "quality-management",
      "events"
    ]
  },
  "downloadPath": "/api-hub/quality-management/events/webhooks.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": false
  },
  "productMeta": {
    "specType": "adoc",
    "slug": "/docs/api/rest/reference/quality-management/events",
    "title": "Quality Management",
    "description": "Zoom Quality Management is designed to help contact centers track and analyze customer interactions, measure agent performance, provide actionable insights, and proactively identify areas for improvement while maintaining organizational compliance.",
    "keywords": "zoom, rest, api, quality managment",
    "params": {
      "productName": "Zoom Quality Management"
    },
    "zoapEndpoint": "webhookByProduct",
    "productDisplayTitle": "Quality Management",
    "scalarSlug": "/docs/api/quality-management/events",
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "svgname": "qualityManagement"
  },
  "frontmatter": {
    "title": "Quality Management Webhooks",
    "keywords": "zoom, rest, api, quality managment",
    "description": "Zoom Quality Management is designed to help contact centers track and analyze customer interactions, measure agent performance, provide actionable insights, and proactively identify areas for improvement while maintaining organizational compliance.",
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
        "quality-management",
        "events"
      ]
    },
    "downloadPath": "/api-hub/quality-management/events/webhooks.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": false
    },
    "productMeta": {
      "specType": "adoc",
      "slug": "/docs/api/rest/reference/quality-management/events",
      "title": "Quality Management",
      "description": "Zoom Quality Management is designed to help contact centers track and analyze customer interactions, measure agent performance, provide actionable insights, and proactively identify areas for improvement while maintaining organizational compliance.",
      "keywords": "zoom, rest, api, quality managment",
      "params": {
        "productName": "Zoom Quality Management"
      },
      "zoapEndpoint": "webhookByProduct",
      "productDisplayTitle": "Quality Management",
      "scalarSlug": "/docs/api/quality-management/events",
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "svgname": "qualityManagement"
    },
    "spec": "{\"openapi\":\"3.1.0\",\"info\":{\"title\":\"Quality Management\",\"description\":\"Zoom Quality Management is designed to help contact centers track and analyze customer interactions, measure agent performance, provide actionable insights, and proactively identify areas for improvement while maintaining organizational compliance.\",\"version\":\"1.0.0\"},\"paths\":{\"quality_management.interaction_analysis_completed\":{\"post\":{\"tags\":[\"quality_management\"],\"operationId\":\"quality_management.interaction_analysis_completed\",\"requestBody\":{\"description\":\"# quality_management.interaction_analysis_completed\\nZoom triggers this event after the Quality Management users interaction analysis is completed.\\n\\n## Prerequisites\\n\\n* A Pro or higher plan.\\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\\n\\t* A valid **Event Notification Endpoint URL**\\n\\t* The **Quality Management interaction analysis completed** subscription must be enabled under the **Quality Management** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `qm_interactions:read:admin`,`qm_interactions:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_quality_management:read:interactions`,`zoom_quality_management:read:interactions:admin`\\n\\n**Event type**: `quality_management.interaction_analysis_completed`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who recorded the interaction.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the interaction.\",\"required\":[\"interaction_info\"],\"properties\":{\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the interaction.\"},\"duration\":{\"type\":\"integer\",\"description\":\"The interaction duration in seconds.\"},\"queue_id\":{\"type\":\"string\",\"description\":\"The ID of the queue.\"},\"queue_name\":{\"type\":\"string\",\"description\":\"The name of the queue.\"},\"engagement_id\":{\"type\":\"string\",\"description\":\"The engagement ID.\"},\"direction\":{\"type\":\"string\",\"description\":\"The direction.\"},\"disposition\":{\"type\":\"string\",\"description\":\"The disposition.\"},\"silence_event_count\":{\"type\":\"integer\",\"description\":\"The count of silence events\"},\"crosstalk_event_count\":{\"type\":\"integer\",\"description\":\"The count of crosstalk events.\"},\"hold_event_count\":{\"type\":\"integer\",\"description\":\"The count of hold events\"},\"primary_language\":{\"type\":\"string\",\"description\":\"The primary language.\"},\"interaction_info\":{\"type\":\"object\",\"description\":\"The interaction infomation.\",\"required\":[\"interaction_id\"],\"properties\":{\"from\":{\"type\":\"string\",\"description\":\"The caller's phone number in E.164 format.\"},\"to\":{\"type\":\"string\",\"description\":\"The recipient's phone number in E.164 format.\"},\"interaction_id\":{\"type\":\"string\",\"description\":\"The interaction ID.\"},\"agent_name\":{\"type\":\"string\",\"description\":\"The agent name.\"},\"consumer_name\":{\"type\":\"string\",\"description\":\"The consumer name.\"},\"channel_type\":{\"type\":\"string\",\"description\":\"The channel type.\",\"enum\":[\"VIDEO\",\"MANUAL\",\"SMS\",\"AUDIO\",\"WEB_CHAT\"]},\"associated_ids\":{\"type\":\"array\",\"description\":\"The associated interaction IDs.\",\"items\":{\"type\":\"string\",\"description\":\"The associated interaction ID.\"}}}},\"summary\":{\"type\":\"object\",\"description\":\"The summary information.\",\"properties\":{\"evaluation_score\":{\"type\":\"number\",\"description\":\"The evaluation score.\"},\"sentiment_score\":{\"type\":\"number\",\"description\":\"The sentiment score.\"},\"engagement_score\":{\"type\":\"number\",\"description\":\"The engagement score.\"}}}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"quality_management.interaction_analysis_completed\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"start_time\\\": \\\"2020-05-01T19:13:02Z\\\",\\n      \\\"duration\\\": 5,\\n      \\\"queue_id\\\": \\\"Je4zLPZ-SBiLCiebE35ALQ\\\",\\n      \\\"queue_name\\\": \\\"Demo\\\",\\n      \\\"interaction_info\\\": {\\n        \\\"from\\\": \\\"+16692520210\\\",\\n        \\\"to\\\": \\\"+12364352078\\\",\\n        \\\"interaction_id\\\": \\\"X51n17BZQTqKtxwF4mmTIw\\\",\\n        \\\"agent_name\\\": \\\"Admin3\\\",\\n        \\\"consumer_name\\\": \\\"+16692520210\\\",\\n        \\\"channel_type\\\": \\\"AUDIO\\\",\\n        \\\"associated_ids\\\": [\\n          \\\"xxxx\\\"\\n        ]\\n      },\\n      \\\"engagement_id\\\": \\\"PH6wferlTACAfBzYGAVOYg\\\",\\n      \\\"direction\\\": \\\"inbound\\\",\\n      \\\"disposition\\\": \\\"dev-disposition\\\",\\n      \\\"silence_event_count\\\": 2,\\n      \\\"crosstalk_event_count\\\": 2,\\n      \\\"hold_event_count\\\": 3,\\n      \\\"summary\\\": {\\n        \\\"evaluation_score\\\": 58.63,\\n        \\\"sentiment_score\\\": 58.63,\\n        \\\"engagement_score\\\": 58.63\\n      },\\n      \\\"primary_language\\\": \\\"en\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who recorded the interaction.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the interaction.\",\"required\":[\"interaction_info\"],\"properties\":{\"start_time\":{\"type\":\"string\",\"description\":\"The start time of the interaction.\"},\"duration\":{\"type\":\"integer\",\"description\":\"The interaction duration in seconds.\"},\"queue_id\":{\"type\":\"string\",\"description\":\"The ID of the queue.\"},\"queue_name\":{\"type\":\"string\",\"description\":\"The name of the queue.\"},\"engagement_id\":{\"type\":\"string\",\"description\":\"The engagement ID.\"},\"direction\":{\"type\":\"string\",\"description\":\"The direction.\"},\"disposition\":{\"type\":\"string\",\"description\":\"The disposition.\"},\"silence_event_count\":{\"type\":\"integer\",\"description\":\"The count of silence events\"},\"crosstalk_event_count\":{\"type\":\"integer\",\"description\":\"The count of crosstalk events.\"},\"hold_event_count\":{\"type\":\"integer\",\"description\":\"The count of hold events\"},\"primary_language\":{\"type\":\"string\",\"description\":\"The primary language.\"},\"interaction_info\":{\"type\":\"object\",\"description\":\"The interaction infomation.\",\"required\":[\"interaction_id\"],\"properties\":{\"from\":{\"type\":\"string\",\"description\":\"The caller's phone number in E.164 format.\"},\"to\":{\"type\":\"string\",\"description\":\"The recipient's phone number in E.164 format.\"},\"interaction_id\":{\"type\":\"string\",\"description\":\"The interaction ID.\"},\"agent_name\":{\"type\":\"string\",\"description\":\"The agent name.\"},\"consumer_name\":{\"type\":\"string\",\"description\":\"The consumer name.\"},\"channel_type\":{\"type\":\"string\",\"description\":\"The channel type.\",\"enum\":[\"VIDEO\",\"MANUAL\",\"SMS\",\"AUDIO\",\"WEB_CHAT\"]},\"associated_ids\":{\"type\":\"array\",\"description\":\"The associated interaction IDs.\",\"items\":{\"type\":\"string\",\"description\":\"The associated interaction ID.\"}}}},\"summary\":{\"type\":\"object\",\"description\":\"The summary information.\",\"properties\":{\"evaluation_score\":{\"type\":\"number\",\"description\":\"The evaluation score.\"},\"sentiment_score\":{\"type\":\"number\",\"description\":\"The sentiment score.\"},\"engagement_score\":{\"type\":\"number\",\"description\":\"The engagement score.\"}}}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"quality_management.interaction_analysis_completed\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"start_time\\\": \\\"2020-05-01T19:13:02Z\\\",\\n      \\\"duration\\\": 5,\\n      \\\"queue_id\\\": \\\"Je4zLPZ-SBiLCiebE35ALQ\\\",\\n      \\\"queue_name\\\": \\\"Demo\\\",\\n      \\\"interaction_info\\\": {\\n        \\\"from\\\": \\\"+16692520210\\\",\\n        \\\"to\\\": \\\"+12364352078\\\",\\n        \\\"interaction_id\\\": \\\"X51n17BZQTqKtxwF4mmTIw\\\",\\n        \\\"agent_name\\\": \\\"Admin3\\\",\\n        \\\"consumer_name\\\": \\\"+16692520210\\\",\\n        \\\"channel_type\\\": \\\"AUDIO\\\",\\n        \\\"associated_ids\\\": [\\n          \\\"xxxx\\\"\\n        ]\\n      },\\n      \\\"engagement_id\\\": \\\"PH6wferlTACAfBzYGAVOYg\\\",\\n      \\\"direction\\\": \\\"inbound\\\",\\n      \\\"disposition\\\": \\\"dev-disposition\\\",\\n      \\\"silence_event_count\\\": 2,\\n      \\\"crosstalk_event_count\\\": 2,\\n      \\\"hold_event_count\\\": 3,\\n      \\\"summary\\\": {\\n        \\\"evaluation_score\\\": 58.63,\\n        \\\"sentiment_score\\\": 58.63,\\n        \\\"engagement_score\\\": 58.63\\n      },\\n      \\\"primary_language\\\": \\\"en\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"quality_management.evaluation_completed\":{\"post\":{\"tags\":[\"quality_management\"],\"operationId\":\"quality_management.evaluation_completed\",\"requestBody\":{\"description\":\"# quality_management.evaluation_completed\\nZoom triggers this event after the Quality Management user evaluation is completed.\\n\\n## Prerequisites\\n\\n* A Pro or higher plan.\\n* Enable **Event Subscriptions** for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\\n\\t* A valid **Event Notification Endpoint URL**\\n\\t* The **Quality Management evaluation completed** subscription must be enabled under the **Quality Management** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `qm_evaluations:read:admin`,`qm_evaluations:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `zoom_quality_management:read:evaluations`,`zoom_quality_management:read:evaluations:admin`\\n\\n**Event type**: `quality_management.evaluation_completed`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who recorded the interaction.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the interaction.\",\"required\":[\"evaluation_id\"],\"properties\":{\"interaction_id\":{\"type\":\"string\",\"description\":\"The interaction's ID.\"},\"agent_user_id\":{\"type\":\"string\",\"description\":\"The agent's ID.\"},\"evaluator_user_id\":{\"type\":\"string\",\"description\":\"The evaluator's ID.\"},\"evaluation_id\":{\"type\":\"string\",\"description\":\"The evaluation's ID.\"},\"agent_info\":{\"type\":\"object\",\"description\":\"The agent information.\",\"properties\":{\"agent_user_id\":{\"type\":\"string\",\"description\":\"The agent's user ID.\"},\"display_name\":{\"type\":\"string\",\"description\":\"The agent's name.\"}}},\"evaluators_info\":{\"type\":\"array\",\"description\":\"The evaluators' information.\",\"items\":{\"type\":\"object\",\"description\":\"The evaluator's information.\",\"properties\":{\"evaluator_user_id\":{\"type\":\"string\",\"description\":\"The evaluator's ID.\"},\"display_name\":{\"type\":\"string\",\"description\":\"The evaluator's name.\"}}}},\"scorecard_name\":{\"type\":\"string\",\"description\":\"The scorecard's name.\"},\"scoring_format\":{\"type\":\"string\",\"description\":\"The scoring format of the scorecard.\"},\"total_possible_score\":{\"type\":\"number\",\"description\":\"the maximum possible score for this evaluation\"},\"evaluation_score\":{\"type\":\"number\",\"description\":\"The overall score of the evaluation, calculated based on the scorecard and answers.\"},\"evaluation_score_ratio\":{\"type\":\"number\",\"description\":\"Score received by the agent, expressed as a ratio 0.00-1.00.\"},\"evaluation_type\":{\"type\":\"string\",\"description\":\"Distinguish between automatic scoring and manual scoring.\",\"enum\":[\"MANUAL\",\"AUTOMATED\"]}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"quality_management.interaction_analysis_completed\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"interaction_id\\\": \\\"ldl3k52kdfl3hd3\\\",\\n      \\\"agent_user_id\\\": \\\"ZWR1C1FB695C654A829FE56553A709E8EBB\\\",\\n      \\\"evaluator_user_id\\\": \\\"ZWR1C1FB695C654A829FE56553A709E8EBB\\\",\\n      \\\"evaluation_id\\\": \\\"0e73a69f8f104d24a589fa0c615abca8\\\",\\n      \\\"agent_info\\\": {\\n        \\\"agent_user_id\\\": \\\"httgPnTgTxq0d65-alfYVw\\\",\\n        \\\"display_name\\\": \\\"Ashlee York\\\"\\n      },\\n      \\\"evaluators_info\\\": [\\n        {\\n          \\\"evaluator_user_id\\\": \\\"b0_yDuNGSq65EF8i1f0PcQ\\\",\\n          \\\"display_name\\\": \\\"Rick Arps\\\"\\n        }\\n      ],\\n      \\\"scorecard_name\\\": \\\"Customer Care\\\",\\n      \\\"scoring_format\\\": \\\"points\\\",\\n      \\\"total_possible_score\\\": 50,\\n      \\\"evaluation_score\\\": 10,\\n      \\\"evaluation_score_ratio\\\": 0.2,\\n      \\\"evaluation_type\\\": \\\"AUTOMATED\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who recorded the interaction.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the interaction.\",\"required\":[\"evaluation_id\"],\"properties\":{\"interaction_id\":{\"type\":\"string\",\"description\":\"The interaction's ID.\"},\"agent_user_id\":{\"type\":\"string\",\"description\":\"The agent's ID.\"},\"evaluator_user_id\":{\"type\":\"string\",\"description\":\"The evaluator's ID.\"},\"evaluation_id\":{\"type\":\"string\",\"description\":\"The evaluation's ID.\"},\"agent_info\":{\"type\":\"object\",\"description\":\"The agent information.\",\"properties\":{\"agent_user_id\":{\"type\":\"string\",\"description\":\"The agent's user ID.\"},\"display_name\":{\"type\":\"string\",\"description\":\"The agent's name.\"}}},\"evaluators_info\":{\"type\":\"array\",\"description\":\"The evaluators' information.\",\"items\":{\"type\":\"object\",\"description\":\"The evaluator's information.\",\"properties\":{\"evaluator_user_id\":{\"type\":\"string\",\"description\":\"The evaluator's ID.\"},\"display_name\":{\"type\":\"string\",\"description\":\"The evaluator's name.\"}}}},\"scorecard_name\":{\"type\":\"string\",\"description\":\"The scorecard's name.\"},\"scoring_format\":{\"type\":\"string\",\"description\":\"The scoring format of the scorecard.\"},\"total_possible_score\":{\"type\":\"number\",\"description\":\"the maximum possible score for this evaluation\"},\"evaluation_score\":{\"type\":\"number\",\"description\":\"The overall score of the evaluation, calculated based on the scorecard and answers.\"},\"evaluation_score_ratio\":{\"type\":\"number\",\"description\":\"Score received by the agent, expressed as a ratio 0.00-1.00.\"},\"evaluation_type\":{\"type\":\"string\",\"description\":\"Distinguish between automatic scoring and manual scoring.\",\"enum\":[\"MANUAL\",\"AUTOMATED\"]}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"quality_management.interaction_analysis_completed\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"object\\\": {\\n      \\\"interaction_id\\\": \\\"ldl3k52kdfl3hd3\\\",\\n      \\\"agent_user_id\\\": \\\"ZWR1C1FB695C654A829FE56553A709E8EBB\\\",\\n      \\\"evaluator_user_id\\\": \\\"ZWR1C1FB695C654A829FE56553A709E8EBB\\\",\\n      \\\"evaluation_id\\\": \\\"0e73a69f8f104d24a589fa0c615abca8\\\",\\n      \\\"agent_info\\\": {\\n        \\\"agent_user_id\\\": \\\"httgPnTgTxq0d65-alfYVw\\\",\\n        \\\"display_name\\\": \\\"Ashlee York\\\"\\n      },\\n      \\\"evaluators_info\\\": [\\n        {\\n          \\\"evaluator_user_id\\\": \\\"b0_yDuNGSq65EF8i1f0PcQ\\\",\\n          \\\"display_name\\\": \\\"Rick Arps\\\"\\n        }\\n      ],\\n      \\\"scorecard_name\\\": \\\"Customer Care\\\",\\n      \\\"scoring_format\\\": \\\"points\\\",\\n      \\\"total_possible_score\\\": 50,\\n      \\\"evaluation_score\\\": 10,\\n      \\\"evaluation_score_ratio\\\": 0.2,\\n      \\\"evaluation_type\\\": \\\"AUTOMATED\\\"\\n    }\\n  }\\n}\"}}}}}}}}},\"servers\":[{\"url\":\"https://{your-endpoint-url}\"}]}",
    "frontmatter": {
      "title": "Quality Management Webhooks",
      "keywords": "zoom, rest, api, quality managment",
      "description": "Zoom Quality Management is designed to help contact centers track and analyze customer interactions, measure agent performance, provide actionable insights, and proactively identify areas for improvement while maintaining organizational compliance.",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
