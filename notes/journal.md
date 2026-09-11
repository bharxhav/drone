# journal

## forms

palantir resources seem to have various forms. Sometimes RID's are the only way, some times api names are the only way. Uncolored data is pure schema, while when post discovery there's a requirement to inspect the data, there's a new form of the actual data's assumed Type.

## inconsistant nomen

they use Load and Execute interchangable. Get is always for definition, or definition of the definitoin object.

## Object Sets

This is quire fascinating. Obvject sets seem to be "temporary" things that receive RIDs. First of it's kind. It not a server side truth UNLESS the object set is inside an executable. Like AIP logic. Technically my query functions are object sets too (in functional sense only), with full TS semantics and mutliple objects. So, these are server side queries with a lifetime. IF created via platform api, their lifetime is 1HR. If they are in automate precondition, I believe it depends on polling time. If inside AIP, max possible is ofcourse 280s.

So, Object Set can be constructed via a query, or some preconditions (using api), or the coolest part other object sets. They can be heterogenous sets. Intersting thing is you can also load them via interfaces. That feels like it's leaking type optimization mechanic into d2d query mechanics.

Object sets are the only ontology concept where the expression is the value. Object type, interface type, query type all separate definition from instance. An object set has no definition endpoint because the definition is the request payload which is exactly why it doesn't belong in foundry-types.
