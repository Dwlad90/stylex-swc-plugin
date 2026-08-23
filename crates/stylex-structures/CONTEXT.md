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

**Evaluation ceiling**:
`maxEvaluationDepth` — how many levels the evaluator descends into a nested
expression before refusing it, resolved from the configured option, then the
`STYLEX_MAX_EVALUATION_DEPTH` environment variable, then the built-in default of 32. Precedence in that order so a stray value in a CI environment cannot change
what a project that configured the option compiles to; zero and anything
non-numeric are read as unset rather than honoured, because a ceiling of zero
refuses the folds the compiler runs to do its own work. Counted in evaluation
steps, not in levels of source nesting — the [evaluation
depth](../stylex-transform/CONTEXT.md) it bounds is defined where it is spent.
_Avoid_: recursion limit, max depth, nesting limit
