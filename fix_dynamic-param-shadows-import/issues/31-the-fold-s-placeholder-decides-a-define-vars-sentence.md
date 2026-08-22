# 31 — The fold's placeholder decides a `defineVars` sentence

Status: `resolved`
Blocked by: None

**What was found:** The object a folded function map materializes to carries a
function in every value slot, because that is what the reference implementation's
`identifiers` holds. The placeholder is never read by a position that refuses on
the key -- but `defineVars` reads the value first, so the placeholder is what
decides the sentence its refusal gives.

```js
import { defineVars, keyframes } from '@stylexjs/stylex';

export const vars = defineVars({ a: keyframes });
```

| | |
| --- | --- |
| Babel 0.19.0 | `Default value is not defined for a variable.` |
| here, before [15](./15-the-function-map-read-where-it-is-not-a-map.md) | `Only static values are allowed inside of a defineVars() call.` |
| here, now | `Function values in defineVars() must be zero-argument and return a static value supported by defineVars().` |

All three refuse the same input, so no build emits the wrong CSS. What changed is
which sentence an author reads, and the new one is a step further from what they
wrote: they wrote `keyframes`, and the message talks about zero-argument
functions.

The defence, and the reason 15 landed it anyway: upstream's object holds a
function in that slot too -- `identifiers[keyframes] = { fn: keyframes$1 }` -- so
the message is not describing a construct this compiler invented. Both compilers
hold a function there and refuse; upstream looks for a `default` key before it
looks at the value, and this one looks at the value first. A check-ordering
difference inside `defineVars`.

The counter-argument, raised in review of 15 and worth keeping: 16 had explicitly
decided this consumer was out of scope -- *"materializing at this consumer too is
a decision about every `defineVars` value, not about this fold"* -- and the
placeholder change reached it anyway, trading one divergence from upstream for a
less helpful one. A synthetic placeholder that reaches a user-facing sentence is
exactly the shape that decision was protecting.

`null` is not the alternative. It was the placeholder before, and it is an
*absent value*: it made `{ ...keyframes, color: 'red' }` compile `{color:red}`,
a style object the author did not write, where upstream refuses. Trading a
wrong-output bug for a worse sentence was the right way round; getting both is
what this ticket is for.

Where to look: `define_vars_utils.rs:122` and the check above it. If the
`default`-key check ran first, as upstream's does, this input would read
upstream's sentence and every other `defineVars` value would keep the sentence it
has. Measure that against the suite before assuming it is only a reorder --
`createTheme` reads the same pair of checks and already agrees with upstream on
which one speaks.

Pinned as it stands in
`validation_stylex_define_vars_test::stylex_validation_define_vars::a_folded_function_map_read_as_a_variable_value_is_refused_for_its_function`.
The `createTheme` half is
`modules-1266-a-folded-function-map-in-a-create-theme-override`; `defineVars`
cannot be a corpus subject, because the corpus hands every subject the same
filename and a `defineVars` call hashes the file that declares it.

- [x] The `defineVars` refusal names the input rather than the placeholder, or
      the sentence is recorded as a decided divergence with the reason stated
- [x] Whatever lands does not change the placeholder back to an absent value

## Resolved

It was only a reorder, and the suite says so. An object value with no `default`
key is refused for the shape it is, before anything looks at what it holds --
the order `normalizeDefineVarsValue` checks in, with the CSS-type test ahead of
it as upstream has (`stylex.types.color('red')` carries its own value under a
`syntax` and has no `default` of its own to look for). The placeholder is
untouched: it is still the function the entry holds, and it is simply no longer
the thing that speaks.

Two more differences in the same sentence, found while measuring it:

- **The name was quoted.** `Default value is not defined for "a" variable.`
  against upstream's `Default value is not defined for a variable.` -- the whole
  of what a build error read differently for this input, in both consumers.
  `missing_default_value` is that sentence now, once.
- **A nested level named the at-rule it was standing on.** Upstream carries the
  variable's own name down the recursion, so `cornerRadius` is what an author is
  told about however deep the object nests; this one passed the at-rule down and
  said `Default value is not defined for @media (min-width: 600px) variable.`

The check reaches **every level** of a value and not only the top one, as
`normalizeDefineVarsValue`'s own recursion does. Checking the top level alone
left a fold buried under an at-rule still reading the function sentence, which
is the same defect one level down -- found in review of this change and pinned
as `a_nested_folded_function_map_is_refused_for_its_missing_default`.

Measured: `modules-1266-a-folded-function-map-in-a-create-theme-override` reads
`both-reject` where it read `both-reject (diverged)` -- the `createTheme` half
of the same pair of checks, closed by the same change. `defineVars` cannot be a
corpus subject, so its half is pinned in
`validation_stylex_define_vars_test::stylex_validation_define_vars`: the fold,
the plain object with no `default`, the nested one, the empty one, and a
parameterized arrow beside an object value -- which still reads the function
sentence, so the reorder did not move that check off the shapes it owns.
