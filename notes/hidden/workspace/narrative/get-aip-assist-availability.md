# AIP Assist availability

## Request

- Method: `GET`
- URL: `https://cbre.palantirfoundry.com/aip-assist/api/chat/checkAvailability`
- Payload: None
- Observed status: `200 OK`

## Observed response

```json
{
  "available": true
}
```

## Description

Checks whether AIP Assist chat is available for the current user and enrollment. The result controls whether the sidebar and home page expose AIP Assist features.
