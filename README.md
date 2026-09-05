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
drone add <rid>
drone <resource> <effect> [<*>] [--json | --toon]
```

<br>
<br>

**project**

A folder containing `Dronefig.toml` is a Drone project. `Dronefig.lock` records resolved resource identities and synchronization state.

A project centralizes application-level Foundry resources: code repositories, AIP Logic, automations, ontology types, actions, and other terminal resource types. The goal is to make every terminal Foundry resource type addable.

Resources may span deployments. Local context and edits live together; Drone provides the plumbing to keep them synchronized with Foundry.

It is also git compatible allowing multi project record management.

A project is like a "view//slice" of foundry resources.

<br>
<br>

**init**

```sh
drone init
```

Creates `Dronefig.toml` and `Dronefig.lock` in the current folder. Git integration is optional; when chosen, initialization also creates a `.gitignore`. If git is enabled, code repositories are added as submodules.

<br>
<br>

**add**

```sh
drone add <rid>
```

Adds a recognized terminal resource to the current project by RID. Adding a resource is not scoped by API name.

Each supported resource kind has a custom type and local representation.

<br>
<br>

**resources**

```sh
drone <resource> <effect>
```

Added resources are addressed by RIDs, API names and aliases. Effects depend on the resource type.

<br>
<br>
