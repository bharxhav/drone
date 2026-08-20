# Communication channel categories

## Request

- Method: `POST`
- URL: `https://cbre.palantirfoundry.com/graphql-gateway/api/bulk?q=CommunicationChannelCategories`
- Content-Type: `application/json`
- Accept: `text/event-stream`
- Observed status: `200 OK`

## Payload

```json
{
  "operations": {
    "0": "query CommunicationChannelCategories { communicationChannelCategories @optional { name description channels { id name description subscribed } } }"
  },
  "requests": [
    {
      "hash": "0",
      "name": "CommunicationChannelCategories",
      "variables": {}
    }
  ]
}
```

## Description

Retrieves categories of communication channels and each channel's ID, name, description, and current subscription state. This supports communication or newsletter subscription controls in the shell.
