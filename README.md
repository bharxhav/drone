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
drone [<resource>...] <verb> [<*>] [--json | --toon]
```

_Resources compose from left to right. Defaults are omittable._

```sh
drone eu prod search
drone prod search
drone search
```

<br>
<br>

**man**

```sh
# Opens an interactive terminal ui.

# Browse Palantir Foundry documentation.
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

**generic verbs**

list

search

save

alias

info

<br>
<br>

**resource-specific verbs**

coming soon

<br>
<br>
