# The fold guard reads values, not syntax

Status: ready-for-agent

Upstream reference: `~/Projects/Facebook/stylex` @ `5f51b2444` (the v0.19.0
release commit), compared against `@stylexjs/babel-plugin` 0.19.0 resolved from
`node_modules`.

## Problem Statement

An author gives a value a name and the build stops compiling it.

```js
import * as stylex from '@stylexjs/stylex';

const fonts = ['Inter', 'sans-serif'];

export const styles = stylex.create({
  x: { fontFamily: fonts.join(', ') },
});
```

That compiles. Change `join` to any other method and it does not:

```
const s = 'ABC'; …s.toLowerCase()
  → [StyleX] x > color > The method 'toLowerCase' is not yet
    supported in static evaluation.

const a = ['1px','solid']; …a.concat(['red']).join(' ')
  → [StyleX] x > borderTop > Unsupported expression: CallExpression
```

Written out in full, both of those compile here today. The only thing that
changed is that the value was given a name — which is what an author does the
moment two rules share it.

Measured against the reference compiler, in-process, on the current build: **35
methods compile under `@stylexjs/babel-plugin` and fail here purely because the
receiver is a binding.**

| receiver       | folds today                     | fails today                                                                                                                                                                                 |
| -------------- | ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Array binding  | `join`; `map`/`filter` at the first link only | `concat` `slice` `flat` `indexOf` `at` `find` `findIndex` `reduce` `every` `some` `toString`                                                                                    |
| String binding | `concat`, `charCodeAt`          | `toLowerCase` `toUpperCase` `trim` `trimStart` `trimEnd` `padStart` `padEnd` `slice` `substring` `substr` `charAt` `codePointAt` `repeat` `replace` `replaceAll` `split` `indexOf` `lastIndexOf` `at` `normalize` `toString` `valueOf` |

Three surfaces fail even with no binding anywhere, because `Math` and `Object`
are themselves identifiers and the guard refuses an identifier:

- `Math.trunc(1.5)` fails with fully literal arguments. The entire `Math`
  surface this compiler accepts is seven names.
- `Object.keys({a: 1}).join(',')` fails with a literal object.
- `Number.prototype` is not reachable in either position — neither
  `(255).toString(16)` nor `const n = 255; n.toString(16)`.

Three more fail because this compiler refuses what the reference compiler
folds: `['b','a'].sort().join(',')`, `['a','b'].push('c')`, and
`'b,a'.split(',').sort().join(',')`.

And one is simply wrong: `a.filter(v => v).join('-')` reports
`[StyleX] Value in not a number: a`, because the filter callback decides
truthiness by converting its result to a number.

Because a class name is a hash of the declaration text, each of these is worse
than a failed build for anyone migrating: where the two compilers both produce
output and disagree, server-rendered markup names a class the client stylesheet
never defines, and nothing errors.

## Solution

Every call to a native JavaScript function is evaluated by the embedded engine,
whatever its receiver is written as. The name tables are deleted.

A [refused fold](../../crates/stylex-transform/CONTEXT.md) remains a refusal —
but only for reasons the compiler can state. Four of them, where this said two
before the work found the others, each recorded on the ticket that added it and
in ADR 0008: a method whose answer needs locale data the engine does not carry;
two ceilings on how much memory one fold may be asked to allocate; a length a
call declares but the guard cannot read without evaluating it; and a read that
escapes the value it is written on — `constructor`, `call`, `apply`, `bind`.
Everything else the reference compiler folds, this compiler folds, to the same
declaration text and the same class name.

Three things fall out of the same work:

- A value that is mutated after its declaration disqualifies the **binding**,
  as it does upstream, rather than the **method** being refused wherever it
  appears. That is both the parity fix and one fewer rule.
- A fold that refuses says which rule refused it, instead of falling through to
  `Unsupported expression`.
- The depth at which a nested expression stops folding becomes the project's
  configured evaluation depth rather than a second, lower number that exists
  only because the engine ran on a small stack.

## User Stories

1. As a StyleX app developer, I want `fonts.join(', ')` to compile when `fonts`
   is a `const`, so that naming a value does not change whether it compiles.
2. As a StyleX app developer, I want every `String.prototype` method to fold on
   a named string, so that I do not have to learn which two of them work.
3. As a StyleX app developer, I want every `Array.prototype` method to fold on
   a named array, for the same reason.
4. As a StyleX app developer, I want `Object.prototype` methods to fold on a
   named object, so that object values are not a special case.
5. As a StyleX app developer, I want `Math.trunc(1.5)` to fold, so that the
   `Math` methods that work are not an arbitrary set of seven.
6. As a StyleX app developer, I want `Object.keys(o)` to fold and to be
   chainable, so that a key list can become a declaration value.
7. As a StyleX app developer, I want `Object.keys(o).join(',')` to fold, so
   that a fold's result is as usable as a value I wrote out.
8. As a StyleX app developer, I want `const n = 255; n.toString(16)` to fold,
   so that `Number.prototype` is reachable at all.
9. As a StyleX app developer, I want `(1.5).toFixed(1)` to keep failing, so
   that a module rejected by the reference compiler is rejected here too.
10. As a StyleX app developer, I want a chain to fold at every link when a
    middle link is a binding, so that `a.concat(b).join(' ')` compiles.
11. As a StyleX app developer, I want `a.map(f).join(',')` to fold, so that a
    mapped list can be joined without the chain dying at the second link.
12. As a StyleX app developer, I want `a.filter(v => v).join('-')` to fold to
    the same value the reference compiler produces, so that a truthy filter is
    not read as a number.
13. As a StyleX app developer, I want a callback with a block body to fold, so
    that adding a statement to an arrow does not break the build.
14. As a StyleX app developer, I want a callback with destructured parameters
    to fold, for the same reason.
15. As a StyleX app developer, I want a callback that reads a named value from
    the surrounding module to fold, so that a callback is not restricted to its
    own parameters.
16. As a StyleX app developer, I want `['b','a'].sort().join(',')` to fold, so
    that sorting a list I wrote out produces a declaration.
17. As a StyleX app developer, I want `'b,a'.split(',').sort().join(',')` to
    fold, so that sorting an intermediate value works.
18. As a StyleX app developer, I want a binding that is mutated anywhere in the
    file to stop folding entirely, so that this compiler agrees with the
    reference compiler about which values are constant.
19. As a StyleX app developer, I want a binding read *before* it is mutated to
    stop folding too, because that is what the reference compiler does and a
    disagreement would emit a class the other build does not define.
20. As a StyleX app developer, I want a reassigned `let` to stop folding, so
    that reassignment and mutation are treated alike.
21. As a StyleX app developer, I want a token read from another file to be
    usable as a method receiver, so that `tokens.color.trim()` works once the
    token has resolved.
22. As a StyleX app developer, I want a locale-sensitive method to refuse
    rather than fold, so that my stylesheet never carries a value computed
    against the wrong locale.
23. As a StyleX app developer, I want `const n = 5; 'x'.repeat(n)` to fold, so
    that a repeat count does not have to be written as a literal.
24. As a StyleX app developer, I want `'x'.repeat(9e9)` to refuse with a
    message naming the limit, so that I can see why and raise it if I mean it.
25. As a StyleX app developer generating styles, I want the two allocation
    ceilings to be project options, so that a project that really produces
    large folds is not blocked by a default sized for hand-written styles.
26. As a StyleX app developer, I want a deeply nested expression to fold up to
    the evaluation depth I configured, rather than a second lower limit I never
    set.
27. As a StyleX app developer, I want a fold that refuses to name the rule that
    refused it, so that I am not handed `Unsupported expression` and left to
    guess.
28. As a StyleX app developer, I want a refusal inside a dynamic style function
    to keep leaving the call for the runtime, so that this change does not turn
    working runtime code into a failed build.
29. As a StyleX app developer, I want a locally-declared `String` to keep
    shadowing the global, so that the compiler never folds a call to my own
    function.
30. As a StyleX app developer migrating an existing codebase, I want class
    names and rule metadata to match the reference compiler exactly, so that
    server-rendered and client-rendered output can be mixed.
31. As a StyleX app developer, I want a build that would otherwise exhaust
    memory or overflow a stack to report a diagnostic instead of dying, so that
    the rest of my build still finishes.
32. As a compiler maintainer, I want the set of foldable methods to stop being
    a list I maintain, so that the method nobody wrote down is not the next bug
    report.
33. As a compiler maintainer, I want one guard walk serving both the receiver
    position and the argument position, so that a shape accepted in one cannot
    be refused in the other.
34. As a compiler maintainer, I want one value bridge in each direction, so
    that the two array dispatch arms that disagree today collapse into one.
35. As a compiler maintainer, I want the engine built only when a file actually
    folds something, so that input with no foldable call pays nothing.
36. As a compiler maintainer, I want a printed expression parsed once however
    many times it is folded, so that a file with a thousand style objects does
    not reparse the same source a thousand times.
37. As a compiler maintainer, I want the guard's cheap refusals to run before
    any binding is resolved, so that a refused fold costs nothing it does not
    have to.
38. As a compiler maintainer, I want the engine's cost measured by a benchmark
    that fails if the fold did not happen, so that a fold that stopped working
    cannot be reported as an improvement.
39. As a compiler maintainer, I want a generated sweep across the prototype
    surface compared against the reference compiler, so that coverage is proved
    rather than curated.
40. As a compiler maintainer, I want every remaining divergence to carry a
    written reason, so that the next person can tell a decision from an
    oversight.
41. As a compiler maintainer, I want the two remaining coercion implementations
    checked against each other, so that the one the operators use cannot drift
    from the one the engine uses.
42. As a compiler maintainer, I want the reason the engine is a permanent
    dependency written down, because the commit that introduced it says it was
    a throwaway and that commit is on the default branch.

## Implementation Decisions

### The line: the engine owns the call

The engine evaluates **calls to native JavaScript functions** — every prototype
method on any receiver, the `Math` and `Object` statics, and the callable
globals.

One thing that is not a native function moves with them, and only one.
`firstThatWorks` answers from its arguments alone, touching no compiler state,
so a callback may call it — `a.map(x => firstThatWorks(x, 'serif'))` is one
JavaScript call per element, which the engine can run and this compiler cannot
reach into. It travels as a function built over the same shared core the
evaluator's own path calls, so the two cannot come to answer differently. Every
other StyleX function stays where it was: the fold hands its call back, and the
dispatch behind the fold calls it as it always has. See ticket 17.

Operators, template literals, `typeof`, `void` and the unary forms stay in
Rust. So do the short-circuiting forms, and that exclusion is load-bearing
rather than incidental: the reference implementation evaluates each side of a
logical expression in a **cloned** evaluation state, so a short-circuited dead
branch may fail to evaluate while the whole expression stays
[confident](../../crates/stylex-transform/CONTEXT.md). An engine handed the
whole expression cannot fork confidence — one unresolvable leaf would refuse
the whole subtree and turn a compiling module into a failing one. Their
operands still reach the engine individually.

Everything StyleX-specific stays where it is: binding resolution, the
[reference resolution chain](../../crates/stylex-transform/CONTEXT.md), import
resolution, [theme references](../../crates/stylex-transform/CONTEXT.md), the
injected function map, the environment object, code frames, and the
confident/deopt machinery.

### Transport: an arrow called with values

The [fold guard](../../crates/stylex-transform/CONTEXT.md) stops asking whether
an expression is *written out* and starts asking whether its every leaf
*resolves to a value the bridge can carry*. The expression is printed as an
arrow taking its free identifiers as parameters, and the resolved values are
passed as arguments:

```
const fonts = ['a','b'], sep = ', ';
fonts.join(sep)

  printed as   (fonts, sep) => fonts.join(sep)
  called with  [<value of fonts>, <value of sep>]
```

Chosen over registering globals on the engine because the engine is one leaked
instance per thread, shared across every file that thread compiles: a name left
behind or shadowed would be a cross-file correctness bug, and there is already
a test asserting no state leaks between folds. Chosen over substituting
literals into the printed source because a large bound value would then be
reprinted and reparsed at every use site, and because a value with no literal
spelling could not cross at all.

### Performance

This is the first constraint, not a later pass.

- **The engine is built on first use and never before.** Input with no foldable
  call pays nothing. This holds today and must survive; the measured cold start
  is ~240 µs and a warm fold ~3.4 µs.
- **A printed arrow is compiled once per distinct source.** The compiled
  function is memoised on the engine's thread-local beside the engine itself,
  keyed by the printed text. A file with a thousand style objects sharing one
  expression shape parses once. This is the single largest lever, because
  printing and parsing dominate a warm fold.
- **The guard refuses before it resolves.** Everything answerable from syntax
  alone — the callee is not a member expression, the method name is computed,
  the name is locale-sensitive — is checked before any binding is resolved or
  any value is converted. Only an expression the guard intends to fold pays for
  resolution.
- **The existing evaluation memo is not bypassed.** A fold result is already
  memoised per file by a structural hash of the subtree; the new path returns
  through the same memo.
- **The expression is cloned once to be printed, and only once.** The printer
  drops spans in place before emitting, so it needs an owned tree and a
  reference will not do. What this rules out is the second copy: going through
  `create_module`, which takes a reference and clones again inside, cost two.
- **Both configurations are priced.** A production/development fixture pair is
  registered for a shape that compiles on the merge base as well as on this
  branch, so the paired comparison has something to compare. See Testing
  Decisions for why a fixture for the *new* capability must not be registered.

### DRY

- **One guard walk** answers both the receiver question and the argument
  question, as it does today, so a shape accepted in one position cannot be
  refused in the other.
- **One bridge in each direction.** The outward bridge answers the evaluator's
  own value type rather than a bare AST node. That single change collapses the
  two array dispatch arms that disagree today — one accepts `join` for a value
  the evaluator produced, the other refuses it for the array literal a fold
  produced, which is why a mapped list cannot be joined. The arms disagree
  about the shape of the context, not about the names, and one value type
  removes the disagreement rather than reconciling it.
- **One set of predicates.** The mutating-method and valid-callee predicates
  already live in a single crate; nothing here reintroduces a second copy.
- **Two coercion implementations remain, deliberately, and are checked against
  each other.** The callable globals move to the engine, so the hand-written
  coercions shrink to what the operators still call. A differential test
  asserts the two agree over the existing input matrix.

### KISS

- Five method-name enumerations, one native-function module and one callable
  global dispatch are **deleted**, not deprecated.
- No feature flag and no fallback path. The engine already ships
  unconditionally on the default branch, and a second path is the drift this
  work exists to end.
- One refusal type. A fold answers either a value or a refusal that names its
  rule; there is no third state.

### Refusals after the change

**The mutating-method refusal is removed.** Measured, the reference compiler
does not refuse mutating methods — it folds them on any receiver not reachable
by name, and instead disqualifies the **binding**. Its mutation test walks a
binding's references with no position check, so the binding is dead for the
whole file in both directions: a read *before* the mutation stops folding too.
The engine therefore only ever mutates a temporary that nothing can name
afterwards, which is unobservable, so the purity objection that justified the
refusal does not survive the measurement. Parity here belongs to the binding
resolution that already exists; this work verifies it matches on every measured
shape and pins each.

Note for the implementer: the commit that added this refusal is on `develop`.
This reverses it deliberately, with the measurement above as the reason.

**Locale-sensitive methods stay refused** — the one remaining category where
the reference compiler compiles and this compiler does not. Four measured
reasons:

1. The engine's internationalisation feature reintroduces the exact dependency
   conflict that forced the engine to be vendored. The vendored tree relaxes
   two version bounds; the feature pins roughly eleven more against the line
   the rest of the workspace is on.
2. That feature carries no compiled locale data and the provider crate is not
   vendored, so building a context would fail at runtime without a data blob
   this project would have to ship itself.
3. It would not fix the locale-formatting method on numbers regardless: the
   engine ignores that method's arguments unconditionally, with no feature gate
   at all.
4. With no locale argument the reference compiler takes the host's default
   locale, so its own answer is machine-dependent. Folding it would pin one
   machine's value into a stylesheet.

**Nesting is solved rather than bounded.** The fold's own nesting ceiling
exists because the engine's parser recurses on the bare thread stack and
overflows around a hundred levels, aborting the process from inside an
evaluation whose whole contract is that it may fail. The evaluator's own
descent already runs inside a growable stack — that is a recorded architectural
decision — and the fold copied its *number* without its *mechanism*. The
evaluation runs inside the same growable stack, the fold's separate ceiling is
deleted, and the limit becomes the project's configured evaluation depth.
Verify empirically at the depths the existing tests already probe before
relying on it.

**The two allocation ceilings become configuration**, following the existing
precedent for evaluation depth: a project option with a machine-level override,
defaults sized for hand-written styles. They remain because the engine bounds
loop iterations, recursion and stack but not allocation — growth inside a
native builtin is not a counted loop, and one measured typo reached 5.37 GB
resident. Both defaults are re-derived and each is stated in terms of what it
costs, because the current numbers were chosen against a guard that required
the count to be written into the source.

**The amplification bound reads the resolved count.** Requiring a written
literal refuses a trivially safe bound count today, which is a divergence in
its own right. The companion rule refusing an amplifying call whose receiver is
itself a call needs rechecking, since a receiver can now be a resolved value.

**The written-numeric-receiver refusal stays.** The reference compiler throws
on a method call against a numeric literal, so both compilers must reject it.
Only a *bound* numeric receiver folds.

### Diagnostics

A fold answers a refusal carrying its reason, and the caller raises it. Message
text is **not** a parity obligation — the comparison harness compares class
name, rule text and style-object shape, never message text, and it already has
a verdict for "both reject, different message". Where this compiler's message
is better than the reference compiler's, it stays better.

### The bridge's contents

Inward: literals, arrays and plain objects, and — amended after the sentence
below was written — a **theme reference**, as a stand-in the engine reads members
off. A group stores no members: every name is derived from the group's identity
as it is read, which is why the identity is what crosses and a proxy over it is
what the engine holds — the same arrangement the reference compiler holds a group
in, with one Rust function deriving the name on both sides of the bridge. The
whole of `Array.prototype` folds on it, and so does a member read at any depth of
the expression: `[colors.glow, '0 0 1px'].join(' ')`, `[colors].join('-')`,
`String(colors)`, a member reached off an element, a computed key a callback is
handed, and a dotted token path were all measured identical to
`@stylexjs/babel-plugin` 0.19.0, with identical class names.

The one thing a stand-in cannot work out for itself is that a chain of two or
more names is a single token — `colors.brand.primary` names `brand.primary` —
because that is a question about the source rather than about any value. The fold
guard reads those paths off the source and names them for the group, from the
same two helpers the dispatch below reads them with.

What is still handed back is the one answer that *is* the group again, such as
`Object(colors)`: the group's members live in the other file and no expression
this side writes stands for it, so the dispatch below — which holds the reference
— answers for it.

The environment object and the injected function map still do not cross: they
are this compiler's own values with no JavaScript form at all, and resolving one
is what mutates compiler state, which happens before the bridge.

They are handed back rather than refused, which is the rule the whole bridge
rests on — a shape it does not carry belongs to the dispatch below, and the
dispatch answers for it. A conversion applied to one is answered there, through
the same coercions `+` and a template literal use, so a theme reference cannot
read one way in `String(x)` and another in an interpolation.

Outward: primitives, arrays, and plain objects. Objects crossing back is what
makes a static-method result chainable at all; it needs a size bound matching
the array bound and a key-order rule checked against the own-key ordering the
object evaluation already implements.

Refused in both directions, as stated rules rather than omissions: function
configurations, callbacks, the environment object, an unresolved theme
reference, and the AST-keyed map variants. A *resolved* theme reference crosses,
under the rule above.

### Callbacks

An arrow whose free variables all resolve to carryable values is printed into
the same transport and runs as a real JavaScript function. Block bodies,
destructuring and closures over named values become free, because the engine
parses them rather than the guard analysing them. An arrow touching a StyleX
function or a dynamic parameter keeps the existing Rust closure path, which
dynamic and inline styles depend on and which is not touched here.

### Glossary and decision record

Two glossary entries assert the guard reads syntax and record the
binding-receiver gap as known; both are rewritten as the terms settle, not
batched at the end. A new architectural decision record states why the guard
now reads values, and — because the commit that introduced the engine describes
itself as a throwaway not for merging, and was merged — states plainly why an
embedded JavaScript engine is a permanent dependency: what it costs in artifact
size and lock file entries, why it is vendored rather than taken from the
registry, and why its instance must be leaked per thread rather than dropped.

## Testing Decisions

A good test here names a behaviour an author can observe: a module goes in, CSS
and metadata come out. No test should reach for the shape of an evaluated
value, a function configuration, or which internal path answered — all of those
are implementation, and all of them are being deleted or rewritten.

Four seams, three of them already in use, and one new one justified below.

**Transform fixtures** — the highest seam available and the one carrying every
parity claim. The existing per-API directories already cover the surfaces in
exactly this style. New cases: each of the 35 methods on a bound receiver, the
three static surfaces with literal arguments, the chain cases, the callback
shapes, the three mutating-receiver shapes that must now fold, the five
binding-disqualification shapes that must deopt, the shadowed global that must
not fold, and the dynamic-arrow case that must stay a runtime value. Expected
values are measured from the reference compiler, not read off its source.

**The parity corpus** — the existing side-by-side harness against the reference
compiler, which is the gate rather than an extra. Every entry pinned against
the syntactic guard is re-measured; entries that flip from divergent to
identical are each a claim needing its own row. The harness gains one rule: an
entry recording that the reference compiler compiles where this one refuses
must carry a written reason, and a row without one fails.

**Coercion unit tests** — the existing matrix beside the predicates, extended
with the differential test asserting the hand-written coercions agree with the
engine. Justified as a second seam only because this is a wide input/output
matrix over pure functions with no compiler types involved, exactly as the
existing matrix is.

**A generated prototype sweep** — the one new seam, and the only one that
proves the central claim. Crossing the reachable prototypes with both receiver
shapes against both compilers is what a curated corpus cannot do, because a
curated corpus is itself a table and the method nobody listed is the next bug
report. Prior art is the existing generated fuzz harness, which crosses an
alphabet with itself and runs nightly rather than per commit for exactly this
reason: a surface defect appears when the surface changes, not on every push. A
curated row is where a *reason* gets written down; the sweep is what finds the
method nobody thought of.

**Benchmarks.** A production/development fixture pair is registered for a
receiver shape that compiles on the merge base too — confirm that empirically
before registering, because a fixture only this branch compiles fails the base
subject and takes the whole paired comparison down before a single measurement,
which is a failure the performance policy names by name. Shapes only this work
makes compilable are correctness cases and belong in the transform fixtures.
Separately, a criterion benchmark prices cold start, a warm fold, the compiled-
function memo and the value bridge; it has no base-compilation constraint. It
must run inside the global scope the transform requires and must fail unless
the fold produced the value it exists to time — a refusal, a deopt and a memo
hit are all fast, and a curve that flattens because the work stopped happening
is indistinguishable from a win.

## Out of Scope

- **Operators, template literals and `typeof`.** They are not calls. They keep
  their Rust implementations and their own coercions. They do *share* the
  character ceiling, which they did not when this was written: a string grown by
  `+` or by an interpolation is the same string and the same memory as one a
  fold builds, and bounding only the fold left the cheapest way to build a huge
  one unbounded. Tickets 20, 23 and 25; the implementations did not move.
- **Short-circuiting forms.** Excluded deliberately; the reasoning is in
  Implementation Decisions and it is a correctness argument, not a scheduling
  one.
- **Running a whole module in the engine.** Import resolution, theme
  references, the injected function map and the environment object stay in
  Rust. The engine is handed expressions, never modules.
- **Enabling internationalisation support.** Measured and rejected, with the
  reasons recorded so the next person does not re-derive them.
- **The Rust closure path for arrows.** Dynamic and inline styles depend on it;
  it is not replaced. Two call sites were adjusted where the fold now answers
  what they used to: an arrow binds a `Vec` result through the shared
  conversion, and the arm that applied a callback moved to the call it belongs
  to. The path itself still runs every case it ran before.
- **Changing how a refused fold behaves at the call site.** Whether an author
  sees a failed build or working runtime code still depends on where the call
  sat, exactly as recorded today.
- **Message-text parity with the reference compiler.** Position parity is the
  contract; the messages here are this compiler's own.

## Further Notes

The commit introducing the engine describes itself as a throwaway not for
merging, and is on `develop`. No throwaway code survives — the temporary
comparison script was deleted by the commit that shipped the fold — but that
sentence is currently the only written statement about why a large embedded
engine is in the default branch's dependency graph, and it says the opposite of
what is true. Replacing it is part of this work.

The glossary records the binding-receiver gap as "tracked, not intended", and
the commit that shipped the fold filed it as issue 12 of the effort that
shipped the fold, where it sits deferred and untriaged. Its measurements were
taken independently of the ones above and agree with them, and it reaches the
same conclusion: of the two ways to close the gap, folding on the resolved
receiver is the one worth pricing, because widening the tables rebuilds by hand
what the engine already answers.

It ends by refusing to start without an answer to one question — that a large
bound value substituted into printed source is the memory hazard the fold
bounded, minus the bound. This spec answers it by not substituting. The value
is passed as an argument rather than printed, so the printed text stays the
size of the expression however large the value is. What does carry over is that
a resolved value has its own size and nesting, independent of the syntax that
named it, so the guard's bounds apply to converted values and not only to
printed ones.

The glossary wording also understates the gap: "a narrower set" is three method
names against two whole prototypes, plus two surfaces that fail with no binding
involved at all.
