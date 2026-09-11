~

QUICK:
token life span

MULTI ACCOUNT: 3. orchestrate between different foundry systems (via implementation of the actual apis) (this is many subcommands) 4. this is the cherry on top, and that cannot be offered by palantir themselves... use stuff in #2 (account system), and then allow for them to orchestrate developer patterns (cloning a repo, resource specific materialization locally, and back). 5. allow them to talk to each other and share information via apis

UX:
show resolved aliases command when running

WISHLIST:
want to be able to like design erds that encode a lot of details and rationale and also persist to palantir.
nomenclature tools so that name spaces can be set for project

ability to have object sets and queries that again get used for automations etc
ability to visually view them too, from same source.

dry run! -- formal verification

TLA+ :actions, aips, etc. inputs (self, this, executor etc), outputs (Anything, Something, Nothing, etc), Error
Executor, Runtime etc seperation too

the flow is always:

1. I pull the things from server.
2. I read code, and annotate and add local context (comments enabled dsl)
3. I make actual edits to types (for desired state, so mark intentional drift) via fde prompts (since no api way)
4. I then validate the action sequences etc with TLA
5. I then want an implementation sequence (prompt is better since it all can be done in a branch)
