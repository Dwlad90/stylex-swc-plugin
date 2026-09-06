# stylex-state

The state one file is compiled against, and the value vocabulary that state
composes. Everything the compiler learns about a file while it walks it is
recorded here. Nothing here decides what a style _means_, and nothing here
evaluates an expression.

The `resolution` module answers the next question after the record: which
declaration binds a given identifier, and what that declaration says. The value
types sit beside the state manager rather than one layer down because they name
each other and it: a function config carries a
[theme reference](#theme-reference), a theme reference reads the state manager,
and an evaluated value can be a function config. The one edge that cannot be
pointed across a crate boundary is named in
[ADR 0001](./docs/adr/0001-the-state-crate-stays-whole-while-a-callback-aliases-it.md).

## Language

**State manager**:
`StateManager` — everything the transform knows about the file it is part-way
through: imports, declarations, discovered style objects, caches, and the
current [transformation cycle](../stylex-enums/CONTEXT.md). One per file, passed
by mutable reference through the whole visitor. It derives `Clone`, and a
dynamic style's callback clones it per invocation, which is why the
binding-write sets, the declared bindings and every candidate index sit behind
`Rc`. The lookup structures it answers its position questions from live in
[stylex-state-index](../stylex-state-index/CONTEXT.md).
_Avoid_: context, session, environment, state

**State writer**:
A `fill_*` function that records what the visitors walked into the state manager
and answers nothing — `fill_top_level_expressions`, `fill_state_declarations`.
Nothing a writer records is a decision about what a declaration _means_.
_Avoid_: collector, populate, scan

**Declaration lookup**:
Which declaration binds a name, asked of the indices the state manager fills
while it walks the file. Four readers over one idea. `declaration_of` and
`import_binding` are methods on the state manager, since a name and an index are
all they need. `get_var_decl_by_ident` and `get_var_decl_parts_by_ident` are
free functions, because they can also answer from the injected function map:
where the state recorded nothing, the declarator lookup falls through to that
map and synthesizes a declarator from a mapper entry.

A lookup only _matches_; what the matched declaration means is the caller's
question. It answers the first steps of the
[reference resolution chain](../stylex-evaluator/CONTEXT.md) without being that
chain.
_Avoid_: binding resolver, identifier lookup, declaration finder, symbol table

**Spelled value**:
What an expression says when read literally, with no fold: the string a literal
or a chain of identifiers spells, the expression a declaration was initialized
with, a template with each substituted identifier replaced by its initializer.
Reading stops at the first thing that is neither a literal nor another
identifier, and answers _nothing_ there rather than refusing — what a
non-literal means belongs to the caller.

A literal read as an authored style value keeps its JavaScript type: a numeric
literal stays a number, and everything else with a string form becomes one,
because that distinction is what decides whether a unit suffix is appended
later. So `42` and `"42"` are two answers, not one. A conversion that would have
to _evaluate_ an expression lives above this crate, and that split is what keeps
the state out of the evaluation cycle.
_Avoid_: literal value, static value, constant folding, resolved value

**Theme reference**:
What an import of a `defineVars` group resolves to: the group as a whole, named
by the hash of the declaring file _and_ the export name. It carries no
expression form and cannot be materialized, because the keys it would need live
in the other file — so a style value's CSS comes from a _member_ read off it
(`zIndex.ten` is `var(--x1ew7r74)`), and the group read without one is refused
wherever a value belongs. Refused, not dropped: answering "no value" compiled
the object as if the declaration had not been written.

A chain of two or more names is one member and not a read of a read:
`colors.brand.primary` names the token `brand.primary`. A chain that is the
callee of a call is not one — `colors.brand.toUpperCase()` resolves
`colors.brand` and calls a string method on it.
_Avoid_: token group, theme object, vars object, defineVars value

**Import specifier kind**:
Which of `{ c }`, `c` or `* as c` bound the name a reference reads, answered by
the same lookup that matched the reference. This is SWC's `ImportSpecifier`, not
the crate's own `ImportKind`, which names the StyleX API an import resolves to.
A named specifier resolves to a theme reference. A default one is refused
outright, because a theme file is read through its named exports. A namespace
specifier names no export at all, so nothing can be built from it and it falls
through to the chain's terminal refusal.

The question is about the specifier and not the declaration, because one
declaration carries two kinds at once:
`import tokens, { colors } from 'colors.stylex.js'` must refuse `tokens` and
still resolve `colors`. A specifier is matched by its **local binding** and
nothing else: `import { spacing as sp }` binds `sp`.
_Avoid_: import kind, import shape

**Binding write**:
A binding whose value can differ from its declaration initializer, either
rebound or mutated in place. All three kinds make the initializer an unsound
stand-in and refuse with the same message, but they are recorded apart —
**reassignment** for a name given a new value, **mutation** for a value changed
under a name that still points at it — because the
[chain](../stylex-evaluator/CONTEXT.md) probes them as two sequential steps. One
walk fills them all, and crossing a member hop is what makes a write the second
kind.

They are keyed by full SWC `Id`, so a write to a shadowing binding never deopts
the one it shadows, and a write to a name this module does not declare never
deopts a global. A write is refused against the _binding_, not a declarator:
destructured names, parameters, `catch` bindings and hoisted `function` /
`class` declarations are all bindings a write makes stale.

The third kind is a **deep mutation** — a write more than one member hop out,
`obj.a.b = 1`. The reference implementation does not count it as a mutation of
`obj` at all and folds the initializer. This refuses instead, but only for a
binding whose initializer the chain would actually inline.
_Avoid_: dirty binding, stale binding, nested mutation, transitive write

**Seen value**:
A memoized evaluation, keyed by the 128-bit
[structural hash](../stylex-utils/CONTEXT.md) of the expression. `resolved`
distinguishes a completed evaluation from one in progress, which is how cyclic
references terminate. It acts on a hash hit without confirming equality, which
is why the key is that wide. The key covers the whole remaining subtree and is
taken again at every level, so the memo costs grow about quadratically with
depth — nearly all of what folding a deep expression costs
([ADR 0005](../stylex-evaluator/docs/adr/0005-the-memo-key-is-a-whole-subtree-hash.md)).
_Avoid_: cache entry, memo
