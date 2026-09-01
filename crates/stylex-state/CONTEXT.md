# stylex-state

The state one file is compiled against, and the value vocabulary that state
composes. Everything the compiler learns about a file while it walks it is
recorded here; nothing here decides what a style _means_, and nothing here
evaluates an expression.

The value types sit beside the state manager rather than one layer down because
they name each other and it: a function config carries a
[theme reference](#theme-reference), a theme reference reads the state manager,
and an evaluated value can be a function config. The knot has no cut that leaves
a smaller crate on either side.

## Language

**State manager**:
`StateManager` — everything the transform knows about the file it is part-way
through: imports, declarations, discovered style objects, caches, and the
current [transformation cycle](../stylex-enums/CONTEXT.md). One per file; passed
by mutable reference through the whole visitor. The lookup structures it answers
its position questions from live in
[stylex-state-index](../stylex-state-index/CONTEXT.md).
_Avoid_: context, session, environment, state

**Theme reference**:
What an import of a `defineVars` group resolves to: the group as a whole, named
by the hash of the file that declares it. It carries no expression form and
cannot be materialized the way a [folded function
map](../stylex-evaluator/CONTEXT.md#folded-function-map) is, because the keys it
would need live in the other file -- so the CSS a style value needs comes from a
_member_ read off it (`zIndex.ten` is `var(--x1ew7r74)`), and the group read
without one is refused wherever a value belongs. Refused, not dropped: answering
"no value" there compiled the object as if the declaration had not been written.

A chain of two or more names is one member and not a read of a read:
`colors.brand.primary` names the token `brand.primary`, which is how a group
whose members are groups is written. A chain that is the callee of a call is not
one — `colors.brand.toUpperCase()` resolves `colors.brand` and calls a string
method on it — so which of the two a chain is, is a question about the source.
Inside a fold it crosses as the [carried
value](../stylex-evaluator/CONTEXT.md#carried-value) a group is, with the paths
the guard read off the source named for it.
_Avoid_: token group, theme object, vars object, defineVars value

**Import specifier kind**:
Which of `{ c }`, `c` or `* as c` bound the name a reference reads, answered by
the same lookup that matched the reference and travelling with the declaration
it belongs to. The three kinds get three answers from the first two steps of the
[chain](../stylex-evaluator/CONTEXT.md#reference-resolution-chain). A named
specifier resolves to a theme reference. A default one is refused outright,
because a theme file is read through its named exports and a default binding
names a value from a file this compiler never evaluates. A namespace specifier
binds the whole export object and so names no export at all, which leaves
nothing for a theme reference to be built from: it resolves nothing and falls
through to the chain's terminal refusal. The question is about the specifier and
not about the declaration, because one declaration carries two kinds at once:
`import tokens, { colors } from 'colors.stylex.js'` must refuse `tokens` and
still resolve `colors`. What a specifier is matched by is its **local binding**
and nothing else: an `import { spacing as sp }` binds `sp`, and the name it was
aliased away from binds nothing in this module, so no reference resolves through
it.
_Avoid_: import kind, import shape

**Binding write**:
A binding whose value can differ from its declaration initializer, either
rebound or mutated in place. Both make the initializer an unsound stand-in at
the use site and both refuse with the same text, but they are recorded apart —
**reassignment** for a name given a new value, **mutation** for a value changed
under a name that still points at it — because the
[chain](../stylex-evaluator/CONTEXT.md#reference-resolution-chain) probes them
as the two sequential steps the reference implementation probes. One walk fills
both, and crossing a member hop is what makes a write the second kind. Keyed by
full SWC `Id`, so a write to a shadowing binding never deopts the one it
shadows, and a write to a name this module does not declare never deopts a
global that spells it. What a write is refused against is the _binding_, not a
declarator: destructured names, parameters, `catch` bindings and hoisted
`function` / `class` declarations are all bindings a write makes stale, and each
is refused for the write rather than for whatever a later step would have found.

A third kind sits behind those two. A write more than one member hop from the
binding — `obj.a.b = 1` — is a **deep mutation**, which the reference
implementation does not count as a mutation of `obj` at all: it folds the
initializer and bakes in a value that has since changed. This refuses instead,
but only for a binding whose initializer the chain would actually inline, so a
`function`, a `class` or a destructured name keeps the refusal it already had
rather than being told its value is not constant.
_Avoid_: dirty binding, stale binding, nested mutation, transitive write

**Seen value**:
A memoized evaluation, keyed by the
[structural hash](../stylex-utils/CONTEXT.md) of the expression. `resolved`
distinguishes a completed evaluation from one
currently in progress, which is how cyclic references terminate.

The key covers the whole remaining subtree and is taken again at every level, so
what the memo costs grows about quadratically with depth -- and is nearly all of
what folding a deep expression costs. Why it stays that way, and what an
incremental key would take, is [stylex-evaluator ADR
0005](../stylex-evaluator/docs/adr/0005-the-memo-key-is-a-whole-subtree-hash.md).
This is also one of the two consumers that acts on a hash hit without confirming
equality, which is why that key is 128 bits wide. _Avoid_: cache entry, memo
