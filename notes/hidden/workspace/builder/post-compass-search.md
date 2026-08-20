# Project preview search

- Method: `POST`
- Endpoint: `/foundry-search/api/compass/v0/search`
- Observed response: `200 OK`
- Observed header: `deprecation: true`

```json
{
  "query": {
    "includeResources": ["FILE"],
    "filter": {"isProjectPreview": true, "type": "isProjectPreview"},
    "text": ""
  },
  "limit": 50,
  "sort": {"order": "ASC", "category": "NAME"},
  "offset": 0
}
```

Fetches up to 50 project-preview resources, sorted by name, for the Pipeline Builder project filter.
