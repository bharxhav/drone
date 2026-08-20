# Home page body data

## Request

- Method: `POST`
- URL: `https://cbre.palantirfoundry.com/graphql-gateway/api/bulk?q=LongTermHomePageBody_Query`
- Content-Type: `application/json`
- Accept: `text/event-stream`
- Observed status: `200 OK`

## Payload

```json
{
  "operations": {
    "0": "query LongTermHomePageBody_Query { favoriteApplications: favorites(filter: {types: [WORKSPACE_APPLICATION]}) { ... } me { ... } recents(limit: 30) { ... } }"
  },
  "requests": [
    {
      "hash": "0",
      "name": "LongTermHomePageBody_Query",
      "variables": {}
    }
  ]
}
```

## Description

Loads data displayed in the home page body: favorite workspace applications, the current user's AIP and support metadata, and up to 30 recently viewed resources with timestamps, icons, names, links, and project metadata.
