# Sandboxes with snapshots

- Method: `POST`
- Endpoint: `/eddie/api/pipelines-v2/get-sandboxes-with-snapshots`
- Observed response: `200 OK`

```json
{
  "requests": [{
    "pipelineRid": "<pipeline RID>",
    "sandboxId": "<sandbox UUID>"
  }]
}
```

Bulk-loads sandbox metadata, current version, backend, and full pipeline snapshot. Snapshots include transforms, targets, groups, expressions, outputs, arguments, compute profile, and backend configuration.
