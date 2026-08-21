# 30 — A member read off a fold names an internal shape

Status: `needs-triage`
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

- [ ] The refusal names the input rather than an internal shape, or the sentence
      is recorded as a decided divergence
- [ ] `stylex.when` as a callee still resolves, guarded by a test
