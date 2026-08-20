# Narrative route APIs

Observed while loading `https://cbre.palantirfoundry.com/workspace/narrative/` on 2026-08-20.

The route rendered the Foundry home page, but requests identified themselves as `narrative-app/6.534.19`. This catalog covers the XHR/fetch calls made during that route load. Repeated calls are documented once. Authentication headers and cookies are intentionally omitted.

## APIs

- `GET /multipass/api/sessions/definitions/current` - [Current session definition](get-multipass-session-definition.md)
- `GET /multipass/api/sessions/selection` - [Scoped session selection](get-multipass-session-selection.md)
- `POST /documentation/api/v2/release-notes/pagination` - [Release notes](post-release-notes-pagination.md)
- `GET /aip-assist/api/chat/checkAvailability` - [AIP Assist availability](get-aip-assist-availability.md)
- `POST /graphql-gateway/api/bulk` - [Initial shell and sidebar data](post-graphql-initial-shell.md)
- `GET /compass/api/registry/actions` - [Compass action registry](get-compass-action-registry.md)
- `POST /graphql-gateway/api/bulk` - [Home page body data](post-graphql-home-page-body.md)
- `GET /workspace/api/lifecycleStatusByApplication` - [Application lifecycle statuses](get-application-lifecycle-statuses.md)
- `POST /graphql-gateway/api/bulk` - [Communication channel categories](post-graphql-communication-channels.md)
- `POST /graphql-gateway/api/bulk` - [Latest announcements](post-graphql-latest-announcements.md)

GraphQL calls share one HTTP endpoint but are separate API operations with distinct payloads and purposes.
