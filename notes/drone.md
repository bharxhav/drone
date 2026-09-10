# Drone

```text
          +---- Any Changes To Ontology Object-types <--------------------------------------+
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
                   drone check [resources...]                                               |
                                 |                                                          |
                                 | always compares                                          |
                                 v                                                          |
                   Palantir Live Data <-> Local Evidence                                    |
                                 |                                                          |
                   +-------------+-------------+                                            |
                   |                           |                                            |
                   v                           v                                            |
          Rust-style diagnostics      drone prompt [resources...]                           |
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

`drone check [resource]` compares Palantir Live Data against requested local resource.

`drone compare [resources...]` compares two local resources for any overlap and reports any mismatch.

`drone draft [resources...]` generates a clean dependency graph (including embedded resources like ts-backed-action which itself calls an aip, which calls an action etc...) and provides one or more prompts that needs to be executed sequenctially on AI FDE. Ideally, you'd execute each prompt on a foundry branch, merge if needed, and apply your vision live.
