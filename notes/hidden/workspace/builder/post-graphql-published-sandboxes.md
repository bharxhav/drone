# Published sandboxes

- Method: `POST`
- Endpoint: `/graphql-gateway/api/bulk?q=PublishedSandboxesQuery`
- Observed response: `200 OK`

```json
{
  "requests": [{
    "hash": "0",
    "name": "PublishedSandboxesQuery",
    "variables": {
      "pipelineRid": "<pipeline RID>",
      "filter": {"publishStatus": ["PUBLISHED"]},
      "limit": 100
    }
  }]
}
```

Lists published sandboxes for a pipeline, including each sandbox ID and backing branch name.
