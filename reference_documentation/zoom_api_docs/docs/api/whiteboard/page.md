# Whiteboard APIs

- Source URL: https://developers.zoom.us/docs/api/whiteboard/
- Snapshot path: docs/api/whiteboard
- Generated (UTC): 2026-02-07T22:40:11.977429+00:00

## Frontmatter

```json
{
  "title": "Whiteboard APIs",
  "keywords": "zoom, marketplace, whiteboard, webhook, event, create, update, delete, data, summary",
  "description": "The [Zoom Whiteboard APIs](https://developers.zoom.us/docs/zoom-whiteboard/) allow you to manage your Zoom Whiteboard. For example, collaborative persistent whiteboard that allows participants to share and collaborate on a virtual whiteboard in real time The Zoom Whiteboard REST API for [Zoom Whiteboard](https://developers.zoom.us/docs/api/rest/whiteboard/) allows developers to interact with various Zoom features, including Zoom Whiteboard. Developers can create and manage whiteboards programmatically using these APIs.",
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
    "title": "Whiteboard",
    "description": "The [Zoom Whiteboard APIs](https://developers.zoom.us/docs/zoom-whiteboard/) allow you to manage your Zoom Whiteboard. For example, collaborative persistent whiteboard that allows participants to share and collaborate on a virtual whiteboard in real time The Zoom Whiteboard REST API for [Zoom Whiteboard](https://developers.zoom.us/docs/api/rest/whiteboard/) allows developers to interact with various Zoom features, including Zoom Whiteboard. Developers can create and manage whiteboards programmatically using these APIs.",
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
    "/whiteboards/sessions": {
      "get": {
        "tags": [
          "Archiving"
        ],
        "summary": "List whiteboards sessions",
        "description": "Creates a list of archived sessions under current account.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_content:read`,`whiteboard_content:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:list_sessions`,`whiteboard:read:list_sessions:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "Createwhiteboardsarchivefiles",
        "parameters": [
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of records returned within a single API call.",
            "required": false,
            "schema": {
              "maximum": 300,
              "type": "integer",
              "example": 30,
              "default": 30
            }
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "The next page token paginates through a large set of results. A next page token returns whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "IAfJX3jsOLW7w3dokmFl84zOa0MAVGyMEB2"
            }
          },
          {
            "name": "from",
            "in": "query",
            "description": "The query session\u00a0`start_time\u00a0`date in`\u00a0yyyy-MM-dd'T'HH:mm:ssZ`\u00a0format. The\u00a0`start_time`\u00a0of a session in the result list should be greater than or equal to\u00a0`from`. If this field has no provided value, the default is the current date. For example, if you make the API request on June 30, 2020, without providing the\u00a0`from`\u00a0and\u00a0`to`\u00a0parameters, by default, the value of\u00a0the `from`\u00a0field will be \u201c2020-06-30,\u201d and the value of the\u00a0`to`\u00a0field will be \u201c2020-07-30\u201d.",
            "required": false,
            "schema": {
              "type": "string",
              "format": "date-time",
              "example": "2021-03-12T02:12:27Z"
            }
          },
          {
            "name": "to",
            "in": "query",
            "description": "The query session`end_time` date, in `yyyy-MM-dd'T'HH:mm:ssZ` format. The `end_time` of session in result list should be less than or equal to `to`.",
            "required": false,
            "schema": {
              "type": "string",
              "format": "date-time",
              "example": "2021-03-18T05:41:36Z"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "The returned archived sessions list.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "from",
                    "next_page_token",
                    "page_size",
                    "to"
                  ],
                  "type": "object",
                  "properties": {
                    "from": {
                      "type": "string",
                      "description": "The queried start date.",
                      "format": "date-time",
                      "example": "2021-03-12T02:12:27Z"
                    },
                    "to": {
                      "type": "string",
                      "description": "The queried end date.",
                      "format": "date-time",
                      "example": "2021-03-12T02:12:27Z"
                    },
                    "page_size": {
                      "maximum": 300,
                      "type": "integer",
                      "description": "The number of records returned per page in a single API call.",
                      "example": 30,
                      "default": 30
                    },
                    "next_page_token": {
                      "type": "string",
                      "description": "The next page token paginates through a large set of results. A next page token returns whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.\n\nNote: If you use `next_page_token` as a parameter, your other request parameters should be changeless to make sure that the large result set is what you want. For example, if your to parameter is for a future time, Zoom resets this value to the current time and returns this value in the response body, along with the `next_page_token` value. Use these same to and `next_page_token` values in requests for the remaining results set; otherwise you will get an invalid token error.",
                      "example": "IAfJX3jsOLW7w3dokmFl84zOa0MAVGyMEB2"
                    },
                    "sessions": {
                      "maxItems": 300,
                      "type": "array",
                      "description": "The whiteboard archived sessions list. ",
                      "items": {
                        "type": "object",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The session's unique ID.",
                            "example": "a2f19f96-9294-4f51-8134-6f0eea108eb2"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The session's start time.\nIt should be `from < start_time`",
                            "format": "date-time",
                            "example": "2021-03-11T05:41:36Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The session's end time.\nIt should be `end_time < to`",
                            "format": "date-time",
                            "example": "2021-03-11T05:41:36Z"
                          },
                          "whiteboard_id": {
                            "type": "string",
                            "description": "The ID of whiteboard.",
                            "example": "HY8-AC_HSou4SXY1S-OmOg"
                          },
                          "whiteboard_name": {
                            "type": "string",
                            "description": "The name of whiteboard.",
                            "example": "example_whiteboard_name"
                          },
                          "expire_time": {
                            "type": "string",
                            "description": "The session's expire time. ",
                            "format": "date-time",
                            "example": "2021-03-26T06:41:36Z"
                          },
                          "regulated_participants": {
                            "maxItems": 1000,
                            "type": "array",
                            "description": "The session participant under this account.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "user_id": {
                                  "type": "string",
                                  "description": "The unique ID of participant user in this session.",
                                  "example": "wf3h7uo1dk0v53bl"
                                },
                                "user_name": {
                                  "type": "string",
                                  "description": "The name of participant user in this session.",
                                  "example": "Jack"
                                },
                                "join_whiteboard_time": {
                                  "type": "string",
                                  "description": "The user join the whiteboard time.",
                                  "format": "date-time",
                                  "example": "2021-03-26T06:41:36Z"
                                },
                                "leave_whiteboard_time": {
                                  "type": "string",
                                  "description": "The user leave the whiteboard time.",
                                  "format": "date-time",
                                  "example": "2021-03-26T07:41:36Z"
                                },
                                "email": {
                                  "type": "string",
                                  "description": "The email of participant. `nullable` If the participant is not part of the session's account, this returns an empty string value.",
                                  "example": "user@example.com"
                                }
                              },
                              "description": "The regulated user info under the account"
                            }
                          }
                        }
                      }
                    },
                    "total_records": {
                      "type": "integer",
                      "description": "The total number of returned sessions records.",
                      "example": 20
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid parameter. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardContent:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_content:read",
            "whiteboard_content:read:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:read:list_sessions",
            "whiteboard:read:list_sessions:admin"
          ]
        }
      }
    },
    "/whiteboards/sessions/activity/download/{path}": {
      "get": {
        "tags": [
          "Archiving"
        ],
        "summary": "Download Whiteboards activity file",
        "description": "Use this API to download the Whiteboard archival file. For obtaining the download URL, please consult the [List whiteboard sessions activities](https://developers.zoom.us/docs/api/rest/reference/whiteboard/methods/#operation/Listwhiteboardsessionsarchivedfiles) documentation. Scroll down under field on schema to documentation URL string.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_content:read`,`whiteboard_content:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:archived_file`,`whiteboard:read:archived_file:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "Downloadwhiteboardsactivityfile",
        "parameters": [
          {
            "name": "path",
            "in": "path",
            "description": "The file path.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "Qg75t7xZBtEbAkjdlgbfdngBBBB"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Returns a file stream.",
            "headers": {
              "Content-Type": {
                "description": "Content-Type.",
                "required": false,
                "schema": {
                  "type": "string",
                  "example": "application/octet-stream"
                }
              },
              "Content-Disposition": {
                "description": "Content-Disposition.",
                "required": false,
                "schema": {
                  "type": "string",
                  "example": "attachment;filename=adcc655d3a3549e692959737fe2a7a0d.pdf"
                }
              },
              "Content-Length": {
                "description": "Content-Length.",
                "required": false,
                "schema": {
                  "type": "string",
                  "example": "102400"
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid path. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `34000` <br>\n The resource not found. The file does not exist or the file has expired. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardContent:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_content:read",
            "whiteboard_content:read:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:read:archived_file",
            "whiteboard:read:archived_file:admin"
          ]
        }
      }
    },
    "/whiteboards/sessions/{seesionId}": {
      "get": {
        "tags": [
          "Archiving"
        ],
        "summary": "List whiteboard sessions activities",
        "description": "Lists a session's activity information.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_content:read`,`whiteboard_content:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:session`,`whiteboard:read:session:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "Listwhiteboardsessionsarchivedfiles",
        "parameters": [
          {
            "name": "seesionId",
            "in": "path",
            "description": "The session unique ID.\n",
            "required": true,
            "schema": {
              "type": "string",
              "example": "f8b9197a-0c8f-c8de-4db3-d8a5e8d9f505"
            }
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of records returned within a single API call.",
            "required": false,
            "schema": {
              "maximum": 300,
              "type": "integer",
              "example": 30,
              "default": 30
            }
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "The next page token paginates through a large set of results. A next page token returns whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "IAfJX3jsOLW7w3dokmFl84zOa0MAVGyMEB2"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "The session activity information.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "page_size",
                    "total_records"
                  ],
                  "type": "object",
                  "properties": {
                    "page_size": {
                      "type": "integer",
                      "description": "The number of records returned within a single API call.",
                      "example": 30
                    },
                    "next_page_token": {
                      "type": "string",
                      "description": "The next page token paginates through a large set of results. A next page token returns whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes. **Note**: if you use `next_page_token` as a parameter, your other request parameters should be changeless to make sure that the large result set is what you want. For example, if your to parameter is for a future time, Zoom resets this value to the current time and returns this value in the response body, along with the `next_page_token` value. Use these same to and `next_page_token` values in requests for the remaining results set; otherwise you will get an invalid token error.",
                      "example": "IAfJX3jsOLW7w3dokmFl84zOa0MAVGyMEB2"
                    },
                    "session_id": {
                      "type": "string",
                      "description": "The unique ID of current session.",
                      "example": "a2f19f96-9294-4f51-8134-6f0eea108eb2"
                    },
                    "whiteboard_id": {
                      "type": "string",
                      "description": "The ID of the Zoom Whiteboard.",
                      "example": "HY8-AC_HSou4SXY1S-OmOg"
                    },
                    "whiteboard_name": {
                      "type": "string",
                      "description": "The name of Zoom Whiteboard.",
                      "example": "example_whiteboard_name"
                    },
                    "start_time": {
                      "type": "string",
                      "description": "The session's start time.",
                      "format": "date-time",
                      "example": "2021-03-11T05:41:36Z"
                    },
                    "end_time": {
                      "type": "string",
                      "description": "The session's end time.",
                      "format": "date-time",
                      "example": "2021-03-11T06:41:36Z"
                    },
                    "expire_time": {
                      "type": "string",
                      "description": "The session's expire time. ",
                      "format": "date-time",
                      "example": "2021-03-26T06:41:36Z"
                    },
                    "activities": {
                      "maxItems": 300,
                      "type": "array",
                      "description": "The activity list of a session.",
                      "items": {
                        "type": "object",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The unique ID of an activity.",
                            "example": "cc63bce7-cb59-c1dc-fadd-2be734ad1d55"
                          },
                          "type": {
                            "type": "string",
                            "description": "The operation description.",
                            "example": "add object"
                          },
                          "download_url": {
                            "type": "string",
                            "description": " The URL to download the the archive file. OAuth apps If a user has authorized and installed your OAuth app that contains `whiteboard:read:archived_file` or `whiteboard:read:archived_file:admin` scopes, use the user's [OAuth access token](https://developers.zoom.us/docs/integrations/oauth/) to download the file. More details refer [Download Whiteboards activity file](https://developers.zoom.us/docs/api/rest/reference/whiteboard/methods/#operation/Downloadwhiteboardsactivityfile)\n For example:  `https://{{base-domain}}/whiteboards/sesssions/activity/download/xxx--header 'Authorization: Bearer {{OAuth-access-token}}'`",
                            "example": "https://example.com/whiteboards/sessions/activity/download/Qg75t7xZBtEbAkjdlgbfdngBBBB"
                          },
                          "create_time": {
                            "type": "string",
                            "description": "The activity create time.",
                            "format": "date-time",
                            "example": "2021-03-11T05:41:36Z"
                          },
                          "user_id": {
                            "type": "string",
                            "description": "The ID of user. `nullable`  If the participant is **not** part of the session's account, this returns an empty string value.",
                            "example": "wf3h7uo1dk0v53bl"
                          },
                          "collaborator_id": {
                            "type": "string",
                            "description": "The participant's unique ID. `not nullable` ",
                            "example": "x4g6hy789k0o878k"
                          },
                          "collaborator_name": {
                            "type": "string",
                            "description": "The participant's display name.",
                            "example": "Jack"
                          },
                          "email": {
                            "type": "string",
                            "description": "The email of participant. `nullable` If the participant is not part of the session's account, this returns an empty string value.",
                            "example": "user@example.com"
                          },
                          "content": {
                            "type": "string",
                            "description": "The text content generated by the activity.",
                            "example": "This is a activity content demo"
                          },
                          "object_ids": {
                            "maxItems": 100,
                            "minItems": 0,
                            "type": "array",
                            "description": "The IDs of the whiteboard object associated with this activity.",
                            "items": {
                              "type": "string",
                              "description": "The object ID in the whiteboard.",
                              "example": "wfx4g6s1dk0v53bl"
                            }
                          }
                        },
                        "description": "activity information"
                      }
                    },
                    "total_records": {
                      "type": "number",
                      "description": "The total number of returned activities records.",
                      "example": 30
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
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `34000` <br>\n The session not exist or expired. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardContent:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_content:read",
            "whiteboard_content:read:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:read:session",
            "whiteboard:read:session:admin"
          ]
        }
      }
    },
    "/whiteboards/{whiteboardId}/collaborator": {
      "get": {
        "tags": [
          "Collaborator"
        ],
        "summary": "Get collaborators of a whiteboard",
        "description": "Returns a list of a whiteboard collaborators. \n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_collaborator:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:list_collaborators:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "GetAWhiteboardCollaborator",
        "parameters": [
          {
            "name": "whiteboardId",
            "in": "path",
            "description": "The ID of the whiteboard.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "HY8-AC_HSou4SXY1S-OmOg"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "**HTTP Status Code:** `200`   \n \nWhiteboard collaborators returned.",
            "content": {
              "application/json": {
                "schema": {
                  "maxItems": 1000,
                  "type": "array",
                  "items": {
                    "type": "object",
                    "properties": {
                      "collaborator_id": {
                        "type": "string",
                        "description": "`not nullable` The collaborator's unique ID.",
                        "example": "x4g6hy789k0o878k"
                      },
                      "collaborator_name": {
                        "type": "string",
                        "description": "The collaborator's display name.",
                        "example": "Jack"
                      },
                      "user_id": {
                        "type": "string",
                        "description": "`nullable` User ID. This response only returns if the `collaborator_type` is `0`. If the `collaborator_type` is **not** `0`, this returns null value.",
                        "example": "wf3h7uo1dk0v53bl"
                      },
                      "channel_id": {
                        "type": "string",
                        "description": "`nullable` Team chat channel ID. This response only returns if the `collaborator_type` is `2`. If the `collaborator_type` is **not** `2`, this returns null value.",
                        "example": "cabc1234567defghijkl01234"
                      },
                      "avatar": {
                        "type": "string",
                        "description": "The collaborator profile picture.",
                        "example": "https://content.zoom.us/swerwerw.jpeg"
                      },
                      "role": {
                        "type": "integer",
                        "description": "The collaborator role type. \nEnum: \n* `0` - OWNER \n* `1` - CO_OWNER \n* `2` - EDITOR \n* `3` - COMMENTER \n* `4` - VIEWER \n",
                        "example": 0,
                        "enum": [
                          0,
                          1,
                          2,
                          3,
                          4
                        ],
                        "x-enum-descriptions": [
                          "OWNER",
                          "CO_OWNER",
                          "EDITOR",
                          "COMMENTER",
                          "VIEWER"
                        ]
                      },
                      "collaborator_type": {
                        "type": "integer",
                        "description": "The collaborator type. \nEnum: \n* `0` - SAME_ACCOUNT_USER \n* `1` - EXTERNAL_USER \n* `2` - CHANNEL \n",
                        "example": 0,
                        "enum": [
                          0,
                          1,
                          2
                        ],
                        "x-enum-descriptions": [
                          "SAME_ACCOUNT_USER",
                          "EXTERNAL_USER",
                          "CHANNEL"
                        ]
                      },
                      "email": {
                        "type": "string",
                        "description": "`nullable` Email. This response only returns if the `collaborator_type` is `0`. If the `collaborator_type` is **not** `0`, this returns null value.",
                        "example": "user@example.com"
                      }
                    },
                    "description": "A user or a team chat channel who works jointly on a whiteboard."
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this document. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_collaborator:read:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:read:list_collaborators:admin"
          ]
        }
      },
      "post": {
        "tags": [
          "Collaborator"
        ],
        "summary": "Share a whiteboard to new users or team chat channels.",
        "description": "Add collaborators to a whiteboard.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_collaborator:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:collaborator:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "AddAWhiteboardCollaborator",
        "parameters": [
          {
            "name": "whiteboardId",
            "in": "path",
            "description": "The ID of the whiteboard.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "HY8-AC_HSou4SXY1S-OmOg"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "skip_notifications": {
                    "type": "boolean",
                    "description": " **Default is false (notifications will be sent)**.\n If set to `true`, collaborators and owners will not receive direct messages or emails when the whiteboard is shared.",
                    "example": false
                  },
                  "invite_message": {
                    "maxLength": 280,
                    "minLength": 0,
                    "type": "string",
                    "description": "The custom message to include with the whiteboard invitation.",
                    "example": "I'm glad to invite you to join this whiteboard."
                  },
                  "collaborators": {
                    "maxItems": 100,
                    "minItems": 1,
                    "type": "array",
                    "description": "Array of collaborator objects to be added to the whiteboard.",
                    "items": {
                      "type": "object",
                      "properties": {
                        "email": {
                          "type": "string",
                          "description": "**User Email Address**\nThe email address of the user you want to share the whiteboard with.\nThis field is **required** when sharing a whiteboard with an individual user.\nIt **must be left empty** when sharing via a channel.\n**Note**: email and channel_id cannot be set at the same time.",
                          "example": "user@example.com."
                        },
                        "channel_id": {
                          "type": "string",
                          "description": "**Channel ID**\nThe unique identifier of the channel you want to share the whiteboard with.\nThis field is **required** when sharing a whiteboard via a channel.\nIt **must be left empty** when sharing with an individual user by email.\n**Note**: channel_id and email cannot be set at the same time.",
                          "example": "cabc1234567defghijkl01234"
                        },
                        "role": {
                          "type": "integer",
                          "description": "Collaborator role type. \nWhen assigning a new `OWNER` or `CO_OWNER`, the specified user must belong to the same account as the original `OWNER`. If a new owner is designated for a whiteboard, the original owner will automatically be demoted to `CO_OWNER`.\n\nRole Enum Values: \n* `0` - OWNER \n* `1` - CO_OWNER \n* `2` - EDITOR \n* `3` - COMMENTER \n* `4` - VIEWER ",
                          "example": 1,
                          "enum": [
                            0,
                            1,
                            2,
                            3,
                            4
                          ],
                          "x-enum-descriptions": [
                            "OWNER",
                            "CO_OWNER",
                            "EDITOR",
                            "COMMENTER",
                            "VIEWER"
                          ]
                        }
                      },
                      "description": "A user or a team chat channel who works jointly on a whiteboard. \n* `Email` or `ChannelId` is required according to the audience `type`."
                    }
                  }
                },
                "description": "Users or team chat channels to be added as collaborators to the whiteboard."
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "**HTTP Status Code:** `201`   \n\n Whiteboard added collaborators returned.",
            "content": {
              "application/json": {
                "schema": {
                  "maxItems": 100,
                  "type": "array",
                  "items": {
                    "type": "object",
                    "properties": {
                      "collaborator_id": {
                        "type": "string",
                        "description": "The collaborator's unique identifier. This field is not nullable.",
                        "example": "x4g6hy789k0o878k"
                      },
                      "collaborator_name": {
                        "type": "string",
                        "description": "The collaborator's display name.",
                        "example": "Jack"
                      },
                      "user_id": {
                        "type": "string",
                        "description": "`nullable` User ID. This response only returns if the `collaborator_type` is `0`. If the `collaborator_type` is **not** `0`, this returns null value.",
                        "example": "wf3h7uo1dk0v53bl"
                      },
                      "channel_id": {
                        "type": "string",
                        "description": "`nullable` Team chat channel ID. This response only returns if the `collaborator_type` is `2`. If the `collaborator_type` is **not** `2`, this returns null value.",
                        "example": "cabc1234567defghijkl01234"
                      },
                      "avatar": {
                        "type": "string",
                        "description": "Collaborator profile picture.",
                        "example": "https://content.zoom.us/swerwerw.jpeg"
                      },
                      "role": {
                        "type": "integer",
                        "description": "Collaborator role type. \nEnum: \n* `0` - OWNER \n* `1` - CO_OWNER \n* `2` - EDITOR \n* `3` - COMMENTER \n* `4` - VIEWER \n",
                        "example": 0,
                        "enum": [
                          0,
                          1,
                          2,
                          3,
                          4
                        ],
                        "x-enum-descriptions": [
                          "OWNER",
                          "CO_OWNER",
                          "EDITOR",
                          "COMMENTER",
                          "VIEWER"
                        ]
                      },
                      "collaborator_type": {
                        "type": "integer",
                        "description": "Collaborator type. \nEnum: \n* `0` - SAME_ACCOUNT_USER \n* `1` - EXTERNAL_USER \n* `2` - CHANNEL \n",
                        "example": 0,
                        "enum": [
                          0,
                          1,
                          2
                        ],
                        "x-enum-descriptions": [
                          "SAME_ACCOUNT_USER",
                          "EXTERNAL_USER",
                          "CHANNEL"
                        ]
                      }
                    },
                    "description": "A user or team chat channel who works jointly on a whiteboard."
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
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
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_collaborator:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:write:collaborator:admin"
          ]
        }
      },
      "patch": {
        "tags": [
          "Collaborator"
        ],
        "summary": "Update whiteboard collaborators",
        "description": "Use this API to update whiteboard collaborators. \n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_collaborator:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:update:collaborator:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "UpdateAWhiteboardCollaborator",
        "parameters": [
          {
            "name": "whiteboardId",
            "in": "path",
            "description": "The ID of the whiteboard.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "HY8-AC_HSou4SXY1S-OmOg"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "collaborators": {
                    "maxItems": 100,
                    "minItems": 1,
                    "type": "array",
                    "description": "An array of collaborator objects.",
                    "items": {
                      "type": "object",
                      "properties": {
                        "collaborator_id": {
                          "type": "string",
                          "description": "The collaborator's unique ID `not nullable` .",
                          "example": "x4g6hy789k0o878k"
                        },
                        "role": {
                          "type": "integer",
                          "description": "Collaborator role type. \nWhen assigning a new `OWNER` or `CO_OWNER`, the specified user must belong to the same account as the original `OWNER`. If a new owner is designated for a whiteboard, the original owner will automatically be demoted to `CO_OWNER`.\n\nRole Enum Values: \n* `0` - OWNER \n* `1` - CO_OWNER \n* `2` - EDITOR \n* `3` - COMMENTER \n* `4` - VIEWER ",
                          "example": 0,
                          "enum": [
                            0,
                            1,
                            2,
                            3,
                            4
                          ],
                          "x-enum-descriptions": [
                            "OWNER",
                            "CO_OWNER",
                            "EDITOR",
                            "COMMENTER",
                            "VIEWER"
                          ]
                        }
                      }
                    }
                  }
                },
                "description": "Update collaborators permission of a whiteboard."
              }
            }
          }
        },
        "responses": {
          "204": {
            "description": "**HTTP Status Code**: `204`   \n \nWhiteboard collaborators updated successfully."
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid collaboratorId {collaboratorId}. <br>\n"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this document. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `34000` <br>\n The resource {whiteboardId} not exist. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_collaborator:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:update:collaborator:admin"
          ]
        }
      }
    },
    "/whiteboards/{whiteboardId}/collaborator/{collaboratorId}": {
      "delete": {
        "tags": [
          "Collaborator"
        ],
        "summary": "Remove the collaborator from a whiteboard",
        "description": "Removes a collaborator from a whiteboard. \n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_collaborator:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:delete:collaborator:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "DeleteAWhiteboardCollaborator",
        "parameters": [
          {
            "name": "whiteboardId",
            "in": "path",
            "description": "The ID of the whiteboard.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "HY8-AC_HSou4SXY1S-OmOg"
            }
          },
          {
            "name": "collaboratorId",
            "in": "path",
            "description": "The ID of the whiteboard collaborator.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "sWvlvz-G5XFQxpfxEBB0PfVALRdcpHdTAqwqigrOPb1TgkxtFJI.3Z6nozeexgNAEecI"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "**HTTP Status Code**: `204`   \n \nWhiteboard collaborators deleted successfully."
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this document. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `34002` <br>\n Resource not found.  <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_collaborator:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:delete:collaborator:admin"
          ]
        }
      }
    },
    "/whiteboards": {
      "get": {
        "tags": [
          "Document"
        ],
        "summary": "List all whiteboards",
        "description": "Use this API to list whiteboards of current user (**Required Scopes**: `whiteboard:read`) or list all whiteboards under the account (**Required Scopes**: `whiteboard:read:admin`).\n\n**Note:** `user_id` can intake `Zoom user ID` or `email` which only applies when scope = `whiteboard:read:admin`. Otherwise, the `user_id` can only intake `me`.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:admin`,`whiteboard:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:list_whiteboards:admin`,`whiteboard:read:list_whiteboards`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "ListWhiteboards",
        "parameters": [
          {
            "name": "search_key",
            "in": "query",
            "description": "The name of a whiteboard. If this parameter is provided, only the matched whiteboard will be returned. Otherwise, all whiteboards will be returned.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "PGsQ3nwITmyTG4JkFNWJcg"
            }
          },
          {
            "name": "user_id",
            "in": "query",
            "description": "The email address or user ID of the whiteboard owner. If this parameter is provided, only the whiteboard owned by the provided user will be returned. \nIf set to `me`, will return the matched whiteboards of current user.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "me"
            }
          },
          {
            "name": "date_filter_type",
            "in": "query",
            "description": "The whiteboard date filter type. \nEnum:\n* `created_date` - Whiteboard create date. \n* `modified_date` - Whiteboard modify date.\n",
            "required": false,
            "schema": {
              "type": "string",
              "example": "modified_date",
              "default": "modified_date",
              "enum": [
                "created_date",
                "modified_date"
              ]
            }
          },
          {
            "name": "from",
            "in": "query",
            "description": "The queried start date and time in `yyyy-MM-dd'T'HH:mm:ss'Z'` UTC format for the created date range for which you would like to retrieve whiteboards. The maximum range can be a month. If no value is provided for this field, the default will be current date. For example, if you make the API request on June 30, 2020, without providing the &ldquo;from&rdquo; and &ldquo;to&rdquo; parameters, by default the value of 'from' field will be &ldquo;2020-06-30&rdquo; and the value of the 'to' field will be &ldquo;2020-07-01&rdquo;.",
            "required": false,
            "schema": {
              "type": "string",
              "format": "date-time",
              "example": "2022-03-08T16:00:00Z"
            }
          },
          {
            "name": "to",
            "in": "query",
            "description": "The queried end date and time in `yyyy-MM-dd'T'HH:mm:ss'Z'` UTC format. ",
            "required": false,
            "schema": {
              "type": "string",
              "format": "date-time",
              "example": "2022-03-08T16:00:00Z"
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
            "description": "The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42"
            }
          },
          {
            "name": "project_id",
            "in": "query",
            "description": "Project ID to search for.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "N6XZFDEQTkWSsMsmO_M1XQ"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "**HTTP Status Code:** `200`   \n \nWhiteboard list returned.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "Whiteboard List",
                  "description": "The whiteboard list.",
                  "allOf": [
                    {
                      "type": "object",
                      "properties": {
                        "whiteboards": {
                          "maxItems": 50,
                          "type": "array",
                          "description": "The information about the whiteboard list.",
                          "items": {
                            "allOf": [
                              {
                                "type": "object",
                                "properties": {
                                  "whiteboard_id": {
                                    "type": "string",
                                    "description": "Whiteboard ID",
                                    "example": "48Z4ppMfQJWONeVWFCcVzA"
                                  },
                                  "whiteboard_name": {
                                    "type": "string",
                                    "description": "Whiteboard name",
                                    "example": "Q3 Plan",
                                    "default": "Untitled"
                                  },
                                  "locked": {
                                    "type": "boolean",
                                    "description": "Indicates whether the whiteboard is locked or not.\n\n`true`: The whiteboard is locked.   \n \n\n`false`: The whiteboard is unlocked",
                                    "example": false,
                                    "default": false
                                  },
                                  "trashed": {
                                    "type": "boolean",
                                    "description": "Indicates whether the whiteboard is moved to trash or not.\n\n`true`: The whiteboard is moved to trash.   \n \n\n`false`: The whiteboard isn't moved to trash",
                                    "example": false,
                                    "default": false
                                  },
                                  "readonly": {
                                    "type": "boolean",
                                    "description": "Indicates whether the whiteboard is editable or not. If a whiteboard is locked, this field will set `true`. \n\n`true`: The whiteboard is editable.   \n \n\n`false`: The whiteboard is readonly",
                                    "example": false,
                                    "default": false
                                  },
                                  "non_cloud": {
                                    "type": "boolean",
                                    "description": "Indicates whether the whiteboard is allowed to be saved to cloud or not.\n\n`true`: The whiteboard is a saved to cloud.   \n \n\n`false`: The whiteboard isn't saved to cloud",
                                    "example": false,
                                    "default": false
                                  },
                                  "created_date": {
                                    "type": "string",
                                    "description": "Whiteboard create date-time in UTC/GMT. Example: &quot;2020-03-31T12:02:00Z&quot;",
                                    "format": "date-time",
                                    "example": "2022-03-25T07:29:29Z"
                                  },
                                  "modified_date": {
                                    "type": "string",
                                    "description": "Whiteboard update date-time in UTC/GMT. Example: &quot;2020-03-31T12:02:00Z&quot;",
                                    "format": "date-time",
                                    "example": "2022-03-25T07:29:29Z"
                                  },
                                  "share_link_setting": {
                                    "type": "object",
                                    "properties": {
                                      "share_scope": {
                                        "type": "integer",
                                        "description": "The access type of whiteboard share link. \n* Enum: \n* `0` - `Restricted`, Only people added can open with this link \n* `1` - `Same account` \n* `2` - Anyone \n* `3` - SAME_ACCOUNT_EXTERNAL_AUTH \n",
                                        "example": 0,
                                        "default": 0,
                                        "enum": [
                                          0,
                                          1,
                                          2,
                                          3
                                        ],
                                        "x-enum-descriptions": [
                                          "Restricted. Only people added can open with this link",
                                          "Same account",
                                          "Anyone",
                                          "Same account external auth"
                                        ]
                                      },
                                      "share_role": {
                                        "type": "integer",
                                        "description": "The access level of whiteboard share link. This field need to used with `share_scope`. If `share_scope` set `0` - `Restricted`, this field is null. If `share_scope` set `1` - `Same account` or `2` - `Anyone`, this filed can be `2` - `Editor`, `3` - `Commenter` or `4` - `Viewer`. \n* Enum: \n* `2` - `Editor`\n* `3` - `Commenter` \n* `4` - Viewer \n",
                                        "example": 2,
                                        "enum": [
                                          2,
                                          3,
                                          4
                                        ],
                                        "x-enum-descriptions": [
                                          "Editor",
                                          "Commenter",
                                          "Viewer"
                                        ]
                                      }
                                    },
                                    "description": "The access level, access type of current whiteboard share link."
                                  },
                                  "advanced_share_setting": {
                                    "type": "object",
                                    "properties": {
                                      "can_share": {
                                        "type": "boolean",
                                        "description": "Indicate whether users can share at or below their permission level or not. \n\n`true`: Users can share at or below their permission level.   \n \n\n`false`: Users cannot share at or below their permission level.",
                                        "example": true
                                      },
                                      "share_to_external": {
                                        "type": "boolean",
                                        "description": "Indicate whether internal users can share with external users or not. \n\n`true`: Internal users can share with external users.   \n \n\n`false`: Internal users cannot share with external users.",
                                        "example": true
                                      },
                                      "external_can_share": {
                                        "type": "boolean",
                                        "description": "Indicate whether external user can share the whiteboard to others or not. \n\n`true`: External user can share the whiteboard to others.   \n \n\n`false`:External user cannot share the whiteboard to others.",
                                        "example": true
                                      }
                                    },
                                    "description": "Whiteboard advanced share setting."
                                  },
                                  "project_id": {
                                    "type": "string",
                                    "description": "qiMWddmtRxqwSFrK22lJPg",
                                    "example": "The Project ID"
                                  },
                                  "created_by_display_name": {
                                    "type": "string",
                                    "description": "The creator display name of the whiteboard.",
                                    "example": "Jack"
                                  }
                                },
                                "description": "Whiteboard object"
                              }
                            ]
                          }
                        },
                        "next_page_token": {
                          "type": "string",
                          "description": "The next page token paginates through a large set of results. A next page token is returned when the available results exceed the current page size. The expiration period for this token is 15 minutes.",
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
                  ]
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Read"
          ],
          "x-macro-scopes": [
            "whiteboard:read:admin",
            "whiteboard:read"
          ],
          "x-granular-scopes": [
            "whiteboard:read:list_whiteboards:admin",
            "whiteboard:read:list_whiteboards"
          ]
        }
      },
      "post": {
        "tags": [
          "Document"
        ],
        "summary": "Create a new whiteboard",
        "description": "Creates a new whiteboard for a user.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:whiteboard`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "newWhiteboardCreate",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "name": {
                    "type": "string",
                    "description": "The whiteboard name.",
                    "example": "Q3 Plan",
                    "default": "Untitled"
                  }
                },
                "description": "Create a new whiteboard request body."
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "**HTTP Status Code:** `201`   \n \nWhiteboard created successfully.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "whiteboard_id": {
                      "type": "string",
                      "description": "The whiteboard ID.",
                      "example": "48Z4ppMfQJWONeVWFCcVzA"
                    },
                    "whiteboard_name": {
                      "type": "string",
                      "description": "The whiteboard name.",
                      "example": "Q3 Plan",
                      "default": "Untitled"
                    },
                    "locked": {
                      "type": "boolean",
                      "description": "Indicates whether the whiteboard is locked or not.\n\n`true`: The whiteboard is locked.   \n \n\n`false`: The whiteboard is unlocked.",
                      "example": false,
                      "default": false
                    },
                    "trashed": {
                      "type": "boolean",
                      "description": "Indicates whether the whiteboard is moved to trash or not.\n\n`true`: The whiteboard is moved to trash.   \n \n\n`false`: The whiteboard was not moved to trash.",
                      "example": false,
                      "default": false
                    },
                    "readonly": {
                      "type": "boolean",
                      "description": "Indicates whether the whiteboard is editable or not. If a whiteboard is locked, this field will set `true`. \n\n`true`: The whiteboard is editable.   \n \n\n`false`: The whiteboard is read only.",
                      "example": false,
                      "default": false
                    },
                    "non_cloud": {
                      "type": "boolean",
                      "description": "Indicates whether the whiteboard is allowed to be saved to cloud or not.\n\n`true`: The whiteboard is a saved to cloud.   \n \n\n`false`: The whiteboard was not saved to cloud.",
                      "example": false,
                      "default": false
                    },
                    "created_date": {
                      "type": "string",
                      "description": "The whiteboard create date-time in UTC/GMT. Example: &quot;`2020-03-31T12:02:00Z`&quot;",
                      "format": "date-time",
                      "example": "2022-03-25T07:29:29Z"
                    },
                    "modified_date": {
                      "type": "string",
                      "description": "The whiteboard update date-time in UTC/GMT. Example: &quot;2020-03-31T12:02:00Z&quot;",
                      "format": "date-time",
                      "example": "2022-03-25T07:29:29Z"
                    },
                    "share_link_setting": {
                      "type": "object",
                      "properties": {
                        "share_scope": {
                          "type": "integer",
                          "description": "The access type of whiteboard share link. \n* Enum: \n* `0` - `Restricted`, Only people added can open with this link \n* `1` - `Same account` \n* `2` - Anyone \n* `3` - SAME_ACCOUNT_EXTERNAL_AUTH \n",
                          "example": 0,
                          "default": 0,
                          "enum": [
                            0,
                            1,
                            2,
                            3
                          ],
                          "x-enum-descriptions": [
                            "Restricted. Only people added can open with this link",
                            "Same account",
                            "Anyone",
                            "Same account external auth"
                          ]
                        },
                        "share_role": {
                          "type": "integer",
                          "description": "The access level of whiteboard share link. This field need to used with `share_scope`. If `share_scope` set `0` - `Restricted`, this field is null. If `share_scope` set `1` - `Same account` or `2` - `Anyone`, this filed can be `2` - `Editor`, `3` - `Commenter` or `4` - `Viewer`. \n* Enum: \n* `2` - `Editor`\n* `3` - `Commenter` \n* `4` - Viewer \n",
                          "example": 2,
                          "enum": [
                            2,
                            3,
                            4
                          ],
                          "x-enum-descriptions": [
                            "Editor",
                            "Commenter",
                            "Viewer"
                          ]
                        }
                      },
                      "description": "The access level, access type of current whiteboard share link."
                    },
                    "advanced_share_setting": {
                      "type": "object",
                      "properties": {
                        "can_share": {
                          "type": "boolean",
                          "description": "Indicate whether users can share at or below their permission level or not. \n\n`true`: Users can share at or below their permission level.   \n \n\n`false`: Users cannot share at or below their permission level.",
                          "example": true
                        },
                        "share_to_external": {
                          "type": "boolean",
                          "description": "Indicate whether internal users can share with external users or not. \n\n`true`: Internal users can share with external users.   \n \n\n`false`: Internal users cannot share with external users.",
                          "example": true
                        },
                        "external_can_share": {
                          "type": "boolean",
                          "description": "Indicate whether external user can share the whiteboard to others or not. \n\n`true`: External user can share the whiteboard to others.   \n \n\n`false`:External user cannot share the whiteboard to others.",
                          "example": true
                        }
                      },
                      "description": "Whiteboard advanced share setting."
                    }
                  },
                  "description": "The whiteboard object."
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
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
            "whiteboard:write"
          ],
          "x-granular-scopes": [
            "whiteboard:write:whiteboard"
          ]
        }
      }
    },
    "/whiteboards/{whiteboardId}": {
      "get": {
        "tags": [
          "Document"
        ],
        "summary": "Get a whiteboard",
        "description": "Returns a whiteboard's information.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:admin`,`whiteboard:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:whiteboard:admin`,`whiteboard:read:whiteboard`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "GetAWhiteboard",
        "parameters": [
          {
            "name": "whiteboardId",
            "in": "path",
            "description": "The whiteboard's ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "HY8-AC_HSou4SXY1S-OmOg"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "**HTTP Status Code:** `200`   \n \nWhiteboard object returned.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "whiteboard_id": {
                      "type": "string",
                      "description": "The whiteboard ID.",
                      "example": "48Z4ppMfQJWONeVWFCcVzA"
                    },
                    "whiteboard_name": {
                      "type": "string",
                      "description": "The whiteboard name.",
                      "example": "Q3 Plan",
                      "default": "Untitled"
                    },
                    "locked": {
                      "type": "boolean",
                      "description": "Indicates whether the whiteboard is locked or not.\n\n`true`-  The whiteboard is locked.   \n \n\n`false` - The whiteboard is unlocked.",
                      "example": false,
                      "default": false
                    },
                    "trashed": {
                      "type": "boolean",
                      "description": "Indicates whether the whiteboard is moved to trash or not.\n\n`true` - The whiteboard is moved to trash.   \n \n\n`false` - The whiteboard isn't moved to trash.",
                      "example": false,
                      "default": false
                    },
                    "readonly": {
                      "type": "boolean",
                      "description": "Indicates whether the whiteboard is editable or not. If a whiteboard is locked, this field will be set to `true`. \n\n`true` - The whiteboard is editable.   \n \n\n`false` - The whiteboard is read only.",
                      "example": false,
                      "default": false
                    },
                    "non_cloud": {
                      "type": "boolean",
                      "description": "Indicates whether the whiteboard is allowed to be saved to cloud or not.\n\n`true` - The whiteboard is a saved to cloud.   \n \n\n`false` - The whiteboard isn't saved to cloud.",
                      "example": false,
                      "default": false
                    },
                    "created_date": {
                      "type": "string",
                      "description": "The whiteboard creation date and time in UTC/GMT. Example: `2020-03-31T12:02:00Z`",
                      "format": "date-time",
                      "example": "2022-03-25T07:29:29Z"
                    },
                    "modified_date": {
                      "type": "string",
                      "description": "The whiteboard update date and time in UTC/GMT. Example: `2020-03-31T12:02:00Z`",
                      "format": "date-time",
                      "example": "2022-03-25T07:29:29Z"
                    },
                    "share_link_setting": {
                      "type": "object",
                      "properties": {
                        "share_scope": {
                          "type": "integer",
                          "description": "The access type of whiteboard sharing link. \n* Enum: \n* `0` - `Restricted`. Only people added can open with this link. \n* `1` - `Same account` \n* `2` - Anyone \n* `3` - `SAME_ACCOUNT_EXTERNAL_AUTH` \n",
                          "example": 0,
                          "default": 0,
                          "enum": [
                            0,
                            1,
                            2,
                            3
                          ],
                          "x-enum-descriptions": [
                            "Restricted. Only people added can open with this link",
                            "Same account",
                            "Anyone",
                            "Same account external auth"
                          ]
                        },
                        "share_role": {
                          "type": "integer",
                          "description": "The access level of whiteboard share link. This field need to used with `share_scope`. If `share_scope` set `0` - `Restricted`. This field is null. If `share_scope` set `1` - `Same account` or `2` - `Anyone`, this field can be `2` - `Editor`, `3` - `Commenter` or `4` - `Viewer`. \n* Enum: \n* `2` - `Editor`\n* `3` - `Commenter` \n* `4` - Viewer \n",
                          "example": 2,
                          "enum": [
                            2,
                            3,
                            4
                          ],
                          "x-enum-descriptions": [
                            "Editor",
                            "Commenter",
                            "Viewer"
                          ]
                        }
                      },
                      "description": "The access level, access type of current whiteboard share link."
                    },
                    "advanced_share_setting": {
                      "type": "object",
                      "properties": {
                        "can_share": {
                          "type": "boolean",
                          "description": "Indicate whether users can share at or below their permission level or not. \n\n`true`: Users can share at or below their permission level.   \n \n\n`false`: Users cannot share at or below their permission level.",
                          "example": true
                        },
                        "share_to_external": {
                          "type": "boolean",
                          "description": "Indicate whether internal users can share with external users or not. \n\n`true`: Internal users can share with external users.   \n \n\n`false`: Internal users cannot share with external users.",
                          "example": true
                        },
                        "external_can_share": {
                          "type": "boolean",
                          "description": "Indicate whether external user can share the whiteboard to others or not. \n\n`true`: External user can share the whiteboard to others.   \n \n\n`false`:External user cannot share the whiteboard to others.",
                          "example": true
                        }
                      },
                      "description": "The whiteboard advanced share setting."
                    },
                    "share_link": {
                      "type": "string",
                      "description": "The whiteboard share link.",
                      "example": "https://zoom.us/wb/doc/T-E0zR9NQWSDuvXd-Crt9A"
                    },
                    "preview_link": {
                      "type": "string",
                      "description": "The whiteboard preview link. This link's validity period is `2 hours`, and the maximum number of downloads is `99 times`. Only those who have the `whiteboard:read:whiteboard` scope or are collaborators of the whiteboard will see this field returned.",
                      "example": "https://a01file.zoom.us/file/xxxxx/attach/preview/192x108?jwt=xxx"
                    },
                    "created_by_display_name": {
                      "type": "string",
                      "description": "The creator display name of the whiteboard.",
                      "example": "Tom"
                    },
                    "modified_by_display_name": {
                      "type": "string",
                      "description": "The latest modify user display name of the whiteboard.",
                      "example": "Jack"
                    },
                    "embedded_content_types": {
                      "type": "array",
                      "description": "The embedded content type list.",
                      "items": {
                        "maximum": 2,
                        "minimum": 0,
                        "type": "integer",
                        "description": "The type of embeded content in whiteboard. it's an enum contains `0`, `1`, `2`.\nThe description of the enums:\n- **0** - **IMAGE**.\n- **1** - **DOCUMENT**, this would include PDFs, PPTX, DOCX, GOOGLE DOCS, SHEETS and SLIDES.\n- **2** - **CARD**, this would include Jira, Asana and Azure DevOps.",
                        "format": "int32",
                        "example": 0,
                        "default": 0,
                        "enum": [
                          0,
                          1,
                          2
                        ],
                        "x-enum-descriptions": [
                          "IMAGE",
                          "DOCUMENT",
                          "CARD"
                        ]
                      }
                    },
                    "project_id": {
                      "type": "string",
                      "description": "qiMWddmtRxqwSFrK22lJPg",
                      "example": "The Project ID"
                    }
                  },
                  "description": "Whiteboard object"
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n Permission is needed to access this document. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Read"
          ],
          "x-macro-scopes": [
            "whiteboard:read:admin",
            "whiteboard:read"
          ],
          "x-granular-scopes": [
            "whiteboard:read:whiteboard:admin",
            "whiteboard:read:whiteboard"
          ]
        }
      },
      "put": {
        "tags": [
          "Document"
        ],
        "summary": "Update whiteboard basic information",
        "description": "Use this API to rename a whiteboard, update whiteboard status (locked or unlocked). \n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:update:whiteboard:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "UpdateAWhiteboardMetadata",
        "parameters": [
          {
            "name": "whiteboardId",
            "in": "path",
            "description": "The ID of the whiteboard.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "HY8-AC_HSou4SXY1S-OmOg"
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
                    "description": "The whiteboard name.",
                    "example": "Q3 Plan",
                    "default": "Untitled"
                  },
                  "locked": {
                    "type": "boolean",
                    "description": "Indicates whether the whiteboard is locked or not.\n\n`true`: The whiteboard is locked.   \n \n\n`false`: The whiteboard is unlocked.",
                    "example": false,
                    "default": false
                  }
                },
                "description": "Update a whiteboard request body"
              }
            }
          }
        },
        "responses": {
          "204": {
            "description": "**HTTP Status Code**: `204`   \n \nWhiteboard updated successfully."
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n Permission is needed to access this document. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:update:whiteboard:admin"
          ]
        }
      },
      "delete": {
        "tags": [
          "Document"
        ],
        "summary": "Delete a whiteboard",
        "description": "Deletes a whiteboard.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:admin`,`whiteboard:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:delete:whiteboard:admin`,`whiteboard:delete:whiteboard`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "DeleteAWhiteboard",
        "parameters": [
          {
            "name": "whiteboardId",
            "in": "path",
            "description": "The ID of the whiteboard.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "HY8-AC_HSou4SXY1S-OmOg"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "**HTTP Status Code**: `204`   \n \nWhiteboard Deleted."
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this document. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard:write:admin",
            "whiteboard:write"
          ],
          "x-granular-scopes": [
            "whiteboard:delete:whiteboard:admin",
            "whiteboard:delete:whiteboard"
          ]
        }
      }
    },
    "/whiteboards/export": {
      "post": {
        "tags": [
          "Export"
        ],
        "summary": "Create whiteboard export",
        "description": " Generates the export for the provided whiteboards. This process will generate a PDF of whiteboard projects along with an audit log, bundling them into a ZIP file.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_export:write:admin`,`whiteboard_export:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:export`,`whiteboard:write:export:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "Createwhiteboardsexport",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "whiteboard_ids"
                ],
                "type": "object",
                "properties": {
                  "whiteboard_ids": {
                    "maxItems": 50,
                    "minItems": 1,
                    "uniqueItems": true,
                    "type": "array",
                    "description": "The list of whiteboard IDs. The list size must be **less than** 50, if more than 50, the API will **intercept** the first 50 IDs.",
                    "items": {
                      "type": "string",
                      "description": "The ID of the whiteboard.",
                      "example": "HY8-AC_HSou4SXY1S-OmOg"
                    }
                  }
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "description": "**HTTP Status Code:** `200`    Successfully sent a request to generate whiteboard export",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "task_id"
                  ],
                  "type": "object",
                  "properties": {
                    "task_id": {
                      "type": "string",
                      "description": "The unique identifier of the generated file, which expires is **30 days**. Use this task_id to query the progress of file generation (For more details, please refer to\uff1a[**Get whiteboard export generation status**](https://developers.zoom.us/docs/zoom-whiteboard/apis/#operation/Getwhiteboardexportdatagenerationstatus). Once the file is ready, you can download it using the user's OAuth token. For example: `https://{base_domain}/v2/whiteboards/export/task/{taskId} --header 'Authorization: Bearer {{OAuth-access-token}}` .",
                      "example": "f7f3cdd37bf74550afe7cdf79128a574"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this document. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `34002` <br>\n The whiteboard {whiteboard_id} not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardContent:Read",
            "WhiteboardManagement:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_export:write:admin",
            "whiteboard_export:write"
          ],
          "x-granular-scopes": [
            "whiteboard:write:export",
            "whiteboard:write:export:admin"
          ]
        }
      }
    },
    "/whiteboards/export/task/{taskId}": {
      "get": {
        "tags": [
          "Export"
        ],
        "summary": "Download whiteboard export",
        "description": "Use this API to download the whiteboard export.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_content:read:admin`,`whiteboard_content:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:export`,`whiteboard:read:export:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "Downloadwhiteboardexport",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the whiteboards file. The `task_id` generated by [**Create whiteboard export**](https://developers.zoom.us/docs/zoom-whiteboard/apis/#operation/Createwhiteboardsexport) interface.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "f7f3cdd37bf74550afe7cdf79128a574"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Returns a file stream, with the file type of ZIP."
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `34002` <br>\n The task {task_id} not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardContent:Read",
            "WhiteboardManagement:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_content:read:admin",
            "whiteboard_content:read"
          ],
          "x-granular-scopes": [
            "whiteboard:read:export",
            "whiteboard:read:export:admin"
          ]
        }
      }
    },
    "/whiteboards/export/task/{taskId}/status": {
      "get": {
        "tags": [
          "Export"
        ],
        "summary": "Get whiteboard export generation status",
        "description": "Use this API to query the status of the whiteboard export generation. Before using this API, confirm that you have invoked the interface for generating the whiteboard file  [**Create whiteboard export**](https://developers.zoom.us/docs/zoom-whiteboard/apis/#operation/Createwhiteboardsexport) and obtained the `task_id`.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_content:read:admin`,`whiteboard_content:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:export`,`whiteboard:read:export:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "Getwhiteboardexportdatagenerationstatus",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the whiteboard export. ",
            "required": true,
            "schema": {
              "type": "string",
              "example": "adcc655d3a3549e692959737fe2a7a0d"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Whiteboard export generation status.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "status",
                    "total"
                  ],
                  "type": "object",
                  "properties": {
                    "processed": {
                      "maxItems": 50,
                      "minItems": 0,
                      "type": "array",
                      "description": "List of processed whiteboard IDs.",
                      "items": {
                        "type": "string",
                        "description": "The whiteboard ID.",
                        "example": "48Z4ppMfQJWONeVWFCcVzA"
                      }
                    },
                    "status": {
                      "type": "string",
                      "description": "Whiteboard export generation status. \n* Enum:\n* `processing` - Generating whiteboard export\n* `successed` - Whiteboard export generated successfully(as long as one is successful, it counts as a success)\n* `failed` - Whiteboard export generation failed",
                      "example": "processing",
                      "enum": [
                        "processing",
                        "successed",
                        "failed"
                      ],
                      "x-enum-descriptions": [
                        "Generating whiteboard file",
                        "Whiteboard file generated successfully.",
                        "Whiteboard file generation failed"
                      ]
                    },
                    "total": {
                      "type": "integer",
                      "description": "Total number of whiteboards requiring export generation.",
                      "example": 3
                    },
                    "failed": {
                      "maxItems": 50,
                      "minItems": 0,
                      "type": "array",
                      "description": "List of failed whiteboard IDs.",
                      "items": {
                        "type": "string",
                        "description": "The whiteboard ID.",
                        "example": "UyKtcVj5REWMFT2qd-4qfg"
                      }
                    },
                    "processing": {
                      "maxItems": 50,
                      "minItems": 0,
                      "type": "array",
                      "description": "List of whiteboards being processed.",
                      "items": {
                        "type": "string",
                        "description": "The whiteboard ID.",
                        "example": "r2Qr1q4hSxu03MG3hk03uA"
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
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `34002` <br>\n The task {task_id} not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardContent:Read",
            "WhiteboardManagement:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_content:read:admin",
            "whiteboard_content:read"
          ],
          "x-granular-scopes": [
            "whiteboard:read:export",
            "whiteboard:read:export:admin"
          ]
        }
      }
    },
    "/whiteboards/files": {
      "post": {
        "tags": [
          "File"
        ],
        "summary": "Upload file for whiteboard import",
        "description": "Uploads a file to whiteboard.\n**Notes**:\n- Base URL: `https://fileapi.zoom.us/v2`.\n- Rate limit: 20 requests per second per user or 2000 requests per second per IP address.\n- The caller must support HTTP 30x redirects.\n- The caller must retain the Authorization header when redirected to a different hostname.\n- Supported file formats: `.pdf`, `.vsdx`.\n- For an account-level [OAuth](https://developers.zoom.us/docs/integrations/create/) app, this API can only be used on behalf of a user who is assigned with a [role](https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0064983#h_80abad9b-86a8-492c-8a3b-5b0141a8c0a9) with **Edit** permission for Whiteboard Management.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_file:write:admin`,`whiteboard_file:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:file`,`whiteboard:write:file:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `HEAVY`\n\n**Not supported in Gov cluster** ",
        "operationId": "Uploadfileforwhiteboardimport",
        "requestBody": {
          "content": {
            "multipart/form-data": {
              "schema": {
                "required": [
                  "file"
                ],
                "type": "object",
                "properties": {
                  "file": {
                    "type": "string",
                    "description": "The upload file is in binary format. You can only send a maximum of 100 MB.",
                    "format": "binary",
                    "example": "Vm0wd2QyUXlVWGxWV0d4V1YwZDRWMVl3WkRSV01WbDNXa1JTVjAxV2JETlhhMUpUVmpBeFYySkVUbGhoTVVwVVZtcEJlRll5U2tWVWJHaG9UVlZ3VlZadGNFSmxSbGw1VTJ0V1ZXSkhhRzlVVmxaM1ZsWmFkR05GU214U2JHdzFWVEowVjFaWFNraGhSemxWVm14YU0xWnNXbUZrUjA1R1UyMTRVMkpIZHpGV1ZFb3dWakZhV0ZOcmFHaFNlbXhXVm0xNFlVMHhXbk5YYlVaclVqQTFSMVV5TVRSVk1rcElaSHBHVjFaRmIzZFdha1poVjBaT2NtRkhhRk5sYlhoWFZtMHhORmxWTUhoWGJrNVlZbFZhY2xWcVFURlNNVlY1VFZSU1ZrMXJjRWxhU0hCSFZqRmFSbUl6WkZkaGExcG9WakJhVDJOdFJraGhSazVzWWxob1dGWnRNSGhPUm14V1RVaG9XR0pyTlZsWmJGWmhZMVphZEdSSFJrNVNiRm93V2xWYVQxWlhTbFpqUldSYVRVWmFNMVpxU2t0V1ZrcFpXa1p3VjFKV2NIbFdWRUpoVkRKT2MyTkZhR3BTYkVwVVZteG9RMWRzV1hoWGJFNVRUV3hHTlZWdGRHdGhiRXAwVld4c1dtSkdXbWhaTW5oWFl6RldjbHBHWkdsU2JrSmFWMnhXWVZReFdsaFRiRnBZVmtWd1YxbHJXa3RUUmxweFUydGFiRlpzV2xwWGExcDNZa2RGZWxGcmJGaFhTRUpJVmtSS1UxWXhXblZWYldoVFlYcFdlbGRYZUc5aU1XUkhWMjVTVGxOSFVuTlZha0p6VGtaVmVXUkhkRmhTTUhCSlZsZDRjMWR0U2tkWGJXaGFUVlp3YUZwRlpGTlRSa3B5VGxaT2FWSnRPVE5XTW5oWFlqSkZlRmRZWkU1V1ZscFVXVlJHZDFZeGJISlhhM1JVVW14d2VGVnRNVWRWTWtwV1lrUmFXR0V4Y0hKWlZXUkdaVWRPU0U5V1pHaGhNSEJ2Vm10U1MxUXlVa2RUYmtwb1VqSm9WRmxZY0ZkbGJHUllaVWM1YVUxWFVraFdNalZUVkd4a1NGVnNXbFZXYkhCSVZGUkdVMVp0UmtoUFYyaHBVbGhDTmxkVVFtRmpNV1IwVTJ0a1dHSlhhR0ZVVmxwM1lVWndSbHBHVGxSU2EzQjVWR3hhVDJGV1NuUlBWRTVYVFc1b1dGbHFTa1psUm1SWldrVTFWMVpzY0ZWWFZsSkhaREZaZUdKSVNsaGhNMUpWVlcxNGQyVkdWblJOVldSV1RXdHdWMWxyVW1GWFIwVjRZMGhLV2xaWFVrZGFWV1JQVTBVNVYxcEhhR2hOU0VKMlZtMTBVMU14VVhsVmEyUlVZbXR3YjFWcVNtOVdSbXhaWTBaa2JHSkhVbGxhVldNMVlWVXhXRlZyYUZkTmFsWlVWa2Q0VDFOSFJrZFJiRnBwVmtWVmQxWnRjRWRWTVZwMFVtdG9VRlp0YUZSVVZXaERUbFphU0dWSFJtcE5WMUl3VlRKMGExZEhTbGhoUjBaVlZucFdkbFl3V25KbFJtUnlXa1prVjJFelFqWldhMlI2VFZaWmQwMVdXbWxsYTFwWVdXeG9RMVJHVW5KWGJFcHNVbTFTZWxsVldsTmhWa3AxVVd4d1YySllVbGhhUkVaYVpVZEtTVk5zYUdoTk1VcFdWbGN4TkdReVZrZFdibEpPVmxkU1YxUlhkSGRXTVd4eVZXMUdXRkl3VmpSWk1HaExWMnhhV0ZWclpHRldWMUpRVlRCVk5WWXhjRWhoUjJoT1UwVktNbFp0TVRCVk1VMTRWVmhzVm1FeVVsVlpiWFIzWWpGV2NWTnRPVmRTYlhoYVdUQmFhMkpIU2toVmJHeGhWbGROTVZsV1ZYaFhSbFp5WVVaa1RtRnNXbFZXYTJRMFlURk9SMVp1VGxoaVJscFlXV3RvUTFOV1drZFZhMlJXVFZad01GVnRkRzlWUmxsNVlVaENWbUpIYUVSVWJYaHJWbFpHZEZKdGNFNVdNVWwzVmxSS01HRXhaRWhUYkdob1VqQmFWbFp1Y0Zka2JGbDNWMjVLYkZKdFVubFhhMXByVmpKRmVsRnFXbGRoTWxJMlZGWmFhMUl4VG5OWGJYQlRWak5vV1ZkWGVHOVJNVkpIVlc1S1dHSkZjSE5WYlRGVFpXeHNWbGRzVG1oU1ZFWjZWVEkxYjFZeFdqWlJhbEpWWVRKU1NGVnFSbUZYVm5CSVlVWk9WMVpHV2xkV2JHTjRUa2RSZVZaclpGZGliRXBQVm14a1UxWXhVbGhrU0dSWFRWZDRlVlpYTVVkWFJrbDNWbXBTV2sxSGFFeFdNbmhoVjBaV2NscEhSbGRXTVVwUlZsUkNWazVXV1hoalJXaG9VakpvVDFVd1ZrdE5iRnAwVFZSQ1ZrMVZNVFJXVm1oelZtMUZlVlZzVmxwaVdGSXpXV3BHVjJOV1RuUlBWbVJUWWxob1lWZFVRbUZoTWtwSVUydG9WbUpIZUdoV2JHUk9UVlpzVjFaWWFGaFNiRnA1V1ZWYWExUnRSbk5YYkZaWFlUSlJNRlpFUms5VFJrcHlXa1pLYVZKdVFuZFdiWFJYVm0xUmVGZHVVbXBTVjFKWFZGWmFkMDFHVm5Sa1J6bFdVbXh3TUZsVldsTldWbHBZWVVWU1ZXSkdjR2hWTUdSWFUwWktkR05GTlZkTlZXd3pWbXhTUzAxSFJYaGFSV2hVWWtkb2IxVnFRbUZXYkZwMVkwWmthMkpHYkROV01qVkxZa1pLZEZWdWJGaGhNWEJ5Vm1wS1JtVnNSbkZYYkdSb1RXeEpNbFpHV21GaE1VbDRWRzVPV0dKWVFsUldhazV2VjFaa1dHVkhPVkpOVmtwSVZsYzFTMWRIU2taalNFNVdZbFJHVkZwWGVITldiR1J6Vkcxb1UxWkZXalpXVkVreFlqRlplRmRxV2xOV1JVcG9WV3RXWVdOc1ZuRlRhM1JVVm14S01GbFZXazloUjFaelYyeEdWMkV4Y0doWFZtUlNaVlphY2xwR1pGaFNNMmg1VmxkMFYxTXhaRWRWYkdSWVltMVNjMVp0TVRCTk1WbDVUbGQwV0ZKcmJETldiWEJUVjJzeFIxTnNRbGRoYTNCSVdUSjRhMk50VmtkYVIyaG9UVEJLVWxac1VrZGhNVTE0VTFob2FsSlhVbWhWYlhNeFYwWlpkMVpyZEU1aVJuQXdWRlpTUTFack1WWk5WRkpYVm0xb2VsWnNXbXRUUjFaSFYyeHdWMUpXYjNwWFYzQkhWakpPVjFWdVNsVmlSMUpVV1d4b2IxbFdaRlZSYlVab1RWZFNTRll5TlU5aGJFcDBaVWRHVlZaV2NGZFVWbHB6VmpGYVdXRkhjR2xXV0VKSFZteGtOR0V4VW5SU2JrNVlZa1phV0ZsVVNsSk5SbXhWVW1zNVUwMVdjREZXUjNodllWWktjMk5HYkZoV00xSjJWVlJHYTFZeFpISmhSM1JUVFVad1dWZFhlR0ZaVms1WFZtNVNhMUo2YkZkVmJYaHpUbFpzVm1GRlRsZGlWWEJKV1ZWV1UxWlhTbGxoU0VwWFlrWndTRnBGV2t0a1IwNUdUbFprVGxaWGQzcFdiWGhUVXpBeFNGSllhR0ZTVjJoVldXdGtiMkl4Vm5GUmJVWlhZa1p3TVZrd1dtdGhNa3BIWWtST1YwMXFWa3haYTFwTFpFWldkV0pHYUdoTldFSjVWbTF3UzFKdFZuTlNia1pZWWtkU2IxUlhlRXBOYkZwSFYyMUdXR0pXUmpOVVZscHJWMGRLV0dWSVRscFdSVm96VmpGYWExZEhWa2hQVmtwT1ZsUkZNRlpzWkRSaE1XeFhVMnhrYWxORk5WZFpiRkpIVmtaWmVXVkhkR3BpUm5CV1ZXMTRhMVJzV25WUmFscFlWa1ZLYUZacVJtdFNNV1JaWTBaYWFXRXpRbGxXYlhSWFdWZFdjMWR1UmxOaVIxSnhWRlprVTJWc2JGWmFTRTVYVFZad01WVlhlRWRXTWtWNFkwZG9XRlp0VWxOYVZscGhZMnh3UjFwR2FGTk5NbWcxVm14a01GWXhUWGhhUldSV1lrZFNXVmx0TVZOak1WcDBUVlJDVGsxV2JEVlViR1F3VjBaS2NtSkVUbGRpV0VKVVZqSnplRk5IUmtabFJtUk9ZbTFvYjFacVFtRldNazV6WTBWb1UySkhVbGhVVmxaM1ZXeGFjMXBJWkZSTlZURTBWVEZvYzFVeVJYbGhSemxXWWtaS1dGWXdXbk5rUjFKR1pFVTFhVkp1UVhkV1JscFRVVEZhY2sxV1drNVdSa3BZVm01d1YxWkdXbkZUYTFwc1ZteGFNVlZ0ZUdGaFZrbDRVbGhrVjJKVVJUQlpla3BPWlVkT1JtRkdRbGRpVmtwVlYxZDBWMlF4WkhOWGEyaHNVak5DVUZadGVITk9SbGw1VGxVNWFHSkZjRmxaVlZwelZqSkZlRlpZYUdGU00yaDZWbXBHWVZkWFJrZGhSazVwVW0wNU5GWXhVa05aVjBWNFZXNU9XRmRIZUc5VmExWjNWMFpTVjFkdVpHaFNiRmt5VlcxMGQySkdTbk5UYWtaWFlsaG9URmxXV2t0ak1rNUhZa1prVTJKRmNFbFdXSEJDVFZkTmVGcElTbWhTTTJoVVZGVmFkMkZHV25SalJXUlVUVlZzTkZaWE5VOVhSMHBXVjJ4a1ZtSllhRE5VVlZwaFYwVTFWbFJzWkU1aE0wSktWMVpXVjFVeFduSk5WbVJxVWpKb1lWUldXbmRWUm10NFYyeGthazFYVW5sVWJGcHJWR3hhV0dRemNGZGlXR2hVVlhwQmVGTkdTbGxoUjBaVFZqSm9WbGRYZEd0aU1rbDRWbTVHVW1KVldsaFphMXAzVFZacmQxZHRkR2hOYTNCSVdXdFNUMVl3TVhGV2EzaGFZVEZ3VEZwRldsZGtWMHBIWVVkb1RsZEZTalZXYlRGM1V6RktkRlp1VGxOaWExcFpXV3RrVTJJeFVsaGxSWEJPWWtad1NGWXlNVWRYUjBwWFVtcE9WVlpzY0hKV01HUkxWMGRXU1ZSc2NGZFNWbTk2Vm1wR1lXRXhaRWhXYTJoUVZtdHdUMVp0ZEhkVFZscHpXWHBHVkUxWFVrbFZNblJoWVd4T1JrNVdaRnBpUmtwSVZtdGFkMWRIVmtsVWJHUnBVakZLTmxaclkzaGlNVmw1VWxod1VsZEhhRmhXYlRGT1pVWnNjVkpzY0d4U2JWSmFXVEJrYjFaR1NsbFJiR3hYWWxoU1dGZFdaRmRqTVdSMVVteE9hVkl4U25oV1JscHJWVEpXYzJKR1dtRlRSVFZZVkZaYWQwMVdWbGhsUldSWFRXdFdORmt3Wkc5WFJscDBWV3hPWVZac2NHaFpNbmgzVWpGd1IyRkdUazVOYldjeFZtMTRhMDFHV1hoVVdHaGhVbGRTVjFsclpGTlhWbXgwVFZaT2FrMVdjREJVVmxKRFZHc3hWMkpFVmxWaVIxRjNWakJhUzJOdFNrVlViR1JwVjBWS1ZWWnFTbnBsUms1SVVtdGFiRkp0VWs5WmJURnZZakZhY1ZGdFJsZE5helY2V1RCV2IxVXlTa2hWYkdoVlZteGFNMVpYZUdGak1YQkpXa1pTVGxaVVJUQldWRVp2WXpGVmVWSlliR2hTZW14WFdXdGFTMWRHV1hkWGJrNXJVakZLUjFwRlZURmhWa3AxVVZoa1YxSnNjRlJWVkVaaFkyc3hWMWRyTlZkU2EzQlpWbTB3ZUdJeVZuTlhiazVZWWxoU1ZWVnFSbUZUUmxsNVpVaGtWMDFWY0ZoWmFrNTNWMFphYzFkdGFGZGhhM0JRVm1wR1UyUldWbk5SYkdScFZtdHdWbFl4WkRCV01sRjRXa2hPV0dFeWFITlZhazVEVlVaYWRHVklUazlTYkd3MVZHeFZOV0ZIU2taalJteGFWbFp3ZWxacVNrWmxSbHBaWVVkR1UwMHlhRFpXYlhCSFdWWmtXRkpyYUdwU01uaFlWakJXUzFOR1duUmxSM1JQVWpCV05GWlhOVTlYUm1SSVpVYzVWbUV4V2pOV01GcHpWMGRTUm1SSGNHbFNiR3Q1VmxjeE1FMUhSblJTYWxwWFlrZG9XVmxVUm5kaFJteFhWMnQwYWsxck5VaFphMXB2VmpBd2VXRkhPVmRXZWtJMFZsY3hWMUl4Y0VsV2JGcHBWMFZLVlZadGNFZFRNV1JIVjJ0a1dHSlZXbkpVVlZKSFUwWmFkRTVXVG1oTlZXOHlXV3RhYTFadFNsbGhSRTVWVmxad2VsWnRlR3RqTVZKeldrWmthVk5GU21GV01WcFhWakZWZUZkc2FGUmhNbEp4VldwS2IxWXhVbFphUms1WVVteFdNMVl5ZERCaE1VbDNUbFZrVldKR2NISldSM2hoVjBkUmVtTkdaR2xYUjJoVlZsaHdRbVZHVGtkVWJHeHBVbXMxYjFSWGVFdFdiR1JZVFZod1RsWnNjRmhaYTJoTFdWWktObUpHYUZwaE1YQXpXbGQ0V21WVk5WaGtSbFpvWld0YVdWZFVRbTlqTVZsM1RWaEdVMkV5YUdGV2FrNXZZVVpyZVdONlJsaFdNSEJJV1ZWa2IxUnNaRVpUYkVwWFRWWndhRmRXV2s1bFZsSjFWV3hXYUUxV2NGcFhWM1JyVlRKT1IxWnVSbEpXUlVwUFZXMHhVMWRzYTNkV2JYUlhUV3R3V0ZWdGNFOVdWbHB6VjI1d1dsWldjRXhaTW5NeFYwVTVWMk5IYUU1aVYyaEdWbTF3UjFsWFJYaFhXR2hxVWxaYVUxbHNhRk5qUmxwMFpFWndUbEpzY0hoVk1uQlRWakF4VjFacVZsWk5ibEp5V1ZkNFQxSnJOVmRhUm5CcFVtdHdTVlp0ZEdGVk1WbDRXa2hPV0dKWWFGaFVWVkpTWlZaYVdFMVVVbWxOVmxZMVZXeG9kMVZ0U2xaWGJHaFhZbFJHVDFwVlduTmpWa3B6WTBkNFUySldTbUZYVkVKcllqRmFSMU5zYkZKaVIxSlpWbTE0UzFSR1dsVlNhemxUVFZad01WVnRlRTloVmxwWFkwWndWMkpVUVhoVlZFWlBWakpLUjFkc2FHaGxiWGhaVmtaYVlXUXhUa2RYV0d4c1VqTlNXRlJYZEhkVFZscElZMFpPVjFZd1ZqVldWM2hQV1ZaYWMyTkhhRnBOYm1nelZXcEdkMUl5UmtkVWF6Vk9ZbGRqZUZadE1UUmhNbEY0VWxob2FWSnRhRlpaVkVwVFYwWnNkR1ZGZEdwTlZsWXpWMnRhVDJGck1WaGxTR3hYVFdwV1VGWkVSbUZrVmtaeldrWndWMVl4UmpOV2FrSmhVekpPZEZOclZsVmlSMUp3VmpCV1MxWldXbkZUYm5Cc1VtdHNORlpITlU5VmJVcElWV3M1V2xZelVtaFZNRnByWXpGa2RGSnNaRTVoZWxZMlYxUkNWMkl4VlhsVGEyaFdZWHBzVmxadWNFZGtiRlY1WlVad2JGSnNXbmxaTUdRMFZUSkdObEpVUWxkV1JWcDJXV3BLUjJNeFRuTmhSMmhVVWxWd1dGZFhlRk5TTWxKelYyNUtXR0pWV25GVVYzUmhVakZhU0dWR1RsVmlSbkF4VlZab2ExWXhTbk5qUmxKV1ZrVmFhRmt5YzNoV01XUnlUbFprVTJFelFscFdNV1F3WVRKSmVWVnVUbGhYUjFKWldXeG9VMVpXVm5GUmJVWlVVbTE0ZVZZeU5XdGhSbHAwVld0c1dsWlhUVEZXYWtwTFZsWktWVkZzY0d4aE0wSlJWMWh3UzFReFNuUlNhMlJTWWtkU1QxUlZWbmRXYkZsNFdrUkNhVTFWVmpOVWJGWnJWMGRLY21OSFJsVldSWEJVVmxWYVlXUkhWa2xVYXpsVFlrZDNNVlpIZUZaT1YwWklVMnRhYWxKdGVHRldiRnAzWld4YWNWTnJXbXhTYkhCYVdWVmtSMVV3TUhsaFJGcFhZbGhDU0ZkV1dtdFhSa3B5V2tkb1UyRjZWbmRXVnpBeFVXc3hWMWRZYUZoaVIxSmhWbXBDVjA1R1dsaE9WazVXVFd0d2VWUnNXbk5YYlVWNFkwZG9WMDFHY0hwV2JGcFBZekZPY2s1V1RtbFdhM0JhVm0xd1MwMUZNVWhTYmtwT1ZtMVNWVmxYZEdGWFJsWjFZMFZrYTJKR2NGWlZNblF3VlRBeGNrNVZhRnBoTVhCeVZsUktTMUp0VGtaTlZtaFhUVEJLU1ZaclVrZFhiVlpIWTBWc1ZHSlhhRlJVVkVwdlZsWmFSMVp0Um10TmExcElWakkxVTFSc1RraGhSbVJWVm14YWVsUnJXbUZYUlRWV1QxWm9VMDFJUWxoV2FrbzBZVEZhZEZOc2JHaFNSVFZXVm14YWQyVnNXblJsUjNScVRWWktlbGRyV210aFZrNUdVMnh3VjAxV1NrUlpWRVpLWlVaa2MxcEdhR2xoZWxaNFZsZHdTMkl4YkZkalJtaHJVakJhVDFSV1dtRmxiRmw1WlVkMGFFMVZiRE5VYkZaclZsZEtSMk5JU2xkU00yaG9WakJrVW1WdFRrZGFSMnhZVWpKb1ZsWnNhSGRSYlZaSFZHdGtWR0pIZUc5VmFrSmhWa1phY1ZOdE9WZGlSMUpaV2tWa01HRlZNWEppUkZKWFlsUldTRlpYTVV0V2JHUnpZVVp3YUUxWVFYcFdSbFpoWkRGYWRGSnJhR3RTYkZwdldsZDBZVmRXWkZWUmJUbHFUVlpzTTFSV2FFZFZNa1Y1WVVkR1YyRnJOWFpaVlZweVpWVXhWazlXVGxkaE1XOTNWbGN4TkZReFdYZE5WbHBZVjBoQ1dGbHNVa2ROTVZZMlVtczFiRlpzU2pGV1IzaFhZVmRGZWxGdVdsZFdla0kwVmxSR1lWSXhWbk5XYkZKcFZrZDRkMVpHV21Ga01sWlhWMnhvYTFKRlNsZFVWVkpIVjBac2NsVnNUbGROVlc4eVZtMTRiMVl5U2xWU2EzaFdZbGhvVUZsNlNrZFNNV1IwWWtaT2FFMHdTbUZXYlRGM1VqRnNWMkpHWkZSWFIyaHdWV3RhZDFaR2JITmFSRkpWVFZad2VGVnRkREJXUmxwMFZXeHNZVkpYVWtoV2FrRjRWakZrZFdOR2FHaE5WbkF5VjFaV1lWTXlUWGhVYms1WVlrZFNjRlpyVm1GV1ZscDBaRWRHYTAxWFVrbFdiWFJ6VmxkS1NGVnVRbFpoYTFwTVZHMTRZV05zYTNwaFIyeE9WbXhaTUZacVNqQlpWbVJJVW01T2FsSnRhRmhaVkVaaFpWWndWbGR1VG1wV2EzQjZXVEJrTkZVeVNsZFRiVVpZVmtWS2FGZFdaRWRXTVU1WllrZG9VMkpyU2xsWFYzaFRVbXN4UjJORlZsTmlhelZZVkZkek1WTkdXWGxsUnpsb1ZtMVNTRlV5TlhOV01rcFZVbGhrWVZKRmNGaFpla1pyWkZaV2NrNVdhRk5XUmxwYVZtdGFZVll5VFhkT1dFNXBVbXh3VjFsclpGTlhSbEpXVld0a1dGWnNjRmhaVldRd1YwWktjMk5FUWxkV00yaFFWMVphWVdNeVRraGhSbkJPWW0xbmVsWlhjRWRrTVU1SVUydG9hVkpyTlZsVmJGWnlaVVphZEUxVVVtaE5SRVpJVmpGb2IxWldaRWhsUmxaWFRVZFNkbGt3V2xka1JURlZWVzF3VjJKR2NEWldNblJoVkRKR2NrMVdaR3BTUlVwb1ZteGFXbVF4YkhKYVJYUlRUV3MxUmxWWGVGZFdNVnB5WTBac1YyRnJTbWhWTWpGU1pWWlNjbGR0YUZOaWEwcFFWbGN3TVZFd01YTlhia1pVWW01Q2MxVnRjekZUVmxaMFpFZEdhVkpyY0RCV1YzTTFWMjFLVlZKdVdscGhhMXBvV1RGYVIyUkdTbk5hUlRWb1pXeFpNbFl4VWtOV01rbDRWbGhzVkdFeWFGZFpXSEJ6Vm14YWRXTkZaR3RpUmxZMVdsVmFkMkpHU25OWGJteGFUVWRSTUZaVVNrZE9iRXBWVVd4a1YxSldjREpYVjNCTFVqSk5lRlJ1VG1oU2JIQndWbXBLYjFkR1pGaGxSemxWWWxaYVNGWXhhR3RYUjBwSlVXNUtWVlpzY0ROYVZscDNVbXhhVlZKdGFGZGhNMEY0VmxaYWIyRXhaRWRYYTFwVVlrZG9ZVlpyVm1GamJHdzJVbTEwV0ZadVFraFhhMlJ2WVZaS1dWRnNjRmROVjFGM1dWUktTbVZXVm5WVWJHaFlVakZLV2xkWGVHOVZNVnBYVm01R1VtSlZXbFZWYlhoelRsWndWbUZIZEdoU2JIQXdWbGQwYzFkc1dsaFVWRVpYWVd0d1RGWnNXa3RXVmtaelkwZDRhRTB3U2xKV01XaDNVakpSZUZSc1pGVmliRXB5Vlcxek1WUXhVbFpXYm1SWFlrWnNORmRyVWtOWGJGbDRVbXBPVldKR1duSldNR1JMVTBkR1JrOVdaR2hOVm5CTlZtdFNSMVl5VWtoV2EyeFZZWHBzVkZsclpETk5WbHBJWlVaYVQxWXdXa2xWTW5SaFlXeEtkR1ZIUmxkaVJuQXpXa2Q0WVdOV1RuSmFSM0JvWld4YVdWWnNaRFJoTVZWNFYyNVdVbUZzY0ZoWmJHaHZZVVpXY1ZKdGRGaFNhM0I1VjJ0YVYyRldTWGhUYmxwWFVteEtSRlV5YzNoak1XUjFWVzEwVTAweFNsRldWekI0VGtaa1IxZHVUbWhUUlRWWVZXcENWMDVHYTNkWGJrNVhUVVJHU1ZsVlZsTldNREZIWTBkb1dtVnJXbkphUmxwM1VqRlNjazVXWkdoTlYzUXpWbTB4ZDFNd01VZFZXR2hZWW14S1ZWbFVSbmRXVm14eVYyNWtWMkpHY0hwV01uUnJWR3N4VjFkcmFGWmlSa3A2VmxaYVYxWlZNVVZoZWpBOQ=="
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "File successfully uploaded.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "file_id": {
                      "type": "string",
                      "description": "The file's unique ID.",
                      "example": "xBvggqyjQUal6TecwMlYwE"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `124` <br>\n Invalid access token. <br>\n**Error Code:** `300` <br>\n Invalid File type.\nExceed max file size: ${maxSize} megabytes. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests  For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_file:write:admin",
            "whiteboard_file:write"
          ],
          "x-granular-scopes": [
            "whiteboard:write:file",
            "whiteboard:write:file:admin"
          ]
        }
      }
    },
    "/whiteboards/{whiteboardId}/files/{fileId}": {
      "get": {
        "tags": [
          "File"
        ],
        "summary": "Download Imported Whiteboard File",
        "description": "Download files imported into the whiteboard, including images, PDFs, DOCX, and other supported formats.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_file:read`,`whiteboard_file:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:file`,`whiteboard:read:file:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Downloadembeddedwhiteboardfile",
        "parameters": [
          {
            "name": "whiteboardId",
            "in": "path",
            "description": "The whiteboard ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "YXw46hQqQCiZMlAGkQVKsA"
            }
          },
          {
            "name": "fileId",
            "in": "path",
            "description": "The ID of the file imported into the whiteboard to download.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "Et2l7DAvQqu5E6jsqnnasw"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Returns a file stream."
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
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
            "WhiteboardContent:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_file:read",
            "whiteboard_file:read:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:read:file",
            "whiteboard:read:file:admin"
          ]
        }
      }
    },
    "/whiteboards/import": {
      "post": {
        "tags": [
          "Import"
        ],
        "summary": "Create a new whiteboard by import",
        "description": "Creates a new whiteboard for a user by converting Miro, Mural, or Visio files.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_import:write`,`whiteboard_import:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:import`,`whiteboard:write:import:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `HEAVY`\n\n**Not supported in Gov cluster** ",
        "operationId": "CreateWhiteboardImport",
        "requestBody": {
          "description": "Creates a new whiteboard for a user by converting Miro, Mural, or Visio files request form-data.",
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "file_id",
                  "type"
                ],
                "type": "object",
                "properties": {
                  "file_id": {
                    "type": "string",
                    "description": "The ID of the file to convert to a whiteboard. Use the **Upload File for Whiteboard Import** API (POST `/whiteboards/files`) to upload the file and get the `file_id`.\nSupported file types:\n- **Visio**: `.vsdx`\n- **Miro** and **Mural**: `.pdf`\n\nMaximum file size: **100 MB**.\nEnsure the file is valid, within the size limit, and in one of the supported formats.",
                    "example": "xBvggqyjQUal6TecwMlYwE"
                  },
                  "type": {
                    "type": "number",
                    "description": "The import file original type. \n\nEnum:  \n* `1` - Miro \n* `2` - Visio \n* `3` - Mural ",
                    "example": 1
                  },
                  "whiteboard_name": {
                    "type": "string",
                    "description": "The name of the whiteboard created by import.",
                    "example": "Q1 plan"
                  },
                  "skip_notifications": {
                    "type": "boolean",
                    "description": " **Default is false (notifications will be sent)**.\n If set to `true`, collaborators and owners will not receive direct messages or emails when the whiteboard is imported.",
                    "example": false
                  },
                  "owner_email": {
                    "type": "string",
                    "description": "The owner email. This field is only available if you have the `whiteboard:write:import:admin` scope. Otherwise, it will automatically be set to `null` and the document owner will **default to the user who imported the file**.",
                    "format": "email",
                    "example": "example@zoom.us"
                  },
                  "collaborators": {
                    "maxItems": 100,
                    "minItems": 0,
                    "type": "array",
                    "description": "Users or team chat channels to add to a whiteboard.",
                    "items": {
                      "type": "object",
                      "properties": {
                        "email": {
                          "type": "string",
                          "description": "The `user email` address to share with.",
                          "format": "email",
                          "example": "user@example.com"
                        },
                        "role": {
                          "type": "string",
                          "description": "Collaborator role type. \n\nEnum:  \n- `0` - OWNER \n- `1` - CO_OWNER \n- `2` - EDITOR \n- `3` - COMMENTER \n- `4` - VIEWER",
                          "example": "2",
                          "enum": [
                            "0",
                            "1",
                            "2",
                            "3",
                            "4"
                          ]
                        },
                        "channel_id": {
                          "type": "string",
                          "description": "The team chat `channel ID` to share with.",
                          "example": "cabc1234567defghijkl01234"
                        }
                      },
                      "description": "Array of collaborator objects."
                    }
                  },
                  "project_id": {
                    "type": "string",
                    "description": "qiMWddmtRxqwSFrK22lJPg",
                    "example": "The Project ID"
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "The imported file is being converted into a whiteboard.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "task_id": {
                      "type": "string",
                      "description": "Use this ID to check the import status via the [Get Whiteboard Import Status](https://developers.zoom.us/docs/api/whiteboard/#tag/import/GET/whiteboards/import/task/{taskId}/status) endpoint (GET `/whiteboards/import/{taskId}/status`). The unique ID for your import.",
                      "example": "ce31de9e5a5f440bab9d60c144fd76b7"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid file type. <br>\n**Error Code:** `38003` <br>\n File size exceed limit. <br>\n**Error Code:** `38004` <br>\n Conversion settings are not enabled. Please contact your account administrator to allow users to import and convert boards and diagrams from other applications into editable whiteboards via the whiteboard account settings. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_import:write",
            "whiteboard_import:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:write:import",
            "whiteboard:write:import:admin"
          ]
        }
      }
    },
    "/whiteboards/import/{taskId}/status": {
      "get": {
        "tags": [
          "Import"
        ],
        "summary": "Get whiteboard import status ",
        "description": "Use this API to query the status of new whiteboard by import. Before using this API, confirm that you have invoked the interface for import file to create new whiteboard **Create a new whiteboard by import** and obtained the `task_id`.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_import:read`,`whiteboard_import:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:import:admin`,`whiteboard:read:import`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "GetWhiteboardimportstatus",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique ID for your import.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "ce31de9e5a5f440bab9d60c144fd76b7"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Get whiteboards import result.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "file_name": {
                      "type": "string",
                      "description": "The file name of import file.",
                      "example": "miro.pdf"
                    },
                    "status": {
                      "type": "string",
                      "description": "\nThe status of the whiteboard created by importing a file.\n\n**Enum**:\n- `processing` - The file is converting to whiteboard.\n- `succeeded` - The file convert to whiteboard successfully.\n- `failed` - The file convert to whiteboard failed.\n",
                      "example": "succeeded",
                      "default": "processing",
                      "enum": [
                        "processing",
                        "failed",
                        "succeeded"
                      ]
                    },
                    "whiteboard_id": {
                      "type": "string",
                      "description": "The whiteboard ID `nullable`. This value is returned only when the `status` is `succeeded`.",
                      "example": "HY8-AC_HSou4SXY1S-OmOg"
                    },
                    "doc_name": {
                      "type": "string",
                      "description": "The whiteboard name `nullable`. This value is returned only when the `status` is `succeeded`.",
                      "example": "Q1 Plan"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `38002` <br>\n Failed to convert the file {fileName}. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `38007` <br>\n The taskId not exist or expired. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_import:read",
            "whiteboard_import:read:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:read:import:admin",
            "whiteboard:read:import"
          ]
        }
      }
    },
    "/whiteboards/projects": {
      "get": {
        "tags": [
          "Project"
        ],
        "summary": "List all projects",
        "description": "Retrieves a list of projects.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:read`,`whiteboard_project:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:list_projects:admin`,`whiteboard:read:list_projects`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\n\n**Not supported in Gov cluster** ",
        "operationId": "Listallprojects",
        "parameters": [
          {
            "name": "search_key",
            "in": "query",
            "description": "The name of a project. If this parameter is provided, only the matched project will be returned. Otherwise, all projets will be returned.",
            "required": false,
            "schema": {
              "maxLength": 50,
              "minLength": 0,
              "type": "string",
              "example": "demo "
            }
          },
          {
            "name": "user_id",
            "in": "query",
            "description": "The email address or user ID of the project owner. If this parameter is provided, only the project owned by the provided user will be returned. If set to `me`, will return the matched projects of current user.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "me"
            }
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of records returned from a single API call.",
            "required": false,
            "schema": {
              "type": "integer",
              "example": 10
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
          }
        ],
        "responses": {
          "200": {
            "description": "The list of projects is successfully retrieved.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "projects": {
                      "type": "array",
                      "description": "The project list.",
                      "items": {
                        "type": "object",
                        "properties": {
                          "collaborator": {
                            "maxItems": 100,
                            "minItems": 0,
                            "type": "array",
                            "description": "The project collaborators, this API returns only the project collaborators who have the 'owner' role, along with the current user's information.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "collaborator_id": {
                                  "type": "string",
                                  "description": "The project collaborator ID.",
                                  "example": "x4g6hy789k0o878k"
                                },
                                "collaborator_name": {
                                  "type": "string",
                                  "description": "The project collaborator name.",
                                  "example": "Jack"
                                },
                                "user_id": {
                                  "type": "string",
                                  "description": "`nullable` User ID. This response only returns if the `collaborator_type` is 0. If the `collaborator_type` is not 0, this returns null value.",
                                  "example": "wf3h7uo1dk0v53bl"
                                },
                                "avatar": {
                                  "type": "string",
                                  "description": "Collaborator profile picture.",
                                  "example": "https://content.zoom.us/swerwerw.jpeg"
                                },
                                "role": {
                                  "type": "integer",
                                  "description": "Collaborator role type.\n\nRole Enum Values:\n\n- `0` - OWNER\n- `1` - CO_OWNER\n- `2` - EDITOR\n- `3` - COMMENTER\n- `4` - VIEWER",
                                  "example": 2,
                                  "enum": [
                                    0,
                                    1,
                                    2,
                                    3,
                                    4
                                  ]
                                },
                                "collaborator_type": {
                                  "type": "integer",
                                  "description": "Collaborator type.\nEnum:\n- `0` - SAME_ACCOUNT_USER\n- `1` - EXTERNAL_USER",
                                  "example": 0,
                                  "enum": [
                                    0,
                                    1
                                  ]
                                }
                              },
                              "description": "The project collaborator information"
                            }
                          },
                          "created_date": {
                            "type": "string",
                            "description": "The project create date-time in UTC/GMT. Example: `2020-03-31T12:02:00Z`",
                            "format": "date-time",
                            "example": "2020-03-31T12:02:00Z"
                          },
                          "modified_date": {
                            "type": "string",
                            "description": "The project update date-time in UTC/GMT. Example: `2020-03-31T12:02:00Z`",
                            "format": "date-time",
                            "example": "2020-03-31T12:02:00Z"
                          },
                          "project_name": {
                            "type": "string",
                            "description": "The  project name.",
                            "example": "Untitle"
                          },
                          "project_id": {
                            "type": "string",
                            "description": "qiMWddmtRxqwSFrK22lJPg",
                            "example": "The Project ID"
                          },
                          "starred": {
                            "type": "boolean",
                            "description": "Indicates whether the project is starred.\n`true`: starred\n`false`: not starred",
                            "example": false
                          },
                          "owner_display_name": {
                            "type": "string",
                            "description": "The project owner display name.",
                            "example": "Example Name"
                          }
                        },
                        "description": "The project basic info."
                      }
                    },
                    "next_page_token": {
                      "type": "string",
                      "description": "The next page token paginates through a large set of results. A next page token is returned when the available results exceed the current page size. The expiration period for this token is 15 minutes.",
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
                      "example": 100
                    }
                  }
                }
              }
            }
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests  For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_project:read",
            "whiteboard_project:read:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:read:list_projects:admin",
            "whiteboard:read:list_projects"
          ]
        }
      },
      "post": {
        "tags": [
          "Project"
        ],
        "summary": "Create a new project",
        "description": "This API endpoint allows you to create a new project for a user, optionally adding collaborators and initializing it with existing whiteboards.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:write`,`whiteboard_project:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:project`,`whiteboard:write:project:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Createproject",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "project_name"
                ],
                "type": "object",
                "properties": {
                  "project_name": {
                    "maxLength": 50,
                    "minLength": 1,
                    "type": "string",
                    "description": "The project name.",
                    "example": "Untitle"
                  },
                  "skip_notifications": {
                    "type": "boolean",
                    "description": " **Default is false (notifications will be sent)**.\n If set to `true`, collaborators and owners will not receive direct messages or emails when the project is created.",
                    "example": false
                  },
                  "invite_message": {
                    "maxLength": 280,
                    "minLength": 0,
                    "type": "string",
                    "description": "The invite message.",
                    "example": "I'm glad to invite you to join this project."
                  },
                  "collaborators": {
                    "title": "Array of collaborator objects.",
                    "maxItems": 100,
                    "minItems": 0,
                    "type": "array",
                    "description": "Array of collaborator objects to be added to the project.",
                    "items": {
                      "type": "object",
                      "properties": {
                        "email": {
                          "type": "string",
                          "description": "The user's email to be added to the project.",
                          "example": "user@example.com"
                        },
                        "role": {
                          "type": "integer",
                          "description": "Collaborator role type.\nWhen assigning a new `OWNER` or `CO_OWNER`, the specified user must belong to the same account as the original `OWNER`. If a new owner is designated for a project, the original owner will automatically be demoted to `CO_OWNER`.\n\nRole Enum Values:\n\n- `0` - OWNER\n- `1` - CO_OWNER\n- `2` - EDITOR\n- `3` - COMMENTER\n- `4` - VIEWER",
                          "example": 2,
                          "default": 2,
                          "enum": [
                            0,
                            1,
                            2,
                            3,
                            4
                          ]
                        }
                      },
                      "description": "A user who works jointly on a project."
                    }
                  },
                  "whiteboard_ids": {
                    "title": "whiteboard_id",
                    "maxItems": 50,
                    "minItems": 0,
                    "uniqueItems": true,
                    "type": "array",
                    "description": "The whiteboard ID list that you want to add to the project.",
                    "items": {
                      "type": "string",
                      "description": "The whiteboard ID.",
                      "example": "J7XZFDEQTkWSsMsmO_M1Xa"
                    }
                  },
                  "owner_email": {
                    "type": "string",
                    "description": "The owner email. This field is only available if you have the `whiteboard_project:write:admin` scope. Otherwise, it will automatically be set to `null` and the project owner will default to the request user.",
                    "format": "email",
                    "example": "owner@example.com"
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "The project has been successfully created.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "collaborator": {
                      "maxItems": 100,
                      "minItems": 0,
                      "type": "array",
                      "description": "The project collaborators.",
                      "items": {
                        "type": "object",
                        "properties": {
                          "collaborator_id": {
                            "type": "string",
                            "description": "The project collaborator ID.",
                            "example": "x4g6hy789k0o878k"
                          },
                          "collaborator_name": {
                            "type": "string",
                            "description": "The project collaborator name.",
                            "example": "Jack"
                          },
                          "user_id": {
                            "type": "string",
                            "description": "`nullable` User ID. This response only returns if the `collaborator_type` is 0. If the `collaborator_type` is not 0, this returns null value.",
                            "example": "wf3h7uo1dk0v53bl"
                          },
                          "avatar": {
                            "type": "string",
                            "description": "Collaborator profile picture.",
                            "example": "https://content.zoom.us/swerwerw.jpeg"
                          },
                          "role": {
                            "type": "integer",
                            "description": "Collaborator role type.\n\nRole Enum Values:\n\n- `0` - OWNER\n- `1` - CO_OWNER\n- `2` - EDITOR\n- `3` - COMMENTER\n- `4` - VIEWER",
                            "example": 2,
                            "enum": [
                              0,
                              1,
                              2,
                              3,
                              4
                            ]
                          },
                          "collaborator_type": {
                            "type": "integer",
                            "description": "Collaborator type.\nEnum:\n- `0` - SAME_ACCOUNT_USER\n- `1` - EXTERNAL_USER",
                            "example": 0,
                            "enum": [
                              0,
                              1
                            ]
                          }
                        },
                        "description": "The project collaborator information."
                      }
                    },
                    "created_date": {
                      "type": "string",
                      "description": "The project creation date-time in UTC/GMT. Example: `2020-03-31T12:02:00Z`",
                      "format": "date-time",
                      "example": "2020-03-31T12:02:00Z"
                    },
                    "modified_date": {
                      "type": "string",
                      "description": "The whiteboard update date-time in UTC/GMT. Example: `2020-03-31T12:02:00Z`",
                      "format": "date-time",
                      "example": "2020-03-31T12:02:00Z"
                    },
                    "project_name": {
                      "type": "string",
                      "description": "The project name.",
                      "example": "Untitle"
                    },
                    "project_id": {
                      "type": "string",
                      "description": "qiMWddmtRxqwSFrK22lJPg",
                      "example": "The Project ID"
                    },
                    "starred": {
                      "type": "boolean",
                      "description": "Indicates whether the project is starred. `true`: starred, `false`: not starred.",
                      "example": false
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
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
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_project:write",
            "whiteboard_project:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:write:project",
            "whiteboard:write:project:admin"
          ]
        }
      }
    },
    "/whiteboards/projects/{projectId}": {
      "get": {
        "tags": [
          "Project"
        ],
        "summary": "Get a project",
        "description": "This API endpoint allows you to retrieve detailed information about a specific project.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:read`,`whiteboard_project:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:project`,`whiteboard:read:project:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Getaproject",
        "parameters": [
          {
            "name": "projectId",
            "in": "path",
            "description": "The project ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "N6XZFDEQTkWSsMsmO_M1XQ"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "The project information has been successfully retrieved.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "collaborator": {
                      "maxItems": 100,
                      "minItems": 0,
                      "type": "array",
                      "description": "The project collaborators. This API returns only the project collaborators who have the 'owner' role, along with the current user's information.",
                      "items": {
                        "type": "object",
                        "properties": {
                          "collaborator_id": {
                            "type": "string",
                            "description": "The project collaborator ID.",
                            "example": "x4g6hy789k0o878k"
                          },
                          "collaborator_name": {
                            "type": "string",
                            "description": "The project collaborator name.",
                            "example": "Jack"
                          },
                          "user_id": {
                            "type": "string",
                            "description": "`nullable` User ID. This response only returns if the `collaborator_type` is 0. If the `collaborator_type` is not 0, this returns null value.",
                            "example": "wf3h7uo1dk0v53bl"
                          },
                          "avatar": {
                            "type": "string",
                            "description": "Collaborator profile picture.",
                            "example": "https://content.zoom.us/swerwerw.jpeg"
                          },
                          "role": {
                            "type": "integer",
                            "description": "Collaborator role type.\n\nRole Enum Values:\n\n- `0` - OWNER\n- `1` - CO_OWNER\n- `2` - EDITOR\n- `3` - COMMENTER\n- `4` - VIEWER",
                            "example": 2,
                            "enum": [
                              0,
                              1,
                              2,
                              3,
                              4
                            ]
                          },
                          "collaborator_type": {
                            "type": "integer",
                            "description": "Collaborator type.\nEnum:\n- `0` - SAME_ACCOUNT_USER\n- `1` - EXTERNAL_USER",
                            "example": 0,
                            "enum": [
                              0,
                              1
                            ]
                          }
                        },
                        "description": "The project collaborator information."
                      }
                    },
                    "created_date": {
                      "type": "string",
                      "description": "The project create date-time in UTC/GMT. Example: `2020-03-31T12:02:00Z`",
                      "format": "date-time",
                      "example": "2020-03-31T12:02:00Z"
                    },
                    "modified_date": {
                      "type": "string",
                      "description": "The project update date-time in UTC/GMT. Example: `2020-03-31T12:02:00Z`",
                      "format": "date-time",
                      "example": "2020-03-31T12:02:00Z"
                    },
                    "project_name": {
                      "type": "string",
                      "description": "The  project name.",
                      "example": "Untitle"
                    },
                    "project_id": {
                      "type": "string",
                      "description": "qiMWddmtRxqwSFrK22lJPg",
                      "example": "The Project ID"
                    },
                    "starred": {
                      "type": "boolean",
                      "description": "Indicates whether the project is starred.\n`true`: starred\n`false`: not starred",
                      "example": false
                    },
                    "owner_display_name": {
                      "type": "string",
                      "description": "The project owner display name.",
                      "example": "Example Name"
                    }
                  }
                }
              }
            }
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this project. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `38007` <br>\n The projectId is not exist. <br>\n"
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
            "WhiteboardManagement:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_project:read",
            "whiteboard_project:read:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:read:project",
            "whiteboard:read:project:admin"
          ]
        }
      },
      "delete": {
        "tags": [
          "Project"
        ],
        "summary": "Delete a project",
        "description": "This API endpoint allows you to delete a specified project. Deleting a project removes all associated whiteboards from the project, setting them to have no project assignment. \n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:write:admin`,`whiteboard_project:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:delete:project`,`whiteboard:delete:project:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Deleteproject",
        "parameters": [
          {
            "name": "projectId",
            "in": "path",
            "description": "The project ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "qiMWddmtRxqwSFrK22lJPg"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "The project has been successfully deleted. No content is returned in the response body."
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this project. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `38007` <br>\n The projectId is not exist. <br>\n"
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
            "whiteboard_project:write:admin",
            "whiteboard_project:write"
          ],
          "x-granular-scopes": [
            "whiteboard:delete:project",
            "whiteboard:delete:project:admin"
          ]
        }
      },
      "patch": {
        "tags": [
          "Project"
        ],
        "summary": "Update project basic information",
        "description": "This API endpoint allows you to update the basic information of a specific project.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:write:admin`,`whiteboard_project:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:update:project:admin`,`whiteboard:update:project`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Updateproject",
        "parameters": [
          {
            "name": "projectId",
            "in": "path",
            "description": "The project ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "qiMWddmtRxqwSFrK22lJPg"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "project_name": {
                    "maxLength": 50,
                    "minLength": 1,
                    "type": "string",
                    "description": "The new project name.",
                    "example": "New Project Name"
                  }
                }
              }
            }
          }
        },
        "responses": {
          "204": {
            "description": "The project's basic information has been successfully updated. No content is returned in the response body."
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `38002` <br>\n The project name is invalid. <br>\n"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this project. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `38007` <br>\n The project ID does not exist. <br>\n"
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
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_project:write:admin",
            "whiteboard_project:write"
          ],
          "x-granular-scopes": [
            "whiteboard:update:project:admin",
            "whiteboard:update:project"
          ]
        }
      }
    },
    "/whiteboards/projects/{projectId}/collaborators": {
      "get": {
        "tags": [
          "Project"
        ],
        "summary": "Get collaborators of a project",
        "description": "Returns a list of a project collaborators.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project_collaborator:read`,`whiteboard_project_collaborator:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:project_collaborator`,`whiteboard:read:project_collaborator:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Getcollaboratorsofaproject",
        "parameters": [
          {
            "name": "projectId",
            "in": "path",
            "description": "The ID of the project.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "HY8-AC_HSou4SXY1S-OmOg"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Get collaborators of a project.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "array",
                  "items": {
                    "required": [
                      "collaborator_id"
                    ],
                    "type": "object",
                    "properties": {
                      "collaborator_id": {
                        "type": "string",
                        "description": "`not nullable` The collaborator's unique ID.",
                        "example": "x4g6hy789k0o878k"
                      },
                      "collaborator_name": {
                        "type": "string",
                        "description": "The collaborator's display name.",
                        "example": "Jack"
                      },
                      "user_id": {
                        "type": "string",
                        "description": "`nullable` User ID. This response only returns if the `collaborator_type` is `0`. If the `collaborator_type` is **not** `0`, this returns null value.",
                        "example": "wf3h7uo1dk0v53bl"
                      },
                      "avatar": {
                        "type": "string",
                        "description": "The collaborator profile picture.0",
                        "example": "https://content.zoom.us/swerwerw.jpeg"
                      },
                      "role": {
                        "type": "integer",
                        "description": "The collaborator role type. Enum:\n * `0` - OWNER \n * `1` - CO_OWNER \n * `2` - EDITOR \n * `3` - COMMENTER \n * `4` - VIEWER ",
                        "example": 2,
                        "enum": [
                          0,
                          1,
                          2,
                          3,
                          4
                        ]
                      },
                      "collaborator_type": {
                        "type": "integer",
                        "description": "The collaborator type. \nEnum: \n* `0` - SAME_ACCOUNT_USER \n* `1` - EXTERNAL_USER ",
                        "example": 0,
                        "enum": [
                          0,
                          1
                        ]
                      },
                      "email": {
                        "type": "string",
                        "description": "`nullable` Email. This response only returns if the `collaborator_type` is `0`. If the `collaborator_type` is **not** `0`, this returns null value.",
                        "example": "user@example.com"
                      }
                    },
                    "description": "A user who works jointly on a project."
                  }
                }
              }
            }
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this project. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `38007` <br>\n The projectId is not exist. <br>\n"
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
            "WhiteboardManagement:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_project_collaborator:read",
            "whiteboard_project_collaborator:read:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:read:project_collaborator",
            "whiteboard:read:project_collaborator:admin"
          ]
        }
      },
      "post": {
        "tags": [
          "Project"
        ],
        "summary": "Share a project to new users",
        "description": "Add new collaborators to a specified project. Project owners or authorized users can invite team members or external users to collaborate on a project with specified roles and permissions.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project_collaborator:write`,`whiteboard_project_collaborator:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:project_collaborator`,`whiteboard:write:project_collaborator:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Shareaprojecttonewusers",
        "parameters": [
          {
            "name": "projectId",
            "in": "path",
            "description": "The project ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "N6XZFDEQTkWSsMsmO_M1XQ"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "collaborators"
                ],
                "type": "object",
                "properties": {
                  "skip_notifications": {
                    "type": "boolean",
                    "description": " **Default is false (notifications will be sent)**.\n If set to `true`, collaborators and owners will not receive direct messages or emails when the project is shared.",
                    "example": false
                  },
                  "invite_message": {
                    "maxLength": 280,
                    "minLength": 0,
                    "type": "string",
                    "description": "A custom message to include with the project invitation.",
                    "example": "I'm glad to invite you to join this project."
                  },
                  "collaborators": {
                    "maxItems": 20,
                    "minItems": 1,
                    "type": "array",
                    "description": "List of users to add as collaborators to the project.",
                    "items": {
                      "type": "object",
                      "properties": {
                        "email": {
                          "type": "string",
                          "description": "**User Email Address**: The email address of the user you want to share the project with.",
                          "example": "user@example.com."
                        },
                        "role": {
                          "type": "integer",
                          "description": "Collaborator role type. When assigning a new `OWNER` or `CO_OWNER`, the specified user must belong to the same account as the original `OWNER`. If a new owner is designated for a project, the original owner will automatically be demoted to `CO_OWNER`.\nRole Enum Values: \n* `0` - OWNER \n* `1` - CO_OWNER \n* `2` - EDITOR \n* `3` - COMMENTER \n* `4` - VIEWER ",
                          "example": 1
                        }
                      },
                      "description": "A user who works jointly on a whiteboard."
                    }
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "The collaborators have been successfully added to the project.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "array",
                  "items": {
                    "title": "root",
                    "type": "object",
                    "properties": {
                      "collaborator_id": {
                        "type": "string",
                        "description": "`not nullable` The collaborator's unique ID.",
                        "example": "x4g6hy789k0o878k"
                      },
                      "collaborator_name": {
                        "type": "string",
                        "description": "The collaborator's display name.",
                        "example": "Jack"
                      },
                      "user_id": {
                        "type": "string",
                        "description": "`nullable` User ID. This response only returns if the `collaborator_type` is `0`. If the `collaborator_type` is **not** `0`, this returns null value.",
                        "example": "wf3h7uo1dk0v53bl"
                      },
                      "avatar": {
                        "type": "string",
                        "description": "Collaborator profile picture.",
                        "example": "https://content.zoom.us/swerwerw.jpeg"
                      },
                      "role": {
                        "type": "integer",
                        "description": "Collaborator role type. \nEnum: \n* `0` - OWNER \n* `1` - CO_OWNER \n* `2` - EDITOR \n* `3` - COMMENTER \n* `4` - VIEWER "
                      },
                      "collaborator_type": {
                        "type": "integer",
                        "description": "Collaborator type. \nEnum: \n* `0` - SAME_ACCOUNT_USER \n* `1` - EXTERNAL_USER"
                      }
                    },
                    "description": "Users who work jointly on a whiteboard."
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
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
            "WhiteboardManagement:Read",
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_project_collaborator:write",
            "whiteboard_project_collaborator:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:write:project_collaborator",
            "whiteboard:write:project_collaborator:admin"
          ]
        }
      },
      "patch": {
        "tags": [
          "Project"
        ],
        "summary": "Update project collaborators",
        "description": "Updates the role of a collaborator in a project, which allows you to modify the permissions of an existing collaborator in a specific project. \nNote that when assigning a new `OWNER` or `CO_OWNER`, the specified user must belong to the same account as the original `OWNER`. The original owner will automatically be demoted to `CO_OWNER` if a new owner is designated.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project_collaborator:write`,`whiteboard_project_collaborator:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:update:project_collaborator`,`whiteboard:update:project_collaborator:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Updateprojectcollaborators",
        "parameters": [
          {
            "name": "projectId",
            "in": "path",
            "description": "The project ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "N6XZFDEQTkWSsMsmO_M1XQ"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "collaborators"
                ],
                "type": "object",
                "properties": {
                  "collaborators": {
                    "maxItems": 20,
                    "minItems": 1,
                    "type": "array",
                    "description": "An array of collaborator objects.",
                    "items": {
                      "required": [
                        "collaborator_id",
                        "role"
                      ],
                      "type": "object",
                      "properties": {
                        "collaborator_id": {
                          "type": "string",
                          "description": "The collaborator's unique ID `not nullable` .",
                          "example": "x4g6hy789k0o878k"
                        },
                        "role": {
                          "type": "integer",
                          "description": "Collaborator role type.\nWhen assigning a new `OWNER` or `CO_OWNER`, the specified user must belong to the same account as the original `OWNER`. If a new owner is designated for a project, the original owner will automatically be demoted to `CO_OWNER`.\n\nRole Enum Values:\n\n0 - OWNER\n1 - CO_OWNER\n2 - EDITOR\n3 - COMMENTER\n4 - VIEWER",
                          "example": 2,
                          "default": 2,
                          "enum": [
                            0,
                            1,
                            2,
                            3,
                            4
                          ]
                        }
                      }
                    }
                  }
                },
                "description": "Update collaborators permission of a whiteboard."
              }
            }
          }
        },
        "responses": {
          "204": {
            "description": "Project collaborators updated successfully."
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid collaborator_id {collaboratorId}. <br>\n"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this project. <br>\n**Error Code:** `34001` <br>\n You do not have permission to update the role of collaborator {collaboratorId}. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `35004` <br>\n The project ID {projectID} is not exist. <br>\n"
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
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_project_collaborator:write",
            "whiteboard_project_collaborator:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:update:project_collaborator",
            "whiteboard:update:project_collaborator:admin"
          ]
        }
      }
    },
    "/whiteboards/projects/{projectId}/collaborators/{collaboratorId}": {
      "delete": {
        "tags": [
          "Project"
        ],
        "summary": "Remove the collaborator from a project",
        "description": "This API endpoint allows you to remove a specific collaborator from a designated project. It enables project managers or authorized users to revoke access or remove a user from a project, enhancing project management and access control capabilities.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project_collaborator:write`,`whiteboard_project_collaborator:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:delete:project_collaborator:admin`,`whiteboard:delete:project_collaborator`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Removethecollaboratorfromaproject",
        "parameters": [
          {
            "name": "projectId",
            "in": "path",
            "description": "The project ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "N6XZFDEQTkWSsMsmO_M1XQ"
            }
          },
          {
            "name": "collaboratorId",
            "in": "path",
            "description": "The collaborator ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "x4g6hy789k0o878k"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "The collaborator has been successfully removed from the project. No content is returned in the response body."
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this project. <br>\n**Error Code:** `34001` <br>\n You do not have permission to remove the collaborator {collaboratorId} from the project. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `38004` <br>\n The project ID {projectID} is not exist. <br>\n**Error Code:** `34005` <br>\n The collaborator ID {collaboratorId} is not exist. <br>\n"
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
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_project_collaborator:write",
            "whiteboard_project_collaborator:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:delete:project_collaborator:admin",
            "whiteboard:delete:project_collaborator"
          ]
        }
      }
    },
    "/whiteboards/projects/{projectId}/whiteboards": {
      "post": {
        "tags": [
          "Project"
        ],
        "summary": "Move whiteboards to a project",
        "description": "This API endpoint allows you to move one or more whiteboards to a specified project. It enables users to organize their whiteboards by associating them with a specific project, enhancing project management capabilities.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:write:admin`,`whiteboard_project:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:project_whiteboard`,`whiteboard:write:project_whiteboard:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Movewhiteboardstoproject",
        "parameters": [
          {
            "name": "projectId",
            "in": "path",
            "description": "The Project ID",
            "required": true,
            "schema": {
              "type": "string",
              "example": "qiMWddmtRxqwSFrK22lJPg"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "whiteboard_ids"
                ],
                "type": "object",
                "properties": {
                  "whiteboard_ids": {
                    "title": "whiteboard_id",
                    "maxItems": 50,
                    "minItems": 1,
                    "type": "array",
                    "description": "The whiteboard ID list",
                    "items": {
                      "type": "string",
                      "description": "The whiteboard ID.",
                      "example": "dZO3Wd9XT4mgXdNPbMm6tw"
                    }
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "The whiteboards have been successfully moved to the project.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "whiteboard_ids": {
                      "type": "array",
                      "description": "The whiteboard ID list",
                      "items": {
                        "type": "string",
                        "description": "The whiteboard ID.",
                        "example": "dZO3Wd9XT4mgXdNPbMm6tw"
                      }
                    }
                  }
                }
              }
            }
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this project. <br>\n**Error Code:** `34001` <br>\n You do not have permission to move {whiteboardID} to the project. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `38007` <br>\n The projectId is not exist. <br>\n"
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
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_project:write:admin",
            "whiteboard_project:write"
          ],
          "x-granular-scopes": [
            "whiteboard:write:project_whiteboard",
            "whiteboard:write:project_whiteboard:admin"
          ]
        }
      },
      "delete": {
        "tags": [
          "Project"
        ],
        "summary": "Remove whiteboards from a project",
        "description": "This API endpoint allows you to remove one or more whiteboards from a specified project. \n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:write:admin`,`whiteboard_project:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:delete:project_whiteboard`,`whiteboard:delete:project_whiteboard:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Removewhiteboardsfromaproject",
        "parameters": [
          {
            "name": "projectId",
            "in": "path",
            "description": "The project ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "N6XZFDEQTkWSsMsmO_M1XQ"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "whiteboard_ids"
                ],
                "type": "object",
                "properties": {
                  "whiteboard_ids": {
                    "maxItems": 50,
                    "minItems": 1,
                    "type": "array",
                    "description": "The whiteboard ID list.",
                    "items": {
                      "type": "string",
                      "description": "The whiteboard ID.",
                      "example": "W6Uuv4qUSfuJlp_GXGi36A"
                    }
                  }
                }
              }
            }
          }
        },
        "responses": {
          "204": {
            "description": "The whiteboards have been successfully removed from the project. No content is returned in the response body."
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this project. <br>\n**Error Code:** `34001` <br>\n You have no permission to add {whiteboardID} to the project. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `38007` <br>\n The project ID does not exist. <br>\n"
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
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_project:write:admin",
            "whiteboard_project:write"
          ],
          "x-granular-scopes": [
            "whiteboard:delete:project_whiteboard",
            "whiteboard:delete:project_whiteboard:admin"
          ]
        }
      }
    },
    "/whiteboards/{whiteboardId}/share_setting": {
      "patch": {
        "tags": [
          "Settings"
        ],
        "summary": "Update whiteboard share setting",
        "description": "Updates the whiteboard share setting. \n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_share_setting:write`,`whiteboard_share_setting:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:update:share_setting:admin`,`whiteboard:update:share_setting`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "UpdateAWhiteboardShareSetting",
        "parameters": [
          {
            "name": "whiteboardId",
            "in": "path",
            "description": "The ID of the whiteboard.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "HY8-AC_HSou4SXY1S-OmOg"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "share_link_setting": {
                    "type": "object",
                    "properties": {
                      "share_scope": {
                        "type": "integer",
                        "description": "The access type of whiteboard share link. \n* Enum: \n* `0` - `Restricted`, Only people added can open with this link \n* `1` - `Same account` \n* `2` - Anyone \n* `3` - SAME_ACCOUNT_EXTERNAL_AUTH \n",
                        "example": 0,
                        "default": 0,
                        "enum": [
                          0,
                          1,
                          2,
                          3
                        ],
                        "x-enum-descriptions": [
                          "Restricted. Only people added can open with this link.",
                          "Same account",
                          "Anyone",
                          "Same account external auth"
                        ]
                      },
                      "share_role": {
                        "type": "integer",
                        "description": "The access level of a whiteboard share link. Use this field with `share_scope`. If `share_scope` set `0` - `Restricted`, this field is null. If `share_scope` set `1` - `Same account` or `2` - `Anyone`, this filed can be `2` - `Editor`, `3` - `Commenter` or `4` - `Viewer`. \n* Enum: \n* `2` - `Editor`\n* `3` - `Commenter` \n* `4` - Viewer \n",
                        "example": 2,
                        "enum": [
                          2,
                          3,
                          4
                        ],
                        "x-enum-descriptions": [
                          "Editor",
                          "Commenter",
                          "Viewer"
                        ]
                      }
                    },
                    "description": "The access level and access type of a current whiteboard's share link."
                  }
                },
                "description": "Updates a whiteboard's share setting."
              }
            }
          }
        },
        "responses": {
          "204": {
            "description": "**HTTP Status Code**: `204`   \n \nWhiteboard share setting updated successfully."
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Share role must be viewer for everyone. <br>\n**Error Code:** `34002` <br>\n Share role can't be null, owner, or co-owner for same account. <br>\n"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this document. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_share_setting:write",
            "whiteboard_share_setting:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:update:share_setting:admin",
            "whiteboard:update:share_setting"
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
    "title": "Whiteboard",
    "description": "The [Zoom Whiteboard APIs](https://developers.zoom.us/docs/zoom-whiteboard/) allow you to manage your Zoom Whiteboard. For example, collaborative persistent whiteboard that allows participants to share and collaborate on a virtual whiteboard in real time The Zoom Whiteboard REST API for [Zoom Whiteboard](https://developers.zoom.us/docs/api/rest/whiteboard/) allows developers to interact with various Zoom features, including Zoom Whiteboard. Developers can create and manage whiteboards programmatically using these APIs.",
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
    "/whiteboards/sessions": {
      "get": {
        "tags": [
          "Archiving"
        ],
        "summary": "List whiteboards sessions",
        "description": "Creates a list of archived sessions under current account.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_content:read`,`whiteboard_content:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:list_sessions`,`whiteboard:read:list_sessions:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "Createwhiteboardsarchivefiles",
        "parameters": [
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of records returned within a single API call.",
            "required": false,
            "schema": {
              "maximum": 300,
              "type": "integer",
              "example": 30,
              "default": 30
            }
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "The next page token paginates through a large set of results. A next page token returns whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "IAfJX3jsOLW7w3dokmFl84zOa0MAVGyMEB2"
            }
          },
          {
            "name": "from",
            "in": "query",
            "description": "The query session\u00a0`start_time\u00a0`date in`\u00a0yyyy-MM-dd'T'HH:mm:ssZ`\u00a0format. The\u00a0`start_time`\u00a0of a session in the result list should be greater than or equal to\u00a0`from`. If this field has no provided value, the default is the current date. For example, if you make the API request on June 30, 2020, without providing the\u00a0`from`\u00a0and\u00a0`to`\u00a0parameters, by default, the value of\u00a0the `from`\u00a0field will be \u201c2020-06-30,\u201d and the value of the\u00a0`to`\u00a0field will be \u201c2020-07-30\u201d.",
            "required": false,
            "schema": {
              "type": "string",
              "format": "date-time",
              "example": "2021-03-12T02:12:27Z"
            }
          },
          {
            "name": "to",
            "in": "query",
            "description": "The query session`end_time` date, in `yyyy-MM-dd'T'HH:mm:ssZ` format. The `end_time` of session in result list should be less than or equal to `to`.",
            "required": false,
            "schema": {
              "type": "string",
              "format": "date-time",
              "example": "2021-03-18T05:41:36Z"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "The returned archived sessions list.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "from",
                    "next_page_token",
                    "page_size",
                    "to"
                  ],
                  "type": "object",
                  "properties": {
                    "from": {
                      "type": "string",
                      "description": "The queried start date.",
                      "format": "date-time",
                      "example": "2021-03-12T02:12:27Z"
                    },
                    "to": {
                      "type": "string",
                      "description": "The queried end date.",
                      "format": "date-time",
                      "example": "2021-03-12T02:12:27Z"
                    },
                    "page_size": {
                      "maximum": 300,
                      "type": "integer",
                      "description": "The number of records returned per page in a single API call.",
                      "example": 30,
                      "default": 30
                    },
                    "next_page_token": {
                      "type": "string",
                      "description": "The next page token paginates through a large set of results. A next page token returns whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.\n\nNote: If you use `next_page_token` as a parameter, your other request parameters should be changeless to make sure that the large result set is what you want. For example, if your to parameter is for a future time, Zoom resets this value to the current time and returns this value in the response body, along with the `next_page_token` value. Use these same to and `next_page_token` values in requests for the remaining results set; otherwise you will get an invalid token error.",
                      "example": "IAfJX3jsOLW7w3dokmFl84zOa0MAVGyMEB2"
                    },
                    "sessions": {
                      "maxItems": 300,
                      "type": "array",
                      "description": "The whiteboard archived sessions list. ",
                      "items": {
                        "type": "object",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The session's unique ID.",
                            "example": "a2f19f96-9294-4f51-8134-6f0eea108eb2"
                          },
                          "start_time": {
                            "type": "string",
                            "description": "The session's start time.\nIt should be `from < start_time`",
                            "format": "date-time",
                            "example": "2021-03-11T05:41:36Z"
                          },
                          "end_time": {
                            "type": "string",
                            "description": "The session's end time.\nIt should be `end_time < to`",
                            "format": "date-time",
                            "example": "2021-03-11T05:41:36Z"
                          },
                          "whiteboard_id": {
                            "type": "string",
                            "description": "The ID of whiteboard.",
                            "example": "HY8-AC_HSou4SXY1S-OmOg"
                          },
                          "whiteboard_name": {
                            "type": "string",
                            "description": "The name of whiteboard.",
                            "example": "example_whiteboard_name"
                          },
                          "expire_time": {
                            "type": "string",
                            "description": "The session's expire time. ",
                            "format": "date-time",
                            "example": "2021-03-26T06:41:36Z"
                          },
                          "regulated_participants": {
                            "maxItems": 1000,
                            "type": "array",
                            "description": "The session participant under this account.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "user_id": {
                                  "type": "string",
                                  "description": "The unique ID of participant user in this session.",
                                  "example": "wf3h7uo1dk0v53bl"
                                },
                                "user_name": {
                                  "type": "string",
                                  "description": "The name of participant user in this session.",
                                  "example": "Jack"
                                },
                                "join_whiteboard_time": {
                                  "type": "string",
                                  "description": "The user join the whiteboard time.",
                                  "format": "date-time",
                                  "example": "2021-03-26T06:41:36Z"
                                },
                                "leave_whiteboard_time": {
                                  "type": "string",
                                  "description": "The user leave the whiteboard time.",
                                  "format": "date-time",
                                  "example": "2021-03-26T07:41:36Z"
                                },
                                "email": {
                                  "type": "string",
                                  "description": "The email of participant. `nullable` If the participant is not part of the session's account, this returns an empty string value.",
                                  "example": "user@example.com"
                                }
                              },
                              "description": "The regulated user info under the account"
                            }
                          }
                        }
                      }
                    },
                    "total_records": {
                      "type": "integer",
                      "description": "The total number of returned sessions records.",
                      "example": 20
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid parameter. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_content:read",
              "whiteboard_content:read:admin",
              "whiteboard:read:list_sessions",
              "whiteboard:read:list_sessions:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardContent:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_content:read",
            "whiteboard_content:read:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:read:list_sessions",
            "whiteboard:read:list_sessions:admin"
          ]
        }
      }
    },
    "/whiteboards/sessions/activity/download/{path}": {
      "get": {
        "tags": [
          "Archiving"
        ],
        "summary": "Download Whiteboards activity file",
        "description": "Use this API to download the Whiteboard archival file. For obtaining the download URL, please consult the [List whiteboard sessions activities](https://developers.zoom.us/docs/api/rest/reference/whiteboard/methods/#operation/Listwhiteboardsessionsarchivedfiles) documentation. Scroll down under field on schema to documentation URL string.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_content:read`,`whiteboard_content:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:archived_file`,`whiteboard:read:archived_file:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "Downloadwhiteboardsactivityfile",
        "parameters": [
          {
            "name": "path",
            "in": "path",
            "description": "The file path.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "Qg75t7xZBtEbAkjdlgbfdngBBBB"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Returns a file stream.",
            "headers": {
              "Content-Type": {
                "description": "Content-Type.",
                "required": false,
                "schema": {
                  "type": "string",
                  "example": "application/octet-stream"
                }
              },
              "Content-Disposition": {
                "description": "Content-Disposition.",
                "required": false,
                "schema": {
                  "type": "string",
                  "example": "attachment;filename=adcc655d3a3549e692959737fe2a7a0d.pdf"
                }
              },
              "Content-Length": {
                "description": "Content-Length.",
                "required": false,
                "schema": {
                  "type": "string",
                  "example": "102400"
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid path. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `34000` <br>\n The resource not found. The file does not exist or the file has expired. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_content:read",
              "whiteboard_content:read:admin",
              "whiteboard:read:archived_file",
              "whiteboard:read:archived_file:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardContent:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_content:read",
            "whiteboard_content:read:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:read:archived_file",
            "whiteboard:read:archived_file:admin"
          ]
        }
      }
    },
    "/whiteboards/sessions/{seesionId}": {
      "get": {
        "tags": [
          "Archiving"
        ],
        "summary": "List whiteboard sessions activities",
        "description": "Lists a session's activity information.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_content:read`,`whiteboard_content:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:session`,`whiteboard:read:session:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "Listwhiteboardsessionsarchivedfiles",
        "parameters": [
          {
            "name": "seesionId",
            "in": "path",
            "description": "The session unique ID.\n",
            "required": true,
            "schema": {
              "type": "string",
              "example": "f8b9197a-0c8f-c8de-4db3-d8a5e8d9f505"
            }
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of records returned within a single API call.",
            "required": false,
            "schema": {
              "maximum": 300,
              "type": "integer",
              "example": 30,
              "default": 30
            }
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "The next page token paginates through a large set of results. A next page token returns whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "IAfJX3jsOLW7w3dokmFl84zOa0MAVGyMEB2"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "The session activity information.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "page_size",
                    "total_records"
                  ],
                  "type": "object",
                  "properties": {
                    "page_size": {
                      "type": "integer",
                      "description": "The number of records returned within a single API call.",
                      "example": 30
                    },
                    "next_page_token": {
                      "type": "string",
                      "description": "The next page token paginates through a large set of results. A next page token returns whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes. **Note**: if you use `next_page_token` as a parameter, your other request parameters should be changeless to make sure that the large result set is what you want. For example, if your to parameter is for a future time, Zoom resets this value to the current time and returns this value in the response body, along with the `next_page_token` value. Use these same to and `next_page_token` values in requests for the remaining results set; otherwise you will get an invalid token error.",
                      "example": "IAfJX3jsOLW7w3dokmFl84zOa0MAVGyMEB2"
                    },
                    "session_id": {
                      "type": "string",
                      "description": "The unique ID of current session.",
                      "example": "a2f19f96-9294-4f51-8134-6f0eea108eb2"
                    },
                    "whiteboard_id": {
                      "type": "string",
                      "description": "The ID of the Zoom Whiteboard.",
                      "example": "HY8-AC_HSou4SXY1S-OmOg"
                    },
                    "whiteboard_name": {
                      "type": "string",
                      "description": "The name of Zoom Whiteboard.",
                      "example": "example_whiteboard_name"
                    },
                    "start_time": {
                      "type": "string",
                      "description": "The session's start time.",
                      "format": "date-time",
                      "example": "2021-03-11T05:41:36Z"
                    },
                    "end_time": {
                      "type": "string",
                      "description": "The session's end time.",
                      "format": "date-time",
                      "example": "2021-03-11T06:41:36Z"
                    },
                    "expire_time": {
                      "type": "string",
                      "description": "The session's expire time. ",
                      "format": "date-time",
                      "example": "2021-03-26T06:41:36Z"
                    },
                    "activities": {
                      "maxItems": 300,
                      "type": "array",
                      "description": "The activity list of a session.",
                      "items": {
                        "type": "object",
                        "properties": {
                          "id": {
                            "type": "string",
                            "description": "The unique ID of an activity.",
                            "example": "cc63bce7-cb59-c1dc-fadd-2be734ad1d55"
                          },
                          "type": {
                            "type": "string",
                            "description": "The operation description.",
                            "example": "add object"
                          },
                          "download_url": {
                            "type": "string",
                            "description": " The URL to download the the archive file. OAuth apps If a user has authorized and installed your OAuth app that contains `whiteboard:read:archived_file` or `whiteboard:read:archived_file:admin` scopes, use the user's [OAuth access token](https://developers.zoom.us/docs/integrations/oauth/) to download the file. More details refer [Download Whiteboards activity file](https://developers.zoom.us/docs/api/rest/reference/whiteboard/methods/#operation/Downloadwhiteboardsactivityfile)\n For example:  `https://{{base-domain}}/whiteboards/sesssions/activity/download/xxx--header 'Authorization: Bearer {{OAuth-access-token}}'`",
                            "example": "https://example.com/whiteboards/sessions/activity/download/Qg75t7xZBtEbAkjdlgbfdngBBBB"
                          },
                          "create_time": {
                            "type": "string",
                            "description": "The activity create time.",
                            "format": "date-time",
                            "example": "2021-03-11T05:41:36Z"
                          },
                          "user_id": {
                            "type": "string",
                            "description": "The ID of user. `nullable`  If the participant is **not** part of the session's account, this returns an empty string value.",
                            "example": "wf3h7uo1dk0v53bl"
                          },
                          "collaborator_id": {
                            "type": "string",
                            "description": "The participant's unique ID. `not nullable` ",
                            "example": "x4g6hy789k0o878k"
                          },
                          "collaborator_name": {
                            "type": "string",
                            "description": "The participant's display name.",
                            "example": "Jack"
                          },
                          "email": {
                            "type": "string",
                            "description": "The email of participant. `nullable` If the participant is not part of the session's account, this returns an empty string value.",
                            "example": "user@example.com"
                          },
                          "content": {
                            "type": "string",
                            "description": "The text content generated by the activity.",
                            "example": "This is a activity content demo"
                          },
                          "object_ids": {
                            "maxItems": 100,
                            "minItems": 0,
                            "type": "array",
                            "description": "The IDs of the whiteboard object associated with this activity.",
                            "items": {
                              "type": "string",
                              "description": "The object ID in the whiteboard.",
                              "example": "wfx4g6s1dk0v53bl"
                            }
                          }
                        },
                        "description": "activity information"
                      }
                    },
                    "total_records": {
                      "type": "number",
                      "description": "The total number of returned activities records.",
                      "example": 30
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
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `34000` <br>\n The session not exist or expired. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_content:read",
              "whiteboard_content:read:admin",
              "whiteboard:read:session",
              "whiteboard:read:session:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardContent:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_content:read",
            "whiteboard_content:read:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:read:session",
            "whiteboard:read:session:admin"
          ]
        }
      }
    },
    "/whiteboards/{whiteboardId}/collaborator": {
      "get": {
        "tags": [
          "Collaborator"
        ],
        "summary": "Get collaborators of a whiteboard",
        "description": "Returns a list of a whiteboard collaborators. \n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_collaborator:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:list_collaborators:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "GetAWhiteboardCollaborator",
        "parameters": [
          {
            "name": "whiteboardId",
            "in": "path",
            "description": "The ID of the whiteboard.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "HY8-AC_HSou4SXY1S-OmOg"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "**HTTP Status Code:** `200`   \n \nWhiteboard collaborators returned.",
            "content": {
              "application/json": {
                "schema": {
                  "maxItems": 1000,
                  "type": "array",
                  "items": {
                    "type": "object",
                    "properties": {
                      "collaborator_id": {
                        "type": "string",
                        "description": "`not nullable` The collaborator's unique ID.",
                        "example": "x4g6hy789k0o878k"
                      },
                      "collaborator_name": {
                        "type": "string",
                        "description": "The collaborator's display name.",
                        "example": "Jack"
                      },
                      "user_id": {
                        "type": "string",
                        "description": "`nullable` User ID. This response only returns if the `collaborator_type` is `0`. If the `collaborator_type` is **not** `0`, this returns null value.",
                        "example": "wf3h7uo1dk0v53bl"
                      },
                      "channel_id": {
                        "type": "string",
                        "description": "`nullable` Team chat channel ID. This response only returns if the `collaborator_type` is `2`. If the `collaborator_type` is **not** `2`, this returns null value.",
                        "example": "cabc1234567defghijkl01234"
                      },
                      "avatar": {
                        "type": "string",
                        "description": "The collaborator profile picture.",
                        "example": "https://content.zoom.us/swerwerw.jpeg"
                      },
                      "role": {
                        "type": "integer",
                        "description": "The collaborator role type. \nEnum: \n* `0` - OWNER \n* `1` - CO_OWNER \n* `2` - EDITOR \n* `3` - COMMENTER \n* `4` - VIEWER \n",
                        "example": 0,
                        "enum": [
                          0,
                          1,
                          2,
                          3,
                          4
                        ],
                        "x-enum-descriptions": [
                          "OWNER",
                          "CO_OWNER",
                          "EDITOR",
                          "COMMENTER",
                          "VIEWER"
                        ]
                      },
                      "collaborator_type": {
                        "type": "integer",
                        "description": "The collaborator type. \nEnum: \n* `0` - SAME_ACCOUNT_USER \n* `1` - EXTERNAL_USER \n* `2` - CHANNEL \n",
                        "example": 0,
                        "enum": [
                          0,
                          1,
                          2
                        ],
                        "x-enum-descriptions": [
                          "SAME_ACCOUNT_USER",
                          "EXTERNAL_USER",
                          "CHANNEL"
                        ]
                      },
                      "email": {
                        "type": "string",
                        "description": "`nullable` Email. This response only returns if the `collaborator_type` is `0`. If the `collaborator_type` is **not** `0`, this returns null value.",
                        "example": "user@example.com"
                      }
                    },
                    "description": "A user or a team chat channel who works jointly on a whiteboard."
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this document. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_collaborator:read:admin",
              "whiteboard:read:list_collaborators:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_collaborator:read:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:read:list_collaborators:admin"
          ]
        }
      },
      "post": {
        "tags": [
          "Collaborator"
        ],
        "summary": "Share a whiteboard to new users or team chat channels.",
        "description": "Add collaborators to a whiteboard.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_collaborator:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:collaborator:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "AddAWhiteboardCollaborator",
        "parameters": [
          {
            "name": "whiteboardId",
            "in": "path",
            "description": "The ID of the whiteboard.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "HY8-AC_HSou4SXY1S-OmOg"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "skip_notifications": {
                    "type": "boolean",
                    "description": " **Default is false (notifications will be sent)**.\n If set to `true`, collaborators and owners will not receive direct messages or emails when the whiteboard is shared.",
                    "example": false
                  },
                  "invite_message": {
                    "maxLength": 280,
                    "minLength": 0,
                    "type": "string",
                    "description": "The custom message to include with the whiteboard invitation.",
                    "example": "I'm glad to invite you to join this whiteboard."
                  },
                  "collaborators": {
                    "maxItems": 100,
                    "minItems": 1,
                    "type": "array",
                    "description": "Array of collaborator objects to be added to the whiteboard.",
                    "items": {
                      "type": "object",
                      "properties": {
                        "email": {
                          "type": "string",
                          "description": "**User Email Address**\nThe email address of the user you want to share the whiteboard with.\nThis field is **required** when sharing a whiteboard with an individual user.\nIt **must be left empty** when sharing via a channel.\n**Note**: email and channel_id cannot be set at the same time.",
                          "example": "user@example.com."
                        },
                        "channel_id": {
                          "type": "string",
                          "description": "**Channel ID**\nThe unique identifier of the channel you want to share the whiteboard with.\nThis field is **required** when sharing a whiteboard via a channel.\nIt **must be left empty** when sharing with an individual user by email.\n**Note**: channel_id and email cannot be set at the same time.",
                          "example": "cabc1234567defghijkl01234"
                        },
                        "role": {
                          "type": "integer",
                          "description": "Collaborator role type. \nWhen assigning a new `OWNER` or `CO_OWNER`, the specified user must belong to the same account as the original `OWNER`. If a new owner is designated for a whiteboard, the original owner will automatically be demoted to `CO_OWNER`.\n\nRole Enum Values: \n* `0` - OWNER \n* `1` - CO_OWNER \n* `2` - EDITOR \n* `3` - COMMENTER \n* `4` - VIEWER ",
                          "example": 1,
                          "enum": [
                            0,
                            1,
                            2,
                            3,
                            4
                          ],
                          "x-enum-descriptions": [
                            "OWNER",
                            "CO_OWNER",
                            "EDITOR",
                            "COMMENTER",
                            "VIEWER"
                          ]
                        }
                      },
                      "description": "A user or a team chat channel who works jointly on a whiteboard. \n* `Email` or `ChannelId` is required according to the audience `type`."
                    }
                  }
                },
                "description": "Users or team chat channels to be added as collaborators to the whiteboard."
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "**HTTP Status Code:** `201`   \n\n Whiteboard added collaborators returned.",
            "content": {
              "application/json": {
                "schema": {
                  "maxItems": 100,
                  "type": "array",
                  "items": {
                    "type": "object",
                    "properties": {
                      "collaborator_id": {
                        "type": "string",
                        "description": "The collaborator's unique identifier. This field is not nullable.",
                        "example": "x4g6hy789k0o878k"
                      },
                      "collaborator_name": {
                        "type": "string",
                        "description": "The collaborator's display name.",
                        "example": "Jack"
                      },
                      "user_id": {
                        "type": "string",
                        "description": "`nullable` User ID. This response only returns if the `collaborator_type` is `0`. If the `collaborator_type` is **not** `0`, this returns null value.",
                        "example": "wf3h7uo1dk0v53bl"
                      },
                      "channel_id": {
                        "type": "string",
                        "description": "`nullable` Team chat channel ID. This response only returns if the `collaborator_type` is `2`. If the `collaborator_type` is **not** `2`, this returns null value.",
                        "example": "cabc1234567defghijkl01234"
                      },
                      "avatar": {
                        "type": "string",
                        "description": "Collaborator profile picture.",
                        "example": "https://content.zoom.us/swerwerw.jpeg"
                      },
                      "role": {
                        "type": "integer",
                        "description": "Collaborator role type. \nEnum: \n* `0` - OWNER \n* `1` - CO_OWNER \n* `2` - EDITOR \n* `3` - COMMENTER \n* `4` - VIEWER \n",
                        "example": 0,
                        "enum": [
                          0,
                          1,
                          2,
                          3,
                          4
                        ],
                        "x-enum-descriptions": [
                          "OWNER",
                          "CO_OWNER",
                          "EDITOR",
                          "COMMENTER",
                          "VIEWER"
                        ]
                      },
                      "collaborator_type": {
                        "type": "integer",
                        "description": "Collaborator type. \nEnum: \n* `0` - SAME_ACCOUNT_USER \n* `1` - EXTERNAL_USER \n* `2` - CHANNEL \n",
                        "example": 0,
                        "enum": [
                          0,
                          1,
                          2
                        ],
                        "x-enum-descriptions": [
                          "SAME_ACCOUNT_USER",
                          "EXTERNAL_USER",
                          "CHANNEL"
                        ]
                      }
                    },
                    "description": "A user or team chat channel who works jointly on a whiteboard."
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_collaborator:write:admin",
              "whiteboard:write:collaborator:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_collaborator:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:write:collaborator:admin"
          ]
        }
      },
      "patch": {
        "tags": [
          "Collaborator"
        ],
        "summary": "Update whiteboard collaborators",
        "description": "Use this API to update whiteboard collaborators. \n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_collaborator:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:update:collaborator:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "UpdateAWhiteboardCollaborator",
        "parameters": [
          {
            "name": "whiteboardId",
            "in": "path",
            "description": "The ID of the whiteboard.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "HY8-AC_HSou4SXY1S-OmOg"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "collaborators": {
                    "maxItems": 100,
                    "minItems": 1,
                    "type": "array",
                    "description": "An array of collaborator objects.",
                    "items": {
                      "type": "object",
                      "properties": {
                        "collaborator_id": {
                          "type": "string",
                          "description": "The collaborator's unique ID `not nullable` .",
                          "example": "x4g6hy789k0o878k"
                        },
                        "role": {
                          "type": "integer",
                          "description": "Collaborator role type. \nWhen assigning a new `OWNER` or `CO_OWNER`, the specified user must belong to the same account as the original `OWNER`. If a new owner is designated for a whiteboard, the original owner will automatically be demoted to `CO_OWNER`.\n\nRole Enum Values: \n* `0` - OWNER \n* `1` - CO_OWNER \n* `2` - EDITOR \n* `3` - COMMENTER \n* `4` - VIEWER ",
                          "example": 0,
                          "enum": [
                            0,
                            1,
                            2,
                            3,
                            4
                          ],
                          "x-enum-descriptions": [
                            "OWNER",
                            "CO_OWNER",
                            "EDITOR",
                            "COMMENTER",
                            "VIEWER"
                          ]
                        }
                      }
                    }
                  }
                },
                "description": "Update collaborators permission of a whiteboard."
              }
            }
          }
        },
        "responses": {
          "204": {
            "description": "**HTTP Status Code**: `204`   \n \nWhiteboard collaborators updated successfully."
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid collaboratorId {collaboratorId}. <br>\n"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this document. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `34000` <br>\n The resource {whiteboardId} not exist. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_collaborator:write:admin",
              "whiteboard:update:collaborator:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_collaborator:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:update:collaborator:admin"
          ]
        }
      }
    },
    "/whiteboards/{whiteboardId}/collaborator/{collaboratorId}": {
      "delete": {
        "tags": [
          "Collaborator"
        ],
        "summary": "Remove the collaborator from a whiteboard",
        "description": "Removes a collaborator from a whiteboard. \n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_collaborator:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:delete:collaborator:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "DeleteAWhiteboardCollaborator",
        "parameters": [
          {
            "name": "whiteboardId",
            "in": "path",
            "description": "The ID of the whiteboard.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "HY8-AC_HSou4SXY1S-OmOg"
            }
          },
          {
            "name": "collaboratorId",
            "in": "path",
            "description": "The ID of the whiteboard collaborator.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "sWvlvz-G5XFQxpfxEBB0PfVALRdcpHdTAqwqigrOPb1TgkxtFJI.3Z6nozeexgNAEecI"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "**HTTP Status Code**: `204`   \n \nWhiteboard collaborators deleted successfully."
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this document. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `34002` <br>\n Resource not found.  <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_collaborator:write:admin",
              "whiteboard:delete:collaborator:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_collaborator:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:delete:collaborator:admin"
          ]
        }
      }
    },
    "/whiteboards": {
      "get": {
        "tags": [
          "Document"
        ],
        "summary": "List all whiteboards",
        "description": "Use this API to list whiteboards of current user (**Required Scopes**: `whiteboard:read`) or list all whiteboards under the account (**Required Scopes**: `whiteboard:read:admin`).\n\n**Note:** `user_id` can intake `Zoom user ID` or `email` which only applies when scope = `whiteboard:read:admin`. Otherwise, the `user_id` can only intake `me`.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:admin`,`whiteboard:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:list_whiteboards:admin`,`whiteboard:read:list_whiteboards`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "ListWhiteboards",
        "parameters": [
          {
            "name": "search_key",
            "in": "query",
            "description": "The name of a whiteboard. If this parameter is provided, only the matched whiteboard will be returned. Otherwise, all whiteboards will be returned.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "PGsQ3nwITmyTG4JkFNWJcg"
            }
          },
          {
            "name": "user_id",
            "in": "query",
            "description": "The email address or user ID of the whiteboard owner. If this parameter is provided, only the whiteboard owned by the provided user will be returned. \nIf set to `me`, will return the matched whiteboards of current user.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "me"
            }
          },
          {
            "name": "date_filter_type",
            "in": "query",
            "description": "The whiteboard date filter type. \nEnum:\n* `created_date` - Whiteboard create date. \n* `modified_date` - Whiteboard modify date.\n",
            "required": false,
            "schema": {
              "type": "string",
              "example": "modified_date",
              "default": "modified_date",
              "enum": [
                "created_date",
                "modified_date"
              ]
            }
          },
          {
            "name": "from",
            "in": "query",
            "description": "The queried start date and time in `yyyy-MM-dd'T'HH:mm:ss'Z'` UTC format for the created date range for which you would like to retrieve whiteboards. The maximum range can be a month. If no value is provided for this field, the default will be current date. For example, if you make the API request on June 30, 2020, without providing the &ldquo;from&rdquo; and &ldquo;to&rdquo; parameters, by default the value of 'from' field will be &ldquo;2020-06-30&rdquo; and the value of the 'to' field will be &ldquo;2020-07-01&rdquo;.",
            "required": false,
            "schema": {
              "type": "string",
              "format": "date-time",
              "example": "2022-03-08T16:00:00Z"
            }
          },
          {
            "name": "to",
            "in": "query",
            "description": "The queried end date and time in `yyyy-MM-dd'T'HH:mm:ss'Z'` UTC format. ",
            "required": false,
            "schema": {
              "type": "string",
              "format": "date-time",
              "example": "2022-03-08T16:00:00Z"
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
            "description": "The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42"
            }
          },
          {
            "name": "project_id",
            "in": "query",
            "description": "Project ID to search for.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "N6XZFDEQTkWSsMsmO_M1XQ"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "**HTTP Status Code:** `200`   \n \nWhiteboard list returned.",
            "content": {
              "application/json": {
                "schema": {
                  "title": "Whiteboard List",
                  "description": "The whiteboard list.",
                  "allOf": [
                    {
                      "type": "object",
                      "properties": {
                        "whiteboards": {
                          "maxItems": 50,
                          "type": "array",
                          "description": "The information about the whiteboard list.",
                          "items": {
                            "allOf": [
                              {
                                "type": "object",
                                "properties": {
                                  "whiteboard_id": {
                                    "type": "string",
                                    "description": "Whiteboard ID",
                                    "example": "48Z4ppMfQJWONeVWFCcVzA"
                                  },
                                  "whiteboard_name": {
                                    "type": "string",
                                    "description": "Whiteboard name",
                                    "example": "Q3 Plan",
                                    "default": "Untitled"
                                  },
                                  "locked": {
                                    "type": "boolean",
                                    "description": "Indicates whether the whiteboard is locked or not.\n\n`true`: The whiteboard is locked.   \n \n\n`false`: The whiteboard is unlocked",
                                    "example": false,
                                    "default": false
                                  },
                                  "trashed": {
                                    "type": "boolean",
                                    "description": "Indicates whether the whiteboard is moved to trash or not.\n\n`true`: The whiteboard is moved to trash.   \n \n\n`false`: The whiteboard isn't moved to trash",
                                    "example": false,
                                    "default": false
                                  },
                                  "readonly": {
                                    "type": "boolean",
                                    "description": "Indicates whether the whiteboard is editable or not. If a whiteboard is locked, this field will set `true`. \n\n`true`: The whiteboard is editable.   \n \n\n`false`: The whiteboard is readonly",
                                    "example": false,
                                    "default": false
                                  },
                                  "non_cloud": {
                                    "type": "boolean",
                                    "description": "Indicates whether the whiteboard is allowed to be saved to cloud or not.\n\n`true`: The whiteboard is a saved to cloud.   \n \n\n`false`: The whiteboard isn't saved to cloud",
                                    "example": false,
                                    "default": false
                                  },
                                  "created_date": {
                                    "type": "string",
                                    "description": "Whiteboard create date-time in UTC/GMT. Example: &quot;2020-03-31T12:02:00Z&quot;",
                                    "format": "date-time",
                                    "example": "2022-03-25T07:29:29Z"
                                  },
                                  "modified_date": {
                                    "type": "string",
                                    "description": "Whiteboard update date-time in UTC/GMT. Example: &quot;2020-03-31T12:02:00Z&quot;",
                                    "format": "date-time",
                                    "example": "2022-03-25T07:29:29Z"
                                  },
                                  "share_link_setting": {
                                    "type": "object",
                                    "properties": {
                                      "share_scope": {
                                        "type": "integer",
                                        "description": "The access type of whiteboard share link. \n* Enum: \n* `0` - `Restricted`, Only people added can open with this link \n* `1` - `Same account` \n* `2` - Anyone \n* `3` - SAME_ACCOUNT_EXTERNAL_AUTH \n",
                                        "example": 0,
                                        "default": 0,
                                        "enum": [
                                          0,
                                          1,
                                          2,
                                          3
                                        ],
                                        "x-enum-descriptions": [
                                          "Restricted. Only people added can open with this link",
                                          "Same account",
                                          "Anyone",
                                          "Same account external auth"
                                        ]
                                      },
                                      "share_role": {
                                        "type": "integer",
                                        "description": "The access level of whiteboard share link. This field need to used with `share_scope`. If `share_scope` set `0` - `Restricted`, this field is null. If `share_scope` set `1` - `Same account` or `2` - `Anyone`, this filed can be `2` - `Editor`, `3` - `Commenter` or `4` - `Viewer`. \n* Enum: \n* `2` - `Editor`\n* `3` - `Commenter` \n* `4` - Viewer \n",
                                        "example": 2,
                                        "enum": [
                                          2,
                                          3,
                                          4
                                        ],
                                        "x-enum-descriptions": [
                                          "Editor",
                                          "Commenter",
                                          "Viewer"
                                        ]
                                      }
                                    },
                                    "description": "The access level, access type of current whiteboard share link."
                                  },
                                  "advanced_share_setting": {
                                    "type": "object",
                                    "properties": {
                                      "can_share": {
                                        "type": "boolean",
                                        "description": "Indicate whether users can share at or below their permission level or not. \n\n`true`: Users can share at or below their permission level.   \n \n\n`false`: Users cannot share at or below their permission level.",
                                        "example": true
                                      },
                                      "share_to_external": {
                                        "type": "boolean",
                                        "description": "Indicate whether internal users can share with external users or not. \n\n`true`: Internal users can share with external users.   \n \n\n`false`: Internal users cannot share with external users.",
                                        "example": true
                                      },
                                      "external_can_share": {
                                        "type": "boolean",
                                        "description": "Indicate whether external user can share the whiteboard to others or not. \n\n`true`: External user can share the whiteboard to others.   \n \n\n`false`:External user cannot share the whiteboard to others.",
                                        "example": true
                                      }
                                    },
                                    "description": "Whiteboard advanced share setting."
                                  },
                                  "project_id": {
                                    "type": "string",
                                    "description": "qiMWddmtRxqwSFrK22lJPg",
                                    "example": "The Project ID"
                                  },
                                  "created_by_display_name": {
                                    "type": "string",
                                    "description": "The creator display name of the whiteboard.",
                                    "example": "Jack"
                                  }
                                },
                                "description": "Whiteboard object"
                              }
                            ]
                          }
                        },
                        "next_page_token": {
                          "type": "string",
                          "description": "The next page token paginates through a large set of results. A next page token is returned when the available results exceed the current page size. The expiration period for this token is 15 minutes.",
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
                  ]
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard:read:admin",
              "whiteboard:read",
              "whiteboard:read:list_whiteboards:admin",
              "whiteboard:read:list_whiteboards"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Read"
          ],
          "x-macro-scopes": [
            "whiteboard:read:admin",
            "whiteboard:read"
          ],
          "x-granular-scopes": [
            "whiteboard:read:list_whiteboards:admin",
            "whiteboard:read:list_whiteboards"
          ]
        }
      },
      "post": {
        "tags": [
          "Document"
        ],
        "summary": "Create a new whiteboard",
        "description": "Creates a new whiteboard for a user.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:whiteboard`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "newWhiteboardCreate",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "name": {
                    "type": "string",
                    "description": "The whiteboard name.",
                    "example": "Q3 Plan",
                    "default": "Untitled"
                  }
                },
                "description": "Create a new whiteboard request body."
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "**HTTP Status Code:** `201`   \n \nWhiteboard created successfully.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "whiteboard_id": {
                      "type": "string",
                      "description": "The whiteboard ID.",
                      "example": "48Z4ppMfQJWONeVWFCcVzA"
                    },
                    "whiteboard_name": {
                      "type": "string",
                      "description": "The whiteboard name.",
                      "example": "Q3 Plan",
                      "default": "Untitled"
                    },
                    "locked": {
                      "type": "boolean",
                      "description": "Indicates whether the whiteboard is locked or not.\n\n`true`: The whiteboard is locked.   \n \n\n`false`: The whiteboard is unlocked.",
                      "example": false,
                      "default": false
                    },
                    "trashed": {
                      "type": "boolean",
                      "description": "Indicates whether the whiteboard is moved to trash or not.\n\n`true`: The whiteboard is moved to trash.   \n \n\n`false`: The whiteboard was not moved to trash.",
                      "example": false,
                      "default": false
                    },
                    "readonly": {
                      "type": "boolean",
                      "description": "Indicates whether the whiteboard is editable or not. If a whiteboard is locked, this field will set `true`. \n\n`true`: The whiteboard is editable.   \n \n\n`false`: The whiteboard is read only.",
                      "example": false,
                      "default": false
                    },
                    "non_cloud": {
                      "type": "boolean",
                      "description": "Indicates whether the whiteboard is allowed to be saved to cloud or not.\n\n`true`: The whiteboard is a saved to cloud.   \n \n\n`false`: The whiteboard was not saved to cloud.",
                      "example": false,
                      "default": false
                    },
                    "created_date": {
                      "type": "string",
                      "description": "The whiteboard create date-time in UTC/GMT. Example: &quot;`2020-03-31T12:02:00Z`&quot;",
                      "format": "date-time",
                      "example": "2022-03-25T07:29:29Z"
                    },
                    "modified_date": {
                      "type": "string",
                      "description": "The whiteboard update date-time in UTC/GMT. Example: &quot;2020-03-31T12:02:00Z&quot;",
                      "format": "date-time",
                      "example": "2022-03-25T07:29:29Z"
                    },
                    "share_link_setting": {
                      "type": "object",
                      "properties": {
                        "share_scope": {
                          "type": "integer",
                          "description": "The access type of whiteboard share link. \n* Enum: \n* `0` - `Restricted`, Only people added can open with this link \n* `1` - `Same account` \n* `2` - Anyone \n* `3` - SAME_ACCOUNT_EXTERNAL_AUTH \n",
                          "example": 0,
                          "default": 0,
                          "enum": [
                            0,
                            1,
                            2,
                            3
                          ],
                          "x-enum-descriptions": [
                            "Restricted. Only people added can open with this link",
                            "Same account",
                            "Anyone",
                            "Same account external auth"
                          ]
                        },
                        "share_role": {
                          "type": "integer",
                          "description": "The access level of whiteboard share link. This field need to used with `share_scope`. If `share_scope` set `0` - `Restricted`, this field is null. If `share_scope` set `1` - `Same account` or `2` - `Anyone`, this filed can be `2` - `Editor`, `3` - `Commenter` or `4` - `Viewer`. \n* Enum: \n* `2` - `Editor`\n* `3` - `Commenter` \n* `4` - Viewer \n",
                          "example": 2,
                          "enum": [
                            2,
                            3,
                            4
                          ],
                          "x-enum-descriptions": [
                            "Editor",
                            "Commenter",
                            "Viewer"
                          ]
                        }
                      },
                      "description": "The access level, access type of current whiteboard share link."
                    },
                    "advanced_share_setting": {
                      "type": "object",
                      "properties": {
                        "can_share": {
                          "type": "boolean",
                          "description": "Indicate whether users can share at or below their permission level or not. \n\n`true`: Users can share at or below their permission level.   \n \n\n`false`: Users cannot share at or below their permission level.",
                          "example": true
                        },
                        "share_to_external": {
                          "type": "boolean",
                          "description": "Indicate whether internal users can share with external users or not. \n\n`true`: Internal users can share with external users.   \n \n\n`false`: Internal users cannot share with external users.",
                          "example": true
                        },
                        "external_can_share": {
                          "type": "boolean",
                          "description": "Indicate whether external user can share the whiteboard to others or not. \n\n`true`: External user can share the whiteboard to others.   \n \n\n`false`:External user cannot share the whiteboard to others.",
                          "example": true
                        }
                      },
                      "description": "Whiteboard advanced share setting."
                    }
                  },
                  "description": "The whiteboard object."
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard:write",
              "whiteboard:write:whiteboard"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [],
          "x-macro-scopes": [
            "whiteboard:write"
          ],
          "x-granular-scopes": [
            "whiteboard:write:whiteboard"
          ]
        }
      }
    },
    "/whiteboards/{whiteboardId}": {
      "get": {
        "tags": [
          "Document"
        ],
        "summary": "Get a whiteboard",
        "description": "Returns a whiteboard's information.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:admin`,`whiteboard:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:whiteboard:admin`,`whiteboard:read:whiteboard`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "GetAWhiteboard",
        "parameters": [
          {
            "name": "whiteboardId",
            "in": "path",
            "description": "The whiteboard's ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "HY8-AC_HSou4SXY1S-OmOg"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "**HTTP Status Code:** `200`   \n \nWhiteboard object returned.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "whiteboard_id": {
                      "type": "string",
                      "description": "The whiteboard ID.",
                      "example": "48Z4ppMfQJWONeVWFCcVzA"
                    },
                    "whiteboard_name": {
                      "type": "string",
                      "description": "The whiteboard name.",
                      "example": "Q3 Plan",
                      "default": "Untitled"
                    },
                    "locked": {
                      "type": "boolean",
                      "description": "Indicates whether the whiteboard is locked or not.\n\n`true`-  The whiteboard is locked.   \n \n\n`false` - The whiteboard is unlocked.",
                      "example": false,
                      "default": false
                    },
                    "trashed": {
                      "type": "boolean",
                      "description": "Indicates whether the whiteboard is moved to trash or not.\n\n`true` - The whiteboard is moved to trash.   \n \n\n`false` - The whiteboard isn't moved to trash.",
                      "example": false,
                      "default": false
                    },
                    "readonly": {
                      "type": "boolean",
                      "description": "Indicates whether the whiteboard is editable or not. If a whiteboard is locked, this field will be set to `true`. \n\n`true` - The whiteboard is editable.   \n \n\n`false` - The whiteboard is read only.",
                      "example": false,
                      "default": false
                    },
                    "non_cloud": {
                      "type": "boolean",
                      "description": "Indicates whether the whiteboard is allowed to be saved to cloud or not.\n\n`true` - The whiteboard is a saved to cloud.   \n \n\n`false` - The whiteboard isn't saved to cloud.",
                      "example": false,
                      "default": false
                    },
                    "created_date": {
                      "type": "string",
                      "description": "The whiteboard creation date and time in UTC/GMT. Example: `2020-03-31T12:02:00Z`",
                      "format": "date-time",
                      "example": "2022-03-25T07:29:29Z"
                    },
                    "modified_date": {
                      "type": "string",
                      "description": "The whiteboard update date and time in UTC/GMT. Example: `2020-03-31T12:02:00Z`",
                      "format": "date-time",
                      "example": "2022-03-25T07:29:29Z"
                    },
                    "share_link_setting": {
                      "type": "object",
                      "properties": {
                        "share_scope": {
                          "type": "integer",
                          "description": "The access type of whiteboard sharing link. \n* Enum: \n* `0` - `Restricted`. Only people added can open with this link. \n* `1` - `Same account` \n* `2` - Anyone \n* `3` - `SAME_ACCOUNT_EXTERNAL_AUTH` \n",
                          "example": 0,
                          "default": 0,
                          "enum": [
                            0,
                            1,
                            2,
                            3
                          ],
                          "x-enum-descriptions": [
                            "Restricted. Only people added can open with this link",
                            "Same account",
                            "Anyone",
                            "Same account external auth"
                          ]
                        },
                        "share_role": {
                          "type": "integer",
                          "description": "The access level of whiteboard share link. This field need to used with `share_scope`. If `share_scope` set `0` - `Restricted`. This field is null. If `share_scope` set `1` - `Same account` or `2` - `Anyone`, this field can be `2` - `Editor`, `3` - `Commenter` or `4` - `Viewer`. \n* Enum: \n* `2` - `Editor`\n* `3` - `Commenter` \n* `4` - Viewer \n",
                          "example": 2,
                          "enum": [
                            2,
                            3,
                            4
                          ],
                          "x-enum-descriptions": [
                            "Editor",
                            "Commenter",
                            "Viewer"
                          ]
                        }
                      },
                      "description": "The access level, access type of current whiteboard share link."
                    },
                    "advanced_share_setting": {
                      "type": "object",
                      "properties": {
                        "can_share": {
                          "type": "boolean",
                          "description": "Indicate whether users can share at or below their permission level or not. \n\n`true`: Users can share at or below their permission level.   \n \n\n`false`: Users cannot share at or below their permission level.",
                          "example": true
                        },
                        "share_to_external": {
                          "type": "boolean",
                          "description": "Indicate whether internal users can share with external users or not. \n\n`true`: Internal users can share with external users.   \n \n\n`false`: Internal users cannot share with external users.",
                          "example": true
                        },
                        "external_can_share": {
                          "type": "boolean",
                          "description": "Indicate whether external user can share the whiteboard to others or not. \n\n`true`: External user can share the whiteboard to others.   \n \n\n`false`:External user cannot share the whiteboard to others.",
                          "example": true
                        }
                      },
                      "description": "The whiteboard advanced share setting."
                    },
                    "share_link": {
                      "type": "string",
                      "description": "The whiteboard share link.",
                      "example": "https://zoom.us/wb/doc/T-E0zR9NQWSDuvXd-Crt9A"
                    },
                    "preview_link": {
                      "type": "string",
                      "description": "The whiteboard preview link. This link's validity period is `2 hours`, and the maximum number of downloads is `99 times`. Only those who have the `whiteboard:read:whiteboard` scope or are collaborators of the whiteboard will see this field returned.",
                      "example": "https://a01file.zoom.us/file/xxxxx/attach/preview/192x108?jwt=xxx"
                    },
                    "created_by_display_name": {
                      "type": "string",
                      "description": "The creator display name of the whiteboard.",
                      "example": "Tom"
                    },
                    "modified_by_display_name": {
                      "type": "string",
                      "description": "The latest modify user display name of the whiteboard.",
                      "example": "Jack"
                    },
                    "embedded_content_types": {
                      "type": "array",
                      "description": "The embedded content type list.",
                      "items": {
                        "maximum": 2,
                        "minimum": 0,
                        "type": "integer",
                        "description": "The type of embeded content in whiteboard. it's an enum contains `0`, `1`, `2`.\nThe description of the enums:\n- **0** - **IMAGE**.\n- **1** - **DOCUMENT**, this would include PDFs, PPTX, DOCX, GOOGLE DOCS, SHEETS and SLIDES.\n- **2** - **CARD**, this would include Jira, Asana and Azure DevOps.",
                        "format": "int32",
                        "example": 0,
                        "default": 0,
                        "enum": [
                          0,
                          1,
                          2
                        ],
                        "x-enum-descriptions": [
                          "IMAGE",
                          "DOCUMENT",
                          "CARD"
                        ]
                      }
                    },
                    "project_id": {
                      "type": "string",
                      "description": "qiMWddmtRxqwSFrK22lJPg",
                      "example": "The Project ID"
                    }
                  },
                  "description": "Whiteboard object"
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n Permission is needed to access this document. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard:read:admin",
              "whiteboard:read",
              "whiteboard:read:whiteboard:admin",
              "whiteboard:read:whiteboard"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Read"
          ],
          "x-macro-scopes": [
            "whiteboard:read:admin",
            "whiteboard:read"
          ],
          "x-granular-scopes": [
            "whiteboard:read:whiteboard:admin",
            "whiteboard:read:whiteboard"
          ]
        }
      },
      "put": {
        "tags": [
          "Document"
        ],
        "summary": "Update whiteboard basic information",
        "description": "Use this API to rename a whiteboard, update whiteboard status (locked or unlocked). \n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:update:whiteboard:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "UpdateAWhiteboardMetadata",
        "parameters": [
          {
            "name": "whiteboardId",
            "in": "path",
            "description": "The ID of the whiteboard.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "HY8-AC_HSou4SXY1S-OmOg"
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
                    "description": "The whiteboard name.",
                    "example": "Q3 Plan",
                    "default": "Untitled"
                  },
                  "locked": {
                    "type": "boolean",
                    "description": "Indicates whether the whiteboard is locked or not.\n\n`true`: The whiteboard is locked.   \n \n\n`false`: The whiteboard is unlocked.",
                    "example": false,
                    "default": false
                  }
                },
                "description": "Update a whiteboard request body"
              }
            }
          }
        },
        "responses": {
          "204": {
            "description": "**HTTP Status Code**: `204`   \n \nWhiteboard updated successfully."
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111800` <br>\n Permission is needed to access this document. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard:write:admin",
              "whiteboard:update:whiteboard:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:update:whiteboard:admin"
          ]
        }
      },
      "delete": {
        "tags": [
          "Document"
        ],
        "summary": "Delete a whiteboard",
        "description": "Deletes a whiteboard.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:admin`,`whiteboard:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:delete:whiteboard:admin`,`whiteboard:delete:whiteboard`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "DeleteAWhiteboard",
        "parameters": [
          {
            "name": "whiteboardId",
            "in": "path",
            "description": "The ID of the whiteboard.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "HY8-AC_HSou4SXY1S-OmOg"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "**HTTP Status Code**: `204`   \n \nWhiteboard Deleted."
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this document. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard:write:admin",
              "whiteboard:write",
              "whiteboard:delete:whiteboard:admin",
              "whiteboard:delete:whiteboard"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard:write:admin",
            "whiteboard:write"
          ],
          "x-granular-scopes": [
            "whiteboard:delete:whiteboard:admin",
            "whiteboard:delete:whiteboard"
          ]
        }
      }
    },
    "/whiteboards/export": {
      "post": {
        "tags": [
          "Export"
        ],
        "summary": "Create whiteboard export",
        "description": " Generates the export for the provided whiteboards. This process will generate a PDF of whiteboard projects along with an audit log, bundling them into a ZIP file.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_export:write:admin`,`whiteboard_export:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:export`,`whiteboard:write:export:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "Createwhiteboardsexport",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "whiteboard_ids"
                ],
                "type": "object",
                "properties": {
                  "whiteboard_ids": {
                    "maxItems": 50,
                    "minItems": 1,
                    "uniqueItems": true,
                    "type": "array",
                    "description": "The list of whiteboard IDs. The list size must be **less than** 50, if more than 50, the API will **intercept** the first 50 IDs.",
                    "items": {
                      "type": "string",
                      "description": "The ID of the whiteboard.",
                      "example": "HY8-AC_HSou4SXY1S-OmOg"
                    }
                  }
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "description": "**HTTP Status Code:** `200`    Successfully sent a request to generate whiteboard export",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "task_id"
                  ],
                  "type": "object",
                  "properties": {
                    "task_id": {
                      "type": "string",
                      "description": "The unique identifier of the generated file, which expires is **30 days**. Use this task_id to query the progress of file generation (For more details, please refer to\uff1a[**Get whiteboard export generation status**](https://developers.zoom.us/docs/zoom-whiteboard/apis/#operation/Getwhiteboardexportdatagenerationstatus). Once the file is ready, you can download it using the user's OAuth token. For example: `https://{base_domain}/v2/whiteboards/export/task/{taskId} --header 'Authorization: Bearer {{OAuth-access-token}}` .",
                      "example": "f7f3cdd37bf74550afe7cdf79128a574"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this document. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `34002` <br>\n The whiteboard {whiteboard_id} not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_export:write:admin",
              "whiteboard_export:write",
              "whiteboard:write:export",
              "whiteboard:write:export:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardContent:Read",
            "WhiteboardManagement:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_export:write:admin",
            "whiteboard_export:write"
          ],
          "x-granular-scopes": [
            "whiteboard:write:export",
            "whiteboard:write:export:admin"
          ]
        }
      }
    },
    "/whiteboards/export/task/{taskId}": {
      "get": {
        "tags": [
          "Export"
        ],
        "summary": "Download whiteboard export",
        "description": "Use this API to download the whiteboard export.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_content:read:admin`,`whiteboard_content:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:export`,`whiteboard:read:export:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "Downloadwhiteboardexport",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the whiteboards file. The `task_id` generated by [**Create whiteboard export**](https://developers.zoom.us/docs/zoom-whiteboard/apis/#operation/Createwhiteboardsexport) interface.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "f7f3cdd37bf74550afe7cdf79128a574"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Returns a file stream, with the file type of ZIP."
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `34002` <br>\n The task {task_id} not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_content:read:admin",
              "whiteboard_content:read",
              "whiteboard:read:export",
              "whiteboard:read:export:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardContent:Read",
            "WhiteboardManagement:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_content:read:admin",
            "whiteboard_content:read"
          ],
          "x-granular-scopes": [
            "whiteboard:read:export",
            "whiteboard:read:export:admin"
          ]
        }
      }
    },
    "/whiteboards/export/task/{taskId}/status": {
      "get": {
        "tags": [
          "Export"
        ],
        "summary": "Get whiteboard export generation status",
        "description": "Use this API to query the status of the whiteboard export generation. Before using this API, confirm that you have invoked the interface for generating the whiteboard file  [**Create whiteboard export**](https://developers.zoom.us/docs/zoom-whiteboard/apis/#operation/Createwhiteboardsexport) and obtained the `task_id`.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_content:read:admin`,`whiteboard_content:read`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:export`,`whiteboard:read:export:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "Getwhiteboardexportdatagenerationstatus",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique identifier of the whiteboard export. ",
            "required": true,
            "schema": {
              "type": "string",
              "example": "adcc655d3a3549e692959737fe2a7a0d"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Whiteboard export generation status.",
            "content": {
              "application/json": {
                "schema": {
                  "required": [
                    "status",
                    "total"
                  ],
                  "type": "object",
                  "properties": {
                    "processed": {
                      "maxItems": 50,
                      "minItems": 0,
                      "type": "array",
                      "description": "List of processed whiteboard IDs.",
                      "items": {
                        "type": "string",
                        "description": "The whiteboard ID.",
                        "example": "48Z4ppMfQJWONeVWFCcVzA"
                      }
                    },
                    "status": {
                      "type": "string",
                      "description": "Whiteboard export generation status. \n* Enum:\n* `processing` - Generating whiteboard export\n* `successed` - Whiteboard export generated successfully(as long as one is successful, it counts as a success)\n* `failed` - Whiteboard export generation failed",
                      "example": "processing",
                      "enum": [
                        "processing",
                        "successed",
                        "failed"
                      ],
                      "x-enum-descriptions": [
                        "Generating whiteboard file",
                        "Whiteboard file generated successfully.",
                        "Whiteboard file generation failed"
                      ]
                    },
                    "total": {
                      "type": "integer",
                      "description": "Total number of whiteboards requiring export generation.",
                      "example": 3
                    },
                    "failed": {
                      "maxItems": 50,
                      "minItems": 0,
                      "type": "array",
                      "description": "List of failed whiteboard IDs.",
                      "items": {
                        "type": "string",
                        "description": "The whiteboard ID.",
                        "example": "UyKtcVj5REWMFT2qd-4qfg"
                      }
                    },
                    "processing": {
                      "maxItems": 50,
                      "minItems": 0,
                      "type": "array",
                      "description": "List of whiteboards being processed.",
                      "items": {
                        "type": "string",
                        "description": "The whiteboard ID.",
                        "example": "r2Qr1q4hSxu03MG3hk03uA"
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
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `34002` <br>\n The task {task_id} not found. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_content:read:admin",
              "whiteboard_content:read",
              "whiteboard:read:export",
              "whiteboard:read:export:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardContent:Read",
            "WhiteboardManagement:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_content:read:admin",
            "whiteboard_content:read"
          ],
          "x-granular-scopes": [
            "whiteboard:read:export",
            "whiteboard:read:export:admin"
          ]
        }
      }
    },
    "/whiteboards/files": {
      "post": {
        "tags": [
          "File"
        ],
        "summary": "Upload file for whiteboard import",
        "description": "Uploads a file to whiteboard.\n**Notes**:\n- Base URL: `https://fileapi.zoom.us/v2`.\n- Rate limit: 20 requests per second per user or 2000 requests per second per IP address.\n- The caller must support HTTP 30x redirects.\n- The caller must retain the Authorization header when redirected to a different hostname.\n- Supported file formats: `.pdf`, `.vsdx`.\n- For an account-level [OAuth](https://developers.zoom.us/docs/integrations/create/) app, this API can only be used on behalf of a user who is assigned with a [role](https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0064983#h_80abad9b-86a8-492c-8a3b-5b0141a8c0a9) with **Edit** permission for Whiteboard Management.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_file:write:admin`,`whiteboard_file:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:file`,`whiteboard:write:file:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `HEAVY`\n\n**Not supported in Gov cluster** ",
        "operationId": "Uploadfileforwhiteboardimport",
        "requestBody": {
          "content": {
            "multipart/form-data": {
              "schema": {
                "required": [
                  "file"
                ],
                "type": "object",
                "properties": {
                  "file": {
                    "type": "string",
                    "description": "The upload file is in binary format. You can only send a maximum of 100 MB.",
                    "format": "binary",
                    "example": "Vm0wd2QyUXlVWGxWV0d4V1YwZDRWMVl3WkRSV01WbDNXa1JTVjAxV2JETlhhMUpUVmpBeFYySkVUbGhoTVVwVVZtcEJlRll5U2tWVWJHaG9UVlZ3VlZadGNFSmxSbGw1VTJ0V1ZXSkhhRzlVVmxaM1ZsWmFkR05GU214U2JHdzFWVEowVjFaWFNraGhSemxWVm14YU0xWnNXbUZrUjA1R1UyMTRVMkpIZHpGV1ZFb3dWakZhV0ZOcmFHaFNlbXhXVm0xNFlVMHhXbk5YYlVaclVqQTFSMVV5TVRSVk1rcElaSHBHVjFaRmIzZFdha1poVjBaT2NtRkhhRk5sYlhoWFZtMHhORmxWTUhoWGJrNVlZbFZhY2xWcVFURlNNVlY1VFZSU1ZrMXJjRWxhU0hCSFZqRmFSbUl6WkZkaGExcG9WakJhVDJOdFJraGhSazVzWWxob1dGWnRNSGhPUm14V1RVaG9XR0pyTlZsWmJGWmhZMVphZEdSSFJrNVNiRm93V2xWYVQxWlhTbFpqUldSYVRVWmFNMVpxU2t0V1ZrcFpXa1p3VjFKV2NIbFdWRUpoVkRKT2MyTkZhR3BTYkVwVVZteG9RMWRzV1hoWGJFNVRUV3hHTlZWdGRHdGhiRXAwVld4c1dtSkdXbWhaTW5oWFl6RldjbHBHWkdsU2JrSmFWMnhXWVZReFdsaFRiRnBZVmtWd1YxbHJXa3RUUmxweFUydGFiRlpzV2xwWGExcDNZa2RGZWxGcmJGaFhTRUpJVmtSS1UxWXhXblZWYldoVFlYcFdlbGRYZUc5aU1XUkhWMjVTVGxOSFVuTlZha0p6VGtaVmVXUkhkRmhTTUhCSlZsZDRjMWR0U2tkWGJXaGFUVlp3YUZwRlpGTlRSa3B5VGxaT2FWSnRPVE5XTW5oWFlqSkZlRmRZWkU1V1ZscFVXVlJHZDFZeGJISlhhM1JVVW14d2VGVnRNVWRWTWtwV1lrUmFXR0V4Y0hKWlZXUkdaVWRPU0U5V1pHaGhNSEJ2Vm10U1MxUXlVa2RUYmtwb1VqSm9WRmxZY0ZkbGJHUllaVWM1YVUxWFVraFdNalZUVkd4a1NGVnNXbFZXYkhCSVZGUkdVMVp0UmtoUFYyaHBVbGhDTmxkVVFtRmpNV1IwVTJ0a1dHSlhhR0ZVVmxwM1lVWndSbHBHVGxSU2EzQjVWR3hhVDJGV1NuUlBWRTVYVFc1b1dGbHFTa1psUm1SWldrVTFWMVpzY0ZWWFZsSkhaREZaZUdKSVNsaGhNMUpWVlcxNGQyVkdWblJOVldSV1RXdHdWMWxyVW1GWFIwVjRZMGhLV2xaWFVrZGFWV1JQVTBVNVYxcEhhR2hOU0VKMlZtMTBVMU14VVhsVmEyUlVZbXR3YjFWcVNtOVdSbXhaWTBaa2JHSkhVbGxhVldNMVlWVXhXRlZyYUZkTmFsWlVWa2Q0VDFOSFJrZFJiRnBwVmtWVmQxWnRjRWRWTVZwMFVtdG9VRlp0YUZSVVZXaERUbFphU0dWSFJtcE5WMUl3VlRKMGExZEhTbGhoUjBaVlZucFdkbFl3V25KbFJtUnlXa1prVjJFelFqWldhMlI2VFZaWmQwMVdXbWxsYTFwWVdXeG9RMVJHVW5KWGJFcHNVbTFTZWxsVldsTmhWa3AxVVd4d1YySllVbGhhUkVaYVpVZEtTVk5zYUdoTk1VcFdWbGN4TkdReVZrZFdibEpPVmxkU1YxUlhkSGRXTVd4eVZXMUdXRkl3VmpSWk1HaExWMnhhV0ZWclpHRldWMUpRVlRCVk5WWXhjRWhoUjJoT1UwVktNbFp0TVRCVk1VMTRWVmhzVm1FeVVsVlpiWFIzWWpGV2NWTnRPVmRTYlhoYVdUQmFhMkpIU2toVmJHeGhWbGROTVZsV1ZYaFhSbFp5WVVaa1RtRnNXbFZXYTJRMFlURk9SMVp1VGxoaVJscFlXV3RvUTFOV1drZFZhMlJXVFZad01GVnRkRzlWUmxsNVlVaENWbUpIYUVSVWJYaHJWbFpHZEZKdGNFNVdNVWwzVmxSS01HRXhaRWhUYkdob1VqQmFWbFp1Y0Zka2JGbDNWMjVLYkZKdFVubFhhMXByVmpKRmVsRnFXbGRoTWxJMlZGWmFhMUl4VG5OWGJYQlRWak5vV1ZkWGVHOVJNVkpIVlc1S1dHSkZjSE5WYlRGVFpXeHNWbGRzVG1oU1ZFWjZWVEkxYjFZeFdqWlJhbEpWWVRKU1NGVnFSbUZYVm5CSVlVWk9WMVpHV2xkV2JHTjRUa2RSZVZaclpGZGliRXBQVm14a1UxWXhVbGhrU0dSWFRWZDRlVlpYTVVkWFJrbDNWbXBTV2sxSGFFeFdNbmhoVjBaV2NscEhSbGRXTVVwUlZsUkNWazVXV1hoalJXaG9VakpvVDFVd1ZrdE5iRnAwVFZSQ1ZrMVZNVFJXVm1oelZtMUZlVlZzVmxwaVdGSXpXV3BHVjJOV1RuUlBWbVJUWWxob1lWZFVRbUZoTWtwSVUydG9WbUpIZUdoV2JHUk9UVlpzVjFaWWFGaFNiRnA1V1ZWYWExUnRSbk5YYkZaWFlUSlJNRlpFUms5VFJrcHlXa1pLYVZKdVFuZFdiWFJYVm0xUmVGZHVVbXBTVjFKWFZGWmFkMDFHVm5Sa1J6bFdVbXh3TUZsVldsTldWbHBZWVVWU1ZXSkdjR2hWTUdSWFUwWktkR05GTlZkTlZXd3pWbXhTUzAxSFJYaGFSV2hVWWtkb2IxVnFRbUZXYkZwMVkwWmthMkpHYkROV01qVkxZa1pLZEZWdWJGaGhNWEJ5Vm1wS1JtVnNSbkZYYkdSb1RXeEpNbFpHV21GaE1VbDRWRzVPV0dKWVFsUldhazV2VjFaa1dHVkhPVkpOVmtwSVZsYzFTMWRIU2taalNFNVdZbFJHVkZwWGVITldiR1J6Vkcxb1UxWkZXalpXVkVreFlqRlplRmRxV2xOV1JVcG9WV3RXWVdOc1ZuRlRhM1JVVm14S01GbFZXazloUjFaelYyeEdWMkV4Y0doWFZtUlNaVlphY2xwR1pGaFNNMmg1VmxkMFYxTXhaRWRWYkdSWVltMVNjMVp0TVRCTk1WbDVUbGQwV0ZKcmJETldiWEJUVjJzeFIxTnNRbGRoYTNCSVdUSjRhMk50VmtkYVIyaG9UVEJLVWxac1VrZGhNVTE0VTFob2FsSlhVbWhWYlhNeFYwWlpkMVpyZEU1aVJuQXdWRlpTUTFack1WWk5WRkpYVm0xb2VsWnNXbXRUUjFaSFYyeHdWMUpXYjNwWFYzQkhWakpPVjFWdVNsVmlSMUpVV1d4b2IxbFdaRlZSYlVab1RWZFNTRll5TlU5aGJFcDBaVWRHVlZaV2NGZFVWbHB6VmpGYVdXRkhjR2xXV0VKSFZteGtOR0V4VW5SU2JrNVlZa1phV0ZsVVNsSk5SbXhWVW1zNVUwMVdjREZXUjNodllWWktjMk5HYkZoV00xSjJWVlJHYTFZeFpISmhSM1JUVFVad1dWZFhlR0ZaVms1WFZtNVNhMUo2YkZkVmJYaHpUbFpzVm1GRlRsZGlWWEJKV1ZWV1UxWlhTbGxoU0VwWFlrWndTRnBGV2t0a1IwNUdUbFprVGxaWGQzcFdiWGhUVXpBeFNGSllhR0ZTVjJoVldXdGtiMkl4Vm5GUmJVWlhZa1p3TVZrd1dtdGhNa3BIWWtST1YwMXFWa3haYTFwTFpFWldkV0pHYUdoTldFSjVWbTF3UzFKdFZuTlNia1pZWWtkU2IxUlhlRXBOYkZwSFYyMUdXR0pXUmpOVVZscHJWMGRLV0dWSVRscFdSVm96VmpGYWExZEhWa2hQVmtwT1ZsUkZNRlpzWkRSaE1XeFhVMnhrYWxORk5WZFpiRkpIVmtaWmVXVkhkR3BpUm5CV1ZXMTRhMVJzV25WUmFscFlWa1ZLYUZacVJtdFNNV1JaWTBaYWFXRXpRbGxXYlhSWFdWZFdjMWR1UmxOaVIxSnhWRlprVTJWc2JGWmFTRTVYVFZad01WVlhlRWRXTWtWNFkwZG9XRlp0VWxOYVZscGhZMnh3UjFwR2FGTk5NbWcxVm14a01GWXhUWGhhUldSV1lrZFNXVmx0TVZOak1WcDBUVlJDVGsxV2JEVlViR1F3VjBaS2NtSkVUbGRpV0VKVVZqSnplRk5IUmtabFJtUk9ZbTFvYjFacVFtRldNazV6WTBWb1UySkhVbGhVVmxaM1ZXeGFjMXBJWkZSTlZURTBWVEZvYzFVeVJYbGhSemxXWWtaS1dGWXdXbk5rUjFKR1pFVTFhVkp1UVhkV1JscFRVVEZhY2sxV1drNVdSa3BZVm01d1YxWkdXbkZUYTFwc1ZteGFNVlZ0ZUdGaFZrbDRVbGhrVjJKVVJUQlpla3BPWlVkT1JtRkdRbGRpVmtwVlYxZDBWMlF4WkhOWGEyaHNVak5DVUZadGVITk9SbGw1VGxVNWFHSkZjRmxaVlZwelZqSkZlRlpZYUdGU00yaDZWbXBHWVZkWFJrZGhSazVwVW0wNU5GWXhVa05aVjBWNFZXNU9XRmRIZUc5VmExWjNWMFpTVjFkdVpHaFNiRmt5VlcxMGQySkdTbk5UYWtaWFlsaG9URmxXV2t0ak1rNUhZa1prVTJKRmNFbFdXSEJDVFZkTmVGcElTbWhTTTJoVVZGVmFkMkZHV25SalJXUlVUVlZzTkZaWE5VOVhSMHBXVjJ4a1ZtSllhRE5VVlZwaFYwVTFWbFJzWkU1aE0wSktWMVpXVjFVeFduSk5WbVJxVWpKb1lWUldXbmRWUm10NFYyeGthazFYVW5sVWJGcHJWR3hhV0dRemNGZGlXR2hVVlhwQmVGTkdTbGxoUjBaVFZqSm9WbGRYZEd0aU1rbDRWbTVHVW1KVldsaFphMXAzVFZacmQxZHRkR2hOYTNCSVdXdFNUMVl3TVhGV2EzaGFZVEZ3VEZwRldsZGtWMHBIWVVkb1RsZEZTalZXYlRGM1V6RktkRlp1VGxOaWExcFpXV3RrVTJJeFVsaGxSWEJPWWtad1NGWXlNVWRYUjBwWFVtcE9WVlpzY0hKV01HUkxWMGRXU1ZSc2NGZFNWbTk2Vm1wR1lXRXhaRWhXYTJoUVZtdHdUMVp0ZEhkVFZscHpXWHBHVkUxWFVrbFZNblJoWVd4T1JrNVdaRnBpUmtwSVZtdGFkMWRIVmtsVWJHUnBVakZLTmxaclkzaGlNVmw1VWxod1VsZEhhRmhXYlRGT1pVWnNjVkpzY0d4U2JWSmFXVEJrYjFaR1NsbFJiR3hYWWxoU1dGZFdaRmRqTVdSMVVteE9hVkl4U25oV1JscHJWVEpXYzJKR1dtRlRSVFZZVkZaYWQwMVdWbGhsUldSWFRXdFdORmt3Wkc5WFJscDBWV3hPWVZac2NHaFpNbmgzVWpGd1IyRkdUazVOYldjeFZtMTRhMDFHV1hoVVdHaGhVbGRTVjFsclpGTlhWbXgwVFZaT2FrMVdjREJVVmxKRFZHc3hWMkpFVmxWaVIxRjNWakJhUzJOdFNrVlViR1JwVjBWS1ZWWnFTbnBsUms1SVVtdGFiRkp0VWs5WmJURnZZakZhY1ZGdFJsZE5helY2V1RCV2IxVXlTa2hWYkdoVlZteGFNMVpYZUdGak1YQkpXa1pTVGxaVVJUQldWRVp2WXpGVmVWSlliR2hTZW14WFdXdGFTMWRHV1hkWGJrNXJVakZLUjFwRlZURmhWa3AxVVZoa1YxSnNjRlJWVkVaaFkyc3hWMWRyTlZkU2EzQlpWbTB3ZUdJeVZuTlhiazVZWWxoU1ZWVnFSbUZUUmxsNVpVaGtWMDFWY0ZoWmFrNTNWMFphYzFkdGFGZGhhM0JRVm1wR1UyUldWbk5SYkdScFZtdHdWbFl4WkRCV01sRjRXa2hPV0dFeWFITlZhazVEVlVaYWRHVklUazlTYkd3MVZHeFZOV0ZIU2taalJteGFWbFp3ZWxacVNrWmxSbHBaWVVkR1UwMHlhRFpXYlhCSFdWWmtXRkpyYUdwU01uaFlWakJXUzFOR1duUmxSM1JQVWpCV05GWlhOVTlYUm1SSVpVYzVWbUV4V2pOV01GcHpWMGRTUm1SSGNHbFNiR3Q1VmxjeE1FMUhSblJTYWxwWFlrZG9XVmxVUm5kaFJteFhWMnQwYWsxck5VaFphMXB2VmpBd2VXRkhPVmRXZWtJMFZsY3hWMUl4Y0VsV2JGcHBWMFZLVlZadGNFZFRNV1JIVjJ0a1dHSlZXbkpVVlZKSFUwWmFkRTVXVG1oTlZXOHlXV3RhYTFadFNsbGhSRTVWVmxad2VsWnRlR3RqTVZKeldrWmthVk5GU21GV01WcFhWakZWZUZkc2FGUmhNbEp4VldwS2IxWXhVbFphUms1WVVteFdNMVl5ZERCaE1VbDNUbFZrVldKR2NISldSM2hoVjBkUmVtTkdaR2xYUjJoVlZsaHdRbVZHVGtkVWJHeHBVbXMxYjFSWGVFdFdiR1JZVFZod1RsWnNjRmhaYTJoTFdWWktObUpHYUZwaE1YQXpXbGQ0V21WVk5WaGtSbFpvWld0YVdWZFVRbTlqTVZsM1RWaEdVMkV5YUdGV2FrNXZZVVpyZVdONlJsaFdNSEJJV1ZWa2IxUnNaRVpUYkVwWFRWWndhRmRXV2s1bFZsSjFWV3hXYUUxV2NGcFhWM1JyVlRKT1IxWnVSbEpXUlVwUFZXMHhVMWRzYTNkV2JYUlhUV3R3V0ZWdGNFOVdWbHB6VjI1d1dsWldjRXhaTW5NeFYwVTVWMk5IYUU1aVYyaEdWbTF3UjFsWFJYaFhXR2hxVWxaYVUxbHNhRk5qUmxwMFpFWndUbEpzY0hoVk1uQlRWakF4VjFacVZsWk5ibEp5V1ZkNFQxSnJOVmRhUm5CcFVtdHdTVlp0ZEdGVk1WbDRXa2hPV0dKWWFGaFVWVkpTWlZaYVdFMVVVbWxOVmxZMVZXeG9kMVZ0U2xaWGJHaFhZbFJHVDFwVlduTmpWa3B6WTBkNFUySldTbUZYVkVKcllqRmFSMU5zYkZKaVIxSlpWbTE0UzFSR1dsVlNhemxUVFZad01WVnRlRTloVmxwWFkwWndWMkpVUVhoVlZFWlBWakpLUjFkc2FHaGxiWGhaVmtaYVlXUXhUa2RYV0d4c1VqTlNXRlJYZEhkVFZscElZMFpPVjFZd1ZqVldWM2hQV1ZaYWMyTkhhRnBOYm1nelZXcEdkMUl5UmtkVWF6Vk9ZbGRqZUZadE1UUmhNbEY0VWxob2FWSnRhRlpaVkVwVFYwWnNkR1ZGZEdwTlZsWXpWMnRhVDJGck1WaGxTR3hYVFdwV1VGWkVSbUZrVmtaeldrWndWMVl4UmpOV2FrSmhVekpPZEZOclZsVmlSMUp3VmpCV1MxWldXbkZUYm5Cc1VtdHNORlpITlU5VmJVcElWV3M1V2xZelVtaFZNRnByWXpGa2RGSnNaRTVoZWxZMlYxUkNWMkl4VlhsVGEyaFdZWHBzVmxadWNFZGtiRlY1WlVad2JGSnNXbmxaTUdRMFZUSkdObEpVUWxkV1JWcDJXV3BLUjJNeFRuTmhSMmhVVWxWd1dGZFhlRk5TTWxKelYyNUtXR0pWV25GVVYzUmhVakZhU0dWR1RsVmlSbkF4VlZab2ExWXhTbk5qUmxKV1ZrVmFhRmt5YzNoV01XUnlUbFprVTJFelFscFdNV1F3WVRKSmVWVnVUbGhYUjFKWldXeG9VMVpXVm5GUmJVWlVVbTE0ZVZZeU5XdGhSbHAwVld0c1dsWlhUVEZXYWtwTFZsWktWVkZzY0d4aE0wSlJWMWh3UzFReFNuUlNhMlJTWWtkU1QxUlZWbmRXYkZsNFdrUkNhVTFWVmpOVWJGWnJWMGRLY21OSFJsVldSWEJVVmxWYVlXUkhWa2xVYXpsVFlrZDNNVlpIZUZaT1YwWklVMnRhYWxKdGVHRldiRnAzWld4YWNWTnJXbXhTYkhCYVdWVmtSMVV3TUhsaFJGcFhZbGhDU0ZkV1dtdFhSa3B5V2tkb1UyRjZWbmRXVnpBeFVXc3hWMWRZYUZoaVIxSmhWbXBDVjA1R1dsaE9WazVXVFd0d2VWUnNXbk5YYlVWNFkwZG9WMDFHY0hwV2JGcFBZekZPY2s1V1RtbFdhM0JhVm0xd1MwMUZNVWhTYmtwT1ZtMVNWVmxYZEdGWFJsWjFZMFZrYTJKR2NGWlZNblF3VlRBeGNrNVZhRnBoTVhCeVZsUktTMUp0VGtaTlZtaFhUVEJLU1ZaclVrZFhiVlpIWTBWc1ZHSlhhRlJVVkVwdlZsWmFSMVp0Um10TmExcElWakkxVTFSc1RraGhSbVJWVm14YWVsUnJXbUZYUlRWV1QxWm9VMDFJUWxoV2FrbzBZVEZhZEZOc2JHaFNSVFZXVm14YWQyVnNXblJsUjNScVRWWktlbGRyV210aFZrNUdVMnh3VjAxV1NrUlpWRVpLWlVaa2MxcEdhR2xoZWxaNFZsZHdTMkl4YkZkalJtaHJVakJhVDFSV1dtRmxiRmw1WlVkMGFFMVZiRE5VYkZaclZsZEtSMk5JU2xkU00yaG9WakJrVW1WdFRrZGFSMnhZVWpKb1ZsWnNhSGRSYlZaSFZHdGtWR0pIZUc5VmFrSmhWa1phY1ZOdE9WZGlSMUpaV2tWa01HRlZNWEppUkZKWFlsUldTRlpYTVV0V2JHUnpZVVp3YUUxWVFYcFdSbFpoWkRGYWRGSnJhR3RTYkZwdldsZDBZVmRXWkZWUmJUbHFUVlpzTTFSV2FFZFZNa1Y1WVVkR1YyRnJOWFpaVlZweVpWVXhWazlXVGxkaE1XOTNWbGN4TkZReFdYZE5WbHBZVjBoQ1dGbHNVa2ROTVZZMlVtczFiRlpzU2pGV1IzaFhZVmRGZWxGdVdsZFdla0kwVmxSR1lWSXhWbk5XYkZKcFZrZDRkMVpHV21Ga01sWlhWMnhvYTFKRlNsZFVWVkpIVjBac2NsVnNUbGROVlc4eVZtMTRiMVl5U2xWU2EzaFdZbGhvVUZsNlNrZFNNV1IwWWtaT2FFMHdTbUZXYlRGM1VqRnNWMkpHWkZSWFIyaHdWV3RhZDFaR2JITmFSRkpWVFZad2VGVnRkREJXUmxwMFZXeHNZVkpYVWtoV2FrRjRWakZrZFdOR2FHaE5WbkF5VjFaV1lWTXlUWGhVYms1WVlrZFNjRlpyVm1GV1ZscDBaRWRHYTAxWFVrbFdiWFJ6VmxkS1NGVnVRbFpoYTFwTVZHMTRZV05zYTNwaFIyeE9WbXhaTUZacVNqQlpWbVJJVW01T2FsSnRhRmhaVkVaaFpWWndWbGR1VG1wV2EzQjZXVEJrTkZVeVNsZFRiVVpZVmtWS2FGZFdaRWRXTVU1WllrZG9VMkpyU2xsWFYzaFRVbXN4UjJORlZsTmlhelZZVkZkek1WTkdXWGxsUnpsb1ZtMVNTRlV5TlhOV01rcFZVbGhrWVZKRmNGaFpla1pyWkZaV2NrNVdhRk5XUmxwYVZtdGFZVll5VFhkT1dFNXBVbXh3VjFsclpGTlhSbEpXVld0a1dGWnNjRmhaVldRd1YwWktjMk5FUWxkV00yaFFWMVphWVdNeVRraGhSbkJPWW0xbmVsWlhjRWRrTVU1SVUydG9hVkpyTlZsVmJGWnlaVVphZEUxVVVtaE5SRVpJVmpGb2IxWldaRWhsUmxaWFRVZFNkbGt3V2xka1JURlZWVzF3VjJKR2NEWldNblJoVkRKR2NrMVdaR3BTUlVwb1ZteGFXbVF4YkhKYVJYUlRUV3MxUmxWWGVGZFdNVnB5WTBac1YyRnJTbWhWTWpGU1pWWlNjbGR0YUZOaWEwcFFWbGN3TVZFd01YTlhia1pVWW01Q2MxVnRjekZUVmxaMFpFZEdhVkpyY0RCV1YzTTFWMjFLVlZKdVdscGhhMXBvV1RGYVIyUkdTbk5hUlRWb1pXeFpNbFl4VWtOV01rbDRWbGhzVkdFeWFGZFpXSEJ6Vm14YWRXTkZaR3RpUmxZMVdsVmFkMkpHU25OWGJteGFUVWRSTUZaVVNrZE9iRXBWVVd4a1YxSldjREpYVjNCTFVqSk5lRlJ1VG1oU2JIQndWbXBLYjFkR1pGaGxSemxWWWxaYVNGWXhhR3RYUjBwSlVXNUtWVlpzY0ROYVZscDNVbXhhVlZKdGFGZGhNMEY0VmxaYWIyRXhaRWRYYTFwVVlrZG9ZVlpyVm1GamJHdzJVbTEwV0ZadVFraFhhMlJ2WVZaS1dWRnNjRmROVjFGM1dWUktTbVZXVm5WVWJHaFlVakZLV2xkWGVHOVZNVnBYVm01R1VtSlZXbFZWYlhoelRsWndWbUZIZEdoU2JIQXdWbGQwYzFkc1dsaFVWRVpYWVd0d1RGWnNXa3RXVmtaelkwZDRhRTB3U2xKV01XaDNVakpSZUZSc1pGVmliRXB5Vlcxek1WUXhVbFpXYm1SWFlrWnNORmRyVWtOWGJGbDRVbXBPVldKR1duSldNR1JMVTBkR1JrOVdaR2hOVm5CTlZtdFNSMVl5VWtoV2EyeFZZWHBzVkZsclpETk5WbHBJWlVaYVQxWXdXa2xWTW5SaFlXeEtkR1ZIUmxkaVJuQXpXa2Q0WVdOV1RuSmFSM0JvWld4YVdWWnNaRFJoTVZWNFYyNVdVbUZzY0ZoWmJHaHZZVVpXY1ZKdGRGaFNhM0I1VjJ0YVYyRldTWGhUYmxwWFVteEtSRlV5YzNoak1XUjFWVzEwVTAweFNsRldWekI0VGtaa1IxZHVUbWhUUlRWWVZXcENWMDVHYTNkWGJrNVhUVVJHU1ZsVlZsTldNREZIWTBkb1dtVnJXbkphUmxwM1VqRlNjazVXWkdoTlYzUXpWbTB4ZDFNd01VZFZXR2hZWW14S1ZWbFVSbmRXVm14eVYyNWtWMkpHY0hwV01uUnJWR3N4VjFkcmFGWmlSa3A2VmxaYVYxWlZNVVZoZWpBOQ=="
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "File successfully uploaded.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "file_id": {
                      "type": "string",
                      "description": "The file's unique ID.",
                      "example": "xBvggqyjQUal6TecwMlYwE"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `124` <br>\n Invalid access token. <br>\n**Error Code:** `300` <br>\n Invalid File type.\nExceed max file size: ${maxSize} megabytes. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests  For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_file:write:admin",
              "whiteboard_file:write",
              "whiteboard:write:file",
              "whiteboard:write:file:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_file:write:admin",
            "whiteboard_file:write"
          ],
          "x-granular-scopes": [
            "whiteboard:write:file",
            "whiteboard:write:file:admin"
          ]
        }
      }
    },
    "/whiteboards/{whiteboardId}/files/{fileId}": {
      "get": {
        "tags": [
          "File"
        ],
        "summary": "Download Imported Whiteboard File",
        "description": "Download files imported into the whiteboard, including images, PDFs, DOCX, and other supported formats.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_file:read`,`whiteboard_file:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:file`,`whiteboard:read:file:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Downloadembeddedwhiteboardfile",
        "parameters": [
          {
            "name": "whiteboardId",
            "in": "path",
            "description": "The whiteboard ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "YXw46hQqQCiZMlAGkQVKsA"
            }
          },
          {
            "name": "fileId",
            "in": "path",
            "description": "The ID of the file imported into the whiteboard to download.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "Et2l7DAvQqu5E6jsqnnasw"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Returns a file stream."
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_file:read",
              "whiteboard_file:read:admin",
              "whiteboard:read:file",
              "whiteboard:read:file:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardContent:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_file:read",
            "whiteboard_file:read:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:read:file",
            "whiteboard:read:file:admin"
          ]
        }
      }
    },
    "/whiteboards/import": {
      "post": {
        "tags": [
          "Import"
        ],
        "summary": "Create a new whiteboard by import",
        "description": "Creates a new whiteboard for a user by converting Miro, Mural, or Visio files.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_import:write`,`whiteboard_import:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:import`,`whiteboard:write:import:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `HEAVY`\n\n**Not supported in Gov cluster** ",
        "operationId": "CreateWhiteboardImport",
        "requestBody": {
          "description": "Creates a new whiteboard for a user by converting Miro, Mural, or Visio files request form-data.",
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "file_id",
                  "type"
                ],
                "type": "object",
                "properties": {
                  "file_id": {
                    "type": "string",
                    "description": "The ID of the file to convert to a whiteboard. Use the **Upload File for Whiteboard Import** API (POST `/whiteboards/files`) to upload the file and get the `file_id`.\nSupported file types:\n- **Visio**: `.vsdx`\n- **Miro** and **Mural**: `.pdf`\n\nMaximum file size: **100 MB**.\nEnsure the file is valid, within the size limit, and in one of the supported formats.",
                    "example": "xBvggqyjQUal6TecwMlYwE"
                  },
                  "type": {
                    "type": "number",
                    "description": "The import file original type. \n\nEnum:  \n* `1` - Miro \n* `2` - Visio \n* `3` - Mural ",
                    "example": 1
                  },
                  "whiteboard_name": {
                    "type": "string",
                    "description": "The name of the whiteboard created by import.",
                    "example": "Q1 plan"
                  },
                  "skip_notifications": {
                    "type": "boolean",
                    "description": " **Default is false (notifications will be sent)**.\n If set to `true`, collaborators and owners will not receive direct messages or emails when the whiteboard is imported.",
                    "example": false
                  },
                  "owner_email": {
                    "type": "string",
                    "description": "The owner email. This field is only available if you have the `whiteboard:write:import:admin` scope. Otherwise, it will automatically be set to `null` and the document owner will **default to the user who imported the file**.",
                    "format": "email",
                    "example": "example@zoom.us"
                  },
                  "collaborators": {
                    "maxItems": 100,
                    "minItems": 0,
                    "type": "array",
                    "description": "Users or team chat channels to add to a whiteboard.",
                    "items": {
                      "type": "object",
                      "properties": {
                        "email": {
                          "type": "string",
                          "description": "The `user email` address to share with.",
                          "format": "email",
                          "example": "user@example.com"
                        },
                        "role": {
                          "type": "string",
                          "description": "Collaborator role type. \n\nEnum:  \n- `0` - OWNER \n- `1` - CO_OWNER \n- `2` - EDITOR \n- `3` - COMMENTER \n- `4` - VIEWER",
                          "example": "2",
                          "enum": [
                            "0",
                            "1",
                            "2",
                            "3",
                            "4"
                          ]
                        },
                        "channel_id": {
                          "type": "string",
                          "description": "The team chat `channel ID` to share with.",
                          "example": "cabc1234567defghijkl01234"
                        }
                      },
                      "description": "Array of collaborator objects."
                    }
                  },
                  "project_id": {
                    "type": "string",
                    "description": "qiMWddmtRxqwSFrK22lJPg",
                    "example": "The Project ID"
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "The imported file is being converted into a whiteboard.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "task_id": {
                      "type": "string",
                      "description": "Use this ID to check the import status via the [Get Whiteboard Import Status](https://developers.zoom.us/docs/api/whiteboard/#tag/import/GET/whiteboards/import/task/{taskId}/status) endpoint (GET `/whiteboards/import/{taskId}/status`). The unique ID for your import.",
                      "example": "ce31de9e5a5f440bab9d60c144fd76b7"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid file type. <br>\n**Error Code:** `38003` <br>\n File size exceed limit. <br>\n**Error Code:** `38004` <br>\n Conversion settings are not enabled. Please contact your account administrator to allow users to import and convert boards and diagrams from other applications into editable whiteboards via the whiteboard account settings. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_import:write",
              "whiteboard_import:write:admin",
              "whiteboard:write:import",
              "whiteboard:write:import:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_import:write",
            "whiteboard_import:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:write:import",
            "whiteboard:write:import:admin"
          ]
        }
      }
    },
    "/whiteboards/import/{taskId}/status": {
      "get": {
        "tags": [
          "Import"
        ],
        "summary": "Get whiteboard import status ",
        "description": "Use this API to query the status of new whiteboard by import. Before using this API, confirm that you have invoked the interface for import file to create new whiteboard **Create a new whiteboard by import** and obtained the `task_id`.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_import:read`,`whiteboard_import:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:import:admin`,`whiteboard:read:import`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "GetWhiteboardimportstatus",
        "parameters": [
          {
            "name": "taskId",
            "in": "path",
            "description": "The unique ID for your import.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "ce31de9e5a5f440bab9d60c144fd76b7"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Get whiteboards import result.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "file_name": {
                      "type": "string",
                      "description": "The file name of import file.",
                      "example": "miro.pdf"
                    },
                    "status": {
                      "type": "string",
                      "description": "\nThe status of the whiteboard created by importing a file.\n\n**Enum**:\n- `processing` - The file is converting to whiteboard.\n- `succeeded` - The file convert to whiteboard successfully.\n- `failed` - The file convert to whiteboard failed.\n",
                      "example": "succeeded",
                      "default": "processing",
                      "enum": [
                        "processing",
                        "failed",
                        "succeeded"
                      ]
                    },
                    "whiteboard_id": {
                      "type": "string",
                      "description": "The whiteboard ID `nullable`. This value is returned only when the `status` is `succeeded`.",
                      "example": "HY8-AC_HSou4SXY1S-OmOg"
                    },
                    "doc_name": {
                      "type": "string",
                      "description": "The whiteboard name `nullable`. This value is returned only when the `status` is `succeeded`.",
                      "example": "Q1 Plan"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `38002` <br>\n Failed to convert the file {fileName}. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `38007` <br>\n The taskId not exist or expired. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_import:read",
              "whiteboard_import:read:admin",
              "whiteboard:read:import:admin",
              "whiteboard:read:import"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_import:read",
            "whiteboard_import:read:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:read:import:admin",
            "whiteboard:read:import"
          ]
        }
      }
    },
    "/whiteboards/projects": {
      "get": {
        "tags": [
          "Project"
        ],
        "summary": "List all projects",
        "description": "Retrieves a list of projects.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:read`,`whiteboard_project:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:list_projects:admin`,`whiteboard:read:list_projects`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\n\n**Not supported in Gov cluster** ",
        "operationId": "Listallprojects",
        "parameters": [
          {
            "name": "search_key",
            "in": "query",
            "description": "The name of a project. If this parameter is provided, only the matched project will be returned. Otherwise, all projets will be returned.",
            "required": false,
            "schema": {
              "maxLength": 50,
              "minLength": 0,
              "type": "string",
              "example": "demo "
            }
          },
          {
            "name": "user_id",
            "in": "query",
            "description": "The email address or user ID of the project owner. If this parameter is provided, only the project owned by the provided user will be returned. If set to `me`, will return the matched projects of current user.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "me"
            }
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of records returned from a single API call.",
            "required": false,
            "schema": {
              "type": "integer",
              "example": 10
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
          }
        ],
        "responses": {
          "200": {
            "description": "The list of projects is successfully retrieved.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "projects": {
                      "type": "array",
                      "description": "The project list.",
                      "items": {
                        "type": "object",
                        "properties": {
                          "collaborator": {
                            "maxItems": 100,
                            "minItems": 0,
                            "type": "array",
                            "description": "The project collaborators, this API returns only the project collaborators who have the 'owner' role, along with the current user's information.",
                            "items": {
                              "type": "object",
                              "properties": {
                                "collaborator_id": {
                                  "type": "string",
                                  "description": "The project collaborator ID.",
                                  "example": "x4g6hy789k0o878k"
                                },
                                "collaborator_name": {
                                  "type": "string",
                                  "description": "The project collaborator name.",
                                  "example": "Jack"
                                },
                                "user_id": {
                                  "type": "string",
                                  "description": "`nullable` User ID. This response only returns if the `collaborator_type` is 0. If the `collaborator_type` is not 0, this returns null value.",
                                  "example": "wf3h7uo1dk0v53bl"
                                },
                                "avatar": {
                                  "type": "string",
                                  "description": "Collaborator profile picture.",
                                  "example": "https://content.zoom.us/swerwerw.jpeg"
                                },
                                "role": {
                                  "type": "integer",
                                  "description": "Collaborator role type.\n\nRole Enum Values:\n\n- `0` - OWNER\n- `1` - CO_OWNER\n- `2` - EDITOR\n- `3` - COMMENTER\n- `4` - VIEWER",
                                  "example": 2,
                                  "enum": [
                                    0,
                                    1,
                                    2,
                                    3,
                                    4
                                  ]
                                },
                                "collaborator_type": {
                                  "type": "integer",
                                  "description": "Collaborator type.\nEnum:\n- `0` - SAME_ACCOUNT_USER\n- `1` - EXTERNAL_USER",
                                  "example": 0,
                                  "enum": [
                                    0,
                                    1
                                  ]
                                }
                              },
                              "description": "The project collaborator information"
                            }
                          },
                          "created_date": {
                            "type": "string",
                            "description": "The project create date-time in UTC/GMT. Example: `2020-03-31T12:02:00Z`",
                            "format": "date-time",
                            "example": "2020-03-31T12:02:00Z"
                          },
                          "modified_date": {
                            "type": "string",
                            "description": "The project update date-time in UTC/GMT. Example: `2020-03-31T12:02:00Z`",
                            "format": "date-time",
                            "example": "2020-03-31T12:02:00Z"
                          },
                          "project_name": {
                            "type": "string",
                            "description": "The  project name.",
                            "example": "Untitle"
                          },
                          "project_id": {
                            "type": "string",
                            "description": "qiMWddmtRxqwSFrK22lJPg",
                            "example": "The Project ID"
                          },
                          "starred": {
                            "type": "boolean",
                            "description": "Indicates whether the project is starred.\n`true`: starred\n`false`: not starred",
                            "example": false
                          },
                          "owner_display_name": {
                            "type": "string",
                            "description": "The project owner display name.",
                            "example": "Example Name"
                          }
                        },
                        "description": "The project basic info."
                      }
                    },
                    "next_page_token": {
                      "type": "string",
                      "description": "The next page token paginates through a large set of results. A next page token is returned when the available results exceed the current page size. The expiration period for this token is 15 minutes.",
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
                      "example": 100
                    }
                  }
                }
              }
            }
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests  For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_project:read",
              "whiteboard_project:read:admin",
              "whiteboard:read:list_projects:admin",
              "whiteboard:read:list_projects"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_project:read",
            "whiteboard_project:read:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:read:list_projects:admin",
            "whiteboard:read:list_projects"
          ]
        }
      },
      "post": {
        "tags": [
          "Project"
        ],
        "summary": "Create a new project",
        "description": "This API endpoint allows you to create a new project for a user, optionally adding collaborators and initializing it with existing whiteboards.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:write`,`whiteboard_project:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:project`,`whiteboard:write:project:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Createproject",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "project_name"
                ],
                "type": "object",
                "properties": {
                  "project_name": {
                    "maxLength": 50,
                    "minLength": 1,
                    "type": "string",
                    "description": "The project name.",
                    "example": "Untitle"
                  },
                  "skip_notifications": {
                    "type": "boolean",
                    "description": " **Default is false (notifications will be sent)**.\n If set to `true`, collaborators and owners will not receive direct messages or emails when the project is created.",
                    "example": false
                  },
                  "invite_message": {
                    "maxLength": 280,
                    "minLength": 0,
                    "type": "string",
                    "description": "The invite message.",
                    "example": "I'm glad to invite you to join this project."
                  },
                  "collaborators": {
                    "title": "Array of collaborator objects.",
                    "maxItems": 100,
                    "minItems": 0,
                    "type": "array",
                    "description": "Array of collaborator objects to be added to the project.",
                    "items": {
                      "type": "object",
                      "properties": {
                        "email": {
                          "type": "string",
                          "description": "The user's email to be added to the project.",
                          "example": "user@example.com"
                        },
                        "role": {
                          "type": "integer",
                          "description": "Collaborator role type.\nWhen assigning a new `OWNER` or `CO_OWNER`, the specified user must belong to the same account as the original `OWNER`. If a new owner is designated for a project, the original owner will automatically be demoted to `CO_OWNER`.\n\nRole Enum Values:\n\n- `0` - OWNER\n- `1` - CO_OWNER\n- `2` - EDITOR\n- `3` - COMMENTER\n- `4` - VIEWER",
                          "example": 2,
                          "default": 2,
                          "enum": [
                            0,
                            1,
                            2,
                            3,
                            4
                          ]
                        }
                      },
                      "description": "A user who works jointly on a project."
                    }
                  },
                  "whiteboard_ids": {
                    "title": "whiteboard_id",
                    "maxItems": 50,
                    "minItems": 0,
                    "uniqueItems": true,
                    "type": "array",
                    "description": "The whiteboard ID list that you want to add to the project.",
                    "items": {
                      "type": "string",
                      "description": "The whiteboard ID.",
                      "example": "J7XZFDEQTkWSsMsmO_M1Xa"
                    }
                  },
                  "owner_email": {
                    "type": "string",
                    "description": "The owner email. This field is only available if you have the `whiteboard_project:write:admin` scope. Otherwise, it will automatically be set to `null` and the project owner will default to the request user.",
                    "format": "email",
                    "example": "owner@example.com"
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "The project has been successfully created.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "collaborator": {
                      "maxItems": 100,
                      "minItems": 0,
                      "type": "array",
                      "description": "The project collaborators.",
                      "items": {
                        "type": "object",
                        "properties": {
                          "collaborator_id": {
                            "type": "string",
                            "description": "The project collaborator ID.",
                            "example": "x4g6hy789k0o878k"
                          },
                          "collaborator_name": {
                            "type": "string",
                            "description": "The project collaborator name.",
                            "example": "Jack"
                          },
                          "user_id": {
                            "type": "string",
                            "description": "`nullable` User ID. This response only returns if the `collaborator_type` is 0. If the `collaborator_type` is not 0, this returns null value.",
                            "example": "wf3h7uo1dk0v53bl"
                          },
                          "avatar": {
                            "type": "string",
                            "description": "Collaborator profile picture.",
                            "example": "https://content.zoom.us/swerwerw.jpeg"
                          },
                          "role": {
                            "type": "integer",
                            "description": "Collaborator role type.\n\nRole Enum Values:\n\n- `0` - OWNER\n- `1` - CO_OWNER\n- `2` - EDITOR\n- `3` - COMMENTER\n- `4` - VIEWER",
                            "example": 2,
                            "enum": [
                              0,
                              1,
                              2,
                              3,
                              4
                            ]
                          },
                          "collaborator_type": {
                            "type": "integer",
                            "description": "Collaborator type.\nEnum:\n- `0` - SAME_ACCOUNT_USER\n- `1` - EXTERNAL_USER",
                            "example": 0,
                            "enum": [
                              0,
                              1
                            ]
                          }
                        },
                        "description": "The project collaborator information."
                      }
                    },
                    "created_date": {
                      "type": "string",
                      "description": "The project creation date-time in UTC/GMT. Example: `2020-03-31T12:02:00Z`",
                      "format": "date-time",
                      "example": "2020-03-31T12:02:00Z"
                    },
                    "modified_date": {
                      "type": "string",
                      "description": "The whiteboard update date-time in UTC/GMT. Example: `2020-03-31T12:02:00Z`",
                      "format": "date-time",
                      "example": "2020-03-31T12:02:00Z"
                    },
                    "project_name": {
                      "type": "string",
                      "description": "The project name.",
                      "example": "Untitle"
                    },
                    "project_id": {
                      "type": "string",
                      "description": "qiMWddmtRxqwSFrK22lJPg",
                      "example": "The Project ID"
                    },
                    "starred": {
                      "type": "boolean",
                      "description": "Indicates whether the project is starred. `true`: starred, `false`: not starred.",
                      "example": false
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_project:write",
              "whiteboard_project:write:admin",
              "whiteboard:write:project",
              "whiteboard:write:project:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_project:write",
            "whiteboard_project:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:write:project",
            "whiteboard:write:project:admin"
          ]
        }
      }
    },
    "/whiteboards/projects/{projectId}": {
      "get": {
        "tags": [
          "Project"
        ],
        "summary": "Get a project",
        "description": "This API endpoint allows you to retrieve detailed information about a specific project.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:read`,`whiteboard_project:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:project`,`whiteboard:read:project:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Getaproject",
        "parameters": [
          {
            "name": "projectId",
            "in": "path",
            "description": "The project ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "N6XZFDEQTkWSsMsmO_M1XQ"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "The project information has been successfully retrieved.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "collaborator": {
                      "maxItems": 100,
                      "minItems": 0,
                      "type": "array",
                      "description": "The project collaborators. This API returns only the project collaborators who have the 'owner' role, along with the current user's information.",
                      "items": {
                        "type": "object",
                        "properties": {
                          "collaborator_id": {
                            "type": "string",
                            "description": "The project collaborator ID.",
                            "example": "x4g6hy789k0o878k"
                          },
                          "collaborator_name": {
                            "type": "string",
                            "description": "The project collaborator name.",
                            "example": "Jack"
                          },
                          "user_id": {
                            "type": "string",
                            "description": "`nullable` User ID. This response only returns if the `collaborator_type` is 0. If the `collaborator_type` is not 0, this returns null value.",
                            "example": "wf3h7uo1dk0v53bl"
                          },
                          "avatar": {
                            "type": "string",
                            "description": "Collaborator profile picture.",
                            "example": "https://content.zoom.us/swerwerw.jpeg"
                          },
                          "role": {
                            "type": "integer",
                            "description": "Collaborator role type.\n\nRole Enum Values:\n\n- `0` - OWNER\n- `1` - CO_OWNER\n- `2` - EDITOR\n- `3` - COMMENTER\n- `4` - VIEWER",
                            "example": 2,
                            "enum": [
                              0,
                              1,
                              2,
                              3,
                              4
                            ]
                          },
                          "collaborator_type": {
                            "type": "integer",
                            "description": "Collaborator type.\nEnum:\n- `0` - SAME_ACCOUNT_USER\n- `1` - EXTERNAL_USER",
                            "example": 0,
                            "enum": [
                              0,
                              1
                            ]
                          }
                        },
                        "description": "The project collaborator information."
                      }
                    },
                    "created_date": {
                      "type": "string",
                      "description": "The project create date-time in UTC/GMT. Example: `2020-03-31T12:02:00Z`",
                      "format": "date-time",
                      "example": "2020-03-31T12:02:00Z"
                    },
                    "modified_date": {
                      "type": "string",
                      "description": "The project update date-time in UTC/GMT. Example: `2020-03-31T12:02:00Z`",
                      "format": "date-time",
                      "example": "2020-03-31T12:02:00Z"
                    },
                    "project_name": {
                      "type": "string",
                      "description": "The  project name.",
                      "example": "Untitle"
                    },
                    "project_id": {
                      "type": "string",
                      "description": "qiMWddmtRxqwSFrK22lJPg",
                      "example": "The Project ID"
                    },
                    "starred": {
                      "type": "boolean",
                      "description": "Indicates whether the project is starred.\n`true`: starred\n`false`: not starred",
                      "example": false
                    },
                    "owner_display_name": {
                      "type": "string",
                      "description": "The project owner display name.",
                      "example": "Example Name"
                    }
                  }
                }
              }
            }
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this project. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `38007` <br>\n The projectId is not exist. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_project:read",
              "whiteboard_project:read:admin",
              "whiteboard:read:project",
              "whiteboard:read:project:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_project:read",
            "whiteboard_project:read:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:read:project",
            "whiteboard:read:project:admin"
          ]
        }
      },
      "delete": {
        "tags": [
          "Project"
        ],
        "summary": "Delete a project",
        "description": "This API endpoint allows you to delete a specified project. Deleting a project removes all associated whiteboards from the project, setting them to have no project assignment. \n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:write:admin`,`whiteboard_project:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:delete:project`,`whiteboard:delete:project:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Deleteproject",
        "parameters": [
          {
            "name": "projectId",
            "in": "path",
            "description": "The project ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "qiMWddmtRxqwSFrK22lJPg"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "The project has been successfully deleted. No content is returned in the response body."
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this project. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `38007` <br>\n The projectId is not exist. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_project:write:admin",
              "whiteboard_project:write",
              "whiteboard:delete:project",
              "whiteboard:delete:project:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [],
          "x-macro-scopes": [
            "whiteboard_project:write:admin",
            "whiteboard_project:write"
          ],
          "x-granular-scopes": [
            "whiteboard:delete:project",
            "whiteboard:delete:project:admin"
          ]
        }
      },
      "patch": {
        "tags": [
          "Project"
        ],
        "summary": "Update project basic information",
        "description": "This API endpoint allows you to update the basic information of a specific project.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:write:admin`,`whiteboard_project:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:update:project:admin`,`whiteboard:update:project`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Updateproject",
        "parameters": [
          {
            "name": "projectId",
            "in": "path",
            "description": "The project ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "qiMWddmtRxqwSFrK22lJPg"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "project_name": {
                    "maxLength": 50,
                    "minLength": 1,
                    "type": "string",
                    "description": "The new project name.",
                    "example": "New Project Name"
                  }
                }
              }
            }
          }
        },
        "responses": {
          "204": {
            "description": "The project's basic information has been successfully updated. No content is returned in the response body."
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `38002` <br>\n The project name is invalid. <br>\n"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this project. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `38007` <br>\n The project ID does not exist. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_project:write:admin",
              "whiteboard_project:write",
              "whiteboard:update:project:admin",
              "whiteboard:update:project"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_project:write:admin",
            "whiteboard_project:write"
          ],
          "x-granular-scopes": [
            "whiteboard:update:project:admin",
            "whiteboard:update:project"
          ]
        }
      }
    },
    "/whiteboards/projects/{projectId}/collaborators": {
      "get": {
        "tags": [
          "Project"
        ],
        "summary": "Get collaborators of a project",
        "description": "Returns a list of a project collaborators.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project_collaborator:read`,`whiteboard_project_collaborator:read:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:project_collaborator`,`whiteboard:read:project_collaborator:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Getcollaboratorsofaproject",
        "parameters": [
          {
            "name": "projectId",
            "in": "path",
            "description": "The ID of the project.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "HY8-AC_HSou4SXY1S-OmOg"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Get collaborators of a project.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "array",
                  "items": {
                    "required": [
                      "collaborator_id"
                    ],
                    "type": "object",
                    "properties": {
                      "collaborator_id": {
                        "type": "string",
                        "description": "`not nullable` The collaborator's unique ID.",
                        "example": "x4g6hy789k0o878k"
                      },
                      "collaborator_name": {
                        "type": "string",
                        "description": "The collaborator's display name.",
                        "example": "Jack"
                      },
                      "user_id": {
                        "type": "string",
                        "description": "`nullable` User ID. This response only returns if the `collaborator_type` is `0`. If the `collaborator_type` is **not** `0`, this returns null value.",
                        "example": "wf3h7uo1dk0v53bl"
                      },
                      "avatar": {
                        "type": "string",
                        "description": "The collaborator profile picture.0",
                        "example": "https://content.zoom.us/swerwerw.jpeg"
                      },
                      "role": {
                        "type": "integer",
                        "description": "The collaborator role type. Enum:\n * `0` - OWNER \n * `1` - CO_OWNER \n * `2` - EDITOR \n * `3` - COMMENTER \n * `4` - VIEWER ",
                        "example": 2,
                        "enum": [
                          0,
                          1,
                          2,
                          3,
                          4
                        ]
                      },
                      "collaborator_type": {
                        "type": "integer",
                        "description": "The collaborator type. \nEnum: \n* `0` - SAME_ACCOUNT_USER \n* `1` - EXTERNAL_USER ",
                        "example": 0,
                        "enum": [
                          0,
                          1
                        ]
                      },
                      "email": {
                        "type": "string",
                        "description": "`nullable` Email. This response only returns if the `collaborator_type` is `0`. If the `collaborator_type` is **not** `0`, this returns null value.",
                        "example": "user@example.com"
                      }
                    },
                    "description": "A user who works jointly on a project."
                  }
                }
              }
            }
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this project. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `38007` <br>\n The projectId is not exist. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_project_collaborator:read",
              "whiteboard_project_collaborator:read:admin",
              "whiteboard:read:project_collaborator",
              "whiteboard:read:project_collaborator:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Read"
          ],
          "x-macro-scopes": [
            "whiteboard_project_collaborator:read",
            "whiteboard_project_collaborator:read:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:read:project_collaborator",
            "whiteboard:read:project_collaborator:admin"
          ]
        }
      },
      "post": {
        "tags": [
          "Project"
        ],
        "summary": "Share a project to new users",
        "description": "Add new collaborators to a specified project. Project owners or authorized users can invite team members or external users to collaborate on a project with specified roles and permissions.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project_collaborator:write`,`whiteboard_project_collaborator:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:project_collaborator`,`whiteboard:write:project_collaborator:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Shareaprojecttonewusers",
        "parameters": [
          {
            "name": "projectId",
            "in": "path",
            "description": "The project ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "N6XZFDEQTkWSsMsmO_M1XQ"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "collaborators"
                ],
                "type": "object",
                "properties": {
                  "skip_notifications": {
                    "type": "boolean",
                    "description": " **Default is false (notifications will be sent)**.\n If set to `true`, collaborators and owners will not receive direct messages or emails when the project is shared.",
                    "example": false
                  },
                  "invite_message": {
                    "maxLength": 280,
                    "minLength": 0,
                    "type": "string",
                    "description": "A custom message to include with the project invitation.",
                    "example": "I'm glad to invite you to join this project."
                  },
                  "collaborators": {
                    "maxItems": 20,
                    "minItems": 1,
                    "type": "array",
                    "description": "List of users to add as collaborators to the project.",
                    "items": {
                      "type": "object",
                      "properties": {
                        "email": {
                          "type": "string",
                          "description": "**User Email Address**: The email address of the user you want to share the project with.",
                          "example": "user@example.com."
                        },
                        "role": {
                          "type": "integer",
                          "description": "Collaborator role type. When assigning a new `OWNER` or `CO_OWNER`, the specified user must belong to the same account as the original `OWNER`. If a new owner is designated for a project, the original owner will automatically be demoted to `CO_OWNER`.\nRole Enum Values: \n* `0` - OWNER \n* `1` - CO_OWNER \n* `2` - EDITOR \n* `3` - COMMENTER \n* `4` - VIEWER ",
                          "example": 1
                        }
                      },
                      "description": "A user who works jointly on a whiteboard."
                    }
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "The collaborators have been successfully added to the project.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "array",
                  "items": {
                    "title": "root",
                    "type": "object",
                    "properties": {
                      "collaborator_id": {
                        "type": "string",
                        "description": "`not nullable` The collaborator's unique ID.",
                        "example": "x4g6hy789k0o878k"
                      },
                      "collaborator_name": {
                        "type": "string",
                        "description": "The collaborator's display name.",
                        "example": "Jack"
                      },
                      "user_id": {
                        "type": "string",
                        "description": "`nullable` User ID. This response only returns if the `collaborator_type` is `0`. If the `collaborator_type` is **not** `0`, this returns null value.",
                        "example": "wf3h7uo1dk0v53bl"
                      },
                      "avatar": {
                        "type": "string",
                        "description": "Collaborator profile picture.",
                        "example": "https://content.zoom.us/swerwerw.jpeg"
                      },
                      "role": {
                        "type": "integer",
                        "description": "Collaborator role type. \nEnum: \n* `0` - OWNER \n* `1` - CO_OWNER \n* `2` - EDITOR \n* `3` - COMMENTER \n* `4` - VIEWER "
                      },
                      "collaborator_type": {
                        "type": "integer",
                        "description": "Collaborator type. \nEnum: \n* `0` - SAME_ACCOUNT_USER \n* `1` - EXTERNAL_USER"
                      }
                    },
                    "description": "Users who work jointly on a whiteboard."
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n "
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n "
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n "
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_project_collaborator:write",
              "whiteboard_project_collaborator:write:admin",
              "whiteboard:write:project_collaborator",
              "whiteboard:write:project_collaborator:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Read",
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_project_collaborator:write",
            "whiteboard_project_collaborator:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:write:project_collaborator",
            "whiteboard:write:project_collaborator:admin"
          ]
        }
      },
      "patch": {
        "tags": [
          "Project"
        ],
        "summary": "Update project collaborators",
        "description": "Updates the role of a collaborator in a project, which allows you to modify the permissions of an existing collaborator in a specific project. \nNote that when assigning a new `OWNER` or `CO_OWNER`, the specified user must belong to the same account as the original `OWNER`. The original owner will automatically be demoted to `CO_OWNER` if a new owner is designated.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project_collaborator:write`,`whiteboard_project_collaborator:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:update:project_collaborator`,`whiteboard:update:project_collaborator:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Updateprojectcollaborators",
        "parameters": [
          {
            "name": "projectId",
            "in": "path",
            "description": "The project ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "N6XZFDEQTkWSsMsmO_M1XQ"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "collaborators"
                ],
                "type": "object",
                "properties": {
                  "collaborators": {
                    "maxItems": 20,
                    "minItems": 1,
                    "type": "array",
                    "description": "An array of collaborator objects.",
                    "items": {
                      "required": [
                        "collaborator_id",
                        "role"
                      ],
                      "type": "object",
                      "properties": {
                        "collaborator_id": {
                          "type": "string",
                          "description": "The collaborator's unique ID `not nullable` .",
                          "example": "x4g6hy789k0o878k"
                        },
                        "role": {
                          "type": "integer",
                          "description": "Collaborator role type.\nWhen assigning a new `OWNER` or `CO_OWNER`, the specified user must belong to the same account as the original `OWNER`. If a new owner is designated for a project, the original owner will automatically be demoted to `CO_OWNER`.\n\nRole Enum Values:\n\n0 - OWNER\n1 - CO_OWNER\n2 - EDITOR\n3 - COMMENTER\n4 - VIEWER",
                          "example": 2,
                          "default": 2,
                          "enum": [
                            0,
                            1,
                            2,
                            3,
                            4
                          ]
                        }
                      }
                    }
                  }
                },
                "description": "Update collaborators permission of a whiteboard."
              }
            }
          }
        },
        "responses": {
          "204": {
            "description": "Project collaborators updated successfully."
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Invalid collaborator_id {collaboratorId}. <br>\n"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this project. <br>\n**Error Code:** `34001` <br>\n You do not have permission to update the role of collaborator {collaboratorId}. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `35004` <br>\n The project ID {projectID} is not exist. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_project_collaborator:write",
              "whiteboard_project_collaborator:write:admin",
              "whiteboard:update:project_collaborator",
              "whiteboard:update:project_collaborator:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_project_collaborator:write",
            "whiteboard_project_collaborator:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:update:project_collaborator",
            "whiteboard:update:project_collaborator:admin"
          ]
        }
      }
    },
    "/whiteboards/projects/{projectId}/collaborators/{collaboratorId}": {
      "delete": {
        "tags": [
          "Project"
        ],
        "summary": "Remove the collaborator from a project",
        "description": "This API endpoint allows you to remove a specific collaborator from a designated project. It enables project managers or authorized users to revoke access or remove a user from a project, enhancing project management and access control capabilities.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project_collaborator:write`,`whiteboard_project_collaborator:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:delete:project_collaborator:admin`,`whiteboard:delete:project_collaborator`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Removethecollaboratorfromaproject",
        "parameters": [
          {
            "name": "projectId",
            "in": "path",
            "description": "The project ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "N6XZFDEQTkWSsMsmO_M1XQ"
            }
          },
          {
            "name": "collaboratorId",
            "in": "path",
            "description": "The collaborator ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "x4g6hy789k0o878k"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "The collaborator has been successfully removed from the project. No content is returned in the response body."
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this project. <br>\n**Error Code:** `34001` <br>\n You do not have permission to remove the collaborator {collaboratorId} from the project. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `38004` <br>\n The project ID {projectID} is not exist. <br>\n**Error Code:** `34005` <br>\n The collaborator ID {collaboratorId} is not exist. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_project_collaborator:write",
              "whiteboard_project_collaborator:write:admin",
              "whiteboard:delete:project_collaborator:admin",
              "whiteboard:delete:project_collaborator"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_project_collaborator:write",
            "whiteboard_project_collaborator:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:delete:project_collaborator:admin",
            "whiteboard:delete:project_collaborator"
          ]
        }
      }
    },
    "/whiteboards/projects/{projectId}/whiteboards": {
      "post": {
        "tags": [
          "Project"
        ],
        "summary": "Move whiteboards to a project",
        "description": "This API endpoint allows you to move one or more whiteboards to a specified project. It enables users to organize their whiteboards by associating them with a specific project, enhancing project management capabilities.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:write:admin`,`whiteboard_project:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:project_whiteboard`,`whiteboard:write:project_whiteboard:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Movewhiteboardstoproject",
        "parameters": [
          {
            "name": "projectId",
            "in": "path",
            "description": "The Project ID",
            "required": true,
            "schema": {
              "type": "string",
              "example": "qiMWddmtRxqwSFrK22lJPg"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "whiteboard_ids"
                ],
                "type": "object",
                "properties": {
                  "whiteboard_ids": {
                    "title": "whiteboard_id",
                    "maxItems": 50,
                    "minItems": 1,
                    "type": "array",
                    "description": "The whiteboard ID list",
                    "items": {
                      "type": "string",
                      "description": "The whiteboard ID.",
                      "example": "dZO3Wd9XT4mgXdNPbMm6tw"
                    }
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "The whiteboards have been successfully moved to the project.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "whiteboard_ids": {
                      "type": "array",
                      "description": "The whiteboard ID list",
                      "items": {
                        "type": "string",
                        "description": "The whiteboard ID.",
                        "example": "dZO3Wd9XT4mgXdNPbMm6tw"
                      }
                    }
                  }
                }
              }
            }
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this project. <br>\n**Error Code:** `34001` <br>\n You do not have permission to move {whiteboardID} to the project. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `38007` <br>\n The projectId is not exist. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_project:write:admin",
              "whiteboard_project:write",
              "whiteboard:write:project_whiteboard",
              "whiteboard:write:project_whiteboard:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_project:write:admin",
            "whiteboard_project:write"
          ],
          "x-granular-scopes": [
            "whiteboard:write:project_whiteboard",
            "whiteboard:write:project_whiteboard:admin"
          ]
        }
      },
      "delete": {
        "tags": [
          "Project"
        ],
        "summary": "Remove whiteboards from a project",
        "description": "This API endpoint allows you to remove one or more whiteboards from a specified project. \n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:write:admin`,`whiteboard_project:write`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:delete:project_whiteboard`,`whiteboard:delete:project_whiteboard:admin`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "Removewhiteboardsfromaproject",
        "parameters": [
          {
            "name": "projectId",
            "in": "path",
            "description": "The project ID.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "N6XZFDEQTkWSsMsmO_M1XQ"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "whiteboard_ids"
                ],
                "type": "object",
                "properties": {
                  "whiteboard_ids": {
                    "maxItems": 50,
                    "minItems": 1,
                    "type": "array",
                    "description": "The whiteboard ID list.",
                    "items": {
                      "type": "string",
                      "description": "The whiteboard ID.",
                      "example": "W6Uuv4qUSfuJlp_GXGi36A"
                    }
                  }
                }
              }
            }
          }
        },
        "responses": {
          "204": {
            "description": "The whiteboards have been successfully removed from the project. No content is returned in the response body."
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this project. <br>\n**Error Code:** `34001` <br>\n You have no permission to add {whiteboardID} to the project. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `38007` <br>\n The project ID does not exist. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_project:write:admin",
              "whiteboard_project:write",
              "whiteboard:delete:project_whiteboard",
              "whiteboard:delete:project_whiteboard:admin"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_project:write:admin",
            "whiteboard_project:write"
          ],
          "x-granular-scopes": [
            "whiteboard:delete:project_whiteboard",
            "whiteboard:delete:project_whiteboard:admin"
          ]
        }
      }
    },
    "/whiteboards/{whiteboardId}/share_setting": {
      "patch": {
        "tags": [
          "Settings"
        ],
        "summary": "Update whiteboard share setting",
        "description": "Updates the whiteboard share setting. \n\n\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_share_setting:write`,`whiteboard_share_setting:write:admin`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:update:share_setting:admin`,`whiteboard:update:share_setting`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`",
        "operationId": "UpdateAWhiteboardShareSetting",
        "parameters": [
          {
            "name": "whiteboardId",
            "in": "path",
            "description": "The ID of the whiteboard.",
            "required": true,
            "schema": {
              "type": "string",
              "example": "HY8-AC_HSou4SXY1S-OmOg"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "share_link_setting": {
                    "type": "object",
                    "properties": {
                      "share_scope": {
                        "type": "integer",
                        "description": "The access type of whiteboard share link. \n* Enum: \n* `0` - `Restricted`, Only people added can open with this link \n* `1` - `Same account` \n* `2` - Anyone \n* `3` - SAME_ACCOUNT_EXTERNAL_AUTH \n",
                        "example": 0,
                        "default": 0,
                        "enum": [
                          0,
                          1,
                          2,
                          3
                        ],
                        "x-enum-descriptions": [
                          "Restricted. Only people added can open with this link.",
                          "Same account",
                          "Anyone",
                          "Same account external auth"
                        ]
                      },
                      "share_role": {
                        "type": "integer",
                        "description": "The access level of a whiteboard share link. Use this field with `share_scope`. If `share_scope` set `0` - `Restricted`, this field is null. If `share_scope` set `1` - `Same account` or `2` - `Anyone`, this filed can be `2` - `Editor`, `3` - `Commenter` or `4` - `Viewer`. \n* Enum: \n* `2` - `Editor`\n* `3` - `Commenter` \n* `4` - Viewer \n",
                        "example": 2,
                        "enum": [
                          2,
                          3,
                          4
                        ],
                        "x-enum-descriptions": [
                          "Editor",
                          "Commenter",
                          "Viewer"
                        ]
                      }
                    },
                    "description": "The access level and access type of a current whiteboard's share link."
                  }
                },
                "description": "Updates a whiteboard's share setting."
              }
            }
          }
        },
        "responses": {
          "204": {
            "description": "**HTTP Status Code**: `204`   \n \nWhiteboard share setting updated successfully."
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `34002` <br>\n Share role must be viewer for everyone. <br>\n**Error Code:** `34002` <br>\n Share role can't be null, owner, or co-owner for same account. <br>\n"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `111804` <br>\n Permission is needed to access this document. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "whiteboard_share_setting:write",
              "whiteboard_share_setting:write:admin",
              "whiteboard:update:share_setting:admin",
              "whiteboard:update:share_setting"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "WhiteboardManagement:Edit"
          ],
          "x-macro-scopes": [
            "whiteboard_share_setting:write",
            "whiteboard_share_setting:write:admin"
          ],
          "x-granular-scopes": [
            "whiteboard:update:share_setting:admin",
            "whiteboard:update:share_setting"
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
              "whiteboard_content:read": "whiteboard_content:read",
              "whiteboard_content:read:admin": "whiteboard_content:read:admin",
              "whiteboard:read:list_sessions": "whiteboard:read:list_sessions",
              "whiteboard:read:list_sessions:admin": "whiteboard:read:list_sessions:admin",
              "whiteboard:read:archived_file": "whiteboard:read:archived_file",
              "whiteboard:read:archived_file:admin": "whiteboard:read:archived_file:admin",
              "whiteboard:read:session": "whiteboard:read:session",
              "whiteboard:read:session:admin": "whiteboard:read:session:admin",
              "whiteboard_collaborator:write:admin": "whiteboard_collaborator:write:admin",
              "whiteboard:update:collaborator:admin": "whiteboard:update:collaborator:admin",
              "whiteboard:write:collaborator:admin": "whiteboard:write:collaborator:admin",
              "whiteboard_collaborator:read:admin": "whiteboard_collaborator:read:admin",
              "whiteboard:read:list_collaborators:admin": "whiteboard:read:list_collaborators:admin",
              "whiteboard:delete:collaborator:admin": "whiteboard:delete:collaborator:admin",
              "whiteboard:write": "whiteboard:write",
              "whiteboard:write:whiteboard": "whiteboard:write:whiteboard",
              "whiteboard:read:admin": "whiteboard:read:admin",
              "whiteboard:read": "whiteboard:read",
              "whiteboard:read:list_whiteboards:admin": "whiteboard:read:list_whiteboards:admin",
              "whiteboard:read:list_whiteboards": "whiteboard:read:list_whiteboards",
              "whiteboard:read:whiteboard:admin": "whiteboard:read:whiteboard:admin",
              "whiteboard:read:whiteboard": "whiteboard:read:whiteboard",
              "whiteboard:write:admin": "whiteboard:write:admin",
              "whiteboard:delete:whiteboard:admin": "whiteboard:delete:whiteboard:admin",
              "whiteboard:delete:whiteboard": "whiteboard:delete:whiteboard",
              "whiteboard:update:whiteboard:admin": "whiteboard:update:whiteboard:admin",
              "whiteboard_export:write:admin": "whiteboard_export:write:admin",
              "whiteboard_export:write": "whiteboard_export:write",
              "whiteboard:write:export": "whiteboard:write:export",
              "whiteboard:write:export:admin": "whiteboard:write:export:admin",
              "whiteboard:read:export": "whiteboard:read:export",
              "whiteboard:read:export:admin": "whiteboard:read:export:admin",
              "whiteboard_file:write:admin": "whiteboard_file:write:admin",
              "whiteboard_file:write": "whiteboard_file:write",
              "whiteboard:write:file": "whiteboard:write:file",
              "whiteboard:write:file:admin": "whiteboard:write:file:admin",
              "whiteboard_file:read": "whiteboard_file:read",
              "whiteboard_file:read:admin": "whiteboard_file:read:admin",
              "whiteboard:read:file": "whiteboard:read:file",
              "whiteboard:read:file:admin": "whiteboard:read:file:admin",
              "whiteboard_import:write": "whiteboard_import:write",
              "whiteboard_import:write:admin": "whiteboard_import:write:admin",
              "whiteboard:write:import": "whiteboard:write:import",
              "whiteboard:write:import:admin": "whiteboard:write:import:admin",
              "whiteboard_import:read": "whiteboard_import:read",
              "whiteboard_import:read:admin": "whiteboard_import:read:admin",
              "whiteboard:read:import:admin": "whiteboard:read:import:admin",
              "whiteboard:read:import": "whiteboard:read:import",
              "whiteboard_project:write": "whiteboard_project:write",
              "whiteboard_project:write:admin": "whiteboard_project:write:admin",
              "whiteboard:write:project": "whiteboard:write:project",
              "whiteboard:write:project:admin": "whiteboard:write:project:admin",
              "whiteboard_project:read": "whiteboard_project:read",
              "whiteboard_project:read:admin": "whiteboard_project:read:admin",
              "whiteboard:read:list_projects:admin": "whiteboard:read:list_projects:admin",
              "whiteboard:read:list_projects": "whiteboard:read:list_projects",
              "whiteboard:update:project:admin": "whiteboard:update:project:admin",
              "whiteboard:update:project": "whiteboard:update:project",
              "whiteboard:delete:project": "whiteboard:delete:project",
              "whiteboard:delete:project:admin": "whiteboard:delete:project:admin",
              "whiteboard:read:project": "whiteboard:read:project",
              "whiteboard:read:project:admin": "whiteboard:read:project:admin",
              "whiteboard_project_collaborator:read": "whiteboard_project_collaborator:read",
              "whiteboard_project_collaborator:read:admin": "whiteboard_project_collaborator:read:admin",
              "whiteboard:read:project_collaborator": "whiteboard:read:project_collaborator",
              "whiteboard:read:project_collaborator:admin": "whiteboard:read:project_collaborator:admin",
              "whiteboard_project_collaborator:write": "whiteboard_project_collaborator:write",
              "whiteboard_project_collaborator:write:admin": "whiteboard_project_collaborator:write:admin",
              "whiteboard:update:project_collaborator": "whiteboard:update:project_collaborator",
              "whiteboard:update:project_collaborator:admin": "whiteboard:update:project_collaborator:admin",
              "whiteboard:write:project_collaborator": "whiteboard:write:project_collaborator",
              "whiteboard:write:project_collaborator:admin": "whiteboard:write:project_collaborator:admin",
              "whiteboard:delete:project_collaborator:admin": "whiteboard:delete:project_collaborator:admin",
              "whiteboard:delete:project_collaborator": "whiteboard:delete:project_collaborator",
              "whiteboard:write:project_whiteboard": "whiteboard:write:project_whiteboard",
              "whiteboard:write:project_whiteboard:admin": "whiteboard:write:project_whiteboard:admin",
              "whiteboard:delete:project_whiteboard": "whiteboard:delete:project_whiteboard",
              "whiteboard:delete:project_whiteboard:admin": "whiteboard:delete:project_whiteboard:admin",
              "whiteboard_share_setting:write": "whiteboard_share_setting:write",
              "whiteboard_share_setting:write:admin": "whiteboard_share_setting:write:admin",
              "whiteboard:update:share_setting:admin": "whiteboard:update:share_setting:admin",
              "whiteboard:update:share_setting": "whiteboard:update:share_setting"
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
      "whiteboard"
    ]
  },
  "downloadPath": "/api-hub/whiteboard/methods/endpoints.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": false
  },
  "productMeta": {
    "specType": "oas",
    "slug": "/docs/api/rest/reference/whiteboard/methods",
    "productName": "Whiteboard",
    "title": "Zoom Whiteboard API",
    "keywords": "zoom, marketplace, whiteboard, webhook, event, create, update, delete, data, summary",
    "zoapEndpoint": "docJsonByFixVersion",
    "params": {
      "productName": "Whiteboard"
    },
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "productDisplayTitle": "Whiteboard",
    "postmanLink": "https://www.postman.com/zoom-developer/zoom-public-workspace/collection/nwtn7hu/zoom-whiteboard-api",
    "scalarSlug": "/docs/api/whiteboard/methods",
    "svgname": "_whiteboard"
  },
  "frontmatter": {
    "title": "Whiteboard APIs",
    "keywords": "zoom, marketplace, whiteboard, webhook, event, create, update, delete, data, summary",
    "description": "The [Zoom Whiteboard APIs](https://developers.zoom.us/docs/zoom-whiteboard/) allow you to manage your Zoom Whiteboard. For example, collaborative persistent whiteboard that allows participants to share and collaborate on a virtual whiteboard in real time The Zoom Whiteboard REST API for [Zoom Whiteboard](https://developers.zoom.us/docs/api/rest/whiteboard/) allows developers to interact with various Zoom features, including Zoom Whiteboard. Developers can create and manage whiteboards programmatically using these APIs.",
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
        "whiteboard"
      ]
    },
    "downloadPath": "/api-hub/whiteboard/methods/endpoints.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": false
    },
    "productMeta": {
      "specType": "oas",
      "slug": "/docs/api/rest/reference/whiteboard/methods",
      "productName": "Whiteboard",
      "title": "Zoom Whiteboard API",
      "keywords": "zoom, marketplace, whiteboard, webhook, event, create, update, delete, data, summary",
      "zoapEndpoint": "docJsonByFixVersion",
      "params": {
        "productName": "Whiteboard"
      },
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "productDisplayTitle": "Whiteboard",
      "postmanLink": "https://www.postman.com/zoom-developer/zoom-public-workspace/collection/nwtn7hu/zoom-whiteboard-api",
      "scalarSlug": "/docs/api/whiteboard/methods",
      "svgname": "_whiteboard"
    },
    "spec": "{\"openapi\":\"3.0.0\",\"info\":{\"title\":\"Whiteboard\",\"description\":\"The [Zoom Whiteboard APIs](https://developers.zoom.us/docs/zoom-whiteboard/) allow you to manage your Zoom Whiteboard. For example, collaborative persistent whiteboard that allows participants to share and collaborate on a virtual whiteboard in real time The Zoom Whiteboard REST API for [Zoom Whiteboard](https://developers.zoom.us/docs/api/rest/whiteboard/) allows developers to interact with various Zoom features, including Zoom Whiteboard. Developers can create and manage whiteboards programmatically using these APIs.\",\"termsOfService\":\"https://zoom.us/docs/en-us/zoom_api_license_and_tou.html\",\"contact\":{\"name\":\"Zoom Developers\",\"url\":\"https://developer.zoom.us/\"},\"version\":\"2\"},\"externalDocs\":{\"description\":\"Find out more about Swagger\",\"url\":\"https://swagger.io\"},\"servers\":[{\"url\":\"https://api.zoom.us/v2\"}],\"paths\":{\"/whiteboards/sessions\":{\"get\":{\"tags\":[\"Archiving\"],\"summary\":\"List whiteboards sessions\",\"description\":\"Creates a list of archived sessions under current account.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_content:read`,`whiteboard_content:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:list_sessions`,`whiteboard:read:list_sessions:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\",\"operationId\":\"Createwhiteboardsarchivefiles\",\"parameters\":[{\"name\":\"page_size\",\"in\":\"query\",\"description\":\"The number of records returned within a single API call.\",\"required\":false,\"schema\":{\"maximum\":300,\"type\":\"integer\",\"example\":30,\"default\":30}},{\"name\":\"next_page_token\",\"in\":\"query\",\"description\":\"The next page token paginates through a large set of results. A next page token returns whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"IAfJX3jsOLW7w3dokmFl84zOa0MAVGyMEB2\"}},{\"name\":\"from\",\"in\":\"query\",\"description\":\"The query session\u00a0`start_time\u00a0`date in`\u00a0yyyy-MM-dd'T'HH:mm:ssZ`\u00a0format. The\u00a0`start_time`\u00a0of a session in the result list should be greater than or equal to\u00a0`from`. If this field has no provided value, the default is the current date. For example, if you make the API request on June 30, 2020, without providing the\u00a0`from`\u00a0and\u00a0`to`\u00a0parameters, by default, the value of\u00a0the `from`\u00a0field will be \u201c2020-06-30,\u201d and the value of the\u00a0`to`\u00a0field will be \u201c2020-07-30\u201d.\",\"required\":false,\"schema\":{\"type\":\"string\",\"format\":\"date-time\",\"example\":\"2021-03-12T02:12:27Z\"}},{\"name\":\"to\",\"in\":\"query\",\"description\":\"The query session`end_time` date, in `yyyy-MM-dd'T'HH:mm:ssZ` format. The `end_time` of session in result list should be less than or equal to `to`.\",\"required\":false,\"schema\":{\"type\":\"string\",\"format\":\"date-time\",\"example\":\"2021-03-18T05:41:36Z\"}}],\"responses\":{\"200\":{\"description\":\"The returned archived sessions list.\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"from\",\"next_page_token\",\"page_size\",\"to\"],\"type\":\"object\",\"properties\":{\"from\":{\"type\":\"string\",\"description\":\"The queried start date.\",\"format\":\"date-time\",\"example\":\"2021-03-12T02:12:27Z\"},\"to\":{\"type\":\"string\",\"description\":\"The queried end date.\",\"format\":\"date-time\",\"example\":\"2021-03-12T02:12:27Z\"},\"page_size\":{\"maximum\":300,\"type\":\"integer\",\"description\":\"The number of records returned per page in a single API call.\",\"example\":30,\"default\":30},\"next_page_token\":{\"type\":\"string\",\"description\":\"The next page token paginates through a large set of results. A next page token returns whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.\\n\\nNote: If you use `next_page_token` as a parameter, your other request parameters should be changeless to make sure that the large result set is what you want. For example, if your to parameter is for a future time, Zoom resets this value to the current time and returns this value in the response body, along with the `next_page_token` value. Use these same to and `next_page_token` values in requests for the remaining results set; otherwise you will get an invalid token error.\",\"example\":\"IAfJX3jsOLW7w3dokmFl84zOa0MAVGyMEB2\"},\"sessions\":{\"maxItems\":300,\"type\":\"array\",\"description\":\"The whiteboard archived sessions list. \",\"items\":{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The session's unique ID.\",\"example\":\"a2f19f96-9294-4f51-8134-6f0eea108eb2\"},\"start_time\":{\"type\":\"string\",\"description\":\"The session's start time.\\nIt should be `from < start_time`\",\"format\":\"date-time\",\"example\":\"2021-03-11T05:41:36Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The session's end time.\\nIt should be `end_time < to`\",\"format\":\"date-time\",\"example\":\"2021-03-11T05:41:36Z\"},\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The ID of whiteboard.\",\"example\":\"HY8-AC_HSou4SXY1S-OmOg\"},\"whiteboard_name\":{\"type\":\"string\",\"description\":\"The name of whiteboard.\",\"example\":\"example_whiteboard_name\"},\"expire_time\":{\"type\":\"string\",\"description\":\"The session's expire time. \",\"format\":\"date-time\",\"example\":\"2021-03-26T06:41:36Z\"},\"regulated_participants\":{\"maxItems\":1000,\"type\":\"array\",\"description\":\"The session participant under this account.\",\"items\":{\"type\":\"object\",\"properties\":{\"user_id\":{\"type\":\"string\",\"description\":\"The unique ID of participant user in this session.\",\"example\":\"wf3h7uo1dk0v53bl\"},\"user_name\":{\"type\":\"string\",\"description\":\"The name of participant user in this session.\",\"example\":\"Jack\"},\"join_whiteboard_time\":{\"type\":\"string\",\"description\":\"The user join the whiteboard time.\",\"format\":\"date-time\",\"example\":\"2021-03-26T06:41:36Z\"},\"leave_whiteboard_time\":{\"type\":\"string\",\"description\":\"The user leave the whiteboard time.\",\"format\":\"date-time\",\"example\":\"2021-03-26T07:41:36Z\"},\"email\":{\"type\":\"string\",\"description\":\"The email of participant. `nullable` If the participant is not part of the session's account, this returns an empty string value.\",\"example\":\"user@example.com\"}},\"description\":\"The regulated user info under the account\"}}}}},\"total_records\":{\"type\":\"integer\",\"description\":\"The total number of returned sessions records.\",\"example\":20}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `34002` <br>\\n Invalid parameter. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardContent:Read\"],\"x-macro-scopes\":[\"whiteboard_content:read\",\"whiteboard_content:read:admin\"],\"x-granular-scopes\":[\"whiteboard:read:list_sessions\",\"whiteboard:read:list_sessions:admin\"]}}},\"/whiteboards/sessions/activity/download/{path}\":{\"get\":{\"tags\":[\"Archiving\"],\"summary\":\"Download Whiteboards activity file\",\"description\":\"Use this API to download the Whiteboard archival file. For obtaining the download URL, please consult the [List whiteboard sessions activities](https://developers.zoom.us/docs/api/rest/reference/whiteboard/methods/#operation/Listwhiteboardsessionsarchivedfiles) documentation. Scroll down under field on schema to documentation URL string.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_content:read`,`whiteboard_content:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:archived_file`,`whiteboard:read:archived_file:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\",\"operationId\":\"Downloadwhiteboardsactivityfile\",\"parameters\":[{\"name\":\"path\",\"in\":\"path\",\"description\":\"The file path.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"Qg75t7xZBtEbAkjdlgbfdngBBBB\"}}],\"responses\":{\"200\":{\"description\":\"Returns a file stream.\",\"headers\":{\"Content-Type\":{\"description\":\"Content-Type.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"application/octet-stream\"}},\"Content-Disposition\":{\"description\":\"Content-Disposition.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"attachment;filename=adcc655d3a3549e692959737fe2a7a0d.pdf\"}},\"Content-Length\":{\"description\":\"Content-Length.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"102400\"}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `34002` <br>\\n Invalid path. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `34000` <br>\\n The resource not found. The file does not exist or the file has expired. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardContent:Read\"],\"x-macro-scopes\":[\"whiteboard_content:read\",\"whiteboard_content:read:admin\"],\"x-granular-scopes\":[\"whiteboard:read:archived_file\",\"whiteboard:read:archived_file:admin\"]}}},\"/whiteboards/sessions/{seesionId}\":{\"get\":{\"tags\":[\"Archiving\"],\"summary\":\"List whiteboard sessions activities\",\"description\":\"Lists a session's activity information.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_content:read`,`whiteboard_content:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:session`,`whiteboard:read:session:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\",\"operationId\":\"Listwhiteboardsessionsarchivedfiles\",\"parameters\":[{\"name\":\"seesionId\",\"in\":\"path\",\"description\":\"The session unique ID.\\n\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"f8b9197a-0c8f-c8de-4db3-d8a5e8d9f505\"}},{\"name\":\"page_size\",\"in\":\"query\",\"description\":\"The number of records returned within a single API call.\",\"required\":false,\"schema\":{\"maximum\":300,\"type\":\"integer\",\"example\":30,\"default\":30}},{\"name\":\"next_page_token\",\"in\":\"query\",\"description\":\"The next page token paginates through a large set of results. A next page token returns whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"IAfJX3jsOLW7w3dokmFl84zOa0MAVGyMEB2\"}}],\"responses\":{\"200\":{\"description\":\"The session activity information.\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"page_size\",\"total_records\"],\"type\":\"object\",\"properties\":{\"page_size\":{\"type\":\"integer\",\"description\":\"The number of records returned within a single API call.\",\"example\":30},\"next_page_token\":{\"type\":\"string\",\"description\":\"The next page token paginates through a large set of results. A next page token returns whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes. **Note**: if you use `next_page_token` as a parameter, your other request parameters should be changeless to make sure that the large result set is what you want. For example, if your to parameter is for a future time, Zoom resets this value to the current time and returns this value in the response body, along with the `next_page_token` value. Use these same to and `next_page_token` values in requests for the remaining results set; otherwise you will get an invalid token error.\",\"example\":\"IAfJX3jsOLW7w3dokmFl84zOa0MAVGyMEB2\"},\"session_id\":{\"type\":\"string\",\"description\":\"The unique ID of current session.\",\"example\":\"a2f19f96-9294-4f51-8134-6f0eea108eb2\"},\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The ID of the Zoom Whiteboard.\",\"example\":\"HY8-AC_HSou4SXY1S-OmOg\"},\"whiteboard_name\":{\"type\":\"string\",\"description\":\"The name of Zoom Whiteboard.\",\"example\":\"example_whiteboard_name\"},\"start_time\":{\"type\":\"string\",\"description\":\"The session's start time.\",\"format\":\"date-time\",\"example\":\"2021-03-11T05:41:36Z\"},\"end_time\":{\"type\":\"string\",\"description\":\"The session's end time.\",\"format\":\"date-time\",\"example\":\"2021-03-11T06:41:36Z\"},\"expire_time\":{\"type\":\"string\",\"description\":\"The session's expire time. \",\"format\":\"date-time\",\"example\":\"2021-03-26T06:41:36Z\"},\"activities\":{\"maxItems\":300,\"type\":\"array\",\"description\":\"The activity list of a session.\",\"items\":{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"string\",\"description\":\"The unique ID of an activity.\",\"example\":\"cc63bce7-cb59-c1dc-fadd-2be734ad1d55\"},\"type\":{\"type\":\"string\",\"description\":\"The operation description.\",\"example\":\"add object\"},\"download_url\":{\"type\":\"string\",\"description\":\" The URL to download the the archive file. OAuth apps If a user has authorized and installed your OAuth app that contains `whiteboard:read:archived_file` or `whiteboard:read:archived_file:admin` scopes, use the user's [OAuth access token](https://developers.zoom.us/docs/integrations/oauth/) to download the file. More details refer [Download Whiteboards activity file](https://developers.zoom.us/docs/api/rest/reference/whiteboard/methods/#operation/Downloadwhiteboardsactivityfile)\\n For example:  `https://{{base-domain}}/whiteboards/sesssions/activity/download/xxx--header 'Authorization: Bearer {{OAuth-access-token}}'`\",\"example\":\"https://example.com/whiteboards/sessions/activity/download/Qg75t7xZBtEbAkjdlgbfdngBBBB\"},\"create_time\":{\"type\":\"string\",\"description\":\"The activity create time.\",\"format\":\"date-time\",\"example\":\"2021-03-11T05:41:36Z\"},\"user_id\":{\"type\":\"string\",\"description\":\"The ID of user. `nullable`  If the participant is **not** part of the session's account, this returns an empty string value.\",\"example\":\"wf3h7uo1dk0v53bl\"},\"collaborator_id\":{\"type\":\"string\",\"description\":\"The participant's unique ID. `not nullable` \",\"example\":\"x4g6hy789k0o878k\"},\"collaborator_name\":{\"type\":\"string\",\"description\":\"The participant's display name.\",\"example\":\"Jack\"},\"email\":{\"type\":\"string\",\"description\":\"The email of participant. `nullable` If the participant is not part of the session's account, this returns an empty string value.\",\"example\":\"user@example.com\"},\"content\":{\"type\":\"string\",\"description\":\"The text content generated by the activity.\",\"example\":\"This is a activity content demo\"},\"object_ids\":{\"maxItems\":100,\"minItems\":0,\"type\":\"array\",\"description\":\"The IDs of the whiteboard object associated with this activity.\",\"items\":{\"type\":\"string\",\"description\":\"The object ID in the whiteboard.\",\"example\":\"wfx4g6s1dk0v53bl\"}}},\"description\":\"activity information\"}},\"total_records\":{\"type\":\"number\",\"description\":\"The total number of returned activities records.\",\"example\":30}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `34000` <br>\\n The session not exist or expired. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardContent:Read\"],\"x-macro-scopes\":[\"whiteboard_content:read\",\"whiteboard_content:read:admin\"],\"x-granular-scopes\":[\"whiteboard:read:session\",\"whiteboard:read:session:admin\"]}}},\"/whiteboards/{whiteboardId}/collaborator\":{\"get\":{\"tags\":[\"Collaborator\"],\"summary\":\"Get collaborators of a whiteboard\",\"description\":\"Returns a list of a whiteboard collaborators. \\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_collaborator:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:list_collaborators:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\",\"operationId\":\"GetAWhiteboardCollaborator\",\"parameters\":[{\"name\":\"whiteboardId\",\"in\":\"path\",\"description\":\"The ID of the whiteboard.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"HY8-AC_HSou4SXY1S-OmOg\"}}],\"responses\":{\"200\":{\"description\":\"**HTTP Status Code:** `200`   \\n \\nWhiteboard collaborators returned.\",\"content\":{\"application/json\":{\"schema\":{\"maxItems\":1000,\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"collaborator_id\":{\"type\":\"string\",\"description\":\"`not nullable` The collaborator's unique ID.\",\"example\":\"x4g6hy789k0o878k\"},\"collaborator_name\":{\"type\":\"string\",\"description\":\"The collaborator's display name.\",\"example\":\"Jack\"},\"user_id\":{\"type\":\"string\",\"description\":\"`nullable` User ID. This response only returns if the `collaborator_type` is `0`. If the `collaborator_type` is **not** `0`, this returns null value.\",\"example\":\"wf3h7uo1dk0v53bl\"},\"channel_id\":{\"type\":\"string\",\"description\":\"`nullable` Team chat channel ID. This response only returns if the `collaborator_type` is `2`. If the `collaborator_type` is **not** `2`, this returns null value.\",\"example\":\"cabc1234567defghijkl01234\"},\"avatar\":{\"type\":\"string\",\"description\":\"The collaborator profile picture.\",\"example\":\"https://content.zoom.us/swerwerw.jpeg\"},\"role\":{\"type\":\"integer\",\"description\":\"The collaborator role type. \\nEnum: \\n* `0` - OWNER \\n* `1` - CO_OWNER \\n* `2` - EDITOR \\n* `3` - COMMENTER \\n* `4` - VIEWER \\n\",\"example\":0,\"enum\":[0,1,2,3,4],\"x-enum-descriptions\":[\"OWNER\",\"CO_OWNER\",\"EDITOR\",\"COMMENTER\",\"VIEWER\"]},\"collaborator_type\":{\"type\":\"integer\",\"description\":\"The collaborator type. \\nEnum: \\n* `0` - SAME_ACCOUNT_USER \\n* `1` - EXTERNAL_USER \\n* `2` - CHANNEL \\n\",\"example\":0,\"enum\":[0,1,2],\"x-enum-descriptions\":[\"SAME_ACCOUNT_USER\",\"EXTERNAL_USER\",\"CHANNEL\"]},\"email\":{\"type\":\"string\",\"description\":\"`nullable` Email. This response only returns if the `collaborator_type` is `0`. If the `collaborator_type` is **not** `0`, this returns null value.\",\"example\":\"user@example.com\"}},\"description\":\"A user or a team chat channel who works jointly on a whiteboard.\"}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111804` <br>\\n Permission is needed to access this document. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Read\"],\"x-macro-scopes\":[\"whiteboard_collaborator:read:admin\"],\"x-granular-scopes\":[\"whiteboard:read:list_collaborators:admin\"]}},\"post\":{\"tags\":[\"Collaborator\"],\"summary\":\"Share a whiteboard to new users or team chat channels.\",\"description\":\"Add collaborators to a whiteboard.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_collaborator:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:collaborator:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\",\"operationId\":\"AddAWhiteboardCollaborator\",\"parameters\":[{\"name\":\"whiteboardId\",\"in\":\"path\",\"description\":\"The ID of the whiteboard.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"HY8-AC_HSou4SXY1S-OmOg\"}}],\"requestBody\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"skip_notifications\":{\"type\":\"boolean\",\"description\":\" **Default is false (notifications will be sent)**.\\n If set to `true`, collaborators and owners will not receive direct messages or emails when the whiteboard is shared.\",\"example\":false},\"invite_message\":{\"maxLength\":280,\"minLength\":0,\"type\":\"string\",\"description\":\"The custom message to include with the whiteboard invitation.\",\"example\":\"I'm glad to invite you to join this whiteboard.\"},\"collaborators\":{\"maxItems\":100,\"minItems\":1,\"type\":\"array\",\"description\":\"Array of collaborator objects to be added to the whiteboard.\",\"items\":{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\",\"description\":\"**User Email Address**\\nThe email address of the user you want to share the whiteboard with.\\nThis field is **required** when sharing a whiteboard with an individual user.\\nIt **must be left empty** when sharing via a channel.\\n**Note**: email and channel_id cannot be set at the same time.\",\"example\":\"user@example.com.\"},\"channel_id\":{\"type\":\"string\",\"description\":\"**Channel ID**\\nThe unique identifier of the channel you want to share the whiteboard with.\\nThis field is **required** when sharing a whiteboard via a channel.\\nIt **must be left empty** when sharing with an individual user by email.\\n**Note**: channel_id and email cannot be set at the same time.\",\"example\":\"cabc1234567defghijkl01234\"},\"role\":{\"type\":\"integer\",\"description\":\"Collaborator role type. \\nWhen assigning a new `OWNER` or `CO_OWNER`, the specified user must belong to the same account as the original `OWNER`. If a new owner is designated for a whiteboard, the original owner will automatically be demoted to `CO_OWNER`.\\n\\nRole Enum Values: \\n* `0` - OWNER \\n* `1` - CO_OWNER \\n* `2` - EDITOR \\n* `3` - COMMENTER \\n* `4` - VIEWER \",\"example\":1,\"enum\":[0,1,2,3,4],\"x-enum-descriptions\":[\"OWNER\",\"CO_OWNER\",\"EDITOR\",\"COMMENTER\",\"VIEWER\"]}},\"description\":\"A user or a team chat channel who works jointly on a whiteboard. \\n* `Email` or `ChannelId` is required according to the audience `type`.\"}}},\"description\":\"Users or team chat channels to be added as collaborators to the whiteboard.\"}}}},\"responses\":{\"201\":{\"description\":\"**HTTP Status Code:** `201`   \\n\\n Whiteboard added collaborators returned.\",\"content\":{\"application/json\":{\"schema\":{\"maxItems\":100,\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"collaborator_id\":{\"type\":\"string\",\"description\":\"The collaborator's unique identifier. This field is not nullable.\",\"example\":\"x4g6hy789k0o878k\"},\"collaborator_name\":{\"type\":\"string\",\"description\":\"The collaborator's display name.\",\"example\":\"Jack\"},\"user_id\":{\"type\":\"string\",\"description\":\"`nullable` User ID. This response only returns if the `collaborator_type` is `0`. If the `collaborator_type` is **not** `0`, this returns null value.\",\"example\":\"wf3h7uo1dk0v53bl\"},\"channel_id\":{\"type\":\"string\",\"description\":\"`nullable` Team chat channel ID. This response only returns if the `collaborator_type` is `2`. If the `collaborator_type` is **not** `2`, this returns null value.\",\"example\":\"cabc1234567defghijkl01234\"},\"avatar\":{\"type\":\"string\",\"description\":\"Collaborator profile picture.\",\"example\":\"https://content.zoom.us/swerwerw.jpeg\"},\"role\":{\"type\":\"integer\",\"description\":\"Collaborator role type. \\nEnum: \\n* `0` - OWNER \\n* `1` - CO_OWNER \\n* `2` - EDITOR \\n* `3` - COMMENTER \\n* `4` - VIEWER \\n\",\"example\":0,\"enum\":[0,1,2,3,4],\"x-enum-descriptions\":[\"OWNER\",\"CO_OWNER\",\"EDITOR\",\"COMMENTER\",\"VIEWER\"]},\"collaborator_type\":{\"type\":\"integer\",\"description\":\"Collaborator type. \\nEnum: \\n* `0` - SAME_ACCOUNT_USER \\n* `1` - EXTERNAL_USER \\n* `2` - CHANNEL \\n\",\"example\":0,\"enum\":[0,1,2],\"x-enum-descriptions\":[\"SAME_ACCOUNT_USER\",\"EXTERNAL_USER\",\"CHANNEL\"]}},\"description\":\"A user or team chat channel who works jointly on a whiteboard.\"}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n \"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n \"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Edit\"],\"x-macro-scopes\":[\"whiteboard_collaborator:write:admin\"],\"x-granular-scopes\":[\"whiteboard:write:collaborator:admin\"]}},\"patch\":{\"tags\":[\"Collaborator\"],\"summary\":\"Update whiteboard collaborators\",\"description\":\"Use this API to update whiteboard collaborators. \\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_collaborator:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:update:collaborator:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\",\"operationId\":\"UpdateAWhiteboardCollaborator\",\"parameters\":[{\"name\":\"whiteboardId\",\"in\":\"path\",\"description\":\"The ID of the whiteboard.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"HY8-AC_HSou4SXY1S-OmOg\"}}],\"requestBody\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"collaborators\":{\"maxItems\":100,\"minItems\":1,\"type\":\"array\",\"description\":\"An array of collaborator objects.\",\"items\":{\"type\":\"object\",\"properties\":{\"collaborator_id\":{\"type\":\"string\",\"description\":\"The collaborator's unique ID `not nullable` .\",\"example\":\"x4g6hy789k0o878k\"},\"role\":{\"type\":\"integer\",\"description\":\"Collaborator role type. \\nWhen assigning a new `OWNER` or `CO_OWNER`, the specified user must belong to the same account as the original `OWNER`. If a new owner is designated for a whiteboard, the original owner will automatically be demoted to `CO_OWNER`.\\n\\nRole Enum Values: \\n* `0` - OWNER \\n* `1` - CO_OWNER \\n* `2` - EDITOR \\n* `3` - COMMENTER \\n* `4` - VIEWER \",\"example\":0,\"enum\":[0,1,2,3,4],\"x-enum-descriptions\":[\"OWNER\",\"CO_OWNER\",\"EDITOR\",\"COMMENTER\",\"VIEWER\"]}}}}},\"description\":\"Update collaborators permission of a whiteboard.\"}}}},\"responses\":{\"204\":{\"description\":\"**HTTP Status Code**: `204`   \\n \\nWhiteboard collaborators updated successfully.\"},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `34002` <br>\\n Invalid collaboratorId {collaboratorId}. <br>\\n\"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111804` <br>\\n Permission is needed to access this document. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `34000` <br>\\n The resource {whiteboardId} not exist. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Edit\"],\"x-macro-scopes\":[\"whiteboard_collaborator:write:admin\"],\"x-granular-scopes\":[\"whiteboard:update:collaborator:admin\"]}}},\"/whiteboards/{whiteboardId}/collaborator/{collaboratorId}\":{\"delete\":{\"tags\":[\"Collaborator\"],\"summary\":\"Remove the collaborator from a whiteboard\",\"description\":\"Removes a collaborator from a whiteboard. \\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_collaborator:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:delete:collaborator:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\",\"operationId\":\"DeleteAWhiteboardCollaborator\",\"parameters\":[{\"name\":\"whiteboardId\",\"in\":\"path\",\"description\":\"The ID of the whiteboard.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"HY8-AC_HSou4SXY1S-OmOg\"}},{\"name\":\"collaboratorId\",\"in\":\"path\",\"description\":\"The ID of the whiteboard collaborator.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"sWvlvz-G5XFQxpfxEBB0PfVALRdcpHdTAqwqigrOPb1TgkxtFJI.3Z6nozeexgNAEecI\"}}],\"responses\":{\"204\":{\"description\":\"**HTTP Status Code**: `204`   \\n \\nWhiteboard collaborators deleted successfully.\"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111804` <br>\\n Permission is needed to access this document. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `34002` <br>\\n Resource not found.  <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Edit\"],\"x-macro-scopes\":[\"whiteboard_collaborator:write:admin\"],\"x-granular-scopes\":[\"whiteboard:delete:collaborator:admin\"]}}},\"/whiteboards\":{\"get\":{\"tags\":[\"Document\"],\"summary\":\"List all whiteboards\",\"description\":\"Use this API to list whiteboards of current user (**Required Scopes**: `whiteboard:read`) or list all whiteboards under the account (**Required Scopes**: `whiteboard:read:admin`).\\n\\n**Note:** `user_id` can intake `Zoom user ID` or `email` which only applies when scope = `whiteboard:read:admin`. Otherwise, the `user_id` can only intake `me`.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:admin`,`whiteboard:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:list_whiteboards:admin`,`whiteboard:read:list_whiteboards`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\",\"operationId\":\"ListWhiteboards\",\"parameters\":[{\"name\":\"search_key\",\"in\":\"query\",\"description\":\"The name of a whiteboard. If this parameter is provided, only the matched whiteboard will be returned. Otherwise, all whiteboards will be returned.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"PGsQ3nwITmyTG4JkFNWJcg\"}},{\"name\":\"user_id\",\"in\":\"query\",\"description\":\"The email address or user ID of the whiteboard owner. If this parameter is provided, only the whiteboard owned by the provided user will be returned. \\nIf set to `me`, will return the matched whiteboards of current user.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"me\"}},{\"name\":\"date_filter_type\",\"in\":\"query\",\"description\":\"The whiteboard date filter type. \\nEnum:\\n* `created_date` - Whiteboard create date. \\n* `modified_date` - Whiteboard modify date.\\n\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"modified_date\",\"default\":\"modified_date\",\"enum\":[\"created_date\",\"modified_date\"]}},{\"name\":\"from\",\"in\":\"query\",\"description\":\"The queried start date and time in `yyyy-MM-dd'T'HH:mm:ss'Z'` UTC format for the created date range for which you would like to retrieve whiteboards. The maximum range can be a month. If no value is provided for this field, the default will be current date. For example, if you make the API request on June 30, 2020, without providing the &ldquo;from&rdquo; and &ldquo;to&rdquo; parameters, by default the value of 'from' field will be &ldquo;2020-06-30&rdquo; and the value of the 'to' field will be &ldquo;2020-07-01&rdquo;.\",\"required\":false,\"schema\":{\"type\":\"string\",\"format\":\"date-time\",\"example\":\"2022-03-08T16:00:00Z\"}},{\"name\":\"to\",\"in\":\"query\",\"description\":\"The queried end date and time in `yyyy-MM-dd'T'HH:mm:ss'Z'` UTC format. \",\"required\":false,\"schema\":{\"type\":\"string\",\"format\":\"date-time\",\"example\":\"2022-03-08T16:00:00Z\"}},{\"name\":\"page_size\",\"in\":\"query\",\"description\":\"The number of records returned from a single API call.\",\"required\":false,\"schema\":{\"maximum\":50,\"type\":\"integer\",\"example\":10,\"default\":10}},{\"name\":\"next_page_token\",\"in\":\"query\",\"description\":\"The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42\"}},{\"name\":\"project_id\",\"in\":\"query\",\"description\":\"Project ID to search for.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"N6XZFDEQTkWSsMsmO_M1XQ\"}}],\"responses\":{\"200\":{\"description\":\"**HTTP Status Code:** `200`   \\n \\nWhiteboard list returned.\",\"content\":{\"application/json\":{\"schema\":{\"title\":\"Whiteboard List\",\"description\":\"The whiteboard list.\",\"allOf\":[{\"type\":\"object\",\"properties\":{\"whiteboards\":{\"maxItems\":50,\"type\":\"array\",\"description\":\"The information about the whiteboard list.\",\"items\":{\"allOf\":[{\"type\":\"object\",\"properties\":{\"whiteboard_id\":{\"type\":\"string\",\"description\":\"Whiteboard ID\",\"example\":\"48Z4ppMfQJWONeVWFCcVzA\"},\"whiteboard_name\":{\"type\":\"string\",\"description\":\"Whiteboard name\",\"example\":\"Q3 Plan\",\"default\":\"Untitled\"},\"locked\":{\"type\":\"boolean\",\"description\":\"Indicates whether the whiteboard is locked or not.\\n\\n`true`: The whiteboard is locked.   \\n \\n\\n`false`: The whiteboard is unlocked\",\"example\":false,\"default\":false},\"trashed\":{\"type\":\"boolean\",\"description\":\"Indicates whether the whiteboard is moved to trash or not.\\n\\n`true`: The whiteboard is moved to trash.   \\n \\n\\n`false`: The whiteboard isn't moved to trash\",\"example\":false,\"default\":false},\"readonly\":{\"type\":\"boolean\",\"description\":\"Indicates whether the whiteboard is editable or not. If a whiteboard is locked, this field will set `true`. \\n\\n`true`: The whiteboard is editable.   \\n \\n\\n`false`: The whiteboard is readonly\",\"example\":false,\"default\":false},\"non_cloud\":{\"type\":\"boolean\",\"description\":\"Indicates whether the whiteboard is allowed to be saved to cloud or not.\\n\\n`true`: The whiteboard is a saved to cloud.   \\n \\n\\n`false`: The whiteboard isn't saved to cloud\",\"example\":false,\"default\":false},\"created_date\":{\"type\":\"string\",\"description\":\"Whiteboard create date-time in UTC/GMT. Example: &quot;2020-03-31T12:02:00Z&quot;\",\"format\":\"date-time\",\"example\":\"2022-03-25T07:29:29Z\"},\"modified_date\":{\"type\":\"string\",\"description\":\"Whiteboard update date-time in UTC/GMT. Example: &quot;2020-03-31T12:02:00Z&quot;\",\"format\":\"date-time\",\"example\":\"2022-03-25T07:29:29Z\"},\"share_link_setting\":{\"type\":\"object\",\"properties\":{\"share_scope\":{\"type\":\"integer\",\"description\":\"The access type of whiteboard share link. \\n* Enum: \\n* `0` - `Restricted`, Only people added can open with this link \\n* `1` - `Same account` \\n* `2` - Anyone \\n* `3` - SAME_ACCOUNT_EXTERNAL_AUTH \\n\",\"example\":0,\"default\":0,\"enum\":[0,1,2,3],\"x-enum-descriptions\":[\"Restricted. Only people added can open with this link\",\"Same account\",\"Anyone\",\"Same account external auth\"]},\"share_role\":{\"type\":\"integer\",\"description\":\"The access level of whiteboard share link. This field need to used with `share_scope`. If `share_scope` set `0` - `Restricted`, this field is null. If `share_scope` set `1` - `Same account` or `2` - `Anyone`, this filed can be `2` - `Editor`, `3` - `Commenter` or `4` - `Viewer`. \\n* Enum: \\n* `2` - `Editor`\\n* `3` - `Commenter` \\n* `4` - Viewer \\n\",\"example\":2,\"enum\":[2,3,4],\"x-enum-descriptions\":[\"Editor\",\"Commenter\",\"Viewer\"]}},\"description\":\"The access level, access type of current whiteboard share link.\"},\"advanced_share_setting\":{\"type\":\"object\",\"properties\":{\"can_share\":{\"type\":\"boolean\",\"description\":\"Indicate whether users can share at or below their permission level or not. \\n\\n`true`: Users can share at or below their permission level.   \\n \\n\\n`false`: Users cannot share at or below their permission level.\",\"example\":true},\"share_to_external\":{\"type\":\"boolean\",\"description\":\"Indicate whether internal users can share with external users or not. \\n\\n`true`: Internal users can share with external users.   \\n \\n\\n`false`: Internal users cannot share with external users.\",\"example\":true},\"external_can_share\":{\"type\":\"boolean\",\"description\":\"Indicate whether external user can share the whiteboard to others or not. \\n\\n`true`: External user can share the whiteboard to others.   \\n \\n\\n`false`:External user cannot share the whiteboard to others.\",\"example\":true}},\"description\":\"Whiteboard advanced share setting.\"},\"project_id\":{\"type\":\"string\",\"description\":\"qiMWddmtRxqwSFrK22lJPg\",\"example\":\"The Project ID\"},\"created_by_display_name\":{\"type\":\"string\",\"description\":\"The creator display name of the whiteboard.\",\"example\":\"Jack\"}},\"description\":\"Whiteboard object\"}]}},\"next_page_token\":{\"type\":\"string\",\"description\":\"The next page token paginates through a large set of results. A next page token is returned when the available results exceed the current page size. The expiration period for this token is 15 minutes.\",\"example\":\"qUEQqB1V0HVhJmwKFQrGOD\"},\"page_size\":{\"type\":\"integer\",\"description\":\"The number of records returned per page in a single API call.\",\"example\":10},\"total_records\":{\"type\":\"integer\",\"description\":\"The total number of records found.\",\"example\":2}}}]}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Read\"],\"x-macro-scopes\":[\"whiteboard:read:admin\",\"whiteboard:read\"],\"x-granular-scopes\":[\"whiteboard:read:list_whiteboards:admin\",\"whiteboard:read:list_whiteboards\"]}},\"post\":{\"tags\":[\"Document\"],\"summary\":\"Create a new whiteboard\",\"description\":\"Creates a new whiteboard for a user.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:whiteboard`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"newWhiteboardCreate\",\"requestBody\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"name\":{\"type\":\"string\",\"description\":\"The whiteboard name.\",\"example\":\"Q3 Plan\",\"default\":\"Untitled\"}},\"description\":\"Create a new whiteboard request body.\"}}}},\"responses\":{\"201\":{\"description\":\"**HTTP Status Code:** `201`   \\n \\nWhiteboard created successfully.\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\",\"example\":\"48Z4ppMfQJWONeVWFCcVzA\"},\"whiteboard_name\":{\"type\":\"string\",\"description\":\"The whiteboard name.\",\"example\":\"Q3 Plan\",\"default\":\"Untitled\"},\"locked\":{\"type\":\"boolean\",\"description\":\"Indicates whether the whiteboard is locked or not.\\n\\n`true`: The whiteboard is locked.   \\n \\n\\n`false`: The whiteboard is unlocked.\",\"example\":false,\"default\":false},\"trashed\":{\"type\":\"boolean\",\"description\":\"Indicates whether the whiteboard is moved to trash or not.\\n\\n`true`: The whiteboard is moved to trash.   \\n \\n\\n`false`: The whiteboard was not moved to trash.\",\"example\":false,\"default\":false},\"readonly\":{\"type\":\"boolean\",\"description\":\"Indicates whether the whiteboard is editable or not. If a whiteboard is locked, this field will set `true`. \\n\\n`true`: The whiteboard is editable.   \\n \\n\\n`false`: The whiteboard is read only.\",\"example\":false,\"default\":false},\"non_cloud\":{\"type\":\"boolean\",\"description\":\"Indicates whether the whiteboard is allowed to be saved to cloud or not.\\n\\n`true`: The whiteboard is a saved to cloud.   \\n \\n\\n`false`: The whiteboard was not saved to cloud.\",\"example\":false,\"default\":false},\"created_date\":{\"type\":\"string\",\"description\":\"The whiteboard create date-time in UTC/GMT. Example: &quot;`2020-03-31T12:02:00Z`&quot;\",\"format\":\"date-time\",\"example\":\"2022-03-25T07:29:29Z\"},\"modified_date\":{\"type\":\"string\",\"description\":\"The whiteboard update date-time in UTC/GMT. Example: &quot;2020-03-31T12:02:00Z&quot;\",\"format\":\"date-time\",\"example\":\"2022-03-25T07:29:29Z\"},\"share_link_setting\":{\"type\":\"object\",\"properties\":{\"share_scope\":{\"type\":\"integer\",\"description\":\"The access type of whiteboard share link. \\n* Enum: \\n* `0` - `Restricted`, Only people added can open with this link \\n* `1` - `Same account` \\n* `2` - Anyone \\n* `3` - SAME_ACCOUNT_EXTERNAL_AUTH \\n\",\"example\":0,\"default\":0,\"enum\":[0,1,2,3],\"x-enum-descriptions\":[\"Restricted. Only people added can open with this link\",\"Same account\",\"Anyone\",\"Same account external auth\"]},\"share_role\":{\"type\":\"integer\",\"description\":\"The access level of whiteboard share link. This field need to used with `share_scope`. If `share_scope` set `0` - `Restricted`, this field is null. If `share_scope` set `1` - `Same account` or `2` - `Anyone`, this filed can be `2` - `Editor`, `3` - `Commenter` or `4` - `Viewer`. \\n* Enum: \\n* `2` - `Editor`\\n* `3` - `Commenter` \\n* `4` - Viewer \\n\",\"example\":2,\"enum\":[2,3,4],\"x-enum-descriptions\":[\"Editor\",\"Commenter\",\"Viewer\"]}},\"description\":\"The access level, access type of current whiteboard share link.\"},\"advanced_share_setting\":{\"type\":\"object\",\"properties\":{\"can_share\":{\"type\":\"boolean\",\"description\":\"Indicate whether users can share at or below their permission level or not. \\n\\n`true`: Users can share at or below their permission level.   \\n \\n\\n`false`: Users cannot share at or below their permission level.\",\"example\":true},\"share_to_external\":{\"type\":\"boolean\",\"description\":\"Indicate whether internal users can share with external users or not. \\n\\n`true`: Internal users can share with external users.   \\n \\n\\n`false`: Internal users cannot share with external users.\",\"example\":true},\"external_can_share\":{\"type\":\"boolean\",\"description\":\"Indicate whether external user can share the whiteboard to others or not. \\n\\n`true`: External user can share the whiteboard to others.   \\n \\n\\n`false`:External user cannot share the whiteboard to others.\",\"example\":true}},\"description\":\"Whiteboard advanced share setting.\"}},\"description\":\"The whiteboard object.\"}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[],\"x-macro-scopes\":[\"whiteboard:write\"],\"x-granular-scopes\":[\"whiteboard:write:whiteboard\"]}}},\"/whiteboards/{whiteboardId}\":{\"get\":{\"tags\":[\"Document\"],\"summary\":\"Get a whiteboard\",\"description\":\"Returns a whiteboard's information.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:admin`,`whiteboard:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:whiteboard:admin`,`whiteboard:read:whiteboard`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\",\"operationId\":\"GetAWhiteboard\",\"parameters\":[{\"name\":\"whiteboardId\",\"in\":\"path\",\"description\":\"The whiteboard's ID.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"HY8-AC_HSou4SXY1S-OmOg\"}}],\"responses\":{\"200\":{\"description\":\"**HTTP Status Code:** `200`   \\n \\nWhiteboard object returned.\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\",\"example\":\"48Z4ppMfQJWONeVWFCcVzA\"},\"whiteboard_name\":{\"type\":\"string\",\"description\":\"The whiteboard name.\",\"example\":\"Q3 Plan\",\"default\":\"Untitled\"},\"locked\":{\"type\":\"boolean\",\"description\":\"Indicates whether the whiteboard is locked or not.\\n\\n`true`-  The whiteboard is locked.   \\n \\n\\n`false` - The whiteboard is unlocked.\",\"example\":false,\"default\":false},\"trashed\":{\"type\":\"boolean\",\"description\":\"Indicates whether the whiteboard is moved to trash or not.\\n\\n`true` - The whiteboard is moved to trash.   \\n \\n\\n`false` - The whiteboard isn't moved to trash.\",\"example\":false,\"default\":false},\"readonly\":{\"type\":\"boolean\",\"description\":\"Indicates whether the whiteboard is editable or not. If a whiteboard is locked, this field will be set to `true`. \\n\\n`true` - The whiteboard is editable.   \\n \\n\\n`false` - The whiteboard is read only.\",\"example\":false,\"default\":false},\"non_cloud\":{\"type\":\"boolean\",\"description\":\"Indicates whether the whiteboard is allowed to be saved to cloud or not.\\n\\n`true` - The whiteboard is a saved to cloud.   \\n \\n\\n`false` - The whiteboard isn't saved to cloud.\",\"example\":false,\"default\":false},\"created_date\":{\"type\":\"string\",\"description\":\"The whiteboard creation date and time in UTC/GMT. Example: `2020-03-31T12:02:00Z`\",\"format\":\"date-time\",\"example\":\"2022-03-25T07:29:29Z\"},\"modified_date\":{\"type\":\"string\",\"description\":\"The whiteboard update date and time in UTC/GMT. Example: `2020-03-31T12:02:00Z`\",\"format\":\"date-time\",\"example\":\"2022-03-25T07:29:29Z\"},\"share_link_setting\":{\"type\":\"object\",\"properties\":{\"share_scope\":{\"type\":\"integer\",\"description\":\"The access type of whiteboard sharing link. \\n* Enum: \\n* `0` - `Restricted`. Only people added can open with this link. \\n* `1` - `Same account` \\n* `2` - Anyone \\n* `3` - `SAME_ACCOUNT_EXTERNAL_AUTH` \\n\",\"example\":0,\"default\":0,\"enum\":[0,1,2,3],\"x-enum-descriptions\":[\"Restricted. Only people added can open with this link\",\"Same account\",\"Anyone\",\"Same account external auth\"]},\"share_role\":{\"type\":\"integer\",\"description\":\"The access level of whiteboard share link. This field need to used with `share_scope`. If `share_scope` set `0` - `Restricted`. This field is null. If `share_scope` set `1` - `Same account` or `2` - `Anyone`, this field can be `2` - `Editor`, `3` - `Commenter` or `4` - `Viewer`. \\n* Enum: \\n* `2` - `Editor`\\n* `3` - `Commenter` \\n* `4` - Viewer \\n\",\"example\":2,\"enum\":[2,3,4],\"x-enum-descriptions\":[\"Editor\",\"Commenter\",\"Viewer\"]}},\"description\":\"The access level, access type of current whiteboard share link.\"},\"advanced_share_setting\":{\"type\":\"object\",\"properties\":{\"can_share\":{\"type\":\"boolean\",\"description\":\"Indicate whether users can share at or below their permission level or not. \\n\\n`true`: Users can share at or below their permission level.   \\n \\n\\n`false`: Users cannot share at or below their permission level.\",\"example\":true},\"share_to_external\":{\"type\":\"boolean\",\"description\":\"Indicate whether internal users can share with external users or not. \\n\\n`true`: Internal users can share with external users.   \\n \\n\\n`false`: Internal users cannot share with external users.\",\"example\":true},\"external_can_share\":{\"type\":\"boolean\",\"description\":\"Indicate whether external user can share the whiteboard to others or not. \\n\\n`true`: External user can share the whiteboard to others.   \\n \\n\\n`false`:External user cannot share the whiteboard to others.\",\"example\":true}},\"description\":\"The whiteboard advanced share setting.\"},\"share_link\":{\"type\":\"string\",\"description\":\"The whiteboard share link.\",\"example\":\"https://zoom.us/wb/doc/T-E0zR9NQWSDuvXd-Crt9A\"},\"preview_link\":{\"type\":\"string\",\"description\":\"The whiteboard preview link. This link's validity period is `2 hours`, and the maximum number of downloads is `99 times`. Only those who have the `whiteboard:read:whiteboard` scope or are collaborators of the whiteboard will see this field returned.\",\"example\":\"https://a01file.zoom.us/file/xxxxx/attach/preview/192x108?jwt=xxx\"},\"created_by_display_name\":{\"type\":\"string\",\"description\":\"The creator display name of the whiteboard.\",\"example\":\"Tom\"},\"modified_by_display_name\":{\"type\":\"string\",\"description\":\"The latest modify user display name of the whiteboard.\",\"example\":\"Jack\"},\"embedded_content_types\":{\"type\":\"array\",\"description\":\"The embedded content type list.\",\"items\":{\"maximum\":2,\"minimum\":0,\"type\":\"integer\",\"description\":\"The type of embeded content in whiteboard. it's an enum contains `0`, `1`, `2`.\\nThe description of the enums:\\n- **0** - **IMAGE**.\\n- **1** - **DOCUMENT**, this would include PDFs, PPTX, DOCX, GOOGLE DOCS, SHEETS and SLIDES.\\n- **2** - **CARD**, this would include Jira, Asana and Azure DevOps.\",\"format\":\"int32\",\"example\":0,\"default\":0,\"enum\":[0,1,2],\"x-enum-descriptions\":[\"IMAGE\",\"DOCUMENT\",\"CARD\"]}},\"project_id\":{\"type\":\"string\",\"description\":\"qiMWddmtRxqwSFrK22lJPg\",\"example\":\"The Project ID\"}},\"description\":\"Whiteboard object\"}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111800` <br>\\n Permission is needed to access this document. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Read\"],\"x-macro-scopes\":[\"whiteboard:read:admin\",\"whiteboard:read\"],\"x-granular-scopes\":[\"whiteboard:read:whiteboard:admin\",\"whiteboard:read:whiteboard\"]}},\"put\":{\"tags\":[\"Document\"],\"summary\":\"Update whiteboard basic information\",\"description\":\"Use this API to rename a whiteboard, update whiteboard status (locked or unlocked). \\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:update:whiteboard:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\",\"operationId\":\"UpdateAWhiteboardMetadata\",\"parameters\":[{\"name\":\"whiteboardId\",\"in\":\"path\",\"description\":\"The ID of the whiteboard.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"HY8-AC_HSou4SXY1S-OmOg\"}}],\"requestBody\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"name\":{\"type\":\"string\",\"description\":\"The whiteboard name.\",\"example\":\"Q3 Plan\",\"default\":\"Untitled\"},\"locked\":{\"type\":\"boolean\",\"description\":\"Indicates whether the whiteboard is locked or not.\\n\\n`true`: The whiteboard is locked.   \\n \\n\\n`false`: The whiteboard is unlocked.\",\"example\":false,\"default\":false}},\"description\":\"Update a whiteboard request body\"}}}},\"responses\":{\"204\":{\"description\":\"**HTTP Status Code**: `204`   \\n \\nWhiteboard updated successfully.\"},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111800` <br>\\n Permission is needed to access this document. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Edit\"],\"x-macro-scopes\":[\"whiteboard:write:admin\"],\"x-granular-scopes\":[\"whiteboard:update:whiteboard:admin\"]}},\"delete\":{\"tags\":[\"Document\"],\"summary\":\"Delete a whiteboard\",\"description\":\"Deletes a whiteboard.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:admin`,`whiteboard:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:delete:whiteboard:admin`,`whiteboard:delete:whiteboard`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\",\"operationId\":\"DeleteAWhiteboard\",\"parameters\":[{\"name\":\"whiteboardId\",\"in\":\"path\",\"description\":\"The ID of the whiteboard.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"HY8-AC_HSou4SXY1S-OmOg\"}}],\"responses\":{\"204\":{\"description\":\"**HTTP Status Code**: `204`   \\n \\nWhiteboard Deleted.\"},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111804` <br>\\n Permission is needed to access this document. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Edit\"],\"x-macro-scopes\":[\"whiteboard:write:admin\",\"whiteboard:write\"],\"x-granular-scopes\":[\"whiteboard:delete:whiteboard:admin\",\"whiteboard:delete:whiteboard\"]}}},\"/whiteboards/export\":{\"post\":{\"tags\":[\"Export\"],\"summary\":\"Create whiteboard export\",\"description\":\" Generates the export for the provided whiteboards. This process will generate a PDF of whiteboard projects along with an audit log, bundling them into a ZIP file.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_export:write:admin`,`whiteboard_export:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:export`,`whiteboard:write:export:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\",\"operationId\":\"Createwhiteboardsexport\",\"requestBody\":{\"content\":{\"application/json\":{\"schema\":{\"required\":[\"whiteboard_ids\"],\"type\":\"object\",\"properties\":{\"whiteboard_ids\":{\"maxItems\":50,\"minItems\":1,\"uniqueItems\":true,\"type\":\"array\",\"description\":\"The list of whiteboard IDs. The list size must be **less than** 50, if more than 50, the API will **intercept** the first 50 IDs.\",\"items\":{\"type\":\"string\",\"description\":\"The ID of the whiteboard.\",\"example\":\"HY8-AC_HSou4SXY1S-OmOg\"}}}}}}},\"responses\":{\"200\":{\"description\":\"**HTTP Status Code:** `200`    Successfully sent a request to generate whiteboard export\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"task_id\"],\"type\":\"object\",\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"The unique identifier of the generated file, which expires is **30 days**. Use this task_id to query the progress of file generation (For more details, please refer to\uff1a[**Get whiteboard export generation status**](https://developers.zoom.us/docs/zoom-whiteboard/apis/#operation/Getwhiteboardexportdatagenerationstatus). Once the file is ready, you can download it using the user's OAuth token. For example: `https://{base_domain}/v2/whiteboards/export/task/{taskId} --header 'Authorization: Bearer {{OAuth-access-token}}` .\",\"example\":\"f7f3cdd37bf74550afe7cdf79128a574\"}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111804` <br>\\n Permission is needed to access this document. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `34002` <br>\\n The whiteboard {whiteboard_id} not found. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardContent:Read\",\"WhiteboardManagement:Read\"],\"x-macro-scopes\":[\"whiteboard_export:write:admin\",\"whiteboard_export:write\"],\"x-granular-scopes\":[\"whiteboard:write:export\",\"whiteboard:write:export:admin\"]}}},\"/whiteboards/export/task/{taskId}\":{\"get\":{\"tags\":[\"Export\"],\"summary\":\"Download whiteboard export\",\"description\":\"Use this API to download the whiteboard export.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_content:read:admin`,`whiteboard_content:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:export`,`whiteboard:read:export:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\",\"operationId\":\"Downloadwhiteboardexport\",\"parameters\":[{\"name\":\"taskId\",\"in\":\"path\",\"description\":\"The unique identifier of the whiteboards file. The `task_id` generated by [**Create whiteboard export**](https://developers.zoom.us/docs/zoom-whiteboard/apis/#operation/Createwhiteboardsexport) interface.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"f7f3cdd37bf74550afe7cdf79128a574\"}}],\"responses\":{\"200\":{\"description\":\"Returns a file stream, with the file type of ZIP.\"},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `34002` <br>\\n The task {task_id} not found. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardContent:Read\",\"WhiteboardManagement:Read\"],\"x-macro-scopes\":[\"whiteboard_content:read:admin\",\"whiteboard_content:read\"],\"x-granular-scopes\":[\"whiteboard:read:export\",\"whiteboard:read:export:admin\"]}}},\"/whiteboards/export/task/{taskId}/status\":{\"get\":{\"tags\":[\"Export\"],\"summary\":\"Get whiteboard export generation status\",\"description\":\"Use this API to query the status of the whiteboard export generation. Before using this API, confirm that you have invoked the interface for generating the whiteboard file  [**Create whiteboard export**](https://developers.zoom.us/docs/zoom-whiteboard/apis/#operation/Createwhiteboardsexport) and obtained the `task_id`.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_content:read:admin`,`whiteboard_content:read`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:export`,`whiteboard:read:export:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\",\"operationId\":\"Getwhiteboardexportdatagenerationstatus\",\"parameters\":[{\"name\":\"taskId\",\"in\":\"path\",\"description\":\"The unique identifier of the whiteboard export. \",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"adcc655d3a3549e692959737fe2a7a0d\"}}],\"responses\":{\"200\":{\"description\":\"Whiteboard export generation status.\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"status\",\"total\"],\"type\":\"object\",\"properties\":{\"processed\":{\"maxItems\":50,\"minItems\":0,\"type\":\"array\",\"description\":\"List of processed whiteboard IDs.\",\"items\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\",\"example\":\"48Z4ppMfQJWONeVWFCcVzA\"}},\"status\":{\"type\":\"string\",\"description\":\"Whiteboard export generation status. \\n* Enum:\\n* `processing` - Generating whiteboard export\\n* `successed` - Whiteboard export generated successfully(as long as one is successful, it counts as a success)\\n* `failed` - Whiteboard export generation failed\",\"example\":\"processing\",\"enum\":[\"processing\",\"successed\",\"failed\"],\"x-enum-descriptions\":[\"Generating whiteboard file\",\"Whiteboard file generated successfully.\",\"Whiteboard file generation failed\"]},\"total\":{\"type\":\"integer\",\"description\":\"Total number of whiteboards requiring export generation.\",\"example\":3},\"failed\":{\"maxItems\":50,\"minItems\":0,\"type\":\"array\",\"description\":\"List of failed whiteboard IDs.\",\"items\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\",\"example\":\"UyKtcVj5REWMFT2qd-4qfg\"}},\"processing\":{\"maxItems\":50,\"minItems\":0,\"type\":\"array\",\"description\":\"List of whiteboards being processed.\",\"items\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\",\"example\":\"r2Qr1q4hSxu03MG3hk03uA\"}}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `34002` <br>\\n The task {task_id} not found. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardContent:Read\",\"WhiteboardManagement:Read\"],\"x-macro-scopes\":[\"whiteboard_content:read:admin\",\"whiteboard_content:read\"],\"x-granular-scopes\":[\"whiteboard:read:export\",\"whiteboard:read:export:admin\"]}}},\"/whiteboards/files\":{\"post\":{\"tags\":[\"File\"],\"summary\":\"Upload file for whiteboard import\",\"description\":\"Uploads a file to whiteboard.\\n**Notes**:\\n- Base URL: `https://fileapi.zoom.us/v2`.\\n- Rate limit: 20 requests per second per user or 2000 requests per second per IP address.\\n- The caller must support HTTP 30x redirects.\\n- The caller must retain the Authorization header when redirected to a different hostname.\\n- Supported file formats: `.pdf`, `.vsdx`.\\n- For an account-level [OAuth](https://developers.zoom.us/docs/integrations/create/) app, this API can only be used on behalf of a user who is assigned with a [role](https://support.zoom.com/hc/en/article?id=zm_kb&sysparm_article=KB0064983#h_80abad9b-86a8-492c-8a3b-5b0141a8c0a9) with **Edit** permission for Whiteboard Management.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_file:write:admin`,`whiteboard_file:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:file`,`whiteboard:write:file:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `HEAVY`\\n\\n**Not supported in Gov cluster** \",\"operationId\":\"Uploadfileforwhiteboardimport\",\"requestBody\":{\"content\":{\"multipart/form-data\":{\"schema\":{\"required\":[\"file\"],\"type\":\"object\",\"properties\":{\"file\":{\"type\":\"string\",\"description\":\"The upload file is in binary format. You can only send a maximum of 100 MB.\",\"format\":\"binary\",\"example\":\"Vm0wd2QyUXlVWGxWV0d4V1YwZDRWMVl3WkRSV01WbDNXa1JTVjAxV2JETlhhMUpUVmpBeFYySkVUbGhoTVVwVVZtcEJlRll5U2tWVWJHaG9UVlZ3VlZadGNFSmxSbGw1VTJ0V1ZXSkhhRzlVVmxaM1ZsWmFkR05GU214U2JHdzFWVEowVjFaWFNraGhSemxWVm14YU0xWnNXbUZrUjA1R1UyMTRVMkpIZHpGV1ZFb3dWakZhV0ZOcmFHaFNlbXhXVm0xNFlVMHhXbk5YYlVaclVqQTFSMVV5TVRSVk1rcElaSHBHVjFaRmIzZFdha1poVjBaT2NtRkhhRk5sYlhoWFZtMHhORmxWTUhoWGJrNVlZbFZhY2xWcVFURlNNVlY1VFZSU1ZrMXJjRWxhU0hCSFZqRmFSbUl6WkZkaGExcG9WakJhVDJOdFJraGhSazVzWWxob1dGWnRNSGhPUm14V1RVaG9XR0pyTlZsWmJGWmhZMVphZEdSSFJrNVNiRm93V2xWYVQxWlhTbFpqUldSYVRVWmFNMVpxU2t0V1ZrcFpXa1p3VjFKV2NIbFdWRUpoVkRKT2MyTkZhR3BTYkVwVVZteG9RMWRzV1hoWGJFNVRUV3hHTlZWdGRHdGhiRXAwVld4c1dtSkdXbWhaTW5oWFl6RldjbHBHWkdsU2JrSmFWMnhXWVZReFdsaFRiRnBZVmtWd1YxbHJXa3RUUmxweFUydGFiRlpzV2xwWGExcDNZa2RGZWxGcmJGaFhTRUpJVmtSS1UxWXhXblZWYldoVFlYcFdlbGRYZUc5aU1XUkhWMjVTVGxOSFVuTlZha0p6VGtaVmVXUkhkRmhTTUhCSlZsZDRjMWR0U2tkWGJXaGFUVlp3YUZwRlpGTlRSa3B5VGxaT2FWSnRPVE5XTW5oWFlqSkZlRmRZWkU1V1ZscFVXVlJHZDFZeGJISlhhM1JVVW14d2VGVnRNVWRWTWtwV1lrUmFXR0V4Y0hKWlZXUkdaVWRPU0U5V1pHaGhNSEJ2Vm10U1MxUXlVa2RUYmtwb1VqSm9WRmxZY0ZkbGJHUllaVWM1YVUxWFVraFdNalZUVkd4a1NGVnNXbFZXYkhCSVZGUkdVMVp0UmtoUFYyaHBVbGhDTmxkVVFtRmpNV1IwVTJ0a1dHSlhhR0ZVVmxwM1lVWndSbHBHVGxSU2EzQjVWR3hhVDJGV1NuUlBWRTVYVFc1b1dGbHFTa1psUm1SWldrVTFWMVpzY0ZWWFZsSkhaREZaZUdKSVNsaGhNMUpWVlcxNGQyVkdWblJOVldSV1RXdHdWMWxyVW1GWFIwVjRZMGhLV2xaWFVrZGFWV1JQVTBVNVYxcEhhR2hOU0VKMlZtMTBVMU14VVhsVmEyUlVZbXR3YjFWcVNtOVdSbXhaWTBaa2JHSkhVbGxhVldNMVlWVXhXRlZyYUZkTmFsWlVWa2Q0VDFOSFJrZFJiRnBwVmtWVmQxWnRjRWRWTVZwMFVtdG9VRlp0YUZSVVZXaERUbFphU0dWSFJtcE5WMUl3VlRKMGExZEhTbGhoUjBaVlZucFdkbFl3V25KbFJtUnlXa1prVjJFelFqWldhMlI2VFZaWmQwMVdXbWxsYTFwWVdXeG9RMVJHVW5KWGJFcHNVbTFTZWxsVldsTmhWa3AxVVd4d1YySllVbGhhUkVaYVpVZEtTVk5zYUdoTk1VcFdWbGN4TkdReVZrZFdibEpPVmxkU1YxUlhkSGRXTVd4eVZXMUdXRkl3VmpSWk1HaExWMnhhV0ZWclpHRldWMUpRVlRCVk5WWXhjRWhoUjJoT1UwVktNbFp0TVRCVk1VMTRWVmhzVm1FeVVsVlpiWFIzWWpGV2NWTnRPVmRTYlhoYVdUQmFhMkpIU2toVmJHeGhWbGROTVZsV1ZYaFhSbFp5WVVaa1RtRnNXbFZXYTJRMFlURk9SMVp1VGxoaVJscFlXV3RvUTFOV1drZFZhMlJXVFZad01GVnRkRzlWUmxsNVlVaENWbUpIYUVSVWJYaHJWbFpHZEZKdGNFNVdNVWwzVmxSS01HRXhaRWhUYkdob1VqQmFWbFp1Y0Zka2JGbDNWMjVLYkZKdFVubFhhMXByVmpKRmVsRnFXbGRoTWxJMlZGWmFhMUl4VG5OWGJYQlRWak5vV1ZkWGVHOVJNVkpIVlc1S1dHSkZjSE5WYlRGVFpXeHNWbGRzVG1oU1ZFWjZWVEkxYjFZeFdqWlJhbEpWWVRKU1NGVnFSbUZYVm5CSVlVWk9WMVpHV2xkV2JHTjRUa2RSZVZaclpGZGliRXBQVm14a1UxWXhVbGhrU0dSWFRWZDRlVlpYTVVkWFJrbDNWbXBTV2sxSGFFeFdNbmhoVjBaV2NscEhSbGRXTVVwUlZsUkNWazVXV1hoalJXaG9VakpvVDFVd1ZrdE5iRnAwVFZSQ1ZrMVZNVFJXVm1oelZtMUZlVlZzVmxwaVdGSXpXV3BHVjJOV1RuUlBWbVJUWWxob1lWZFVRbUZoTWtwSVUydG9WbUpIZUdoV2JHUk9UVlpzVjFaWWFGaFNiRnA1V1ZWYWExUnRSbk5YYkZaWFlUSlJNRlpFUms5VFJrcHlXa1pLYVZKdVFuZFdiWFJYVm0xUmVGZHVVbXBTVjFKWFZGWmFkMDFHVm5Sa1J6bFdVbXh3TUZsVldsTldWbHBZWVVWU1ZXSkdjR2hWTUdSWFUwWktkR05GTlZkTlZXd3pWbXhTUzAxSFJYaGFSV2hVWWtkb2IxVnFRbUZXYkZwMVkwWmthMkpHYkROV01qVkxZa1pLZEZWdWJGaGhNWEJ5Vm1wS1JtVnNSbkZYYkdSb1RXeEpNbFpHV21GaE1VbDRWRzVPV0dKWVFsUldhazV2VjFaa1dHVkhPVkpOVmtwSVZsYzFTMWRIU2taalNFNVdZbFJHVkZwWGVITldiR1J6Vkcxb1UxWkZXalpXVkVreFlqRlplRmRxV2xOV1JVcG9WV3RXWVdOc1ZuRlRhM1JVVm14S01GbFZXazloUjFaelYyeEdWMkV4Y0doWFZtUlNaVlphY2xwR1pGaFNNMmg1VmxkMFYxTXhaRWRWYkdSWVltMVNjMVp0TVRCTk1WbDVUbGQwV0ZKcmJETldiWEJUVjJzeFIxTnNRbGRoYTNCSVdUSjRhMk50VmtkYVIyaG9UVEJLVWxac1VrZGhNVTE0VTFob2FsSlhVbWhWYlhNeFYwWlpkMVpyZEU1aVJuQXdWRlpTUTFack1WWk5WRkpYVm0xb2VsWnNXbXRUUjFaSFYyeHdWMUpXYjNwWFYzQkhWakpPVjFWdVNsVmlSMUpVV1d4b2IxbFdaRlZSYlVab1RWZFNTRll5TlU5aGJFcDBaVWRHVlZaV2NGZFVWbHB6VmpGYVdXRkhjR2xXV0VKSFZteGtOR0V4VW5SU2JrNVlZa1phV0ZsVVNsSk5SbXhWVW1zNVUwMVdjREZXUjNodllWWktjMk5HYkZoV00xSjJWVlJHYTFZeFpISmhSM1JUVFVad1dWZFhlR0ZaVms1WFZtNVNhMUo2YkZkVmJYaHpUbFpzVm1GRlRsZGlWWEJKV1ZWV1UxWlhTbGxoU0VwWFlrWndTRnBGV2t0a1IwNUdUbFprVGxaWGQzcFdiWGhUVXpBeFNGSllhR0ZTVjJoVldXdGtiMkl4Vm5GUmJVWlhZa1p3TVZrd1dtdGhNa3BIWWtST1YwMXFWa3haYTFwTFpFWldkV0pHYUdoTldFSjVWbTF3UzFKdFZuTlNia1pZWWtkU2IxUlhlRXBOYkZwSFYyMUdXR0pXUmpOVVZscHJWMGRLV0dWSVRscFdSVm96VmpGYWExZEhWa2hQVmtwT1ZsUkZNRlpzWkRSaE1XeFhVMnhrYWxORk5WZFpiRkpIVmtaWmVXVkhkR3BpUm5CV1ZXMTRhMVJzV25WUmFscFlWa1ZLYUZacVJtdFNNV1JaWTBaYWFXRXpRbGxXYlhSWFdWZFdjMWR1UmxOaVIxSnhWRlprVTJWc2JGWmFTRTVYVFZad01WVlhlRWRXTWtWNFkwZG9XRlp0VWxOYVZscGhZMnh3UjFwR2FGTk5NbWcxVm14a01GWXhUWGhhUldSV1lrZFNXVmx0TVZOak1WcDBUVlJDVGsxV2JEVlViR1F3VjBaS2NtSkVUbGRpV0VKVVZqSnplRk5IUmtabFJtUk9ZbTFvYjFacVFtRldNazV6WTBWb1UySkhVbGhVVmxaM1ZXeGFjMXBJWkZSTlZURTBWVEZvYzFVeVJYbGhSemxXWWtaS1dGWXdXbk5rUjFKR1pFVTFhVkp1UVhkV1JscFRVVEZhY2sxV1drNVdSa3BZVm01d1YxWkdXbkZUYTFwc1ZteGFNVlZ0ZUdGaFZrbDRVbGhrVjJKVVJUQlpla3BPWlVkT1JtRkdRbGRpVmtwVlYxZDBWMlF4WkhOWGEyaHNVak5DVUZadGVITk9SbGw1VGxVNWFHSkZjRmxaVlZwelZqSkZlRlpZYUdGU00yaDZWbXBHWVZkWFJrZGhSazVwVW0wNU5GWXhVa05aVjBWNFZXNU9XRmRIZUc5VmExWjNWMFpTVjFkdVpHaFNiRmt5VlcxMGQySkdTbk5UYWtaWFlsaG9URmxXV2t0ak1rNUhZa1prVTJKRmNFbFdXSEJDVFZkTmVGcElTbWhTTTJoVVZGVmFkMkZHV25SalJXUlVUVlZzTkZaWE5VOVhSMHBXVjJ4a1ZtSllhRE5VVlZwaFYwVTFWbFJzWkU1aE0wSktWMVpXVjFVeFduSk5WbVJxVWpKb1lWUldXbmRWUm10NFYyeGthazFYVW5sVWJGcHJWR3hhV0dRemNGZGlXR2hVVlhwQmVGTkdTbGxoUjBaVFZqSm9WbGRYZEd0aU1rbDRWbTVHVW1KVldsaFphMXAzVFZacmQxZHRkR2hOYTNCSVdXdFNUMVl3TVhGV2EzaGFZVEZ3VEZwRldsZGtWMHBIWVVkb1RsZEZTalZXYlRGM1V6RktkRlp1VGxOaWExcFpXV3RrVTJJeFVsaGxSWEJPWWtad1NGWXlNVWRYUjBwWFVtcE9WVlpzY0hKV01HUkxWMGRXU1ZSc2NGZFNWbTk2Vm1wR1lXRXhaRWhXYTJoUVZtdHdUMVp0ZEhkVFZscHpXWHBHVkUxWFVrbFZNblJoWVd4T1JrNVdaRnBpUmtwSVZtdGFkMWRIVmtsVWJHUnBVakZLTmxaclkzaGlNVmw1VWxod1VsZEhhRmhXYlRGT1pVWnNjVkpzY0d4U2JWSmFXVEJrYjFaR1NsbFJiR3hYWWxoU1dGZFdaRmRqTVdSMVVteE9hVkl4U25oV1JscHJWVEpXYzJKR1dtRlRSVFZZVkZaYWQwMVdWbGhsUldSWFRXdFdORmt3Wkc5WFJscDBWV3hPWVZac2NHaFpNbmgzVWpGd1IyRkdUazVOYldjeFZtMTRhMDFHV1hoVVdHaGhVbGRTVjFsclpGTlhWbXgwVFZaT2FrMVdjREJVVmxKRFZHc3hWMkpFVmxWaVIxRjNWakJhUzJOdFNrVlViR1JwVjBWS1ZWWnFTbnBsUms1SVVtdGFiRkp0VWs5WmJURnZZakZhY1ZGdFJsZE5helY2V1RCV2IxVXlTa2hWYkdoVlZteGFNMVpYZUdGak1YQkpXa1pTVGxaVVJUQldWRVp2WXpGVmVWSlliR2hTZW14WFdXdGFTMWRHV1hkWGJrNXJVakZLUjFwRlZURmhWa3AxVVZoa1YxSnNjRlJWVkVaaFkyc3hWMWRyTlZkU2EzQlpWbTB3ZUdJeVZuTlhiazVZWWxoU1ZWVnFSbUZUUmxsNVpVaGtWMDFWY0ZoWmFrNTNWMFphYzFkdGFGZGhhM0JRVm1wR1UyUldWbk5SYkdScFZtdHdWbFl4WkRCV01sRjRXa2hPV0dFeWFITlZhazVEVlVaYWRHVklUazlTYkd3MVZHeFZOV0ZIU2taalJteGFWbFp3ZWxacVNrWmxSbHBaWVVkR1UwMHlhRFpXYlhCSFdWWmtXRkpyYUdwU01uaFlWakJXUzFOR1duUmxSM1JQVWpCV05GWlhOVTlYUm1SSVpVYzVWbUV4V2pOV01GcHpWMGRTUm1SSGNHbFNiR3Q1VmxjeE1FMUhSblJTYWxwWFlrZG9XVmxVUm5kaFJteFhWMnQwYWsxck5VaFphMXB2VmpBd2VXRkhPVmRXZWtJMFZsY3hWMUl4Y0VsV2JGcHBWMFZLVlZadGNFZFRNV1JIVjJ0a1dHSlZXbkpVVlZKSFUwWmFkRTVXVG1oTlZXOHlXV3RhYTFadFNsbGhSRTVWVmxad2VsWnRlR3RqTVZKeldrWmthVk5GU21GV01WcFhWakZWZUZkc2FGUmhNbEp4VldwS2IxWXhVbFphUms1WVVteFdNMVl5ZERCaE1VbDNUbFZrVldKR2NISldSM2hoVjBkUmVtTkdaR2xYUjJoVlZsaHdRbVZHVGtkVWJHeHBVbXMxYjFSWGVFdFdiR1JZVFZod1RsWnNjRmhaYTJoTFdWWktObUpHYUZwaE1YQXpXbGQ0V21WVk5WaGtSbFpvWld0YVdWZFVRbTlqTVZsM1RWaEdVMkV5YUdGV2FrNXZZVVpyZVdONlJsaFdNSEJJV1ZWa2IxUnNaRVpUYkVwWFRWWndhRmRXV2s1bFZsSjFWV3hXYUUxV2NGcFhWM1JyVlRKT1IxWnVSbEpXUlVwUFZXMHhVMWRzYTNkV2JYUlhUV3R3V0ZWdGNFOVdWbHB6VjI1d1dsWldjRXhaTW5NeFYwVTVWMk5IYUU1aVYyaEdWbTF3UjFsWFJYaFhXR2hxVWxaYVUxbHNhRk5qUmxwMFpFWndUbEpzY0hoVk1uQlRWakF4VjFacVZsWk5ibEp5V1ZkNFQxSnJOVmRhUm5CcFVtdHdTVlp0ZEdGVk1WbDRXa2hPV0dKWWFGaFVWVkpTWlZaYVdFMVVVbWxOVmxZMVZXeG9kMVZ0U2xaWGJHaFhZbFJHVDFwVlduTmpWa3B6WTBkNFUySldTbUZYVkVKcllqRmFSMU5zYkZKaVIxSlpWbTE0UzFSR1dsVlNhemxUVFZad01WVnRlRTloVmxwWFkwWndWMkpVUVhoVlZFWlBWakpLUjFkc2FHaGxiWGhaVmtaYVlXUXhUa2RYV0d4c1VqTlNXRlJYZEhkVFZscElZMFpPVjFZd1ZqVldWM2hQV1ZaYWMyTkhhRnBOYm1nelZXcEdkMUl5UmtkVWF6Vk9ZbGRqZUZadE1UUmhNbEY0VWxob2FWSnRhRlpaVkVwVFYwWnNkR1ZGZEdwTlZsWXpWMnRhVDJGck1WaGxTR3hYVFdwV1VGWkVSbUZrVmtaeldrWndWMVl4UmpOV2FrSmhVekpPZEZOclZsVmlSMUp3VmpCV1MxWldXbkZUYm5Cc1VtdHNORlpITlU5VmJVcElWV3M1V2xZelVtaFZNRnByWXpGa2RGSnNaRTVoZWxZMlYxUkNWMkl4VlhsVGEyaFdZWHBzVmxadWNFZGtiRlY1WlVad2JGSnNXbmxaTUdRMFZUSkdObEpVUWxkV1JWcDJXV3BLUjJNeFRuTmhSMmhVVWxWd1dGZFhlRk5TTWxKelYyNUtXR0pWV25GVVYzUmhVakZhU0dWR1RsVmlSbkF4VlZab2ExWXhTbk5qUmxKV1ZrVmFhRmt5YzNoV01XUnlUbFprVTJFelFscFdNV1F3WVRKSmVWVnVUbGhYUjFKWldXeG9VMVpXVm5GUmJVWlVVbTE0ZVZZeU5XdGhSbHAwVld0c1dsWlhUVEZXYWtwTFZsWktWVkZzY0d4aE0wSlJWMWh3UzFReFNuUlNhMlJTWWtkU1QxUlZWbmRXYkZsNFdrUkNhVTFWVmpOVWJGWnJWMGRLY21OSFJsVldSWEJVVmxWYVlXUkhWa2xVYXpsVFlrZDNNVlpIZUZaT1YwWklVMnRhYWxKdGVHRldiRnAzWld4YWNWTnJXbXhTYkhCYVdWVmtSMVV3TUhsaFJGcFhZbGhDU0ZkV1dtdFhSa3B5V2tkb1UyRjZWbmRXVnpBeFVXc3hWMWRZYUZoaVIxSmhWbXBDVjA1R1dsaE9WazVXVFd0d2VWUnNXbk5YYlVWNFkwZG9WMDFHY0hwV2JGcFBZekZPY2s1V1RtbFdhM0JhVm0xd1MwMUZNVWhTYmtwT1ZtMVNWVmxYZEdGWFJsWjFZMFZrYTJKR2NGWlZNblF3VlRBeGNrNVZhRnBoTVhCeVZsUktTMUp0VGtaTlZtaFhUVEJLU1ZaclVrZFhiVlpIWTBWc1ZHSlhhRlJVVkVwdlZsWmFSMVp0Um10TmExcElWakkxVTFSc1RraGhSbVJWVm14YWVsUnJXbUZYUlRWV1QxWm9VMDFJUWxoV2FrbzBZVEZhZEZOc2JHaFNSVFZXVm14YWQyVnNXblJsUjNScVRWWktlbGRyV210aFZrNUdVMnh3VjAxV1NrUlpWRVpLWlVaa2MxcEdhR2xoZWxaNFZsZHdTMkl4YkZkalJtaHJVakJhVDFSV1dtRmxiRmw1WlVkMGFFMVZiRE5VYkZaclZsZEtSMk5JU2xkU00yaG9WakJrVW1WdFRrZGFSMnhZVWpKb1ZsWnNhSGRSYlZaSFZHdGtWR0pIZUc5VmFrSmhWa1phY1ZOdE9WZGlSMUpaV2tWa01HRlZNWEppUkZKWFlsUldTRlpYTVV0V2JHUnpZVVp3YUUxWVFYcFdSbFpoWkRGYWRGSnJhR3RTYkZwdldsZDBZVmRXWkZWUmJUbHFUVlpzTTFSV2FFZFZNa1Y1WVVkR1YyRnJOWFpaVlZweVpWVXhWazlXVGxkaE1XOTNWbGN4TkZReFdYZE5WbHBZVjBoQ1dGbHNVa2ROTVZZMlVtczFiRlpzU2pGV1IzaFhZVmRGZWxGdVdsZFdla0kwVmxSR1lWSXhWbk5XYkZKcFZrZDRkMVpHV21Ga01sWlhWMnhvYTFKRlNsZFVWVkpIVjBac2NsVnNUbGROVlc4eVZtMTRiMVl5U2xWU2EzaFdZbGhvVUZsNlNrZFNNV1IwWWtaT2FFMHdTbUZXYlRGM1VqRnNWMkpHWkZSWFIyaHdWV3RhZDFaR2JITmFSRkpWVFZad2VGVnRkREJXUmxwMFZXeHNZVkpYVWtoV2FrRjRWakZrZFdOR2FHaE5WbkF5VjFaV1lWTXlUWGhVYms1WVlrZFNjRlpyVm1GV1ZscDBaRWRHYTAxWFVrbFdiWFJ6VmxkS1NGVnVRbFpoYTFwTVZHMTRZV05zYTNwaFIyeE9WbXhaTUZacVNqQlpWbVJJVW01T2FsSnRhRmhaVkVaaFpWWndWbGR1VG1wV2EzQjZXVEJrTkZVeVNsZFRiVVpZVmtWS2FGZFdaRWRXTVU1WllrZG9VMkpyU2xsWFYzaFRVbXN4UjJORlZsTmlhelZZVkZkek1WTkdXWGxsUnpsb1ZtMVNTRlV5TlhOV01rcFZVbGhrWVZKRmNGaFpla1pyWkZaV2NrNVdhRk5XUmxwYVZtdGFZVll5VFhkT1dFNXBVbXh3VjFsclpGTlhSbEpXVld0a1dGWnNjRmhaVldRd1YwWktjMk5FUWxkV00yaFFWMVphWVdNeVRraGhSbkJPWW0xbmVsWlhjRWRrTVU1SVUydG9hVkpyTlZsVmJGWnlaVVphZEUxVVVtaE5SRVpJVmpGb2IxWldaRWhsUmxaWFRVZFNkbGt3V2xka1JURlZWVzF3VjJKR2NEWldNblJoVkRKR2NrMVdaR3BTUlVwb1ZteGFXbVF4YkhKYVJYUlRUV3MxUmxWWGVGZFdNVnB5WTBac1YyRnJTbWhWTWpGU1pWWlNjbGR0YUZOaWEwcFFWbGN3TVZFd01YTlhia1pVWW01Q2MxVnRjekZUVmxaMFpFZEdhVkpyY0RCV1YzTTFWMjFLVlZKdVdscGhhMXBvV1RGYVIyUkdTbk5hUlRWb1pXeFpNbFl4VWtOV01rbDRWbGhzVkdFeWFGZFpXSEJ6Vm14YWRXTkZaR3RpUmxZMVdsVmFkMkpHU25OWGJteGFUVWRSTUZaVVNrZE9iRXBWVVd4a1YxSldjREpYVjNCTFVqSk5lRlJ1VG1oU2JIQndWbXBLYjFkR1pGaGxSemxWWWxaYVNGWXhhR3RYUjBwSlVXNUtWVlpzY0ROYVZscDNVbXhhVlZKdGFGZGhNMEY0VmxaYWIyRXhaRWRYYTFwVVlrZG9ZVlpyVm1GamJHdzJVbTEwV0ZadVFraFhhMlJ2WVZaS1dWRnNjRmROVjFGM1dWUktTbVZXVm5WVWJHaFlVakZLV2xkWGVHOVZNVnBYVm01R1VtSlZXbFZWYlhoelRsWndWbUZIZEdoU2JIQXdWbGQwYzFkc1dsaFVWRVpYWVd0d1RGWnNXa3RXVmtaelkwZDRhRTB3U2xKV01XaDNVakpSZUZSc1pGVmliRXB5Vlcxek1WUXhVbFpXYm1SWFlrWnNORmRyVWtOWGJGbDRVbXBPVldKR1duSldNR1JMVTBkR1JrOVdaR2hOVm5CTlZtdFNSMVl5VWtoV2EyeFZZWHBzVkZsclpETk5WbHBJWlVaYVQxWXdXa2xWTW5SaFlXeEtkR1ZIUmxkaVJuQXpXa2Q0WVdOV1RuSmFSM0JvWld4YVdWWnNaRFJoTVZWNFYyNVdVbUZzY0ZoWmJHaHZZVVpXY1ZKdGRGaFNhM0I1VjJ0YVYyRldTWGhUYmxwWFVteEtSRlV5YzNoak1XUjFWVzEwVTAweFNsRldWekI0VGtaa1IxZHVUbWhUUlRWWVZXcENWMDVHYTNkWGJrNVhUVVJHU1ZsVlZsTldNREZIWTBkb1dtVnJXbkphUmxwM1VqRlNjazVXWkdoTlYzUXpWbTB4ZDFNd01VZFZXR2hZWW14S1ZWbFVSbmRXVm14eVYyNWtWMkpHY0hwV01uUnJWR3N4VjFkcmFGWmlSa3A2VmxaYVYxWlZNVVZoZWpBOQ==\"}}}}}},\"responses\":{\"201\":{\"description\":\"File successfully uploaded.\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"file_id\":{\"type\":\"string\",\"description\":\"The file's unique ID.\",\"example\":\"xBvggqyjQUal6TecwMlYwE\"}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `124` <br>\\n Invalid access token. <br>\\n**Error Code:** `300` <br>\\n Invalid File type.\\nExceed max file size: ${maxSize} megabytes. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests  For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Edit\"],\"x-macro-scopes\":[\"whiteboard_file:write:admin\",\"whiteboard_file:write\"],\"x-granular-scopes\":[\"whiteboard:write:file\",\"whiteboard:write:file:admin\"]}}},\"/whiteboards/{whiteboardId}/files/{fileId}\":{\"get\":{\"tags\":[\"File\"],\"summary\":\"Download Imported Whiteboard File\",\"description\":\"Download files imported into the whiteboard, including images, PDFs, DOCX, and other supported formats.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_file:read`,`whiteboard_file:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:file`,`whiteboard:read:file:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"Downloadembeddedwhiteboardfile\",\"parameters\":[{\"name\":\"whiteboardId\",\"in\":\"path\",\"description\":\"The whiteboard ID.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"YXw46hQqQCiZMlAGkQVKsA\"}},{\"name\":\"fileId\",\"in\":\"path\",\"description\":\"The ID of the file imported into the whiteboard to download.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"Et2l7DAvQqu5E6jsqnnasw\"}}],\"responses\":{\"200\":{\"description\":\"Returns a file stream.\"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n \"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n \"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardContent:Read\"],\"x-macro-scopes\":[\"whiteboard_file:read\",\"whiteboard_file:read:admin\"],\"x-granular-scopes\":[\"whiteboard:read:file\",\"whiteboard:read:file:admin\"]}}},\"/whiteboards/import\":{\"post\":{\"tags\":[\"Import\"],\"summary\":\"Create a new whiteboard by import\",\"description\":\"Creates a new whiteboard for a user by converting Miro, Mural, or Visio files.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_import:write`,`whiteboard_import:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:import`,`whiteboard:write:import:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `HEAVY`\\n\\n**Not supported in Gov cluster** \",\"operationId\":\"CreateWhiteboardImport\",\"requestBody\":{\"description\":\"Creates a new whiteboard for a user by converting Miro, Mural, or Visio files request form-data.\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"file_id\",\"type\"],\"type\":\"object\",\"properties\":{\"file_id\":{\"type\":\"string\",\"description\":\"The ID of the file to convert to a whiteboard. Use the **Upload File for Whiteboard Import** API (POST `/whiteboards/files`) to upload the file and get the `file_id`.\\nSupported file types:\\n- **Visio**: `.vsdx`\\n- **Miro** and **Mural**: `.pdf`\\n\\nMaximum file size: **100 MB**.\\nEnsure the file is valid, within the size limit, and in one of the supported formats.\",\"example\":\"xBvggqyjQUal6TecwMlYwE\"},\"type\":{\"type\":\"number\",\"description\":\"The import file original type. \\n\\nEnum:  \\n* `1` - Miro \\n* `2` - Visio \\n* `3` - Mural \",\"example\":1},\"whiteboard_name\":{\"type\":\"string\",\"description\":\"The name of the whiteboard created by import.\",\"example\":\"Q1 plan\"},\"skip_notifications\":{\"type\":\"boolean\",\"description\":\" **Default is false (notifications will be sent)**.\\n If set to `true`, collaborators and owners will not receive direct messages or emails when the whiteboard is imported.\",\"example\":false},\"owner_email\":{\"type\":\"string\",\"description\":\"The owner email. This field is only available if you have the `whiteboard:write:import:admin` scope. Otherwise, it will automatically be set to `null` and the document owner will **default to the user who imported the file**.\",\"format\":\"email\",\"example\":\"example@zoom.us\"},\"collaborators\":{\"maxItems\":100,\"minItems\":0,\"type\":\"array\",\"description\":\"Users or team chat channels to add to a whiteboard.\",\"items\":{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\",\"description\":\"The `user email` address to share with.\",\"format\":\"email\",\"example\":\"user@example.com\"},\"role\":{\"type\":\"string\",\"description\":\"Collaborator role type. \\n\\nEnum:  \\n- `0` - OWNER \\n- `1` - CO_OWNER \\n- `2` - EDITOR \\n- `3` - COMMENTER \\n- `4` - VIEWER\",\"example\":\"2\",\"enum\":[\"0\",\"1\",\"2\",\"3\",\"4\"]},\"channel_id\":{\"type\":\"string\",\"description\":\"The team chat `channel ID` to share with.\",\"example\":\"cabc1234567defghijkl01234\"}},\"description\":\"Array of collaborator objects.\"}},\"project_id\":{\"type\":\"string\",\"description\":\"qiMWddmtRxqwSFrK22lJPg\",\"example\":\"The Project ID\"}}}}}},\"responses\":{\"201\":{\"description\":\"The imported file is being converted into a whiteboard.\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"task_id\":{\"type\":\"string\",\"description\":\"Use this ID to check the import status via the [Get Whiteboard Import Status](https://developers.zoom.us/docs/api/whiteboard/#tag/import/GET/whiteboards/import/task/{taskId}/status) endpoint (GET `/whiteboards/import/{taskId}/status`). The unique ID for your import.\",\"example\":\"ce31de9e5a5f440bab9d60c144fd76b7\"}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `34002` <br>\\n Invalid file type. <br>\\n**Error Code:** `38003` <br>\\n File size exceed limit. <br>\\n**Error Code:** `38004` <br>\\n Conversion settings are not enabled. Please contact your account administrator to allow users to import and convert boards and diagrams from other applications into editable whiteboards via the whiteboard account settings. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Edit\"],\"x-macro-scopes\":[\"whiteboard_import:write\",\"whiteboard_import:write:admin\"],\"x-granular-scopes\":[\"whiteboard:write:import\",\"whiteboard:write:import:admin\"]}}},\"/whiteboards/import/{taskId}/status\":{\"get\":{\"tags\":[\"Import\"],\"summary\":\"Get whiteboard import status \",\"description\":\"Use this API to query the status of new whiteboard by import. Before using this API, confirm that you have invoked the interface for import file to create new whiteboard **Create a new whiteboard by import** and obtained the `task_id`.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_import:read`,`whiteboard_import:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:import:admin`,`whiteboard:read:import`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"GetWhiteboardimportstatus\",\"parameters\":[{\"name\":\"taskId\",\"in\":\"path\",\"description\":\"The unique ID for your import.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"ce31de9e5a5f440bab9d60c144fd76b7\"}}],\"responses\":{\"200\":{\"description\":\"Get whiteboards import result.\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"file_name\":{\"type\":\"string\",\"description\":\"The file name of import file.\",\"example\":\"miro.pdf\"},\"status\":{\"type\":\"string\",\"description\":\"\\nThe status of the whiteboard created by importing a file.\\n\\n**Enum**:\\n- `processing` - The file is converting to whiteboard.\\n- `succeeded` - The file convert to whiteboard successfully.\\n- `failed` - The file convert to whiteboard failed.\\n\",\"example\":\"succeeded\",\"default\":\"processing\",\"enum\":[\"processing\",\"failed\",\"succeeded\"]},\"whiteboard_id\":{\"type\":\"string\",\"description\":\"The whiteboard ID `nullable`. This value is returned only when the `status` is `succeeded`.\",\"example\":\"HY8-AC_HSou4SXY1S-OmOg\"},\"doc_name\":{\"type\":\"string\",\"description\":\"The whiteboard name `nullable`. This value is returned only when the `status` is `succeeded`.\",\"example\":\"Q1 Plan\"}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `38002` <br>\\n Failed to convert the file {fileName}. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `38007` <br>\\n The taskId not exist or expired. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Edit\"],\"x-macro-scopes\":[\"whiteboard_import:read\",\"whiteboard_import:read:admin\"],\"x-granular-scopes\":[\"whiteboard:read:import:admin\",\"whiteboard:read:import\"]}}},\"/whiteboards/projects\":{\"get\":{\"tags\":[\"Project\"],\"summary\":\"List all projects\",\"description\":\"Retrieves a list of projects.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:read`,`whiteboard_project:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:list_projects:admin`,`whiteboard:read:list_projects`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\\n\\n**Not supported in Gov cluster** \",\"operationId\":\"Listallprojects\",\"parameters\":[{\"name\":\"search_key\",\"in\":\"query\",\"description\":\"The name of a project. If this parameter is provided, only the matched project will be returned. Otherwise, all projets will be returned.\",\"required\":false,\"schema\":{\"maxLength\":50,\"minLength\":0,\"type\":\"string\",\"example\":\"demo \"}},{\"name\":\"user_id\",\"in\":\"query\",\"description\":\"The email address or user ID of the project owner. If this parameter is provided, only the project owned by the provided user will be returned. If set to `me`, will return the matched projects of current user.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"me\"}},{\"name\":\"page_size\",\"in\":\"query\",\"description\":\"The number of records returned from a single API call.\",\"required\":false,\"schema\":{\"type\":\"integer\",\"example\":10}},{\"name\":\"next_page_token\",\"in\":\"query\",\"description\":\"The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42\"}}],\"responses\":{\"200\":{\"description\":\"The list of projects is successfully retrieved.\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"projects\":{\"type\":\"array\",\"description\":\"The project list.\",\"items\":{\"type\":\"object\",\"properties\":{\"collaborator\":{\"maxItems\":100,\"minItems\":0,\"type\":\"array\",\"description\":\"The project collaborators, this API returns only the project collaborators who have the 'owner' role, along with the current user's information.\",\"items\":{\"type\":\"object\",\"properties\":{\"collaborator_id\":{\"type\":\"string\",\"description\":\"The project collaborator ID.\",\"example\":\"x4g6hy789k0o878k\"},\"collaborator_name\":{\"type\":\"string\",\"description\":\"The project collaborator name.\",\"example\":\"Jack\"},\"user_id\":{\"type\":\"string\",\"description\":\"`nullable` User ID. This response only returns if the `collaborator_type` is 0. If the `collaborator_type` is not 0, this returns null value.\",\"example\":\"wf3h7uo1dk0v53bl\"},\"avatar\":{\"type\":\"string\",\"description\":\"Collaborator profile picture.\",\"example\":\"https://content.zoom.us/swerwerw.jpeg\"},\"role\":{\"type\":\"integer\",\"description\":\"Collaborator role type.\\n\\nRole Enum Values:\\n\\n- `0` - OWNER\\n- `1` - CO_OWNER\\n- `2` - EDITOR\\n- `3` - COMMENTER\\n- `4` - VIEWER\",\"example\":2,\"enum\":[0,1,2,3,4]},\"collaborator_type\":{\"type\":\"integer\",\"description\":\"Collaborator type.\\nEnum:\\n- `0` - SAME_ACCOUNT_USER\\n- `1` - EXTERNAL_USER\",\"example\":0,\"enum\":[0,1]}},\"description\":\"The project collaborator information\"}},\"created_date\":{\"type\":\"string\",\"description\":\"The project create date-time in UTC/GMT. Example: `2020-03-31T12:02:00Z`\",\"format\":\"date-time\",\"example\":\"2020-03-31T12:02:00Z\"},\"modified_date\":{\"type\":\"string\",\"description\":\"The project update date-time in UTC/GMT. Example: `2020-03-31T12:02:00Z`\",\"format\":\"date-time\",\"example\":\"2020-03-31T12:02:00Z\"},\"project_name\":{\"type\":\"string\",\"description\":\"The  project name.\",\"example\":\"Untitle\"},\"project_id\":{\"type\":\"string\",\"description\":\"qiMWddmtRxqwSFrK22lJPg\",\"example\":\"The Project ID\"},\"starred\":{\"type\":\"boolean\",\"description\":\"Indicates whether the project is starred.\\n`true`: starred\\n`false`: not starred\",\"example\":false},\"owner_display_name\":{\"type\":\"string\",\"description\":\"The project owner display name.\",\"example\":\"Example Name\"}},\"description\":\"The project basic info.\"}},\"next_page_token\":{\"type\":\"string\",\"description\":\"The next page token paginates through a large set of results. A next page token is returned when the available results exceed the current page size. The expiration period for this token is 15 minutes.\",\"example\":\"qUEQqB1V0HVhJmwKFQrGOD\"},\"page_size\":{\"type\":\"integer\",\"description\":\"The number of records returned per page in a single API call.\",\"example\":10},\"total_records\":{\"type\":\"integer\",\"description\":\"The total number of records found.\",\"example\":100}}}}}},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests  For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Read\"],\"x-macro-scopes\":[\"whiteboard_project:read\",\"whiteboard_project:read:admin\"],\"x-granular-scopes\":[\"whiteboard:read:list_projects:admin\",\"whiteboard:read:list_projects\"]}},\"post\":{\"tags\":[\"Project\"],\"summary\":\"Create a new project\",\"description\":\"This API endpoint allows you to create a new project for a user, optionally adding collaborators and initializing it with existing whiteboards.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:write`,`whiteboard_project:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:project`,`whiteboard:write:project:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"Createproject\",\"requestBody\":{\"content\":{\"application/json\":{\"schema\":{\"required\":[\"project_name\"],\"type\":\"object\",\"properties\":{\"project_name\":{\"maxLength\":50,\"minLength\":1,\"type\":\"string\",\"description\":\"The project name.\",\"example\":\"Untitle\"},\"skip_notifications\":{\"type\":\"boolean\",\"description\":\" **Default is false (notifications will be sent)**.\\n If set to `true`, collaborators and owners will not receive direct messages or emails when the project is created.\",\"example\":false},\"invite_message\":{\"maxLength\":280,\"minLength\":0,\"type\":\"string\",\"description\":\"The invite message.\",\"example\":\"I'm glad to invite you to join this project.\"},\"collaborators\":{\"title\":\"Array of collaborator objects.\",\"maxItems\":100,\"minItems\":0,\"type\":\"array\",\"description\":\"Array of collaborator objects to be added to the project.\",\"items\":{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\",\"description\":\"The user's email to be added to the project.\",\"example\":\"user@example.com\"},\"role\":{\"type\":\"integer\",\"description\":\"Collaborator role type.\\nWhen assigning a new `OWNER` or `CO_OWNER`, the specified user must belong to the same account as the original `OWNER`. If a new owner is designated for a project, the original owner will automatically be demoted to `CO_OWNER`.\\n\\nRole Enum Values:\\n\\n- `0` - OWNER\\n- `1` - CO_OWNER\\n- `2` - EDITOR\\n- `3` - COMMENTER\\n- `4` - VIEWER\",\"example\":2,\"default\":2,\"enum\":[0,1,2,3,4]}},\"description\":\"A user who works jointly on a project.\"}},\"whiteboard_ids\":{\"title\":\"whiteboard_id\",\"maxItems\":50,\"minItems\":0,\"uniqueItems\":true,\"type\":\"array\",\"description\":\"The whiteboard ID list that you want to add to the project.\",\"items\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\",\"example\":\"J7XZFDEQTkWSsMsmO_M1Xa\"}},\"owner_email\":{\"type\":\"string\",\"description\":\"The owner email. This field is only available if you have the `whiteboard_project:write:admin` scope. Otherwise, it will automatically be set to `null` and the project owner will default to the request user.\",\"format\":\"email\",\"example\":\"owner@example.com\"}}}}}},\"responses\":{\"201\":{\"description\":\"The project has been successfully created.\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"collaborator\":{\"maxItems\":100,\"minItems\":0,\"type\":\"array\",\"description\":\"The project collaborators.\",\"items\":{\"type\":\"object\",\"properties\":{\"collaborator_id\":{\"type\":\"string\",\"description\":\"The project collaborator ID.\",\"example\":\"x4g6hy789k0o878k\"},\"collaborator_name\":{\"type\":\"string\",\"description\":\"The project collaborator name.\",\"example\":\"Jack\"},\"user_id\":{\"type\":\"string\",\"description\":\"`nullable` User ID. This response only returns if the `collaborator_type` is 0. If the `collaborator_type` is not 0, this returns null value.\",\"example\":\"wf3h7uo1dk0v53bl\"},\"avatar\":{\"type\":\"string\",\"description\":\"Collaborator profile picture.\",\"example\":\"https://content.zoom.us/swerwerw.jpeg\"},\"role\":{\"type\":\"integer\",\"description\":\"Collaborator role type.\\n\\nRole Enum Values:\\n\\n- `0` - OWNER\\n- `1` - CO_OWNER\\n- `2` - EDITOR\\n- `3` - COMMENTER\\n- `4` - VIEWER\",\"example\":2,\"enum\":[0,1,2,3,4]},\"collaborator_type\":{\"type\":\"integer\",\"description\":\"Collaborator type.\\nEnum:\\n- `0` - SAME_ACCOUNT_USER\\n- `1` - EXTERNAL_USER\",\"example\":0,\"enum\":[0,1]}},\"description\":\"The project collaborator information.\"}},\"created_date\":{\"type\":\"string\",\"description\":\"The project creation date-time in UTC/GMT. Example: `2020-03-31T12:02:00Z`\",\"format\":\"date-time\",\"example\":\"2020-03-31T12:02:00Z\"},\"modified_date\":{\"type\":\"string\",\"description\":\"The whiteboard update date-time in UTC/GMT. Example: `2020-03-31T12:02:00Z`\",\"format\":\"date-time\",\"example\":\"2020-03-31T12:02:00Z\"},\"project_name\":{\"type\":\"string\",\"description\":\"The project name.\",\"example\":\"Untitle\"},\"project_id\":{\"type\":\"string\",\"description\":\"qiMWddmtRxqwSFrK22lJPg\",\"example\":\"The Project ID\"},\"starred\":{\"type\":\"boolean\",\"description\":\"Indicates whether the project is starred. `true`: starred, `false`: not starred.\",\"example\":false}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Edit\"],\"x-macro-scopes\":[\"whiteboard_project:write\",\"whiteboard_project:write:admin\"],\"x-granular-scopes\":[\"whiteboard:write:project\",\"whiteboard:write:project:admin\"]}}},\"/whiteboards/projects/{projectId}\":{\"get\":{\"tags\":[\"Project\"],\"summary\":\"Get a project\",\"description\":\"This API endpoint allows you to retrieve detailed information about a specific project.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:read`,`whiteboard_project:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:project`,`whiteboard:read:project:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"Getaproject\",\"parameters\":[{\"name\":\"projectId\",\"in\":\"path\",\"description\":\"The project ID.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"N6XZFDEQTkWSsMsmO_M1XQ\"}}],\"responses\":{\"200\":{\"description\":\"The project information has been successfully retrieved.\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"collaborator\":{\"maxItems\":100,\"minItems\":0,\"type\":\"array\",\"description\":\"The project collaborators. This API returns only the project collaborators who have the 'owner' role, along with the current user's information.\",\"items\":{\"type\":\"object\",\"properties\":{\"collaborator_id\":{\"type\":\"string\",\"description\":\"The project collaborator ID.\",\"example\":\"x4g6hy789k0o878k\"},\"collaborator_name\":{\"type\":\"string\",\"description\":\"The project collaborator name.\",\"example\":\"Jack\"},\"user_id\":{\"type\":\"string\",\"description\":\"`nullable` User ID. This response only returns if the `collaborator_type` is 0. If the `collaborator_type` is not 0, this returns null value.\",\"example\":\"wf3h7uo1dk0v53bl\"},\"avatar\":{\"type\":\"string\",\"description\":\"Collaborator profile picture.\",\"example\":\"https://content.zoom.us/swerwerw.jpeg\"},\"role\":{\"type\":\"integer\",\"description\":\"Collaborator role type.\\n\\nRole Enum Values:\\n\\n- `0` - OWNER\\n- `1` - CO_OWNER\\n- `2` - EDITOR\\n- `3` - COMMENTER\\n- `4` - VIEWER\",\"example\":2,\"enum\":[0,1,2,3,4]},\"collaborator_type\":{\"type\":\"integer\",\"description\":\"Collaborator type.\\nEnum:\\n- `0` - SAME_ACCOUNT_USER\\n- `1` - EXTERNAL_USER\",\"example\":0,\"enum\":[0,1]}},\"description\":\"The project collaborator information.\"}},\"created_date\":{\"type\":\"string\",\"description\":\"The project create date-time in UTC/GMT. Example: `2020-03-31T12:02:00Z`\",\"format\":\"date-time\",\"example\":\"2020-03-31T12:02:00Z\"},\"modified_date\":{\"type\":\"string\",\"description\":\"The project update date-time in UTC/GMT. Example: `2020-03-31T12:02:00Z`\",\"format\":\"date-time\",\"example\":\"2020-03-31T12:02:00Z\"},\"project_name\":{\"type\":\"string\",\"description\":\"The  project name.\",\"example\":\"Untitle\"},\"project_id\":{\"type\":\"string\",\"description\":\"qiMWddmtRxqwSFrK22lJPg\",\"example\":\"The Project ID\"},\"starred\":{\"type\":\"boolean\",\"description\":\"Indicates whether the project is starred.\\n`true`: starred\\n`false`: not starred\",\"example\":false},\"owner_display_name\":{\"type\":\"string\",\"description\":\"The project owner display name.\",\"example\":\"Example Name\"}}}}}},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111804` <br>\\n Permission is needed to access this project. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `38007` <br>\\n The projectId is not exist. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Read\"],\"x-macro-scopes\":[\"whiteboard_project:read\",\"whiteboard_project:read:admin\"],\"x-granular-scopes\":[\"whiteboard:read:project\",\"whiteboard:read:project:admin\"]}},\"delete\":{\"tags\":[\"Project\"],\"summary\":\"Delete a project\",\"description\":\"This API endpoint allows you to delete a specified project. Deleting a project removes all associated whiteboards from the project, setting them to have no project assignment. \\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:write:admin`,`whiteboard_project:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:delete:project`,`whiteboard:delete:project:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"Deleteproject\",\"parameters\":[{\"name\":\"projectId\",\"in\":\"path\",\"description\":\"The project ID.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"qiMWddmtRxqwSFrK22lJPg\"}}],\"responses\":{\"204\":{\"description\":\"The project has been successfully deleted. No content is returned in the response body.\"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111804` <br>\\n Permission is needed to access this project. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `38007` <br>\\n The projectId is not exist. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[],\"x-macro-scopes\":[\"whiteboard_project:write:admin\",\"whiteboard_project:write\"],\"x-granular-scopes\":[\"whiteboard:delete:project\",\"whiteboard:delete:project:admin\"]}},\"patch\":{\"tags\":[\"Project\"],\"summary\":\"Update project basic information\",\"description\":\"This API endpoint allows you to update the basic information of a specific project.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:write:admin`,`whiteboard_project:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:update:project:admin`,`whiteboard:update:project`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"Updateproject\",\"parameters\":[{\"name\":\"projectId\",\"in\":\"path\",\"description\":\"The project ID.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"qiMWddmtRxqwSFrK22lJPg\"}}],\"requestBody\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"project_name\":{\"maxLength\":50,\"minLength\":1,\"type\":\"string\",\"description\":\"The new project name.\",\"example\":\"New Project Name\"}}}}}},\"responses\":{\"204\":{\"description\":\"The project's basic information has been successfully updated. No content is returned in the response body.\"},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `38002` <br>\\n The project name is invalid. <br>\\n\"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111804` <br>\\n Permission is needed to access this project. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `38007` <br>\\n The project ID does not exist. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Edit\"],\"x-macro-scopes\":[\"whiteboard_project:write:admin\",\"whiteboard_project:write\"],\"x-granular-scopes\":[\"whiteboard:update:project:admin\",\"whiteboard:update:project\"]}}},\"/whiteboards/projects/{projectId}/collaborators\":{\"get\":{\"tags\":[\"Project\"],\"summary\":\"Get collaborators of a project\",\"description\":\"Returns a list of a project collaborators.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project_collaborator:read`,`whiteboard_project_collaborator:read:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:read:project_collaborator`,`whiteboard:read:project_collaborator:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"Getcollaboratorsofaproject\",\"parameters\":[{\"name\":\"projectId\",\"in\":\"path\",\"description\":\"The ID of the project.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"HY8-AC_HSou4SXY1S-OmOg\"}}],\"responses\":{\"200\":{\"description\":\"Get collaborators of a project.\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"array\",\"items\":{\"required\":[\"collaborator_id\"],\"type\":\"object\",\"properties\":{\"collaborator_id\":{\"type\":\"string\",\"description\":\"`not nullable` The collaborator's unique ID.\",\"example\":\"x4g6hy789k0o878k\"},\"collaborator_name\":{\"type\":\"string\",\"description\":\"The collaborator's display name.\",\"example\":\"Jack\"},\"user_id\":{\"type\":\"string\",\"description\":\"`nullable` User ID. This response only returns if the `collaborator_type` is `0`. If the `collaborator_type` is **not** `0`, this returns null value.\",\"example\":\"wf3h7uo1dk0v53bl\"},\"avatar\":{\"type\":\"string\",\"description\":\"The collaborator profile picture.0\",\"example\":\"https://content.zoom.us/swerwerw.jpeg\"},\"role\":{\"type\":\"integer\",\"description\":\"The collaborator role type. Enum:\\n * `0` - OWNER \\n * `1` - CO_OWNER \\n * `2` - EDITOR \\n * `3` - COMMENTER \\n * `4` - VIEWER \",\"example\":2,\"enum\":[0,1,2,3,4]},\"collaborator_type\":{\"type\":\"integer\",\"description\":\"The collaborator type. \\nEnum: \\n* `0` - SAME_ACCOUNT_USER \\n* `1` - EXTERNAL_USER \",\"example\":0,\"enum\":[0,1]},\"email\":{\"type\":\"string\",\"description\":\"`nullable` Email. This response only returns if the `collaborator_type` is `0`. If the `collaborator_type` is **not** `0`, this returns null value.\",\"example\":\"user@example.com\"}},\"description\":\"A user who works jointly on a project.\"}}}}},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111804` <br>\\n Permission is needed to access this project. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `38007` <br>\\n The projectId is not exist. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Read\"],\"x-macro-scopes\":[\"whiteboard_project_collaborator:read\",\"whiteboard_project_collaborator:read:admin\"],\"x-granular-scopes\":[\"whiteboard:read:project_collaborator\",\"whiteboard:read:project_collaborator:admin\"]}},\"post\":{\"tags\":[\"Project\"],\"summary\":\"Share a project to new users\",\"description\":\"Add new collaborators to a specified project. Project owners or authorized users can invite team members or external users to collaborate on a project with specified roles and permissions.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project_collaborator:write`,`whiteboard_project_collaborator:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:project_collaborator`,`whiteboard:write:project_collaborator:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"Shareaprojecttonewusers\",\"parameters\":[{\"name\":\"projectId\",\"in\":\"path\",\"description\":\"The project ID.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"N6XZFDEQTkWSsMsmO_M1XQ\"}}],\"requestBody\":{\"content\":{\"application/json\":{\"schema\":{\"required\":[\"collaborators\"],\"type\":\"object\",\"properties\":{\"skip_notifications\":{\"type\":\"boolean\",\"description\":\" **Default is false (notifications will be sent)**.\\n If set to `true`, collaborators and owners will not receive direct messages or emails when the project is shared.\",\"example\":false},\"invite_message\":{\"maxLength\":280,\"minLength\":0,\"type\":\"string\",\"description\":\"A custom message to include with the project invitation.\",\"example\":\"I'm glad to invite you to join this project.\"},\"collaborators\":{\"maxItems\":20,\"minItems\":1,\"type\":\"array\",\"description\":\"List of users to add as collaborators to the project.\",\"items\":{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\",\"description\":\"**User Email Address**: The email address of the user you want to share the project with.\",\"example\":\"user@example.com.\"},\"role\":{\"type\":\"integer\",\"description\":\"Collaborator role type. When assigning a new `OWNER` or `CO_OWNER`, the specified user must belong to the same account as the original `OWNER`. If a new owner is designated for a project, the original owner will automatically be demoted to `CO_OWNER`.\\nRole Enum Values: \\n* `0` - OWNER \\n* `1` - CO_OWNER \\n* `2` - EDITOR \\n* `3` - COMMENTER \\n* `4` - VIEWER \",\"example\":1}},\"description\":\"A user who works jointly on a whiteboard.\"}}}}}}},\"responses\":{\"201\":{\"description\":\"The collaborators have been successfully added to the project.\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"array\",\"items\":{\"title\":\"root\",\"type\":\"object\",\"properties\":{\"collaborator_id\":{\"type\":\"string\",\"description\":\"`not nullable` The collaborator's unique ID.\",\"example\":\"x4g6hy789k0o878k\"},\"collaborator_name\":{\"type\":\"string\",\"description\":\"The collaborator's display name.\",\"example\":\"Jack\"},\"user_id\":{\"type\":\"string\",\"description\":\"`nullable` User ID. This response only returns if the `collaborator_type` is `0`. If the `collaborator_type` is **not** `0`, this returns null value.\",\"example\":\"wf3h7uo1dk0v53bl\"},\"avatar\":{\"type\":\"string\",\"description\":\"Collaborator profile picture.\",\"example\":\"https://content.zoom.us/swerwerw.jpeg\"},\"role\":{\"type\":\"integer\",\"description\":\"Collaborator role type. \\nEnum: \\n* `0` - OWNER \\n* `1` - CO_OWNER \\n* `2` - EDITOR \\n* `3` - COMMENTER \\n* `4` - VIEWER \"},\"collaborator_type\":{\"type\":\"integer\",\"description\":\"Collaborator type. \\nEnum: \\n* `0` - SAME_ACCOUNT_USER \\n* `1` - EXTERNAL_USER\"}},\"description\":\"Users who work jointly on a whiteboard.\"}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n \"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n \"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n \"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Read\",\"WhiteboardManagement:Edit\"],\"x-macro-scopes\":[\"whiteboard_project_collaborator:write\",\"whiteboard_project_collaborator:write:admin\"],\"x-granular-scopes\":[\"whiteboard:write:project_collaborator\",\"whiteboard:write:project_collaborator:admin\"]}},\"patch\":{\"tags\":[\"Project\"],\"summary\":\"Update project collaborators\",\"description\":\"Updates the role of a collaborator in a project, which allows you to modify the permissions of an existing collaborator in a specific project. \\nNote that when assigning a new `OWNER` or `CO_OWNER`, the specified user must belong to the same account as the original `OWNER`. The original owner will automatically be demoted to `CO_OWNER` if a new owner is designated.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project_collaborator:write`,`whiteboard_project_collaborator:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:update:project_collaborator`,`whiteboard:update:project_collaborator:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"Updateprojectcollaborators\",\"parameters\":[{\"name\":\"projectId\",\"in\":\"path\",\"description\":\"The project ID.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"N6XZFDEQTkWSsMsmO_M1XQ\"}}],\"requestBody\":{\"content\":{\"application/json\":{\"schema\":{\"required\":[\"collaborators\"],\"type\":\"object\",\"properties\":{\"collaborators\":{\"maxItems\":20,\"minItems\":1,\"type\":\"array\",\"description\":\"An array of collaborator objects.\",\"items\":{\"required\":[\"collaborator_id\",\"role\"],\"type\":\"object\",\"properties\":{\"collaborator_id\":{\"type\":\"string\",\"description\":\"The collaborator's unique ID `not nullable` .\",\"example\":\"x4g6hy789k0o878k\"},\"role\":{\"type\":\"integer\",\"description\":\"Collaborator role type.\\nWhen assigning a new `OWNER` or `CO_OWNER`, the specified user must belong to the same account as the original `OWNER`. If a new owner is designated for a project, the original owner will automatically be demoted to `CO_OWNER`.\\n\\nRole Enum Values:\\n\\n0 - OWNER\\n1 - CO_OWNER\\n2 - EDITOR\\n3 - COMMENTER\\n4 - VIEWER\",\"example\":2,\"default\":2,\"enum\":[0,1,2,3,4]}}}}},\"description\":\"Update collaborators permission of a whiteboard.\"}}}},\"responses\":{\"204\":{\"description\":\"Project collaborators updated successfully.\"},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `34002` <br>\\n Invalid collaborator_id {collaboratorId}. <br>\\n\"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111804` <br>\\n Permission is needed to access this project. <br>\\n**Error Code:** `34001` <br>\\n You do not have permission to update the role of collaborator {collaboratorId}. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `35004` <br>\\n The project ID {projectID} is not exist. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Edit\"],\"x-macro-scopes\":[\"whiteboard_project_collaborator:write\",\"whiteboard_project_collaborator:write:admin\"],\"x-granular-scopes\":[\"whiteboard:update:project_collaborator\",\"whiteboard:update:project_collaborator:admin\"]}}},\"/whiteboards/projects/{projectId}/collaborators/{collaboratorId}\":{\"delete\":{\"tags\":[\"Project\"],\"summary\":\"Remove the collaborator from a project\",\"description\":\"This API endpoint allows you to remove a specific collaborator from a designated project. It enables project managers or authorized users to revoke access or remove a user from a project, enhancing project management and access control capabilities.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project_collaborator:write`,`whiteboard_project_collaborator:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:delete:project_collaborator:admin`,`whiteboard:delete:project_collaborator`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"Removethecollaboratorfromaproject\",\"parameters\":[{\"name\":\"projectId\",\"in\":\"path\",\"description\":\"The project ID.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"N6XZFDEQTkWSsMsmO_M1XQ\"}},{\"name\":\"collaboratorId\",\"in\":\"path\",\"description\":\"The collaborator ID.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"x4g6hy789k0o878k\"}}],\"responses\":{\"204\":{\"description\":\"The collaborator has been successfully removed from the project. No content is returned in the response body.\"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111804` <br>\\n Permission is needed to access this project. <br>\\n**Error Code:** `34001` <br>\\n You do not have permission to remove the collaborator {collaboratorId} from the project. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `38004` <br>\\n The project ID {projectID} is not exist. <br>\\n**Error Code:** `34005` <br>\\n The collaborator ID {collaboratorId} is not exist. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Edit\"],\"x-macro-scopes\":[\"whiteboard_project_collaborator:write\",\"whiteboard_project_collaborator:write:admin\"],\"x-granular-scopes\":[\"whiteboard:delete:project_collaborator:admin\",\"whiteboard:delete:project_collaborator\"]}}},\"/whiteboards/projects/{projectId}/whiteboards\":{\"post\":{\"tags\":[\"Project\"],\"summary\":\"Move whiteboards to a project\",\"description\":\"This API endpoint allows you to move one or more whiteboards to a specified project. It enables users to organize their whiteboards by associating them with a specific project, enhancing project management capabilities.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:write:admin`,`whiteboard_project:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:write:project_whiteboard`,`whiteboard:write:project_whiteboard:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"Movewhiteboardstoproject\",\"parameters\":[{\"name\":\"projectId\",\"in\":\"path\",\"description\":\"The Project ID\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"qiMWddmtRxqwSFrK22lJPg\"}}],\"requestBody\":{\"content\":{\"application/json\":{\"schema\":{\"required\":[\"whiteboard_ids\"],\"type\":\"object\",\"properties\":{\"whiteboard_ids\":{\"title\":\"whiteboard_id\",\"maxItems\":50,\"minItems\":1,\"type\":\"array\",\"description\":\"The whiteboard ID list\",\"items\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\",\"example\":\"dZO3Wd9XT4mgXdNPbMm6tw\"}}}}}}},\"responses\":{\"201\":{\"description\":\"The whiteboards have been successfully moved to the project.\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"whiteboard_ids\":{\"type\":\"array\",\"description\":\"The whiteboard ID list\",\"items\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\",\"example\":\"dZO3Wd9XT4mgXdNPbMm6tw\"}}}}}}},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111804` <br>\\n Permission is needed to access this project. <br>\\n**Error Code:** `34001` <br>\\n You do not have permission to move {whiteboardID} to the project. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `38007` <br>\\n The projectId is not exist. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Edit\"],\"x-macro-scopes\":[\"whiteboard_project:write:admin\",\"whiteboard_project:write\"],\"x-granular-scopes\":[\"whiteboard:write:project_whiteboard\",\"whiteboard:write:project_whiteboard:admin\"]}},\"delete\":{\"tags\":[\"Project\"],\"summary\":\"Remove whiteboards from a project\",\"description\":\"This API endpoint allows you to remove one or more whiteboards from a specified project. \\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_project:write:admin`,`whiteboard_project:write`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:delete:project_whiteboard`,`whiteboard:delete:project_whiteboard:admin`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"Removewhiteboardsfromaproject\",\"parameters\":[{\"name\":\"projectId\",\"in\":\"path\",\"description\":\"The project ID.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"N6XZFDEQTkWSsMsmO_M1XQ\"}}],\"requestBody\":{\"content\":{\"application/json\":{\"schema\":{\"required\":[\"whiteboard_ids\"],\"type\":\"object\",\"properties\":{\"whiteboard_ids\":{\"maxItems\":50,\"minItems\":1,\"type\":\"array\",\"description\":\"The whiteboard ID list.\",\"items\":{\"type\":\"string\",\"description\":\"The whiteboard ID.\",\"example\":\"W6Uuv4qUSfuJlp_GXGi36A\"}}}}}}},\"responses\":{\"204\":{\"description\":\"The whiteboards have been successfully removed from the project. No content is returned in the response body.\"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111804` <br>\\n Permission is needed to access this project. <br>\\n**Error Code:** `34001` <br>\\n You have no permission to add {whiteboardID} to the project. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `38007` <br>\\n The project ID does not exist. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Edit\"],\"x-macro-scopes\":[\"whiteboard_project:write:admin\",\"whiteboard_project:write\"],\"x-granular-scopes\":[\"whiteboard:delete:project_whiteboard\",\"whiteboard:delete:project_whiteboard:admin\"]}}},\"/whiteboards/{whiteboardId}/share_setting\":{\"patch\":{\"tags\":[\"Settings\"],\"summary\":\"Update whiteboard share setting\",\"description\":\"Updates the whiteboard share setting. \\n\\n\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard_share_setting:write`,`whiteboard_share_setting:write:admin`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `whiteboard:update:share_setting:admin`,`whiteboard:update:share_setting`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `MEDIUM`\",\"operationId\":\"UpdateAWhiteboardShareSetting\",\"parameters\":[{\"name\":\"whiteboardId\",\"in\":\"path\",\"description\":\"The ID of the whiteboard.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"HY8-AC_HSou4SXY1S-OmOg\"}}],\"requestBody\":{\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"share_link_setting\":{\"type\":\"object\",\"properties\":{\"share_scope\":{\"type\":\"integer\",\"description\":\"The access type of whiteboard share link. \\n* Enum: \\n* `0` - `Restricted`, Only people added can open with this link \\n* `1` - `Same account` \\n* `2` - Anyone \\n* `3` - SAME_ACCOUNT_EXTERNAL_AUTH \\n\",\"example\":0,\"default\":0,\"enum\":[0,1,2,3],\"x-enum-descriptions\":[\"Restricted. Only people added can open with this link.\",\"Same account\",\"Anyone\",\"Same account external auth\"]},\"share_role\":{\"type\":\"integer\",\"description\":\"The access level of a whiteboard share link. Use this field with `share_scope`. If `share_scope` set `0` - `Restricted`, this field is null. If `share_scope` set `1` - `Same account` or `2` - `Anyone`, this filed can be `2` - `Editor`, `3` - `Commenter` or `4` - `Viewer`. \\n* Enum: \\n* `2` - `Editor`\\n* `3` - `Commenter` \\n* `4` - Viewer \\n\",\"example\":2,\"enum\":[2,3,4],\"x-enum-descriptions\":[\"Editor\",\"Commenter\",\"Viewer\"]}},\"description\":\"The access level and access type of a current whiteboard's share link.\"}},\"description\":\"Updates a whiteboard's share setting.\"}}}},\"responses\":{\"204\":{\"description\":\"**HTTP Status Code**: `204`   \\n \\nWhiteboard share setting updated successfully.\"},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `34002` <br>\\n Share role must be viewer for everyone. <br>\\n**Error Code:** `34002` <br>\\n Share role can't be null, owner, or co-owner for same account. <br>\\n\"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `111804` <br>\\n Permission is needed to access this document. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"WhiteboardManagement:Edit\"],\"x-macro-scopes\":[\"whiteboard_share_setting:write\",\"whiteboard_share_setting:write:admin\"],\"x-granular-scopes\":[\"whiteboard:update:share_setting:admin\",\"whiteboard:update:share_setting\"]}}}},\"components\":{\"securitySchemes\":{\"openapi_authorization\":{\"type\":\"http\",\"scheme\":\"bearer\"}}}}",
    "frontmatter": {
      "title": "Whiteboard APIs",
      "keywords": "zoom, marketplace, whiteboard, webhook, event, create, update, delete, data, summary",
      "description": "The [Zoom Whiteboard APIs](https://developers.zoom.us/docs/zoom-whiteboard/) allow you to manage your Zoom Whiteboard. For example, collaborative persistent whiteboard that allows participants to share and collaborate on a virtual whiteboard in real time The Zoom Whiteboard REST API for [Zoom Whiteboard](https://developers.zoom.us/docs/api/rest/whiteboard/) allows developers to interact with various Zoom features, including Zoom Whiteboard. Developers can create and manage whiteboards programmatically using these APIs.",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
