# Current session definition

## Request

- Method: `GET`
- URL: `https://cbre.palantirfoundry.com/multipass/api/sessions/definitions/current`
- Payload: None
- Observed status: `204 No Content`

## Description

Checks for the current Multipass scoped-session definition. A `204` response indicates that there is no session definition body to apply. The route issued this request multiple times while initializing authenticated clients.
