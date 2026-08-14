# Draft — upstream defect report

Ticket 07's deliverable. **Not posted.** Filing happens under the maintainer's
own account, only on an explicit go-ahead, and the resulting issue number is
recorded in `issues/07-draft-the-upstream-defect-report.md` once it exists.

Target repository: `facebook/stylex`.
Suggested title: `LogicalExpression: value confused with truthiness in ?? and &&`

---

## Body

### Summary

Two defects in the `LogicalExpression` branch of the static evaluator share one
root cause: the branch tests an operand's **truthiness** where it means the
operand's **value**. One makes `??` refuse to compile a statically-resolvable
expression; the other makes `&&` crash the compiler with an unhandled
`TypeError` instead of producing a diagnostic. They sit in the same block and a
single edit fixes both, which is why they are reported together.

Observed on `@stylexjs/babel-plugin@0.19.0`, with `dev: false`,
`treeshakeCompensation: true` and CommonJS module resolution.

### Defect 1 — the nullish guard tests truthiness where it means nullishness

The nullish branch is guarded, in effect, by:

```js
leftConfident && !!(left ?? rightConfident)
```

When the left side is falsy but **not** nullish — `0`, `false`, `''` — the `??`
yields that falsy value, `!!` turns it into `false`, and control falls through
to an unconditional deopt. So a `??` whose operands are both statically known
fails to compile, purely because the left one happens to be falsy.

The guard appears to have been intended as `left != null || rightConfident`.

#### Reproduction

```js
// repro-1.js
import * as stylex from '@stylexjs/stylex';

const zero = 0;

export const styles = stylex.create({
  a: { flexGrow: zero ?? 5 },
});
```

**Expected:** `flex-grow: 0`. Both operands are statically known, and `0 ?? 5`
is `0` — there is nothing here the evaluator lacks.

**Observed:** the build fails with `unknown error`, the deopt reason the branch
falls through to.

The same shape inside a template literal, with an empty string:

```js
// repro-1b.js
import * as stylex from '@stylexjs/stylex';

const token = '';

export const styles = stylex.create({
  a: { color: `${token ?? 'red'}` },
});
```

**Expected:** `color: ''` — `'' ?? 'red'` is `''`.
**Observed:** the same `unknown error`.

For contrast, the identical expression with a nullish left side compiles as it
should, which is what isolates the guard as the cause:

```js
const token = null;
export const styles = stylex.create({ a: { color: token ?? 'red' } });
// compiles to color: red
```

### Defect 2 — `&&` with a falsy confident left side crashes rather than diagnosing

The `&&` branch correctly returns `left` when the left side is falsy — that is
what the language says. A later consumer does not expect that value and dies on
it.

#### Reproduction

```js
// repro-2.js
import * as stylex from '@stylexjs/stylex';

const token = '';

export const styles = stylex.create({
  a: { color: token && 'red' },
});
```

**Expected:** either the declaration is dropped, or a diagnostic naming the
property that could not be compiled.

**Observed:** the compiler throws

```
TypeError: Cannot read properties of undefined (reading 'type')
```

with no code frame and no mention of `a` or `color`. Inside a style object of
any size, that is the difference between a two-minute fix and an afternoon of
bisecting.

### Why one issue

Both branches decide what to do by asking whether an operand is *truthy*, when
the question the operator actually asks is what the operand *is* — nullish in
the first case, and simply "the value, whatever it is" in the second. Fixing
defect 1 means replacing `!!(left ?? rightConfident)` with a nullishness test;
fixing defect 2 means letting the returned falsy value reach a consumer that
handles it. Both live in the same block.

---

## Notes for the maintainer of this port (not part of the issue body)

- Defect 1 is reproduced here **bug-for-bug**, `!!` included, and pinned by
  `nullish_refuses_a_zero_left_side`, `nullish_refuses_a_false_left_side` and
  `nullish_refuses_an_empty_string_left_side`. The reasoning is in the spec:
  owning a divergence in the permissive direction is worse than inheriting the
  restrictive one, because a value this compiler folds and upstream refuses is a
  silent CSS difference between two builds of the same source. If upstream fixes
  the guard, those three fixtures are what changes.
- Defect 2 is **not** reproduced. This port returns the falsy operand faithfully
  and lets the existing downstream handling drop the declaration, which is
  `and_returns_a_falsy_left_side`. A crash is not a behaviour worth porting.
