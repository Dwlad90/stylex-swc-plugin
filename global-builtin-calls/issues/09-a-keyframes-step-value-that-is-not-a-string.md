# 09 — A keyframes step value that is not a string

**What to build:** An animation step whose declaration value is an object or an
array compiles, instead of failing the build from inside a string converter.

Today every one of these panics with
`[StyleX] Expression in not a string, got Known(Obj)`, raised at
`crates/stylex-transform/src/shared/utils/ast/convertors.rs:114` — a step value
is read through `convert_expr_to_str`, which accepts an identifier or a literal
and refuses everything else:

```js
stylex.keyframes({ from: { color: { default: 'red' } }, to: { color: 'blue' } });
stylex.keyframes({ from: { color: ['red', 'blue'] },    to: { color: 'blue' } });
```

The reference implementation compiles all of them, emitting the step with the
declaration dropped: `@keyframes x1mv4754-B{from{}to{color:blue;}}`. A nested
value object has no meaning inside a step — there is no condition to apply —
and neither does a fallback array, so upstream neither honours nor rejects
them; the declaration simply does not survive.

## Why it is filed separately

Found while covering the API surfaces in 07. It is **not** part of the fold:
the failure reproduces from a bare `{ default: 'red' }` step with no call
anywhere. But it blocks a surface 07 would otherwise cover, because
`Object(null)` folds to an empty object and `Array('red', 'blue')` folds to an
array, so both land on exactly this refusal — where upstream leaves the step
empty and compiles. `String` and `Number` are unaffected, because neither folds
to an object or an array, and their keyframes coverage landed in 07.

## What to measure first

Whether "drop the declaration" is the whole contract or only the part these
three inputs reveal. Measured so far, all against
`@stylexjs/babel-plugin` 0.19.0:

| input                                | reference output                              |
| ------------------------------------ | --------------------------------------------- |
| `{ color: { default: 'red' } }`      | `@keyframes x1mv4754-B{from{}to{color:blue;}}` |
| `{ color: Object(null) }`            | `@keyframes x1mv4754-B{from{}to{color:blue;}}` |
| `{ color: Array('red', 'blue') }`    | `@keyframes xdtusy9-B{from{}to{color:green;}}` |

Note the animation name differs between the first two and the third only
because the neighbouring step differs, not because the dropped declaration
does. Both hashes are of a step that came out empty.

**Found while implementing 08:** a **`null`** step value is a fourth shape that
lands on the same refusal, and it is the plainest of them — no call, no object,
no array. `stylex.keyframes({ from: { color: null }, to: { color: 'blue' } })`
fails this compiler with `Expected a string value but received a different
type.` where upstream emits `@keyframes x1mv4754-B{from{}to{color:blue;}}` —
byte-identical to what it emits for the nested-object shape above, so all four
shapes share one outcome. Confirmed independent of 08: the keyframes path
touches neither `flatten_raw_style_object` nor `convert_style_to_class_name`, so
routing blank values to the drop left this untouched. Worth using as the first
test, being the shortest input that reproduces.

Follow the read of the step value rather than adding a guard at the call site:
the panic is a string converter refusing a non-string, and the question is what
a step value is allowed to be, which is one decision in one place.

**Blocked by:** None — independent of the fold.

**Status:** done

- [x] A step whose declaration value is a nested value object compiles, with
      that declaration dropped
- [x] A step whose declaration value is an array compiles, with that
      declaration dropped
- [x] A step whose declaration value is `null` compiles, with that declaration
      dropped
- [x] `Object(…)` and `Array(…)` in a step reach the same outcome, pinned in the
      keyframes transform directory beside the `String` and `Number` cases
- [x] Expected animation names and rule text are measured reference output
- [ ] ~~No existing fixture shifts~~ — **five do**, all in `positionTry`, each by
      exactly one space in the `@position-try` rule text, in both the LTR and the
      RTL form. Recorded under Outcome but ticked here as though it had not
      happened; corrected rather than left false. The shift is a parity fix, not
      a regression — see the sign-off below.

## Sign-off: the `@position-try` spacing

Raised in review as unfiled scope. It is a deliberate parity fix, not creep, and
is signed off here rather than reverted:

- 08 found it and explicitly did not file it: _"the at-rule is serialized as
  `@position-try --x{…}` against upstream's `@position-try --x {…}`. Untouched by
  this ticket"_. 09 then rebuilt the same at-rule body to route it through
  `Pair::as_css_text`, which put the serialization under the hand doing the work.
- The space sits **outside** the hash, which is taken from the body alone, so no
  generated name moves. The only change is rule text, which is compared byte for
  byte wherever both compilers' output meets — the parity claim user story 18
  makes.
- Reverting would restore a byte-level divergence from the reference
  implementation that this branch is otherwise closing.

## Outcome

"Drop the declaration" is the whole contract, and it covers more shapes than the
three measured up front: `null`, a nested value object, a fallback array, plus
`undefined` and a boolean. All five reach the animation name a step declaring
nothing produces, and `Object(null)` and `Array('red', 'blue')` reach it through
the fold — the coverage 07 had to defer.

The panic was a symptom one level up from where it was raised.
`convert_expr_to_str` returns an `Option` and then panicked for the one case
that `Option` exists to express, so every caller inherited a decision it never
made. It now answers `None`, because what a non-string means is the caller's
question: a step of an animation declares nothing, a namespace name is a hard
error, and answering it in the converter forces one onto the other. Every caller
already had a `None` arm, so the other twenty-odd sites got better diagnostics
rather than different behaviour — the site that knows the context now names the
problem. `ident_to_string` had the same shape one level down again, which is
what `undefined` needed, being an ordinary global identifier with no binding to
read rather than a literal.

That left the step-value read as a single decision: a value that is not a string
or a number yields no declaration, and the step keeps whatever else it declares.

`Object(primitive)` is deliberately absent. It fails with the existing
style-value rejection, which is exactly what `create` gives it — 06's recorded
decision, inherited rather than re-made here.

### Found and fixed alongside

The same two defects in `positionTry` and `viewTransitionClass`, both recorded
in 08 as "found but not filed": a `null` value failed the build where upstream
drops the declaration, and a blank value emitted `top:` where upstream crashes.
Both APIs read their values through the same shorthand expansion, which treated
a literal spelling no string as a hard error when that is precisely what `null`
is; it now yields the property with nothing to declare, the answer
`PreRuleValue::Null` already gave through the same route.

The three at-rule bodies assembled from pairs each formatted `key:value;` their
own way and only `keyframes` skipped a blank one. `Pair::as_declaration` is now
that rule in one place, asked by all three, which is what makes 08's blank rule
reach these two APIs at all. `positionTry` needed one thing more: its doubled
`key:key;key:value;` form repeats the property name as its own value, and that
first half is a companion to the real value, so when the value spells nothing
neither half is written — otherwise a stray `top:top;` survives and moves the
name.

Also closed: `@position-try --x{…}` now reads `@position-try --x {…}`. The space
sits outside the hash, so no name moves; five fixtures shift by exactly one
space each, in both the LTR and the RTL rule, all measured.

### Review outcome

Three findings applied. Shorthand expansion had been widened too far: reading
*every* literal that spells no string as an absent value is right for `null` and
a boolean, both of which upstream drops, and wrong for a regular expression,
which upstream rejects — an unusable value is not an absent one, and the two are
now matched separately. `_evaluate_style_object` read any non-string value as the
`$$css` flag, so an object became `true` rather than raising; it was narrowed to
skip the property instead of inventing a flag. `write_tuple_declarations` became
`tuple_declarations`, returning its declarations rather than writing them, with a
name that says which half is which — its argument is not a key/value pair despite
the type.

Three tests closed gaps: the converter's `None` arms including an unbound
identifier, a blank keyframes step value that the pair rule started dropping
with nothing pinning it, and an unresolvable step value still being an error.

### Second review pass

Four of the three above did not survive re-review, and the record is corrected
rather than left standing:

- **`_evaluate_style_object` is gone.** It is private, underscore-prefixed and
  called from nowhere; narrowing it was a change to unreachable code, and the
  narrowing itself dropped a property where the previous shape had reached
  `convert_expr_to_bool`. Deleted, per `docs/agents/domain.md`'s "staleness is a
  delete signal".
- **The boolean arm of the expansion had no test.** The claim that upstream drops
  a boolean was measured but nothing pinned it: the suite stayed green with the
  arm removed, because `create` refuses a boolean before the expansion runs and a
  keyframes step drops one by its own route. `positionTry` is the one surface
  that reaches it, and now has `a_boolean_value_declares_nothing` pinning the
  anchor-only name.
- **The names moved again.** `Pair::as_declaration` and `tuple_declarations`
  both used terms `crates/stylex-structures/CONTEXT.md` lists under `_Avoid_` for
  **Pair** (`declaration`, `tuple`). They are now `Pair::as_css_text` and
  `doubled_css_text`, and **CSS text** is a term in that glossary — which is what
  the avoid-list was asking for, rather than a synonym hunt.

Two findings declined, both measured rather than argued:

- An unresolvable identifier in a step was reported as newly silent. It still
  fails with `Only static values are allowed inside of a keyframes() call.` —
  upstream's own message — because evaluation rejects it before any step value is
  read. The new validation test pins that line, since a name with no binding to
  read is not a value with nothing to say.
- `concat_view_transition_class_style_str` was reported as a fourth hand-rolled
  body builder that should ask `Pair::as_declaration`. It writes the string the
  class name is hashed from, where a selector whose body came out empty still has
  to appear for the name to agree with upstream, so it keeps its own formatting
  and now records why.

### Divergences left standing

Only where upstream crashes and this compiler now emits valid CSS: a blank
string value in a keyframes step, in `positionTry` and in `viewTransitionClass`.
Each converges on what upstream does deliberately for `null`, which is 08's
rationale applied to three more surfaces.
