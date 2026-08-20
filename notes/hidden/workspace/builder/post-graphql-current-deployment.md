# Current pipeline deployment

- Method: `POST`
- Endpoint: `/graphql-gateway/api/bulk?q=CurrentPipelineDeploymentQuery`
- Observed response: `200 OK`

```json
{
  "requests": [{
    "hash": "0",
    "name": "CurrentPipelineDeploymentQuery",
    "variables": {
      "pipelineRid": "<pipeline RID>",
      "sandboxId": "<sandbox UUID>"
    }
  }]
}
```

Loads the sandbox's active/current deployment, including status, tasks, target builds, jobs, state changes, ontology publication, and deployment version. The observed pipeline had `currentDeployment: null`.
