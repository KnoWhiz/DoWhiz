# Batching Requests

## Overview

The Gmail API supports batching to reduce HTTP connection overhead. Instead of sending multiple API calls separately, you can group them into a single HTTP request to improve efficiency.

**Key use cases:**
- Initial data upload when starting API integration
- Synchronizing local data after offline disconnection

### Limitations

- Maximum of 100 calls per batch request
- Use multiple batch requests if you need more calls
- Batches larger than 50 requests may trigger rate limiting (not recommended)

## Batch Request Details

### Format

A batch request uses `multipart/mixed` content type containing multiple Gmail API calls. Each part includes:

- `Content-Type: application/http` header
- Optional `Content-ID` header
- Complete nested HTTP request (path only, no full URLs)

**Header behavior:** Outer request headers apply to all calls unless individual calls override them.

### Response Format

The server returns a `multipart/mixed` response with parts corresponding to requests in the same order. Each response part contains:

- Complete HTTP response (status code, headers, body)
- `Content-ID` header prefixed with `response-` if the request included one

**Important:** The server might perform your calls in any order. Don't count on their being executed in the order in which you specified them.

## Example Request Structure

```
POST /batch/farm/v1 HTTP/1.1
Authorization: Bearer your_auth_token
Host: www.googleapis.com
Content-Type: multipart/mixed; boundary=batch_foobarbaz
Content-Length: total_content_length

--batch_foobarbaz
Content-Type: application/http
Content-ID: <item1:12930812@barnyard.example.com>

GET /farm/v1/animals/pony

--batch_foobarbaz
Content-Type: application/http
Content-ID: <item2:12930812@barnyard.example.com>

PUT /farm/v1/animals/sheep
Content-Type: application/json
Content-Length: part_content_length
If-Match: "etag/sheep"

{
  "animalName": "sheep",
  "animalAge": "5"
  "peltColor": "green",
}

--batch_foobarbaz
Content-Type: application/http
Content-ID: <item3:12930812@barnyard.example.com>

GET /farm/v1/animals
If-None-Match: "etag/animals"

--batch_foobarbaz--
```

## Example Response Structure

```
HTTP/1.1 200
Content-Length: response_total_content_length
Content-Type: multipart/mixed; boundary=batch_foobarbaz

--batch_foobarbaz
Content-Type: application/http
Content-ID: <response-item1:12930812@barnyard.example.com>

HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: response_part_1_content_length
ETag: "etag/pony"

{
  "kind": "farm#animal",
  "etag": "etag/pony",
  "selfLink": "/farm/v1/animals/pony",
  "animalName": "pony",
  "animalAge": 34,
  "peltColor": "white"
}

--batch_foobarbaz
Content-Type: application/http
Content-ID: <response-item2:12930812@barnyard.example.com>

HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: response_part_2_content_length
ETag: "etag/sheep"

{
  "kind": "farm#animal",
  "etag": "etag/sheep",
  "selfLink": "/farm/v1/animals/sheep",
  "animalName": "sheep",
  "animalAge": 5,
  "peltColor": "green"
}

--batch_foobarbaz
Content-Type: application/http
Content-ID: <response-item3:12930812@barnyard.example.com>

HTTP/1.1 304 Not Modified
ETag: "etag/animals"

--batch_foobarbaz--
```

## Important Notes

- A set of n requests batched together counts toward your usage limit as n requests, not as one request
- Syntax follows OData batch processing but with different semantics
- Client libraries may provide simplified batch request methods
