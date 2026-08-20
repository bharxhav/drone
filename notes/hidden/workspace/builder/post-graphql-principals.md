# Principal search

- Method: `POST`
- Endpoint: `/graphql-gateway/api/bulk?q=PrincipalsQuery`
- Observed response: `200 OK`

```json
{
  "requests": [{
    "hash": "0",
    "name": "PrincipalsQuery",
    "variables": {
      "pageSize": 30,
      "principalTypes": ["USER"],
      "shouldFetchCurrentUser": true,
      "query": ""
    }
  }]
}
```

Retrieves a paginated list of user principals for the Created by filter and also fetches the current user.
