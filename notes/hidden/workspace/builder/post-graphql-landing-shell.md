# Landing and shell GraphQL batch

- Method: `POST`
- Endpoint: `/graphql-gateway/api/bulk`
- Query parameter `q`: `CustomerDataSupportedQuery,GetRemoteStoreRid,PipelineLandingRecentsAndFavoritesQuery,PipelineLandingSearchQuery,RecentInstallationsPopoverQuery,ShellSidebarQuery,SidebarWithPanelQuery,WorkspaceSidebarQuery`
- Response type: `text/event-stream`
- Observed response: `200 OK`

```json
{
  "operations": {
    "0": "query CustomerDataSupportedQuery { ... }",
    "1": "query WorkspaceSidebarQuery(...) { ... }",
    "...": "remaining named query documents"
  },
  "requests": [
    {"hash": "<operation index>", "name": "<operation name>", "variables": {}}
  ]
}
```

Bootstraps the Builder landing page and shared shell. It retrieves sidebar data, customer-data eligibility, Pipeline Builder recents/favorites and search results, recent installations, and the Marketplace remote-store RID. The captured request was large and DevTools truncated its tail.
