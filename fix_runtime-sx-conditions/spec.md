# A panic aborts a build from inside a recoverable evaluation

Status: ready-for-agent

Tracks GitHub issue
[#1265](https://github.com/Dwlad90/stylex-swc-plugin/issues/1265).

Upstream reference: `~/Projects/Facebook/stylex` @ `@stylexjs/babel-plugin`
0.19.0, `src/utils/evaluate-path.js`.

## Problem Statement

An author writes a boolean `sx` condition whose right operand is a method call
on a string literal, and `0.18.4-rc.2` fails the build:

```jsx
const showAlternate =
  query.length > 0 && 'documentation'.startsWith(lowerQuery);

return <section sx={[styles.base, showAlternate && styles.alternate]} />;
```

```
error: [StyleX] The method 'startsWith' is not yet supported in static evaluation.
```

The argument is a runtime value, so nothing here is foldable and nothing should
be folded. The condition belongs in the output verbatim.

## What the version matrix says

Measured by installing each published package and running the reporter's
script:

| version       | result                                     |
| ------------- | ------------------------------------------ |
| `0.18.3`      | compiles, condition preserved              |
| `0.18.4-rc.1` | compiles, condition preserved              |
| `0.18.4-rc.2` | `The method 'startsWith' is not supported` |

`StringJS` is byte-identical across all three -- `concat` and `charCodeAt`, and
nothing else. The panic branch that fires here has been in the tree the whole
time. Only its reachability changed.

## Root cause

`1322be8c1` ("fold the logical operators in style values") added
`nodes/logical_expression.rs`. It evaluates the right operand of `&&` under a
forked confidence, which is what the reference implementation does at
`evaluate-path.js:829-831`:

```js
const stateForRight = { ...state, deoptPath: null, confident: true };
const right = evaluateCached(rightPath, stateForRight);
```

The fork exists so that an operand which cannot be folded sets
`confident = false` and the expression deopts. Before `1322be8c1`, `&&` fell
through `binary_expr_to_num` / `binary_expr_to_string`, both failed, and the
right operand was never evaluated at all -- which is why rc.1 passes.

`stylex_panic_with_context!` does not participate in that fork. It aborts the
whole build from inside an evaluation whose entire contract is that it is
allowed to fail.

**So the defect is not a missing `startsWith`.** It is an unrecoverable failure
sitting in a position the evaluator requires to be recoverable. `startsWith` is
the first method a reporter happened to put on the right side of an `&&`; every
other unsupported shape in `evaluate/` is the same bug awaiting a different
input.

`logical_expression.rs` is correct and must not be reverted.

## Solution

Two kinds of failure currently share one construct. Separate them:

- **Unsupported input shape** -- the evaluator does not fold this, which is an
  ordinary answer. Deopt, matching the reference implementation's terminal
  `deopt(path, state, errMsgs.UNSUPPORTED_EXPRESSION(path.node.type))` at
  `evaluate-path.js:1055`.
- **Invariant violation** -- the evaluator's own reasoning broke. Panic.

Every site in `evaluate/` is audited and classified. Unsupported-shape sites
move to a distinct construct so the two read differently at every call site and
a reviewer can spot a misuse; convention alone is what failed here.

With that in place the reported input deopts, `showAlternate` survives into the
output, and the result matches both rc.1 and the reference implementation.

## Scope

In scope:

- The panic/deopt split across `evaluate/`, and the audit that applies it.
- Deopt reasons naming the node kind, so the split does not trade a precise
  message for `Unsupported expression: Unknown`.
- `"abc".length` evaluating to `"abc"` -- a silently wrong CSS value, found
  during this investigation.
- Regression coverage at the logical-expression seam and in the parity corpus.

Out of scope, tracked separately as the parity issue in `issues/`:

- Folding the `String.prototype` / `Array.prototype` / `Object.prototype`
  surface that the reference implementation reaches by reflection. That gap is
  real and measured, but it predates this regression, shipped in `0.18.3` and
  `0.18.4-rc.1`, and has never been reported. Fixing the panic does not depend
  on it.

## Phases

**Phase 1 — the regression.** Issues 02, 03, 04. Closes
[#1265](https://github.com/Dwlad90/stylex-swc-plugin/issues/1265) and is
independently shippable as `0.18.4-rc.3`. Restores full parity with `0.18.3`
and with the reference implementation for every runtime position; leaves the
static-position fold gap exactly where `0.18.3` left it.

**Phase 2 — wrong output found on the way.** Issues 01, 07 and 08. Each changes
emitted CSS and generated class names, so each needs a release note and none
should ride along with a fix that has to go out quickly. 01 is done; 08 was
filed from it, and is the array-hole representation its fix works around.

Issue 09 was filed from 01 as well but belongs to **Phase 1**: an unpaired
surrogate still aborts two string folds, which is the panic family 02 closed for
`evaluate/` reached one layer down in a convertor. It ships no wrong value and
can ride along with a fix that has to go out quickly.

**Deferred.** Issues 05 and 06, in that order. Not scheduled; 06 does not start
until 05 answers how.

## Non-goals

Reproducing the reference implementation's numeric-receiver behaviour.
`(5).toFixed(2)` throws there -- `Number.prototype.toFixed requires that 'this'
be a Number` -- because `evaluate-path.js:1010` sets the call context only for
string receivers and then applies against `undefined`. Both compilers reject
the input; only the wording differs, and no build can depend on the text.
Pinning it would bake an upstream defect into this suite. File it upstream
instead.
