# Gmail API Performance Tips

## Overview

This guide presents techniques to enhance application performance when using the Gmail API. The document emphasizes bandwidth reduction and efficient resource utilization through compression and partial data requests.

## Compression using gzip

To reduce bandwidth consumption, enable gzip compression. While this requires additional CPU processing, the trade-off with network costs usually makes it very worthwhile.

### Implementation Requirements

Two steps are necessary:
1. Set an `Accept-Encoding` header
2. Modify your user agent to contain the string `gzip`

**Example headers:**
```
Accept-Encoding: gzip
User-Agent: my program (gzip)
```

## Working with Partial Resources

Improve performance by sending and receiving only the portion of the data that you're interested in. This reduces transfers of unneeded fields and conserves network, CPU, and memory resources.

Two partial request types exist:

### Partial Response

Use the `fields` request parameter to specify which fields to include in responses.

**Example request (full resource):**
```
https://www.googleapis.com/demo/v1
```

**Example response (full resource):**
```
{
  "kind": "demo",
  ...
  "items": [
  {
    "title": "First title",
    "comment": "First comment.",
    "characteristics": {
      "length": "short",
      "accuracy": "high",
      "followers": ["Jo", "Will"],
    },
    "status": "active",
    ...
  },
  {
    "title": "Second title",
    "comment": "Second comment.",
    "characteristics": {
      "length": "long",
      "accuracy": "medium"
      "followers": [ ],
    },
    "status": "pending",
    ...
  },
  ...
  ]
}
```

**Example partial request:**
```
https://www.googleapis.com/demo/v1?fields=kind,items(title,characteristics/length)
```

**Example partial response status:**
```
200 OK
```

**Example partial response body:**
```
{
  "kind": "demo",
  "items": [{
    "title": "First title",
    "characteristics": {
      "length": "short"
    }
  }, {
    "title": "Second title",
    "characteristics": {
      "length": "long"
    }
  },
  ...
  ]
}
```

**Fields Parameter Syntax:**
- Comma-separated list for multiple fields
- `a/b` notation for nested fields
- Parentheses `()` for sub-selections
- Wildcards supported (e.g., `items/pagemap/*`)

**Collection-level examples:**

| Syntax | Result |
|--------|--------|
| `items` | All array elements with all fields |
| `etag,items` | Both etag field and items array |
| `items/title` | Only title field for items array |
| `context/facets/label` | Label field for facets members under context |

**Resource-level examples:**

| Syntax | Result |
|--------|--------|
| `title` | Title field only |
| `author/uri` | URI sub-field of author object |
| `links/*/href` | Href field of all objects under links |

#### Handling partial responses

**Example request:**
```
https://www.googleapis.com/demo/v1?fields=kind,items(title,characteristics/length)
```

**Example response status:**
```
200 OK
```

**Example response body:**
```
{
  "kind": "demo",
  "items": [{
    "title": "First title",
    "characteristics": {
      "length": "short"
    }
  }, {
    "title": "Second title",
    "characteristics": {
      "length": "long"
    }
  },
  ...
  ]
}
```

### Patch (Partial Update)

Use the HTTP `PATCH` verb to send updated data only for fields you're changing, minimizing request payload size.

**Example patch request:**
```
PATCH https://www.googleapis.com/demo/v1/324
Authorization: Bearer your_auth_token
Content-Type: application/json

{
  "title": "New title"
}
```

**Example response status:**
```
200 OK
```

**Example response body:**
```
{
  "title": "New title",
  "comment": "First comment.",
  "characteristics": {
    "length": "short",
    "accuracy": "high",
    "followers": ["Jo", "Will"],
  },
  "status": "active",
  ...
}
```

#### Patch Semantics

- **Add:** Specify new field and its value
- **Modify:** Set field to new value
- **Delete:** Set field to `null`

**Note on arrays:** Patch requests replace entire arrays; piecemeal modifications aren't supported.

#### Read-Modify-Write Cycle

Retrieve partial data first (especially for resources using ETags), modify values, then send back the updated representation:

```
GET https://www.googleapis.com/demo/v1/324?fields=etag,title,comment,characteristics
Authorization: Bearer your_auth_token
```

**Example response status:**
```
200 OK
```

**Example response body:**
```
{
  "etag": "ETagString"
  "title": "New title"
  "comment": "First comment.",
  "characteristics": {
    "length": "short",
    "level": "5",
    "followers": ["Jo", "Will"],
  }
}
```

Then patch with ETag validation:
```
PATCH https://www.googleapis.com/demo/v1/324?fields=etag,title,comment,characteristics
Authorization: Bearer your_auth_token
Content-Type: application/json
If-Match: "ETagString"
```

**Example patch body:**
```
{
  "etag": "ETagString"
  "title": "",                  /* Clear the value of the title by setting it to the empty string. */
  "comment": null,              /* Delete the comment by replacing its value with null. */
  "characteristics": {
    "length": "short",
    "level": "10",              /* Modify the level value. */
    "followers": ["Jo", "Liz"], /* Replace the followers array to delete Will and add Liz. */
    "accuracy": "high"          /* Add a new characteristic. */
  },
}
```

**Example response status:**
```
200 OK
```

**Example response body:**
```
{
  "etag": "newETagString"
  "title": "",                 /* Title is cleared; deleted comment field is missing. */
  "characteristics": {
    "length": "short",
    "level": "10",             /* Value is updated.*/
    "followers": ["Jo" "Liz"], /* New follower Liz is present; deleted Will is missing. */
    "accuracy": "high"         /* New characteristic is present. */
  }
}
```

#### Direct Patch Construction

For simple updates, construct patches without prior retrieval:

```
PATCH https://www.googleapis.com/demo/v1/324?fields=comment,characteristics
Authorization: Bearer your_auth_token
Content-Type: application/json

{
  "comment": "A new comment",
  "characteristics": {
    "volume": "loud",
    "accuracy": null
  }
}
```

#### Handling Patch Responses

Valid patch requests return `200 OK` with complete resource representation. Invalid requests return `400 Bad Request` or `422 Unprocessable Entity`, leaving resources unchanged.

#### PATCH Workaround

If firewalls block PATCH requests, use POST with override header:
```
POST https://www.googleapis.com/...
X-HTTP-Method-Override: PATCH
...
```

#### Patch vs. PUT

PATCH is safer than PUT because you only supply data for the fields you want to change; fields that you omit are not cleared. PUT requests fail if required parameters are missing and clear optional fields not supplied.
