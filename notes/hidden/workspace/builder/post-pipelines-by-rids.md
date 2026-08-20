# Pipelines by RID

- Method: `POST`
- Endpoint: `/eddie/api/pipelines-v2/get/by-rids`
- Observed response: `200 OK`

```json
["<pipeline RID>"]
```

Bulk-loads Pipeline Builder metadata for the supplied pipeline RIDs. The response is a `pipelines` map keyed by RID and includes name, attribution, published sandbox IDs, heartbeat data, and Compass trash/favorite state.
