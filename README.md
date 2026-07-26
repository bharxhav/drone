<p align="center">
  <img src="assets/logo.svg" width="144" alt="drone logo">
</p>

<h1 align="center">drone</h1>

<p align="center">a stateful cli for palantir foundry</p>

<br>
<br>

**synopsis**

```sh
drone [--help | --version]
drone <verb> [--json] [<domain> [<*>]]
```

_`<*>` are domain-specific scope segments._

<br>
<br>

**verbs**

man

```sh
# Browse Palantir Foundry documentation.
# Opens an interactive terminal ui.
drone man [--json]

# Browse product guides such as `notepad`, `aip-features`.
drone man product <*>

# Browse the Platform API V2 reference.
drone man platform <*>

# Browse Foundry platform updates.
drone man updates <*>
```

<br>
<br>
