# 05 — Route `+`'s string side through the shared coercion crate

**What to build:** `'x' + true` produces `"xtrue"` and `'x' + null` produces
`"xnull"`, rather than failing the build with an unsupported-expression
diagnostic.

The coercion crate already answers all of this correctly — its `ToString`
handles booleans, `null`, `undefined`, big integers, regular expressions,
arrays and objects, the last through the method pair a string coercion prefers.
These operands fail today only because the addition arm reaches for the
evaluator's own, weaker string conversion instead.

Point it at the shared coercion, through the bridging helper that ticket 01 put
in scope. This retires the second string conversion for this operator, which is
the same duplication shape that let the logical-expression bug survive
unnoticed.

## Read the suite diff before calling this correct

This is the one ticket in the effort with an unbounded blast radius, and the
spec records it as an open risk.

The shared `ToString` succeeds on strictly **more** inputs than the conversion
being replaced. A value that previously failed to resolve — and therefore fell
to the runtime as a
[dynamic style](../../../crates/stylex-transform/CONTEXT.md) — may now fold
statically. For the three measured rows that is precisely the intended fix. It
is not, *a priori*, bounded to them.

So the acceptance bar here is not "the suite is green". It is that every moved
snapshot has been read and understood. A fixture that moves because a value
that used to reach the runtime now folds at compile time is a real behaviour
change for anyone shipping that code, and if it lands in fixtures unrelated to
`+` it is a scope conversation — stop and raise it rather than absorbing it
into this branch.

**Blocked by:** 04.

**Status:** resolved

- [x] The addition arm's string side reaches the shared coercion crate through
      the evaluator's bridging helper
- [x] `'x' + true` produces `"xtrue"`; `'x' + null` produces `"xnull"`;
      `'x' + undefined` produces `"xundefined"`
- [x] Everything ticket 04 fixed or preserved still holds
- [x] The evaluator's weaker string conversion is no longer reached from the
      addition arm
- [x] **Every** snapshot that moves has been read and its cause named in the
      commit body — not merely re-recorded
- [x] A snapshot moving in a fixture unrelated to `+` is escalated, not
      absorbed
- [x] `pnpm run --filter=@stylexswc/rs-compiler build` before any suite that
      reaches the compiler through the Node package
- [x] `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`, `pnpm test` all
      pass
- [x] Lands as `fix(stylex-transform):` with fixtures

## Answer

Landed as `8ede5c368`.

`binary_expr_to_string` now converts each operand through
`evaluate_result_to_js_string`, the bridging helper ticket 01 put in scope, and
`convert_expr_to_str_or_err!` is deleted along with its last caller.

**The blast radius was empty.** No existing snapshot moved anywhere in the
workspace — `git status` after the change listed only the new fixture file and
its own snapshot directory. Nothing had to be escalated.

**One deliberate behaviour change beyond the coercion swap.** The operator is
now tested *before* either operand is converted, and a non-`Add` operator is
refused with an `Err` where it used to `stylex_panic!`. That is not cosmetic:
the shared `ToString` succeeds on more inputs, so with the panic left in place
`null - 1` and `[1, 2] * 2` — which reach this function only through the number
path's fallback, and which previously deopted — would have started *failing
builds*. Refusing early keeps them deopting, and matches the language, which
reads `'a' * 'b'` as `NaN` rather than an error. Two unit tests that pinned the
panic were rewritten to pin the refusal.

**Measured against `@stylexjs/babel-plugin@0.19.0`**, every folded value agrees:

| source | both compilers |
| --- | --- |
| `'x' + true` / `'x' + false` | `"xtrue"` / `"xfalse"` |
| `true + 'x'` | `"truex"` |
| `'x' + null` / `'x' + undefined` | `"xnull"` / `"xundefined"` |
| `'x' + NaN` / `'x' + Infinity` | `"xNaN"` / `"xInfinity"` |
| `'x' + [1, 2]` / `'x' + []` | `"x1,2"` / `"x"` |
| `'x' + {}` | `"x[object Object]"` |
| the reporter's `borderRadius` template | `border-radius: 0 0 .25rem .25rem` |

Two rows diverge, in the permissive direction and not on value: `'x' + 1n` and
`'x' + /ab/g` fold here, where the reference implementation refuses the literal
outright with `Unsupported expression`. Left as-is — the folded strings are what
JavaScript says, and ticket 07 is where that goes.
