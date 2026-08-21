# 31 — The fold's placeholder decides a `defineVars` sentence

Status: `needs-triage`
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

- [ ] The `defineVars` refusal names the input rather than the placeholder, or
      the sentence is recorded as a decided divergence with the reason stated
- [ ] Whatever lands does not change the placeholder back to an absent value
