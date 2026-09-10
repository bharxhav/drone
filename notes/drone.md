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
