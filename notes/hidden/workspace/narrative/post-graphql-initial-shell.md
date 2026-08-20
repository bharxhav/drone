# Initial shell and sidebar data

## Request

- Method: `POST`
- URL: `https://cbre.palantirfoundry.com/graphql-gateway/api/bulk?q=CustomerDataSupportedQuery,LongTermHomePage_NameQuery,ShellSidebarQuery,SidebarWithPanelQuery,WorkspaceSidebarQuery`
- Content-Type: `application/json`
- Accept: `text/event-stream`
- Observed status: `200 OK`

## Payload

The bulk envelope contains GraphQL query documents in `operations` and references them from `requests`:

```json
{
  "operations": {
    "0": "query CustomerDataSupportedQuery { ... }",
    "1": "query WorkspaceSidebarQuery($currentAppRid: RID!, $_currentAppRidIsMissing: Boolean!) { ... }",
    "2": "query LongTermHomePage_NameQuery { ... }",
    "3": "query ShellSidebarQuery { ... }",
    "4": "query SidebarWithPanelQuery { ... }"
  },
  "requests": [
    {
      "hash": "0",
      "name": "CustomerDataSupportedQuery",
      "variables": {}
    },
    {
      "hash": "1",
      "name": "WorkspaceSidebarQuery",
      "variables": {
        "currentAppRid": "<application RID>",
        "_currentAppRidIsMissing": "<boolean>"
      }
    }
  ]
}
```

The captured body was large and the request inspector truncated its tail; operation names and variable shapes above are the stable API contract visible in the request.

## Description

Bootstraps the shared Foundry shell in one batched GraphQL request. It retrieves customer-data banner eligibility, the user's display name, account/support details, favorite applications and resources, current application metadata, and sidebar/panel state.
