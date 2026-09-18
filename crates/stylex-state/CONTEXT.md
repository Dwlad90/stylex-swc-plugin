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
and answers nothing — `fill_top_level_expressions`, `fill_state_declarations`,
`fill_call_positions`. Nothing a writer records is a decision about what a
declaration _means_.
_Avoid_: collector, populate, scan

**Call position**:
Where a call is written, which an SWC visitor cannot ask because it carries no
parent. `fill_call_positions` reads every position from the module once and
`CallPositions` holds them, keyed by the span of the call. Three positions, each
with a reader on the state manager:

- **program level** — a statement of the module itself holds the call, with no
  function, no namespace and no second statement between. It decides whether the
  compiled styles stay where the call was written or are hoisted to a
  declaration above the statement that holds them (`is_program_level_call`).
- **bare statement** — the call is a whole expression statement, so nothing
  reads what it answers. It is the one position a `create` call is refused in
  (`is_bare_call_statement`).
- **type asserted** — a type assertion wraps the call
  (`is_type_asserted_call`).

_Avoid_: top level, module level, root level

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

**Folded value**:
What one folded expression turned out to hold, in the kind JavaScript gives
it: text, a number, a boolean, a `null`, a value that was never given, a list,
or an object of the same kinds again. `folded_value` reads one, and
`FlatCompiledStylesValue` is what it answers. It is not a
[spelled value](#spelled-value), which is read without any fold, and it is not
an `EvaluateResultValue`, which is what the evaluator hands back rather than
the kind one value of it is.

The kind is kept rather than flattened to text, because two readers spell the
same value differently and each needs to know what it has: a `props` call
writes a number back as a number, and a `defineConsts` call writes one into
the module, into the metadata a build tool reads, and into the rule the
runtime is handed. Where a value has to cross a type below this crate, it
travels as the JSON `to_json_text` spells and `from_json_text` reads back --
with the four values JSON has no word for spelled the way JavaScript spells
them.
_Avoid_: evaluated value, resolved value, static value, literal

**Inline style**:
The plain object a `props`-family call was given beside the compiled styles,
which the runtime applies as a `style` property rather than as classes. It is
held in the same `FlatCompiledStylesValue` map a compiled namespace is, and the
two read that map by different rules: a namespace value is a class name, a
`null` for a property it clears, or the compiled marker, while an inline value
is whatever the author wrote -- text, a number, a boolean, a `null`, or an
object of declarations of its own such as the body of `:hover`. The kind is
kept because the two readers of it disagree: `props` writes each value back as
the kind it is, and `attrs` spells it as text the way JavaScript does.
_Avoid_: style property, runtime style, dynamic style, style pairs

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

**Rule-call function map**:
The function map a rule call folds its argument with, held on the state for the
life of the file. One per set of helpers -- see
[stylex-transform](../stylex-transform/CONTEXT.md), **Rule call**. It is built
by the first such call of the module, which is sound only because every writer
of the import sets runs in the `Discover` cycle and the calls run in
`TransformProducers`. A map built before the sets close holds fewer names than
the source spells, and a name it does not hold stops the whole call folding.
_Avoid_: eval config cache, helper table

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

**Registered object**:
The object a producer left where the author wrote the call, or the name a
compiled `keyframes`, `positionTry` or `viewTransitionClass` answered with.
It is what a runtime injection call is keyed to, and the walk that decides
where that call goes looks for it anywhere in a statement —
`export const all = [wrap(stylex.create({…}))]` holds one behind a call, and
the rules still belong to the statement rather than to the object. The walk
stops at the object it matched, because nothing a registered object holds is
registered itself, and it does not read a function body or a namespace, because
a call written in one is not at program level. What the walk costs, on the
shape it saves the most and on the shape it costs anything, is
[ADR 0002](./docs/adr/0002-the-injection-walk-over-a-large-array-is-measured-and-kept.md);
what keying and queueing the call costs is
[ADR 0003](./docs/adr/0003-the-injection-queue-is-deduped-by-a-pair-of-hashes.md).
_Avoid_: held candidate, nested styles, wrapped call, inner object

**Nested rule**:
A rule a producer call written inside another producer's argument declares —
`create({ a: { animationName: keyframes({…}) } })`, or the same `keyframes` in
a `createTheme` value. The inner call is folded to its name where it stands and
registers nothing of its own, so its rule is filed on the state and the
producer that holds it must carry it. `take_nested_rules_before` is the one way
to read them: it puts them in front of the producer's own rules, so the
`@keyframes` block stands in front of the rule that names it, and it takes them
off the state, because they belong to the one call that folded them. A producer
that forgets them prints a name that no stylesheet defines; a producer that
leaves them behind makes the next declaration of the module carry them too.
_Avoid_: dependency rule, other rule, inherited rule, rule call
