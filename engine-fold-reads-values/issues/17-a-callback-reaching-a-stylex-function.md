# 17 — A callback reaching a StyleX function

**What to build:** An array method whose callback names a StyleX function folds
again.

```js
import { firstThatWorks } from '@stylexjs/stylex';
const a = ['a', 'b'];

export const styles = stylex.create({
  x: { fontFamily: a.map(x => firstThatWorks(x, 'serif')).join(',') },
});
```

The reference compiler folds that to `serif,a,serif,b`, measured. This compiler
refuses it:

```
[StyleX] x > fontFamily > Cannot fold 'map' at compile time.
Its receiver or one of its arguments is not in a form the compiler can evaluate.
```

Neither compiler emits anything for the module, so nothing they produce
disagrees — but the reference compiler builds it and this one does not, which is
the migration case the effort exists to close.

Nothing is wrong with the fold guard here. `firstThatWorks` is not a JavaScript
function; it lives in this compiler's injected function map, so the guard
correctly declines to print a call to it and hands the call back. What is
missing is below: ticket 06 deleted the hand-written array methods when
`Array.prototype` moved into the engine, so no dispatch answers `map` any more,
and the arm reports the fold's refusal instead. Carrying the function map inward
was measured and rejected in ticket 07 — its values are placeholders the engine
would throw on, which would turn a compiling module into a failing one.

So closing this means running the callback per element outside the engine, over
the values the engine cannot hold. That is the arrow-to-Rust-closure path that
dynamic and inline styles already use, reached for a receiver rather than for a
style function — not a re-added method table.

Found while building ticket 10, whose checklist named the shape; the divergence
predates it and was pinned as
`a_callback_reaching_a_stylex_function_refuses` in
`transform_stylex_create_test/callback_bodies.rs`, which is now
`a_callback_reaching_a_stylex_function_folds`.

**Blocked by:** none.

**Status:** resolved

- [x] An array method whose callback names a StyleX function folds to the
      reference compiler's declaration text and class name
- [x] The same shape on a string and an object receiver is measured and either
      folds or carries a written reason
- [x] The pinned refusal test becomes a fold test
- [x] A parity corpus entry records the shape

## Answer

Closed by giving the *function* to the engine, not by taking the callback out of
it. The ticket argued for running the callback per element in Rust; that is a
table of the callback-taking methods under another name, and it would have had to
agree with the engine about `map`, `filter`, `flatMap`, `reduce` and the rest.

`firstThatWorks` is the one function of the injected map whose answer is a
function of its arguments alone — it reorders the fallbacks it was handed and
folds the CSS variables among them into one `var()` chain, touching no state. So
it crosses as an ordinary transport parameter holding a native function of the
engine's own, and the callback stays one JavaScript call per element. That is a
rule and not a list: a function that writes into the build cannot cross, because
running one once per element of a receiver, inside a speculative read, would
inject what the source describes once as many times as the receiver is long.
`keyframes` is the case that proves it, and it still refuses.

What the mechanism is, in four parts:

- **The ordering is shared Rust.** `plan_fallbacks`, `fold_fallback_chain` and
  `css_variable_name` moved out of `stylex_first_that_works` as a pure core over
  *positions*, and both callers read their own values back at those positions.
  The expression path lost its copy of the index arithmetic, so the engine's
  answer and the evaluator's cannot drift. `css_variable_name` answers the test
  and the slice as one question, so no caller can slice a name out of text that
  has none. The core is answered by unit tests of its own, over lists of
  booleans, which is what that arithmetic reads as plainly.
- **The guard admits the call as a value, never as the fold.** `admit_value`'s
  call arm asks `engine_stylex_functions::engine_callable` first, because every
  question `admit_call` asks answers "not mine" for a callee the module bound. A
  call written on its own still belongs to the older dispatch — deliberately, so
  that its arguments keep being resolved this compiler's own way, a theme
  reference included, which the engine has no value for.
- **The name travels, not the map.** `Carried::Function` is the one variant not
  copied out of the module and the one that costs neither ceiling — nothing is
  copied, so there is no text and no entry count to measure. A namespace
  spelling carries an object, so `stylex.firstThatWorks(…)` prints as it was
  written; the object holds the whole engine-callable surface rather than the
  property this call site named, so what a name carries is a function of the name
  and the transport's one-value-per-name rule cannot drop a second naming. Every
  other property of the real namespace is a function the engine may not call, and
  a call naming one is declined before anything is printed. Nothing is registered
  on the engine, so nothing is left behind for the next file the thread
  compiles.
- **Which name is which function is asked of the module's import record**, not of
  the function map. A map entry only says that some Rust function stands behind a
  name and every one of those looks alike from there; the record says *which*
  StyleX function it is. One `CALLABLE` set holds the answer to the one question
  that has no source to read it off — does this function answer from its
  arguments alone — and both spellings, the bare name and the namespace property,
  are matched against it. A callback parameter of the same spelling is the
  callback's.

### Measured against `@stylexjs/babel-plugin` 0.19.0

Thirty-three cases in
`transform_stylex_create_test/stylex_functions_in_a_fold.rs`, each asserting the
declaration text **and** the class name. Everything the reference compiler folds,
this one now folds identically: the reported shape, the namespace and renamed
spellings, string and object receivers, `filter`/`reduce`/`flatMap`, chains on
both sides of the call, the call nested in itself, an inner arrow, two calls in
one body, non-string arguments, the chain that stops at the first value after the
variables and drops what follows, no arguments at all, text outside ASCII, five
hundred elements and a two-thousand-character fallback. An argument the bridge
cannot carry names the function inside a callback and stays the dispatch's
outside one, which is the two halves of that rule pinned. The corpus row
`modules-17-a-callback-naming-a-stylex-function` reads `identical`, and the
whole run is still `unexpected 0`, `changed 0`.

Three shapes stay divergent, each with the reason written down:

- **A function that writes into the build** — `keyframes` inside a callback.
  Folds upstream, refuses here. The boundary above, and the one thing this
  mechanism cannot be widened to without deciding what "inject once per element
  of a receiver nothing measured, during a speculation" should mean.
- **The function read as a value** rather than called — `typeof firstThatWorks`.
  Upstream answers `object`, because its evaluator holds the function as a
  configuration object. Answering it would mean handing the engine a value whose
  own properties lead back into the compiler, and no stylesheet asks the
  question.
- **A computed callee** — `stylex['firstThatWorks'](…)`. Folds upstream. Refused
  here by the fold's existing rule that a computed member name is a lookup the
  guard does not resolve even when it is written as a literal, which predates
  this ticket and refuses the same spelling outside a callback too.

Two more shapes this ticket deliberately did not touch, because they are not
about StyleX functions: a **local** function of the module's own called inside a
callback (the neighbour of ticket 19, refused here and folded upstream), and an
amplifying call beside this one inside a body, which ticket 12's callback bound
refuses on purpose.

### Where the sentence a refusal carries changed

An argument the bridge cannot carry now reads
`Only static values can be passed to firstThatWorks().` — the applied global's
own sentence, reused because it says exactly the right thing and a second
near-identical message is how two sites come to word one rule differently. The
function's own spelling is named rather than the namespace in front of it, since
that is the half an author changes.

### For 19

19's body asks whoever takes both to decide whether one mechanism answers them.
It does not. This one reaches a function the *compiler* owns, whose Rust body is
available to be called; a named arrow the author wrote has no such body to hand
over, and answering it means printing the arrow the name resolves to into the
transport. 19 is unchanged by this and still open.
