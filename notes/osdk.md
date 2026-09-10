# OSDK

Intended way by palantir team:

```text
@osdk/maker
typed ontology definitions
      |
      | compile + bundle + publish
      v
Product v2.0.0
      |
      | install / upgrade
      v
Live Foundry Ontology
server source of truth
      |
      +-------------------------------------------------------+
      |                         |                             |
      | generate v0.19.0        | generate v1.4.0             | regenerate locally
      v                         v                             v
Functions repository      React repository              SuperRepo
resources.json selects    Developer Console selects     watches Maker definitions
its required entities     its required entities         and stays current
      |                         |                             |
      | still pinned            | regenerated after           | regenerated whenever
      | to old snapshot         | Product v2.0.0 install      | definitions change
      v                         v                             v
@ontology/sdk@0.19.0      @company/sdk@1.4.0             local @ontology/sdk
STALE                     CURRENT                         CURRENT
      |                         |                             |
      +-------------------------+-----------------------------+
                                |
                                v
                         @osdk/api definitions
               frozen, repository-specific TypeScript views
                    of the Ontology at generation time
```

## criticism

1. Object types cannot have shared ownership.
2. The model requires careful coordination and firefighting.
3. Publications do not break packages inside the monorepo, but can break other products that depend on the object types.
4. There are no migrations.

## advantages

1. Clean architecture.
2. Build once, deploy anywhere.
3. Dependencies inside the monorepo are guaranteed to surface in CI.

## my take

I feel the benefits don't outway the problems. Ordinarily teams are quite chaotic and this is unnecessary centralization.
