# Release notes pagination

## Request

- Method: `POST`
- URL: `https://cbre.palantirfoundry.com/documentation/api/v2/release-notes/pagination`
- Content-Type: `application/json`
- Observed status: `200 OK`
- Observed response header: `deprecation: true`

```json
{
  "productId": "FOUNDRY",
  "applicationIds": [],
  "pageSize": 30
}
```

## Description

Loads a page of Foundry release notes for the What's New experience. An empty `applicationIds` array requests notes across applications, and `pageSize` limits the returned page to 30 records. This endpoint was called twice with the same payload during route initialization.
