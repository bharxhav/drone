# Latest announcements

## Request

- Method: `POST`
- URL: `https://cbre.palantirfoundry.com/graphql-gateway/api/bulk?q=LatestAnnouncementQuery`
- Content-Type: `application/json`
- Accept: `text/event-stream`
- Observed status: `200 OK`

## Payload

```json
{
  "operations": {
    "0": "query LatestAnnouncementQuery { pinnedNotifications(filter: {types: [ANNOUNCEMENT_CRITICAL, ANNOUNCEMENT_INFO, ANNOUNCEMENT_WARNING]}, limit: 20) { ... } }"
  },
  "requests": [
    {
      "hash": "0",
      "name": "LatestAnnouncementQuery",
      "variables": {}
    }
  ]
}
```

## Description

Loads up to 20 pinned critical, informational, or warning announcements for the current user. The complete query also requests notification timestamps, acknowledgement and revocation state, timeout, text, permissions, and rendering fields used to determine whether and how an announcement should be displayed.
