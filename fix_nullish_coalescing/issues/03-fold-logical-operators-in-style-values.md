# 03 — Fold `??`, `||` and `&&` in style values

**What to build:** The fix for
[#1254](https://github.com/Dwlad90/stylex-swc-plugin/issues/1254). An author
writes a guarded token in a style value and it compiles to the same CSS the
reference implementation produces:

```js
const radius = { s: '0.25rem' };
export const styles = stylex.create({
  a: { borderRadius: `0 0 ${radius.s ?? ''} ${radius.s ?? ''}` },
});
```

Today this fails the build with `For string expressions, only addition is
supported, got "??"`. After this ticket it emits
`border-radius: 0 0 .25rem .25rem`, and the same holds for `||` and `&&`, in
template literals and in direct style values alike.

The reference implementation has a distinct logical-expression node, evaluated
over JavaScript **values**, tested ahead of the binary-expression node. SWC has
no such node — the three operators arrive as binary expressions alongside `+`
and `>>` — so this port never grew the branch, and the operators fell to
whichever of the number path or the string path claimed them. Add the missing
node and dispatch to it first, returning before the number-then-string fallback
can run.

The node returns the winning operand **verbatim**. Upstream returns the value,
so an object, an array or a callback that wins stays what it is. Do not
normalise it into a re-created string or number literal — `config ?? {}` and
`list ?? []` are exactly the idiom the reporter has across their files.

## Reproduce upstream's guard exactly, including its bug

Upstream's nullish branch guards with, in effect, `leftConfident && !!(left ?? rightConfident)`.
When the left side is falsy but not nullish — `0`, `false`, `''` — the `??`
yields that falsy value, `!!` turns it to `false`, control falls through, and
the expression deopts with `unknown error`. So upstream **refuses to fold**
`x ?? 5` when `x` is `0`, even though both sides are confident. The guard was
evidently meant to test nullishness, not truthiness.

Reproduce it as-is, with a comment recording that the shape is upstream's and
why it is not being corrected here. Parity is the goal, and a value this
compiler folds where upstream refuses is a silent CSS difference between two
builds of the same source — worse than inheriting the restriction. Ticket 07
reports it upstream.

For `&&` with a falsy but confident left side, upstream returns that value and
then a later consumer crashes on it with a bare `TypeError` rather than a
diagnostic. A crash is not a behaviour worth reproducing; the evaluator itself
is unambiguous. Return the falsy operand faithfully and let the existing
downstream handling decide what an empty value means — recent work already
settled that a declaration whose value spells nothing is dropped.

Deopt reasons are reproduced verbatim, `unknown error` included. That string is
already this port's established fallback elsewhere.

## Delete the stand-ins

The number path's logical arms and the ad-hoc `LogicalOr` early-outs in both
the number and string paths have no counterpart upstream — its binary-expression
switch has no logical cases at all, because those nodes never reach it. Delete
all of them. Once the new node dispatches first, a logical operator cannot
arrive at either path, and the existing refusal on an unrecognised operator is
the right shape for one that somehow does.

## The fixture trap

A fixture asserting only "the build fails" is worthless for the falsy
non-nullish cases: this compiler already fails them today, for the unrelated
reason that it refuses *every* `??`. Those fixtures must assert the diagnostic,
so that they distinguish "refused by upstream's guard" from "the operator is
still unimplemented".

**Blocked by:** 01, 02.

**Status:** resolved

- [x] A logical-expression node joins the evaluator's node set, dispatched from
      the binary-expression node for the three logical operators and returning
      before the number-then-string fallback
- [x] `??`, `||` and `&&` fold in template-literal style values and in direct
      style values, matching the reference implementation
- [x] `??` folds when the left side is `null`, and when it is a property simply
      missing from an object
- [x] The winning operand is returned verbatim — a winning object stays an
      object, a winning array stays an array
- [x] `??` with a falsy non-nullish left side (`0`, `false`, `''`) refuses,
      reaching upstream's guard; the fixture asserts the diagnostic, not merely
      that the build failed
- [x] `&&` with a falsy confident left side returns that operand, and the
      surrounding declaration is handled by the existing empty-value rule
- [x] Deopt reasons match upstream's strings verbatim
- [x] The logical arms and both `LogicalOr` early-outs are gone from the number
      and string paths; the pre-existing logical unit test goes with them
- [x] Existing dynamic-style fixtures — `??` on a function parameter — are
      byte-identical. A logical operator whose sides cannot be resolved still
      falls to the runtime rather than failing the build
- [x] `pnpm run --filter=@stylexswc/rs-compiler build` before any suite that
      reaches the compiler through the Node package
- [x] `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`, `pnpm test` all
      pass
- [x] Lands as `fix(stylex-transform):` with fixtures

## Comments

Verified against `@stylexjs/babel-plugin@0.19.0` run locally on the same
inputs. Every folded fixture's class names and rule text are byte-identical to
its output, including the issue's own reproduction
(`.x1v5h5rg{border-radius:0 0 .25rem .25rem}`). The three falsy non-nullish
`??` inputs throw `unknown error` there and here.

A missing object property evaluated to no value at all rather than to
`undefined`, which the enclosing node then turned into an
`Unsupported expression: Unknown` deopt — so `token.missing ?? fallback` could
not fold. The member-expression node now answers `undefined`, as the reference
implementation's plain `object[property]` does. A bare `token.missing` therefore fails the build now,
where it used to fall to the runtime — the reference implementation fails it
too, wording the refusal differently; a fixture pins ours. Array indexing out of
range is the same shape and still deopts; it is untouched here.
