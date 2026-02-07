# Tasks APIs

- Source URL: https://developers.zoom.us/docs/api/tasks/
- Snapshot path: docs/api/tasks
- Generated (UTC): 2026-02-07T22:40:11.645811+00:00

## Frontmatter

```json
{
  "title": "Tasks APIs",
  "keywords": "zoom, rest, api, ai, aic, tasks",
  "description": "The Zoom Tasks APIs let you manage and automate tasks within the Zoom ecosystem. Create, view, update, and delete tasks that help teams stay organized across meetings and projects. The Zoom Tasks REST API lets developers programmatically integrate task management into their workflows, enabling seamless coordination and productivity directly through Zoom.",
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
    "title": "Tasks",
    "description": "The Zoom Tasks APIs let you manage and automate tasks within the Zoom ecosystem. Create, view, update, and delete tasks that help teams stay organized across meetings and projects. The Zoom Tasks REST API lets developers programmatically integrate task management into their workflows, enabling seamless coordination and productivity directly through Zoom.",
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
    "/tasks/items/{taskId}/assignees": {
      "get": {
        "tags": [
          "Assignee"
        ],
        "summary": "Get assignees of a task",
        "description": "Retrieves the list of assignees of a specified task.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_assignee:read:admin`,`tasks_assignee:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:assignees`,`tasks:read:assignees:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "GetAssigneesOfATask",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Assignees retrieved successfully.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "task_id": {
                      "type": "string",
                      "description": "The unique ID of the task.",
                      "example": "z2vo9qNRRHKArlJqkVHNUw"
                    },
                    "assignees": {
                      "type": "array",
                      "description": "A list of all assignees associated with the task. ",
                      "items": {
                        "required": [
                          "display_name",
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The Zoom user ID of the task assignee.",
                            "example": "CcrEGgmeQem1uyJsuIRKwA"
                          },
                          "display_name": {
                            "type": "string",
                            "description": "The display name of the assignee.",
                            "example": "John Doe"
                          },
                          "email": {
                            "type": "string",
                            "description": "The email address of the assignee.",
                            "format": "email",
                            "example": "john.doe@example.com"
                          },
                          "avatar": {
                            "type": "string",
                            "description": "The assignee's avatar image URL.",
                            "format": "uri",
                            "example": "https://example.com/avatar.jpg"
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You don't have permission to request the task <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The resource not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Read"
          ],
          "x-macro-scopes": [
            "tasks_assignee:read:admin",
            "tasks_assignee:read"
          ],
          "x-granular-scopes": [
            "tasks:read:assignees",
            "tasks:read:assignees:admin"
          ]
        }
      },
      "post": {
        "tags": [
          "Assignee"
        ],
        "summary": "Add assignees to a task",
        "description": "Add one or more assignees to a specified task. If a user is an assignee, they are also considered a collaborator.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_assignee:write`,`tasks_assignee:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:write:assignees`,`tasks:write:assignees:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "addTasksAssignees",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          }
        ],
        "requestBody": {
          "description": "Request body containing the assignees to add, along with optional invitation details.",
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "assignees"
                ],
                "type": "object",
                "properties": {
                  "invite_message": {
                    "maxLength": 200,
                    "type": "string",
                    "description": "An optional message to include with the assignee invitation.",
                    "example": "Please help with this task."
                  },
                  "skip_notifications": {
                    "type": "boolean",
                    "description": "Indicates whether to skip sending notifications to the added assignees. By default, notifications are sent.",
                    "example": false,
                    "default": false
                  },
                  "assignees": {
                    "maxItems": 20,
                    "minItems": 1,
                    "type": "array",
                    "description": "The email address of the assignee to add. The user must belong to the same account as the requesting user.",
                    "items": {
                      "required": [
                        "email"
                      ],
                      "type": "object",
                      "properties": {
                        "email": {
                          "type": "string",
                          "description": "The email address of the assignee to invite.",
                          "format": "email",
                          "example": "example@zoom.us"
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "Assignees added successfully.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "assignees",
                    "task_id"
                  ],
                  "type": "object",
                  "properties": {
                    "task_id": {
                      "type": "string",
                      "description": "The unique ID of the task.",
                      "example": "z2vo9qNRRHKArlJqkVHNUw"
                    },
                    "assignees": {
                      "type": "array",
                      "description": "A list of all assignees associated with the task. ",
                      "items": {
                        "required": [
                          "display_name",
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The Zoom user ID of the assignee.",
                            "example": "CcrEGgmeQem1uyJsuIRKwA"
                          },
                          "display_name": {
                            "type": "string",
                            "description": "The display name of the assignee.",
                            "example": "John Doe"
                          },
                          "email": {
                            "type": "string",
                            "description": "The email address of the assignee.",
                            "format": "email",
                            "example": "john.doe@example.com"
                          },
                          "avatar": {
                            "type": "string",
                            "description": "The avatar image URL of the assignee.",
                            "format": "uri",
                            "example": "https://example.com/avatar.jpg"
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
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid email {email}.  <br>\n"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You are no permission to request the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The resource not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Edit"
          ],
          "x-macro-scopes": [
            "tasks_assignee:write",
            "tasks_assignee:write:admin"
          ],
          "x-granular-scopes": [
            "tasks:write:assignees",
            "tasks:write:assignees:admin"
          ]
        }
      }
    },
    "/tasks/items/{taskId}/assignees/{userId}": {
      "delete": {
        "tags": [
          "Assignee"
        ],
        "summary": "Remove Assignee from task",
        "description": "Removes a specific assignee from a task. \n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_assignee:write`,`tasks_assignee:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:delete:assignees`,`tasks:delete:assignees:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "removeTaskAssignee",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          },
          {
            "name": "userId",
            "in": "path",
            "description": "The Zoom user ID of the task assignee.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "CcrEGgmeQem1uyJsuIRKwA"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "Collaborator removed successfully"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You do not have permission to operate on the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The task not found. <br>\n**Error Code:** `112100` <br>\n The assignee not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Edit"
          ],
          "x-macro-scopes": [
            "tasks_assignee:write",
            "tasks_assignee:write:admin"
          ],
          "x-granular-scopes": [
            "tasks:delete:assignees",
            "tasks:delete:assignees:admin"
          ]
        }
      }
    },
    "/tasks/items/{taskId}/collaborators": {
      "get": {
        "tags": [
          "Collaborator"
        ],
        "summary": "Get collaborators of a task",
        "description": "Retrieves all collaborators of a specified task.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_collaborator:read:admin`,`tasks_collaborator:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:list_collaborators`,`tasks:read:list_collaborators:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Getcollaboratorsofatask",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique ID of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Collaborators retrieved successfully.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "task_id": {
                      "type": "string",
                      "description": "The unique ID of the task.",
                      "example": "z2vo9qNRRHKArlJqkVHNUw"
                    },
                    "collaborators": {
                      "type": "array",
                      "description": "A list of all collaborators associated with the task.",
                      "items": {
                        "required": [
                          "display_name",
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The Zoom user ID of the collaborator.",
                            "example": "CcrEGgmeQem1uyJsuIRKwA"
                          },
                          "display_name": {
                            "type": "string",
                            "description": "The display name of the collaborator.",
                            "example": "John Doe"
                          },
                          "email": {
                            "type": "string",
                            "description": "The email address of the collaborator.",
                            "format": "email",
                            "example": "john.doe@example.com"
                          },
                          "avatar": {
                            "type": "string",
                            "description": "The avatar image URL of the collaborator.",
                            "format": "uri",
                            "example": "https://example.com/avatar.jpg"
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You don't have permission to request the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The resource not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Read"
          ],
          "x-macro-scopes": [
            "tasks_collaborator:read:admin",
            "tasks_collaborator:read"
          ],
          "x-granular-scopes": [
            "tasks:read:list_collaborators",
            "tasks:read:list_collaborators:admin"
          ]
        }
      },
      "post": {
        "tags": [
          "Collaborator"
        ],
        "summary": "Add collaborators to a task",
        "description": "Adds one or more collaborators to a specified task. \n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_collaborator:write`,`tasks_collaborator:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:write:collaborators`,`tasks:write:collaborators:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "addTasksCollaborators",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          }
        ],
        "requestBody": {
          "description": "Request body containing the collaborators to add, along with optional invitation details.",
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "collaborators"
                ],
                "type": "object",
                "properties": {
                  "invite_message": {
                    "maxLength": 200,
                    "type": "string",
                    "description": "An optional message to include with the collaborator invitation.",
                    "example": "Please help with this task."
                  },
                  "skip_notifications": {
                    "type": "boolean",
                    "description": "Indicates whether to skip sending notifications to the added collaborators.\n By default, notifications are sent.",
                    "example": false
                  },
                  "collaborators": {
                    "maxItems": 20,
                    "minItems": 1,
                    "type": "array",
                    "description": "A list of collaborators to add to the task. The user must belong to the same account as the requesting user.",
                    "items": {
                      "required": [
                        "email"
                      ],
                      "type": "object",
                      "properties": {
                        "email": {
                          "type": "string",
                          "description": "The email address of the collaborator to invite.",
                          "format": "email",
                          "example": "example@zoom.us"
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "Collaborators added successfully.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "collaborators",
                    "task_id"
                  ],
                  "type": "object",
                  "properties": {
                    "task_id": {
                      "type": "string",
                      "description": "The unique ID of the task.",
                      "example": "z2vo9qNRRHKArlJqkVHNUw"
                    },
                    "collaborators": {
                      "type": "array",
                      "description": "A list of all collaborators associated with the task.",
                      "items": {
                        "required": [
                          "display_name",
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The Zoom user ID of the collaborator.",
                            "example": "CcrEGgmeQem1uyJsuIRKwA"
                          },
                          "display_name": {
                            "type": "string",
                            "description": "The display name of the collaborator.",
                            "example": "John Doe"
                          },
                          "email": {
                            "type": "string",
                            "description": "The email address of the collaborator.",
                            "format": "email",
                            "example": "john.doe@example.com"
                          },
                          "avatar": {
                            "type": "string",
                            "description": "The avatar image URL of the collaborator.",
                            "format": "uri",
                            "example": "https://example.com/avatar.jpg"
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
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid email {email}.  <br>\n"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You are no permission to request the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The resource not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Edit"
          ],
          "x-macro-scopes": [
            "tasks_collaborator:write",
            "tasks_collaborator:write:admin"
          ],
          "x-granular-scopes": [
            "tasks:write:collaborators",
            "tasks:write:collaborators:admin"
          ]
        }
      }
    },
    "/tasks/items/{taskId}/collaborators/{userId}": {
      "delete": {
        "tags": [
          "Collaborator"
        ],
        "summary": "Remove collaborator from task",
        "description": "Removes a specific collaborator from a task. This operation will completely remove the user's access to the task.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_collaborator:write`,`tasks_collaborator:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:delete:collaborator:admin`,`tasks:delete:collaborator`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "removeTaskCollaborator",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique ID of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          },
          {
            "name": "userId",
            "in": "path",
            "description": "Zoom user ID of the collaborator.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "CcrEGgmeQem1uyJsuIRKwA"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "Collaborator removed successfully"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You do not have permission to operate on the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The task not found. <br>\n**Error Code:** `112100` <br>\n The assignee not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Edit"
          ],
          "x-macro-scopes": [
            "tasks_collaborator:write",
            "tasks_collaborator:write:admin"
          ],
          "x-granular-scopes": [
            "tasks:delete:collaborator:admin",
            "tasks:delete:collaborator"
          ]
        }
      }
    },
    "/tasks/items/{taskId}/comments": {
      "get": {
        "tags": [
          "Comment"
        ],
        "summary": "Get a task's comments",
        "description": "Retrieves all comments associated with a specific task.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_comment:read`,`tasks_comment:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:comments`,`tasks:read:comments:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "GetAV1TasksComment",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of records returned from a single API call.",
            "required": false,
            "schema": {
              "maximum": 50,
              "type": "integer",
              "example": 10,
              "default": 10
            }
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "The next page token used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Task comments retrieved successfully.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "comments": {
                      "type": "array",
                      "description": "The list of comments associated with the task.",
                      "items": {
                        "type": "object",
                        "properties": {
                          "comment_id": {
                            "type": "string",
                            "description": "The unique identifier of the comment.",
                            "example": "comment_abc123def456"
                          },
                          "content": {
                            "type": "array",
                            "description": "The array of content fragments that make up the comment. Each fragment must define its order and contain either plain text or a mention. Fragments may also include attributes such as a hyperlink.",
                            "items": {
                              "required": [
                                "order"
                              ],
                              "type": "object",
                              "properties": {
                                "text": {
                                  "type": "string",
                                  "description": "The plain text fragment. May include letters, numbers, symbols, or emoji. Required if no `mention` is provided. If combined with `attributes.link`, this text is rendered as a hyperlink.",
                                  "example": "the text demo"
                                },
                                "order": {
                                  "type": "number",
                                  "description": "The position of the fragment within the comment. Must be unique within the `content` array. Defines the sequence for rendering.",
                                  "example": 0
                                },
                                "attributes": {
                                  "type": "object",
                                  "properties": {
                                    "link": {
                                      "type": "string",
                                      "description": "The URL associated with the fragment's text. Typically an absolute `http://` or `https://` link.",
                                      "example": "https://google.cn"
                                    }
                                  },
                                  "description": "Optional metadata for the fragment. Currently supports only hyperlinks."
                                },
                                "mention": {
                                  "type": "object",
                                  "properties": {
                                    "email": {
                                      "type": "string",
                                      "description": "The email address of the user being mentioned.",
                                      "example": "example@zoom.us"
                                    },
                                    "user_id": {
                                      "type": "string",
                                      "description": "The user ID of the user being mentioned.",
                                      "example": "3afwV0HiZS9mCLpxSQQbVz"
                                    },
                                    "display_name": {
                                      "type": "string",
                                      "description": "The display name of the user.",
                                      "example": "John Doe"
                                    },
                                    "avatar": {
                                      "type": "string",
                                      "description": "The avatar image URL of the user.",
                                      "format": "uri",
                                      "example": "https://example.com/avatar.jpg"
                                    }
                                  },
                                  "description": "Represents a mention of an entity (e.g., a user). If present, the fragment represents a mention instead of plain text."
                                }
                              }
                            }
                          },
                          "content_text": {
                            "type": "string",
                            "description": "A plain text version of the entire comment content.",
                            "example": "the text demo"
                          },
                          "create_time": {
                            "type": "integer",
                            "description": "Creation timestamp in milliseconds.",
                            "format": "int64",
                            "example": 1752715586000
                          }
                        },
                        "description": "A comment is represented as an ordered array of fragments. Each fragment may contain plain text, a mention, or a text snippet with a hyperlink."
                      }
                    },
                    "next_page_token": {
                      "type": "string",
                      "description": "Use the next page token to paginate through a large set of results. A next page token is returned when the available results exceed the current page size. This token's expiration period is 15 minutes.",
                      "example": "qUEQqB1V0HVhJmwKFQrGOD"
                    },
                    "page_size": {
                      "type": "integer",
                      "description": "The number of records returned per page in a single API call.",
                      "example": 10
                    },
                    "total_records": {
                      "type": "integer",
                      "description": "The total number of records found.",
                      "example": 2
                    }
                  }
                }
              }
            }
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You don't have permission to operate on the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The task not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Read",
            "TasksContent:Read"
          ],
          "x-macro-scopes": [
            "tasks_comment:read",
            "tasks_comment:read:admin"
          ],
          "x-granular-scopes": [
            "tasks:read:comments",
            "tasks:read:comments:admin"
          ]
        }
      },
      "post": {
        "tags": [
          "Comment"
        ],
        "summary": "Add a comment to task",
        "description": "Adds a comment to a specific task.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_comment:write`,`tasks_comment:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:write:comment:admin`,`tasks:write:comment`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "addComment",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "content",
                  "content_text"
                ],
                "type": "object",
                "properties": {
                  "content": {
                    "maxItems": 100,
                    "minItems": 1,
                    "type": "array",
                    "description": "The array of content fragments that make up the comment. Each fragment must define its order and contain either plain text or a mention. Fragments may also include attributes such as hyperlinks.",
                    "items": {
                      "required": [
                        "order"
                      ],
                      "type": "object",
                      "properties": {
                        "text": {
                          "type": "string",
                          "description": "A plain text fragment that may include letters, numbers, symbols, or emoji. Required if no mention is provided. If combined with attributes.link, this text is rendered as a hyperlink.",
                          "example": "the text demo"
                        },
                        "order": {
                          "type": "number",
                          "description": "The position of the fragment within the comment.\nMust be unique within the `content` array.\nDefines the sequence for rendering.",
                          "example": 0
                        },
                        "attributes": {
                          "type": "object",
                          "properties": {
                            "link": {
                              "type": "string",
                              "description": "The URL associated with the fragment's text, typically an absolute http:// or https:// link.",
                              "example": "https://google.cn"
                            }
                          },
                          "description": "Optional metadata for the fragment. Currently supports only hyperlinks."
                        },
                        "mention": {
                          "type": "object",
                          "properties": {
                            "email": {
                              "type": "string",
                              "description": "The email address of the user being mentioned.",
                              "format": "email",
                              "example": "example@zoom.us"
                            },
                            "user_id": {
                              "type": "string",
                              "description": "The user ID of the user being mentioned.",
                              "example": "3afwV0HiZS9mCLpxSQQbV"
                            }
                          },
                          "description": "Represents a user mention. Provide either the email or the user ID. If present, the fragment will be treated as a mention instead of plain text."
                        }
                      }
                    }
                  },
                  "content_text": {
                    "maxLength": 2000,
                    "minLength": 1,
                    "type": "string",
                    "description": "A plain text version of the entire comment content.",
                    "example": "the text demo"
                  }
                },
                "description": "Request body for adding a comment. A comment is represented as an ordered array of fragments. Each fragment may contain plain text, a mention, or a text snippet with a hyperlink."
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "Comment added successfully",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "comment_id",
                    "content"
                  ],
                  "type": "object",
                  "properties": {
                    "comment_id": {
                      "type": "string",
                      "description": "The unique identifier of the created comment.",
                      "example": "comment_abc123def456"
                    },
                    "content": {
                      "type": "array",
                      "description": "The array of content fragments that make up the comment.\nEach fragment must define its order and contain either plain text or a mention.\nFragments may also include attributes such as a hyperlink.",
                      "items": {
                        "required": [
                          "order"
                        ],
                        "type": "object",
                        "properties": {
                          "text": {
                            "type": "string",
                            "description": "A plain text fragment that may include letters, numbers, symbols, or emoji. Required if no mention is provided. If combined with attributes.link, this text is rendered as a hyperlink.",
                            "example": "the text demo"
                          },
                          "order": {
                            "type": "number",
                            "description": "The position of the fragment within the comment.\nMust be unique within the `content` array.\nDefines the sequence for rendering.",
                            "example": 0
                          },
                          "attributes": {
                            "type": "object",
                            "properties": {
                              "link": {
                                "type": "string",
                                "description": "The URL associated with the fragment's text, typically an absolute http:// or https:// link.",
                                "example": "https://google.cn"
                              }
                            },
                            "description": "Optional metadata for the fragment. Currently supports only hyperlinks."
                          },
                          "mention": {
                            "type": "object",
                            "properties": {
                              "email": {
                                "type": "string",
                                "description": "The email address of the user being mentioned.",
                                "example": "example@zoom.us"
                              },
                              "user_id": {
                                "type": "string",
                                "description": "The user ID of the user being mentioned.",
                                "example": "3afwV0HiZS9mCLpxSQQbV"
                              },
                              "display_name": {
                                "type": "string",
                                "description": "The display name of the user.",
                                "example": "John Doe"
                              },
                              "avatar": {
                                "type": "string",
                                "description": "The avatar image URL of the user.",
                                "format": "uri",
                                "example": "https://example.com/avatar.jpg"
                              }
                            },
                            "description": "Represents a mention of an entity (e.g., a user). If present, the fragment represents a mention instead of plain text."
                          }
                        }
                      }
                    },
                    "content_text": {
                      "type": "string",
                      "description": "A plain text version of the entire comment content.",
                      "example": "the text demo"
                    },
                    "create_time": {
                      "type": "integer",
                      "description": "Creation timestamp in milliseconds.",
                      "format": "int64",
                      "example": 1752715586000
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid request body. Please verify the content and try again. <br>\n"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You don't have permission to request the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The resource not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Edit"
          ],
          "x-macro-scopes": [
            "tasks_comment:write",
            "tasks_comment:write:admin"
          ],
          "x-granular-scopes": [
            "tasks:write:comment:admin",
            "tasks:write:comment"
          ]
        }
      }
    },
    "/tasks/items/{taskId}/comments/{commentId}": {
      "delete": {
        "tags": [
          "Comment"
        ],
        "summary": "Delete a task's comment",
        "description": "Deletes a specific comment from a task by its ID.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_comment:write`,`tasks_comment:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:delete:comment:admin`,`tasks:delete:comment`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "DeleteTaskComment",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          },
          {
            "name": "commentId",
            "in": "path",
            "description": "The unique identifier of the comment to delete.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "c1vo9qNRRHKArlJqkVHNUx"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "Comment deleted successfully. No content is returned."
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You don't have permission to operate on the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The task not found. <br>\n**Error Code:** `112100` <br>\n The comment not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Edit"
          ],
          "x-macro-scopes": [
            "tasks_comment:write",
            "tasks_comment:write:admin"
          ],
          "x-granular-scopes": [
            "tasks:delete:comment:admin",
            "tasks:delete:comment"
          ]
        }
      }
    },
    "/tasks/items": {
      "get": {
        "tags": [
          "Tasks"
        ],
        "summary": "List tasks",
        "description": "Retrieves tasks assigned to, or created by, the current user with advanced filtering, sorting, and pagination options.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read`,`tasks:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:list_tasks:admin`,`tasks:read:list_tasks`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "getMyTasks",
        "parameters": [
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of records returned from a single API call.",
            "required": false,
            "schema": {
              "maximum": 50,
              "type": "integer",
              "example": 10,
              "default": 10
            }
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42"
            }
          },
          {
            "name": "search_key",
            "in": "query",
            "description": "Search keyword for task title and description.",
            "required": false,
            "schema": {
              "maxLength": 50,
              "type": "string",
              "example": "project documentation"
            }
          },
          {
            "name": "sort_by",
            "in": "query",
            "description": "Determines the order in which tasks are returned.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "due_date",
              "default": "due_date",
              "enum": [
                "update_time",
                "create_time",
                "due_date",
                "status",
                "priority"
              ]
            }
          },
          {
            "name": "sort_asc",
            "in": "query",
            "description": "Sort in ascending order (default depends on sort column)",
            "required": false,
            "schema": {
              "type": "boolean",
              "example": false
            }
          },
          {
            "name": "assignee_email",
            "in": "query",
            "description": "The email address of the assignee to search for.",
            "required": false,
            "schema": {
              "type": "string",
              "format": "email",
              "example": "assignee@email.com"
            }
          },
          {
            "name": "due_date_start",
            "in": "query",
            "description": "Start date for due date filter. \n Format: (yyyy-mm-dd)",
            "required": false,
            "schema": {
              "type": "string",
              "format": "date",
              "example": "2024-01-01"
            }
          },
          {
            "name": "due_date_end",
            "in": "query",
            "description": "End date for due date filter.\n Format: (yyyy-mm-dd)",
            "required": false,
            "schema": {
              "type": "string",
              "format": "date",
              "example": "2024-12-31"
            }
          },
          {
            "name": "unread",
            "in": "query",
            "description": "Set to only return unread tasks for the current user.",
            "required": false,
            "schema": {
              "type": "boolean",
              "example": false
            }
          },
          {
            "name": "status",
            "in": "query",
            "description": "Current status of the task.",
            "required": false,
            "schema": {
              "type": "string",
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
          {
            "name": "starred",
            "in": "query",
            "description": "Filter tasks by star status.",
            "required": false,
            "schema": {
              "type": "boolean",
              "example": true
            }
          },
          {
            "name": "source_type",
            "in": "query",
            "description": "The source that created the task.",
            "required": false,
            "schema": {
              "type": "string",
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
          }
        ],
        "responses": {
          "200": {
            "description": "Tasks retrieved successfully",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "tasks",
                    "total_records"
                  ],
                  "type": "object",
                  "properties": {
                    "next_page_token": {
                      "type": "string",
                      "description": "Use the next page token to paginate through a large set of results. A next page token is returned when the available results exceed the current page size. This token's expiration period is 15 minutes.",
                      "example": "qUEQqB1V0HVhJmwKFQrGOD"
                    },
                    "page_size": {
                      "type": "integer",
                      "description": "The number of records returned per page in a single API call.",
                      "example": 10
                    },
                    "total_records": {
                      "type": "integer",
                      "description": "The total number of records found.",
                      "example": 2
                    },
                    "tasks": {
                      "type": "array",
                      "description": "List of tasks matching the query criteria.",
                      "items": {
                        "required": [
                          "ai_generated",
                          "create_time",
                          "is_public",
                          "modify_time",
                          "priority",
                          "source_type",
                          "starred",
                          "status",
                          "task_id",
                          "title"
                        ],
                        "type": "object",
                        "properties": {
                          "task_id": {
                            "type": "string",
                            "description": "The unique identifier of the task.",
                            "example": "z2vo9qNRRHKArlJqkVHNUw"
                          },
                          "title": {
                            "type": "string",
                            "description": "The title of the task.",
                            "example": "Complete project documentation"
                          },
                          "description": {
                            "maxLength": 200,
                            "minLength": 0,
                            "type": "string",
                            "description": "A description of the task that will be truncated. \n The maximum length is 200 characters.",
                            "example": "Write comprehensive documentation for the new API endpoints..."
                          },
                          "priority": {
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
                          "status": {
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
                          "is_public": {
                            "type": "boolean",
                            "description": "Determines the task's visibility.\n* `true` - The task is accessible to all members of your organization.\n* `false` - The task is accessible only to the assigned users and collaborators.",
                            "example": false,
                            "default": false
                          },
                          "due_date": {
                            "type": "string",
                            "description": "The task's due date in UTC, formatted as `yyyy-MM-dd`.",
                            "format": "date",
                            "example": "2025-09-24"
                          },
                          "ai_generated": {
                            "type": "boolean",
                            "description": "Whether the task was generated by AI.",
                            "example": false
                          },
                          "starred": {
                            "type": "boolean",
                            "description": "Whether the task is starred by the user.",
                            "example": true
                          },
                          "unread": {
                            "type": "boolean",
                            "description": "Whether the task is unread by the current user.",
                            "example": false
                          },
                          "assignees": {
                            "maxItems": 5,
                            "minItems": 0,
                            "type": "array",
                            "description": "A list of all assignees associated with the task.\n Only the first 5 assignees are returned.",
                            "items": {
                              "required": [
                                "display_name",
                                "user_id"
                              ],
                              "type": "object",
                              "properties": {
                                "user_id": {
                                  "type": "string",
                                  "description": "The Zoom user ID of the assignee.",
                                  "example": "CcrEGgmeQem1uyJsuIRKwA"
                                },
                                "display_name": {
                                  "type": "string",
                                  "description": "The display name of the assignee.",
                                  "example": "John Doe"
                                },
                                "email": {
                                  "type": "string",
                                  "description": "The email address of the assignee.",
                                  "format": "email",
                                  "example": "john.doe@example.com"
                                },
                                "avatar": {
                                  "type": "string",
                                  "description": "The avatar image URL of the assignee.",
                                  "format": "uri",
                                  "example": "https://example.com/avatar.jpg"
                                }
                              }
                            }
                          },
                          "collaborators": {
                            "type": "array",
                            "description": "A list of all collaborators associated with the task.\n Only the first 5 collaborators are returned.",
                            "items": {
                              "required": [
                                "display_name",
                                "user_id"
                              ],
                              "type": "object",
                              "properties": {
                                "user_id": {
                                  "type": "string",
                                  "description": "The Zoom user ID of the collaborator.",
                                  "example": "CcrEGgmeQem1uyJsuIRKwA"
                                },
                                "display_name": {
                                  "type": "string",
                                  "description": "The display name of the collaborator.",
                                  "example": "John Doe"
                                },
                                "email": {
                                  "type": "string",
                                  "description": "The email address of the collaborator.",
                                  "format": "email",
                                  "example": "john.doe@example.com"
                                },
                                "avatar": {
                                  "type": "string",
                                  "description": "The avatar image URL of the collaborator.",
                                  "format": "uri",
                                  "example": "https://example.com/avatar.jpg"
                                }
                              }
                            }
                          },
                          "friendly_task_id": {
                            "type": "string",
                            "description": "The task friendly ID.",
                            "example": "Task-1"
                          },
                          "link": {
                            "type": "string",
                            "description": "The task share link.",
                            "example": "https://tasks.zoomdev.us/ws53iQgOOkOdzVTxZaVoRA/TASK-1"
                          },
                          "create_time": {
                            "type": "string",
                            "description": "The task create date-time in UTC/GMT.",
                            "format": "date-time",
                            "example": "2025-03-31T12:02:00Z"
                          },
                          "modify_time": {
                            "type": "string",
                            "description": "The task modify date-time in UTC/GMT.",
                            "format": "date-time",
                            "example": "2025-03-31T12:02:00Z"
                          },
                          "source_type": {
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
                        }
                      }
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `38004` <br>\n invalid params {params_name} <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Read",
            "TasksContent:Read"
          ],
          "x-macro-scopes": [
            "tasks:read",
            "tasks:read:admin"
          ],
          "x-granular-scopes": [
            "tasks:read:list_tasks:admin",
            "tasks:read:list_tasks"
          ]
        }
      },
      "post": {
        "tags": [
          "Tasks"
        ],
        "summary": "Create a new task",
        "description": "Provide essential details to create a new task without file attachments.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:write:task`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "createTask",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "title"
                ],
                "type": "object",
                "properties": {
                  "title": {
                    "maxLength": 500,
                    "minLength": 1,
                    "type": "string",
                    "description": "The tasks's title. Maximum 500 characters.",
                    "example": "Complete project documentation"
                  },
                  "description": {
                    "maxLength": 10000,
                    "minLength": 1,
                    "type": "string",
                    "description": "The task's detailed description. Maximum 10,000 characters.",
                    "example": "Write comprehensive documentation for the new API endpoints"
                  },
                  "priority": {
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
                  "status": {
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
                  "due_date": {
                    "type": "string",
                    "description": "The task's due date in UTC, formatted as `yyyy-MM-dd`.",
                    "format": "date",
                    "example": "2025-09-24"
                  },
                  "is_public": {
                    "type": "boolean",
                    "description": "Determines the task's visibility.\n* `true` - The task is accessible to all members of your organization.\n* `false` - The task is accessible only to the assigned users and collaborators.",
                    "example": false,
                    "default": false
                  },
                  "starred": {
                    "type": "boolean",
                    "description": "Specifies whether the task is starred.",
                    "example": false,
                    "default": false
                  },
                  "collaborators": {
                    "maxItems": 20,
                    "minItems": 0,
                    "type": "array",
                    "description": "A list of collaborators to invite to the task. Maximum 20.",
                    "items": {
                      "type": "object",
                      "properties": {
                        "email": {
                          "type": "string",
                          "description": "The email address of the collaborator to invite. The potential collaborator must belong to the same account as the requesting user.",
                          "example": "potential-collaborator@example.com"
                        }
                      }
                    }
                  },
                  "invite_message": {
                    "maxLength": 280,
                    "type": "string",
                    "description": "Include an optional message when inviting a user as a collaborator or assignee. Maximum length is 200 characters.",
                    "example": "Please join this task to help review the API documentation."
                  },
                  "skip_notifications": {
                    "type": "boolean",
                    "description": "Whether to skip sending notifications to collaborators.",
                    "example": false
                  },
                  "assignees": {
                    "maxItems": 20,
                    "minItems": 0,
                    "type": "array",
                    "description": "A list of assignees to add to the task. Maximum of 20. If empty, the request user is added by default.",
                    "items": {
                      "type": "object",
                      "properties": {
                        "email": {
                          "type": "string",
                          "description": "The email address of the assignee to invite. The potential assignee must belong to the same account as the requesting user.",
                          "example": "invited-assignee@example.com"
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "Task created successfully.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "create_time",
                    "is_public",
                    "priority",
                    "starred",
                    "status",
                    "task_id",
                    "title"
                  ],
                  "type": "object",
                  "properties": {
                    "task_id": {
                      "type": "string",
                      "description": "The created task's unique identifier.",
                      "example": "z2vo9qNRRHKArlJqkVHNUw"
                    },
                    "title": {
                      "type": "string",
                      "description": "The task's title.",
                      "example": "Complete project documentation"
                    },
                    "description": {
                      "type": "string",
                      "description": "The task's detailed description.",
                      "example": "Write comprehensive documentation for the new API endpoints"
                    },
                    "priority": {
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
                    "status": {
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
                    "is_public": {
                      "type": "boolean",
                      "description": "Determines the task's visibility.\n* `true` - The task is accessible to all members of your organization.\n* `false` - The task is accessible only to the assigned users and collaborators.",
                      "example": false,
                      "default": false
                    },
                    "due_date": {
                      "type": "string",
                      "description": "The task's due date in UTC, formatted as `yyyy-MM-dd`.",
                      "format": "date",
                      "example": "2025-09-24"
                    },
                    "starred": {
                      "type": "boolean",
                      "description": "Indicates whether the task is starred by the user.",
                      "example": false
                    },
                    "collaborators": {
                      "type": "array",
                      "description": "A list of collaborators associated with the task.",
                      "items": {
                        "required": [
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The collaborator's Zoom user ID.",
                            "example": "w0RChiauQeqRlv5fgxYULQ"
                          },
                          "email": {
                            "type": "string",
                            "description": "The email address of the collaborator.",
                            "example": "john.doe@example.com"
                          }
                        }
                      }
                    },
                    "assignees": {
                      "type": "array",
                      "description": "A list of all assignees associated with the task. ",
                      "items": {
                        "required": [
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The assignee's Zoom user ID.",
                            "example": "w0RChiauQeqRlv5fgxYULQ"
                          },
                          "email": {
                            "type": "string",
                            "description": "The email address of the assignee.",
                            "example": "invited-assignee@example.com"
                          }
                        }
                      }
                    },
                    "create_time": {
                      "type": "string",
                      "description": "The creation date and time in UTC/GMT format. ",
                      "format": "date-time",
                      "example": "2025-03-25T07:29:29Z"
                    },
                    "friendly_task_id": {
                      "type": "string",
                      "description": "The human-readable identifier of the task.",
                      "example": "Task-1"
                    },
                    "link": {
                      "type": "string",
                      "description": "The direct URL link to the task.",
                      "example": "https://tasks.example.com/ws53iQgOOkOdzVTxZaVoRA/TASK-1"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid email {email}. <br>\n**Error Code:** `34002` <br>\n The title is too long. <br>\n**Error Code:** `34002` <br>\n The description is too long. <br>\n**Error Code:** `34002` <br>\n Invalid field {field_name}. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
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
            "tasks:write"
          ],
          "x-granular-scopes": [
            "tasks:write:task"
          ]
        }
      }
    },
    "/tasks/items/{taskId}": {
      "get": {
        "tags": [
          "Tasks"
        ],
        "summary": "Get task details",
        "description": "Retrieves detailed information about a specific task by it\u2019s ID\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read`,`tasks:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:task`,`tasks:read:task:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "getTaskDetail",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Task details retrieved successfully",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "create_time",
                    "is_ai_generated",
                    "is_public",
                    "modify_time",
                    "priority",
                    "source_type",
                    "starred",
                    "status",
                    "task_id",
                    "title"
                  ],
                  "type": "object",
                  "properties": {
                    "task_id": {
                      "type": "string",
                      "description": "The unique identifier of the task.",
                      "example": "z2vo9qNRRHKArlJqkVHNUw"
                    },
                    "title": {
                      "type": "string",
                      "description": "The title of the task.",
                      "example": "the tasks title demo"
                    },
                    "description": {
                      "type": "string",
                      "description": "A detailed description of the task.",
                      "example": "the task description"
                    },
                    "priority": {
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
                    "status": {
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
                    "is_public": {
                      "type": "boolean",
                      "description": "Determines the task's visibility.\n* `true` - The task is accessible to all members of your organization.\n* `false` - The task is accessible only to the assigned users and collaborators.",
                      "example": false,
                      "default": false
                    },
                    "due_date": {
                      "type": "string",
                      "description": "The task's due date in UTC, formatted as `yyyy-MM-dd`.",
                      "format": "date",
                      "example": "2025-09-24"
                    },
                    "is_ai_generated": {
                      "type": "boolean",
                      "description": "Whether the task was generated by AI.",
                      "example": false
                    },
                    "starred": {
                      "type": "boolean",
                      "description": "Whether the task is starred by the user.",
                      "example": false
                    },
                    "tasks_attachments": {
                      "type": "array",
                      "description": "List of attachments associated with the task",
                      "example": [],
                      "items": {
                        "type": "object",
                        "properties": {
                          "attachment_id": {
                            "type": "string",
                            "description": "The unique identifier of the attachment.",
                            "example": "dz6nqEeQT6C1VoZFxteGTw"
                          },
                          "file_name": {
                            "type": "string",
                            "description": "The name of the attached file.",
                            "example": "example.pdf"
                          },
                          "file_size": {
                            "type": "integer",
                            "description": "The size of the file in bytes.",
                            "example": 1024
                          },
                          "mime_type": {
                            "type": "string",
                            "description": "The MIME type of the file.",
                            "example": "application/pdf"
                          },
                          "upload_time": {
                            "type": "string",
                            "description": "The file upload date-time in UTC/GMT.",
                            "format": "date-time",
                            "example": "2025-03-31T12:02:00Z"
                          },
                          "download_url": {
                            "type": "string",
                            "description": "The download link for the attachment will remain valid for 7,200 seconds.",
                            "example": "https://file.zoom.us/file/dz6nqEeQT6C1VoZFxteGTw?jwt=xxx"
                          }
                        }
                      }
                    },
                    "assignees": {
                      "type": "array",
                      "description": "A list of all assignees associated with the task. ",
                      "items": {
                        "required": [
                          "display_name",
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The Zoom user ID of the assignee.",
                            "example": "w0RChiauQeqRlv5fgxYULQ"
                          },
                          "display_name": {
                            "type": "string",
                            "description": "The display name of the assignee.",
                            "example": "John Doe"
                          },
                          "email": {
                            "type": "string",
                            "description": "The email address of the assignee.",
                            "format": "email",
                            "example": "john.doe@example.com"
                          },
                          "avatar": {
                            "type": "string",
                            "description": "The avatar image URL of the assignee.",
                            "format": "uri",
                            "example": "https://example.com/avatar.jpg"
                          }
                        }
                      }
                    },
                    "collaborators": {
                      "type": "array",
                      "description": "A list of all collaborators associated with the task.",
                      "items": {
                        "required": [
                          "display_name",
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The Zoom user ID of the collaborator.",
                            "example": "w0RChiauQeqRlv5fgxYULQ"
                          },
                          "display_name": {
                            "type": "string",
                            "description": "The display name of the collaborator.",
                            "example": "John Doe"
                          },
                          "email": {
                            "type": "string",
                            "description": "The email address of the collaborator.",
                            "format": "email",
                            "example": "john.doe@example.com"
                          },
                          "avatar": {
                            "type": "string",
                            "description": "The avatar image URL of the collaborator.",
                            "format": "uri",
                            "example": "https://example.com/avatar.jpg"
                          }
                        }
                      }
                    },
                    "source_type": {
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
                    },
                    "friendly_task_id": {
                      "type": "string",
                      "description": "The task friendly ID.",
                      "example": "Task-1"
                    },
                    "link": {
                      "type": "string",
                      "description": "The task share link.",
                      "example": "https://tasks.zoomdev.us/ws53iQgOOkOdzVTxZaVoRA/TASK-1"
                    },
                    "associated_meeting": {
                      "type": "object",
                      "properties": {
                        "id": {
                          "type": "integer",
                          "description": "[Meeting ID](https://support.zoom.us/hc/en-us/articles/201362373-What-is-a-Meeting-ID-): Unique identifier of the meeting in **long** format, represented as int64 data type in JSON, also known as the meeting number.",
                          "format": "int64",
                          "example": 97763643886
                        },
                        "uuid": {
                          "type": "string",
                          "description": "The unique Meeting ID. Each meeting instance will generate its own Meeting UUID.",
                          "example": "aDYlohsHRtCd4ii1uC2+hA=="
                        },
                        "topic": {
                          "type": "string",
                          "description": "The topic of the associated meeting",
                          "example": "Tasks Meeting"
                        },
                        "meeting_date": {
                          "type": "string",
                          "description": "The meeting start date-time in UTC/GMT.",
                          "format": "date-time",
                          "example": "2025-03-31T12:02:00Z"
                        }
                      },
                      "description": "Associated meeting information"
                    },
                    "create_time": {
                      "type": "string",
                      "description": "The task create date-time in UTC/GMT.",
                      "format": "date-time",
                      "example": "2025-03-31T12:02:00Z"
                    },
                    "modify_time": {
                      "type": "string",
                      "description": "The task modify date-time in UTC/GMT.",
                      "format": "date-time",
                      "example": "2025-03-31T12:02:00Z"
                    }
                  }
                }
              }
            }
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You don't have permission to request the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The resource not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Read",
            "TasksContent:Read"
          ],
          "x-macro-scopes": [
            "tasks:read",
            "tasks:read:admin"
          ],
          "x-granular-scopes": [
            "tasks:read:task",
            "tasks:read:task:admin"
          ]
        }
      },
      "delete": {
        "tags": [
          "Tasks"
        ],
        "summary": "Delete a task",
        "description": "Permanently delete a task and all its associated data including descriptions, attachments, collaborators, and integrations. This operation cannot be undone.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:delete`,`tasks:delete:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:delete:task:admin`,`tasks:delete:task`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "deleteTask",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task to delete",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "Task deleted successfully"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You don't have permission to request the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The resource not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Edit"
          ],
          "x-macro-scopes": [
            "tasks:delete",
            "tasks:delete:admin"
          ],
          "x-granular-scopes": [
            "tasks:delete:task:admin",
            "tasks:delete:task"
          ]
        }
      },
      "patch": {
        "tags": [
          "Tasks"
        ],
        "summary": "Update task fields",
        "description": "Updates specific fields of an existing task. Only provided fields will be updated, others will remain unchanged.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:write`,`tasks:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:update:task`,`tasks:update:task:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "updateTask",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task to update",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "title": {
                    "maxLength": 500,
                    "minLength": 1,
                    "type": "string",
                    "description": "The title of the task. \nMaximum 500 characters.",
                    "example": "Complete project documentation"
                  },
                  "description": {
                    "maxLength": 10000,
                    "minLength": 1,
                    "type": "string",
                    "description": "A detailed description of the task. \nMaximum 10,000 characters.",
                    "example": "Write comprehensive documentation for the new API endpoints"
                  },
                  "priority": {
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
                  "status": {
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
                  "due_date": {
                    "type": "string",
                    "description": "The task's due date in UTC, formatted as `yyyy-MM-dd`.",
                    "format": "date",
                    "example": "2025-09-24"
                  },
                  "is_public": {
                    "type": "boolean",
                    "description": "Determines the task's visibility.\n* `true` - The task is accessible to all members of your organization.\n* `false` - The task is accessible only to the assigned users and collaborators.",
                    "example": false,
                    "default": false
                  },
                  "starred": {
                    "type": "boolean",
                    "description": "Whether the task should be starred.",
                    "example": true
                  }
                }
              }
            }
          }
        },
        "responses": {
          "204": {
            "description": "Task updated successfully"
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid field {field_name}. <br>\n**Error Code:** `34002` <br>\n The title is too long. <br>\n**Error Code:** `34002` <br>\n The description is too long. <br>\n"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You don't have permission to request the task. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Edit"
          ],
          "x-macro-scopes": [
            "tasks:write",
            "tasks:write:admin"
          ],
          "x-granular-scopes": [
            "tasks:update:task",
            "tasks:update:task:admin"
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
    "title": "Tasks",
    "description": "The Zoom Tasks APIs let you manage and automate tasks within the Zoom ecosystem. Create, view, update, and delete tasks that help teams stay organized across meetings and projects. The Zoom Tasks REST API lets developers programmatically integrate task management into their workflows, enabling seamless coordination and productivity directly through Zoom.",
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
    "/tasks/items/{taskId}/assignees": {
      "get": {
        "tags": [
          "Assignee"
        ],
        "summary": "Get assignees of a task",
        "description": "Retrieves the list of assignees of a specified task.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_assignee:read:admin`,`tasks_assignee:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:assignees`,`tasks:read:assignees:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "GetAssigneesOfATask",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Assignees retrieved successfully.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "task_id": {
                      "type": "string",
                      "description": "The unique ID of the task.",
                      "example": "z2vo9qNRRHKArlJqkVHNUw"
                    },
                    "assignees": {
                      "type": "array",
                      "description": "A list of all assignees associated with the task. ",
                      "items": {
                        "required": [
                          "display_name",
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The Zoom user ID of the task assignee.",
                            "example": "CcrEGgmeQem1uyJsuIRKwA"
                          },
                          "display_name": {
                            "type": "string",
                            "description": "The display name of the assignee.",
                            "example": "John Doe"
                          },
                          "email": {
                            "type": "string",
                            "description": "The email address of the assignee.",
                            "format": "email",
                            "example": "john.doe@example.com"
                          },
                          "avatar": {
                            "type": "string",
                            "description": "The assignee's avatar image URL.",
                            "format": "uri",
                            "example": "https://example.com/avatar.jpg"
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You don't have permission to request the task <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The resource not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "tasks_assignee:read:admin",
              "tasks_assignee:read",
              "tasks:read:assignees",
              "tasks:read:assignees:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Read"
          ],
          "x-macro-scopes": [
            "tasks_assignee:read:admin",
            "tasks_assignee:read"
          ],
          "x-granular-scopes": [
            "tasks:read:assignees",
            "tasks:read:assignees:admin"
          ]
        }
      },
      "post": {
        "tags": [
          "Assignee"
        ],
        "summary": "Add assignees to a task",
        "description": "Add one or more assignees to a specified task. If a user is an assignee, they are also considered a collaborator.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_assignee:write`,`tasks_assignee:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:write:assignees`,`tasks:write:assignees:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "addTasksAssignees",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          }
        ],
        "requestBody": {
          "description": "Request body containing the assignees to add, along with optional invitation details.",
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "assignees"
                ],
                "type": "object",
                "properties": {
                  "invite_message": {
                    "maxLength": 200,
                    "type": "string",
                    "description": "An optional message to include with the assignee invitation.",
                    "example": "Please help with this task."
                  },
                  "skip_notifications": {
                    "type": "boolean",
                    "description": "Indicates whether to skip sending notifications to the added assignees. By default, notifications are sent.",
                    "example": false,
                    "default": false
                  },
                  "assignees": {
                    "maxItems": 20,
                    "minItems": 1,
                    "type": "array",
                    "description": "The email address of the assignee to add. The user must belong to the same account as the requesting user.",
                    "items": {
                      "required": [
                        "email"
                      ],
                      "type": "object",
                      "properties": {
                        "email": {
                          "type": "string",
                          "description": "The email address of the assignee to invite.",
                          "format": "email",
                          "example": "example@zoom.us"
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "Assignees added successfully.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "assignees",
                    "task_id"
                  ],
                  "type": "object",
                  "properties": {
                    "task_id": {
                      "type": "string",
                      "description": "The unique ID of the task.",
                      "example": "z2vo9qNRRHKArlJqkVHNUw"
                    },
                    "assignees": {
                      "type": "array",
                      "description": "A list of all assignees associated with the task. ",
                      "items": {
                        "required": [
                          "display_name",
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The Zoom user ID of the assignee.",
                            "example": "CcrEGgmeQem1uyJsuIRKwA"
                          },
                          "display_name": {
                            "type": "string",
                            "description": "The display name of the assignee.",
                            "example": "John Doe"
                          },
                          "email": {
                            "type": "string",
                            "description": "The email address of the assignee.",
                            "format": "email",
                            "example": "john.doe@example.com"
                          },
                          "avatar": {
                            "type": "string",
                            "description": "The avatar image URL of the assignee.",
                            "format": "uri",
                            "example": "https://example.com/avatar.jpg"
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
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid email {email}.  <br>\n"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You are no permission to request the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The resource not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "tasks_assignee:write",
              "tasks_assignee:write:admin",
              "tasks:write:assignees",
              "tasks:write:assignees:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Edit"
          ],
          "x-macro-scopes": [
            "tasks_assignee:write",
            "tasks_assignee:write:admin"
          ],
          "x-granular-scopes": [
            "tasks:write:assignees",
            "tasks:write:assignees:admin"
          ]
        }
      }
    },
    "/tasks/items/{taskId}/assignees/{userId}": {
      "delete": {
        "tags": [
          "Assignee"
        ],
        "summary": "Remove Assignee from task",
        "description": "Removes a specific assignee from a task. \n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_assignee:write`,`tasks_assignee:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:delete:assignees`,`tasks:delete:assignees:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "removeTaskAssignee",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          },
          {
            "name": "userId",
            "in": "path",
            "description": "The Zoom user ID of the task assignee.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "CcrEGgmeQem1uyJsuIRKwA"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "Collaborator removed successfully"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You do not have permission to operate on the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The task not found. <br>\n**Error Code:** `112100` <br>\n The assignee not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "tasks_assignee:write",
              "tasks_assignee:write:admin",
              "tasks:delete:assignees",
              "tasks:delete:assignees:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Edit"
          ],
          "x-macro-scopes": [
            "tasks_assignee:write",
            "tasks_assignee:write:admin"
          ],
          "x-granular-scopes": [
            "tasks:delete:assignees",
            "tasks:delete:assignees:admin"
          ]
        }
      }
    },
    "/tasks/items/{taskId}/collaborators": {
      "get": {
        "tags": [
          "Collaborator"
        ],
        "summary": "Get collaborators of a task",
        "description": "Retrieves all collaborators of a specified task.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_collaborator:read:admin`,`tasks_collaborator:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:list_collaborators`,`tasks:read:list_collaborators:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Getcollaboratorsofatask",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique ID of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Collaborators retrieved successfully.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "task_id": {
                      "type": "string",
                      "description": "The unique ID of the task.",
                      "example": "z2vo9qNRRHKArlJqkVHNUw"
                    },
                    "collaborators": {
                      "type": "array",
                      "description": "A list of all collaborators associated with the task.",
                      "items": {
                        "required": [
                          "display_name",
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The Zoom user ID of the collaborator.",
                            "example": "CcrEGgmeQem1uyJsuIRKwA"
                          },
                          "display_name": {
                            "type": "string",
                            "description": "The display name of the collaborator.",
                            "example": "John Doe"
                          },
                          "email": {
                            "type": "string",
                            "description": "The email address of the collaborator.",
                            "format": "email",
                            "example": "john.doe@example.com"
                          },
                          "avatar": {
                            "type": "string",
                            "description": "The avatar image URL of the collaborator.",
                            "format": "uri",
                            "example": "https://example.com/avatar.jpg"
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You don't have permission to request the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The resource not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "tasks_collaborator:read:admin",
              "tasks_collaborator:read",
              "tasks:read:list_collaborators",
              "tasks:read:list_collaborators:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Read"
          ],
          "x-macro-scopes": [
            "tasks_collaborator:read:admin",
            "tasks_collaborator:read"
          ],
          "x-granular-scopes": [
            "tasks:read:list_collaborators",
            "tasks:read:list_collaborators:admin"
          ]
        }
      },
      "post": {
        "tags": [
          "Collaborator"
        ],
        "summary": "Add collaborators to a task",
        "description": "Adds one or more collaborators to a specified task. \n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_collaborator:write`,`tasks_collaborator:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:write:collaborators`,`tasks:write:collaborators:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "addTasksCollaborators",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          }
        ],
        "requestBody": {
          "description": "Request body containing the collaborators to add, along with optional invitation details.",
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "collaborators"
                ],
                "type": "object",
                "properties": {
                  "invite_message": {
                    "maxLength": 200,
                    "type": "string",
                    "description": "An optional message to include with the collaborator invitation.",
                    "example": "Please help with this task."
                  },
                  "skip_notifications": {
                    "type": "boolean",
                    "description": "Indicates whether to skip sending notifications to the added collaborators.\n By default, notifications are sent.",
                    "example": false
                  },
                  "collaborators": {
                    "maxItems": 20,
                    "minItems": 1,
                    "type": "array",
                    "description": "A list of collaborators to add to the task. The user must belong to the same account as the requesting user.",
                    "items": {
                      "required": [
                        "email"
                      ],
                      "type": "object",
                      "properties": {
                        "email": {
                          "type": "string",
                          "description": "The email address of the collaborator to invite.",
                          "format": "email",
                          "example": "example@zoom.us"
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "Collaborators added successfully.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "collaborators",
                    "task_id"
                  ],
                  "type": "object",
                  "properties": {
                    "task_id": {
                      "type": "string",
                      "description": "The unique ID of the task.",
                      "example": "z2vo9qNRRHKArlJqkVHNUw"
                    },
                    "collaborators": {
                      "type": "array",
                      "description": "A list of all collaborators associated with the task.",
                      "items": {
                        "required": [
                          "display_name",
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The Zoom user ID of the collaborator.",
                            "example": "CcrEGgmeQem1uyJsuIRKwA"
                          },
                          "display_name": {
                            "type": "string",
                            "description": "The display name of the collaborator.",
                            "example": "John Doe"
                          },
                          "email": {
                            "type": "string",
                            "description": "The email address of the collaborator.",
                            "format": "email",
                            "example": "john.doe@example.com"
                          },
                          "avatar": {
                            "type": "string",
                            "description": "The avatar image URL of the collaborator.",
                            "format": "uri",
                            "example": "https://example.com/avatar.jpg"
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
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid email {email}.  <br>\n"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You are no permission to request the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The resource not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "tasks_collaborator:write",
              "tasks_collaborator:write:admin",
              "tasks:write:collaborators",
              "tasks:write:collaborators:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Edit"
          ],
          "x-macro-scopes": [
            "tasks_collaborator:write",
            "tasks_collaborator:write:admin"
          ],
          "x-granular-scopes": [
            "tasks:write:collaborators",
            "tasks:write:collaborators:admin"
          ]
        }
      }
    },
    "/tasks/items/{taskId}/collaborators/{userId}": {
      "delete": {
        "tags": [
          "Collaborator"
        ],
        "summary": "Remove collaborator from task",
        "description": "Removes a specific collaborator from a task. This operation will completely remove the user's access to the task.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_collaborator:write`,`tasks_collaborator:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:delete:collaborator:admin`,`tasks:delete:collaborator`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "removeTaskCollaborator",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique ID of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          },
          {
            "name": "userId",
            "in": "path",
            "description": "Zoom user ID of the collaborator.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "CcrEGgmeQem1uyJsuIRKwA"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "Collaborator removed successfully"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You do not have permission to operate on the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The task not found. <br>\n**Error Code:** `112100` <br>\n The assignee not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "tasks_collaborator:write",
              "tasks_collaborator:write:admin",
              "tasks:delete:collaborator:admin",
              "tasks:delete:collaborator"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Edit"
          ],
          "x-macro-scopes": [
            "tasks_collaborator:write",
            "tasks_collaborator:write:admin"
          ],
          "x-granular-scopes": [
            "tasks:delete:collaborator:admin",
            "tasks:delete:collaborator"
          ]
        }
      }
    },
    "/tasks/items/{taskId}/comments": {
      "get": {
        "tags": [
          "Comment"
        ],
        "summary": "Get a task's comments",
        "description": "Retrieves all comments associated with a specific task.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_comment:read`,`tasks_comment:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:comments`,`tasks:read:comments:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "GetAV1TasksComment",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of records returned from a single API call.",
            "required": false,
            "schema": {
              "maximum": 50,
              "type": "integer",
              "example": 10,
              "default": 10
            }
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "The next page token used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Task comments retrieved successfully.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "comments": {
                      "type": "array",
                      "description": "The list of comments associated with the task.",
                      "items": {
                        "type": "object",
                        "properties": {
                          "comment_id": {
                            "type": "string",
                            "description": "The unique identifier of the comment.",
                            "example": "comment_abc123def456"
                          },
                          "content": {
                            "type": "array",
                            "description": "The array of content fragments that make up the comment. Each fragment must define its order and contain either plain text or a mention. Fragments may also include attributes such as a hyperlink.",
                            "items": {
                              "required": [
                                "order"
                              ],
                              "type": "object",
                              "properties": {
                                "text": {
                                  "type": "string",
                                  "description": "The plain text fragment. May include letters, numbers, symbols, or emoji. Required if no `mention` is provided. If combined with `attributes.link`, this text is rendered as a hyperlink.",
                                  "example": "the text demo"
                                },
                                "order": {
                                  "type": "number",
                                  "description": "The position of the fragment within the comment. Must be unique within the `content` array. Defines the sequence for rendering.",
                                  "example": 0
                                },
                                "attributes": {
                                  "type": "object",
                                  "properties": {
                                    "link": {
                                      "type": "string",
                                      "description": "The URL associated with the fragment's text. Typically an absolute `http://` or `https://` link.",
                                      "example": "https://google.cn"
                                    }
                                  },
                                  "description": "Optional metadata for the fragment. Currently supports only hyperlinks."
                                },
                                "mention": {
                                  "type": "object",
                                  "properties": {
                                    "email": {
                                      "type": "string",
                                      "description": "The email address of the user being mentioned.",
                                      "example": "example@zoom.us"
                                    },
                                    "user_id": {
                                      "type": "string",
                                      "description": "The user ID of the user being mentioned.",
                                      "example": "3afwV0HiZS9mCLpxSQQbVz"
                                    },
                                    "display_name": {
                                      "type": "string",
                                      "description": "The display name of the user.",
                                      "example": "John Doe"
                                    },
                                    "avatar": {
                                      "type": "string",
                                      "description": "The avatar image URL of the user.",
                                      "format": "uri",
                                      "example": "https://example.com/avatar.jpg"
                                    }
                                  },
                                  "description": "Represents a mention of an entity (e.g., a user). If present, the fragment represents a mention instead of plain text."
                                }
                              }
                            }
                          },
                          "content_text": {
                            "type": "string",
                            "description": "A plain text version of the entire comment content.",
                            "example": "the text demo"
                          },
                          "create_time": {
                            "type": "integer",
                            "description": "Creation timestamp in milliseconds.",
                            "format": "int64",
                            "example": 1752715586000
                          }
                        },
                        "description": "A comment is represented as an ordered array of fragments. Each fragment may contain plain text, a mention, or a text snippet with a hyperlink."
                      }
                    },
                    "next_page_token": {
                      "type": "string",
                      "description": "Use the next page token to paginate through a large set of results. A next page token is returned when the available results exceed the current page size. This token's expiration period is 15 minutes.",
                      "example": "qUEQqB1V0HVhJmwKFQrGOD"
                    },
                    "page_size": {
                      "type": "integer",
                      "description": "The number of records returned per page in a single API call.",
                      "example": 10
                    },
                    "total_records": {
                      "type": "integer",
                      "description": "The total number of records found.",
                      "example": 2
                    }
                  }
                }
              }
            }
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You don't have permission to operate on the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The task not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "tasks_comment:read",
              "tasks_comment:read:admin",
              "tasks:read:comments",
              "tasks:read:comments:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Read",
            "TasksContent:Read"
          ],
          "x-macro-scopes": [
            "tasks_comment:read",
            "tasks_comment:read:admin"
          ],
          "x-granular-scopes": [
            "tasks:read:comments",
            "tasks:read:comments:admin"
          ]
        }
      },
      "post": {
        "tags": [
          "Comment"
        ],
        "summary": "Add a comment to task",
        "description": "Adds a comment to a specific task.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_comment:write`,`tasks_comment:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:write:comment:admin`,`tasks:write:comment`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "addComment",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "content",
                  "content_text"
                ],
                "type": "object",
                "properties": {
                  "content": {
                    "maxItems": 100,
                    "minItems": 1,
                    "type": "array",
                    "description": "The array of content fragments that make up the comment. Each fragment must define its order and contain either plain text or a mention. Fragments may also include attributes such as hyperlinks.",
                    "items": {
                      "required": [
                        "order"
                      ],
                      "type": "object",
                      "properties": {
                        "text": {
                          "type": "string",
                          "description": "A plain text fragment that may include letters, numbers, symbols, or emoji. Required if no mention is provided. If combined with attributes.link, this text is rendered as a hyperlink.",
                          "example": "the text demo"
                        },
                        "order": {
                          "type": "number",
                          "description": "The position of the fragment within the comment.\nMust be unique within the `content` array.\nDefines the sequence for rendering.",
                          "example": 0
                        },
                        "attributes": {
                          "type": "object",
                          "properties": {
                            "link": {
                              "type": "string",
                              "description": "The URL associated with the fragment's text, typically an absolute http:// or https:// link.",
                              "example": "https://google.cn"
                            }
                          },
                          "description": "Optional metadata for the fragment. Currently supports only hyperlinks."
                        },
                        "mention": {
                          "type": "object",
                          "properties": {
                            "email": {
                              "type": "string",
                              "description": "The email address of the user being mentioned.",
                              "format": "email",
                              "example": "example@zoom.us"
                            },
                            "user_id": {
                              "type": "string",
                              "description": "The user ID of the user being mentioned.",
                              "example": "3afwV0HiZS9mCLpxSQQbV"
                            }
                          },
                          "description": "Represents a user mention. Provide either the email or the user ID. If present, the fragment will be treated as a mention instead of plain text."
                        }
                      }
                    }
                  },
                  "content_text": {
                    "maxLength": 2000,
                    "minLength": 1,
                    "type": "string",
                    "description": "A plain text version of the entire comment content.",
                    "example": "the text demo"
                  }
                },
                "description": "Request body for adding a comment. A comment is represented as an ordered array of fragments. Each fragment may contain plain text, a mention, or a text snippet with a hyperlink."
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "Comment added successfully",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "comment_id",
                    "content"
                  ],
                  "type": "object",
                  "properties": {
                    "comment_id": {
                      "type": "string",
                      "description": "The unique identifier of the created comment.",
                      "example": "comment_abc123def456"
                    },
                    "content": {
                      "type": "array",
                      "description": "The array of content fragments that make up the comment.\nEach fragment must define its order and contain either plain text or a mention.\nFragments may also include attributes such as a hyperlink.",
                      "items": {
                        "required": [
                          "order"
                        ],
                        "type": "object",
                        "properties": {
                          "text": {
                            "type": "string",
                            "description": "A plain text fragment that may include letters, numbers, symbols, or emoji. Required if no mention is provided. If combined with attributes.link, this text is rendered as a hyperlink.",
                            "example": "the text demo"
                          },
                          "order": {
                            "type": "number",
                            "description": "The position of the fragment within the comment.\nMust be unique within the `content` array.\nDefines the sequence for rendering.",
                            "example": 0
                          },
                          "attributes": {
                            "type": "object",
                            "properties": {
                              "link": {
                                "type": "string",
                                "description": "The URL associated with the fragment's text, typically an absolute http:// or https:// link.",
                                "example": "https://google.cn"
                              }
                            },
                            "description": "Optional metadata for the fragment. Currently supports only hyperlinks."
                          },
                          "mention": {
                            "type": "object",
                            "properties": {
                              "email": {
                                "type": "string",
                                "description": "The email address of the user being mentioned.",
                                "example": "example@zoom.us"
                              },
                              "user_id": {
                                "type": "string",
                                "description": "The user ID of the user being mentioned.",
                                "example": "3afwV0HiZS9mCLpxSQQbV"
                              },
                              "display_name": {
                                "type": "string",
                                "description": "The display name of the user.",
                                "example": "John Doe"
                              },
                              "avatar": {
                                "type": "string",
                                "description": "The avatar image URL of the user.",
                                "format": "uri",
                                "example": "https://example.com/avatar.jpg"
                              }
                            },
                            "description": "Represents a mention of an entity (e.g., a user). If present, the fragment represents a mention instead of plain text."
                          }
                        }
                      }
                    },
                    "content_text": {
                      "type": "string",
                      "description": "A plain text version of the entire comment content.",
                      "example": "the text demo"
                    },
                    "create_time": {
                      "type": "integer",
                      "description": "Creation timestamp in milliseconds.",
                      "format": "int64",
                      "example": 1752715586000
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid request body. Please verify the content and try again. <br>\n"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You don't have permission to request the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The resource not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "tasks_comment:write",
              "tasks_comment:write:admin",
              "tasks:write:comment:admin",
              "tasks:write:comment"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Edit"
          ],
          "x-macro-scopes": [
            "tasks_comment:write",
            "tasks_comment:write:admin"
          ],
          "x-granular-scopes": [
            "tasks:write:comment:admin",
            "tasks:write:comment"
          ]
        }
      }
    },
    "/tasks/items/{taskId}/comments/{commentId}": {
      "delete": {
        "tags": [
          "Comment"
        ],
        "summary": "Delete a task's comment",
        "description": "Deletes a specific comment from a task by its ID.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_comment:write`,`tasks_comment:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:delete:comment:admin`,`tasks:delete:comment`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "DeleteTaskComment",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          },
          {
            "name": "commentId",
            "in": "path",
            "description": "The unique identifier of the comment to delete.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "c1vo9qNRRHKArlJqkVHNUx"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "Comment deleted successfully. No content is returned."
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You don't have permission to operate on the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The task not found. <br>\n**Error Code:** `112100` <br>\n The comment not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "tasks_comment:write",
              "tasks_comment:write:admin",
              "tasks:delete:comment:admin",
              "tasks:delete:comment"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Edit"
          ],
          "x-macro-scopes": [
            "tasks_comment:write",
            "tasks_comment:write:admin"
          ],
          "x-granular-scopes": [
            "tasks:delete:comment:admin",
            "tasks:delete:comment"
          ]
        }
      }
    },
    "/tasks/items": {
      "get": {
        "tags": [
          "Tasks"
        ],
        "summary": "List tasks",
        "description": "Retrieves tasks assigned to, or created by, the current user with advanced filtering, sorting, and pagination options.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read`,`tasks:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:list_tasks:admin`,`tasks:read:list_tasks`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "getMyTasks",
        "parameters": [
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of records returned from a single API call.",
            "required": false,
            "schema": {
              "maximum": 50,
              "type": "integer",
              "example": 10,
              "default": 10
            }
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42"
            }
          },
          {
            "name": "search_key",
            "in": "query",
            "description": "Search keyword for task title and description.",
            "required": false,
            "schema": {
              "maxLength": 50,
              "type": "string",
              "example": "project documentation"
            }
          },
          {
            "name": "sort_by",
            "in": "query",
            "description": "Determines the order in which tasks are returned.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "due_date",
              "default": "due_date",
              "enum": [
                "update_time",
                "create_time",
                "due_date",
                "status",
                "priority"
              ]
            }
          },
          {
            "name": "sort_asc",
            "in": "query",
            "description": "Sort in ascending order (default depends on sort column)",
            "required": false,
            "schema": {
              "type": "boolean",
              "example": false
            }
          },
          {
            "name": "assignee_email",
            "in": "query",
            "description": "The email address of the assignee to search for.",
            "required": false,
            "schema": {
              "type": "string",
              "format": "email",
              "example": "assignee@email.com"
            }
          },
          {
            "name": "due_date_start",
            "in": "query",
            "description": "Start date for due date filter. \n Format: (yyyy-mm-dd)",
            "required": false,
            "schema": {
              "type": "string",
              "format": "date",
              "example": "2024-01-01"
            }
          },
          {
            "name": "due_date_end",
            "in": "query",
            "description": "End date for due date filter.\n Format: (yyyy-mm-dd)",
            "required": false,
            "schema": {
              "type": "string",
              "format": "date",
              "example": "2024-12-31"
            }
          },
          {
            "name": "unread",
            "in": "query",
            "description": "Set to only return unread tasks for the current user.",
            "required": false,
            "schema": {
              "type": "boolean",
              "example": false
            }
          },
          {
            "name": "status",
            "in": "query",
            "description": "Current status of the task.",
            "required": false,
            "schema": {
              "type": "string",
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
          {
            "name": "starred",
            "in": "query",
            "description": "Filter tasks by star status.",
            "required": false,
            "schema": {
              "type": "boolean",
              "example": true
            }
          },
          {
            "name": "source_type",
            "in": "query",
            "description": "The source that created the task.",
            "required": false,
            "schema": {
              "type": "string",
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
          }
        ],
        "responses": {
          "200": {
            "description": "Tasks retrieved successfully",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "tasks",
                    "total_records"
                  ],
                  "type": "object",
                  "properties": {
                    "next_page_token": {
                      "type": "string",
                      "description": "Use the next page token to paginate through a large set of results. A next page token is returned when the available results exceed the current page size. This token's expiration period is 15 minutes.",
                      "example": "qUEQqB1V0HVhJmwKFQrGOD"
                    },
                    "page_size": {
                      "type": "integer",
                      "description": "The number of records returned per page in a single API call.",
                      "example": 10
                    },
                    "total_records": {
                      "type": "integer",
                      "description": "The total number of records found.",
                      "example": 2
                    },
                    "tasks": {
                      "type": "array",
                      "description": "List of tasks matching the query criteria.",
                      "items": {
                        "required": [
                          "ai_generated",
                          "create_time",
                          "is_public",
                          "modify_time",
                          "priority",
                          "source_type",
                          "starred",
                          "status",
                          "task_id",
                          "title"
                        ],
                        "type": "object",
                        "properties": {
                          "task_id": {
                            "type": "string",
                            "description": "The unique identifier of the task.",
                            "example": "z2vo9qNRRHKArlJqkVHNUw"
                          },
                          "title": {
                            "type": "string",
                            "description": "The title of the task.",
                            "example": "Complete project documentation"
                          },
                          "description": {
                            "maxLength": 200,
                            "minLength": 0,
                            "type": "string",
                            "description": "A description of the task that will be truncated. \n The maximum length is 200 characters.",
                            "example": "Write comprehensive documentation for the new API endpoints..."
                          },
                          "priority": {
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
                          "status": {
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
                          "is_public": {
                            "type": "boolean",
                            "description": "Determines the task's visibility.\n* `true` - The task is accessible to all members of your organization.\n* `false` - The task is accessible only to the assigned users and collaborators.",
                            "example": false,
                            "default": false
                          },
                          "due_date": {
                            "type": "string",
                            "description": "The task's due date in UTC, formatted as `yyyy-MM-dd`.",
                            "format": "date",
                            "example": "2025-09-24"
                          },
                          "ai_generated": {
                            "type": "boolean",
                            "description": "Whether the task was generated by AI.",
                            "example": false
                          },
                          "starred": {
                            "type": "boolean",
                            "description": "Whether the task is starred by the user.",
                            "example": true
                          },
                          "unread": {
                            "type": "boolean",
                            "description": "Whether the task is unread by the current user.",
                            "example": false
                          },
                          "assignees": {
                            "maxItems": 5,
                            "minItems": 0,
                            "type": "array",
                            "description": "A list of all assignees associated with the task.\n Only the first 5 assignees are returned.",
                            "items": {
                              "required": [
                                "display_name",
                                "user_id"
                              ],
                              "type": "object",
                              "properties": {
                                "user_id": {
                                  "type": "string",
                                  "description": "The Zoom user ID of the assignee.",
                                  "example": "CcrEGgmeQem1uyJsuIRKwA"
                                },
                                "display_name": {
                                  "type": "string",
                                  "description": "The display name of the assignee.",
                                  "example": "John Doe"
                                },
                                "email": {
                                  "type": "string",
                                  "description": "The email address of the assignee.",
                                  "format": "email",
                                  "example": "john.doe@example.com"
                                },
                                "avatar": {
                                  "type": "string",
                                  "description": "The avatar image URL of the assignee.",
                                  "format": "uri",
                                  "example": "https://example.com/avatar.jpg"
                                }
                              }
                            }
                          },
                          "collaborators": {
                            "type": "array",
                            "description": "A list of all collaborators associated with the task.\n Only the first 5 collaborators are returned.",
                            "items": {
                              "required": [
                                "display_name",
                                "user_id"
                              ],
                              "type": "object",
                              "properties": {
                                "user_id": {
                                  "type": "string",
                                  "description": "The Zoom user ID of the collaborator.",
                                  "example": "CcrEGgmeQem1uyJsuIRKwA"
                                },
                                "display_name": {
                                  "type": "string",
                                  "description": "The display name of the collaborator.",
                                  "example": "John Doe"
                                },
                                "email": {
                                  "type": "string",
                                  "description": "The email address of the collaborator.",
                                  "format": "email",
                                  "example": "john.doe@example.com"
                                },
                                "avatar": {
                                  "type": "string",
                                  "description": "The avatar image URL of the collaborator.",
                                  "format": "uri",
                                  "example": "https://example.com/avatar.jpg"
                                }
                              }
                            }
                          },
                          "friendly_task_id": {
                            "type": "string",
                            "description": "The task friendly ID.",
                            "example": "Task-1"
                          },
                          "link": {
                            "type": "string",
                            "description": "The task share link.",
                            "example": "https://tasks.zoomdev.us/ws53iQgOOkOdzVTxZaVoRA/TASK-1"
                          },
                          "create_time": {
                            "type": "string",
                            "description": "The task create date-time in UTC/GMT.",
                            "format": "date-time",
                            "example": "2025-03-31T12:02:00Z"
                          },
                          "modify_time": {
                            "type": "string",
                            "description": "The task modify date-time in UTC/GMT.",
                            "format": "date-time",
                            "example": "2025-03-31T12:02:00Z"
                          },
                          "source_type": {
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
                        }
                      }
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `38004` <br>\n invalid params {params_name} <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "tasks:read",
              "tasks:read:admin",
              "tasks:read:list_tasks:admin",
              "tasks:read:list_tasks"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Read",
            "TasksContent:Read"
          ],
          "x-macro-scopes": [
            "tasks:read",
            "tasks:read:admin"
          ],
          "x-granular-scopes": [
            "tasks:read:list_tasks:admin",
            "tasks:read:list_tasks"
          ]
        }
      },
      "post": {
        "tags": [
          "Tasks"
        ],
        "summary": "Create a new task",
        "description": "Provide essential details to create a new task without file attachments.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:write:task`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "createTask",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "title"
                ],
                "type": "object",
                "properties": {
                  "title": {
                    "maxLength": 500,
                    "minLength": 1,
                    "type": "string",
                    "description": "The tasks's title. Maximum 500 characters.",
                    "example": "Complete project documentation"
                  },
                  "description": {
                    "maxLength": 10000,
                    "minLength": 1,
                    "type": "string",
                    "description": "The task's detailed description. Maximum 10,000 characters.",
                    "example": "Write comprehensive documentation for the new API endpoints"
                  },
                  "priority": {
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
                  "status": {
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
                  "due_date": {
                    "type": "string",
                    "description": "The task's due date in UTC, formatted as `yyyy-MM-dd`.",
                    "format": "date",
                    "example": "2025-09-24"
                  },
                  "is_public": {
                    "type": "boolean",
                    "description": "Determines the task's visibility.\n* `true` - The task is accessible to all members of your organization.\n* `false` - The task is accessible only to the assigned users and collaborators.",
                    "example": false,
                    "default": false
                  },
                  "starred": {
                    "type": "boolean",
                    "description": "Specifies whether the task is starred.",
                    "example": false,
                    "default": false
                  },
                  "collaborators": {
                    "maxItems": 20,
                    "minItems": 0,
                    "type": "array",
                    "description": "A list of collaborators to invite to the task. Maximum 20.",
                    "items": {
                      "type": "object",
                      "properties": {
                        "email": {
                          "type": "string",
                          "description": "The email address of the collaborator to invite. The potential collaborator must belong to the same account as the requesting user.",
                          "example": "potential-collaborator@example.com"
                        }
                      }
                    }
                  },
                  "invite_message": {
                    "maxLength": 280,
                    "type": "string",
                    "description": "Include an optional message when inviting a user as a collaborator or assignee. Maximum length is 200 characters.",
                    "example": "Please join this task to help review the API documentation."
                  },
                  "skip_notifications": {
                    "type": "boolean",
                    "description": "Whether to skip sending notifications to collaborators.",
                    "example": false
                  },
                  "assignees": {
                    "maxItems": 20,
                    "minItems": 0,
                    "type": "array",
                    "description": "A list of assignees to add to the task. Maximum of 20. If empty, the request user is added by default.",
                    "items": {
                      "type": "object",
                      "properties": {
                        "email": {
                          "type": "string",
                          "description": "The email address of the assignee to invite. The potential assignee must belong to the same account as the requesting user.",
                          "example": "invited-assignee@example.com"
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "Task created successfully.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "create_time",
                    "is_public",
                    "priority",
                    "starred",
                    "status",
                    "task_id",
                    "title"
                  ],
                  "type": "object",
                  "properties": {
                    "task_id": {
                      "type": "string",
                      "description": "The created task's unique identifier.",
                      "example": "z2vo9qNRRHKArlJqkVHNUw"
                    },
                    "title": {
                      "type": "string",
                      "description": "The task's title.",
                      "example": "Complete project documentation"
                    },
                    "description": {
                      "type": "string",
                      "description": "The task's detailed description.",
                      "example": "Write comprehensive documentation for the new API endpoints"
                    },
                    "priority": {
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
                    "status": {
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
                    "is_public": {
                      "type": "boolean",
                      "description": "Determines the task's visibility.\n* `true` - The task is accessible to all members of your organization.\n* `false` - The task is accessible only to the assigned users and collaborators.",
                      "example": false,
                      "default": false
                    },
                    "due_date": {
                      "type": "string",
                      "description": "The task's due date in UTC, formatted as `yyyy-MM-dd`.",
                      "format": "date",
                      "example": "2025-09-24"
                    },
                    "starred": {
                      "type": "boolean",
                      "description": "Indicates whether the task is starred by the user.",
                      "example": false
                    },
                    "collaborators": {
                      "type": "array",
                      "description": "A list of collaborators associated with the task.",
                      "items": {
                        "required": [
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The collaborator's Zoom user ID.",
                            "example": "w0RChiauQeqRlv5fgxYULQ"
                          },
                          "email": {
                            "type": "string",
                            "description": "The email address of the collaborator.",
                            "example": "john.doe@example.com"
                          }
                        }
                      }
                    },
                    "assignees": {
                      "type": "array",
                      "description": "A list of all assignees associated with the task. ",
                      "items": {
                        "required": [
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The assignee's Zoom user ID.",
                            "example": "w0RChiauQeqRlv5fgxYULQ"
                          },
                          "email": {
                            "type": "string",
                            "description": "The email address of the assignee.",
                            "example": "invited-assignee@example.com"
                          }
                        }
                      }
                    },
                    "create_time": {
                      "type": "string",
                      "description": "The creation date and time in UTC/GMT format. ",
                      "format": "date-time",
                      "example": "2025-03-25T07:29:29Z"
                    },
                    "friendly_task_id": {
                      "type": "string",
                      "description": "The human-readable identifier of the task.",
                      "example": "Task-1"
                    },
                    "link": {
                      "type": "string",
                      "description": "The direct URL link to the task.",
                      "example": "https://tasks.example.com/ws53iQgOOkOdzVTxZaVoRA/TASK-1"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid email {email}. <br>\n**Error Code:** `34002` <br>\n The title is too long. <br>\n**Error Code:** `34002` <br>\n The description is too long. <br>\n**Error Code:** `34002` <br>\n Invalid field {field_name}. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "tasks:write",
              "tasks:write:task"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [],
          "x-macro-scopes": [
            "tasks:write"
          ],
          "x-granular-scopes": [
            "tasks:write:task"
          ]
        }
      }
    },
    "/tasks/items/{taskId}": {
      "get": {
        "tags": [
          "Tasks"
        ],
        "summary": "Get task details",
        "description": "Retrieves detailed information about a specific task by it\u2019s ID\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read`,`tasks:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:task`,`tasks:read:task:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "getTaskDetail",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Task details retrieved successfully",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "create_time",
                    "is_ai_generated",
                    "is_public",
                    "modify_time",
                    "priority",
                    "source_type",
                    "starred",
                    "status",
                    "task_id",
                    "title"
                  ],
                  "type": "object",
                  "properties": {
                    "task_id": {
                      "type": "string",
                      "description": "The unique identifier of the task.",
                      "example": "z2vo9qNRRHKArlJqkVHNUw"
                    },
                    "title": {
                      "type": "string",
                      "description": "The title of the task.",
                      "example": "the tasks title demo"
                    },
                    "description": {
                      "type": "string",
                      "description": "A detailed description of the task.",
                      "example": "the task description"
                    },
                    "priority": {
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
                    "status": {
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
                    "is_public": {
                      "type": "boolean",
                      "description": "Determines the task's visibility.\n* `true` - The task is accessible to all members of your organization.\n* `false` - The task is accessible only to the assigned users and collaborators.",
                      "example": false,
                      "default": false
                    },
                    "due_date": {
                      "type": "string",
                      "description": "The task's due date in UTC, formatted as `yyyy-MM-dd`.",
                      "format": "date",
                      "example": "2025-09-24"
                    },
                    "is_ai_generated": {
                      "type": "boolean",
                      "description": "Whether the task was generated by AI.",
                      "example": false
                    },
                    "starred": {
                      "type": "boolean",
                      "description": "Whether the task is starred by the user.",
                      "example": false
                    },
                    "tasks_attachments": {
                      "type": "array",
                      "description": "List of attachments associated with the task",
                      "example": [],
                      "items": {
                        "type": "object",
                        "properties": {
                          "attachment_id": {
                            "type": "string",
                            "description": "The unique identifier of the attachment.",
                            "example": "dz6nqEeQT6C1VoZFxteGTw"
                          },
                          "file_name": {
                            "type": "string",
                            "description": "The name of the attached file.",
                            "example": "example.pdf"
                          },
                          "file_size": {
                            "type": "integer",
                            "description": "The size of the file in bytes.",
                            "example": 1024
                          },
                          "mime_type": {
                            "type": "string",
                            "description": "The MIME type of the file.",
                            "example": "application/pdf"
                          },
                          "upload_time": {
                            "type": "string",
                            "description": "The file upload date-time in UTC/GMT.",
                            "format": "date-time",
                            "example": "2025-03-31T12:02:00Z"
                          },
                          "download_url": {
                            "type": "string",
                            "description": "The download link for the attachment will remain valid for 7,200 seconds.",
                            "example": "https://file.zoom.us/file/dz6nqEeQT6C1VoZFxteGTw?jwt=xxx"
                          }
                        }
                      }
                    },
                    "assignees": {
                      "type": "array",
                      "description": "A list of all assignees associated with the task. ",
                      "items": {
                        "required": [
                          "display_name",
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The Zoom user ID of the assignee.",
                            "example": "w0RChiauQeqRlv5fgxYULQ"
                          },
                          "display_name": {
                            "type": "string",
                            "description": "The display name of the assignee.",
                            "example": "John Doe"
                          },
                          "email": {
                            "type": "string",
                            "description": "The email address of the assignee.",
                            "format": "email",
                            "example": "john.doe@example.com"
                          },
                          "avatar": {
                            "type": "string",
                            "description": "The avatar image URL of the assignee.",
                            "format": "uri",
                            "example": "https://example.com/avatar.jpg"
                          }
                        }
                      }
                    },
                    "collaborators": {
                      "type": "array",
                      "description": "A list of all collaborators associated with the task.",
                      "items": {
                        "required": [
                          "display_name",
                          "user_id"
                        ],
                        "type": "object",
                        "properties": {
                          "user_id": {
                            "type": "string",
                            "description": "The Zoom user ID of the collaborator.",
                            "example": "w0RChiauQeqRlv5fgxYULQ"
                          },
                          "display_name": {
                            "type": "string",
                            "description": "The display name of the collaborator.",
                            "example": "John Doe"
                          },
                          "email": {
                            "type": "string",
                            "description": "The email address of the collaborator.",
                            "format": "email",
                            "example": "john.doe@example.com"
                          },
                          "avatar": {
                            "type": "string",
                            "description": "The avatar image URL of the collaborator.",
                            "format": "uri",
                            "example": "https://example.com/avatar.jpg"
                          }
                        }
                      }
                    },
                    "source_type": {
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
                    },
                    "friendly_task_id": {
                      "type": "string",
                      "description": "The task friendly ID.",
                      "example": "Task-1"
                    },
                    "link": {
                      "type": "string",
                      "description": "The task share link.",
                      "example": "https://tasks.zoomdev.us/ws53iQgOOkOdzVTxZaVoRA/TASK-1"
                    },
                    "associated_meeting": {
                      "type": "object",
                      "properties": {
                        "id": {
                          "type": "integer",
                          "description": "[Meeting ID](https://support.zoom.us/hc/en-us/articles/201362373-What-is-a-Meeting-ID-): Unique identifier of the meeting in **long** format, represented as int64 data type in JSON, also known as the meeting number.",
                          "format": "int64",
                          "example": 97763643886
                        },
                        "uuid": {
                          "type": "string",
                          "description": "The unique Meeting ID. Each meeting instance will generate its own Meeting UUID.",
                          "example": "aDYlohsHRtCd4ii1uC2+hA=="
                        },
                        "topic": {
                          "type": "string",
                          "description": "The topic of the associated meeting",
                          "example": "Tasks Meeting"
                        },
                        "meeting_date": {
                          "type": "string",
                          "description": "The meeting start date-time in UTC/GMT.",
                          "format": "date-time",
                          "example": "2025-03-31T12:02:00Z"
                        }
                      },
                      "description": "Associated meeting information"
                    },
                    "create_time": {
                      "type": "string",
                      "description": "The task create date-time in UTC/GMT.",
                      "format": "date-time",
                      "example": "2025-03-31T12:02:00Z"
                    },
                    "modify_time": {
                      "type": "string",
                      "description": "The task modify date-time in UTC/GMT.",
                      "format": "date-time",
                      "example": "2025-03-31T12:02:00Z"
                    }
                  }
                }
              }
            }
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You don't have permission to request the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The resource not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "tasks:read",
              "tasks:read:admin",
              "tasks:read:task",
              "tasks:read:task:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Read",
            "TasksContent:Read"
          ],
          "x-macro-scopes": [
            "tasks:read",
            "tasks:read:admin"
          ],
          "x-granular-scopes": [
            "tasks:read:task",
            "tasks:read:task:admin"
          ]
        }
      },
      "delete": {
        "tags": [
          "Tasks"
        ],
        "summary": "Delete a task",
        "description": "Permanently delete a task and all its associated data including descriptions, attachments, collaborators, and integrations. This operation cannot be undone.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:delete`,`tasks:delete:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:delete:task:admin`,`tasks:delete:task`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "deleteTask",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task to delete",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "Task deleted successfully"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You don't have permission to request the task. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `112100` <br>\n The resource not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "tasks:delete",
              "tasks:delete:admin",
              "tasks:delete:task:admin",
              "tasks:delete:task"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Edit"
          ],
          "x-macro-scopes": [
            "tasks:delete",
            "tasks:delete:admin"
          ],
          "x-granular-scopes": [
            "tasks:delete:task:admin",
            "tasks:delete:task"
          ]
        }
      },
      "patch": {
        "tags": [
          "Tasks"
        ],
        "summary": "Update task fields",
        "description": "Updates specific fields of an existing task. Only provided fields will be updated, others will remain unchanged.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:write`,`tasks:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:update:task`,`tasks:update:task:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "updateTask",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the task to update",
            "required": true,
            "schema": {
              "type": "string",
              "example": "z2vo9qNRRHKArlJqkVHNUw"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "title": {
                    "maxLength": 500,
                    "minLength": 1,
                    "type": "string",
                    "description": "The title of the task. \nMaximum 500 characters.",
                    "example": "Complete project documentation"
                  },
                  "description": {
                    "maxLength": 10000,
                    "minLength": 1,
                    "type": "string",
                    "description": "A detailed description of the task. \nMaximum 10,000 characters.",
                    "example": "Write comprehensive documentation for the new API endpoints"
                  },
                  "priority": {
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
                  "status": {
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
                  "due_date": {
                    "type": "string",
                    "description": "The task's due date in UTC, formatted as `yyyy-MM-dd`.",
                    "format": "date",
                    "example": "2025-09-24"
                  },
                  "is_public": {
                    "type": "boolean",
                    "description": "Determines the task's visibility.\n* `true` - The task is accessible to all members of your organization.\n* `false` - The task is accessible only to the assigned users and collaborators.",
                    "example": false,
                    "default": false
                  },
                  "starred": {
                    "type": "boolean",
                    "description": "Whether the task should be starred.",
                    "example": true
                  }
                }
              }
            }
          }
        },
        "responses": {
          "204": {
            "description": "Task updated successfully"
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid field {field_name}. <br>\n**Error Code:** `34002` <br>\n The title is too long. <br>\n**Error Code:** `34002` <br>\n The description is too long. <br>\n"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n You don't have permission to request the task. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "tasks:write",
              "tasks:write:admin",
              "tasks:update:task",
              "tasks:update:task:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "TasksMgt:Edit"
          ],
          "x-macro-scopes": [
            "tasks:write",
            "tasks:write:admin"
          ],
          "x-granular-scopes": [
            "tasks:update:task",
            "tasks:update:task:admin"
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
              "tasks_assignee:write": "tasks_assignee:write",
              "tasks_assignee:write:admin": "tasks_assignee:write:admin",
              "tasks:write:assignees": "tasks:write:assignees",
              "tasks:write:assignees:admin": "tasks:write:assignees:admin",
              "tasks_assignee:read:admin": "tasks_assignee:read:admin",
              "tasks_assignee:read": "tasks_assignee:read",
              "tasks:read:assignees": "tasks:read:assignees",
              "tasks:read:assignees:admin": "tasks:read:assignees:admin",
              "tasks:delete:assignees": "tasks:delete:assignees",
              "tasks:delete:assignees:admin": "tasks:delete:assignees:admin",
              "tasks_collaborator:read:admin": "tasks_collaborator:read:admin",
              "tasks_collaborator:read": "tasks_collaborator:read",
              "tasks:read:list_collaborators": "tasks:read:list_collaborators",
              "tasks:read:list_collaborators:admin": "tasks:read:list_collaborators:admin",
              "tasks_collaborator:write": "tasks_collaborator:write",
              "tasks_collaborator:write:admin": "tasks_collaborator:write:admin",
              "tasks:write:collaborators": "tasks:write:collaborators",
              "tasks:write:collaborators:admin": "tasks:write:collaborators:admin",
              "tasks:delete:collaborator:admin": "tasks:delete:collaborator:admin",
              "tasks:delete:collaborator": "tasks:delete:collaborator",
              "tasks_comment:write": "tasks_comment:write",
              "tasks_comment:write:admin": "tasks_comment:write:admin",
              "tasks:write:comment:admin": "tasks:write:comment:admin",
              "tasks:write:comment": "tasks:write:comment",
              "tasks_comment:read": "tasks_comment:read",
              "tasks_comment:read:admin": "tasks_comment:read:admin",
              "tasks:read:comments": "tasks:read:comments",
              "tasks:read:comments:admin": "tasks:read:comments:admin",
              "tasks:delete:comment:admin": "tasks:delete:comment:admin",
              "tasks:delete:comment": "tasks:delete:comment",
              "tasks:write": "tasks:write",
              "tasks:write:task": "tasks:write:task",
              "tasks:read": "tasks:read",
              "tasks:read:admin": "tasks:read:admin",
              "tasks:read:list_tasks:admin": "tasks:read:list_tasks:admin",
              "tasks:read:list_tasks": "tasks:read:list_tasks",
              "tasks:read:task": "tasks:read:task",
              "tasks:read:task:admin": "tasks:read:task:admin",
              "tasks:delete": "tasks:delete",
              "tasks:delete:admin": "tasks:delete:admin",
              "tasks:delete:task:admin": "tasks:delete:task:admin",
              "tasks:delete:task": "tasks:delete:task",
              "tasks:write:admin": "tasks:write:admin",
              "tasks:update:task": "tasks:update:task",
              "tasks:update:task:admin": "tasks:update:task:admin"
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
      "tasks"
    ]
  },
  "downloadPath": "/api-hub/tasks/methods/endpoints.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": false
  },
  "productMeta": {
    "specType": "oas",
    "slug": "/docs/api/rest/reference/tasks/methods",
    "title": "Tasks",
    "keywords": "zoom, rest, api, ai, aic, tasks",
    "params": {
      "productName": "Tasks"
    },
    "zoapEndpoint": "docJsonByFixVersion",
    "productDisplayTitle": "Tasks",
    "scalarSlug": "/docs/api/tasks/methods",
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "svgname": "tasks"
  },
  "frontmatter": {
    "title": "Tasks APIs",
    "keywords": "zoom, rest, api, ai, aic, tasks",
    "description": "The Zoom Tasks APIs let you manage and automate tasks within the Zoom ecosystem. Create, view, update, and delete tasks that help teams stay organized across meetings and projects. The Zoom Tasks REST API lets developers programmatically integrate task management into their workflows, enabling seamless coordination and productivity directly through Zoom.",
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
        "tasks"
      ]
    },
    "downloadPath": "/api-hub/tasks/methods/endpoints.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": false
    },
    "productMeta": {
      "specType": "oas",
      "slug": "/docs/api/rest/reference/tasks/methods",
      "title": "Tasks",
      "keywords": "zoom, rest, api, ai, aic, tasks",
      "params": {
        "productName": "Tasks"
      },
      "zoapEndpoint": "docJsonByFixVersion",
      "productDisplayTitle": "Tasks",
      "scalarSlug": "/docs/api/tasks/methods",
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "svgname": "tasks"
    },
    "spec": "{\"openapi\":\"3.0.0\",\"info\":{\"title\":\"Tasks\",\"description\":\"The Zoom Tasks APIs let you manage and automate tasks within the Zoom ecosystem. Create, view, update, and delete tasks that help teams stay organized across meetings and projects. The Zoom Tasks REST API lets developers programmatically integrate task management into their workflows, enabling seamless coordination and productivity directly through Zoom.\",\"termsOfService\":\"https://zoom.us/docs/en-us/zoom_api_license_and_tou.html\",\"contact\":{\"name\":\"Zoom Developers\",\"url\":\"https://developer.zoom.us/\"},\"version\":\"2\"},\"externalDocs\":{\"description\":\"Find out more about Swagger\",\"url\":\"https://swagger.io\"},\"servers\":[{\"url\":\"https://api.zoom.us/v2\"}],\"paths\":{\"/tasks/items/{taskId}/assignees\":{\"get\":{\"tags\":[\"Assignee\"],\"summary\":\"Get assignees of a task\",\"description\":\"Retrieves the list of assignees of a specified task.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_assignee:read:admin`,`tasks_assignee:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:assignees`,`tasks:read:assignees:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"GetAssigneesOfATask\",\"parameters\":[{\"name\":\"taskId\",\"in\":\"path\",\"description\":\"The unique identifier of the task.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"}}],\"responses\":{\"200\":{\"description\":\"Assignees retrieved successfully.\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The unique ID of the task.\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"},\"assignees\":{\"type\":\"array\",\"description\":\"A list of all assignees associated with the task. \",\"items\":{\"required\":[\"display_name\",\"user_id\"],\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the task assignee.\",\"example\":\"CcrEGgmeQem1uyJsuIRKwA\"},\"display_name\":{\"type\":\"string\",\"description\":\"The display name of the assignee.\",\"example\":\"John Doe\"},\"email\":{\"type\":\"string\",\"description\":\"The email address of the assignee.\",\"format\":\"email\",\"example\":\"john.doe@example.com\"},\"avatar\":{\"type\":\"string\",\"description\":\"The assignee's avatar image URL.\",\"format\":\"uri\",\"example\":\"https://example.com/avatar.jpg\"}}}}}}}}},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111800` <br>\\n You don't have permission to request the task <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `112100` <br>\\n The resource not found. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"TasksMgt:Read\"],\"x-macro-scopes\":[\"tasks_assignee:read:admin\",\"tasks_assignee:read\"],\"x-granular-scopes\":[\"tasks:read:assignees\",\"tasks:read:assignees:admin\"]}},\"post\":{\"tags\":[\"Assignee\"],\"summary\":\"Add assignees to a task\",\"description\":\"Add one or more assignees to a specified task. If a user is an assignee, they are also considered a collaborator.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_assignee:write`,`tasks_assignee:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:write:assignees`,`tasks:write:assignees:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"addTasksAssignees\",\"parameters\":[{\"name\":\"taskId\",\"in\":\"path\",\"description\":\"The unique identifier of the task.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"}}],\"requestBody\":{\"description\":\"Request body containing the assignees to add, along with optional invitation details.\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"assignees\"],\"type\":\"object\",\"properties\":{\"invite_message\":{\"maxLength\":200,\"type\":\"string\",\"description\":\"An optional message to include with the assignee invitation.\",\"example\":\"Please help with this task.\"},\"skip_notifications\":{\"type\":\"boolean\",\"description\":\"Indicates whether to skip sending notifications to the added assignees. By default, notifications are sent.\",\"example\":false,\"default\":false},\"assignees\":{\"maxItems\":20,\"minItems\":1,\"type\":\"array\",\"description\":\"The email address of the assignee to add. The user must belong to the same account as the requesting user.\",\"items\":{\"required\":[\"email\"],\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\",\"description\":\"The email address of the assignee to invite.\",\"format\":\"email\",\"example\":\"example@zoom.us\"}}}}}}}}},\"responses\":{\"201\":{\"description\":\"Assignees added successfully.\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"assignees\",\"task_id\"],\"type\":\"object\",\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The unique ID of the task.\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"},\"assignees\":{\"type\":\"array\",\"description\":\"A list of all assignees associated with the task. \",\"items\":{\"required\":[\"display_name\",\"user_id\"],\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the assignee.\",\"example\":\"CcrEGgmeQem1uyJsuIRKwA\"},\"display_name\":{\"type\":\"string\",\"description\":\"The display name of the assignee.\",\"example\":\"John Doe\"},\"email\":{\"type\":\"string\",\"description\":\"The email address of the assignee.\",\"format\":\"email\",\"example\":\"john.doe@example.com\"},\"avatar\":{\"type\":\"string\",\"description\":\"The avatar image URL of the assignee.\",\"format\":\"uri\",\"example\":\"https://example.com/avatar.jpg\"}}}}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `34002` <br>\\n Invalid email {email}.  <br>\\n\"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111800` <br>\\n You are no permission to request the task. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `112100` <br>\\n The resource not found. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"TasksMgt:Edit\"],\"x-macro-scopes\":[\"tasks_assignee:write\",\"tasks_assignee:write:admin\"],\"x-granular-scopes\":[\"tasks:write:assignees\",\"tasks:write:assignees:admin\"]}}},\"/tasks/items/{taskId}/assignees/{userId}\":{\"delete\":{\"tags\":[\"Assignee\"],\"summary\":\"Remove Assignee from task\",\"description\":\"Removes a specific assignee from a task. \\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_assignee:write`,`tasks_assignee:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:delete:assignees`,`tasks:delete:assignees:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"removeTaskAssignee\",\"parameters\":[{\"name\":\"taskId\",\"in\":\"path\",\"description\":\"The unique identifier of the task.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"}},{\"name\":\"userId\",\"in\":\"path\",\"description\":\"The Zoom user ID of the task assignee.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"CcrEGgmeQem1uyJsuIRKwA\"}}],\"responses\":{\"204\":{\"description\":\"Collaborator removed successfully\"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111800` <br>\\n You do not have permission to operate on the task. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `112100` <br>\\n The task not found. <br>\\n**Error Code:** `112100` <br>\\n The assignee not found. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"TasksMgt:Edit\"],\"x-macro-scopes\":[\"tasks_assignee:write\",\"tasks_assignee:write:admin\"],\"x-granular-scopes\":[\"tasks:delete:assignees\",\"tasks:delete:assignees:admin\"]}}},\"/tasks/items/{taskId}/collaborators\":{\"get\":{\"tags\":[\"Collaborator\"],\"summary\":\"Get collaborators of a task\",\"description\":\"Retrieves all collaborators of a specified task.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_collaborator:read:admin`,`tasks_collaborator:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:list_collaborators`,`tasks:read:list_collaborators:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"Getcollaboratorsofatask\",\"parameters\":[{\"name\":\"taskId\",\"in\":\"path\",\"description\":\"The unique ID of the task.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"}}],\"responses\":{\"200\":{\"description\":\"Collaborators retrieved successfully.\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The unique ID of the task.\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"},\"collaborators\":{\"type\":\"array\",\"description\":\"A list of all collaborators associated with the task.\",\"items\":{\"required\":[\"display_name\",\"user_id\"],\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the collaborator.\",\"example\":\"CcrEGgmeQem1uyJsuIRKwA\"},\"display_name\":{\"type\":\"string\",\"description\":\"The display name of the collaborator.\",\"example\":\"John Doe\"},\"email\":{\"type\":\"string\",\"description\":\"The email address of the collaborator.\",\"format\":\"email\",\"example\":\"john.doe@example.com\"},\"avatar\":{\"type\":\"string\",\"description\":\"The avatar image URL of the collaborator.\",\"format\":\"uri\",\"example\":\"https://example.com/avatar.jpg\"}}}}}}}}},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111800` <br>\\n You don't have permission to request the task. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `112100` <br>\\n The resource not found. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"TasksMgt:Read\"],\"x-macro-scopes\":[\"tasks_collaborator:read:admin\",\"tasks_collaborator:read\"],\"x-granular-scopes\":[\"tasks:read:list_collaborators\",\"tasks:read:list_collaborators:admin\"]}},\"post\":{\"tags\":[\"Collaborator\"],\"summary\":\"Add collaborators to a task\",\"description\":\"Adds one or more collaborators to a specified task. \\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_collaborator:write`,`tasks_collaborator:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:write:collaborators`,`tasks:write:collaborators:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"addTasksCollaborators\",\"parameters\":[{\"name\":\"taskId\",\"in\":\"path\",\"description\":\"The unique identifier of the task.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"}}],\"requestBody\":{\"description\":\"Request body containing the collaborators to add, along with optional invitation details.\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"collaborators\"],\"type\":\"object\",\"properties\":{\"invite_message\":{\"maxLength\":200,\"type\":\"string\",\"description\":\"An optional message to include with the collaborator invitation.\",\"example\":\"Please help with this task.\"},\"skip_notifications\":{\"type\":\"boolean\",\"description\":\"Indicates whether to skip sending notifications to the added collaborators.\\n By default, notifications are sent.\",\"example\":false},\"collaborators\":{\"maxItems\":20,\"minItems\":1,\"type\":\"array\",\"description\":\"A list of collaborators to add to the task. The user must belong to the same account as the requesting user.\",\"items\":{\"required\":[\"email\"],\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\",\"description\":\"The email address of the collaborator to invite.\",\"format\":\"email\",\"example\":\"example@zoom.us\"}}}}}}}}},\"responses\":{\"201\":{\"description\":\"Collaborators added successfully.\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"collaborators\",\"task_id\"],\"type\":\"object\",\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The unique ID of the task.\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"},\"collaborators\":{\"type\":\"array\",\"description\":\"A list of all collaborators associated with the task.\",\"items\":{\"required\":[\"display_name\",\"user_id\"],\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the collaborator.\",\"example\":\"CcrEGgmeQem1uyJsuIRKwA\"},\"display_name\":{\"type\":\"string\",\"description\":\"The display name of the collaborator.\",\"example\":\"John Doe\"},\"email\":{\"type\":\"string\",\"description\":\"The email address of the collaborator.\",\"format\":\"email\",\"example\":\"john.doe@example.com\"},\"avatar\":{\"type\":\"string\",\"description\":\"The avatar image URL of the collaborator.\",\"format\":\"uri\",\"example\":\"https://example.com/avatar.jpg\"}}}}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `34002` <br>\\n Invalid email {email}.  <br>\\n\"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111800` <br>\\n You are no permission to request the task. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `112100` <br>\\n The resource not found. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"TasksMgt:Edit\"],\"x-macro-scopes\":[\"tasks_collaborator:write\",\"tasks_collaborator:write:admin\"],\"x-granular-scopes\":[\"tasks:write:collaborators\",\"tasks:write:collaborators:admin\"]}}},\"/tasks/items/{taskId}/collaborators/{userId}\":{\"delete\":{\"tags\":[\"Collaborator\"],\"summary\":\"Remove collaborator from task\",\"description\":\"Removes a specific collaborator from a task. This operation will completely remove the user's access to the task.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_collaborator:write`,`tasks_collaborator:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:delete:collaborator:admin`,`tasks:delete:collaborator`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"removeTaskCollaborator\",\"parameters\":[{\"name\":\"taskId\",\"in\":\"path\",\"description\":\"The unique ID of the task.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"}},{\"name\":\"userId\",\"in\":\"path\",\"description\":\"Zoom user ID of the collaborator.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"CcrEGgmeQem1uyJsuIRKwA\"}}],\"responses\":{\"204\":{\"description\":\"Collaborator removed successfully\"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111800` <br>\\n You do not have permission to operate on the task. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `112100` <br>\\n The task not found. <br>\\n**Error Code:** `112100` <br>\\n The assignee not found. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"TasksMgt:Edit\"],\"x-macro-scopes\":[\"tasks_collaborator:write\",\"tasks_collaborator:write:admin\"],\"x-granular-scopes\":[\"tasks:delete:collaborator:admin\",\"tasks:delete:collaborator\"]}}},\"/tasks/items/{taskId}/comments\":{\"get\":{\"tags\":[\"Comment\"],\"summary\":\"Get a task's comments\",\"description\":\"Retrieves all comments associated with a specific task.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_comment:read`,`tasks_comment:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:comments`,`tasks:read:comments:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"GetAV1TasksComment\",\"parameters\":[{\"name\":\"taskId\",\"in\":\"path\",\"description\":\"The unique identifier of the task.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"}},{\"name\":\"page_size\",\"in\":\"query\",\"description\":\"The number of records returned from a single API call.\",\"required\":false,\"schema\":{\"maximum\":50,\"type\":\"integer\",\"example\":10,\"default\":10}},{\"name\":\"next_page_token\",\"in\":\"query\",\"description\":\"The next page token used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42\"}}],\"responses\":{\"200\":{\"description\":\"Task comments retrieved successfully.\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"comments\":{\"type\":\"array\",\"description\":\"The list of comments associated with the task.\",\"items\":{\"type\":\"object\",\"properties\":{\"comment_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the comment.\",\"example\":\"comment_abc123def456\"},\"content\":{\"type\":\"array\",\"description\":\"The array of content fragments that make up the comment. Each fragment must define its order and contain either plain text or a mention. Fragments may also include attributes such as a hyperlink.\",\"items\":{\"required\":[\"order\"],\"type\":\"object\",\"properties\":{\"text\":{\"type\":\"string\",\"description\":\"The plain text fragment. May include letters, numbers, symbols, or emoji. Required if no `mention` is provided. If combined with `attributes.link`, this text is rendered as a hyperlink.\",\"example\":\"the text demo\"},\"order\":{\"type\":\"number\",\"description\":\"The position of the fragment within the comment. Must be unique within the `content` array. Defines the sequence for rendering.\",\"example\":0},\"attributes\":{\"type\":\"object\",\"properties\":{\"link\":{\"type\":\"string\",\"description\":\"The URL associated with the fragment's text. Typically an absolute `http://` or `https://` link.\",\"example\":\"https://google.cn\"}},\"description\":\"Optional metadata for the fragment. Currently supports only hyperlinks.\"},\"mention\":{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\",\"description\":\"The email address of the user being mentioned.\",\"example\":\"example@zoom.us\"},\"user_id\":{\"type\":\"string\",\"description\":\"The user ID of the user being mentioned.\",\"example\":\"3afwV0HiZS9mCLpxSQQbVz\"},\"display_name\":{\"type\":\"string\",\"description\":\"The display name of the user.\",\"example\":\"John Doe\"},\"avatar\":{\"type\":\"string\",\"description\":\"The avatar image URL of the user.\",\"format\":\"uri\",\"example\":\"https://example.com/avatar.jpg\"}},\"description\":\"Represents a mention of an entity (e.g., a user). If present, the fragment represents a mention instead of plain text.\"}}}},\"content_text\":{\"type\":\"string\",\"description\":\"A plain text version of the entire comment content.\",\"example\":\"the text demo\"},\"create_time\":{\"type\":\"integer\",\"description\":\"Creation timestamp in milliseconds.\",\"format\":\"int64\",\"example\":1752715586000}},\"description\":\"A comment is represented as an ordered array of fragments. Each fragment may contain plain text, a mention, or a text snippet with a hyperlink.\"}},\"next_page_token\":{\"type\":\"string\",\"description\":\"Use the next page token to paginate through a large set of results. A next page token is returned when the available results exceed the current page size. This token's expiration period is 15 minutes.\",\"example\":\"qUEQqB1V0HVhJmwKFQrGOD\"},\"page_size\":{\"type\":\"integer\",\"description\":\"The number of records returned per page in a single API call.\",\"example\":10},\"total_records\":{\"type\":\"integer\",\"description\":\"The total number of records found.\",\"example\":2}}}}}},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111800` <br>\\n You don't have permission to operate on the task. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `112100` <br>\\n The task not found. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"TasksMgt:Read\",\"TasksContent:Read\"],\"x-macro-scopes\":[\"tasks_comment:read\",\"tasks_comment:read:admin\"],\"x-granular-scopes\":[\"tasks:read:comments\",\"tasks:read:comments:admin\"]}},\"post\":{\"tags\":[\"Comment\"],\"summary\":\"Add a comment to task\",\"description\":\"Adds a comment to a specific task.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_comment:write`,`tasks_comment:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:write:comment:admin`,`tasks:write:comment`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"addComment\",\"parameters\":[{\"name\":\"taskId\",\"in\":\"path\",\"description\":\"The unique identifier of the task.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"}}],\"requestBody\":{\"content\":{\"application/json\":{\"schema\":{\"required\":[\"content\",\"content_text\"],\"type\":\"object\",\"properties\":{\"content\":{\"maxItems\":100,\"minItems\":1,\"type\":\"array\",\"description\":\"The array of content fragments that make up the comment. Each fragment must define its order and contain either plain text or a mention. Fragments may also include attributes such as hyperlinks.\",\"items\":{\"required\":[\"order\"],\"type\":\"object\",\"properties\":{\"text\":{\"type\":\"string\",\"description\":\"A plain text fragment that may include letters, numbers, symbols, or emoji. Required if no mention is provided. If combined with attributes.link, this text is rendered as a hyperlink.\",\"example\":\"the text demo\"},\"order\":{\"type\":\"number\",\"description\":\"The position of the fragment within the comment.\\nMust be unique within the `content` array.\\nDefines the sequence for rendering.\",\"example\":0},\"attributes\":{\"type\":\"object\",\"properties\":{\"link\":{\"type\":\"string\",\"description\":\"The URL associated with the fragment's text, typically an absolute http:// or https:// link.\",\"example\":\"https://google.cn\"}},\"description\":\"Optional metadata for the fragment. Currently supports only hyperlinks.\"},\"mention\":{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\",\"description\":\"The email address of the user being mentioned.\",\"format\":\"email\",\"example\":\"example@zoom.us\"},\"user_id\":{\"type\":\"string\",\"description\":\"The user ID of the user being mentioned.\",\"example\":\"3afwV0HiZS9mCLpxSQQbV\"}},\"description\":\"Represents a user mention. Provide either the email or the user ID. If present, the fragment will be treated as a mention instead of plain text.\"}}}},\"content_text\":{\"maxLength\":2000,\"minLength\":1,\"type\":\"string\",\"description\":\"A plain text version of the entire comment content.\",\"example\":\"the text demo\"}},\"description\":\"Request body for adding a comment. A comment is represented as an ordered array of fragments. Each fragment may contain plain text, a mention, or a text snippet with a hyperlink.\"}}}},\"responses\":{\"201\":{\"description\":\"Comment added successfully\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"comment_id\",\"content\"],\"type\":\"object\",\"properties\":{\"comment_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the created comment.\",\"example\":\"comment_abc123def456\"},\"content\":{\"type\":\"array\",\"description\":\"The array of content fragments that make up the comment.\\nEach fragment must define its order and contain either plain text or a mention.\\nFragments may also include attributes such as a hyperlink.\",\"items\":{\"required\":[\"order\"],\"type\":\"object\",\"properties\":{\"text\":{\"type\":\"string\",\"description\":\"A plain text fragment that may include letters, numbers, symbols, or emoji. Required if no mention is provided. If combined with attributes.link, this text is rendered as a hyperlink.\",\"example\":\"the text demo\"},\"order\":{\"type\":\"number\",\"description\":\"The position of the fragment within the comment.\\nMust be unique within the `content` array.\\nDefines the sequence for rendering.\",\"example\":0},\"attributes\":{\"type\":\"object\",\"properties\":{\"link\":{\"type\":\"string\",\"description\":\"The URL associated with the fragment's text, typically an absolute http:// or https:// link.\",\"example\":\"https://google.cn\"}},\"description\":\"Optional metadata for the fragment. Currently supports only hyperlinks.\"},\"mention\":{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\",\"description\":\"The email address of the user being mentioned.\",\"example\":\"example@zoom.us\"},\"user_id\":{\"type\":\"string\",\"description\":\"The user ID of the user being mentioned.\",\"example\":\"3afwV0HiZS9mCLpxSQQbV\"},\"display_name\":{\"type\":\"string\",\"description\":\"The display name of the user.\",\"example\":\"John Doe\"},\"avatar\":{\"type\":\"string\",\"description\":\"The avatar image URL of the user.\",\"format\":\"uri\",\"example\":\"https://example.com/avatar.jpg\"}},\"description\":\"Represents a mention of an entity (e.g., a user). If present, the fragment represents a mention instead of plain text.\"}}}},\"content_text\":{\"type\":\"string\",\"description\":\"A plain text version of the entire comment content.\",\"example\":\"the text demo\"},\"create_time\":{\"type\":\"integer\",\"description\":\"Creation timestamp in milliseconds.\",\"format\":\"int64\",\"example\":1752715586000}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `34002` <br>\\n Invalid request body. Please verify the content and try again. <br>\\n\"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111800` <br>\\n You don't have permission to request the task. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `112100` <br>\\n The resource not found. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"TasksMgt:Edit\"],\"x-macro-scopes\":[\"tasks_comment:write\",\"tasks_comment:write:admin\"],\"x-granular-scopes\":[\"tasks:write:comment:admin\",\"tasks:write:comment\"]}}},\"/tasks/items/{taskId}/comments/{commentId}\":{\"delete\":{\"tags\":[\"Comment\"],\"summary\":\"Delete a task's comment\",\"description\":\"Deletes a specific comment from a task by its ID.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks_comment:write`,`tasks_comment:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:delete:comment:admin`,`tasks:delete:comment`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"DeleteTaskComment\",\"parameters\":[{\"name\":\"taskId\",\"in\":\"path\",\"description\":\"The unique identifier of the task.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"}},{\"name\":\"commentId\",\"in\":\"path\",\"description\":\"The unique identifier of the comment to delete.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"c1vo9qNRRHKArlJqkVHNUx\"}}],\"responses\":{\"204\":{\"description\":\"Comment deleted successfully. No content is returned.\"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111800` <br>\\n You don't have permission to operate on the task. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `112100` <br>\\n The task not found. <br>\\n**Error Code:** `112100` <br>\\n The comment not found. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"TasksMgt:Edit\"],\"x-macro-scopes\":[\"tasks_comment:write\",\"tasks_comment:write:admin\"],\"x-granular-scopes\":[\"tasks:delete:comment:admin\",\"tasks:delete:comment\"]}}},\"/tasks/items\":{\"get\":{\"tags\":[\"Tasks\"],\"summary\":\"List tasks\",\"description\":\"Retrieves tasks assigned to, or created by, the current user with advanced filtering, sorting, and pagination options.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read`,`tasks:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:list_tasks:admin`,`tasks:read:list_tasks`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"getMyTasks\",\"parameters\":[{\"name\":\"page_size\",\"in\":\"query\",\"description\":\"The number of records returned from a single API call.\",\"required\":false,\"schema\":{\"maximum\":50,\"type\":\"integer\",\"example\":10,\"default\":10}},{\"name\":\"next_page_token\",\"in\":\"query\",\"description\":\"The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42\"}},{\"name\":\"search_key\",\"in\":\"query\",\"description\":\"Search keyword for task title and description.\",\"required\":false,\"schema\":{\"maxLength\":50,\"type\":\"string\",\"example\":\"project documentation\"}},{\"name\":\"sort_by\",\"in\":\"query\",\"description\":\"Determines the order in which tasks are returned.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"due_date\",\"default\":\"due_date\",\"enum\":[\"update_time\",\"create_time\",\"due_date\",\"status\",\"priority\"]}},{\"name\":\"sort_asc\",\"in\":\"query\",\"description\":\"Sort in ascending order (default depends on sort column)\",\"required\":false,\"schema\":{\"type\":\"boolean\",\"example\":false}},{\"name\":\"assignee_email\",\"in\":\"query\",\"description\":\"The email address of the assignee to search for.\",\"required\":false,\"schema\":{\"type\":\"string\",\"format\":\"email\",\"example\":\"assignee@email.com\"}},{\"name\":\"due_date_start\",\"in\":\"query\",\"description\":\"Start date for due date filter. \\n Format: (yyyy-mm-dd)\",\"required\":false,\"schema\":{\"type\":\"string\",\"format\":\"date\",\"example\":\"2024-01-01\"}},{\"name\":\"due_date_end\",\"in\":\"query\",\"description\":\"End date for due date filter.\\n Format: (yyyy-mm-dd)\",\"required\":false,\"schema\":{\"type\":\"string\",\"format\":\"date\",\"example\":\"2024-12-31\"}},{\"name\":\"unread\",\"in\":\"query\",\"description\":\"Set to only return unread tasks for the current user.\",\"required\":false,\"schema\":{\"type\":\"boolean\",\"example\":false}},{\"name\":\"status\",\"in\":\"query\",\"description\":\"Current status of the task.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"To do\",\"default\":\"To do\",\"enum\":[\"To do\",\"In progress\",\"Done\",\"Blocked\",\"Recommended\"]}},{\"name\":\"starred\",\"in\":\"query\",\"description\":\"Filter tasks by star status.\",\"required\":false,\"schema\":{\"type\":\"boolean\",\"example\":true}},{\"name\":\"source_type\",\"in\":\"query\",\"description\":\"The source that created the task.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"user\",\"enum\":[\"user\",\"aic_meeting_summary\",\"whiteboard\",\"notes\",\"docs\",\"phone\",\"chat\",\"email\",\"meeting\",\"aic\"]}}],\"responses\":{\"200\":{\"description\":\"Tasks retrieved successfully\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"tasks\",\"total_records\"],\"type\":\"object\",\"properties\":{\"next_page_token\":{\"type\":\"string\",\"description\":\"Use the next page token to paginate through a large set of results. A next page token is returned when the available results exceed the current page size. This token's expiration period is 15 minutes.\",\"example\":\"qUEQqB1V0HVhJmwKFQrGOD\"},\"page_size\":{\"type\":\"integer\",\"description\":\"The number of records returned per page in a single API call.\",\"example\":10},\"total_records\":{\"type\":\"integer\",\"description\":\"The total number of records found.\",\"example\":2},\"tasks\":{\"type\":\"array\",\"description\":\"List of tasks matching the query criteria.\",\"items\":{\"required\":[\"ai_generated\",\"create_time\",\"is_public\",\"modify_time\",\"priority\",\"source_type\",\"starred\",\"status\",\"task_id\",\"title\"],\"type\":\"object\",\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the task.\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"},\"title\":{\"type\":\"string\",\"description\":\"The title of the task.\",\"example\":\"Complete project documentation\"},\"description\":{\"maxLength\":200,\"minLength\":0,\"type\":\"string\",\"description\":\"A description of the task that will be truncated. \\n The maximum length is 200 characters.\",\"example\":\"Write comprehensive documentation for the new API endpoints...\"},\"priority\":{\"type\":\"string\",\"description\":\"The task's priority level.\",\"example\":\"Medium\",\"default\":\"Medium\",\"enum\":[\"Low\",\"Medium\",\"High\",\"Highest\"]},\"status\":{\"type\":\"string\",\"description\":\"The task's current status.\",\"example\":\"To do\",\"default\":\"To do\",\"enum\":[\"To do\",\"In progress\",\"Done\",\"Blocked\",\"Recommended\"]},\"is_public\":{\"type\":\"boolean\",\"description\":\"Determines the task's visibility.\\n* `true` - The task is accessible to all members of your organization.\\n* `false` - The task is accessible only to the assigned users and collaborators.\",\"example\":false,\"default\":false},\"due_date\":{\"type\":\"string\",\"description\":\"The task's due date in UTC, formatted as `yyyy-MM-dd`.\",\"format\":\"date\",\"example\":\"2025-09-24\"},\"ai_generated\":{\"type\":\"boolean\",\"description\":\"Whether the task was generated by AI.\",\"example\":false},\"starred\":{\"type\":\"boolean\",\"description\":\"Whether the task is starred by the user.\",\"example\":true},\"unread\":{\"type\":\"boolean\",\"description\":\"Whether the task is unread by the current user.\",\"example\":false},\"assignees\":{\"maxItems\":5,\"minItems\":0,\"type\":\"array\",\"description\":\"A list of all assignees associated with the task.\\n Only the first 5 assignees are returned.\",\"items\":{\"required\":[\"display_name\",\"user_id\"],\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the assignee.\",\"example\":\"CcrEGgmeQem1uyJsuIRKwA\"},\"display_name\":{\"type\":\"string\",\"description\":\"The display name of the assignee.\",\"example\":\"John Doe\"},\"email\":{\"type\":\"string\",\"description\":\"The email address of the assignee.\",\"format\":\"email\",\"example\":\"john.doe@example.com\"},\"avatar\":{\"type\":\"string\",\"description\":\"The avatar image URL of the assignee.\",\"format\":\"uri\",\"example\":\"https://example.com/avatar.jpg\"}}}},\"collaborators\":{\"type\":\"array\",\"description\":\"A list of all collaborators associated with the task.\\n Only the first 5 collaborators are returned.\",\"items\":{\"required\":[\"display_name\",\"user_id\"],\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the collaborator.\",\"example\":\"CcrEGgmeQem1uyJsuIRKwA\"},\"display_name\":{\"type\":\"string\",\"description\":\"The display name of the collaborator.\",\"example\":\"John Doe\"},\"email\":{\"type\":\"string\",\"description\":\"The email address of the collaborator.\",\"format\":\"email\",\"example\":\"john.doe@example.com\"},\"avatar\":{\"type\":\"string\",\"description\":\"The avatar image URL of the collaborator.\",\"format\":\"uri\",\"example\":\"https://example.com/avatar.jpg\"}}}},\"friendly_task_id\":{\"type\":\"string\",\"description\":\"The task friendly ID.\",\"example\":\"Task-1\"},\"link\":{\"type\":\"string\",\"description\":\"The task share link.\",\"example\":\"https://tasks.zoomdev.us/ws53iQgOOkOdzVTxZaVoRA/TASK-1\"},\"create_time\":{\"type\":\"string\",\"description\":\"The task create date-time in UTC/GMT.\",\"format\":\"date-time\",\"example\":\"2025-03-31T12:02:00Z\"},\"modify_time\":{\"type\":\"string\",\"description\":\"The task modify date-time in UTC/GMT.\",\"format\":\"date-time\",\"example\":\"2025-03-31T12:02:00Z\"},\"source_type\":{\"type\":\"string\",\"description\":\"Type of the source that created the task.\",\"example\":\"user\",\"enum\":[\"user\",\"aic_meeting_summary\",\"whiteboard\",\"notes\",\"docs\",\"phone\",\"chat\",\"email\",\"meeting\",\"aic\"]}}}}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `38004` <br>\\n invalid params {params_name} <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"TasksMgt:Read\",\"TasksContent:Read\"],\"x-macro-scopes\":[\"tasks:read\",\"tasks:read:admin\"],\"x-granular-scopes\":[\"tasks:read:list_tasks:admin\",\"tasks:read:list_tasks\"]}},\"post\":{\"tags\":[\"Tasks\"],\"summary\":\"Create a new task\",\"description\":\"Provide essential details to create a new task without file attachments.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:write:task`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"createTask\",\"requestBody\":{\"content\":{\"application/json\":{\"schema\":{\"required\":[\"title\"],\"type\":\"object\",\"properties\":{\"title\":{\"maxLength\":500,\"minLength\":1,\"type\":\"string\",\"description\":\"The tasks's title. Maximum 500 characters.\",\"example\":\"Complete project documentation\"},\"description\":{\"maxLength\":10000,\"minLength\":1,\"type\":\"string\",\"description\":\"The task's detailed description. Maximum 10,000 characters.\",\"example\":\"Write comprehensive documentation for the new API endpoints\"},\"priority\":{\"type\":\"string\",\"description\":\"The task's priority level.\",\"example\":\"Medium\",\"default\":\"Medium\",\"enum\":[\"Low\",\"Medium\",\"High\",\"Highest\"]},\"status\":{\"type\":\"string\",\"description\":\"The task's current status.\",\"example\":\"To do\",\"default\":\"To do\",\"enum\":[\"To do\",\"In progress\",\"Done\",\"Blocked\",\"Recommended\"]},\"due_date\":{\"type\":\"string\",\"description\":\"The task's due date in UTC, formatted as `yyyy-MM-dd`.\",\"format\":\"date\",\"example\":\"2025-09-24\"},\"is_public\":{\"type\":\"boolean\",\"description\":\"Determines the task's visibility.\\n* `true` - The task is accessible to all members of your organization.\\n* `false` - The task is accessible only to the assigned users and collaborators.\",\"example\":false,\"default\":false},\"starred\":{\"type\":\"boolean\",\"description\":\"Specifies whether the task is starred.\",\"example\":false,\"default\":false},\"collaborators\":{\"maxItems\":20,\"minItems\":0,\"type\":\"array\",\"description\":\"A list of collaborators to invite to the task. Maximum 20.\",\"items\":{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\",\"description\":\"The email address of the collaborator to invite. The potential collaborator must belong to the same account as the requesting user.\",\"example\":\"potential-collaborator@example.com\"}}}},\"invite_message\":{\"maxLength\":280,\"type\":\"string\",\"description\":\"Include an optional message when inviting a user as a collaborator or assignee. Maximum length is 200 characters.\",\"example\":\"Please join this task to help review the API documentation.\"},\"skip_notifications\":{\"type\":\"boolean\",\"description\":\"Whether to skip sending notifications to collaborators.\",\"example\":false},\"assignees\":{\"maxItems\":20,\"minItems\":0,\"type\":\"array\",\"description\":\"A list of assignees to add to the task. Maximum of 20. If empty, the request user is added by default.\",\"items\":{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\",\"description\":\"The email address of the assignee to invite. The potential assignee must belong to the same account as the requesting user.\",\"example\":\"invited-assignee@example.com\"}}}}}}}}},\"responses\":{\"201\":{\"description\":\"Task created successfully.\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"create_time\",\"is_public\",\"priority\",\"starred\",\"status\",\"task_id\",\"title\"],\"type\":\"object\",\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The created task's unique identifier.\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"},\"title\":{\"type\":\"string\",\"description\":\"The task's title.\",\"example\":\"Complete project documentation\"},\"description\":{\"type\":\"string\",\"description\":\"The task's detailed description.\",\"example\":\"Write comprehensive documentation for the new API endpoints\"},\"priority\":{\"type\":\"string\",\"description\":\"The task's priority level.\",\"example\":\"Medium\",\"default\":\"Medium\",\"enum\":[\"Low\",\"Medium\",\"High\",\"Highest\"]},\"status\":{\"type\":\"string\",\"description\":\"The task's current status.\",\"example\":\"To do\",\"default\":\"To do\",\"enum\":[\"To do\",\"In progress\",\"Done\",\"Blocked\",\"Recommended\"]},\"is_public\":{\"type\":\"boolean\",\"description\":\"Determines the task's visibility.\\n* `true` - The task is accessible to all members of your organization.\\n* `false` - The task is accessible only to the assigned users and collaborators.\",\"example\":false,\"default\":false},\"due_date\":{\"type\":\"string\",\"description\":\"The task's due date in UTC, formatted as `yyyy-MM-dd`.\",\"format\":\"date\",\"example\":\"2025-09-24\"},\"starred\":{\"type\":\"boolean\",\"description\":\"Indicates whether the task is starred by the user.\",\"example\":false},\"collaborators\":{\"type\":\"array\",\"description\":\"A list of collaborators associated with the task.\",\"items\":{\"required\":[\"user_id\"],\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The collaborator's Zoom user ID.\",\"example\":\"w0RChiauQeqRlv5fgxYULQ\"},\"email\":{\"type\":\"string\",\"description\":\"The email address of the collaborator.\",\"example\":\"john.doe@example.com\"}}}},\"assignees\":{\"type\":\"array\",\"description\":\"A list of all assignees associated with the task. \",\"items\":{\"required\":[\"user_id\"],\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The assignee's Zoom user ID.\",\"example\":\"w0RChiauQeqRlv5fgxYULQ\"},\"email\":{\"type\":\"string\",\"description\":\"The email address of the assignee.\",\"example\":\"invited-assignee@example.com\"}}}},\"create_time\":{\"type\":\"string\",\"description\":\"The creation date and time in UTC/GMT format. \",\"format\":\"date-time\",\"example\":\"2025-03-25T07:29:29Z\"},\"friendly_task_id\":{\"type\":\"string\",\"description\":\"The human-readable identifier of the task.\",\"example\":\"Task-1\"},\"link\":{\"type\":\"string\",\"description\":\"The direct URL link to the task.\",\"example\":\"https://tasks.example.com/ws53iQgOOkOdzVTxZaVoRA/TASK-1\"}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `34002` <br>\\n Invalid email {email}. <br>\\n**Error Code:** `34002` <br>\\n The title is too long. <br>\\n**Error Code:** `34002` <br>\\n The description is too long. <br>\\n**Error Code:** `34002` <br>\\n Invalid field {field_name}. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[],\"x-macro-scopes\":[\"tasks:write\"],\"x-granular-scopes\":[\"tasks:write:task\"]}}},\"/tasks/items/{taskId}\":{\"get\":{\"tags\":[\"Tasks\"],\"summary\":\"Get task details\",\"description\":\"Retrieves detailed information about a specific task by it\u2019s ID\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read`,`tasks:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:read:task`,`tasks:read:task:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"getTaskDetail\",\"parameters\":[{\"name\":\"taskId\",\"in\":\"path\",\"description\":\"The unique identifier of the task.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"}}],\"responses\":{\"200\":{\"description\":\"Task details retrieved successfully\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"create_time\",\"is_ai_generated\",\"is_public\",\"modify_time\",\"priority\",\"source_type\",\"starred\",\"status\",\"task_id\",\"title\"],\"type\":\"object\",\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the task.\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"},\"title\":{\"type\":\"string\",\"description\":\"The title of the task.\",\"example\":\"the tasks title demo\"},\"description\":{\"type\":\"string\",\"description\":\"A detailed description of the task.\",\"example\":\"the task description\"},\"priority\":{\"type\":\"string\",\"description\":\"The task's priority level.\",\"example\":\"Medium\",\"default\":\"Medium\",\"enum\":[\"Low\",\"Medium\",\"High\",\"Highest\"]},\"status\":{\"type\":\"string\",\"description\":\"The task's current status.\",\"example\":\"To do\",\"default\":\"To do\",\"enum\":[\"To do\",\"In progress\",\"Done\",\"Blocked\",\"Recommended\"]},\"is_public\":{\"type\":\"boolean\",\"description\":\"Determines the task's visibility.\\n* `true` - The task is accessible to all members of your organization.\\n* `false` - The task is accessible only to the assigned users and collaborators.\",\"example\":false,\"default\":false},\"due_date\":{\"type\":\"string\",\"description\":\"The task's due date in UTC, formatted as `yyyy-MM-dd`.\",\"format\":\"date\",\"example\":\"2025-09-24\"},\"is_ai_generated\":{\"type\":\"boolean\",\"description\":\"Whether the task was generated by AI.\",\"example\":false},\"starred\":{\"type\":\"boolean\",\"description\":\"Whether the task is starred by the user.\",\"example\":false},\"tasks_attachments\":{\"type\":\"array\",\"description\":\"List of attachments associated with the task\",\"example\":[],\"items\":{\"type\":\"object\",\"properties\":{\"attachment_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the attachment.\",\"example\":\"dz6nqEeQT6C1VoZFxteGTw\"},\"file_name\":{\"type\":\"string\",\"description\":\"The name of the attached file.\",\"example\":\"example.pdf\"},\"file_size\":{\"type\":\"integer\",\"description\":\"The size of the file in bytes.\",\"example\":1024},\"mime_type\":{\"type\":\"string\",\"description\":\"The MIME type of the file.\",\"example\":\"application/pdf\"},\"upload_time\":{\"type\":\"string\",\"description\":\"The file upload date-time in UTC/GMT.\",\"format\":\"date-time\",\"example\":\"2025-03-31T12:02:00Z\"},\"download_url\":{\"type\":\"string\",\"description\":\"The download link for the attachment will remain valid for 7,200 seconds.\",\"example\":\"https://file.zoom.us/file/dz6nqEeQT6C1VoZFxteGTw?jwt=xxx\"}}}},\"assignees\":{\"type\":\"array\",\"description\":\"A list of all assignees associated with the task. \",\"items\":{\"required\":[\"display_name\",\"user_id\"],\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the assignee.\",\"example\":\"w0RChiauQeqRlv5fgxYULQ\"},\"display_name\":{\"type\":\"string\",\"description\":\"The display name of the assignee.\",\"example\":\"John Doe\"},\"email\":{\"type\":\"string\",\"description\":\"The email address of the assignee.\",\"format\":\"email\",\"example\":\"john.doe@example.com\"},\"avatar\":{\"type\":\"string\",\"description\":\"The avatar image URL of the assignee.\",\"format\":\"uri\",\"example\":\"https://example.com/avatar.jpg\"}}}},\"collaborators\":{\"type\":\"array\",\"description\":\"A list of all collaborators associated with the task.\",\"items\":{\"required\":[\"display_name\",\"user_id\"],\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The Zoom user ID of the collaborator.\",\"example\":\"w0RChiauQeqRlv5fgxYULQ\"},\"display_name\":{\"type\":\"string\",\"description\":\"The display name of the collaborator.\",\"example\":\"John Doe\"},\"email\":{\"type\":\"string\",\"description\":\"The email address of the collaborator.\",\"format\":\"email\",\"example\":\"john.doe@example.com\"},\"avatar\":{\"type\":\"string\",\"description\":\"The avatar image URL of the collaborator.\",\"format\":\"uri\",\"example\":\"https://example.com/avatar.jpg\"}}}},\"source_type\":{\"type\":\"string\",\"description\":\"Type of the source that created the task.\",\"example\":\"user\",\"enum\":[\"user\",\"aic_meeting_summary\",\"whiteboard\",\"notes\",\"docs\",\"phone\",\"chat\",\"email\",\"meeting\",\"aic\"]},\"friendly_task_id\":{\"type\":\"string\",\"description\":\"The task friendly ID.\",\"example\":\"Task-1\"},\"link\":{\"type\":\"string\",\"description\":\"The task share link.\",\"example\":\"https://tasks.zoomdev.us/ws53iQgOOkOdzVTxZaVoRA/TASK-1\"},\"associated_meeting\":{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"description\":\"[Meeting ID](https://support.zoom.us/hc/en-us/articles/201362373-What-is-a-Meeting-ID-): Unique identifier of the meeting in **long** format, represented as int64 data type in JSON, also known as the meeting number.\",\"format\":\"int64\",\"example\":97763643886},\"uuid\":{\"type\":\"string\",\"description\":\"The unique Meeting ID. Each meeting instance will generate its own Meeting UUID.\",\"example\":\"aDYlohsHRtCd4ii1uC2+hA==\"},\"topic\":{\"type\":\"string\",\"description\":\"The topic of the associated meeting\",\"example\":\"Tasks Meeting\"},\"meeting_date\":{\"type\":\"string\",\"description\":\"The meeting start date-time in UTC/GMT.\",\"format\":\"date-time\",\"example\":\"2025-03-31T12:02:00Z\"}},\"description\":\"Associated meeting information\"},\"create_time\":{\"type\":\"string\",\"description\":\"The task create date-time in UTC/GMT.\",\"format\":\"date-time\",\"example\":\"2025-03-31T12:02:00Z\"},\"modify_time\":{\"type\":\"string\",\"description\":\"The task modify date-time in UTC/GMT.\",\"format\":\"date-time\",\"example\":\"2025-03-31T12:02:00Z\"}}}}}},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111800` <br>\\n You don't have permission to request the task. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `112100` <br>\\n The resource not found. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"TasksMgt:Read\",\"TasksContent:Read\"],\"x-macro-scopes\":[\"tasks:read\",\"tasks:read:admin\"],\"x-granular-scopes\":[\"tasks:read:task\",\"tasks:read:task:admin\"]}},\"delete\":{\"tags\":[\"Tasks\"],\"summary\":\"Delete a task\",\"description\":\"Permanently delete a task and all its associated data including descriptions, attachments, collaborators, and integrations. This operation cannot be undone.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:delete`,`tasks:delete:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:delete:task:admin`,`tasks:delete:task`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"deleteTask\",\"parameters\":[{\"name\":\"taskId\",\"in\":\"path\",\"description\":\"The unique identifier of the task to delete\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"}}],\"responses\":{\"204\":{\"description\":\"Task deleted successfully\"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111800` <br>\\n You don't have permission to request the task. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `112100` <br>\\n The resource not found. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"TasksMgt:Edit\"],\"x-macro-scopes\":[\"tasks:delete\",\"tasks:delete:admin\"],\"x-granular-scopes\":[\"tasks:delete:task:admin\",\"tasks:delete:task\"]}},\"patch\":{\"tags\":[\"Tasks\"],\"summary\":\"Update task fields\",\"description\":\"Updates specific fields of an existing task. Only provided fields will be updated, others will remain unchanged.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:write`,`tasks:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `tasks:update:task`,`tasks:update:task:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"updateTask\",\"parameters\":[{\"name\":\"taskId\",\"in\":\"path\",\"description\":\"The unique identifier of the task to update\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"z2vo9qNRRHKArlJqkVHNUw\"}}],\"requestBody\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"title\":{\"maxLength\":500,\"minLength\":1,\"type\":\"string\",\"description\":\"The title of the task. \\nMaximum 500 characters.\",\"example\":\"Complete project documentation\"},\"description\":{\"maxLength\":10000,\"minLength\":1,\"type\":\"string\",\"description\":\"A detailed description of the task. \\nMaximum 10,000 characters.\",\"example\":\"Write comprehensive documentation for the new API endpoints\"},\"priority\":{\"type\":\"string\",\"description\":\"The task's priority level.\",\"example\":\"Medium\",\"default\":\"Medium\",\"enum\":[\"Low\",\"Medium\",\"High\",\"Highest\"]},\"status\":{\"type\":\"string\",\"description\":\"The task's current status.\",\"example\":\"To do\",\"default\":\"To do\",\"enum\":[\"To do\",\"In progress\",\"Done\",\"Blocked\",\"Recommended\"]},\"due_date\":{\"type\":\"string\",\"description\":\"The task's due date in UTC, formatted as `yyyy-MM-dd`.\",\"format\":\"date\",\"example\":\"2025-09-24\"},\"is_public\":{\"type\":\"boolean\",\"description\":\"Determines the task's visibility.\\n* `true` - The task is accessible to all members of your organization.\\n* `false` - The task is accessible only to the assigned users and collaborators.\",\"example\":false,\"default\":false},\"starred\":{\"type\":\"boolean\",\"description\":\"Whether the task should be starred.\",\"example\":true}}}}}},\"responses\":{\"204\":{\"description\":\"Task updated successfully\"},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `34002` <br>\\n Invalid field {field_name}. <br>\\n**Error Code:** `34002` <br>\\n The title is too long. <br>\\n**Error Code:** `34002` <br>\\n The description is too long. <br>\\n\"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111800` <br>\\n You don't have permission to request the task. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"TasksMgt:Edit\"],\"x-macro-scopes\":[\"tasks:write\",\"tasks:write:admin\"],\"x-granular-scopes\":[\"tasks:update:task\",\"tasks:update:task:admin\"]}}}},\"components\":{\"securitySchemes\":{\"openapi_authorization\":{\"type\":\"http\",\"scheme\":\"bearer\"}}}}",
    "frontmatter": {
      "title": "Tasks APIs",
      "keywords": "zoom, rest, api, ai, aic, tasks",
      "description": "The Zoom Tasks APIs let you manage and automate tasks within the Zoom ecosystem. Create, view, update, and delete tasks that help teams stay organized across meetings and projects. The Zoom Tasks REST API lets developers programmatically integrate task management into their workflows, enabling seamless coordination and productivity directly through Zoom.",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
