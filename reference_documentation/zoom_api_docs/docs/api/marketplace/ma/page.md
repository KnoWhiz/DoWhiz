# Marketplace Master APIs

- Source URL: https://developers.zoom.us/docs/api/marketplace/ma/
- Snapshot path: docs/api/marketplace/ma
- Generated (UTC): 2026-02-07T22:40:10.846387+00:00

## Frontmatter

```json
{
  "title": "Marketplace Master APIs",
  "keywords": "zoom, api, marketplace, master account",
  "description": "The Marketplace APIs allow developers to access data related to their app and its usage in the marketplace.",
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
    "title": "Marketplace",
    "description": "The Marketplace APIs allow developers to access data related to their app and its usage in the marketplace.",
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
    "/accounts/{accountId}/marketplace/apps": {
      "post": {
        "tags": [
          "App"
        ],
        "summary": "Create apps",
        "description": "Create `Server to Server OAuth`, `Meeting SDK` or `General` apps in the Zoom Marketplace. \n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `marketplace_app:master`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `marketplace:write:app:master`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `HEAVY`",
        "operationId": "CreateApps",
        "parameters": [
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
        "requestBody": {
          "description": "/v2/marketplace/apps needs to pass the manifest parameter to create a General app.\n/v2/marketplace/apps/accounts/{accountld} needs to pass other parameters to create Server OAuth, Meeting SDK app.",
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "app_name",
                  "app_type",
                  "company_name",
                  "contact_email",
                  "contact_name"
                ],
                "type": "object",
                "properties": {
                  "app_type": {
                    "type": "string",
                    "description": "The app's type: \n* `s2s_oauth` &mdash; Server to Server OAuth app. \n* `meeting_sdk` &mdash; Meeting SDK app.\n* `general`&mdash; UBF General app.",
                    "example": "s2s_oauth",
                    "enum": [
                      "s2s_oauth",
                      "meeting_sdk",
                      "general"
                    ]
                  },
                  "app_name": {
                    "maxLength": 50,
                    "type": "string",
                    "description": "The app's name.",
                    "example": "My App"
                  },
                  "scopes": {
                    "type": "array",
                    "description": "The scopes of the created app:\n* `s2s_oauth` &mdash; If `active` field is true, there must be at least one scope. \n* `meeting_sdk` &mdash; Will automatically include `user_zak:read` scope.",
                    "items": {
                      "type": "string",
                      "example": "meeting:read"
                    }
                  },
                  "contact_name": {
                    "maxLength": 150,
                    "type": "string",
                    "description": "The developer's name.",
                    "example": "ZOOM"
                  },
                  "contact_email": {
                    "maxLength": 150,
                    "type": "string",
                    "description": "The developer's email.",
                    "format": "email",
                    "example": "example@example.com"
                  },
                  "company_name": {
                    "maxLength": 150,
                    "type": "string",
                    "description": "The company's name.",
                    "example": "ZOOM"
                  },
                  "active": {
                    "type": "boolean",
                    "description": "Only valid for s2s_oauth app, whether to activate automatically: \n* `true` &mdash; Automatically activate the app after creation. It is required that at least one scope should be included in the input parameter. \n* `false` &mdash; Do not activate the app after creation.",
                    "example": false,
                    "default": false
                  },
                  "publish": {
                    "type": "boolean",
                    "description": "Only valid for meeting_sdk app, whether to publish the app to the Marketplace: \n* `true` &mdash; The app contains development credentials and production credentials. \n* `false` &mdash; The app contains only development credentials.",
                    "example": false,
                    "default": false
                  },
                  "manifest": {
                    "type": "object",
                    "description": "An app manifest object. It should meet the valid manifest schema. If the app_type equals general, you need to pass this param.\n\nManifest resources: https://developers.zoom.us/docs/build-flow/manifests/"
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "**HTTP Status Code:** `201`   \n \nApp created successfully.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "created_at": {
                      "type": "string",
                      "description": "Date and time this app was created.",
                      "format": "date-time",
                      "example": "2023-02-16T17:32:28Z"
                    },
                    "app_id": {
                      "type": "string",
                      "description": "The app's id.",
                      "example": "Bwpq5zXvQr-4PKhtYOD23g"
                    },
                    "app_name": {
                      "type": "string",
                      "description": "The app's name.",
                      "example": "My App"
                    },
                    "app_type": {
                      "type": "string",
                      "description": "The app's type: \n* `s2s_oauth` &mdash; Server to Server OAuth app. \n* `meeting_sdk` &mdash; Meeting SDK app.",
                      "example": "s2s_oauth",
                      "enum": [
                        "s2s_oauth",
                        "meeting_sdk"
                      ]
                    },
                    "scopes": {
                      "type": "array",
                      "description": "The app's scopes.",
                      "items": {
                        "type": "string",
                        "example": "meeting:read"
                      }
                    },
                    "production_credentials": {
                      "type": "object",
                      "properties": {
                        "client_id": {
                          "type": "string",
                          "description": "The app's client id.",
                          "example": "2J5nVOGXQXCvXoRWyGQDow"
                        },
                        "client_secret": {
                          "type": "string",
                          "description": "The app's client secret.",
                          "example": "BzFOrwgq1Gw6KCsyeYnmETU5e4zakGwQ"
                        }
                      },
                      "description": "Only when the type is `meeting_sdk` and the `publish` field is true, there will be `production_credentials`"
                    },
                    "development_credentials": {
                      "type": "object",
                      "properties": {
                        "client_id": {
                          "type": "string",
                          "description": "The app's client id.",
                          "example": "2J5nVOGXQXCvXoRWyGQDow"
                        },
                        "client_secret": {
                          "type": "string",
                          "description": "The app's client secret.",
                          "example": "BzFOrwgq1Gw6KCsyeYnmETU5e4zakGwQ"
                        }
                      },
                      "description": "Only when the type is `meeting_sdk` and the `publish` field is true, there will be `production_credentials`"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `300` <br>\n * Validation Failed. <br>\n"
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n **Error Code:** `201` <br>\n * Invalid access token. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `1500` <br>\n * The request body is missing required fields. For more information, see the API documentation. \n* Only `Server to Server OAuth` and `Meeting SDK` apps can be created. \n* Can't get account owner info. \n* Only account's owner can use this API.\n* Can't get account info. <br>\n"
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
            "marketplace_app:master"
          ],
          "x-granular-scopes": [
            "marketplace:write:app:master"
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
    "title": "Marketplace",
    "description": "The Marketplace APIs allow developers to access data related to their app and its usage in the marketplace.",
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
    "/accounts/{accountId}/marketplace/apps": {
      "post": {
        "tags": [
          "App"
        ],
        "summary": "Create apps",
        "description": "Create `Server to Server OAuth`, `Meeting SDK` or `General` apps in the Zoom Marketplace. \n\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `marketplace_app:master`\n\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `marketplace:write:app:master`\n\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `HEAVY`",
        "operationId": "CreateApps",
        "parameters": [
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
        "requestBody": {
          "description": "/v2/marketplace/apps needs to pass the manifest parameter to create a General app.\n/v2/marketplace/apps/accounts/{accountld} needs to pass other parameters to create Server OAuth, Meeting SDK app.",
          "content": {
            "application/json": {
              "schema": {
                "required": [
                  "app_name",
                  "app_type",
                  "company_name",
                  "contact_email",
                  "contact_name"
                ],
                "type": "object",
                "properties": {
                  "app_type": {
                    "type": "string",
                    "description": "The app's type: \n* `s2s_oauth` &mdash; Server to Server OAuth app. \n* `meeting_sdk` &mdash; Meeting SDK app.\n* `general`&mdash; UBF General app.",
                    "example": "s2s_oauth",
                    "enum": [
                      "s2s_oauth",
                      "meeting_sdk",
                      "general"
                    ]
                  },
                  "app_name": {
                    "maxLength": 50,
                    "type": "string",
                    "description": "The app's name.",
                    "example": "My App"
                  },
                  "scopes": {
                    "type": "array",
                    "description": "The scopes of the created app:\n* `s2s_oauth` &mdash; If `active` field is true, there must be at least one scope. \n* `meeting_sdk` &mdash; Will automatically include `user_zak:read` scope.",
                    "items": {
                      "type": "string",
                      "example": "meeting:read"
                    }
                  },
                  "contact_name": {
                    "maxLength": 150,
                    "type": "string",
                    "description": "The developer's name.",
                    "example": "ZOOM"
                  },
                  "contact_email": {
                    "maxLength": 150,
                    "type": "string",
                    "description": "The developer's email.",
                    "format": "email",
                    "example": "example@example.com"
                  },
                  "company_name": {
                    "maxLength": 150,
                    "type": "string",
                    "description": "The company's name.",
                    "example": "ZOOM"
                  },
                  "active": {
                    "type": "boolean",
                    "description": "Only valid for s2s_oauth app, whether to activate automatically: \n* `true` &mdash; Automatically activate the app after creation. It is required that at least one scope should be included in the input parameter. \n* `false` &mdash; Do not activate the app after creation.",
                    "example": false,
                    "default": false
                  },
                  "publish": {
                    "type": "boolean",
                    "description": "Only valid for meeting_sdk app, whether to publish the app to the Marketplace: \n* `true` &mdash; The app contains development credentials and production credentials. \n* `false` &mdash; The app contains only development credentials.",
                    "example": false,
                    "default": false
                  },
                  "manifest": {
                    "type": "object",
                    "description": "An app manifest object. It should meet the valid manifest schema. If the app_type equals general, you need to pass this param.\n\nManifest resources: https://developers.zoom.us/docs/build-flow/manifests/"
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "**HTTP Status Code:** `201`   \n \nApp created successfully.",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "created_at": {
                      "type": "string",
                      "description": "Date and time this app was created.",
                      "format": "date-time",
                      "example": "2023-02-16T17:32:28Z"
                    },
                    "app_id": {
                      "type": "string",
                      "description": "The app's id.",
                      "example": "Bwpq5zXvQr-4PKhtYOD23g"
                    },
                    "app_name": {
                      "type": "string",
                      "description": "The app's name.",
                      "example": "My App"
                    },
                    "app_type": {
                      "type": "string",
                      "description": "The app's type: \n* `s2s_oauth` &mdash; Server to Server OAuth app. \n* `meeting_sdk` &mdash; Meeting SDK app.",
                      "example": "s2s_oauth",
                      "enum": [
                        "s2s_oauth",
                        "meeting_sdk"
                      ]
                    },
                    "scopes": {
                      "type": "array",
                      "description": "The app's scopes.",
                      "items": {
                        "type": "string",
                        "example": "meeting:read"
                      }
                    },
                    "production_credentials": {
                      "type": "object",
                      "properties": {
                        "client_id": {
                          "type": "string",
                          "description": "The app's client id.",
                          "example": "2J5nVOGXQXCvXoRWyGQDow"
                        },
                        "client_secret": {
                          "type": "string",
                          "description": "The app's client secret.",
                          "example": "BzFOrwgq1Gw6KCsyeYnmETU5e4zakGwQ"
                        }
                      },
                      "description": "Only when the type is `meeting_sdk` and the `publish` field is true, there will be `production_credentials`"
                    },
                    "development_credentials": {
                      "type": "object",
                      "properties": {
                        "client_id": {
                          "type": "string",
                          "description": "The app's client id.",
                          "example": "2J5nVOGXQXCvXoRWyGQDow"
                        },
                        "client_secret": {
                          "type": "string",
                          "description": "The app's client secret.",
                          "example": "BzFOrwgq1Gw6KCsyeYnmETU5e4zakGwQ"
                        }
                      },
                      "description": "Only when the type is `meeting_sdk` and the `publish` field is true, there will be `production_credentials`"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "**HTTP Status Code:** `400` <br>\n Bad Request  \n\n **Error Code:** `300` <br>\n * Validation Failed. <br>\n"
          },
          "401": {
            "description": "**HTTP Status Code:** `401` <br>\n Unauthorized  \n\n **Error Code:** `201` <br>\n * Invalid access token. <br>\n"
          },
          "404": {
            "description": "**HTTP Status Code:** `404` <br>\n Not Found  \n\n **Error Code:** `1500` <br>\n * The request body is missing required fields. For more information, see the API documentation. \n* Only `Server to Server OAuth` and `Meeting SDK` apps can be created. \n* Can't get account owner info. \n* Only account's owner can use this API.\n* Can't get account info. <br>\n"
          },
          "429": {
            "description": "**HTTP Status Code:** `429` <br>\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \n\n "
          }
        },
        "security": [
          {
            "openapi_oauth": [
              "marketplace_app:master",
              "marketplace:write:app:master"
            ],
            "openapi_authorization": []
          }
        ],
        "x-extensions": {
          "x-permissions": [],
          "x-macro-scopes": [
            "marketplace_app:master"
          ],
          "x-granular-scopes": [
            "marketplace:write:app:master"
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
              "marketplace_app:master": "marketplace_app:master",
              "marketplace:write:app:master": "marketplace:write:app:master"
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
      "ma"
    ]
  },
  "downloadPath": "/api-hub/marketplace/ma/master.json",
  "apiOptions": {
    "events": true,
    "methods": true,
    "ma": true
  },
  "productMeta": {
    "specType": "oas",
    "slug": "/docs/api/rest/reference/marketplace/ma",
    "title": "Zoom Marketplace Master API",
    "description": "",
    "keywords": "zoom, api, marketplace, master account",
    "params": {
      "productName": "Marketplace",
      "maFlag": true
    },
    "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
    "zoapEndpoint": "docJsonByFixVersion",
    "productDisplayTitle": "Marketplace",
    "scalarSlug": "/docs/api/marketplace/ma",
    "svgname": "marketplace"
  },
  "frontmatter": {
    "title": "Marketplace Master APIs",
    "keywords": "zoom, api, marketplace, master account",
    "description": "The Marketplace APIs allow developers to access data related to their app and its usage in the marketplace.",
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
        "ma"
      ]
    },
    "downloadPath": "/api-hub/marketplace/ma/master.json",
    "apiOptions": {
      "events": true,
      "methods": true,
      "ma": true
    },
    "productMeta": {
      "specType": "oas",
      "slug": "/docs/api/rest/reference/marketplace/ma",
      "title": "Zoom Marketplace Master API",
      "description": "",
      "keywords": "zoom, api, marketplace, master account",
      "params": {
        "productName": "Marketplace",
        "maFlag": true
      },
      "alertPath": "./content/docs/_includes/api/new-api-docs-alert.mdx",
      "zoapEndpoint": "docJsonByFixVersion",
      "productDisplayTitle": "Marketplace",
      "scalarSlug": "/docs/api/marketplace/ma",
      "svgname": "marketplace"
    },
    "spec": "{\"openapi\":\"3.0.0\",\"info\":{\"title\":\"Marketplace\",\"description\":\"The Marketplace APIs allow developers to access data related to their app and its usage in the marketplace.\",\"termsOfService\":\"https://zoom.us/docs/en-us/zoom_api_license_and_tou.html\",\"contact\":{\"name\":\"Zoom Developers\",\"url\":\"https://developer.zoom.us/\"},\"version\":\"2\"},\"externalDocs\":{\"description\":\"Find out more about Swagger\",\"url\":\"https://swagger.io\"},\"servers\":[{\"url\":\"https://api.zoom.us/v2\"}],\"paths\":{\"/accounts/{accountId}/marketplace/apps\":{\"post\":{\"tags\":[\"App\"],\"summary\":\"Create apps\",\"description\":\"Create `Server to Server OAuth`, `Meeting SDK` or `General` apps in the Zoom Marketplace. \\n\\n**[Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `marketplace_app:master`\\n\\n**[Granular Scopes](https://developers.zoom.us/docs/integrations/oauth-scopes-overview/):** `marketplace:write:app:master`\\n\\n**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `HEAVY`\",\"operationId\":\"CreateApps\",\"parameters\":[{\"name\":\"accountId\",\"in\":\"path\",\"description\":\"Unique identifier of the account.\",\"required\":true,\"schema\":{\"type\":\"string\",\"example\":\"TQTvjT52Tmi_wrhASNNOEw\"}}],\"requestBody\":{\"description\":\"/v2/marketplace/apps needs to pass the manifest parameter to create a General app.\\n/v2/marketplace/apps/accounts/{accountld} needs to pass other parameters to create Server OAuth, Meeting SDK app.\",\"content\":{\"application/json\":{\"schema\":{\"required\":[\"app_name\",\"app_type\",\"company_name\",\"contact_email\",\"contact_name\"],\"type\":\"object\",\"properties\":{\"app_type\":{\"type\":\"string\",\"description\":\"The app's type: \\n* `s2s_oauth` &mdash; Server to Server OAuth app. \\n* `meeting_sdk` &mdash; Meeting SDK app.\\n* `general`&mdash; UBF General app.\",\"example\":\"s2s_oauth\",\"enum\":[\"s2s_oauth\",\"meeting_sdk\",\"general\"]},\"app_name\":{\"maxLength\":50,\"type\":\"string\",\"description\":\"The app's name.\",\"example\":\"My App\"},\"scopes\":{\"type\":\"array\",\"description\":\"The scopes of the created app:\\n* `s2s_oauth` &mdash; If `active` field is true, there must be at least one scope. \\n* `meeting_sdk` &mdash; Will automatically include `user_zak:read` scope.\",\"items\":{\"type\":\"string\",\"example\":\"meeting:read\"}},\"contact_name\":{\"maxLength\":150,\"type\":\"string\",\"description\":\"The developer's name.\",\"example\":\"ZOOM\"},\"contact_email\":{\"maxLength\":150,\"type\":\"string\",\"description\":\"The developer's email.\",\"format\":\"email\",\"example\":\"example@example.com\"},\"company_name\":{\"maxLength\":150,\"type\":\"string\",\"description\":\"The company's name.\",\"example\":\"ZOOM\"},\"active\":{\"type\":\"boolean\",\"description\":\"Only valid for s2s_oauth app, whether to activate automatically: \\n* `true` &mdash; Automatically activate the app after creation. It is required that at least one scope should be included in the input parameter. \\n* `false` &mdash; Do not activate the app after creation.\",\"example\":false,\"default\":false},\"publish\":{\"type\":\"boolean\",\"description\":\"Only valid for meeting_sdk app, whether to publish the app to the Marketplace: \\n* `true` &mdash; The app contains development credentials and production credentials. \\n* `false` &mdash; The app contains only development credentials.\",\"example\":false,\"default\":false},\"manifest\":{\"type\":\"object\",\"description\":\"An app manifest object. It should meet the valid manifest schema. If the app_type equals general, you need to pass this param.\\n\\nManifest resources: https://developers.zoom.us/docs/build-flow/manifests/\"}}}}}},\"responses\":{\"201\":{\"description\":\"**HTTP Status Code:** `201`   \\n \\nApp created successfully.\",\"content\":{\"application/json\":{\"schema\":{\"type\":\"object\",\"properties\":{\"created_at\":{\"type\":\"string\",\"description\":\"Date and time this app was created.\",\"format\":\"date-time\",\"example\":\"2023-02-16T17:32:28Z\"},\"app_id\":{\"type\":\"string\",\"description\":\"The app's id.\",\"example\":\"Bwpq5zXvQr-4PKhtYOD23g\"},\"app_name\":{\"type\":\"string\",\"description\":\"The app's name.\",\"example\":\"My App\"},\"app_type\":{\"type\":\"string\",\"description\":\"The app's type: \\n* `s2s_oauth` &mdash; Server to Server OAuth app. \\n* `meeting_sdk` &mdash; Meeting SDK app.\",\"example\":\"s2s_oauth\",\"enum\":[\"s2s_oauth\",\"meeting_sdk\"]},\"scopes\":{\"type\":\"array\",\"description\":\"The app's scopes.\",\"items\":{\"type\":\"string\",\"example\":\"meeting:read\"}},\"production_credentials\":{\"type\":\"object\",\"properties\":{\"client_id\":{\"type\":\"string\",\"description\":\"The app's client id.\",\"example\":\"2J5nVOGXQXCvXoRWyGQDow\"},\"client_secret\":{\"type\":\"string\",\"description\":\"The app's client secret.\",\"example\":\"BzFOrwgq1Gw6KCsyeYnmETU5e4zakGwQ\"}},\"description\":\"Only when the type is `meeting_sdk` and the `publish` field is true, there will be `production_credentials`\"},\"development_credentials\":{\"type\":\"object\",\"properties\":{\"client_id\":{\"type\":\"string\",\"description\":\"The app's client id.\",\"example\":\"2J5nVOGXQXCvXoRWyGQDow\"},\"client_secret\":{\"type\":\"string\",\"description\":\"The app's client secret.\",\"example\":\"BzFOrwgq1Gw6KCsyeYnmETU5e4zakGwQ\"}},\"description\":\"Only when the type is `meeting_sdk` and the `publish` field is true, there will be `production_credentials`\"}}}}}},\"400\":{\"description\":\"**HTTP Status Code:** `400` <br>\\n Bad Request  \\n\\n **Error Code:** `300` <br>\\n * Validation Failed. <br>\\n\"},\"401\":{\"description\":\"**HTTP Status Code:** `401` <br>\\n Unauthorized  \\n\\n **Error Code:** `201` <br>\\n * Invalid access token. <br>\\n\"},\"404\":{\"description\":\"**HTTP Status Code:** `404` <br>\\n Not Found  \\n\\n **Error Code:** `1500` <br>\\n * The request body is missing required fields. For more information, see the API documentation. \\n* Only `Server to Server OAuth` and `Meeting SDK` apps can be created. \\n* Can't get account owner info. \\n* Only account's owner can use this API.\\n* Can't get account info. <br>\\n\"},\"429\":{\"description\":\"**HTTP Status Code:** `429` <br>\\n Too Many Requests. For more information, see [rate limits](https://developers.zoom.us/docs/api/rest/rate-limits/). \\n\\n \"}},\"security\":[{\"openapi_authorization\":[]}],\"x-extensions\":{\"x-permissions\":[],\"x-macro-scopes\":[\"marketplace_app:master\"],\"x-granular-scopes\":[\"marketplace:write:app:master\"]}}}},\"components\":{\"securitySchemes\":{\"openapi_authorization\":{\"type\":\"http\",\"scheme\":\"bearer\"}}}}",
    "frontmatter": {
      "title": "Marketplace Master APIs",
      "keywords": "zoom, api, marketplace, master account",
      "description": "The Marketplace APIs allow developers to access data related to their app and its usage in the marketplace.",
      "skipQualtrics": true
    },
    "isScalarApiRef": true
  },
  "__N_SSG": true
}
```
