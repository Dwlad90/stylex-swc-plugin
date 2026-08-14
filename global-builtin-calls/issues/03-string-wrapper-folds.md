# 03 — `String(x)` around a style value compiles

**What to build:** The reported bug, fixed. An author writes `String(x)` around
a style value and it compiles, producing the same class names and rule metadata
as the reference implementation:

```js
export const vars = stylex.defineVars({ background: String('#fff') });
```

Today this fails with `Only static values are allowed inside of a defineVars()
call.`, and the same input inside `create` fails as `Unsupported expression:
Unknown`. This is 18 of the 30 failing files in the report on
[#1253](https://github.com/Dwlad90/stylex-swc-plugin/issues/1253).

The evaluator already recognises the callee — the predicate and the constant
set match upstream character-for-character — but the branch body is an explicit
no-op comment, so the call falls through to the catch-all deopt. Upstream's
corresponding branch resolves the callee to the real global and applies it.

This ticket is the first callee through the whole path, so it carries the
shared machinery the later callees reuse: a `ToString` coercion in the
JS-semantics crate's new coercions module, an enum naming the foldable globals,
a callback variant carrying it, and the wiring that hands a function
configuration to the same apply-site the member built-ins already use — so
argument evaluation, spread rejection, and the confidence check are shared
rather than written a second time.

Keep the coercions module beside the predicates, both declared from the crate
root with neither re-exported, so import sites say which kind of helper they
reach for. Widen the crate's glossary charter from predicates-only to
predicates and coercions.

Do not add the foldable globals to the existing per-type method enums: those map
*method* names, and a callable global is a different concept that would need an
unreachable arm there.

`ToString` renders `null`, `undefined`, and `NaN` as their JavaScript
spellings, joins arrays with commas rendering empty elements as nothing, and
renders plain objects as `[object Object]`. Reuse the existing number-formatting
helper for `NaN` rather than spelling the rule twice.

Values with no JavaScript counterpart split by what they are upstream:
cross-file token references and environment objects are objects there, so they
stringify to `[object Object]`; callbacks and function configurations are
functions there, where `String(fn)` yields source text this evaluator does not
retain, so they deopt rather than produce a confidently wrong value.

Two things must keep **not** folding, and this ticket is what creates the risk:
a locally-declared binding shadowing the global is an ordinary function and is
left alone, and a coercion of a dynamic style function's parameter still
compiles to a CSS custom property.

Zero arguments give the empty string — that is what the language says and what
upstream does. Surplus arguments are ignored: `String(1, 2)` is `"1"`. Spread
arguments keep deopting.

**Blocked by:** 02 — the coercions need the JS-semantics crate wired into the
evaluator, and the predicate this hangs off should have one definition first.

**Status:** resolved

- [x] The issue's `defineVars`, `create`, and `createTheme` inputs compile, with
      class names and rule metadata matching `@stylexjs/babel-plugin` 0.19.0
      resolved from `node_modules`
- [x] `createTheme` is exercised against a token module that genuinely resolves —
      a fixture whose import fails resolution proves nothing
- [x] Zero arguments give the empty string; surplus arguments are ignored; a
      spread argument deopts
- [x] A locally-declared binding shadowing the global is not folded
- [x] A coercion of a dynamic style function's parameter still compiles to a
      custom property
- [x] Cross-file token references and environment objects stringify to
      `[object Object]`; callbacks and function configurations deopt
      — **corrected**, see the answer below
- [x] The crate glossary records the widened charter, and the transform crate's
      glossary defines what a callable global is
- [x] Expected values are taken from measured reference output, not from
      reading its source, and no comparison harness is committed

## Answer

`String(x)` folds. The evaluator's valid-callee branch now builds a function
configuration carrying a new `CallableGlobalJS` enum and hands it to the same
apply site the member built-ins use; the coercion itself is a pure function in
the JS-semantics crate's new `coercions` module.

Only `String` is in the enum. `Number`, `Array` and `Object` are still valid
callees whose call does not fold, exactly as before this change, so tickets
04–06 add one variant each without a behaviour cliff in between.

**One claim in the spec was wrong and is not implemented as written.** A
cross-file token reference does *not* stringify to `[object Object]`: the
reference implementation's var-group proxy intercepts `toString` and answers
the var group hash. Measured, `String(colors)` gives `color:x13pcrg7`, and this
compiler now matches — `ThemeRef::to_string_value` already existed for exactly
that key. Environment objects do take the object default, as the spec said.

Divergences from the reference implementation, all deliberate:

- `String(fn)` is the function's source text there; this evaluator retains no
  source, so it raises a diagnostic instead of folding a wrong value.
- `String(voidExpr)` deopts rather than folding to `"undefined"`: the shared
  argument evaluation drops a confidently-valueless argument, which would shift
  the remaining ones, so the fold refuses a list it cannot trust.

Parity was measured against `@stylexjs/babel-plugin@0.19.0` with matching file
names and root directories, including the `createTheme` fixture under
`tests/__virtual__/app`. Every committed snapshot is byte-identical to the
measured output. The probes live in `.scratch/fix_string-wrapper/` and are not
committed.
