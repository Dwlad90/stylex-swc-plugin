# 05 — The arrow transport, proved on a named string receiver

**What to build:** Giving a string a name stops changing whether it compiles.

```js
const s = 'ABC';
export const styles = stylex.create({ x: { content: s.toLowerCase() } });
```

That fails today, along with twenty-one other string methods, while the same
call on a string written out in full folds. The fold guard asks whether an
expression is *written out*; this ticket changes it to ask whether every leaf
*resolves to a value the bridge can carry*.

The expression is printed as an arrow taking its free identifiers as
parameters, and the resolved values are passed as arguments:

```
const fonts = ['a','b'], sep = ', ';
fonts.join(sep)

  printed as   (fonts, sep) => fonts.join(sep)
  called with  [<value of fonts>, <value of sep>]
```

Chosen over registering names on the engine because the engine is one leaked
instance per thread, shared across every file that thread compiles — a name
left behind or shadowed would be a cross-file correctness bug, and a test
already exists asserting nothing leaks between folds. Chosen over substituting
literals into the printed source because a large bound value would then be
reprinted and reparsed at every use site, and a value with no literal spelling
could not cross at all.

This ticket carries all the new machinery but admits **string receivers
only**, so its test surface is one prototype and one name table. Ticket 06
reuses it for arrays.

Performance is a constraint here, not a later pass. The guard must refuse on
everything answerable from syntax alone — the callee is not a member
expression, the method name is computed, the name is locale-sensitive —
*before* resolving any binding or converting any value. Only an expression the
guard intends to fold pays for resolution. The engine must still be built on
first use and never before.

A hazard recorded on issue 12 of the effort that shipped the fold, which
deferred itself pending an answer: a large bound value substituted into printed
source is the memory hazard the fold bounded, minus the bound. The transport
above answers it by not substituting — the value is passed as an argument, so
the printed text stays the size of the expression however large the value is.
What carries over is that a resolved value has its own size and nesting,
independent of the syntax that named it, so the guard's bounds have to apply to
what crosses the bridge and not only to what is printed.

**Blocked by:** 03, 04.

**Status:** resolved

- [x] All twenty-two string methods that fail on a named receiver today
      compile, and each produces the same declaration text and class name as
      the reference compiler
- [x] A string a theme reference already resolved to is a usable receiver; the
      unresolved reference itself never crosses the bridge
- [x] A locally-declared shadow of a global is still called rather than folded
- [x] A receiver that cannot be resolved refuses with a reason, and a call
      inside a dynamic style function is still left for the runtime
- [x] The string method name table is deleted
- [x] One guard walk answers both the receiver question and the argument
      question, so a shape accepted in one position cannot be refused in the
      other
- [x] Every refusal answerable from syntax runs before any binding is resolved
- [x] Input with no foldable call still builds no engine
- [x] A resolved value's own size and nesting are bounded when it crosses the
      bridge, not only the printed expression's — a deeply nested or very large
      bound value refuses rather than being converted
- [x] The benchmark from 01 is re-run and the change in cost is recorded

## Note from ticket 02/04

Two things this ticket now inherits, both found in review of that work.

**The guard settles candidacy before it applies a rule.** Its checkbox "Every
refusal answerable from syntax runs before any binding is resolved" is the
opposite ordering, and both are right at different times. Today the walk
resolves nothing, so deciding candidacy first is free and prevents a rule
answering for a call that was never the fold's — which would stop it reaching
the dispatch that still folds `Math` and the callable globals. The moment this
ticket makes the walk resolve bindings, that ordering starts costing resolution
on calls the guard means to refuse, and the cheap name-only rules
(locale-sensitive, numeric-literal receiver, amplification) have to move back in
front of the walk. `admit_call` carries a comment saying so.

**Depth is answered before candidacy** and is left that way deliberately: a
chain past the ceiling reads the same sentence whichever guard answers it,
because the evaluator's own ceiling is the same number with the same words. If
this ticket splits the walk, that stays true only while the two numbers agree —
which ticket 11 is about.

## Answer

### The transport

`engine_fold` prints a fold that resolved names as an arrow over them and calls
it with their values: `s.toLowerCase()` becomes `(s)=>s.toLowerCase()` applied to
the string `s` holds. A fold that resolved nothing is still evaluated directly —
the first draft used the arrow for both and the benchmark priced that at +44% on
the cheapest leg, which the Measured section below sets out.

The parameter is the **author's own name**, which is what keeps the whole change
free of tree rewriting: the printed arrow is the expression verbatim under a
parameter list. It also settles shadowing for free — a callback parameter of the
same name shadows the transported value in the printed arrow exactly as it does
in the module, so `['a'].map(x => x + 'b').join('') + x` folds to `abZ` and not
to `aZ` or `abb`.

Values cross as arguments rather than as substituted literals. That answers the
hazard issue 12 of the shipping effort deferred on: the printed text stays the
size of the *expression* however large the value is. What still needs a bound is
the value itself, since it is copied into the engine — bounded by the same
number that bounds a folded string on the way out, counted in UTF-16 code units
so both sides measure the same thing, and named for the binding rather than the
method because the size belongs to what the name holds.

It also makes the printed source immune to the value's content. A quote, a
backslash, a newline, a backtick, a `${`, an unbalanced parenthesis and a NUL all
fold exactly, each to the reference compiler's own class name, because none of
them is ever text.

### What crosses

A resolved value crosses only as a **string**, which is this ticket's own bound,
and the inward size limit is on the running total of what a fold carries rather
than on each value — eight names each just under it would otherwise copy eight
megabytes into the engine.
Every other shape the evaluator answers — array, plain object, number, function
configuration, unresolved theme reference — is refused as "not a candidate" and
handed back, so the dispatch below keeps answering for it. That is also the
nesting bound: a string is one level deep by construction and anything nested is
refused for not being one. Ticket 06 widens it to arrays, 08 to numbers.

A theme reference therefore crosses only as the `var(--…)` string it already
resolved to, never as the reference.

Resolution goes through `evaluate_cached`, so the fold reads the binding every
other position reads — including the disqualifications that live there. A
reassigned binding, one mutated in place, and one read above its own declaration
all answer nothing here because they answer nothing there, which is why ticket
03's behaviour survived a named receiver reaching the guard with no rule of its
own.

### Ordering, and the one rule that stayed behind the walk

The cheap name-only rules moved in front of the walk, as the note asked:
locale-sensitive method, numeric-literal receiver, amplification. Two more joined
them because they are answerable without evaluating anything — the callee's shape
and, new here, whether the receiver names an unshadowed global the older dispatch
owns.

That last one is what keeps `Math` and `Object` reaching the dispatch that folds
them, and it is asked of the binding table rather than of the name alone.
Measured, the reference compiler folds `const String = 'AbC'; String.toUpperCase()`
to `ABC` — it resolves the binding — so treating the name as the global would
have refused input it compiles. Unshadowed, `Math.round(1.5)` still folds through
the older dispatch and matches upstream's class.

The **escaping-property** check on the method name deliberately did *not* move.
It costs nothing either way, but in front of the walk a chain of escaping reads
is refused outermost-first, so `''.constructor.constructor('return 1').call()`
would be named for its `call` instead of for the `constructor` that is the whole
of the reason. The resolution it costs is one binding on a call already certain
to refuse.

### A speculative read leaves no trace

Resolving a name to decide whether a fold is *possible* is not the same as
folding, and the evaluator had no way to say so. A failed read set
`confident = false` and wrote a `resolved: false` entry into the per-file memo;
the dispatch below then re-evaluated the same name, hit that entry, and reported
`Could not resolve the code being evaluated.` where it had reported
`Referenced constant is not defined.`

`StateManager::speculating` is the fix, and is deliberately shaped after the
`depth_refused` flag beside it: a refusal raised under it is not the subtree's
answer, so the memo withholds it. Values that resolve are still memoized — only
the refusal is withheld, because only the refusal is the speculation's own.
`Reader::resolve` saves and restores the flag rather than clearing it, so a fold
reached from inside another guard's read stays a speculation for as long as that
read lasts.

### The name table is deleted

`StringJS`, its `TryFrom`, its unit test, `CallbackType::String` and both
hand-written method arms are gone. `stylex_utils::char_code_at` and
`char_code_at_f64` went with them — the deleted `charCodeAt` arm was their only
caller — and the `utf16_length` cross-check that used one was rewritten to read
the code units directly, so a live function keeps its coverage.

The string-receiver arm of the older dispatch stays, as an unconditional refusal
that **names the method**. A string receiver reaching it means the fold declined,
and since the whole prototype surface folds there, what is left is a call whose
arguments hold something with no compile-time value. It asks the shared argument
evaluation first, which owns the spread sentence — so `'a'.concat(...['b'])` still
reads the same sentence a spread reads on every other callee, and
`'documentation'.startsWith(lowerQuery)` now names the undefined binding, which
is the same sentence its sibling `['a','b'].filter(Boolean)` earns.

Template literals joined the walk so deleting the table cost no fold:
`` `${s}px`.concat('!') `` folded through `concat` before and folds through the
engine now. That hands a hole's coercion to the engine where every other position
uses the hand-written one, so the two were measured against each other on the
seam they could part on — a number whose shortest spelling is exponential, and one
written with a trailing zero. Both compilers write `1e+21px`, `1e-7px` and
`1.5px`, and the case is pinned. Only a hole written as a *literal* gets there at
all: a named numeric hole has to be a carryable string first, and a number is not
one.

### Diagnostics that got better

Three sentences improved as a side effect and are re-pinned rather than
preserved. Message text is not a parity obligation — the comparison harness
compares class name, rule text and style-object shape — and in each case both
compilers reject the input either way.

- A mutated or reassigned binding read *through a call* now names the rule the
  binding broke (`Referenced value is not a constant.`) instead of the node the
  author wrote (`Unsupported expression: CallExpression`). The receiver is
  evaluated under a state of its own, so its reason was being dropped with that
  state; it is carried over now.
- `stylex.env.getTheme()` without the `env` option names the option. That case
  was pinned as it behaved rather than as it ought to, with a comment asking that
  an improvement report as a moved snapshot. This is that move.
- `'documentation'.startsWith(lowerQuery)` names the undefined binding.

### Known divergences, each measured

- A resolved amplification count is still not read: `const n = 3;
  'x'.repeat(n)` refuses where upstream folds `xxx`. Upstream also folds
  `'x'.repeat(200000000)` — measured, a two-hundred-megabyte rule — which is why
  the bound wants the count written. Ticket 12 owns closing the small divergence
  without reopening the large one.
- A member receiver whose object is a name is still handed back:
  `const o = { a: '1' }; o.a.toUpperCase()` folds upstream and refuses here. It
  needs an object across the bridge rather than new machinery, so it closes with
  tickets 06/07.
- A named receiver holding a number or a boolean refuses — tickets 08 and beyond.
- An unpaired surrogate folds to the same declaration as upstream and to a
  different class name, which is the divergence already recorded for a value
  written out: upstream hashes the surrogate it still holds, this compiler hashes
  the replacement character the outward bridge substituted.

### Tests

`transform_stylex_create_test::named_string_receivers` is the new file, 18 cases
at the highest seam there is, every expectation measured from
`@stylexjs/babel-plugin` 0.19.0 under the same options: the 29 non-locale methods
and properties of `String.prototype` on a named receiver, the name in every
position the walk reaches, a name read twice, the shadowing callback parameter,
the theme-resolved receiver, the shadowed globals, the nine values that could not
be printed safely, CSS-shaped values, the surrogate round trip, and the refusals.

`engine_fold_tests::input_with_no_foldable_call_builds_no_engine` is the other
new one, and needed two test-only readers on the module — the claim is about the
slot being *empty* after an input the guard declined, so it has to start from a
thread holding no engine. Six inputs, one per way a call leaves the guard.


### Measured — Apple M1 Max, `aarch64-apple-darwin`

Criterion defaults, median of 100 samples, the benchmark from 01 unchanged. Its
legs all have written-out receivers, so what they price is what the transport
costs a fold that resolves **nothing** — which is every expression that folded
before this work, and so the number that matters for not making the common case
worse.

Reading it needs the `engine/` leg beside each `fold/` leg, not the `fold/` leg
against 01's column. The `engine/` legs are unchanged code and still moved 4-8%
between sessions, so absolute numbers carry machine drift; the gap between the
pair is measured within one run and does not.

| Leg | `fold` | `engine` | Round trip adds | 01 recorded |
| --- | --- | --- | --- | --- |
| string | 4.01 µs | 2.27 µs | 1.74 µs | 1.64 µs |
| callback | 15.63 µs | 11.43 µs | 4.20 µs | 4.07 µs |
| chain | 9.20 µs | 6.31 µs | 2.89 µs | 2.72 µs |
| array-answer | 8.27 µs | 5.42 µs | 2.85 µs | 2.66 µs |

Cold start 117.14 µs against 113.09 µs, `p = 0.54` — no change, and the same
drift the engine legs show.

So the fold's own overhead is up **0.10-0.19 µs**, 4-6% of the overhead and
1-2% of a fold. That is the guard doing more before it walks: the valid-callee
and binding-table question, and the reordered rules. No leg resolves a name, so
none of it is resolution.

#### The regression this caught, and why the transport branches

The first implementation printed *every* fold as an arrow and invoked it, on the
argument that one shape beats two paths. Measured, that cost **+44%** on the
string leg (3.92 → 5.64 µs), **+24%** on chain and callback, and **+21%** on
array-answer, while every `engine/` leg stayed flat — a function object and a VM
frame charged on top of the expression, to exactly the folds that gain nothing
from a transport.

`apply` and `print_fold` now branch on whether the transport carried anything: a
fold that resolved no name is evaluated directly, as before, and the arrow is
printed only when there is something to pass it. That is what the table above
measures. The branch is on one question, and both arms hand the same expression
to the same engine, so it is not the two-tables-that-must-agree this effort
exists to remove.

Left as a note for 13 rather than fixed here: **no leg prices a resolved name.**
Seeding a binding needs `StateManager::push_declaration`, which is `pub(crate)`,
and widening a crate internal for a bench is a worse trade than recording the
gap. Ticket 13 has to measure against 01's baseline too and needs the same
harness, so the leg belongs with it — its own checkbox already says the numbers
go into that ticket.
