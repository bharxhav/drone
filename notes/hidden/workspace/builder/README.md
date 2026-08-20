# `/workspace/builder`

Pipeline Builder landing route. Observed on 2026-08-20 using `pipeline-builder-app/6.534.15`.

The initial load issued 27 XHR/fetch requests covering 21 unique API variants. Authentication headers and cookies are omitted from all notes.

## API index

- [Project preview search](post-compass-search.md)
- [Transformation definitions](get-transformations.md)
- [Language model service enabled](get-language-model-service-enabled.md)
- [Natural-language/AIP capabilities](post-natural-language-aip-enabled.md)
- [Pipeline Builder settings](get-pipeline-builder-settings.md)
- [Landing and shell GraphQL batch](post-graphql-landing-shell.md)
- [AIP Assist walkthroughs](post-graphql-walkthroughs.md)
- [Principal search](post-graphql-principals.md)
- [Marketplace templates](post-graphql-templates.md)
- [Pipelines by RID](post-pipelines-by-rids.md)
- [Users by ID](post-users.md)
- [Published sandboxes](post-graphql-published-sandboxes.md)
- [Sandbox snapshots](post-sandboxes-with-snapshots.md)
- [Current pipeline deployment](post-graphql-current-deployment.md)

## APIs documented in another route

These calls were observed on Builder but already have canonical documentation under `/workspace/narrative`, so no duplicate Builder Markdown file is kept:

- `GET /multipass/api/sessions/definitions/current`: [documentation](../narrative/get-multipass-session-definition.md)
- `GET /multipass/api/sessions/selection`: [documentation](../narrative/get-multipass-session-selection.md)
- `POST /documentation/api/v2/release-notes/pagination`: [documentation](../narrative/post-release-notes-pagination.md)
- `GET /aip-assist/api/chat/checkAvailability`: [documentation](../narrative/get-aip-assist-availability.md)
- `GET /workspace/api/lifecycleStatusByApplication?track=default`: [documentation](../narrative/get-application-lifecycle-statuses.md)
- GraphQL `LatestAnnouncementQuery`: [documentation](../narrative/post-graphql-latest-announcements.md)

## Repeated calls

- `GET /multipass/api/sessions/definitions/current`: 5 calls total.
- `GET /multipass/api/sessions/selection`: 3 calls total.
- `POST /documentation/api/v2/release-notes/pagination`: 2 calls total with the same payload.

GraphQL operations sharing `/graphql-gateway/api/bulk` are documented separately because each has a distinct operation name, payload, and purpose.
