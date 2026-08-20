# Compass action registry

## Request

- Method: `GET`
- URL: `https://cbre.palantirfoundry.com/compass/api/registry/actions?track=default&locale=en`
- Payload: None
- Observed status: `200 OK`

## Query parameters

- `track`: Asset/release track. Observed value: `default`.
- `locale`: Localization language. Observed value: `en`.

## Description

Returns the registry of actions available for Foundry resource types. Each action describes matching resource RIDs, its HTTP method and target URL, display label/icon, and optional category or precedence. The shell uses this registry to build context menus and resource actions such as open, download, analyze, or create pipeline.
