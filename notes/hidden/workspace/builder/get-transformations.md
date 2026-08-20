# Transformation definitions

- Method: `GET`
- Endpoint: `/eddie/api/transformations/all`
- Payload: None
- Observed response: `200 OK`

Observed repeated query parameters enable transformation and migration capabilities:

```text
featureFlags=MEDIA_GENERATORS
featureFlags=MEDIA_FILTER
featureFlags=EXTRACT_TEXT_V2_EXPRESSIONS
featureFlags=MEDIA_TRANSFORMATIONS
featureFlags=APPROXIMATE_NEAREST_NEIGHBOURS_JOIN
featureFlags=JSON_V2_TRANSCRIPTION_EXPRESSIONS
migrationFeatureFlags=REGEX_MATCH_V2
migrationFeatureFlags=USE_LLM_V3
migrationFeatureFlags=CAST_V2
migrationFeatureFlags=CPU_JSON_TRANSCRIPTION_V2
migrationFeatureFlags=GPU_JSON_TRANSCRIPTION_V2
language=ENGLISH
```

Returns `allTransformDefinitions`, the transformation catalog used to construct and validate Pipeline Builder nodes. Definitions include IDs, versions, labels, descriptions, arguments, output types, categories, and capabilities.
