# AIP Assist walkthroughs

- Method: `POST`
- Endpoint: `/graphql-gateway/api/bulk?q=SelectWalkthroughsForAppQuery`
- Observed response: `200 OK`

```json
{
  "requests": [{
    "hash": "0",
    "name": "SelectWalkthroughsForAppQuery",
    "variables": {"appRid": "ri.workspace..application.pipeline-builder"}
  }]
}
```

Loads AIP Assist walkthroughs and user progress metrics for Pipeline Builder. The observed response contained an empty walkthrough list.
