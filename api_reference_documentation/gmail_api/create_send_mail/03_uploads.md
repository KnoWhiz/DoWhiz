# Uploading Attachments

## Overview

The Gmail API enables file uploads when creating/updating drafts or inserting/sending messages through specific endpoints.

## Upload Options

The API supports three upload methods, specified via the `uploadType` parameter:

### 1. Simple Upload

- **Use case**: Small files (5 MB or less), no metadata needed
- **Method**: POST/PUT to `/upload` URI with `uploadType=media`
- **Headers**: `Content-Type` and `Content-Length`

**Example endpoint:**
```
POST https://www.googleapis.com/upload/gmail/v1/users/userId/messages/send?uploadType=media
```

**Example request:**
```
POST /upload/gmail/v1/users/userId/messages/send?uploadType=media HTTP/1.1
Host: www.googleapis.com
Content-Type: message/rfc822
Content-Length: number_of_bytes_in_file
Authorization: Bearer your_auth_token

Email Message data
```

**Example response:**
```
HTTP/1.1 200
Content-Type: application/json

{
  "id": string,
  "threadId": string,
  "labelIds": [
    string
  ],
  "snippet": string,
  "historyId": unsigned long,
  "payload": {
    "partId": string,
    "mimeType": string,
    "filename": string,
    "headers": [
      {
        "name": string,
        "value": string
      }
    ],
    "body": users.messages.attachments Resource,
    "parts": [
      (MessagePart)
    ]
  },
  "sizeEstimate": integer,
  "raw": bytes
}
```

### 2. Multipart Upload

- **Use case**: Small files with metadata in a single request
- **Method**: POST/PUT to `/upload` URI with `uploadType=multipart`
- **Format**: `multipart/related` content type with two parts (metadata first, media second)

**Example endpoint:**
```
POST https://www.googleapis.com/upload/gmail/v1/users/userId/messages/send?uploadType=multipart
```

**Example request:**
```
POST /upload/gmail/v1/users/userId/messages/send?uploadType=multipart HTTP/1.1
Host: www.googleapis.com
Authorization: Bearer your_auth_token
Content-Type: multipart/related; boundary=foo_bar_baz
Content-Length: number_of_bytes_in_entire_request_body

--foo_bar_baz
Content-Type: application/json; charset=UTF-8

{
  "id": string,
  "threadId": string,
  "labelIds": [
    string
  ],
  "snippet": string,
  "historyId": unsigned long,
  "payload": {
    "partId": string,
    "mimeType": string,
    "filename": string,
    "headers": [
      {
        "name": string,
        "value": string
      }
    ],
    "body": users.messages.attachments Resource,
    "parts": [
      (MessagePart)
    ]
  },
  "sizeEstimate": integer,
  "raw": bytes
}

--foo_bar_baz
Content-Type: message/rfc822

Email Message data
--foo_bar_baz--
```

**Example response:**
```
HTTP/1.1 200
Content-Type: application/json

{
  "id": string,
  "threadId": string,
  "labelIds": [
    string
  ],
  "snippet": string,
  "historyId": unsigned long,
  "payload": {
    "partId": string,
    "mimeType": string,
    "filename": string,
    "headers": [
      {
        "name": string,
        "value": string
      }
    ],
    "body": users.messages.attachments Resource,
    "parts": [
      (MessagePart)
    ]
  },
  "sizeEstimate": integer,
  "raw": bytes
}
```

### 3. Resumable Upload

- **Use case**: Reliable transfer, especially for larger files and network interruptions
- **Process**: Three-step procedure (initiate, save URI, upload file)
- **Method**: POST/PUT to `/upload` URI with `uploadType=resumable`

**Example endpoint:**
```
POST https://www.googleapis.com/upload/gmail/v1/users/userId/messages/send?uploadType=resumable
```

## Resumable Upload Process

### Step 1: Initiate Session

```
POST /upload/gmail/v1/users/userId/messages/send?uploadType=resumable HTTP/1.1
Host: www.googleapis.com
Authorization: Bearer your_auth_token
Content-Length: 38
Content-Type: application/json; charset=UTF-8
X-Upload-Content-Type: message/rfc822
X-Upload-Content-Length: 2000000

{
  "id": string,
  "threadId": string,
  "labelIds": [
    string
  ],
  "snippet": string,
  "historyId": unsigned long,
  "payload": {
    "partId": string,
    "mimeType": string,
    "filename": string,
    "headers": [
      {
        "name": string,
        "value": string
      }
    ],
    "body": users.messages.attachments Resource,
    "parts": [
      (MessagePart)
    ]
  },
  "sizeEstimate": integer,
  "raw": bytes
}
```

**Required headers:**
- `X-Upload-Content-Type`: Media MIME type
- `X-Upload-Content-Length`: File size in bytes
- `Content-Type`: Metadata format (if providing metadata)

### Step 2: Save Session URI

Server responds with `200 OK` and provides a `Location` header containing the session URI with an `upload_id` parameter.

**Example response:**
```
HTTP/1.1 200 OK
Location: https://www.googleapis.com/upload/gmail/v1/users/userId/messages/send?uploadType=resumable&upload_id=xa298sd_sdlkj2
Content-Length: 0
```

### Step 3: Upload File

```
PUT session_uri
```

```
PUT https://www.googleapis.com/upload/gmail/v1/users/userId/messages/send?uploadType=resumable&upload_id=xa298sd_sdlkj2 HTTP/1.1
Content-Length: 2000000
Content-Type: message/rfc822

bytes 0-1999999
```

## Chunked Uploads

For resumable uploads, files can be split into chunks:
- **Chunk size**: Must be multiple of 256 KB (except final chunk)
- **Header**: Include `Content-Range: bytes 0-524287/2000000`
- **Response**: Server responds with `308 Resume Incomplete` and `Range` header

**Example chunk upload request:**
```
PUT {session_uri} HTTP/1.1
Host: www.googleapis.com
Content-Length: 524288
Content-Type: message/rfc822
Content-Range: bytes 0-524287/2000000

bytes 0-524288
```

**Example response:**
```
HTTP/1.1 308 Resume Incomplete
Content-Length: 0
Range: bytes=0-524287
```

## Resuming Interrupted Uploads

**Step 1**: Query status with empty PUT request
```
PUT {session_uri} HTTP/1.1
Content-Length: 0
Content-Range: bytes */2000000
```

**Example response:**
```
HTTP/1.1 308 Resume Incomplete
Content-Length: 0
Range: 0-42
```

**Step 2**: Extract bytes received from response `Range` header

**Step 3**: Resume upload from that point with remaining data
```
PUT {session_uri} HTTP/1.1
Content-Length: 1999957
Content-Range: bytes 43-1999999/2000000

bytes 43-1999999
```

## Best Practices

- Resume uploads failing due to connection issues or 5xx errors
- Use exponential backoff for 5xx server errors
- Retry other failures with a limit (e.g., 10 retries max)
- Restart entirely on `404 Not Found` or `410 Gone` errors

### Exponential Backoff Strategy

Wait periods follow: (2^n) + random milliseconds where n starts at 0 and increments each retry. Example sequence: 1s, 2s, 4s, 8s, 16s delays. Maximum recommended n=5 (~32 second total delay).

## URI Endpoints

Two endpoint types:
- **Upload URI** (`/upload` prefix): For media data transfer
- **Standard URI**: For metadata operations only

## Supported Media Types & Limits

Consult the API reference for:
- Maximum upload file size per method
- Accepted MIME types (e.g., `message/rfc822` for email)

## API Client Libraries

Support available for: .NET, Java, PHP, Python, Ruby
