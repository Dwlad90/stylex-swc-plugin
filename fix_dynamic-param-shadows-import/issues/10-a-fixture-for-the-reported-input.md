# 10 — A fixture for the reported input

Status: `resolved`
Blocked by: 02

**What to build:** The reported module pinned as emitted text, in both
development and production mode, not only as rule metadata.

The corpus compares rule metadata between the two compilers, which is the right
question for a divergence but says nothing about the code we emit around it. A
shadowed dynamic parameter is exactly the shape where the emitted module
matters: the parameter has to survive into the runtime function, while the
import stays a theme reference for the static prop beside it.

Add the reported input as a fixture case, following the existing ones — a
`.stylex.js` input with a development-mode and a production-mode expected
output, picked up by the fixture runner without registration.

- [ ] The fixture case exists with both expected outputs
- [ ] Both are checked in as generated, not hand-edited to what looks right
- [ ] The dynamic function keeps its parameter, and the static prop beside it
      keeps its theme reference

## Answer

Two fixture cases under `crates/stylex-transform/tests/fixture/`, both picked up
by `tests/fixtures.rs` without registration, both `output.js` and
`output_prod.js` written by the runner under `UPDATE=1` rather than by hand.

### `dynamic-param-shadows-import/`

The reported module verbatim. Production mode is byte-identical to
`@stylexjs/babel-plugin` 0.19.0 under the same options (modulo Babel's own
printer whitespace and its `_temp` / our `_temp2` counter):

```js
zIndex: (zIndex) => [
  { kY2c9j: zIndex != null ? "xr3buco" : zIndex, $$css: true },
  { "--x-zIndex": zIndex != null ? zIndex : undefined },
]
```

Both acceptance criteria read off the emitted text: the dynamic function keeps
`(zIndex)` as its parameter, and `wrapper` beside it keeps the theme reference —
`.zIndex-x1bsllxr{z-index:var(--_10-x19xkwqv)}` in development,
`.x25bfn{z-index:var(--x19xkwqv)}` in production. Both rule texts match Babel.

### `dynamic-param-shadows-import-edges/`

A second case carrying the shapes the reported one does not reach, all in one
module so their interaction is pinned too: a non-ASCII parameter name shadowing
an aliased import, the same name spelled as a `ü…` escape (the parser folds
it, so it shadows the same binding), a parameter named `firstThatWorks`, a
shorthand and a `marginInline` expanded under one parameter, two custom
properties as keys, two vendor-prefix-expanding properties, eight levels of
nested conditions, the parameter read through `+`, a template literal and
`calc()`, two parameters where only one shadows, and the import still read as a
theme reference after every name above has been taken.

Measured against Babel 0.19.0, prod: **21 of 24 class names and rule texts
identical.** The three that differ are the innermost three of the eight nesting
levels — `xd94ota`/`xfmfnbh`/`x1b7ijjw` against Babel's
`x1gxqx9w`/`xqw1h1y`/`x15stwyu` — which is issue 19's nested-pseudo-class
ordering divergence, not something this fixture introduces. Every custom
property name derived from a key path (`--x-gsepj1`, `--x-hsbtju`, …) agrees, so
the key-path hashing is not what differs.

Two pre-existing whole-suite emit differences are visible in the diff and are
out of scope here: we drop the treeshake-compensation side-effect imports in
production mode (the `counter-with-dynamic-styles` fixture does too — the prod
config passes `RuntimeInjection::Boolean(false)`), and our hoist counter starts
at `_temp2` where Babel's starts at `_temp`.

### Edge cases beyond emitted text

Refusals and generated boundary conditions cannot be fixtures — the runner
compares printed output, and a refusal panics. They went to a new
`tests/transform_stylex_create_test/dynamic_param_shadowing_edges.rs`, 21 tests,
each measured against Babel 0.19.0:

| input | Babel 0.19.0 | ours |
| --- | --- | --- |
| `color: 'rgb(0,0,'` beside a shadowing param | `Rule contains an unclosed function` | same, plus the rule text |
| `'@media (min-width:'` around one | `Invalid media query syntax.` | identical |
| rest / destructured / defaulted param | `Only named parameters are allowed…` | identical |
| `...zIndex` spread, `[zIndex]` computed key | `Only static values are allowed inside of a create() call.` | `Referenced constant is not defined.` |
| `content: '"unterminated'` | accepts, `.xbjs7n6{content:""unterminated"}` | identical |
| `':hoverr'`, `':'`, `'@media'` with no condition | accepts as written | identical |
| `'--dépth'`, `'"My\ Font"'`, `'"—"'` | accepts | identical |
| shadowed `firstThatWorks` called | folds to the helper, not the param | identical |
| param called (`zIndex()`), member-read (`zIndex._10`) | accepts, param wins | identical |
| `{}`, `null`, no parameter at all | accepts, nothing emitted | identical |
| 128 nested conditions, 64 parameters, a 5000-char value | accepts | accepts, no stack exhaustion |

Eleven accepting cases, class name and rule text identical to Babel in every
one. The only divergence is the spread / computed-key **message**: both compilers
refuse, the outcome agrees, the text does not. Left as-is and pinned, because
changing it is a constant-text decision like the ones issues 03 and 06 made, not
a test's to take.

## Review

Both axes ran. Spec confirmed the counts and that both expected outputs are what
the runner writes; Standards found no hard breach of a documented standard.
Findings acted on, in `a92dbbe65`:

- **A vacuous assertion.** `a_five_thousand_character_value_beside_a_shadowing_param`
  asserted `contains(rule) || contains(long)`, and both reviewers found it passing
  only through the right branch — the escaping in the left one never matched, so
  the rule text was never compared, and the right branch would hold if the 5000
  characters appeared anywhere at all, untransformed included. Now asserts
  `.x1ahcjaz{content:"…"}` whole. The class name is the hash of all five thousand
  characters, which is what the comment claimed all along.
- **A duplicated fixture helper.** `shadowing_transform` was a verbatim copy of
  the one in `dynamic_styles.rs`, filename and resolution and all, so the two
  files that ask this question could drift apart. Lifted to `tests/utils`.
- **A middle man.** A one-caller `stylex_transform` wrapper around
  `build_test_transform`; deleted.
- **Two comments claiming more than their test checked** — one about not scaling
  with parameter count, one promising a named divergence that was not named.
  Reworded to what is asserted, and the bare-colon case now records that Babel
  0.19.0 emits the same unparseable text under the same class names.
- **An unexplained `6`.** `assert_eq!(output.matches("--x-").count(), 6)` now says
  where six comes from: three mentions per property name, two properties.

One Spec finding was a real gap rather than a wording problem: nothing exercised
an evaluator-recursion boundary, so "no stack exhaustion" was only ever asserted
well inside the safe range. Measured, and it found a divergence —
`two_hundred_and_fifty_six_levels_of_arithmetic_around_a_shadowing_param` pins
the depth both compilers agree on, and issue 20 carries the rest: at 512 levels
we abort the process where Babel accepts 768 and raises a catchable `RangeError`
past that. Not assertable here, because a test that crosses the boundary takes
the test binary with it.

The remaining Standards note — most test names state the input rather than the
expected behaviour — is left as-is, matching `dynamic_styles.rs` rather than
diverging in one file.
