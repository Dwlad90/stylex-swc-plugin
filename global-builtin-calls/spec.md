# Calls to the global built-ins are not evaluated

Status: ready-for-agent

Tracks GitHub issue
[#1253](https://github.com/Dwlad90/stylex-swc-plugin/issues/1253).

Upstream reference: `~/Projects/Facebook/stylex` @ `5f51b2444` (the v0.19.0
release commit).

## Problem Statement

An author writes `String(x)` around a style value — a common coercion when a
typed token reaches a place that wants a string — and the build fails:

```js
import * as stylex from "@stylexjs/stylex";
export const vars = stylex.defineVars({
  background: String("#fff"),
});
```

```
error: [StyleX] Only static values are allowed inside of a defineVars() call.
```

The same source compiles under `@stylexjs/babel-plugin` 0.19.0, which folds the
call at compile time and emits `:root, .x11h0rru{--x1yidaq2:#fff;}`. Inside
`stylex.create()` the same input fails differently, as
`[StyleX] <key> > <prop> > Unsupported expression: Unknown`.

The reporter has 30 files that fail to compile against this compiler; 18 of them
fail for this one reason. Because the divergence is a hard error rather than
wrong output, a codebase cannot be migrated at all until it is fixed — there is
no partial adoption.

## Solution

Calls to the JavaScript global built-ins — `String`, `Number`, `Object`,
`Array` — are folded during evaluation, exactly as the reference implementation
folds them, so any module that compiles against it compiles here and produces
byte-identical CSS.

The predicate that recognises these callees already exists and already matches
upstream: a **valid callee** is one of `VALID_CALLEES` with no binding in scope,
so a locally-shadowed `const String = () => 'x'` is still treated as an ordinary
function and never folded. The gap is only that recognising one currently leads
nowhere — the branch is an explicit no-op comment. This spec fills that branch
in and supplies the ES coercions it needs.

Two things fall out of the same work:

- A **callable global** used in a way JavaScript rejects — bare `Math(x)`, which
  is not a function — fails the build with a diagnostic rather than compiling.
- Declarations whose value is empty or whitespace-only are dropped rather than
  emitted as `color:`, which is invalid CSS no browser accepts.

## User Stories

1. As a StyleX app developer, I want `String(x)` inside `defineVars` to compile,
   so that I can coerce a typed token without hand-inlining its value.
2. As a StyleX app developer, I want `String(x)` inside `create` to compile, so
   that the same coercion works wherever I write styles.
3. As a StyleX app developer, I want `String(x)` inside `createTheme` to
   compile, so that theme overrides accept the same expressions as the token
   definitions they override.
4. As a StyleX app developer, I want `String(x)` inside `keyframes` to compile,
   so that animation steps are not a special case I have to remember.
5. As a StyleX app developer, I want `String(x)` in a computed style key to
   compile, so that a coerced property name behaves like a literal one.
6. As a StyleX app developer, I want `String(x)` inside a nested value object
   (`{ default: …, ':hover': … }`) to compile, so that conditional styles accept
   coercions at every branch.
7. As a StyleX app developer, I want `String(param)` inside a dynamic style
   arrow function to keep compiling to a CSS custom property, so that a runtime
   value is not mistaken for a foldable one.
8. As a StyleX app developer, I want `Number('10')` to fold to `10`, so that a
   numeric token written as a string produces the same rule as the number.
9. As a StyleX app developer, I want `Number('0x1f')` to fold to `31`, so that
   the compiler reads numeric strings the way JavaScript does rather than the
   way Rust does.
10. As a StyleX app developer, I want `Number(' 10 ')` to fold to `10`, so that
    incidental whitespace does not silently become `NaN` in my stylesheet.
11. As a StyleX app developer, I want `Number('inf')` to be `NaN` rather than
    infinity, so that a Rust float spelling JavaScript rejects does not leak
    into my CSS.
12. As a StyleX app developer, I want `Array(a, b)` to fold to a style array, so
    that a generated list of fallback values compiles.
13. As a StyleX app developer, I want `Object(x)` to behave as it does under the
    reference implementation, so that an incidental wrapper call neither changes
    my output nor breaks my build in a new way.
14. As a StyleX app developer, I want a locally-declared `String` to shadow the
    global, so that the compiler never folds a call to _my_ function.
15. As a StyleX app developer, I want nested calls like `String(String(1))` to
    fold, so that coercions compose.
16. As a StyleX app developer, I want a coercion of a value the compiler cannot
    know to deopt rather than produce a wrong value, so that I get an error
    instead of a stylesheet that is quietly incorrect.
17. As a StyleX app developer, I want `Math(x)` to fail with a diagnostic naming
    the real problem, so that I am not handed a stack trace from inside the
    compiler.
18. As a StyleX app developer migrating an existing codebase, I want the class
    names and rule metadata to match the reference implementation exactly, so
    that server-rendered and client-rendered output can be mixed during a
    gradual migration.
19. As a StyleX app developer, I want an empty style value to produce no
    declaration, so that my stylesheet stays valid CSS.
20. As a compiler maintainer, I want the ES coercions to live in one place with
    their own tests, so that a future divergence has an obvious home.
21. As a compiler maintainer, I want a single copy of the evaluator predicates,
    so that a change to the set of foldable callees cannot land in one copy and
    miss the other.
22. As a compiler maintainer, I want the decision to hard-error rather than
    deopt recorded in the domain glossary, so that the next person to read the
    `Object(…)` branch does not mistake a deliberate choice for an oversight.
23. As a reviewer, I want the coercion work reviewable without reading any
    evaluator code, so that its correctness can be checked against the language
    specification alone.

## Implementation Decisions

### Where the fold happens

The call-expression node of the evaluator gains a real body for the
already-existing valid-callee branch. It builds a function configuration and
hands it to the same apply-site the member-call built-ins (`Math.pow`,
`String.prototype.concat`) already use, so **argument evaluation and the
confidence check** are shared rather than duplicated. A spread argument continues
to deopt, matching upstream. Surplus arguments are ignored — `String(1, 2)` is
`"1"`.

**Spread rejection is not shared, and cannot be** (corrected after
implementation; the paragraph above originally claimed it was). The shared
argument evaluation reads *through* a spread to its operand, which would fold
`String(...['a','b'])` to `"a,b"` — a wrong value, not a refusal. So the callable
global rejects a spread before reaching the apply-site. It also uses the spread
element's own wording rather than the member built-ins' `SPREAD_NOT_SUPPORTED`,
because those reject a spread they could otherwise have used, which is a
different complaint from one that is unknowable.

### Naming the callables

A new enum in the JS-enums crate names the four foldable globals, in a single
type distinct from the existing per-type method enums. The method enums map
_method names_ through `TryFrom<&str>`; the callable global is a different
concept and does not belong in them. The callback type used by function
configurations gains one variant carrying this enum.

### Where the coercions live

The orphaned JS-semantics crate — purpose-built for "predicates over JavaScript
semantics", documented with its own glossary, but until now depended on by
nothing — becomes the home for the ES coercions. Its charter widens from
predicates-only to predicates and coercions, recorded in its glossary. The
coercions go in their own module beside the predicates rather than into the
predicate file; the crate root declares both modules and re-exports neither, so
every import site says which kind of helper it is reaching for.

Taking that dependency for the first time puts two identical copies of the
evaluator predicates in scope at once — the valid-callee, invalid-method, and
mutating-method tests exist verbatim in both the crate and the evaluator. The
evaluator's copies are deleted and its call sites repointed. This is mechanical
and behaviour-preserving; the existing suite is the proof.

The crate also drops its dynamic-library artifact, which nothing loads. Every
crate in the workspace declares one and only the compiler entry point needs it,
but the repo-wide cleanup is out of scope here.

### Coercion semantics

`ToString` and `ToNumber` follow the language specification, because the
alternative is silently wrong CSS rather than a failed build:

- `ToNumber` over strings implements the numeric-literal grammar — hexadecimal,
  binary, and octal prefixes, leading and trailing whitespace, the empty string
  as zero, `Infinity` — and rejects the Rust float spellings (`inf`, `nan`) that
  JavaScript does not accept.
- `ToString` renders `null`, `undefined`, and `NaN` as their JavaScript
  spellings, joins arrays with commas rendering empty elements as nothing, and
  renders plain objects as `[object Object]`. It also renders the two literals
  whose string is neither a primitive spelling nor the object default: a big
  integer as its digits with no `n` suffix, and a regular expression as its own
  source text. Both were missing from the first implementation, which deopted on
  them while `ToObject` already classified a regular expression as an object —
  the two coercions disagreeing about which values they know. Added in review.

Values with no JavaScript counterpart in this evaluator are split by whether the
upstream value is an object or a function. Cross-file token references and
environment objects are objects upstream, so they stringify to
`[object Object]`. Callbacks and function configurations are functions upstream,
where `String(fn)` yields the function's source text; the evaluator does not
retain source, so these deopt rather than produce a confidently wrong value.

### Per-callee behaviour

- `String` — full `ToString`. Zero arguments yield the empty string, which is
  what the language says and what the reference implementation does.
- `Number` — full `ToNumber`. `NaN` is a legitimate result and flows into the
  value, as it does upstream.
- `Array` — a single numeric argument produces that many holes; two or more
  produce a list. Holes are represented as `undefined` — spelled
  `EvaluateResultValue::Null`, which is the evaluator's name for a confidently
  evaluated value that is *absent* rather than for JavaScript's `null` — which
  everything downstream already understands and which reaches the existing "a
  style array value can only contain strings or numbers" rejection. The
  sparse/dense distinction has no observable consequence here. A count past a
  documented budget is refused; see issue 05 for the sign-off.
- `Object` — `null` and `undefined` produce an empty object; an object argument
  is the identity. A primitive argument produces a boxed wrapper upstream, whose
  only observable effect is the existing "a style value can only contain an
  array, string or number" rejection, so it maps straight to that rejection
  rather than adding a wrapper representation to the evaluator's value type.
- `Math` — not callable. It is in the valid-callee set because its _methods_ are
  foldable, so a bare call reaches this branch and must be rejected. Upstream
  leaks a raw `TypeError` from inside its own evaluator; that is a defect, not a
  contract, so this compiler raises its own diagnostic. The observable contract
  — this program does not compile — is preserved.

### Empty values

An empty or whitespace-only declaration value produces no declaration, matching
what the reference implementation already does for `null` values and for empty
strings inside a style array. This is not evaluator work and is independent of
the fold; it is included because the compilers demonstrably diverge, this
compiler emits invalid CSS today, and the reference implementation's behaviour
in the remaining case is a null-dereference inside its value normaliser rather
than a decision worth copying.

## Testing Decisions

A good test here names an observable behaviour and would survive any rewrite of
how the fold is implemented: a module goes in, CSS and metadata come out. No
test should reach for a function configuration, a callback variant, or the
shape of an evaluated value — those are all implementation.

Two seams, both already in use, no new ones:

**Transform fixtures** — the highest seam available, carrying every parity
claim. Prior art is the per-API transform test directories, which already
cover `create`, `defineVars`, `createTheme`, `keyframes`, and value
normalisation in exactly this style. New cases cover each API named in the
issue, the computed-key and nested-value positions, the dynamic-arrow case that
must _not_ fold, the shadowed-global case that must not fold, and the failing
cases (`Math`, `Object` of a primitive, `Array` of a count) which assert the
diagnostic. The empty-value change is pinned in the value-normalisation
directory.

**Coercion unit tests** — beside the existing predicate tests in the
JS-semantics crate. Justified as a second seam only because `ToNumber`
correctness is a wide input/output matrix — hexadecimal, whitespace, `Infinity`,
empty, arrays, the rejected Rust spellings — and routing forty cases through
full transforms would be slow and would obscure which coercion failed. These
test pure functions over primitives, with no compiler types involved.

The expected values for both seams are taken from measured output of
`@stylexjs/babel-plugin` 0.19.0 resolved from `node_modules`, not from reading
its source. That comparison is a development instrument and is not committed:
this repo has no suite that shells out to the reference implementation, and
adding that dependency for one fix is not worth its maintenance.

## Out of Scope

- **Removing the dynamic-library artifact from the other crates.** All sixteen
  declare one; only the compiler entry point needs it, and one already has the
  line commented out. A repo-wide cleanup is unrelated to this issue and belongs
  on its own branch.
- **Committing a reference-implementation comparison harness.** Used during
  development, deliberately not added to the suite.
- **`new String(…)` and other construction forms.** Upstream folds call
  expressions only; construction is a different node and diverges identically in
  both compilers.
- **Widening the set of foldable callees.** The set matches upstream exactly and
  is not extended here.

## Further Notes

The predicate and the constant sets were already ported faithfully — the
valid-callee names, the invalid methods, and both mutating-method sets are
character-for-character identical to upstream. Only the branch body was left
empty, which is why the divergence looks larger than the fix.

Two upstream defects were found while measuring and are deliberately not
mirrored: the bare-`Math` call leaking a `TypeError`, and the empty-value
null-dereference inside the value normaliser. Both are noted above with the
reasoning for diverging.

`String()` with no arguments was initially read as an upstream crash. It is not:
it evaluates to the empty string correctly, and the crash observed inside
`create` reproduces with a plain empty string literal and no coercion anywhere.
That is the empty-value divergence, tracked separately in this spec.
