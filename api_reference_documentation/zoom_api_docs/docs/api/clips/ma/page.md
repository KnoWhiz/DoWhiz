# Clips Master APIs

- Source URL: https://developers.zoom.us/docs/api/clips/ma/
- Snapshot path: docs/api/clips/ma
- Generated (UTC): 2026-02-07T22:40:10.508440+00:00

## Frontmatter

```json
{
  "title": "Clips Master APIs",
  "keywords": "zoom, rest, api, clips, master account",
  "description": "Clips APIs allow developers to interface with [Zoom Clips](https://developers.zoom.us/docs/zoom-clips/) features programmatically. Zoom Clips enables users to capture, manage, and share short video recordings, making it a valuable tool for collaboration, communication, and documentation.",
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
    "title": "Clips",
    "description": "Clips APIs allow developers to interface with [Zoom Clips](https://developers.zoom.us/docs/zoom-clips/) features programmatically. Zoom Clips enables users to capture, manage, and share short video recordings, making it a valuable tool for collaboration, communication, and documentation.",
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
      "url": "https://api.zoom.us/"
    }
  ],
  "paths": {
    "/accounts/{accountId}/clips": {
      "get": {
        "tags": [
          "list subAccount clips in account"
        ],
        "summary": "List all clips",
        "description": "Returns a list of clips of the current user or a list of all clips under the account. \n\n **Required scopes** \n`clips:read:list_user_clips:admin)`\n\n**Note** \n`user_id` can intake Zoom user ID or email, which only applies when `scope = clips:read:list_user_clips:admin` or `scope=clips:read:list_user_clips:master`. Otherwise, the `user_id` can only intake `me`.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `clips:read:master`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `clips:read:list_user_clips:master`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "GetUserClips",
        "parameters": [
          {
            "name": "user_id",
            "in": "query",
            "description": "The email address or user ID of the clip owner. Providing this parameter will return only the clip owned by the specified user. If set to `me`, it returns the matched clips of current user.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "me"
            }
          },
          {
            "name": "search_key",
            "in": "query",
            "description": "The name of a clip. Providing this parameter enables the system to return only the matching clip. Otherwise, all clips will be returned.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "Clip-20250303"
            }
          },
          {
            "name": "date_filter_type",
            "in": "query",
            "description": "The clip date filter type. Enum:\n\nCREATED_DATE - clip create date.\nMODIFIED_DATE - clip modify date.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "MODIFIED_DATE",
              "default": "MODIFIED_DATE",
              "enum": [
                "CREATED_DATE",
                "MODIFIED_DATE"
              ]
            }
          },
          {
            "name": "from",
            "in": "query",
            "description": "The queried start date and time in yyyy-MM-dd'T'HH:mm:ss'Z' UTC format for the created date range for which you would like to retrieve clips. The maximum range can be a month. If no value is provided for this field, the default will be current date. For example, if you make the API request on June 30, 2024, without providing the \u201cfrom\u201d and \u201cto\u201d parameters, by default the value of 'from' field will be \u201c2024-06-30\u201d and the value of the 'to' field will be \u201c2024-07-01\u201d.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "2024-03-08T16:00:00Z"
            }
          },
          {
            "name": "to",
            "in": "query",
            "description": "The queried end date and time in yyyy-MM-dd'T'HH:mm:ss'Z' UTC format.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "2024-03-08T16:00:00Z"
            }
          },
          {
            "name": "status",
            "in": "query",
            "description": "The status of the clips. Enum:\n\nRECORD_START - clips init status\nRECORD_FAILED - clips failed\nRECORD_END - clips can play but can't download,no thumbnail and duration\nRECORD_SUCCESS - clips can play and download\nRECORD_DELETE - clips has been delete\nSUSPENDED - clips has been suspended",
            "required": false,
            "schema": {
              "type": "string",
              "example": "RECORD_SUCCESS",
              "enum": [
                "RECORD_START",
                "RECORD_FAILED",
                "RECORD_END",
                "RECORD_SUCCESS",
                "RECORD_DELETE",
                "SUSPENDED"
              ]
            }
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of records returned within a single API call.",
            "required": false,
            "schema": {
              "maximum": 300,
              "minimum": 30,
              "type": "integer",
              "example": 30
            }
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "The next page token paginates through large result sets. A next page token returns whenever the set of available results exceeds the current page size. This token's expiration period is 15 minutes.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "IAfJX3jsOLW7w3dokmFl84zOa0MAVGyMEB2"
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
            "description": "Clips list returned.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "data": {
                      "type": "array",
                      "description": "The list of clips.",
                      "items": {
                        "type": "object",
                        "properties": {
                          "title": {
                            "type": "string",
                            "description": "The title of the clip. ",
                            "example": "clips test title"
                          },
                          "description": {
                            "type": "string",
                            "description": "The description of the clip.",
                            "example": "just for test"
                          },
                          "duration": {
                            "type": "number",
                            "description": "The duration of the clip, where the unit of duration is seconds.",
                            "example": 6000
                          },
                          "status": {
                            "type": "string",
                            "description": "The clip status.\n\nRECORD_START - It means the clip recording has started but has not yet ended.\n\nRECORD_FAILED - It means the clip recording failed.\n\nRECORD_END - It means the clip recording has ended but the MP4 transcription is not yet complete.\n\nRECORD_SUCCESS - It means the clip recording has reached its final state and is fully complete.\n\nRECORD_DELETE - It means the clip has been soft deleted.\n\nSUSPENDED - It means the clip has been suspended by the trust & safety team.",
                            "example": "RECORD_SUCCESS",
                            "enum": [
                              "RECORD_START",
                              "RECORD_FAILED",
                              "RECORD_END",
                              "RECORD_SUCCESS",
                              "RECORD_DELETE",
                              "SUSPENDED"
                            ]
                          },
                          "clip_id": {
                            "type": "string",
                            "description": "The clip's ID.",
                            "example": "A2RldhYyU08zeDNIeFJJVzBOcHFRVlZQWXlRAQ"
                          },
                          "owner_id": {
                            "type": "string",
                            "description": "The owner's ID.",
                            "example": "pRof_6gpQP2YKDxmglRziA"
                          },
                          "file_size": {
                            "type": "integer",
                            "description": "The clip file size in bytes",
                            "example": 639229
                          },
                          "share_link": {
                            "type": "string",
                            "description": "The share link of the clip.",
                            "example": "https://dev-integration.zoomdev.us/clips/share/A2RldhYyU08zeDNIeFJJVzBOcHFRVlZQWXlRAQ"
                          },
                          "share_link_settings": {
                            "type": "object",
                            "properties": {
                              "passcode": {
                                "type": "string",
                                "description": "The passcode. It exists only when `enable_passcode=true`",
                                "example": "Dmpy*xy7"
                              },
                              "share_scope": {
                                "type": "string",
                                "description": "ANYONE - Anyone with this clip link can watch it.\n\nSAME_ORGANIZATON - Only users who are in the same organization as the clip owner can watch it.\n\nINVITED_MEMBERS_ONLY - Only invited users can watch it.\n\nPRIVATE - Only the clip's owner and account administrators can watch it.",
                                "example": "ANYONE",
                                "enum": [
                                  "ANYONE",
                                  "SAME_ORGANIZATON",
                                  "INVITED_MEMBERS_ONLY",
                                  "PRIVATE"
                                ]
                              },
                              "enable_passcode": {
                                "type": "boolean",
                                "description": " Whether the passcode is enabled.",
                                "example": true
                              }
                            },
                            "description": "The access level, which indicates the access type of the current clips' share link."
                          },
                          "thumbnail_link": {
                            "type": "string",
                            "description": " The clip thumbnail link.",
                            "example": "https://file.zoomdev.us/file/2SO3x3HxRIW0NpqQVVPYyQ?attachType=preview&filename=2SO3x3HxRIW0NpqQVVPYyQ.jpg&jwt=eyJhbGciOiJFUzI1NiIsImsiOiJLSzQxYXo3LyJ9.eyJpc3MiOiJmaWxlIiwiYXVkIjoiemZzIiwiaWljIjoiZGV2IiwiaWF0IjoxNzMwNDM4OTE2LCJvcmkiOiJtYXJ2ZWwiLCJkaWciOiI5NDhhMjIxYjU2ZTFhOWM1YTMyOTE0ODdlNjFmMGU1YjA2NzkzNTM3NTY5NDRhZWVjNzcyYmU3OTQ3ZWQ3ZTAwIiwiZXhwIjoxNzMwNDQ2MTE2LCJoZGlnIjpmYWxzZX0.Lgk1SRRKsxrxWN6F9Y9aZgfqXCSZ1mJg-UsunpVCMuRMYUR1Sf-gGn1_2_Fc1gFexkBKr0C50jT7Vo1_1YQUlQ&subType=552x310"
                          },
                          "created_date": {
                            "type": "string",
                            "description": "The clip created date-time in UTC/GMT. Example: \"2020-03-31T12:02:00Z\"",
                            "example": "2024-10-11T01:51:50.631+00:00"
                          },
                          "modified_date": {
                            "type": "string",
                            "description": "The clip modified date-time in UTC/GMT. Example: \"2020-03-31T12:02:00Z\"",
                            "example": "2024-10-31T10:06:08.460+00:00"
                          }
                        },
                        "description": "The information about the clips list."
                      }
                    },
                    "next_page_token": {
                      "maxLength": 100,
                      "type": "string",
                      "description": "The next page token paginates through large result sets. A next page token returns whenever the set of available results exceeds the current page size. This token's expiration period is 15 minutes.",
                      "example": "R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42"
                    },
                    "page_size": {
                      "maximum": 300,
                      "type": "integer",
                      "description": "The number of records returned within a single API call.",
                      "example": 30,
                      "default": 30
                    },
                    "total_records": {
                      "type": "number",
                      "description": "The total number of records found.",
                      "example": 30
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `300` <br>\n Pagination Error <br>\n"
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n **Error Code:** `1002` <br>\n No authenticated <br>\n"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `1003` <br>\n No permission to operate <br>\n"
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
            "ClipContent:Read"
          ],
          "x-macro-scopes": [
            "clips:read:master"
          ],
          "x-granular-scopes": [
            "clips:read:list_user_clips:master"
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
    "title": "Clips",
    "description": "Clips APIs allow developers to interface with [Zoom Clips](https://developers.zoom.us/docs/zoom-clips/) features programmatically. Zoom Clips enables users to capture, manage, and share short video recordings, making it a valuable tool for collaboration, communication, and documentation.",
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
      "url": "https://api.zoom.us/"
    }
  ],
  "paths": {
    "/accounts/{accountId}/clips": {
      "get": {
        "tags": [
          "list subAccount clips in account"
        ],
        "summary": "List all clips",
        "description": "Returns a list of clips of the current user or a list of all clips under the account. \n\n **Required scopes** \n`clips:read:list_user_clips:admin)`\n\n**Note** \n`user_id` can intake Zoom user ID or email, which only applies when `scope = clips:read:list_user_clips:admin` or `scope=clips:read:list_user_clips:master`. Otherwise, the `user_id` can only intake `me`.\n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `clips:read:master`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `clips:read:list_user_clips:master`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`",
        "operationId": "GetUserClips",
        "parameters": [
          {
            "name": "user_id",
            "in": "query",
            "description": "The email address or user ID of the clip owner. Providing this parameter will return only the clip owned by the specified user. If set to `me`, it returns the matched clips of current user.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "me"
            }
          },
          {
            "name": "search_key",
            "in": "query",
            "description": "The name of a clip. Providing this parameter enables the system to return only the matching clip. Otherwise, all clips will be returned.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "Clip-20250303"
            }
          },
          {
            "name": "date_filter_type",
            "in": "query",
            "description": "The clip date filter type. Enum:\n\nCREATED_DATE - clip create date.\nMODIFIED_DATE - clip modify date.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "MODIFIED_DATE",
              "default": "MODIFIED_DATE",
              "enum": [
                "CREATED_DATE",
                "MODIFIED_DATE"
              ]
            }
          },
          {
            "name": "from",
            "in": "query",
            "description": "The queried start date and time in yyyy-MM-dd'T'HH:mm:ss'Z' UTC format for the created date range for which you would like to retrieve clips. The maximum range can be a month. If no value is provided for this field, the default will be current date. For example, if you make the API request on June 30, 2024, without providing the \u201cfrom\u201d and \u201cto\u201d parameters, by default the value of 'from' field will be \u201c2024-06-30\u201d and the value of the 'to' field will be \u201c2024-07-01\u201d.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "2024-03-08T16:00:00Z"
            }
          },
          {
            "name": "to",
            "in": "query",
            "description": "The queried end date and time in yyyy-MM-dd'T'HH:mm:ss'Z' UTC format.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "2024-03-08T16:00:00Z"
            }
          },
          {
            "name": "status",
            "in": "query",
            "description": "The status of the clips. Enum:\n\nRECORD_START - clips init status\nRECORD_FAILED - clips failed\nRECORD_END - clips can play but can't download,no thumbnail and duration\nRECORD_SUCCESS - clips can play and download\nRECORD_DELETE - clips has been delete\nSUSPENDED - clips has been suspended",
            "required": false,
            "schema": {
              "type": "string",
              "example": "RECORD_SUCCESS",
              "enum": [
                "RECORD_START",
                "RECORD_FAILED",
                "RECORD_END",
                "RECORD_SUCCESS",
                "RECORD_DELETE",
                "SUSPENDED"
              ]
            }
          },
          {
            "name": "page_size",
            "in": "query",
            "description": "The number of records returned within a single API call.",
            "required": false,
            "schema": {
              "maximum": 300,
              "minimum": 30,
              "type": "integer",
              "example": 30
            }
          },
          {
            "name": "next_page_token",
            "in": "query",
            "description": "The next page token paginates through large result sets. A next page token returns whenever the set of available results exceeds the current page size. This token's expiration period is 15 minutes.",
            "required": false,
            "schema": {
              "type": "string",
              "example": "IAfJX3jsOLW7w3dokmFl84zOa0MAVGyMEB2"
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
            "description": "Clips list returned.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "data": {
                      "type": "array",
                      "description": "The list of clips.",
                      "items": {
                        "type": "object",
                        "properties": {
                          "title": {
                            "type": "string",
                            "description": "The title of the clip. ",
                            "example": "clips test title"
                          },
                          "description": {
                            "type": "string",
                            "description": "The description of the clip.",
                            "example": "just for test"
                          },
                          "duration": {
                            "type": "number",
                            "description": "The duration of the clip, where the unit of duration is seconds.",
                            "example": 6000
                          },
                          "status": {
                            "type": "string",
                            "description": "The clip status.\n\nRECORD_START - It means the clip recording has started but has not yet ended.\n\nRECORD_FAILED - It means the clip recording failed.\n\nRECORD_END - It means the clip recording has ended but the MP4 transcription is not yet complete.\n\nRECORD_SUCCESS - It means the clip recording has reached its final state and is fully complete.\n\nRECORD_DELETE - It means the clip has been soft deleted.\n\nSUSPENDED - It means the clip has been suspended by the trust & safety team.",
                            "example": "RECORD_SUCCESS",
                            "enum": [
                              "RECORD_START",
                              "RECORD_FAILED",
                              "RECORD_END",
                              "RECORD_SUCCESS",
                              "RECORD_DELETE",
                              "SUSPENDED"
                            ]
                          },
                          "clip_id": {
                            "type": "string",
                            "description": "The clip's ID.",
                            "example": "A2RldhYyU08zeDNIeFJJVzBOcHFRVlZQWXlRAQ"
                          },
                          "owner_id": {
                            "type": "string",
                            "description": "The owner's ID.",
                            "example": "pRof_6gpQP2YKDxmglRziA"
                          },
                          "file_size": {
                            "type": "integer",
                            "description": "The clip file size in bytes",
                            "example": 639229
                          },
                          "share_link": {
                            "type": "string",
                            "description": "The share link of the clip.",
                            "example": "https://dev-integration.zoomdev.us/clips/share/A2RldhYyU08zeDNIeFJJVzBOcHFRVlZQWXlRAQ"
                          },
                          "share_link_settings": {
                            "type": "object",
                            "properties": {
                              "passcode": {
                                "type": "string",
                                "description": "The passcode. It exists only when `enable_passcode=true`",
                                "example": "Dmpy*xy7"
                              },
                              "share_scope": {
                                "type": "string",
                                "description": "ANYONE - Anyone with this clip link can watch it.\n\nSAME_ORGANIZATON - Only users who are in the same organization as the clip owner can watch it.\n\nINVITED_MEMBERS_ONLY - Only invited users can watch it.\n\nPRIVATE - Only the clip's owner and account administrators can watch it.",
                                "example": "ANYONE",
                                "enum": [
                                  "ANYONE",
                                  "SAME_ORGANIZATON",
                                  "INVITED_MEMBERS_ONLY",
                                  "PRIVATE"
                                ]
                              },
                              "enable_passcode": {
                                "type": "boolean",
                                "description": " Whether the passcode is enabled.",
                                "example": true
                              }
                            },
                            "description": "The access level, which indicates the access type of the current clips' share link."
                          },
                          "thumbnail_link": {
                            "type": "string",
                            "description": " The clip thumbnail link.",
                            "example": "https://file.zoomdev.us/file/2SO3x3HxRIW0NpqQVVPYyQ?attachType=preview&filename=2SO3x3HxRIW0NpqQVVPYyQ.jpg&jwt=eyJhbGciOiJFUzI1NiIsImsiOiJLSzQxYXo3LyJ9.eyJpc3MiOiJmaWxlIiwiYXVkIjoiemZzIiwiaWljIjoiZGV2IiwiaWF0IjoxNzMwNDM4OTE2LCJvcmkiOiJtYXJ2ZWwiLCJkaWciOiI5NDhhMjIxYjU2ZTFhOWM1YTMyOTE0ODdlNjFmMGU1YjA2NzkzNTM3NTY5NDRhZWVjNzcyYmU3OTQ3ZWQ3ZTAwIiwiZXhwIjoxNzMwNDQ2MTE2LCJoZGlnIjpmYWxzZX0.Lgk1SRRKsxrxWN6F9Y9aZgfqXCSZ1mJg-UsunpVCMuRMYUR1Sf-gGn1_2_Fc1gFexkBKr0C50jT7Vo1_1YQUlQ&subType=552x310"
                          },
                          "created_date": {
                            "type": "string",
                            "description": "The clip created date-time in UTC/GMT. Example: \"2020-03-31T12:02:00Z\"",
                            "example": "2024-10-11T01:51:50.631+00:00"
                          },
                          "modified_date": {
                            "type": "string",
                            "description": "The clip modified date-time in UTC/GMT. Example: \"2020-03-31T12:02:00Z\"",
                            "example": "2024-10-31T10:06:08.460+00:00"
                          }
                        },
                        "description": "The information about the clips list."
                      }
                    },
                    "next_page_token": {
                      "maxLength": 100,
                      "type": "string",
                      "description": "The next page token paginates through large result sets. A next page token returns whenever the set of available results exceeds the current page size. This token's expiration period is 15 minutes.",
                      "example": "R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42"
                    },
                    "page_size": {
                      "maximum": 300,
                      "type": "integer",
                      "description": "The number of records returned within a single API call.",
                      "example": 30,
                      "default": 30
                    },
                    "total_records": {
                      "type": "number",
                      "description": "The total number of records found.",
                      "example": 30
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `300` <br>\n Pagination Error <br>\n"
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n **Error Code:** `1002` <br>\n No authenticated <br>\n"
          },
          "403": {
            "description": "**HTTP Status Code:** `403` <br>\n Forbidden  \n\n **Error Code:** `1003` <br>\n No permission to operate <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "clips:read:master",
              "clips:read:list_user_clips:master"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [
            "ClipContent:Read"
          ],
          "x-macro-scopes": [
            "clips:read:master"
          ],
          "x-granular-scopes": [
            "clips:read:list_user_clips:master"
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
              "clips:read:master": "clips:read:master",
              "clips:read:list_user_clips:master": "clips:read:list_user_clips:master"
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
      "clips",
      "ma"
    ]
  },
  "downloadPath": "/api-hub/clips/ma/master.json",
  "apiOptions": {
    "events": false,
    "methods": true,
    "ma": true
  },
  "productMeta": {
    "specType": "oas",
    "slug": "/docs/api/rest/reference/zoom-clips/ma",
    "title": "Zoom Clips API",
    "description": "",
    "keywords": "zoom, rest, api, clips, master account",
    "params": {
      "productName": "Zoom Clips",
      "maFlag": true
    },
    "zoapEndpoint": "docJsonByFixVersion",
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "productDisplayTitle": "Clips",
    "scalarSlug": "/docs/api/clips/ma",
    "svgname": "_zoomclips"
  },
  "frontmatter": {
    "title": "Clips Master APIs",
    "keywords": "zoom, rest, api, clips, master account",
    "description": "Clips APIs allow developers to interface with [Zoom Clips](https://developers.zoom.us/docs/zoom-clips/) features programmatically. Zoom Clips enables users to capture, manage, and share short video recordings, making it a valuable tool for collaboration, communication, and documentation.",
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
        "clips",
        "ma"
      ]
    },
    "downloadPath": "/api-hub/clips/ma/master.json",
    "apiOptions": {
      "events": false,
      "methods": true,
      "ma": true
    },
    "productMeta": {
      "specType": "oas",
      "slug": "/docs/api/rest/reference/zoom-clips/ma",
      "title": "Zoom Clips API",
      "description": "",
      "keywords": "zoom, rest, api, clips, master account",
      "params": {
        "productName": "Zoom Clips",
        "maFlag": true
      },
      "zoapEndpoint": "docJsonByFixVersion",
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "productDisplayTitle": "Clips",
      "scalarSlug": "/docs/api/clips/ma",
      "svgname": "_zoomclips"
    },
    "spec": "{\"openapi\":\"3.0.0\",\"info\":{\"title\":\"Clips\",\"description\":\"Clips APIs allow developers to interface with [Zoom Clips](https://developers.zoom.us/docs/zoom-clips/) features programmatically. Zoom Clips enables users to capture, manage, and share short video recordings, making it a valuable tool for collaboration, communication, and documentation.\",\"termsOfService\":\"https://zoom.us/docs/en-us/zoom_api_license_and_tou.html\",\"contact\":{\"name\":\"Zoom Developers\",\"url\":\"https://developer.zoom.us/\"},\"version\":\"2\"},\"externalDocs\":{\"description\":\"Find out more about Swagger\",\"url\":\"https://swagger.io\"},\"servers\":[{\"url\":\"https://api.zoom.us/\"}],\"paths\":{\"/accounts/{accountId}/clips\":{\"get\":{\"tags\":[\"list subAccount clips in account\"],\"summary\":\"List all clips\",\"description\":\"Returns a list of clips of the current user or a list of all clips under the account. \\n\\n **Required scopes** \\n`clips:read:list_user_clips:admin)`\\n\\n**Note** \\n`user_id` can intake Zoom user ID or email, which only applies when `scope = clips:read:list_user_clips:admin` or `scope=clips:read:list_user_clips:master`. Otherwise, the `user_id` can only intake `me`.\\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `clips:read:master`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `clips:read:list_user_clips:master`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `LIGHT`\",\"operationId\":\"GetUserClips\",\"parameters\":[{\"name\":\"user_id\",\"in\":\"query\",\"description\":\"The email address or user ID of the clip owner. Providing this parameter will return only the clip owned by the specified user. If set to `me`, it returns the matched clips of current user.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"me\"}},{\"name\":\"search_key\",\"in\":\"query\",\"description\":\"The name of a clip. Providing this parameter enables the system to return only the matching clip. Otherwise, all clips will be returned.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"Clip-20250303\"}},{\"name\":\"date_filter_type\",\"in\":\"query\",\"description\":\"The clip date filter type. Enum:\\n\\nCREATED_DATE - clip create date.\\nMODIFIED_DATE - clip modify date.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"MODIFIED_DATE\",\"default\":\"MODIFIED_DATE\",\"enum\":[\"CREATED_DATE\",\"MODIFIED_DATE\"]}},{\"name\":\"from\",\"in\":\"query\",\"description\":\"The queried start date and time in yyyy-MM-dd'T'HH:mm:ss'Z' UTC format for the created date range for which you would like to retrieve clips. The maximum range can be a month. If no value is provided for this field, the default will be current date. For example, if you make the API request on June 30, 2024, without providing the \u201cfrom\u201d and \u201cto\u201d parameters, by default the value of 'from' field will be \u201c2024-06-30\u201d and the value of the 'to' field will be \u201c2024-07-01\u201d.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"2024-03-08T16:00:00Z\"}},{\"name\":\"to\",\"in\":\"query\",\"description\":\"The queried end date and time in yyyy-MM-dd'T'HH:mm:ss'Z' UTC format.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"2024-03-08T16:00:00Z\"}},{\"name\":\"status\",\"in\":\"query\",\"description\":\"The status of the clips. Enum:\\n\\nRECORD_START - clips init status\\nRECORD_FAILED - clips failed\\nRECORD_END - clips can play but can't download,no thumbnail and duration\\nRECORD_SUCCESS - clips can play and download\\nRECORD_DELETE - clips has been delete\\nSUSPENDED - clips has been suspended\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"RECORD_SUCCESS\",\"enum\":[\"RECORD_START\",\"RECORD_FAILED\",\"RECORD_END\",\"RECORD_SUCCESS\",\"RECORD_DELETE\",\"SUSPENDED\"]}},{\"name\":\"page_size\",\"in\":\"query\",\"description\":\"The number of records returned within a single API call.\",\"required\":false,\"schema\":{\"maximum\":300,\"minimum\":30,\"type\":\"integer\",\"example\":30}},{\"name\":\"next_page_token\",\"in\":\"query\",\"description\":\"The next page token paginates through large result sets. A next page token returns whenever the set of available results exceeds the current page size. This token's expiration period is 15 minutes.\",\"required\":false,\"schema\":{\"type\":\"string\",\"example\":\"IAfJX3jsOLW7w3dokmFl84zOa0MAVGyMEB2\"}},{\"name\":\"accountId\",\"in\":\"path\",\"description\":\"Unique identifier of the account.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"TQTvjT52Tmi_wrhASNNOEw\"}}],\"responses\":{\"200\":{\"description\":\"Clips list returned.\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"data\":{\"type\":\"array\",\"description\":\"The list of clips.\",\"items\":{\"type\":\"object\",\"properties\":{\"title\":{\"type\":\"string\",\"description\":\"The title of the clip. \",\"example\":\"clips test title\"},\"description\":{\"type\":\"string\",\"description\":\"The description of the clip.\",\"example\":\"just for test\"},\"duration\":{\"type\":\"number\",\"description\":\"The duration of the clip, where the unit of duration is seconds.\",\"example\":6000},\"status\":{\"type\":\"string\",\"description\":\"The clip status.\\n\\nRECORD_START - It means the clip recording has started but has not yet ended.\\n\\nRECORD_FAILED - It means the clip recording failed.\\n\\nRECORD_END - It means the clip recording has ended but the MP4 transcription is not yet complete.\\n\\nRECORD_SUCCESS - It means the clip recording has reached its final state and is fully complete.\\n\\nRECORD_DELETE - It means the clip has been soft deleted.\\n\\nSUSPENDED - It means the clip has been suspended by the trust & safety team.\",\"example\":\"RECORD_SUCCESS\",\"enum\":[\"RECORD_START\",\"RECORD_FAILED\",\"RECORD_END\",\"RECORD_SUCCESS\",\"RECORD_DELETE\",\"SUSPENDED\"]},\"clip_id\":{\"type\":\"string\",\"description\":\"The clip's ID.\",\"example\":\"A2RldhYyU08zeDNIeFJJVzBOcHFRVlZQWXlRAQ\"},\"owner_id\":{\"type\":\"string\",\"description\":\"The owner's ID.\",\"example\":\"pRof_6gpQP2YKDxmglRziA\"},\"file_size\":{\"type\":\"integer\",\"description\":\"The clip file size in bytes\",\"example\":639229},\"share_link\":{\"type\":\"string\",\"description\":\"The share link of the clip.\",\"example\":\"https://dev-integration.zoomdev.us/clips/share/A2RldhYyU08zeDNIeFJJVzBOcHFRVlZQWXlRAQ\"},\"share_link_settings\":{\"type\":\"object\",\"properties\":{\"passcode\":{\"type\":\"string\",\"description\":\"The passcode. It exists only when `enable_passcode=true`\",\"example\":\"Dmpy*xy7\"},\"share_scope\":{\"type\":\"string\",\"description\":\"ANYONE - Anyone with this clip link can watch it.\\n\\nSAME_ORGANIZATON - Only users who are in the same organization as the clip owner can watch it.\\n\\nINVITED_MEMBERS_ONLY - Only invited users can watch it.\\n\\nPRIVATE - Only the clip's owner and account administrators can watch it.\",\"example\":\"ANYONE\",\"enum\":[\"ANYONE\",\"SAME_ORGANIZATON\",\"INVITED_MEMBERS_ONLY\",\"PRIVATE\"]},\"enable_passcode\":{\"type\":\"boolean\",\"description\":\" Whether the passcode is enabled.\",\"example\":true}},\"description\":\"The access level, which indicates the access type of the current clips' share link.\"},\"thumbnail_link\":{\"type\":\"string\",\"description\":\" The clip thumbnail link.\",\"example\":\"https://file.zoomdev.us/file/2SO3x3HxRIW0NpqQVVPYyQ?attachType=preview&filename=2SO3x3HxRIW0NpqQVVPYyQ.jpg&jwt=eyJhbGciOiJFUzI1NiIsImsiOiJLSzQxYXo3LyJ9.eyJpc3MiOiJmaWxlIiwiYXVkIjoiemZzIiwiaWljIjoiZGV2IiwiaWF0IjoxNzMwNDM4OTE2LCJvcmkiOiJtYXJ2ZWwiLCJkaWciOiI5NDhhMjIxYjU2ZTFhOWM1YTMyOTE0ODdlNjFmMGU1YjA2NzkzNTM3NTY5NDRhZWVjNzcyYmU3OTQ3ZWQ3ZTAwIiwiZXhwIjoxNzMwNDQ2MTE2LCJoZGlnIjpmYWxzZX0.Lgk1SRRKsxrxWN6F9Y9aZgfqXCSZ1mJg-UsunpVCMuRMYUR1Sf-gGn1_2_Fc1gFexkBKr0C50jT7Vo1_1YQUlQ&subType=552x310\"},\"created_date\":{\"type\":\"string\",\"description\":\"The clip created date-time in UTC/GMT. Example: \\\"2020-03-31T12:02:00Z\\\"\",\"example\":\"2024-10-11T01:51:50.631+00:00\"},\"modified_date\":{\"type\":\"string\",\"description\":\"The clip modified date-time in UTC/GMT. Example: \\\"2020-03-31T12:02:00Z\\\"\",\"example\":\"2024-10-31T10:06:08.460+00:00\"}},\"description\":\"The information about the clips list.\"}},\"next_page_token\":{\"maxLength\":100,\"type\":\"string\",\"description\":\"The next page token paginates through large result sets. A next page token returns whenever the set of available results exceeds the current page size. This token's expiration period is 15 minutes.\",\"example\":\"R4aF9Oj0fVM2hhezJTEmSKaBSkfesDwGy42\"},\"page_size\":{\"maximum\":300,\"type\":\"integer\",\"description\":\"The number of records returned within a single API call.\",\"example\":30,\"default\":30},\"total_records\":{\"type\":\"number\",\"description\":\"The total number of records found.\",\"example\":30}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `300` <br>\\n Pagination Error <br>\\n\"},\"401\":{\"description\":\"**HTTP Status Code:** `401` <br>\\n Unauthorized  \\n\\n **Error Code:** `1002` <br>\\n No authenticated <br>\\n\"},\"403\":{\"description\":\"**HTTP Status Code:** `403` <br>\\n Forbidden  \\n\\n **Error Code:** `1003` <br>\\n No permission to operate <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[\"ClipContent:Read\"],\"x-macro-scopes\":[\"clips:read:master\"],\"x-granular-scopes\":[\"clips:read:list_user_clips:master\"]}}}},\"components\":{\"securitySchemes\":{\"openapi_authorization\":{\"type\":\"http\",\"scheme\":\"bearer\"}}}}",
    "frontmatter": {
      "title": "Clips Master APIs",
      "keywords": "zoom, rest, api, clips, master account",
      "description": "Clips APIs allow developers to interface with [Zoom Clips](https://developers.zoom.us/docs/zoom-clips/) features programmatically. Zoom Clips enables users to capture, manage, and share short video recordings, making it a valuable tool for collaboration, communication, and documentation.",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
