# Scoped session selection

## Request

- Method: `GET`
- URL: `https://cbre.palantirfoundry.com/multipass/api/sessions/selection`
- Payload: None
- Observed status: `204 No Content`

## Description

Checks the user's current Multipass scoped-session selection. The observed `204` response means no selection document was returned. This is part of authenticated application initialization.
