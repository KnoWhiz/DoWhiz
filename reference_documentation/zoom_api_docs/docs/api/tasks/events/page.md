# Tasks Webhooks

- Source URL: https://developers.zoom.us/docs/api/tasks/events/
- Snapshot path: docs/api/tasks/events
- Generated (UTC): 2026-02-07T22:40:11.652532+00:00

## Frontmatter

```json
{
  "title": "Tasks Webhooks",
  "keywords": "zoom, rest, api, ai, aic, tasks",
  "description": "Tasks events",
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
    "title": "Tasks",
    "description": "",
    "version": "1.0.0"
  },
  "paths": {
    "task.deleted": {
      "post": {
        "tags": [
          "task"
        ],
        "operationId": "task.deleted",
        "requestBody": {
          "description": "# task.deleted\n\nThe **Task deleted** event is triggered every time a task is delete by one of your app users or account users.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Task deleted** subscription enabled under the **Task** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:admin`,`tasks:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:task`,`tasks:read:task:admin`\n\n**Event type**: `task.deleted`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "description": "The timestamp of when the event occurred."
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
                        "description": "The account ID of the user who deleted the task."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The email of the user who deleted the task."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user who deleted the task."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the task",
                        "required": [
                          "task_id"
                        ],
                        "properties": {
                          "task_id": {
                            "type": "string",
                            "description": "The task ID."
                          },
                          "description": {
                            "type": "string",
                            "description": "The detailed description of the task."
                          },
                          "title": {
                            "type": "string",
                            "description": "The title of the task."
                          },
                          "priority": {
                            "$ref": "#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/PriorityId-Tasks"
                          },
                          "status": {
                            "$ref": "#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/StatusId-Tasks"
                          },
                          "is_public": {
                            "$ref": "#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/IsPublic-Tasks"
                          },
                          "due_date": {
                            "$ref": "#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/DueDate-Tasks"
                          }
                        },
                        "additionalProperties": true
                      }
                    },
                    "additionalProperties": true
                  }
                },
                "additionalProperties": true,
                "components": {
                  "schemas": {
                    "PriorityId-Tasks": {
                      "type": "string",
                      "description": "The priority level of the task.",
                      "example": "Medium",
                      "default": "Medium",
                      "enum": [
                        "Low",
                        "Medium",
                        "High",
                        "Highest"
                      ]
                    },
                    "StatusId-Tasks": {
                      "type": "string",
                      "description": "The current status of the task.",
                      "example": "To do",
                      "default": "To do",
                      "enum": [
                        "To do",
                        "In progress",
                        "Done",
                        "Blocked",
                        "Recommended"
                      ]
                    },
                    "DueDate-Tasks": {
                      "type": "string",
                      "description": "The due date of the task in UTC.\n Format: yyyy-mm-dd.",
                      "format": "date",
                      "example": "2025-09-24"
                    },
                    "IsPublic-Tasks": {
                      "type": "boolean",
                      "description": "Determines the task\u2019s visibility.\ntrue: The task is accessible to all members of your organization.\nfalse: The task is accessible only to the assigned users and collaborators.",
                      "example": false,
                      "default": false
                    }
                  },
                  "parameters": {}
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"task.deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"task_id\": \"12321312323\",\n      \"title\": \"Complete project documentation\",\n      \"description\": \"Write comprehensive documentation for the new API endpoints\",\n      \"status\": \"To do\",\n      \"priority\": \"Medium\",\n      \"is_public\": true,\n      \"due_date\": \"2025-08-22\"\n    }\n  }\n}"
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
                      "description": "The timestamp of when the event occurred."
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
                          "description": "The account ID of the user who deleted the task."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The email of the user who deleted the task."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user who deleted the task."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the task",
                          "required": [
                            "task_id"
                          ],
                          "properties": {
                            "task_id": {
                              "type": "string",
                              "description": "The task ID."
                            },
                            "description": {
                              "type": "string",
                              "description": "The detailed description of the task."
                            },
                            "title": {
                              "type": "string",
                              "description": "The title of the task."
                            },
                            "priority": {
                              "$ref": "#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/PriorityId-Tasks"
                            },
                            "status": {
                              "$ref": "#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/StatusId-Tasks"
                            },
                            "is_public": {
                              "$ref": "#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/IsPublic-Tasks"
                            },
                            "due_date": {
                              "$ref": "#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/DueDate-Tasks"
                            }
                          },
                          "additionalProperties": true
                        }
                      },
                      "additionalProperties": true
                    }
                  },
                  "additionalProperties": true,
                  "components": {
                    "schemas": {
                      "PriorityId-Tasks": {
                        "type": "string",
                        "description": "The priority level of the task.",
                        "example": "Medium",
                        "default": "Medium",
                        "enum": [
                          "Low",
                          "Medium",
                          "High",
                          "Highest"
                        ]
                      },
                      "StatusId-Tasks": {
                        "type": "string",
                        "description": "The current status of the task.",
                        "example": "To do",
                        "default": "To do",
                        "enum": [
                          "To do",
                          "In progress",
                          "Done",
                          "Blocked",
                          "Recommended"
                        ]
                      },
                      "DueDate-Tasks": {
                        "type": "string",
                        "description": "The due date of the task in UTC.\n Format: yyyy-mm-dd.",
                        "format": "date",
                        "example": "2025-09-24"
                      },
                      "IsPublic-Tasks": {
                        "type": "boolean",
                        "description": "Determines the task\u2019s visibility.\ntrue: The task is accessible to all members of your organization.\nfalse: The task is accessible only to the assigned users and collaborators.",
                        "example": false,
                        "default": false
                      }
                    },
                    "parameters": {}
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"task.deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"task_id\": \"12321312323\",\n      \"title\": \"Complete project documentation\",\n      \"description\": \"Write comprehensive documentation for the new API endpoints\",\n      \"status\": \"To do\",\n      \"priority\": \"Medium\",\n      \"is_public\": true,\n      \"due_date\": \"2025-08-22\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "task.updated": {
      "post": {
        "tags": [
          "task"
        ],
        "operationId": "task.updated",
        "requestBody": {
          "description": "# task.updated\n\nThe **Task was updated** event is triggered every time a task is updated.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Task was updated** subscription enabled under the **Task** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:admin`,`tasks:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:task`,`tasks:read:task:admin`\n\n**Event type**: `task.updated`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "description": "The timestamp of when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "operator",
                      "operator_id",
                      "task_id"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who updated the task."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The email of the user who updated the task."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": " The Zoom user ID of the operator who closed the task. \n It should be null if the user is external to the task's organization."
                      },
                      "task_id": {
                        "type": "string",
                        "description": "The task ID."
                      },
                      "field_changes": {
                        "type": "array",
                        "description": "The list of field changes in this update.",
                        "items": {
                          "type": "object",
                          "properties": {
                            "field_name": {
                              "type": "string",
                              "enum": [
                                "title",
                                "status",
                                "priority",
                                "is_public",
                                "due_date",
                                "starred",
                                "description"
                              ],
                              "description": "The name of the field that was changed."
                            },
                            "old_value": {
                              "type": "string",
                              "description": "The previous value of the field."
                            },
                            "new_value": {
                              "type": "string",
                              "description": "The new value of the field."
                            },
                            "operation_type": {
                              "type": "string",
                              "enum": [
                                "added",
                                "removed",
                                "updated"
                              ],
                              "description": "The operation performed on the field."
                            },
                            "changed_at": {
                              "type": "integer",
                              "description": "The unix timestamp when the field was changed."
                            }
                          },
                          "required": [
                            "field_name",
                            "operation_type",
                            "changed_at"
                          ]
                        }
                      },
                      "file_changes": {
                        "type": "array",
                        "description": "The list of file changes in this update.",
                        "items": {
                          "type": "object",
                          "properties": {
                            "file_id": {
                              "type": "string",
                              "description": "The unique identifier of the file."
                            },
                            "file_name": {
                              "type": "string",
                              "description": "The name of the file."
                            },
                            "file_type": {
                              "type": "string",
                              "description": "The MIME type of the file."
                            },
                            "file_size": {
                              "type": "integer",
                              "description": "The size of the file in bytes."
                            },
                            "file_download_link": {
                              "type": "string",
                              "description": "The temporary download link for the file."
                            },
                            "download_link_expire_time": {
                              "type": "integer",
                              "description": "The unix timestamp when the download link expires."
                            },
                            "operation_type": {
                              "type": "string",
                              "enum": [
                                "added",
                                "removed"
                              ],
                              "description": "The operation performed on the attachment."
                            },
                            "changed_at": {
                              "type": "integer",
                              "description": "The unix timestamp when the attachment was changed."
                            }
                          },
                          "required": [
                            "file_id",
                            "file_name",
                            "operation_type",
                            "changed_at"
                          ]
                        }
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
                  "value": "{\n  \"event\": \"task.updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n    \"field_changes\": [\n      {\n        \"field_name\": \"title\",\n        \"old_value\": \"Test Plan\",\n        \"new_value\": \"New Test Plan\",\n        \"operation_type\": \"updated\",\n        \"changed_at\": 1758184629\n      }\n    ],\n    \"file_changes\": [\n      {\n        \"file_id\": \"file_456789\",\n        \"file_name\": \"project_requirements.pdf\",\n        \"file_type\": \"application/pdf\",\n        \"file_size\": 2048576,\n        \"file_download_link\": \"https://api.zoom.us/v2/files/file_456789/download?token=temp_token_123\",\n        \"download_link_expire_time\": 1703209856789,\n        \"operation_type\": \"added\",\n        \"changed_at\": 1703123456789\n      }\n    ]\n  }\n}"
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
                      "description": "The timestamp of when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "operator",
                        "operator_id",
                        "task_id"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who updated the task."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The email of the user who updated the task."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": " The Zoom user ID of the operator who closed the task. \n It should be null if the user is external to the task's organization."
                        },
                        "task_id": {
                          "type": "string",
                          "description": "The task ID."
                        },
                        "field_changes": {
                          "type": "array",
                          "description": "The list of field changes in this update.",
                          "items": {
                            "type": "object",
                            "properties": {
                              "field_name": {
                                "type": "string",
                                "enum": [
                                  "title",
                                  "status",
                                  "priority",
                                  "is_public",
                                  "due_date",
                                  "starred",
                                  "description"
                                ],
                                "description": "The name of the field that was changed."
                              },
                              "old_value": {
                                "type": "string",
                                "description": "The previous value of the field."
                              },
                              "new_value": {
                                "type": "string",
                                "description": "The new value of the field."
                              },
                              "operation_type": {
                                "type": "string",
                                "enum": [
                                  "added",
                                  "removed",
                                  "updated"
                                ],
                                "description": "The operation performed on the field."
                              },
                              "changed_at": {
                                "type": "integer",
                                "description": "The unix timestamp when the field was changed."
                              }
                            },
                            "required": [
                              "field_name",
                              "operation_type",
                              "changed_at"
                            ]
                          }
                        },
                        "file_changes": {
                          "type": "array",
                          "description": "The list of file changes in this update.",
                          "items": {
                            "type": "object",
                            "properties": {
                              "file_id": {
                                "type": "string",
                                "description": "The unique identifier of the file."
                              },
                              "file_name": {
                                "type": "string",
                                "description": "The name of the file."
                              },
                              "file_type": {
                                "type": "string",
                                "description": "The MIME type of the file."
                              },
                              "file_size": {
                                "type": "integer",
                                "description": "The size of the file in bytes."
                              },
                              "file_download_link": {
                                "type": "string",
                                "description": "The temporary download link for the file."
                              },
                              "download_link_expire_time": {
                                "type": "integer",
                                "description": "The unix timestamp when the download link expires."
                              },
                              "operation_type": {
                                "type": "string",
                                "enum": [
                                  "added",
                                  "removed"
                                ],
                                "description": "The operation performed on the attachment."
                              },
                              "changed_at": {
                                "type": "integer",
                                "description": "The unix timestamp when the attachment was changed."
                              }
                            },
                            "required": [
                              "file_id",
                              "file_name",
                              "operation_type",
                              "changed_at"
                            ]
                          }
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
                    "value": "{\n  \"event\": \"task.updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n    \"field_changes\": [\n      {\n        \"field_name\": \"title\",\n        \"old_value\": \"Test Plan\",\n        \"new_value\": \"New Test Plan\",\n        \"operation_type\": \"updated\",\n        \"changed_at\": 1758184629\n      }\n    ],\n    \"file_changes\": [\n      {\n        \"file_id\": \"file_456789\",\n        \"file_name\": \"project_requirements.pdf\",\n        \"file_type\": \"application/pdf\",\n        \"file_size\": 2048576,\n        \"file_download_link\": \"https://api.zoom.us/v2/files/file_456789/download?token=temp_token_123\",\n        \"download_link_expire_time\": 1703209856789,\n        \"operation_type\": \"added\",\n        \"changed_at\": 1703123456789\n      }\n    ]\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "task.comment_updated": {
      "post": {
        "tags": [
          "task"
        ],
        "operationId": "task.comment_updated",
        "requestBody": {
          "description": "# task.comment_updated\n\nThe **Task comment was updated** event is triggered every time a task comment is updated to task.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Task comment was updated** subscription enabled under the **Task** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_comment:read`,`tasks_comment:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:comments`,`tasks:read:comments:admin`\n\n**Event type**: `task.comment_updated`\n",
          "content": {
            "application/json": {
              "schema": {
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
                      "operation_type",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who updated the comment with a task."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The email of the user who updated the comment with a task."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user who updated the comment with a task."
                      },
                      "operation_type": {
                        "type": "integer",
                        "description": "Operation type. \n\n*0 - REMOVE \n*1 - ADD \n*2 - UPDATED",
                        "enum": [
                          0,
                          1,
                          2
                        ],
                        "x-enum-descriptions": [
                          "REMOVE",
                          "ADD",
                          "UPDATED"
                        ]
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the task.",
                        "required": [
                          "task_id",
                          "comment"
                        ],
                        "properties": {
                          "task_id": {
                            "type": "string",
                            "description": "The task ID."
                          },
                          "comment": {
                            "type": "object",
                            "description": "The comments that had been added, removed, or updated.",
                            "required": [
                              "content_text",
                              "comment_id"
                            ],
                            "properties": {
                              "content_text": {
                                "type": "string",
                                "description": "The task comment content."
                              },
                              "comment_id": {
                                "type": "string",
                                "description": "The task comment ID."
                              }
                            },
                            "additionalProperties": false
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
                  "value": "{\n  \"event\": \"task.comment_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operation_type\": 1,\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"comment\": {\n        \"comment_id\": \"472a9f05-d46d-44bb-bf5a-93e731d1f6d0\",\n        \"content_text\": \"Jack update comment\"\n      }\n    }\n  }\n}"
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
                        "operation_type",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who updated the comment with a task."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The email of the user who updated the comment with a task."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user who updated the comment with a task."
                        },
                        "operation_type": {
                          "type": "integer",
                          "description": "Operation type. \n\n*0 - REMOVE \n*1 - ADD \n*2 - UPDATED",
                          "enum": [
                            0,
                            1,
                            2
                          ],
                          "x-enum-descriptions": [
                            "REMOVE",
                            "ADD",
                            "UPDATED"
                          ]
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the task.",
                          "required": [
                            "task_id",
                            "comment"
                          ],
                          "properties": {
                            "task_id": {
                              "type": "string",
                              "description": "The task ID."
                            },
                            "comment": {
                              "type": "object",
                              "description": "The comments that had been added, removed, or updated.",
                              "required": [
                                "content_text",
                                "comment_id"
                              ],
                              "properties": {
                                "content_text": {
                                  "type": "string",
                                  "description": "The task comment content."
                                },
                                "comment_id": {
                                  "type": "string",
                                  "description": "The task comment ID."
                                }
                              },
                              "additionalProperties": false
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
                    "value": "{\n  \"event\": \"task.comment_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operation_type\": 1,\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"comment\": {\n        \"comment_id\": \"472a9f05-d46d-44bb-bf5a-93e731d1f6d0\",\n        \"content_text\": \"Jack update comment\"\n      }\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "task.completed": {
      "post": {
        "tags": [
          "task"
        ],
        "operationId": "task.completed",
        "requestBody": {
          "description": "# task.completed\n\nThe **task was completed** event is triggered every time a task status is done.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Task was completed** subscription enabled under the **Task** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:admin`,`tasks:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:task`,`tasks:read:task:admin`\n\n**Event type**: `task.completed`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "description": "A timestamp of when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "operator",
                      "object",
                      "operator_id"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who completed the task."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The email of the user who completed the task."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The Zoom user ID of the operator who completed the task."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the task.",
                        "required": [
                          "task_id",
                          "status"
                        ],
                        "properties": {
                          "task_id": {
                            "type": "string",
                            "description": "The task ID."
                          },
                          "status": {
                            "$ref": "#/paths/task.completed/post/requestBody/content/application~1json/schema/components/schemas/StatusId-Tasks"
                          }
                        },
                        "additionalProperties": false
                      }
                    },
                    "additionalProperties": false
                  }
                },
                "additionalProperties": false,
                "components": {
                  "schemas": {
                    "StatusId-Tasks": {
                      "type": "string",
                      "description": "The current status of the task.",
                      "example": "To do",
                      "default": "To do",
                      "enum": [
                        "To do",
                        "In progress",
                        "Done",
                        "Blocked",
                        "Recommended"
                      ]
                    }
                  },
                  "parameters": {}
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"task.completed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"status\": \"Done\"\n    }\n  }\n}"
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
                      "description": "A timestamp of when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "operator",
                        "object",
                        "operator_id"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who completed the task."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The email of the user who completed the task."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The Zoom user ID of the operator who completed the task."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the task.",
                          "required": [
                            "task_id",
                            "status"
                          ],
                          "properties": {
                            "task_id": {
                              "type": "string",
                              "description": "The task ID."
                            },
                            "status": {
                              "$ref": "#/paths/task.completed/post/requestBody/content/application~1json/schema/components/schemas/StatusId-Tasks"
                            }
                          },
                          "additionalProperties": false
                        }
                      },
                      "additionalProperties": false
                    }
                  },
                  "additionalProperties": false,
                  "components": {
                    "schemas": {
                      "StatusId-Tasks": {
                        "type": "string",
                        "description": "The current status of the task.",
                        "example": "To do",
                        "default": "To do",
                        "enum": [
                          "To do",
                          "In progress",
                          "Done",
                          "Blocked",
                          "Recommended"
                        ]
                      }
                    },
                    "parameters": {}
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"task.completed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"status\": \"Done\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "task.collaborator_updated": {
      "post": {
        "tags": [
          "task"
        ],
        "operationId": "task.collaborator_updated",
        "requestBody": {
          "description": "# task.collaborator_updated\n\nThe **Task collaborator were updated** event is triggered every time a task collaborator is added or deleted to share a task with a Zoom user\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Task collaborator were updated** subscription enabled under the **Task** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_collaborator:read`,`tasks_collaborator:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:list_collaborators`,`tasks:read:list_collaborators:admin`\n\n**Event type**: `task.collaborator_updated`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "description": "A timestamp of when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "operator",
                      "operator_id",
                      "operation_type",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who updated the collaborators of the task."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The email of the user who updated the collaborators of the task."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The Zoom user ID of the operator who updated the collaborators of the task."
                      },
                      "operation_type": {
                        "type": "integer",
                        "description": "Operation type. \nEnum: \n*0 - REMOVE \n*1 - ADD \n",
                        "enum": [
                          0,
                          1
                        ],
                        "x-enum-descriptions": [
                          "REMOVE",
                          "ADDED"
                        ]
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the task.",
                        "required": [
                          "task_id",
                          "collaborators"
                        ],
                        "properties": {
                          "task_id": {
                            "type": "string",
                            "description": "The task ID."
                          },
                          "collaborators": {
                            "type": "array",
                            "description": "Collaborators that has been added or removed.",
                            "items": {
                              "anyOf": [
                                {
                                  "type": "object",
                                  "required": [
                                    "user_id",
                                    "display_name"
                                  ],
                                  "properties": {
                                    "user_id": {
                                      "type": "string",
                                      "description": "The Zoom user ID of the collaborator. "
                                    },
                                    "display_name": {
                                      "type": "string",
                                      "description": "The display name of the collaborator."
                                    },
                                    "email": {
                                      "type": "string",
                                      "format": "email",
                                      "description": "The email address of the collaborator."
                                    },
                                    "avatar": {
                                      "type": "string",
                                      "format": "uri",
                                      "description": "The avatar image URL of the collaborator."
                                    }
                                  },
                                  "additionalProperties": false
                                }
                              ]
                            },
                            "additionalProperties": false
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
                  "value": "{\n  \"event\": \"task.collaborator_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operation_type\": 1,\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"collaborators\": [\n        {\n          \"user_id\": \"CcrEGgmeQem1uyJsuIRKwA\",\n          \"display_name\": \"John Doe\",\n          \"email\": \"john.doe@example.com\",\n          \"avatar\": \"https://example.com/avatar.jpg\"\n        }\n      ]\n    }\n  }\n}"
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
                      "description": "A timestamp of when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "operator",
                        "operator_id",
                        "operation_type",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who updated the collaborators of the task."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The email of the user who updated the collaborators of the task."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The Zoom user ID of the operator who updated the collaborators of the task."
                        },
                        "operation_type": {
                          "type": "integer",
                          "description": "Operation type. \nEnum: \n*0 - REMOVE \n*1 - ADD \n",
                          "enum": [
                            0,
                            1
                          ],
                          "x-enum-descriptions": [
                            "REMOVE",
                            "ADDED"
                          ]
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the task.",
                          "required": [
                            "task_id",
                            "collaborators"
                          ],
                          "properties": {
                            "task_id": {
                              "type": "string",
                              "description": "The task ID."
                            },
                            "collaborators": {
                              "type": "array",
                              "description": "Collaborators that has been added or removed.",
                              "items": {
                                "anyOf": [
                                  {
                                    "type": "object",
                                    "required": [
                                      "user_id",
                                      "display_name"
                                    ],
                                    "properties": {
                                      "user_id": {
                                        "type": "string",
                                        "description": "The Zoom user ID of the collaborator. "
                                      },
                                      "display_name": {
                                        "type": "string",
                                        "description": "The display name of the collaborator."
                                      },
                                      "email": {
                                        "type": "string",
                                        "format": "email",
                                        "description": "The email address of the collaborator."
                                      },
                                      "avatar": {
                                        "type": "string",
                                        "format": "uri",
                                        "description": "The avatar image URL of the collaborator."
                                      }
                                    },
                                    "additionalProperties": false
                                  }
                                ]
                              },
                              "additionalProperties": false
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
                    "value": "{\n  \"event\": \"task.collaborator_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operation_type\": 1,\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"collaborators\": [\n        {\n          \"user_id\": \"CcrEGgmeQem1uyJsuIRKwA\",\n          \"display_name\": \"John Doe\",\n          \"email\": \"john.doe@example.com\",\n          \"avatar\": \"https://example.com/avatar.jpg\"\n        }\n      ]\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "task.created": {
      "post": {
        "tags": [
          "task"
        ],
        "operationId": "task.created",
        "requestBody": {
          "description": "# task.created\n\nThe **Task was created** event is triggered every time a task is created by one of your app users or account users.\n\nZoom offers multiple methods for users to create a new task. Users can manually create tasks in the Zoom client or the [Zoom web portal](https://zoom.us/). Users can also programmatically create tasks via the [**Create a new task**] API.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Task was created** subscription enabled under the **Task** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:admin`,`tasks:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:task`,`tasks:read:task:admin`\n\n**Event type**: `task.created`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "description": "A timestamp of when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "operator",
                      "account_id",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who created the task."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The email of the user who created the task."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user who created the task."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the task.",
                        "required": [
                          "create_time",
                          "is_public",
                          "priority",
                          "starred",
                          "status",
                          "task_id",
                          "title",
                          "source_type"
                        ],
                        "properties": {
                          "task_id": {
                            "type": "string",
                            "description": "The task ID."
                          },
                          "description": {
                            "type": "string",
                            "description": "The task's detailed description."
                          },
                          "title": {
                            "type": "string",
                            "description": "The task's title."
                          },
                          "priority": {
                            "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/PriorityId-Tasks"
                          },
                          "status": {
                            "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/StatusId-Tasks"
                          },
                          "is_public": {
                            "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/IsPublic-Tasks"
                          },
                          "due_date": {
                            "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/DueDate-Tasks"
                          },
                          "source_type": {
                            "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/source_type-Tasks"
                          },
                          "starred": {
                            "type": "boolean",
                            "description": "Indicates whether the task is marked as starred."
                          },
                          "collaborators": {
                            "type": "array",
                            "description": "A list of collaborators to invite to the task. Maximum user count is 20.",
                            "items": {
                              "type": "object",
                              "required": [
                                "user_id",
                                "email"
                              ],
                              "properties": {
                                "user_id": {
                                  "type": "string",
                                  "description": "The collaborator's user to invite. The invited user must belong to the same account as the requesting user."
                                },
                                "email": {
                                  "type": "string",
                                  "format": "email",
                                  "description": "The collaborator's email address."
                                }
                              }
                            }
                          },
                          "create_time": {
                            "type": "string",
                            "description": "The creation date-time in UTC/GMT. ",
                            "format": "date-time",
                            "example": "2025-03-25T07:29:29Z"
                          },
                          "friendly_task_id": {
                            "type": "string",
                            "description": "The task's human-readable identifier.",
                            "example": "Task-1"
                          },
                          "link": {
                            "type": "string",
                            "description": "The direct URL link to the task.",
                            "example": "https://tasks.example.com/ws53iQgOOkOdzVTxZaVoRA/TASK-1"
                          },
                          "assignees": {
                            "type": "array",
                            "description": "A list of assignees to add to the task.",
                            "items": {
                              "required": [
                                "user_id",
                                "email"
                              ],
                              "type": "object",
                              "properties": {
                                "user_id": {
                                  "type": "string",
                                  "description": "The assignee's Zoom user ID."
                                },
                                "email": {
                                  "type": "string",
                                  "format": "email",
                                  "description": "The assignee's email address."
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
                "additionalProperties": false,
                "components": {
                  "schemas": {
                    "PriorityId-Tasks": {
                      "type": "string",
                      "description": "The task's priority level.",
                      "example": "Medium",
                      "default": "Medium",
                      "enum": [
                        "Low",
                        "Medium",
                        "High",
                        "Highest"
                      ]
                    },
                    "StatusId-Tasks": {
                      "type": "string",
                      "description": "The task's current status.",
                      "example": "To do",
                      "default": "To do",
                      "enum": [
                        "To do",
                        "In progress",
                        "Done",
                        "Blocked",
                        "Recommended"
                      ]
                    },
                    "DueDate-Tasks": {
                      "type": "string",
                      "description": "The task's due date in UTC, formatted as yyyy-MM-dd.",
                      "format": "date",
                      "example": "2025-09-24"
                    },
                    "IsPublic-Tasks": {
                      "type": "boolean",
                      "description": "The task's visibility.\n- true - The task is accessible to all members of your organization.\n- false - The task is accessible only to the assigned users and collaborators.",
                      "example": false,
                      "default": false
                    },
                    "source_type-Tasks": {
                      "type": "string",
                      "description": "Type of the source that created the task.",
                      "example": "user",
                      "enum": [
                        "user",
                        "aic_meeting_summary",
                        "whiteboard",
                        "notes",
                        "docs",
                        "phone",
                        "chat",
                        "email",
                        "meeting",
                        "aic"
                      ]
                    }
                  },
                  "parameters": {}
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"task.created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"task_id\": \"12321312323\",\n      \"title\": \"Complete project documentation\",\n      \"description\": \"Write comprehensive documentation for the new API endpoints\",\n      \"status\": \"To do\",\n      \"priority\": \"Medium\",\n      \"is_public\": true,\n      \"due_date\": \"2025-08-22\",\n      \"source_type\": \"user\",\n      \"starred\": false,\n      \"collaborators\": [\n        {\n          \"user_id\": \"w0RChiauQeqRlv5fgxYULQ\",\n          \"email\": \"userB@example.com\"\n        }\n      ],\n      \"create_time\": \"2025-03-25T07:29:29Z\",\n      \"friendly_task_id\": \"Task-1\",\n      \"link\": \"https://tasks.zoomdev.us/ws53iQgOOkOdzVTxZaVoRA/TASK-1\",\n      \"assignees\": [\n        {\n          \"user_id\": \"w0RChiauQeqRlv5fgxYULQ\",\n          \"email\": \"userB@example.com\"\n        }\n      ]\n    }\n  }\n}"
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
                      "description": "A timestamp of when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "operator",
                        "account_id",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who created the task."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The email of the user who created the task."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user who created the task."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the task.",
                          "required": [
                            "create_time",
                            "is_public",
                            "priority",
                            "starred",
                            "status",
                            "task_id",
                            "title",
                            "source_type"
                          ],
                          "properties": {
                            "task_id": {
                              "type": "string",
                              "description": "The task ID."
                            },
                            "description": {
                              "type": "string",
                              "description": "The task's detailed description."
                            },
                            "title": {
                              "type": "string",
                              "description": "The task's title."
                            },
                            "priority": {
                              "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/PriorityId-Tasks"
                            },
                            "status": {
                              "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/StatusId-Tasks"
                            },
                            "is_public": {
                              "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/IsPublic-Tasks"
                            },
                            "due_date": {
                              "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/DueDate-Tasks"
                            },
                            "source_type": {
                              "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/source_type-Tasks"
                            },
                            "starred": {
                              "type": "boolean",
                              "description": "Indicates whether the task is marked as starred."
                            },
                            "collaborators": {
                              "type": "array",
                              "description": "A list of collaborators to invite to the task. Maximum user count is 20.",
                              "items": {
                                "type": "object",
                                "required": [
                                  "user_id",
                                  "email"
                                ],
                                "properties": {
                                  "user_id": {
                                    "type": "string",
                                    "description": "The collaborator's user to invite. The invited user must belong to the same account as the requesting user."
                                  },
                                  "email": {
                                    "type": "string",
                                    "format": "email",
                                    "description": "The collaborator's email address."
                                  }
                                }
                              }
                            },
                            "create_time": {
                              "type": "string",
                              "description": "The creation date-time in UTC/GMT. ",
                              "format": "date-time",
                              "example": "2025-03-25T07:29:29Z"
                            },
                            "friendly_task_id": {
                              "type": "string",
                              "description": "The task's human-readable identifier.",
                              "example": "Task-1"
                            },
                            "link": {
                              "type": "string",
                              "description": "The direct URL link to the task.",
                              "example": "https://tasks.example.com/ws53iQgOOkOdzVTxZaVoRA/TASK-1"
                            },
                            "assignees": {
                              "type": "array",
                              "description": "A list of assignees to add to the task.",
                              "items": {
                                "required": [
                                  "user_id",
                                  "email"
                                ],
                                "type": "object",
                                "properties": {
                                  "user_id": {
                                    "type": "string",
                                    "description": "The assignee's Zoom user ID."
                                  },
                                  "email": {
                                    "type": "string",
                                    "format": "email",
                                    "description": "The assignee's email address."
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
                  "additionalProperties": false,
                  "components": {
                    "schemas": {
                      "PriorityId-Tasks": {
                        "type": "string",
                        "description": "The task's priority level.",
                        "example": "Medium",
                        "default": "Medium",
                        "enum": [
                          "Low",
                          "Medium",
                          "High",
                          "Highest"
                        ]
                      },
                      "StatusId-Tasks": {
                        "type": "string",
                        "description": "The task's current status.",
                        "example": "To do",
                        "default": "To do",
                        "enum": [
                          "To do",
                          "In progress",
                          "Done",
                          "Blocked",
                          "Recommended"
                        ]
                      },
                      "DueDate-Tasks": {
                        "type": "string",
                        "description": "The task's due date in UTC, formatted as yyyy-MM-dd.",
                        "format": "date",
                        "example": "2025-09-24"
                      },
                      "IsPublic-Tasks": {
                        "type": "boolean",
                        "description": "The task's visibility.\n- true - The task is accessible to all members of your organization.\n- false - The task is accessible only to the assigned users and collaborators.",
                        "example": false,
                        "default": false
                      },
                      "source_type-Tasks": {
                        "type": "string",
                        "description": "Type of the source that created the task.",
                        "example": "user",
                        "enum": [
                          "user",
                          "aic_meeting_summary",
                          "whiteboard",
                          "notes",
                          "docs",
                          "phone",
                          "chat",
                          "email",
                          "meeting",
                          "aic"
                        ]
                      }
                    },
                    "parameters": {}
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"task.created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"task_id\": \"12321312323\",\n      \"title\": \"Complete project documentation\",\n      \"description\": \"Write comprehensive documentation for the new API endpoints\",\n      \"status\": \"To do\",\n      \"priority\": \"Medium\",\n      \"is_public\": true,\n      \"due_date\": \"2025-08-22\",\n      \"source_type\": \"user\",\n      \"starred\": false,\n      \"collaborators\": [\n        {\n          \"user_id\": \"w0RChiauQeqRlv5fgxYULQ\",\n          \"email\": \"userB@example.com\"\n        }\n      ],\n      \"create_time\": \"2025-03-25T07:29:29Z\",\n      \"friendly_task_id\": \"Task-1\",\n      \"link\": \"https://tasks.zoomdev.us/ws53iQgOOkOdzVTxZaVoRA/TASK-1\",\n      \"assignees\": [\n        {\n          \"user_id\": \"w0RChiauQeqRlv5fgxYULQ\",\n          \"email\": \"userB@example.com\"\n        }\n      ]\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "task.assignee_updated": {
      "post": {
        "tags": [
          "task"
        ],
        "operationId": "task.assignee_updated",
        "requestBody": {
          "description": "# task.assignee_updated\n\nThe **Task assignee were updated** event is triggered every time a task assignee is added or deleted to share a task with a Zoom user\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Task assignee were updated** subscription enabled under the **Task** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_assignee:read`,`tasks_assignee:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:assignees`,`tasks:read:assignees:admin`\n\n**Event type**: `task.assignee_updated`\n",
          "content": {
            "application/json": {
              "schema": {
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
                      "operation_type",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who updated assignee to the task "
                      },
                      "operator": {
                        "type": "string",
                        "description": "The email of the user who updated assignee to the task."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The Zoom user ID of the operator who updated assignee to the task."
                      },
                      "operation_type": {
                        "type": "integer",
                        "description": "Operation type. \nEnum: \n*0 - REMOVE \n*1 - ADD \n",
                        "enum": [
                          0,
                          1
                        ],
                        "x-enum-descriptions": [
                          "REMOVE",
                          "ADDED"
                        ]
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the task.",
                        "required": [
                          "task_id",
                          "assignees"
                        ],
                        "properties": {
                          "task_id": {
                            "type": "string",
                            "description": "The task ID."
                          },
                          "assignees": {
                            "type": "array",
                            "description": "Assignees that has been added or removed",
                            "items": {
                              "anyOf": [
                                {
                                  "type": "object",
                                  "required": [
                                    "user_id",
                                    "display_name"
                                  ],
                                  "properties": {
                                    "user_id": {
                                      "type": "string",
                                      "description": "The Zoom user ID of the collaborator. "
                                    },
                                    "display_name": {
                                      "type": "string",
                                      "description": "The display name of the collaborator."
                                    },
                                    "email": {
                                      "type": "string",
                                      "format": "email",
                                      "description": "The email address of the collaborator."
                                    },
                                    "avatar": {
                                      "type": "string",
                                      "format": "uri",
                                      "description": "The avatar image URL of the collaborator."
                                    }
                                  },
                                  "additionalProperties": false
                                }
                              ]
                            },
                            "additionalProperties": false
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
                  "value": "{\n  \"event\": \"task.assignee_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operation_type\": 1,\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"assignees\": [\n        {\n          \"user_id\": \"CcrEGgmeQem1uyJsuIRKwA\",\n          \"display_name\": \"John Doe\",\n          \"email\": \"john.doe@example.com\",\n          \"avatar\": \"https://example.com/avatar.jpg\"\n        }\n      ]\n    }\n  }\n}"
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
                        "operation_type",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who updated assignee to the task "
                        },
                        "operator": {
                          "type": "string",
                          "description": "The email of the user who updated assignee to the task."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The Zoom user ID of the operator who updated assignee to the task."
                        },
                        "operation_type": {
                          "type": "integer",
                          "description": "Operation type. \nEnum: \n*0 - REMOVE \n*1 - ADD \n",
                          "enum": [
                            0,
                            1
                          ],
                          "x-enum-descriptions": [
                            "REMOVE",
                            "ADDED"
                          ]
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the task.",
                          "required": [
                            "task_id",
                            "assignees"
                          ],
                          "properties": {
                            "task_id": {
                              "type": "string",
                              "description": "The task ID."
                            },
                            "assignees": {
                              "type": "array",
                              "description": "Assignees that has been added or removed",
                              "items": {
                                "anyOf": [
                                  {
                                    "type": "object",
                                    "required": [
                                      "user_id",
                                      "display_name"
                                    ],
                                    "properties": {
                                      "user_id": {
                                        "type": "string",
                                        "description": "The Zoom user ID of the collaborator. "
                                      },
                                      "display_name": {
                                        "type": "string",
                                        "description": "The display name of the collaborator."
                                      },
                                      "email": {
                                        "type": "string",
                                        "format": "email",
                                        "description": "The email address of the collaborator."
                                      },
                                      "avatar": {
                                        "type": "string",
                                        "format": "uri",
                                        "description": "The avatar image URL of the collaborator."
                                      }
                                    },
                                    "additionalProperties": false
                                  }
                                ]
                              },
                              "additionalProperties": false
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
                    "value": "{\n  \"event\": \"task.assignee_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operation_type\": 1,\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"assignees\": [\n        {\n          \"user_id\": \"CcrEGgmeQem1uyJsuIRKwA\",\n          \"display_name\": \"John Doe\",\n          \"email\": \"john.doe@example.com\",\n          \"avatar\": \"https://example.com/avatar.jpg\"\n        }\n      ]\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "task.reopened": {
      "post": {
        "tags": [
          "task"
        ],
        "operationId": "task.reopened",
        "requestBody": {
          "description": "# task.reopened\n\nThe **task was reopened** event is triggered every time a task is opened.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Task was reopened** subscription enabled under the **Task** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:admin`,`tasks:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:task`,`tasks:read:task:admin`\n\n**Event type**: `task.reopened`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "description": "A timestamp of when the event occurred."
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
                        "description": "The account ID of the user who opened the task."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The email of the user who opened the task."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The Zoom user ID of the operator who opened the task."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the task.",
                        "required": [
                          "task_id"
                        ],
                        "properties": {
                          "task_id": {
                            "type": "string",
                            "description": "The task ID."
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
                  "value": "{\n  \"event\": \"task.reopened\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\"\n    }\n  }\n}"
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
                      "description": "A timestamp of when the event occurred."
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
                          "description": "The account ID of the user who opened the task."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The email of the user who opened the task."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The Zoom user ID of the operator who opened the task."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the task.",
                          "required": [
                            "task_id"
                          ],
                          "properties": {
                            "task_id": {
                              "type": "string",
                              "description": "The task ID."
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
                    "value": "{\n  \"event\": \"task.reopened\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\"\n    }\n  }\n}"
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
    "title": "Tasks",
    "description": "",
    "version": "1.0.0"
  },
  "webhooks": {
    "task.deleted": {
      "post": {
        "tags": [
          "task"
        ],
        "operationId": "task.deleted",
        "requestBody": {
          "description": "# task.deleted\n\nThe **Task deleted** event is triggered every time a task is delete by one of your app users or account users.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Task deleted** subscription enabled under the **Task** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:admin`,`tasks:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:task`,`tasks:read:task:admin`\n\n**Event type**: `task.deleted`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "description": "The timestamp of when the event occurred."
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
                        "description": "The account ID of the user who deleted the task."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The email of the user who deleted the task."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user who deleted the task."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the task",
                        "required": [
                          "task_id"
                        ],
                        "properties": {
                          "task_id": {
                            "type": "string",
                            "description": "The task ID."
                          },
                          "description": {
                            "type": "string",
                            "description": "The detailed description of the task."
                          },
                          "title": {
                            "type": "string",
                            "description": "The title of the task."
                          },
                          "priority": {
                            "$ref": "#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/PriorityId-Tasks"
                          },
                          "status": {
                            "$ref": "#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/StatusId-Tasks"
                          },
                          "is_public": {
                            "$ref": "#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/IsPublic-Tasks"
                          },
                          "due_date": {
                            "$ref": "#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/DueDate-Tasks"
                          }
                        },
                        "additionalProperties": true
                      }
                    },
                    "additionalProperties": true
                  }
                },
                "additionalProperties": true,
                "components": {
                  "schemas": {
                    "PriorityId-Tasks": {
                      "type": "string",
                      "description": "The priority level of the task.",
                      "example": "Medium",
                      "default": "Medium",
                      "enum": [
                        "Low",
                        "Medium",
                        "High",
                        "Highest"
                      ]
                    },
                    "StatusId-Tasks": {
                      "type": "string",
                      "description": "The current status of the task.",
                      "example": "To do",
                      "default": "To do",
                      "enum": [
                        "To do",
                        "In progress",
                        "Done",
                        "Blocked",
                        "Recommended"
                      ]
                    },
                    "DueDate-Tasks": {
                      "type": "string",
                      "description": "The due date of the task in UTC.\n Format: yyyy-mm-dd.",
                      "format": "date",
                      "example": "2025-09-24"
                    },
                    "IsPublic-Tasks": {
                      "type": "boolean",
                      "description": "Determines the task\u2019s visibility.\ntrue: The task is accessible to all members of your organization.\nfalse: The task is accessible only to the assigned users and collaborators.",
                      "example": false,
                      "default": false
                    }
                  },
                  "parameters": {}
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"task.deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"task_id\": \"12321312323\",\n      \"title\": \"Complete project documentation\",\n      \"description\": \"Write comprehensive documentation for the new API endpoints\",\n      \"status\": \"To do\",\n      \"priority\": \"Medium\",\n      \"is_public\": true,\n      \"due_date\": \"2025-08-22\"\n    }\n  }\n}"
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
                      "description": "The timestamp of when the event occurred."
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
                          "description": "The account ID of the user who deleted the task."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The email of the user who deleted the task."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user who deleted the task."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the task",
                          "required": [
                            "task_id"
                          ],
                          "properties": {
                            "task_id": {
                              "type": "string",
                              "description": "The task ID."
                            },
                            "description": {
                              "type": "string",
                              "description": "The detailed description of the task."
                            },
                            "title": {
                              "type": "string",
                              "description": "The title of the task."
                            },
                            "priority": {
                              "$ref": "#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/PriorityId-Tasks"
                            },
                            "status": {
                              "$ref": "#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/StatusId-Tasks"
                            },
                            "is_public": {
                              "$ref": "#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/IsPublic-Tasks"
                            },
                            "due_date": {
                              "$ref": "#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/DueDate-Tasks"
                            }
                          },
                          "additionalProperties": true
                        }
                      },
                      "additionalProperties": true
                    }
                  },
                  "additionalProperties": true,
                  "components": {
                    "schemas": {
                      "PriorityId-Tasks": {
                        "type": "string",
                        "description": "The priority level of the task.",
                        "example": "Medium",
                        "default": "Medium",
                        "enum": [
                          "Low",
                          "Medium",
                          "High",
                          "Highest"
                        ]
                      },
                      "StatusId-Tasks": {
                        "type": "string",
                        "description": "The current status of the task.",
                        "example": "To do",
                        "default": "To do",
                        "enum": [
                          "To do",
                          "In progress",
                          "Done",
                          "Blocked",
                          "Recommended"
                        ]
                      },
                      "DueDate-Tasks": {
                        "type": "string",
                        "description": "The due date of the task in UTC.\n Format: yyyy-mm-dd.",
                        "format": "date",
                        "example": "2025-09-24"
                      },
                      "IsPublic-Tasks": {
                        "type": "boolean",
                        "description": "Determines the task\u2019s visibility.\ntrue: The task is accessible to all members of your organization.\nfalse: The task is accessible only to the assigned users and collaborators.",
                        "example": false,
                        "default": false
                      }
                    },
                    "parameters": {}
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"task.deleted\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"task_id\": \"12321312323\",\n      \"title\": \"Complete project documentation\",\n      \"description\": \"Write comprehensive documentation for the new API endpoints\",\n      \"status\": \"To do\",\n      \"priority\": \"Medium\",\n      \"is_public\": true,\n      \"due_date\": \"2025-08-22\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "task.updated": {
      "post": {
        "tags": [
          "task"
        ],
        "operationId": "task.updated",
        "requestBody": {
          "description": "# task.updated\n\nThe **Task was updated** event is triggered every time a task is updated.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Task was updated** subscription enabled under the **Task** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:admin`,`tasks:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:task`,`tasks:read:task:admin`\n\n**Event type**: `task.updated`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "description": "The timestamp of when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "operator",
                      "operator_id",
                      "task_id"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who updated the task."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The email of the user who updated the task."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": " The Zoom user ID of the operator who closed the task. \n It should be null if the user is external to the task's organization."
                      },
                      "task_id": {
                        "type": "string",
                        "description": "The task ID."
                      },
                      "field_changes": {
                        "type": "array",
                        "description": "The list of field changes in this update.",
                        "items": {
                          "type": "object",
                          "properties": {
                            "field_name": {
                              "type": "string",
                              "enum": [
                                "title",
                                "status",
                                "priority",
                                "is_public",
                                "due_date",
                                "starred",
                                "description"
                              ],
                              "description": "The name of the field that was changed."
                            },
                            "old_value": {
                              "type": "string",
                              "description": "The previous value of the field."
                            },
                            "new_value": {
                              "type": "string",
                              "description": "The new value of the field."
                            },
                            "operation_type": {
                              "type": "string",
                              "enum": [
                                "added",
                                "removed",
                                "updated"
                              ],
                              "description": "The operation performed on the field."
                            },
                            "changed_at": {
                              "type": "integer",
                              "description": "The unix timestamp when the field was changed."
                            }
                          },
                          "required": [
                            "field_name",
                            "operation_type",
                            "changed_at"
                          ]
                        }
                      },
                      "file_changes": {
                        "type": "array",
                        "description": "The list of file changes in this update.",
                        "items": {
                          "type": "object",
                          "properties": {
                            "file_id": {
                              "type": "string",
                              "description": "The unique identifier of the file."
                            },
                            "file_name": {
                              "type": "string",
                              "description": "The name of the file."
                            },
                            "file_type": {
                              "type": "string",
                              "description": "The MIME type of the file."
                            },
                            "file_size": {
                              "type": "integer",
                              "description": "The size of the file in bytes."
                            },
                            "file_download_link": {
                              "type": "string",
                              "description": "The temporary download link for the file."
                            },
                            "download_link_expire_time": {
                              "type": "integer",
                              "description": "The unix timestamp when the download link expires."
                            },
                            "operation_type": {
                              "type": "string",
                              "enum": [
                                "added",
                                "removed"
                              ],
                              "description": "The operation performed on the attachment."
                            },
                            "changed_at": {
                              "type": "integer",
                              "description": "The unix timestamp when the attachment was changed."
                            }
                          },
                          "required": [
                            "file_id",
                            "file_name",
                            "operation_type",
                            "changed_at"
                          ]
                        }
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
                  "value": "{\n  \"event\": \"task.updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n    \"field_changes\": [\n      {\n        \"field_name\": \"title\",\n        \"old_value\": \"Test Plan\",\n        \"new_value\": \"New Test Plan\",\n        \"operation_type\": \"updated\",\n        \"changed_at\": 1758184629\n      }\n    ],\n    \"file_changes\": [\n      {\n        \"file_id\": \"file_456789\",\n        \"file_name\": \"project_requirements.pdf\",\n        \"file_type\": \"application/pdf\",\n        \"file_size\": 2048576,\n        \"file_download_link\": \"https://api.zoom.us/v2/files/file_456789/download?token=temp_token_123\",\n        \"download_link_expire_time\": 1703209856789,\n        \"operation_type\": \"added\",\n        \"changed_at\": 1703123456789\n      }\n    ]\n  }\n}"
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
                      "description": "The timestamp of when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "operator",
                        "operator_id",
                        "task_id"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who updated the task."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The email of the user who updated the task."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": " The Zoom user ID of the operator who closed the task. \n It should be null if the user is external to the task's organization."
                        },
                        "task_id": {
                          "type": "string",
                          "description": "The task ID."
                        },
                        "field_changes": {
                          "type": "array",
                          "description": "The list of field changes in this update.",
                          "items": {
                            "type": "object",
                            "properties": {
                              "field_name": {
                                "type": "string",
                                "enum": [
                                  "title",
                                  "status",
                                  "priority",
                                  "is_public",
                                  "due_date",
                                  "starred",
                                  "description"
                                ],
                                "description": "The name of the field that was changed."
                              },
                              "old_value": {
                                "type": "string",
                                "description": "The previous value of the field."
                              },
                              "new_value": {
                                "type": "string",
                                "description": "The new value of the field."
                              },
                              "operation_type": {
                                "type": "string",
                                "enum": [
                                  "added",
                                  "removed",
                                  "updated"
                                ],
                                "description": "The operation performed on the field."
                              },
                              "changed_at": {
                                "type": "integer",
                                "description": "The unix timestamp when the field was changed."
                              }
                            },
                            "required": [
                              "field_name",
                              "operation_type",
                              "changed_at"
                            ]
                          }
                        },
                        "file_changes": {
                          "type": "array",
                          "description": "The list of file changes in this update.",
                          "items": {
                            "type": "object",
                            "properties": {
                              "file_id": {
                                "type": "string",
                                "description": "The unique identifier of the file."
                              },
                              "file_name": {
                                "type": "string",
                                "description": "The name of the file."
                              },
                              "file_type": {
                                "type": "string",
                                "description": "The MIME type of the file."
                              },
                              "file_size": {
                                "type": "integer",
                                "description": "The size of the file in bytes."
                              },
                              "file_download_link": {
                                "type": "string",
                                "description": "The temporary download link for the file."
                              },
                              "download_link_expire_time": {
                                "type": "integer",
                                "description": "The unix timestamp when the download link expires."
                              },
                              "operation_type": {
                                "type": "string",
                                "enum": [
                                  "added",
                                  "removed"
                                ],
                                "description": "The operation performed on the attachment."
                              },
                              "changed_at": {
                                "type": "integer",
                                "description": "The unix timestamp when the attachment was changed."
                              }
                            },
                            "required": [
                              "file_id",
                              "file_name",
                              "operation_type",
                              "changed_at"
                            ]
                          }
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
                    "value": "{\n  \"event\": \"task.updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n    \"field_changes\": [\n      {\n        \"field_name\": \"title\",\n        \"old_value\": \"Test Plan\",\n        \"new_value\": \"New Test Plan\",\n        \"operation_type\": \"updated\",\n        \"changed_at\": 1758184629\n      }\n    ],\n    \"file_changes\": [\n      {\n        \"file_id\": \"file_456789\",\n        \"file_name\": \"project_requirements.pdf\",\n        \"file_type\": \"application/pdf\",\n        \"file_size\": 2048576,\n        \"file_download_link\": \"https://api.zoom.us/v2/files/file_456789/download?token=temp_token_123\",\n        \"download_link_expire_time\": 1703209856789,\n        \"operation_type\": \"added\",\n        \"changed_at\": 1703123456789\n      }\n    ]\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "task.comment_updated": {
      "post": {
        "tags": [
          "task"
        ],
        "operationId": "task.comment_updated",
        "requestBody": {
          "description": "# task.comment_updated\n\nThe **Task comment was updated** event is triggered every time a task comment is updated to task.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Task comment was updated** subscription enabled under the **Task** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_comment:read`,`tasks_comment:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:comments`,`tasks:read:comments:admin`\n\n**Event type**: `task.comment_updated`\n",
          "content": {
            "application/json": {
              "schema": {
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
                      "operation_type",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who updated the comment with a task."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The email of the user who updated the comment with a task."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user who updated the comment with a task."
                      },
                      "operation_type": {
                        "type": "integer",
                        "description": "Operation type. \n\n*0 - REMOVE \n*1 - ADD \n*2 - UPDATED",
                        "enum": [
                          0,
                          1,
                          2
                        ],
                        "x-enum-descriptions": [
                          "REMOVE",
                          "ADD",
                          "UPDATED"
                        ]
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the task.",
                        "required": [
                          "task_id",
                          "comment"
                        ],
                        "properties": {
                          "task_id": {
                            "type": "string",
                            "description": "The task ID."
                          },
                          "comment": {
                            "type": "object",
                            "description": "The comments that had been added, removed, or updated.",
                            "required": [
                              "content_text",
                              "comment_id"
                            ],
                            "properties": {
                              "content_text": {
                                "type": "string",
                                "description": "The task comment content."
                              },
                              "comment_id": {
                                "type": "string",
                                "description": "The task comment ID."
                              }
                            },
                            "additionalProperties": false
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
                  "value": "{\n  \"event\": \"task.comment_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operation_type\": 1,\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"comment\": {\n        \"comment_id\": \"472a9f05-d46d-44bb-bf5a-93e731d1f6d0\",\n        \"content_text\": \"Jack update comment\"\n      }\n    }\n  }\n}"
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
                        "operation_type",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who updated the comment with a task."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The email of the user who updated the comment with a task."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user who updated the comment with a task."
                        },
                        "operation_type": {
                          "type": "integer",
                          "description": "Operation type. \n\n*0 - REMOVE \n*1 - ADD \n*2 - UPDATED",
                          "enum": [
                            0,
                            1,
                            2
                          ],
                          "x-enum-descriptions": [
                            "REMOVE",
                            "ADD",
                            "UPDATED"
                          ]
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the task.",
                          "required": [
                            "task_id",
                            "comment"
                          ],
                          "properties": {
                            "task_id": {
                              "type": "string",
                              "description": "The task ID."
                            },
                            "comment": {
                              "type": "object",
                              "description": "The comments that had been added, removed, or updated.",
                              "required": [
                                "content_text",
                                "comment_id"
                              ],
                              "properties": {
                                "content_text": {
                                  "type": "string",
                                  "description": "The task comment content."
                                },
                                "comment_id": {
                                  "type": "string",
                                  "description": "The task comment ID."
                                }
                              },
                              "additionalProperties": false
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
                    "value": "{\n  \"event\": \"task.comment_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operation_type\": 1,\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"comment\": {\n        \"comment_id\": \"472a9f05-d46d-44bb-bf5a-93e731d1f6d0\",\n        \"content_text\": \"Jack update comment\"\n      }\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "task.completed": {
      "post": {
        "tags": [
          "task"
        ],
        "operationId": "task.completed",
        "requestBody": {
          "description": "# task.completed\n\nThe **task was completed** event is triggered every time a task status is done.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Task was completed** subscription enabled under the **Task** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:admin`,`tasks:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:task`,`tasks:read:task:admin`\n\n**Event type**: `task.completed`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "description": "A timestamp of when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "operator",
                      "object",
                      "operator_id"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who completed the task."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The email of the user who completed the task."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The Zoom user ID of the operator who completed the task."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the task.",
                        "required": [
                          "task_id",
                          "status"
                        ],
                        "properties": {
                          "task_id": {
                            "type": "string",
                            "description": "The task ID."
                          },
                          "status": {
                            "$ref": "#/paths/task.completed/post/requestBody/content/application~1json/schema/components/schemas/StatusId-Tasks"
                          }
                        },
                        "additionalProperties": false
                      }
                    },
                    "additionalProperties": false
                  }
                },
                "additionalProperties": false,
                "components": {
                  "schemas": {
                    "StatusId-Tasks": {
                      "type": "string",
                      "description": "The current status of the task.",
                      "example": "To do",
                      "default": "To do",
                      "enum": [
                        "To do",
                        "In progress",
                        "Done",
                        "Blocked",
                        "Recommended"
                      ]
                    }
                  },
                  "parameters": {}
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"task.completed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"status\": \"Done\"\n    }\n  }\n}"
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
                      "description": "A timestamp of when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "operator",
                        "object",
                        "operator_id"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who completed the task."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The email of the user who completed the task."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The Zoom user ID of the operator who completed the task."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the task.",
                          "required": [
                            "task_id",
                            "status"
                          ],
                          "properties": {
                            "task_id": {
                              "type": "string",
                              "description": "The task ID."
                            },
                            "status": {
                              "$ref": "#/paths/task.completed/post/requestBody/content/application~1json/schema/components/schemas/StatusId-Tasks"
                            }
                          },
                          "additionalProperties": false
                        }
                      },
                      "additionalProperties": false
                    }
                  },
                  "additionalProperties": false,
                  "components": {
                    "schemas": {
                      "StatusId-Tasks": {
                        "type": "string",
                        "description": "The current status of the task.",
                        "example": "To do",
                        "default": "To do",
                        "enum": [
                          "To do",
                          "In progress",
                          "Done",
                          "Blocked",
                          "Recommended"
                        ]
                      }
                    },
                    "parameters": {}
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"task.completed\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"status\": \"Done\"\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "task.collaborator_updated": {
      "post": {
        "tags": [
          "task"
        ],
        "operationId": "task.collaborator_updated",
        "requestBody": {
          "description": "# task.collaborator_updated\n\nThe **Task collaborator were updated** event is triggered every time a task collaborator is added or deleted to share a task with a Zoom user\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Task collaborator were updated** subscription enabled under the **Task** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_collaborator:read`,`tasks_collaborator:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:list_collaborators`,`tasks:read:list_collaborators:admin`\n\n**Event type**: `task.collaborator_updated`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "description": "A timestamp of when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "operator",
                      "operator_id",
                      "operation_type",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who updated the collaborators of the task."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The email of the user who updated the collaborators of the task."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The Zoom user ID of the operator who updated the collaborators of the task."
                      },
                      "operation_type": {
                        "type": "integer",
                        "description": "Operation type. \nEnum: \n*0 - REMOVE \n*1 - ADD \n",
                        "enum": [
                          0,
                          1
                        ],
                        "x-enum-descriptions": [
                          "REMOVE",
                          "ADDED"
                        ]
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the task.",
                        "required": [
                          "task_id",
                          "collaborators"
                        ],
                        "properties": {
                          "task_id": {
                            "type": "string",
                            "description": "The task ID."
                          },
                          "collaborators": {
                            "type": "array",
                            "description": "Collaborators that has been added or removed.",
                            "items": {
                              "anyOf": [
                                {
                                  "type": "object",
                                  "required": [
                                    "user_id",
                                    "display_name"
                                  ],
                                  "properties": {
                                    "user_id": {
                                      "type": "string",
                                      "description": "The Zoom user ID of the collaborator. "
                                    },
                                    "display_name": {
                                      "type": "string",
                                      "description": "The display name of the collaborator."
                                    },
                                    "email": {
                                      "type": "string",
                                      "format": "email",
                                      "description": "The email address of the collaborator."
                                    },
                                    "avatar": {
                                      "type": "string",
                                      "format": "uri",
                                      "description": "The avatar image URL of the collaborator."
                                    }
                                  },
                                  "additionalProperties": false
                                }
                              ]
                            },
                            "additionalProperties": false
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
                  "value": "{\n  \"event\": \"task.collaborator_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operation_type\": 1,\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"collaborators\": [\n        {\n          \"user_id\": \"CcrEGgmeQem1uyJsuIRKwA\",\n          \"display_name\": \"John Doe\",\n          \"email\": \"john.doe@example.com\",\n          \"avatar\": \"https://example.com/avatar.jpg\"\n        }\n      ]\n    }\n  }\n}"
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
                      "description": "A timestamp of when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "operator",
                        "operator_id",
                        "operation_type",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who updated the collaborators of the task."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The email of the user who updated the collaborators of the task."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The Zoom user ID of the operator who updated the collaborators of the task."
                        },
                        "operation_type": {
                          "type": "integer",
                          "description": "Operation type. \nEnum: \n*0 - REMOVE \n*1 - ADD \n",
                          "enum": [
                            0,
                            1
                          ],
                          "x-enum-descriptions": [
                            "REMOVE",
                            "ADDED"
                          ]
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the task.",
                          "required": [
                            "task_id",
                            "collaborators"
                          ],
                          "properties": {
                            "task_id": {
                              "type": "string",
                              "description": "The task ID."
                            },
                            "collaborators": {
                              "type": "array",
                              "description": "Collaborators that has been added or removed.",
                              "items": {
                                "anyOf": [
                                  {
                                    "type": "object",
                                    "required": [
                                      "user_id",
                                      "display_name"
                                    ],
                                    "properties": {
                                      "user_id": {
                                        "type": "string",
                                        "description": "The Zoom user ID of the collaborator. "
                                      },
                                      "display_name": {
                                        "type": "string",
                                        "description": "The display name of the collaborator."
                                      },
                                      "email": {
                                        "type": "string",
                                        "format": "email",
                                        "description": "The email address of the collaborator."
                                      },
                                      "avatar": {
                                        "type": "string",
                                        "format": "uri",
                                        "description": "The avatar image URL of the collaborator."
                                      }
                                    },
                                    "additionalProperties": false
                                  }
                                ]
                              },
                              "additionalProperties": false
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
                    "value": "{\n  \"event\": \"task.collaborator_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operation_type\": 1,\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"collaborators\": [\n        {\n          \"user_id\": \"CcrEGgmeQem1uyJsuIRKwA\",\n          \"display_name\": \"John Doe\",\n          \"email\": \"john.doe@example.com\",\n          \"avatar\": \"https://example.com/avatar.jpg\"\n        }\n      ]\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "task.created": {
      "post": {
        "tags": [
          "task"
        ],
        "operationId": "task.created",
        "requestBody": {
          "description": "# task.created\n\nThe **Task was created** event is triggered every time a task is created by one of your app users or account users.\n\nZoom offers multiple methods for users to create a new task. Users can manually create tasks in the Zoom client or the [Zoom web portal](https://zoom.us/). Users can also programmatically create tasks via the [**Create a new task**] API.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Task was created** subscription enabled under the **Task** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:admin`,`tasks:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:task`,`tasks:read:task:admin`\n\n**Event type**: `task.created`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "description": "A timestamp of when the event occurred."
                  },
                  "payload": {
                    "type": "object",
                    "required": [
                      "account_id",
                      "operator",
                      "account_id",
                      "operator_id",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who created the task."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The email of the user who created the task."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The user ID of the user who created the task."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the task.",
                        "required": [
                          "create_time",
                          "is_public",
                          "priority",
                          "starred",
                          "status",
                          "task_id",
                          "title",
                          "source_type"
                        ],
                        "properties": {
                          "task_id": {
                            "type": "string",
                            "description": "The task ID."
                          },
                          "description": {
                            "type": "string",
                            "description": "The task's detailed description."
                          },
                          "title": {
                            "type": "string",
                            "description": "The task's title."
                          },
                          "priority": {
                            "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/PriorityId-Tasks"
                          },
                          "status": {
                            "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/StatusId-Tasks"
                          },
                          "is_public": {
                            "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/IsPublic-Tasks"
                          },
                          "due_date": {
                            "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/DueDate-Tasks"
                          },
                          "source_type": {
                            "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/source_type-Tasks"
                          },
                          "starred": {
                            "type": "boolean",
                            "description": "Indicates whether the task is marked as starred."
                          },
                          "collaborators": {
                            "type": "array",
                            "description": "A list of collaborators to invite to the task. Maximum user count is 20.",
                            "items": {
                              "type": "object",
                              "required": [
                                "user_id",
                                "email"
                              ],
                              "properties": {
                                "user_id": {
                                  "type": "string",
                                  "description": "The collaborator's user to invite. The invited user must belong to the same account as the requesting user."
                                },
                                "email": {
                                  "type": "string",
                                  "format": "email",
                                  "description": "The collaborator's email address."
                                }
                              }
                            }
                          },
                          "create_time": {
                            "type": "string",
                            "description": "The creation date-time in UTC/GMT. ",
                            "format": "date-time",
                            "example": "2025-03-25T07:29:29Z"
                          },
                          "friendly_task_id": {
                            "type": "string",
                            "description": "The task's human-readable identifier.",
                            "example": "Task-1"
                          },
                          "link": {
                            "type": "string",
                            "description": "The direct URL link to the task.",
                            "example": "https://tasks.example.com/ws53iQgOOkOdzVTxZaVoRA/TASK-1"
                          },
                          "assignees": {
                            "type": "array",
                            "description": "A list of assignees to add to the task.",
                            "items": {
                              "required": [
                                "user_id",
                                "email"
                              ],
                              "type": "object",
                              "properties": {
                                "user_id": {
                                  "type": "string",
                                  "description": "The assignee's Zoom user ID."
                                },
                                "email": {
                                  "type": "string",
                                  "format": "email",
                                  "description": "The assignee's email address."
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
                "additionalProperties": false,
                "components": {
                  "schemas": {
                    "PriorityId-Tasks": {
                      "type": "string",
                      "description": "The task's priority level.",
                      "example": "Medium",
                      "default": "Medium",
                      "enum": [
                        "Low",
                        "Medium",
                        "High",
                        "Highest"
                      ]
                    },
                    "StatusId-Tasks": {
                      "type": "string",
                      "description": "The task's current status.",
                      "example": "To do",
                      "default": "To do",
                      "enum": [
                        "To do",
                        "In progress",
                        "Done",
                        "Blocked",
                        "Recommended"
                      ]
                    },
                    "DueDate-Tasks": {
                      "type": "string",
                      "description": "The task's due date in UTC, formatted as yyyy-MM-dd.",
                      "format": "date",
                      "example": "2025-09-24"
                    },
                    "IsPublic-Tasks": {
                      "type": "boolean",
                      "description": "The task's visibility.\n- true - The task is accessible to all members of your organization.\n- false - The task is accessible only to the assigned users and collaborators.",
                      "example": false,
                      "default": false
                    },
                    "source_type-Tasks": {
                      "type": "string",
                      "description": "Type of the source that created the task.",
                      "example": "user",
                      "enum": [
                        "user",
                        "aic_meeting_summary",
                        "whiteboard",
                        "notes",
                        "docs",
                        "phone",
                        "chat",
                        "email",
                        "meeting",
                        "aic"
                      ]
                    }
                  },
                  "parameters": {}
                }
              },
              "examples": {
                "json-example": {
                  "summary": "JSON example",
                  "value": "{\n  \"event\": \"task.created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"task_id\": \"12321312323\",\n      \"title\": \"Complete project documentation\",\n      \"description\": \"Write comprehensive documentation for the new API endpoints\",\n      \"status\": \"To do\",\n      \"priority\": \"Medium\",\n      \"is_public\": true,\n      \"due_date\": \"2025-08-22\",\n      \"source_type\": \"user\",\n      \"starred\": false,\n      \"collaborators\": [\n        {\n          \"user_id\": \"w0RChiauQeqRlv5fgxYULQ\",\n          \"email\": \"userB@example.com\"\n        }\n      ],\n      \"create_time\": \"2025-03-25T07:29:29Z\",\n      \"friendly_task_id\": \"Task-1\",\n      \"link\": \"https://tasks.zoomdev.us/ws53iQgOOkOdzVTxZaVoRA/TASK-1\",\n      \"assignees\": [\n        {\n          \"user_id\": \"w0RChiauQeqRlv5fgxYULQ\",\n          \"email\": \"userB@example.com\"\n        }\n      ]\n    }\n  }\n}"
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
                      "description": "A timestamp of when the event occurred."
                    },
                    "payload": {
                      "type": "object",
                      "required": [
                        "account_id",
                        "operator",
                        "account_id",
                        "operator_id",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who created the task."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The email of the user who created the task."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The user ID of the user who created the task."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the task.",
                          "required": [
                            "create_time",
                            "is_public",
                            "priority",
                            "starred",
                            "status",
                            "task_id",
                            "title",
                            "source_type"
                          ],
                          "properties": {
                            "task_id": {
                              "type": "string",
                              "description": "The task ID."
                            },
                            "description": {
                              "type": "string",
                              "description": "The task's detailed description."
                            },
                            "title": {
                              "type": "string",
                              "description": "The task's title."
                            },
                            "priority": {
                              "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/PriorityId-Tasks"
                            },
                            "status": {
                              "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/StatusId-Tasks"
                            },
                            "is_public": {
                              "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/IsPublic-Tasks"
                            },
                            "due_date": {
                              "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/DueDate-Tasks"
                            },
                            "source_type": {
                              "$ref": "#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/source_type-Tasks"
                            },
                            "starred": {
                              "type": "boolean",
                              "description": "Indicates whether the task is marked as starred."
                            },
                            "collaborators": {
                              "type": "array",
                              "description": "A list of collaborators to invite to the task. Maximum user count is 20.",
                              "items": {
                                "type": "object",
                                "required": [
                                  "user_id",
                                  "email"
                                ],
                                "properties": {
                                  "user_id": {
                                    "type": "string",
                                    "description": "The collaborator's user to invite. The invited user must belong to the same account as the requesting user."
                                  },
                                  "email": {
                                    "type": "string",
                                    "format": "email",
                                    "description": "The collaborator's email address."
                                  }
                                }
                              }
                            },
                            "create_time": {
                              "type": "string",
                              "description": "The creation date-time in UTC/GMT. ",
                              "format": "date-time",
                              "example": "2025-03-25T07:29:29Z"
                            },
                            "friendly_task_id": {
                              "type": "string",
                              "description": "The task's human-readable identifier.",
                              "example": "Task-1"
                            },
                            "link": {
                              "type": "string",
                              "description": "The direct URL link to the task.",
                              "example": "https://tasks.example.com/ws53iQgOOkOdzVTxZaVoRA/TASK-1"
                            },
                            "assignees": {
                              "type": "array",
                              "description": "A list of assignees to add to the task.",
                              "items": {
                                "required": [
                                  "user_id",
                                  "email"
                                ],
                                "type": "object",
                                "properties": {
                                  "user_id": {
                                    "type": "string",
                                    "description": "The assignee's Zoom user ID."
                                  },
                                  "email": {
                                    "type": "string",
                                    "format": "email",
                                    "description": "The assignee's email address."
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
                  "additionalProperties": false,
                  "components": {
                    "schemas": {
                      "PriorityId-Tasks": {
                        "type": "string",
                        "description": "The task's priority level.",
                        "example": "Medium",
                        "default": "Medium",
                        "enum": [
                          "Low",
                          "Medium",
                          "High",
                          "Highest"
                        ]
                      },
                      "StatusId-Tasks": {
                        "type": "string",
                        "description": "The task's current status.",
                        "example": "To do",
                        "default": "To do",
                        "enum": [
                          "To do",
                          "In progress",
                          "Done",
                          "Blocked",
                          "Recommended"
                        ]
                      },
                      "DueDate-Tasks": {
                        "type": "string",
                        "description": "The task's due date in UTC, formatted as yyyy-MM-dd.",
                        "format": "date",
                        "example": "2025-09-24"
                      },
                      "IsPublic-Tasks": {
                        "type": "boolean",
                        "description": "The task's visibility.\n- true - The task is accessible to all members of your organization.\n- false - The task is accessible only to the assigned users and collaborators.",
                        "example": false,
                        "default": false
                      },
                      "source_type-Tasks": {
                        "type": "string",
                        "description": "Type of the source that created the task.",
                        "example": "user",
                        "enum": [
                          "user",
                          "aic_meeting_summary",
                          "whiteboard",
                          "notes",
                          "docs",
                          "phone",
                          "chat",
                          "email",
                          "meeting",
                          "aic"
                        ]
                      }
                    },
                    "parameters": {}
                  }
                },
                "examples": {
                  "json-example": {
                    "summary": "JSON example",
                    "value": "{\n  \"event\": \"task.created\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"task_id\": \"12321312323\",\n      \"title\": \"Complete project documentation\",\n      \"description\": \"Write comprehensive documentation for the new API endpoints\",\n      \"status\": \"To do\",\n      \"priority\": \"Medium\",\n      \"is_public\": true,\n      \"due_date\": \"2025-08-22\",\n      \"source_type\": \"user\",\n      \"starred\": false,\n      \"collaborators\": [\n        {\n          \"user_id\": \"w0RChiauQeqRlv5fgxYULQ\",\n          \"email\": \"userB@example.com\"\n        }\n      ],\n      \"create_time\": \"2025-03-25T07:29:29Z\",\n      \"friendly_task_id\": \"Task-1\",\n      \"link\": \"https://tasks.zoomdev.us/ws53iQgOOkOdzVTxZaVoRA/TASK-1\",\n      \"assignees\": [\n        {\n          \"user_id\": \"w0RChiauQeqRlv5fgxYULQ\",\n          \"email\": \"userB@example.com\"\n        }\n      ]\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "task.assignee_updated": {
      "post": {
        "tags": [
          "task"
        ],
        "operationId": "task.assignee_updated",
        "requestBody": {
          "description": "# task.assignee_updated\n\nThe **Task assignee were updated** event is triggered every time a task assignee is added or deleted to share a task with a Zoom user\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Task assignee were updated** subscription enabled under the **Task** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_assignee:read`,`tasks_assignee:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:assignees`,`tasks:read:assignees:admin`\n\n**Event type**: `task.assignee_updated`\n",
          "content": {
            "application/json": {
              "schema": {
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
                      "operation_type",
                      "object"
                    ],
                    "properties": {
                      "account_id": {
                        "type": "string",
                        "description": "The account ID of the user who updated assignee to the task "
                      },
                      "operator": {
                        "type": "string",
                        "description": "The email of the user who updated assignee to the task."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The Zoom user ID of the operator who updated assignee to the task."
                      },
                      "operation_type": {
                        "type": "integer",
                        "description": "Operation type. \nEnum: \n*0 - REMOVE \n*1 - ADD \n",
                        "enum": [
                          0,
                          1
                        ],
                        "x-enum-descriptions": [
                          "REMOVE",
                          "ADDED"
                        ]
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the task.",
                        "required": [
                          "task_id",
                          "assignees"
                        ],
                        "properties": {
                          "task_id": {
                            "type": "string",
                            "description": "The task ID."
                          },
                          "assignees": {
                            "type": "array",
                            "description": "Assignees that has been added or removed",
                            "items": {
                              "anyOf": [
                                {
                                  "type": "object",
                                  "required": [
                                    "user_id",
                                    "display_name"
                                  ],
                                  "properties": {
                                    "user_id": {
                                      "type": "string",
                                      "description": "The Zoom user ID of the collaborator. "
                                    },
                                    "display_name": {
                                      "type": "string",
                                      "description": "The display name of the collaborator."
                                    },
                                    "email": {
                                      "type": "string",
                                      "format": "email",
                                      "description": "The email address of the collaborator."
                                    },
                                    "avatar": {
                                      "type": "string",
                                      "format": "uri",
                                      "description": "The avatar image URL of the collaborator."
                                    }
                                  },
                                  "additionalProperties": false
                                }
                              ]
                            },
                            "additionalProperties": false
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
                  "value": "{\n  \"event\": \"task.assignee_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operation_type\": 1,\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"assignees\": [\n        {\n          \"user_id\": \"CcrEGgmeQem1uyJsuIRKwA\",\n          \"display_name\": \"John Doe\",\n          \"email\": \"john.doe@example.com\",\n          \"avatar\": \"https://example.com/avatar.jpg\"\n        }\n      ]\n    }\n  }\n}"
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
                        "operation_type",
                        "object"
                      ],
                      "properties": {
                        "account_id": {
                          "type": "string",
                          "description": "The account ID of the user who updated assignee to the task "
                        },
                        "operator": {
                          "type": "string",
                          "description": "The email of the user who updated assignee to the task."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The Zoom user ID of the operator who updated assignee to the task."
                        },
                        "operation_type": {
                          "type": "integer",
                          "description": "Operation type. \nEnum: \n*0 - REMOVE \n*1 - ADD \n",
                          "enum": [
                            0,
                            1
                          ],
                          "x-enum-descriptions": [
                            "REMOVE",
                            "ADDED"
                          ]
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the task.",
                          "required": [
                            "task_id",
                            "assignees"
                          ],
                          "properties": {
                            "task_id": {
                              "type": "string",
                              "description": "The task ID."
                            },
                            "assignees": {
                              "type": "array",
                              "description": "Assignees that has been added or removed",
                              "items": {
                                "anyOf": [
                                  {
                                    "type": "object",
                                    "required": [
                                      "user_id",
                                      "display_name"
                                    ],
                                    "properties": {
                                      "user_id": {
                                        "type": "string",
                                        "description": "The Zoom user ID of the collaborator. "
                                      },
                                      "display_name": {
                                        "type": "string",
                                        "description": "The display name of the collaborator."
                                      },
                                      "email": {
                                        "type": "string",
                                        "format": "email",
                                        "description": "The email address of the collaborator."
                                      },
                                      "avatar": {
                                        "type": "string",
                                        "format": "uri",
                                        "description": "The avatar image URL of the collaborator."
                                      }
                                    },
                                    "additionalProperties": false
                                  }
                                ]
                              },
                              "additionalProperties": false
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
                    "value": "{\n  \"event\": \"task.assignee_updated\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"operation_type\": 1,\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\",\n      \"assignees\": [\n        {\n          \"user_id\": \"CcrEGgmeQem1uyJsuIRKwA\",\n          \"display_name\": \"John Doe\",\n          \"email\": \"john.doe@example.com\",\n          \"avatar\": \"https://example.com/avatar.jpg\"\n        }\n      ]\n    }\n  }\n}"
                  }
                }
              }
            }
          }
        }
      }
    },
    "task.reopened": {
      "post": {
        "tags": [
          "task"
        ],
        "operationId": "task.reopened",
        "requestBody": {
          "description": "# task.reopened\n\nThe **task was reopened** event is triggered every time a task is opened.\n\n## Prerequisites\n\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\n\t* A valid **Event Notification Endpoint URL**.\n\t* The **Task was reopened** subscription enabled under the **Task** event.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:admin`,`tasks:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:task`,`tasks:read:task:admin`\n\n**Event type**: `task.reopened`\n",
          "content": {
            "application/json": {
              "schema": {
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
                    "description": "A timestamp of when the event occurred."
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
                        "description": "The account ID of the user who opened the task."
                      },
                      "operator": {
                        "type": "string",
                        "description": "The email of the user who opened the task."
                      },
                      "operator_id": {
                        "type": "string",
                        "description": "The Zoom user ID of the operator who opened the task."
                      },
                      "object": {
                        "type": "object",
                        "description": "Information about the task.",
                        "required": [
                          "task_id"
                        ],
                        "properties": {
                          "task_id": {
                            "type": "string",
                            "description": "The task ID."
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
                  "value": "{\n  \"event\": \"task.reopened\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\"\n    }\n  }\n}"
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
                      "description": "A timestamp of when the event occurred."
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
                          "description": "The account ID of the user who opened the task."
                        },
                        "operator": {
                          "type": "string",
                          "description": "The email of the user who opened the task."
                        },
                        "operator_id": {
                          "type": "string",
                          "description": "The Zoom user ID of the operator who opened the task."
                        },
                        "object": {
                          "type": "object",
                          "description": "Information about the task.",
                          "required": [
                            "task_id"
                          ],
                          "properties": {
                            "task_id": {
                              "type": "string",
                              "description": "The task ID."
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
                    "value": "{\n  \"event\": \"task.reopened\",\n  \"event_ts\": 1626230691572,\n  \"payload\": {\n    \"account_id\": \"AAAAAABBBB\",\n    \"operator\": \"user@example.com\",\n    \"operator_id\": \"z8yCxjabcdEFGHfp8uQ\",\n    \"object\": {\n      \"task_id\": \"X4Hy02w3QUOdskKofgb9Jg\"\n    }\n  }\n}"
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
      "tasks",
      "events"
    ]
  },
  "downloadPath": "/api-hub/tasks/events/webhooks.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": false
  },
  "productMeta": {
    "specType": "adoc",
    "slug": "/docs/api/rest/reference/tasks/events",
    "title": "Tasks",
    "keywords": "zoom, rest, api, ai, aic, tasks",
    "params": {
      "productName": "Tasks"
    },
    "zoapEndpoint": "webhookByProduct",
    "productDisplayTitle": "Tasks",
    "scalarSlug": "/docs/api/tasks/events",
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "svgname": "tasks"
  },
  "frontmatter": {
    "title": "Tasks Webhooks",
    "keywords": "zoom, rest, api, ai, aic, tasks",
    "description": "Tasks events",
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
        "tasks",
        "events"
      ]
    },
    "downloadPath": "/api-hub/tasks/events/webhooks.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": false
    },
    "productMeta": {
      "specType": "adoc",
      "slug": "/docs/api/rest/reference/tasks/events",
      "title": "Tasks",
      "keywords": "zoom, rest, api, ai, aic, tasks",
      "params": {
        "productName": "Tasks"
      },
      "zoapEndpoint": "webhookByProduct",
      "productDisplayTitle": "Tasks",
      "scalarSlug": "/docs/api/tasks/events",
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "svgname": "tasks"
    },
    "spec": "{\"openapi\":\"3.1.0\",\"info\":{\"title\":\"Tasks\",\"description\":\"\",\"version\":\"1.0.0\"},\"paths\":{\"task.deleted\":{\"post\":{\"tags\":[\"task\"],\"operationId\":\"task.deleted\",\"requestBody\":{\"description\":\"# task.deleted\\n\\nThe **Task deleted** event is triggered every time a task is delete by one of your app users or account users.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Task deleted** subscription enabled under the **Task** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:admin`,`tasks:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:task`,`tasks:read:task:admin`\\n\\n**Event type**: `task.deleted`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who deleted the task.\"},\"operator\":{\"type\":\"string\",\"description\":\"The email of the user who deleted the task.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user who deleted the task.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the task\",\"required\":[\"task_id\"],\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The task ID.\"},\"description\":{\"type\":\"string\",\"description\":\"The detailed description of the task.\"},\"title\":{\"type\":\"string\",\"description\":\"The title of the task.\"},\"priority\":{\"$ref\":\"#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/PriorityId-Tasks\"},\"status\":{\"$ref\":\"#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/StatusId-Tasks\"},\"is_public\":{\"$ref\":\"#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/IsPublic-Tasks\"},\"due_date\":{\"$ref\":\"#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/DueDate-Tasks\"}},\"additionalProperties\":true}},\"additionalProperties\":true}},\"additionalProperties\":true,\"components\":{\"schemas\":{\"PriorityId-Tasks\":{\"type\":\"string\",\"description\":\"The priority level of the task.\",\"example\":\"Medium\",\"default\":\"Medium\",\"enum\":[\"Low\",\"Medium\",\"High\",\"Highest\"]},\"StatusId-Tasks\":{\"type\":\"string\",\"description\":\"The current status of the task.\",\"example\":\"To do\",\"default\":\"To do\",\"enum\":[\"To do\",\"In progress\",\"Done\",\"Blocked\",\"Recommended\"]},\"DueDate-Tasks\":{\"type\":\"string\",\"description\":\"The due date of the task in UTC.\\n Format: yyyy-mm-dd.\",\"format\":\"date\",\"example\":\"2025-09-24\"},\"IsPublic-Tasks\":{\"type\":\"boolean\",\"description\":\"Determines the task\u2019s visibility.\\ntrue: The task is accessible to all members of your organization.\\nfalse: The task is accessible only to the assigned users and collaborators.\",\"example\":false,\"default\":false}},\"parameters\":{}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"task.deleted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"user@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"object\\\": {\\n      \\\"task_id\\\": \\\"12321312323\\\",\\n      \\\"title\\\": \\\"Complete project documentation\\\",\\n      \\\"description\\\": \\\"Write comprehensive documentation for the new API endpoints\\\",\\n      \\\"status\\\": \\\"To do\\\",\\n      \\\"priority\\\": \\\"Medium\\\",\\n      \\\"is_public\\\": true,\\n      \\\"due_date\\\": \\\"2025-08-22\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who deleted the task.\"},\"operator\":{\"type\":\"string\",\"description\":\"The email of the user who deleted the task.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user who deleted the task.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the task\",\"required\":[\"task_id\"],\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The task ID.\"},\"description\":{\"type\":\"string\",\"description\":\"The detailed description of the task.\"},\"title\":{\"type\":\"string\",\"description\":\"The title of the task.\"},\"priority\":{\"$ref\":\"#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/PriorityId-Tasks\"},\"status\":{\"$ref\":\"#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/StatusId-Tasks\"},\"is_public\":{\"$ref\":\"#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/IsPublic-Tasks\"},\"due_date\":{\"$ref\":\"#/paths/task.deleted/post/requestBody/content/application~1json/schema/components/schemas/DueDate-Tasks\"}},\"additionalProperties\":true}},\"additionalProperties\":true}},\"additionalProperties\":true,\"components\":{\"schemas\":{\"PriorityId-Tasks\":{\"type\":\"string\",\"description\":\"The priority level of the task.\",\"example\":\"Medium\",\"default\":\"Medium\",\"enum\":[\"Low\",\"Medium\",\"High\",\"Highest\"]},\"StatusId-Tasks\":{\"type\":\"string\",\"description\":\"The current status of the task.\",\"example\":\"To do\",\"default\":\"To do\",\"enum\":[\"To do\",\"In progress\",\"Done\",\"Blocked\",\"Recommended\"]},\"DueDate-Tasks\":{\"type\":\"string\",\"description\":\"The due date of the task in UTC.\\n Format: yyyy-mm-dd.\",\"format\":\"date\",\"example\":\"2025-09-24\"},\"IsPublic-Tasks\":{\"type\":\"boolean\",\"description\":\"Determines the task\u2019s visibility.\\ntrue: The task is accessible to all members of your organization.\\nfalse: The task is accessible only to the assigned users and collaborators.\",\"example\":false,\"default\":false}},\"parameters\":{}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"task.deleted\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"user@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"object\\\": {\\n      \\\"task_id\\\": \\\"12321312323\\\",\\n      \\\"title\\\": \\\"Complete project documentation\\\",\\n      \\\"description\\\": \\\"Write comprehensive documentation for the new API endpoints\\\",\\n      \\\"status\\\": \\\"To do\\\",\\n      \\\"priority\\\": \\\"Medium\\\",\\n      \\\"is_public\\\": true,\\n      \\\"due_date\\\": \\\"2025-08-22\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"task.updated\":{\"post\":{\"tags\":[\"task\"],\"operationId\":\"task.updated\",\"requestBody\":{\"description\":\"# task.updated\\n\\nThe **Task was updated** event is triggered every time a task is updated.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Task was updated** subscription enabled under the **Task** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:admin`,`tasks:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:task`,`tasks:read:task:admin`\\n\\n**Event type**: `task.updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"task_id\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who updated the task.\"},\"operator\":{\"type\":\"string\",\"description\":\"The email of the user who updated the task.\"},\"operator_id\":{\"type\":\"string\",\"description\":\" The Zoom user ID of the operator who closed the task. \\n It should be null if the user is external to the task's organization.\"},\"task_id\":{\"type\":\"string\",\"description\":\"The task ID.\"},\"field_changes\":{\"type\":\"array\",\"description\":\"The list of field changes in this update.\",\"items\":{\"type\":\"object\",\"properties\":{\"field_name\":{\"type\":\"string\",\"enum\":[\"title\",\"status\",\"priority\",\"is_public\",\"due_date\",\"starred\",\"description\"],\"description\":\"The name of the field that was changed.\"},\"old_value\":{\"type\":\"string\",\"description\":\"The previous value of the field.\"},\"new_value\":{\"type\":\"string\",\"description\":\"The new value of the field.\"},\"operation_type\":{\"type\":\"string\",\"enum\":[\"added\",\"removed\",\"updated\"],\"description\":\"The operation performed on the field.\"},\"changed_at\":{\"type\":\"integer\",\"description\":\"The unix timestamp when the field was changed.\"}},\"required\":[\"field_name\",\"operation_type\",\"changed_at\"]}},\"file_changes\":{\"type\":\"array\",\"description\":\"The list of file changes in this update.\",\"items\":{\"type\":\"object\",\"properties\":{\"file_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the file.\"},\"file_name\":{\"type\":\"string\",\"description\":\"The name of the file.\"},\"file_type\":{\"type\":\"string\",\"description\":\"The MIME type of the file.\"},\"file_size\":{\"type\":\"integer\",\"description\":\"The size of the file in bytes.\"},\"file_download_link\":{\"type\":\"string\",\"description\":\"The temporary download link for the file.\"},\"download_link_expire_time\":{\"type\":\"integer\",\"description\":\"The unix timestamp when the download link expires.\"},\"operation_type\":{\"type\":\"string\",\"enum\":[\"added\",\"removed\"],\"description\":\"The operation performed on the attachment.\"},\"changed_at\":{\"type\":\"integer\",\"description\":\"The unix timestamp when the attachment was changed.\"}},\"required\":[\"file_id\",\"file_name\",\"operation_type\",\"changed_at\"]}}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"task.updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"user@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"task_id\\\": \\\"X4Hy02w3QUOdskKofgb9Jg\\\",\\n    \\\"field_changes\\\": [\\n      {\\n        \\\"field_name\\\": \\\"title\\\",\\n        \\\"old_value\\\": \\\"Test Plan\\\",\\n        \\\"new_value\\\": \\\"New Test Plan\\\",\\n        \\\"operation_type\\\": \\\"updated\\\",\\n        \\\"changed_at\\\": 1758184629\\n      }\\n    ],\\n    \\\"file_changes\\\": [\\n      {\\n        \\\"file_id\\\": \\\"file_456789\\\",\\n        \\\"file_name\\\": \\\"project_requirements.pdf\\\",\\n        \\\"file_type\\\": \\\"application/pdf\\\",\\n        \\\"file_size\\\": 2048576,\\n        \\\"file_download_link\\\": \\\"https://api.zoom.us/v2/files/file_456789/download?token=temp_token_123\\\",\\n        \\\"download_link_expire_time\\\": 1703209856789,\\n        \\\"operation_type\\\": \\\"added\\\",\\n        \\\"changed_at\\\": 1703123456789\\n      }\\n    ]\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"The timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"task_id\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who updated the task.\"},\"operator\":{\"type\":\"string\",\"description\":\"The email of the user who updated the task.\"},\"operator_id\":{\"type\":\"string\",\"description\":\" The Zoom user ID of the operator who closed the task. \\n It should be null if the user is external to the task's organization.\"},\"task_id\":{\"type\":\"string\",\"description\":\"The task ID.\"},\"field_changes\":{\"type\":\"array\",\"description\":\"The list of field changes in this update.\",\"items\":{\"type\":\"object\",\"properties\":{\"field_name\":{\"type\":\"string\",\"enum\":[\"title\",\"status\",\"priority\",\"is_public\",\"due_date\",\"starred\",\"description\"],\"description\":\"The name of the field that was changed.\"},\"old_value\":{\"type\":\"string\",\"description\":\"The previous value of the field.\"},\"new_value\":{\"type\":\"string\",\"description\":\"The new value of the field.\"},\"operation_type\":{\"type\":\"string\",\"enum\":[\"added\",\"removed\",\"updated\"],\"description\":\"The operation performed on the field.\"},\"changed_at\":{\"type\":\"integer\",\"description\":\"The unix timestamp when the field was changed.\"}},\"required\":[\"field_name\",\"operation_type\",\"changed_at\"]}},\"file_changes\":{\"type\":\"array\",\"description\":\"The list of file changes in this update.\",\"items\":{\"type\":\"object\",\"properties\":{\"file_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the file.\"},\"file_name\":{\"type\":\"string\",\"description\":\"The name of the file.\"},\"file_type\":{\"type\":\"string\",\"description\":\"The MIME type of the file.\"},\"file_size\":{\"type\":\"integer\",\"description\":\"The size of the file in bytes.\"},\"file_download_link\":{\"type\":\"string\",\"description\":\"The temporary download link for the file.\"},\"download_link_expire_time\":{\"type\":\"integer\",\"description\":\"The unix timestamp when the download link expires.\"},\"operation_type\":{\"type\":\"string\",\"enum\":[\"added\",\"removed\"],\"description\":\"The operation performed on the attachment.\"},\"changed_at\":{\"type\":\"integer\",\"description\":\"The unix timestamp when the attachment was changed.\"}},\"required\":[\"file_id\",\"file_name\",\"operation_type\",\"changed_at\"]}}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"task.updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"user@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"task_id\\\": \\\"X4Hy02w3QUOdskKofgb9Jg\\\",\\n    \\\"field_changes\\\": [\\n      {\\n        \\\"field_name\\\": \\\"title\\\",\\n        \\\"old_value\\\": \\\"Test Plan\\\",\\n        \\\"new_value\\\": \\\"New Test Plan\\\",\\n        \\\"operation_type\\\": \\\"updated\\\",\\n        \\\"changed_at\\\": 1758184629\\n      }\\n    ],\\n    \\\"file_changes\\\": [\\n      {\\n        \\\"file_id\\\": \\\"file_456789\\\",\\n        \\\"file_name\\\": \\\"project_requirements.pdf\\\",\\n        \\\"file_type\\\": \\\"application/pdf\\\",\\n        \\\"file_size\\\": 2048576,\\n        \\\"file_download_link\\\": \\\"https://api.zoom.us/v2/files/file_456789/download?token=temp_token_123\\\",\\n        \\\"download_link_expire_time\\\": 1703209856789,\\n        \\\"operation_type\\\": \\\"added\\\",\\n        \\\"changed_at\\\": 1703123456789\\n      }\\n    ]\\n  }\\n}\"}}}}}}}},\"task.comment_updated\":{\"post\":{\"tags\":[\"task\"],\"operationId\":\"task.comment_updated\",\"requestBody\":{\"description\":\"# task.comment_updated\\n\\nThe **Task comment was updated** event is triggered every time a task comment is updated to task.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Task comment was updated** subscription enabled under the **Task** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_comment:read`,`tasks_comment:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:comments`,`tasks:read:comments:admin`\\n\\n**Event type**: `task.comment_updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"operation_type\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who updated the comment with a task.\"},\"operator\":{\"type\":\"string\",\"description\":\"The email of the user who updated the comment with a task.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user who updated the comment with a task.\"},\"operation_type\":{\"type\":\"integer\",\"description\":\"Operation type. \\n\\n*0 - REMOVE \\n*1 - ADD \\n*2 - UPDATED\",\"enum\":[0,1,2],\"x-enum-descriptions\":[\"REMOVE\",\"ADD\",\"UPDATED\"]},\"object\":{\"type\":\"object\",\"description\":\"Information about the task.\",\"required\":[\"task_id\",\"comment\"],\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The task ID.\"},\"comment\":{\"type\":\"object\",\"description\":\"The comments that had been added, removed, or updated.\",\"required\":[\"content_text\",\"comment_id\"],\"properties\":{\"content_text\":{\"type\":\"string\",\"description\":\"The task comment content.\"},\"comment_id\":{\"type\":\"string\",\"description\":\"The task comment ID.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"task.comment_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"user@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"operation_type\\\": 1,\\n    \\\"object\\\": {\\n      \\\"task_id\\\": \\\"X4Hy02w3QUOdskKofgb9Jg\\\",\\n      \\\"comment\\\": {\\n        \\\"comment_id\\\": \\\"472a9f05-d46d-44bb-bf5a-93e731d1f6d0\\\",\\n        \\\"content_text\\\": \\\"Jack update comment\\\"\\n      }\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"operation_type\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who updated the comment with a task.\"},\"operator\":{\"type\":\"string\",\"description\":\"The email of the user who updated the comment with a task.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user who updated the comment with a task.\"},\"operation_type\":{\"type\":\"integer\",\"description\":\"Operation type. \\n\\n*0 - REMOVE \\n*1 - ADD \\n*2 - UPDATED\",\"enum\":[0,1,2],\"x-enum-descriptions\":[\"REMOVE\",\"ADD\",\"UPDATED\"]},\"object\":{\"type\":\"object\",\"description\":\"Information about the task.\",\"required\":[\"task_id\",\"comment\"],\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The task ID.\"},\"comment\":{\"type\":\"object\",\"description\":\"The comments that had been added, removed, or updated.\",\"required\":[\"content_text\",\"comment_id\"],\"properties\":{\"content_text\":{\"type\":\"string\",\"description\":\"The task comment content.\"},\"comment_id\":{\"type\":\"string\",\"description\":\"The task comment ID.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"task.comment_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"user@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"operation_type\\\": 1,\\n    \\\"object\\\": {\\n      \\\"task_id\\\": \\\"X4Hy02w3QUOdskKofgb9Jg\\\",\\n      \\\"comment\\\": {\\n        \\\"comment_id\\\": \\\"472a9f05-d46d-44bb-bf5a-93e731d1f6d0\\\",\\n        \\\"content_text\\\": \\\"Jack update comment\\\"\\n      }\\n    }\\n  }\\n}\"}}}}}}}},\"task.completed\":{\"post\":{\"tags\":[\"task\"],\"operationId\":\"task.completed\",\"requestBody\":{\"description\":\"# task.completed\\n\\nThe **task was completed** event is triggered every time a task status is done.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Task was completed** subscription enabled under the **Task** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:admin`,`tasks:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:task`,`tasks:read:task:admin`\\n\\n**Event type**: `task.completed`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"object\",\"operator_id\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who completed the task.\"},\"operator\":{\"type\":\"string\",\"description\":\"The email of the user who completed the task.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the operator who completed the task.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the task.\",\"required\":[\"task_id\",\"status\"],\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The task ID.\"},\"status\":{\"$ref\":\"#/paths/task.completed/post/requestBody/content/application~1json/schema/components/schemas/StatusId-Tasks\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false,\"components\":{\"schemas\":{\"StatusId-Tasks\":{\"type\":\"string\",\"description\":\"The current status of the task.\",\"example\":\"To do\",\"default\":\"To do\",\"enum\":[\"To do\",\"In progress\",\"Done\",\"Blocked\",\"Recommended\"]}},\"parameters\":{}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"task.completed\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"user@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"object\\\": {\\n      \\\"task_id\\\": \\\"X4Hy02w3QUOdskKofgb9Jg\\\",\\n      \\\"status\\\": \\\"Done\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"object\",\"operator_id\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who completed the task.\"},\"operator\":{\"type\":\"string\",\"description\":\"The email of the user who completed the task.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the operator who completed the task.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the task.\",\"required\":[\"task_id\",\"status\"],\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The task ID.\"},\"status\":{\"$ref\":\"#/paths/task.completed/post/requestBody/content/application~1json/schema/components/schemas/StatusId-Tasks\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false,\"components\":{\"schemas\":{\"StatusId-Tasks\":{\"type\":\"string\",\"description\":\"The current status of the task.\",\"example\":\"To do\",\"default\":\"To do\",\"enum\":[\"To do\",\"In progress\",\"Done\",\"Blocked\",\"Recommended\"]}},\"parameters\":{}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"task.completed\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"user@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"object\\\": {\\n      \\\"task_id\\\": \\\"X4Hy02w3QUOdskKofgb9Jg\\\",\\n      \\\"status\\\": \\\"Done\\\"\\n    }\\n  }\\n}\"}}}}}}}},\"task.collaborator_updated\":{\"post\":{\"tags\":[\"task\"],\"operationId\":\"task.collaborator_updated\",\"requestBody\":{\"description\":\"# task.collaborator_updated\\n\\nThe **Task collaborator were updated** event is triggered every time a task collaborator is added or deleted to share a task with a Zoom user\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Task collaborator were updated** subscription enabled under the **Task** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_collaborator:read`,`tasks_collaborator:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:list_collaborators`,`tasks:read:list_collaborators:admin`\\n\\n**Event type**: `task.collaborator_updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"operation_type\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who updated the collaborators of the task.\"},\"operator\":{\"type\":\"string\",\"description\":\"The email of the user who updated the collaborators of the task.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the operator who updated the collaborators of the task.\"},\"operation_type\":{\"type\":\"integer\",\"description\":\"Operation type. \\nEnum: \\n*0 - REMOVE \\n*1 - ADD \\n\",\"enum\":[0,1],\"x-enum-descriptions\":[\"REMOVE\",\"ADDED\"]},\"object\":{\"type\":\"object\",\"description\":\"Information about the task.\",\"required\":[\"task_id\",\"collaborators\"],\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The task ID.\"},\"collaborators\":{\"type\":\"array\",\"description\":\"Collaborators that has been added or removed.\",\"items\":{\"anyOf\":[{\"type\":\"object\",\"required\":[\"user_id\",\"display_name\"],\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the collaborator. \"},\"display_name\":{\"type\":\"string\",\"description\":\"The display name of the collaborator.\"},\"email\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the collaborator.\"},\"avatar\":{\"type\":\"string\",\"format\":\"uri\",\"description\":\"The avatar image URL of the collaborator.\"}},\"additionalProperties\":false}]},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"task.collaborator_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"user@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"operation_type\\\": 1,\\n    \\\"object\\\": {\\n      \\\"task_id\\\": \\\"X4Hy02w3QUOdskKofgb9Jg\\\",\\n      \\\"collaborators\\\": [\\n        {\\n          \\\"user_id\\\": \\\"CcrEGgmeQem1uyJsuIRKwA\\\",\\n          \\\"display_name\\\": \\\"John Doe\\\",\\n          \\\"email\\\": \\\"john.doe@example.com\\\",\\n          \\\"avatar\\\": \\\"https://example.com/avatar.jpg\\\"\\n        }\\n      ]\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"operation_type\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who updated the collaborators of the task.\"},\"operator\":{\"type\":\"string\",\"description\":\"The email of the user who updated the collaborators of the task.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the operator who updated the collaborators of the task.\"},\"operation_type\":{\"type\":\"integer\",\"description\":\"Operation type. \\nEnum: \\n*0 - REMOVE \\n*1 - ADD \\n\",\"enum\":[0,1],\"x-enum-descriptions\":[\"REMOVE\",\"ADDED\"]},\"object\":{\"type\":\"object\",\"description\":\"Information about the task.\",\"required\":[\"task_id\",\"collaborators\"],\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The task ID.\"},\"collaborators\":{\"type\":\"array\",\"description\":\"Collaborators that has been added or removed.\",\"items\":{\"anyOf\":[{\"type\":\"object\",\"required\":[\"user_id\",\"display_name\"],\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the collaborator. \"},\"display_name\":{\"type\":\"string\",\"description\":\"The display name of the collaborator.\"},\"email\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the collaborator.\"},\"avatar\":{\"type\":\"string\",\"format\":\"uri\",\"description\":\"The avatar image URL of the collaborator.\"}},\"additionalProperties\":false}]},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"task.collaborator_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"user@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"operation_type\\\": 1,\\n    \\\"object\\\": {\\n      \\\"task_id\\\": \\\"X4Hy02w3QUOdskKofgb9Jg\\\",\\n      \\\"collaborators\\\": [\\n        {\\n          \\\"user_id\\\": \\\"CcrEGgmeQem1uyJsuIRKwA\\\",\\n          \\\"display_name\\\": \\\"John Doe\\\",\\n          \\\"email\\\": \\\"john.doe@example.com\\\",\\n          \\\"avatar\\\": \\\"https://example.com/avatar.jpg\\\"\\n        }\\n      ]\\n    }\\n  }\\n}\"}}}}}}}},\"task.created\":{\"post\":{\"tags\":[\"task\"],\"operationId\":\"task.created\",\"requestBody\":{\"description\":\"# task.created\\n\\nThe **Task was created** event is triggered every time a task is created by one of your app users or account users.\\n\\nZoom offers multiple methods for users to create a new task. Users can manually create tasks in the Zoom client or the [Zoom web portal](https://zoom.us/). Users can also programmatically create tasks via the [**Create a new task**] API.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Task was created** subscription enabled under the **Task** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:admin`,`tasks:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:task`,`tasks:read:task:admin`\\n\\n**Event type**: `task.created`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"account_id\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who created the task.\"},\"operator\":{\"type\":\"string\",\"description\":\"The email of the user who created the task.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user who created the task.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the task.\",\"required\":[\"create_time\",\"is_public\",\"priority\",\"starred\",\"status\",\"task_id\",\"title\",\"source_type\"],\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The task ID.\"},\"description\":{\"type\":\"string\",\"description\":\"The task's detailed description.\"},\"title\":{\"type\":\"string\",\"description\":\"The task's title.\"},\"priority\":{\"$ref\":\"#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/PriorityId-Tasks\"},\"status\":{\"$ref\":\"#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/StatusId-Tasks\"},\"is_public\":{\"$ref\":\"#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/IsPublic-Tasks\"},\"due_date\":{\"$ref\":\"#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/DueDate-Tasks\"},\"source_type\":{\"$ref\":\"#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/source_type-Tasks\"},\"starred\":{\"type\":\"boolean\",\"description\":\"Indicates whether the task is marked as starred.\"},\"collaborators\":{\"type\":\"array\",\"description\":\"A list of collaborators to invite to the task. Maximum user count is 20.\",\"items\":{\"type\":\"object\",\"required\":[\"user_id\",\"email\"],\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The collaborator's user to invite. The invited user must belong to the same account as the requesting user.\"},\"email\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The collaborator's email address.\"}}}},\"create_time\":{\"type\":\"string\",\"description\":\"The creation date-time in UTC/GMT. \",\"format\":\"date-time\",\"example\":\"2025-03-25T07:29:29Z\"},\"friendly_task_id\":{\"type\":\"string\",\"description\":\"The task's human-readable identifier.\",\"example\":\"Task-1\"},\"link\":{\"type\":\"string\",\"description\":\"The direct URL link to the task.\",\"example\":\"https://tasks.example.com/ws53iQgOOkOdzVTxZaVoRA/TASK-1\"},\"assignees\":{\"type\":\"array\",\"description\":\"A list of assignees to add to the task.\",\"items\":{\"required\":[\"user_id\",\"email\"],\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The assignee's Zoom user ID.\"},\"email\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The assignee's email address.\"}}}}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false,\"components\":{\"schemas\":{\"PriorityId-Tasks\":{\"type\":\"string\",\"description\":\"The task's priority level.\",\"example\":\"Medium\",\"default\":\"Medium\",\"enum\":[\"Low\",\"Medium\",\"High\",\"Highest\"]},\"StatusId-Tasks\":{\"type\":\"string\",\"description\":\"The task's current status.\",\"example\":\"To do\",\"default\":\"To do\",\"enum\":[\"To do\",\"In progress\",\"Done\",\"Blocked\",\"Recommended\"]},\"DueDate-Tasks\":{\"type\":\"string\",\"description\":\"The task's due date in UTC, formatted as yyyy-MM-dd.\",\"format\":\"date\",\"example\":\"2025-09-24\"},\"IsPublic-Tasks\":{\"type\":\"boolean\",\"description\":\"The task's visibility.\\n- true - The task is accessible to all members of your organization.\\n- false - The task is accessible only to the assigned users and collaborators.\",\"example\":false,\"default\":false},\"source_type-Tasks\":{\"type\":\"string\",\"description\":\"Type of the source that created the task.\",\"example\":\"user\",\"enum\":[\"user\",\"aic_meeting_summary\",\"whiteboard\",\"notes\",\"docs\",\"phone\",\"chat\",\"email\",\"meeting\",\"aic\"]}},\"parameters\":{}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"task.created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"user@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"object\\\": {\\n      \\\"task_id\\\": \\\"12321312323\\\",\\n      \\\"title\\\": \\\"Complete project documentation\\\",\\n      \\\"description\\\": \\\"Write comprehensive documentation for the new API endpoints\\\",\\n      \\\"status\\\": \\\"To do\\\",\\n      \\\"priority\\\": \\\"Medium\\\",\\n      \\\"is_public\\\": true,\\n      \\\"due_date\\\": \\\"2025-08-22\\\",\\n      \\\"source_type\\\": \\\"user\\\",\\n      \\\"starred\\\": false,\\n      \\\"collaborators\\\": [\\n        {\\n          \\\"user_id\\\": \\\"w0RChiauQeqRlv5fgxYULQ\\\",\\n          \\\"email\\\": \\\"userB@example.com\\\"\\n        }\\n      ],\\n      \\\"create_time\\\": \\\"2025-03-25T07:29:29Z\\\",\\n      \\\"friendly_task_id\\\": \\\"Task-1\\\",\\n      \\\"link\\\": \\\"https://tasks.zoomdev.us/ws53iQgOOkOdzVTxZaVoRA/TASK-1\\\",\\n      \\\"assignees\\\": [\\n        {\\n          \\\"user_id\\\": \\\"w0RChiauQeqRlv5fgxYULQ\\\",\\n          \\\"email\\\": \\\"userB@example.com\\\"\\n        }\\n      ]\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"account_id\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who created the task.\"},\"operator\":{\"type\":\"string\",\"description\":\"The email of the user who created the task.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The user ID of the user who created the task.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the task.\",\"required\":[\"create_time\",\"is_public\",\"priority\",\"starred\",\"status\",\"task_id\",\"title\",\"source_type\"],\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The task ID.\"},\"description\":{\"type\":\"string\",\"description\":\"The task's detailed description.\"},\"title\":{\"type\":\"string\",\"description\":\"The task's title.\"},\"priority\":{\"$ref\":\"#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/PriorityId-Tasks\"},\"status\":{\"$ref\":\"#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/StatusId-Tasks\"},\"is_public\":{\"$ref\":\"#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/IsPublic-Tasks\"},\"due_date\":{\"$ref\":\"#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/DueDate-Tasks\"},\"source_type\":{\"$ref\":\"#/paths/task.created/post/requestBody/content/application~1json/schema/components/schemas/source_type-Tasks\"},\"starred\":{\"type\":\"boolean\",\"description\":\"Indicates whether the task is marked as starred.\"},\"collaborators\":{\"type\":\"array\",\"description\":\"A list of collaborators to invite to the task. Maximum user count is 20.\",\"items\":{\"type\":\"object\",\"required\":[\"user_id\",\"email\"],\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The collaborator's user to invite. The invited user must belong to the same account as the requesting user.\"},\"email\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The collaborator's email address.\"}}}},\"create_time\":{\"type\":\"string\",\"description\":\"The creation date-time in UTC/GMT. \",\"format\":\"date-time\",\"example\":\"2025-03-25T07:29:29Z\"},\"friendly_task_id\":{\"type\":\"string\",\"description\":\"The task's human-readable identifier.\",\"example\":\"Task-1\"},\"link\":{\"type\":\"string\",\"description\":\"The direct URL link to the task.\",\"example\":\"https://tasks.example.com/ws53iQgOOkOdzVTxZaVoRA/TASK-1\"},\"assignees\":{\"type\":\"array\",\"description\":\"A list of assignees to add to the task.\",\"items\":{\"required\":[\"user_id\",\"email\"],\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The assignee's Zoom user ID.\"},\"email\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The assignee's email address.\"}}}}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false,\"components\":{\"schemas\":{\"PriorityId-Tasks\":{\"type\":\"string\",\"description\":\"The task's priority level.\",\"example\":\"Medium\",\"default\":\"Medium\",\"enum\":[\"Low\",\"Medium\",\"High\",\"Highest\"]},\"StatusId-Tasks\":{\"type\":\"string\",\"description\":\"The task's current status.\",\"example\":\"To do\",\"default\":\"To do\",\"enum\":[\"To do\",\"In progress\",\"Done\",\"Blocked\",\"Recommended\"]},\"DueDate-Tasks\":{\"type\":\"string\",\"description\":\"The task's due date in UTC, formatted as yyyy-MM-dd.\",\"format\":\"date\",\"example\":\"2025-09-24\"},\"IsPublic-Tasks\":{\"type\":\"boolean\",\"description\":\"The task's visibility.\\n- true - The task is accessible to all members of your organization.\\n- false - The task is accessible only to the assigned users and collaborators.\",\"example\":false,\"default\":false},\"source_type-Tasks\":{\"type\":\"string\",\"description\":\"Type of the source that created the task.\",\"example\":\"user\",\"enum\":[\"user\",\"aic_meeting_summary\",\"whiteboard\",\"notes\",\"docs\",\"phone\",\"chat\",\"email\",\"meeting\",\"aic\"]}},\"parameters\":{}}},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"task.created\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"user@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"object\\\": {\\n      \\\"task_id\\\": \\\"12321312323\\\",\\n      \\\"title\\\": \\\"Complete project documentation\\\",\\n      \\\"description\\\": \\\"Write comprehensive documentation for the new API endpoints\\\",\\n      \\\"status\\\": \\\"To do\\\",\\n      \\\"priority\\\": \\\"Medium\\\",\\n      \\\"is_public\\\": true,\\n      \\\"due_date\\\": \\\"2025-08-22\\\",\\n      \\\"source_type\\\": \\\"user\\\",\\n      \\\"starred\\\": false,\\n      \\\"collaborators\\\": [\\n        {\\n          \\\"user_id\\\": \\\"w0RChiauQeqRlv5fgxYULQ\\\",\\n          \\\"email\\\": \\\"userB@example.com\\\"\\n        }\\n      ],\\n      \\\"create_time\\\": \\\"2025-03-25T07:29:29Z\\\",\\n      \\\"friendly_task_id\\\": \\\"Task-1\\\",\\n      \\\"link\\\": \\\"https://tasks.zoomdev.us/ws53iQgOOkOdzVTxZaVoRA/TASK-1\\\",\\n      \\\"assignees\\\": [\\n        {\\n          \\\"user_id\\\": \\\"w0RChiauQeqRlv5fgxYULQ\\\",\\n          \\\"email\\\": \\\"userB@example.com\\\"\\n        }\\n      ]\\n    }\\n  }\\n}\"}}}}}}}},\"task.assignee_updated\":{\"post\":{\"tags\":[\"task\"],\"operationId\":\"task.assignee_updated\",\"requestBody\":{\"description\":\"# task.assignee_updated\\n\\nThe **Task assignee were updated** event is triggered every time a task assignee is added or deleted to share a task with a Zoom user\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Task assignee were updated** subscription enabled under the **Task** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_assignee:read`,`tasks_assignee:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:assignees`,`tasks:read:assignees:admin`\\n\\n**Event type**: `task.assignee_updated`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"operation_type\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who updated assignee to the task \"},\"operator\":{\"type\":\"string\",\"description\":\"The email of the user who updated assignee to the task.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the operator who updated assignee to the task.\"},\"operation_type\":{\"type\":\"integer\",\"description\":\"Operation type. \\nEnum: \\n*0 - REMOVE \\n*1 - ADD \\n\",\"enum\":[0,1],\"x-enum-descriptions\":[\"REMOVE\",\"ADDED\"]},\"object\":{\"type\":\"object\",\"description\":\"Information about the task.\",\"required\":[\"task_id\",\"assignees\"],\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The task ID.\"},\"assignees\":{\"type\":\"array\",\"description\":\"Assignees that has been added or removed\",\"items\":{\"anyOf\":[{\"type\":\"object\",\"required\":[\"user_id\",\"display_name\"],\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the collaborator. \"},\"display_name\":{\"type\":\"string\",\"description\":\"The display name of the collaborator.\"},\"email\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the collaborator.\"},\"avatar\":{\"type\":\"string\",\"format\":\"uri\",\"description\":\"The avatar image URL of the collaborator.\"}},\"additionalProperties\":false}]},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"task.assignee_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"user@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"operation_type\\\": 1,\\n    \\\"object\\\": {\\n      \\\"task_id\\\": \\\"X4Hy02w3QUOdskKofgb9Jg\\\",\\n      \\\"assignees\\\": [\\n        {\\n          \\\"user_id\\\": \\\"CcrEGgmeQem1uyJsuIRKwA\\\",\\n          \\\"display_name\\\": \\\"John Doe\\\",\\n          \\\"email\\\": \\\"john.doe@example.com\\\",\\n          \\\"avatar\\\": \\\"https://example.com/avatar.jpg\\\"\\n        }\\n      ]\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp at which the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"operation_type\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who updated assignee to the task \"},\"operator\":{\"type\":\"string\",\"description\":\"The email of the user who updated assignee to the task.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the operator who updated assignee to the task.\"},\"operation_type\":{\"type\":\"integer\",\"description\":\"Operation type. \\nEnum: \\n*0 - REMOVE \\n*1 - ADD \\n\",\"enum\":[0,1],\"x-enum-descriptions\":[\"REMOVE\",\"ADDED\"]},\"object\":{\"type\":\"object\",\"description\":\"Information about the task.\",\"required\":[\"task_id\",\"assignees\"],\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The task ID.\"},\"assignees\":{\"type\":\"array\",\"description\":\"Assignees that has been added or removed\",\"items\":{\"anyOf\":[{\"type\":\"object\",\"required\":[\"user_id\",\"display_name\"],\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the collaborator. \"},\"display_name\":{\"type\":\"string\",\"description\":\"The display name of the collaborator.\"},\"email\":{\"type\":\"string\",\"format\":\"email\",\"description\":\"The email address of the collaborator.\"},\"avatar\":{\"type\":\"string\",\"format\":\"uri\",\"description\":\"The avatar image URL of the collaborator.\"}},\"additionalProperties\":false}]},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"task.assignee_updated\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"user@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"operation_type\\\": 1,\\n    \\\"object\\\": {\\n      \\\"task_id\\\": \\\"X4Hy02w3QUOdskKofgb9Jg\\\",\\n      \\\"assignees\\\": [\\n        {\\n          \\\"user_id\\\": \\\"CcrEGgmeQem1uyJsuIRKwA\\\",\\n          \\\"display_name\\\": \\\"John Doe\\\",\\n          \\\"email\\\": \\\"john.doe@example.com\\\",\\n          \\\"avatar\\\": \\\"https://example.com/avatar.jpg\\\"\\n        }\\n      ]\\n    }\\n  }\\n}\"}}}}}}}},\"task.reopened\":{\"post\":{\"tags\":[\"task\"],\"operationId\":\"task.reopened\",\"requestBody\":{\"description\":\"# task.reopened\\n\\nThe **task was reopened** event is triggered every time a task is opened.\\n\\n## Prerequisites\\n\\n* **Event Subscriptions** must be enabled for your [Marketplace app](https://marketplace.zoom.us/user/build) with the following configurations:\\n\\t* A valid **Event Notification Endpoint URL**.\\n\\t* The **Task was reopened** subscription enabled under the **Task** event.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:admin`,`tasks:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:task`,`tasks:read:task:admin`\\n\\n**Event type**: `task.reopened`\\n\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who opened the task.\"},\"operator\":{\"type\":\"string\",\"description\":\"The email of the user who opened the task.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the operator who opened the task.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the task.\",\"required\":[\"task_id\"],\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The task ID.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"task.reopened\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"user@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"object\\\": {\\n      \\\"task_id\\\": \\\"X4Hy02w3QUOdskKofgb9Jg\\\"\\n    }\\n  }\\n}\"}}}}},\"responses\":{\"200\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"required\":[\"event\",\"event_ts\",\"payload\"],\"properties\":{\"event\":{\"type\":\"string\",\"description\":\"The name of the event.\"},\"event_ts\":{\"type\":\"integer\",\"format\":\"int64\",\"description\":\"A timestamp of when the event occurred.\"},\"payload\":{\"type\":\"object\",\"required\":[\"account_id\",\"operator\",\"operator_id\",\"object\"],\"properties\":{\"account_id\":{\"type\":\"string\",\"description\":\"The account ID of the user who opened the task.\"},\"operator\":{\"type\":\"string\",\"description\":\"The email of the user who opened the task.\"},\"operator_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the operator who opened the task.\"},\"object\":{\"type\":\"object\",\"description\":\"Information about the task.\",\"required\":[\"task_id\"],\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The task ID.\"}},\"additionalProperties\":false}},\"additionalProperties\":false}},\"additionalProperties\":false},\"examples\":{\"json-example\":{\"summary\":\"JSON example\",\"value\":\"{\\n  \\\"event\\\": \\\"task.reopened\\\",\\n  \\\"event_ts\\\": 1626230691572,\\n  \\\"payload\\\": {\\n    \\\"account_id\\\": \\\"AAAAAABBBB\\\",\\n    \\\"operator\\\": \\\"user@example.com\\\",\\n    \\\"operator_id\\\": \\\"z8yCxjabcdEFGHfp8uQ\\\",\\n    \\\"object\\\": {\\n      \\\"task_id\\\": \\\"X4Hy02w3QUOdskKofgb9Jg\\\"\\n    }\\n  }\\n}\"}}}}}}}}},\"servers\":[{\"url\":\"https://{your-endpoint-url}\"}]}",
    "frontmatter": {
      "title": "Tasks Webhooks",
      "keywords": "zoom, rest, api, ai, aic, tasks",
      "description": "Tasks events",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
