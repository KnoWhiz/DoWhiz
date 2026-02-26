# Retrieve Email Metrics - Gmail Postmaster Tools API

## Overview

The Gmail Postmaster Tools API allows you to retrieve email metrics for specific domains. Metrics are available for individual days or aggregated across all days.

**Important limitation:** Only domains that send mail to at least 50 users per day receive metrics. Domains sending below this threshold will not show metric values.

For optimization guidance, refer to Google's documentation on preventing mail from being blocked or marked as spam.

## Retrieve Metrics for a Specific Day

Use the `domains.trafficStats.get()` method to fetch metrics for a particular date.

### Java Example

```java
public static TrafficStats getTrafficStats(PostmasterTools service,
    String domainName, String date) throws IOException {
  String query = String.format("domains/%s/trafficStats/%s",
      domainName, date);
  TrafficStats trafficStats = service.domains()
      .trafficStats().get(query).execute();
  System.out.println(trafficStats.toPrettyString());
  return trafficStats;
}
```

**Parameters:**
- `service`: Authorized Gmail PostmasterTools API instance
- `domainName`: Fully qualified domain name
- `date`: Date in "YYYYMMDD" format

**Response:** Returns a `TrafficStats` object containing the domain's traffic data for that date.

### Python Example

```python
def get_traffic_stats(service, domain_name, date):
  try:
    query = 'domains/%s/trafficStats/%s' % (domain_name, date)
    traffic_stats = service.domains().trafficStats()
        .get(name=query).execute()
    print(traffic_stats)
    return traffic_stats
  except errors.HttpError as err:
    print('An error occurred: %s' % err)
```

## Retrieve Metrics for All Days

Use the `domains.trafficStats.list()` method to fetch paginated results across all available days.

### Java Example

```java
public static ListTrafficStatsResponse listTrafficStats(
    PostmasterTools service, String domainName, int pageSize,
    String pageToken) throws IOException {
  ListTrafficStatsResponse listTrafficStatsResponse = service
      .domains().trafficStats().list("domains/" + domainName)
      .setPageSize(pageSize)
      .setPageToken(pageToken)
      .execute();
  System.out.println(listTrafficStatsResponse.toPrettyString());
  return null;
}
```

### Python Example

```python
def list_traffic_stats(service, domain_name, page_size, page_token):
  try:
    query = 'domains/' + domain_name
    list_traffic_stats_response = service.domains()
        .trafficStats().list(parent=query, pageSize=page_size,
        pageToken=page_token).execute()
    print(list_traffic_stats_response)
    return list_traffic_stats_response
  except errors.HttpError as err:
    print('An error occurred: %s' % err)
```

## Response Format

The list operation returns a paginated structure:

```json
{
  "trafficStats": [
    {
      "object": "TrafficStats"
    }
  ],
  "nextPageToken": "string"
}
```

The `nextPageToken` enables pagination through large result sets.
