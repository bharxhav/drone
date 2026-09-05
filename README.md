<p align="center">
  <img src="assets/logo.svg" width="144" alt="drone logo">
</p>

<h1 align="center">drone</h1>

<p align="center">palantir foundry project manager</p>

<br>
<br>

**synopsis**

```sh
drone [--help | --version]
drone init
drone import <rid>
drone <resource> <effect> [<*>] [--json | --toon]
```

<br>
<br>

**project**

A folder containing `Dronefig.toml` is a Drone project. `Dronefig.lock` records resolved resource identities and synchronization state.

A project centralizes application-level Foundry resources: code repositories, AIP Logic, automations, ontology types, actions, and other terminal resource types. The goal is to make every terminal Foundry resource type importable.

Resources may span deployments. Local context and edits live together; Drone provides the plumbing to keep them synchronized with Foundry.

It is also git compatible allowing multi project record management.

A project is like a "view//slice" of foundry resources.
