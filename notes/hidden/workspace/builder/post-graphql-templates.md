# Marketplace templates

- Method: `POST`
- Endpoint: `/graphql-gateway/api/bulk?q=BrowseTemplatesViewQuery`
- Observed response: `200 OK`

```json
{
  "requests": [{
    "hash": "0",
    "name": "BrowseTemplatesViewQuery",
    "variables": {
      "storeRid": "<marketplace remote-store RID>",
      "predicateQuery": [{"category": "SPLASH_PAGE", "tag": "pipelineBuilder"}]
    }
  }]
}
```

Searches up to 500 installable Marketplace products tagged for the Pipeline Builder splash page, sorted by name. It returns template metadata, versions, output shapes, localized descriptions, and thumbnail attachment paths.
