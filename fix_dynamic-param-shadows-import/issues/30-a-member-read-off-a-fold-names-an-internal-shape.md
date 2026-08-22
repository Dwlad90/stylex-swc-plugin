# 30 — A member read off a fold names an internal shape

Status: `resolved`
Blocked by: None

**What was found:** A member read off a folded function map, where the fold has
no entry for the name, refuses with a sentence about this compiler rather than
about the input. Measured against `@stylexjs/babel-plugin` 0.19.0 under the
parity harness's configuration, while closing
[15](./15-the-function-map-read-where-it-is-not-a-map.md).

```js
import { create, keyframes } from '@stylexjs/stylex';

export const styles = create({ a: { height: keyframes.nope } });
export const other  = create({ a: { height: keyframes[0] } });
```

| | |
| --- | --- |
| Babel | `A style value can only contain an array, string or number.` |
| here | `Unexpected error:` / `Could not determine the property being accessed.` |

Both compilers refuse, so no build emits the wrong CSS -- the divergence is in
what the author is told. Upstream reads `undefined` off the plain object it
folded the reference to and refuses that as a value, which is the sentence a
missing member gets everywhere else. This compiler's member step cannot name the
property on a `FunctionConfigMap` receiver and reports that it could not, with
an `Unexpected error:` prefix that reads like a bug in the compiler.

`docs/adr/0002-a-refusal-and-a-broken-invariant-are-separate-constructs.md` is
the rule this breaks: a member an author wrote is author input, and an input
this evaluator cannot fold is a refusal with the input's own message, not an
unexpected condition.

Recorded as `modules-1266-a-missing-member-read-off-a-folded-function-map` and
`modules-1266-an-index-read-off-a-folded-function-map`, both `both-reject
(diverged)`, and pinned in
`validation_stylex_create_test::invalid_values::{a_missing_member_read_off_a_static_fold_is_refused,an_index_read_off_a_static_fold_is_refused}`.

Where to look: `shared/utils/js/evaluate/nodes/member_expression.rs:448`, the
`FunctionConfigMap` arm. `function_fold_to_object_expr`
(`js/evaluate/mod.rs`) already answers what the fold stands for, so the member
step has an object to read the name off -- which would make a hit resolve and a
miss deopt as `undefined` does anywhere else. Measure before assuming that is
the whole shape: a *hit* on that path is `stylex.when` as a callee, which must
keep resolving to the map's own form.

- [x] The refusal names the input rather than an internal shape, or the sentence
      is recorded as a decided divergence
- [x] `stylex.when` as a callee still resolves, guarded by a test

## Resolved

The member step reads the key off the object `function_fold_to_object` already
builds, for both fold receivers -- the namespace map and one entry of it reached
through a named import. A key the object carries resolves; a key it does not
answers `undefined`, which is what the reference implementation reads off its
own `identifiers` object.

`stylex.when` as a callee is unaffected: a hit on the map is still answered in
the map's own form, ahead of any materialization, and the fold is consulted only
where that lookup found nothing. Guarded by
`transform_stylex_when_test::when_functions_transform::when_read_off_the_namespace_resolves_through_either_spelling`
and by `modules-1266-when-read-off-the-namespace-by-either-spelling`, which
reads `identical`.

The map lookup reads the key through the same coercion the fold does, which it
did not before: it recognised `stylex.when` and not `stylex['when']`, so with
the fold behind it the second spelling would have answered the placeholder
function where the first answered the callable. Two spellings of one property in
the language, and the map lookup is what decides whether they agree -- found in
review of this change.

The `undefined` had one more step to travel. A style value position reported
`Only static values are allowed inside of a stylex() call.` for it, which is a
sentence about resolution and names neither the value nor the input --
`undefined` is a value this evaluator is confident about, not a name it failed
to resolve. It is refused as a style value now, with the reference
implementation's own text. That closes the pre-existing gap
`transform_stylex_create_test::logical_operators::a_bare_missing_property_is_rejected_as_a_style_value`
recorded, and it is the same sentence a key an object does not carry and an
index past the end of an array now earn.

`is_js_undefined` in `stylex-ast` is that test, once: four private copies of it
had accumulated across the evaluator and the flattener, which is how they could
have come to disagree about which of them was looking at a value.

Measured: `modules-1266-a-missing-member-read-off-a-folded-function-map` and
`modules-1266-an-index-read-off-a-folded-function-map` both read `both-reject`
where they read `both-reject (diverged)`. Pinned in
`validation_stylex_create_test::invalid_values`, with the `fn` key the entry
does carry and the namespace fold's own miss added beside the two that were
already there.
