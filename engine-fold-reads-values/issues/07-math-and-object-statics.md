# 07 — `Math` and `Object` statics fold

**What to build:** The static methods stop being an arbitrary set of names.

```js
zIndex: Math.trunc(1.5)                    // fails today
gridTemplateAreas: Object.keys({a:1}).join(',')  // fails today
```

Both fail with fully literal arguments, because `Math` and `Object` are
themselves identifiers and the guard refuses an identifier receiver — the same
root cause as the named-receiver gap, in a place the glossary never mentions.
The entire `Math` surface this compiler accepts is seven names; the reference
compiler folds the rest.

A key list is an array, so this depends on arrays crossing the bridge.

**Blocked by:** 06.

**Status:** resolved

- [x] The `Math` methods the reference compiler folds all fold here, with
      matching declaration text and class names — up to the fold's own nesting
      ceiling, which a static now answers to. Recorded on 11
- [x] The `Object` statics the reference compiler folds all fold here — with one
      exception, `Object.getPrototypeOf`, whose answer is an object carrying
      functions and so cannot cross the outward bridge. Recorded on 14
- [x] A static result is chainable — a key list can be joined
- [x] Both static name tables are deleted
- [x] A static the reference compiler deliberately refuses is still refused
      here, with its own reason

## Comments

### From 06

Deleting the array method table took the chain off a static with it —
`Object.entries(o).filter(f)` folded through that table, and there is nothing
below the fold that answers for an array any more. So 06 narrowed the hand-back
rather than the surface: a receiver naming one of these globals is still handed
to the dispatch when it is the *outermost* call, and the engine answers it when
it is a link inside a chain. Nested, the names the reference compiler refuses are
refused here by `INVALID_METHODS` — its own set, and the one the dispatch already
gates its statics on — so `Math.random().toString()` does not fold a different
class name on every build.

Two things that leaves for this ticket.

**The asymmetry is the thing to remove.** `Math.trunc(1.5)` refuses written alone
and folds written inside a chain, because alone it is the dispatch's seven-name
table that answers. Position deciding the answer is the opposite of what one
guard walk is for; it is recorded in `admit_call` and in the glossary, and it
goes away when the surface moves here.

**`__proto__` is a divergence already there.** Measured, upstream reads
`Object.keys({ __proto__: 'x', a: 'y' })` as `a`; this compiler answers
`__proto__,a` through the dispatch, and the fold's inward bridge was made to
agree with the dispatch rather than with upstream — one answer between the two
paths, wrong in the same way, rather than two. Whichever way this ticket settles
it, it settles it for both.

Chaining is pinned by
`transform_stylex_create_test::named_array_receivers::a_chain_off_a_static_folds_end_to_end`
and `::a_static_inside_a_callback_folds` — regression cover for what the deleted
table folded, not this ticket's acceptance. The surface itself is still
unmeasured: `Math` is seven names here against the whole of the reference
compiler's.

## Answer

### The hand-back goes, and with it the last position rule

`admit_call` asked two questions of a receiver naming one of these globals: is it
one, and is this the outermost call. The second is gone, so `Math.trunc(1.5)`
folds written alone exactly as it folded written inside a chain, and the whole
surface is the language's rather than a list of eleven names. `Guard::outermost`
was that rule's only reader and is deleted with it — nothing the guard carries
now records *where* in an expression the walk is.

What is left of the first question is narrower and does more: a global's name
carries no value across the bridge, because the printed source names it and the
language answers. Still only where the module declares no binding of that name,
which is the same map read as before — `const String = 'abc'; String.toUpperCase()`
has to fold, and does.

The names the reference compiler refuses by name are refused here with a reason
of their own: `unfoldable_static`, over `INVALID_METHODS`. Each of them either
answers something new on every build or answers by changing what it was handed,
and a class name is a hash of the declaration it names — so the one sentence
covers both, and says the thing that is true of both rather than picking one. It
names the receiver with the method — `Object.assign`, not `assign` — because on a
static the receiver is the half that disambiguates.

Five refusals now share that first line, so it is one `cannot_fold(call, reason)`
rather than five `format!`s: the line is what a reader learns to recognise, and a
site that spelled it differently would read as a different class of failure
without being one.

### Deleted

`MathJS` and `ObjectJS`, both `TryFrom<&str>` impls and their unit tests.
`CallbackType::Math` and `CallbackType::Object`, which leaves that enum two
variants that are not method names at all. About 370 lines of dispatch: the
seven-name `Math` arm, its five callback arms, `Object.fromEntries` on both
sides. `args_to_numbers` and `push_args_to_numbers`, whose only callers were
those arms. `sort_numbers_factory` and `BUILT_IN_FUNCTION`, left with no caller
by the same deletion.

And `EvaluateResultValue::Entries`, which nothing produced once the own-keys
answer was built directly rather than routed through a callback — a whole result
variant, its `as_entries` reader, and its four match arms.

`context` went with them. It was written at three sites and read only by the
deleted arms, and the guard in front of it (`ARGUMENT_WITHOUT_VALUE` when the
receiver produced no value) could not fire for the one caller left. Its eager
argument pass is gone too; the callback arm evaluates the arguments itself
through `evaluate_func_call_args`, which refuses a spread by name where the eager
pass returned a `Null` placeholder and let the call fall out unconfident with no
sentence.

### What stayed below the fold, and why it is not a table

Three statics still answer there: `Object.keys`, `values` and `entries`. Not
because those three are a surface this compiler chose — `getOwnPropertyNames`,
`fromEntries`, `hasOwn`, `is`, `groupBy` and the rest all fold in the engine now —
but because their *receiver* can be something the engine never sees. Two of those
exist: this compiler's own folded function map, which is not a JavaScript value
at all and is what `Object.keys(stylex)` asks about, and an array with a hole in
it, which the fold will not print. `normalize_object_method_receiver` already
knew both, and its three call sites collapsed into one `OwnKeysQuestion` over one
walk — so keys, values and entries can no longer disagree about what a property
reads as, which the three copied loops could. The walk hangs off
`ObjectMethodReceiver` itself, since it reads nothing else.

Carrying the function map inward instead was considered and rejected. It would
make `stylex.firstThatWorks('a','b')` a fold candidate whose receiver is an
object of placeholders, and the engine would throw on it — turning a compiling
module into a failing one. The narrowing is deliberate and is what the arm's
comment says.

### `__proto__`, settled for both paths

Upstream reads `Object.keys({ __proto__: 'x', a: 'y' })` as `a`; this compiler
answered `__proto__,a` through the deleted table, and the fold's inward bridge
was built to agree with the table. Both now answer `a`.

The written-out case needed nothing: the expression is printed and the language
drops the key, which is what the key means. The named case is the inward bridge,
where the evaluator's object form keeps `__proto__` as an ordinary property and
the object was being rebuilt property by property — so it is dropped there, at
`PROTOTYPE_KEY`, and the two directions agree.

### The two things this ticket had to take on

**A name holding a number crosses now.** `Math.round(BASE / Math.pow(SCALE, 3) / 0.16)`
is how a fluid type scale is written, and every operand of it is a binding — the
`global-tokens` fixture is exactly that, and it stopped compiling the moment the
statics moved. `is_a_carryable_receiver` admits numbers as a result, which also makes
`const n = 5; n.toFixed(1)` fold to `5.0` as upstream does. A boolean comes with
it, not for a case of its own: once a name may hold a number, a boolean is the
only primitive left outside and there is no sentence that would say why — a table
of one, which is the shape this module exists to delete. That
is most of ticket 08's mechanism, landed here because refusing a numeric argument
while accepting a numeric element would be deciding by position again — the thing
this ticket removes. The refusal 08 exists to protect is untouched: it is about
how the receiver was *written*, and `(5).toFixed(1)` still refuses in
`receiver_is_a_written_number`.

**A spread argument is a rule now, not a hand-back.** It has to be: with the
`Math` arm gone there is nothing below to answer it, and `Math.max(...[1, 2])`
read `Cannot fold 'max'` where upstream says `Unsupported expression:
SpreadElement`. The receiver is walked before the arguments, so a call reaching
`admit_argument` is one this module owns and can name its own sentence for. This
also closed a divergence: the deleted arm folded `Math.max(3, 1, 2, ...[5, 0.1, 0.3], 4)`
to `5` by flattening the spread into the argument list, which upstream refuses.

### The cost, and who owns it

A nested `Math.max` folded 158 levels deep and now folds 16. `Math` answers to the
fold's own nesting ceiling — 32 engine levels, two per source level for a call
that also adds — instead of the evaluator's configured one, and raising
`maxEvaluationDepth` does not move it. That is the second, lower limit ticket 11
exists to remove. It is `Math` joining a rule rather than a rule made for `Math`:
the string and array surfaces have answered to it since their own tables were
deleted. Pinned at the new boundary in `evaluation_depth_budget`, with the reason
and the pointer written next to it.

### Measured against `@stylexjs/babel-plugin` 0.19.0

**Every one of the 34 `Math` methods folds to the reference compiler's own class
name and rule text**, against seven before. So do 12 `Object` statics against
four, the chains, a named receiver, a static inside a callback, and both
`__proto__` spellings. Every input upstream rejects, this rejects.

Two divergences, both measured and both named where they are:

| Input | Upstream | Here | Owner |
| --- | --- | --- | --- |
| `Object.getPrototypeOf({a:1})` | `[object Object]` | refuses | outward bridge — a prototype carries functions |
| `Math(1)` | `func.apply is not a function` | `Math is not a function.` | pre-existing, and the better sentence |
| nested `Math.max` past 16 levels | folds | refuses | 11 |

A third is closed rather than opened: the `Object.keys(null)` corpus row moved
from `both-reject` to `both-reject-divergent`, because the refusal is the
engine's own throw now — `cannot convert 'null' or 'undefined' to object` against
the reference runtime's `Cannot convert undefined or null to object`. Both name
the receiver and neither builds the module; the row carries the reason.

**Parity corpus: 1123 subjects, 0 changed, 0 unexpected** after that one row was
re-pinned. `parity:positions` unchanged. The JS suite is green (86/86 turbo
tasks), and `cargo test --workspace --all-features` is 7295 passing.

### Tests

`transform_stylex_create_test::math_and_object_statics`, 16 cases at the highest
seam there is: the two surfaces one method per case, the argument shapes a
stylesheet actually writes, the three positions a static can be written in, the
chains, the named receiver, both `__proto__` spellings, the five refusals with
their own sentences, the shadowed global, the answers that are `NaN` and
`Infinity`, the key-list edges (integer-like ordering, non-ASCII, a primitive
receiver), and a 500-entry list that folds whole against a 20 000-entry one that
refuses by naming the limit.

Rewritten rather than deleted, each because its subject moved:
`unsupported_shape_tests` now has four cases where it had two — the surface, the
nondeterministic refusal, the mutating refusals, and the ones the language itself
throws on. `object_own_keys` says in its header what it is now about.
`names_an_expression_with_no_numeric_reading` and its transform-level twin are
gone: the label they pinned was only reachable through `Math.abs({})`, which
folds to `NaN` here and upstream, and the operators reach the same coercion
through a sentence of their own.

### Measured — Apple M1 Max, `aarch64-apple-darwin`

The benchmark from 01, at `--measurement-time 6 --warm-up-time 2`. Every leg has
written-out receivers and no static in it, so what they price is what this ticket
costs a fold that resolves nothing.

| Leg | `fold` | `engine` | Round trip adds | 06 recorded |
| --- | --- | --- | --- | --- |
| string | 4.15 µs | 2.34 µs | 1.81 µs | 1.75 µs |
| callback | 15.06 µs | 10.69 µs | 4.37 µs | 4.43 µs |
| chain | 8.87 µs | 6.23 µs | 2.65 µs | 2.76 µs |
| array-answer | 8.13 µs | 5.58 µs | 2.55 µs | 2.99 µs |

Cold start 116.52 µs against 06's 120.08. Read each `engine/` leg beside its own
`fold/` leg rather than against 06's column: the engine legs are unchanged code
and moved on their own between sessions. The gap moved −0.44 to +0.06 µs, inside
that drift in both directions — which is what the change predicts. The guard asks
one question *fewer* per call now, since the outermost check is gone, and
everything else this ticket does is paid only by a fold that names a static.

### A note for whoever reads this next

`is_a_carryable_receiver` is the lever. Widening it is how a ticket that looks
like "one surface moves" turns into a fixture regression three crates away, and
the fixture is the thing that catches it — `global-tokens` is the only input in
the repo that writes real arithmetic through a static. Run the whole workspace,
not the crate.
