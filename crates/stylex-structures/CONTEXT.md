# stylex-structures

The data the compiler carries between phases: options as configured, the
per-file pass, and the small value types the CSS and transform layers pass
around. Types here hold state; they do not act on it.

## Language

**Pair**:
One CSS declaration as `{ key, value }`, both already strings. `PairCow` is the
same thing borrowed, used on paths that usually pass a declaration through
unchanged.
_Avoid_: declaration, property-value, tuple

**CSS text**:
What a value spells once it reaches a stylesheet, via `as_css_text`. On a
**Pair** that is the whole `key:value;`, and `None` when either half is
[blank CSS text](../stylex-utils/CONTEXT.md) — `top:` is not valid CSS, so
nothing is emitted rather than an empty declaration. On a `TRawValue` it is the
value half alone, and always exists. This is the name for the emitted string,
which is why a pair still is not called a declaration.
_Avoid_: declaration, serialized pair, rule text

**Order pair**:
`OrderPair(property, Option<value>)` — one entry in an expansion of a shorthand,
where `None` means the property is emitted as null to clear it. Its key is
`Cow<'static, str>` because roughly a thousand construction sites in the
[order strategies](../stylex-css/CONTEXT.md) pass a literal.
_Avoid_: expanded pair, shorthand entry

**Plugin pass**:
The per-file context handed in by the host — the filename and the working
directory. It is the compiler's only knowledge of where the code came from.
_Avoid_: context, file info, options

**UID generator**:
The source of generated identifiers, counting per prefix. Its counter is either
local to the instance or thread-local
([counter mode](../stylex-enums/CONTEXT.md)); the thread-local mode exists so
tests do not observe each other's numbering.
_Avoid_: name generator, counter, id factory

**Top-level expression**:
`TopLevelExpression(kind, expr, Option<name>)` — a module-level expression the
`Discover` phase recorded, so a later phase can rewrite or drop it without
walking the module again.
_Avoid_: statement, module expression

**Base CSS type**:
A `{ syntax, value }` object — a variable's declared
[CSS syntax](../stylex-enums/CONTEXT.md) paired with its value. Converts to and
from an object literal, which is how it survives the round trip through
authored code.
_Avoid_: typed value, css var type

**Env entry**:
One value in the `env` configuration map: either a static expression or a
compile-time `JSFunction` taking `Vec<Expr>` and returning an `Expr`. The
function case is what lets configuration compute a value per call site.
_Avoid_: constant, env var, config value

**Ceiling**:
A bound a project can raise, and the rule for choosing its value: the configured
option, then that ceiling's environment variable, then the built-in default,
clamped to a limit past which neither is honoured. Precedence in that order so a
stray value in a CI environment cannot change what a project that configured the
option compiles to; zero and anything non-numeric are read as unset rather than
honoured, because a ceiling of zero refuses the folds the compiler runs to do
its own work. The environment is read once per process, since a `getenv` per
options value measured at roughly 3% on a small module. Three exist — the
evaluation ceiling and the two allocation ceilings below — and each is a
declaration of what it bounds, since the rule is not per bound.

A ceiling's **limit** is the number past which neither an option nor the
environment is honoured — the ceiling on the ceiling, and a separate concept
from the ceiling itself. It exists because a bound the failure arrives before is
not a bound: it is the old crash under a new name.
_Avoid_ (for the ceiling): threshold, budget, knob, tuning value

**Evaluation ceiling**:
`maxEvaluationDepth` — how many levels the evaluator descends into a nested
expression before refusing it, default 32, `STYLEX_MAX_EVALUATION_DEPTH`.
Counted in evaluation steps, not in levels of source nesting — the [evaluation
depth](../stylex-transform/CONTEXT.md) it bounds is defined where it is spent.
_Avoid_: recursion limit, max depth, nesting limit

**Allocation ceilings**:
`maxFoldedCharacters` and `maxFoldedEntries` — how long a string and how many
array elements and object properties one [fold](../stylex-transform/CONTEXT.md)
may build or carry. Defaults 1000000 and 10000,
`STYLEX_MAX_FOLDED_CHARACTERS` and `STYLEX_MAX_FOLDED_ENTRIES`. They exist
because the engine a fold runs on bounds loops, recursion and stack but not
allocation: growth inside a native builtin is not a counted loop, so a mistyped
repeat count agrees with the language and reaches gigabytes. Two rather than one
because the costs do not stand in for each other — measured, a code unit is
about 19 bytes of peak resident memory while it is being built and an entry
about 190, and a bounded string can still become one entry per code unit. Each
bounds both directions: what a resolved name copies into the engine, and what an
answer carries back.
_Avoid_: string limit, size cap, memory budget, amplification limit
