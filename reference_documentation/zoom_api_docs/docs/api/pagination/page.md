# Pagination

- Source URL: https://developers.zoom.us/docs/api/pagination/
- Snapshot path: docs/api/pagination
- Generated (UTC): 2026-02-07T22:40:11.110970+00:00

## Frontmatter

```json
{
  "title": "Pagination"
}
```

## Content

```md

# Pagination

Zoom APIs use pagination to organize large response datasets into manageable chunks. This approach helps you efficiently handle substantial data payloads when making `GET` requests that list large numbers of resources.

## Response limits

The number of items returned per page varies based on the API and data retrieval level:

- Light data retrieval: Up to 300 items per page
- Heavy data retrieval: Up to 30, 50, or 100 items per page, depending on the specific API

For details on light and heavy APIs, see the [Rate limits](/docs/api/rate-limits) documentation.

## Using `next_page_token`

Most Zoom APIs, including [List Meeting participants](/docs/api/accounts/#tag/dashboards/GET/metrics/meetings/{meetingId}/participants)
and [List Webinars](/docs/api/accounts/#tag/dashboards/GET/metrics/webinars/{webinarId}/participants), use the `next_page_token` parameter for pagination.

- If the initial API query returns a complete response, the `next_page_token` value will be an empty string.
- A non-empty `next_page_token` value (e.g., `"abc3445rg"`) indicates that additional results are available.

To retrieve additional responses, include the `next_page_token` value in subsequent API requests using the same query parameters.

A request to the List Meetings API can return a response with a `next_page_token` similar to:

```json
{
  "from": "2024-02-07",
  "meetings": [
    {
      "duration": "05:27",
      "email": "jchill@example.com",
      "end_time": "2024-02-15T23:30:19Z",
      "has_recording": false,
      "id": 123456,
      "participants": 2,
      "start_time": "2024-02-15T23:24:52Z",
      "topic": "My Meeting",
      "user_type": "Pro|Webinar1000",
      "uuid": "abcd1234=="
    }
  ],
  "next_page_token": "def5678",
  "page_count": 5,
  "page_size": 1,
  "to": "2024-03-07",
  "total_records": 5
}
```


### Example: [List all users](/docs/api/users/#tag/users/GET/users) with `next_page_token`:

<Tabs>
<Tab eventKey="nodejs" title="Node.js">
```js
const axios = require('axios');

async function getAllUsers(baseUrl, accessToken) {
let allUsers = [];
let nextPageToken = '';

    while (true) {
        const headers = {
            'Authorization': `Bearer ${accessToken}`,
            'Content-Type': 'application/json'
        };

        const params = { page_size: 300 };
        if (nextPageToken) {
            params.next_page_token = nextPageToken;
        }

        try {
            const response = await axios.get(baseUrl, { headers, params });
            const data = response.data;

            allUsers = allUsers.concat(data.users || []);
            nextPageToken = data.next_page_token || '';

            if (!nextPageToken) {
                break;
            }
        } catch (error) {
            console.error('Error fetching users:', error.message);
            break;
        }
    }

    return allUsers;

}

// Usage
const baseUrl = 'https://api.zoom.us/v2/users';
const accessToken = 'YOUR_ACCESS_TOKEN';

getAllUsers(baseUrl, accessToken)
    .then(users => console.log(`Total users retrieved: ${users.length}`))
    .catch(error => console.error('Error:', error.message));
```
</Tab>
<Tab eventKey="python" title="Python">
```python
import requests

def get_all_users(base_url, access_token):
    all_users = []
    next_page_token = ''

    while True:
        headers = {
            "Authorization": f"Bearer {access_token}",
            "Content-Type": "application/json"
        }

        params = {"page_size": 300}
        if next_page_token:
            params["next_page_token"] = next_page_token

        response = requests.get(base_url, headers=headers, params=params)
        data = response.json()

        all_users.extend(data.get('users', []))

        next_page_token = data.get('next_page_token', '')
        if not next_page_token:
            break

    return all_users

# Usage
base_url = "https://api.zoom.us/v2/users"
access_token = "YOUR_ACCESS_TOKEN"
users = get_all_users(base_url, access_token)
print(f"Total users retrieved: {len(users)}")
```

</Tab>
</Tabs>

## Legacy pagination: page numbers

Some Zoom APIs, like [List Users](/docs/api/users/#tag/users/GET/users), support pagination using `page_size` and `page_number` parameters. However, this method is being phased out in favor of `next_page_token`.

- `page_size`: Number of records to return per page
- `page_number`: Specific page to retrieve

We recommend setting these values based on your needs, rather than relying on default values.

For example, a request to [List Meetings](/docs/api/meetings/#tag/meetings/GET/users/{userId}/meetings) with `page_number` and `page_size` parameters can return a response similar to:

```json
{
  "meetings": [
    {
      "agenda": "Weekly standup",
      "created_at": "2024-03-14T21:14:55Z",
      "duration": 60,
      "id": 987654,
      "join_url": "https://zoom.us/j/987654",
      "start_time": "2024-03-15T22:00:00Z",
      "topic": "Team Meeting",
      "type": 2
    }
  ],
  "page_count": 1,
  "page_number": 1,
  "page_size": 1,
  "total_records": 2
}
```

### Example: List users with page numbers

<Tabs>
<Tab eventKey="nodejs" title="Node.js">
```js
const axios = require('axios');

async function getAllUsers(baseUrl, accessToken) {
    let allUsers = [];
    let pageNumber = 1;
    const pageSize = 300;

    while (true) {
        const headers = {
            'Authorization': `Bearer ${accessToken}`,
            'Content-Type': 'application/json'
        };

        const params = {
            page_size: pageSize,
            page_number: pageNumber
        };

        try {
            const response = await axios.get(baseUrl, { headers, params });
            const data = response.data;

            allUsers = allUsers.concat(data.users || []);
            
            if (pageNumber >= data.page_count) {
                break;
            }
            
            pageNumber++;
        } catch (error) {
            console.error('Error fetching users:', error.message);
            break;
        }
    }

    return allUsers;
}

// Usage
const baseUrl = 'https://api.zoom.us/v2/users';
const accessToken = 'YOUR_ACCESS_TOKEN';

getAllUsers(baseUrl, accessToken)
    .then(users => console.log(`Total users retrieved: ${users.length}`))
    .catch(error => console.error('Error:', error.message));
```

</Tab>
<Tab eventKey="python" title="Python">
```python
import requests

def get_all_users(base_url, access_token):
    all_users = []
    page_number = 1
    page_size = 300
    
    while True:
        headers = {
            "Authorization": f"Bearer {access_token}",
            "Content-Type": "application/json"
        }
        
        params = {
            "page_size": page_size,
            "page_number": page_number
        }
        
        response = requests.get(base_url, headers=headers, params=params)
        data = response.json()
        
        all_users.extend(data.get('users', []))
        
        if page_number >= data.get('page_count', 0):
            break
        
        page_number += 1
    
    return all_users

# Usage
base_url = "https://api.zoom.us/v2/users"
access_token = "YOUR_ACCESS_TOKEN"
users = get_all_users(base_url, access_token)
print(f"Total users retrieved: {len(users)}")
```
</Tab>
</Tabs>
```

## OpenAPI

No OpenAPI spec found for this page.

## Downloads

No download files found.

## Metadata

```json
{
  "sidebarName": "api"
}
```

## Raw Next.js page data

```json
{
  "pageProps": {
    "mainContent": {
      "code": "var Component=(()=>{var g=Object.create;var i=Object.defineProperty;var _=Object.getOwnPropertyDescriptor;var m=Object.getOwnPropertyNames;var b=Object.getPrototypeOf,x=Object.prototype.hasOwnProperty;var f=(s,e)=>()=>(e||s((e={exports:{}}).exports,e),e.exports),k=(s,e)=>{for(var a in e)i(s,a,{get:e[a],enumerable:!0})},l=(s,e,a,r)=>{if(e&&typeof e==\"object\"||typeof e==\"function\")for(let t of m(e))!x.call(s,t)&&t!==a&&i(s,t,{get:()=>e[t],enumerable:!(r=_(e,t))||r.enumerable});return s};var T=(s,e,a)=>(a=s!=null?g(b(s)):{},l(e||!s||!s.__esModule?i(a,\"default\",{value:s,enumerable:!0}):a,s)),y=s=>l(i({},\"__esModule\",{value:!0}),s);var d=f((z,c)=>{c.exports=_jsx_runtime});var v={};k(v,{default:()=>h,frontmatter:()=>U});var n=T(d());var{useMDXComponents:o}=MdxJsReact;var U={title:\"Pagination\"};function p(s){let e={a:\"a\",code:\"code\",h1:\"h1\",h2:\"h2\",h3:\"h3\",i:\"i\",li:\"li\",p:\"p\",pre:\"pre\",ul:\"ul\",...o(),...s.components},{Tab:a,Tabs:r}=e;return a||u(\"Tab\",!0),r||u(\"Tabs\",!0),(0,n.jsxs)(n.Fragment,{children:[(0,n.jsx)(e.h1,{id:\"pagination\",children:\"Pagination\"}),`\n`,(0,n.jsxs)(e.p,{children:[\"Zoom APIs use pagination to organize large response datasets into manageable chunks. This approach helps you efficiently handle substantial data payloads when making \",(0,n.jsx)(e.code,{children:\"GET\"}),\" requests that list large numbers of resources.\"]}),`\n`,(0,n.jsx)(e.h2,{id:\"response-limits\",children:(0,n.jsxs)(e.a,{href:\"#response-limits\",children:[\"Response limits\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsx)(e.p,{children:\"The number of items returned per page varies based on the API and data retrieval level:\"}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsx)(e.li,{children:\"Light data retrieval: Up to 300 items per page\"}),`\n`,(0,n.jsx)(e.li,{children:\"Heavy data retrieval: Up to 30, 50, or 100 items per page, depending on the specific API\"}),`\n`]}),`\n`,(0,n.jsxs)(e.p,{children:[\"For details on light and heavy APIs, see the \",(0,n.jsx)(e.a,{href:\"/docs/api/rate-limits\",children:\"Rate limits\"}),\" documentation.\"]}),`\n`,(0,n.jsx)(e.h2,{id:\"using-next_page_token\",children:(0,n.jsxs)(e.a,{href:\"#using-next_page_token\",children:[\"Using \",(0,n.jsx)(e.code,{children:\"next_page_token\"}),(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.p,{children:[\"Most Zoom APIs, including \",(0,n.jsx)(e.a,{href:\"/docs/api/accounts/#tag/dashboards/get/metrics/meetings/{meetingId}/participants\",children:\"List Meeting participants\"}),`\nand `,(0,n.jsx)(e.a,{href:\"/docs/api/accounts/#tag/dashboards/get/metrics/webinars/{webinarId}/participants\",children:\"List Webinars\"}),\", use the \",(0,n.jsx)(e.code,{children:\"next_page_token\"}),\" parameter for pagination.\"]}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsxs)(e.li,{children:[\"If the initial API query returns a complete response, the \",(0,n.jsx)(e.code,{children:\"next_page_token\"}),\" value will be an empty string.\"]}),`\n`,(0,n.jsxs)(e.li,{children:[\"A non-empty \",(0,n.jsx)(e.code,{children:\"next_page_token\"}),\" value (e.g., \",(0,n.jsx)(e.code,{children:'\"abc3445rg\"'}),\") indicates that additional results are available.\"]}),`\n`]}),`\n`,(0,n.jsxs)(e.p,{children:[\"To retrieve additional responses, include the \",(0,n.jsx)(e.code,{children:\"next_page_token\"}),\" value in subsequent API requests using the same query parameters.\"]}),`\n`,(0,n.jsxs)(e.p,{children:[\"A request to the List Meetings API can return a response with a \",(0,n.jsx)(e.code,{children:\"next_page_token\"}),\" similar to:\"]}),`\n`,(0,n.jsx)(e.pre,{children:(0,n.jsx)(e.code,{className:\"language-json\",children:`{\n  \"from\": \"2024-02-07\",\n  \"meetings\": [\n    {\n      \"duration\": \"05:27\",\n      \"email\": \"jchill@example.com\",\n      \"end_time\": \"2024-02-15T23:30:19Z\",\n      \"has_recording\": false,\n      \"id\": 123456,\n      \"participants\": 2,\n      \"start_time\": \"2024-02-15T23:24:52Z\",\n      \"topic\": \"My Meeting\",\n      \"user_type\": \"Pro|Webinar1000\",\n      \"uuid\": \"abcd1234==\"\n    }\n  ],\n  \"next_page_token\": \"def5678\",\n  \"page_count\": 5,\n  \"page_size\": 1,\n  \"to\": \"2024-03-07\",\n  \"total_records\": 5\n}\n`})}),`\n`,(0,n.jsx)(e.h3,{id:\"example-list-all-users-with-next_page_token\",children:(0,n.jsxs)(e.a,{href:\"#example-list-all-users-with-next_page_token\",children:[\"Example: \",(0,n.jsx)(e.a,{href:\"/docs/api/users/#tag/users/get/users\",children:\"List all users\"}),\" with \",(0,n.jsx)(e.code,{children:\"next_page_token\"}),\":\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(r,{children:[(0,n.jsx)(a,{eventKey:\"nodejs\",title:\"Node.js\",children:(0,n.jsx)(e.pre,{children:(0,n.jsx)(e.code,{className:\"language-js\",children:`const axios = require('axios');\n\nasync function getAllUsers(baseUrl, accessToken) {\nlet allUsers = [];\nlet nextPageToken = '';\n\n    while (true) {\n        const headers = {\n            'Authorization': \\`Bearer \\${accessToken}\\`,\n            'Content-Type': 'application/json'\n        };\n\n        const params = { page_size: 300 };\n        if (nextPageToken) {\n            params.next_page_token = nextPageToken;\n        }\n\n        try {\n            const response = await axios.get(baseUrl, { headers, params });\n            const data = response.data;\n\n            allUsers = allUsers.concat(data.users || []);\n            nextPageToken = data.next_page_token || '';\n\n            if (!nextPageToken) {\n                break;\n            }\n        } catch (error) {\n            console.error('Error fetching users:', error.message);\n            break;\n        }\n    }\n\n    return allUsers;\n\n}\n\n// Usage\nconst baseUrl = 'https://api.zoom.us/v2/users';\nconst accessToken = 'YOUR_ACCESS_TOKEN';\n\ngetAllUsers(baseUrl, accessToken)\n    .then(users => console.log(\\`Total users retrieved: \\${users.length}\\`))\n    .catch(error => console.error('Error:', error.message));\n`})})}),(0,n.jsx)(a,{eventKey:\"python\",title:\"Python\",children:(0,n.jsx)(e.pre,{children:(0,n.jsx)(e.code,{className:\"language-python\",children:`import requests\n\ndef get_all_users(base_url, access_token):\n    all_users = []\n    next_page_token = ''\n\n    while True:\n        headers = {\n            \"Authorization\": f\"Bearer {access_token}\",\n            \"Content-Type\": \"application/json\"\n        }\n\n        params = {\"page_size\": 300}\n        if next_page_token:\n            params[\"next_page_token\"] = next_page_token\n\n        response = requests.get(base_url, headers=headers, params=params)\n        data = response.json()\n\n        all_users.extend(data.get('users', []))\n\n        next_page_token = data.get('next_page_token', '')\n        if not next_page_token:\n            break\n\n    return all_users\n\n# Usage\nbase_url = \"https://api.zoom.us/v2/users\"\naccess_token = \"YOUR_ACCESS_TOKEN\"\nusers = get_all_users(base_url, access_token)\nprint(f\"Total users retrieved: {len(users)}\")\n`})})})]}),`\n`,(0,n.jsx)(e.h2,{id:\"legacy-pagination-page-numbers\",children:(0,n.jsxs)(e.a,{href:\"#legacy-pagination-page-numbers\",children:[\"Legacy pagination: page numbers\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(e.p,{children:[\"Some Zoom APIs, like \",(0,n.jsx)(e.a,{href:\"/docs/api/users/#tag/users/get/users\",children:\"List Users\"}),\", support pagination using \",(0,n.jsx)(e.code,{children:\"page_size\"}),\" and \",(0,n.jsx)(e.code,{children:\"page_number\"}),\" parameters. However, this method is being phased out in favor of \",(0,n.jsx)(e.code,{children:\"next_page_token\"}),\".\"]}),`\n`,(0,n.jsxs)(e.ul,{children:[`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.code,{children:\"page_size\"}),\": Number of records to return per page\"]}),`\n`,(0,n.jsxs)(e.li,{children:[(0,n.jsx)(e.code,{children:\"page_number\"}),\": Specific page to retrieve\"]}),`\n`]}),`\n`,(0,n.jsx)(e.p,{children:\"We recommend setting these values based on your needs, rather than relying on default values.\"}),`\n`,(0,n.jsxs)(e.p,{children:[\"For example, a request to \",(0,n.jsx)(e.a,{href:\"/docs/api/meetings/#tag/meetings/get/users/{userId}/meetings\",children:\"List Meetings\"}),\" with \",(0,n.jsx)(e.code,{children:\"page_number\"}),\" and \",(0,n.jsx)(e.code,{children:\"page_size\"}),\" parameters can return a response similar to:\"]}),`\n`,(0,n.jsx)(e.pre,{children:(0,n.jsx)(e.code,{className:\"language-json\",children:`{\n  \"meetings\": [\n    {\n      \"agenda\": \"Weekly standup\",\n      \"created_at\": \"2024-03-14T21:14:55Z\",\n      \"duration\": 60,\n      \"id\": 987654,\n      \"join_url\": \"https://zoom.us/j/987654\",\n      \"start_time\": \"2024-03-15T22:00:00Z\",\n      \"topic\": \"Team Meeting\",\n      \"type\": 2\n    }\n  ],\n  \"page_count\": 1,\n  \"page_number\": 1,\n  \"page_size\": 1,\n  \"total_records\": 2\n}\n`})}),`\n`,(0,n.jsx)(e.h3,{id:\"example-list-users-with-page-numbers\",children:(0,n.jsxs)(e.a,{href:\"#example-list-users-with-page-numbers\",children:[\"Example: List users with page numbers\",(0,n.jsx)(e.i,{\"aria-hidden\":\"true\"})]})}),`\n`,(0,n.jsxs)(r,{children:[(0,n.jsx)(a,{eventKey:\"nodejs\",title:\"Node.js\",children:(0,n.jsx)(e.pre,{children:(0,n.jsx)(e.code,{className:\"language-js\",children:`const axios = require('axios');\n\nasync function getAllUsers(baseUrl, accessToken) {\n    let allUsers = [];\n    let pageNumber = 1;\n    const pageSize = 300;\n\n    while (true) {\n        const headers = {\n            'Authorization': \\`Bearer \\${accessToken}\\`,\n            'Content-Type': 'application/json'\n        };\n\n        const params = {\n            page_size: pageSize,\n            page_number: pageNumber\n        };\n\n        try {\n            const response = await axios.get(baseUrl, { headers, params });\n            const data = response.data;\n\n            allUsers = allUsers.concat(data.users || []);\n            \n            if (pageNumber >= data.page_count) {\n                break;\n            }\n            \n            pageNumber++;\n        } catch (error) {\n            console.error('Error fetching users:', error.message);\n            break;\n        }\n    }\n\n    return allUsers;\n}\n\n// Usage\nconst baseUrl = 'https://api.zoom.us/v2/users';\nconst accessToken = 'YOUR_ACCESS_TOKEN';\n\ngetAllUsers(baseUrl, accessToken)\n    .then(users => console.log(\\`Total users retrieved: \\${users.length}\\`))\n    .catch(error => console.error('Error:', error.message));\n`})})}),(0,n.jsx)(a,{eventKey:\"python\",title:\"Python\",children:(0,n.jsx)(e.pre,{children:(0,n.jsx)(e.code,{className:\"language-python\",children:`import requests\n\ndef get_all_users(base_url, access_token):\n    all_users = []\n    page_number = 1\n    page_size = 300\n    \n    while True:\n        headers = {\n            \"Authorization\": f\"Bearer {access_token}\",\n            \"Content-Type\": \"application/json\"\n        }\n        \n        params = {\n            \"page_size\": page_size,\n            \"page_number\": page_number\n        }\n        \n        response = requests.get(base_url, headers=headers, params=params)\n        data = response.json()\n        \n        all_users.extend(data.get('users', []))\n        \n        if page_number >= data.get('page_count', 0):\n            break\n        \n        page_number += 1\n    \n    return all_users\n\n# Usage\nbase_url = \"https://api.zoom.us/v2/users\"\naccess_token = \"YOUR_ACCESS_TOKEN\"\nusers = get_all_users(base_url, access_token)\nprint(f\"Total users retrieved: {len(users)}\")\n`})})})]})]})}function h(s={}){let{wrapper:e}={...o(),...s.components};return e?(0,n.jsx)(e,{...s,children:(0,n.jsx)(p,{...s})}):p(s)}function u(s,e){throw new Error(\"Expected \"+(e?\"component\":\"object\")+\" `\"+s+\"` to be defined: you likely forgot to import, pass, or provide it.\")}return y(v);})();\n;return Component;",
      "frontmatter": {
        "title": "Pagination"
      },
      "errors": [],
      "matter": {
        "content": "\n# Pagination\n\nZoom APIs use pagination to organize large response datasets into manageable chunks. This approach helps you efficiently handle substantial data payloads when making `GET` requests that list large numbers of resources.\n\n## Response limits\n\nThe number of items returned per page varies based on the API and data retrieval level:\n\n- Light data retrieval: Up to 300 items per page\n- Heavy data retrieval: Up to 30, 50, or 100 items per page, depending on the specific API\n\nFor details on light and heavy APIs, see the [Rate limits](/docs/api/rate-limits) documentation.\n\n## Using `next_page_token`\n\nMost Zoom APIs, including [List Meeting participants](/docs/api/accounts/#tag/dashboards/GET/metrics/meetings/{meetingId}/participants)\nand [List Webinars](/docs/api/accounts/#tag/dashboards/GET/metrics/webinars/{webinarId}/participants), use the `next_page_token` parameter for pagination.\n\n- If the initial API query returns a complete response, the `next_page_token` value will be an empty string.\n- A non-empty `next_page_token` value (e.g., `\"abc3445rg\"`) indicates that additional results are available.\n\nTo retrieve additional responses, include the `next_page_token` value in subsequent API requests using the same query parameters.\n\nA request to the List Meetings API can return a response with a `next_page_token` similar to:\n\n```json\n{\n  \"from\": \"2024-02-07\",\n  \"meetings\": [\n    {\n      \"duration\": \"05:27\",\n      \"email\": \"jchill@example.com\",\n      \"end_time\": \"2024-02-15T23:30:19Z\",\n      \"has_recording\": false,\n      \"id\": 123456,\n      \"participants\": 2,\n      \"start_time\": \"2024-02-15T23:24:52Z\",\n      \"topic\": \"My Meeting\",\n      \"user_type\": \"Pro|Webinar1000\",\n      \"uuid\": \"abcd1234==\"\n    }\n  ],\n  \"next_page_token\": \"def5678\",\n  \"page_count\": 5,\n  \"page_size\": 1,\n  \"to\": \"2024-03-07\",\n  \"total_records\": 5\n}\n```\n\n\n### Example: [List all users](/docs/api/users/#tag/users/GET/users) with `next_page_token`:\n\n<Tabs>\n<Tab eventKey=\"nodejs\" title=\"Node.js\">\n```js\nconst axios = require('axios');\n\nasync function getAllUsers(baseUrl, accessToken) {\nlet allUsers = [];\nlet nextPageToken = '';\n\n    while (true) {\n        const headers = {\n            'Authorization': `Bearer ${accessToken}`,\n            'Content-Type': 'application/json'\n        };\n\n        const params = { page_size: 300 };\n        if (nextPageToken) {\n            params.next_page_token = nextPageToken;\n        }\n\n        try {\n            const response = await axios.get(baseUrl, { headers, params });\n            const data = response.data;\n\n            allUsers = allUsers.concat(data.users || []);\n            nextPageToken = data.next_page_token || '';\n\n            if (!nextPageToken) {\n                break;\n            }\n        } catch (error) {\n            console.error('Error fetching users:', error.message);\n            break;\n        }\n    }\n\n    return allUsers;\n\n}\n\n// Usage\nconst baseUrl = 'https://api.zoom.us/v2/users';\nconst accessToken = 'YOUR_ACCESS_TOKEN';\n\ngetAllUsers(baseUrl, accessToken)\n    .then(users => console.log(`Total users retrieved: ${users.length}`))\n    .catch(error => console.error('Error:', error.message));\n```\n</Tab>\n<Tab eventKey=\"python\" title=\"Python\">\n```python\nimport requests\n\ndef get_all_users(base_url, access_token):\n    all_users = []\n    next_page_token = ''\n\n    while True:\n        headers = {\n            \"Authorization\": f\"Bearer {access_token}\",\n            \"Content-Type\": \"application/json\"\n        }\n\n        params = {\"page_size\": 300}\n        if next_page_token:\n            params[\"next_page_token\"] = next_page_token\n\n        response = requests.get(base_url, headers=headers, params=params)\n        data = response.json()\n\n        all_users.extend(data.get('users', []))\n\n        next_page_token = data.get('next_page_token', '')\n        if not next_page_token:\n            break\n\n    return all_users\n\n# Usage\nbase_url = \"https://api.zoom.us/v2/users\"\naccess_token = \"YOUR_ACCESS_TOKEN\"\nusers = get_all_users(base_url, access_token)\nprint(f\"Total users retrieved: {len(users)}\")\n```\n\n</Tab>\n</Tabs>\n\n## Legacy pagination: page numbers\n\nSome Zoom APIs, like [List Users](/docs/api/users/#tag/users/GET/users), support pagination using `page_size` and `page_number` parameters. However, this method is being phased out in favor of `next_page_token`.\n\n- `page_size`: Number of records to return per page\n- `page_number`: Specific page to retrieve\n\nWe recommend setting these values based on your needs, rather than relying on default values.\n\nFor example, a request to [List Meetings](/docs/api/meetings/#tag/meetings/GET/users/{userId}/meetings) with `page_number` and `page_size` parameters can return a response similar to:\n\n```json\n{\n  \"meetings\": [\n    {\n      \"agenda\": \"Weekly standup\",\n      \"created_at\": \"2024-03-14T21:14:55Z\",\n      \"duration\": 60,\n      \"id\": 987654,\n      \"join_url\": \"https://zoom.us/j/987654\",\n      \"start_time\": \"2024-03-15T22:00:00Z\",\n      \"topic\": \"Team Meeting\",\n      \"type\": 2\n    }\n  ],\n  \"page_count\": 1,\n  \"page_number\": 1,\n  \"page_size\": 1,\n  \"total_records\": 2\n}\n```\n\n### Example: List users with page numbers\n\n<Tabs>\n<Tab eventKey=\"nodejs\" title=\"Node.js\">\n```js\nconst axios = require('axios');\n\nasync function getAllUsers(baseUrl, accessToken) {\n    let allUsers = [];\n    let pageNumber = 1;\n    const pageSize = 300;\n\n    while (true) {\n        const headers = {\n            'Authorization': `Bearer ${accessToken}`,\n            'Content-Type': 'application/json'\n        };\n\n        const params = {\n            page_size: pageSize,\n            page_number: pageNumber\n        };\n\n        try {\n            const response = await axios.get(baseUrl, { headers, params });\n            const data = response.data;\n\n            allUsers = allUsers.concat(data.users || []);\n            \n            if (pageNumber >= data.page_count) {\n                break;\n            }\n            \n            pageNumber++;\n        } catch (error) {\n            console.error('Error fetching users:', error.message);\n            break;\n        }\n    }\n\n    return allUsers;\n}\n\n// Usage\nconst baseUrl = 'https://api.zoom.us/v2/users';\nconst accessToken = 'YOUR_ACCESS_TOKEN';\n\ngetAllUsers(baseUrl, accessToken)\n    .then(users => console.log(`Total users retrieved: ${users.length}`))\n    .catch(error => console.error('Error:', error.message));\n```\n\n</Tab>\n<Tab eventKey=\"python\" title=\"Python\">\n```python\nimport requests\n\ndef get_all_users(base_url, access_token):\n    all_users = []\n    page_number = 1\n    page_size = 300\n    \n    while True:\n        headers = {\n            \"Authorization\": f\"Bearer {access_token}\",\n            \"Content-Type\": \"application/json\"\n        }\n        \n        params = {\n            \"page_size\": page_size,\n            \"page_number\": page_number\n        }\n        \n        response = requests.get(base_url, headers=headers, params=params)\n        data = response.json()\n        \n        all_users.extend(data.get('users', []))\n        \n        if page_number >= data.get('page_count', 0):\n            break\n        \n        page_number += 1\n    \n    return all_users\n\n# Usage\nbase_url = \"https://api.zoom.us/v2/users\"\naccess_token = \"YOUR_ACCESS_TOKEN\"\nusers = get_all_users(base_url, access_token)\nprint(f\"Total users retrieved: {len(users)}\")\n```\n</Tab>\n</Tabs>\n\n\n\n",
        "data": {
          "title": "Pagination"
        },
        "isEmpty": false,
        "excerpt": "",
        "path": "./content/docs/api/pagination.mdx"
      }
    },
    "sidebarName": "api"
  },
  "__N_SSG": true
}
```
