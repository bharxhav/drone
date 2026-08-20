# Application lifecycle statuses

## Request

- Method: `GET`
- URL: `https://cbre.palantirfoundry.com/workspace/api/lifecycleStatusByApplication?track=default`
- Payload: None
- Observed status: `200 OK`

## Query parameters

- `track`: Asset/release track. Observed value: `default`.

## Description

Returns a map from Foundry application ID to lifecycle status, such as `GA`, `BETA`, `EXPERIMENTAL`, `INTERNAL`, `MANDATORY`, `LEGACY`, or `SUNSET`. The workspace shell uses this metadata to classify applications and control how they are presented or made available. The observed map included `narrative-app: MANDATORY`.
