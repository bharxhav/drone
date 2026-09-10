# Drone

```text
          +---- Any Changes to any resource <-----------------------------------------------+
          |                                                                                 |
          v                                                                                 |
 Palantir Live Data                                                                         |
          |                                                                                 |
          +---------------------------------------------+                                   |
          |                                             |                                   |
          v                                             v                                   |
   drone import                                   drone sync                                |
first-time local snapshot                  refresh local snapshots                          |
          |                                             |                                   |
          +----------------------+----------------------+                                   |
                                 |                                                          |
                                 v                                                          |
                          Local Evidence                                                    |
                 snapshots, repositories, generated APIs                                    |
                                 |                                                          |
                                 v                                                          |
                   drone compare [resources...]                                             |
                                 |                                                          |
                                 |                                                          |
                                 |                                                          |
                   +-------------+-------------+                                            |
                   |                           |                                            |
                   v                           v                                            |
          Rust-style diagnostics      drone draft [resources...]                            |
                                      AI-ready Foundry change prompt                        |
                                                  |                                         |
                                                  v                                         |
                                                AI FDE                                      |
                                                  |                                         |
                                                  +-----------------------------------------+
```

## Project Model

A Drone project is a sequence of terminal Foundry resources. Palantir Live Data remains the operational source of truth. Drone keeps local representations so that every live resource can be compared against concrete repository code, generated APIs, and previous snapshots. Thus a working copy!

Terminal resources include:

- Ontology object types and related ontology entities.
- TypeScript v2 Functions repositories.
- Python Transforms repositories.
- React application repositories.
- Other Foundry code repositories and resources. (depends on availability)

Code repositories are added as Git submodules for composability.

## Commands

`drone import ri..` imports a Palantir resource into the Drone project for the first time. For ontology resources, it records a local snapshot of the live definition and identity. It creates `dronfig.toml` and `dronfig.lock` for housekeeping.

`drone sync [resources....]` refresh a stale resource.

`drone compare [resources...]` compares two local resources for any overlap and reports any mismatch.

`drone draft [resources...]` generates a clean dependency graph (including embedded resources like ts-backed-action which itself calls an aip, which calls an action etc...) and provides one or more prompts that needs to be executed sequenctially on AI FDE. Ideally, you'd execute each prompt on a foundry branch, merge if needed, and apply your vision live.

## What Drone Solves

### Shared ownership

Foundry products assign an object type to one product owner. Drone instead lets any project import the same live object type into its working copy.

A team can propose a change from one repository with that project's point of view, apply it to the live object type, sync the next project, make the next compatible update, and continue across the dependency graph. Ownership remains in Foundry while development context can come from every affected project.

### Coordination and firefighting

Drone discovers dependencies and compares live resources with local repositories before changes are made. `drone draft` orders the required changes and produces separate prompts when they must be applied sequentially.

The developer therefore receives an explicit path instead of manually discovering downstream breakage after deployment.

```text
Project A requirement
      -> update live object type on a Foundry branch
      -> merge
      -> sync Project B
      -> update Project B from its own point of view
      -> repeat until every dependent resource agrees
```

### Cross-product breakage

A monorepo CI build only protects packages inside that monorepo. Drone can include repositories from different products in one working copy and compare each generated API against the same live resource.

Before a live change is accepted, Drone can identify:

- Repositories pinned to stale generated APIs.
- Removed or changed properties still used by another product.
- Actions, Functions, AIP Logic, and applications that depend on the changed resource.
- Consumers that are absent from the proposed update sequence.

### Migrations

Foundry does not provide a general schema migration sequence for these resources. Drone models migration as an ordered dependency graph produced by `drone draft`.

A migration may include additive live changes, consumer updates, generated API refreshes, data backfills, verification, and eventual removal of deprecated fields. Each step can be applied on a Foundry branch and checked before the next step begins.

Drone does not silently migrate production. It makes the migration explicit, reviewable, and executable through AI FDE.

### Drift

Generated APIs are repository-specific snapshots and can disagree with the live Ontology. `drone sync` refreshes local evidence; `drone compare` reports differences between the live resource, imported snapshot, repository declaration, and generated API version.

This makes stale generated code visible instead of allowing compile-time truth and runtime truth to diverge unnoticed.

## Preserved Advantages

### Clean architecture

Palantir remains the live source of truth. Repositories retain their native structure and generated SDKs. Drone adds a working-copy and coordination layer without replacing Foundry, Maker, OSDK, or repository build systems.

### Build once, deploy anywhere

Drone does not replace product packaging. Existing products can still be built once and installed in multiple environments. Drone tracks the resources and versions involved so the same intended graph can be checked in each target environment.

### CI guarantees

Dependencies inside one monorepo continue to fail its existing CI. Drone extends this guarantee across imported repositories by configuring the appropriate native checks and then comparing their generated resource views against Palantir Live Data.

The result is two layers of safety:

```text
repository CI
      checks code within one repository or monorepo

Drone
      checks resource consistency across repositories and live Foundry
```

## Additional Benefits

- Projects can adopt Drone incrementally; importing one resource does not require moving its owning repository.
- Teams can investigate a change without claiming ownership of every affected resource.
- A working copy can contain only the resources relevant to one change rather than the whole enrollment.
- Git submodule revisions make the exact repository evidence used for a comparison reproducible.
- Rust-style diagnostics identify the resource, repository, expected state, observed state, and next command.
- AI FDE receives bounded prompts grounded in actual dependencies instead of an unstructured description of the desired outcome.
- Foundry branches provide a review and verification boundary for each generated change step.
