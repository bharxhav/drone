# Pipeline Builder settings

- Method: `GET`
- Endpoint: `/eddie/api/pipeline-builder-settings`
- Payload: None
- Observed response: `200 OK`

```json
{
  "edgeType": {"type": "curved", "curved": {}},
  "previewMode": {"type": "automatic", "automatic": {}}
}
```

Loads user or enrollment-level Pipeline Builder display settings, including graph edge rendering and preview behavior.
